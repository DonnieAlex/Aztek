//! Smaller rigging commands: rigid bind, geomBind, jointCluster, pairBlend, matchTransform, inheritTransform, container,
//! controller, the dimension and annotation shapes, dropoffLocator, findDeformers and copyDeformerWeights.
//!
//! Differs from the reference: bindSkin makes a skinCluster with one influence per point, the weights a rigid bind has, rather than
//! The reference's clusters per joint. jointCluster records its settings on a node and softens nothing; dropoffLocator marks the
//! place on a wire and doesn't change the wire's falloff. flexor, combinationShape, skeletonEmbed and weightsColor are in
//! rig_extra.

use std::any::Any;
use std::sync::Arc;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::graph::{Graph, SelectMode};
use crate::ref_core::{asked, f};
use crate::node::{types, vec3_attr, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::rig::{find, plug};
use crate::session::Session;
use crate::system_cmds::q_flags;
use crate::transform::{DagNode, Locator, Shape, Transform};

type Cmd = CommandDef<Session>;
use ArgType::{Bool as B, Float as F, Int as I, List as L, String as S};

const IN_STORE: u32 = flags::INPUT | flags::STORABLE;

fn attr(name: &str, short: &str, ty: PlugType, def: PlugValue, fl: u32, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: def, flags: fl, doc: doc.into(), ..Default::default() }
}
fn float_in(name: &str, short: &str, def: f64) -> AttrDef {
    attr(name, short, PlugType::Float, PlugValue::Float(def), IN_STORE | flags::KEYABLE, "A setting.")
}
fn float_out(name: &str, short: &str) -> AttrDef {
    attr(name, short, PlugType::Float, PlugValue::Float(0.0), flags::OUTPUT, "Worked out from the inputs.")
}
fn str_in(name: &str, short: &str) -> AttrDef {
    attr(name, short, PlugType::String, PlugValue::String(String::new()), IN_STORE, "A setting.")
}

// ---------------------------------------------------------------------------------------------------------------------
// Node types

/// distanceDimShape: the distance between its two points.
struct DistanceDim;

const DD_START: usize = Shape::COUNT;
const DD_END: usize = Shape::COUNT + 1;
const DD_DISTANCE: usize = Shape::COUNT + 2;

impl NodeBehavior for DistanceDim {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
        let v = (d.vec3(DD_END) - d.vec3(DD_START)).length();
        d.set_output(DD_DISTANCE, PlugValue::Float(v));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A shape that only keeps its matrices current.
struct PlainShape;

impl NodeBehavior for PlainShape {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

const AN_TARGET: usize = Shape::COUNT + 2;

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let shape = types::find("shape").expect("shape");
        let reg = |t: NodeType| {
            let name = t.name.clone();
            types::register_type(t).expect(&name);
        };
        reg(NodeType {
            name: "geomBind".into(),
            icon: "bone".into(),
            doc: "The settings a geodesic voxel bind computed skinCluster weights with.".into(),
            own_attrs: vec![
                attr("skinClusters", "skc", PlugType::Int, PlugValue::Int(0), flags::INPUT | flags::ARRAY | flags::MESSAGE | flags::HIDDEN, "The skinClusters bound."),
                attr("bindMethod", "bm", PlugType::Int, PlugValue::Int(3), IN_STORE, "3 geodesic voxel."),
                float_in("falloff", "fo", 0.2),
                attr("maxInfluences", "mi", PlugType::Int, PlugValue::Int(-1), IN_STORE, "Most influences on a point; -1 all of them."),
                attr("maxResolution", "mr", PlugType::Int, PlugValue::Int(256), IN_STORE, "The voxel grid's largest resolution, a power of two."),
                attr("validateVoxels", "vv", PlugType::Bool, PlugValue::Bool(true), IN_STORE, "Check the voxels' state after voxelising."),
            ],
            create: Some(Arc::new(|| Box::new(crate::node::Plain))),
            ..Default::default()
        });
        reg(NodeType {
            name: "distanceDimShape".into(),
            base: Some(shape.clone()),
            dag: true,
            icon: "ruler".into(),
            doc: "A measured distance between two points, drawn as a line.".into(),
            own_attrs: vec![
                vec3_attr("startPoint", "sp", DVec3::ZERO, IN_STORE, "One end, world space."),
                vec3_attr("endPoint", "ep", DVec3::ZERO, IN_STORE, "The other end, world space."),
                float_out("distance", "dist"),
            ],
            create: Some(Arc::new(|| Box::new(DistanceDim))),
            ..Default::default()
        });
        reg(NodeType {
            name: "paramDimension".into(),
            base: Some(shape.clone()),
            dag: true,
            icon: "ruler".into(),
            doc: "A point on a curve or surface, labelled with its parameter.".into(),
            own_attrs: vec![
                str_in("object", "obj"),
                float_in("uParamValue", "upv", 0.0),
                float_in("vParamValue", "vpv", 0.0),
            ],
            create: Some(Arc::new(|| Box::new(PlainShape))),
            ..Default::default()
        });
        reg(NodeType {
            name: "annotationShape".into(),
            base: Some(shape.clone()),
            dag: true,
            icon: "text".into(),
            doc: "A note in the viewport with a line to what it's about.".into(),
            own_attrs: vec![
                str_in("text", "txt"),
                attr("displayArrow", "daro", PlugType::Bool, PlugValue::Bool(true), IN_STORE, "Draw the line to the object."),
                vec3_attr("targetPosition", "tp", DVec3::ZERO, IN_STORE, "What the line points at, world space."),
                attr("dagObjectMatrix", "dom", PlugType::Matrix, PlugValue::Matrix(glam::DMat4::IDENTITY), flags::INPUT | flags::HIDDEN | flags::ARRAY, "The object the line points at: its world matrix."),
            ],
            create: Some(Arc::new(|| Box::new(PlainShape))),
            ..Default::default()
        });
        reg(NodeType {
            name: "dropoffLocator".into(),
            base: Some(shape.clone()),
            dag: true,
            icon: "locator".into(),
            doc: "A place along a wire with a dropoff percentage of its own.".into(),
            own_attrs: vec![float_in("param", "pr", 0.0), float_in("percent", "pc", 100.0), float_in("dropoff", "dp", 1.0), str_in("wire", "wr")],
            create: Some(Arc::new(|| Box::new(PlainShape))),
            ..Default::default()
        });
        for (name, doc, attrs) in [
            ("container", "Groups nodes so they read as one thing.", vec![attr("member", "mb", PlugType::String, PlugValue::String(String::new()), IN_STORE | flags::ARRAY, "Each node in it."), attr("isAsset", "isa", PlugType::Bool, PlugValue::Bool(false), IN_STORE, "Made with -asset."), attr("blackBox", "bbx", PlugType::Bool, PlugValue::Bool(false), IN_STORE, "Its members are hidden from selection: picking one picks the container."), attr("publishedNode", "pnod", PlugType::String, PlugValue::String(String::new()), IN_STORE | flags::ARRAY, "Members published: they can be picked through a black box.")]),
            ("controller", "Tags an object as an animation control, with its place among the controls.", {
                let link = |name: &str, short: &str, array: u32, doc: &str| attr(name, short, PlugType::Int, PlugValue::Int(0), flags::INPUT | flags::MESSAGE | flags::HIDDEN | array, doc);
                vec![
                    link("controllerObject", "act", 0, "The object this tags: its message."),
                    link("parent", "pare", 0, "Connected to the parent tag's children."),
                    link("children", "child", flags::ARRAY, "The tags under this one."),
                    attr("prepopulate", "prep", PlugType::Bool, PlugValue::Bool(true), IN_STORE, "Evaluate the controls below ahead of use."),
                    attr("parentprepopulate", "ppp", PlugType::Bool, PlugValue::Bool(true), IN_STORE, "Take prepopulate from the parent."),
                    attr("cycleWalkSibling", "cwsb", PlugType::Bool, PlugValue::Bool(false), IN_STORE, "Pick walking cycles among siblings."),
                    attr("visibilityMode", "vism", PlugType::Enum, PlugValue::Int(0), IN_STORE, "When the control shows."),
                ]
            }),
            (
                "jointCluster",
                "A joint's rigid bind settings. Kept for scripts: nothing softens here.",
                vec![
                    float_in("aboveBound", "ab", 10.0),
                    float_in("belowBound", "bb", 10.0),
                    float_in("aboveValue", "av", 50.0),
                    float_in("belowValue", "bv", 50.0),
                    str_in("aboveDropoffType", "adt"),
                    str_in("belowDropoffType", "bdt"),
                    str_in("joint", "jt"),
                ],
            ),
        ] {
            reg(NodeType { name: name.into(), doc: doc.into(), own_attrs: attrs, create: Some(Arc::new(|| Box::new(crate::node::Plain))), ..Default::default() });
        }
    });
}

/// Segments to draw for the shapes here, world space. The viewport calls this.
pub fn shape_lines(n: &crate::node::Node) -> Option<Vec<(DVec3, DVec3)>> {
    let t = n.ty();
    if t.name == "distanceDimShape" {
        let (a, b) = (n.data().vec3(DD_START), n.data().vec3(DD_END));
        let dir = (b - a).normalize_or_zero();
        let side = if dir.y.abs() > 0.9 { dir.cross(DVec3::X) } else { dir.cross(DVec3::Y) }.normalize_or_zero() * 0.1;
        return Some(vec![(a, b), (a - side, a + side), (b - side, b + side)]);
    }
    if t.name == "annotationShape" {
        let o = n.data().matrix(DagNode::WORLD_MATRIX).w_axis.truncate();
        let target = n.data().elements(Shape::COUNT + 3).first().map_or(n.data().vec3(AN_TARGET), |(_, m)| m.as_matrix().w_axis.truncate());
        return Some(if n.data().bool(Shape::COUNT + 1) { vec![(o, target)] } else { Vec::new() });
    }
    if t.name == "paramDimension" || t.name == "dropoffLocator" || t.name == "arcLengthDimension" {
        let o = n.data().matrix(DagNode::WORLD_MATRIX).w_axis.truncate();
        let k = 0.15;
        return Some(vec![(o - DVec3::X * k, o + DVec3::X * k), (o - DVec3::Y * k, o + DVec3::Y * k), (o - DVec3::Z * k, o + DVec3::Z * k)]);
    }
    None
}

/// The label a shape shows, with where it goes. The viewport calls this.
pub fn shape_label(n: &crate::node::Node) -> Option<(String, DVec3)> {
    if let Some(x) = crate::nurbs_tools::dimension_label(n) {
        return Some(x);
    }
    let t = n.ty();
    if t.name == "distanceDimShape" {
        return Some((format!("{:.3}", n.data().float(DD_DISTANCE)), (n.data().vec3(DD_START) + n.data().vec3(DD_END)) * 0.5));
    }
    if t.name == "annotationShape" {
        return Some((n.data().string(Shape::COUNT).to_string(), n.data().matrix(DagNode::WORLD_MATRIX).w_axis.truncate()));
    }
    if t.name == "paramDimension" {
        let d = n.data();
        return Some((format!("{:.3}, {:.3}", d.float(Shape::COUNT + 1), d.float(Shape::COUNT + 2)), d.matrix(DagNode::WORLD_MATRIX).w_axis.truncate()));
    }
    None
}

// ---------------------------------------------------------------------------------------------------------------------
// Binding

/// Locks or unlocks the translate, rotate and scale of the objects that aren't joints, as a rigid bind leaves them.
fn lock_trs(g: &mut Graph, names: &[String], lock: bool) -> CmdResult<()> {
    for n in names {
        let Ok(Some(id)) = g.find(n) else { continue };
        let tr = crate::dag::transform_of(g, id);
        if crate::rig::is_joint(g, tr) || !g.node(tr).is_some_and(|x| x.ty().is_a("transform")) {
            continue;
        }
        for attr in [Transform::TRANSLATE, Transform::ROTATE, Transform::SCALE] {
            g.set_locked(&plug(tr, attr), lock)?;
        }
    }
    Ok(())
}

fn bind_skin(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    if a.bool("unlock") {
        lock_trs(&mut s.graph, &names, false)?;
        return Ok(json!(1));
    }
    if a.bool("unbind") || a.bool("unbindKeepHistory") || a.bool("delete") {
        let mut call = json!({"args": names, "edit": true});
        call[if a.bool("unbindKeepHistory") { "unbindKeepHistory" } else { "unbind" }] = json!(true);
        s.run("skinCluster", call)?;
        lock_trs(&mut s.graph, &names, false)?;
        return Ok(json!(1));
    }
    if names.len() < 2 {
        fail!("bindSkin: name or select a skeleton and geometry");
    }
    let mut call = json!({"args": names, "maximumInfluences": 1, "toSelectedBones": a.bool("toSelectedBones") || a.bool("doNotDescend"), "bindMethod": 0, "dropoffRate": 10.0});
    if a.has("name") {
        call["name"] = json!(a.string("name", ""));
    }
    s.run("skinCluster", call)?;
    // Bound objects go wrong moved away from the skeleton, so the reference locks them.
    lock_trs(&mut s.graph, &names, true)?;
    Ok(json!(1))
}

/// The skinCluster a name stands for: itself, or the one skinning a mesh.
fn skin_named(g: &Graph, name: &str) -> CmdResult<NodeId> {
    let id = find(g, name, "geomBind")?;
    let sk = if g.node(id).is_some_and(|n| n.ty().is_a(crate::skin::SKIN_TYPE)) { Some(id) } else { crate::components::as_mesh(g, id).and_then(|m| crate::skin::skin_of(g, m)) };
    match sk {
        Some(sk) => Ok(sk),
        None => fail!("geomBind: {name} is not bound"),
    }
}

/// The geomBind node holding a skinCluster's bind settings, or the node named.
fn geom_bind_node(g: &Graph, name: &str) -> Option<NodeId> {
    let id = g.find(name).ok()??;
    if g.node(id).is_some_and(|n| n.ty().name == "geomBind") {
        return Some(id);
    }
    let sk = skin_named(g, name).ok()?;
    let msg = g.node(sk)?.ty().attr_index("message")? as i32;
    g.destinations(&PlugRef::new(sk, msg)).into_iter().map(|p| p.node).find(|&n| g.node(n).is_some_and(|x| x.ty().name == "geomBind"))
}

/// Weights computed again from a geomBind node's settings, for every skinCluster it holds.
fn geom_bind_apply(g: &mut Graph, node: NodeId) -> CmdResult<()> {
    let n = g.node(node).unwrap();
    let t = n.ty();
    let (fo, mi) = (n.float(t.attr_index("falloff").unwrap()), n.int(t.attr_index("maxInfluences").unwrap()));
    let skc = t.attr_index("skinClusters").unwrap() as i32;
    let skins: Vec<NodeId> = g.elements(&PlugRef::new(node, skc)).iter().filter_map(|e| g.source(e)).map(|p| p.node).collect();
    // The reference's geodesic voxel bind is the heat bind here: both follow the surface rather than straight lines.
    let max = if mi < 0 { usize::MAX } else { (mi as usize).max(1) };
    for sk in skins {
        crate::skin::rebind_weights(g, sk, 2, max, fo * 20.0)?;
    }
    Ok(())
}

fn geom_bind(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let Some(first) = names.first() else { fail!("geomBind: name a skinCluster") };
    let g = &mut s.graph;
    let gvp: Vec<Value> = match a.get("geodesicVoxelParams") {
        Some(Value::Array(v)) if !a.bool("query") => v.iter().flat_map(|x| match x {
            Value::Array(inner) => inner.clone(),
            o => vec![o.clone()],
        }).collect(),
        _ => Vec::new(),
    };
    if a.bool("query") {
        let Some(node) = geom_bind_node(g, first) else { fail!("geomBind: {first} has no geomBind settings") };
        let n = g.node(node).unwrap();
        let t = n.ty();
        let get = |k: &str| t.attr_index(k).unwrap();
        return Ok(if asked(a, "falloff") {
            json!(n.float(get("falloff")))
        } else if asked(a, "maxInfluences") {
            json!(n.int(get("maxInfluences")))
        } else if asked(a, "geodesicVoxelParams") {
            json!([n.int(get("maxResolution")), n.bool(get("validateVoxels"))])
        } else if asked(a, "bindMethod") {
            json!(n.int(get("bindMethod")))
        } else {
            fail!("geomBind: name a flag to query: -fo, -mi or -gvp")
        });
    }
    let node = if a.bool("edit") {
        match geom_bind_node(g, first) {
            Some(n) => n,
            None => fail!("geomBind: {first} has no geomBind settings to edit"),
        }
    } else {
        let method = a.int("bindMethod", 3);
        if method != 3 {
            fail!("geomBind: -bindMethod {method} is not a bind method: 3 (geodesic voxel) is the one there is");
        }
        let mut skins = Vec::new();
        for n in &names {
            skins.push(skin_named(g, n)?);
        }
        let t = types::find("geomBind").unwrap();
        let node = g.create_node(&t, "geomBind1", NO_NODE, true)?;
        let skc = t.attr_index("skinClusters").unwrap() as i32;
        for (k, sk) in skins.iter().enumerate() {
            let msg = g.node(*sk).unwrap().ty().attr_index("message").unwrap() as i32;
            g.connect(&PlugRef::new(*sk, msg), &PlugRef::element(node, skc, k as i32), false)?;
        }
        node
    };
    let t = g.node(node).unwrap().type_arc().clone();
    let at = |k: &str| PlugRef::new(node, t.attr_index(k).unwrap() as i32);
    if a.has("falloff") {
        g.set_value(at("falloff"), PlugValue::Float(a.float("falloff", 0.2).clamp(0.0, 1.0)))?;
    }
    if a.has("maxInfluences") {
        g.set_value(at("maxInfluences"), PlugValue::Int(a.int("maxInfluences", -1)))?;
    }
    if let Some(r) = gvp.first().and_then(Value::as_i64) {
        if r <= 0 || (r & (r - 1)) != 0 {
            fail!("geomBind: the voxel resolution {r} is not a power of two");
        }
        g.set_value(at("maxResolution"), PlugValue::Int(r))?;
    }
    if let Some(v) = gvp.get(1) {
        g.set_value(at("validateVoxels"), PlugValue::Bool(v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0)))?;
    }
    geom_bind_apply(g, node)?;
    Ok(Value::Null)
}

const JC_FLOATS: [&str; 4] = ["aboveBound", "belowBound", "aboveValue", "belowValue"];
const JC_DROPOFFS: [&str; 4] = ["linear", "exponential", "sine", "none"];

/// The jointCluster acting about a joint.
fn joint_cluster_of(g: &Graph, joint: NodeId) -> Option<NodeId> {
    let path = g.path(joint);
    let t = types::find("jointCluster")?;
    let at = t.attr_index("joint")?;
    g.nodes().into_iter().find(|&n| g.node(n).is_some_and(|x| x.ty().name == "jointCluster" && x.data().string(at) == path))
}

/// The jointCluster a name stands for: the node, or the joint it acts about.
fn joint_cluster_named(g: &Graph, name: &str) -> CmdResult<NodeId> {
    let id = find(g, name, "jointCluster")?;
    if g.node(id).is_some_and(|x| x.ty().name == "jointCluster") {
        return Ok(id);
    }
    match joint_cluster_of(g, id) {
        Some(n) => Ok(n),
        None => fail!("jointCluster: {name} has no jointCluster"),
    }
}

fn set_joint_cluster(g: &mut Graph, n: NodeId, a: &Args) -> CmdResult<()> {
    let t = g.node(n).unwrap().type_arc().clone();
    let at = |k: &str| PlugRef::new(n, t.attr_index(k).unwrap() as i32);
    for flag in JC_FLOATS {
        if a.has(flag) {
            g.set_value(at(flag), PlugValue::Float(a.float(flag, 0.0)))?;
        }
    }
    for flag in ["aboveDropoffType", "belowDropoffType"] {
        if let Some(v) = a.get(flag).and_then(Value::as_str) {
            if !JC_DROPOFFS.contains(&v) {
                fail!("jointCluster: -{flag} takes linear, exponential, sine or none");
            }
            g.set_value(at(flag), PlugValue::String(v.to_string()))?;
        }
    }
    Ok(())
}

fn joint_cluster(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") || a.bool("edit") {
        let names = objects(s, a);
        let Some(name) = names.first() else { fail!("jointCluster: name the jointCluster or its joint") };
        let g = &mut s.graph;
        let n = joint_cluster_named(g, name)?;
        if a.bool("edit") {
            g.open_chunk("jointCluster");
            let r = set_joint_cluster(g, n, a);
            g.close_chunk();
            r?;
            return Ok(json!(g.path(n)));
        }
        let t = g.node(n).unwrap().type_arc().clone();
        let joint = g.find(&g.node(n).unwrap().data().string(t.attr_index("joint").unwrap()).to_string())?;
        for flag in JC_FLOATS {
            if asked(a, flag) {
                return Ok(json!(g.node(n).unwrap().float(t.attr_index(flag).unwrap())));
            }
        }
        for flag in ["aboveDropoffType", "belowDropoffType"] {
            if asked(a, flag) {
                return Ok(json!(g.node(n).unwrap().data().string(t.attr_index(flag).unwrap())));
            }
        }
        // The cluster of the bone above is the parent joint's; the one of this joint is its own.
        if asked(a, "aboveCluster") {
            let parent = joint.map(|j| g.parent(j)).filter(|&p| p != NO_NODE && crate::rig::is_joint(g, p));
            return Ok(json!(parent.and_then(|p| joint_cluster_of(g, p)).map(|c| g.path(c)).unwrap_or_default()));
        }
        if asked(a, "belowCluster") {
            return Ok(json!(g.path(n)));
        }
        if asked(a, "deformerTools") {
            return Ok(json!(joint.map(|j| vec![g.path(j)]).unwrap_or_default()));
        }
        fail!("jointCluster: name a flag to query");
    }
    let joint = a.get("joint").and_then(Value::as_str).map(str::to_string).or_else(|| objects(s, a).into_iter().next());
    let Some(joint) = joint else { fail!("jointCluster: give -j, or select a joint") };
    let g = &mut s.graph;
    let j = find(g, &joint, "jointCluster")?;
    if !crate::rig::is_joint(g, j) {
        fail!("jointCluster: {joint} is not a joint");
    }
    let t = types::find("jointCluster").unwrap();
    g.open_chunk("jointCluster");
    let r = (|| -> CmdResult<NodeId> {
        let n = g.create_node(&t, &a.string("name", "jointCluster1"), NO_NODE, true)?;
        let at = |k: &str| PlugRef::new(n, t.attr_index(k).unwrap() as i32);
        for k in ["aboveDropoffType", "belowDropoffType"] {
            g.set_value(at(k), PlugValue::String("linear".into()))?;
        }
        set_joint_cluster(g, n, a)?;
        let path = g.path(j);
        g.set_value(at("joint"), PlugValue::String(path))?;
        Ok(n)
    })();
    g.close_chunk();
    let n = r?;
    Ok(json!(g.path(n)))
}

// ---------------------------------------------------------------------------------------------------------------------
// pairBlend

/// Which pair of pairBlend inputs an attribute goes through, and which of its parts.
fn blend_slot(g: &Graph, p: &PlugRef) -> Option<(&'static str, &'static str, &'static str)> {
    match g.attr(p).name.as_str() {
        "translate" => Some(("inTranslate1", "inTranslate2", "outTranslate")),
        "rotate" => Some(("inRotate1", "inRotate2", "outRotate")),
        _ => None,
    }
}

fn pair_blend(s: &mut Session, a: &Args) -> CmdResult {
    let attrs: Vec<String> = match a.get("attribute") {
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Some(Value::String(t)) => vec![t.clone()],
        _ => Vec::new(),
    };
    let g = &mut s.graph;
    if a.bool("query") {
        let Some(pb) = objects_of(a).first().and_then(|n| g.find(n).ok().flatten()) else { fail!("pairBlend: name a pairBlend") };
        let t = g.node(pb).unwrap().type_arc().clone();
        let list = |g: &Graph, names: &[&str]| -> Vec<Value> {
            let mut out = Vec::new();
            for n in names {
                let at = t.attr_index(n).unwrap() as i32;
                for (d, sr) in g.connections() {
                    if d.node == pb && d.attr == at {
                        out.push(json!(g.plug_name(&sr)));
                    }
                }
            }
            out
        };
        if asked(a, "input1") {
            return Ok(Value::Array(list(g, &["inTranslate1", "inRotate1"])));
        }
        if asked(a, "input2") {
            return Ok(Value::Array(list(g, &["inTranslate2", "inRotate2"])));
        }
        if asked(a, "attribute") {
            let mut out = Vec::new();
            for n in ["outTranslate", "outRotate"] {
                let at = t.attr_index(n).unwrap() as i32;
                for (d, sr) in g.connections() {
                    if sr.node == pb && sr.attr == at {
                        out.push(json!(g.plug_name(&d)));
                    }
                }
            }
            return Ok(Value::Array(out));
        }
        fail!("pairBlend: query what?");
    }
    let Some(node_name) = a.get("node").and_then(Value::as_str) else { fail!("pairBlend: give -node") };
    if attrs.is_empty() {
        fail!("pairBlend: give -attribute");
    }
    let t = types::find("pairBlend").expect("pairBlend");
    g.open_chunk("pairBlend");
    let r = (|| -> CmdResult<NodeId> {
        let pb = g.create_node(&t, &a.string("name", "pairBlend1"), NO_NODE, true)?;
        for at in &attrs {
            let dst = g.plug(&format!("{node_name}.{at}"))?;
            let Some((in1, _, out)) = blend_slot(g, &dst) else { fail!("pairBlend: {at} isn't a translate or rotate") };
            let part = |attr: &str, comp: i32| PlugRef { node: pb, attr: t.attr_index(attr).unwrap() as i32, component: comp, element: -1 };
            // What drives the attribute now feeds the first input, or its value does.
            match g.source(&dst) {
                Some(src) => {
                    g.connect(&src, &part(in1, dst.component), true)?;
                }
                None => {
                    let v = g.value(&dst)?;
                    g.set_value(part(in1, dst.component), v)?;
                }
            }
            g.connect(&part(out, dst.component), &dst, true)?;
        }
        Ok(pb)
    })();
    g.close_chunk();
    let pb = r?;
    g.evaluate();
    Ok(json!(g.path(pb)))
}

fn objects_of(a: &Args) -> Vec<String> {
    a.named()
}

// ---------------------------------------------------------------------------------------------------------------------
// matchTransform and inheritTransform

fn xq(s: &mut Session, name: &str, flag: &str) -> CmdResult<DVec3> {
    let v = s.run("xform", json!({"args": [name], "query": true, "worldSpace": true, flag: true}))?;
    let list: Vec<f64> = v.as_array().map(|x| x.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
    if list.len() < 3 {
        fail!("matchTransform: can't read {flag} of {name}");
    }
    Ok(DVec3::new(list[0], list[1], list[2]))
}

/// The size of an object's bounding box with its children, in world space.
fn box_size(s: &mut Session, name: &str) -> CmdResult<DVec3> {
    let v = s.run("exactWorldBoundingBox", json!({"args": [name]}))?;
    let b: Vec<f64> = v.as_array().map(|x| x.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
    if b.len() < 6 {
        fail!("matchTransform: {name} has no bounding box");
    }
    Ok(DVec3::new(b[3] - b[0], b[4] - b[1], b[5] - b[2]))
}

/// Which of X, Y and Z a family of flags asks for: the whole flag (-pos) all three, else the axis flags.
fn axes(a: &Args, whole: &str, each: [&str; 3]) -> [bool; 3] {
    let all = a.bool(whole);
    [all || a.bool(each[0]), all || a.bool(each[1]), all || a.bool(each[2])]
}

/// The components of `want` on the axes asked for, the rest from `have`.
fn pick(have: DVec3, want: DVec3, on: [bool; 3]) -> DVec3 {
    DVec3::new(if on[0] { want.x } else { have.x }, if on[1] { want.y } else { have.y }, if on[2] { want.z } else { have.z })
}

fn match_transform(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    if names.len() < 2 {
        fail!("matchTransform: name or select the objects, then the target");
    }
    let target = names.last().unwrap().clone();
    let mut pos = axes(a, "position", ["positionX", "positionY", "positionZ"]);
    let mut rot = axes(a, "rotation", ["rotationX", "rotationY", "rotationZ"]);
    let mut scl = axes(a, "scale", ["scaleX", "scaleY", "scaleZ"]);
    let (piv, rp, sp) = (a.bool("pivots"), a.bool("rotatePivot") || a.bool("pivots"), a.bool("scalePivot") || a.bool("pivots"));
    let any = |v: [bool; 3]| v.iter().any(|&x| x);
    // No flags: position, rotation and scale.
    if !(any(pos) || any(rot) || any(scl) || piv || rp || sp) {
        pos = [true; 3];
        rot = [true; 3];
        scl = [true; 3];
    }
    s.graph.open_chunk("Match Transform");
    let r = (|| -> CmdResult<()> {
        for name in &names[..names.len() - 1] {
            if any(rot) {
                let (want, have) = (xq(s, &target, "rotation")?, xq(s, name, "rotation")?);
                let ro = pick(have, want, rot);
                s.run("xform", json!({"args": [name], "worldSpace": true, "rotation": [ro.x, ro.y, ro.z]}))?;
            }
            if any(scl) {
                let have = xq(s, name, "scale")?;
                // -scaleBox: the source's scale times how much bigger the target's box is than its own.
                let want = if a.bool("scaleBox") {
                    let (tb, sb) = (box_size(s, &target)?, box_size(s, name)?);
                    let ratio = |t: f64, o: f64| if o.abs() > 1e-12 { t / o } else { 1.0 };
                    DVec3::new(have.x * ratio(tb.x, sb.x), have.y * ratio(tb.y, sb.y), have.z * ratio(tb.z, sb.z))
                } else {
                    xq(s, &target, "scale")?
                };
                let sc = pick(have, want, scl);
                s.run("xform", json!({"args": [name], "worldSpace": true, "scale": [sc.x, sc.y, sc.z]}))?;
            }
            if rp {
                let p = xq(s, &target, "rotatePivot")?;
                s.run("xform", json!({"args": [name], "worldSpace": true, "rotatePivot": [p.x, p.y, p.z]}))?;
            }
            if sp {
                let p = xq(s, &target, "scalePivot")?;
                s.run("xform", json!({"args": [name], "worldSpace": true, "scalePivot": [p.x, p.y, p.z]}))?;
            }
            if any(pos) {
                // The pivots line up, as Match Translation does, on the axes asked for.
                let (want, have) = (xq(s, &target, "rotatePivot")?, xq(s, name, "rotatePivot")?);
                let d = pick(DVec3::ZERO, want - have, pos);
                s.run("xform", json!({"args": [name], "worldSpace": true, "relative": true, "translation": [d.x, d.y, d.z]}))?;
            }
        }
        Ok(())
    })();
    s.graph.close_chunk();
    r?;
    s.graph.evaluate();
    Ok(Value::Null)
}

fn inherit_transform(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    let mut ids = Vec::new();
    for n in &names {
        let id = find(g, n, "inheritTransform")?;
        if !g.node(id).is_some_and(|x| x.ty().is_a("transform")) {
            fail!("inheritTransform: {n} isn't a transform");
        }
        ids.push(id);
    }
    if a.bool("query") {
        return Ok(Value::Array(ids.iter().map(|&i| json!(g.node(i).unwrap().bool(Transform::INHERITS_TRANSFORM))).collect()));
    }
    g.open_chunk("inheritTransform");
    let r = (|| -> CmdResult<()> {
        for &id in &ids {
            g.evaluate();
            let was = g.node(id).unwrap().bool(Transform::INHERITS_TRANSFORM);
            let world = g.node(id).unwrap().matrix(Transform::WORLD_MATRIX);
            let on = match a.get("toggle") {
                Some(v) if !v.is_boolean() || !v.as_bool().unwrap_or(false) => v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0) ^ true,
                _ => !was,
            };
            let on = if a.bool("off") {
                false
            } else if a.has("on") {
                a.bool("on")
            } else {
                on
            };
            g.set_value(plug(id, Transform::INHERITS_TRANSFORM), PlugValue::Bool(on))?;
            if a.bool("preserve") {
                g.evaluate();
                let parent = g.node(id).unwrap().matrix(Transform::PARENT_MATRIX);
                let local = if on { parent.inverse() * world } else { world };
                let (sc, q, t) = local.to_scale_rotation_translation();
                let euler = q.to_euler(glam::EulerRot::ZYX);
                g.set_value(plug(id, Transform::TRANSLATE), PlugValue::Vec3(t))?;
                g.set_value(plug(id, Transform::ROTATE), PlugValue::Vec3(DVec3::new(euler.2, euler.1, euler.0)))?;
                g.set_value(plug(id, Transform::SCALE), PlugValue::Vec3(sc))?;
            }
        }
        Ok(())
    })();
    g.close_chunk();
    r?;
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// Containers and controllers: references to nodes held as "uuid|name"

fn node_ref(g: &Graph, id: NodeId) -> String {
    let n = g.node(id).unwrap();
    format!("{}|{}", n.uuid(), g.path(id))
}

fn resolve_ref(g: &Graph, text: &str) -> Option<NodeId> {
    let (uuid, name) = text.split_once('|').unwrap_or(("", text));
    if !uuid.is_empty() {
        if let Some(id) = g.nodes().into_iter().find(|&i| g.node(i).is_some_and(|n| n.uuid() == uuid)) {
            return Some(id);
        }
    }
    g.find(name).ok().flatten()
}

const CT_MEMBER: usize = 0;

fn is_type(g: &Graph, id: NodeId, ty: &str) -> bool {
    g.node(id).is_some_and(|n| n.ty().name == ty)
}

/// What select -containerCentric picks for a node: the black-boxed container holding it, unless the node is published,
/// or the node itself.
pub(crate) fn container_centric(g: &Graph, id: NodeId) -> NodeId {
    let mut out = id;
    for c in g.nodes() {
        if !is_type(g, c, "container") {
            continue;
        }
        let n = g.node(c).unwrap();
        let black = n.ty().attr_index("blackBox").is_some_and(|i| n.data().bool(i));
        if !black || !container_members(g, c).iter().any(|(_, m)| *m == id) {
            continue;
        }
        let published = n.ty().attr_index("publishedNode").is_some_and(|i| {
            n.elements(i).iter().any(|(_, v)| matches!(v, PlugValue::String(t) if resolve_ref(g, t) == Some(id)))
        });
        if !published {
            out = c;
        }
    }
    out
}

fn container_members(g: &Graph, c: NodeId) -> Vec<(i32, NodeId)> {
    let n = g.node(c).unwrap();
    n.elements(CT_MEMBER).iter().filter_map(|(i, v)| if let PlugValue::String(t) = v { resolve_ref(g, t).map(|x| (*i, x)) } else { None }).collect()
}

fn str_list(a: &Args, flag: &str) -> Vec<String> {
    match a.get(flag) {
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Some(Value::String(t)) => vec![t.clone()],
        _ => Vec::new(),
    }
}

fn container_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let g = &mut s.graph;
    let target = a.named().first().and_then(|n| g.find(n).ok().flatten());
    if a.bool("query") {
        if asked(a, "findContainer") || a.has("findContainer") {
            let names = if a.has("findContainer") { str_list(a, "findContainer") } else { a.named() };
            let Some(n) = names.first() else { fail!("container: name a node") };
            let id = find(g, n, "container")?;
            for c in g.nodes() {
                if is_type(g, c, "container") && container_members(g, c).iter().any(|(_, m)| *m == id) {
                    return Ok(json!(g.path(c)));
                }
            }
            return Ok(json!(""));
        }
        let Some(c) = target.filter(|&x| is_type(g, x, "container")) else { fail!("container: name a container") };
        if asked(a, "nodeList") {
            return Ok(Value::Array(container_members(g, c).iter().map(|(_, m)| json!(g.path(*m))).collect()));
        }
        if asked(a, "isContainer") {
            return Ok(json!(true));
        }
        fail!("container: query what?");
    }
    let add: Vec<String> = str_list(a, "addNode").into_iter().chain(str_list(a, "add")).collect();
    let rm: Vec<String> = str_list(a, "removeNode").into_iter().chain(str_list(a, "rm")).collect();
    g.open_chunk("container");
    let r = (|| -> CmdResult<NodeId> {
        let c = if a.bool("edit") {
            let Some(c) = target.filter(|&x| is_type(g, x, "container")) else { fail!("container: name a container") };
            c
        } else {
            let t = types::find("container").unwrap();
            let c = g.create_node(&t, &a.string("name", "container1"), NO_NODE, true)?;
            if a.has("asset") {
                g.set_value(plug(c, 1), PlugValue::Bool(true))?;
            }
            c
        };
        let mut next = g.node(c).unwrap().elements(CT_MEMBER).iter().map(|e| e.0 + 1).max().unwrap_or(0);
        for name in &add {
            let id = find(g, name, "container")?;
            if container_members(g, c).iter().any(|(_, m)| *m == id) {
                continue;
            }
            let text = node_ref(g, id);
            g.set_value(PlugRef::element(c, CT_MEMBER as i32, next), PlugValue::String(text))?;
            next += 1;
        }
        for name in &rm {
            let id = find(g, name, "container")?;
            for (i, m) in container_members(g, c) {
                if m == id {
                    g.remove_element(&PlugRef::element(c, CT_MEMBER as i32, i))?;
                }
            }
        }
        Ok(c)
    })();
    g.close_chunk();
    let c = r?;
    Ok(json!(g.path(c)))
}

const CTL_OBJECT: usize = 0;
const CTL_PARENT: usize = 1;
const CTL_CHILDREN: usize = 2;

fn message_of(g: &Graph, id: NodeId) -> Option<PlugRef> {
    let a = g.node(id)?.ty().attr_index("message")?;
    Some(plug(id, a))
}

fn tag_of(g: &Graph, obj: NodeId) -> Option<NodeId> {
    g.destinations(&message_of(g, obj)?).into_iter().find(|d| d.attr as usize == CTL_OBJECT && is_type(g, d.node, "controller")).map(|d| d.node)
}

fn object_of_tag(g: &Graph, tag: NodeId) -> Option<NodeId> {
    g.source(&plug(tag, CTL_OBJECT)).map(|s| s.node)
}

/// As the reference wires it: a tag's `parent` goes into its parent's `children`.
fn parent_tag(g: &Graph, tag: NodeId) -> Option<NodeId> {
    g.destinations(&plug(tag, CTL_PARENT)).into_iter().find(|d| d.attr as usize == CTL_CHILDREN).map(|d| d.node)
}

fn set_parent_tag(g: &mut Graph, tag: NodeId, parent: Option<NodeId>) -> CmdResult<()> {
    for d in g.destinations(&plug(tag, CTL_PARENT)) {
        g.disconnect(&d);
    }
    if let Some(p) = parent {
        g.connect(&plug(tag, CTL_PARENT), &plug(p, CTL_CHILDREN), true)?;
    }
    Ok(())
}

/// A tag's children in their order: the elements of its children multi.
fn child_tags(g: &Graph, tag: NodeId) -> Vec<NodeId> {
    let mut els = g.elements(&plug(tag, CTL_CHILDREN));
    els.sort_by_key(|e| e.element);
    els.iter().filter_map(|e| g.source(e)).filter(|s| s.attr as usize == CTL_PARENT).map(|s| s.node).collect()
}

/// Puts a tag at `index` among its parent's children, the others keeping their order.
fn reorder_tag(g: &mut Graph, tag: NodeId, index: i64) -> CmdResult<()> {
    let Some(p) = parent_tag(g, tag) else { fail!("controller: {} has no parent to be ordered under", g.path(tag)) };
    let mut kids = child_tags(g, p);
    kids.retain(|&k| k != tag);
    kids.insert((index.max(0) as usize).min(kids.len()), tag);
    for &k in &kids {
        for d in g.destinations(&plug(k, CTL_PARENT)) {
            g.disconnect(&d);
        }
    }
    for (i, &k) in kids.iter().enumerate() {
        g.connect(&plug(k, CTL_PARENT), &PlugRef::element(p, CTL_CHILDREN as i32, i as i32), true)?;
    }
    Ok(())
}

fn controller_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    if asked(a, "allControllers") {
        return Ok(Value::Array(g.nodes().into_iter().filter(|&c| is_type(g, c, "controller")).filter_map(|c| object_of_tag(g, c)).map(|o| json!(g.path(o))).collect()));
    }
    let tag_named = |g: &Graph, n: &str| -> CmdResult<Option<NodeId>> {
        let id = find(g, n, "controller")?;
        Ok(if is_type(g, id, "controller") { Some(id) } else { tag_of(g, id) })
    };
    if a.bool("query") || asked(a, "isController") {
        let Some(n) = names.first() else { fail!("controller: name an object") };
        let tag = tag_named(g, n)?;
        if asked(a, "isController") {
            return Ok(json!(tag.is_some()));
        }
        let walks = ["pickWalkUp", "pickWalkDown", "pickWalkLeft", "pickWalkRight", "index"];
        let plain = !asked(a, "parent") && !asked(a, "children") && !walks.iter().any(|w| asked(a, w));
        // A plain query names the object's controller node, as the reference's does; nothing for an object without one.
        if plain {
            return Ok(match tag {
                Some(t) => json!([g.path(t)]),
                None => Value::Null,
            });
        }
        let Some(tag) = tag else { fail!("controller: {n} is not a controller") };
        if asked(a, "parent") {
            return Ok(match parent_tag(g, tag).and_then(|p| object_of_tag(g, p)) {
                Some(o) => json!(g.path(o)),
                None => Value::Null,
            });
        }
        // A tag with no object (made with -group) answers by its own name.
        let named = |g: &Graph, t: NodeId| json!(g.path(object_of_tag(g, t).unwrap_or(t)));
        if asked(a, "children") {
            return Ok(Value::Array(child_tags(g, tag).into_iter().map(|c| named(g, c)).collect()));
        }
        let parent = parent_tag(g, tag);
        let siblings = parent.map(|p| child_tags(g, p)).unwrap_or_default();
        let at = siblings.iter().position(|&x| x == tag);
        if asked(a, "index") {
            return Ok(json!(at.map_or(-1, |i| i as i64)));
        }
        if asked(a, "pickWalkUp") {
            return Ok(parent.map_or(Value::Null, |p| named(g, p)));
        }
        if asked(a, "pickWalkDown") {
            return Ok(child_tags(g, tag).first().map_or(Value::Null, |&c| named(g, c)));
        }
        // Left and right walk the siblings, round from the ends when the parent's cycleWalkSibling is on.
        let cycle = parent.and_then(|p| g.node(p).and_then(|n| n.ty().attr_index("cycleWalkSibling").map(|i| n.bool(i)))).unwrap_or(false);
        let step = |d: i64| -> Value {
            let Some(i) = at else { return Value::Null };
            let n = siblings.len() as i64;
            let j = i as i64 + d;
            let j = if cycle { j.rem_euclid(n) } else { j };
            if j < 0 || j >= n || j == i as i64 {
                return Value::Null;
            }
            named(g, siblings[j as usize])
        };
        if asked(a, "pickWalkLeft") {
            return Ok(step(-1));
        }
        if asked(a, "pickWalkRight") {
            return Ok(step(1));
        }
    }
    if a.bool("edit") {
        if let Some(i) = a.get("index").and_then(Value::as_i64) {
            for n in &names {
                if let Some(tag) = tag_named(g, n)? {
                    reorder_tag(g, tag, i)?;
                }
            }
            return Ok(Value::Null);
        }
    }
    if a.bool("group") {
        // A tag of no object, the parent of those named or selected.
        let t = types::find("controller").unwrap();
        g.open_chunk("controller");
        let r = (|| -> CmdResult {
            let group = g.create_node(&t, "controller1", NO_NODE, true)?;
            for n in &names {
                let id = find(g, n, "controller")?;
                let tag = match tag_of(g, id) {
                    Some(x) => x,
                    None => {
                        let short = g.node(id).unwrap().name().to_string();
                        let x = g.create_node(&t, &format!("{short}_tag"), NO_NODE, true)?;
                        let Some(msg) = message_of(g, id) else { fail!("controller: {n} has no message") };
                        g.connect(&msg, &plug(x, CTL_OBJECT), true)?;
                        x
                    }
                };
                set_parent_tag(g, tag, Some(group))?;
            }
            Ok(json!(g.path(group)))
        })();
        g.close_chunk();
        return r;
    }
    if a.bool("unparent") {
        for n in &names {
            if let Some(tag) = tag_named(g, n)? {
                set_parent_tag(g, tag, None)?;
            }
        }
        return Ok(Value::Null);
    }
    let t = types::find("controller").unwrap();
    let parent_name = a.get("parent").and_then(Value::as_str).map(str::to_string);
    g.open_chunk("controller");
    let r = (|| -> CmdResult<Vec<Value>> {
        let mut made = Vec::new();
        let mut tagged = |g: &mut Graph, n: &str| -> CmdResult<NodeId> {
            let id = find(g, n, "controller")?;
            if let Some(tag) = tag_of(g, id) {
                return Ok(tag);
            }
            let short = g.node(id).unwrap().name().to_string();
            let tag = g.create_node(&t, &format!("{short}_tag"), NO_NODE, true)?;
            let Some(msg) = message_of(g, id) else { fail!("controller: {n} has no message") };
            g.connect(&msg, &plug(tag, CTL_OBJECT), true)?;
            made.push(json!(g.path(tag)));
            Ok(tag)
        };
        let parent = match &parent_name {
            Some(p) => Some(tagged(g, p)?),
            None => None,
        };
        for n in &names {
            let tag = tagged(g, n)?;
            if let Some(p) = parent {
                set_parent_tag(g, tag, Some(p))?;
            }
        }
        Ok(made)
    })();
    g.close_chunk();
    Ok(Value::Array(r?))
}

// ---------------------------------------------------------------------------------------------------------------------
// Dimensions

fn vec_flag(a: &Args, flag: &str) -> Option<DVec3> {
    a.vec3(flag).map(DVec3::from_array)
}

/// A locator exactly there, or a new one.
fn locator_at(g: &mut Graph, at: DVec3) -> CmdResult<NodeId> {
    g.evaluate();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if n.ty().is_a("locator") && n.data().vec3(Locator::WORLD_POSITION).abs_diff_eq(at, 1e-9) {
            return Ok(id);
        }
    }
    let (tr, sh) = crate::dag::create_with_shape(g, "locator", "locator1", NO_NODE)?;
    g.set_value(plug(tr, Transform::TRANSLATE), PlugValue::Vec3(at))?;
    Ok(sh)
}

fn distance_dimension(s: &mut Session, a: &Args) -> CmdResult {
    let (Some(sp), Some(ep)) = (vec_flag(a, "startPoint"), vec_flag(a, "endPoint")) else { fail!("distanceDimension: give -sp and -ep") };
    let g = &mut s.graph;
    g.open_chunk("Distance Tool");
    let r = (|| -> CmdResult<NodeId> {
        let (l0, l1) = (locator_at(g, sp)?, locator_at(g, ep)?);
        let (tr, sh) = crate::dag::create_with_shape(g, "distanceDimShape", "distanceDimension1", NO_NODE)?;
        let _ = tr;
        g.connect(&plug(l0, Locator::WORLD_POSITION), &plug(sh, DD_START), false)?;
        g.connect(&plug(l1, Locator::WORLD_POSITION), &plug(sh, DD_END), false)?;
        Ok(sh)
    })();
    g.close_chunk();
    let sh = r?;
    g.evaluate();
    g.select(&[crate::dag::transform_of(g, sh)], SelectMode::Replace);
    Ok(json!(g.path(sh)))
}

/// curve1.u[0.5], curve1.ep[2], surface1.uv[0.5][1], surface1.u[1]: the object's shape and parameters.
fn param_of(g: &mut Graph, text: &str) -> Option<(NodeId, f64, f64)> {
    let (head, rest) = text.split_once('[')?;
    let (obj, kind) = head.rsplit_once('.')?;
    let id = g.find(obj).ok()??;
    let inner = rest.trim_end_matches(']');
    let (a, b) = inner.split_once("][").unwrap_or((inner, "0"));
    let (x, y): (f64, f64) = (a.parse().ok()?, b.parse().ok()?);
    if let Some(c) = crate::curves::as_curve(g, id) {
        let u = if kind == "ep" { crate::curves::local_curve(g, c)?.distinct_params().get(x as usize).copied()? } else { x };
        return Some((c, u, 0.0));
    }
    let s = crate::surfaces::as_surface(g, id)?;
    Some((s, x, y))
}

fn point_on(g: &mut Graph, shape: NodeId, u: f64, v: f64) -> Option<DVec3> {
    if g.node(shape)?.ty().is_a("nurbsCurve") {
        return crate::curves::world_curve(g, shape).map(|c| c.eval(u));
    }
    crate::surfaces::world_surface(g, shape).map(|s| s.eval(u, v))
}

fn param_dimension(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    let Some(text) = names.first() else { fail!("paramDimension: name a point: curve1.u[0.5] or surface1.uv[0.5][0.5]") };
    let Some((shape, u, v)) = param_of(g, text) else { fail!("paramDimension: {text} isn't a point on a curve or surface") };
    let Some(at) = point_on(g, shape, u, v) else { fail!("paramDimension: {text} has no geometry") };
    g.open_chunk("paramDimension");
    let r = (|| -> CmdResult<NodeId> {
        let (tr, sh) = crate::dag::create_with_shape(g, "paramDimension", "paramDimension1", NO_NODE)?;
        g.set_value(plug(tr, Transform::TRANSLATE), PlugValue::Vec3(at))?;
        g.set_value(plug(sh, Shape::COUNT), PlugValue::String(text.clone()))?;
        g.set_value(plug(sh, Shape::COUNT + 1), PlugValue::Float(u))?;
        g.set_value(plug(sh, Shape::COUNT + 2), PlugValue::Float(v))?;
        Ok(sh)
    })();
    g.close_chunk();
    Ok(json!(g.path(r?)))
}

fn annotate(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    let at = vec_flag(a, "point");
    let text = a.string("text", "");
    let target = names.first().map(|n| find(g, n, "annotate")).transpose()?;
    g.evaluate();
    let target_pos = target.and_then(|t| g.node(t)).filter(|n| n.ty().dag).map(|n| n.matrix(DagNode::WORLD_MATRIX).w_axis.truncate());
    let at = at.or(target_pos).unwrap_or(DVec3::ZERO);
    g.open_chunk("annotate");
    let r = (|| -> CmdResult<NodeId> {
        let (tr, sh) = crate::dag::create_with_shape(g, "annotationShape", "annotation1", NO_NODE)?;
        g.set_value(plug(tr, Transform::TRANSLATE), PlugValue::Vec3(at))?;
        g.set_value(plug(sh, Shape::COUNT), PlugValue::String(text))?;
        match target {
            Some(t) if g.node(t).is_some_and(|n| n.ty().is_a("locator")) => {
                g.connect(&plug(t, Locator::WORLD_POSITION), &plug(sh, AN_TARGET), false)?;
            }
            _ => {
                g.set_value(plug(sh, AN_TARGET), PlugValue::Vec3(target_pos.unwrap_or(at)))?;
                if target.is_none() {
                    g.set_value(plug(sh, Shape::COUNT + 1), PlugValue::Bool(false))?;
                }
            }
        }
        Ok(sh)
    })();
    g.close_chunk();
    Ok(json!(g.path(r?)))
}

fn dropoff_locator(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let nums: Vec<f64> = a.positional.iter().filter_map(Value::as_f64).collect();
    let [dropoff, percent] = nums.as_slice() else { fail!("dropoffLocator: dropoff percent curve.u[...]") };
    let g = &mut s.graph;
    let Some(text) = names.first() else { fail!("dropoffLocator: name a wire curve's point") };
    let Some((shape, u, v)) = param_of(g, text) else { fail!("dropoffLocator: {text} isn't a point on a curve") };
    let Some(at) = point_on(g, shape, u, v) else { fail!("dropoffLocator: {text} has no geometry") };
    g.open_chunk("dropoffLocator");
    let r = (|| -> CmdResult<NodeId> {
        let (tr, sh) = crate::dag::create_with_shape(g, "dropoffLocator", "dropoffLocator1", NO_NODE)?;
        g.set_value(plug(tr, Transform::TRANSLATE), PlugValue::Vec3(at))?;
        g.set_value(plug(sh, Shape::COUNT), PlugValue::Float(u))?;
        g.set_value(plug(sh, Shape::COUNT + 1), PlugValue::Float(*percent))?;
        g.set_value(plug(sh, Shape::COUNT + 2), PlugValue::Float(*dropoff))?;
        g.set_value(plug(sh, Shape::COUNT + 3), PlugValue::String(text.clone()))?;
        Ok(tr)
    })();
    g.close_chunk();
    Ok(json!([g.path(r?)]))
}

// ---------------------------------------------------------------------------------------------------------------------
// Asking about and copying deformers

fn find_deformers(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    let mut out = Vec::new();
    for n in names {
        let id = find(g, &n, "findDeformers")?;
        let Some(m) = crate::components::as_mesh(g, id) else { continue };
        for h in crate::history::history_of(g, m) {
            let ty = g.node(h).map(|x| x.ty().name.clone()).unwrap_or_default();
            if crate::deform::is_deformer(&ty) || ty == crate::skin::SKIN_TYPE {
                out.push(json!(g.path(h)));
            }
        }
    }
    Ok(Value::Array(out))
}

fn copy_deformer_weights(s: &mut Session, a: &Args) -> CmdResult {
    let (ss, ds) = (a.string("sourceShape", ""), a.string("destinationShape", ""));
    let (sd, dd) = (a.string("sourceDeformer", ""), a.string("destinationDeformer", ""));
    if a.bool("query") {
        let g = &s.graph;
        let asked = |k: &str| crate::skin::asked(a, k);
        // The deformers on a shape, nearest the shape first.
        let on = |shape: &str| -> Vec<String> {
            let Some(m) = g.find(shape).ok().flatten().and_then(|id| crate::components::as_mesh(g, id)) else { return Vec::new() };
            crate::history::history_of(g, m).into_iter().filter(|&h| g.node(h).is_some_and(|n| crate::deform::is_deformer(n.ty().name.as_str()))).map(|h| g.path(h)).collect()
        };
        if asked("sourceDeformer") {
            return Ok(json!(if ss.is_empty() { vec![sd] } else { on(&ss) }));
        }
        if asked("destinationDeformer") {
            return Ok(json!(if ds.is_empty() { vec![dd] } else { on(&ds) }));
        }
        if asked("sourceShape") || asked("destinationShape") {
            let d = if asked("sourceShape") { &sd } else { &dd };
            let Some(n) = g.find(d).ok().flatten() else { return Ok(json!([])) };
            return Ok(json!(crate::deform::geometry_names(g, n)));
        }
        if asked("mirrorMode") {
            return Ok(json!(a.string("mirrorMode", "XY")));
        }
        if asked("surfaceAssociation") {
            return Ok(json!(a.string("surfaceAssociation", "closestComponent")));
        }
        fail!("copyDeformerWeights: query -sd, -dd, -ss, -ds, -mm or -sa");
    }
    // Mirroring on one deformer needs it named once.
    let dd = if dd.is_empty() && (ds.is_empty() || ds == ss) { sd.clone() } else { dd };
    if sd.is_empty() || dd.is_empty() {
        fail!("copyDeformerWeights: give -sd and -dd");
    }
    let g = &mut s.graph;
    let (src, dst) = (crate::deform_cmds::deformer_named(g, &sd, "copyDeformerWeights")?, crate::deform_cmds::deformer_named(g, &dd, "copyDeformerWeights")?);
    g.evaluate();
    let Some(input_s) = crate::deform::input_mesh(g, src) else { fail!("copyDeformerWeights: {sd} has no mesh") };
    let Some(input_d) = crate::deform::input_mesh(g, dst) else { fail!("copyDeformerWeights: {dd} has no mesh") };
    let (ns, nd) = (input_s.point_capacity() as usize, input_d.point_capacity() as usize);
    let mut from = crate::deform::floats_of(g.node(src).unwrap().data(), crate::deform::WEIGHTS).map(|f| f.0.clone()).unwrap_or_else(|| vec![1.0; ns]);
    from.resize(ns, 0.0);
    let geom = |g: &mut Graph, name: &str, fallback: NodeId| -> DMat4 {
        let id = if name.is_empty() { None } else { g.find(name).ok().flatten() };
        let id = id.or_else(|| crate::deform::geometry_names(g, fallback).first().and_then(|n| g.find(n).ok().flatten()));
        match id {
            Some(i) => crate::rig::world(g, crate::dag::transform_of(g, i)),
            None => DMat4::IDENTITY,
        }
    };
    let (ws, wd) = (geom(g, &ss, src), geom(g, &ds, dst));
    // Copying between two deformers is a plain copy unless a mirror plane is named; on one deformer it mirrors (XY).
    let mirror = if a.bool("noMirror") || (src != dst && !a.has("mirrorMode")) { None } else { Some(a.string("mirrorMode", "XY").to_uppercase()) };
    let axis = match mirror.as_deref() {
        Some("XY") => Some(2usize),
        Some("XZ") => Some(1),
        Some("YZ") => Some(0),
        _ => None,
    };
    let inverse = a.bool("mirrorInverse");
    let smooth = a.bool("smooth");
    let mut kept = crate::deform::floats_of(g.node(dst).unwrap().data(), crate::deform::WEIGHTS).map(|f| f.0.clone()).unwrap_or_else(|| vec![1.0; nd]);
    kept.resize(nd, 0.0);
    let flip = |p: DVec3| -> DVec3 {
        match mirror.as_deref() {
            Some("XY") => DVec3::new(p.x, p.y, -p.z),
            Some("XZ") => DVec3::new(p.x, -p.y, p.z),
            Some("YZ") => DVec3::new(-p.x, p.y, p.z),
            _ => p,
        }
    };
    // Closest point: the source points in the destination's world.
    let spots: Vec<DVec3> = (0..ns as u32).filter(|&p| input_s.point_valid(p)).map(|p| ws.transform_point3(input_s.position(p))).collect();
    let keys: Vec<usize> = (0..ns as u32).filter(|&p| input_s.point_valid(p)).map(|p| p as usize).collect();
    let mut w = if axis.is_some() { kept } else { vec![0.0; nd] };
    for p in 0..nd as u32 {
        if !input_d.point_valid(p) {
            continue;
        }
        let at = wd.transform_point3(input_d.position(p));
        // Mirroring: the positive side copies onto the negative, -mirrorInverse the other way; the side copied from
        // stays as it is.
        if let Some(ax) = axis {
            let on_dest = if inverse { at[ax] > 1e-9 } else { at[ax] < -1e-9 };
            if !on_dest {
                continue;
            }
        }
        let q = flip(at);
        let mut by: Vec<(f64, usize)> = spots.iter().enumerate().map(|(i, sp)| (sp.distance(q), keys[i])).collect();
        by.sort_by(|a, b| a.0.total_cmp(&b.0));
        // -smooth: between the nearest points by distance, rather than from the one nearest.
        let take = if smooth { 4 } else { 1 };
        by.truncate(take);
        w[p as usize] = if by.first().is_some_and(|x| x.0 < 1e-9) || !smooth {
            by.first().and_then(|x| from.get(x.1)).copied().unwrap_or(0.0)
        } else {
            let total: f64 = by.iter().map(|x| 1.0 / x.0).sum();
            by.iter().map(|x| from.get(x.1).copied().unwrap_or(0.0) / x.0 / total).sum()
        };
    }
    crate::deform::set_floats(g, dst, crate::deform::WEIGHTS, w)?;
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    for name in ["bindSkin", "rigidBind"] {
        c.register(
            Cmd::new(name, "Rigid bind: each point follows one joint, a skinCluster of 1.0 weights. -tsb only the bones named, -ub unbinds, -ubk keeps history. Returns 1.", bind_skin)
                .positional("skeleton and geometry")
                .flag(f("byClosestPoint", "bcp", B, "Accepted: closest point is how it binds."))
                .flag(f("byPartition", "bp", B, "Accepted for existing scripts."))
                .flag(f("colorJoints", "cj", B, "Accepted for existing scripts."))
                .flag(f("delete", "d", B, "Detach the skin from the skeletons and remove its history."))
                .flag(f("doNotDescend", "dnd", B, "Only the joints named."))
                .flag(f("enable", "en", B, "Accepted for existing scripts."))
                .flag(f("name", "n", S, "The skinCluster's name."))
                .flag(f("partition", "p", S, "Accepted for existing scripts."))
                .flag(f("toAll", "ta", B, "The whole skeleton (the default)."))
                .flag(f("toSelectedBones", "tsb", B, "Only the joints named."))
                .flag(f("toSkeleton", "ts", B, "The whole skeleton of each joint named (the default)."))
                .flag(f("unlock", "ul", B, "Unlock the translate, rotate and scale that binding locked on the objects."))
                .flag(f("unbind", "ub", B, "Remove the skin."))
                .flag(f("unbindKeepHistory", "ubk", B, "Stop skinning, keep the node.")),
        );
    }
    c.register(
        q_flags(Cmd::new(
            "geomBind",
            "Compute skinClusters' weights following the surface (the geodesic voxel bind, which runs the heat bind here): -fo, -mi, -gvp. \
             Makes a geomBind node holding the settings, connected to the skinClusters; -e on a skinCluster computes again with new ones, \
             -q reads them.",
            geom_bind,
        ))
        .positional("skinClusters or skinned meshes")
        .flag(f("bindMethod", "bm", I, "3 geodesic voxel (3), the only method."))
        .flag(f("falloff", "fo", F, "0 to 1: the bind's stiffness, how far influences reach."))
        .flag(f("maxInfluences", "mi", I, "Most influences on a point (-1: all)."))
        .flag(f("geodesicVoxelParams", "gvp", L, "The voxel grid's largest resolution, a power of two (256), and whether to check the voxels after (on).").multi(2))
        .flag(f("postVoxelSmooth", "pvs", I, "Smoothing passes after voxelising, held for existing scripts.")),
    );
    c.register(
        q_flags(Cmd::new(
            "jointCluster",
            "Record a joint's rigid bind settings on a jointCluster node: -j, -ab, -bb, -av, -bv, -adt, -bdt. Returns the node. -q and -e \
             take the node or its joint. It softens nothing here.",
            joint_cluster,
        ))
            .positional("joint, or with -q and -e the jointCluster")
            .flag(f("joint", "j", S, "The joint the cluster acts about."))
            .flag(f("aboveBound", "ab", F, "Where the drop-off starts along the bone above, percent of its length (10)."))
            .flag(f("belowBound", "bb", F, "Where the drop-off ends along the bone below, percent of its length (10)."))
            .flag(f("aboveValue", "av", F, "The drop-off percentage toward the bone above (50)."))
            .flag(f("belowValue", "bv", F, "The drop-off percentage toward the joint below (50)."))
            .flag(f("aboveDropoffType", "adt", S, "The drop-off curve above: linear, exponential, sine or none (linear)."))
            .flag(f("belowDropoffType", "bdt", S, "The drop-off curve below: linear, exponential, sine or none (linear)."))
            .flag(f("aboveCluster", "ac", B, "With -q: the cluster of the bone above, the parent joint's jointCluster."))
            .flag(f("belowCluster", "bc", B, "With -q: this joint's cluster."))
            .flag(f("deformerTools", "dt", B, "With -q: the helper nodes, the joint it acts about."))
            .flag(f("name", "n", S, "The node's name.")),
    );
    c.register(
        q_flags(Cmd::new("pairBlend", "Put a pairBlend on translates or rotates: -nd node -at attr. What drove the attribute feeds input 1. Returns the node; -q -in1 -in2 -at read its connections.", pair_blend))
            .positional("pairBlend, with -q")
            .flag(f("node", "nd", S, "The node driven."))
            .flag(f("attribute", "at", ArgType::Any, "The attributes.").multi(1))
            .flag(f("input1", "i1", B, "With -q: what feeds the first pair."))
            .flag(f("input2", "i2", B, "With -q: what feeds the second pair."))
            .flag(f("name", "n", S, "The node's name.")),
    );
    c.register(
        Cmd::new("matchTransform", "Match objects to the last one: -pos, -rot, -scl, -piv; all three transforms when none.", match_transform)
            .positional("objects, then the target")
            .flag(f("position", "pos", B, "Line the pivots up."))
            .flag(f("positionX", "px", B, "Match the X position."))
            .flag(f("positionY", "py", B, "Match the Y position."))
            .flag(f("positionZ", "pz", B, "Match the Z position."))
            .flag(f("rotation", "rot", B, "Match rotation."))
            .flag(f("rotationX", "rx", B, "Match the X rotation."))
            .flag(f("rotationY", "ry", B, "Match the Y rotation."))
            .flag(f("rotationZ", "rz", B, "Match the Z rotation."))
            .flag(f("scale", "scl", B, "Match scale."))
            .flag(f("scaleX", "sx", B, "Match the X scale."))
            .flag(f("scaleY", "sy", B, "Match the Y scale."))
            .flag(f("scaleZ", "sz", B, "Match the Z scale."))
            .flag(f("scaleBox", "box", B, "Match scaling by the bounding boxes of the objects and their children."))
            .flag(f("pivots", "piv", B, "Match the rotate and scale pivots to the target's."))
            .flag(f("rotatePivot", "rp", B, "Match the rotate pivot to the target's."))
            .flag(f("scalePivot", "sp", B, "Match the scale pivot to the target's."))
            .flag(f("positionConstraint", "pcp", B, "Accepted for existing scripts.")),
    );
    c.register(
        q_flags(Cmd::new("inheritTransform", "Whether objects follow their parent's transform: toggles, -on sets, -p keeps them where they are; -q reads each.", inherit_transform))
            .positional("objects")
            .flag(f("toggle", "tgl", ArgType::Any, "Toggle (what it does anyway)."))
            .flag(f("on", "on", B, "Turn inheriting on."))
            .flag(f("off", "off", B, "Turn inheriting off: the object acts as if under the world."))
            .flag(f("preserve", "p", B, "Keep the world position.")),
    );
    c.register(
        q_flags(Cmd::new("container", "Group nodes in a container node: -n, -an adds, -rn removes; with -e on an existing one. -q -nl lists, -q -fc finds the one holding a node.", container_cmd))
            .positional("container")
            .flag(f("name", "n", S, "The container's name."))
            .flag(f("addNode", "an", ArgType::Any, "Nodes to add."))
            .flag(f("add", "add", ArgType::Any, "Nodes to add."))
            .flag(f("removeNode", "rn", ArgType::Any, "Nodes to take out."))
            .flag(f("rm", "rm", ArgType::Any, "Nodes to take out."))
            .flag(f("nodeList", "nl", B, "With -q: its nodes."))
            .flag(f("findContainer", "fc", ArgType::Any, "With -q: the container a node is in."))
            .flag(f("isContainer", "isc", B, "With -q: whether it's a container."))
            .flag(f("asset", "a", S, "Marks it an asset."))
            .flag(f("force", "f", B, "Accepted for existing scripts."))
            .flag(f("type", "typ", S, "Accepted for existing scripts."))
            .flag(f("includeHierarchyBelow", "ihb", B, "Accepted for existing scripts.")),
    );
    c.register(
        q_flags(Cmd::new("controller", "Tag objects as animation controls: controller obj; -p parent makes them its children; -q -p, -q -c read the tree; -ac all; -ic is it one; -up unparents.", controller_cmd))
            .positional("objects")
            .flag(f("parent", "p", ArgType::Any, "The parent control, or with -q ask."))
            .flag(f("children", "cld", B, "With -q: its children, in order."))
            .flag(f("allControllers", "ac", B, "All the controls."))
            .flag(f("isController", "ic", B, "Whether the object is a control."))
            .flag(f("unparent", "unp", B, "Take from its parent."))
            .flag(f("group", "g", B, "Make a control of no object, the parent of those named or selected."))
            .flag(f("index", "idx", I, "Its place among its parent's children: -q reads it, -e moves it there."))
            .flag(f("pickWalkDown", "pwd", B, "With -q: its first child."))
            .flag(f("pickWalkUp", "pwu", B, "With -q: its parent."))
            .flag(f("pickWalkLeft", "pwl", B, "With -q: the sibling before it."))
            .flag(f("pickWalkRight", "pwr", B, "With -q: the sibling after it.")),
    );
    c.register(
        Cmd::new("annotate", "A note in the viewport: -tx text, -p where; the line points at the object named. Returns the shape.", annotate)
            .positional("object")
            .flag(f("text", "tx", S, "The note."))
            .flag(f("point", "p", ArgType::Vec3, "Where it goes.")),
    );
    c.register(
        Cmd::new("distanceDimension", "Measure between two points: -sp, -ep, with locators there (made when none is). Returns the shape, whose .distance reads the length.", distance_dimension)
            .flag(f("startPoint", "sp", ArgType::Vec3, "One end."))
            .flag(f("endPoint", "ep", ArgType::Vec3, "The other end.")),
    );
    c.register(Cmd::new("paramDimension", "Label a point with its parameter: curve1.u[0.5] or surface1.uv[0.5][0.5]. Returns the shape.", param_dimension).positional("point"));
    c.register(Cmd::new("dropoffLocator", "Mark a wire's dropoff at a point: dropoff percent wire.u[...]. Returns the locator. The wire's falloff stays as it is.", dropoff_locator).positional("dropoff percent point"));
    c.register(Cmd::new("findDeformers", "The deformers on a mesh, nearest the shape first.", find_deformers).positional("objects").undoable(false));
    c.register(
        Cmd::new("copyDeformerWeights", "Copy a deformer's weights to another by closest point in world space: -sd, -dd, -ss, -ds; -mm XY, XZ or YZ to mirror them.", copy_deformer_weights)
            .flag(f("sourceDeformer", "sd", S, "The deformer to copy from."))
            .flag(f("destinationDeformer", "dd", S, "The deformer to copy to."))
            .flag(f("sourceShape", "ss", S, "Its shape."))
            .flag(f("destinationShape", "ds", S, "Its shape."))
            .flag(f("mirrorMode", "mm", S, "XY, XZ or YZ."))
            .flag(f("mirrorInverse", "mi", B, "Mirror from the negative side to the positive."))
            .flag(f("noMirror", "nm", B, "Don't mirror."))
            .flag(f("smooth", "sm", B, "Between the nearest points by distance, rather than from the one nearest."))
            .flag(f("surfaceAssociation", "sa", S, "Accepted: closest point is what's used."))
            .flag(f("uvSpace", "uv", L, "Accepted for existing scripts.")),
    );
    if let Some(d) = c.find("copyDeformerWeights").cloned() {
        c.register(crate::skin::modes(d));
    }
}
