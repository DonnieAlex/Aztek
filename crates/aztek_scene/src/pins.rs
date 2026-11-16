//! Constrain > UV Pin and Proximity Pin, as the reference's UVPin and ProximityPin: the objects selected pinned to the
//! mesh selected last, through a uvPin or proximityPin node driving each object's offsetParentMatrix, the objects
//! staying where they are.

use std::sync::Arc;

use aztek_geo::Mesh;
use aztek_ops::{fail, CmdResult, CommandDef, Commands};
use glam::{DMat4, DVec2};
use serde_json::{json, Value};

use crate::mesh_nodes::MeshNode;
use crate::plug::{NodeId, PlugRef, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::DagNode;

fn mesh_shape(s: &Session, id: NodeId) -> Option<NodeId> {
    let g = &s.graph;
    if g.node(id).is_some_and(|n| n.ty().is_a("mesh")) {
        return Some(id);
    }
    crate::dag::shapes(g, id, false).into_iter().find(|&k| g.node(k).is_some_and(|n| n.ty().is_a("mesh")))
}

fn world(s: &Session, id: NodeId) -> DMat4 {
    s.graph.node(id).map(|n| n.data().matrix(DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY)
}

/// The mesh as it is, and with its points in world space.
fn meshes(s: &Session, shape: NodeId) -> Option<(Arc<Mesh>, Mesh)> {
    let local = s.graph.node(shape)?.data().value(MeshNode::OUT_MESH).as_mesh().cloned()?;
    let w = world(s, shape);
    let mut wm = (*local).clone();
    for p in 0..wm.point_capacity() {
        if wm.point_valid(p) {
            let q = w.transform_point3(wm.position(p));
            wm.set_position(p, q);
        }
    }
    Some((local, wm))
}

/// The objects and the mesh: the selection, mesh last.
fn picked(s: &mut Session, cmd: &str) -> CmdResult<(Vec<NodeId>, NodeId)> {
    s.graph.evaluate();
    let sel: Vec<NodeId> = s.graph.selection().to_vec();
    if sel.len() < 2 {
        fail!("{cmd}: select the objects to pin, then the mesh");
    }
    let last = *sel.last().unwrap();
    let Some(mesh) = mesh_shape(s, last) else { fail!("{cmd}: the last selected is not a mesh") };
    let objects: Vec<NodeId> = sel[..sel.len() - 1].iter().copied().filter(|&o| crate::dag::is_transform(&s.graph, o)).collect();
    if objects.is_empty() {
        fail!("{cmd}: select transforms to pin");
    }
    Ok((objects, mesh))
}

/// Sets an object's local matrix so that, under its parent and the pin's matrix, it stays at world.
fn keep_in_place(s: &mut Session, obj: NodeId, pin: DMat4, was: DMat4) -> CmdResult<()> {
    let parent = s.graph.parent(obj);
    let pw = if parent == NO_NODE { DMat4::IDENTITY } else { world(s, parent) };
    let local = (pw * pin).inverse() * was;
    let path = s.graph.path(obj);
    s.run("xform", json!({"args": [path], "matrix": local.to_cols_array().to_vec()}))?;
    Ok(())
}

fn uv_at(m: &Mesh, c: u32) -> DVec2 {
    let uv = m.corner_attrs().get::<[f32; 2]>(aztek_geo::names::uv(0).as_str());
    uv.get(c as usize).map_or(DVec2::ZERO, |u| DVec2::new(u[0] as f64, u[1] as f64))
}

fn output_matrix(s: &mut Session, node: &str, k: usize) -> CmdResult<DMat4> {
    let p = s.graph.plug(&format!("{node}.outputMatrix[{k}]"))?;
    Ok(s.graph.value(&p)?.as_matrix())
}

fn uv_pin(s: &mut Session) -> CmdResult<Value> {
    let (objects, mesh) = picked(s, "UVPin")?;
    let Some((local, wm)) = meshes(s, mesh) else { fail!("UVPin: the mesh has no geometry") };
    if local.corner_attrs().get::<[f32; 2]>(aztek_geo::names::uv(0).as_str()).is_empty() {
        fail!("UVPin: the mesh has no UVs");
    }
    let mp = s.graph.path(mesh);
    let node = s.run("createNode", json!({"args": ["uvPin"], "skipSelect": true}))?.as_str().unwrap_or_default().to_string();
    s.run("connectAttr", json!({"args": [format!("{mp}.worldMesh[0]"), format!("{node}.deformedGeometry")]}))?;
    let was: Vec<DMat4> = objects.iter().map(|&o| world(s, o)).collect();
    for (k, &o) in objects.iter().enumerate() {
        let at = was[k].w_axis.truncate();
        let Some((_, corners, w, _)) = crate::xjen::groom::closest(&wm, at) else { continue };
        let uv = corners.iter().zip(w).fold(DVec2::ZERO, |acc, (&c, wk)| acc + uv_at(&local, c) * wk);
        s.run("setAttr", json!({"args": [format!("{node}.coordinate[{k}].coordinateU"), uv.x]}))?;
        s.run("setAttr", json!({"args": [format!("{node}.coordinate[{k}].coordinateV"), uv.y]}))?;
        let pin = output_matrix(s, &node, k)?;
        let op = s.graph.path(o);
        s.run("connectAttr", json!({"args": [format!("{node}.outputMatrix[{k}]"), format!("{op}.offsetParentMatrix")], "force": true}))?;
        keep_in_place(s, o, pin, was[k])?;
    }
    Ok(json!(node))
}

fn proximity_pin(s: &mut Session) -> CmdResult<Value> {
    let (objects, mesh) = picked(s, "ProximityPin")?;
    let Some((_, wm)) = meshes(s, mesh) else { fail!("ProximityPin: the mesh has no geometry") };
    let mp = s.graph.path(mesh);
    // The mesh as it is now, in world space, kept as the pin's original: a hidden intermediate shape under a transform
    // of its own at the root, so that it stays put while the mesh moves.
    let stem = s.graph.node(s.graph.parent(mesh)).map(|n| n.name().to_string()).unwrap_or_default();
    let (otr, orig) = crate::mesh_nodes::create_mesh_node(&mut s.graph, &format!("{stem}PinOrig"))?;
    s.graph.set_value(PlugRef::new(orig, MeshNode::IN_MESH as i32), PlugValue::Mesh(Some(Arc::new(wm))))?;
    s.graph.set_value(PlugRef::new(orig, DagNode::INTERMEDIATE_OBJECT as i32), PlugValue::Bool(true))?;
    let otp = s.graph.path(otr);
    s.run("setAttr", json!({"args": [format!("{otp}.visibility"), false]}))?;
    let op_orig = s.graph.path(orig);
    let node = s.run("createNode", json!({"args": ["proximityPin"], "skipSelect": true}))?.as_str().unwrap_or_default().to_string();
    s.run("connectAttr", json!({"args": [format!("{mp}.worldMesh[0]"), format!("{node}.deformedGeometry")]}))?;
    s.run("connectAttr", json!({"args": [format!("{op_orig}.worldMesh[0]"), format!("{node}.originalGeometry")]}))?;
    let was: Vec<DMat4> = objects.iter().map(|&o| world(s, o)).collect();
    for (k, &o) in objects.iter().enumerate() {
        let p = s.graph.plug(&format!("{node}.inputMatrix[{k}]"))?;
        s.graph.set_value(p, PlugValue::Matrix(was[k]))?;
        let pin = output_matrix(s, &node, k)?;
        let op = s.graph.path(o);
        s.run("connectAttr", json!({"args": [format!("{node}.outputMatrix[{k}]"), format!("{op}.offsetParentMatrix")], "force": true}))?;
        keep_in_place(s, o, pin, was[k])?;
    }
    Ok(json!(node))
}

fn chunk(s: &mut Session, name: &str, f: fn(&mut Session) -> CmdResult<Value>) -> CmdResult<Value> {
    s.graph.open_chunk(name);
    let r = f(s);
    s.graph.close_chunk();
    if r.is_err() {
        s.undo();
    }
    r
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "UVPin",
            "Constrain > UV Pin: pins the selected objects to the mesh selected last at the UV nearest each, through a uvPin \
             node into their offsetParentMatrix; they stay where they are and follow the mesh as it moves and deforms.",
            |s, _a| chunk(s, "UV Pin", uv_pin),
        ),
    );
    c.register(
        CommandDef::<Session>::new(
            "ProximityPin",
            "Constrain > Proximity Pin: pins the selected objects to the nearest place on the mesh selected last, through a \
             proximityPin node (with the mesh as it is now kept, hidden, as its original) into their offsetParentMatrix.",
            |s, _a| chunk(s, "Proximity Pin", proximity_pin),
        ),
    );
}
