//! XJEN's commands: making an Interactive Groom description, and xgmSplineQuery.

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::DVec3;
use serde_json::{json, Value};

use super::{descriptions, render_splines, sculpt, BASE, DESCRIPTION, SCALE, SCULPT};
use crate::plug::{NodeId, PlugValue, NO_NODE};
use crate::session::Session;

/// The commands this file registers.
pub const NAMES: &[&str] = &["xjenCreateGroom", "xgmSplineQuery", "xjenGroomCtx", "xjenGroomStroke", "xjenSculptLayer", "xgmSplineSelect", "xjenAddModifier", "xjenGroomInfo", "xjenConvertToPolygons", "xjenPreset", "xjenTransfer", "xjenRemoveModifier", "xjenExportCache", "xjenImportCache", "xjenBakeModifier"];

/// The description a name means: its shape, from the shape or its transform.
pub fn description(s: &Session, n: &str) -> Option<NodeId> {
    let id = s.graph.find(n).ok().flatten()?;
    if s.graph.node(id)?.ty().is_a(DESCRIPTION) {
        return Some(id);
    }
    s.graph.children(id).iter().copied().find(|&k| s.graph.node(k).is_some_and(|x| x.ty().is_a(DESCRIPTION)))
}

/// The descriptions named, or else selected.
fn named_descriptions(s: &Session, a: &aztek_ops::Args) -> Vec<NodeId> {
    let names = a.named();
    if !names.is_empty() {
        return names.iter().filter_map(|n| description(s, n)).collect();
    }
    s.graph.selection().iter().filter_map(|&id| description(s, &s.graph.path(id))).collect()
}

/// A mesh shape and the faces named on it (empty: all of them).
struct Target {
    shape: String,
    faces: Vec<u32>,
}

fn mesh_shape(s: &mut Session, name: &str) -> Option<String> {
    let id = s.graph.find(name).ok().flatten()?;
    let is_mesh = |s: &Session, id: NodeId| s.graph.node(id).is_some_and(|n| n.ty().is_a("mesh"));
    if is_mesh(s, id) {
        return Some(s.graph.path(id));
    }
    let kids = s.run("listRelatives", json!({"args": [name], "shapes": true, "fullPath": true})).ok()?;
    kids.as_array()?.iter().filter_map(Value::as_str).find(|k| {
        let id = s.graph.find(k).ok().flatten();
        id.is_some_and(|id| is_mesh(s, id) && !s.graph.node(id).is_some_and(|n| n.ty().attr_index("intermediateObject").is_some_and(|i| n.data().value(i).as_bool())))
    }).map(str::to_string)
}

/// Meshes and faces from names such as pPlane1 or pPlane1.f[2:5].
fn targets(s: &mut Session, names: &[String]) -> Vec<Target> {
    let mut out: Vec<Target> = Vec::new();
    for n in names {
        let (obj, faces) = match n.split_once(".f[") {
            Some((o, rest)) => (o.to_string(), super::parse_faces(rest.trim_end_matches(']'))),
            None => (n.clone(), Vec::new()),
        };
        let Some(shape) = mesh_shape(s, &obj) else { continue };
        match out.iter_mut().find(|t| t.shape == shape) {
            Some(t) => {
                if t.faces.is_empty() || faces.is_empty() {
                    t.faces.clear();
                } else {
                    t.faces.extend(faces);
                }
            }
            None => out.push(Target { shape, faces }),
        }
    }
    out
}

fn faces_text(faces: &[u32]) -> String {
    faces.iter().map(u32::to_string).collect::<Vec<_>>().join(" ")
}

fn set(s: &mut Session, plug: String, v: Value) -> CmdResult<()> {
    s.run("setAttr", json!({"args": [plug, v]}))?;
    Ok(())
}
fn connect(s: &mut Session, a: String, b: String) -> CmdResult<()> {
    s.run("connectAttr", json!({"args": [a, b]}))?;
    Ok(())
}
fn create(s: &mut Session, ty: &str, name: &str) -> CmdResult<String> {
    let r = s.run("createNode", json!({"args": [ty], "name": name, "skipSelect": true}))?;
    Ok(r.as_str().unwrap_or_default().to_string())
}

/// One description on a mesh, with its base, Scale and Sculpt modifiers and a hair shader. Returns the transform.
fn make(s: &mut Session, t: &Target, a: &aztek_ops::Args) -> CmdResult<String> {
    let name = a.string("name", "description1");
    let (tr, sh) = crate::dag::create_with_shape(&mut s.graph, DESCRIPTION, &name, NO_NODE)?;
    let (tr, sh) = (s.graph.path(tr), s.graph.path(sh));
    let short = tr.rsplit('|').next().unwrap_or(&tr).to_string();
    let base = create(s, BASE, &format!("{short}_base"))?;
    let scale = create(s, SCALE, &format!("{short}_scale"))?;
    let sculpt = create(s, SCULPT, &format!("{short}_sculpt"))?;
    connect(s, format!("{}.worldMesh[0]", t.shape), format!("{base}.boundMesh"))?;
    connect(s, format!("{base}.outSplineData"), format!("{scale}.inSplineData"))?;
    connect(s, format!("{scale}.outSplineData"), format!("{sculpt}.inSplineData"))?;
    connect(s, format!("{sculpt}.outSplineData"), format!("{sh}.inSplineData"))?;
    set(s, format!("{base}.boundFaces"), json!(faces_text(&t.faces)))?;
    set(s, format!("{base}.seed"), json!(a.int("seed", 0)))?;
    set(s, format!("{base}.density"), json!(a.float("density", 10.0)))?;
    set(s, format!("{base}.initialLength"), json!(a.float("length", 1.0)))?;
    set(s, format!("{base}.cvCount"), json!(a.int("cvCount", 5).clamp(2, 1000)))?;
    set(s, format!("{sh}.widthScale"), json!(a.float("widthScale", 0.01)))?;
    set(s, format!("{sculpt}.layer[0].layerName"), json!("Base"))?;
    let id = s.graph.find(&sculpt)?.unwrap_or(NO_NODE);
    let data = sculpt::SculptData { layers: vec![sculpt::Layer { name: "Base".into(), ..Default::default() }], ..Default::default() };
    if let Some(p) = crate::harold::network::attr(&s.graph, id, "sculptData") {
        s.graph.set_value(p, PlugValue::Data(Some(std::sync::Arc::new(data))))?;
    }
    // A default hair shader, as the reference gives a new description.
    let shader = s.run("shadingNode", json!({"args": ["aiStandardHair"], "asShader": true, "name": format!("{short}_hair")}))?;
    let shader = shader.as_str().unwrap_or_default().to_string();
    let sg = s.run("sets", json!({"renderable": true, "noSurfaceShader": true, "empty": true, "name": format!("{shader}SG")}))?;
    let sg = sg.as_str().unwrap_or_default().to_string();
    connect(s, format!("{shader}.outColor"), format!("{sg}.surfaceShader"))?;
    s.run("sets", json!({"edit": true, "forceElement": sg, "args": [sh]}))?;
    Ok(tr)
}

fn layer_attrs(s: &mut Session, sculpt: &str, names: &[(String, f64, bool)], before: usize) -> CmdResult<()> {
    for (k, (n, w, v)) in names.iter().enumerate() {
        set(s, format!("{sculpt}.layer[{k}].layerName"), json!(n))?;
        set(s, format!("{sculpt}.layer[{k}].layerWeight"), json!(w))?;
        set(s, format!("{sculpt}.layer[{k}].layerVisibility"), json!(v))?;
    }
    for k in names.len()..before {
        for a in ["layerName", "layerWeight", "layerVisibility", "layerGroup"] {
            let _ = s.run("removeMultiInstance", json!({"args": [format!("{sculpt}.layer[{k}].{a}")], "break": true}));
        }
    }
    Ok(())
}

fn set_sculpt(s: &mut Session, sculpt: NodeId, d: sculpt::SculptData) -> CmdResult<()> {
    if let Some(p) = crate::harold::network::attr(&s.graph, sculpt, "sculptData") {
        s.graph.set_value(p, PlugValue::Data(Some(std::sync::Arc::new(d))))?;
    }
    Ok(())
}

fn sculpt_of(s: &Session, desc: NodeId) -> Option<NodeId> {
    super::stack(&s.graph, desc).into_iter().rev().find(|&n| s.graph.node(n).is_some_and(|x| x.ty().is_a(SCULPT)))
}

/// Short names the command takes for the modifier types.
pub(super) const MODIFIER_NAMES: &[(&str, &str)] = &[
    ("clump", "xgmModifierClump"),
    ("noise", "xgmModifierNoise"),
    ("cut", "xgmModifierCut"),
    ("collide", "xgmModifierCollide"),
    ("displace", "xgmModifierDisplace"),
    ("guide", "xgmModifierGuide"),
    ("linearWire", "xgmModifierLinearWire"),
    ("curveToSpline", "xgmCurveToSpline"),
    ("scale", "xgmModifierScale"),
    ("sculpt", "xgmModifierSculpt"),
    ("splineCache", "xgmModifierSplineCache"),
];

/// Curves made from some of a description's splines, as guides or wires: every spline whose id hashes below the
/// fraction. Returns their transforms.
fn curves_from_splines(s: &mut Session, desc: NodeId, fraction: f64, stem: &str) -> CmdResult<Vec<String>> {
    s.graph.evaluate();
    let pts: Vec<Vec<[f64; 3]>> = render_splines(&s.graph, desc)
        .map(|d| d.splines.iter().filter(|x| super::hash01(x.id ^ 0x6D1D) < fraction).map(|x| x.points.iter().map(|p| p.to_array()).collect()).collect())
        .unwrap_or_default();
    let mut out = Vec::new();
    for (k, p) in pts.into_iter().enumerate() {
        let deg = if p.len() >= 4 { 3 } else { 1 };
        let r = s.run("curve", json!({"degree": deg, "point": p, "name": format!("{stem}{}", k + 1)}))?;
        let name = r.as_str().map(str::to_string).or_else(|| r[0].as_str().map(str::to_string)).unwrap_or_default();
        out.push(name);
    }
    Ok(out)
}

fn shape_of_curve(s: &mut Session, name: &str) -> Option<String> {
    let id = s.graph.find(name).ok().flatten()?;
    if s.graph.node(id)?.ty().is_a(crate::curves::SHAPE_TYPE) {
        return Some(s.graph.path(id));
    }
    s.graph.children(id).iter().copied().find(|&k| s.graph.node(k).is_some_and(|n| n.ty().is_a(crate::curves::SHAPE_TYPE))).map(|k| s.graph.path(k))
}

fn frame_range(s: &Session, a: &aztek_ops::Args) -> (f64, f64) {
    let f = crate::anim::frame(&s.graph);
    match a.get("frameRange").and_then(Value::as_array) {
        Some(v) if v.len() == 2 => (v[0].as_f64().unwrap_or(f), v[1].as_f64().unwrap_or(f)),
        _ => (f, f),
    }
}

pub fn register(c: &mut Commands<Session>) {
    use super::groom;
    super::extra::register(c);
    c.register(
        CommandDef::<Session>::new(
            "xjenExportCache",
            "Cache > Export Cache: the description named (or selected), as Alembic curves with widths, over -frameRange.",
            |s, a| {
                let Some(&d) = named_descriptions(s, &a).first() else { fail!("xjenExportCache: name or select a description") };
                let Some(file) = a.get("file").and_then(Value::as_str).map(str::to_string) else { fail!("xjenExportCache: give -file") };
                let (f0, f1) = frame_range(s, &a);
                let tr = s.graph.path(s.graph.parent(d));
                let job = crate::alembic::Job { file: file.clone(), roots: vec![tr], start: f0, end: f1, step: 1.0, uv: false, world_space: true };
                crate::alembic::export(s, &job)?;
                Ok(json!(file))
            },
        )
        .positional("description")
        .flag(FlagDef::new("file", "f", ArgType::String, Value::Null, "The Alembic file to write."))
        .flag(FlagDef::new("frameRange", "fr", ArgType::List, Value::Null, "First and last frame; the current frame by default."))
        .undoable(false),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenImportCache",
            "Cache > Import Cache: a Spline Cache modifier at the top of the description's stack, playing the file's curves.",
            |s, a| {
                let Some(&d) = named_descriptions(s, &a).first() else { fail!("xjenImportCache: name or select a description") };
                let Some(file) = a.get("file").and_then(Value::as_str).map(str::to_string) else { fail!("xjenImportCache: give -file") };
                let tr = s.graph.path(s.graph.parent(d));
                let node = s.run("xjenAddModifier", json!({"args": [tr], "type": "splineCache"}))?;
                let node = node.as_str().unwrap_or_default().to_string();
                s.run("setAttr", json!({"args": [format!("{node}.cacheFile"), file], "type": "string"}))?;
                Ok(json!(node))
            },
        )
        .positional("description")
        .flag(FlagDef::new("file", "f", ArgType::String, Value::Null, "The Alembic file to play.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenBakeModifier",
            "Bakes a Noise modifier: its output over -frameRange written to -file as Alembic curves, and the modifier set to \
             Baked, playing that file.",
            |s, a| {
                let names = a.named();
                let Some(n) = names.first() else { fail!("xjenBakeModifier: name a modifier") };
                let id = s.graph.find(n)?.ok_or_else(|| aztek_ops::CommandError::new(format!("xjenBakeModifier: no {n}")))?;
                let Some(file) = a.get("file").and_then(Value::as_str).map(str::to_string) else { fail!("xjenBakeModifier: give -file") };
                let (f0, f1) = frame_range(s, &a);
                let out = crate::harold::network::attr(&s.graph, id, "outSplineData").ok_or_else(|| aztek_ops::CommandError::new("xjenBakeModifier: not a modifier"))?;
                let was = crate::anim::frame(&s.graph);
                let (mut pts, mut counts) = (Vec::new(), Vec::new());
                let mut f = f0;
                while f <= f1 + 1e-6 {
                    crate::anim::set_frame(&mut s.graph, f);
                    let v = s.graph.value(&out)?;
                    let d = super::spline_data(&v).cloned().unwrap_or_default();
                    if counts.is_empty() {
                        counts = d.splines.iter().map(|x| x.points.len() as i32).collect();
                    }
                    pts.push(d.splines.iter().flat_map(|x| x.points.iter().map(|p| p.as_vec3().to_array())).collect::<Vec<_>>());
                    f += 1.0;
                }
                crate::anim::set_frame(&mut s.graph, was);
                let fps = crate::anim::fps();
                let mut arc = aztek_abc::Archive::default();
                arc.meta = "_ai_Application=Aztek".into();
                arc.samplings.push(aztek_abc::TimeSampling::Uniform { start: f0 / fps, step: 1.0 / fps });
                arc.top.children.push(aztek_abc::geom::curves("bake", 1, &pts, &[], &counts, false));
                std::fs::write(&file, aztek_abc::write(&arc)).map_err(|e| aztek_ops::CommandError::new(format!("xjenBakeModifier: {file}: {e}")))?;
                let p = s.graph.path(id);
                s.run("setAttr", json!({"args": [format!("{p}.cacheFile"), file.clone()], "type": "string"}))?;
                s.run("setAttr", json!({"args": [format!("{p}.mode"), 1]}))?;
                Ok(json!(file))
            },
        )
        .positional("modifier")
        .flag(FlagDef::new("file", "f", ArgType::String, Value::Null, "The Alembic file to write."))
        .flag(FlagDef::new("frameRange", "fr", ArgType::List, Value::Null, "First and last frame; the current frame by default.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenAddModifier",
            "Adds a modifier to a description's stack, at the top (just under the shape) or -after a node in it. -type: clump, \
             noise, cut, collide, displace, guide, linearWire, curveToSpline, scale or sculpt. Guide and Linear Wire take \
             -curves, or make them from -fraction of the splines (0.1 by default) when none are given; a Linear Wire keeps \
             copies of its wires as their reference state. Collide takes -meshes. Returns the modifier.",
            |s, a| {
                let Some(&desc) = named_descriptions(s, &a).first() else { fail!("xjenAddModifier: name or select a description") };
                let kind = a.string("type", "");
                let Some(&(_, ty)) = MODIFIER_NAMES.iter().find(|(k, t)| *k == kind || *t == kind) else {
                    fail!("xjenAddModifier: -type is one of {}", MODIFIER_NAMES.iter().map(|x| x.0).collect::<Vec<_>>().join(", "))
                };
                let sh = s.graph.path(desc);
                let short = s.graph.node(s.graph.parent(desc)).map(|n| n.name().to_string()).unwrap_or_default();
                let stem = ty.trim_start_matches("xgmModifier").trim_start_matches("xgm");
                let node = create(s, ty, &format!("{short}_{}1", stem[..1].to_lowercase() + &stem[1..]))?;
                // Where it goes: under the node named by -after's consumer, or under the shape.
                let below = match a.get("after").and_then(Value::as_str) {
                    Some(after) => {
                        let id = s.graph.find(after)?.ok_or_else(|| aztek_ops::CommandError::new(format!("xjenAddModifier: no {after}")))?;
                        let out = crate::harold::network::attr(&s.graph, id, "outSplineData").ok_or_else(|| aztek_ops::CommandError::new(format!("xjenAddModifier: {after} is not in a stack")))?;
                        let consumer = s.graph.destinations(&out).into_iter().next().ok_or_else(|| aztek_ops::CommandError::new(format!("xjenAddModifier: nothing reads {after}")))?;
                        (s.graph.path(consumer.node), s.graph.path(id))
                    }
                    None => {
                        let p = crate::harold::network::attr(&s.graph, desc, "inSplineData").unwrap();
                        let src = s.graph.source(&p).ok_or_else(|| aztek_ops::CommandError::new("xjenAddModifier: the description has no stack"))?;
                        (sh.clone(), s.graph.path(src.node))
                    }
                };
                let (consumer, feeder) = below;
                connect(s, format!("{feeder}.outSplineData"), format!("{node}.inSplineData"))?;
                s.run("connectAttr", json!({"args": [format!("{node}.outSplineData"), format!("{consumer}.inSplineData")], "force": true}))?;
                let mut curves: Vec<String> = a.get("curves").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
                if matches!(ty, "xgmModifierGuide" | "xgmModifierLinearWire") && curves.is_empty() {
                    let stem = if ty == "xgmModifierGuide" { format!("{short}_guide") } else { format!("{short}_wire") };
                    curves = curves_from_splines(s, desc, a.float("fraction", 0.1).clamp(0.0, 1.0), &stem)?;
                }
                let attr = match ty {
                    "xgmModifierGuide" => Some("guideCurves"),
                    "xgmModifierLinearWire" => Some("wireCurves"),
                    "xgmCurveToSpline" => Some("inputCurves"),
                    _ => None,
                };
                if let Some(attr) = attr {
                    for (k, cname) in curves.iter().enumerate() {
                        let Some(cs) = shape_of_curve(s, cname) else { fail!("xjenAddModifier: {cname} is not a curve") };
                        connect(s, format!("{cs}.worldSpace"), format!("{node}.{attr}[{k}]"))?;
                        if ty == "xgmModifierLinearWire" {
                            // The reference state: a hidden copy, as the wire is now.
                            let dup = s.run("duplicate", json!({"args": [cname], "name": format!("{cname}_reference")}))?;
                            let dup = dup.as_array().and_then(|x| x.first()).and_then(Value::as_str).map(str::to_string).unwrap_or_default();
                            s.run("setAttr", json!({"args": [format!("{dup}.visibility"), false]}))?;
                            let ds = shape_of_curve(s, &dup).unwrap_or(dup);
                            connect(s, format!("{ds}.worldSpace"), format!("{node}.referenceWires[{k}]"))?;
                        }
                    }
                }
                if ty == "xgmModifierCollide" {
                    let meshes: Vec<String> = a.get("meshes").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
                    for (k, m) in meshes.iter().enumerate() {
                        let Some(ms) = mesh_shape(s, m) else { fail!("xjenAddModifier: {m} is not a mesh") };
                        connect(s, format!("{ms}.worldMesh[0]"), format!("{node}.collisionMesh[{k}]"))?;
                    }
                }
                if ty == "xgmCurveToSpline" {
                    if let Some(m) = super::bound_mesh(&s.graph, desc) {
                        let mp = s.graph.path(m);
                        connect(s, format!("{mp}.worldMesh[0]"), format!("{node}.boundMesh"))?;
                    }
                }
                if ty == "xgmModifierSculpt" {
                    let id = s.graph.find(&node)?.unwrap_or(NO_NODE);
                    let data = sculpt::SculptData { layers: vec![sculpt::Layer { name: "Base".into(), ..Default::default() }], ..Default::default() };
                    if let Some(p) = crate::harold::network::attr(&s.graph, id, "sculptData") {
                        s.graph.set_value(p, PlugValue::Data(Some(std::sync::Arc::new(data))))?;
                    }
                    set(s, format!("{node}.layer[0].layerName"), json!("Base"))?;
                }
                Ok(json!(node))
            },
        )
        .positional("description")
        .flag(FlagDef::new("type", "typ", ArgType::String, Value::Null, "The modifier.").choices(&["clump", "noise", "cut", "collide", "displace", "guide", "linearWire", "curveToSpline", "scale", "sculpt", "splineCache"]))
        .flag(FlagDef::new("after", "aft", ArgType::String, Value::Null, "A node of the stack to put it just above."))
        .flag(FlagDef::new("curves", "crv", ArgType::List, Value::Null, "Guide, Linear Wire, Curve to Spline: the curves."))
        .flag(FlagDef::new("fraction", "fr", ArgType::Float, Value::Null, "Guide, Linear Wire with no curves: the part of the splines made into curves; 0.1 by default.").min(0.0).max(1.0))
        .flag(FlagDef::new("meshes", "msh", ArgType::List, Value::Null, "Collide: the meshes.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenGroomCtx",
            "The Interactive Groom tools' settings: -tool (density, place, length, cut, width, twist, comb, grab, smooth, noise, \
             clump, part, freeze, select), -size (pixels), -strength, -collide, -collisionDistance, -invertFrozen, -altMode \
             (the tool's other mode, N in the views). -q with a flag asks it. -clearSelection forgets the splines picked.",
            |_s, a| {
                let mut c = groom::ctx().lock().unwrap();
                if a.bool("query") {
                    let q: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
                    let asked = |n: &str| a.bool(n) || q.iter().any(|x| x == n);
                    return Ok(if asked("tool") {
                        json!(c.tool)
                    } else if asked("size") {
                        json!(c.size)
                    } else if asked("strength") {
                        json!(c.strength)
                    } else if asked("collide") {
                        json!(c.collide)
                    } else if asked("collisionDistance") {
                        json!(c.collision_distance)
                    } else if asked("invertFrozen") {
                        json!(c.invert_frozen)
                    } else if asked("altMode") {
                        json!(c.alt_mode)
                    } else {
                        json!({"tool": c.tool, "size": c.size, "strength": c.strength, "collide": c.collide, "collisionDistance": c.collision_distance, "invertFrozen": c.invert_frozen, "altMode": c.alt_mode})
                    });
                }
                if let Some(t) = a.get("tool").and_then(Value::as_str) {
                    if !groom::TOOLS.contains(&t) {
                        fail!("xjenGroomCtx: no tool {t}; one of {}", groom::TOOLS.join(", "));
                    }
                    c.tool = t.to_string();
                }
                if a.has("size") {
                    c.size = a.float("size", 60.0).max(1.0);
                }
                if a.has("strength") {
                    c.strength = a.float("strength", 0.5).clamp(0.0, 1.0);
                }
                if a.has("collide") {
                    c.collide = a.bool("collide");
                }
                if a.has("collisionDistance") {
                    c.collision_distance = a.float("collisionDistance", 0.1).max(0.0);
                }
                if a.has("invertFrozen") {
                    c.invert_frozen = a.bool("invertFrozen");
                }
                if a.has("altMode") {
                    c.alt_mode = a.bool("altMode");
                }
                if a.bool("clearSelection") {
                    c.selection.clear();
                }
                Ok(Value::Null)
            },
        )
        .flag(FlagDef::new("tool", "t", ArgType::String, Value::Null, "The tool.").choices(groom::TOOLS))
        .flag(FlagDef::new("size", "sz", ArgType::Float, Value::Null, "Brush size, pixels on screen.").min(1.0))
        .flag(FlagDef::new("strength", "st", ArgType::Float, Value::Null, "0 to 1.").min(0.0).max(1.0))
        .flag(FlagDef::new("collide", "col", ArgType::Bool, Value::Null, "Keep splines out of the bound mesh."))
        .flag(FlagDef::new("collisionDistance", "cd", ArgType::Float, Value::Null, "How far above the surface they are kept, scene units.").min(0.0))
        .flag(FlagDef::new("invertFrozen", "ivf", ArgType::Bool, Value::Null, "Work on frozen splines only."))
        .flag(FlagDef::new("altMode", "alt", ArgType::Bool, Value::Null, "The tool's other mode."))
        .flag(FlagDef::new("clearSelection", "cs", ArgType::Bool, Value::Null, "Forget the splines picked with Select."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Ask."))
        .undoable(false),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenGroomStroke",
            "A stroke of the current grooming tool on a description, from a script: -points the brush's centres in world \
             space, in order, -radius its size in scene units, -normal the surface's (up by default), -invert as Ctrl. \
             Undoes as one step.",
            |s, a| {
                let names = a.named();
                let Some(desc) = names.first().and_then(|n| description(s, n)) else { fail!("xjenGroomStroke: name a description") };
                // [[x, y, z], ...], or flat as MEL gives it: {x, y, z, x, y, z, ...}.
                let raw: Vec<Value> = a.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
                let flat: Vec<f64> = raw.iter().flat_map(|p| match p {
                    Value::Array(v) => v.iter().filter_map(Value::as_f64).collect::<Vec<_>>(),
                    other => other.as_f64().into_iter().collect(),
                }).collect();
                let pts: Vec<DVec3> = flat.chunks_exact(3).map(|c| DVec3::new(c[0], c[1], c[2])).collect();
                if pts.is_empty() {
                    fail!("xjenGroomStroke: -points needs at least one point");
                }
                let n = a.vec3("normal").map(DVec3::from_array).unwrap_or(DVec3::Y);
                let r = a.float("radius", 1.0).max(1e-6);
                let Some(mut st) = groom::Stroke::begin(s, desc, a.bool("invert"), false) else { fail!("xjenGroomStroke: the description has no Sculpt modifier") };
                for p in pts {
                    st.dab(s, p, n, r);
                }
                Ok(json!(st.changed()))
            },
        )
        .positional("description")
        .flag(FlagDef::new("points", "p", ArgType::List, Value::Null, "Brush centres, world space: [[x, y, z], ...], or x, y, z after one another."))
        .flag(FlagDef::new("radius", "r", ArgType::Float, Value::Null, "Brush radius, scene units; 1 by default.").min(0.0))
        .flag(FlagDef::new("normal", "nr", ArgType::Vec3, Value::Null, "The surface normal under the brush."))
        .flag(FlagDef::new("invert", "inv", ArgType::Bool, Value::Null, "The tool inverted, as with Ctrl held.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenSculptLayer",
            "A description's sculpt layers: -add a new layer (made active), -remove one, -active the one the tools work on, \
             -merge some into the first of them, -mergeVisible all visible ones into the first, -list them as \
             [name, weight, visible].",
            |s, a| {
                let names = a.named();
                let Some(desc) = names.first().and_then(|n| description(s, n)) else { fail!("xjenSculptLayer: name a description") };
                let Some(sc) = sculpt_of(s, desc) else { fail!("xjenSculptLayer: the description has no Sculpt modifier") };
                let scn = s.graph.path(sc);
                let before = groom::layers(&s.graph, sc);
                if a.bool("list") {
                    return Ok(json!(before.iter().map(|(n, w, v)| json!([n, w, v])).collect::<Vec<_>>()));
                }
                let read = |s: &Session| {
                    let n = s.graph.node(sc).unwrap();
                    sculpt::sculpt_data(n.data().value(n.ty().attr_index("sculptData").unwrap())).cloned().unwrap_or_default()
                };
                if let Some(name) = a.get("add").and_then(Value::as_str) {
                    let mut d = read(s);
                    d.layers.push(sculpt::Layer { name: name.to_string(), ..Default::default() });
                    let k = d.layers.len() - 1;
                    set_sculpt(s, sc, d)?;
                    set(s, format!("{scn}.layer[{k}].layerName"), json!(name))?;
                    set(s, format!("{scn}.activeLayer"), json!(k))?;
                    return Ok(json!(k));
                }
                if a.has("active") {
                    let k = a.int("active", 0);
                    if k < 0 || k as usize >= before.len() {
                        fail!("xjenSculptLayer: no layer {k}");
                    }
                    set(s, format!("{scn}.activeLayer"), json!(k))?;
                    return Ok(json!(k));
                }
                if a.has("remove") {
                    let k = a.int("remove", -1);
                    let mut d = read(s);
                    if k < 0 || k as usize >= d.layers.len() || d.layers.len() == 1 {
                        fail!("xjenSculptLayer: can't remove layer {k}");
                    }
                    d.layers.remove(k as usize);
                    let mut keep = before.clone();
                    keep.remove(k as usize);
                    set_sculpt(s, sc, d)?;
                    layer_attrs(s, &scn, &keep, before.len())?;
                    set(s, format!("{scn}.activeLayer"), json!(0))?;
                    return Ok(Value::Null);
                }
                let which: Option<Vec<usize>> = if a.bool("mergeVisible") {
                    Some((0..before.len()).filter(|&k| before[k].2).collect())
                } else {
                    a.get("merge").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_u64).map(|x| x as usize).collect())
                };
                if let Some(which) = which {
                    let Some(d) = groom::merged(&s.graph, sc, &which) else { fail!("xjenSculptLayer: nothing to merge") };
                    let mut w: Vec<usize> = which.clone();
                    w.sort_unstable();
                    w.dedup();
                    let first = w[0];
                    let keep: Vec<(String, f64, bool)> = before.iter().enumerate().filter(|(k, _)| *k == first || !w.contains(k)).map(|(k, x)| if k == first { (x.0.clone(), 1.0, true) } else { x.clone() }).collect();
                    set_sculpt(s, sc, d)?;
                    layer_attrs(s, &scn, &keep, before.len())?;
                    set(s, format!("{scn}.activeLayer"), json!(keep.iter().position(|x| x.0 == before[first].0).unwrap_or(0)))?;
                    return Ok(Value::Null);
                }
                fail!("xjenSculptLayer: give -add, -remove, -active, -merge, -mergeVisible or -list")
            },
        )
        .positional("description")
        .flag(FlagDef::new("add", "a", ArgType::String, Value::Null, "A new layer of this name."))
        .flag(FlagDef::new("remove", "rm", ArgType::Int, Value::Null, "Removes the layer at this index."))
        .flag(FlagDef::new("active", "ac", ArgType::Int, Value::Null, "Makes the layer at this index the one the tools work on."))
        .flag(FlagDef::new("merge", "m", ArgType::List, Value::Null, "Merges these layers into the first of them."))
        .flag(FlagDef::new("mergeVisible", "mv", ArgType::Bool, Value::Null, "Merges the visible layers into the first of them."))
        .flag(FlagDef::new("list", "ls", ArgType::Bool, Value::Null, "The layers: [name, weight, visible] each.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xgmSplineSelect",
            "Spline selection on descriptions: -replaceBySelectedFaces picks the splines rooted on the faces selected, \
             -convertToFreeze freezes the splines picked and clears the pick. Applies to the descriptions named, or else \
             every description.",
            |s, a| {
                let mut ds = named_descriptions(s, &a);
                if ds.is_empty() {
                    ds = descriptions(&s.graph);
                }
                s.graph.evaluate();
                if a.bool("replaceBySelectedFaces") {
                    let sel = s.run("ls", json!({"selection": true, "flatten": true}))?;
                    let picks: Vec<String> = sel.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
                    let mut total = 0;
                    for d in &ds {
                        let Some(base) = super::base_of(&s.graph, *d) else { continue };
                        let src = crate::harold::network::attr(&s.graph, base, "boundMesh").and_then(|p| s.graph.source(&p));
                        let Some(src) = src else { continue };
                        let mut faces = std::collections::HashSet::new();
                        for t in targets(s, &picks) {
                            if s.graph.find(&t.shape).ok().flatten() == Some(src.node) {
                                faces.extend(t.faces);
                            }
                        }
                        let ids = groom::splines_on_faces(&s.graph, *d, &faces);
                        total += ids.len();
                        groom::ctx().lock().unwrap().selection.insert(*d, ids);
                    }
                    return Ok(json!(total));
                }
                if a.bool("convertToFreeze") {
                    let mut total = 0;
                    for d in &ds {
                        total += groom::freeze_selected(s, *d);
                        groom::ctx().lock().unwrap().selection.remove(d);
                    }
                    return Ok(json!(total));
                }
                fail!("xgmSplineSelect: give -replaceBySelectedFaces or -convertToFreeze")
            },
        )
        .positional("descriptions...")
        .flag(FlagDef::new("replaceBySelectedFaces", "rsf", ArgType::Bool, Value::Null, "Pick the splines on the selected faces."))
        .flag(FlagDef::new("convertToFreeze", "ctf", ArgType::Bool, Value::Null, "Freeze the picked splines.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenCreateGroom",
            "Create Interactive Groom Splines: a description on each mesh named or selected (or on the faces named), \
             grown by density and seed, with a Scale and a Sculpt modifier and a default aiStandardHair shader. Returns \
             the descriptions' transforms. The reference's own command for this is not documented; this is Aztek's.",
            |s, a| {
                let mut names = a.named();
                if names.is_empty() {
                    let sel = s.run("ls", json!({"selection": true, "flatten": true}))?;
                    names = sel.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
                }
                let ts = targets(s, &names);
                if ts.is_empty() {
                    fail!("xjenCreateGroom: select a mesh, or faces on one");
                }
                let mut made = Vec::new();
                for t in &ts {
                    made.push(make(s, t, &a)?);
                }
                s.run("select", json!({"args": made.clone(), "replace": true}))?;
                Ok(json!(made))
            },
        )
        .positional("meshes or faces...")
        .flag(FlagDef::new("name", "n", ArgType::String, Value::Null, "New Description Name; taken with its next free number if in use."))
        .flag(FlagDef::new("seed", "sd", ArgType::Int, Value::Null, "Generator Seed."))
        .flag(FlagDef::new("density", "den", ArgType::Float, Value::Null, "Splines per face; 10 by default.").min(0.0))
        .flag(FlagDef::new("length", "len", ArgType::Float, Value::Null, "Length of the splines, in scene units; 1 by default.").min(0.0))
        .flag(FlagDef::new("widthScale", "ws", ArgType::Float, Value::Null, "Width of the splines, in scene units; 0.01 by default.").min(0.0))
        .flag(FlagDef::new("cvCount", "cvc", ArgType::Int, Value::Null, "Points on each spline; 5 by default.").min(2.0)),
    );
    c.register(
        CommandDef::<Session>::new(
            "xgmSplineQuery",
            "Asks about Interactive Groom descriptions: -listSplineDescriptions lists them, -isSplineDescription says whether \
             the object named is one (its transform or shape), -splineCount counts a description's splines.",
            |s, a| {
                s.graph.evaluate();
                let shape_of = |s: &Session, n: &str| -> Option<NodeId> {
                    let id = s.graph.find(n).ok().flatten()?;
                    if s.graph.node(id)?.ty().is_a(DESCRIPTION) {
                        return Some(id);
                    }
                    s.graph.children(id).iter().copied().find(|&k| s.graph.node(k).is_some_and(|x| x.ty().is_a(DESCRIPTION)))
                };
                if a.bool("listSplineDescriptions") {
                    let long = a.bool("long");
                    let v: Vec<String> = descriptions(&s.graph)
                        .into_iter()
                        .map(|id| {
                            let tr = s.graph.parent(id);
                            if long { s.graph.path(tr) } else { s.graph.node(tr).map(|n| n.name().to_string()).unwrap_or_default() }
                        })
                        .collect();
                    return Ok(json!(v));
                }
                let names = a.named();
                let Some(n) = names.first() else { fail!("xgmSplineQuery: name a description") };
                if a.bool("isSplineDescription") {
                    return Ok(json!(shape_of(s, n).is_some()));
                }
                if a.bool("splineCount") {
                    let Some(id) = shape_of(s, n) else { fail!("xgmSplineQuery: {n} is not a spline description") };
                    return Ok(json!(render_splines(&s.graph, id).map_or(0, |d| d.splines.len())));
                }
                fail!("xgmSplineQuery: give -listSplineDescriptions, -isSplineDescription or -splineCount")
            },
        )
        .positional("description")
        .flag(FlagDef::new("listSplineDescriptions", "lsd", ArgType::Bool, Value::Null, "Every description in the scene."))
        .flag(FlagDef::new("long", "l", ArgType::Bool, Value::Null, "With -listSplineDescriptions: full paths."))
        .flag(FlagDef::new("isSplineDescription", "isd", ArgType::Bool, Value::Null, "Whether the object named is a description."))
        .flag(FlagDef::new("splineCount", "sc", ArgType::Bool, Value::Null, "How many splines the description has."))
        .undoable(false),
    );
}
