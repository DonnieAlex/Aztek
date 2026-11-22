//! Small system commands: setNodeTypeFlag (whether a node type shows in the interface), tolerance (the positional
//! and tangential tolerances modelling operations take by default), unknownPlugin (the plug-ins a scene names that
//! could not be loaded), shapeCompare (whether two shapes are the same) and suitePrefs (a suite install's shared mouse
//! and keyboard settings).

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use aztek_ops::units::Unit;
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::NodeId;
use crate::session::Session;

type Cmd = CommandDef<Session>;
const B: ArgType = ArgType::Bool;

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

fn q_flags(c: Cmd) -> Cmd {
    c.flag(f("query", "q", B, "Query mode.")).flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
}

fn asked(a: &Args) -> Vec<String> {
    let mut v: Vec<String> = a.get("queried").and_then(Value::as_array).map(|l| l.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    for (k, x) in &a.flags {
        if x.as_bool() == Some(true) && !matches!(k.as_str(), "query" | "queried") && !v.contains(k) {
            v.push(k.clone());
        }
    }
    v
}

fn truth(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|s| matches!(s, "1" | "true" | "on" | "yes")))
}

// ---------------------------------------------------------------------------------------------------------------------
// setNodeTypeFlag

static HIDDEN_TYPES: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

/// Whether a node type is shown in the interface (setNodeTypeFlag -display).
pub fn type_displayed(name: &str) -> bool {
    !HIDDEN_TYPES.lock().unwrap().contains(name)
}

/// A node type named, or given by its type tag as objectType -tagFromType gives it.
fn node_type_arg(a: &Args) -> CmdResult<String> {
    let Some(v) = a.positional.first() else { fail!("setNodeTypeFlag: name a node type") };
    let as_tag = v.as_i64().or_else(|| v.as_str().and_then(|s| s.trim().parse::<i64>().ok()));
    let name = match as_tag {
        Some(tag) => crate::mb_file::type_of_id((tag as u32).to_be_bytes()).map(str::to_string).unwrap_or_default(),
        None => v.as_str().unwrap_or("").to_string(),
    };
    if name.is_empty() || crate::node::types::find(&name).is_none() {
        fail!("setNodeTypeFlag: Unknown node type '{}'", v.as_str().map(str::to_string).unwrap_or_else(|| v.to_string()));
    }
    Ok(name)
}

fn set_node_type_flag(_s: &mut Session, a: &Args) -> CmdResult {
    let name = node_type_arg(a)?;
    if a.bool("query") {
        let q = asked(a);
        if q.iter().any(|k| k == "threadSafe") {
            // Obsolete in the reference: nodes are thread safe here.
            return Ok(json!(true));
        }
        return Ok(json!(type_displayed(&name)));
    }
    if let Some(v) = a.get("display") {
        let mut h = HIDDEN_TYPES.lock().unwrap();
        if truth(v) {
            h.remove(&name);
        } else {
            h.insert(name);
        }
    }
    Ok(json!(true))
}

// ---------------------------------------------------------------------------------------------------------------------
// tolerance

/// Centimetres; 0.01 as the reference's preferences start.
static LINEAR: AtomicU64 = AtomicU64::new(0x3F84_7AE1_47AE_147B);
/// Radians; 0.1 degrees as the reference's preferences start.
static ANGULAR: AtomicU64 = AtomicU64::new(0);

/// The positional tolerance modelling operations use when not given one, in centimetres.
pub fn linear_tolerance() -> f64 {
    f64::from_bits(LINEAR.load(Ordering::Relaxed))
}

/// The tangential tolerance, in radians.
pub fn angular_tolerance() -> f64 {
    let b = ANGULAR.load(Ordering::Relaxed);
    if b == 0 {
        0.1_f64.to_radians()
    } else {
        f64::from_bits(b)
    }
}

fn tolerance(_s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let q = asked(a);
        if q.iter().any(|k| k == "angular") {
            return Ok(json!(angular_tolerance()));
        }
        if q.iter().any(|k| k == "linear") {
            return Ok(json!(linear_tolerance()));
        }
        fail!("tolerance: query -linear or -angular");
    }
    if let Some(l) = a.get("linear").and_then(Value::as_f64) {
        if l <= 0.0 {
            fail!("tolerance: the linear tolerance must be greater than 0");
        }
        LINEAR.store(l.to_bits(), Ordering::Relaxed);
    }
    if let Some(r) = a.get("angular").and_then(Value::as_f64) {
        if r <= 0.0 {
            fail!("tolerance: the angular tolerance must be greater than 0");
        }
        ANGULAR.store(r.to_bits(), Ordering::Relaxed);
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// unknownPlugin

/// A plug-in a scene's requires line named that could not be loaded: its version, node types and data types.
#[derive(Clone, Debug, Default)]
pub struct UnknownPlugin {
    pub version: String,
    pub node_types: Vec<String>,
    pub data_types: Vec<String>,
}

/// The unknown nodes' plug-ins: name to (version, types kept).
fn plugins_of_unknown_nodes(g: &Graph) -> BTreeMap<String, (String, BTreeSet<String>)> {
    let mut out: BTreeMap<String, (String, BTreeSet<String>)> = BTreeMap::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if !crate::core_nodes::is_unknown(&n.ty().name) {
            continue;
        }
        let get = |a: &str| n.ty().attr_index(a).map(|i| n.value(i).as_str().to_string()).unwrap_or_default();
        let p = get("filePlugin");
        if p.is_empty() {
            continue;
        }
        let e = out.entry(p).or_insert_with(|| (get("filePluginVersion"), BTreeSet::new()));
        let t = get("fileType");
        if !t.is_empty() {
            e.1.insert(t);
        }
    }
    out
}

/// Every unknown plug-in of the scene, from its requires lines and from the unknown nodes it holds.
pub fn unknown_plugins(s: &Session) -> BTreeMap<String, UnknownPlugin> {
    let mut out = s.extras.unknown_plugins.clone();
    for (p, (v, tys)) in plugins_of_unknown_nodes(&s.graph) {
        let e = out.entry(p).or_default();
        if e.version.is_empty() {
            e.version = v;
        }
        for t in tys {
            if !e.node_types.contains(&t) {
                e.node_types.push(t);
            }
        }
    }
    out
}

fn unknown_plugin(s: &mut Session, a: &Args) -> CmdResult {
    let all = unknown_plugins(s);
    if a.bool("query") {
        let q = asked(a);
        if q.iter().any(|k| k == "list") {
            return Ok(json!(all.keys().cloned().collect::<Vec<_>>()));
        }
        let Some(name) = a.pos_str(0) else { fail!("unknownPlugin: name an unknown plug-in") };
        let Some(p) = all.get(name) else { fail!("unknownPlugin: '{name}' is not an unknown plug-in") };
        if q.iter().any(|k| k == "version") {
            return Ok(json!(p.version));
        }
        if q.iter().any(|k| k == "nodeTypes") {
            return Ok(json!(p.node_types));
        }
        if q.iter().any(|k| k == "dataTypes") {
            return Ok(json!(p.data_types));
        }
        fail!("unknownPlugin: query -list, -version, -nodeTypes or -dataTypes");
    }
    if a.bool("remove") {
        let Some(name) = a.pos_str(0).map(str::to_string) else { fail!("unknownPlugin: name the unknown plug-in to remove") };
        let Some(p) = all.get(&name) else { fail!("unknownPlugin: '{name}' is not an unknown plug-in") };
        // Its node types still in use keep it.
        let types: HashSet<&String> = p.node_types.iter().collect();
        let g = &s.graph;
        let used = g.nodes().into_iter().any(|id| {
            g.node(id).is_some_and(|n| {
                crate::core_nodes::is_unknown(&n.ty().name) && {
                    let get = |at: &str| n.ty().attr_index(at).map(|i| n.value(i).as_str().to_string()).unwrap_or_default();
                    get("filePlugin") == name || types.contains(&get("fileType"))
                }
            })
        });
        if used {
            fail!("unknownPlugin: '{name}' can't be removed: nodes of its types are in the scene");
        }
        s.extras.unknown_plugins.remove(&name);
        return Ok(Value::Null);
    }
    fail!("unknownPlugin: give -remove, or query with -list, -version, -nodeTypes or -dataTypes");
}

// ---------------------------------------------------------------------------------------------------------------------
// shapeCompare

/// What a shape is, as compared: its kind and numbers.
fn shape_numbers(g: &mut Graph, id: NodeId) -> Option<(&'static str, Vec<i64>, Vec<f64>)> {
    if let Some(m) = crate::components::as_mesh(g, id) {
        g.evaluate();
        let mesh = g.node(m).and_then(|n| n.data().mesh(crate::mesh_nodes::MeshNode::OUT_MESH).cloned())?;
        // Points renumbered in order, so holes left by edits don't count.
        let mut remap = vec![-1i64; mesh.point_capacity() as usize];
        let mut pos = Vec::new();
        let mut k = 0;
        for p in 0..mesh.point_capacity() {
            if mesh.point_valid(p) {
                remap[p as usize] = k;
                k += 1;
                let x = mesh.position(p);
                pos.extend([x.x, x.y, x.z]);
            }
        }
        let mut topo = vec![k];
        for fc in 0..mesh.face_capacity() {
            if mesh.face_valid(fc) {
                let pts = mesh.face_points(fc);
                topo.push(pts.len() as i64);
                topo.extend(pts.iter().map(|&p| remap[p as usize]));
            }
        }
        return Some(("mesh", topo, pos));
    }
    if let Some(c) = crate::curves::as_curve(g, id) {
        let cv = crate::curves::local_curve(g, c)?;
        let mut nums: Vec<f64> = cv.cvs.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        nums.extend(&cv.knots);
        return Some(("nurbsCurve", vec![cv.degree as i64, cv.form as i64, cv.cvs.len() as i64], nums));
    }
    if let Some(sh) = crate::dag::shape_of_type(g, id, "nurbsSurface") {
        let sf = crate::surfaces::local_surface(g, sh)?;
        let mut nums: Vec<f64> = sf.cvs.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        nums.extend(&sf.knots_u);
        nums.extend(&sf.knots_v);
        return Some(("nurbsSurface", vec![sf.degree_u as i64, sf.degree_v as i64, sf.form_u as i64, sf.form_v as i64, sf.nu as i64, sf.nv as i64, sf.trim.len() as i64], nums));
    }
    None
}

fn shape_compare(s: &mut Session, a: &Args) -> CmdResult {
    let mut names = a.named();
    if names.is_empty() {
        names = s.graph.selection().iter().map(|&id| s.graph.path(id)).collect();
    }
    if names.len() != 2 {
        fail!("shapeCompare: give or select two shapes");
    }
    let mut got = Vec::new();
    for n in &names {
        let Some(id) = s.graph.find(n)? else { fail!("shapeCompare: No object matches name: {n}") };
        let Some(x) = shape_numbers(&mut s.graph, id) else { fail!("shapeCompare: {n} is not a mesh, curve or surface") };
        got.push(x);
    }
    let (a0, b0) = (&got[0], &got[1]);
    let same = a0.0 == b0.0 && a0.1 == b0.1 && a0.2.len() == b0.2.len() && a0.2.iter().zip(&b0.2).all(|(x, y)| (x - y).abs() <= 1e-6 * x.abs().max(y.abs()).max(1.0));
    Ok(json!(if same { 0 } else { 1 }))
}

// ---------------------------------------------------------------------------------------------------------------------
// suitePrefs

static SUITE_APPLY: Mutex<String> = Mutex::new(String::new());

fn suite_prefs(_s: &mut Session, a: &Args) -> CmdResult {
    // A stand-alone install: not part of a suite, so no other application's settings apply.
    let q = if a.bool("query") { asked(a) } else { Vec::new() };
    if a.bool("installedAsSuite") || q.iter().any(|k| k == "installedAsSuite") {
        return Ok(json!(false));
    }
    if a.bool("isCompleteSuite") || q.iter().any(|k| k == "isCompleteSuite") {
        return Ok(json!(false));
    }
    if a.bool("query") {
        if q.iter().any(|k| k == "applyToSuite") {
            let v = SUITE_APPLY.lock().unwrap().clone();
            return Ok(json!(if v.is_empty() { "undefined".to_string() } else { v }));
        }
        fail!("suitePrefs: query -applyToSuite, -installedAsSuite or -isCompleteSuite");
    }
    if let Some(v) = a.get("applyToSuite") {
        let text = match v {
            Value::String(t) => t.clone(),
            Value::Number(n) if n.as_i64() == Some(1) => "Maya".to_string(),
            Value::Number(_) => "undefined".to_string(),
            Value::Bool(true) => "Maya".to_string(),
            _ => "undefined".to_string(),
        };
        let pick = ["Maya", "3dsMax", "undefined"].into_iter().find(|x| x.eq_ignore_ascii_case(&text));
        let Some(p) = pick else { fail!("suitePrefs: -applyToSuite takes \"Maya\", \"3dsMax\" or \"undefined\"") };
        *SUITE_APPLY.lock().unwrap() = p.to_string();
    }
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        q_flags(Cmd::new(
            "setNodeTypeFlag",
            "Static data on a node type, by name or type tag (objectType -tagFromType). -display false keeps nodes of the type out of the interface's editors. Returns 1.",
            set_node_type_flag,
        ))
        .positional("node type name or tag")
        .undoable(false)
        .flag(f("display", "dsp", B, "Whether the node type shows in the interface (on)."))
        .flag(f("threadSafe", "ts", B, "Obsolete: has no effect.")),
    );
    let mut lin = f("linear", "l", ArgType::Float, "The positional tolerance, in the current linear unit (0.01 cm).");
    lin.unit = Unit::Linear;
    let mut ang = f("angular", "a", ArgType::Float, "The tangential tolerance, in the current angular unit (0.1 degrees).");
    ang.unit = Unit::Angle;
    c.register(q_flags(Cmd::new("tolerance", "The tolerances modelling operations use when not given their own: -linear (positional) and -angular (tangential).", tolerance)).undoable(false).flag(lin).flag(ang));
    c.register(
        q_flags(Cmd::new("unknownPlugin", "The plug-ins the scene names that could not be loaded: -q -list, and of one plug-in -q -version, -nodeTypes, -dataTypes; -remove takes one away when none of its node types is in use.", unknown_plugin))
            .positional("plug-in name")
            .undoable(false)
            .flag(f("list", "l", B, "With -q: the unknown plug-ins."))
            .flag(f("version", "v", B, "With -q: the version the scene gave the plug-in."))
            .flag(f("nodeTypes", "nt", B, "With -q: the node types the plug-in gave."))
            .flag(f("dataTypes", "dt", B, "With -q: the data types the plug-in gave."))
            .flag(f("remove", "r", B, "Removes the plug-in from the scene.")),
    );
    c.register(
        Cmd::new("shapeCompare", "Compares two shapes (named, or the two selected): meshes by topology and points, curves and surfaces by degree, form, CVs and knots. Returns 0 when they are the same, 1 when not.", shape_compare)
            .positional("two shapes")
            .undoable(false),
    );
    c.register(
        q_flags(Cmd::new("suitePrefs", "A suite install's shared mouse and keyboard interaction. This is a stand-alone install: -installedAsSuite and -isCompleteSuite answer false.", suite_prefs))
            .undoable(false)
            .flag(f("applyToSuite", "ats", ArgType::Any, "Whose interaction settings every suite application uses: \"Maya\", \"3dsMax\" or \"undefined\" (each its own)."))
            .flag(f("installedAsSuite", "ias", B, "Whether this is part of a suite install."))
            .flag(f("isCompleteSuite", "ics", B, "Whether the suite install has every product of the suite.")),
    );
}
