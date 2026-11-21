//! Polygon commands of the reference from polyBevel to polyExtrudeEdge that were missing or lacked their query and edit
//! modes: query and edit of each command's history node, polyCanBridgeEdge, polyCacheMonitor, polyCircularizeEdge,
//! polyCircularizeFace, polyCollapseTweaks and the flags polyEvaluate gained (face areas, UV areas, UV edge pairs).

use std::collections::BTreeMap;

use aztek_geo::{Index, Mesh};
use aztek_ops::{fail, ArgType, Args, CmdResult, Commands, FlagDef};
use glam::{DVec2, DVec3};
use serde_json::{json, Value};

use crate::components::{self, ComponentMode as K};
use crate::history_qe::{f, Cmd};
use crate::plug::NodeId;
use crate::session::Session;

const B: ArgType = ArgType::Bool;
const I: ArgType = ArgType::Int;
const S: ArgType = ArgType::String;

/// Each command and the node types its history node may be.
const HISTORY: &[(&str, &[&str])] = &[
    ("polyBevel3", &["polyBevel3"]),
    ("polyBlendColor", &["polyBlendColor"]),
    ("polyBoolOp", &["polyBoolOp", "polyCBoolOp"]),
    ("polyBooleanCmd", &["polyCBoolOp", "polyBoolOp"]),
    ("polyCBoolOp", &["polyCBoolOp", "polyBoolOp"]),
    ("polyBridgeEdge", &["polyBridgeEdge"]),
    ("polyChipOff", &["polyChipOff"]),
    ("polyCircularize", &["polyCircularize"]),
    ("polyClean", &["polyClean"]),
    ("polyCloseBorder", &["polyCloseBorder"]),
    ("polyCollapseEdge", &["polyCollapseEdge"]),
    ("polyCollapseFacet", &["polyCollapseF"]),
    ("polyColorDel", &["polyColorDel"]),
    ("polyColorMod", &["polyColorMod"]),
    ("polyCone", &["polyCone"]),
    ("polyConnectComponents", &["polyConnectComponents"]),
    ("polyContourProjection", &["polyContourProj"]),
    ("polyCopyUV", &["polyCopyUV"]),
    ("polyCreateFacet", &["polyCreateFace"]),
    ("polyCut", &["polyCut"]),
    ("polyCube", &["polyCube"]),
    ("polyCylinder", &["polyCylinder"]),
    ("polyDelEdge", &["polyDelEdge"]),
    ("polyDelFacet", &["polyDelFacet"]),
    ("polyDelVertex", &["polyDelVertex"]),
    ("polyDuplicateEdge", &["polyDuplicateEdge"]),
    ("polyEditEdgeFlow", &["polyEditEdgeFlow"]),
    ("polyExtrudeEdge", &["polyExtrudeEdge"]),
];

/// polyBevel's flags whose attribute on the node has another name.
const BEVEL_MAP: &[(&str, &str)] = &[("subdivideNgons", "fillNgons")];

pub fn register(c: &mut Commands<Session>) {
    for &(name, types) in HISTORY {
        crate::history_qe::extend(c, name, types, &[], Vec::new());
    }
    crate::history_qe::extend(c, "polyBevel", &["polyBevel3", "polyBevelFace"], BEVEL_MAP, Vec::new());
    polyCut_parts(c);
    for (name, as_kind, doc) in [
        ("polyCircularizeEdge", K::Edge, "Make the loops of the edges named or selected round: a polyCircularize node. Returns the node."),
        ("polyCircularizeFace", K::Face, "Make the border of each region of the faces named or selected round: a polyCircularize node. Returns the node."),
    ] {
        register_circularize(c, name, as_kind, doc);
    }
    c.register(
        Cmd::new("polyCanBridgeEdge", "True when the edges named or selected can be bridged by polyBridgeEdge: border edges in two runs of the same length on one mesh.", can_bridge)
            .positional("edges (the selection when none)")
            .undoable(false),
    );
    c.register(
        Cmd::new("polyCacheMonitor", "Count a node's cache up (-chv on, the default) or down (-chv off). While the count is above zero the node keeps its output mesh.", cache_monitor)
            .flag(f("cacheValue", "chv", B, "On counts the node's cache up, off counts it down."))
            .flag(f("nodeName", "nm", S, "The node whose cache count changes."))
            .undoable(false),
    );
    c.register(
        Cmd::new("polyCollapseTweaks", "Bake a mesh's vertex tweaks into its points (the selected mesh when none is named). -q -hvt says whether it has any.", collapse_tweaks)
            .positional("a mesh (the selected one when none)")
            .flag(f("hasVertexTweaks", "hvt", B, "With -q: whether the mesh has vertex tweaks."))
            .flag(f("query", "q", B, "Query mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );
    evaluate_areas(c);
    cylindrical(c);
    uv_set_wrapped(c, "polyEditUV");
    uv_set_wrapped(c, "polyEditUVShell");
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCut's per-axis flags

#[allow(non_snake_case)]
fn polyCut_parts(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyCut").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = old;
    let lin = aztek_ops::units::Unit::Linear;
    let ang = aztek_ops::units::Unit::Angle;
    for (name, short, unit, doc) in [
        ("cutPlaneCenterX", "pcx", lin, "The cut plane's centre in X."),
        ("cutPlaneCenterY", "pcy", lin, "The cut plane's centre in Y."),
        ("cutPlaneCenterZ", "pcz", lin, "The cut plane's centre in Z."),
        ("cutPlaneRotateX", "rx", ang, "The cut plane's rotation about X."),
        ("cutPlaneRotateY", "ry", ang, "The cut plane's rotation about Y."),
        ("cutPlaneRotateZ", "rz", ang, "The cut plane's rotation about Z."),
        ("extractOffsetX", "eox", lin, "How far extracted faces move in X."),
        ("extractOffsetY", "eoy", lin, "How far extracted faces move in Y."),
        ("extractOffsetZ", "eoz", lin, "How far extracted faces move in Z."),
    ] {
        let mut fl = f(name, short, ArgType::Float, doc);
        fl.unit = unit;
        cmd = crate::history_qe::upsert(cmd, fl);
    }
    let mut size = f("cutPlaneSize", "ps", ArgType::Any, "The cut plane's width and height.").multi(2);
    size.unit = lin;
    cmd = crate::history_qe::upsert(cmd, size);
    cmd = crate::history_qe::upsert(cmd, f("cuttingDirection", "cd", S, "x, y or z: the plane across that axis through the middle of the faces' bounding box."));
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let editing = a.bool("query") || a.bool("edit");
        let mut b = a.clone();
        // The parts fold into their vectors; a query or edit of a part reaches its child attribute.
        if !editing {
            for (vec, parts) in [
                ("cutPlaneCenter", ["cutPlaneCenterX", "cutPlaneCenterY", "cutPlaneCenterZ"]),
                ("cutPlaneRotate", ["cutPlaneRotateX", "cutPlaneRotateY", "cutPlaneRotateZ"]),
                ("extractOffset", ["extractOffsetX", "extractOffsetY", "extractOffsetZ"]),
            ] {
                if parts.iter().all(|p| !b.has(p)) {
                    continue;
                }
                let mut v = b.vec3(vec).unwrap_or(if vec == "extractOffset" { [0.5; 3] } else { [0.0; 3] });
                for (i, p) in parts.iter().enumerate() {
                    if let Some(x) = b.flags.remove(*p).and_then(|x| x.as_f64()) {
                        v[i] = x;
                    }
                }
                b.flags.insert(vec.into(), json!(v));
            }
            if let Some(sz) = b.flags.remove("cutPlaneSize").and_then(|v| v.as_array().cloned()) {
                if let Some(w) = sz.first().and_then(Value::as_f64) {
                    b.flags.insert("cutPlaneWidth".into(), json!(w));
                }
                if let Some(h) = sz.get(1).and_then(Value::as_f64) {
                    b.flags.insert("cutPlaneHeight".into(), json!(h));
                }
            }
            if let Some(dir) = b.flags.remove("cuttingDirection").and_then(|v| v.as_str().map(str::to_lowercase)) {
                let rot = match dir.as_str() {
                    "x" => [0.0, std::f64::consts::FRAC_PI_2, 0.0],
                    "y" => [-std::f64::consts::FRAC_PI_2, 0.0, 0.0],
                    "z" => [0.0, 0.0, 0.0],
                    d => fail!("polyCut: -cuttingDirection {d} is not x, y or z"),
                };
                b.flags.insert("cutPlaneRotate".into(), json!(rot));
                if !a.has("cutPlaneCenter") {
                    if let Some(mid) = selection_middle(s, a)? {
                        b.flags.insert("cutPlaneCenter".into(), json!([mid.x, mid.y, mid.z]));
                    }
                }
            }
        } else {
            let asked_size = a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == "cutPlaneSize"));
            if a.bool("query") && asked_size {
                let w = s.run("polyCut", json!({"args": a.named(), "query": true, "queried": ["cutPlaneWidth"]}))?;
                let h = s.run("polyCut", json!({"args": a.named(), "query": true, "queried": ["cutPlaneHeight"]}))?;
                return Ok(json!([first_num(&w), first_num(&h)]));
            }
            if let Some(sz) = b.flags.remove("cutPlaneSize") {
                if a.bool("query") {
                    let w = s.run("polyCut", json!({"args": a.named(), "query": true, "queried": ["cutPlaneWidth"]}))?;
                    let h = s.run("polyCut", json!({"args": a.named(), "query": true, "queried": ["cutPlaneHeight"]}))?;
                    return Ok(json!([first_num(&w), first_num(&h)]));
                }
                let l = sz.as_array().cloned().unwrap_or_default();
                let mut call = json!({"args": a.named(), "edit": true});
                if let Some(w) = l.first() {
                    call["cutPlaneWidth"] = w.clone();
                }
                if let Some(h) = l.get(1) {
                    call["cutPlaneHeight"] = h.clone();
                }
                let out = s.run("polyCut", call)?;
                // The rest of the edit, if it named more.
                let more = s.commands.current_given().is_some_and(|g| g.iter().any(|k| !matches!(k.as_str(), "cutPlaneSize" | "edit" | "args")));
                if !more {
                    return Ok(out);
                }
            }
            if a.has("cuttingDirection") {
                fail!("polyCut: -cuttingDirection can only be given when cutting");
            }
        }
        run(s, &b)
    });
    c.register(cmd);
}

fn first_num(v: &Value) -> f64 {
    match v {
        Value::Array(a) => a.first().map(first_num).unwrap_or(0.0),
        x => x.as_f64().unwrap_or(0.0),
    }
}

/// The middle of the world bounding box of the faces a call names or has selected.
fn selection_middle(s: &mut Session, a: &Args) -> CmdResult<Option<DVec3>> {
    let (targets, _) = crate::mesh_edit::ref_poly::gather(s, a, K::Face)?;
    let g = &mut s.graph;
    let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
    for (mesh, ids) in targets {
        let Some(m) = crate::modifiers::cage(g, mesh) else { continue };
        let w = crate::rig::world(g, crate::dag::transform_of(g, mesh));
        for f in ids {
            if !m.face_valid(f) {
                continue;
            }
            for p in m.face_points(f) {
                let q = w.transform_point3(m.position(p));
                lo = lo.min(q);
                hi = hi.max(q);
            }
        }
    }
    Ok((lo.x <= hi.x).then(|| (lo + hi) * 0.5))
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCircularizeEdge and polyCircularizeFace

fn register_circularize(c: &mut Commands<Session>, name: &'static str, as_kind: K, doc: &str) {
    let r = |n: &str, s: &str, t: ArgType, d: Value, doc: &str| FlagDef::new(n, s, t, d, doc);
    let cmd = Cmd::new(name, doc, move |s: &mut Session, a: &Args| {
        if let Some(r) = crate::history_qe::qe(s, a, name, &["polyCircularize"], &[]) {
            return r;
        }
        let made = crate::mesh_edit::ref_poly::mesh_cmds::circularize_node_as(s, a, name, Some(as_kind))?;
        for n in made.as_array().into_iter().flatten().filter_map(Value::as_str) {
            if let Ok(Some(id)) = s.graph.find(n) {
                crate::history_qe::apply_common(&mut s.graph, id, a)?;
            }
        }
        if a.bool("createCurve") {
            curves_for_loops(s, &made)?;
        }
        Ok(made)
    })
    .positional("edges or faces (the selection when none); with -q and -e, the node or the mesh")
    .flag(r("alignment", "al", I, json!(0), "How the circle is turned against the surface."))
    .flag(r("evenlyDistribute", "ed", B, json!(true), "Space the points evenly round the circle."))
    .flag(r("normalOrientation", "nor", I, json!(0), "How the circle's plane is found: 0 from the faces' normals, 1 from the loop."))
    .flag(r("radialOffset", "ro", ArgType::Float, json!(0.0), "How far the points move out along the radius."))
    .flag(r("smoothingAngle", "sa", ArgType::Float, json!(30.0), "New edges sharper than this are hard."))
    .flag(r("worldSpace", "ws", B, Value::Null, "Values in world space (on) or the object's (off)."))
    .flag(r("createCurve", "cc", B, Value::Null, "Also make a NURBS circle along each loop made round."))
    .flag(r("inputCurve", "inc", S, Value::Null, "A curve for the operation to follow."))
    .flag(r("constructionHistory", "ch", B, Value::Null, "Keep the history node (on)."))
    .flag(r("name", "n", S, Value::Null, "The node's name."));
    let cmd = crate::history_qe::node_flags(crate::history_qe::qe_flags(cmd));
    c.register(cmd);
}

/// A circle curve along each closed loop of the edges a polyCircularize node holds, in world space.
fn curves_for_loops(s: &mut Session, made: &Value) -> CmdResult<()> {
    for n in made.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>() {
        let comps = s.run("getAttr", json!({"args": [format!("{n}.inputComponents")]})).ok();
        let Some(spec) = comps.as_ref().and_then(|v| v.as_str().map(str::to_string)).or_else(|| comps.as_ref().and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str().map(str::to_string))) else { continue };
        let edges = crate::history::parse_components(&spec);
        let out = s.run("listConnections", json!({"args": [format!("{n}.output")], "shapes": true}))?;
        let Some(shape) = out.as_array().and_then(|a| a.first()).and_then(Value::as_str).map(str::to_string) else { continue };
        let Ok(Some(mesh)) = s.graph.find(&shape) else { continue };
        let g = &mut s.graph;
        g.evaluate();
        let Some(m) = crate::components::mesh_of(g, mesh) else { continue };
        let w = crate::rig::world(g, crate::dag::transform_of(g, mesh));
        for chain in point_chains(&m, &edges) {
            if chain.len() < 3 {
                continue;
            }
            let pts: Vec<DVec3> = chain.iter().map(|&p| w.transform_point3(m.position(p))).collect();
            let mid = pts.iter().copied().sum::<DVec3>() / pts.len() as f64;
            let radius = pts.iter().map(|p| p.distance(mid)).sum::<f64>() / pts.len() as f64;
            let mut nrm = DVec3::ZERO;
            for i in 0..pts.len() {
                let (p, q) = (pts[i] - mid, pts[(i + 1) % pts.len()] - mid);
                nrm += p.cross(q);
            }
            let nrm = nrm.try_normalize().unwrap_or(DVec3::Y);
            s.run("circle", json!({"center": [mid.x, mid.y, mid.z], "normal": [nrm.x, nrm.y, nrm.z], "radius": radius.max(1e-6), "constructionHistory": false}))?;
        }
    }
    Ok(())
}

/// The points of each run of connected edges, in order along it.
fn point_chains(m: &Mesh, edges: &[Index]) -> Vec<Vec<Index>> {
    let mut by_point: BTreeMap<Index, Vec<Index>> = BTreeMap::new();
    let valid: Vec<Index> = edges.iter().copied().filter(|&e| m.edge_valid(e)).collect();
    for &e in &valid {
        by_point.entry(m.edge_point(e, 0)).or_default().push(e);
        by_point.entry(m.edge_point(e, 1)).or_default().push(e);
    }
    let mut used = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for &start in &valid {
        if used.contains(&start) {
            continue;
        }
        // Walk to one end first, so open runs come out whole.
        let mut e = start;
        let mut p = m.edge_point(e, 0);
        let mut seen = std::collections::BTreeSet::new();
        loop {
            seen.insert(e);
            let next = by_point.get(&p).and_then(|l| l.iter().copied().find(|&x| x != e && !seen.contains(&x)));
            match next {
                Some(x) => {
                    p = m.edge_other_point(x, p);
                    e = x;
                }
                None => break,
            }
        }
        // Then along from that end.
        let mut chain = vec![p];
        let mut cur = p;
        let mut prev_e = Index::MAX;
        loop {
            let next = by_point.get(&cur).and_then(|l| l.iter().copied().find(|&x| x != prev_e && !used.contains(&x)));
            let Some(x) = next else { break };
            used.insert(x);
            cur = m.edge_other_point(x, cur);
            prev_e = x;
            if cur == chain[0] {
                break;
            }
            chain.push(cur);
        }
        out.push(chain);
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCanBridgeEdge

fn can_bridge(s: &mut Session, a: &Args) -> CmdResult {
    let (targets, _) = crate::mesh_edit::ref_poly::gather(s, a, K::Edge)?;
    if targets.len() != 1 {
        return Ok(json!(false));
    }
    let g = &mut s.graph;
    let (mesh, ids) = targets.into_iter().next().unwrap();
    let Some(m) = crate::modifiers::cage(g, mesh) else { return Ok(json!(false)) };
    if ids.is_empty() || ids.iter().any(|&e| !m.edge_valid(e) || !m.is_boundary_edge(e)) {
        return Ok(json!(false));
    }
    let runs = edge_runs(&m, &ids);
    Ok(json!(runs.len() == 2 && runs[0] == runs[1]))
}

/// How many edges each connected run of the edges holds.
fn edge_runs(m: &Mesh, edges: &[Index]) -> Vec<usize> {
    let mut parent: BTreeMap<Index, Index> = BTreeMap::new();
    fn root(p: &mut BTreeMap<Index, Index>, x: Index) -> Index {
        let mut r = x;
        while let Some(&up) = p.get(&r) {
            if up == r {
                break;
            }
            r = up;
        }
        p.insert(x, r);
        r
    }
    for &e in edges {
        let (a, b) = (m.edge_point(e, 0), m.edge_point(e, 1));
        parent.entry(a).or_insert(a);
        parent.entry(b).or_insert(b);
        let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
        if ra != rb {
            parent.insert(ra, rb);
        }
    }
    let mut count: BTreeMap<Index, usize> = BTreeMap::new();
    for &e in edges {
        let r = root(&mut parent, m.edge_point(e, 0));
        *count.entry(r).or_default() += 1;
    }
    count.into_values().collect()
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCacheMonitor

fn cache_monitor(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.get("nodeName").and_then(Value::as_str).map(str::to_string) else { fail!("polyCacheMonitor: give -nodeName") };
    let Some(id) = s.graph.find(&name).ok().flatten() else { fail!("polyCacheMonitor: no node {name}") };
    let up = a.get("cacheValue").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(1) != 0));
    let path = s.graph.path(id);
    let plug = format!("{path}.cacheInput");
    if s.graph.plug(&plug).is_err() {
        s.run("addAttr", json!({"args": [path.clone()], "longName": "cacheInput", "attributeType": "long", "defaultValue": 0}))?;
    }
    let was = s.run("getAttr", json!({"args": [plug.clone()]}))?.as_f64().unwrap_or(0.0) as i64;
    let now = if up { was + 1 } else { (was - 1).max(0) };
    s.run("setAttr", json!({"args": [plug, now]}))?;
    // The count keeps the node's output: its caching is on while the count is above zero.
    let caching = format!("{path}.caching");
    if s.graph.plug(&caching).is_ok() {
        s.run("setAttr", json!({"args": [caching, (now > 0) as i64]}))?;
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCollapseTweaks

/// The mesh shape a call names, or the one selected.
fn one_mesh(s: &mut Session, a: &Args, cmd: &str) -> CmdResult<NodeId> {
    let names = a.named();
    let g = &s.graph;
    let ids: Vec<NodeId> = if names.is_empty() { g.selection().to_vec() } else { names.iter().filter_map(|n| g.find(n).ok().flatten()).collect() };
    let meshes: Vec<NodeId> = ids.into_iter().filter_map(|i| components::as_mesh(g, i)).collect();
    match meshes.as_slice() {
        [m] => Ok(*m),
        [] => fail!("{cmd}: name or select a mesh"),
        _ => fail!("{cmd}: name or select one mesh"),
    }
}

/// The polyTweak feeding the mesh directly, when its moves aren't empty.
fn live_tweak(g: &crate::graph::Graph, mesh: NodeId) -> Option<NodeId> {
    let (_, src) = crate::modifiers::chain_base(g, mesh);
    let src = src?;
    let node = g.node(src.node)?;
    if node.ty().name != crate::history::TWEAK_TYPE {
        return None;
    }
    let has = match node.data().value(crate::history::TWEAK) {
        crate::plug::PlugValue::Data(Some(t)) => t.as_any().downcast_ref::<crate::history::Tweaks>().is_some_and(|t| t.points.values().any(|v| *v != DVec3::ZERO)),
        _ => false,
    };
    has.then_some(src.node)
}

fn collapse_tweaks(s: &mut Session, a: &Args) -> CmdResult {
    let mesh = one_mesh(s, a, "polyCollapseTweaks")?;
    let tweak = live_tweak(&s.graph, mesh);
    if a.bool("query") {
        return Ok(json!(tweak.is_some()));
    }
    let Some(tw) = tweak else { return Ok(Value::Null) };
    let g = &mut s.graph;
    let input = g.source(&crate::plug::PlugRef::new(tw, crate::history::INPUT_POLYMESH as i32));
    let procedural = input.is_some_and(|i| !crate::modifiers::is_mesh_source_node(g, i.node));
    if procedural {
        // Under history the moves become part of it: their node stays where it is and an empty one takes the mesh's
        // next moves, as the reference's mesh starts again with no tweaks.
        let Some(t) = crate::node::types::find(crate::history::TWEAK_TYPE) else { return Ok(Value::Null) };
        g.open_chunk("Collapse Tweaks");
        let r = (|| -> CmdResult<()> {
            let (base, src) = crate::modifiers::chain_base(g, mesh);
            let Some(src) = src else { return Ok(()) };
            let fresh = g.create_node(&t, "", crate::plug::NO_NODE, true)?;
            g.disconnect(&base);
            g.connect(&src, &crate::plug::PlugRef::new(fresh, crate::history::INPUT_POLYMESH as i32), false)?;
            g.connect(&crate::plug::PlugRef::new(fresh, crate::history::OUTPUT as i32), &base, false)?;
            Ok(())
        })();
        g.close_chunk();
        g.evaluate();
        return r.map(|_| Value::Null);
    }
    g.evaluate();
    let Some(now) = crate::components::mesh_of(g, mesh).map(|m| (*m).clone()) else { return Ok(Value::Null) };
    g.open_chunk("Collapse Tweaks");
    let r = (|| -> CmdResult<()> {
        let out = crate::plug::PlugRef::new(tw, crate::history::OUTPUT as i32);
        let dst = g.destinations(&out);
        let src = input;
        g.delete_node(tw)?;
        if let Some(src) = src {
            for d in dst {
                g.connect(&src, &d, false)?;
            }
        }
        let base = crate::mesh_edit::base_plug(g, mesh)?;
        crate::mesh_edit::store_edited(g, mesh, base, now, &Default::default())
    })();
    g.close_chunk();
    r.map(|_| Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// polyEvaluate: per-face areas, UV areas and UV edge pairs

fn polygon_area(pts: &[DVec3]) -> f64 {
    let mut n = DVec3::ZERO;
    for i in 0..pts.len() {
        n += pts[i].cross(pts[(i + 1) % pts.len()]);
    }
    n.length() * 0.5
}

fn uv_area(pts: &[DVec2]) -> f64 {
    let mut a = 0.0;
    for i in 0..pts.len() {
        let (p, q) = (pts[i], pts[(i + 1) % pts.len()]);
        a += p.x * q.y - q.x * p.y;
    }
    (a * 0.5).abs()
}

const AREA_FLAGS: &[&str] = &["faceArea", "worldFaceArea", "uvFaceArea", "uvArea", "uvEdgePairs"];

fn evaluate_areas(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyEvaluate").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = old;
    for (name, short, doc) in [
        ("faceArea", "fa", "The area of each face named or selected, in the object's space."),
        ("worldFaceArea", "wfa", "The area of each face named or selected, in world space."),
        ("uvFaceArea", "ufa", "The UV area of each face named or selected."),
        ("uvArea", "uva", "The UV area of the object's faces (those named, when components are)."),
        ("uvEdgePairs", "uep", "The UVs at the ends of each edge named or selected, a pair for each face along it."),
    ] {
        cmd = crate::history_qe::upsert(cmd, f(name, short, B, doc));
    }
    // The reference's short name for the UV shell of each face or UV.
    if let Some(fl) = cmd.flags.iter_mut().find(|fl| fl.name == "uvShellIds") {
        fl.short_name = "usi".into();
    }
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        // The UV shell of each face named.
        let faces_named = a.named().iter().any(|n| n.contains(".f[")) || (a.named().is_empty() && components::mode(&s.graph) == K::Face);
        if a.bool("uvShellIds") && faces_named {
            return face_shell_ids(s, a);
        }
        let Some(flag) = AREA_FLAGS.iter().copied().find(|fl| a.bool(fl)) else { return run(s, a) };
        evaluate_area(s, a, flag)
    });
    c.register(cmd);
}

fn evaluate_area(s: &mut Session, a: &Args, flag: &str) -> CmdResult {
    let uv_set = a.get("uvSetName").and_then(Value::as_str).map(str::to_string);
    let kind = if flag == "uvEdgePairs" { K::Edge } else { K::Face };
    // Components named, or selected for the call (the host selects them first).
    let named = a.named().iter().any(|n| n.contains('.')) || (a.named().is_empty() && components::mode(&s.graph) != K::Object);
    let (targets, _) = crate::mesh_edit::ref_poly::gather(s, a, kind)?;
    let mut list: Vec<Value> = Vec::new();
    let mut total = 0.0;
    for (mesh, ids) in targets {
        let g = &mut s.graph;
        g.evaluate();
        let Some(m) = crate::components::mesh_of(g, mesh) else { continue };
        let w = crate::rig::world(g, crate::dag::transform_of(g, mesh));
        let layer = match &uv_set {
            Some(set) => match uv_layer(&m, set) {
                Some(l) => l,
                None => fail!("polyEvaluate: no UV set {set}"),
            },
            None => crate::uv_sets::layer(&m, &crate::uv_sets::current(&m)).unwrap_or_else(|| aztek_geo::names::uv(0)),
        };
        let uv = m.corner_attrs().get::<[f32; 2]>(layer.as_str());
        let uv_at = |c: Index| uv.get(c as usize).map_or(DVec2::ZERO, |x| DVec2::new(x[0] as f64, x[1] as f64));
        // uvArea covers the whole object unless components were named.
        let faces: Vec<Index> = if flag == "uvArea" && !named { (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect() } else { ids.clone() };
        match flag {
            "faceArea" | "worldFaceArea" => {
                for f in faces.iter().copied().filter(|&f| m.face_valid(f)) {
                    let pts: Vec<DVec3> = m.face_points(f).into_iter().map(|p| if flag == "worldFaceArea" { w.transform_point3(m.position(p)) } else { m.position(p) }).collect();
                    list.push(json!(polygon_area(&pts)));
                }
            }
            "uvFaceArea" | "uvArea" => {
                for f in faces.iter().copied().filter(|&f| m.face_valid(f)) {
                    let pts: Vec<DVec2> = m.face_corners(f).into_iter().map(uv_at).collect();
                    let ar = uv_area(&pts);
                    if flag == "uvArea" {
                        total += ar;
                    } else {
                        list.push(json!(ar));
                    }
                }
            }
            _ => {
                let tp = crate::uv_ops::topo(&m);
                let path = components::display_path(&s.graph, crate::dag::transform_of(&s.graph, mesh));
                for e in ids.iter().copied().filter(|&e| m.edge_valid(e)) {
                    let (p0, p1) = (m.edge_point(e, 0), m.edge_point(e, 1));
                    let mut pairs: Vec<(Index, Index)> = Vec::new();
                    m.for_each_edge_corner(e, |c| {
                        let f = m.corner_face(c);
                        let at = |p: Index| m.face_corners(f).into_iter().find(|&k| m.corner_point(k) == p).map(|k| tp.corner_map[k as usize]);
                        if let (Some(a0), Some(a1)) = (at(p0), at(p1)) {
                            let key = (a0.min(a1), a0.max(a1));
                            if !pairs.iter().any(|&(x, y)| (x.min(y), x.max(y)) == key) {
                                pairs.push((a0, a1));
                            }
                        }
                    });
                    for (x, y) in pairs {
                        list.push(json!(format!("{path}.map[{x}]")));
                        list.push(json!(format!("{path}.map[{y}]")));
                    }
                }
            }
        }
    }
    let mut v = if flag == "uvArea" { json!(total) } else { Value::Array(list) };
    if matches!(flag, "faceArea" | "worldFaceArea") && aztek_ops::units::user_call() {
        aztek_ops::units::json_to_user(aztek_ops::units::Unit::Area, &mut v);
    }
    Ok(v)
}

/// The corner layer holding a UV set, by the set's name.
pub(crate) fn uv_layer(m: &Mesh, set: &str) -> Option<String> {
    crate::uv_sets::layer(m, set)
}

// ---------------------------------------------------------------------------------------------------------------------
// polyCylindricalProjection: the projection's settings on its polyCylProj node

/// Each single-value flag and the polyCylProj attribute it reads and writes.
const CYL_SINGLE: &[(&str, &str)] = &[
    ("projectionCenterX", "projectionCenterX"),
    ("projectionCenterY", "projectionCenterY"),
    ("projectionCenterZ", "projectionCenterZ"),
    ("projectionHeight", "projectionHeight"),
    ("projectionHorizontalSweep", "projectionHorizontalSweep"),
    ("projectionScaleU", "projectionHorizontalSweep"),
    ("projectionScaleV", "projectionHeight"),
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
const CYL_MULTI: &[(&str, &[&str])] = &[
    ("projectionCenter", &["projectionCenterX", "projectionCenterY", "projectionCenterZ"]),
    ("projectionScale", &["projectionHorizontalSweep", "projectionHeight"]),
    ("imageCenter", &["imageCenterX", "imageCenterY"]),
    ("imageScale", &["imageScaleX", "imageScaleY"]),
];

fn set_plug(s: &mut Session, plug: &str, v: &Value) -> CmdResult<()> {
    let v = match v {
        Value::Bool(b) => json!(*b as i64),
        x => x.clone(),
    };
    s.run("setAttr", json!({"args": [plug, v]}))?;
    Ok(())
}

fn get_plug(s: &mut Session, plug: &str) -> CmdResult<Value> {
    let v = s.run("getAttr", json!({"args": [plug]}))?;
    Ok(match v {
        Value::Array(a) if a.len() == 1 => a[0].clone(),
        x => x,
    })
}

/// A projection command's flags and the node attributes they stand for: single-value flags, then flags of several
/// values.
pub(crate) type ProjTables = (&'static [(&'static str, &'static str)], &'static [(&'static str, &'static [&'static str])]);

/// Sets the settings the call names on one projection node.
fn cyl_apply(s: &mut Session, a: &Args, node: &str, only_given: bool, tables: ProjTables) -> CmdResult<()> {
    let given: Option<Vec<String>> = if only_given { s.commands.current_given().map(|g| g.to_vec()) } else { None };
    let named = |k: &str| given.as_ref().is_none_or(|g| g.iter().any(|x| x == k));
    let (single, multi) = tables;
    for &(flag, attrs) in multi {
        if !named(flag) {
            continue;
        }
        let Some(Value::Array(vals)) = a.get(flag) else { continue };
        for (attr, v) in attrs.iter().zip(vals) {
            // projectionScale's sweep comes in degrees in a script, like the other angles.
            let mut v = v.clone();
            if *attr == "projectionHorizontalSweep" && aztek_ops::units::user_call() {
                if let Some(x) = v.as_f64() {
                    v = json!(x.to_radians());
                }
            }
            set_plug(s, &format!("{node}.{attr}"), &v)?;
        }
    }
    for &(flag, attr) in single {
        if !named(flag) {
            continue;
        }
        let Some(v) = a.get(flag).cloned() else { continue };
        let plug = format!("{node}.{attr}");
        if s.graph.plug(&plug).is_ok() {
            set_plug(s, &plug, &v)?;
        }
    }
    Ok(())
}

pub(crate) fn cyl_flags(mut c: Cmd) -> Cmd {
    let lin = aztek_ops::units::Unit::Linear;
    let ang = aztek_ops::units::Unit::Angle;
    let fl = |n: &str, sh: &str, t: ArgType, unit: aztek_ops::units::Unit, doc: &str| {
        let mut x = f(n, sh, t, doc);
        x.unit = unit;
        x
    };
    for x in [
        fl("projectionCenter", "pc", ArgType::Vec3, lin, "Where the projection sits."),
        fl("projectionHeight", "ph", ArgType::Float, lin, "The cylinder's height."),
        fl("projectionHorizontalSweep", "phs", ArgType::Float, ang, "How far round the cylinder the map wraps."),
        fl("projectionScale", "ps", ArgType::Any, aztek_ops::units::Unit::None, "The map's sweep (degrees) and height.").multi(2),
        fl("imageCenter", "ic", ArgType::Any, aztek_ops::units::Unit::None, "Where the middle of the UVs goes (0.5 0.5).").multi(2),
        fl("imageScale", "is", ArgType::Any, aztek_ops::units::Unit::None, "How the UVs scale about that middle (1 1).").multi(2),
        fl("radius", "r", ArgType::Float, lin, "The manipulator's radius (10)."),
        fl("rotationAngle", "ra", ArgType::Float, ang, "How far the UVs turn, counterclockwise."),
        fl("projectionScaleU", "psu", ArgType::Float, ang, "The map's sweep round the cylinder."),
        fl("projectionScaleV", "psv", ArgType::Float, lin, "The map's height."),
        fl("seamCorrect", "sc", B, aztek_ops::units::Unit::None, "Cut the seam where the map wraps."),
        fl("worldSpace", "ws", B, aztek_ops::units::Unit::None, "Values in world space (on) or the object's (off)."),
        fl("perInstance", "pi", B, aztek_ops::units::Unit::None, "A new map for this instance only; UV sets are shared by every instance here."),
        fl("name", "n", S, aztek_ops::units::Unit::None, "The node's name."),
    ] {
        c = crate::history_qe::upsert(c, x);
    }
    crate::history_qe::node_flags(crate::history_qe::qe_flags(c))
}

fn cylindrical(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyCylindricalProjection").cloned() else { return };
    projection_qe(c, cyl_flags(old), "polyCylProj", (CYL_SINGLE, CYL_MULTI));
}

/// Gives a projection command its settings on its node: set as it is made, read with -q, changed with -e.
pub(crate) fn projection_qe(c: &mut Commands<Session>, mut cmd: Cmd, node_type: &'static str, tables: ProjTables) {
    let run = cmd.run.clone();
    let name = cmd.name.clone();
    let (single, multi) = tables;
    cmd.positional_doc = format!("faces or meshes; with -q and -e, the {node_type} node or the mesh");
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if a.bool("query") || a.bool("edit") {
            let names = a.named();
            let Some(first) = names.first() else { fail!("{name}: name the {node_type} node or a mesh it feeds") };
            let Some(node) = crate::history_qe::find_hist(&s.graph, first, &[node_type]) else { fail!("{name}: {first} has no {node_type} node") };
            let path = s.graph.path(node);
            if a.bool("edit") {
                cyl_apply(s, a, &path, true, tables)?;
                s.graph.evaluate();
                return Ok(json!([path]));
            }
            let asked: Vec<String> = a.get("queried").and_then(Value::as_array).map(|q| q.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            let Some(flag) = asked.first() else { fail!("{name}: name a flag to query") };
            if let Some((_, attrs)) = multi.iter().find(|m| m.0 == flag) {
                let mut out = Vec::new();
                for attr in attrs.iter() {
                    let mut v = get_plug(s, &format!("{path}.{attr}"))?;
                    if *attr == "projectionHorizontalSweep" && aztek_ops::units::user_call() {
                        if let Some(x) = v.as_f64() {
                            v = json!(x.to_degrees());
                        }
                    }
                    out.push(v);
                }
                return Ok(Value::Array(out));
            }
            let Some(&(_, attr)) = single.iter().find(|m| m.0 == flag) else { fail!("{name}: -{flag} can't be queried") };
            let v = get_plug(s, &format!("{path}.{attr}"))?;
            return Ok(match (flag.as_str(), v) {
                ("seamCorrect" | "worldSpace" | "caching", x) => json!(x.as_f64().map_or(x.as_bool().unwrap_or(false), |n| n != 0.0)),
                ("nodeState", x) => json!(x.as_f64().unwrap_or(0.0) as i64),
                (_, x) => x,
            });
        }
        // The settings go on the node once it is made, in place of the fitted ones.
        let mut b = a.clone();
        for k in multi.iter().map(|m| m.0).chain(single.iter().map(|m| m.0)) {
            b.flags.remove(k);
        }
        let made = run(s, &b)?;
        let mut out = Vec::new();
        for n in made.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>() {
            let mut n = n;
            if s.graph.find(&n).ok().flatten().is_some_and(|id| s.graph.node(id).is_some_and(|x| x.ty().name == node_type)) {
                cyl_apply(s, a, &n, true, tables)?;
                if let Some(name) = a.get("name").and_then(Value::as_str).filter(|x| !x.is_empty()) {
                    if let Some(new) = s.run("rename", json!({"args": [n.clone(), name]}))?.as_str() {
                        n = new.to_string();
                    }
                }
            }
            out.push(json!(n));
        }
        s.graph.evaluate();
        Ok(if made.is_array() { Value::Array(out) } else { made })
    });
    c.register(cmd);
}

// ---------------------------------------------------------------------------------------------------------------------
// -uvSetName on polyEditUV and polyEditUVShell

/// Runs a UV command on another UV set: each mesh it names (or has selected) makes that set current for the call, then
/// goes back.
pub(crate) fn uv_set_wrapped(c: &mut Commands<Session>, name: &'static str) {
    let Some(old) = c.find(name).cloned() else { return };
    let run = old.run.clone();
    let mut cmd = old;
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let Some(set) = a.get("uvSetName").and_then(Value::as_str).filter(|x| !x.is_empty()).map(str::to_string) else { return run(s, a) };
        let names = a.named();
        let mut meshes: Vec<String> = Vec::new();
        let ids: Vec<NodeId> = if names.is_empty() {
            let mut v: Vec<NodeId> = s.graph.selection().to_vec();
            for k in [K::Vertex, K::Edge, K::Face, K::Uv] {
                v.extend(components::selected_all(&s.graph, k).keys().copied());
            }
            v
        } else {
            names.iter().filter_map(|n| s.graph.find(n.split('.').next().unwrap_or(n)).ok().flatten()).collect()
        };
        for id in ids {
            if let Some(m) = components::as_mesh(&s.graph, id) {
                let p = s.graph.path(m);
                if !meshes.contains(&p) {
                    meshes.push(p);
                }
            }
        }
        let mut restore = Vec::new();
        for m in &meshes {
            let all = s.run("polyUVSet", json!({"args": [m], "query": true, "queried": ["allUVSets"]}))?;
            if !all.as_array().is_some_and(|l| l.iter().any(|x| x.as_str() == Some(set.as_str()))) {
                // -createNewMap: the set is made, a copy of the current one, for the command to work on.
                if !a.bool("createNewMap") {
                    fail!("{name}: {m} has no UV set {set}");
                }
                s.run("polyUVSet", json!({"args": [m], "copy": true, "newUVSet": set}))?;
            }
            let cur = s.run("polyUVSet", json!({"args": [m], "query": true, "queried": ["currentUVSet"]}))?;
            let cur = cur.as_array().and_then(|l| l.first()).and_then(Value::as_str).unwrap_or_default().to_string();
            if cur != set {
                s.run("polyUVSet", json!({"args": [m], "currentUVSet": true, "uvSet": set}))?;
                restore.push((m.clone(), cur));
            }
        }
        let r = run(s, a);
        for (m, cur) in restore {
            let _ = s.run("polyUVSet", json!({"args": [m], "currentUVSet": true, "uvSet": cur}));
        }
        r
    });
    c.register(cmd);
}

fn face_shell_ids(s: &mut Session, a: &Args) -> CmdResult {
    let (targets, _) = crate::mesh_edit::ref_poly::gather(s, a, K::Face)?;
    let mut out = Vec::new();
    for (mesh, ids) in targets {
        let Some(m) = crate::components::mesh_of(&mut s.graph, mesh) else { continue };
        let tp = crate::uv_ops::topo(&m);
        out.extend(ids.iter().filter_map(|&f| tp.face_shell.get(f as usize).copied()).map(|x| json!(x)));
    }
    Ok(Value::Array(out))
}
