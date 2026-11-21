//! The evaluation manager's custom evaluators (`evaluator`) and the deformer evaluator's partitioning
//! (`deformerEvaluator`).
//!
//! Each evaluator claims the nodes it handles, in clusters, by priority: dynamics the bodies, solvers and fields;
//! frozen the frozen nodes; invisibility the hidden DAG nodes and what only they use; deformer the deformation chains it
//! can take (every deformer in the chain one it supports, the mesh large enough, nothing blocking it). Node types turned
//! on or off for an evaluator with -nodeType change what it claims. The graph evaluates as before whatever is claimed:
//! what these commands report is the partitioning, kept consistent with the scene.

use std::collections::{BTreeMap, BTreeSet};

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::NodeId;
use crate::ref_core::{asked, f};
use crate::session::Session;
use crate::sys_misc::print;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Int as I, String as S};

/// One evaluator: whether it runs, its priority, the node types turned on and off for it, its configuration.
#[derive(Clone, Debug)]
pub struct Evaluator {
    pub name: &'static str,
    pub enabled: bool,
    pub priority: i64,
    pub types_on: BTreeSet<String>,
    pub types_off: BTreeSet<String>,
    pub config: BTreeMap<String, String>,
}

/// The evaluators: name, on by default, priority, what it does, the configuration it takes as key=value (the first value
/// is the default).
const EVALUATORS: &[(&str, bool, i64, &str, &[(&str, &[&str])])] = &[
    ("dynamics", true, 103000, "Evaluates the dynamics: bodies, solvers, fields and emitters, as one cluster for each solver.", &[("disablingNodes", &["unsupported", "legacy2016", "none"]), ("handledNodes", &["dynamics", "none"]), ("action", &["evaluate", "freeze", "none"])]),
    ("frozen", true, 102000, "Leaves out the frozen nodes, and those only they feed when propagation is on.", &[("runtimePropagation", &["on", "off"]), ("invisible", &["on", "off"]), ("displayLayers", &["on", "off"]), ("downstream", &["safe", "off", "force"]), ("upstream", &["safe", "off", "force"])]),
    ("invisibility", false, 101000, "Leaves out the nodes that only hidden objects use.", &[("isolateSelected", &["off", "on"]), ("textureInvisibility", &["off", "on"])]),
    ("pruneRoots", true, 1000, "Groups the graph's roots with what they feed, so fewer tasks are scheduled.", &[]),
    ("deformer", false, 2000, "Takes the deformation chains it supports: every deformer of the chain one it handles, the mesh large enough.", &[("cluster", &["chain", "subgraph"])]),
    ("cache", true, 3000, "Plays back cached evaluation for the nodes it handles.", &[("mode", &["animated", "all"])]),
    ("curveManager", false, 4000, "Evaluates animation curves together, ahead of what they drive.", &[("mode", &["all", "static"])]),
    ("transformFlattening", true, 5000, "Evaluates chains of transforms as one.", &[]),
    ("reference", true, 6000, "Keeps unloaded references' nodes out of evaluation.", &[]),
    ("ikSystem", true, 7000, "Evaluates the IK handles with their solvers and joints.", &[]),
    ("timeEditorCurveEvaluator", true, 8000, "Evaluates the time editor's curves.", &[]),
    ("disabling", false, 9000, "Turns evaluation off for the node types given to it.", &[]),
    ("nodeOverride", false, 10000, "Uses the nodes' own evaluation where they declare one.", &[]),
];

fn defaults() -> Vec<Evaluator> {
    EVALUATORS
        .iter()
        .map(|(name, on, prio, _, conf)| Evaluator { name, enabled: *on, priority: *prio, types_on: BTreeSet::new(), types_off: BTreeSet::new(), config: conf.iter().map(|(k, v)| (k.to_string(), v[0].to_string())).collect() })
        .collect()
}

/// What this module keeps for the session.
#[derive(Default)]
pub struct EvalState {
    evaluators: Option<Vec<Evaluator>>,
    pub deformer: DeformerSettings,
}

impl EvalState {
    pub fn evaluators(&mut self) -> &mut Vec<Evaluator> {
        self.evaluators.get_or_insert_with(defaults)
    }
}

/// The deformer evaluator's settings.
#[derive(Clone, Debug)]
pub struct DeformerSettings {
    pub allow_downloads: bool,
    pub allow_download_rejections: bool,
    pub opencl_valid: bool,
    pub limit_minimum_verts: bool,
    pub minimum_verts: i64,
    pub verbose: bool,
    /// How GPU nodes are kept when the graph is partitioned again: for deformers and for display meshes.
    pub reuse: (String, String),
    /// Nodes' blocking policies, by node.
    pub block: BTreeMap<NodeId, String>,
    /// How many times the graph has been partitioned.
    pub partitions: u64,
}

impl Default for DeformerSettings {
    fn default() -> Self {
        DeformerSettings { allow_downloads: true, allow_download_rejections: true, opencl_valid: true, limit_minimum_verts: true, minimum_verts: 500, verbose: false, reuse: ("immediate".into(), "immediate".into()), block: BTreeMap::new(), partitions: 0 }
    }
}

/// The deformer types the deformer evaluator takes, and those passing their input on.
pub const GPU_DEFORMERS: &[&str] = &["skinCluster", "blendShape", "cluster", "softMod", "deltaMush", "tension", "ffd", "nonLinear", "wire", "solidify", "proximityWrap", "morph", "jiggle", "sculpt", "textureDeformer"];
const PASSTHROUGH: &[&str] = &["groupParts", "groupId", "tweak"];
const POLICIES: &[&str] = &["off", "on", "group", "upstream", "upstreamExcl", "upstreamMesh", "downstream", "downstreamExcl", "groupDownload"];

/// A node's state on the GPU: (code, text).
const STATUS: &[(i64, &str)] = &[(0, "not handled"), (1, "active"), (2, "unsupported"), (3, "too few vertices"), (4, "blocked"), (5, "evaluator off"), (6, "no OpenCL")];

fn status_text(code: i64) -> &'static str {
    STATUS.iter().find(|s| s.0 == code).map_or("unknown", |s| s.1)
}

/// One deformation chain: its deformers from the mesh up, the passthrough nodes among them, the mesh, and its state.
#[derive(Clone, Debug)]
struct Chain {
    mesh: NodeId,
    nodes: Vec<NodeId>,
    status: i64,
    messages: Vec<String>,
}

fn is_gpu_type(t: &crate::node::NodeType) -> bool {
    GPU_DEFORMERS.iter().any(|d| t.is_a(d)) || crate::deform::NONLINEAR.contains(&t.name.as_str())
}

fn is_chain_node(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| {
        let t = n.ty();
        PASSTHROUGH.contains(&t.name.as_str()) || crate::deform::is_deformer(&t.name) || t.is_a("geometryFilter") || t.name == "skinCluster"
    })
}

fn chains(s: &mut Session) -> Vec<Chain> {
    let evaluator_on = s.extras.eval.evaluators().iter().any(|e| e.name == "deformer" && e.enabled);
    let types_off: BTreeSet<String> = s.extras.eval.evaluators().iter().find(|e| e.name == "deformer").map(|e| e.types_off.clone()).unwrap_or_default();
    let set = s.extras.eval.deformer.clone();
    let g = &s.graph;
    let mut out = Vec::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if !n.ty().is_a("mesh") || n.bool(crate::transform::DagNode::INTERMEDIATE_OBJECT) {
            continue;
        }
        let hist = crate::history::history_of(g, id);
        let nodes: Vec<NodeId> = hist.into_iter().take_while(|&h| is_chain_node(g, h)).collect();
        if nodes.iter().all(|&h| g.node(h).is_some_and(|x| PASSTHROUGH.contains(&x.ty().name.as_str()))) {
            continue;
        }
        let mut messages = Vec::new();
        let mut status = 1;
        for &h in &nodes {
            let x = g.node(h).unwrap();
            let t = x.ty();
            if PASSTHROUGH.contains(&t.name.as_str()) {
                continue;
            }
            if !is_gpu_type(t) || types_off.iter().any(|o| t.is_a(o)) {
                messages.push(format!("{}: the {} deformer is not supported", x.name(), t.name));
                status = 2;
            }
            if set.block.get(&h).is_some_and(|p| p != "off") {
                messages.push(format!("{}: blocked by its GPU block policy ({})", x.name(), set.block[&h]));
                status = status.max(4).min(if status == 2 { 2 } else { 4 });
            }
        }
        if set.block.get(&id).is_some_and(|p| p != "off") {
            messages.push(format!("{}: blocked by its GPU block policy ({})", n.name(), set.block[&id]));
            if status == 1 {
                status = 4;
            }
        }
        let verts = cage_points(g, id);
        if status == 1 && set.limit_minimum_verts && (verts as i64) < set.minimum_verts {
            messages.push(format!("{}: {verts} vertices, under the minimum of {}", n.name(), set.minimum_verts));
            status = 3;
        }
        if status == 1 && !evaluator_on {
            messages.push("the deformer evaluator is off (evaluator -en on -n deformer)".into());
            status = 5;
        }
        if status == 1 && !set.opencl_valid {
            messages.push("OpenCL is not valid".into());
            status = 6;
        }
        out.push(Chain { mesh: id, nodes, status, messages });
    }
    out
}

/// A mesh shape's vertex count, from what it holds.
fn cage_points(g: &Graph, id: NodeId) -> usize {
    let Some(n) = g.node(id) else { return 0 };
    ["outMesh", "inMesh", "cachedInMesh"].iter().filter_map(|a| n.ty().attr_index(a)).find_map(|i| n.data().mesh(i).map(|m| m.num_points() as usize)).unwrap_or(0)
}

fn chain_of<'a>(chains: &'a [Chain], id: NodeId) -> Option<&'a Chain> {
    chains.iter().find(|c| c.mesh == id || c.nodes.contains(&id))
}

fn node_status(chains: &[Chain], g: &Graph, id: NodeId) -> i64 {
    match chain_of(chains, id) {
        None => 0,
        Some(c) => {
            if c.status != 1 {
                return c.status;
            }
            let _ = g;
            1
        }
    }
}

fn deformer_evaluator(s: &mut Session, a: &Args) -> CmdResult {
    let q = a.bool("query");
    // Settings: set, or with -q read.
    {
        let d = &mut s.extras.eval.deformer;
        let bools: [(&str, &mut bool); 5] = [("allowDownloads", &mut d.allow_downloads), ("allowDownloadRejections", &mut d.allow_download_rejections), ("isOpenCLValid", &mut d.opencl_valid), ("limitMinimumVerts", &mut d.limit_minimum_verts), ("verbose", &mut d.verbose)];
        for (k, slot) in bools {
            if q && asked(a, k) && a.get(k).is_none_or(|v| v.as_bool() == Some(true) || v.is_null()) && !matches!(k, "verbose") {
                return Ok(json!(*slot));
            }
            if !q {
                if let Some(v) = a.get(k) {
                    *slot = v.as_bool().unwrap_or_else(|| v.as_i64().is_some_and(|i| i != 0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true")));
                }
            }
        }
        if q && (asked(a, "minimumVerts")) && a.get("minimumVerts").is_none_or(Value::is_null) {
            return Ok(json!(d.minimum_verts));
        }
        if let (false, Some(v)) = (q, a.get("minimumVerts").and_then(Value::as_i64)) {
            d.minimum_verts = v.max(0);
        }
        if q && asked(a, "reuseMode") && a.get("reuseMode").is_none_or(|v| !v.is_array()) {
            return Ok(json!([d.reuse.0, d.reuse.1]));
        }
        if let (false, Some(Value::Array(v))) = (q, a.get("reuseMode")) {
            let modes: Vec<String> = v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect();
            for m in &modes {
                if !matches!(m.as_str(), "never" | "immediate" | "always") {
                    fail!("deformerEvaluator: reuse mode {m} is not never, immediate or always");
                }
            }
            if let Some(m) = modes.first() {
                d.reuse.0 = m.clone();
            }
            if let Some(m) = modes.get(1) {
                d.reuse.1 = m.clone();
            }
        }
    }
    if a.bool("partition") {
        s.extras.eval.deformer.partitions += 1;
        return Ok(Value::Null);
    }
    if a.bool("purge") {
        return Ok(Value::Null);
    }
    let all = chains(s);
    let g = &s.graph;
    let names = objects(s, a);
    let mut picked: Vec<NodeId> = Vec::new();
    for n in &names {
        match g.find(n)? {
            Some(id) => {
                // A transform stands for its mesh shape.
                picked.push(crate::components::as_mesh(g, id).filter(|_| !g.node(id).is_some_and(|x| x.ty().is_a("mesh"))).unwrap_or(id));
            }
            None => fail!("deformerEvaluator: no object named {n}"),
        }
    }
    // MEL reads on and off as switches.
    let policy = a.get("gpuBlockPolicy").and_then(|v| v.as_str().map(str::to_string).or_else(|| v.as_bool().map(|b| if b { "on".into() } else { "off".into() })));
    if let Some(policy) = policy.as_deref().filter(|_| !q) {
        if !POLICIES.contains(&policy) {
            fail!("deformerEvaluator: {policy} is not a GPU block policy: {}", POLICIES.join(", "));
        }
        for &id in &picked {
            if policy == "off" {
                s.extras.eval.deformer.block.remove(&id);
            } else {
                s.extras.eval.deformer.block.insert(id, policy.to_string());
            }
        }
        return Ok(Value::Null);
    }
    let g = &s.graph;
    let name_of = |id: NodeId| g.node(id).map(|n| n.name().to_string()).unwrap_or_default();
    let meshes_only = a.bool("meshes");
    let active_nodes = |c: &Chain| -> Vec<NodeId> {
        let mut v: Vec<NodeId> = c.nodes.iter().copied().filter(|&n| !(a.bool("omitPassthroughs") && g.node(n).is_some_and(|x| PASSTHROUGH.contains(&x.ty().name.as_str())))).collect();
        v.reverse();
        v.push(c.mesh);
        v
    };
    let lines: Vec<String> = if a.bool("deformers") {
        GPU_DEFORMERS.iter().filter(|t| crate::node::types::find(t).is_some()).map(|t| t.to_string()).collect()
    } else if a.bool("list") {
        let mut v = Vec::new();
        for c in all.iter().filter(|c| c.status == 1) {
            if meshes_only {
                v.push(name_of(c.mesh));
            } else {
                v.extend(active_nodes(c).into_iter().map(name_of));
            }
        }
        v
    } else if q && asked(a, "gpuBlockPolicy") {
        picked.iter().map(|id| s.extras.eval.deformer.block.get(id).cloned().unwrap_or_else(|| "off".into())).collect()
    } else if a.bool("dumpInfo") {
        let list: Vec<Value> = all
            .iter()
            .filter(|c| c.status != 2)
            .map(|c| json!({"mesh": name_of(c.mesh), "status": status_text(c.status), "active": c.status == 1, "nodes": active_nodes(c).into_iter().map(|n| json!({"name": name_of(n), "type": g.node(n).map(|x| x.ty().name.clone()).unwrap_or_default()})).collect::<Vec<_>>()}))
            .collect();
        vec![serde_json::to_string(&json!({"clusters": list})).unwrap_or_default()]
    } else if a.bool("dumpOutliner") || a.bool("outlinerHash") {
        let mut items = Vec::new();
        for c in all.iter().filter(|c| c.status != 2) {
            for n in active_nodes(c) {
                items.push(format!("{{'name': '{}', 'type': '{}', 'status': {}, 'mesh': '{}'}}", name_of(n), g.node(n).map(|x| x.ty().name.clone()).unwrap_or_default(), node_status(&all, g, n), name_of(c.mesh)));
            }
        }
        let text = format!("[{}]", items.join(", "));
        if a.bool("outlinerHash") {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            text.hash(&mut h);
            vec![format!("{:016x}", h.finish())]
        } else {
            vec![text]
        }
    } else {
        // About nodes: those named or selected, or with -active every active one.
        let targets: Vec<NodeId> = if a.bool("active") {
            all.iter().filter(|c| c.status == 1).flat_map(|c| if meshes_only || a.bool("deformerChain") { vec![c.mesh] } else { active_nodes(c) }).collect()
        } else if !picked.is_empty() {
            picked.clone()
        } else {
            g.selection().iter().map(|&id| crate::components::as_mesh(g, id).filter(|_| !g.node(id).is_some_and(|x| x.ty().is_a("mesh"))).unwrap_or(id)).collect()
        };
        let targets: Vec<NodeId> = targets.into_iter().filter(|&t| !meshes_only || g.node(t).is_some_and(|x| x.ty().is_a("mesh"))).collect();
        let prefix = |id: NodeId| if a.bool("asNodeName") { format!("{}: ", name_of(id)) } else { String::new() };
        let mut v = Vec::new();
        for &t in &targets {
            let st = node_status(&all, g, t);
            let chain = chain_of(&all, t);
            if a.bool("members") {
                if let Some(c) = chain {
                    v.extend(active_nodes(c).into_iter().map(|n| format!("{}{}", prefix(t), name_of(n))));
                }
            } else if a.bool("message") {
                match chain {
                    Some(c) if !c.messages.is_empty() => v.extend(c.messages.iter().map(|m| format!("{}{m}", prefix(t)))),
                    Some(_) => v.push(format!("{}on the GPU", prefix(t))),
                    None => v.push(format!("{}not part of a deformation chain", prefix(t))),
                }
            } else if a.bool("nodeStatus") {
                v.push(if a.bool("asText") { format!("{}{}", prefix(t), status_text(st)) } else if a.bool("asNodeName") { format!("{}{st}", prefix(t)) } else { st.to_string() });
            } else if a.bool("deformerChain") {
                if let Some(c) = chain {
                    let path: Vec<String> = active_nodes(c).into_iter().map(name_of).collect();
                    v.push(format!("{}: {} ({})", name_of(c.mesh), path.join(" -> "), status_text(c.status)));
                } else {
                    v.push(format!("{}: no deformation chain", name_of(t)));
                }
            } else {
                // nodeInfo, and what is shown with no flag.
                let ty = g.node(t).map(|x| x.ty().name.clone()).unwrap_or_default();
                let mut line = format!("{} ({ty}): {}", name_of(t), status_text(st));
                if let Some(c) = chain {
                    line.push_str(&format!(", chain of {} on {}", name_of(c.mesh), active_nodes(c).len()));
                    if a.bool("nodeInfo") || s.extras.eval.deformer.verbose || a.get("verbose").is_some() {
                        line.push_str(&format!(", {} vertices", cage_points(g, c.mesh)));
                        if let Some(p) = s.extras.eval.deformer.block.get(&t) {
                            line.push_str(&format!(", block policy {p}"));
                        }
                        for m in &c.messages {
                            line.push_str(&format!("; {m}"));
                        }
                    }
                }
                v.push(line);
            }
        }
        v
    };
    if q {
        return Ok(json!(lines));
    }
    for l in &lines {
        print(l.clone());
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// evaluator

fn derived(t: &str) -> Vec<String> {
    let mut v: Vec<String> = crate::node::types::all().iter().filter(|x| x.is_a(t)).map(|x| x.name.clone()).collect();
    v.sort();
    // The type itself first.
    v.retain(|x| x != t);
    v.insert(0, t.to_string());
    v
}

/// The nodes an evaluator claims, in clusters.
fn clusters(s: &mut Session, name: &str) -> Vec<Vec<NodeId>> {
    let ev = s.extras.eval.evaluators().iter().find(|e| e.name == name).cloned();
    let Some(ev) = ev else { return Vec::new() };
    if !ev.enabled {
        return Vec::new();
    }
    let by_type = |g: &Graph| -> Vec<NodeId> { g.nodes().into_iter().filter(|&n| g.node(n).is_some_and(|x| ev.types_on.iter().any(|t| x.ty().is_a(t)))).collect() };
    let mut out: Vec<Vec<NodeId>> = match name {
        "deformer" => chains(s).into_iter().filter(|c| c.status == 1).map(|c| {
            let mut v: Vec<NodeId> = c.nodes.clone();
            v.reverse();
            v.push(c.mesh);
            v
        }).collect(),
        "dynamics" => {
            let g = &s.graph;
            crate::dyn_solve::solvers(g).into_iter().map(|sv| {
                let mut v = vec![sv];
                v.extend(crate::dyn_solve::members(g, sv));
                v
            }).collect()
        }
        "frozen" => {
            let g = &s.graph;
            let v: Vec<NodeId> = g.nodes().into_iter().filter(|&n| g.node(n).and_then(|x| x.ty().frozen_attr.map(|i| x.bool(i))).unwrap_or(false)).collect();
            if v.is_empty() { Vec::new() } else { vec![v] }
        }
        "invisibility" => {
            let g = &s.graph;
            let v: Vec<NodeId> = g.nodes().into_iter().filter(|&n| g.node(n).is_some_and(|x| x.ty().dag && x.ty().attr_index("visibility").is_some_and(|i| !x.bool(i)))).collect();
            if v.is_empty() { Vec::new() } else { vec![v] }
        }
        _ => Vec::new(),
    };
    let g = &s.graph;
    let extra = by_type(g);
    if !extra.is_empty() {
        out.push(extra);
    }
    // Node types turned off for it are not claimed.
    for c in &mut out {
        c.retain(|&n| !g.node(n).is_some_and(|x| ev.types_off.iter().any(|t| x.ty().is_a(t))));
    }
    out.retain(|c| !c.is_empty());
    out
}

fn evaluator(s: &mut Session, a: &Args) -> CmdResult {
    let q = a.bool("query");
    let mut name = a.get("name").and_then(Value::as_str).map(str::to_string);
    if q && name.is_none() && asked(a, "name") {
        name = a.named().into_iter().next();
    }
    let node_types: Vec<String> = match a.get("nodeType") {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    let enable = a.get("enable").filter(|v| !v.is_null()).map(|v| v.as_bool().unwrap_or_else(|| v.as_i64().is_some_and(|i| i != 0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true"))));
    let list = s.extras.eval.evaluators().clone();
    let names = || json!(list.iter().map(|e| e.name).collect::<Vec<_>>());
    let found = name.as_deref().and_then(|n| list.iter().position(|e| e.name == n));
    if q {
        let Some(n) = name.as_deref() else {
            // -enable alone: the evaluators in that state.
            if let Some(state) = enable.or_else(|| asked(a, "enable").then_some(true)) {
                return Ok(json!(list.iter().filter(|e| e.enabled == state).map(|e| e.name).collect::<Vec<_>>()));
            }
            return Ok(names());
        };
        // An unknown name lists the evaluators there are.
        let Some(i) = found else { return Ok(names()) };
        let ev = &list[i];
        if asked(a, "clusters") {
            let cl = clusters(s, n);
            let g = &s.graph;
            let mut out = Vec::new();
            for c in cl {
                out.push(json!(c.len().to_string()));
                out.extend(c.into_iter().map(|id| json!(g.path(id))));
            }
            return Ok(Value::Array(out));
        }
        if asked(a, "info") {
            return Ok(json!(EVALUATORS.iter().find(|e| e.0 == n).map_or("", |e| e.3)));
        }
        if asked(a, "priority") {
            return Ok(json!(ev.priority));
        }
        if asked(a, "configuration") {
            let conf = EVALUATORS.iter().find(|e| e.0 == n).map(|e| e.4).unwrap_or(&[]);
            return Ok(json!(conf.iter().map(|(k, v)| format!("{k}={}", v.join("|"))).collect::<Vec<_>>()));
        }
        if let Some(v) = a.get("valueName").and_then(Value::as_str).map(str::to_string).or_else(|| asked(a, "valueName").then(|| a.named().into_iter().find(|w| w != n)).flatten()) {
            return match ev.config.get(&v) {
                Some(x) => Ok(json!(x)),
                None => match v.as_str() {
                    "priority" => Ok(json!(ev.priority.to_string())),
                    "clusterCount" => Ok(json!(clusters(s, n).len().to_string())),
                    _ => fail!("evaluator: {n} has no value {v}"),
                },
            };
        }
        if !node_types.is_empty() || asked(a, "nodeType") {
            let types: Vec<String> = if node_types.is_empty() { ev.types_on.iter().cloned().collect() } else { node_types };
            if asked(a, "nodeType") && a.get("nodeType").is_none() {
                return Ok(json!(types));
            }
            return Ok(json!(types.iter().all(|t| ev.types_on.contains(t))));
        }
        return Ok(json!(ev.enabled));
    }
    let Some(n) = name else { fail!("evaluator: -name is required") };
    let Some(i) = found else { fail!("evaluator: no evaluator named {n}; evaluator -q lists them") };
    let confs: Vec<String> = match a.get("configuration") {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    if !confs.is_empty() {
        let accepted = EVALUATORS.iter().find(|e| e.0 == n).map(|e| e.4).unwrap_or(&[]);
        let mut ok = true;
        for c in confs {
            let (k, v) = c.split_once('=').map_or((c.clone(), String::new()), |(k, v)| (k.to_string(), v.to_string()));
            match accepted.iter().find(|x| x.0 == k) {
                Some((_, vals)) if vals.contains(&v.as_str()) || (v.is_empty() && enable.is_some()) => {
                    let v = if v.is_empty() { if enable == Some(true) { "on".into() } else { "off".into() } } else { v };
                    s.extras.eval.evaluators()[i].config.insert(k, v);
                }
                _ => ok = false,
            }
        }
        return Ok(json!(ok));
    }
    if let Some(p) = a.get("priority").and_then(Value::as_i64) {
        let evs = s.extras.eval.evaluators();
        if evs.iter().enumerate().any(|(j, e)| j != i && e.priority == p) {
            fail!("evaluator: another evaluator has priority {p}; priorities are unique");
        }
        evs[i].priority = p;
        return Ok(json!(true));
    }
    let Some(state) = enable else { fail!("evaluator: give -enable, -priority or -configuration") };
    if !node_types.is_empty() {
        let mut done = Vec::new();
        for t in &node_types {
            if crate::node::types::find(t).is_none() {
                fail!("evaluator: {t} is not a node type");
            }
            let all = if a.bool("nodeTypeChildren") { derived(t) } else { vec![t.clone()] };
            for x in all {
                let ev = &mut s.extras.eval.evaluators()[i];
                if state {
                    ev.types_off.remove(&x);
                    ev.types_on.insert(x.clone());
                } else {
                    ev.types_on.remove(&x);
                    ev.types_off.insert(x.clone());
                }
                done.push(x);
            }
        }
        return Ok(json!(done));
    }
    let ev = &mut s.extras.eval.evaluators()[i];
    let was = ev.enabled;
    ev.enabled = state;
    Ok(json!(was))
}

pub fn register(c: &mut Commands<Session>) {
    use crate::system_cmds::q_flags;
    c.register(
        q_flags(Cmd::new(
            "evaluator",
            "The evaluation manager's custom evaluators: -en on|off -n turns one on or off (returning its state before), with \
             -nt for node types it handles (-ntc and their derived types); -p its priority; -c configuration messages. -q lists \
             them; -q -n with -en, -cl, -i, -p, -c or -vn asks about one.",
            evaluator,
        ))
        .positional("name")
        .undoable(false)
        .flag(f("clusters", "cl", B, "Query: the clusters the evaluator claims, each its node count then its nodes."))
        .flag(f("configuration", "c", A, "A configuration message, key=value; with -q, those the evaluator takes.").multi(1))
        .flag(f("enable", "en", A, "Turn the evaluator on or off, or the node types given to it; with -q, its state."))
        .flag(f("info", "i", B, "Query: what the evaluator does."))
        .flag(f("name", "n", S, "The evaluator."))
        .flag(f("nodeType", "nt", A, "A node type for the evaluator to handle or not.").multi(1))
        .flag(f("nodeTypeChildren", "ntc", B, "With -nt: the types derived from it too."))
        .flag(f("priority", "p", I, "The evaluator's priority: higher ones claim nodes first. Priorities are unique."))
        .flag(f("valueName", "vn", S, "Query: a value the evaluator answers.")),
    );
    c.register(
        q_flags(Cmd::new(
            "deformerEvaluator",
            "The deformer evaluator's state: what it takes (deformation chains of the deformers it supports, on large enough \
             meshes, not blocked), with -ls the nodes on the GPU, -d the deformers it supports, -dch the chains, -mbr, -msg, \
             -ns, -ni about the nodes named or selected (-act every active one), -di and -dol everything. Shown in the Script \
             Editor, or returned with -q.",
            deformer_evaluator,
        ))
        .positional("objects")
        .undoable(false)
        .flag(f("active", "act", B, "Every active node, in place of the selection."))
        .flag(f("allowDownloadRejections", "adr", A, "Whether parts of the graph that depend on downloads may be rejected."))
        .flag(f("allowDownloads", "adl", A, "Whether data may come back from the GPU to the CPU."))
        .flag(f("asNodeName", "nm", B, "Answers start with the node's name."))
        .flag(f("asText", "txt", B, "Node states as text, not codes."))
        .flag(f("deformerChain", "dch", B, "The deformation chains of the meshes."))
        .flag(f("deformers", "d", B, "The deformers the evaluator supports."))
        .flag(f("dumpInfo", "di", B, "Every supported chain, as JSON."))
        .flag(f("dumpOutliner", "dol", B, "Every supported node, as a Python list."))
        .flag(f("gpuBlockPolicy", "gbp", A, "A node's GPU blocking policy: off, on, group, upstream, upstreamExcl, upstreamMesh, downstream, downstreamExcl, groupDownload."))
        .flag(f("isOpenCLValid", "clv", A, "Whether OpenCL is valid and set up."))
        .flag(f("limitMinimumVerts", "lmv", A, "Whether meshes under the minimum vertex count stay off the GPU."))
        .flag(f("list", "ls", B, "The nodes on the GPU."))
        .flag(f("members", "mbr", B, "The nodes in the same cluster as those named."))
        .flag(f("meshes", "m", B, "Only meshes."))
        .flag(f("message", "msg", B, "Why the nodes are or are not on the GPU."))
        .flag(f("minimumVerts", "mnv", I, "The fewest vertices a mesh needs to go on the GPU (500)."))
        .flag(f("nodeInfo", "ni", B, "All that partitioning found about the nodes."))
        .flag(f("nodeStatus", "ns", B, "The nodes' states on the GPU, as codes (0 not handled, 1 active, 2 unsupported, 3 too few vertices, 4 blocked, 5 evaluator off, 6 no OpenCL) or with -txt as text."))
        .flag(f("omitPassthroughs", "opt", B, "Leave out the nodes that pass their input on (groupParts, groupId, tweak)."))
        .flag(f("outlinerHash", "olh", B, "A hash of what -dol shows."))
        .flag(f("partition", "prt", B, "Partition the graph again."))
        .flag(f("purge", "prg", B, "Drop the data of unused GPU nodes."))
        .flag(f("reuseMode", "ru", A, "How GPU nodes are kept on partitioning, for deformers and display meshes: never, immediate or always.").multi(2))
        .flag(f("verbose", "v", A, "More detail.")),
    );
}
