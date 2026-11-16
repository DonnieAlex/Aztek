//! XJEN's commands: making an Interactive Groom description, and xgmSplineQuery.

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use super::{descriptions, render_splines, sculpt, BASE, DESCRIPTION, SCALE, SCULPT};
use crate::plug::{NodeId, PlugValue, NO_NODE};
use crate::session::Session;

/// The commands this file registers.
pub const NAMES: &[&str] = &["xjenCreateGroom", "xgmSplineQuery"];

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
    let data = sculpt::SculptData { layers: vec![sculpt::Layer { name: "Base".into(), ..Default::default() }] };
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

pub fn register(c: &mut Commands<Session>) {
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
