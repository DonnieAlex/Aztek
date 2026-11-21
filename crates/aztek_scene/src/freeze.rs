//! The reference's freeze and freezeOptions, and the parts of evaluationManager that look at the graph: frozen nodes keep
//! what they computed (the graph's evaluation honours `frozen`, and `frozenAffected` where a freeze added it), the freeze
//! command sets the state directly and spreads it up or down the graph, and its options are preferences.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::{AttrDef, NodeId, PlugRef, PlugType, PlugValue};
use crate::ref_core::{asked, f};
use crate::session::Session;

type Cmd = CommandDef<Session>;
use ArgType::{Bool as B, Int as I, String as S};

/// What evaluationManager holds beyond its mode.
#[derive(Default)]
pub struct EvalManager {
    /// nodeTypeParallel, nodeTypeSerialize, nodeTypeGloballySerialize and nodeTypeUntrusted: the types each is on for.
    pub type_overrides: BTreeMap<String, BTreeSet<String>>,
    /// idleAction, manipulationPrevalidation, reduceGraphRebuild, safeMode and the rest, as set.
    pub settings: BTreeMap<String, Value>,
}

const OVERRIDES: &[&str] = &["nodeTypeParallel", "nodeTypeSerialize", "nodeTypeGloballySerialize", "nodeTypeUntrusted"];

fn on(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Value::String(s) => matches!(s.as_str(), "true" | "on" | "yes" | "1"),
        _ => false,
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// The graph as evaluation sees it

/// Nodes feeding a node through connections, and those it feeds. Parenting counts: a parent's world matrix feeds its
/// children.
fn neighbours(g: &Graph, n: NodeId, down: bool) -> Vec<NodeId> {
    let mut out: Vec<NodeId> = Vec::new();
    for (d, s) in g.connections() {
        if down && s.node == n && d.node != n {
            out.push(d.node);
        } else if !down && d.node == n && s.node != n {
            out.push(s.node);
        }
    }
    if g.node(n).is_some_and(|x| x.ty().dag) {
        if down {
            out.extend(g.children(n));
        } else {
            let p = g.parent(n);
            if p != crate::plug::NO_NODE {
                out.push(p);
            }
        }
    }
    let mut seen = HashSet::new();
    out.retain(|x| seen.insert(*x));
    out
}

/// A breadth-first walk from a node: (level, node) for it and everything reached, each once.
fn walk(g: &Graph, from: NodeId, down: bool) -> Vec<(usize, NodeId)> {
    let mut out = vec![(0, from)];
    let mut seen: HashSet<NodeId> = [from].into();
    let mut queue: VecDeque<(usize, NodeId)> = [(0, from)].into();
    while let Some((lvl, n)) = queue.pop_front() {
        for m in neighbours(g, n, down) {
            if seen.insert(m) {
                out.push((lvl + 1, m));
                queue.push_back((lvl + 1, m));
            }
        }
    }
    out
}

/// The nodes in a cycle with this one: those both upstream and downstream of it.
fn cycle_of(g: &Graph, n: NodeId) -> Vec<NodeId> {
    let down: HashSet<NodeId> = walk(g, n, true).into_iter().skip(1).map(|x| x.1).collect();
    let up: Vec<NodeId> = walk(g, n, false).into_iter().skip(1).map(|x| x.1).collect();
    let mut out: Vec<NodeId> = up.into_iter().filter(|x| down.contains(x)).collect();
    if out.is_empty() {
        return out;
    }
    out.push(n);
    out.sort_unstable();
    out
}

fn named(g: &Graph, name: &str, cmd: &str) -> CmdResult<NodeId> {
    match g.find(name)? {
        Some(n) => Ok(n),
        None => fail!("{cmd}: no object matches name: {name}"),
    }
}

/// evaluationManager's flags that look at the graph and hold its finer settings. None when the call names none of them.
pub(crate) fn evaluation_manager(s: &mut Session, a: &Args) -> Option<CmdResult> {
    let mode_on = s.extras.evaluation_mode.as_deref().is_some_and(|m| m != "off");
    let q = a.bool("query");
    let text = |k: &str| a.get(k).and_then(Value::as_str).map(str::to_string);
    for (flag, down) in [("downstreamFrom", true), ("upstreamFrom", false)] {
        if let Some(name) = text(flag) {
            let g = &s.graph;
            let n = match named(g, &name, "evaluationManager") {
                Ok(n) => n,
                Err(e) => return Some(Err(e)),
            };
            if q {
                // The whole walk: level then node, for each node reached.
                let mut out = Vec::new();
                for (lvl, m) in walk(g, n, down) {
                    out.push(json!(lvl.to_string()));
                    out.push(json!(g.path(m)));
                }
                return Some(Ok(Value::Array(out)));
            }
            return Some(Ok(Value::Array(neighbours(g, n, down).into_iter().map(|m| json!(g.path(m))).collect())));
        }
    }
    if let Some(name) = text("cycleCluster") {
        let g = &s.graph;
        let n = match named(g, &name, "evaluationManager") {
            Ok(n) => n,
            Err(e) => return Some(Err(e)),
        };
        if !mode_on {
            return Some(Ok(json!([])));
        }
        return Some(Ok(Value::Array(cycle_of(g, n).into_iter().map(|m| json!(g.path(m))).collect())));
    }
    let objects: Vec<String> = a.named();
    for flag in OVERRIDES {
        if !a.has(flag) {
            continue;
        }
        let em = &mut s.extras.eval_manager;
        if q {
            let set = em.type_overrides.get(*flag).cloned().unwrap_or_default();
            if objects.is_empty() {
                return Some(Ok(Value::Array(set.into_iter().map(Value::from).collect())));
            }
            return Some(Ok(Value::Array(objects.iter().map(|t| json!(set.contains(t))).collect())));
        }
        if objects.is_empty() {
            return Some(Err(aztek_ops::CommandError::new(format!("evaluationManager: -{flag} needs node type names"))));
        }
        for t in &objects {
            if crate::node::types::find(t).is_none() {
                return Some(Err(aztek_ops::CommandError::new(format!("evaluationManager: {t} is not a node type"))));
            }
        }
        let set = em.type_overrides.entry(flag.to_string()).or_default();
        for t in objects {
            if on(a.get(flag).unwrap()) {
                set.insert(t);
            } else {
                set.remove(&t);
            }
        }
        return Some(Ok(Value::Null));
    }
    if q {
        let em = &s.extras.eval_manager;
        let setting = |k: &str, def: Value| em.settings.get(k).cloned().unwrap_or(def);
        if asked(a, "disableInfo") {
            // Nothing has turned it off on its own.
            return Some(Ok(json!([])));
        }
        if asked(a, "empty") {
            let g = &s.graph;
            let any = g.nodes().iter().any(|&n| g.node(n).is_some_and(|x| !crate::defaults::is_default_node(x.name()) && !crate::shading::is_default_node(x.name())));
            return Some(Ok(json!(!any)));
        }
        if asked(a, "fallbackTriggered") {
            return Some(Ok(setting("fallbackTriggered", json!(false))));
        }
        if asked(a, "manipulationReady") {
            return Some(Ok(json!(true)));
        }
        for (k, def) in [("idleAction", json!(1)), ("manipulationPrevalidation", json!(true)), ("reduceGraphRebuild", json!(false)), ("safeMode", json!(true))] {
            if asked(a, k) {
                return Some(Ok(setting(k, def)));
            }
        }
        return None;
    }
    let mut set_any = false;
    for k in ["idleAction", "manipulationPrevalidation", "reduceGraphRebuild", "safeMode"] {
        if let Some(v) = a.get(k) {
            let v = if k == "idleAction" {
                let i = v.as_i64().unwrap_or(0);
                if !(0..=3).contains(&i) {
                    return Some(Err(aztek_ops::CommandError::new(format!("evaluationManager: -idleAction {i} is not 0, 1, 2 or 3"))));
                }
                json!(i)
            } else {
                json!(on(v))
            };
            s.extras.eval_manager.settings.insert(k.to_string(), v);
            set_any = true;
        }
    }
    if a.has("mode") {
        // A new mode starts without a fallback.
        s.extras.eval_manager.settings.insert("fallbackTriggered".into(), json!(false));
    }
    set_any.then(|| Ok(Value::Null))
}

pub(crate) fn evaluation_manager_flags(c: Cmd) -> Cmd {
    c.flag(f("cycleCluster", "ccl", S, "The nodes in a cycle with this one; none while the mode is off or it is in no cycle."))
        .flag(f("disableInfo", "di", B, "Query: why evaluation was turned off by a problem; empty while it runs as set."))
        .flag(f("downstreamFrom", "dst", S, "The nodes the one named feeds; with -q the whole walk down, level then node."))
        .flag(f("upstreamFrom", "ust", S, "The nodes feeding the one named; with -q the whole walk up, level then node."))
        .flag(f("empty", "mt", B, "Query: whether the graph has no nodes of the scene's own."))
        .flag(f("fallbackTriggered", "ft", B, "Query: whether errors made evaluation fall back to serial since the mode was set."))
        .flag(f("idleAction", "ia", I, "What runs on idle: 0 nothing, 1 graph rebuild, 2 manipulation preparation, 3 both."))
        .flag(f("manipulationPrevalidation", "mp", B, "Check for known manipulation patterns first."))
        .flag(f("manipulationReady", "mr", B, "Query: whether manipulation is ready."))
        .flag(f("nodeTypeGloballySerialize", "ntg", B, "Schedule the node types named globally serial; -q lists the types, or answers for those named."))
        .flag(f("nodeTypeParallel", "ntp", B, "Schedule the node types named in parallel; -q lists the types, or answers for those named."))
        .flag(f("nodeTypeSerialize", "nts", B, "Schedule the node types named locally serial; -q lists the types, or answers for those named."))
        .flag(f("nodeTypeUntrusted", "ntu", B, "Schedule the node types named apart from the rest; -q lists the types, or answers for those named."))
        .flag(f("reduceGraphRebuild", "rgr", B, "Rebuild the graph less when animation is connected."))
        .flag(f("safeMode", "sfm", B, "Fall back to serial when the graph is missing dependencies (on)."))
}

// ---------------------------------------------------------------------------------------------------------------------
// freeze

fn frozen_index(g: &Graph, n: NodeId) -> Option<usize> {
    g.node(n)?.ty().frozen_attr
}

fn affected_index(g: &Graph, n: NodeId) -> Option<usize> {
    g.node(n)?.ty().attr_index("frozenAffected")
}

fn bool_at(g: &Graph, n: NodeId, i: Option<usize>) -> bool {
    i.and_then(|i| g.node(n).map(|x| matches!(x.value(i), PlugValue::Bool(true)))).unwrap_or(false)
}

fn is_frozen(g: &Graph, n: NodeId) -> bool {
    bool_at(g, n, frozen_index(g, n))
}

fn is_affected(g: &Graph, n: NodeId) -> bool {
    bool_at(g, n, affected_index(g, n))
}

fn set_affected(g: &mut Graph, n: NodeId, on: bool) -> CmdResult<()> {
    let i = match affected_index(g, n) {
        Some(i) => i,
        None if !on => return Ok(()),
        None => g.add_attr(
            n,
            AttrDef {
                name: "frozenAffected".into(),
                short_name: "fza".into(),
                ty: PlugType::Bool,
                default_value: PlugValue::Bool(false),
                doc: "Frozen by a freeze that reached it from another node.".into(),
                ..Default::default()
            },
        )?,
    };
    g.set_value(PlugRef::new(n, i as i32), PlugValue::Bool(on))?;
    Ok(())
}

/// An attribute the frozen state doesn't pass through when something drives it.
fn stops_propagation(g: &Graph, p: &PlugRef) -> bool {
    p.attr >= 0 && g.node(p.node).is_some_and(|n| matches!(n.ty().attrs[p.attr as usize].name.as_str(), "frozen" | "visibility" | "displayType" | "enabled"))
}

fn visible(g: &Graph, n: NodeId) -> bool {
    g.node(n).and_then(|x| x.ty().attr_index("visibility").map(|i| !matches!(x.value(i), PlugValue::Bool(false)))).unwrap_or(true)
}

fn driven(g: &Graph, n: NodeId, attr: &str) -> bool {
    g.node(n).and_then(|x| x.ty().attr_index(attr)).is_some_and(|i| g.source(&PlugRef::new(n, i as i32)).is_some())
}

fn animated(g: &Graph, n: NodeId, attr: &str) -> bool {
    g.node(n).and_then(|x| x.ty().attr_index(attr)).and_then(|i| g.source(&PlugRef::new(n, i as i32))).is_some_and(|s| g.node(s.node).is_some_and(|x| x.ty().name.starts_with("animCurve")))
}

/// The nodes a freeze acts on, from the flags.
fn chosen(s: &Session, a: &Args) -> Vec<NodeId> {
    let g = &s.graph;
    let named: Vec<NodeId> = a.named().iter().filter_map(|n| g.find(n).ok().flatten()).collect();
    let base: Vec<NodeId> = if !named.is_empty() {
        named
    } else if a.bool("allNodes") || g.selection().is_empty() {
        if a.bool("allNodes") {
            g.nodes()
        } else {
            Vec::new()
        }
    } else {
        g.selection().to_vec()
    };
    let (inv, dl) = (a.bool("invisible"), a.bool("displayLayers"));
    let mut out: Vec<NodeId> = if inv || dl {
        let mut picked: Vec<NodeId> = Vec::new();
        if inv {
            // Under an invisible node whose visibility nothing drives (or that animation drives).
            for &n in &base {
                if g.node(n).is_some_and(|x| x.ty().dag) && !visible(g, n) && (!driven(g, n, "visibility") || animated(g, n, "visibility")) {
                    picked.push(n);
                    picked.extend(g.descendants(n));
                }
            }
        }
        if dl {
            // Members of an invisible display layer whose visibility nothing drives (or that animation drives).
            for &n in &base {
                if g.node(n).is_some_and(|x| x.ty().name == "displayLayer") && !visible(g, n) && (!driven(g, n, "visibility") || animated(g, n, "visibility")) {
                    for m in crate::display_layers::members(g, n) {
                        picked.push(m);
                        picked.extend(g.descendants(m));
                    }
                }
            }
        }
        picked
    } else {
        base
    };
    if a.bool("frozen") {
        out.retain(|&n| is_frozen(g, n));
    }
    let mut seen = HashSet::new();
    out.retain(|n| seen.insert(*n) && g.node(*n).is_some());
    out
}

/// Nodes the frozen state reaches from `set`: downstream ones whose inputs all come from frozen nodes (or any, with
/// force), upstream ones that only frozen nodes use.
fn spread(g: &Graph, set: &HashSet<NodeId>, down: bool, force: bool) -> Vec<NodeId> {
    let mut reached: HashSet<NodeId> = set.clone();
    let mut out = Vec::new();
    let conns = g.connections();
    let mut changed = true;
    while changed {
        changed = false;
        let candidates: BTreeSet<NodeId> = conns
            .iter()
            .filter_map(|(d, s)| {
                if down && reached.contains(&s.node) && !reached.contains(&d.node) && !stops_propagation(g, d) {
                    Some(d.node)
                } else if !down && reached.contains(&d.node) && !reached.contains(&s.node) && !stops_propagation(g, d) {
                    Some(s.node)
                } else {
                    None
                }
            })
            .collect();
        for c in candidates {
            let ok = if down {
                force || conns.iter().filter(|(d, _)| d.node == c).all(|(_, s)| reached.contains(&s.node))
            } else {
                // Kept unfrozen while anything unfrozen still uses it.
                conns.iter().filter(|(_, s)| s.node == c).all(|(d, _)| reached.contains(&d.node))
            };
            if ok {
                reached.insert(c);
                out.push(c);
                changed = true;
            }
        }
    }
    out
}

fn freeze(s: &mut Session, a: &Args) -> CmdResult {
    let set = chosen(s, a);
    let unfreeze = a.bool("unfreeze");
    let g = &mut s.graph;
    if a.bool("query") {
        // Three lists: the frozen state set (or with -uf unset), frozenAffected set (or unset), the rest.
        let (mut first, mut second, mut rest) = (Vec::new(), Vec::new(), Vec::new());
        for &n in &set {
            let (fz, af) = (is_frozen(g, n), is_affected(g, n));
            if fz != unfreeze && (!unfreeze || !af) {
                first.push(n);
            } else if af != unfreeze {
                second.push(n);
            } else {
                rest.push(n);
            }
        }
        let names = |v: &[NodeId]| v.iter().map(|&n| json!(g.path(n))).collect::<Vec<_>>();
        let mut out = names(&first);
        out.push(json!(""));
        out.extend(names(&second));
        out.push(json!(""));
        out.extend(names(&rest));
        return Ok(Value::Array(out));
    }
    let direct: HashSet<NodeId> = set.iter().copied().collect();
    let mut indirect: Vec<NodeId> = Vec::new();
    let (dn, fd, up) = (a.bool("downstream"), a.bool("forceDownstream"), a.bool("upstream"));
    if dn || fd {
        indirect.extend(spread(g, &direct, true, fd));
    }
    if up {
        indirect.extend(spread(g, &direct, false, false));
    }
    let mut seen: HashSet<NodeId> = direct.clone();
    indirect.retain(|n| seen.insert(*n));
    if !a.bool("noFreeze") {
        g.open_chunk(if unfreeze { "Unfreeze" } else { "Freeze" });
        let r = (|| -> CmdResult<()> {
            for &n in &set {
                if let Some(i) = frozen_index(g, n) {
                    g.set_value(PlugRef::new(n, i as i32), PlugValue::Bool(!unfreeze))?;
                }
                if unfreeze {
                    set_affected(g, n, false)?;
                }
            }
            for &n in &indirect {
                set_affected(g, n, !unfreeze)?;
            }
            Ok(())
        })();
        g.close_chunk();
        r?;
        g.evaluate();
    }
    let mut out: Vec<Value> = set.iter().map(|&n| json!(g.path(n))).collect();
    if !indirect.is_empty() {
        out.push(json!(""));
        out.extend(indirect.iter().map(|&n| json!(g.path(n))));
    }
    Ok(Value::Array(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// freezeOptions

const OPTIONS: &[(&str, &str, &str)] = &[
    ("displayLayers", "freezeOptionsDisplayLayers", "b"),
    ("downstream", "freezeOptionsDownstream", "s"),
    ("explicitPropagation", "freezeOptionsExplicitPropagation", "b"),
    ("invisible", "freezeOptionsInvisible", "b"),
    ("referencedNodes", "freezeOptionsReferencedNodes", "b"),
    ("runtimePropagation", "freezeOptionsRuntimePropagation", "b"),
    ("upstream", "freezeOptionsUpstream", "s"),
];

fn option_default(flag: &str) -> Value {
    match flag {
        "downstream" => json!("safe"),
        "upstream" => json!("none"),
        "displayLayers" | "invisible" | "referencedNodes" => json!(false),
        "explicitPropagation" => json!(false),
        _ => json!(true),
    }
}

fn freeze_options(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        for (flag, key, _) in OPTIONS {
            if asked(a, flag) {
                return Ok(s.option_vars.get(*key).cloned().unwrap_or_else(|| option_default(flag)));
            }
        }
        fail!("freezeOptions: name an option to query");
    }
    for (flag, key, kind) in OPTIONS {
        let Some(v) = a.get(flag) else { continue };
        let v = if *kind == "s" {
            let t = v.as_str().unwrap_or_default();
            if !["none", "safe", "force"].contains(&t) {
                fail!("freezeOptions: -{flag} takes none, safe or force, not {t}");
            }
            json!(t)
        } else {
            json!(on(v))
        };
        s.option_vars.insert(key.to_string(), v);
    }
    Ok(json!(true))
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        Cmd::new(
            "freeze",
            "Freeze nodes: a frozen node ignores changes to its inputs and keeps what it had. The named or selected nodes (-all every \
             node) are frozen directly; -dn, -fd and -up spread it indirectly (frozenAffected). -i takes the nodes under invisible ones, \
             -dl the members of invisible display layers, -f those already frozen. -uf unfreezes, -n only reports. Returns the nodes set, \
             then after an empty string those reached. -q lists frozen, then affected, then the rest.",
            freeze,
        )
        .positional("nodes (the selection when none)")
        .flag(f("allNodes", "all", B, "Every node in the scene, when none are named."))
        .flag(f("displayLayers", "dl", B, "The members of invisible display layers among the nodes."))
        .flag(f("downstream", "dn", B, "Spread the state downstream, to nodes whose inputs are all frozen."))
        .flag(f("forceDownstream", "fd", B, "Spread the state downstream, whatever else feeds those nodes."))
        .flag(f("frozen", "f", B, "Only nodes already frozen: to spread their state."))
        .flag(f("invisible", "i", B, "The nodes under invisible nodes among the nodes."))
        .flag(f("noFreeze", "n", B, "Report what would be frozen, changing nothing."))
        .flag(f("unfreeze", "uf", B, "Unfreeze rather than freeze; with -q, list the unfrozen."))
        .flag(f("upstream", "up", B, "Spread the state upstream, to nodes only frozen nodes use."))
        .flag(f("query", "q", B, "Query mode: list nodes by their frozen state."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );
    c.register(
        Cmd::new(
            "freezeOptions",
            "How the frozen state spreads and what counts as frozen, kept as preferences: -dn and -up none, safe or force, -inv, -dl, \
             -rn, -ep, -rt. -q reads one.",
            freeze_options,
        )
        .undoable(false)
        .flag(f("displayLayers", "dl", B, "Nodes in an invisible display layer count as frozen."))
        .flag(f("downstream", "dn", S, "Downstream: none, safe (all upstream frozen) or force (any upstream frozen)."))
        .flag(f("explicitPropagation", "ep", B, "Spread the state when the graph is built."))
        .flag(f("invisible", "inv", B, "Invisible nodes count as frozen."))
        .flag(f("referencedNodes", "rn", B, "Nodes of a frozen reference count as frozen."))
        .flag(f("runtimePropagation", "rt", B, "Spread the state as the graph evaluates."))
        .flag(f("upstream", "up", S, "Upstream: none, safe (all downstream frozen) or force (any downstream frozen)."))
        .flag(f("query", "q", B, "Query mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );
}
