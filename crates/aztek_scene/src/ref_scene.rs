//! The reference's scene commands with their full flag sets: hierarchy, naming, selection, sets, namespaces, undo and the small
//! commands scripts lean on. Older commands of the same name keep doing what they did; these add to them.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use aztek_ops::{fail, objects, run, ArgType, Args, CmdResult, CommandDef, Commands};
use glam::DVec3;
use serde_json::{json, Value};

use crate::graph::{Graph, SelectMode};
use crate::ref_core::{asked, extend, f, matching_nodes};
use crate::node::types;
use crate::plug::{NodeId, PlugRef, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::{DagNode, Transform};

type Cmd = CommandDef<Session>;
const B: ArgType = ArgType::Bool;
const S: ArgType = ArgType::String;
const A: ArgType = ArgType::Any;

/// A relative as listRelatives names it: its leaf name, even if that isn't unique, a path only when asked for.
fn relative_name(g: &Graph, id: NodeId, a: &Args) -> String {
    if a.bool("fullPath") {
        g.full_path(id)
    } else if a.bool("path") || !aztek_ops::units::user_call() {
        g.path(id)
    } else {
        g.node(id).map_or_else(String::new, |n| n.name().to_string())
    }
}

fn type_list(a: &Args, flag: &str) -> Vec<String> {
    match a.get(flag) {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

fn node_of(g: &Graph, name: &str, cmd: &str) -> CmdResult<NodeId> {
    g.find(name)?.ok_or_else(|| aztek_ops::CommandError::new(format!("{cmd}: no object named {name}")))
}

fn list_relatives(s: &mut Session, a: &Args) -> CmdResult {
    let g = &s.graph;
    let types_wanted = type_list(a, "type");
    let ni = a.bool("noIntermediate");
    let mut out: Vec<NodeId> = Vec::new();
    for name in objects(s, a) {
        let id = node_of(g, &name, "listRelatives")?;
        let found: Vec<NodeId> = if a.bool("allParents") {
            // An instanced shape has a parent per instance.
            let shared = crate::instancing::one_shape(g, id);
            let all = if shared.len() > 1 { shared } else { vec![id] };
            all.into_iter().map(|m| g.parent(m)).filter(|&p| p != NO_NODE).collect()
        } else if a.bool("parent") {
            let p = g.parent(id);
            if p == NO_NODE { Vec::new() } else { vec![p] }
        } else if a.bool("allDescendents") {
            // Deepest first, as the reference lists them.
            let mut d = g.descendants(id);
            d.reverse();
            d
        } else if a.bool("shapes") {
            if crate::dag::is_shape(g, id) { vec![id] } else { g.children(id).to_vec() }
        } else {
            g.children(id).to_vec()
        };
        for x in found {
            if !out.contains(&x) {
                out.push(x);
            }
        }
    }
    out.retain(|&x| {
        let Some(n) = g.node(x) else { return false };
        (!a.bool("shapes") || crate::dag::is_shape(g, x))
            && (!ni || !n.ty().dag || !n.bool(DagNode::INTERMEDIATE_OBJECT))
            && (types_wanted.is_empty() || types_wanted.iter().any(|t| n.ty().is_a(t)))
    });
    Ok(json!(out.iter().map(|&x| relative_name(g, x, a)).collect::<Vec<_>>()))
}

/// listConnections: walking through unitConversion nodes when asked, a shape reported as its transform unless -sh.
fn list_connections(s: &mut Session, a: &Args) -> CmdResult {
    let g = &s.graph;
    let want_src = a.get("source").and_then(Value::as_bool).unwrap_or(true);
    let want_dst = a.get("destination").and_then(Value::as_bool).unwrap_or(true);
    let (plugs, pairs, shapes, skip) = (a.bool("plugs"), a.bool("connections"), a.bool("shapes"), a.bool("skipConversionNodes"));
    let (ty, exact) = (type_list(a, "type"), type_list(a, "exactType"));
    let conns = g.connections();
    let conversion = |id: NodeId| g.node(id).is_some_and(|n| n.ty().name == "unitConversion");
    let mut out: Vec<Value> = Vec::new();
    for name in objects(s, a) {
        let (node, attr): (NodeId, Option<PlugRef>) = if name.contains('.') {
            let p = g.plug(&name)?;
            (p.node, Some(p))
        } else {
            (node_of(g, &name, "listConnections")?, None)
        };
        let matches = |p: &PlugRef| {
            p.node == node
                && attr.as_ref().is_none_or(|q| {
                    p.attr == q.attr
                        && (q.element < 0 || p.element < 0 || p.element == q.element)
                        && (q.component < 0 || p.component < 0 || p.component == q.component)
                })
        };
        for (d, sp) in &conns {
            let mut hits = Vec::new();
            if want_dst && matches(sp) {
                hits.push((*sp, *d));
            }
            if want_src && matches(d) {
                hits.push((*d, *sp));
            }
            for (mine, mut other) in hits {
                if skip {
                    // Walk through conversion nodes to what is beyond them.
                    let upstream = other == *sp;
                    let mut guard = 0;
                    while conversion(other.node) && guard < 32 {
                        guard += 1;
                        let next = if upstream {
                            conns.iter().find(|(x, _)| x.node == other.node).map(|(_, y)| *y)
                        } else {
                            conns.iter().find(|(_, y)| y.node == other.node).map(|(x, _)| *x)
                        };
                        let Some(n) = next else { break };
                        other = n;
                    }
                }
                let Some(on) = g.node(other.node) else { continue };
                if (!ty.is_empty() && !ty.iter().any(|t| on.ty().is_a(t))) || (!exact.is_empty() && !exact.iter().any(|t| on.ty().name == *t)) {
                    continue;
                }
                // -fullNodeName: nodes by their full DAG path.
                let full = a.bool("fullNodeName");
                let plug_text = |p: &PlugRef| {
                    let name = g.plug_name(p);
                    if !full {
                        return name;
                    }
                    let short = g.path(p.node);
                    match name.strip_prefix(&short) {
                        Some(rest) => format!("{}{rest}", g.full_name(p.node)),
                        None => name,
                    }
                };
                if pairs {
                    out.push(json!(plug_text(&mine)));
                }
                let shown = if !shapes && crate::dag::is_shape(g, other.node) { crate::dag::transform_of(g, other.node) } else { other.node };
                let v = if plugs {
                    json!(plug_text(&other))
                } else if full {
                    json!(g.full_name(shown))
                } else {
                    json!(g.path(shown))
                };
                if pairs || !out.contains(&v) {
                    out.push(v);
                }
            }
        }
    }
    Ok(Value::Array(out))
}

fn is_constraint(t: &crate::node::NodeType) -> bool {
    t.name.ends_with("Constraint")
}

fn is_anim_curve(t: &crate::node::NodeType) -> bool {
    t.name.starts_with("animCurve")
}

/// dgdirty: dirty messages, cleaning, and the lists of what is dirty or clean.
fn dgdirty(s: &mut Session, a: &Args) -> CmdResult {
    const KINDS: [(&str, &str, &str); 3] = [("d", "data", "plugs with dirty or clean data"), ("p", "plug", "plugs in a dirty or clean state"), ("c", "connection", "connections in a dirty or clean state, as pairs")];
    if a.bool("query") {
        // The kinds of list, or what one is.
        if let Some(v) = a.get("list").and_then(Value::as_str).filter(|v| *v != "true") {
            let Some(k) = KINDS.iter().find(|k| k.0 == v || k.1 == v) else { fail!("dgdirty: -list {v} is not data, plug or connection") };
            return Ok(json!(format!("{}: {}", k.1, k.2)));
        }
        return Ok(json!(KINDS.iter().map(|k| format!("{}/{}", k.0, k.1)).collect::<Vec<_>>()));
    }
    let started = std::time::Instant::now();
    let names = objects(s, a);
    let g = &mut s.graph;
    let all = a.bool("allPlugs") || (names.is_empty() && a.get("list").is_some());
    let mut ids: Vec<NodeId> = Vec::new();
    let mut plugs: Vec<PlugRef> = Vec::new();
    if all {
        ids = g.nodes().into_iter().filter(|&id| a.bool("implicit") || g.node(id).is_some_and(|n| !(n.namespace.is_empty() && crate::defaults::is_default_node(n.name())))).collect();
    } else {
        for n in &names {
            if n.contains('.') {
                plugs.push(g.plug(n)?);
            } else {
                ids.push(node_of(g, n, "dgdirty")?);
            }
        }
    }
    // A node's plugs that carry anything: those connected.
    let conns = g.connections();
    for &id in &ids {
        for (d, sr) in &conns {
            for p in [d, sr] {
                if p.node == id && !plugs.contains(p) {
                    plugs.push(*p);
                }
            }
        }
    }
    let clean = a.bool("clean");
    if let Some(kind) = a.get("list") {
        let kind = kind.as_str().unwrap_or("data");
        let Some(k) = KINDS.iter().find(|k| k.0 == kind || k.1 == kind) else { fail!("dgdirty: -list {kind} is not data, plug or connection") };
        let dirty_node = |g: &Graph, n: NodeId| g.node(n).is_some_and(|x| x.is_dirty());
        let mut out = Vec::new();
        if k.1 == "connection" {
            for (d, sr) in &conns {
                if (plugs.contains(d) || plugs.contains(sr)) && dirty_node(g, d.node) != clean {
                    out.push(json!(g.plug_name(sr)));
                    out.push(json!(g.plug_name(d)));
                }
            }
        } else {
            for p in &plugs {
                // A plug's data is dirty while its node waits to compute; its state, while it is the destination of a
                // dirty connection.
                let dirty = if k.1 == "data" { dirty_node(g, p.node) } else { dirty_node(g, p.node) && g.source(p).is_some_and(|sr| dirty_node(g, sr.node) || dirty_node(g, p.node)) };
                if dirty != clean {
                    out.push(json!(g.plug_name(p)));
                }
            }
        }
        return Ok(Value::Array(out));
    }
    if clean {
        g.evaluate();
        for p in &plugs {
            let _ = g.value(p);
        }
    } else {
        let mut touched: Vec<NodeId> = plugs.iter().map(|p| p.node).collect();
        touched.sort();
        touched.dedup();
        for id in touched {
            g.touch(id);
        }
    }
    if a.bool("verbose") {
        for p in &plugs {
            println!("{} {}", if clean { "clean" } else { "dirty" }, g.plug_name(p));
        }
    }
    if a.bool("showTiming") {
        println!("// Messages took {} microseconds to process.", started.elapsed().as_micros());
    }
    Ok(json!(plugs.len()))
}

fn delete_cmd(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    if a.bool("constructionHistory") {
        return prev(s, a);
    }
    let selective = ["channels", "expressions", "constraints", "staticChannels", "timeAnimationCurves", "unitlessAnimationCurves", "shape", "inputConnectionsAndNodes"].iter().any(|k| a.bool(k)) || a.has("attribute");
    if a.bool("all") && !selective {
        // -all names no kind of its own: with no other flag there is nothing to delete.
        return Ok(Value::Null);
    }
    if a.bool("all") && a.bool("constraints") {
        // Every constraint in the scene.
        let g = &mut s.graph;
        let all: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| is_constraint(n.ty()))).collect();
        for id in all {
            if g.node(id).is_some() {
                g.delete_node(id)?;
            }
        }
        return Ok(Value::Null);
    }
    if !selective {
        // What a plain delete leaves driving nothing goes too.
        let mut doomed = Vec::new();
        for n in objects(s, a) {
            if let Some(id) = s.graph.find(&n)? {
                doomed.push(id);
                doomed.extend(s.graph.descendants(id));
            }
        }
        let orphans = crate::expression::orphans(&s.graph, &doomed);
        let r = prev(s, a)?;
        for o in orphans {
            if s.graph.node(o).is_some() {
                s.graph.delete_node(o)?;
            }
        }
        return Ok(r);
    }
    let mut targets = Vec::new();
    // -all: every node in the scene, for the kinds the other flags name.
    let names: Vec<String> = if a.bool("all") { s.graph.nodes().into_iter().map(|id| s.graph.path(id)).collect() } else { objects(s, a) };
    for n in names {
        let id = if n.contains('.') { s.graph.plug(&n)?.node } else { node_of(&s.graph, &n, "delete")? };
        targets.push(id);
        if a.string("hierarchy", "none") == "below" || a.string("hierarchy", "none") == "both" {
            targets.extend(s.graph.descendants(id));
        }
    }
    let attrs = type_list(a, "attribute");
    let g = &mut s.graph;
    for id in targets {
        if a.bool("shape") {
            for sh in crate::dag::shapes(g, id, true) {
                g.delete_node(sh)?;
            }
            continue;
        }
        let mut doomed = Vec::new();
        for (d, sp) in g.connections() {
            if d.node != id {
                continue;
            }
            if !attrs.is_empty() {
                let def = g.attr(&d);
                let hit = attrs.iter().any(|w| *w == def.name || *w == def.short_name || (d.component >= 0 && (*w == def.child_names[d.component as usize] || *w == def.child_short[d.component as usize])));
                if !hit {
                    continue;
                }
            }
            let Some(src) = g.node(sp.node) else { continue };
            let t = src.ty();
            let curve = is_anim_curve(t);
            let driven_key = t.name.starts_with("animCurveU");
            let hit = (a.bool("expressions") && t.name == "expression")
                || (a.bool("constraints") && is_constraint(t))
                || (a.bool("channels") && (curve || t.name == "expression"))
                || (a.bool("timeAnimationCurves") && curve && !driven_key)
                || (a.bool("unitlessAnimationCurves") && driven_key)
                || (a.bool("staticChannels") && curve && crate::anim::keys_of(g, sp.node).keys.windows(2).all(|w| (w[0].v - w[1].v).abs() < 1e-9))
                || a.bool("inputConnectionsAndNodes")
                || (a.has("attribute") && !a.bool("channels") && !a.bool("expressions") && !a.bool("constraints"));
            if hit && !doomed.contains(&sp.node) {
                doomed.push(sp.node);
            }
        }
        // Constraint nodes parented under the object count as its constraints too, and so does the symmetry constraint
        // of a joint driving its twin.
        if a.bool("constraints") {
            for (d, sp) in g.connections() {
                if sp.node == id && g.node(d.node).is_some_and(|n| n.ty().name == "symmetryConstraint") && !doomed.contains(&d.node) {
                    doomed.push(d.node);
                }
            }
            for &c in g.children(id).to_vec().iter() {
                if g.node(c).is_some_and(|n| is_constraint(n.ty())) && !doomed.contains(&c) {
                    doomed.push(c);
                }
            }
        }
        for d in doomed {
            if g.node(d).is_some() {
                g.delete_node(d)?;
            }
        }
    }
    Ok(Value::Null)
}

fn select_cmd(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    let g = &s.graph;
    let mut ids: Vec<NodeId> = Vec::new();
    let mut items: Vec<Value> = Vec::new();
    if a.bool("all") {
        // Every object, not the default cameras.
        ids = g.nodes().into_iter().filter(|&i| g.node(i).is_some_and(|n| n.ty().dag) && !crate::dag::is_shape(g, i) && !crate::cameras::is_default_camera(g, i)).collect();
    } else if a.bool("allDagObjects") {
        ids = g.roots().iter().copied().filter(|&i| g.node(i).is_some_and(|n| n.ty().dag)).collect();
    } else if a.bool("allDependencyNodes") {
        ids = g.nodes();
    }
    for name in a.named() {
        if crate::components::is_component_string(&name) {
            items.push(json!(name));
            continue;
        }
        let found = if name.contains('*') || name.contains('?') { matching_nodes(g, &name) } else { vec![node_of(g, &name, "select")?] };
        for id in found {
            // A set stands for its members, and those of the sets it holds, unless -ne.
            if !a.bool("noExpand") && g.node(id).is_some_and(crate::object_sets::is_set) {
                let list = crate::object_sets::flattened(g, id);
                for m in crate::object_sets::format(g, &list) {
                    items.push(json!(m));
                }
            } else {
                ids.push(id);
            }
        }
    }
    if a.bool("hierarchy") {
        let mut all = Vec::new();
        for &id in &ids {
            if !all.contains(&id) {
                all.push(id);
            }
            for d in g.descendants(id) {
                if !all.contains(&d) {
                    all.push(d);
                }
            }
        }
        ids = all;
    }
    if a.bool("visible") {
        ids.retain(|&i| crate::transform::effectively_visible(g, i));
    }
    let expanded = a.bool("all") || a.bool("allDagObjects") || a.bool("allDependencyNodes") || a.bool("hierarchy") || !a.named().is_empty();
    if a.bool("addFirst") {
        let mut front = ids.clone();
        front.extend(g.selection().iter().copied().filter(|i| !ids.contains(i)));
        s.graph.select(&front, SelectMode::Replace);
        return Ok(Value::Null);
    }
    if !expanded {
        return prev(s, a);
    }
    let mut call = a.clone();
    call.positional = ids.iter().map(|&i| json!(s.graph.path(i))).chain(items).collect();
    if call.positional.is_empty() && !a.bool("add") && !a.bool("deselect") && !a.bool("toggle") {
        s.graph.select(&[], SelectMode::Replace);
        return Ok(Value::Null);
    }
    if call.positional.is_empty() {
        return Ok(Value::Null);
    }
    prev(s, &call)
}

/// Copies a node's input connections onto its duplicate (duplicate -ic): the copy is fed by the very nodes that feed
/// the original, so they share their history.
fn copy_inputs(g: &mut Graph, from: NodeId, to: NodeId) -> CmdResult<()> {
    for (d, sp) in g.connections() {
        if d.node == from {
            g.connect(&sp, &PlugRef { node: to, ..d }, true)?;
        }
    }
    Ok(())
}

/// The copy of an upstream node for duplicate -un: a node of the duplicated hierarchy is its copy there; a DAG node
/// outside it, or one every scene has (time1), is shared; anything else is copied once, with what feeds it in turn.
fn upstream_copy(g: &mut Graph, n: NodeId, pairs: &HashMap<NodeId, NodeId>, made: &mut HashMap<NodeId, NodeId>) -> CmdResult<NodeId> {
    if let Some(&c) = pairs.get(&n).or_else(|| made.get(&n)) {
        return Ok(c);
    }
    let Some(ty) = g.node(n).map(|x| x.type_arc().clone()) else { return Ok(n) };
    if ty.dag || crate::defaults::undeletable(g, n) {
        return Ok(n);
    }
    let stem = g.node(n).map(|x| x.name().to_string()).unwrap_or_default();
    let name = unique_everywhere(g, &g.available_name(&stem, NO_NODE));
    let c = g.create_node(&ty, &name, NO_NODE, true)?;
    made.insert(n, c);
    crate::duplicate::copy_values(g, n, c)?;
    for (d, sp) in g.connections() {
        if d.node == n {
            let from = upstream_copy(g, sp.node, pairs, made)?;
            g.connect(&PlugRef { node: from, ..sp }, &PlugRef { node: c, ..d }, true)?;
        }
    }
    Ok(c)
}

/// Each node of a duplicated hierarchy with its copy: the top, then the descendants, matched in order while their types
/// agree.
fn hierarchy_pairs(g: &Graph, from: NodeId, to: NodeId) -> Vec<(NodeId, NodeId)> {
    let a: Vec<NodeId> = std::iter::once(from).chain(g.descendants(from)).collect();
    let b: Vec<NodeId> = std::iter::once(to).chain(g.descendants(to)).collect();
    let ty = |id: NodeId| g.node(id).map(|n| n.ty().name.clone());
    if a.len() != b.len() || a.iter().zip(&b).any(|(x, y)| ty(*x) != ty(*y)) {
        return vec![(from, to)];
    }
    a.into_iter().zip(b).collect()
}

/// The objects as duplicate takes them: a shape stands for its transform, and only the top of a picked hierarchy counts.
fn duplicate_sources(s: &Session, a: &Args) -> Vec<NodeId> {
    let g = &s.graph;
    let ids: Vec<NodeId> = objects(s, a)
        .iter()
        .filter_map(|n| g.find(n).ok().flatten())
        .map(|id| {
            let shape = g.node(id).is_some_and(|x| x.ty().dag && !x.ty().is_a("transform"));
            if shape && g.parent(id) != NO_NODE {
                g.parent(id)
            } else {
                id
            }
        })
        .collect();
    crate::transform_ops::top_level(g, &ids)
}

fn duplicate_cmd(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    if a.bool("smartTransform") {
        return run(s, "duplicateWithTransform", json!({}));
    }
    let sources = duplicate_sources(s, a);
    let count = a.int("count", 1).max(1) as usize;
    let instanced = a.string("mode", "copy") == "instance";
    // Group under: the copies' parent (the default), the world, or a new group (duplicate's own -group).
    let under = a.string("groupUnder", if a.bool("group") { "group" } else { "parent" });
    let mut call = a.clone();
    call.flags.insert("group".into(), json!(under == "group"));
    let made = prev(s, &call)?;
    let new: Vec<String> = made.as_array().map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let mut roots = Vec::new();
    for n in &new {
        if let Some(id) = s.graph.find(n)? {
            roots.push(id);
        }
    }
    let g = &mut s.graph;
    // The copies come source by source, `count` of each.
    let mut made_up = HashMap::new();
    for (k, &id) in roots.iter().enumerate() {
        let source = sources.get(k / count).copied();
        if a.bool("parentOnly") {
            for c in g.children(id).to_vec() {
                if !crate::dag::is_shape(g, c) {
                    g.delete_node(c)?;
                }
            }
            for c in g.children(id).to_vec() {
                g.delete_node(c)?;
            }
        } else if a.bool("transformsOnly") {
            for d in g.descendants(id) {
                if g.node(d).is_some() && crate::dag::is_shape(g, d) {
                    g.delete_node(d)?;
                }
            }
        }
        let Some(src) = source else { continue };
        let pairs = hierarchy_pairs(g, src, id);
        if instanced {
            continue;
        }
        if a.bool("instanceLeaf") {
            // The hierarchy is copied, its leaves are instances of the originals'.
            for &(from, to) in &pairs {
                if g.children(from).is_empty() && crate::dag::is_shape(g, from) {
                    let parent = g.parent(to);
                    let name = g.node(to).map(|x| x.name().to_string()).unwrap_or_default();
                    g.delete_node(to)?;
                    let mut copies = HashMap::new();
                    let c = crate::duplicate::copy_tree(g, from, parent, crate::duplicate::Mode::Instance, "", &mut copies)?;
                    g.rename(c, &name)?;
                }
            }
            continue;
        }
        if a.bool("inputConnections") {
            for &(from, to) in &pairs {
                copy_inputs(g, from, to)?;
            }
        } else if a.bool("upstreamNodes") {
            let map: HashMap<NodeId, NodeId> = pairs.iter().copied().collect();
            for &(from, to) in &pairs {
                for (d, sp) in g.connections() {
                    if d.node != from {
                        continue;
                    }
                    let up = upstream_copy(g, sp.node, &map, &mut made_up)?;
                    if up != sp.node || g.source(&PlugRef { node: to, ..d }).is_none() {
                        g.connect(&PlugRef { node: up, ..sp }, &PlugRef { node: to, ..d }, true)?;
                    }
                }
            }
        }
    }
    if under == "world" {
        for &id in &roots {
            if g.parent(id) != NO_NODE {
                g.set_parent(id, NO_NODE, true, usize::MAX)?;
            }
        }
    }
    let mut out = Vec::new();
    for &id in &roots {
        if a.bool("renameChildren") {
            for d in g.descendants(id) {
                let stem = g.node(d).unwrap().name().to_string();
                let wanted = g.available_name(&stem, NO_NODE);
                let unique = unique_everywhere(g, &wanted);
                g.rename(d, &unique)?;
            }
        }
        out.push(json!(g.path(id)));
        if !a.bool("returnRootsOnly") && !a.bool("parentOnly") {
            for d in g.descendants(id) {
                if !crate::dag::is_shape(g, d) {
                    out.push(json!(g.path(d)));
                }
            }
        }
    }
    Ok(Value::Array(out))
}

/// A name no node in the scene holds, counting up from what was asked.
fn unique_everywhere(g: &Graph, wanted: &str) -> String {
    if g.find_all(wanted).is_empty() {
        return wanted.to_string();
    }
    let digits = wanted.len() - wanted.bytes().rev().take_while(u8::is_ascii_digit).count();
    let stem = &wanted[..digits];
    let width = wanted.len() - digits;
    let mut k: u64 = wanted[digits..].parse().unwrap_or(1);
    loop {
        let name = format!("{stem}{k:0width$}");
        if g.find_all(&name).is_empty() {
            return name;
        }
        k += 1;
    }
}

fn parent_cmd(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    let objs = objects(s, a);
    if a.bool("removeObject") {
        for n in &objs {
            let id = node_of(&s.graph, n, "parent")?;
            s.graph.delete_node(id)?;
        }
        return Ok(Value::Null);
    }
    if a.bool("addObject") && a.bool("shape") && objs.len() >= 2 {
        // A shape added under another transform: the reference makes it one shape with two parents. Aztek has no
        // shapes with two parents, so the shape itself moves under the new transform, as named, and its old transform
        // keeps a copy. A script holding the shape (to rename it, as rigging tools do next) holds the one under the new
        // transform.
        let p = node_of(&s.graph, objs.last().unwrap(), "parent")?;
        let mut out = Vec::new();
        for n in &objs[..objs.len() - 1] {
            let id = node_of(&s.graph, n, "parent")?;
            let shapes = if crate::dag::is_shape(&s.graph, id) { vec![id] } else { crate::dag::shapes(&s.graph, id, true) };
            for sh in shapes {
                let old_parent = s.graph.parent(sh);
                if old_parent != NO_NODE {
                    let holder = s.graph.path(old_parent);
                    let made = run(s, "duplicate", json!({"args": [holder]}))?;
                    let copy_root = node_of(&s.graph, made[0].as_str().unwrap_or_default(), "parent")?;
                    let short = s.graph.node(sh).map(|x| x.name().to_string()).unwrap_or_default();
                    // The copy of this shape is the one of the same place among the copied transform's shapes.
                    let place = crate::dag::shapes(&s.graph, old_parent, true).iter().position(|&x| x == sh);
                    let copies = crate::dag::shapes(&s.graph, copy_root, true);
                    if let Some(c) = place.and_then(|i| copies.get(i).copied()) {
                        s.graph.set_parent(c, old_parent, false, usize::MAX)?;
                        let _ = s.graph.rename(c, &format!("{short}Orig"));
                    }
                    s.graph.delete_node(copy_root)?;
                }
                s.graph.set_parent(sh, p, false, usize::MAX)?;
                out.push(json!(s.graph.path(sh)));
            }
        }
        return Ok(Value::Array(out));
    }
    if a.bool("addObject") && objs.len() >= 2 {
        let parent = objs.last().unwrap().clone();
        let mut out = Vec::new();
        for n in &objs[..objs.len() - 1] {
            let made = run(s, "instance", json!({"args": [n]}))?;
            let name = made[0].as_str().unwrap_or_default().to_string();
            let id = node_of(&s.graph, &name, "parent")?;
            let p = node_of(&s.graph, &parent, "parent")?;
            s.graph.set_parent(id, p, !a.bool("relative"), usize::MAX)?;
            out.push(json!(s.graph.path(id)));
        }
        return Ok(Value::Array(out));
    }
    if a.bool("shape") && objs.len() >= 2 {
        // Shapes move under another transform as they are: this is how curve shapes join into one control.
        let p = node_of(&s.graph, objs.last().unwrap(), "parent")?;
        let mut out = Vec::new();
        for n in &objs[..objs.len() - 1] {
            let id = node_of(&s.graph, n, "parent")?;
            for sh in if crate::dag::is_shape(&s.graph, id) { vec![id] } else { crate::dag::shapes(&s.graph, id, true) } {
                s.graph.set_parent(sh, p, false, usize::MAX)?;
                out.push(json!(s.graph.path(sh)));
            }
        }
        return Ok(Value::Array(out));
    }
    prev(s, a)
}

fn group_cmd(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    // A group goes under its members' lowest common parent unless -w or -p says otherwise.
    let mut call = a.clone();
    if !a.bool("world") && !a.has("parent") && !a.bool("empty") {
        let g = &s.graph;
        let names = aztek_ops::objects(s, a);
        let ids: Vec<NodeId> = names.iter().filter_map(|n| g.find(n).ok().flatten()).collect();
        let ancestors = |id: NodeId| {
            let mut v = Vec::new();
            let mut p = g.parent(id);
            while p != NO_NODE {
                v.push(p);
                p = g.parent(p);
            }
            v
        };
        if let Some(&first) = ids.first() {
            let common = ancestors(first).into_iter().find(|p| ids[1..].iter().all(|&i| ancestors(i).contains(p)));
            if let Some(p) = common {
                call.flags.insert("parent".into(), json!(g.path(p)));
            }
        }
    }
    call.flags.remove("world");
    call.flags.remove("absolute");
    call.flags.remove("relative");
    let made = prev(s, &call)?;
    // Its pivots sit at the world's origin, wherever the group ends up, unless it was made empty.
    if !a.bool("empty") {
        let name = made.as_str().unwrap_or_default().to_string();
        s.run("xform", json!({"args": [name], "worldSpace": true, "rotatePivot": [0.0, 0.0, 0.0], "scalePivot": [0.0, 0.0, 0.0]}))?;
    }
    Ok(made)
}

/// One question to polyEvaluate. Several in one call give a dictionary by long name in Python and an array in MEL.
fn poly_evaluate(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    const SINGLE: &[&str] = &["vertex", "edge", "face", "uvcoord", "triangle", "shell", "boundingBox", "boundingBox2d", "area", "worldArea", "vertexComponent", "edgeComponent", "faceComponent", "uvComponent", "triangleComponent", "boundingBoxComponent", "boundingBoxComponent2d", "uvShell"];
    let asked: Vec<&str> = SINGLE.iter().copied().filter(|f| a.bool(f)).collect();
    if asked.len() > 1 {
        let mut out = serde_json::Map::new();
        for f in asked {
            let mut one = a.clone();
            for g in SINGLE {
                one.flags.remove(*g);
            }
            one.flags.insert(f.to_string(), json!(true));
            out.insert(f.to_string(), poly_evaluate_one(s, &one, prev)?);
        }
        return Ok(Value::Object(out));
    }
    poly_evaluate_one(s, a, prev)
}

/// polyEvaluate's UV questions, over each mesh named or selected.
fn uv_answer(s: &mut Session, a: &Args, flag: &str) -> CmdResult {
    let shell = a.get("uvsInShell").and_then(Value::as_i64).unwrap_or(-1);
    let targets = crate::uv_ops::targets(s, a, "polyEvaluate")?;
    let mut sum = 0i64;
    let mut list = Vec::new();
    let mut bounds: Option<[f64; 4]> = None;
    for (node, t) in targets {
        let Some(m) = crate::components::mesh_of(&mut s.graph, node) else { continue };
        let name = s.graph.path(crate::dag::transform_of(&s.graph, node));
        match crate::uv_ops::evaluate(&m, &t, flag, shell, &name) {
            Value::Number(n) => sum += n.as_i64().unwrap_or(0),
            Value::Array(v) if flag.starts_with("boundingBox") => {
                let n = |i: usize, k: usize| v[i][k].as_f64().unwrap_or(0.0);
                let b = bounds.get_or_insert([f64::MAX, f64::MIN, f64::MAX, f64::MIN]);
                *b = [b[0].min(n(0, 0)), b[1].max(n(0, 1)), b[2].min(n(1, 0)), b[3].max(n(1, 1))];
            }
            Value::Array(v) => list.extend(v),
            _ => {}
        }
    }
    Ok(if flag.starts_with("boundingBox") {
        let b = bounds.unwrap_or([0.0; 4]);
        json!([[b[0], b[1]], [b[2], b[3]]])
    } else if matches!(flag, "uvShell" | "uvComponent") {
        json!(sum)
    } else {
        Value::Array(list)
    })
}

fn poly_evaluate_one(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
    for flag in ["uvShell", "uvComponent", "uvShellIds", "activeUVShells"] {
        if a.bool(flag) {
            return uv_answer(s, a, flag);
        }
    }
    if a.has("uvsInShell") {
        return uv_answer(s, a, "uvsInShell");
    }
    if a.bool("activeShells") {
        let comps = crate::components::component_strings(&s.graph, true);
        let g = &mut s.graph;
        let mut ids = Vec::new();
        for (mesh, points) in crate::skin::points_named(g, &comps) {
            let Some(m) = crate::components::mesh_of(g, mesh) else { continue };
            let (of, _) = aztek_geo::multibox::shells_of(&m);
            ids.extend(points.iter().filter_map(|&p| of.get(p as usize).copied()));
        }
        ids.sort_unstable();
        ids.dedup();
        return Ok(json!(ids));
    }
    // Lengths and areas read in the current unit.
    let unit = if a.bool("boundingBox") || a.bool("boundingBoxComponent") {
        aztek_ops::units::Unit::Linear
    } else if a.bool("area") || a.bool("worldArea") {
        aztek_ops::units::Unit::Area
    } else {
        aztek_ops::units::Unit::None
    };
    let mut v = poly_evaluate_counts(s, a, prev)?;
    if aztek_ops::units::user_call() {
        aztek_ops::units::json_to_user(unit, &mut v);
    }
    Ok(v)
}

fn poly_evaluate_counts(s: &mut Session, a: &Args, prev: &aztek_ops::RunFn<Session>) -> CmdResult {
            let names = objects(s, a);
            if a.bool("area") || a.bool("worldArea") {
                return Ok(json!(mesh_area(&mut s.graph, &names, a.bool("worldArea"))));
            }
            if a.bool("boundingBoxComponent") {
                let comps = crate::components::component_strings(&s.graph, true);
                let g = &mut s.graph;
                g.evaluate();
                let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
                for (mesh, ids) in crate::skin::points_named(g, &comps) {
                    let Some(m) = crate::components::mesh_of(g, mesh) else { continue };
                    let w = crate::rig::world(g, crate::dag::transform_of(g, mesh));
                    for p in ids {
                        let q = w.transform_point3(m.position(p));
                        lo = lo.min(q);
                        hi = hi.max(q);
                    }
                }
                return Ok(json!([[lo.x, hi.x], [lo.y, hi.y], [lo.z, hi.z]]));
            }
            if a.bool("vertexComponent") || a.bool("edgeComponent") || a.bool("faceComponent") {
                let kind = if a.bool("vertexComponent") { ".vtx[" } else if a.bool("edgeComponent") { ".e[" } else { ".f[" };
                let comps = crate::components::component_strings(&s.graph, true);
                return Ok(json!(comps.iter().filter(|c| c.contains(kind)).count()));
            }
            if a.bool("format") {
                let r = prev(s, &Args { positional: a.positional.clone(), ..Default::default() })?;
                let o = r.as_object().cloned().unwrap_or_default();
                return Ok(json!(o.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")));
            }
            if a.bool("triangleComponent") {
                let mut call = a.clone();
                call.flags.remove("triangleComponent");
                call.flags.insert("triangle".into(), json!(true));
                return prev(s, &call);
            }
            if a.bool("boundingBox2d") || a.bool("boundingBoxComponent2d") {
                return uv_answer(s, a, if a.bool("boundingBox2d") { "boundingBox2d" } else { "boundingBoxComponent2d" });
            }
            prev(s, a)
}

fn sets_cmd(s: &mut Session, a: &Args) -> CmdResult {
    use crate::object_sets as os;
    let g = &mut s.graph;
    let items = a.named();
    let set_named = |g: &Graph, name: &str| -> CmdResult<NodeId> {
        let id = node_of(g, name, "sets")?;
        if !g.node(id).is_some_and(os::is_set) {
            fail!("sets: {name} is not a set");
        }
        Ok(id)
    };
    let parse = |g: &Graph, list: &[String]| os::parse(g, list).map_err(aztek_ops::CommandError::new);
    let annotation = |g: &Graph, id: NodeId| g.node(id).and_then(|n| n.ty().attr_index("annotation").map(|i| PlugRef::new(id, i as i32)));
    if a.bool("query") {
        let Some(first) = items.first() else { fail!("sets: name a set") };
        let id = set_named(g, first)?;
        if asked(a, "text") {
            return Ok(match annotation(g, id) {
                Some(p) => json!(g.value(&p).map(|v| v.as_str().to_string()).unwrap_or_default()),
                None => json!(""),
            });
        }
        let list = os::members(g, id);
        if asked(a, "size") {
            return Ok(json!(list.len()));
        }
        if asked(a, "nodesOnly") || a.bool("nodesOnly") {
            let mut nodes: Vec<String> = Vec::new();
            for m in &list {
                let p = g.path(m.node);
                if !nodes.contains(&p) {
                    nodes.push(p);
                }
            }
            return Ok(json!(nodes));
        }
        return Ok(json!(os::format(g, &list)));
    }
    // -text with -edit sets the note on the set named.
    if a.bool("edit") {
        if let (Some(t), Some(first)) = (a.get("text").and_then(Value::as_str), items.first()) {
            let id = set_named(g, first)?;
            if let Some(p) = annotation(g, id) {
                g.set_value(p, PlugValue::String(t.to_string()))?;
            }
            return Ok(Value::Null);
        }
    }
    for (flag, add) in [("addElement", true), ("forceElement", true), ("include", true), ("remove", false)] {
        let Some(name) = a.get(flag).and_then(Value::as_str) else { continue };
        let id = set_named(g, name)?;
        let list = if items.is_empty() { os::from_selection(g) } else { parse(g, &items)? };
        if add {
            os::add(g, id, &list)?;
        } else {
            os::remove(g, id, &list)?;
        }
        return Ok(Value::Null);
    }
    if let Some(name) = a.get("clear").and_then(Value::as_str) {
        let id = set_named(g, name)?;
        let list = os::members(g, id);
        os::remove(g, id, &list)?;
        return Ok(Value::Null);
    }
    // -anyMember: whether any of the items is in the set.
    if let Some(name) = a.get("anyMember").and_then(Value::as_str) {
        let id = set_named(g, name)?;
        let wanted = if items.is_empty() { os::from_selection(g) } else { parse(g, &items)? };
        let held = os::members(g, id);
        let any = wanted.iter().any(|w| held.iter().any(|h| h.node == w.node && (h.kind == crate::components::ComponentMode::Object || (h.kind == w.kind && w.ids.iter().any(|i| h.ids.contains(i))))));
        return Ok(json!(any));
    }
    // -copy: a new set holding the named set's members; -split: the items move out of it into a new set.
    if let Some(name) = a.get("copy").and_then(Value::as_str) {
        let src = set_named(g, name)?;
        let list = os::members(g, src);
        let id = os::create(g, &a.string("name", "set1"), &list, false)?;
        return Ok(json!(g.path(id)));
    }
    if let Some(name) = a.get("split").and_then(Value::as_str) {
        let src = set_named(g, name)?;
        let list = if items.is_empty() { os::from_selection(g) } else { parse(g, &items)? };
        os::remove(g, src, &list)?;
        let id = os::create(g, &a.string("name", "set1"), &list, false)?;
        return Ok(json!(g.path(id)));
    }
    if let Some(name) = a.get("isMember").and_then(Value::as_str) {
        let id = set_named(g, name)?;
        // By component, not by how the member happens to be written: vtx[1] is in vtx[0:3].
        let wanted = if items.is_empty() { os::from_selection(g) } else { parse(g, &items)? };
        let wanted = if crate::deform_sets::deformer_of_set(g, id).is_some() { crate::deform_sets::as_points(g, &wanted) } else { wanted };
        let held = os::members(g, id);
        let inside = wanted.iter().all(|w| held.iter().any(|h| h.node == w.node && (h.kind == crate::components::ComponentMode::Object || (h.kind == w.kind && w.ids.iter().all(|i| h.ids.contains(i))))));
        return Ok(json!(inside));
    }
    if a.has("union") || a.has("intersection") || a.has("subtract") || a.has("isIntersecting") {
        let key = ["union", "intersection", "subtract", "isIntersecting"].into_iter().find(|k| a.has(k)).unwrap();
        let first = a.string(key, "");
        let mut names = vec![first];
        names.extend(items.iter().cloned());
        let mut lists = Vec::new();
        for n in &names {
            let id = set_named(g, n)?;
            lists.push(os::format(g, &os::members(g, id)));
        }
        let mut acc = lists[0].clone();
        for l in &lists[1..] {
            match key {
                "union" => {
                    for x in l {
                        if !acc.contains(x) {
                            acc.push(x.clone());
                        }
                    }
                }
                "subtract" => acc.retain(|x| !l.contains(x)),
                _ => acc.retain(|x| l.contains(x)),
            }
        }
        if key == "isIntersecting" {
            return Ok(json!(!acc.is_empty()));
        }
        return Ok(json!(acc));
    }
    // Create.
    let list = if a.bool("empty") {
        Vec::new()
    } else if items.is_empty() {
        os::from_selection(g)
    } else {
        parse(g, &items)?
    };
    let id = os::create(g, &a.string("name", "set1"), &list, false)?;
    if let (Some(t), Some(p)) = (a.get("text").and_then(Value::as_str), annotation(g, id)) {
        g.set_value(p, PlugValue::String(t.to_string()))?;
    }
    Ok(json!(g.path(id)))
}

fn set_shape_names(g: &mut Graph, id: NodeId) -> CmdResult<()> {
    let name = g.node(id).unwrap().name().to_string();
    let short = name.rsplit(':').next().unwrap_or(&name).to_string();
    let ns = name.strip_suffix(&short).unwrap_or("").to_string();
    for sh in crate::dag::shapes(g, id, true) {
        // A shape keeps a name it was given by hand.
        if g.node(sh).is_some_and(|n| n.bool(DagNode::INTERMEDIATE_OBJECT)) {
            continue;
        }
        g.rename(sh, &crate::dag::shape_name(&format!("{ns}{short}")))?;
    }
    Ok(())
}

/// ":a:b" and "a:b" relative to the current namespace, as a full name without the leading colon. "" is the root.
fn absolute_namespace(g: &Graph, name: &str) -> String {
    if let Some(rest) = name.strip_prefix(':') {
        return rest.to_string();
    }
    if g.current_namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}:{name}", g.current_namespace)
    }
}

pub(crate) fn namespace_exists(g: &Graph, full: &str) -> bool {
    full.is_empty()
        || g.namespaces.contains(full)
        || g.nodes().iter().any(|&i| g.node(i).is_some_and(|n| n.namespace() == full || n.namespace().starts_with(&format!("{full}:"))))
}

fn nodes_in(g: &Graph, full: &str) -> Vec<NodeId> {
    g.nodes().into_iter().filter(|&i| g.node(i).is_some_and(|n| n.namespace() == full)).collect()
}

fn child_namespaces(g: &Graph, full: &str, recurse: bool) -> Vec<String> {
    let prefix = if full.is_empty() { String::new() } else { format!("{full}:") };
    let mut all: Vec<String> = g.namespaces.iter().cloned().collect();
    for i in g.nodes() {
        if let Some(n) = g.node(i) {
            if !n.namespace().is_empty() && !all.iter().any(|x| x == n.namespace()) {
                all.push(n.namespace().to_string());
            }
        }
    }
    all.retain(|x| x.starts_with(&prefix) && x.len() > prefix.len() && (recurse || !x[prefix.len()..].contains(':')));
    all.sort();
    all
}

/// Renames a namespace's nodes to live under another (`to` is "" for the root).
fn move_namespace_nodes(g: &mut Graph, from: &str, to: &str) -> CmdResult<()> {
    let prefix = format!("{from}:");
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if let Some(rest) = n.name().strip_prefix(&prefix) {
            let new = if to.is_empty() { rest.to_string() } else { format!("{to}:{rest}") };
            let saved = std::mem::take(&mut g.current_namespace);
            let r = g.rename(id, &new);
            g.current_namespace = saved;
            r?;
        }
    }
    Ok(())
}

fn namespace_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let g = &mut s.graph;
    if a.has("validateName") {
        let name = a.string("validateName", "");
        let clean: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == ':' { c } else { '_' }).collect();
        return Ok(json!(clean));
    }
    if a.bool("query") {
        if asked(a, "relativeNames") {
            return Ok(json!(false));
        }
        if asked(a, "isRootNamespace") {
            let n = a.named().first().cloned().or_else(|| a.get("isRootNamespace").and_then(Value::as_str).map(str::to_string)).unwrap_or_default();
            return Ok(json!(n == ":" || n.is_empty()));
        }
        if asked(a, "exists") || a.pos_str(0).is_some() {
            let n = a.pos_str(0).unwrap_or_default().to_string();
            let full = absolute_namespace(g, &n);
            return Ok(json!(namespace_exists(g, &full)));
        }
        return Ok(json!(if g.current_namespace.is_empty() { ":".to_string() } else { format!(":{}", g.current_namespace) }));
    }
    if let Some(n) = a.get("exists").and_then(Value::as_str) {
        let full = absolute_namespace(g, n);
        return Ok(json!(namespace_exists(g, &full)));
    }
    if a.bool("exists") {
        let n = a.pos_str(0).unwrap_or_default().to_string();
        let full = absolute_namespace(g, &n);
        return Ok(json!(namespace_exists(g, &full)));
    }
    if a.has("addNamespace") {
        let name = a.string("addNamespace", "");
        if !Graph::valid_name(&name) {
            fail!("namespace: invalid name '{name}'");
        }
        let full = match a.get("parent").and_then(Value::as_str) {
            Some(":") => name.clone(),
            Some(p) => format!("{}:{name}", p.trim_start_matches(':')),
            None => absolute_namespace(g, &name),
        };
        if namespace_exists(g, &full) {
            fail!("namespace: namespace {full} already exists");
        }
        g.set_namespace_known(&full, true);
        return Ok(json!(name));
    }
    if a.has("setNamespace") {
        let name = a.string("setNamespace", "");
        let full = if name == ":" { String::new() } else if name == ".." {
            g.current_namespace.rsplit_once(':').map(|x| x.0.to_string()).unwrap_or_default()
        } else {
            absolute_namespace(g, &name)
        };
        if !namespace_exists(g, &full) {
            fail!("namespace: namespace {full} doesn't exist");
        }
        g.current_namespace = full.clone();
        return Ok(json!(if full.is_empty() { ":".to_string() } else { full }));
    }
    if let Some(name) = a.get("collapseAncestors").and_then(Value::as_str).map(str::to_string) {
        // Each empty namespace above the one named goes, nearest first, up to the first that holds something.
        let full = absolute_namespace(g, &name);
        let mut at = full.rsplit_once(':').map(|x| x.0.to_string());
        while let Some(ns) = at {
            if ns.is_empty() || !nodes_in(g, &ns).is_empty() || child_namespaces(g, &ns, false).iter().any(|c| namespace_exists(g, c)) {
                break;
            }
            g.set_namespace_known(&ns, false);
            if g.current_namespace == ns {
                g.current_namespace = String::new();
            }
            at = ns.rsplit_once(':').map(|x| x.0.to_string());
        }
        return Ok(Value::Null);
    }
    if a.has("removeNamespace") {
        let name = a.string("removeNamespace", "");
        let full = absolute_namespace(g, &name);
        if !namespace_exists(g, &full) {
            fail!("namespace: namespace {full} doesn't exist");
        }
        let mut order = child_namespaces(g, &full, true);
        order.reverse();
        order.push(full.clone());
        let content = a.bool("deleteNamespaceContent");
        let other = a.get("mergeNamespaceWithOther").and_then(Value::as_str).map(str::to_string);
        if [a.bool("mergeNamespaceWithRoot"), a.bool("mergeNamespaceWithParent"), other.is_some()].iter().filter(|x| **x).count() > 1 {
            fail!("namespace: -mergeNamespaceWithRoot, -mergeNamespaceWithParent and -mergeNamespaceWithOther go one at a time");
        }
        if let Some(o) = &other {
            // What is left moves to the namespace named first, child namespaces with it.
            let to = if o == ":" { String::new() } else { absolute_namespace(g, o) };
            if !to.is_empty() && !namespace_exists(g, &to) {
                fail!("namespace: namespace {to} doesn't exist");
            }
            if to == full || to.starts_with(&format!("{full}:")) {
                fail!("namespace: can't merge {full} into itself");
            }
            for child in child_namespaces(g, &full, true) {
                let new = format!("{}{}", if to.is_empty() { String::new() } else { format!("{to}:") }, &child[full.len() + 1..]);
                move_namespace_nodes(g, &child, &new)?;
                g.set_namespace_known(&new, true);
            }
            move_namespace_nodes(g, &full, &to)?;
        } else if a.bool("mergeNamespaceWithRoot") || a.bool("mergeNamespaceWithParent") {
            let to = if a.bool("mergeNamespaceWithRoot") { String::new() } else { full.rsplit_once(':').map(|x| x.0.to_string()).unwrap_or_default() };
            move_namespace_nodes(g, &full, &to)?;
        } else if content {
            let doomed: Vec<NodeId> = g.nodes().into_iter().filter(|&i| g.node(i).is_some_and(|n| n.namespace() == full || n.namespace().starts_with(&format!("{full}:")))).collect();
            for d in doomed {
                if g.node(d).is_some() {
                    g.delete_node(d)?;
                }
            }
        } else if !nodes_in(g, &full).is_empty() || !child_namespaces(g, &full, false).is_empty() {
            fail!("namespace: namespace {full} is not empty");
        }
        for n in order {
            g.set_namespace_known(&n, false);
        }
        if g.current_namespace == full || g.current_namespace.starts_with(&format!("{full}:")) {
            g.current_namespace = String::new();
        }
        return Ok(Value::Null);
    }
    if let Some(Value::Array(pair)) = a.get("rename").or(a.get("moveNamespace")) {
        let (Some(from), Some(to)) = (pair.first().and_then(Value::as_str), pair.get(1).and_then(Value::as_str)) else { fail!("namespace: expected two names") };
        let (from, to) = (absolute_namespace(g, from), absolute_namespace(g, to));
        if a.has("rename") {
            for child in child_namespaces(g, &from, true) {
                let new = format!("{to}{}", &child[from.len()..]);
                move_namespace_nodes(g, &child, &new)?;
                g.set_namespace_known(&child, false);
                g.set_namespace_known(&new, true);
            }
            move_namespace_nodes(g, &from, &to)?;
            g.set_namespace_known(&from, false);
            g.set_namespace_known(&to, true);
        } else {
            move_namespace_nodes(g, &from, &to)?;
        }
        return Ok(Value::Null);
    }
    Ok(Value::Null)
}

/// The internal nodes the interface keeps outside the scene (item filters), for namespaceInfo -internal. The host sets it.
pub static INTERNAL_NODES: std::sync::RwLock<Option<fn() -> Vec<String>>> = std::sync::RwLock::new(None);

fn namespace_info(s: &mut Session, a: &Args) -> CmdResult {
    let g = &s.graph;
    let target = match a.pos_str(0) {
        Some(":") => String::new(),
        Some(n) => absolute_namespace(g, n),
        None => g.current_namespace.clone(),
    };
    if a.bool("currentNamespace") {
        return Ok(json!(if target.is_empty() { ":".to_string() } else if a.bool("absoluteName") { format!(":{target}") } else { target }));
    }
    if a.bool("baseName") {
        return Ok(json!(target.rsplit(':').next().unwrap_or("")));
    }
    if a.bool("fullName") {
        return Ok(json!(target));
    }
    if a.bool("absoluteName") {
        return Ok(json!(format!(":{target}")));
    }
    if a.bool("parent") {
        return Ok(json!(target.rsplit_once(':').map(|x| x.0.to_string()).unwrap_or(":".to_string())));
    }
    if a.bool("isRootNamespace") {
        return Ok(json!(target.is_empty()));
    }
    let recurse = a.bool("recurse");
    let nodes = |g: &Graph| -> Vec<String> {
        let mut out: Vec<NodeId> = nodes_in(g, &target);
        if recurse {
            for c in child_namespaces(g, &target, true) {
                out.extend(nodes_in(g, &c));
            }
        }
        out.iter().map(|&i| if a.bool("dagPath") { g.full_path(i) } else { g.path(i) }).collect()
    };
    let spaces = child_namespaces(g, &target, recurse);
    if a.bool("listOnlyNamespaces") {
        return Ok(json!(spaces));
    }
    if a.bool("listOnlyDependencyNodes") {
        let mut list = nodes(g);
        // -internal: the interface's own nodes too (item filters and the like), which live in the root namespace.
        if a.bool("internal") && target.is_empty() {
            if let Some(more) = *INTERNAL_NODES.read().unwrap() {
                list.extend(more());
            }
        }
        return Ok(json!(list));
    }
    if a.bool("listNamespace") {
        let mut out = spaces;
        out.extend(nodes(g));
        return Ok(json!(out));
    }
    Ok(json!(if target.is_empty() { ":".to_string() } else { target }))
}

/// `name value` for optionVar: MEL leaves the value positional, Python gives a pair.
/// The project: its root, the folder file browsers start in, its file rules and variables, as `workspace` keeps them.
#[derive(Default, Clone, Debug)]
pub struct Workspace {
    /// The project's root; None until one is opened, when the working folder stands in.
    pub root: Option<String>,
    /// -directory: where file browsers start.
    pub directory: Option<String>,
    /// File rules and variables, in the order they were set.
    pub rules: Vec<(String, String)>,
    pub variables: Vec<(String, String)>,
    /// Names given with -shortName and -fullName, and -baseWorkspace.
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub base: String,
}

fn slash(p: &str) -> String {
    p.replace('\\', "/")
}

impl Workspace {
    pub fn root(&self) -> String {
        self.root.clone().or_else(|| aztek_core::env::get("MAYA_PROJECT")).map(|r| slash(&r)).unwrap_or_else(|| std::env::current_dir().map(|d| slash(&d.display().to_string())).unwrap_or_default())
    }
    fn set(list: &mut Vec<(String, String)>, k: &str, v: &str) {
        match list.iter_mut().find(|(n, _)| n == k) {
            Some(e) => e.1 = v.to_string(),
            None => list.push((k.to_string(), v.to_string())),
        }
    }
    fn get<'a>(list: &'a [(String, String)], k: &str) -> Option<&'a str> {
        list.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
    }
    /// Reads the project's workspace.mel, if it has one: its `workspace -fr` and `-v` lines are taken as data.
    fn read(&mut self) {
        self.rules.clear();
        self.variables.clear();
        let Ok(text) = std::fs::read_to_string(std::path::Path::new(&self.root()).join("workspace.mel")) else { return };
        // Comments off each line first (none of the reference's rule paths hold "//").
        let text: String = text.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n");
        for stmt in text.split(';') {
            let words = mel_words(stmt);
            if words.first().map(String::as_str) != Some("workspace") {
                continue;
            }
            let mut k = 1;
            while k + 2 < words.len() + 1 {
                match words.get(k).map(String::as_str) {
                    Some("-fr" | "-fileRule" | "-ot" | "-objectType" | "-rt" | "-renderType") if k + 2 < words.len() => {
                        Self::set(&mut self.rules, &words[k + 1], &words[k + 2]);
                        k += 3;
                    }
                    Some("-v" | "-variable") if k + 2 < words.len() => {
                        Self::set(&mut self.variables, &words[k + 1], &words[k + 2]);
                        k += 3;
                    }
                    Some(_) => k += 1,
                    None => break,
                }
            }
        }
    }
    /// Writes the rules and variables to the project's workspace.mel.
    fn save(&self) -> std::io::Result<()> {
        let mut out = String::from("//Aztek project definition file\n");
        for (k, v) in &self.rules {
            out.push_str(&format!("workspace -fr \"{k}\" \"{v}\";\n"));
        }
        for (k, v) in &self.variables {
            out.push_str(&format!("workspace -v \"{k}\" \"{v}\";\n"));
        }
        std::fs::write(std::path::Path::new(&self.root()).join("workspace.mel"), out)
    }
    /// A name with the project's variables and the environment's put in, and made full under the root when relative.
    pub fn expand(&self, name: &str) -> String {
        let root = self.root();
        if name.is_empty() {
            return format!("{}/", root.trim_end_matches('/'));
        }
        let env_only = name.starts_with('~');
        let look = |v: &str| -> Option<String> { if env_only { None } else { Self::get(&self.variables, v).map(str::to_string) }.or_else(|| std::env::var(v).ok()) };
        let mut out = String::new();
        let chars: Vec<char> = name.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '%' {
                if let Some(end) = chars[i + 1..].iter().position(|&x| x == '%') {
                    let v: String = chars[i + 1..i + 1 + end].iter().collect();
                    if let Some(val) = look(&v) {
                        out.push_str(&val);
                        i += end + 2;
                        continue;
                    }
                }
            } else if c == '$' {
                let (v, used) = if chars.get(i + 1) == Some(&'{') {
                    match chars[i + 2..].iter().position(|&x| x == '}') {
                        Some(end) => (chars[i + 2..i + 2 + end].iter().collect::<String>(), end + 3),
                        None => (String::new(), 0),
                    }
                } else {
                    let n = chars[i + 1..].iter().take_while(|x| x.is_alphanumeric() || **x == '_').count();
                    (chars[i + 1..i + 1 + n].iter().collect::<String>(), n + 1)
                };
                if !v.is_empty() {
                    if let Some(val) = look(&v) {
                        out.push_str(&val);
                        i += used;
                        continue;
                    }
                }
            }
            out.push(c);
            i += 1;
        }
        // ~, ~/ and ~user at the start: home folders.
        if let Some(rest) = out.strip_prefix('~') {
            let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default();
            out = if rest.is_empty() || rest.starts_with('/') { format!("{home}{rest}") } else { format!("{}/{rest}", std::path::Path::new(&home).parent().map(|p| p.display().to_string()).unwrap_or_default()) };
        }
        let out = slash(&out);
        let absolute = out.starts_with('/') || (out.len() > 1 && out.as_bytes()[1] == b':');
        if absolute { out } else { format!("{}/{}", root.trim_end_matches('/'), out) }
    }
}

/// The words of a MEL statement, quotes taken off: enough to read workspace.mel as data.
fn mel_words(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let (mut quoted, mut any) = (false, false);
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                quoted = !quoted;
                any = true;
            }
            '\\' if quoted => {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            }
            c if c.is_whitespace() && !quoted => {
                if any || !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                any = false;
            }
            c => cur.push(c),
        }
    }
    if any || !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn workspace_cmd() -> Cmd {
    let mut c = Cmd::new(
        "workspace",
        "The project: -o opens one, -q -rd its root, -fr and -fre its file rules, -v and -ve its variables, -en expands a name under it, -pp makes a path relative to it, -dir where file browsers start, -cr makes a folder, -s saves workspace.mel.",
        workspace_run,
    )
    .positional("[path]")
    .undoable(false)
    .flag(f("query", "q", B, "Query."))
    .flag(f("queried", "", ArgType::List, "What -q asks for."));
    for (name, short) in [("active", "act"), ("fileRuleList", "frl"), ("objectTypeList", "otl"), ("renderTypeList", "rtl"), ("variableList", "vl"), ("listWorkspaces", "lw"), ("listFullWorkspaces", "lfw"), ("rootDirectory", "rd"), ("saveWorkspace", "s"), ("update", "u"), ("updateAll", "ua"), ("filter", "f")] {
        c = c.flag(f(name, short, B, "See the description."));
    }
    for (name, short) in [("baseWorkspace", "bw"), ("create", "cr"), ("directory", "dir"), ("expandName", "en"), ("fileRuleEntry", "fre"), ("objectTypeEntry", "ote"), ("renderTypeEntry", "rte"), ("fullName", "fn"), ("list", "l"), ("newWorkspace", "n"), ("openWorkspace", "o"), ("projectPath", "pp"), ("removeFileRuleEntry", "rfr"), ("removeVariableEntry", "rve"), ("shortName", "sn"), ("variableEntry", "ve")] {
        c = c.flag(f(name, short, A, "See the description."));
    }
    for (name, short) in [("fileRule", "fr"), ("objectType", "ot"), ("renderType", "rt"), ("variable", "v")] {
        c = c.flag(f(name, short, A, "A name and a value.").multi(2));
    }
    c
}

fn workspace_run(s: &mut Session, a: &Args) -> CmdResult {
    let str_of = |k: &str| a.get(k).and_then(|v| match v {
        Value::String(t) => Some(t.clone()),
        Value::Bool(_) | Value::Null => None,
        o => Some(o.to_string()),
    });
    let queried: Vec<String> = a.get("queried").and_then(Value::as_array).map(|q| q.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let ws = &mut s.extras.workspace;
    let pairs = |v: Option<&Value>| -> Vec<(String, String)> {
        let flat: Vec<String> = match v {
            Some(Value::Array(x)) => x.iter().flat_map(|e| match e {
                Value::Array(y) => y.iter().map(|z| z.as_str().map(str::to_string).unwrap_or_else(|| z.to_string())).collect::<Vec<_>>(),
                z => vec![z.as_str().map(str::to_string).unwrap_or_else(|| z.to_string())],
            }).collect(),
            _ => Vec::new(),
        };
        flat.chunks(2).filter(|c| c.len() == 2).map(|c| (c[0].clone(), c[1].clone())).collect()
    };
    if a.bool("query") {
        let q = queried.first().cloned().unwrap_or_default();
        let entry = |list: &[(String, String)], k: &str| json!(Workspace::get(list, k).unwrap_or(""));
        return Ok(match q.as_str() {
            "rootDirectory" => json!(format!("{}/", ws.root().trim_end_matches('/'))),
            "fullName" => json!(ws.full_name.clone().unwrap_or_else(|| ws.root().trim_end_matches('/').to_string())),
            "shortName" => json!(ws.short_name.clone().unwrap_or_else(|| ws.root().trim_end_matches('/').rsplit('/').next().unwrap_or("").to_string())),
            "directory" => json!(format!("{}/", ws.directory.clone().unwrap_or_else(|| ws.root()).trim_end_matches('/'))),
            "baseWorkspace" => json!(if ws.base.is_empty() { "default" } else { ws.base.as_str() }),
            "fileRule" | "objectType" | "renderType" => json!(ws.rules.iter().flat_map(|(k, v)| [k.clone(), v.clone()]).collect::<Vec<_>>()),
            "fileRuleList" | "objectTypeList" | "renderTypeList" => json!(ws.rules.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>()),
            "fileRuleEntry" | "objectTypeEntry" | "renderTypeEntry" => entry(&ws.rules, &a.pos_str(0).map(str::to_string).or_else(|| str_of(&q)).unwrap_or_default()),
            "variable" => json!(ws.variables.iter().flat_map(|(k, v)| [k.clone(), v.clone()]).collect::<Vec<_>>()),
            "variableList" => json!(ws.variables.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>()),
            "variableEntry" => entry(&ws.variables, &a.pos_str(0).map(str::to_string).or_else(|| str_of(&q)).unwrap_or_default()),
            "expandName" => json!(ws.expand(&a.pos_str(0).map(str::to_string).or_else(|| str_of(&q)).unwrap_or_default())),
            "projectPath" => json!(project_path(ws, &a.pos_str(0).map(str::to_string).or_else(|| str_of(&q)).unwrap_or_default())),
            "listWorkspaces" => json!([ws.root().trim_end_matches('/').rsplit('/').next().unwrap_or("")]),
            "listFullWorkspaces" => json!([ws.root()]),
            "list" => json!(list_dir(&ws.directory.clone().unwrap_or_else(|| ws.root()))),
            "active" | "openWorkspace" => json!(ws.root()),
            _ => json!(ws.root()),
        });
    }
    // Removals first, then what is set.
    if let Some(k) = str_of("removeFileRuleEntry") {
        ws.rules.retain(|(n, _)| *n != k);
    }
    if let Some(k) = str_of("removeVariableEntry") {
        ws.variables.retain(|(n, _)| *n != k);
    }
    for flag in ["fileRule", "objectType", "renderType"] {
        for (k, v) in pairs(a.get(flag)) {
            Workspace::set(&mut ws.rules, &k, &v);
        }
    }
    for (k, v) in pairs(a.get("variable")) {
        Workspace::set(&mut ws.variables, &k, &v);
    }
    if let Some(n) = str_of("shortName") {
        ws.short_name = Some(n);
    }
    if let Some(n) = str_of("fullName") {
        ws.full_name = Some(n);
    }
    if let Some(n) = str_of("baseWorkspace") {
        ws.base = n;
    }
    if let Some(d) = str_of("directory") {
        let full = if std::path::Path::new(&d).is_absolute() { slash(&d) } else { ws.expand(&d) };
        ws.directory = Some(full.clone());
        return Ok(json!(full));
    }
    if let Some(d) = str_of("openWorkspace").or_else(|| a.bool("active").then(|| a.pos_str(0).map(str::to_string)).flatten()) {
        ws.root = Some(slash(&d).trim_end_matches('/').to_string());
        ws.directory = None;
        ws.read();
        return Ok(json!(ws.root()));
    }
    if let Some(d) = str_of("newWorkspace") {
        let full = if std::path::Path::new(&d).is_absolute() { slash(&d) } else { format!("{}/{}", ws.directory.clone().unwrap_or_else(|| ws.root()).trim_end_matches('/'), d) };
        if std::path::Path::new(&full).join("workspace.mel").exists() {
            fail!("workspace: a workspace already exists at {full}");
        }
        return Ok(json!(full));
    }
    if let Some(d) = str_of("create") {
        let full = if std::path::Path::new(&d).is_absolute() { slash(&d) } else { format!("{}/{}", ws.directory.clone().unwrap_or_else(|| ws.root()).trim_end_matches('/'), d) };
        std::fs::create_dir_all(&full).map_err(|e| aztek_ops::CommandError::new(format!("workspace: {full}: {e}")))?;
        return Ok(json!(full));
    }
    if let Some(n) = str_of("fileRuleEntry").or_else(|| str_of("objectTypeEntry")).or_else(|| str_of("renderTypeEntry")) {
        return Ok(json!(Workspace::get(&ws.rules, &n).unwrap_or("")));
    }
    if let Some(n) = str_of("variableEntry") {
        return Ok(json!(Workspace::get(&ws.variables, &n).unwrap_or("")));
    }
    if let Some(n) = str_of("expandName") {
        return Ok(json!(ws.expand(&n)));
    }
    if let Some(n) = str_of("projectPath") {
        return Ok(json!(project_path(ws, &n)));
    }
    if a.bool("fileRuleList") || a.bool("objectTypeList") || a.bool("renderTypeList") {
        return Ok(json!(ws.rules.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>()));
    }
    if a.bool("variableList") {
        return Ok(json!(ws.variables.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>()));
    }
    if a.has("list") {
        let d = str_of("list").map(|d| ws.expand(&d)).unwrap_or_else(|| ws.directory.clone().unwrap_or_else(|| ws.root()));
        return Ok(json!(list_dir(&d)));
    }
    if a.bool("saveWorkspace") {
        ws.save().map_err(|e| aztek_ops::CommandError::new(format!("workspace: {e}")))?;
        return Ok(Value::Null);
    }
    if a.bool("update") || a.bool("updateAll") {
        ws.read();
        return Ok(Value::Null);
    }
    if a.bool("rootDirectory") {
        return Ok(json!(format!("{}/", ws.root().trim_end_matches('/'))));
    }
    Ok(Value::Null)
}

/// A path relative to the project's root where it lies under it, else as given.
fn project_path(ws: &Workspace, p: &str) -> String {
    let root = format!("{}/", ws.root().trim_end_matches('/'));
    let full = slash(p);
    full.strip_prefix(&root).map(str::to_string).unwrap_or(full)
}

fn list_dir(d: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(d).map(|r| r.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default();
    v.sort();
    v
}

/// What optionVar keeps beside each value.
#[derive(Default, Clone, Debug)]
pub struct OptionVarMeta {
    /// Default values, set by -default or -init.
    pub defaults: BTreeMap<String, Value>,
    /// The category each name was given with -category.
    pub category: BTreeMap<String, String>,
    /// The preferences file each category saves to (-prefFile).
    pub pref_file: BTreeMap<String, String>,
    /// Copies made by -stash.
    pub stash: BTreeMap<String, Value>,
    /// Names not kept between sessions.
    pub transient: BTreeSet<String>,
    /// The element type of each array, so an empty one keeps its type: 0 int, 1 float, 2 string.
    pub kind: BTreeMap<String, u8>,
    /// The preferences version -version gave.
    pub version: Option<i64>,
}

impl OptionVarMeta {
    /// Whether this name is kept between sessions.
    pub fn saved(&self, name: &str) -> bool {
        !self.transient.contains(name) && !name.starts_with("__")
    }
}

/// A flag's values, flat: a multi-use flag gathers every use into one list, and Python may pass a list per use.
fn flat(v: Option<&Value>) -> Vec<Value> {
    fn push(out: &mut Vec<Value>, v: &Value) {
        match v {
            Value::Array(a) => a.iter().for_each(|x| push(out, x)),
            other => out.push(other.clone()),
        }
    }
    let mut out = Vec::new();
    if let Some(v) = v {
        push(&mut out, v);
    }
    out
}

/// Names given to a flag taking one string per use.
fn names_of(a: &Args, flag: &str) -> Vec<String> {
    flat(a.get(flag)).iter().filter_map(|v| match v {
        Value::String(s) => Some(s.clone()),
        Value::Bool(_) | Value::Null => None,
        other => Some(other.to_string()),
    }).collect()
}

/// One value as the type a flag sets: 0 int, 1 float, 2 string.
fn typed(v: &Value, kind: u8) -> Value {
    let num = || v.as_f64().or_else(|| v.as_bool().map(|b| b as i64 as f64)).or_else(|| v.as_str().and_then(|s| s.trim().parse::<f64>().ok())).unwrap_or(0.0);
    match kind {
        0 => json!(num() as i64),
        1 => json!(num()),
        _ => match v {
            Value::String(s) => json!(s),
            other => json!(other.to_string()),
        },
    }
}

fn option_var(s: &mut Session, a: &Args) -> CmdResult {
    let is_default = a.bool("default");
    let transient = a.bool("transient");
    // Questions first: each answers on its own and ignores the other flags.
    if a.bool("query") {
        let name = a.pos_str(0).map(str::to_string).or_else(|| names_of(a, "queried").into_iter().next()).unwrap_or_default();
        let meta = &s.extras.option_var_meta;
        let found = if is_default { meta.defaults.get(&name) } else { s.option_vars.get(&name) };
        return Ok(found.cloned().unwrap_or(json!(0)));
    }
    if let Some(n) = names_of(a, "exists").into_iter().next() {
        let meta = &s.extras.option_var_meta;
        let mut there = s.option_vars.contains_key(&n);
        if is_default {
            there = meta.defaults.contains_key(&n);
        }
        if transient {
            there = there && meta.transient.contains(&n);
        }
        return Ok(json!(there));
    }
    if let Some(n) = names_of(a, "arraySize").into_iter().next() {
        return Ok(json!(match s.option_vars.get(&n) {
            None => 0,
            Some(Value::Array(v)) => v.len(),
            Some(_) => 1,
        }));
    }
    let category = a.get("category").and_then(Value::as_str).map(str::to_string);
    if a.bool("listCategories") {
        let meta = &s.extras.option_var_meta;
        let cats: BTreeSet<&String> = meta.category.values().chain(meta.pref_file.keys()).collect();
        return Ok(json!(cats.into_iter().collect::<Vec<_>>()));
    }
    if a.bool("list") || a.bool("listModified") {
        let meta = &s.extras.option_var_meta;
        let modified = a.bool("listModified");
        let names: Vec<&String> = s
            .option_vars
            .iter()
            .filter(|(n, _)| !n.starts_with("__"))
            .filter(|(n, _)| category.as_ref().is_none_or(|c| meta.category.get(*n) == Some(c)))
            .filter(|(n, _)| !transient || meta.transient.contains(*n))
            .filter(|(n, v)| match (modified, meta.defaults.get(*n)) {
                (false, d) => !is_default || d.is_some(),
                (true, Some(d)) => d != *v,
                (true, None) => !is_default,
            })
            .map(|(n, _)| n)
            .collect();
        return Ok(json!(names));
    }

    // Changes. Removals come before anything set in the same command.
    for n in names_of(a, "remove") {
        s.option_vars.remove(&n);
        let meta = &mut s.extras.option_var_meta;
        meta.defaults.remove(&n);
        meta.category.remove(&n);
        meta.transient.remove(&n);
        meta.kind.remove(&n);
    }
    for n in names_of(a, "clearArray") {
        // An array named so is emptied; empty arrays aren't saved with the preferences.
        if let Some(Value::Array(v)) = s.option_vars.get_mut(&n) {
            v.clear();
        }
    }
    if let Some(v) = a.get("version").and_then(Value::as_i64) {
        s.extras.option_var_meta.version = Some(v);
    }
    if let (Some(c), Some(file)) = (&category, a.get("prefFile").and_then(Value::as_str)) {
        s.extras.option_var_meta.pref_file.insert(c.clone(), file.to_string());
    }

    // -init true, or a name not yet set: current and default both; -init false: only the default; -default: both.
    let init = a.get("init").map(|v| v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0)));
    let mut touched: Vec<String> = Vec::new();
    let mut set = |s: &mut Session, name: String, value: Value, kind: u8| {
        let exists = s.option_vars.contains_key(&name);
        let (current, default) = match init {
            Some(true) => (true, true),
            Some(false) => (!exists, true),
            None => (true, is_default),
        };
        if current {
            s.option_vars.insert(name.clone(), value.clone());
        }
        let meta = &mut s.extras.option_var_meta;
        if default {
            meta.defaults.insert(name.clone(), value);
        }
        meta.kind.insert(name.clone(), kind);
        touched.push(name);
    };
    for (flag, kind, count) in [
        ("intValue", 0u8, 1usize), ("intValue2", 0, 2), ("intValue3", 0, 3), ("intValue4", 0, 4),
        ("floatValue", 1, 1), ("floatValue2", 1, 2), ("floatValue3", 1, 3), ("floatValue4", 1, 4),
        ("stringValue", 2, 1),
    ] {
        let mut vals = flat(a.get(flag));
        if vals.len() == 1 {
            // The name alone in the flag, the values left positional.
            vals.extend(a.positional.iter().take(count).cloned());
        }
        for chunk in vals.chunks(count + 1) {
            if chunk.len() != count + 1 {
                fail!("optionVar: -{flag} takes a name and {count} value(s)");
            }
            let Some(name) = chunk[0].as_str().map(str::to_string) else { fail!("optionVar: -{flag} needs a name first") };
            let value = if count == 1 { typed(&chunk[1], kind) } else { Value::Array(chunk[1..].iter().map(|v| typed(v, kind)).collect()) };
            set(s, name, value, kind);
        }
    }
    for (flag, kind) in [("intArray", 0u8), ("floatArray", 1), ("stringArray", 2)] {
        for name in names_of(a, flag) {
            set(s, name, json!([]), kind);
        }
    }
    for (flag, kind) in [("intValueAppend", 0u8), ("floatValueAppend", 1), ("stringValueAppend", 2)] {
        let vals = flat(a.get(flag));
        for chunk in vals.chunks(2) {
            let (Some(name), Some(v)) = (chunk.first().and_then(Value::as_str), chunk.get(1)) else { fail!("optionVar: -{flag} takes a name and a value") };
            let v = typed(v, kind);
            let entry = s.option_vars.entry(name.to_string()).or_insert(json!([]));
            if !entry.is_array() {
                *entry = json!([entry.clone()]);
            }
            entry.as_array_mut().unwrap().push(v);
            s.extras.option_var_meta.kind.insert(name.to_string(), kind);
            touched.push(name.to_string());
        }
    }
    let rfa = flat(a.get("removeFromArray"));
    for chunk in rfa.chunks(2) {
        let (Some(name), Some(i)) = (chunk.first().and_then(Value::as_str), chunk.get(1).and_then(Value::as_f64)) else { fail!("optionVar: -removeFromArray takes a name and an index") };
        if let Some(Value::Array(v)) = s.option_vars.get_mut(name) {
            let i = i as usize;
            if i < v.len() {
                v.remove(i);
            }
        }
    }
    for n in names_of(a, "stash") {
        if let Some(v) = s.option_vars.get(&n).cloned() {
            s.extras.option_var_meta.stash.insert(n, v);
        }
    }
    for n in names_of(a, "unstash") {
        if let Some(v) = s.extras.option_var_meta.stash.get(&n).cloned() {
            s.option_vars.insert(n, v);
        }
    }
    for n in names_of(a, "clearStash") {
        s.extras.option_var_meta.stash.remove(&n);
    }
    let meta = &mut s.extras.option_var_meta;
    for n in touched {
        if let Some(c) = &category {
            meta.category.insert(n.clone(), c.clone());
        }
        if transient {
            meta.transient.insert(n);
        }
    }
    Ok(Value::Null)
}

fn about(_s: &mut Session, a: &Args) -> CmdResult {
    let (win, linux, mac) = (cfg!(windows), cfg!(target_os = "linux"), cfg!(target_os = "macos"));
    Ok(if a.bool("version") {
        json!(aztek_core::REFERENCE_VERSION)
    } else if a.bool("apiVersion") {
        json!(20250000)
    } else if a.bool("operatingSystem") {
        json!(if win { "win64" } else if mac { "mac" } else { "linux64" })
    } else if a.bool("windows") || a.bool("nt") {
        json!(win)
    } else if a.bool("linux") {
        json!(linux)
    } else if a.bool("macOS") {
        json!(mac)
    } else if a.bool("batch") {
        json!(aztek_core::batch_mode())
    } else if a.bool("is64") {
        json!(true)
    } else if a.bool("uiLanguage") || a.bool("codeset") {
        json!("en_US")
    } else if a.bool("product") || a.bool("application") {
        json!("Aztek")
    } else if a.bool("qtVersion") {
        // Set by the application when Qt starts; empty in batch.
        json!(std::env::var("AZTEK_QT_VERSION").unwrap_or_default())
    } else {
        fail!("about: nothing asked")
    })
}

fn bounds_of(g: &mut Graph, names: &[String], ignore_invisible: bool) -> CmdResult<Option<(DVec3, DVec3)>> {
    g.evaluate();
    let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
    for name in names {
        let id = node_of(g, name, "exactWorldBoundingBox")?;
        let mut under = g.descendants(id);
        under.push(id);
        for u in under {
            if !crate::dag::is_shape(g, u) || (ignore_invisible && !crate::transform::effectively_visible(g, u)) || g.node(u).is_some_and(|n| n.bool(DagNode::INTERMEDIATE_OBJECT)) {
                continue;
            }
            let b = crate::node_bounds::world_bounds(g, u);
            if b.valid() {
                lo = lo.min(b.min);
                hi = hi.max(b.max);
            }
        }
    }
    Ok((lo.x <= hi.x).then_some((lo, hi)))
}

/// The values polyEvaluate's area flags ask for: object space, or world.
fn mesh_area(g: &mut Graph, names: &[String], world: bool) -> f64 {
    let mut total = 0.0;
    for n in names {
        let Ok(Some(id)) = g.find(n) else { continue };
        let Some(mesh) = crate::components::as_mesh(g, id) else { continue };
        let Some(m) = crate::components::mesh_of(g, mesh) else { continue };
        let w = crate::rig::world(g, crate::dag::transform_of(g, mesh));
        for face in 0..m.face_capacity() {
            if !m.face_valid(face) {
                continue;
            }
            let pts: Vec<DVec3> = m.face_points(face).into_iter().map(|p| if world { w.transform_point3(m.position(p)) } else { m.position(p) }).collect();
            for k in 1..pts.len().saturating_sub(1) {
                total += (pts[k] - pts[0]).cross(pts[k + 1] - pts[0]).length() * 0.5;
            }
        }
    }
    total
}

fn toggle_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let attr = [("localAxis", Transform::DISPLAY_LOCAL_AXIS), ("rotatePivot", Transform::DISPLAY_ROTATE_PIVOT), ("scalePivot", Transform::DISPLAY_SCALE_PIVOT), ("selectHandle", Transform::DISPLAY_HANDLE), ("template", DagNode::TEMPLATE)];
    let chosen = attr.iter().find(|(n, _)| a.bool(n) || asked(a, n));
    let Some(&(_, index)) = chosen else { fail!("toggle: say what: -la, -rp, -sp, -sh, -te") };
    let names = objects(s, a);
    let mut last = false;
    for n in &names {
        let id = node_of(&s.graph, n, "toggle")?;
        if !crate::dag::is_transform(&s.graph, id) && index != DagNode::TEMPLATE {
            continue;
        }
        let g = &mut s.graph;
        let now = g.node(id).unwrap().bool(index);
        if a.bool("query") {
            return Ok(json!(now));
        }
        let next = if a.has("state") { a.bool("state") } else { !now };
        g.set_value(PlugRef::new(id, index as i32), PlugValue::Bool(next))?;
        last = next;
    }
    Ok(json!(last))
}

fn align_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    if names.len() < 2 {
        fail!("align: select at least two objects");
    }
    let g = &mut s.graph;
    g.evaluate();
    // The axes: the world's, or those of the object -cs names.
    let frame = match a.get("coordinateSystem").and_then(Value::as_str) {
        Some(cs) if !cs.is_empty() => {
            let id = node_of(g, cs, "align")?;
            let w = crate::rig::world(g, id);
            let (_, r, t) = w.to_scale_rotation_translation();
            glam::DMat4::from_rotation_translation(r, t)
        }
        _ => glam::DMat4::IDENTITY,
    };
    let inv = frame.inverse();
    let mut boxes = Vec::new();
    for n in &names {
        let Some((lo, hi)) = bounds_of(g, std::slice::from_ref(n), false)? else { fail!("align: {n} has no bounds") };
        let (mut blo, mut bhi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
        for k in 0..8 {
            let c = DVec3::new(if k & 1 == 0 { lo.x } else { hi.x }, if k & 2 == 0 { lo.y } else { hi.y }, if k & 4 == 0 { lo.z } else { hi.z });
            let p = inv.transform_point3(c);
            blo = blo.min(p);
            bhi = bhi.max(p);
        }
        boxes.push((blo, bhi));
    }
    let mut deltas = vec![DVec3::ZERO; names.len()];
    for (axis, flag) in ["xAxis", "yAxis", "zAxis"].iter().enumerate() {
        let Some(mode) = a.get(flag).and_then(Value::as_str).map(str::to_ascii_lowercase) else { continue };
        let mid = |b: &(DVec3, DVec3)| (b.0[axis] + b.1[axis]) * 0.5;
        match mode.as_str() {
            "none" | "" => {}
            "min" | "mid" | "max" => {
                let at = |b: &(DVec3, DVec3)| match mode.as_str() {
                    "min" => b.0[axis],
                    "max" => b.1[axis],
                    _ => mid(b),
                };
                let target = if a.bool("alignToLead") {
                    // The lead object is the one selected last.
                    at(&boxes[names.len() - 1])
                } else {
                    // Averaged over the objects.
                    boxes.iter().map(at).sum::<f64>() / boxes.len() as f64
                };
                for (i, b) in boxes.iter().enumerate() {
                    deltas[i][axis] = target - at(b);
                }
            }
            "dist" => {
                // Evenly spread between the outermost two, by their middles.
                let mut order: Vec<usize> = (0..boxes.len()).collect();
                order.sort_by(|&x, &y| mid(&boxes[x]).total_cmp(&mid(&boxes[y])));
                let (first, last) = (mid(&boxes[order[0]]), mid(&boxes[*order.last().unwrap()]));
                let step = (last - first) / (order.len() - 1) as f64;
                for (k, &i) in order.iter().enumerate() {
                    deltas[i][axis] = first + step * k as f64 - mid(&boxes[i]);
                }
            }
            "stack" => {
                // End to end, from the lowest, in the order they sit.
                let mut order: Vec<usize> = (0..boxes.len()).collect();
                order.sort_by(|&x, &y| boxes[x].0[axis].total_cmp(&boxes[y].0[axis]));
                let mut next = boxes[order[0]].1[axis];
                for &i in order.iter().skip(1) {
                    deltas[i][axis] = next - boxes[i].0[axis];
                    next += boxes[i].1[axis] - boxes[i].0[axis];
                }
            }
            other => fail!("align: {other} is not none, min, mid, max, dist or stack"),
        }
    }
    for (n, d) in names.iter().zip(deltas) {
        if d == DVec3::ZERO {
            continue;
        }
        let id = node_of(g, n, "align")?;
        let world = frame.transform_vector3(d);
        crate::transform_ops::move_by(g, &[id], world, crate::transform_ops::Space::World)?;
    }
    Ok(json!(true))
}

fn search_replace(s: &mut Session, a: &Args) -> CmdResult {
    let (Some(from), Some(to)) = (a.pos_str(0).map(str::to_string), a.pos_str(1).map(str::to_string)) else { fail!("searchReplaceNames: expected (search, replace, scope)") };
    let scope = a.pos_str(2).unwrap_or("all").to_string();
    let g = &mut s.graph;
    let mut ids: Vec<NodeId> = match scope.as_str() {
        "selected" => g.selection().to_vec(),
        "hierarchy" => {
            let mut out = Vec::new();
            for &id in g.selection() {
                out.push(id);
                out.extend(g.descendants(id));
            }
            out
        }
        _ => g.nodes(),
    };
    // Children first: a renamed parent changes nothing for them, but paths stay valid this way round.
    ids.reverse();
    let mut count = 0;
    for id in ids {
        let name = g.node(id).unwrap().name().to_string();
        if name.contains(&from) {
            g.rename(id, &name.replace(&from, &to))?;
            count += 1;
        }
    }
    Ok(json!(count))
}

pub fn register(c: &mut Commands<Session>) {
    extend(
        c,
        "listRelatives",
        vec![
            f("children", "c", B, "Children (the default)."),
            f("shapes", "s", B, "Only shapes."),
            f("allParents", "ap", B, "Every parent."),
            f("fullPath", "f", B, "Full paths."),
            f("path", "pa", B, "Shortest unique paths."),
            f("noIntermediate", "ni", B, "Leave out intermediate objects."),
            f("type", "typ", A, "Only this type or these."),
        ],
        |s, a, _| list_relatives(s, a),
    );
    extend(
        c,
        "listConnections",
        vec![
            f("exactType", "et", A, "With -type: exactly this type, not derived ones."),
            f("shapes", "sh", B, "A connected shape is named, not its transform."),
            f("skipConversionNodes", "scn", B, "Report what is beyond unitConversion nodes."),
            f("type", "t", A, "Only nodes of this type."),
            f("fullNodeName", "fnn", B, "Nodes by their full DAG paths."),
        ],
        |s, a, _| list_connections(s, a),
    );
    extend(c, "objExists", vec![], |s, a, prev| {
        if let Some(n) = a.pos_str(0) {
            if n.contains('.') {
                return Ok(json!(s.graph.plug(n).is_ok()));
            }
        }
        prev(s, a)
    });
    extend(
        c,
        "nodeType",
        vec![
            f("inherited", "i", B, "Its types, base first."),
            f("apiType", "api", B, "The API type name."),
            f("derived", "d", B, "Types derived from it."),
            f("isTypeName", "itn", B, "The argument is a type, not a node."),
            f("ufeRuntimeName", "urn", B, "The runtime the object belongs to: Maya-DG for the scene's own nodes, USD for prims of a USD stage."),
        ],
        |s, a, prev| {
            let name = a.pos_str(0).map(str::to_string).or_else(|| objects(s, a).first().cloned());
            let Some(name) = name else { fail!("nodeType: no object") };
            if a.bool("ufeRuntimeName") {
                // A prim is named proxyShape,/path.
                if name.contains(",/") {
                    return Ok(json!("USD"));
                }
                node_of(&s.graph, &name, "nodeType")?;
                return Ok(json!("Maya-DG"));
            }
            let ty = if a.bool("isTypeName") {
                let Some(t) = types::find(&name) else { fail!("nodeType: unknown type {name}") };
                t
            } else {
                let id = node_of(&s.graph, &name, "nodeType")?;
                s.graph.node(id).unwrap().type_arc().clone()
            };
            if a.bool("inherited") {
                let mut chain = Vec::new();
                let mut t = Some(ty.as_ref());
                while let Some(x) = t {
                    chain.push(x.name.clone());
                    t = x.base.as_deref();
                }
                chain.reverse();
                return Ok(json!(chain));
            }
            if a.bool("derived") {
                let out: Vec<String> = types::all().iter().filter(|t| t.is_a(&ty.name)).map(|t| t.name.clone()).collect();
                return Ok(json!(out));
            }
            if a.bool("apiType") {
                let mut cs = ty.name.chars();
                let cap: String = cs.next().map(|c| c.to_uppercase().collect::<String>() + cs.as_str()).unwrap_or_default();
                return Ok(json!(format!("k{cap}")));
            }
            if a.bool("isTypeName") {
                return Ok(json!(ty.name));
            }
            prev(s, a)
        },
    );
    extend(
        c,
        "objectType",
        vec![
            f("isAType", "isa", S, "Whether it is this type or derived from it."),
            f("isType", "i", S, "Whether it is exactly this type."),
            f("tagFromType", "tgt", S, "The type tag of the type named: the four bytes that mark its nodes in binary scene files, as a number."),
            f("typeFromTag", "tpt", ArgType::Int, "The type a type tag stands for."),
            f("typeTag", "tt", B, "The object's type tag; 0 for a type that has none."),
        ],
        |s, a, prev| {
            if let Some(t) = a.get("tagFromType").and_then(Value::as_str) {
                return Ok(json!(crate::mb_file::id_of(t).map_or(0, |b| u32::from_be_bytes(b) as i64)));
            }
            if let Some(tag) = a.get("typeFromTag").and_then(Value::as_i64) {
                return Ok(json!(crate::mb_file::type_of_id((tag as u32).to_be_bytes()).unwrap_or_default()));
            }
            if a.bool("typeTag") {
                let Some(name) = a.pos_str(0) else { fail!("objectType: name an object") };
                let id = node_of(&s.graph, name, "objectType")?;
                let ty = s.graph.node(id).unwrap().ty().name.clone();
                return Ok(json!(crate::mb_file::id_of(&ty).map_or(0, |b| u32::from_be_bytes(b) as i64)));
            }
            if let Some(t) = a.get("isAType").and_then(Value::as_str) {
                let Some(name) = a.pos_str(0) else { fail!("objectType: name an object") };
                let id = node_of(&s.graph, name, "objectType")?;
                return Ok(json!(s.graph.node(id).unwrap().ty().is_a(t)));
            }
            prev(s, a)
        },
    );
    extend(
        c,
        "rename",
        vec![f("ignoreShape", "is", B, "Leave the shape's name alone."), f("uuid", "uid", B, "The first argument is a uuid.")],
        |s, a, prev| {
            let mut call = a.clone();
            if a.bool("uuid") {
                let Some(u) = a.pos_str(0) else { fail!("rename: name a uuid") };
                let found = s.graph.nodes().into_iter().find(|&i| s.graph.node(i).is_some_and(|n| n.uuid() == u));
                let Some(id) = found else { fail!("rename: no node with uuid {u}") };
                call.positional[0] = json!(s.graph.path(id));
            }
            let r = prev(s, &call)?;
            if !a.bool("ignoreShape") {
                if let Some(id) = r.as_str().and_then(|n| s.graph.find(n).ok().flatten()) {
                    if crate::dag::is_transform(&s.graph, id) {
                        let before: Vec<String> = crate::dag::shapes(&s.graph, id, false).iter().map(|&x| s.graph.path(x)).collect();
                        if !before.is_empty() {
                            set_shape_names(&mut s.graph, id)?;
                        }
                    }
                }
            }
            Ok(json!(r.as_str().and_then(|n| s.graph.find(n).ok().flatten()).map(|i| s.graph.path(i)).unwrap_or_default()))
        },
    );
    extend(c, "createNode", vec![f("shared", "s", B, "Reuse a node of that name when there is one.")], |s, a, prev| {
        if a.bool("shared") && a.has("name") {
            if let Some(id) = s.graph.find(&a.string("name", ""))? {
                return Ok(json!(s.graph.path(id)));
            }
        }
        let r = prev(s, a)?;
        if let Some(id) = r.as_str().and_then(|n| s.graph.find(n).ok().flatten()) {
            crate::utility_nodes::seed(&mut s.graph, id)?;
        }
        Ok(r)
    });
    c.register(
        Cmd::new("shadingNode", "Create a rendering or utility node: shadingNode -asUtility multiplyDivide -n name.", |s, a| {
            let Some(ty) = a.pos_str(0) else { fail!("shadingNode: name a node type") };
            let Some(t) = types::find(ty) else { fail!("shadingNode: unknown type {ty}") };
            if !(a.bool("asRendering") || a.bool("asUtility") || a.bool("asShader") || a.bool("asTexture") || a.bool("asLight") || a.bool("asPostProcess") || a.bool("asRendering")) {
                fail!("shadingNode: give -asUtility, -asShader, -asTexture or -asLight");
            }
            if t.dag && !t.is_a("transform") {
                fail!("shadingNode: {ty} is not a shading node");
            }
            if a.bool("shared") && a.has("name") {
                if let Some(id) = s.graph.find(&a.string("name", ""))? {
                    return Ok(json!(s.graph.path(id)));
                }
            }
            let id = s.graph.create_node(&t, &a.string("name", ""), NO_NODE, true)?;
            crate::utility_nodes::seed(&mut s.graph, id)?;
            Ok(json!(s.graph.path(id)))
        })
        .positional("type")
        .flag(f("isColorManaged", "icm", B, "The node's colours are colour managed. Accepted: textures read their colour space here."))
        .flag(f("asRendering", "ar", B, "A rendering node."))
        .flag(f("asUtility", "au", B, "A utility."))
        .flag(f("asShader", "as", B, "A shader."))
        .flag(f("asTexture", "at", B, "A texture."))
        .flag(f("asLight", "al", B, "A light."))
        .flag(f("asPostProcess", "app", B, "A post process."))
        .flag(f("asRendering", "ar", B, "A rendering node."))
        .flag(f("name", "n", S, "Its name."))
        .flag(f("shared", "s", B, "Reuse a node of that name."))
        .flag(f("skipSelect", "ss", B, "Accepted: shading nodes don't change the selection.")),
    );
    c.register(
        Cmd::new("getClassification", "A node type's classification strings: utility/general, and so on.", |_s, a| {
            let Some(ty) = a.pos_str(0) else { fail!("getClassification: name a node type") };
            let classes: Vec<String> = crate::utility_nodes::classification(ty).filter(|c| !c.is_empty()).map(str::to_string).into_iter().collect();
            if let Some(want) = a.get("satisfies").and_then(Value::as_str) {
                return Ok(json!(classes.iter().flat_map(|x| x.split(':')).any(|x| x == want || x.starts_with(&format!("{want}/")))));
            }
            Ok(json!(classes))
        })
        .positional("type")
        .flag(f("satisfies", "sat", S, "Whether the type's classification satisfies this."))
        .undoable(false),
    );
    {
        let mut v = Vec::new();
        for (name, short, ty, doc) in [
            ("channels", "c", B, "Animation curves and expressions on the channels."), ("expressions", "e", B, "Expressions on the channels."), ("constraints", "cn", B, "Constraints on the object."),
            ("hierarchy", "hi", S, "below or both: the objects under it too."), ("shape", "s", B, "Only the shapes."), ("staticChannels", "sc", B, "Curves whose keys all hold one value."),
            ("timeAnimationCurves", "tac", B, "Curves over time."), ("unitlessAnimationCurves", "uac", B, "Driven key curves."), ("attribute", "at", A, "Only on these attributes."), ("inputConnectionsAndNodes", "icn", B, "Everything feeding the channels."),
            ("controlPoints", "cp", B, "Accepted for existing scripts."), ("motionPaths", "mp", B, "Accepted for existing scripts."),
            ("all", "all", B, "Every object in the scene, for the kind the other flags name: delete -all -c, delete -all -e, delete -all -cn."),
        ] {
            v.push(f(name, short, ty, doc));
        }
        extend(c, "delete", v, delete_cmd);
    }
    extend(
        c,
        "parent",
        vec![
            f("absolute", "a", B, "Keep world placement (the default)."),
            f("addObject", "add", B, "Add as an instance under the parent."),
            f("removeObject", "rm", B, "Remove this instance."),
            f("shape", "s", B, "Move the shapes, not the transforms."),
            f("noConnections", "nc", B, "Accepted for existing scripts."),
            f("noInvScale", "nis", B, "Accepted for existing scripts."),
        ],
        parent_cmd,
    );
    extend(
        c,
        "group",
        vec![f("world", "w", B, "Under the world, not the members' shared parent."), f("absolute", "a", B, "Keep world placement (the default)."), f("relative", "r", B, "Keep local placement."), f("useAsGroup", "uag", S, "Accepted for existing scripts.")],
        group_cmd,
    );
    extend(
        c,
        "ungroup",
        vec![f("world", "w", B, "Move the children to the world."), f("parent", "p", S, "Move the children here."), f("absolute", "a", B, "Keep world placement (the default)."), f("relative", "r", B, "Keep local placement.")],
        |s, a, prev| {
            if !a.bool("world") && !a.has("parent") && !a.bool("relative") {
                return prev(s, a);
            }
            let up = if a.has("parent") { node_of(&s.graph, &a.string("parent", ""), "ungroup")? } else { NO_NODE };
            let keep = !a.bool("relative");
            let mut out = Vec::new();
            for name in objects(s, a) {
                let id = node_of(&s.graph, &name, "ungroup")?;
                let to = if a.bool("world") || a.has("parent") { up } else { s.graph.parent(id) };
                for k in s.graph.children(id).to_vec() {
                    s.graph.set_parent(k, to, keep, usize::MAX)?;
                    out.push(json!(s.graph.path(k)));
                }
                s.graph.delete_node(id)?;
            }
            Ok(Value::Array(out))
        },
    );
    {
        let mut v = Vec::new();
        for (name, short, doc) in [
            ("returnRootsOnly", "rr", "Only the top nodes in the result."), ("upstreamNodes", "un", "Copy the nodes feeding it too."), ("parentOnly", "po", "Only the node itself, no children."),
            ("inputConnections", "ic", "The copy takes the same inputs."), ("renameChildren", "rc", "Give the children new unique names."), ("smartTransform", "st", "Repeat the last duplicate's step."),
            ("transformsOnly", "to", "Leave out the shapes."), ("instanceLeaf", "ilf", "Copy the hierarchy, with its leaf shapes instances of the originals'."),
        ] {
            v.push(f(name, short, B, doc));
        }
        v.push(f("groupUnder", "gu", ArgType::String, "Where the copies go: parent (beside their originals, the default), world, or group (a new group, as -group)."));
        v.push(f("fullPath", "f", B, "Return full path names rather than object names."));
        extend(c, "duplicate", v, |s, a, prev| {
            let out = duplicate_cmd(s, a, prev)?;
            if !a.bool("fullPath") {
                return Ok(out);
            }
            let g = &s.graph;
            let full = |v: &Value| match v.as_str().and_then(|n| g.find(n).ok().flatten()) {
                Some(id) => json!(g.full_path(id)),
                None => v.clone(),
            };
            Ok(match &out {
                Value::Array(list) => Value::Array(list.iter().map(full).collect()),
                other => full(other),
            })
        });
    }
    extend(c, "instance", vec![f("leaf", "lf", B, "Accepted for existing scripts."), f("smartTransform", "st", B, "Accepted for existing scripts.")], |s, a, prev| prev(s, a));
    extend(
        c,
        "select",
        vec![
            f("replace", "r", B, "Replace the selection (the default)."),
            f("noExpand", "ne", B, "A set itself, not its members."),
            f("hierarchy", "hi", B, "Everything below too."),
            f("addFirst", "af", B, "Add at the front of the list."),
            f("allDagObjects", "ado", B, "Every top level object."),
            f("allDependencyNodes", "adn", B, "Every node."),
            f("visible", "vis", B, "Only visible objects."),
            f("all", "all", B, "Every object."),
            f("symmetry", "sym", B, "Accepted for existing scripts."),
            f("symmetrySide", "sys", ArgType::Int, "Accepted for existing scripts."),
        ],
        select_cmd,
    );
    c.register(
        Cmd::new("selected", "The selection: objects, or components in a component mode.", |s, _| run(s, "ls", json!({"selection": true}))).undoable(false),
    );
    extend(
        c,
        "pickWalk",
        vec![
            f("direction", "d", S, "up, down, left or right."),
            f("type", "typ", S, "Accepted for existing scripts."),
            f("recurse", "r", B, "Keep walking: up to the top of the hierarchy, or down to the end of it."),
        ],
        |s, a, prev| {
            prev(s, a)?;
            let dir = a.string("direction", "down");
            if a.bool("recurse") && matches!(dir.as_str(), "up" | "down") {
                let mut call = a.clone();
                call.positional.clear();
                for _ in 0..1000 {
                    let before = s.graph.selection().to_vec();
                    prev(s, &call)?;
                    if s.graph.selection() == before.as_slice() {
                        break;
                    }
                }
            }
            run(s, "ls", json!({"selection": true}))
        },
    );
    {
        let mut sets = Cmd::new("sets", "The reference's object set command: create, add to, take from, list and combine sets.", sets_cmd).positional("items, or a set with -q");
        sets = sets.flag(f("query", "q", B, "List a set's members.")).flag(f("edit", "e", B, "Edit.")).flag(f("queried", "", ArgType::List, "What -q asks for."));
        for (name, short, ty, doc) in [
            ("name", "n", S, "The new set's name."), ("empty", "em", B, "Hold nothing."), ("addElement", "add", S, "Add the items to this set."), ("forceElement", "fe", S, "Add the items to this set."),
            ("include", "in", S, "Add the items to this set."), ("remove", "rm", S, "Take the items out of this set."), ("clear", "cl", S, "Empty this set."), ("isMember", "im", S, "Whether the items are in this set."),
            ("isIntersecting", "ii", S, "Whether this set and the others share members."), ("union", "un", S, "This set's members and the others'."), ("intersection", "int", S, "What this set shares with the others."), ("subtract", "sub", S, "This set less the others."),
            ("size", "s", B, "With -q: how many."), ("flatten", "fl", B, "Accepted for existing scripts."), ("noWarnings", "nw", B, "Accepted for existing scripts."), ("renderable", "r", B, "Accepted for existing scripts."), ("vertices", "v", B, "Accepted for existing scripts."), ("edges", "ed", B, "Accepted for existing scripts."), ("facets", "fc", B, "Accepted for existing scripts."), ("text", "t", S, "A note kept with the set: on creation, with -e, or read with -q."), ("anyMember", "am", S, "Whether any of the items is in this set."), ("copy", "cp", S, "A new set holding this set's members."), ("split", "sp", S, "The items moved out of this set into a new one."), ("nodesOnly", "no", B, "With -q: each node once."), ("ordered", "o", B, "Members in the order they were added (as they are kept)."), ("noIntermediate", "ni", B, "Accepted for existing scripts."), ("color", "co", ArgType::Int, "Accepted for existing scripts."), ("layer", "l", B, "Obsolete."),
        ] {
            sets = sets.flag(f(name, short, ty, doc));
        }
        c.register(sets);
    }
    extend(
        c,
        "undoInfo",
        vec![
            f("query", "q", B, "Query."),
            f("queried", "", ArgType::List, "What -q asks for."),
            f("state", "st", B, "Turn undo on or off; off flushes the queue, as in the reference."),
            f("stateWithoutFlush", "swf", B, "Turn undo on or off, keeping the queue."),
            f("undoName", "un", S, "With -q: what undo would undo."),
            f("redoName", "rn", S, "With -q: what redo would redo."),
            f("undoQueueEmpty", "uqe", B, "With -q: whether there is nothing to undo."),
            f("redoQueueEmpty", "rqe", B, "With -q: whether there is nothing to redo."),
            f("length", "l", ArgType::Int, "How many steps to keep."),
            f("infinity", "infinity", B, "Keep every step."),
            f("printQueue", "pq", B, "Accepted for existing scripts."),
        ],
        |s, a, prev| {
            if a.bool("query") {
                let u = s.graph.undo_stack();
                let has = |x: &str| asked(a, x);
                return Ok(if has("undoName") {
                    json!(u.undo_label())
                } else if has("redoName") {
                    json!(u.redo_label())
                } else if has("undoQueueEmpty") {
                    json!(!u.can_undo())
                } else if has("redoQueueEmpty") {
                    json!(u.redo_count() == 0)
                } else if has("length") {
                    json!(u.undo_count())
                } else if has("infinity") {
                    json!(true)
                } else {
                    json!(u.enabled())
                });
            }
            if a.has("stateWithoutFlush") {
                s.graph.undo_stack_mut().set_enabled(a.bool("stateWithoutFlush"));
            }
            if a.has("length") {
                s.graph.undo_stack_mut().set_limit(a.int("length", 0).max(0) as usize);
            }
            if a.bool("infinity") {
                s.graph.undo_stack_mut().set_limit(0);
            }
            let mut call = a.clone();
            call.flags.remove("stateWithoutFlush");
            prev(s, &call)
        },
    );
    {
        let mut ab = Cmd::new("about", "About the application and the system: -v, -os, -win, -linux, -mac, -batch, -api, -qt.", about).undoable(false);
        for (name, short) in [("version", "v"), ("apiVersion", "api"), ("operatingSystem", "os"), ("windows", "win"), ("nt", "nt"), ("linux", "li"), ("macOS", "mac"), ("batch", "b"), ("is64", "x64"), ("uiLanguage", "uil"), ("codeset", "cs"), ("product", "p"), ("application", "a"), ("qtVersion", "qt")] {
            ab = ab.flag(f(name, short, B, "What to ask."));
        }
        c.register(ab);
    }
    {
        let mut ov = Cmd::new("optionVar", "Named preference values kept between sessions, with defaults, categories and stashes.", option_var).positional("a name with -q").undoable(false);
        ov = ov
            .flag(f("query", "q", B, "The value of the name given; with -default, its default."))
            .flag(f("default", "d", B, "Set the default as well as the value; with -q, -exists, -list or -listModified, work on defaults."))
            .flag(f("transient", "t", B, "Not kept between sessions; with -exists or -list, only transient names."))
            .flag(f("init", "ini", B, "true: set value and default; false: only the default (and the value if the name isn't set)."))
            .flag(f("category", "cat", S, "The category the names set belong to; with -list or -listModified, only that category."))
            .flag(f("prefFile", "pf", S, "With -category: the preferences file the category is saved to."))
            .flag(f("list", "l", B, "Every name."))
            .flag(f("listModified", "lm", B, "The names whose value differs from their default."))
            .flag(f("listCategories", "lc", B, "Every category."))
            .flag(f("exists", "ex", S, "Whether this name is set."))
            .flag(f("arraySize", "as", S, "The array's size: 0 when unset, 1 when not an array."))
            .flag(f("version", "v", ArgType::Int, "The preferences version."));
        for (name, short) in [("remove", "rm"), ("clearArray", "ca"), ("intArray", "ia"), ("floatArray", "fa"), ("stringArray", "sa"), ("stash", "st"), ("unstash", "us"), ("clearStash", "cs")] {
            ov = ov.flag(f(name, short, A, "A name.").multi(1));
        }
        for (name, short, n) in [
            ("intValue", "iv", 2), ("intValue2", "iv2", 3), ("intValue3", "iv3", 4), ("intValue4", "iv4", 5),
            ("floatValue", "fv", 2), ("floatValue2", "fv2", 3), ("floatValue3", "fv3", 4), ("floatValue4", "fv4", 5),
            ("stringValue", "sv", 2), ("intValueAppend", "iva", 2), ("floatValueAppend", "fva", 2), ("stringValueAppend", "sva", 2),
            ("removeFromArray", "rfa", 2),
        ] {
            ov = ov.flag(f(name, short, A, "A name and its values.").multi(n));
        }
        c.register(ov);
    }
    c.register(
        Cmd::new("getenv", "An environment variable's value, or \"\".", |_s, a| {
            let Some(n) = a.pos_str(0) else { fail!("getenv: name a variable") };
            Ok(json!(std::env::var(n).unwrap_or_default()))
        })
        .positional("name")
        .undoable(false),
    );
    c.register(
        Cmd::new("putenv", "Sets an environment variable for this process.", |_s, a| {
            let (Some(n), Some(v)) = (a.pos_str(0), a.pos_str(1)) else { fail!("putenv: expected (name, value)") };
            // Commands run on the main thread, before anything else reads the environment concurrently.
            unsafe { std::env::set_var(n, v) };
            Ok(Value::Null)
        })
        .positional("name value")
        .undoable(false),
    );
    c.register(
        Cmd::new("env", "Every environment variable, as NAME=value lines.", |_s, _a| Ok(json!(std::env::vars().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>()))).undoable(false),
    );
    c.register(workspace_cmd());
    {
        let mut tg = Cmd::new("toggle", "Turns display switches on the objects on and off: -la, -rp, -sp, -sh, -te; -st for a value, -q to ask.", toggle_cmd).positional("objects");
        for (name, short) in [("localAxis", "la"), ("rotatePivot", "rp"), ("scalePivot", "sp"), ("selectHandle", "sh"), ("template", "te")] {
            tg = tg.flag(f(name, short, B, "What to switch."));
        }
        c.register(tg.flag(f("state", "st", B, "On or off, rather than the other way round.")).flag(f("query", "q", B, "Query.")).flag(f("queried", "", ArgType::List, "What -q asks for.")));
    }
    extend(
        c,
        "hide",
        vec![
            f("clearSelection", "cs", B, "Clear the selection afterwards."),
            f("returnHidden", "rh", B, "Return what was hidden."),
            f("invertComponents", "ic", B, "Accepted for existing scripts."),
            f("testVisibility", "tv", B, "Change nothing: 1 when they are all hidden, 2 all visible, 3 some of each."),
            f("clearLastHidden", "clh", B, "Forget what was hidden last: the caller keeps its own list for showing them again."),
            f("allObjects", "all", B, "Hide every top level object."),
        ],
        |s, a, prev| {
            let clear = a.bool("clearLastHidden");
            let mut a = a.clone();
            if a.bool("allObjects") {
                // Every object at the top of the hierarchy, the scene's own cameras aside.
                let g = &s.graph;
                let tops: Vec<Value> = g
                    .nodes()
                    .into_iter()
                    .filter(|&n| g.parent(n) == crate::plug::NO_NODE && g.node(n).is_some_and(|x| x.ty().dag && x.ty().is_a("transform")) && !crate::defaults::undeletable(g, n))
                    .map(|n| json!(g.path(n)))
                    .collect();
                if tops.is_empty() {
                    return Ok(Value::Null);
                }
                a.positional = tops;
            }
            let a = &a;
            let r = (|| -> CmdResult {
                if a.bool("testVisibility") {
                    let names = objects(s, a);
                    let ids: Vec<_> = names.iter().filter_map(|n| s.graph.find(n).ok().flatten()).collect();
                    let shown = ids.iter().filter(|&&i| crate::transform::effectively_visible(&s.graph, i)).count();
                    return Ok(json!(if shown == 0 { 1 } else if shown == ids.len() { 2 } else { 3 }));
                }
                let names = objects(s, a);
                let r = prev(s, a)?;
                if a.bool("clearSelection") {
                    s.graph.select(&[], crate::graph::SelectMode::Replace);
                }
                Ok(if a.bool("returnHidden") { json!(names) } else { r })
            })();
            if clear {
                s.graph.last_hidden.clear();
            }
            r
        },
    );
    c.register(
        Cmd::new("showHidden", "Show objects: named, -a every one, -b those below the named. Components (or, with none named, the component selection in a component mode) show their hidden faces.", |s, a| {
            if !a.bool("all") && !a.bool("below") && !a.bool("above") {
                if let Some(faces) = crate::hidden_components::picked_faces(&mut s.graph, &a.named()).map_err(aztek_ops::CommandError::new)? {
                    crate::hidden_components::show_picked(&mut s.graph, &faces);
                    return Ok(Value::Null);
                }
            }
            let g = &mut s.graph;
            let mut ids = Vec::new();
            if a.bool("all") {
                ids = g.nodes().into_iter().filter(|&i| g.node(i).is_some_and(|n| n.ty().dag)).collect();
            }
            for n in a.named() {
                let id = node_of(g, &n, "showHidden")?;
                if !a.bool("below") {
                    ids.push(id);
                }
                if a.bool("below") {
                    ids.extend(g.descendants(id));
                }
                if a.bool("above") {
                    let mut p = g.parent(id);
                    while p != NO_NODE {
                        ids.push(p);
                        p = g.parent(p);
                    }
                }
            }
            for id in ids {
                if !g.node(id).is_some_and(|n| n.bool(DagNode::INTERMEDIATE_OBJECT)) {
                    g.set_value(PlugRef::new(id, DagNode::VISIBILITY as i32), PlugValue::Bool(true))?;
                }
            }
            Ok(Value::Null)
        })
        .positional("objects")
        .flag(f("all", "a", B, "Every hidden object."))
        .flag(f("below", "b", B, "The objects under those named."))
        .flag(f("above", "ab", B, "The objects above those named.")),
    );
    c.register(
        Cmd::new("displayCull", "Back face culling per object, kept for scripts that ask: -bfc on or off, -q.", |s, a| {
            let names = objects(s, a);
            let key = |n: &str| format!("__cull:{n}");
            if a.bool("query") {
                let Some(n) = names.first() else { fail!("displayCull: name an object") };
                return Ok(s.option_vars.get(&key(n)).cloned().unwrap_or(json!(false)));
            }
            if a.has("backFaceCulling") {
                for n in &names {
                    s.option_vars.insert(key(n), json!(a.bool("backFaceCulling")));
                    // The views cull it too.
                    if let Ok(Some(id)) = s.graph.find(n) {
                        s.extras.display_surface.insert((id, "backFaceCulling".to_string()), a.bool("backFaceCulling"));
                    }
                }
            }
            Ok(Value::Null)
        })
        .positional("objects")
        .flag(f("backFaceCulling", "bfc", B, "On or off."))
        .flag(f("query", "q", B, "Query."))
        .flag(f("queried", "", ArgType::List, "What -q asks for."))
        .undoable(false),
    );
    c.register(
        Cmd::new("refresh", "Brings the scene up to date; the viewports draw on their own.", |s, _| {
            s.graph.evaluate();
            Ok(Value::Null)
        })
        .flag(f("force", "f", B, "Accepted for existing scripts."))
        .flag(f("suspend", "su", B, "Accepted for existing scripts."))
        .flag(f("currentView", "cv", B, "Accepted for existing scripts."))
        .undoable(false),
    );
    c.register(
        Cmd::new(
            "dgdirty",
            "Sends dirty messages to the nodes or plugs named or selected, or -a every one; -c cleans them (brings them up to date) \
             instead. Returns how many plugs took a message. -l data, plug or connection lists the dirty ones (the clean ones with -c) \
             rather than sending anything.",
            dgdirty,
        )
        .positional("objects or plugs")
        .flag(f("allPlugs", "a", B, "Every node."))
        .flag(f("clean", "c", B, "Clean rather than dirty; with -l, list the clean ones."))
        .flag(f("verbose", "v", B, "Print each plug set dirty."))
        .flag(f("implicit", "i", B, "With -a: the nodes every scene has too."))
        .flag(f("list", "l", A, "List instead: data (plugs with dirty data), plug (plugs in a dirty state), connection (dirty connections, in pairs)."))
        .flag(f("propagation", "p", B, "Dirty messages flow on through the graph (as they always do here)."))
        .flag(f("showTiming", "st", B, "Print how long the messages took."))
        .flag(f("query", "q", B, "With -l: the kinds of list there are."))
        .flag(f("queried", "", A, "Flags asked about in query mode."))
        .undoable(false),
    );
    c.register(
        Cmd::new("dgeval", "Brings the nodes or plugs named or selected up to date. Returns how many plugs were evaluated.", |s, a| {
            let names = objects(s, a);
            let g = &mut s.graph;
            let mut count = 0usize;
            for n in names {
                if n.contains('.') {
                    let p = g.plug(&n)?;
                    let v = g.value(&p)?;
                    count += 1;
                    if a.bool("verbose") {
                        println!("{n} = {v:?}");
                    }
                } else {
                    let id = node_of(g, &n, "dgeval")?;
                    // Its connected plugs.
                    let plugs: Vec<PlugRef> = g.connections().into_iter().flat_map(|(d, sr)| [d, sr]).filter(|p| p.node == id).collect();
                    for p in plugs {
                        let v = g.value(&p)?;
                        count += 1;
                        if a.bool("verbose") {
                            println!("{} = {v:?}", g.plug_name(&p));
                        }
                    }
                }
            }
            g.evaluate();
            Ok(json!(count))
        })
        .positional("objects or plugs")
        .flag(f("verbose", "v", B, "Print what each plug evaluated to."))
        .flag(f("src", "src", B, "Obsolete: has no effect."))
        .undoable(false),
    );
    c.register(
        Cmd::new("reorder", "Moves objects among their siblings: -f front, -b back, -r n places.", |s, a| {
            let g = &mut s.graph;
            for n in objects_of(a, g) {
                let id = node_of(g, &n, "reorder")?;
                let p = g.parent(id);
                let sibs = if p == NO_NODE { g.roots().to_vec() } else { g.children(p).to_vec() };
                let at = sibs.iter().position(|&x| x == id).unwrap_or(0) as i64;
                let to = if a.bool("front") {
                    0
                } else if a.bool("back") {
                    sibs.len() as i64 - 1
                } else {
                    (at + a.int("relative", 0)).clamp(0, sibs.len() as i64 - 1)
                };
                g.set_parent(id, p, false, to as usize)?;
            }
            Ok(Value::Null)
        })
        .positional("objects")
        .flag(f("front", "f", B, "First among its siblings."))
        .flag(f("back", "b", B, "Last among its siblings."))
        .flag(f("relative", "r", ArgType::Int, "This many places along.")),
    );
    extend(
        c,
        "selectMode",
        vec![
            f("query", "q", B, "Query."),
            f("queried", "", ArgType::List, "What -q asks for."),
            f("object", "o", B, "Object mode."),
            f("component", "co", B, "Component mode."),
            f("hierarchical", "h", B, "Hierarchy mode: objects here."),
            f("preset", "p", B, "Accepted for existing scripts."),
            f("root", "r", B, "Accepted for existing scripts."),
            f("leaf", "l", B, "Accepted for existing scripts."),
            f("template", "t", B, "Accepted for existing scripts."),
        ],
        |s, a, prev| {
            let plain = Args::default();
            if a.bool("query") {
                let now = prev(s, &plain)?;
                let object = now == "object";
                return Ok(if asked(a, "component") { json!(!object) } else if asked(a, "object") || asked(a, "hierarchical") { json!(object) } else { now });
            }
            if a.bool("object") || a.bool("hierarchical") {
                let call = Args { positional: vec![json!("object")], ..Default::default() };
                return prev(s, &call);
            }
            if a.bool("component") {
                let now = prev(s, &plain)?;
                if now == "object" {
                    let call = Args { positional: vec![json!("vertex")], ..Default::default() };
                    return prev(s, &call);
                }
                return Ok(now);
            }
            prev(s, a)
        },
    );
    c.register(
        Cmd::new("hilite", "Hilite objects for component work: named, -r to replace, -u to take away, -tgl.", |s, a| {
            let names = a.named();
            let mode = if a.bool("unHilite") { "deselect" } else if a.bool("toggle") { "toggle" } else if a.bool("replace") { "" } else { "add" };
            let mut call = json!({"args": names});
            if !mode.is_empty() {
                call[mode] = json!(true);
            }
            run(s, "select", call)
        })
        .positional("objects")
        .flag(f("replace", "r", B, "Replace what is hilited."))
        .flag(f("toggle", "tgl", B, "Toggle."))
        .flag(f("unHilite", "u", B, "Take away.")),
    );
    c.register(
        Cmd::new("align", "Align or spread objects along X, Y and Z by their bounding boxes: align -x mid a b c. Per axis none, min, mid or max (to their average, or the lead object's with -atl), dist (spread evenly) or stack (end to end). -cs takes the axes from an object. Returns true.", align_cmd)
            .positional("objects (the selection when none)")
            .flag(f("xAxis", "x", S, "none, min, mid, max, dist or stack (none)."))
            .flag(f("yAxis", "y", S, "none, min, mid, max, dist or stack (none)."))
            .flag(f("zAxis", "z", S, "none, min, mid, max, dist or stack (none)."))
            .flag(f("coordinateSystem", "cs", S, "An object whose axes are X, Y and Z (the world's)."))
            .flag(f("alignToLead", "atl", B, "Min, mid or max from the lead object (the last selected) rather than averaged over all.")),
    );
    {
        let mut sn = Cmd::new("snapMode", "Move snapping switches: -grid, -point, -curve; -q to ask.", |s, a| {
            for key in ["grid", "point", "curve", "liveFace", "viewPlane", "meshCenter"] {
                if a.bool("query") {
                    if asked(a, key) {
                        return Ok(json!(s.snap_modes.get(key).copied().unwrap_or(false)));
                    }
                } else if a.has(key) {
                    s.snap_modes.insert(key.to_string(), a.bool(key));
                }
            }
            Ok(Value::Null)
        })
        .undoable(false);
        for (name, short) in [("grid", "gr"), ("point", "p"), ("curve", "c"), ("liveFace", "lf"), ("viewPlane", "vp"), ("meshCenter", "mc")] {
            sn = sn.flag(f(name, short, B, "This snap."));
        }
        c.register(sn.flag(f("query", "q", B, "Query.")).flag(f("queried", "", ArgType::List, "What -q asks for.")));
    }
    c.register(
        Cmd::new("angleBetween", "The rotation from -v1 to -v2: axis and angle in degrees, or -er Euler angles. -ch on makes an angleBetween node instead and returns it.", |s, a| {
            let vec_of = |whole: &str, parts: [&str; 3]| -> Option<DVec3> {
                let mut v = a.vec3(whole).map(DVec3::from_array);
                for (i, p) in parts.iter().enumerate() {
                    if let Some(x) = a.get(p).and_then(Value::as_f64) {
                        let mut w = v.unwrap_or(DVec3::ZERO);
                        w[i] = x;
                        v = Some(w);
                    }
                }
                v
            };
            let v1 = vec_of("vector1", ["vector1X", "vector1Y", "vector1Z"]);
            let v2 = vec_of("vector2", ["vector2X", "vector2Y", "vector2Z"]);
            if a.get("constructionHistory").is_some_and(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0)) {
                let g = &mut s.graph;
                let t = crate::node::types::find("angleBetween").expect("angleBetween");
                let n = g.create_node(&t, "angleBetween1", crate::plug::NO_NODE, true)?;
                for (attr, v) in [("vector1", v1), ("vector2", v2)] {
                    if let (Some(v), Some(i)) = (v, t.attr_index(attr)) {
                        g.set_value(PlugRef::new(n, i as i32), PlugValue::Vec3(v))?;
                    }
                }
                crate::history_qe::apply_common(g, n, a)?;
                g.evaluate();
                return Ok(json!(g.path(n)));
            }
            let (Some(x), Some(y)) = (v1, v2) else { fail!("angleBetween: give -v1 and -v2") };
            let (x, y) = (x.normalize_or_zero(), y.normalize_or_zero());
            let angle = x.angle_between(y);
            let axis = x.cross(y).normalize_or_zero();
            if a.bool("euler") {
                let q = if axis == DVec3::ZERO { glam::DQuat::IDENTITY } else { glam::DQuat::from_axis_angle(axis, angle) };
                let e = crate::transform::matrix_to_euler(&glam::DMat3::from_quat(q), crate::transform::RotateOrder::Xyz);
                let k = if aztek_ops::units::user_call() { 1.0 / aztek_ops::units::rad_per_unit() } else { 1.0 };
                return Ok(json!([e.x * k, e.y * k, e.z * k]));
            }
            let angle = if aztek_ops::units::user_call() { aztek_ops::units::to_user(aztek_ops::units::Unit::Angle, angle) } else { angle };
            Ok(json!([axis.x, axis.y, axis.z, angle]))
        })
        .flag(f("vector1", "v1", ArgType::Vec3, "The vector to rotate from."))
        .flag(f("vector1X", "v1x", ArgType::Float, "vector1's X."))
        .flag(f("vector1Y", "v1y", ArgType::Float, "vector1's Y."))
        .flag(f("vector1Z", "v1z", ArgType::Float, "vector1's Z."))
        .flag(f("vector2", "v2", ArgType::Vec3, "The vector to rotate to."))
        .flag(f("vector2X", "v2x", ArgType::Float, "vector2's X."))
        .flag(f("vector2Y", "v2y", ArgType::Float, "vector2's Y."))
        .flag(f("vector2Z", "v2z", ArgType::Float, "vector2's Z."))
        .flag(f("euler", "er", B, "Euler angles instead of axis and angle."))
        .flag(f("constructionHistory", "ch", B, "Make an angleBetween node computing it, and return the node."))
        .flag(f("caching", "cch", B, "With -ch: the node's caching."))
        .flag(f("nodeState", "nds", ArgType::Int, "With -ch: the node's state."))
    );
    c.register(
        Cmd::new("getModifiers", "Which modifier keys are down, as bits: none, when a script asks.", |_s, _a| Ok(json!(0))).undoable(false),
    );
    c.register(
        Cmd::new("exactWorldBoundingBox", "World bounds of objects with what is under them: xmin ymin zmin xmax ymax zmax.", |s, a| {
            let names = objects(s, a);
            let found = bounds_of(&mut s.graph, &names, a.bool("ignoreInvisible"))?;
            let (lo, hi) = found.unwrap_or((DVec3::ZERO, DVec3::ZERO));
            Ok(json!([lo.x, lo.y, lo.z, hi.x, hi.y, hi.z]))
        })
        .positional("objects")
        .flag(f("ignoreInvisible", "ii", B, "Leave out hidden shapes."))
        .flag(f("calculateExactly", "ce", B, "Accepted: bounds are always exact here."))
        .undoable(false),
    );
    extend(
        c,
        "polyEvaluate",
        vec![
            f("area", "a", B, "Area, object space."),
            f("worldArea", "wa", B, "Area, world space."),
            f("boundingBoxComponent", "bc", B, "World bounds of the selected components."),
            f("boundingBoxComponent2d", "bc2", B, "UV bounds of the selected components."),
            f("boundingBox2d", "b2", B, "UV bounds."),
            f("format", "fmt", B, "The counts as text."),
            f("accurateEvaluation", "ae", B, "Accepted: counts are always exact here."),
            f("ac", "ac", B, "Accepted for existing scripts."),
            f("triangleComponent", "tc", B, "Triangle count of the selected components."),
            f("usu", "usu", B, "Accepted for existing scripts."),
            f("vertexComponent", "vc", B, "Selected vertex count."),
            f("edgeComponent", "ec", B, "Selected edge count."),
            f("faceComponent", "fc", B, "Selected face count."),
            f("uvComponent", "uvc", B, "Selected UV count."),
            f("uvShell", "us", B, "UV shell count."),
            f("uvShellIds", "uid", B, "The UV shell of each selected UV."),
            f("uvsInShell", "uis", ArgType::Int, "The UVs in this UV shell."),
            f("activeUVShells", "aus", B, "The UV shells the selection touches."),
            f("activeShells", "as", B, "The mesh shells the selection touches."),
            f("uvSetName", "uvs", S, "The UV set: the current one here."),
            f("displayStats", "ds", B, "Accepted for existing scripts."),
        ],
        poly_evaluate,
    );
    c.register(Cmd::new("searchReplaceNames", "searchReplaceNames(search, replace, 'all'|'hierarchy'|'selected'): renames what holds search. Returns how many.", search_replace).positional("search replace scope"));
    c.register(
        Cmd::new("prefixHierarchy", "prefixHierarchy(root, prefix): puts prefix in front of the name of root and all under it.", |s, a| {
            let (Some(root), Some(prefix)) = (a.pos_str(0).map(str::to_string), a.pos_str(1).map(str::to_string)) else { fail!("prefixHierarchy: expected (root, prefix)") };
            let g = &mut s.graph;
            let id = node_of(g, &root, "prefixHierarchy")?;
            let mut ids = g.descendants(id);
            ids.reverse();
            ids.push(id);
            let mut out = Vec::new();
            for x in ids {
                let name = g.node(x).unwrap().name().to_string();
                g.rename(x, &format!("{prefix}{name}"))?;
                out.push(json!(g.path(x)));
            }
            out.reverse();
            Ok(Value::Array(out))
        })
        .positional("root prefix"),
    );
    c.register(
        Cmd::new("pwd", "The current working directory.", |_s, _a| {
            std::env::current_dir().map(|d| json!(d.to_string_lossy().replace('\\', "/"))).map_err(|e| aztek_ops::CommandError::new(e.to_string()))
        })
        .undoable(false),
    );
    c.register(
        Cmd::new("chdir", "Change the current working directory.", |_s, a| {
            let Some(d) = a.pos_str(0) else { fail!("chdir: give a directory") };
            std::env::set_current_dir(d).map_err(|e| aztek_ops::CommandError::new(format!("chdir: {d}: {e}")))?;
            Ok(Value::Null)
        })
        .positional("directory")
        .undoable(false),
    );
    extend(c, "ls", vec![f("uuid", "uid", B, "UUIDs rather than names.")], |s, a, prev| {
        // A UUID given stands for its node.
        let mut call = a.clone();
        let is_uuid = |x: &str| x.len() == 36 && x.as_bytes().get(8) == Some(&b'-');
        let mut uuid_named = false;
        for v in call.positional.iter_mut() {
            if let Some(u) = v.as_str().filter(|x| is_uuid(x)).map(str::to_string) {
                uuid_named = true;
                if let Some(id) = s.graph.nodes().into_iter().find(|&i| s.graph.node(i).is_some_and(|n| n.uuid() == u)) {
                    *v = json!(s.graph.path(id));
                } else {
                    *v = json!("__no_such_node__");
                }
            }
        }
        call.flags.remove("uuid");
        let r = prev(s, &call)?;
        if !a.bool("uuid") || uuid_named {
            return Ok(r);
        }
        let names: Vec<String> = r.as_array().map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let mut out = Vec::new();
        for n in names {
            if let Ok(Some(id)) = s.graph.find(&n) {
                out.push(json!(s.graph.node(id).unwrap().uuid()));
            }
        }
        Ok(Value::Array(out))
    });
    {
        let mut ns = Cmd::new("namespace", "The reference's namespace command: -add, -set, -rm, -ex, -q, -mnr, -dnc, -ren.", namespace_cmd).positional("[namespace]");
        ns = ns.flag(f("query", "q", B, "Query.")).flag(f("queried", "", ArgType::List, "What -q asks for."));
        for (name, short, ty) in [
            ("addNamespace", "add", S), ("setNamespace", "set", S), ("removeNamespace", "rm", S), ("parent", "p", S), ("exists", "ex", A), ("relativeNames", "rel", B), ("isRootNamespace", "ir", A), ("validateName", "vn", S),
            ("mergeNamespaceWithRoot", "mnr", B), ("mergeNamespaceWithParent", "mnp", B), ("mergeNamespaceWithOther", "mno", S), ("collapseAncestors", "ch", S), ("deleteNamespaceContent", "dnc", B), ("rename", "ren", A), ("moveNamespace", "mv", A), ("force", "f", B), ("recurse", "r", B), ("absoluteName", "an", B),
        ] {
            ns = ns.flag(f(name, short, ty, "As in the reference."));
        }
        c.register(ns);
    }
    {
        let mut ni = Cmd::new("namespaceInfo", "What a namespace holds: -cur, -lon, -lod, -ls, -r, -dp, -bn, -p.", namespace_info).positional("[namespace]").undoable(false);
        for (name, short) in [("currentNamespace", "cur"), ("listOnlyNamespaces", "lon"), ("listOnlyDependencyNodes", "lod"), ("listNamespace", "ls"), ("recurse", "r"), ("dagPath", "dp"), ("fullName", "fn"), ("absoluteName", "an"), ("baseName", "bn"), ("parent", "p"), ("isRootNamespace", "ir"), ("shortName", "sn"), ("internal", "int")] {
            ni = ni.flag(f(name, short, B, "As in the reference."));
        }
        c.register(ni);
    }
}

fn objects_of(a: &Args, g: &Graph) -> Vec<String> {
    let named = a.named();
    if named.is_empty() {
        g.selection().iter().map(|&i| g.path(i)).collect()
    } else {
        named
    }
}
