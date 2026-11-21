//! The reference's debugging commands of this module: dbmessage (message monitors), dbtrace (trace objects), dgfilter
//! (filters for traces and for testing objects), dbpeek (looks into the scene's data), dgmodified (nodes changed since
//! the scene was saved), dgValidateCurve (animation curves' static tags), displayString (a string table) and
//! disableIncorrectNameWarning.
//!
//! The graph calls `hook` on what it does (computes, dirty propagation, nodes made, removed and renamed, connections
//! made and broken, values set) while a monitor or a trace is on. Monitors and traces belong to the graph whose
//! session turned them on, by its serial, so one session does not hear another's.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Value};

use crate::plug::{NodeId, PlugRef};
use crate::ref_core::{f, glob};
use crate::session::Session;
use crate::sys_misc::print;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Int as I, String as S};

// ---------------------------------------------------------------------------------------------------------------------
// What the graph tells

/// What the graph did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev {
    Compute,
    Dirty,
    NodeAdded,
    NodeRemoved,
    Renamed,
    Connected,
    Disconnected,
    Set,
}

/// The message types dbmessage monitors, with the events each is sent on.
const MESSAGES: &[(&str, Ev, &str)] = &[
    ("dgNodeAdded", Ev::NodeAdded, "A node was added to the graph."),
    ("dgNodeRemoved", Ev::NodeRemoved, "A node was removed from the graph."),
    ("dgNameChanged", Ev::Renamed, "A node was renamed."),
    ("dgConnectionMade", Ev::Connected, "A connection was made."),
    ("dgConnectionBroken", Ev::Disconnected, "A connection was broken."),
    ("dgAttributeChanged", Ev::Set, "A value was set on a plug."),
    ("dgNodeDirty", Ev::Dirty, "A node was marked dirty."),
    ("dgNodeComputed", Ev::Compute, "A node computed its outputs."),
];

/// The trace objects dbtrace works: (name, keyword, what it traces, the events it hears).
const TRACES: &[(&str, &str, &str, &[Ev])] = &[
    ("computeTrace", "compute", "Each node's compute, as it runs.", &[Ev::Compute]),
    ("dirtyTrace", "dirty", "Dirty propagation: each node marked dirty.", &[Ev::Dirty]),
    ("connectTrace", "connect", "Connections made and broken.", &[Ev::Connected, Ev::Disconnected]),
    ("nodeTrace", "node", "Nodes created, deleted and renamed.", &[Ev::NodeAdded, Ev::NodeRemoved, Ev::Renamed]),
    ("setAttrTrace", "setAttr", "Values set on plugs.", &[Ev::Set]),
];

/// A filter of graph objects, as dgfilter makes them.
#[derive(Clone, Debug)]
enum Filter {
    Attribute(String),
    Plug(String),
    Node(String),
    NodeType(String),
    And(String, String),
    Or(String, String),
    Not(String),
    /// Several criteria given at once: all must pass.
    All(Vec<Filter>),
    /// No criteria: passes everything, as an invalid filter does.
    None,
}

/// One trace object's state.
#[derive(Clone, Debug, Default)]
struct Trace {
    on: bool,
    output: String,
    filter: String,
    timed: bool,
    since: Option<std::time::Instant>,
}

/// What one graph's monitors and traces hold.
#[derive(Default)]
struct PerGraph {
    monitored: HashSet<&'static str>,
    message_output: String,
    traces: BTreeMap<&'static str, Trace>,
    filters: BTreeMap<String, Filter>,
    marks: u64,
}

#[derive(Default)]
struct Global {
    graphs: HashMap<u64, PerGraph>,
}

static ACTIVE: AtomicBool = AtomicBool::new(false);
static GLOBAL: Mutex<Option<Global>> = Mutex::new(None);

fn with<R>(serial: u64, f: impl FnOnce(&mut PerGraph) -> R) -> R {
    let mut g = GLOBAL.lock().unwrap_or_else(|e| e.into_inner());
    let g = g.get_or_insert_with(Global::default);
    let r = f(g.graphs.entry(serial).or_default());
    let any = g.graphs.values().any(|p| !p.monitored.is_empty() || p.traces.values().any(|t| t.on));
    ACTIVE.store(any, Ordering::Relaxed);
    r
}

/// Whether any monitor or trace is on: the graph calls `hook` only then.
#[inline]
pub fn active() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

/// What the graph tells, while `active`: an event on a node (its name and type), and on one of its attributes (long and
/// short names) where there is one; `other` is the far end of a connection.
pub fn hook(serial: u64, ev: Ev, node: &str, ty: &str, attr: Option<(&str, &str)>, other: Option<&str>) {
    let mut lines: Vec<(String, String)> = Vec::new();
    {
        let mut g = GLOBAL.lock().unwrap_or_else(|e| e.into_inner());
        let Some(p) = g.as_mut().and_then(|g| g.graphs.get_mut(&serial)) else { return };
        let what = match attr {
            Some((long, _)) => format!("{node}.{long}"),
            None => node.to_string(),
        };
        let what = match other {
            Some(o) if matches!(ev, Ev::Connected | Ev::Disconnected) => format!("{o} -> {what}"),
            Some(o) if ev == Ev::Renamed => format!("{o} -> {what}"),
            _ => what,
        };
        for (name, e, _) in MESSAGES {
            if *e == ev && p.monitored.contains(name) {
                lines.push((p.message_output.clone(), format!("{name}: {what} ({ty})")));
            }
        }
        let filters = p.filters.clone();
        for (name, kw, _, evs) in TRACES {
            let Some(t) = p.traces.get(name) else { continue };
            if !t.on || !evs.contains(&ev) {
                continue;
            }
            if !t.filter.is_empty() && !passes(&filters, &t.filter, node, ty, attr, 0) {
                continue;
            }
            let stamp = if t.timed { format!("[{:.6}] ", t.since.map_or(0.0, |s| s.elapsed().as_secs_f64())) } else { String::new() };
            lines.push((t.output.clone(), format!("{stamp}{kw}: {what} ({ty})")));
        }
    }
    for (dest, line) in lines {
        emit(&dest, &line);
    }
}

fn to_window(dest: &str) -> bool {
    dest.is_empty() || matches!(dest, "stdout" | "stderr" | "msdev")
}

/// A line to an output: the Script Editor, or appended to a file.
fn emit(dest: &str, line: &str) {
    if to_window(dest) {
        print(line);
    } else if let Ok(mut fh) = std::fs::OpenOptions::new().create(true).append(true).open(dest) {
        let _ = writeln!(fh, "{line}");
    }
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true" | "yes" | "1")))
}

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Filters

fn attr_matches(pat: &str, attr: Option<(&str, &str)>) -> bool {
    attr.is_some_and(|(long, short)| glob(pat, long) || (!short.is_empty() && glob(pat, short)))
}

fn passes(all: &BTreeMap<String, Filter>, name: &str, node: &str, ty: &str, attr: Option<(&str, &str)>, depth: u32) -> bool {
    match all.get(name) {
        // An invalid filter passes every object.
        None => true,
        Some(fl) => test(all, fl, node, ty, attr, depth),
    }
}

fn test(all: &BTreeMap<String, Filter>, fl: &Filter, node: &str, ty: &str, attr: Option<(&str, &str)>, depth: u32) -> bool {
    if depth > 32 {
        return true;
    }
    let leaf = node.rsplit('|').next().unwrap_or(node);
    match fl {
        Filter::Attribute(p) => attr_matches(p, attr),
        Filter::Plug(p) => {
            let (np, ap) = p.split_once('.').unwrap_or((p.as_str(), "*"));
            (glob(np, leaf) || glob(np, node)) && attr_matches(ap, attr)
        }
        Filter::Node(p) => glob(p, leaf) || glob(p, node),
        Filter::NodeType(p) => {
            // A type pattern matches the node's type or one it derives from.
            if glob(p, ty) {
                return true;
            }
            let mut t = crate::node::types::find(ty);
            while let Some(x) = t {
                if glob(p, &x.name) {
                    return true;
                }
                t = x.base.clone();
            }
            false
        }
        Filter::And(a, b) => passes(all, a, node, ty, attr, depth + 1) && passes(all, b, node, ty, attr, depth + 1),
        Filter::Or(a, b) => passes(all, a, node, ty, attr, depth + 1) || passes(all, b, node, ty, attr, depth + 1),
        Filter::Not(a) => !passes(all, a, node, ty, attr, depth + 1),
        Filter::All(v) => v.iter().all(|x| test(all, x, node, ty, attr, depth + 1)),
        Filter::None => true,
    }
}

fn describe(all: &BTreeMap<String, Filter>, fl: &Filter, depth: u32) -> String {
    let named = |n: &str| match all.get(n) {
        Some(x) if depth < 32 => describe(all, x, depth + 1),
        _ => format!("Invalid({n})"),
    };
    match fl {
        Filter::Attribute(p) => format!("Attribute({p})"),
        Filter::Plug(p) => format!("Plug({p})"),
        Filter::Node(p) => format!("Node({p})"),
        Filter::NodeType(p) => format!("NodeType({p})"),
        // The second filter first, as the reference describes them.
        Filter::And(a, b) => format!("logicalAnd( {}, {} )", named(b), named(a)),
        Filter::Or(a, b) => format!("logicalOr( {}, {} )", named(b), named(a)),
        Filter::Not(a) => format!("logicalNot( {} )", named(a)),
        Filter::All(v) => format!("logicalAnd( {} )", v.iter().map(|x| describe(all, x, depth + 1)).collect::<Vec<_>>().join(", ")),
        Filter::None => "None".into(),
    }
}

/// An attribute named in a filter, by its short name where a node type knows it by that long name.
fn attr_short(name: &str) -> String {
    if name.contains(['*', '?']) {
        return name.to_string();
    }
    for t in ["transform", "mesh", "dagNode"].iter().filter_map(|t| crate::node::types::find(t)).chain(crate::node::types::all()) {
        for a in &t.attrs {
            if a.name == name && !a.short_name.is_empty() {
                return a.short_name.clone();
            }
            for c in 0..3 {
                if a.child_names[c] == name && !a.child_short[c].is_empty() {
                    return a.child_short[c].clone();
                }
            }
        }
    }
    name.to_string()
}

/// A node and attribute names an object stands for: `pCube1.tx` or `pCube1`.
fn object_parts(s: &mut Session, text: &str) -> (String, String, Option<(String, String)>) {
    let g = &s.graph;
    let (node_text, attr_text) = match text.split_once('.') {
        Some((n, a)) => (n.to_string(), Some(a.to_string())),
        None => (text.to_string(), None),
    };
    let id = g.find(&node_text).ok().flatten();
    let node = id.map_or(node_text.clone(), |i| g.node(i).map(|n| n.name().to_string()).unwrap_or(node_text.clone()));
    let ty = id.and_then(|i| g.node(i)).map(|n| n.ty().name.clone()).unwrap_or_default();
    let attr = attr_text.map(|a| match g.plug(text) {
        Ok(p) => plug_names(g, &p).unwrap_or((a.clone(), String::new())),
        Err(_) => (a.clone(), String::new()),
    });
    (node, ty, attr)
}

/// A plug's attribute names, long and short, the child's for a component.
pub(crate) fn plug_names(g: &crate::graph::Graph, p: &PlugRef) -> Option<(String, String)> {
    let n = g.node(p.node)?;
    let a = n.ty().attrs.get(usize::try_from(p.attr).ok()?)?;
    if (0..3).contains(&p.component) {
        let c = p.component as usize;
        return Some((a.child_names[c].clone(), a.child_short[c].clone()));
    }
    Some((a.name.clone(), a.short_name.clone()))
}

fn dgfilter(s: &mut Session, a: &Args) -> CmdResult {
    let serial = s.graph.serial();
    let name = a.get("name").and_then(Value::as_str).map(str::to_string);
    if a.bool("list") {
        return with(serial, |p| match &name {
            Some(n) => match p.filters.get(n) {
                Some(fl) => Ok(json!(describe(&p.filters, fl, 0))),
                None => fail!("dgfilter: no filter named {n}"),
            },
            None => Ok(json!(p.filters.keys().cloned().collect::<Vec<_>>())),
        });
    }
    let mut crit = Vec::new();
    if let Some(v) = a.get("attribute").and_then(Value::as_str) {
        crit.push(Filter::Attribute(attr_short(v)));
    }
    if let Some(v) = a.get("plug").and_then(Value::as_str) {
        crit.push(Filter::Plug(v.to_string()));
    }
    if let Some(v) = a.get("node").and_then(Value::as_str) {
        crit.push(Filter::Node(v.to_string()));
    }
    if let Some(v) = a.get("nodeType").and_then(Value::as_str) {
        crit.push(Filter::NodeType(v.to_string()));
    }
    for (flag, both) in [("logicalAnd", true), ("logicalOr", false)] {
        let v = strings(a.get(flag));
        if v.is_empty() {
            continue;
        }
        if v.len() != 2 {
            fail!("dgfilter: -{flag} takes two filter names");
        }
        crit.push(if both { Filter::And(v[0].clone(), v[1].clone()) } else { Filter::Or(v[0].clone(), v[1].clone()) });
    }
    if let Some(v) = a.get("logicalNot").and_then(Value::as_str) {
        crit.push(Filter::Not(v.to_string()));
    }
    let objs = a.named();
    let name = with(serial, |p| {
        let name = name.clone().unwrap_or_else(|| (1..).map(|i| format!("filter{i}")).find(|n| !p.filters.contains_key(n)).unwrap());
        if !crit.is_empty() {
            let fl = if crit.len() == 1 { crit.pop().unwrap() } else { Filter::All(crit) };
            p.filters.insert(name.clone(), fl);
        } else if !p.filters.contains_key(&name) && objs.is_empty() {
            p.filters.insert(name.clone(), Filter::None);
        }
        name
    });
    if objs.is_empty() {
        return Ok(json!(name));
    }
    let mut all = true;
    for o in objs {
        let (node, ty, attr) = object_parts(s, &o);
        let attr_ref = attr.as_ref().map(|(l, sh)| (l.as_str(), sh.as_str()));
        all &= with(serial, |p| passes(&p.filters, &name, &node, &ty, attr_ref, 0));
    }
    Ok(json!(all))
}

// ---------------------------------------------------------------------------------------------------------------------
// dbmessage

fn dbmessage(s: &mut Session, a: &Args) -> CmdResult {
    let serial = s.graph.serial();
    let kw = a.string("type", "");
    let picked = |n: &str| kw.is_empty() || n.contains(kw.as_str());
    if let Some(file) = a.get("file").and_then(Value::as_str) {
        let file = file.to_string();
        with(serial, |p| p.message_output = file);
    }
    if let Some(v) = a.get("monitor") {
        let state = on(v);
        let names: Vec<&'static str> = MESSAGES.iter().map(|m| m.0).filter(|n| picked(n)).collect();
        if names.is_empty() {
            fail!("dbmessage: no message type matches '{kw}'");
        }
        let now = with(serial, |p| {
            for n in names {
                if state {
                    p.monitored.insert(n);
                } else {
                    p.monitored.remove(n);
                }
            }
            MESSAGES.iter().map(|m| m.0).filter(|n| p.monitored.contains(n)).collect::<Vec<_>>()
        });
        return Ok(json!(now));
    }
    if a.bool("list") {
        let lines: Vec<String> = with(serial, |p| MESSAGES.iter().filter(|m| picked(m.0)).map(|(n, _, doc)| format!("{n} ({}): {doc}", if p.monitored.contains(n) { "on" } else { "off" })).collect());
        for l in lines {
            print(l);
        }
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// dbtrace

fn dbtrace(s: &mut Session, a: &Args) -> CmdResult {
    let serial = s.graph.serial();
    let query = a.bool("query");
    let mut keys = strings(a.get("keyword"));
    if query && keys.is_empty() && a.has("keyword") {
        keys = a.named();
    }
    if query && !a.has("keyword") && crate::ref_core::asked(a, "keyword") {
        keys = a.named();
    }
    let verbose = a.bool("verbose");
    let by_key = |k: &str| TRACES.iter().filter(|t| t.1 == k || t.0 == k).map(|t| t.0).collect::<Vec<_>>();
    if query {
        return with(serial, |p| {
            let state = |n: &str| p.traces.get(n).is_some_and(|t| t.on);
            let chosen: Vec<&'static str> = if keys.is_empty() { TRACES.iter().map(|t| t.0).collect() } else { keys.iter().flat_map(|k| by_key(k)).collect() };
            let off = crate::ref_core::asked(a, "off");
            let get = |n: &str| p.traces.get(n).cloned().unwrap_or_default();
            if crate::ref_core::asked(a, "info") {
                return Ok(json!(chosen.iter().filter_map(|n| TRACES.iter().find(|t| t.0 == *n)).map(|t| format!("{} ({}): {}", t.0, t.1, t.2)).collect::<Vec<_>>()));
            }
            // Output, filter and timing: of the trace objects on, or all with -verbose.
            let listed: Vec<&'static str> = chosen.iter().copied().filter(|n| verbose || state(n)).collect();
            if crate::ref_core::asked(a, "output") {
                return Ok(json!(listed.iter().map(|n| { let o = get(n).output; if o.is_empty() { "stdout".to_string() } else { o } }).collect::<Vec<_>>()));
            }
            if crate::ref_core::asked(a, "filter") {
                return Ok(json!(listed.iter().map(|n| get(n).filter).collect::<Vec<_>>()));
            }
            if crate::ref_core::asked(a, "timed") {
                return Ok(json!(listed.iter().map(|n| get(n).timed).collect::<Vec<_>>()));
            }
            // Which keywords are on, or off with -off.
            let mut out: Vec<String> = Vec::new();
            for n in chosen {
                if state(n) != off || verbose {
                    let kw = TRACES.iter().find(|t| t.0 == n).unwrap().1.to_string();
                    if !out.contains(&kw) {
                        out.push(kw);
                    }
                }
            }
            Ok(json!(out))
        });
    }
    let chosen: Vec<&'static str> = if keys.is_empty() {
        Vec::new()
    } else {
        let v: Vec<&'static str> = keys.iter().flat_map(|k| by_key(k)).collect();
        if v.is_empty() {
            fail!("dbtrace: no trace objects with keyword {}", keys.join(", "));
        }
        v
    };
    let filter_name = a.get("filter").and_then(Value::as_str).map(str::to_string);
    let changes = a.bool("off") || a.has("output") || a.has("filter") || a.bool("mark") || a.has("title") || a.has("timed");
    let mut lines: Vec<(String, String)> = Vec::new();
    with(serial, |p| {
        // Without a keyword, the trace objects now on (or all of them to turn on).
        let targets: Vec<&'static str> = if !chosen.is_empty() {
            chosen.clone()
        } else if changes {
            TRACES.iter().map(|t| t.0).filter(|n| p.traces.get(n).is_some_and(|t| t.on)).collect()
        } else {
            TRACES.iter().map(|t| t.0).collect()
        };
        if let Some(fl) = &filter_name {
            // A name that is no filter yet: a filter on that node type, as in the reference's example.
            if !fl.is_empty() && !p.filters.contains_key(fl) {
                p.filters.insert(fl.clone(), Filter::NodeType(fl.clone()));
            }
        }
        if a.bool("mark") {
            p.marks += 1;
        }
        let marks = p.marks;
        for n in targets {
            let t = p.traces.entry(n).or_default();
            if a.bool("off") {
                t.on = false;
            } else if !changes || (filter_name.is_some() && !chosen.is_empty()) {
                t.on = true;
                t.since.get_or_insert_with(std::time::Instant::now);
            }
            if let Some(o) = a.get("output").and_then(Value::as_str) {
                t.output = o.to_string();
            }
            if let Some(fl) = &filter_name {
                t.filter = fl.clone();
            }
            if let Some(v) = a.get("timed") {
                t.timed = on(v);
                t.since = Some(std::time::Instant::now());
            }
            if a.bool("mark") {
                lines.push((t.output.clone(), format!("// ---- {n}: mark {marks} ----")));
            }
            if let Some(title) = a.get("title").and_then(Value::as_str) {
                lines.push((t.output.clone(), format!("// ==== {n}: {title} ====")));
            }
        }
    });
    for (d, l) in lines {
        emit(&d, &l);
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// dbpeek

/// The peek operations: (name, what it does, its arguments with what each does, what -count means).
const PEEKS: &[(&str, &str, &[(&str, &str)], &str)] = &[
    (
        "nodes",
        "Counts the nodes in the scene by type.",
        &[("attributes", "Also count each type's attributes: static, extension and dynamic."), ("visible", "Leave out the hidden and internal nodes (all are shown by default).")],
        "Not used.",
    ),
    ("plugIterator", "Performance test: iterates over every networked plug in the scene, timing each pass. A count of at least 1000000 is suggested.", &[], "The number of plugs to visit, over as many passes as that takes."),
    ("metadata", "Shows the metadata on the nodes.", &[("summary", "One line per stream: Node <name> : <channel>( <stream>[<count>] ).")], "The most values shown for each stream; 0 for all."),
    ("references", "Lists the file references: node, namespace, file, whether loaded, and how many edits each holds.", &[], "Not used."),
];

fn peek_help(op: &str) -> Option<String> {
    let (name, doc, args, count) = PEEKS.iter().find(|p| p.0 == op)?;
    let mut t = format!("Peek operation '{name}': {doc}\n");
    if args.is_empty() {
        t.push_str("    It takes no arguments.\n");
    } else {
        t.push_str("    Arguments:\n");
        for (a, d) in args.iter() {
            t.push_str(&format!("        {a} : {d}\n"));
        }
    }
    t.push_str(&format!("    -count: {count}\n"));
    Some(t)
}

/// The nodes animation reaches: animation curves and expressions and everything downstream of them.
fn evaluation_graph(g: &crate::graph::Graph) -> HashSet<NodeId> {
    let mut seen = HashSet::new();
    let mut stack: Vec<NodeId> = g.nodes().into_iter().filter(|&n| g.node(n).is_some_and(|x| x.ty().name.starts_with("animCurve") || x.ty().name == "expression")).collect();
    let conns = g.connections();
    let mut down: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for (dp, sp) in conns {
        down.entry(sp.node).or_default().push(dp.node);
    }
    while let Some(n) = stack.pop() {
        if !seen.insert(n) {
            continue;
        }
        stack.extend(down.get(&n).into_iter().flatten().copied());
        // A transform moves what is below it.
        if g.node(n).is_some_and(|x| x.ty().dag) {
            stack.extend(g.children(n).iter().copied());
        }
    }
    seen
}

fn dbpeek(s: &mut Session, a: &Args) -> CmdResult {
    let mut op = a.get("operation").and_then(Value::as_str).map(str::to_string);
    let out_file = a.get("outputFile").and_then(Value::as_str).map(str::to_string);
    if a.bool("query") {
        if op.is_none() {
            op = a.named().into_iter().next();
        }
        let Some(op) = op else { return Ok(json!(PEEKS.iter().map(|p| p.0).collect::<Vec<_>>())) };
        let Some(help) = peek_help(&op) else { return Ok(json!(PEEKS.iter().map(|p| p.0).collect::<Vec<_>>())) };
        if crate::ref_core::asked(a, "argument") || a.has("argument") {
            let (_, _, args, _) = PEEKS.iter().find(|p| p.0 == op).unwrap();
            return Ok(json!(args.iter().map(|(n, d)| format!("{n} : {d}")).collect::<Vec<_>>()));
        }
        return match out_file {
            Some(f) => Ok(json!(write_out(&f, &help))),
            None => Ok(json!(help)),
        };
    }
    let op = op.unwrap_or_else(|| "nodes".into());
    if !PEEKS.iter().any(|p| p.0 == op) {
        fail!("dbpeek: no peek operation '{op}'; dbpeek -q -op lists them");
    }
    let args = strings(a.get("argument"));
    let arg_on = |n: &str| args.iter().any(|x| x == n || x.split_once('=').is_some_and(|(k, v)| k == n && on(&json!(v))));
    let count = a.get("count").and_then(Value::as_u64);
    // The objects: those named or selected, or all.
    let named = a.named();
    let g = &s.graph;
    let mut ids: Vec<NodeId> = if a.bool("allObjects") {
        g.nodes()
    } else if !named.is_empty() {
        let mut v = Vec::new();
        for n in &named {
            match g.find(n)? {
                Some(id) => v.push(id),
                None => fail!("dbpeek: no object named {n}"),
            }
        }
        v
    } else if !g.selection().is_empty() {
        g.selection().to_vec()
    } else {
        g.nodes()
    };
    if a.bool("evaluationGraph") {
        let eg = evaluation_graph(g);
        ids.retain(|n| eg.contains(n));
    }
    let text = match op.as_str() {
        "nodes" => {
            let visible = arg_on("visible");
            let attrs = arg_on("attributes");
            let mut by: BTreeMap<String, (u64, u64, u64, u64)> = BTreeMap::new();
            for &id in &ids {
                let Some(n) = g.node(id) else { continue };
                if visible && (crate::defaults::is_default_node(n.name()) || crate::shading::is_default_node(n.name()) || crate::defaults::is_scene_node(n.name())) {
                    continue;
                }
                let e = by.entry(n.ty().name.clone()).or_default();
                e.0 += 1;
                if attrs {
                    let base = n.ty().user_of.as_ref().map_or(n.ty().attrs.len(), |t| t.attrs.len());
                    for (i, d) in n.ty().attrs.iter().enumerate() {
                        if d.flags & crate::plug::flags::EXTENSION != 0 {
                            e.2 += 1;
                        } else if i >= base || d.flags & crate::plug::flags::USER != 0 {
                            e.3 += 1;
                        } else {
                            e.1 += 1;
                        }
                    }
                }
            }
            let mut nodes = serde_json::Map::new();
            for (t, (c, st, ex, dy)) in by {
                nodes.insert(t, if attrs { json!({"count": c, "staticAttributes": st, "extensionAttributes": ex, "dynamicAttributes": dy}) } else { json!(c) });
            }
            serde_json::to_string_pretty(&json!({"nodes": nodes})).unwrap_or_default()
        }
        "plugIterator" => {
            let plugs: Vec<PlugRef> = g.connections().into_iter().flat_map(|(d, s)| [s, d]).collect::<HashSet<_>>().into_iter().collect();
            let want = count.unwrap_or(1).max(1);
            let per = plugs.len().max(1) as u64;
            let loops = want.div_ceil(per).max(1);
            let (mut total, mut most, mut least) = (0.0f64, 0.0f64, f64::MAX);
            let mut seen = 0u64;
            for _ in 0..loops {
                let t0 = std::time::Instant::now();
                for p in &plugs {
                    seen = seen.wrapping_add(g.node(p.node).map_or(0, |n| n.ty().attrs.len() as u64) + p.attr as u64);
                }
                let dt = t0.elapsed().as_secs_f64();
                total += dt;
                most = most.max(dt);
                least = least.min(dt);
            }
            std::hint::black_box(seen);
            format!(
                "Plugs = {}\nLoops = {loops}\nTotal time = {total:.6} s\nMaximum = {most:.9} s\nMinimum = {least:.9} s\nAverage = {:.9} s",
                plugs.len(),
                total / loops as f64
            )
        }
        "metadata" => {
            let summary = arg_on("summary");
            let limit = count.unwrap_or(0) as usize;
            let mut lines = Vec::new();
            for &id in &ids {
                let meta = crate::general_cmds::meta_value(&mut s.graph, id);
                let Some(o) = meta.as_object() else { continue };
                let name = s.graph.node(id).map(|n| n.name().to_string()).unwrap_or_default();
                for (stream, st) in o {
                    let channel = st.get("channel").and_then(Value::as_str).unwrap_or("");
                    let values = st.get("values").and_then(Value::as_object).cloned().unwrap_or_default();
                    if summary {
                        lines.push(format!("Node {name} : {channel}( {stream}[{}] )", values.len()));
                    } else {
                        lines.push(format!("Node {name} : {channel}( {stream} ) structure {}", st.get("structure").and_then(Value::as_str).unwrap_or("")));
                        for (k, (idx, v)) in values.iter().enumerate() {
                            if limit > 0 && k >= limit {
                                lines.push(format!("    ... {} more", values.len() - limit));
                                break;
                            }
                            lines.push(format!("    [{idx}] {v}"));
                        }
                    }
                }
            }
            lines.join("\n")
        }
        _ => {
            let mut lines = Vec::new();
            let t = &s.extras.references;
            for e in &t.entries {
                lines.push(format!("{} : namespace \"{}\" file \"{}\" {} with {} edits", e.rfn, e.namespace, e.path, if e.loaded { "loaded" } else { "unloaded" }, e.edits.len() + e.connections.len()));
            }
            lines.join("\n")
        }
    };
    match out_file {
        Some(f) => Ok(json!(write_out(&f, &text))),
        None => Ok(json!(text)),
    }
}

/// Writes to an output file or the Script Editor; the number of errors met.
fn write_out(dest: &str, text: &str) -> i64 {
    if to_window(dest) {
        for l in text.lines() {
            print(l);
        }
        return 0;
    }
    match std::fs::write(dest, format!("{text}\n")) {
        Ok(_) => 0,
        Err(e) => {
            log::warn!("dbpeek: can't write {dest}: {e}");
            1
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// dgmodified

fn dgmodified(s: &mut Session, _a: &Args) -> CmdResult {
    let g = &s.graph;
    let mut out: Vec<String> = g.modified_nodes().into_iter().filter(|&n| g.node(n).is_some()).map(|n| g.path(n)).collect();
    out.sort();
    out.dedup();
    Ok(json!(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// dgValidateCurve

/// What this module keeps for the session.
#[derive(Default)]
pub struct DbState {
    /// Animation curves' static tags, as last validated: a curve with no tag is taken as animated.
    pub static_tags: HashMap<NodeId, bool>,
    /// displayString's strings.
    pub strings: BTreeMap<String, String>,
}

fn is_static(k: &crate::anim::Keys) -> bool {
    if k.keys.len() < 2 {
        return true;
    }
    let v0 = k.keys[0].v;
    (0..k.keys.len()).all(|i| {
        let (a, b) = k.slopes(i);
        (k.keys[i].v - v0).abs() < 1e-9 && a.abs() < 1e-9 && b.abs() < 1e-9
    })
}

fn dg_validate_curve(s: &mut Session, a: &Args) -> CmdResult {
    let g = &s.graph;
    let curves: Vec<NodeId> = if a.bool("allCurves") {
        g.nodes().into_iter().filter(|&n| crate::anim::is_curve(g, n)).collect()
    } else {
        let mut v = Vec::new();
        for name in objects(s, a) {
            let Some(id) = g.find(&name)? else { fail!("dgValidateCurve: no object named {name}") };
            if crate::anim::is_curve(g, id) {
                v.push(id);
            } else {
                v.extend(crate::anim::curves_on(g, id).into_iter().map(|x| x.1));
            }
        }
        v
    };
    let mut changed = 0;
    for c in curves {
        let st = is_static(&crate::anim::keys_of(&s.graph, c));
        let was = s.extras.db.static_tags.get(&c).copied().unwrap_or(false);
        if st != was {
            changed += 1;
        }
        s.extras.db.static_tags.insert(c, st);
        if a.bool("verbose") {
            print(format!("{}: {}", s.graph.path(c), if st { "static" } else { "not static" }));
        }
    }
    Ok(json!(changed))
}

// ---------------------------------------------------------------------------------------------------------------------
// displayString

fn display_string(s: &mut Session, a: &Args) -> CmdResult {
    let words = a.named();
    let Some(key) = words.first().cloned() else {
        if a.bool("keys") || crate::ref_core::asked(a, "keys") {
            return Ok(json!(s.extras.db.strings.keys().cloned().collect::<Vec<_>>()));
        }
        fail!("displayString: name a string identifier");
    };
    let table = &mut s.extras.db.strings;
    if a.bool("keys") || crate::ref_core::asked(a, "keys") {
        return Ok(json!(table.keys().filter(|k| k.contains(key.as_str())).cloned().collect::<Vec<_>>()));
    }
    if a.bool("exists") {
        return Ok(json!(table.contains_key(&key)));
    }
    if a.bool("delete") {
        if table.remove(&key).is_none() {
            fail!("displayString: no identifier {key}");
        }
        return Ok(Value::Null);
    }
    if a.bool("query") {
        let Some(v) = table.get(&key) else { fail!("displayString: no identifier {key}") };
        // ^1s to ^3s take the arguments after the identifier, which need not be identifiers themselves.
        let mut out = v.clone();
        for (i, w) in words.iter().skip(1).take(3).enumerate() {
            let with = table.get(w).cloned().unwrap_or_else(|| w.clone());
            out = out.replace(&format!("^{}s", i + 1), &with);
        }
        return Ok(json!(out));
    }
    if table.contains_key(&key) && !a.bool("replace") {
        fail!("displayString: identifier {key} already exists; use -replace to change it");
    }
    let value = a.get("value").and_then(Value::as_str).map_or(key.clone(), str::to_string);
    table.insert(key, value);
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// disableIncorrectNameWarning

static NAME_WARNING_OFF: AtomicBool = AtomicBool::new(false);

/// Whether reading a file still warns about node names the reference would not take.
pub fn name_warning_on() -> bool {
    !NAME_WARNING_OFF.load(Ordering::Relaxed)
}

pub fn register(c: &mut Commands<Session>) {
    use crate::system_cmds::q_flags;
    c.register(
        Cmd::new(
            "dbmessage",
            "Monitor the graph's messages as they are sent: -m on|off for the types matching -t (all by default), returning \
             those monitored; -l lists the types and their state; -f where the lines go (stdout, stderr, msdev or a file).",
            dbmessage,
        )
        .undoable(false)
        .flag(f("file", "f", S, "Where the monitor lines go: stdout (the default), stderr or msdev for the Script Editor, or a file."))
        .flag(f("list", "l", B, "List the message types and whether each is monitored."))
        .flag(f("monitor", "m", A, "Turn monitoring on or off; returns the types monitored after."))
        .flag(f("type", "t", S, "Only the message types whose names hold this.")),
    );
    c.register(
        q_flags(Cmd::new(
            "dbtrace",
            "Trace objects: turn on those of a -k keyword (the default action), -off them, send them to -o a file, -f filter \
             them with a dgfilter, -m mark or -t title their outputs, -tm time them. -q lists the keywords on (off with -off), \
             or with -o, -f, -tm or -i what each trace object has.",
            dbtrace,
        ))
        .undoable(false)
        .flag(f("filter", "f", S, "A dgfilter for the trace objects; a name that is no filter yet filters on that node type."))
        .flag(f("info", "i", B, "Query: what each trace object traces."))
        .flag(f("keyword", "k", A, "The keyword of the trace objects to change or ask about.").multi(1))
        .flag(f("mark", "m", B, "Write a mark to the outputs of the trace objects."))
        .flag(f("off", "", B, "Turn the trace objects off; with -q, list the keywords off."))
        .flag(f("output", "o", S, "Where the trace objects write: stdout, stderr or msdev for the Script Editor, or a file."))
        .flag(f("timed", "tm", A, "Time each line of the trace objects' output."))
        .flag(f("title", "t", S, "Write a title line to the outputs of the trace objects."))
        .flag(f("verbose", "v", B, "Every trace object in the output and filter queries, not only those on.")),
    );
    c.register(
        Cmd::new(
            "dgfilter",
            "Filters of graph objects: by -atr attribute, -p plug, -nd node or -nt node type pattern, or -and, -or and -not of \
             other filters, under -n a name. With objects, whether all pass the filter; without, the filter's name. -l lists \
             the filters, -l -n describes one.",
            dgfilter,
        )
        .positional("objects")
        .undoable(false)
        .flag(f("attribute", "atr", S, "Objects whose attribute names match this pattern."))
        .flag(f("list", "l", B, "List the filters; with -n, describe that one."))
        .flag(f("logicalAnd", "and", A, "Two filters that must both pass.").multi(2))
        .flag(f("logicalNot", "not", S, "A filter that must fail."))
        .flag(f("logicalOr", "or", A, "Two filters of which one must pass.").multi(2))
        .flag(f("name", "n", S, "The filter to use, or to make under that name."))
        .flag(f("node", "nd", S, "Objects whose node names match this pattern."))
        .flag(f("nodeType", "nt", S, "Objects whose node types match this pattern."))
        .flag(f("plug", "p", S, "Objects whose plugs (node.attribute) match this pattern.")),
    );
    c.register(
        q_flags(Cmd::new(
            "dbpeek",
            "Look into the scene's data with a peek operation (-op nodes, plugIterator, metadata, references) over the objects \
             named or selected, or all; -a its arguments, -c its count, -eg only the nodes animation reaches, -of a file. \
             -q -op lists the operations; -q -op name describes one, -q -op name -a its arguments.",
            dbpeek,
        ))
        .positional("objects")
        .undoable(false)
        .flag(f("allObjects", "all", B, "Every applicable object, whatever is named or selected."))
        .flag(f("argument", "a", A, "An argument for the operation: name or name=value.").multi(1))
        .flag(f("count", "c", I, "A count for the operation: what it means depends on it (see -q -op)."))
        .flag(f("evaluationGraph", "eg", B, "Only the nodes animation reaches."))
        .flag(f("operation", "op", S, "The peek operation."))
        .flag(f("outputFile", "of", S, "A file for the output, or stdout, stderr or msdev for the Script Editor; returns the errors met.")),
    );
    c.register(Cmd::new("dgmodified", "The nodes changed since the scene was last opened, saved or made new.", dgmodified).undoable(false));
    c.register(
        Cmd::new(
            "dgValidateCurve",
            "Check that the animation curves named or selected (-a all) carry the static tag their keys call for, and fix it; \
             returns how many changed. -v prints each.",
            dg_validate_curve,
        )
        .positional("objects")
        .undoable(false)
        .flag(f("allCurves", "a", B, "Every animation curve, whatever is named or selected."))
        .flag(f("verbose", "v", B, "Print each curve and whether it is static.")),
    );
    c.register(
        q_flags(Cmd::new(
            "displayString",
            "A string table: an identifier given a -v value (the identifier itself by default); -r to change one; -q -v reads it \
             with ^1s to ^3s taken from the arguments after; -ex whether it exists, -k the identifiers holding a text, -d deletes.",
            display_string,
        ))
        .positional("identifier")
        .undoable(false)
        .flag(f("delete", "d", B, "Remove the identifier; fails if it does not exist."))
        .flag(f("exists", "ex", B, "Whether the identifier exists."))
        .flag(f("keys", "k", B, "The identifiers that hold the text given."))
        .flag(f("replace", "r", B, "Change an identifier that exists."))
        .flag(f("value", "v", S, "The string.")),
    );
    c.register(
        Cmd::new("disableIncorrectNameWarning", "Stop warning, for the rest of the session, when a file read holds node names the reference would not take.", |_s, _a| {
            NAME_WARNING_OFF.store(true, Ordering::Relaxed);
            Ok(Value::Null)
        })
        .undoable(false),
    );
}
