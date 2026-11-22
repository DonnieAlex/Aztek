//! Construction history for poly edits: with history on, a component edit inserts a node (polyExtrudeFace1,
//! polyBevel1, ...) between whatever fed the mesh and the mesh, holding the edit's settings as attributes and the
//! components it worked on in inputComponents. Changing an attribute re-runs the edit on its input. A mesh with no
//! history gets an intermediate copy of itself (polySurfaceShape1) to feed the first node.

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use aztek_geo::symmetry::{clear_symmetry_keys, SymmetryMap, SymmetrySpec};
use aztek_geo::{Index, Mesh, MeshRemap};
use aztek_ops::{objects, ArgType, Args, CmdResult, CommandError, Commands};
use glam::DVec3;
use serde_json::{json, Map, Value};

use crate::attr_defs::mesh_attr;
use crate::components::{self, ComponentMode, Ids};
use crate::graph::Graph;
use crate::mesh_edit::{symmetry_tolerance, EditResult};
use crate::mesh_nodes::MeshNode;
use crate::node::{types, vec3_attr, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::DagNode;

/// One edit as a pure function of its flags, the mesh, the components and the symmetry.
pub type OpFn = fn(&Args, &mut Mesh, &Ids, Option<&SymmetryMap>) -> CmdResult<EditResult>;

pub struct HistoryOp {
    /// The history node's type: polyExtrudeFace.
    pub node_type: &'static str,
    /// The command whose flags become the node's attributes.
    pub command: &'static str,
    /// Undo label.
    pub label: &'static str,
    /// What inputComponents holds.
    pub kind: ComponentMode,
    pub run: OpFn,
    /// Command flags that are not settings of the edit (they pick a mode or a target).
    pub skip_flags: &'static [&'static str],
    pub doc: &'static str,
}

static OPS: RwLock<BTreeMap<String, Arc<HistoryOp>>> = RwLock::new(BTreeMap::new());

pub fn op(node_type: &str) -> Option<Arc<HistoryOp>> {
    OPS.read().unwrap().get(node_type).cloned()
}

pub fn is_history(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.ty().is_a(BASE_TYPE))
}

pub(crate) const BASE_TYPE: &str = "polyModifier";

pub const INPUT_POLYMESH: usize = 0;
pub const OUTPUT: usize = 1;
pub const INPUT_COMPONENTS: usize = 2;
/// -1 off, else the mirror axis 0-2.
pub const SYMMETRY_AXIS: usize = 3;
pub const SYMMETRY_OFFSET: usize = 4;
/// Where the op's own settings start.
pub const COUNT: usize = 5;

/// Whether edits record history; constructionHistory -toggle.
pub fn enabled(g: &Graph) -> bool {
    g.construction_history
}

// ---------------------------------------------------------------------------------------------------------------------
// Component strings

fn prefix(kind: ComponentMode) -> &'static str {
    match kind {
        ComponentMode::Vertex => "vtx",
        ComponentMode::Edge => "e",
        ComponentMode::Face => "f",
        ComponentMode::Uv => "map",
        ComponentMode::VertexFace => "vtxFace",
        ComponentMode::Object => "",
    }
}

/// The reference's component list: "f[0:3] f[5]". Empty for the whole mesh.
pub fn format_components(kind: ComponentMode, ids: &[Index]) -> String {
    let mut v = ids.to_vec();
    v.sort_unstable();
    v.dedup();
    let p = prefix(kind);
    let mut out = Vec::new();
    let mut i = 0;
    while i < v.len() {
        let mut j = i;
        while j + 1 < v.len() && v[j + 1] == v[j] + 1 {
            j += 1;
        }
        out.push(if i == j { format!("{p}[{}]", v[i]) } else { format!("{p}[{}:{}]", v[i], v[j]) });
        i = j + 1;
    }
    out.join(" ")
}

/// format_components() back to ids; anything unreadable is skipped.
pub fn parse_components(s: &str) -> Ids {
    let mut out = Vec::new();
    for item in s.split_whitespace() {
        let (Some(open), true) = (item.find('['), item.ends_with(']')) else { continue };
        let range = &item[open + 1..item.len() - 1];
        match range.split_once(':') {
            Some((a, b)) => {
                if let (Ok(a), Ok(b)) = (a.parse::<Index>(), b.parse::<Index>()) {
                    out.extend(a..=b);
                }
            }
            None => {
                if let Ok(a) = range.parse::<Index>() {
                    out.push(a);
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// Running an edit

/// What one edit did to a mesh: the op's result ids and the compaction that followed.
pub struct Applied {
    pub result: EditResult,
    pub remap: MeshRemap,
}

/// Runs `f` on mesh the way every component edit does: with the symmetry map spec gives, the new points' symmetry keys
/// cleared, the moved and new points' orbits enforced from the driving wedge, then compacted.
pub fn apply_edit(
    mesh: &mut Mesh,
    spec: Option<&SymmetrySpec>,
    sel: &Ids,
    kind: ComponentMode,
    f: &mut dyn FnMut(&mut Mesh, &Ids, Option<&SymmetryMap>) -> CmdResult<EditResult>,
) -> CmdResult<Applied> {
    let sym = spec.map(|s| {
        let tol = symmetry_tolerance(mesh);
        SymmetryMap::persistent(mesh, s, tol, None)
    });
    let mut before = Vec::new();
    let mut drive = 0;
    if let Some(sym) = &sym {
        before = mesh.positions().to_vec();
        drive = sym.majority_wedge(&components::convert(mesh, kind, sel, ComponentMode::Vertex, false));
    }
    let first_new = mesh.point_capacity();
    // The action takes the picked side and its mirror image, whichever action it is.
    let mirrored = crate::mesh_edit::with_mirror(mesh, sym.as_ref(), sel, kind);
    let result = f(mesh, &mirrored, sym.as_ref())?;
    clear_symmetry_keys(mesh, first_new);
    if let Some(spec) = spec {
        enforce(mesh, spec, &before, drive);
    }
    let mut remap = MeshRemap::default();
    mesh.compact(Some(&mut remap));
    Ok(Applied { result, remap })
}

/// maintain_symmetry for a spec rather than a mesh node.
pub fn enforce(mesh: &mut Mesh, spec: &SymmetrySpec, before: &[DVec3], drive_wedge: i32) {
    let first_new = before.len() as Index;
    let tol = symmetry_tolerance(mesh);
    let map = SymmetryMap::persistent(mesh, spec, tol, None);
    let touched: Ids = (0..mesh.point_capacity())
        .filter(|&p| {
            mesh.point_valid(p)
                && (p >= first_new || before[p as usize].to_array().map(f64::to_bits) != mesh.position(p).to_array().map(f64::to_bits))
        })
        .collect();
    map.enforce_wedge(mesh, &touched, drive_wedge);
}

// ---------------------------------------------------------------------------------------------------------------------
// The node

pub struct HistoryNode {
    op: Arc<HistoryOp>,
}

/// The node's settings as the flags its command reads.
fn args_of(d: &NodeData) -> Args {
    let mut flags = Map::new();
    let ty = d.ty();
    for (i, a) in ty.attrs.iter().enumerate().skip(COUNT) {
        let v = match d.value(i) {
            PlugValue::Bool(b) => json!(b),
            PlugValue::Int(v) => json!(v),
            PlugValue::Float(v) => json!(v),
            PlugValue::Vec3(v) => json!([v.x, v.y, v.z]),
            PlugValue::String(s) => {
                // List and Any flags are stored as JSON text.
                if a.nice_name == "json" {
                    serde_json::from_str(s).unwrap_or(Value::Null)
                } else {
                    json!(s)
                }
            }
            _ => continue,
        };
        flags.insert(a.name.clone(), v);
    }
    Args { positional: Vec::new(), flags }
}

fn spec_of(d: &NodeData) -> Option<SymmetrySpec> {
    let axis = d.int(SYMMETRY_AXIS);
    if !(0..=2).contains(&axis) {
        return None;
    }
    Some(SymmetrySpec { axis: axis as i32, offset: d.float(SYMMETRY_OFFSET), radial: None, plane: None, free_seam: false })
}

thread_local! {
    /// The result a command just worked out for the history node it made: (its input, the node's settings, the result).
    /// The node's first compute takes it instead of running the edit again, which on a dense mesh is most of the time
    /// an edit takes. Anything else about the node (another input, other settings) and it computes as usual.
    static PRIMED: std::cell::RefCell<Option<(crate::plug::MeshPtr, String, crate::plug::MeshPtr)>> = const { std::cell::RefCell::new(None) };
}

fn settings_key(d: &NodeData) -> String {
    format!("{}|{}|{}|{}|{}", d.ty().name, d.string(INPUT_COMPONENTS), d.int(SYMMETRY_AXIS), d.float(SYMMETRY_OFFSET), Value::Object(args_of(d).flags))
}

/// Hands history node `h` the result its command computed from `input`, for its next compute.
pub fn prime(g: &Graph, h: NodeId, input: crate::plug::MeshPtr, result: crate::plug::MeshPtr) {
    let Some(n) = g.node(h) else { return };
    let key = settings_key(n.data());
    PRIMED.with(|p| *p.borrow_mut() = Some((input, key, result)));
}

fn take_primed(input: &crate::plug::MeshPtr, d: &NodeData) -> Option<crate::plug::MeshPtr> {
    PRIMED.with(|p| {
        let mut p = p.borrow_mut();
        let hit = p.as_ref().is_some_and(|(i, k, _)| Arc::ptr_eq(i, input) && *k == settings_key(d));
        if hit {
            p.take().map(|(_, _, r)| r)
        } else {
            None
        }
    })
}

impl NodeBehavior for HistoryNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let Some(input) = d.mesh(INPUT_POLYMESH).cloned() else {
            d.set_output(OUTPUT, PlugValue::Mesh(None));
            return;
        };
        // HasNoEffect: the mesh passes through.
        if crate::history_qe::node_state(d) == 1 {
            d.set_output(OUTPUT, PlugValue::Mesh(Some(input)));
            return;
        }
        if let Some(r) = take_primed(&input, d) {
            d.set_output(OUTPUT, PlugValue::Mesh(Some(r)));
            return;
        }
        let args = args_of(d);
        let ids = parse_components(d.string(INPUT_COMPONENTS));
        let spec = spec_of(d);
        let mut m = (*input).clone();
        let run = self.op.run;
        match apply_edit(&mut m, spec.as_ref(), &ids, self.op.kind, &mut |m, s, sym| run(&args, m, s, sym)) {
            Ok(_) => d.set_output(OUTPUT, PlugValue::Mesh(Some(Arc::new(m)))),
            Err(e) => {
                log::warn!("{}: {e}", self.op.node_type);
                d.set_output(OUTPUT, PlugValue::Mesh(Some(input)));
            }
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Command flags that say how the command runs rather than what the edit does: never attributes.
const NOT_ATTRS: [&str; 7] = ["constructionHistory", "name", "query", "edit", "queried", "object", "createHistory"];
/// Values the command works out for its node (a curve's samples, the object's matrix): stored but not shown.
const HIDDEN_ATTRS: [&str; 6] = ["worldInverse", "curvePoints", "vertices", "creaseVertices", "blindColors", "geoSamples"];
/// Vector flags whose X, Y and Z flags are the vector's parts: the node has one attribute with three children.
const VECTOR_PARTS: [(&str, &str); 14] = [
    ("translate", "translate"),
    ("cutPlaneCenter", "cutPlaneCenter"),
    ("cutPlaneRotate", "cutPlaneRotate"),
    ("extractOffset", "extractOffset"),
    ("rotate", "rotate"),
    ("scale", "scale"),
    ("localTranslate", "localTranslate"),
    ("localRotate", "localRotate"),
    ("localScale", "localScale"),
    ("localDirection", "localDirection"),
    ("pivot", "pivot"),
    ("gravity", "gravity"),
    ("magnet", "magn"),
    ("direction", "direction"),
];

/// Folds -ltx/-lty/-ltz style flags into their vector flag, so one attribute holds them.
pub fn fold_vectors(a: &Args) -> Args {
    let mut out = a.clone();
    for (vector, part) in VECTOR_PARTS {
        let parts: Vec<Option<f64>> = ["X", "Y", "Z"].iter().map(|c| out.flags.remove(&format!("{part}{c}")).and_then(|v| v.as_f64())).collect();
        if parts.iter().all(Option::is_none) {
            continue;
        }
        let was = out.vec3(vector).unwrap_or(if vector.ends_with("cale") { [1.0; 3] } else { [0.0; 3] });
        let now: Vec<f64> = (0..3).map(|i| parts[i].unwrap_or(was[i])).collect();
        out.flags.insert(vector.to_string(), json!(now));
    }
    out
}

fn is_vector_part(all: &[aztek_ops::FlagDef], f: &aztek_ops::FlagDef) -> bool {
    VECTOR_PARTS.iter().any(|(vector, part)| {
        ["X", "Y", "Z"].iter().any(|c| f.name == format!("{part}{c}")) && all.iter().any(|x| x.name == *vector && x.ty == ArgType::Vec3)
    })
}

fn attr_for(f: &aztek_ops::FlagDef) -> Option<AttrDef> {
    let mut fl = flags::INPUT | flags::STORABLE;
    if HIDDEN_ATTRS.contains(&f.name.as_str()) {
        fl |= flags::HIDDEN;
    }
    let a = match f.ty {
        ArgType::Bool => AttrDef {
            name: f.name.clone(),
            ty: PlugType::Bool,
            default_value: PlugValue::Bool(f.default_value.as_bool().unwrap_or(false)),
            flags: fl,
            doc: f.doc.clone(),
            short_name: f.short_name.clone(),
            ..Default::default()
        },
        ArgType::Int => AttrDef {
            name: f.name.clone(),
            ty: PlugType::Int,
            default_value: PlugValue::Int(f.default_value.as_i64().unwrap_or(0)),
            short_name: f.short_name.clone(),
            flags: fl | flags::KEYABLE,
            doc: f.doc.clone(),
            min_value: f.min,
            max_value: f.max,
            ..Default::default()
        },
        ArgType::Float => AttrDef {
            name: f.name.clone(),
            ty: PlugType::Float,
            default_value: PlugValue::Float(f.default_value.as_f64().unwrap_or(0.0)),
            short_name: f.short_name.clone(),
            flags: fl | flags::KEYABLE,
            doc: f.doc.clone(),
            min_value: f.min,
            max_value: f.max,
            ..Default::default()
        },
        ArgType::String => AttrDef {
            name: f.name.clone(),
            ty: PlugType::String,
            default_value: PlugValue::String(f.default_value.as_str().unwrap_or("").to_string()),
            short_name: f.short_name.clone(),
            flags: fl,
            doc: f.doc.clone(),
            ..Default::default()
        },
        ArgType::Vec3 => {
            let c = |i: usize| f.default_value.get(i).and_then(Value::as_f64).unwrap_or(0.0);
            vec3_attr(&f.name, &f.short_name, DVec3::new(c(0), c(1), c(2)), fl | flags::KEYABLE, &f.doc)
        }
        ArgType::List | ArgType::Any => AttrDef {
            name: f.name.clone(),
            ty: PlugType::String,
            default_value: PlugValue::String(if f.default_value.is_null() { String::new() } else { f.default_value.to_string() }),
            short_name: f.short_name.clone(),
            flags: fl,
            doc: f.doc.clone(),
            nice_name: "json".into(),
            ..Default::default()
        },
    };
    Some(a)
}

/// Registers an op; its node type is made by register_types once its command exists.
pub fn register_op(op: HistoryOp) {
    OPS.write().unwrap().insert(op.node_type.to_string(), Arc::new(op));
}

// Sessions can start on several threads at once.
static TYPE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The node type every history node derives from, for modules making their own kinds of history node.
pub fn ensure_base_type() {
    let _guard = TYPE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_base_locked();
}

fn ensure_base_locked() {
    register_tweak_type();
    if types::find(BASE_TYPE).is_none() {
        let base = NodeType {
            name: BASE_TYPE.into(),
            icon: "history".into(),
            doc: "A recorded poly edit: re-runs on its input whenever its settings or input change.".into(),
            own_attrs: vec![
                mesh_attr("inputPolymesh", "ip", flags::INPUT | flags::HIDDEN, "The mesh before the edit."),
                mesh_attr("output", "out", flags::OUTPUT | flags::HIDDEN, "The mesh after the edit."),
                AttrDef {
                    name: "inputComponents".into(),
                    short_name: "ics".into(),
                    ty: PlugType::String,
                    default_value: PlugValue::String(String::new()),
                    // Kept in the file and in scripts, but not a field in the editors: which components an edit takes
                    // is changed through the selection, as in the reference (#191).
                    flags: flags::INPUT | flags::STORABLE | flags::HIDDEN,
                    doc: "The components the edit works on, as f[0:3] e[5]; empty for the whole mesh.".into(),
                    ..Default::default()
                },
                AttrDef {
                    name: "symmetryAxis".into(),
                    ty: PlugType::Int,
                    default_value: PlugValue::Int(-1),
                    flags: flags::INPUT | flags::STORABLE | flags::HIDDEN,
                    ..Default::default()
                },
                AttrDef {
                    name: "symmetryOffset".into(),
                    ty: PlugType::Float,
                    default_value: PlugValue::Float(0.0),
                    flags: flags::INPUT | flags::STORABLE | flags::HIDDEN,
                    ..Default::default()
                },
            ],
            create: None,
            ..Default::default()
        };
        types::register_type(base).expect(BASE_TYPE);
    }
}

/// Node types for every registered op, attributes taken from their commands' flags.
pub fn register_types(c: &Commands<Session>) {
    let _guard = TYPE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_base_locked();
    let base = types::find(BASE_TYPE).unwrap();
    for op in OPS.read().unwrap().values() {
        if types::find(op.node_type).is_some() {
            continue;
        }
        let Some(cmd) = c.find(op.command) else {
            log::warn!("history op {}: no command {}", op.node_type, op.command);
            continue;
        };
        let own: Vec<AttrDef> = cmd
            .flags
            .iter()
            .filter(|f| !op.skip_flags.contains(&f.name.as_str()) && !NOT_ATTRS.contains(&f.name.as_str()) && !is_vector_part(&cmd.flags, f))
            .filter_map(attr_for)
            .collect();
        let op2 = op.clone();
        let t = NodeType {
            name: op.node_type.into(),
            base: Some(base.clone()),
            doc: op.doc.into(),
            own_attrs: own,
            create: Some(Arc::new(move || Box::new(HistoryNode { op: op2.clone() }))),
            ..Default::default()
        };
        types::register_type(t).expect(op.node_type);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Inserting and deleting

fn plug(node: NodeId, attr: usize) -> PlugRef {
    PlugRef::new(node, attr as i32)
}

/// Whether an edit on mesh can record history: history on, the mesh its own (not an instance) and not mid edge edit.
pub fn can_record(g: &Graph, mesh: NodeId) -> bool {
    enabled(g)
        && crate::instancing::kind(g, mesh) == crate::instancing::Kind::None
        && !crate::modifiers::stack(g, mesh).iter().any(|&id| crate::modifiers::editing_input(g, id))
}

/// The plug feeding the bottom of mesh's chain, making an intermediate copy of the mesh to feed it when nothing does.
pub(crate) fn upstream(g: &mut Graph, mesh: NodeId) -> CmdResult<(PlugRef, PlugRef)> {
    let (base, src) = crate::modifiers::chain_base(g, mesh);
    if let Some(src) = src {
        return Ok((base, src));
    }
    g.evaluate();
    let stored = g.stored_value(&base).clone();
    let xf = crate::dag::transform_of(g, mesh);
    let mt = types::find("mesh").unwrap();
    let orig = g.create_node(&mt, "polySurfaceShape1", if xf == mesh { NO_NODE } else { xf }, true)?;
    g.set_value(plug(orig, DagNode::INTERMEDIATE_OBJECT), PlugValue::Bool(true))?;
    g.set_value(plug(orig, MeshNode::IN_MESH), stored)?;
    Ok((base, plug(orig, MeshNode::OUT_MESH)))
}

/// A polyTweak's offsets as they can be saved.
pub(crate) fn tweaks_json(g: &Graph, tweak: NodeId) -> Value {
    match g.node(tweak).map(|n| n.data().value(TWEAK)) {
        Some(PlugValue::Data(Some(d))) => d.to_json(),
        _ => Value::Null,
    }
}

/// Puts a polyTweak's offsets back, or empties them.
pub(crate) fn set_tweaks_json(g: &mut Graph, tweak: NodeId, saved: Option<&Value>) -> CmdResult<()> {
    let mut t = Tweaks::default();
    for e in saved.and_then(|s| s.get("points")).and_then(Value::as_array).into_iter().flatten() {
        let c = |k: usize| e.get(k).and_then(Value::as_f64).unwrap_or(0.0);
        t.points.insert(c(0) as Index, DVec3::new(c(1), c(2), c(3)));
    }
    g.set_value(plug(tweak, TWEAK), PlugValue::Data(Some(Arc::new(t))))?;
    Ok(())
}

/// Inserts a history node for op at the bottom of mesh's chain, holding args, the components and the symmetry.
pub fn insert(
    g: &mut Graph,
    mesh: NodeId,
    op: &HistoryOp,
    args: &Args,
    ids: &[Index],
    spec: Option<&SymmetrySpec>,
) -> CmdResult<NodeId> {
    let t = types::find(op.node_type).ok_or_else(|| CommandError::new(format!("no history node type {}", op.node_type)))?;
    let (base, src) = upstream(g, mesh)?;
    // The node type's number isn't part of its name: polyBevel3 makes polyBevel1.
    let h = g.create_node(&t, if op.node_type == "polyBevel3" { "polyBevel1" } else { "" }, NO_NODE, true)?;
    let attrs = t.attrs.clone();
    for (i, a) in attrs.iter().enumerate().skip(COUNT) {
        let Some(v) = args.flags.get(&a.name) else { continue };
        if v.is_null() {
            continue;
        }
        let pv = match a.ty {
            PlugType::Bool => v.as_bool().map(PlugValue::Bool),
            PlugType::Int => v.as_i64().map(PlugValue::Int),
            PlugType::Float => v.as_f64().map(PlugValue::Float),
            PlugType::Vec3 => {
                let c = |k: usize| v.get(k).and_then(Value::as_f64).unwrap_or(0.0);
                Some(PlugValue::Vec3(DVec3::new(c(0), c(1), c(2))))
            }
            PlugType::String if a.nice_name == "json" => Some(PlugValue::String(v.to_string())),
            PlugType::String => v.as_str().map(|s| PlugValue::String(s.to_string())),
            _ => None,
        };
        if let Some(pv) = pv {
            g.set_value(plug(h, i), pv)?;
        }
    }
    g.set_value(plug(h, INPUT_COMPONENTS), PlugValue::String(format_components(op.kind, ids)))?;
    if let Some(s) = spec {
        g.set_value(plug(h, SYMMETRY_AXIS), PlugValue::Int(s.axis as i64))?;
        g.set_value(plug(h, SYMMETRY_OFFSET), PlugValue::Float(s.offset))?;
    }
    g.disconnect(&base);
    g.connect(&src, &plug(h, INPUT_POLYMESH), false)?;
    g.connect(&plug(h, OUTPUT), &base, false)?;
    Ok(h)
}

/// Everything upstream of plug that only feeds it: generators, history nodes, intermediate shapes. Nearest first.
pub fn upstream_nodes(g: &Graph, from: &PlugRef) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut stack = vec![*from];
    while let Some(p) = stack.pop() {
        let Some(src) = g.source(&p) else { continue };
        let id = src.node;
        if out.contains(&id) {
            continue;
        }
        let Some(n) = g.node(id) else { continue };
        let dag = n.ty().dag;
        let intermediate = dag && n.ty().is_a("mesh") && n.bool(DagNode::INTERMEDIATE_OBJECT);
        if dag && !intermediate {
            continue;
        }
        // Shared with something else downstream: it stays.
        let feeds_other = n
            .ty()
            .attrs
            .iter()
            .enumerate()
            .flat_map(|(i, _)| g.destinations(&plug(id, i)))
            .any(|d| !(d.node == p.node && d.attr == p.attr) && !out.contains(&d.node));
        if feeds_other {
            continue;
        }
        out.push(id);
        for (i, a) in n.ty().attrs.iter().enumerate() {
            if a.flags & flags::INPUT != 0 {
                stack.push(plug(id, i));
            }
        }
    }
    out
}

/// bakePartialHistory -prePostDeformers: the modelling edits between a deformed mesh's deformers and the mesh move to
/// feed its original instead, so baking the original takes them in. Skin weights follow the points: a point the edits
/// add or renumber takes the weights of where it lies on the original before, blended between the points round it
/// (#179); `smooth` (-postSmooth) evens out the new points' weights with their neighbours'.
fn move_post_deformer_edits(g: &mut Graph, shape: NodeId, orig: NodeId, smooth: bool) -> CmdResult<()> {
    let is_edit = |g: &Graph, id: NodeId| {
        g.node(id).is_some_and(|n| n.ty().is_a(BASE_TYPE) || n.ty().name == TWEAK_TYPE || crate::uv_ops::is_uv_node(n.ty().name.as_str()))
    };
    // The edits after the deformers, nearest the mesh first.
    let mut post = Vec::new();
    let mut p = plug(shape, MeshNode::IN_MESH);
    while let Some(src) = g.source(&p) {
        if !is_edit(g, src.node) {
            break;
        }
        post.push(src.node);
        p = plug(src.node, INPUT_POLYMESH);
    }
    let Some(deformed) = g.source(&p) else { return Ok(()) };
    if post.is_empty() || deformed.node == orig || g.node(deformed.node).is_some_and(|n| n.ty().is_a("mesh")) {
        return Ok(());
    }
    g.evaluate();
    let mesh_of = |g: &Graph, m: NodeId| -> Option<Arc<aztek_geo::Mesh>> {
        match g.node(m).map(|n| n.data().value(MeshNode::OUT_MESH)) {
            Some(PlugValue::Mesh(Some(mesh))) => Some(mesh.clone()),
            _ => None,
        }
    };
    let points_of = |mesh: &Option<Arc<aztek_geo::Mesh>>| -> Vec<DVec3> {
        match mesh {
            Some(mesh) => (0..mesh.point_capacity()).map(|i| if mesh.point_valid(i) { mesh.position(i) } else { DVec3::splat(f64::NAN) }).collect(),
            None => Vec::new(),
        }
    };
    let old_mesh = mesh_of(g, orig);
    let before = points_of(&old_mesh);
    let (first, last) = (post[0], *post.last().unwrap());
    // Out of the deformed end: the deformers feed the mesh.
    g.disconnect(&plug(last, INPUT_POLYMESH));
    g.disconnect(&plug(shape, MeshNode::IN_MESH));
    g.connect(&deformed, &plug(shape, MeshNode::IN_MESH), false)?;
    // Into the original's: what made the original feeds the edits, which make the original.
    let made = match g.source(&plug(orig, MeshNode::IN_MESH)) {
        Some(s) => s,
        None => {
            let stored = g.stored_value(&plug(orig, MeshNode::IN_MESH)).clone();
            let src = g.create_node(&types::find("meshSource").ok_or_else(|| CommandError::new("no meshSource node type"))?, "", NO_NODE, true)?;
            g.set_value(plug(src, crate::modifiers::MeshSourceNode::MESH), stored)?;
            plug(src, crate::modifiers::MeshSourceNode::OUT_MESH)
        }
    };
    g.disconnect(&plug(orig, MeshNode::IN_MESH));
    g.connect(&made, &plug(last, INPUT_POLYMESH), false)?;
    g.connect(&plug(first, OUTPUT), &plug(orig, MeshNode::IN_MESH), false)?;
    g.evaluate();
    let new_mesh = mesh_of(g, orig);
    let after = points_of(&new_mesh);
    if after.len() != before.len() || after.iter().zip(&before).any(|(a, b)| (*a - *b).length_squared() > 1e-18) {
        let mut d = deformed.node;
        loop {
            if g.node(d).is_some_and(|n| n.ty().name == "skinCluster") {
                match (&old_mesh, &new_mesh) {
                    (Some(o), Some(n)) => crate::skin::remap_onto_surface(g, d, o, n, smooth)?,
                    _ => crate::skin::remap_to_nearest(g, d, &before, &after)?,
                }
            }
            // Deformers and skins both take the mesh at their first attribute.
            match g.source(&plug(d, 0)) {
                Some(s) if s.node != orig => d = s.node,
                _ => break,
            }
        }
    }
    Ok(())
}

/// Every deformed mesh's edits after its deformers moved before them, as bakePartialHistory -prePostDeformers does
/// but leaving all the history in place: what a file is written from (#180).
pub(crate) fn move_post_edits_everywhere(g: &mut Graph) {
    let shapes: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().is_a("mesh") && !n.bool(DagNode::INTERMEDIATE_OBJECT))).collect();
    for sh in shapes {
        // Tweaks alone after the deformers are written as the reference's tweak in the chain; only an edit that isn't
        // one needs moving.
        let mut p = plug(sh, MeshNode::IN_MESH);
        let mut edit = false;
        while let Some(src) = g.source(&p) {
            let Some(n) = g.node(src.node) else { break };
            if n.ty().name == TWEAK_TYPE {
                p = plug(src.node, INPUT_POLYMESH);
                continue;
            }
            edit = n.ty().is_a(BASE_TYPE) || crate::uv_ops::is_uv_node(n.ty().name.as_str());
            break;
        }
        if !edit {
            continue;
        }
        let original = history_of(g, sh).into_iter().find(|&h| g.node(h).is_some_and(|n| n.ty().is_a("mesh") && n.bool(DagNode::INTERMEDIATE_OBJECT)));
        if let Some(orig) = original {
            if let Err(e) = move_post_deformer_edits(g, sh, orig, false) {
                log::warn!("{e}");
            }
        }
    }
}

/// delete on a node in the middle of a mesh's history, as the reference's: the chain closes up round it, what fed it
/// feeding what it fed, so the nodes before it stay in the history (#173). Nodes with no mesh in and out are left.
pub(crate) fn splice_out(g: &mut Graph, n: NodeId) -> CmdResult<()> {
    let Some(node) = g.node(n) else { return Ok(()) };
    if node.ty().dag {
        return Ok(());
    }
    let (Some(i), Some(o)) = (node.ty().attr_index("inputPolymesh"), node.ty().attr_index("output")) else { return Ok(()) };
    let (input, output) = (PlugRef::new(n, i as i32), PlugRef::new(n, o as i32));
    let Some(src) = g.source(&input) else { return Ok(()) };
    let dst = g.destinations(&output);
    if dst.is_empty() {
        return Ok(());
    }
    g.disconnect(&input);
    for d in dst {
        g.disconnect(&d);
        g.connect(&src, &d, false)?;
    }
    Ok(())
}

/// delete -ch: the mesh keeps what its history makes now, and the history nodes go.
pub fn delete_history(g: &mut Graph, mesh: NodeId) -> CmdResult<bool> {
    g.evaluate();
    let (base, src) = crate::modifiers::chain_base(g, mesh);
    let Some(src) = src else { return Ok(false) };
    if crate::modifiers::is_mesh_source_node(g, src.node) {
        return Ok(false);
    }
    let value = g.value(&src)?;
    let doomed = upstream_nodes(g, &base);
    // A deformer that binds and locks the transform (the skinCluster) takes its lock with it: the history is where the
    // deformer lives, so the object moves again, as Detach Skin leaves it. A deformer that does not lock leaves the
    // locks alone: they are the user's (#207).
    let bound = doomed.iter().any(|&id| g.node(id).is_some_and(|n| crate::deform::locks_transform(&n.ty().name)));
    g.disconnect(&base);
    g.set_value(base, value)?;
    let drivers = crate::deform::orphan_drivers(g, &doomed);
    for id in doomed {
        if g.node(id).is_some() {
            g.delete_node(id)?;
        }
    }
    for id in drivers {
        if g.node(id).is_some() {
            g.delete_node(id)?;
        }
    }
    if bound {
        crate::skin::unlock_bound_transform(g, mesh)?;
    }
    Ok(true)
}

/// The history nodes feeding a mesh, nearest first (listHistory).
pub fn history_of(g: &Graph, mesh: NodeId) -> Vec<NodeId> {
    let (base, _) = crate::modifiers::chain_base(g, mesh);
    let mut out = Vec::new();
    let mut p = base;
    while let Some(src) = g.source(&p) {
        out.push(src.node);
        let Some(n) = g.node(src.node) else { break };
        // The deformers' tweak node passes its input on as they do.
        if n.ty().is_a(BASE_TYPE) || n.ty().name == TWEAK_TYPE || n.ty().name == "tweak" || crate::uv_ops::is_uv_node(n.ty().name.as_str()) || crate::rig::is_rig_chain_node(n.ty().name.as_str()) {
            p = plug(src.node, INPUT_POLYMESH);
        } else if n.ty().is_a("mesh") && n.bool(DagNode::INTERMEDIATE_OBJECT) {
            // A deformed mesh's original: what made it is history too.
            p = plug(src.node, MeshNode::IN_MESH);
        } else if n.ty().name == "transformGeometry" {
            // Freeze Transformations' move: what it moves is history too.
            p = plug(src.node, 0);
        } else {
            break;
        }
    }
    out
}

/// A node's history for what isn't a mesh: its shapes, or itself, then what feeds them, breadth first.
fn upstream_of(g: &Graph, id: NodeId) -> Vec<NodeId> {
    let is_transform = |n: NodeId| g.node(n).is_some_and(|x| x.ty().is_a("transform"));
    let start: Vec<NodeId> = if is_transform(id) {
        let shapes = crate::dag::shapes(g, id, false);
        if shapes.is_empty() { vec![id] } else { shapes }
    } else {
        vec![id]
    };
    let mut feeds: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    for (dst, src) in g.connections() {
        if src.node != dst.node {
            feeds.entry(dst.node).or_default().push(src.node);
        }
    }
    let mut out: Vec<NodeId> = Vec::new();
    let mut queue: std::collections::VecDeque<NodeId> = start.into_iter().collect();
    while let Some(n) = queue.pop_front() {
        if out.contains(&n) {
            continue;
        }
        out.push(n);
        if is_transform(n) && n != id {
            continue;
        }
        for &up in feeds.get(&n).into_iter().flatten() {
            if !out.contains(&up) {
                queue.push_back(up);
            }
        }
    }
    if is_transform(id) {
        out.retain(|&n| n != id);
    }
    out
}

/// listHistory's walk with -future, -allConnections or -levels: the node's shapes (or the node), then every node one
/// connection further in turn, nearest first. Downstream with `future`. Transforms are listed but not walked through
/// unless `all`. `levels` 0 is no limit.
fn walk(g: &Graph, id: NodeId, future: bool, all: bool, levels: usize) -> Vec<NodeId> {
    let is_transform = |n: NodeId| g.node(n).is_some_and(|x| x.ty().is_a("transform"));
    let start: Vec<NodeId> = if is_transform(id) && !all {
        let shapes = crate::dag::shapes(g, id, false);
        if shapes.is_empty() { vec![id] } else { shapes }
    } else {
        vec![id]
    };
    let mut next: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    for (dst, src) in g.connections() {
        if src.node != dst.node {
            let (from, to) = if future { (src.node, dst.node) } else { (dst.node, src.node) };
            let l = next.entry(from).or_default();
            if !l.contains(&to) {
                l.push(to);
            }
        }
    }
    let mut out: Vec<NodeId> = Vec::new();
    let mut queue: std::collections::VecDeque<(NodeId, usize)> = start.into_iter().map(|n| (n, 0)).collect();
    while let Some((n, depth)) = queue.pop_front() {
        if out.contains(&n) {
            continue;
        }
        out.push(n);
        if (is_transform(n) && n != id && !all) || (levels > 0 && depth >= levels) {
            continue;
        }
        for &m in next.get(&n).into_iter().flatten() {
            if !out.contains(&m) {
                queue.push_back((m, depth + 1));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands

/// A shape's geometry attributes: (history, local future, world future), as the reference names them.
fn geometry_attrs(ty: &crate::node::NodeType) -> Option<(&'static str, &'static str, &'static str)> {
    for (base, x) in [
        ("mesh", ("inMesh", "outMesh", "worldMesh")),
        ("nurbsCurve", ("create", "local", "worldSpace")),
        ("nurbsSurface", ("create", "local", "worldSpace")),
        ("lattice", ("latticeInput", "latticeOutput", "worldLattice")),
        ("subdiv", ("create", "outSubdiv", "worldSubdiv")),
    ] {
        if ty.is_a(base) {
            return Some(x);
        }
    }
    None
}

/// The same walk as `walk`, depth first: each branch to its end before the next.
fn walk_depth(g: &Graph, id: NodeId, future: bool, all: bool, levels: usize) -> Vec<NodeId> {
    let is_transform = |n: NodeId| g.node(n).is_some_and(|x| x.ty().is_a("transform"));
    let start: Vec<NodeId> = if is_transform(id) && !all {
        let shapes = crate::dag::shapes(g, id, false);
        if shapes.is_empty() { vec![id] } else { shapes }
    } else {
        vec![id]
    };
    let mut next: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    for (dst, src) in g.connections() {
        if src.node != dst.node {
            let (from, to) = if future { (src.node, dst.node) } else { (dst.node, src.node) };
            let l = next.entry(from).or_default();
            if !l.contains(&to) {
                l.push(to);
            }
        }
    }
    let mut out: Vec<NodeId> = Vec::new();
    let mut stack: Vec<(NodeId, usize)> = start.into_iter().rev().map(|n| (n, 0)).collect();
    while let Some((n, depth)) = stack.pop() {
        if out.contains(&n) {
            continue;
        }
        out.push(n);
        if (is_transform(n) && n != id && !all) || (levels > 0 && depth >= levels) {
            continue;
        }
        for &m in next.get(&n).into_iter().flatten().rev() {
            if !out.contains(&m) {
                stack.push((m, depth + 1));
            }
        }
    }
    out
}

/// What a shape's future is without -allFuture: when its output geometry is connected, only what is downstream of that.
fn future_from_output(g: &Graph, id: NodeId, depth_first: bool, levels: usize) -> Option<Vec<NodeId>> {
    let shape = if g.node(id).is_some_and(|n| n.ty().is_a("transform")) { crate::dag::shapes(g, id, false).into_iter().next()? } else { id };
    let ty = g.node(shape)?.type_arc().clone();
    let (_, local, world) = geometry_attrs(&ty)?;
    let outs: Vec<i32> = [local, world].iter().filter_map(|a| ty.attr_index(a)).map(|i| i as i32).collect();
    let mut firsts: Vec<NodeId> = Vec::new();
    for (dst, src) in g.connections() {
        if src.node == shape && outs.contains(&src.attr) && dst.node != shape && !firsts.contains(&dst.node) {
            firsts.push(dst.node);
        }
    }
    if firsts.is_empty() {
        return None;
    }
    let mut out = vec![shape];
    let lv = if levels > 0 { levels - 1 } else { 0 };
    if levels == 1 {
        out.extend(firsts);
        return Some(out);
    }
    for f in firsts {
        let more = if depth_first { walk_depth(g, f, true, false, lv) } else { walk(g, f, true, false, lv) };
        for m in more {
            if !out.contains(&m) {
                out.push(m);
            }
        }
    }
    Some(out)
}

fn list_history(s: &mut Session, a: &Args) -> CmdResult {
    let g = &s.graph;
    if a.bool("query") {
        let Some(name) = objects(s, a).into_iter().next() else { aztek_ops::fail!("listHistory: name a shape") };
        let Some(id) = g.find(&name)? else { aztek_ops::fail!("listHistory: no object named {name}") };
        let shape = if g.node(id).is_some_and(|n| n.ty().is_a("transform")) { crate::dag::shapes(g, id, false).into_iter().next().unwrap_or(id) } else { id };
        let attrs = g.node(shape).and_then(|n| geometry_attrs(&n.type_arc().clone()));
        let pick = |x: (&'static str, &'static str, &'static str)| {
            if crate::skin::asked(a, "historyAttr") {
                Some(x.0)
            } else if crate::skin::asked(a, "futureLocalAttr") {
                Some(x.1)
            } else if crate::skin::asked(a, "futureWorldAttr") {
                Some(x.2)
            } else {
                None
            }
        };
        if !["historyAttr", "futureLocalAttr", "futureWorldAttr"].iter().any(|k| crate::skin::asked(a, k)) {
            aztek_ops::fail!("listHistory: -q takes -historyAttr, -futureLocalAttr or -futureWorldAttr");
        }
        return Ok(json!(attrs.and_then(pick).map(|x| vec![x.to_string()]).unwrap_or_default()));
    }
    let ty = a.get("type").and_then(Value::as_str);
    let prune = a.bool("pruneDagObjects");
    let interest = a.get("interestLevel").and_then(Value::as_i64);
    let wanted = |id: NodeId| {
        ty.is_none_or(|t| g.node(id).is_some_and(|n| n.ty().is_a(t)))
            && !(prune && g.node(id).is_some_and(|n| n.ty().dag))
            && interest.is_none_or(|lv| g.node(id).is_some_and(|n| n.ty().attr_index("isHistoricallyInteresting").is_none_or(|i| n.data().int(i) >= lv)))
    };
    let name_of = |id: NodeId| if a.bool("fullNodeName") { json!(g.full_name(id)) } else { json!(g.path(id)) };
    let mut out = Vec::new();
    let (future, all, levels) = (a.bool("future"), a.bool("allConnections"), a.get("levels").and_then(Value::as_u64).unwrap_or(0) as usize);
    let breadth = a.bool("breadthFirst") || a.bool("groupLevels");
    let leaf = a.get("leaf").and_then(Value::as_bool).unwrap_or(true);
    for name in objects(s, a) {
        let Some(id) = g.find(&name)? else { aztek_ops::fail!("listHistory: no object named {name}") };
        let is_transform = g.node(id).is_some_and(|n| n.ty().is_a("transform"));
        // -leaf off on a transform: the transform's own history, what drives it.
        if is_transform && !leaf {
            let mut list = if breadth { walk(g, id, future, true, levels.max(1)) } else { walk_depth(g, id, future, true, levels.max(1)) };
            if levels == 0 {
                list = if breadth { walk(g, id, future, true, 0) } else { walk_depth(g, id, future, true, 0) };
            }
            for h in list {
                if wanted(h) && !out.contains(&name_of(h)) {
                    out.push(name_of(h));
                }
            }
            continue;
        }
        if future && !all && !a.bool("allFuture") {
            if let Some(list) = future_from_output(g, id, !breadth, levels) {
                for h in list {
                    if wanted(h) && !out.contains(&name_of(h)) {
                        out.push(name_of(h));
                    }
                }
                continue;
            }
        }
        if future || all || levels > 0 {
            let list = if breadth { walk(g, id, future, all, levels) } else { walk_depth(g, id, future, all, levels) };
            for h in list {
                if wanted(h) && !out.contains(&name_of(h)) {
                    out.push(name_of(h));
                }
            }
            continue;
        }
        let Some(m) = components::as_mesh(g, id) else {
            // Anything else, as the reference walks it: the object's shapes (or the node), then every node
            // upstream of them through their inputs, nearest first. Transforms are listed but not walked
            // through, so a parent's place doesn't drag the scene in.
            for h in upstream_of(g, id) {
                if wanted(h) && !out.contains(&name_of(h)) {
                    out.push(name_of(h));
                }
            }
            continue;
        };
        if wanted(m) {
            out.push(name_of(m));
        }
        for h in history_of(g, m) {
            if wanted(h) {
                out.push(name_of(h));
            }
        }
    }
    Ok(Value::Array(out))
}

pub fn register(c: &mut Commands<Session>) {
    use aztek_ops::{CommandDef, FlagDef};
    c.register(
        CommandDef::<Session>::new(
            "bakePartialHistory",
            "Delete by Type > Non-Deformer History: bakes what feeds the first deformer and keeps the deformers. A mesh with no deformers loses its history.",
            |s, a| {
                let g = &mut s.graph;
                let named: Vec<NodeId> = if a.bool("allShapes") {
                    // Every shape in the scene; none is no error.
                    g.nodes().into_iter().filter(|&i| g.node(i).is_some_and(|n| n.ty().dag && n.ty().is_a("mesh") && !n.bool(DagNode::INTERMEDIATE_OBJECT))).collect()
                } else {
                    let names = a.named();
                    if names.is_empty() {
                        g.selection().to_vec()
                    } else {
                        names.iter().filter_map(|n| g.find(n).ok().flatten()).collect()
                    }
                };
                // Neither -pre nor -pc: before and after the deformers, the default.
                let pre_cache = a.bool("preCache");
                let all = a.bool("prePostDeformers") || (!a.bool("preDeformers") && !pre_cache);
                let smooth = a.bool("postSmooth");
                let mut shapes_of = Vec::new();
                for id in named {
                    let shapes = if g.node(id).is_some_and(|n| n.ty().is_a("transform")) { crate::dag::shapes(g, id, false) } else { vec![id] };
                    for sh in shapes {
                        if g.node(sh).is_some_and(|n| n.ty().is_a("mesh")) && !shapes_of.contains(&sh) {
                            shapes_of.push(sh);
                        }
                    }
                }
                // What would be baked: history before a cache node, or the modelling nodes (before the deformers, or
                // all of them), in the order listHistory gives them, nearest first.
                let cache_at = |g: &Graph, sh: NodeId| history_of(g, sh).iter().position(|&h| g.node(h).is_some_and(|n| matches!(n.ty().name.as_str(), "cacheFile" | "historySwitch" | "cacheBlend")));
                if a.bool("query") {
                    let mut out: Vec<String> = Vec::new();
                    for sh in shapes_of.clone() {
                        let path = s.graph.path(sh);
                        let hist: Vec<String> = match s.run("listHistory", json!({"args": [path]}))? {
                            Value::Array(v) => v.into_iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
                            _ => Vec::new(),
                        };
                        let g = &s.graph;
                        let ids: Vec<NodeId> = hist.iter().filter_map(|n| g.find(n).ok().flatten()).filter(|&h| h != sh).collect();
                        let is_deformer = |h: NodeId| g.node(h).is_some_and(|n| crate::deform::is_deformer(&n.ty().name) || n.ty().name == "tweak" || n.ty().is_a("geometryFilter") || n.ty().dag);
                        let picked: Vec<NodeId> = if pre_cache {
                            match cache_at(g, sh) {
                                Some(_) => {
                                    let cache = ids.iter().position(|&h| g.node(h).is_some_and(|n| matches!(n.ty().name.as_str(), "cacheFile" | "historySwitch" | "cacheBlend")));
                                    cache.map(|i| ids[i + 1..].iter().copied().filter(|&h| !g.node(h).is_some_and(|n| n.ty().dag)).collect()).unwrap_or_default()
                                }
                                None => Vec::new(),
                            }
                        } else {
                            let first_deformer = ids.iter().position(|&h| is_deformer(h) && !g.node(h).is_some_and(|n| n.ty().dag));
                            ids.iter().enumerate().filter(|(i, &h)| !is_deformer(h) && (all || first_deformer.is_none_or(|f| *i > f))).map(|(_, &h)| h).collect()
                        };
                        for h in picked {
                            let n = g.path(h);
                            if !out.contains(&n) {
                                out.push(n);
                            }
                        }
                    }
                    return Ok(json!(out));
                }
                let g = &mut s.graph;
                let mut baked: Vec<String> = Vec::new();
                g.open_chunk("Bake Partial History");
                let mut r = Ok(());
                for sh in shapes_of {
                    {
                        if pre_cache {
                            // Everything feeding the cache becomes the mesh it gave; a mesh with no cache keeps its history.
                            let Some(i) = cache_at(g, sh) else { continue };
                            let hist = history_of(g, sh);
                            let cache = hist[i];
                            let into = g.node(cache).and_then(|n| n.ty().attr_index("inMesh").or(n.ty().attr_index("inp")).or(n.ty().attr_index("input")));
                            if let Some(at) = into {
                                let p = PlugRef::new(cache, at as i32);
                                if let (Some(_), Ok(v)) = (g.source(&p), g.value(&p)) {
                                    let doomed = upstream_nodes(g, &p);
                                    g.disconnect(&p);
                                    let _ = g.set_value(p, v);
                                    for id in doomed {
                                        if g.node(id).is_some() {
                                            let _ = g.delete_node(id);
                                        }
                                    }
                                    baked.push(g.path(sh));
                                }
                            }
                            continue;
                        }
                        baked.push(g.path(sh));
                        // A deformed mesh's history runs through its original: bake that.
                        let original = history_of(g, sh).into_iter().find(|&h| g.node(h).is_some_and(|n| n.ty().is_a("mesh") && n.bool(DagNode::INTERMEDIATE_OBJECT)));
                        // All non-deformer history: the edits made after the deformers go before them first, so they're
                        // baked with the rest (#172). History before deformers only leaves them.
                        if all {
                            if let Some(orig) = original {
                                if let Err(e) = move_post_deformer_edits(g, sh, orig, smooth) {
                                    r = Err(e);
                                    continue;
                                }
                            }
                        }
                        if let Err(e) = delete_history(g, original.unwrap_or(sh)) {
                            r = Err(e);
                        }
                    }
                }
                g.close_chunk();
                r?;
                Ok(json!(baked))
            },
        )
        .positional("objects")
        .flag(FlagDef::new("allShapes", "all", ArgType::Bool, Value::Null, "Every shape in the scene rather than the selection."))
        .flag(FlagDef::new("preCache", "pc", ArgType::Bool, Value::Null, "What comes before a cache, deformers included; -q: the nodes that would be baked."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What -q asks for."))
        .flag(FlagDef::new("prePostDeformers", "ppt", ArgType::Bool, Value::Null, "Before and after the deformers."))
        .flag(FlagDef::new("preDeformers", "pre", ArgType::Bool, Value::Null, "Before the deformers."))
        .flag(FlagDef::new("postSmooth", "nps", ArgType::Bool, Value::Null, "With -prePostDeformers: the weights of the points the edits after the deformers made are smoothed with their neighbours'."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Query.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "constructionHistory",
            "Turn construction history on or off for later poly edits, or query it. Returns whether it is on.",
            |s, a| {
                // -q -tgl or -q -st: whether it is on, changing nothing.
                if a.bool("query") {
                    return Ok(json!(s.graph.construction_history));
                }
                if a.bool("toggle") {
                    s.graph.construction_history = !s.graph.construction_history;
                } else if let Some(v) = a.get("state").and_then(Value::as_bool) {
                    s.graph.construction_history = v;
                }
                Ok(json!(s.graph.construction_history))
            },
        )
        .undoable(false)
        .flag(FlagDef::new("toggle", "tgl", ArgType::Bool, json!(false), "Turn it on if off, off if on."))
        .flag(FlagDef::new("state", "st", ArgType::Bool, Value::Null, "Turn it on (True) or off (False)."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Query: whether it is on.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "listHistory",
            "The nodes feeding an object's mesh, nearest first: history nodes, then the generator or intermediate shape \
             at the bottom.",
            list_history,
        )
        .undoable(false)
        .positional("objects")
        .flag(FlagDef::new("type", "typ", ArgType::String, Value::Null, "Only nodes of this type or a type derived from it (geometryFilter for every deformer)."))
        .flag(FlagDef::new("pruneDagObjects", "pdo", ArgType::Bool, json!(false), "Leave out shapes and transforms: only the history nodes."))
        .flag(FlagDef::new("future", "f", ArgType::Bool, json!(false), "The nodes downstream instead: what the object feeds."))
        .flag(FlagDef::new("allConnections", "ac", ArgType::Bool, json!(false), "Walk every connection, through transforms as well."))
        .flag(FlagDef::new("allFuture", "af", ArgType::Bool, json!(false), "With -f: all of the future; otherwise a shape whose output geometry is connected gives only what is downstream of that."))
        .flag(FlagDef::new("allGraphs", "ag", ArgType::Bool, Value::Null, "Obsolete in the reference: no effect."))
        .flag(FlagDef::new("breadthFirst", "bf", ArgType::Bool, json!(false), "Nearest nodes first; otherwise each branch is followed to its end first."))
        .flag(FlagDef::new("fastIteration", "fi", ArgType::Bool, Value::Null, "The faster walk for nodes with many connections: the same results here."))
        .flag(FlagDef::new("fullNodeName", "fnn", ArgType::Bool, Value::Null, "Nodes by their full DAG paths."))
        .flag(FlagDef::new("groupLevels", "gl", ArgType::Bool, Value::Null, "The nodes level by level: each level's nodes together, nearest level first."))
        .flag(FlagDef::new("interestLevel", "il", ArgType::Int, Value::Null, "Only nodes whose isHistoricallyInteresting is at least this: 0 for everyone, 1 for TDs, 2 for users."))
        .flag(FlagDef::new("leaf", "lf", ArgType::Bool, json!(true), "For a transform, the history of its shape (on); off, of the transform itself."))
        .flag(FlagDef::new("futureLocalAttr", "fl", ArgType::Bool, Value::Null, "With -q: the shape's attribute its local-space future comes from."))
        .flag(FlagDef::new("futureWorldAttr", "fw", ArgType::Bool, Value::Null, "With -q: the shape's attribute its world-space future comes from."))
        .flag(FlagDef::new("historyAttr", "ha", ArgType::Bool, Value::Null, "With -q: the shape's attribute its history connects to."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Query mode."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "Flags asked about in query mode."))
        .flag(FlagDef::new("levels", "lv", ArgType::Int, json!(0), "How many connections out to go; 0 for no limit.")),
    );
    register_types(c);
}

// ---------------------------------------------------------------------------------------------------------------------
// Tweaks

/// Point offsets a polyTweak adds to its input, by point id.
#[derive(Debug, Clone, Default)]
pub struct Tweaks {
    pub points: BTreeMap<Index, DVec3>,
}

impl crate::plug::Data for Tweaks {
    fn kind(&self) -> &str {
        "polyTweaks"
    }
    fn to_json(&self) -> Value {
        let pts: Vec<Value> = self.points.iter().map(|(&p, v)| json!([p, v.x, v.y, v.z])).collect();
        json!({"kind": "polyTweaks", "points": pts})
    }
    fn bytes(&self) -> usize {
        self.points.len() * 40
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Tweaks {
    pub fn from_json(j: &Value) -> Tweaks {
        let mut points = BTreeMap::new();
        for e in j.get("points").and_then(Value::as_array).into_iter().flatten() {
            let f = |i: usize| e.get(i).and_then(Value::as_f64).unwrap_or(0.0);
            points.insert(f(0) as Index, DVec3::new(f(1), f(2), f(3)));
        }
        Tweaks { points }
    }
}

pub const TWEAK_TYPE: &str = "polyTweak";
pub const TWEAK: usize = 2;

pub struct TweakNode;

impl NodeBehavior for TweakNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let Some(input) = d.mesh(INPUT_POLYMESH).cloned() else {
            d.set_output(OUTPUT, PlugValue::Mesh(None));
            return;
        };
        let tweaks = match d.value(TWEAK) {
            PlugValue::Data(Some(t)) => t.as_any().downcast_ref::<Tweaks>().cloned(),
            _ => None,
        };
        let Some(t) = tweaks.filter(|t| !t.points.is_empty()) else {
            d.set_output(OUTPUT, PlugValue::Mesh(Some(input)));
            return;
        };
        let mut m = (*input).clone();
        for (&p, &v) in &t.points {
            if p < m.point_capacity() && m.point_valid(p) {
                let at = m.position(p);
                m.set_position(p, at + v);
            }
        }
        d.set_output(OUTPUT, PlugValue::Mesh(Some(Arc::new(m))));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_tweak_type() {
    if types::find(TWEAK_TYPE).is_some() {
        return;
    }
    crate::plug::data::register_kind("polyTweaks", Arc::new(|j| Ok(Arc::new(Tweaks::from_json(j)) as Arc<dyn crate::plug::Data>)));
    let t = NodeType {
        name: TWEAK_TYPE.into(),
        icon: "arrows-move".into(),
        doc: "Point moves made on a mesh with history, added to its input.".into(),
        own_attrs: vec![
            mesh_attr("inputPolymesh", "ip", flags::INPUT | flags::HIDDEN, "The mesh before the moves."),
            mesh_attr("output", "out", flags::OUTPUT | flags::HIDDEN, "The mesh after the moves."),
            AttrDef {
                name: "tweak".into(),
                short_name: "tk".into(),
                ty: PlugType::Data,
                default_value: PlugValue::Data(None),
                flags: flags::INPUT | flags::STORABLE | flags::HIDDEN,
                doc: "The offset of each moved point.".into(),
                ..Default::default()
            },
        ],
        create: Some(Arc::new(|| Box::new(TweakNode))),
        ..Default::default()
    };
    types::register_type(t).expect(TWEAK_TYPE);
}

/// The polyTweak that point moves on mesh go into, made at the bottom of its chain when history is on and something
/// procedural feeds the mesh; None when moves should go into the stored base instead.
pub fn tweak_node(g: &mut Graph, mesh: NodeId) -> CmdResult<Option<NodeId>> {
    // Symmetry pairs live on the stored mesh, which a tweak can't carry yet: those moves bake.
    if !can_record(g, mesh) || crate::mesh_nodes::symmetry_spec(g, mesh).is_some() {
        return Ok(None);
    }
    let (base, src) = crate::modifiers::chain_base(g, mesh);
    let Some(src) = src else { return Ok(None) };
    if crate::modifiers::is_mesh_source_node(g, src.node) {
        return Ok(None);
    }
    if g.node(src.node).is_some_and(|n| n.ty().name == TWEAK_TYPE) && src.attr == OUTPUT as i32 {
        return Ok(Some(src.node));
    }
    let t = types::find(TWEAK_TYPE).unwrap();
    let tw = g.create_node(&t, "", NO_NODE, true)?;
    g.disconnect(&base);
    g.connect(&src, &plug(tw, INPUT_POLYMESH), false)?;
    g.connect(&plug(tw, OUTPUT), &base, false)?;
    Ok(Some(tw))
}

/// Stores moved as the tweak's result: each point's offset from the tweak's input. moved has the input's topology.
pub fn set_tweak(g: &mut Graph, tweak: NodeId, moved: &Mesh) -> CmdResult<()> {
    g.evaluate();
    let input = g.node(tweak).and_then(|n| n.data().mesh(INPUT_POLYMESH).cloned());
    let Some(input) = input else { return Ok(()) };
    let mut t = Tweaks::default();
    for p in 0..moved.point_capacity().min(input.point_capacity()) {
        if moved.point_valid(p) && input.point_valid(p) {
            let d = moved.position(p) - input.position(p);
            if d != DVec3::ZERO {
                t.points.insert(p, d);
            }
        }
    }
    g.set_value(plug(tweak, TWEAK), PlugValue::Data(Some(Arc::new(t))))?;
    Ok(())
}
