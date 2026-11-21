//! The cluster command: points that follow a handle, each by its weight. As the reference's: -relative (only the handle's
//! own transform counts, not its parents'), -weightedNode (an existing transform drives the cluster in place of a new
//! handle), -bindState (the points stay put however the driver sits when bound), -envelope, -resetGeometry, and the
//! creation, query and edit flags every deformer takes.

use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::components::{self, ComponentMode};
use crate::deform::{self, f, plug, Cmd, Place, ENVELOPE, HANDLE, WEIGHTS};
use crate::graph::{Graph, SelectMode};
use crate::plug::{NodeId, PlugValue, NO_NODE};
use crate::rig::world;
use crate::session::Session;
use crate::transform::Transform;

fn attr(g: &Graph, n: NodeId, name: &str) -> Option<usize> {
    g.node(n).and_then(|x| x.ty().attr_index(name))
}

fn is_relative(g: &Graph, n: NodeId) -> bool {
    attr(g, n, "relative").is_some_and(|i| g.node(n).is_some_and(|x| x.data().bool(i)))
}

/// The transform a cluster follows, and the node its matrix is read from.
fn driver(g: &Graph, n: NodeId) -> Option<NodeId> {
    g.source(&plug(n, HANDLE)).map(|p| p.node)
}

/// Connects the driver to a cluster node: its world matrix to matrix, its own matrix to weightedMatrix. With bind, the
/// bind pre-matrix is the inverse of the one the cluster reads now, so nothing moves.
fn drive(g: &mut Graph, n: NodeId, weighted: NodeId, eval: NodeId, bind: bool) -> CmdResult<()> {
    g.connect(&plug(eval, Transform::WORLD_MATRIX), &plug(n, HANDLE), true)?;
    if let Some(w) = attr(g, n, "weightedMatrix") {
        g.connect(&plug(weighted, Transform::MATRIX), &plug(n, w), true)?;
    }
    g.evaluate();
    let read = if is_relative(g, n) { local(g, weighted) } else { world(g, eval) };
    let pre = if bind { read.inverse() } else { DMat4::IDENTITY };
    let mut data = g.node(n).map(|x| deform::json_of(x.data(), deform::DATA)).unwrap_or(Value::Null);
    if !data.is_object() {
        data = json!({});
    }
    data["bindPre"] = deform::mat_json(&pre);
    data["weightedNode"] = json!([g.path(weighted), g.path(eval)]);
    deform::set_data(g, n, data)
}

fn local(g: &mut Graph, n: NodeId) -> DMat4 {
    match g.value(&plug(n, Transform::MATRIX)) {
        Ok(PlugValue::Matrix(m)) => m,
        _ => DMat4::IDENTITY,
    }
}

/// -resetGeometry: each deformed object's matrix is taken again as it is now.
fn reset_geometry(g: &mut Graph, n: NodeId) -> CmdResult<()> {
    g.evaluate();
    for x in deform::group(g, n) {
        let Some(shape) = deform::mesh_fed_by(g, x) else { continue };
        let geom = world(g, crate::dag::transform_of(g, shape));
        let mut data = g.node(x).map(|d| deform::json_of(d.data(), deform::DATA)).unwrap_or(Value::Null);
        if !data.is_object() {
            data = json!({});
        }
        data["geom"] = deform::mat_json(&geom);
        deform::set_data(g, x, data)?;
    }
    Ok(())
}

fn weighted_of(g: &Graph, n: NodeId) -> Value {
    let data = g.node(n).map(|x| deform::json_of(x.data(), deform::DATA)).unwrap_or(Value::Null);
    match data.get("weightedNode") {
        Some(v @ Value::Array(_)) => v.clone(),
        _ => driver(g, n).map_or(json!([]), |h| json!([g.path(h), g.path(h)])),
    }
}

fn weighted_arg(g: &Graph, a: &Args) -> CmdResult<Option<(NodeId, NodeId)>> {
    let Some(v) = a.get("weightedNode") else { return Ok(None) };
    let names: Vec<String> = match v {
        Value::Array(x) => x.iter().filter_map(|s| s.as_str().map(str::to_string)).collect(),
        Value::String(x) => vec![x.clone()],
        _ => Vec::new(),
    };
    let Some(first) = names.first() else { fail!("cluster: -weightedNode takes a transform and the node below it to read") };
    let w = crate::rig::find(g, first, "cluster")?;
    let e = match names.get(1) {
        Some(x) => crate::rig::find(g, x, "cluster")?,
        None => w,
    };
    Ok(Some((w, e)))
}

fn cluster_named(g: &Graph, name: &str) -> CmdResult<NodeId> {
    let id = crate::rig::find(g, name, "cluster")?;
    if g.node(id).is_some_and(|n| n.ty().name == "cluster") {
        return Ok(id);
    }
    // A handle names the cluster it drives.
    if let Some(c) = g.destinations(&plug(id, Transform::WORLD_MATRIX)).into_iter().find(|p| p.attr as usize == HANDLE && g.node(p.node).is_some_and(|n| n.ty().name == "cluster")) {
        return Ok(deform::leader_of(g, c.node));
    }
    fail!("cluster: {name} is not a cluster")
}

fn query(s: &mut Session, a: &Args, n: NodeId) -> CmdResult {
    let g = &mut s.graph;
    let asked = |k: &str| crate::skin::asked(a, k);
    if asked("envelope") {
        return Ok(json!(g.node(n).map_or(1.0, |x| x.data().float(ENVELOPE))));
    }
    if asked("weightedNode") {
        return Ok(weighted_of(g, n));
    }
    if asked("relative") {
        return Ok(json!(is_relative(g, n)));
    }
    let tools: Vec<NodeId> = driver(g, n).into_iter().collect();
    if let Some(v) = crate::deform_common::shared_query(g, n, a, &tools) {
        return Ok(v);
    }
    fail!("cluster: query -envelope, -weightedNode, -geometry, -components and the like")
}

fn edit(s: &mut Session, a: &Args, name: &str) -> CmdResult {
    let n = cluster_named(&s.graph, name)?;
    let leader = deform::leader_of(&s.graph, n);
    let path = s.graph.path(leader);
    if let Some(r) = crate::deform_common::shared_edit(s, a, &path, "cluster") {
        r?;
    }
    let g = &mut s.graph;
    if let Some(v) = a.get("envelope").and_then(Value::as_f64) {
        for x in deform::group(g, leader) {
            g.set_value(plug(x, ENVELOPE), PlugValue::Float(v))?;
        }
    }
    if let Some((w, e)) = weighted_arg(g, a)? {
        for x in deform::group(g, leader) {
            drive(g, x, w, e, a.bool("bindState"))?;
        }
    }
    if a.bool("resetGeometry") {
        reset_geometry(g, leader)?;
    }
    g.evaluate();
    Ok(json!([g.path(leader)]))
}

/// Makes a cluster of the points named or selected: [cluster, handle].
pub(crate) fn cluster_cmd(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let Some(name) = a.named().into_iter().next() else { fail!("cluster: name the cluster to query") };
        let n = cluster_named(&s.graph, &name)?;
        return query(s, a, n);
    }
    if a.bool("edit") {
        let Some(name) = a.named().into_iter().next() else { fail!("cluster: name the cluster to edit") };
        return edit(s, a, &name);
    }
    let a = &crate::deform_common::creation_args(s, a, "cluster")?;
    let meshes = deform::targets(s, a)?;
    let nurbs = deform::nurbs_targets(s, a)?;
    if meshes.is_empty() && nurbs.is_empty() {
        fail!("cluster: select vertices, CVs, meshes, curves or surfaces");
    }
    let weighted = weighted_arg(&s.graph, a)?;
    let relative = a.bool("relative");
    let place = Place::from_args(a);
    let g = &mut s.graph;
    g.evaluate();
    // The handle sits at the middle of the picked points.
    let mut sum = DVec3::ZERO;
    let mut n_pts = 0.0;
    for (m, pts) in &meshes {
        let cage = crate::modifiers::cage(g, *m).unwrap();
        let w = world(g, crate::dag::transform_of(g, *m));
        for p in 0..cage.point_capacity() {
            if cage.point_valid(p) && pts.as_ref().is_none_or(|v| v.contains(&p)) {
                sum += w.transform_point3(cage.position(p));
                n_pts += 1.0;
            }
        }
    }
    for (shape, pts) in &nurbs {
        for (i, p) in deform::world_cvs(g, *shape).into_iter().enumerate() {
            if pts.as_ref().is_none_or(|v| v.contains(&(i as u32))) {
                sum += p;
                n_pts += 1.0;
            }
        }
    }
    let centre = if n_pts > 0.0 { sum / n_pts } else { DVec3::ZERO };
    let name = a.string("name", "cluster1");
    g.open_chunk("Cluster");
    let r = (|| -> CmdResult<(NodeId, NodeId)> {
        // As the reference's: at zero, its pivot on the points' middle, so its translate is how far it has moved. With
        // -weightedNode the transform named drives it instead, and no handle is made.
        let (h, eval) = match weighted {
            Some(x) => x,
            None => {
                let h = deform::make_handle(g, &format!("{name}Handle"), "cluster", DVec3::ZERO, DVec3::ONE)?;
                g.set_value(plug(h, Transform::ROTATE_PIVOT), PlugValue::Vec3(centre))?;
                g.set_value(plug(h, Transform::SCALE_PIVOT), PlugValue::Vec3(centre))?;
                // The reference's handle shape draws at the same place.
                if let Some(sh) = deform::handle_shape(g, h) {
                    if let Some(a) = g.node(sh).and_then(|x| x.ty().attr_index("origin")) {
                        g.set_value(plug(sh, a), PlugValue::Vec3(centre))?;
                    }
                }
                (h, h)
            }
        };
        g.evaluate();
        // A new handle is where the points are, so binding to it moves nothing; an existing transform is bound as it sits
        // only with -bindState.
        let bind = weighted.is_none() || a.bool("bindState");
        let mut first = NO_NODE;
        let mut made = Vec::new();
        for (m, pts) in &meshes {
            let cage = crate::modifiers::cage(g, *m).unwrap();
            let geom = world(g, crate::dag::transform_of(g, *m));
            let n = deform::insert_at(g, *m, "cluster", &name, place)?;
            deform::set_data(g, n, json!({"geom": deform::mat_json(&geom)}))?;
            deform::limit_to(g, n, cage.point_capacity(), pts)?;
            made.push(n);
        }
        for (shape, pts) in &nurbs {
            let count = deform::world_cvs(g, *shape).len() as u32;
            let geom = world(g, crate::dag::transform_of(g, *shape));
            let n = deform::nurbs_insert(g, *shape, "cluster", &name)?;
            deform::set_data(g, n, json!({"geom": deform::mat_json(&geom)}))?;
            if let Some(w) = deform::weights_for(count, pts) {
                deform::set_floats(g, n, WEIGHTS, w)?;
            }
            made.push(n);
        }
        for n in made {
            if let (true, Some(i)) = (relative, attr(g, n, "relative")) {
                g.set_value(plug(n, i), PlugValue::Bool(true))?;
            }
            if let Some(e) = a.get("envelope").and_then(Value::as_f64) {
                g.set_value(plug(n, ENVELOPE), PlugValue::Float(e))?;
            }
            drive(g, n, h, eval, bind)?;
            if first == NO_NODE {
                first = n;
            } else {
                deform::link_follower(g, first, n)?;
            }
        }
        crate::deform_common::after_create(g, first, a)?;
        components::set_mode_undoable(g, ComponentMode::Object);
        g.select(&[h], SelectMode::Replace);
        Ok((first, h))
    })();
    g.close_chunk();
    let (n, h) = r?;
    Ok(json!([g.path(n), g.path(h)]))
}

pub(crate) fn register(c: &mut Commands<Session>) {
    let cmd = Cmd::new(
        "cluster",
        "Make a cluster of the selected vertices or CVs (or whole meshes, curves and surfaces): a handle they follow. -rel: only the \
         handle's own transform moves them. -wn transform [node below it]: an existing transform drives it in place of a new handle; \
         -bs binds it as it sits. Returns [cluster, handle].",
        cluster_cmd,
    )
    .positional("meshes, curves, surfaces, vertices or CVs (the selection when none); the cluster or its handle with -e and -q")
    .flag(f("name", "n", ArgType::String, "Name of the cluster; its handle takes the name plus Handle."))
    .flag(f("before", "bf", ArgType::Bool, "Go just above the deformer nearest the shape."))
    .flag(f("after", "af", ArgType::Bool, "Go nearest the shape, after the other deformers (the default)."))
    .flag(f("frontOfChain", "foc", ArgType::Bool, "Go above every other deformer."))
    .flag(f("envelope", "en", ArgType::Float, "How much of the deformation applies (1)."))
    .flag(f("relative", "rel", ArgType::Bool, "Only the transform directly above the cluster moves the points, not its parents (off)."))
    .flag(f("bindState", "bs", ArgType::Bool, "The points keep their places however the driving transform sits when bound."))
    .flag(f("weightedNode", "wn", ArgType::List, "The transform that drives the cluster and the node below it its matrix is read from (the handle).").multi(2))
    .flag(f("resetGeometry", "rg", ArgType::Bool, "With -e: the deformed objects' matrices are taken again as they are now, after scaling one, say."));
    let cmd = crate::deform_common::geometry_flags(crate::deform_common::creation_flags(crate::skin::modes(cmd)));
    c.register(cmd);
}
