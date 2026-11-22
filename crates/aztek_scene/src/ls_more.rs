//! The rest of ls's flags: more kinds of node (containers, partitions, references, render settings, construction
//! planes, ghosts, live objects, templated ones, persistent nodes), -excludeType, -modified, -recursive across
//! namespaces, -showNamespace with -absoluteName, -allPaths for instances, -orderedComponentSelection.

use std::collections::HashSet;

use aztek_ops::{fail, ArgType, Args, CmdResult, Commands, FlagDef, RunFn};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::{NodeId, PlugValue};
use crate::session::Session;

/// The attribute `plane` puts on the shape of a construction plane, which ls -planes looks for.
pub const PLANE_ATTR: &str = "constructionPlane";

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

fn flag_bool(g: &mut Graph, id: NodeId, attr: &str) -> Option<bool> {
    let p = g.plug(&format!("{}.{attr}", g.path(id))).ok()?;
    match g.value(&p).ok()? {
        PlugValue::Bool(b) => Some(b),
        PlugValue::Int(i) => Some(i != 0),
        _ => None,
    }
}

const OWN: &[&str] = &[
    "absoluteName", "allPaths", "containerType", "containers", "dependencyNodes", "excludeType", "ghost", "live", "modified", "orderedComponentSelection", "partitions", "persistentNodes", "planes", "preSelectHilite", "recursive", "references",
    "renderGlobals", "renderQualities", "renderResolutions", "renderSetups", "showNamespace", "templated", "ufeObjects", "untemplated",
];

fn run(s: &mut Session, a: &Args, prev: &RunFn<Session>) -> CmdResult {
    if !OWN.iter().any(|f| a.has(f)) {
        return prev(s, a);
    }
    let on = |f: &str| a.bool(f);
    if a.has("containerType") && (a.has("type") || a.has("exactType")) {
        fail!("ls: -containerType can't be used with -type or -exactType");
    }
    if a.has("excludeType") && (a.has("type") || a.has("exactType")) {
        fail!("ls: -excludeType can't be used with -type or -exactType");
    }
    if on("showNamespace") && on("showType") {
        fail!("ls: -showNamespace can't be used with -showType");
    }
    if on("persistentNodes") {
        let mut list = crate::ref_scene::INTERNAL_NODES.read().unwrap().map(|f| f()).unwrap_or_default();
        let names = a.named();
        if !names.is_empty() {
            list.retain(|n| names.iter().any(|p| crate::ref_core::glob(p, n)));
        }
        return Ok(json!(list));
    }
    if on("preSelectHilite") {
        // Pre-selection is the viewport's, drawn under the mouse; nothing is pre-selected outside it.
        return Ok(json!([]));
    }
    let mut call = a.clone();
    for f in OWN {
        call.flags.remove(*f);
    }
    let show_type = call.bool("showType");
    call.flags.remove("showType");
    // -recursive: a name without a namespace matches in every namespace.
    if on("recursive") {
        let g = &s.graph;
        let mut out: Vec<Value> = Vec::new();
        for p in a.named() {
            if p.contains(':') || p.contains('.') || p.contains('|') {
                out.push(json!(p));
                continue;
            }
            let mut any = false;
            for id in g.nodes() {
                let Some(n) = g.node(id) else { continue };
                let leaf = n.name().rsplit(':').next().unwrap_or(n.name());
                if crate::ref_core::glob(&p, leaf) {
                    out.push(json!(g.path(id)));
                    any = true;
                }
            }
            if !any {
                out.push(json!(p));
            }
        }
        if !out.is_empty() {
            call.positional = out;
        }
    }
    let r = prev(s, &call)?;
    let listed: Vec<String> = strings(Some(&r));
    let excluded = strings(a.get("excludeType"));
    let ctypes = strings(a.get("containerType"));
    let modified: HashSet<NodeId> = s.graph.modified_nodes().into_iter().collect();
    let live: Vec<NodeId> = s.live_surface.clone();
    let mut kept: Vec<(String, Option<NodeId>)> = Vec::new();
    for name in listed {
        let id = if name.contains('.') { None } else { s.graph.find(&name).ok().flatten() };
        let Some(id) = id else {
            // Components and plugs pass only when no node kind is asked for.
            let kinds = ["containers", "containerType", "partitions", "references", "renderGlobals", "renderSetups", "renderQualities", "renderResolutions", "planes", "ghost", "live", "modified", "templated", "untemplated"];
            if !kinds.iter().any(|k| a.has(k) && a.get(k) != Some(&Value::Bool(false))) {
                kept.push((name, None));
            }
            continue;
        };
        let g = &mut s.graph;
        let Some(n) = g.node(id) else { continue };
        let ty = n.type_arc().clone();
        let dag = ty.dag;
        let mut ok = true;
        if !excluded.is_empty() {
            ok &= !excluded.iter().any(|t| ty.is_a(t));
        }
        if on("containers") {
            ok &= ty.is_a("container") || ty.name == "dagContainer";
        }
        if !ctypes.is_empty() {
            ok &= (ty.is_a("container") || ty.name == "dagContainer") && ctypes.iter().any(|t| *t == ty.name || t == "container");
        }
        if on("partitions") {
            ok &= ty.name == "partition";
        }
        if on("references") {
            let nm = n.name().to_string();
            ok &= ty.name == crate::references::REF_NODE_TYPE && nm != "sharedReferenceNode" && !nm.starts_with("_UNKNOWN_REF_NODE_");
        }
        if on("renderGlobals") || on("renderSetups") {
            ok &= ty.name == "renderGlobals";
        }
        if on("renderQualities") {
            ok &= ty.name == "renderQuality";
        }
        if on("renderResolutions") {
            ok &= ty.name == "resolution";
        }
        if on("modified") {
            ok &= modified.contains(&id);
        }
        if on("live") {
            ok &= live.contains(&id) || crate::dag::shapes(g, id, false).iter().any(|sh| live.contains(sh));
        }
        if on("planes") {
            ok &= dag && !ty.is_a("transform") && flag_bool(g, id, PLANE_ATTR) == Some(true);
        }
        if on("ghost") {
            ok &= dag && flag_bool(g, id, "ghosting") == Some(true);
        }
        if on("templated") {
            ok &= dag && flag_bool(g, id, "template") == Some(true);
        }
        if on("untemplated") {
            ok &= dag && flag_bool(g, id, "template") != Some(true);
        }
        if ok {
            kept.push((name, Some(id)));
        }
    }
    // -allPaths: a shape shared by instances, once for each of its places.
    if on("allPaths") {
        let mut out = Vec::new();
        for (name, id) in kept {
            let members = id.map(|i| crate::instancing::members(&s.graph, i)).unwrap_or_default();
            if members.len() > 1 {
                for m in members {
                    let p = s.graph.full_path(m);
                    if !out.iter().any(|x: &(String, Option<NodeId>)| x.0 == p) {
                        out.push((p, Some(m)));
                    }
                }
            } else {
                out.push((name, id));
            }
        }
        kept = out;
    }
    // -orderedComponentSelection: components together by their object, objects in the order first picked.
    if on("orderedComponentSelection") {
        let mut order: Vec<String> = Vec::new();
        for (name, _) in &kept {
            let obj = name.split('.').next().unwrap_or(name).to_string();
            if !order.contains(&obj) {
                order.push(obj);
            }
        }
        let mut out = Vec::new();
        for o in order {
            out.extend(kept.iter().filter(|(n, _)| n.split('.').next() == Some(o.as_str())).cloned());
        }
        kept = out;
    }
    if on("showNamespace") {
        let absolute = on("absoluteName");
        let mut out = Vec::new();
        for (name, id) in kept {
            let ns = id.and_then(|i| s.graph.node(i).map(|n| n.namespace().to_string())).unwrap_or_else(|| {
                let leaf = name.split('.').next().unwrap_or(&name).rsplit('|').next().unwrap_or("").to_string();
                leaf.rsplit_once(':').map(|x| x.0.to_string()).unwrap_or_default()
            });
            out.push(json!(name));
            out.push(json!(if ns.is_empty() { ":".to_string() } else if absolute { format!(":{ns}") } else { ns }));
        }
        return Ok(Value::Array(out));
    }
    if show_type {
        let mut out = Vec::new();
        for (name, id) in kept {
            let t = id.and_then(|i| s.graph.node(i).map(|n| n.ty().name.clone())).unwrap_or_default();
            out.push(json!(name));
            out.push(json!(t));
        }
        return Ok(Value::Array(out));
    }
    Ok(json!(kept.into_iter().map(|x| x.0).collect::<Vec<_>>()))
}

pub fn register(c: &mut Commands<Session>) {
    let Some(old) = c.find("ls").cloned() else { return };
    let mut def = old;
    let prev = def.run.clone();
    let b = |n: &str, sh: &str, doc: &str| FlagDef::new(n, sh, ArgType::Bool, Value::Null, doc);
    let m = |n: &str, sh: &str, doc: &str| FlagDef::new(n, sh, ArgType::Any, Value::Null, doc).multi(1);
    for fl in [
        b("absoluteName", "an", "With -showNamespace: namespaces from the root, :a:b."),
        b("allPaths", "ap", "Every path to an instanced shape, with -dag or names."),
        m("containerType", "ct", "Only containers of this type; several -ct for several."),
        b("containers", "con", "Only containers."),
        b("dependencyNodes", "dep", "Dependency nodes, DAG ones included (every node)."),
        m("excludeType", "ext", "Leave out nodes of this type or derived; several -ext for several."),
        b("ghost", "gh", "Only ghosted objects."),
        b("live", "lv", "Only the live objects."),
        b("modified", "mod", "Only nodes changed since the scene was last opened, saved or made new."),
        b("orderedComponentSelection", "ocs", "With -sl: components together by their object."),
        b("partitions", "pr", "Only partitions."),
        b("persistentNodes", "pn", "The nodes a new scene keeps: item filters and the like."),
        b("planes", "pl", "Only construction planes."),
        b("preSelectHilite", "psh", "The components hilited for pre-selection."),
        b("recursive", "r", "Names match in every namespace."),
        b("references", "rf", "Only reference nodes of files."),
        b("renderGlobals", "rg", "Only render settings nodes."),
        b("renderQualities", "rq", "Only render quality nodes."),
        b("renderResolutions", "rr", "Only resolution nodes."),
        b("renderSetups", "rs", "As -renderGlobals."),
        b("showNamespace", "sns", "Each name followed by its namespace."),
        b("templated", "tm", "Only templated DAG nodes."),
        b("ufeObjects", "ufe", "With -sl: other objects in the selection as well."),
        b("untemplated", "ut", "Only DAG nodes that aren't templated."),
    ] {
        def.flags.retain(|o| o.name != fl.name);
        def.flags.push(fl);
    }
    def.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| run(s, a, &prev));
    c.register(def);
}
