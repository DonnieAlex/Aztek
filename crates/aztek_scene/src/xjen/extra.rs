//! The rest of the Interactive Groom menu: what the editor shows (xjenGroomInfo), Convert Interactive Groom to
//! Polygons, presets (export and import), and Transfer to another mesh.

use std::sync::Arc;

use aztek_geo::Mesh;
use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::DVec3;
use serde_json::{json, Map, Value};

use super::cmds::description;
use super::{descriptions, render_splines, sculpt, stack, BASE, DESCRIPTION, SCULPT};
use crate::plug::{flags, NodeId, PlugRef, PlugType, PlugValue};
use crate::session::Session;

pub const NAMES: &[&str] = &["xjenGroomInfo", "xjenConvertToPolygons", "xjenPreset", "xjenTransfer", "xjenRemoveModifier"];

/// Ribbons (two points across at each spline point, each the spline's width) or tubes (sides around), with UVs: U
/// across or around, V root to tip.
pub fn polygons(splines: &super::SplineData, tube: bool, sides: usize) -> Mesh {
    let (mut pts, mut sizes, mut idx, mut uvs) = (Vec::new(), Vec::new(), Vec::new(), Vec::<[f32; 2]>::new());
    let around = if tube { sides.max(3) } else { 2 };
    for sp in &splines.splines {
        let n = sp.points.len();
        if n < 2 {
            continue;
        }
        let base = pts.len() as u32;
        for j in 0..n {
            let dir = if j + 1 < n { sp.points[j + 1] - sp.points[j] } else { sp.points[j] - sp.points[j - 1] }.normalize_or_zero();
            let side = dir.cross(sp.normal).normalize_or_zero();
            let side = if side == DVec3::ZERO { sp.tangent } else { side };
            let up = side.cross(dir).normalize_or_zero();
            let r = sp.width.get(j).copied().unwrap_or(0.01) as f64 * 0.5;
            for k in 0..around {
                let off = if tube {
                    let a = k as f64 / around as f64 * std::f64::consts::TAU;
                    (side * a.cos() + up * a.sin()) * r
                } else {
                    side * if k == 0 { -r } else { r }
                };
                pts.push(sp.points[j] + off);
            }
        }
        let rings = if tube { around } else { 1 };
        for j in 0..n - 1 {
            for k in 0..rings {
                let k1 = (k + 1) % around;
                let a = base + (j * around + k) as u32;
                let b = base + (j * around + k1) as u32;
                let c = base + ((j + 1) * around + k1) as u32;
                let d = base + ((j + 1) * around + k) as u32;
                sizes.push(4);
                idx.extend([a, b, c, d]);
                let (u0, u1) = if tube { (k as f32 / around as f32, (k + 1) as f32 / around as f32) } else { (0.0, 1.0) };
                let (v0, v1) = (j as f32 / (n - 1) as f32, (j + 1) as f32 / (n - 1) as f32);
                uvs.extend([[u0, v0], [u1, v0], [u1, v1], [u0, v1]]);
            }
        }
    }
    let mut m = Mesh::from_polygons(&pts, &sizes, &idx);
    let uv = m.corner_attrs_mut().get_or_add::<[f32; 2]>(&aztek_geo::names::uv(0));
    for (dst, src) in uv.iter_mut().zip(&uvs) {
        *dst = *src;
    }
    m
}

/// A node's stored plain values, for a preset: numbers, switches, words and colours, by attribute name.
fn values_of(s: &Session, id: NodeId) -> Map<String, Value> {
    let mut out = Map::new();
    let Some(n) = s.graph.node(id) else { return out };
    let t = n.ty();
    for (i, a) in t.attrs.iter().enumerate() {
        if !a.has(flags::STORABLE) || a.has(flags::ARRAY) || a.name.contains('.') || matches!(a.name.as_str(), "visibility" | "intermediateObject") {
            continue;
        }
        let v = n.data().value(i);
        let j = match (a.ty, v) {
            (PlugType::Float, PlugValue::Float(x)) => json!(x),
            (PlugType::Int | PlugType::Enum, PlugValue::Int(x)) => json!(x),
            (PlugType::Bool, PlugValue::Bool(x)) => json!(x),
            (PlugType::String, PlugValue::String(x)) => json!(x),
            (PlugType::Vec3, PlugValue::Vec3(x)) => json!(x.to_array()),
            (PlugType::Data, _) => match crate::scripted::plugin_data_text(v) {
                Some((kind, text)) => json!({"kind": kind, "text": text}),
                None => continue,
            },
            _ => continue,
        };
        if *v == a.default_value {
            continue;
        }
        out.insert(a.name.clone(), j);
    }
    // Sculpt layers' names, weights and visibility.
    if t.is_a(SCULPT) {
        out.insert("layers".into(), json!(super::groom::layers(&s.graph, id).iter().map(|(n, w, v)| json!([n, w, v])).collect::<Vec<_>>()));
    }
    out
}

fn apply_values(s: &mut Session, node: &str, vals: &Map<String, Value>) -> CmdResult<()> {
    let id = s.graph.find(node)?.unwrap_or(0);
    for (k, v) in vals {
        if k == "layers" {
            for (i, l) in v.as_array().into_iter().flatten().enumerate() {
                s.run("setAttr", json!({"args": [format!("{node}.layer[{i}].layerName"), l[0]]}))?;
                s.run("setAttr", json!({"args": [format!("{node}.layer[{i}].layerWeight"), l[1]]}))?;
                s.run("setAttr", json!({"args": [format!("{node}.layer[{i}].layerVisibility"), l[2]]}))?;
            }
            continue;
        }
        if let (Some(kind), Some(text)) = (v.get("kind").and_then(Value::as_str), v.get("text").and_then(Value::as_str)) {
            let Some(load) = crate::plug::data::find(kind) else { continue };
            let d = load(&json!({"kind": kind, "pluginData": true, "text": text}))?;
            if let Some(p) = crate::harold::network::attr(&s.graph, id, k) {
                s.graph.set_value(p, PlugValue::Data(Some(d)))?;
            }
            continue;
        }
        let mut args = vec![json!(format!("{node}.{k}"))];
        match v {
            Value::Array(a) => args.extend(a.iter().cloned()),
            other => args.push(other.clone()),
        }
        if let Value::String(_) = v {
            s.run("setAttr", json!({"args": args, "type": "string"}))?;
        } else {
            s.run("setAttr", json!({"args": args}))?;
        }
    }
    Ok(())
}

/// A description as a preset: the shape's and every stack node's settings, sculpt layers and edits included.
pub fn preset(s: &Session, desc: NodeId) -> Value {
    let st: Vec<Value> = stack(&s.graph, desc).into_iter().map(|n| json!({"type": s.graph.node(n).map(|x| x.ty().name.clone()).unwrap_or_default(), "values": values_of(s, n)})).collect();
    json!({"format": "Aztek XJEN preset", "version": 1, "description": values_of(s, desc), "stack": st})
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "xjenRemoveModifier",
            "Takes a modifier out of its description's stack, joining what fed it to what it fed, and deletes it. The base \
             cannot be removed.",
            |s, a| {
                let names = a.named();
                let Some(n) = names.first() else { fail!("xjenRemoveModifier: name a modifier") };
                let id = s.graph.find(n)?.ok_or_else(|| aztek_ops::CommandError::new(format!("xjenRemoveModifier: no {n}")))?;
                if s.graph.node(id).is_some_and(|x| x.ty().is_a(BASE)) {
                    fail!("xjenRemoveModifier: {n} is the base");
                }
                let (Some(inp), Some(out)) = (crate::harold::network::attr(&s.graph, id, "inSplineData"), crate::harold::network::attr(&s.graph, id, "outSplineData")) else {
                    fail!("xjenRemoveModifier: {n} is not a modifier")
                };
                let feeder = s.graph.source(&inp).map(|p| s.graph.path(p.node));
                let consumers: Vec<String> = s.graph.destinations(&out).into_iter().map(|p| s.graph.path(p.node)).collect();
                let path = s.graph.path(id);
                s.run("delete", json!({"args": [path]}))?;
                if let Some(f) = feeder {
                    for c in consumers {
                        s.run("connectAttr", json!({"args": [format!("{f}.outSplineData"), format!("{c}.inSplineData")], "force": true}))?;
                    }
                }
                Ok(Value::Null)
            },
        )
        .positional("modifier"),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenGroomInfo",
            "Every Interactive Groom description, for the editor: its transform and shape, its stack from the top (shape side) \\
             down, each node's type and Mute, and its Sculpt modifier's layers with the active one.",
            |s, _a| {
                s.graph.evaluate();
                let g = &s.graph;
                let mut out = Vec::new();
                for d in descriptions(g) {
                    let tr = g.parent(d);
                    let mut st: Vec<Value> = stack(g, d)
                        .into_iter()
                        .map(|n| {
                            let node = g.node(n).unwrap();
                            let mute = node.ty().attr_index("mute").map(|i| node.data().value(i).as_bool()).unwrap_or(false);
                            json!({"node": g.path(n), "type": node.ty().name, "mute": mute})
                        })
                        .collect();
                    st.reverse();
                    let sculpts: Vec<Value> = stack(g, d)
                        .into_iter()
                        .filter(|&n| g.node(n).is_some_and(|x| x.ty().is_a(SCULPT)))
                        .map(|n| {
                            let node = g.node(n).unwrap();
                            let active = node.ty().attr_index("activeLayer").map(|i| node.data().value(i).as_int()).unwrap_or(0);
                            json!({"node": g.path(n), "active": active, "layers": super::groom::layers(g, n).iter().map(|(name, w, v)| json!({"name": name, "weight": w, "visible": v})).collect::<Vec<_>>()})
                        })
                        .collect();
                    out.push(json!({
                        "transform": g.path(tr),
                        "shape": g.path(d),
                        "splines": render_splines(g, d).map_or(0, |x| x.splines.len()),
                        "mesh": super::bound_mesh(g, d).map(|m| g.path(m)),
                        "stack": st,
                        "sculpt": sculpts,
                    }));
                }
                Ok(json!(out))
            },
        )
        .undoable(false),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenConvertToPolygons",
            "Convert Interactive Groom to Polygons: each description named (or selected) as a mesh of ribbons, two points \\
             across each spline point (-tube makes tubes of -sides sides), each its spline's width, with UVs. Returns \\
             the meshes.",
            |s, a| {
                let names = a.named();
                let mut ds: Vec<NodeId> = names.iter().filter_map(|n| description(s, n)).collect();
                if ds.is_empty() {
                    ds = s.graph.selection().iter().filter_map(|&id| description(s, &s.graph.path(id))).collect();
                }
                if ds.is_empty() {
                    fail!("xjenConvertToPolygons: name or select a description");
                }
                s.graph.evaluate();
                let tube = a.bool("tube");
                let sides = a.int("sides", 4).clamp(3, 32) as usize;
                let mut made = Vec::new();
                for d in ds {
                    let Some(data) = render_splines(&s.graph, d).cloned() else { continue };
                    let world = s.graph.node(d).map(|n| n.data().matrix(crate::transform::DagNode::WORLD_MATRIX)).unwrap_or_default();
                    let mut m = polygons(&data, tube, sides);
                    for p in 0..m.point_capacity() {
                        if m.point_valid(p) {
                            let q = world.transform_point3(m.position(p));
                            m.set_position(p, q);
                        }
                    }
                    let tr_name = s.graph.node(s.graph.parent(d)).map(|n| format!("{}_mesh", n.name())).unwrap_or_default();
                    let (xf, n) = crate::mesh_nodes::create_mesh_node(&mut s.graph, &tr_name)?;
                    s.graph.set_value(PlugRef::new(n, crate::mesh_nodes::MeshNode::IN_MESH as i32), PlugValue::Mesh(Some(Arc::new(m))))?;
                    made.push(s.graph.path(xf));
                }
                s.run("select", json!({"args": made.clone(), "replace": true}))?;
                Ok(json!(made))
            },
        )
        .positional("descriptions...")
        .flag(FlagDef::new("tube", "tb", ArgType::Bool, Value::Null, "Tubes rather than ribbons."))
        .flag(FlagDef::new("sides", "sd", ArgType::Int, Value::Null, "Sides of a tube; 4 by default.").min(3.0).max(32.0)),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenPreset",
            "Description presets, in Aztek's own JSON: -export writes the description named to a file (its shape's and every \\
             modifier's settings, sculpt layers and the splines placed or taken away); -import makes a description from one \\
             on the mesh named or selected. Guide and wire curves are not kept in a preset.",
            |s, a| {
                if let Some(path) = a.get("export").and_then(Value::as_str) {
                    let names = a.named();
                    let Some(d) = names.first().and_then(|n| description(s, n)) else { fail!("xjenPreset: name a description to export") };
                    let text = serde_json::to_string_pretty(&preset(s, d)).unwrap_or_default();
                    std::fs::write(path, text).map_err(|e| aztek_ops::CommandError::new(format!("xjenPreset: {path}: {e}")))?;
                    return Ok(json!(path));
                }
                let Some(path) = a.get("import").and_then(Value::as_str).map(str::to_string) else { fail!("xjenPreset: give -export or -import") };
                let text = std::fs::read_to_string(&path).map_err(|e| aztek_ops::CommandError::new(format!("xjenPreset: {path}: {e}")))?;
                let p: Value = serde_json::from_str(&text).map_err(|e| aztek_ops::CommandError::new(format!("xjenPreset: {path}: {e}")))?;
                let mut call = json!({});
                let names = a.named();
                if !names.is_empty() {
                    call["args"] = json!(names);
                }
                if let Some(n) = a.get("name") {
                    call["name"] = n.clone();
                }
                let made = s.run("xjenCreateGroom", call)?;
                let tr = made[0].as_str().unwrap_or_default().to_string();
                let Some(d) = description(s, &tr) else { fail!("xjenPreset: no description made") };
                // The stack as the preset has it: the new base kept, the rest replaced.
                let st = stack(&s.graph, d);
                let base = s.graph.path(st[0]);
                for &n in &st[1..] {
                    let path = s.graph.path(n);
                    s.run("delete", json!({"args": [path]}))?;
                }
                let sh = s.graph.path(d);
                s.run("connectAttr", json!({"args": [format!("{base}.outSplineData"), format!("{sh}.inSplineData")], "force": true}))?;
                let entries = p["stack"].as_array().cloned().unwrap_or_default();
                for e in &entries {
                    let ty = e["type"].as_str().unwrap_or_default();
                    let vals = e["values"].as_object().cloned().unwrap_or_default();
                    if ty == BASE {
                        apply_values(s, &base, &vals)?;
                        continue;
                    }
                    let Some(&(short, _)) = super::cmds::MODIFIER_NAMES.iter().find(|x| x.1 == ty) else { continue };
                    let node = s.run("xjenAddModifier", json!({"args": [tr.clone()], "type": short, "fraction": 0.0}))?;
                    let node = node.as_str().unwrap_or_default().to_string();
                    apply_values(s, &node, &vals)?;
                }
                if let Some(vals) = p["description"].as_object() {
                    apply_values(s, &sh, vals)?;
                }
                let _ = DESCRIPTION;
                let _ = sculpt::KIND;
                Ok(json!(tr))
            },
        )
        .positional("description or mesh")
        .flag(FlagDef::new("export", "ex", ArgType::String, Value::Null, "The file to write."))
        .flag(FlagDef::new("import", "im", ArgType::String, Value::Null, "The file to read."))
        .flag(FlagDef::new("name", "n", ArgType::String, Value::Null, "Import: the new description's name.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "xjenTransfer",
            "Transfer: the description named grows on another mesh (-mesh) from now on. Its sculpt moves and placed splines \\
             follow by face, so they carry across only where the faces match.",
            |s, a| {
                let names = a.named();
                let Some(d) = names.first().and_then(|n| description(s, n)) else { fail!("xjenTransfer: name a description") };
                let Some(target) = a.get("mesh").and_then(Value::as_str).map(str::to_string) else { fail!("xjenTransfer: give -mesh") };
                let id = s.graph.find(&target)?.ok_or_else(|| aztek_ops::CommandError::new(format!("xjenTransfer: no {target}")))?;
                let shape = if s.graph.node(id).is_some_and(|n| n.ty().is_a("mesh")) { Some(id) } else { crate::dag::shapes(&s.graph, id, false).into_iter().find(|&k| s.graph.node(k).is_some_and(|n| n.ty().is_a("mesh"))) };
                let Some(shape) = shape else { fail!("xjenTransfer: {target} is not a mesh") };
                let mp = s.graph.path(shape);
                for n in stack(&s.graph, d) {
                    if crate::harold::network::attr(&s.graph, n, "boundMesh").is_some() {
                        let np = s.graph.path(n);
                        s.run("connectAttr", json!({"args": [format!("{mp}.worldMesh[0]"), format!("{np}.boundMesh")], "force": true}))?;
                    }
                }
                Ok(Value::Null)
            },
        )
        .positional("description")
        .flag(FlagDef::new("mesh", "m", ArgType::String, Value::Null, "The mesh to grow on.")),
    );
}
