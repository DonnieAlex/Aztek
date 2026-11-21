//! Polygon commands of the reference from polySoftEdge to polyWedgeFace that lacked their query and edit modes or some
//! flags: query and edit of each command's history node, polySphericalProjection's settings on its node, polyTransfer's
//! node copying live from the other mesh, polyUnsmooth -replaceOriginal, and the new polyUniteSkinned.

use aztek_geo::{colors, names, Mesh};
use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use glam::DMat4;
use serde_json::{json, Value};

use crate::history_qe::{f, Cmd};
use crate::node::NodeData;
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue};
use crate::session::Session;

const B: ArgType = ArgType::Bool;
const I: ArgType = ArgType::Int;
const S: ArgType = ArgType::String;

/// Each command and the node types its history node may be.
const HISTORY: &[(&str, &[&str])] = &[
    ("polySoftEdge", &["polySoftEdge"]),
    ("polySphere", &["polySphere"]),
    ("polySplit", &["polySplit"]),
    ("polySplitEdge", &["polySplitEdge"]),
    ("polySplitRing", &["polySplitRing"]),
    ("polySplitVertex", &["polySplitVert"]),
    ("polyTorus", &["polyTorus"]),
    ("polyTriangulate", &["polyTriangulate"]),
    ("polyUnite", &["polyUnite"]),
    ("polyUVRectangle", &["polyUVRectangle"]),
    ("polyWedgeFace", &["polyWedgeFace"]),
];

pub fn register(c: &mut Commands<Session>) {
    for &(name, types) in HISTORY {
        crate::history_qe::extend(c, name, types, &[], Vec::new());
    }
    unsmooth(c);
    transfer(c);
    spherical(c);
    unite_skinned(c);
}

pub(crate) fn first_num(v: &Value) -> f64 {
    match v {
        Value::Array(a) => a.first().map_or(0.0, first_num),
        x => x.as_f64().unwrap_or(0.0),
    }
}

fn truthy(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0))
}

// ---------------------------------------------------------------------------------------------------------------------
// polyUnsmooth -replaceOriginal

fn unsmooth(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyUnsmooth").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = old;
    cmd = crate::history_qe::upsert(cmd, f("replaceOriginal", "rpo", B, "Make the result in place of the original (on); off, on a copy of each mesh, which keeps no shaders from the original."));
    cmd = crate::history_qe::upsert(cmd, f("frozen", "fzn", B, "The node's frozen state."));
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if a.bool("query") || a.bool("edit") || a.get("replaceOriginal").is_none_or(truthy) {
            return run(s, a);
        }
        // Off: each mesh is copied first and the copy unsmoothed; the copy wears the default shader.
        let names = crate::mesh_nodes::mesh_objects(s, a, "polyUnsmooth")?;
        let mut copies = Vec::new();
        for n in names {
            let obj = n.split('.').next().unwrap_or(&n).to_string();
            let made = s.run("duplicate", json!({"args": [obj]}))?;
            let Some(copy) = made.as_array().and_then(|l| l.first()).and_then(Value::as_str).map(str::to_string) else { fail!("polyUnsmooth: {n} could not be copied") };
            s.run("sets", json!({"args": [copy.clone()], "edit": true, "forceElement": "initialShadingGroup"}))?;
            copies.push(json!(copy));
        }
        let mut b = a.clone();
        b.positional = copies;
        b.flags.remove("replaceOriginal");
        run(s, &b)
    });
    c.register(cmd);
    crate::history_qe::extend(c, "polyUnsmooth", &["polyUnsmooth"], &[], Vec::new());
}

// ---------------------------------------------------------------------------------------------------------------------
// polyTransfer: a node that copies from the other mesh as it changes

/// The attributes a UV node type adds here: polyTransfer's other mesh and what it copies.
pub(crate) fn uv_node_attrs(node_type: &str) -> Vec<AttrDef> {
    if node_type != "polyTransfer" {
        return Vec::new();
    }
    let k = flags::INPUT | flags::STORABLE;
    let b = |name: &str, short: &str, def: bool, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Bool, default_value: PlugValue::Bool(def), flags: k, doc: doc.into(), ..Default::default() };
    vec![
        crate::attr_defs::mesh_attr("otherPoly", "op", flags::INPUT | flags::HIDDEN, "The mesh copied from."),
        b("uvSets", "uv", true, "Copy the UV sets."),
        b("vertexColor", "vc", false, "Copy the vertex colours."),
        b("vertices", "v", false, "Copy the vertex positions."),
        AttrDef { name: "alternateObject".into(), short_name: "ao".into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: k, doc: "The object copied from.".into(), ..Default::default() },
    ]
}

/// What polyTransfer copies from one mesh onto another of the same topology.
fn copy_from(m: &mut Mesh, src: &Mesh, uvs: bool, vertices: bool, color: bool) {
    if vertices {
        for p in 0..m.point_capacity().min(src.point_capacity()) {
            if m.point_valid(p) && src.point_valid(p) {
                m.set_position(p, src.position(p));
            }
        }
    }
    if uvs {
        for set in 0..8 {
            let name = names::uv(set);
            if !src.corner_attrs().has(name.as_str()) {
                continue;
            }
            let from: Vec<[f32; 2]> = src.corner_attrs().get::<[f32; 2]>(name.as_str()).to_vec();
            let to = m.corner_attrs_mut().get_or_add::<[f32; 2]>(name.as_str());
            let n = to.len().min(from.len());
            to[..n].copy_from_slice(&from[..n]);
        }
    }
    if color {
        for set in colors::sets(src) {
            let vals = colors::values(src, &set).to_vec();
            colors::add_white(m, &set);
            let layer = colors::values_mut(m, &set);
            let k = layer.len().min(vals.len());
            layer[..k].copy_from_slice(&vals[..k]);
        }
    }
}

/// A polyTransfer node's mesh: its input with what it copies taken from the other mesh. None for other nodes, or with
/// no other mesh connected.
pub(crate) fn uv_node_live(d: &NodeData, input: &Mesh) -> Option<Mesh> {
    if d.ty().name != "polyTransfer" {
        return None;
    }
    let other = d.mesh(d.ty().attr_index("otherPoly")?)?.clone();
    let at = |n: &str| d.ty().attr_index(n).map(|i| d.bool(i));
    let mut m = input.clone();
    copy_from(&mut m, &other, at("uvSets").unwrap_or(true), at("vertices").unwrap_or(false), at("vertexColor").unwrap_or(false));
    Some(m)
}

fn mesh_of(s: &Session, name: &str, cmd: &str) -> CmdResult<NodeId> {
    crate::mesh_edit::mesh_arg(&s.graph, name, cmd)
}

/// Connects the other object's mesh to a polyTransfer node.
fn connect_other(s: &mut Session, node: NodeId, other: NodeId) -> CmdResult<()> {
    let g = &mut s.graph;
    let Some(op) = g.node(node).and_then(|n| n.ty().attr_index("otherPoly")) else { fail!("polyTransfer: not a polyTransfer node") };
    let out = PlugRef::new(other, crate::mesh_nodes::MeshNode::OUT_MESH as i32);
    g.connect(&out, &PlugRef::new(node, op as i32), true)?;
    if let Some(i) = g.node(node).and_then(|n| n.ty().attr_index("alternateObject")) {
        let name = g.path(crate::dag::transform_of(g, other));
        g.set_value(PlugRef::new(node, i as i32), PlugValue::String(name))?;
    }
    Ok(())
}

fn transfer(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyTransfer").cloned() else { return };
    let mut cmd = old;
    for fl in [
        f("alternateObject", "ao", S, "The object to copy from: the same topology as the object changed."),
        f("uvSets", "uv", B, "Copy the UV sets (on)."),
        f("vertexColor", "vc", B, "Copy the vertex colours (off)."),
        f("vertices", "v", B, "Copy the vertex positions (off)."),
    ] {
        cmd = crate::history_qe::upsert(cmd, fl);
    }
    cmd.doc = "Copy UVs (-uv, on), vertex positions (-v) and vertex colours (-vc) from the -ao object, of the same topology, through a polyTransfer \
               node that follows the other object as it changes. Returns the node."
        .into();
    cmd.run = std::sync::Arc::new(|s: &mut Session, a: &Args| transfer_run(s, a));
    c.register(cmd);
    crate::history_qe::extend(c, "polyTransfer", &["polyTransfer"], &[], Vec::new());
    // -ao in query and edit mode: the object connected, or a new one connected.
    let Some(ext) = c.find("polyTransfer").cloned() else { return };
    let run = ext.run.clone();
    let mut cmd = ext;
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let asked_ao = a.bool("query") && (a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == "alternateObject")) || a.get("alternateObject").is_some_and(|v| v == &json!(true)));
        if asked_ao || (a.bool("edit") && a.get("alternateObject").is_some_and(Value::is_string)) {
            let names = a.named();
            let Some(first) = names.first() else { fail!("polyTransfer: name the polyTransfer node or a mesh it feeds") };
            let Some(node) = crate::history_qe::find_hist(&s.graph, first, &["polyTransfer"]) else { fail!("polyTransfer: {first} has no polyTransfer node") };
            if asked_ao {
                let g = &s.graph;
                let op = g.node(node).and_then(|n| n.ty().attr_index("otherPoly")).unwrap_or(0);
                let src = g.source(&PlugRef::new(node, op as i32)).map(|p| g.path(crate::dag::transform_of(g, p.node)));
                return Ok(json!(src.unwrap_or_default()));
            }
            let other = a.string("alternateObject", "");
            let other = mesh_of(s, &other, "polyTransfer")?;
            connect_other(s, node, other)?;
            let mut b = a.clone();
            b.flags.remove("alternateObject");
            let given: Vec<String> = s.commands.current_given().map(|g| g.to_vec()).unwrap_or_default();
            if given.iter().any(|k| matches!(k.as_str(), "uvSets" | "vertices" | "vertexColor" | "caching" | "nodeState")) {
                run(s, &b)?;
            }
            s.graph.evaluate();
            return Ok(json!([s.graph.path(node)]));
        }
        run(s, a)
    });
    c.register(cmd);
}

fn transfer_run(s: &mut Session, a: &Args) -> CmdResult {
    let from = a.string("alternateObject", "");
    if from.is_empty() {
        fail!("polyTransfer: name the other mesh with -ao");
    }
    let source = mesh_of(s, &from, "polyTransfer")?;
    let (uvs, vertices, color) = (a.get("uvSets").is_none_or(truthy), a.get("vertices").is_some_and(truthy), a.get("vertexColor").is_some_and(truthy));
    let targets = crate::uv_ops::targets(s, a, "polyTransfer")?;
    let ch = a.get("constructionHistory").is_none_or(truthy);
    s.graph.evaluate();
    let Some(src) = crate::modifiers::cage(&mut s.graph, source) else { fail!("polyTransfer: {from} has no mesh") };
    s.graph.open_chunk("Transfer");
    let mut made = Vec::new();
    let r = (|| -> CmdResult<()> {
        for &mesh in targets.keys() {
            if mesh == source {
                continue;
            }
            let recorded = if ch { crate::uv_ops::record_edit(&mut s.graph, mesh, "polyTransfer", crate::uv_ops::UvEdit::default())? } else { None };
            match recorded {
                Some(h) => {
                    for (attr, v) in [("uvSets", uvs), ("vertices", vertices), ("vertexColor", color)] {
                        if let Some(i) = s.graph.node(h).and_then(|n| n.ty().attr_index(attr)) {
                            s.graph.set_value(PlugRef::new(h, i as i32), PlugValue::Bool(v))?;
                        }
                    }
                    connect_other(s, h, source)?;
                    crate::history_qe::apply_common(&mut s.graph, h, a)?;
                    if let Some(n) = a.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                        s.graph.rename(h, n)?;
                    }
                    made.push(json!(s.graph.path(h)));
                }
                None => {
                    // No history: the mesh itself takes the copy.
                    let base = crate::mesh_edit::base_plug(&mut s.graph, mesh)?;
                    let mut m = crate::mesh_edit::base_mesh(&mut s.graph, &base);
                    copy_from(&mut m, &src, uvs, vertices, color);
                    s.graph.set_value(base, PlugValue::Mesh(Some(std::sync::Arc::new(m))))?;
                }
            }
        }
        Ok(())
    })();
    s.graph.close_chunk();
    r?;
    s.graph.evaluate();
    Ok(match made.len() {
        1 => made.remove(0),
        _ => Value::Array(made),
    })
}

// ---------------------------------------------------------------------------------------------------------------------
// polySphericalProjection: the projection's settings on its polySphProj node

/// Each single-value flag and the polySphProj attribute it reads and writes.
const SPH_SINGLE: &[(&str, &str)] = &[
    ("projectionCenterX", "projectionCenterX"),
    ("projectionCenterY", "projectionCenterY"),
    ("projectionCenterZ", "projectionCenterZ"),
    ("projectionHorizontalSweep", "projectionHorizontalSweep"),
    ("projectionScaleV", "projectionScaleV"),
    ("imageCenterX", "imageCenterX"),
    ("imageCenterY", "imageCenterY"),
    ("imageScaleU", "imageScaleX"),
    ("imageScaleV", "imageScaleY"),
    ("rotationAngle", "rotationAngle"),
    ("radius", "radius"),
    ("seamCorrect", "seamCorrect"),
    ("worldSpace", "worldSpace"),
    ("rotateX", "rotateX"),
    ("rotateY", "rotateY"),
    ("rotateZ", "rotateZ"),
    ("caching", "caching"),
    ("nodeState", "nodeState"),
];

/// The flags of several values and the attributes each value goes to.
const SPH_MULTI: &[(&str, &[&str])] = &[
    ("projectionCenter", &["projectionCenterX", "projectionCenterY", "projectionCenterZ"]),
    ("imageCenter", &["imageCenterX", "imageCenterY"]),
    ("imageScale", &["imageScaleX", "imageScaleY"]),
];

/// The attributes polySphProj adds to the projection nodes': the map's height as an angle.
pub(crate) fn sph_attrs(node_type: &str) -> Vec<AttrDef> {
    if node_type != "polySphProj" {
        return Vec::new();
    }
    vec![crate::attr_defs::float_attr_short("projectionScaleV", "psv", std::f64::consts::FRAC_PI_2, "Half the angle the map covers up and down from the middle.", None, None)]
}

fn spherical(c: &mut Commands<Session>) {
    let Some(old) = c.find("polySphericalProjection").cloned() else { return };
    let mut cmd = crate::poly_extra::cyl_flags(old);
    cmd.flags.retain(|fl| fl.name != "projectionHeight");
    let ang = aztek_ops::units::Unit::Angle;
    let fl = |n: &str, sh: &str, t: ArgType, unit: aztek_ops::units::Unit, doc: &str| {
        let mut x = f(n, sh, t, doc);
        x.unit = unit;
        x
    };
    for x in [
        fl("projectionHorizontalSweep", "phs", ArgType::Float, ang, "The angle the map covers round the sphere, 0 to 360."),
        fl("projectionScale", "ps", ArgType::Any, aztek_ops::units::Unit::None, "The map's width and height as angles (180 90): half of what it covers each way.").multi(2),
        fl("projectionScaleU", "psu", ArgType::Float, ang, "The map's width as an angle (180): half of what it covers round the sphere."),
        fl("projectionScaleV", "psv", ArgType::Float, ang, "The map's height as an angle (90): half of what it covers up and down."),
        fl("rotate", "ro", ArgType::Any, aztek_ops::units::Unit::None, "The projection's turn (0 0 0).").multi(3),
        fl("radius", "r", ArgType::Float, aztek_ops::units::Unit::Linear, "The manipulator's radius."),
    ] {
        cmd = crate::history_qe::upsert(cmd, x);
    }
    crate::poly_extra::projection_qe(c, cmd, "polySphProj", (SPH_SINGLE, SPH_MULTI));
    // -projectionScale, -projectionScaleU and -rotate: worked out from and onto the sweep and the angles.
    let Some(inner) = c.find("polySphericalProjection").cloned() else { return };
    let run = inner.run.clone();
    let mut cmd = inner;
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let user = aztek_ops::units::user_call();
        if a.bool("query") {
            let asked: Vec<String> = a.get("queried").and_then(Value::as_array).map(|q| q.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            if let Some(flag) = asked.first().filter(|f| matches!(f.as_str(), "projectionScale" | "projectionScaleU" | "rotate")) {
                let names = a.named();
                let Some(first) = names.first() else { fail!("polySphericalProjection: name the polySphProj node or a mesh it feeds") };
                let Some(node) = crate::history_qe::find_hist(&s.graph, first, &["polySphProj"]) else { fail!("polySphericalProjection: {first} has no polySphProj node") };
                let path = s.graph.path(node);
                let mut at = |n: &str| -> f64 { s.run("getAttr", json!({"args": [format!("{path}.{n}")]})).ok().map(|v| crate::poly_tail::first_num(&v)).unwrap_or(0.0) };
                let deg = |x: f64| if user { x.to_degrees() } else { x };
                let sweep = at("projectionHorizontalSweep");
                let v = at("projectionScaleV");
                let r = [at("rotateX"), at("rotateY"), at("rotateZ")];
                return Ok(match flag.as_str() {
                    // The flag's own unit converts this one as the command returns.
                    "projectionScaleU" => json!(sweep * 0.5),
                    "projectionScale" => json!([deg(sweep * 0.5), deg(v)]),
                    _ => json!([deg(r[0]), deg(r[1]), deg(r[2])]),
                });
            }
            return run(s, a);
        }
        let given: Option<Vec<String>> = s.commands.current_given().map(|g| g.to_vec());
        let named = |k: &str| given.as_ref().is_none_or(|g| g.iter().any(|x| x == k));
        let mut b = a.clone();
        let mut extra: Vec<(&str, f64)> = Vec::new();
        let rad = |x: f64| if user { x.to_radians() } else { x };
        if named("projectionScaleU") {
            if let Some(u) = a.get("projectionScaleU").and_then(Value::as_f64) {
                extra.push(("projectionHorizontalSweep", u * 2.0));
            }
        }
        if named("projectionScale") {
            if let Some(Value::Array(v)) = a.get("projectionScale") {
                if let Some(u) = v.first().and_then(Value::as_f64) {
                    extra.push(("projectionHorizontalSweep", rad(u) * 2.0));
                }
                if let Some(x) = v.get(1).and_then(Value::as_f64) {
                    extra.push(("projectionScaleV", rad(x)));
                }
            }
        }
        if named("rotate") {
            if let Some(Value::Array(v)) = a.get("rotate") {
                for (k, attr) in ["rotateX", "rotateY", "rotateZ"].into_iter().enumerate() {
                    if let Some(x) = v.get(k).and_then(Value::as_f64) {
                        extra.push((attr, rad(x)));
                    }
                }
            }
        }
        for k in ["projectionScaleU", "projectionScale", "rotate"] {
            b.flags.remove(k);
        }
        let out = run(s, &b)?;
        if extra.is_empty() {
            return Ok(out);
        }
        let mut nodes: Vec<String> = out.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if let Some(x) = out.as_str() {
            nodes.push(x.to_string());
        }
        for n in nodes {
            let Ok(Some(id)) = s.graph.find(&n) else { continue };
            if !s.graph.node(id).is_some_and(|x| x.ty().name == "polySphProj") {
                continue;
            }
            let path = s.graph.path(id);
            for (attr, v) in &extra {
                s.run("setAttr", json!({"args": [format!("{path}.{attr}"), v]}))?;
            }
        }
        s.graph.evaluate();
        Ok(out)
    });
    c.register(cmd);
}

// ---------------------------------------------------------------------------------------------------------------------
// polyUniteSkinned

fn unite_skinned(c: &mut Commands<Session>) {
    let cmd = Cmd::new(
        "polyUniteSkinned",
        "Combine meshes as polyUnite does, keeping their smooth skinning: the result is bound to every influence of the meshes, each point keeping \
         its weights. The meshes are combined as they are at bind. Returns [transform, node], or [transform] with -ch 0.",
        unite_skinned_run,
    )
    .positional("the meshes (the selection when none)")
    .flag(f("centerPivot", "cp", B, "Put the result's pivot at the middle of the meshes' bounding box."))
    .flag(f("constructionHistory", "ch", B, "Keep the polyUnite node and the meshes feeding it (on)."))
    .flag(f("mergeUVSets", "muv", I, "0 every UV set of every mesh a set of its own, 1 sets of the same name merged (the default), 2 merged so UV links keep working."))
    .flag(f("objectPivot", "op", B, "Put the result's pivot at the last mesh's pivot."));
    c.register(crate::history_qe::qe_flags(cmd));
}

fn unite_skinned_run(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") || a.bool("edit") {
        // The settings live on the polyUnite node: mergeUVSets there.
        return crate::history_qe::qe(s, a, "polyUniteSkinned", &["polyUnite"], &[]).unwrap_or(Ok(Value::Null));
    }
    let shapes = crate::mesh_edit::ref_poly::mesh_cmds::shapes_named(s, a, "polyUniteSkinned")?;
    if shapes.len() < 2 {
        fail!("polyUniteSkinned: give at least two meshes");
    }
    // Each mesh's skin: its influences, their bind matrices and the points' weights.
    let mut joints: Vec<NodeId> = Vec::new();
    let mut bind_pre: Vec<DMat4> = Vec::new();
    let mut weights: Vec<Vec<(u32, f64)>> = Vec::new();
    let mut skins: Vec<(NodeId, i64)> = Vec::new();
    s.graph.evaluate();
    for &m in &shapes {
        let g = &s.graph;
        let count = g.node(m).and_then(|n| n.data().mesh(crate::mesh_nodes::MeshNode::OUT_MESH).map(|x| x.point_capacity())).unwrap_or(0) as usize;
        let Some(sk) = crate::skin::skin_of(g, m) else {
            // An unskinned mesh: its points have no weights.
            weights.extend(std::iter::repeat_n(Vec::new(), count));
            continue;
        };
        let Some(bind) = crate::skin::bind_of(g, sk) else { fail!("polyUniteSkinned: {} has no skin weights", g.path(sk)) };
        let mut remap = std::collections::BTreeMap::new();
        for (el, j) in crate::skin::influences(g, sk) {
            let k = match joints.iter().position(|&x| x == j) {
                Some(k) => k,
                None => {
                    joints.push(j);
                    bind_pre.push(bind.bind_pre.get(el as usize).copied().unwrap_or(DMat4::IDENTITY));
                    joints.len() - 1
                }
            };
            remap.insert(el as u32, k as u32);
        }
        for p in 0..count {
            let w = bind.weights.get(p).map(|w| w.iter().filter_map(|&(i, x)| remap.get(&i).map(|&k| (k, x))).collect()).unwrap_or_default();
            weights.push(w);
        }
        let state = s.graph.node(sk).map_or(0, |n| crate::history_qe::node_state(n.data()));
        skins.push((sk, state));
    }
    if joints.is_empty() {
        fail!("polyUniteSkinned: none of the meshes is skinned");
    }
    // The meshes combine as they are at bind: their skins pass the meshes through while polyUnite reads them.
    let state_attr = |s: &Session, sk: NodeId| s.graph.node(sk).and_then(|n| n.ty().attr_index("nodeState"));
    for &(sk, _) in &skins {
        if let Some(i) = state_attr(s, sk) {
            s.graph.set_value(PlugRef::new(sk, i as i32), PlugValue::Int(1))?;
        }
    }
    let mut b = a.clone();
    b.flags.remove("centerPivot");
    b.flags.remove("objectPivot");
    let made = crate::mesh_edit::ref_poly::mesh_cmds::multi_input(s, &b, "polyUnite", &shapes, 0, "polySurface1");
    let ch = a.get("constructionHistory").is_none_or(truthy) && crate::history::enabled(&s.graph);
    if let Ok(out) = &made {
        if ch {
            // The polyUnite node reads each mesh before its skin, so the result isn't deformed twice.
            let node = out.get(1).and_then(Value::as_str).and_then(|n| s.graph.find(n).ok().flatten());
            if let Some(h) = node {
                for (k, &m) in shapes.iter().enumerate() {
                    let Some(sk) = crate::skin::skin_of(&s.graph, m) else { continue };
                    if let Some(src) = s.graph.source(&PlugRef::new(sk, crate::skin::SK_IN as i32)) {
                        let ip = s.graph.node(h).and_then(|n| n.ty().attr_index("inputPoly")).unwrap_or(0);
                        s.graph.connect(&src, &PlugRef::element(h, ip as i32, k as i32), true)?;
                    }
                }
            }
        }
    }
    for &(sk, state) in &skins {
        if s.graph.node(sk).is_some() {
            if let Some(i) = state_attr(s, sk) {
                s.graph.set_value(PlugRef::new(sk, i as i32), PlugValue::Int(state))?;
            }
        }
    }
    let out = made?;
    let Some(xf) = out.get(0).and_then(Value::as_str).and_then(|n| s.graph.find(n).ok().flatten()) else { fail!("polyUniteSkinned: no result") };
    s.graph.evaluate();
    crate::skin::bind_imported(&mut s.graph, xf, &joints, bind_pre, DMat4::IDENTITY, weights, true, &[])?;
    s.graph.evaluate();
    let path = s.graph.path(xf);
    if a.bool("centerPivot") {
        s.run("xform", json!({"args": [path.clone()], "centerPivots": true}))?;
    } else if a.bool("objectPivot") {
        let last = crate::dag::transform_of(&s.graph, *shapes.last().unwrap());
        let piv = s.run("xform", json!({"args": [s.graph.path(last)], "query": true, "worldSpace": true, "queried": ["rotatePivot"]}))?;
        let v: Vec<f64> = piv.as_array().into_iter().flatten().filter_map(Value::as_f64).take(3).collect();
        if v.len() == 3 {
            s.run("xform", json!({"args": [path.clone()], "worldSpace": true, "pivots": v}))?;
        }
    }
    s.run("select", json!({"args": [path], "replace": true}))?;
    Ok(out)
}
