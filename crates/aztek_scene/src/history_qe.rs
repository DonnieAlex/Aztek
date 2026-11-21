//! Query and edit on commands that make a history node, as the reference's NURBS commands work: `circle -q -r circle1`
//! reads the makeNurbCircle node's radius, `attachCurve -e -kmk 0 attachCurve1` changes the node. The name given may be
//! the node or an object whose history holds one. Also the -caching and -nodeState every such command takes.

use std::collections::VecDeque;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, FlagDef};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::node::NodeData;
use crate::plug::{flags, NodeId, PlugRef};
use crate::session::Session;

pub(crate) type Cmd = CommandDef<Session>;

pub(crate) fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

/// Adds a flag, or replaces the one of that name. A short name another flag holds is taken from it.
pub(crate) fn upsert(mut c: Cmd, fl: FlagDef) -> Cmd {
    if !fl.short_name.is_empty() {
        for other in c.flags.iter_mut().filter(|o| o.name != fl.name && o.short_name == fl.short_name) {
            other.short_name.clear();
        }
    }
    match c.flags.iter_mut().find(|o| o.name == fl.name) {
        Some(o) => *o = fl,
        None => c.flags.push(fl),
    }
    c
}

/// -query, -edit and the list of what a query asks for.
pub(crate) fn qe_flags(c: Cmd) -> Cmd {
    let c = upsert(c, f("query", "q", ArgType::Bool, "Query mode: answer the flags named from the node."));
    let c = upsert(c, f("edit", "e", ArgType::Bool, "Edit mode: change the node's settings the flags name."));
    upsert(c, f("queried", "", ArgType::List, "Flags asked about in query mode."))
}

/// -caching and -nodeState, which every history node takes.
pub(crate) fn node_flags(c: Cmd) -> Cmd {
    let c = upsert(c, f("caching", "cch", ArgType::Bool, "The node's caching: keeps every attribute computed so nothing is computed again."));
    upsert(
        c,
        f("nodeState", "nds", ArgType::Int, "The node's state: 0 normal, 1 has no effect (the input passes through), 2 blocking (it stops computing and keeps what it gave)."),
    )
}

/// The every-node nodeState of a node as it computes.
pub(crate) fn node_state(d: &NodeData) -> i64 {
    d.ty.attr_index("nodeState").map(|i| d.int(i)).unwrap_or(0)
}

/// Sets -caching and -nodeState on a node just made.
pub(crate) fn apply_common(g: &mut Graph, node: NodeId, a: &Args) -> CmdResult<()> {
    for name in ["caching", "nodeState"] {
        let Some(v) = a.get(name) else { continue };
        let Some(i) = g.node(node).and_then(|n| n.ty().attr_index(name)) else { continue };
        let def = g.node(node).unwrap().ty().attrs[i].clone();
        g.set_value(PlugRef::new(node, i as i32), crate::plug::from_json(&def, v)?)?;
    }
    Ok(())
}

fn type_matches(g: &Graph, id: NodeId, types: &[&str]) -> bool {
    g.node(id).is_some_and(|n| types.iter().any(|t| n.ty().is_a(t)))
}

/// The node of one of these types a name stands for: the node itself, or the nearest one upstream of the object.
pub(crate) fn find_hist(g: &Graph, name: &str, types: &[&str]) -> Option<NodeId> {
    let id = g.find(name).ok()??;
    if type_matches(g, id, types) {
        return Some(id);
    }
    let mut start = vec![id];
    if g.node(id).is_some_and(|n| n.ty().is_a("transform")) {
        start.extend(crate::dag::shapes(g, id, false));
    }
    let mut seen: Vec<NodeId> = start.clone();
    let mut queue: VecDeque<NodeId> = start.into();
    while let Some(n) = queue.pop_front() {
        if seen.len() > 400 {
            break;
        }
        let Some(node) = g.node(n) else { continue };
        let count = node.ty().attrs.len();
        for i in 0..count {
            let p = PlugRef::new(n, i as i32);
            let mut srcs: Vec<NodeId> = g.source(&p).map(|s| s.node).into_iter().collect();
            if g.node(n).is_some_and(|x| x.ty().attrs[i].flags & flags::ARRAY != 0) {
                srcs.extend(g.elements(&p).iter().filter_map(|e| g.source(e)).map(|s| s.node));
            }
            for s in srcs {
                if seen.contains(&s) {
                    continue;
                }
                if type_matches(g, s, types) {
                    return Some(s);
                }
                seen.push(s);
                queue.push_back(s);
            }
        }
    }
    None
}

/// Flags that steer the command rather than name a setting of the node.
const META: &[&str] = &["query", "edit", "queried", "name", "constructionHistory", "replaceOriginal", "object", "polygon", "range", "args"];

/// The node attribute a flag stands for.
fn attr_for<'a>(flag: &'a str, map: &[(&'a str, &'a str)]) -> &'a str {
    map.iter().find(|m| m.0 == flag).map_or(flag, |m| m.1)
}

/// Answers -q and carries out -e for a command whose settings live on a history node of one of `types`. None when the
/// call is neither. `map` names the attribute a flag stands for where the names differ.
pub(crate) fn qe(s: &mut Session, a: &Args, cmd: &str, types: &[&str], map: &[(&str, &str)]) -> Option<CmdResult> {
    if !a.bool("query") && !a.bool("edit") {
        return None;
    }
    Some(qe_run(s, a, cmd, types, map))
}

fn qe_run(s: &mut Session, a: &Args, cmd: &str, types: &[&str], map: &[(&str, &str)]) -> CmdResult {
    let names = objects(s, a);
    let Some(first) = names.first() else { fail!("{cmd}: name the {} node or an object it feeds", types[0]) };
    let Some(node) = find_hist(&s.graph, first, types) else { fail!("{cmd}: {first} has no {} node", types[0]) };
    let path = s.graph.path(node);
    if a.bool("query") {
        let asked: Vec<String> = match a.get("queried").and_then(Value::as_array) {
            Some(q) if !q.is_empty() => q.iter().filter_map(Value::as_str).map(str::to_string).collect(),
            _ => a.flags.iter().filter(|(k, v)| !META.contains(&k.as_str()) && v.as_bool() == Some(true)).map(|(k, _)| k.clone()).collect(),
        };
        let Some(flag) = asked.into_iter().find(|k| !META.contains(&k.as_str())) else { fail!("{cmd}: name a flag to query") };
        let attr = attr_for(&flag, map);
        let plug_name = format!("{path}.{attr}");
        let Ok(p) = s.graph.plug(&plug_name) else { fail!("{cmd}: -{flag} can't be queried") };
        let (unit, ty) = (s.graph.attr(&p).unit, s.graph.attr(&p).ty);
        let mut v = s.run("getAttr", json!({"args": [plug_name]}))?;
        // Whole numbers and switches answer as such.
        match ty {
            crate::plug::PlugType::Int | crate::plug::PlugType::Enum => {
                if let Some(x) = v.as_f64() {
                    v = json!(x.round() as i64);
                }
            }
            crate::plug::PlugType::Bool => {
                if let Some(x) = v.as_f64() {
                    v = json!(x != 0.0);
                }
            }
            _ => {}
        }
        // getAttr wraps a compound's value in a list of one.
        if let Value::Array(outer) = &v {
            if outer.len() == 1 && outer[0].is_array() {
                v = outer[0].clone();
            }
        }
        if aztek_ops::units::user_call() {
            aztek_ops::units::json_to_user(unit, &mut v);
        }
        return Ok(v);
    }
    let edits: Vec<(String, Value)> = a.flags.iter().filter(|(k, v)| !META.contains(&k.as_str()) && !v.is_null()).map(|(k, v)| (k.clone(), v.clone())).collect();
    for (flag, v) in edits {
        let attr = attr_for(&flag, map);
        let plug_name = format!("{path}.{attr}");
        if s.graph.plug(&plug_name).is_err() {
            fail!("{cmd}: -{flag} can't be edited");
        }
        let mut args = vec![json!(plug_name)];
        match v {
            Value::Array(x) => args.extend(x),
            Value::Bool(b) => args.push(json!(b as i64)),
            x => args.push(x),
        }
        s.run("setAttr", json!({ "args": args }))?;
    }
    s.graph.evaluate();
    Ok(json!([path]))
}

/// Gives a registered command -q, -e, -caching and -nodeState on its history node, which the call's result names. `extra`
/// are further flags of the reference's that the node's attributes carry.
pub(crate) fn extend(c: &mut aztek_ops::Commands<Session>, name: &str, types: &'static [&'static str], map: &'static [(&'static str, &'static str)], extra: Vec<FlagDef>) {
    let Some(old) = c.find(name).cloned() else {
        log::warn!("{name}: nothing to extend");
        return;
    };
    let run = old.run.clone();
    let mut cmd = node_flags(qe_flags(old));
    for fl in extra {
        cmd = upsert(cmd, fl);
    }
    let cname = name.to_string();
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if let Some(r) = qe(s, a, &cname, types, map) {
            return r;
        }
        let out = run(s, a)?;
        if a.has("caching") || a.has("nodeState") {
            let names: Vec<String> = match &out {
                Value::Array(v) => v.iter().filter_map(Value::as_str).map(str::to_string).collect(),
                Value::String(x) => vec![x.clone()],
                _ => Vec::new(),
            };
            for n in names {
                if let Ok(Some(id)) = s.graph.find(&n) {
                    if type_matches(&s.graph, id, types) {
                        apply_common(&mut s.graph, id, a)?;
                    }
                }
            }
        }
        Ok(out)
    });
    c.register(cmd);
}
