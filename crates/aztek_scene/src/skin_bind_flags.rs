//! The flags skinCluster shares with every deformer command, and those of its own beyond binding and weights: where the
//! skin goes in the chain, -exclusive, -multi, -toSkeletonAndTransforms, volume binding, -selectInfluenceVerts, the
//! deformer queries (-components, -dumpInfo and the rest), -remove and -prune.

use std::sync::Arc;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, Commands};
use serde_json::{json, Value};

use crate::components::ComponentMode;
use crate::deform::{self, Place};
use crate::graph::Graph;
use crate::history_qe::{f, upsert};
use crate::object_sets::{self as os, Member};
use crate::plug::{NodeId, PlugRef, PlugValue};
use crate::session::Session;
use crate::skin::{self, asked, SKIN_TYPE};

/// The set holding a skin's points when -exclusive puts them in a partition.
fn exclusive_set(g: &Graph, sk: NodeId) -> Option<NodeId> {
    let name = format!("{}Set", g.node(sk)?.name());
    g.find(&name).ok().flatten().filter(|&id| g.node(id).is_some_and(os::is_set))
}

fn skin_arg(s: &Session, a: &Args) -> CmdResult<NodeId> {
    let g = &s.graph;
    let name = match objects(s, a).first() {
        Some(n) => n.clone(),
        None => fail!("skinCluster: name a skinCluster or a skinned mesh"),
    };
    let Some(sk) = skin::skin_named(g, &name) else { fail!("skinCluster: {name} has no skin") };
    Ok(sk)
}

fn points(g: &Graph, mesh: NodeId, ids: Vec<u32>) -> Vec<String> {
    if ids.is_empty() {
        return Vec::new();
    }
    os::format(g, &[Member { node: mesh, kind: ComponentMode::Vertex, ids }])
}

fn query(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    let sk = skin_arg(s, a)?;
    let g = &mut s.graph;
    if let Some(v) = crate::deform_common::query(g, sk, a, &[]) {
        // -exclusive: the partition the skin's points are in.
        if asked(a, "exclusive") {
            let part = exclusive_set(g, sk)
                .and_then(|set| {
                    let msg = g.node(set)?.ty().attr_index("message")?;
                    g.destinations(&PlugRef::new(set, msg as i32)).into_iter().find(|d| g.node(d.node).is_some_and(|x| x.ty().name == "partition")).map(|d| g.path(d.node))
                })
                .unwrap_or_default();
            return Ok(json!(part));
        }
        return Ok(v);
    }
    if asked(a, "geometryIndices") {
        return Ok(json!(skin::skin_geometry(g, sk).map(|_| vec![0]).unwrap_or_default()));
    }
    if asked(a, "ignoreHierarchy") {
        let i = g.node(sk).and_then(|n| n.ty().attr_index("bindMethod")).map(|i| g.node(sk).unwrap().data().int(i)).unwrap_or(0);
        return Ok(json!(i == 0));
    }
    if asked(a, "nurbsSamples") {
        let i = g.node(sk).and_then(|n| n.ty().attr_index("nurbsSamples")).map(|i| g.node(sk).unwrap().data().int(i)).unwrap_or(10);
        return Ok(json!(i));
    }
    prev(s, a)
}

fn edit(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    // -selectInfluenceVerts: the points an influence moves, selected (-addToSelection adds them, -removeFromSelection
    // takes them off).
    if let Some(inf) = a.get("selectInfluenceVerts").and_then(Value::as_str).map(str::to_string) {
        let sk = skin_arg(s, a)?;
        let g = &s.graph;
        let j = crate::rig::find(g, &inf, "skinCluster")?;
        let Some(mesh) = skin::skin_geometry(g, sk) else { fail!("skinCluster: {} deforms nothing", g.path(sk)) };
        let list = points(g, mesh, skin::influence_points(g, sk, j));
        let mut call = serde_json::Map::new();
        if a.bool("addToSelection") {
            call.insert("add".into(), json!(true));
        } else if a.bool("removeFromSelection") {
            call.insert("deselect".into(), json!(true));
        } else if list.is_empty() {
            call.insert("clear".into(), json!(true));
        } else {
            call.insert("replace".into(), json!(true));
        }
        if !list.is_empty() {
            call.insert("args".into(), json!(list));
        }
        s.run("select", Value::Object(call))?;
        return Ok(Value::Null);
    }
    // -remove -geometry: the geometry is no longer skinned.
    if a.bool("remove") {
        let geo: Vec<String> = match a.get("geometry") {
            Some(Value::String(x)) => vec![x.clone()],
            Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
            _ => Vec::new(),
        };
        if geo.is_empty() {
            fail!("skinCluster: -e -rm takes -g geometry");
        }
        return prev(s, &Args { positional: geo.into_iter().map(Value::String).collect(), flags: [("edit".to_string(), json!(true)), ("unbind".to_string(), json!(true))].into_iter().collect() });
    }
    // -prune: points no influence moves leave the skin's set.
    if a.bool("prune") {
        let sk = skin_arg(s, a)?;
        let g = &mut s.graph;
        if let (Some(set), Some(mesh)) = (exclusive_set(g, sk), skin::skin_geometry(g, sk)) {
            let ids = skin::unweighted_points(g, sk);
            if !ids.is_empty() {
                os::remove(g, set, &[Member { node: mesh, kind: ComponentMode::Vertex, ids }])?;
            }
        }
        return Ok(json!([g.path(sk)]));
    }
    let mine = ["nurbsSamples", "ignoreBindPose", "edit", "args"];
    if let Some(n) = a.get("nurbsSamples").and_then(Value::as_i64) {
        let sk = skin_arg(s, a)?;
        let g = &mut s.graph;
        if let Some(i) = g.node(sk).and_then(|x| x.ty().attr_index("nurbsSamples")) {
            g.set_value(PlugRef::new(sk, i as i32), PlugValue::Int(n.max(1)))?;
        }
    }
    let others = s.commands.current_given().is_none_or(|given| given.iter().any(|k| !mine.contains(&k.as_str())));
    if others {
        return prev(s, a);
    }
    Ok(Value::Null)
}

fn create(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    if a.bool("ignoreSelected") && a.named().is_empty() {
        fail!("skinCluster: -ignoreSelected: name the joints and the geometry");
    }
    let mut b = a.clone();
    // The older way of asking for a closest distance bind.
    if a.has("ignoreHierarchy") && !a.has("bindMethod") {
        b.flags.insert("bindMethod".into(), json!(if a.bool("ignoreHierarchy") { 0 } else { 1 }));
    }
    let r = prev(s, &b)?;
    let Some(sk) = r.as_array().and_then(|v| v.first()).and_then(Value::as_str).and_then(|n| s.graph.find(n).ok().flatten()) else { return Ok(r) };
    let g = &mut s.graph;
    if !g.node(sk).is_some_and(|n| n.ty().name == SKIN_TYPE) {
        return Ok(r);
    }
    let Some(mesh) = skin::skin_geometry(g, sk) else { return Ok(r) };
    g.open_chunk("Bind Skin Options");
    let out = (|| -> CmdResult<()> {
        // Where it goes in the chain.
        if ["before", "after", "frontOfChain", "afterReference", "split", "parallel", "automatic"].iter().any(|k| a.bool(k)) {
            let place = if a.bool("parallel") { Place::Front } else { crate::deform_common::placement(g, a, mesh) };
            deform::relocate(g, mesh, sk, place)?;
        }
        if let Some(n) = a.get("nurbsSamples").and_then(Value::as_i64) {
            if let Some(i) = g.node(sk).and_then(|x| x.ty().attr_index("nurbsSamples")) {
                g.set_value(PlugRef::new(sk, i as i32), PlugValue::Int(n.max(1)))?;
            }
        }
        // -exclusive: the skin's points in a set of that partition.
        if let Some(part) = a.get("exclusive").and_then(Value::as_str) {
            let ids: Vec<u32> = crate::modifiers::cage(g, mesh).map(|m| (0..m.point_capacity()).filter(|&p| m.point_valid(p)).collect()).unwrap_or_default();
            let name = format!("{}Set", g.node(sk).unwrap().name());
            let set = os::create(g, &name, &[Member { node: mesh, kind: ComponentMode::Vertex, ids }], false)?;
            let pid = match g.find(part)? {
                Some(p) if g.node(p).is_some_and(|x| x.ty().name == "partition") => p,
                Some(_) => fail!("skinCluster: {part} is not a partition"),
                None => {
                    let t = crate::node::types::find("partition").expect("partition");
                    g.create_node(&t, part, crate::plug::NO_NODE, true)?
                }
            };
            let (Some(sets), Some(msg)) = (g.node(pid).and_then(|x| x.ty().attr_index("sets")), g.node(set).and_then(|x| x.ty().attr_index("message"))) else { return Ok(()) };
            let next = g.elements(&PlugRef::new(pid, sets as i32)).iter().map(|e| e.element + 1).max().unwrap_or(0);
            g.connect(&PlugRef::new(set, msg as i32), &PlugRef::element(pid, sets as i32, next), false)?;
        }
        // Volume binding.
        if let Some(min) = a.get("volumeBind").and_then(Value::as_f64) {
            let kind = a.int("volumeType", 0);
            if !(0..=2).contains(&kind) {
                fail!("skinCluster: -volumeType is 0, 1 (capsule) or 2 (cylinder)");
            }
            skin::volume_bind(g, sk, min.clamp(1e-6, 1.0), kind)?;
        }
        Ok(())
    })();
    g.close_chunk();
    out?;
    g.evaluate();
    Ok(r)
}

pub(crate) fn register(c: &mut Commands<Session>) {
    let Some(mut cmd) = c.find("skinCluster").cloned() else { return };
    use ArgType::{Bool as B, Float as Fl, Int as I, String as S};
    for fl in [
        f("after", "af", B, "Go nearest the mesh, after the other deformers."),
        f("before", "bf", B, "Go just above the deformer nearest the mesh."),
        f("frontOfChain", "foc", B, "Go above every other deformer, but not above tweaks."),
        f("afterReference", "ar", B, "Go before the other deformers, or after them when the geometry is referenced."),
        f("parallel", "par", B, "Work beside the other deformers, from the shape they start from: in front of them here."),
        f("split", "sp", B, "A branch of its own from the original shape rather than into the chain: in front of it here."),
        f("exclusive", "ex", S, "The partition the skin's points go into (a set of them is made); -q: the partition."),
        f("useComponentTags", "uct", B, "Work by component tags: no set, groupId, groupParts or tweak is made (none is for a skin here)."),
        f("ignoreSelected", "is", B, "Bind only what is named, not the selection."),
        f("multi", "mul", B, "Allow another skinCluster on geometry already skinned (off: that is an error)."),
        f("toSkeletonAndTransforms", "tst", B, "Bind to the skeleton and every transform in its hierarchy too; the last object is what is bound."),
        f("ignoreHierarchy", "ih", B, "Obsolete, use -bindMethod: closest distance rather than closest in the hierarchy. -q: whether the bind ignored it."),
        f("ignoreBindPose", "ibp", B, "Obsolete, and ignored."),
        f("nurbsSamples", "ns", I, "Samples along a NURBS curve influence or across a surface one (10)."),
        f("volumeBind", "vb", Fl, "Volume binding: a volume round each bone, grown to take the points weighted at least this much to it, gives the weights."),
        f("volumeType", "vt", I, "The volumes' shape with -volumeBind: 0 the default (a capsule), 1 a capsule, 2 a cylinder."),
        f("selectInfluenceVerts", "siv", S, "With -e: select the points this influence moves."),
        f("addToSelection", "ats", B, "With -siv: add them to the selection."),
        f("removeFromSelection", "rfs", B, "With -siv: take them off the selection."),
        f("remove", "rm", B, "With -e -g: the geometry is no longer skinned."),
        f("geometry", "g", ArgType::Any, "Query: the deformed shape. With -e -rm: the geometry to take off."),
        f("prune", "pr", B, "With -e: points no influence moves leave the skin's set."),
        f("components", "cmp", B, "Query: the points the skin acts on."),
        f("selectedComponents", "cms", B, "Query: those of its points that are selected."),
        f("deformerTools", "dt", B, "Query: the skin's tool objects (it has none)."),
        f("dumpInfo", "dui", B, "Query: per geometry, its index, how many points it acts on and how many there are."),
        f("geometryIndices", "gi", B, "Query: the index of each geometry."),
    ] {
        cmd = upsert(cmd, fl);
    }
    let prev = cmd.run.clone();
    cmd.run = Arc::new(move |s: &mut Session, a: &Args| {
        if a.bool("query") {
            return query(s, a, &prev);
        }
        if a.bool("edit") {
            return edit(s, a, &prev);
        }
        create(s, a, &prev)
    });
    c.register(cmd);
}
