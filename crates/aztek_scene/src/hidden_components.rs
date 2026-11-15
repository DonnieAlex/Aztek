//! Hidden polygon components, as the reference hides them: Display > Hide > Hide Selection (H, Ctrl+H) in a
//! component mode hides the selected faces, with selected edges and vertices taken as the faces they touch. Every
//! view leaves hidden faces out, with the edges and vertices only they use, and component tools skip them as they skip
//! what isolation leaves out. Shift+H shows the selected components; H with nothing selected, or with hidden
//! components among the selection, and Ctrl+Alt+H show every hidden component of the selected objects.
//! Undoable. Kept per mesh shape by face id; not saved with the scene.

use std::collections::BTreeMap;

use aztek_undo::FnAction;

use crate::components::{self, ComponentMode, Ids};
use crate::graph::Graph;
use crate::isolation::faces_of;
use crate::modifiers;
use crate::plug::NodeId;

pub type FaceMap = BTreeMap<NodeId, Ids>;

/// The node's hidden faces, sorted.
pub fn hidden(g: &Graph, node: NodeId) -> &[u32] {
    g.hidden_faces.get(&node).map(Vec::as_slice).unwrap_or(&[])
}

fn put(g: &mut Graph, map: FaceMap) {
    g.hidden_faces = map;
    g.hidden_faces_version += 1;
    g.components.notify();
}

/// Replaces the hidden faces as one undo step.
fn set_undoable(g: &mut Graph, after: FaceMap) {
    let before = g.hidden_faces.clone();
    if before == after {
        return;
    }
    let (b, a) = (before, after.clone());
    g.undo_stack_mut().record(FnAction::new(move |g: &mut Graph| put(g, b.clone()), move |g: &mut Graph| put(g, a.clone())));
    put(g, after);
}

fn shape_of(g: &Graph, node: NodeId) -> NodeId {
    components::as_mesh(g, node).unwrap_or(node)
}

/// The faces components stand for, by mesh shape: named components, or with none named, the component selection of
/// the current mode. None when neither is about components (objects named, or object mode).
pub fn picked_faces(g: &mut Graph, names: &[String]) -> Result<Option<FaceMap>, String> {
    let named: Vec<String> = names.iter().filter(|n| components::is_component_string(n)).cloned().collect();
    let mut out = FaceMap::new();
    if !named.is_empty() {
        for m in crate::object_sets::parse(g, &named)? {
            if m.kind == ComponentMode::Object {
                continue;
            }
            let node = shape_of(g, m.node);
            if let Some(mesh) = modifiers::cage(g, node) {
                let f = faces_of(&mesh, m.kind, &m.ids);
                out.entry(node).or_default().extend(f);
            }
        }
    } else if names.is_empty() && components::mode(g) != ComponentMode::Object {
        let mode = components::mode(g);
        for (node, ids) in components::selected_all(g, mode).clone() {
            let node = shape_of(g, node);
            if let Some(mesh) = modifiers::cage(g, node) {
                let f = faces_of(&mesh, mode, &ids);
                if !f.is_empty() {
                    out.entry(node).or_default().extend(f);
                }
            }
        }
    } else {
        return Ok(None);
    }
    for ids in out.values_mut() {
        ids.sort_unstable();
        ids.dedup();
    }
    Ok(Some(out))
}

/// Hides the faces.
pub fn hide(g: &mut Graph, faces: &FaceMap) {
    let mut after = g.hidden_faces.clone();
    for (node, ids) in faces {
        let h = after.entry(*node).or_default();
        h.extend(ids);
        h.sort_unstable();
        h.dedup();
    }
    after.retain(|_, v| !v.is_empty());
    set_undoable(g, after);
}

/// Shows the faces.
pub fn show(g: &mut Graph, faces: &FaceMap) {
    let mut after = g.hidden_faces.clone();
    for (node, ids) in faces {
        if let Some(h) = after.get_mut(node) {
            h.retain(|f| ids.binary_search(f).is_err());
        }
    }
    after.retain(|_, v| !v.is_empty());
    set_undoable(g, after);
}

/// Shows every hidden face of the nodes (a transform stands for its mesh shape).
pub fn show_all(g: &mut Graph, nodes: &[NodeId]) {
    let mut after = g.hidden_faces.clone();
    for &n in nodes {
        after.remove(&shape_of(g, n));
        after.remove(&n);
    }
    set_undoable(g, after);
}

/// The selected objects, or the objects the components belong to.
fn owners(g: &Graph, faces: &FaceMap) -> Vec<NodeId> {
    let mut v: Vec<NodeId> = g.selection().to_vec();
    v.extend(faces.keys().copied());
    v
}

/// The H key in a component mode: hides the faces when every one of them shows; with none, or with hidden ones
/// among them, shows every hidden face of the objects. Returns whether it hid.
pub fn toggle(g: &mut Graph, faces: &FaceMap) -> bool {
    let any = faces.values().any(|v| !v.is_empty());
    let all_shown = faces.iter().all(|(n, ids)| {
        let h = hidden(g, *n);
        ids.iter().all(|f| h.binary_search(f).is_err())
    });
    if any && all_shown {
        hide(g, faces);
        true
    } else {
        let nodes = owners(g, faces);
        show_all(g, &nodes);
        false
    }
}

/// Shift+H in a component mode: shows the selected components, or with none selected every hidden face of the
/// selected objects.
pub fn show_picked(g: &mut Graph, faces: &FaceMap) {
    if faces.values().any(|v| !v.is_empty()) {
        show(g, faces);
    } else {
        let nodes = owners(g, faces);
        show_all(g, &nodes);
    }
}

/// The mesh without its hidden faces, and the edges and points only they used. Ids stay as they are.
pub fn masked(mesh: &aztek_geo::Mesh, hidden: &[u32]) -> aztek_geo::Mesh {
    let mut m = mesh.clone();
    let faces: Vec<u32> = hidden.iter().copied().filter(|&f| f < m.face_capacity() && m.face_valid(f)).collect();
    aztek_geo::ops::common::remove_faces_clean(&mut m, &faces);
    m
}
