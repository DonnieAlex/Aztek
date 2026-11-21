//! The reference's system and utility commands of this module: choice, affectedNet (with its TdnAffect nodes),
//! backgroundEvaluationManager, dagObjectCompare, dbcount, dbfootprint and aaf2fcp.
//!
//! dbcount's counters count what the graph does (computes, dirty messages, nodes made, connections made), by node type.
//! What a command prints to the Script Editor goes through `print` and the host's history.

use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::ref_core::f;
use crate::session::Session;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Int as I, String as S};

/// State the commands of this module hold for the session.
#[derive(Default)]
pub struct SysState {
    /// backgroundEvaluationManager: mode, pause requests, fast interruption.
    pub bg_mode: Option<String>,
    pub bg_pauses: i64,
    pub bg_interrupt: bool,
}

// ---------------------------------------------------------------------------------------------------------------------
// Lines for the Script Editor

static PRINTED: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// A line for the Script Editor, as the reference's commands print to it.
pub fn print(line: impl Into<String>) {
    PRINTED.lock().unwrap_or_else(|e| e.into_inner()).push(line.into());
}

/// The lines printed since last asked, for the host's history.
pub fn take_printed() -> Vec<String> {
    std::mem::take(&mut *PRINTED.lock().unwrap_or_else(|e| e.into_inner()))
}

fn strings(a: &Args, k: &str) -> Vec<String> {
    match a.get(k) {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string).or_else(|| x.as_f64().map(|n| n.to_string()))).collect(),
        _ => Vec::new(),
    }
}

fn numbers(a: &Args, k: &str) -> Vec<f64> {
    fn walk(v: &Value, out: &mut Vec<f64>) {
        match v {
            Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            Value::String(t) => {
                // "1:30" ranges are not times here: each end counts.
                for p in t.split(':') {
                    out.extend(p.trim().parse::<f64>().ok());
                }
            }
            x => out.extend(x.as_f64()),
        }
    }
    let mut out = Vec::new();
    if let Some(v) = a.get(k) {
        walk(v, &mut out);
    }
    out
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true" | "yes" | "1")))
}

// ---------------------------------------------------------------------------------------------------------------------
// choice

/// The choice node feeding a plug: its output drives the plug.
fn choice_on(g: &Graph, p: &PlugRef) -> Option<NodeId> {
    let s = g.source(p)?;
    let n = g.node(s.node)?;
    (n.ty().name == "choice" && n.ty().attrs[s.attr as usize].name == "output").then_some(s.node)
}

/// A channel a choice node drives keeps its own keys on the input the selector picks now, as the reference's choice
/// example keys its channel between its choices.
pub(crate) fn choice_slot(g: &Graph, p: &PlugRef) -> Option<PlugRef> {
    let c = choice_on(g, p)?;
    let n = g.node(c)?;
    let sel = n.ty().attr_index("selector")?;
    let input = n.ty().attr_index("input")?;
    let pick = n.data().value(sel).scalar().unwrap_or(0.0).round() as i32;
    Some(PlugRef::element(c, input as i32, pick))
}

/// The plugs the choice command works on: -at on each object (or its shapes), else every keyable attribute, with the
/// shapes' below (-shape, on by default) and their control points with -cp.
fn choice_plugs(s: &mut Session, a: &Args) -> CmdResult<Vec<PlugRef>> {
    let objs = objects(s, a);
    if objs.is_empty() {
        fail!("choice: name or select the objects");
    }
    let attrs = strings(a, "attribute");
    let with_shapes = a.get("shape").map(on).unwrap_or(true);
    let points = a.get("controlPoints").is_some_and(on);
    let mut out = Vec::new();
    for o in objs {
        // A plug named in full.
        if o.contains('.') {
            out.push(s.graph.plug(&o)?);
            continue;
        }
        let Some(id) = s.graph.find(&o)? else { fail!("choice: no object named {o}") };
        let mut holders = vec![id];
        if with_shapes && s.graph.node(id).is_some_and(|n| n.ty().is_a("transform")) {
            holders.extend(crate::dag::shapes(&s.graph, id, false));
        }
        if !attrs.is_empty() {
            for at in &attrs {
                let found = holders.iter().find_map(|&h| s.graph.plug(&format!("{}.{at}", s.graph.path(h))).ok());
                match found {
                    Some(p) => out.push(p),
                    None => fail!("choice: {o} has no attribute {at}"),
                }
            }
            continue;
        }
        for h in holders {
            let path = s.graph.path(h);
            let names = s.run("listAttr", json!({"args": [path.clone()], "keyable": true}))?;
            for n in names.as_array().into_iter().flatten().filter_map(Value::as_str) {
                if !points && (n.starts_with("controlPoints") || n.starts_with("pnts") || n.starts_with("cp[")) {
                    continue;
                }
                if let Ok(p) = s.graph.plug(&format!("{path}.{n}")) {
                    if !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
    }
    Ok(out)
}

fn choice(s: &mut Session, a: &Args) -> CmdResult {
    let plugs = choice_plugs(s, a)?;
    let asked = |k: &str| crate::ref_core::asked(a, k);
    if a.bool("query") {
        let g = &s.graph;
        if asked("selector") {
            let mut out = Vec::new();
            for p in &plugs {
                if let Some(c) = choice_on(g, p) {
                    let sel = g.node(c).unwrap().ty().attr_index("selector").unwrap();
                    out.push(json!(g.source(&PlugRef::new(c, sel as i32)).map(|x| g.plug_name(&x)).unwrap_or_default()));
                }
            }
            return Ok(Value::Array(out));
        }
        if asked("index") {
            let mut times = numbers(a, "time");
            if times.is_empty() {
                times.push(s.graph.time());
            }
            let mut out = Vec::new();
            for p in plugs {
                let Some(c) = choice_on(&s.graph, &p) else { continue };
                let name = format!("{}.selector", s.graph.path(c));
                for &t in &times {
                    let v = s.run("getAttr", json!({"args": [name.clone()], "time": t}))?;
                    out.push(json!(v.as_f64().unwrap_or(0.0).round() as i64));
                }
            }
            return Ok(Value::Array(out));
        }
        if asked("name") {
            let g = &s.graph;
            return Ok(Value::Array(plugs.iter().filter_map(|p| choice_on(g, p)).map(|c| json!(g.path(c))).collect()));
        }
        fail!("choice: query -selector, -index or -name");
    }
    let times = {
        let t = numbers(a, "time");
        if t.is_empty() {
            vec![s.graph.time()]
        } else {
            t
        }
    };
    let source = a.get("sourceAttribute").and_then(Value::as_str).map(str::to_string);
    let selector = a.get("selector").and_then(Value::as_str).map(str::to_string);
    let mut nodes = Vec::new();
    let mut keyed = Vec::new();
    for p in plugs {
        let (c, fresh) = match choice_on(&s.graph, &p) {
            Some(c) => (c, false),
            None => {
                let name = a.string("name", "choice#");
                let made = s.run("createNode", json!({"args": ["choice"], "name": if name.is_empty() { "choice#".to_string() } else { name }}))?;
                let made = made.as_str().map(str::to_string).or_else(|| made.get(0).and_then(Value::as_str).map(str::to_string)).unwrap_or_default();
                let Some(c) = s.graph.find(&made)? else { fail!("choice: couldn't make a choice node") };
                // What drove the channel becomes the first input.
                let input = s.graph.node(c).unwrap().ty().attr_index("input").unwrap() as i32;
                let output = s.graph.node(c).unwrap().ty().attr_index("output").unwrap() as i32;
                let whole = PlugRef { component: -1, ..p };
                let by_whole = if p.component >= 0 && s.graph.source(&p).is_none() { s.graph.source(&whole) } else { None };
                if let Some(w) = by_whole {
                    // One axis of a vector driven whole: the other axes stay connected axis by axis.
                    s.graph.disconnect(&whole);
                    for k in 0..3 {
                        let from = PlugRef { component: k, ..w };
                        if k == p.component {
                            s.graph.connect(&from, &PlugRef::element(c, input, 0), true)?;
                        } else {
                            s.graph.connect(&from, &PlugRef { component: k, ..whole }, true)?;
                        }
                    }
                } else if let Some(was) = s.graph.source(&p) {
                    s.graph.disconnect(&p);
                    s.graph.connect(&was, &PlugRef::element(c, input, 0), true)?;
                } else {
                    // Nothing drove it: its value is the first input's.
                    let v = s.graph.value(&p)?;
                    s.graph.set_value(PlugRef::element(c, input, 0), v)?;
                }
                s.graph.connect(&PlugRef::new(c, output), &p, true)?;
                (c, true)
            }
        };
        let ty = s.graph.node(c).unwrap().type_arc().clone();
        let input = ty.attr_index("input").unwrap() as i32;
        let sel = ty.attr_index("selector").unwrap() as i32;
        let used: Vec<i32> = s.graph.node(c).unwrap().elements(input as usize).iter().map(|(k, _)| *k).collect();
        let src = match &source {
            Some(x) => Some(s.graph.plug(x)?),
            None => None,
        };
        // The input to use: -index, where the source already is, or a new one; the first for a new node.
        let index = if let Some(i) = a.get("index").and_then(Value::as_i64) {
            i as i32
        } else if let Some(sp) = src {
            used.iter().copied().find(|&k| s.graph.source(&PlugRef::element(c, input, k)) == Some(sp)).unwrap_or_else(|| used.iter().max().map_or(0, |m| m + 1))
        } else if fresh {
            0
        } else {
            used.iter().max().map_or(0, |m| m + 1)
        };
        if let Some(sp) = src {
            s.graph.connect(&sp, &PlugRef::element(c, input, index), true)?;
        } else if !used.contains(&index) {
            // A new input starts at the channel's value now.
            let v = s.graph.value(&PlugRef::element(c, input, *used.first().unwrap_or(&0))).unwrap_or(PlugValue::Float(0.0));
            s.graph.set_value(PlugRef::element(c, input, index), v)?;
        }
        let cname = s.graph.path(c);
        if let Some(sl) = &selector {
            let sp = s.graph.plug(sl)?;
            s.graph.connect(&sp, &PlugRef::new(c, sel), true)?;
        } else if s.graph.source(&PlugRef::new(c, sel)).is_none_or(|x| crate::anim::is_curve(&s.graph, x.node)) {
            for &t in &times {
                s.run("setKeyframe", json!({"args": [format!("{cname}.selector")], "time": t, "value": index, "inTangentType": "step", "outTangentType": "step"}))?;
            }
            keyed.push(format!("{cname}.selector"));
        }
        if !nodes.contains(&cname) {
            nodes.push(cname);
        }
    }
    let mut out: Vec<Value> = nodes.into_iter().map(Value::String).collect();
    for k in keyed {
        if !out.iter().any(|x| x.as_str() == Some(&k)) {
            out.push(json!(k));
        }
    }
    Ok(Value::Array(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// affectedNet

struct Inert;
impl NodeBehavior for Inert {
    fn compute(&mut self, _ctx: &EvalContext, _d: &mut NodeData) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn affect_type() {
    if types::find("TdnAffect").is_some() {
        return;
    }
    let message = |name: &str, short: &str, fl: u32| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: fl | flags::MESSAGE, ..Default::default() };
    let _ = types::register_type(NodeType {
        name: "TdnAffect".into(),
        doc: "One attribute in an affectedNet network: what affects it connects to its input, what it affects to its output.".into(),
        own_attrs: vec![
            AttrDef { name: "attributeName".into(), short_name: "an".into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: flags::INPUT | flags::STORABLE, ..Default::default() },
            message("input", "i", flags::INPUT | flags::ARRAY),
            message("output", "o", flags::OUTPUT),
        ],
        create: Some(Arc::new(|| Box::new(Inert))),
        ignores_added_attrs: true,
        ..Default::default()
    });
}

/// Which attributes of a type affect which: every input affects every output, as `affects` answers.
fn affect_pairs(ty: &NodeType) -> (Vec<String>, Vec<(String, String)>) {
    let shown = |d: &AttrDef| !d.name.contains('.');
    let is_in = |d: &AttrDef| d.has(flags::INPUT) && !d.has(flags::OUTPUT);
    let is_out = |d: &AttrDef| d.has(flags::OUTPUT) && !d.has(flags::INPUT);
    let names: Vec<String> = ty.attrs.iter().filter(|d| shown(d)).map(|d| d.name.clone()).collect();
    let mut pairs = Vec::new();
    for i in ty.attrs.iter().filter(|d| shown(d) && is_in(d)) {
        for o in ty.attrs.iter().filter(|d| shown(d) && is_out(d)) {
            pairs.push((i.name.clone(), o.name.clone()));
        }
    }
    (names, pairs)
}

fn affected_net(s: &mut Session, a: &Args) -> CmdResult {
    affect_type();
    let mut kinds: Vec<std::sync::Arc<NodeType>> = Vec::new();
    for t in strings(a, "type") {
        match types::find(&t) {
            Some(x) => kinds.push(x),
            None => fail!("affectedNet: unknown node type '{t}'"),
        }
    }
    if kinds.is_empty() {
        for o in objects(s, a) {
            let Some(id) = s.graph.find(&o)? else { fail!("affectedNet: no node named {o}") };
            kinds.push(s.graph.node(id).unwrap().type_arc().clone());
        }
    }
    if kinds.is_empty() {
        fail!("affectedNet: name or select a node, or give -type");
    }
    // The attributes of all the types, each once; and each type's pairs.
    let mut names: Vec<String> = Vec::new();
    let mut pairs: Vec<(String, String)> = Vec::new();
    for t in &kinds {
        let (n, p) = affect_pairs(t);
        for x in n {
            if !names.contains(&x) {
                names.push(x);
            }
        }
        for x in p {
            if !pairs.contains(&x) {
                pairs.push(x);
            }
        }
    }
    let label = kinds.iter().map(|t| t.name.clone()).collect::<Vec<_>>().join("_");
    let ty = types::find("TdnAffect").unwrap();
    let mut made: HashMap<String, NodeId> = HashMap::new();
    for n in &names {
        let id = s.graph.create_node(&ty, &format!("{n}Affect"), NO_NODE, true)?;
        s.graph.set_value(PlugRef::new(id, ty.attr_index("attributeName").unwrap() as i32), PlugValue::String(format!("{label}.{n}")))?;
        made.insert(n.clone(), id);
    }
    let (input, output) = (ty.attr_index("input").unwrap() as i32, ty.attr_index("output").unwrap() as i32);
    let mut next: HashMap<NodeId, i32> = HashMap::new();
    for (from, to) in pairs {
        let (Some(&x), Some(&y)) = (made.get(&from), made.get(&to)) else { continue };
        let k = next.entry(y).or_insert(0);
        s.graph.connect(&PlugRef::new(x, output), &PlugRef::element(y, input, *k), false)?;
        *k += 1;
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// backgroundEvaluationManager

fn background_evaluation(s: &mut Session, a: &Args) -> CmdResult {
    let st = &mut s.extras.sys;
    if a.bool("query") {
        let asked = |k: &str| crate::ref_core::asked(a, k);
        return Ok(if asked("mode") {
            json!(st.bg_mode.clone().unwrap_or_else(|| "parallel".into()))
        } else if asked("pause") {
            json!(st.bg_pauses > 0)
        } else if asked("interrupt") {
            json!(st.bg_interrupt)
        } else {
            fail!("backgroundEvaluationManager: query -mode, -pause or -interrupt")
        });
    }
    if let Some(m) = a.get("mode").and_then(Value::as_str) {
        if !matches!(m, "serial" | "parallel") {
            fail!("backgroundEvaluationManager: the mode is serial or parallel, not '{m}'");
        }
        st.bg_mode = Some(m.to_string());
    }
    if let Some(v) = a.get("interrupt") {
        st.bg_interrupt = on(v);
    }
    if a.bool("pause") {
        st.bg_pauses += 1;
    }
    if a.bool("resume") {
        // A pause someone else asked for stays: only this caller's own is taken back.
        st.bg_pauses = (st.bg_pauses - 1).max(0);
    }
    Ok(json!(true))
}

/// Whether background evaluation is paused, for whatever runs in the background to wait on.
pub fn background_paused(s: &Session) -> bool {
    s.extras.sys.bg_pauses > 0
}

// ---------------------------------------------------------------------------------------------------------------------
// dagObjectCompare

fn strip_ns(name: &str, ns: &str) -> String {
    let leaf = name.rsplit('|').next().unwrap_or(name);
    if !ns.is_empty() {
        if let Some(rest) = leaf.strip_prefix(&format!("{ns}:")) {
            return rest.to_string();
        }
    }
    leaf.to_string()
}

fn values_match(x: &PlugValue, y: &PlugValue) -> bool {
    const E: f64 = 1e-6;
    match (x, y) {
        (PlugValue::Float(a), PlugValue::Float(b)) => (a - b).abs() < E,
        (PlugValue::Vec3(a), PlugValue::Vec3(b)) => (*a - *b).abs().max_element() < E,
        (PlugValue::Matrix(a), PlugValue::Matrix(b)) => a.abs_diff_eq(*b, E),
        (PlugValue::Int(a), PlugValue::Int(b)) => a == b,
        (PlugValue::Bool(a), PlugValue::Bool(b)) => a == b,
        (PlugValue::String(a), PlugValue::String(b)) => a == b,
        (PlugValue::Mesh(a), PlugValue::Mesh(b)) => a.is_some() == b.is_some(),
        (PlugValue::Data(a), PlugValue::Data(b)) => a.is_some() == b.is_some(),
        (a, b) => a.scalar().zip(b.scalar()).is_some_and(|(p, q)| (p - q).abs() < E),
    }
}

/// One object's connections as (own attribute, direction, other node without namespace, other attribute).
fn links(g: &Graph, id: NodeId, ns: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (d, s) in g.connections() {
        if d.node == id {
            let other = g.plug_name(&s);
            let (node, attr) = other.split_once('.').unwrap_or((&other, ""));
            out.push(format!("{} <- {}.{attr}", g.plug_name(&d).split_once('.').map_or("", |x| x.1), strip_ns(node, ns)));
        }
        if s.node == id {
            let other = g.plug_name(&d);
            let (node, attr) = other.split_once('.').unwrap_or((&other, ""));
            out.push(format!("{} -> {}.{attr}", g.plug_name(&s).split_once('.').map_or("", |x| x.1), strip_ns(node, ns)));
        }
    }
    out.sort();
    out
}

fn dag_object_compare(s: &mut Session, a: &Args) -> CmdResult {
    let ns = a.string("namespace", "");
    let bail = a.string("bail", "never");
    if !matches!(bail.as_str(), "never" | "first" | "category") {
        fail!("dagObjectCompare: -bail is never, first or category, not '{bail}'");
    }
    let objs = objects(s, a);
    // Two objects compare with each other; with -namespace, each one with its namesake in the baseline namespace.
    let mut pairs: Vec<(NodeId, NodeId)> = Vec::new();
    let find = |s: &Session, n: &str| -> CmdResult<NodeId> {
        match s.graph.find(n)? {
            Some(id) => Ok(id),
            None => fail!("dagObjectCompare: no object named {n}"),
        }
    };
    if objs.len() == 2 {
        pairs.push((find(s, &objs[0])?, find(s, &objs[1])?));
    } else if !ns.is_empty() && !objs.is_empty() {
        for o in &objs {
            let leaf = strip_ns(o, &ns);
            if o.rsplit('|').next().unwrap_or(o).starts_with(&format!("{ns}:")) {
                continue;
            }
            pairs.push((find(s, &format!("{ns}:{leaf}"))?, find(s, o)?));
        }
    } else {
        fail!("dagObjectCompare: name or select two DAG objects, or objects with -namespace for their baselines");
    }
    for &(x, y) in &pairs {
        if !s.graph.node(x).is_some_and(|n| n.ty().dag) || !s.graph.node(y).is_some_and(|n| n.ty().dag) {
            fail!("dagObjectCompare: only DAG objects compare");
        }
    }
    let all = !["type", "relative", "connection", "attribute"].iter().any(|k| a.bool(k));
    let short = a.bool("short");
    let mut errors = 0usize;
    'pairs: for (base, other) in pairs {
        let (bn, on_) = (s.graph.path(base), s.graph.path(other));
        let report = |cat: &str, what: String, errors: &mut usize| {
            *errors += 1;
            print(if short { format!("{cat}: {what}") } else { format!("dagObjectCompare: {on_} differs from {bn} in {cat}: {what}") });
        };
        for cat in ["type", "relative", "connection", "attribute"] {
            if !(all || a.bool(cat)) {
                continue;
            }
            let before = errors;
            let mut stop = false;
            match cat {
                "type" => {
                    let (t1, t2) = (s.graph.node(base).unwrap().ty().name.clone(), s.graph.node(other).unwrap().ty().name.clone());
                    if t1 != t2 {
                        report(cat, format!("{t2} against {t1}"), &mut errors);
                    }
                }
                "relative" => {
                    let g = &s.graph;
                    let name = |id: NodeId| if id == NO_NODE { String::new() } else { strip_ns(&g.path(id), &ns) };
                    let (p1, p2) = (name(g.parent(base)), name(g.parent(other)));
                    if p1 != p2 {
                        report(cat, format!("parent '{p2}' against '{p1}'"), &mut errors);
                        stop = bail == "first";
                    }
                    let kids = |id: NodeId| g.children(id).iter().map(|&c| name(c)).collect::<Vec<_>>();
                    let (c1, c2) = (kids(base), kids(other));
                    for c in c1.iter().filter(|c| !c2.contains(c)) {
                        if stop {
                            break;
                        }
                        report(cat, format!("no child {c}"), &mut errors);
                        stop = bail == "first";
                    }
                    for c in c2.iter().filter(|c| !c1.contains(c)) {
                        if stop {
                            break;
                        }
                        report(cat, format!("extra child {c}"), &mut errors);
                        stop = bail == "first";
                    }
                }
                "connection" => {
                    let (l1, l2) = (links(&s.graph, base, &ns), links(&s.graph, other, &ns));
                    for l in l1.iter().filter(|l| !l2.contains(l)) {
                        if stop {
                            break;
                        }
                        report(cat, format!("no connection {l}"), &mut errors);
                        stop = bail == "first";
                    }
                    for l in l2.iter().filter(|l| !l1.contains(l)) {
                        if stop {
                            break;
                        }
                        report(cat, format!("extra connection {l}"), &mut errors);
                        stop = bail == "first";
                    }
                }
                _ => {
                    let attrs: Vec<AttrDef> = s.graph.node(base).unwrap().ty().attrs.clone();
                    for (i, d) in attrs.iter().enumerate() {
                        if stop {
                            break;
                        }
                        if d.has(flags::HIDDEN) || d.is_array() || d.has(flags::MESSAGE) || !d.has(flags::INPUT) {
                            continue;
                        }
                        let Some(j) = s.graph.node(other).unwrap().ty().attr_index(&d.name) else {
                            report(cat, format!("no attribute {}", d.name), &mut errors);
                            stop = bail == "first";
                            continue;
                        };
                        let dj = s.graph.node(other).unwrap().ty().attrs[j].clone();
                        if dj.ty != d.ty {
                            report(cat, format!("{} is of another type", d.name), &mut errors);
                            stop = bail == "first";
                            continue;
                        }
                        let (v1, v2) = (s.graph.value(&PlugRef::new(base, i as i32))?, s.graph.value(&PlugRef::new(other, j as i32))?);
                        if !values_match(&v1, &v2) {
                            report(cat, format!("{} is {} against {}", d.name, crate::plug::to_json(&v2), crate::plug::to_json(&v1)), &mut errors);
                            stop = bail == "first";
                        }
                    }
                }
            }
            if errors > before && (bail == "first" || bail == "category") {
                break 'pairs;
            }
        }
    }
    if errors == 0 && !short {
        print("dagObjectCompare: no differences found");
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// dbcount

/// The counters, top level first: what the graph does, each counted by node type below it.
const COUNTERS: &[&str] = &["compute", "dirty", "createNode", "connect"];

static COUNTING: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
struct Counts {
    hits: BTreeMap<String, u64>,
    on: BTreeMap<String, bool>,
}

static COUNTS: Mutex<Option<Counts>> = Mutex::new(None);

fn with_counts<R>(f: impl FnOnce(&mut Counts) -> R) -> R {
    let mut c = COUNTS.lock().unwrap_or_else(|e| e.into_inner());
    f(c.get_or_insert_with(Counts::default))
}

/// A hit on a counter (one of `COUNTERS`) for a node type, while counters are on.
pub fn count(counter: &str, node_type: &str) {
    if !COUNTING.load(Ordering::Relaxed) {
        return;
    }
    with_counts(|c| {
        if !c.on.get(counter).copied().unwrap_or(false) {
            return;
        }
        *c.hits.entry(counter.to_string()).or_insert(0) += 1;
        *c.hits.entry(format!("{counter}/{node_type}")).or_insert(0) += 1;
    });
}

fn dbcount(_s: &mut Session, a: &Args) -> CmdResult {
    let keyword = a.string("keyword", "");
    let picked = |name: &str| keyword.is_empty() || name.contains(&keyword);
    if let Some(v) = a.get("enabled") {
        let state = on(v);
        let affected: Vec<String> = COUNTERS.iter().filter(|c| picked(c)).map(|c| c.to_string()).collect();
        with_counts(|c| {
            for n in &affected {
                c.on.insert(n.clone(), state);
            }
            COUNTING.store(c.on.values().any(|x| *x), Ordering::Relaxed);
        });
        return Ok(json!(affected));
    }
    if a.bool("list") {
        let lines: Vec<String> = with_counts(|c| COUNTERS.iter().filter(|n| picked(n)).map(|n| format!("{n} ({})", if c.on.get(*n).copied().unwrap_or(false) { "on" } else { "off" })).collect());
        for l in &lines {
            print(l.clone());
        }
        return Ok(Value::Null);
    }
    if !COUNTING.load(Ordering::Relaxed) {
        fail!("dbcount: the counters are off: turn them on with -enabled on");
    }
    if a.bool("reset") {
        let affected: Vec<String> = COUNTERS.iter().filter(|c| picked(c)).map(|c| c.to_string()).collect();
        with_counts(|c| {
            c.hits.retain(|k, _| !affected.iter().any(|n| k.starts_with(&format!("{n}/"))));
            for n in &affected {
                c.hits.insert(n.clone(), 0);
            }
        });
        return Ok(json!(affected));
    }
    let max_depth = a.get("maxdepth").and_then(Value::as_u64).unwrap_or(0);
    let depth_ok = |d: u64| max_depth == 0 || d < max_depth;
    let hits = with_counts(|c| c.hits.clone());
    let enabled = with_counts(|c| c.on.clone());
    let mut lines = Vec::new();
    if a.bool("spreadsheet") {
        lines.push("Counter\tCount".to_string());
    }
    for top in COUNTERS.iter().filter(|n| enabled.get(**n).copied().unwrap_or(false)) {
        let kids: Vec<(&String, &u64)> = hits.iter().filter(|(k, _)| k.starts_with(&format!("{top}/")) && (picked(top) || picked(k))).collect();
        if !picked(top) && kids.is_empty() {
            continue;
        }
        let total = hits.get(*top).copied().unwrap_or(0);
        if a.bool("spreadsheet") {
            lines.push(format!("{top}\t{total}"));
            if !a.bool("quick") && depth_ok(1) {
                lines.extend(kids.iter().map(|(k, v)| format!("{k}\t{v}")));
            }
        } else if a.bool("quick") || kids.is_empty() || !depth_ok(1) {
            lines.push(format!("{top} = {total}"));
        } else {
            lines.push(format!("{top} = {total} {{"));
            lines.extend(kids.iter().map(|(k, v)| format!("    {} = {v}", k.split_once('/').map_or(k.as_str(), |x| x.1))));
            lines.push("}".into());
        }
    }
    let file = a.string("file", "");
    if file.is_empty() || matches!(file.as_str(), "stdout" | "stderr" | "msdev") {
        for l in lines {
            print(l);
        }
    } else {
        let mut fh = std::fs::File::create(&file).map_err(|e| aztek_ops::CommandError::new(format!("dbcount: can't write {file}: {e}")))?;
        for l in lines {
            let _ = writeln!(fh, "{l}");
        }
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// dbfootprint

/// The footprint types, with what each measures.
const FOOTPRINTS: &[(&str, &str)] = &[("nodes", "The memory each dependency node holds: its attribute values, array elements and geometry, in bytes.")];

fn value_bytes(v: &PlugValue) -> usize {
    std::mem::size_of::<PlugValue>()
        + match v {
            PlugValue::String(t) => t.capacity(),
            PlugValue::Mesh(Some(m)) => m.memory_bytes(),
            _ => 0,
        }
}

fn node_bytes(g: &Graph, id: NodeId) -> usize {
    let Some(n) = g.node(id) else { return 0 };
    let d = n.data();
    let mut b = std::mem::size_of::<crate::node::Node>() + n.name().len();
    for i in 0..n.ty().attrs.len() {
        b += value_bytes(d.value(i));
        b += n.elements(i).iter().map(|(_, v)| 4 + value_bytes(v)).sum::<usize>();
    }
    b
}

fn dbfootprint(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let t = a.get("type").and_then(Value::as_str).filter(|t| !t.is_empty());
        return Ok(match t {
            Some(t) => match FOOTPRINTS.iter().find(|(n, _)| *n == t) {
                Some((_, doc)) => json!(doc),
                None => fail!("dbfootprint: no footprint type '{t}'"),
            },
            None => json!(FOOTPRINTS.iter().map(|(n, _)| *n).collect::<Vec<_>>()),
        });
    }
    let ty = a.string("type", "nodes");
    if !FOOTPRINTS.iter().any(|(n, _)| *n == ty) {
        fail!("dbfootprint: no footprint type '{ty}'");
    }
    let named = a.named();
    let ids: Vec<NodeId> = if a.bool("allObjects") {
        s.graph.nodes()
    } else if !named.is_empty() {
        let mut v = Vec::new();
        for n in &named {
            match s.graph.find(n)? {
                Some(id) => v.push(id),
                None => fail!("dbfootprint: no node named {n}"),
            }
        }
        v
    } else if !s.graph.selection().is_empty() {
        s.graph.selection().to_vec()
    } else {
        s.graph.nodes()
    };
    let mut objects = serde_json::Map::new();
    let mut total = 0usize;
    for id in ids {
        let b = node_bytes(&s.graph, id);
        total += b;
        let n = s.graph.node(id).unwrap();
        objects.insert(s.graph.path(id), json!({"type": n.ty().name, "bytes": b}));
    }
    let text = serde_json::to_string_pretty(&json!({"footprint": ty, "objects": objects, "total": total})).unwrap_or_default();
    match a.get("outputFile").and_then(Value::as_str) {
        None => Ok(json!(text)),
        Some("stdout" | "cout") => {
            println!("{text}");
            Ok(Value::Null)
        }
        Some("stderr" | "cerr") => {
            eprintln!("{text}");
            Ok(Value::Null)
        }
        Some(path) => {
            std::fs::write(path, &text).map_err(|e| aztek_ops::CommandError::new(format!("dbfootprint: can't write {path}: {e}")))?;
            Ok(Value::Null)
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// aaf2fcp

/// A conversion running in the background.
#[derive(Default)]
struct Job {
    progress: i64,
    done: bool,
    error: Option<String>,
    out: std::path::PathBuf,
}

static JOBS: Mutex<BTreeMap<i64, Arc<Mutex<Job>>>> = Mutex::new(BTreeMap::new());
static NEXT_JOB: AtomicI64 = AtomicI64::new(1);

/// The structured storage file an AAF file is: its directory's entries, each (name, kind, first sector, size), and the
/// streams read whole by name.
struct Storage {
    data: Vec<u8>,
    sector: usize,
    fat: Vec<u32>,
    mini_fat: Vec<u32>,
    mini_stream: Vec<u8>,
    entries: Vec<(String, u8, u32, u64)>,
}

const CFB_MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

impl Storage {
    fn open(data: Vec<u8>) -> Result<Storage, String> {
        if data.len() < 512 || data[..8] != CFB_MAGIC {
            return Err("not an AAF file (no structured storage header)".into());
        }
        let u16_at = |o: usize| u16::from_le_bytes([data[o], data[o + 1]]);
        let u32_at = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap());
        let sector = 1usize << u16_at(0x1E);
        let mini_cutoff = u32_at(0x38) as u64;
        let _ = mini_cutoff;
        let first_dir = u32_at(0x30);
        let first_mini_fat = u32_at(0x3C);
        let mut fat_sectors: Vec<u32> = (0..109).map(|i| u32_at(0x4C + i * 4)).filter(|&x| x < 0xFFFF_FFFA).collect();
        // More FAT sectors listed in the DIFAT chain.
        let mut difat = u32_at(0x44);
        let mut guard = 0;
        while difat < 0xFFFF_FFFA && guard < 10_000 {
            let base = 512 + difat as usize * sector;
            if base + sector > data.len() {
                break;
            }
            for i in 0..sector / 4 - 1 {
                let x = u32::from_le_bytes(data[base + i * 4..base + i * 4 + 4].try_into().unwrap());
                if x < 0xFFFF_FFFA {
                    fat_sectors.push(x);
                }
            }
            difat = u32::from_le_bytes(data[base + sector - 4..base + sector].try_into().unwrap());
            guard += 1;
        }
        let mut fat = Vec::new();
        for s in fat_sectors {
            let base = 512 + s as usize * sector;
            if base + sector > data.len() {
                return Err("damaged AAF file (FAT outside the file)".into());
            }
            fat.extend((0..sector / 4).map(|i| u32::from_le_bytes(data[base + i * 4..base + i * 4 + 4].try_into().unwrap())));
        }
        let mut st = Storage { data, sector, fat, mini_fat: Vec::new(), mini_stream: Vec::new(), entries: Vec::new() };
        let dir = st.chain(first_dir);
        for e in dir.chunks(128) {
            if e.len() < 128 {
                break;
            }
            let len = (u16::from_le_bytes([e[64], e[65]]) as usize).min(64);
            let units: Vec<u16> = e[..len.saturating_sub(2)].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            let name = String::from_utf16_lossy(&units);
            let kind = e[66];
            let start = u32::from_le_bytes(e[116..120].try_into().unwrap());
            let size = u64::from_le_bytes(e[120..128].try_into().unwrap());
            st.entries.push((name, kind, start, size));
        }
        if let Some(root) = st.entries.first().cloned() {
            st.mini_stream = st.chain(root.2);
        }
        if first_mini_fat < 0xFFFF_FFFA {
            let mf = st.chain(first_mini_fat);
            st.mini_fat = mf.chunks(4).filter(|c| c.len() == 4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        }
        Ok(st)
    }

    fn chain(&self, mut s: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut guard = 0;
        while s < 0xFFFF_FFFA && guard < 1_000_000 {
            let base = 512 + s as usize * self.sector;
            if base + self.sector > self.data.len() {
                break;
            }
            out.extend_from_slice(&self.data[base..base + self.sector]);
            s = self.fat.get(s as usize).copied().unwrap_or(0xFFFF_FFFE);
            guard += 1;
        }
        out
    }

    fn stream(&self, i: usize) -> Vec<u8> {
        let (_, _, start, size) = self.entries[i];
        let mut out = if size < 4096 {
            let mut v = Vec::new();
            let mut s = start;
            let mut guard = 0;
            while s < 0xFFFF_FFFA && guard < 1_000_000 {
                let base = s as usize * 64;
                if base + 64 > self.mini_stream.len() {
                    break;
                }
                v.extend_from_slice(&self.mini_stream[base..base + 64]);
                s = self.mini_fat.get(s as usize).copied().unwrap_or(0xFFFF_FFFE);
                guard += 1;
            }
            v
        } else {
            self.chain(start)
        };
        out.truncate(size as usize);
        out
    }
}

/// The names the AAF file's mobs carry (property 0x4402, MobName), in the order they are stored.
fn mob_names(st: &Storage) -> Vec<String> {
    let mut out = Vec::new();
    for (i, (name, kind, _, _)) in st.entries.iter().enumerate() {
        if *kind != 2 || name != "properties" {
            continue;
        }
        let p = st.stream(i);
        if p.len() < 4 {
            continue;
        }
        let count = u16::from_le_bytes([p[2], p[3]]) as usize;
        let mut at = 4 + count * 6;
        for k in 0..count {
            let o = 4 + k * 6;
            if o + 6 > p.len() {
                break;
            }
            let pid = u16::from_le_bytes([p[o], p[o + 1]]);
            let len = u16::from_le_bytes([p[o + 4], p[o + 5]]) as usize;
            if pid == 0x4402 && at + len <= p.len() {
                let units: Vec<u16> = p[at..at + len].chunks(2).filter(|c| c.len() == 2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&u| u != 0).collect();
                let n = String::from_utf16_lossy(&units);
                if !n.is_empty() && !out.contains(&n) {
                    out.push(n);
                }
            }
            at += len;
        }
    }
    out
}

fn xml_escape(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Final Cut Pro XML for an AAF file: a sequence named after it holding a clip for each of its mobs.
fn convert(src: &std::path::Path, job: &Arc<Mutex<Job>>) -> Result<(), String> {
    let data = std::fs::read(src).map_err(|e| format!("can't read {}: {e}", src.display()))?;
    job.lock().unwrap().progress = 20;
    let st = Storage::open(data)?;
    job.lock().unwrap().progress = 50;
    let names = mob_names(&st);
    let stem = src.file_stem().and_then(|x| x.to_str()).unwrap_or("sequence").to_string();
    let mut x = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE xmeml>\n<xmeml version=\"4\">\n");
    x += &format!("  <sequence id=\"{}\">\n    <name>{}</name>\n    <media>\n      <video>\n        <track>\n", xml_escape(&stem), xml_escape(&stem));
    for (i, n) in names.iter().enumerate() {
        x += &format!("          <clipitem id=\"clipitem-{}\">\n            <name>{}</name>\n          </clipitem>\n", i + 1, xml_escape(n));
    }
    x += "        </track>\n      </video>\n    </media>\n  </sequence>\n</xmeml>\n";
    job.lock().unwrap().progress = 80;
    let out = job.lock().unwrap().out.clone();
    std::fs::write(&out, x).map_err(|e| format!("can't write {}: {e}", out.display()))?;
    Ok(())
}

fn aaf2fcp(_s: &mut Session, a: &Args) -> CmdResult {
    let handle_of = |k: &str| a.get(k).and_then(Value::as_i64);
    let job_of = |h: i64| -> CmdResult<Arc<Mutex<Job>>> {
        match JOBS.lock().unwrap().get(&h).cloned() {
            Some(j) => Ok(j),
            None => fail!("aaf2fcp: no conversion {h}"),
        }
    };
    if let Some(h) = handle_of("getFileName") {
        let j = job_of(h)?;
        let out = j.lock().unwrap().out.clone();
        return Ok(json!(out.to_string_lossy()));
    }
    if let Some(h) = handle_of("progress") {
        let j = job_of(h)?;
        let j = j.lock().unwrap();
        return Ok(json!(if j.error.is_some() { -1 } else { j.progress }));
    }
    if let Some(h) = handle_of("waitCompletion") {
        let j = job_of(h)?;
        loop {
            {
                let j = j.lock().unwrap();
                if let Some(e) = &j.error {
                    fail!("aaf2fcp: {e}");
                }
                if j.done {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        return Ok(json!(1));
    }
    if let Some(h) = handle_of("terminate") {
        let j = job_of(h)?;
        JOBS.lock().unwrap().remove(&h);
        if a.get("deleteFile").is_some_and(on) {
            let out = j.lock().unwrap().out.clone();
            let _ = std::fs::remove_file(out);
        }
        return Ok(json!(1));
    }
    let src = a.string("srcFile", "");
    let dst = a.string("dstPath", "");
    if src.is_empty() || dst.is_empty() {
        fail!("aaf2fcp: give -srcFile and -dstPath");
    }
    let srcp = std::path::PathBuf::from(&src);
    let mut head = [0u8; 8];
    let ok = std::fs::File::open(&srcp).and_then(|mut f| std::io::Read::read_exact(&mut f, &mut head)).is_ok();
    if !ok {
        fail!("aaf2fcp: can't read {src}");
    }
    if head != CFB_MAGIC {
        fail!("aaf2fcp: {src} is not an AAF file");
    }
    let stem = srcp.file_stem().and_then(|x| x.to_str()).unwrap_or("out").to_string();
    let out = std::path::Path::new(&dst).join(format!("{stem}.xml"));
    let h = NEXT_JOB.fetch_add(1, Ordering::Relaxed);
    let job = Arc::new(Mutex::new(Job { out, ..Default::default() }));
    JOBS.lock().unwrap().insert(h, job.clone());
    std::thread::spawn(move || {
        let r = convert(&srcp, &job);
        let mut j = job.lock().unwrap();
        match r {
            Ok(()) => {
                j.progress = 100;
                j.done = true;
            }
            Err(e) => j.error = Some(e),
        }
    });
    Ok(json!(h))
}

// ---------------------------------------------------------------------------------------------------------------------

pub fn register(c: &mut Commands<Session>) {
    affect_type();
    c.register(
        crate::system_cmds::q_flags(Cmd::new(
            "choice",
            "Switch what drives a channel over time: a choice node goes between the channel and what drove it, -sa connects another \
             source to an input, and the selector is keyed to pick that input at each -t (the current time by default). \
             Returns the choice nodes and the selector plugs keyed.",
            choice,
        ))
        .positional("objects")
        .flag(f("attribute", "at", A, "The attributes to put choice nodes on (every keyable one by default).").multi(1))
        .flag(f("controlPoints", "cp", B, "Take in a shape's control points too (off)."))
        .flag(f("shape", "s", B, "Take in the attributes of the shapes below a transform (on)."))
        .flag(f("index", "in", I, "The input to connect the source to; with -q, the input picked at each -t."))
        .flag(f("name", "n", S, "The name of new choice nodes; with -q, the choice nodes."))
        .flag(f("selector", "sl", S, "An attribute to drive the selector, in place of keys; with -q, the attribute driving it."))
        .flag(f("sourceAttribute", "sa", S, "The attribute to connect to the choice for the -t times."))
        .flag(f("time", "t", A, "When the choice picks the source (the current time by default).").multi(1)),
    );
    c.register(
        Cmd::new(
            "affectedNet",
            "A network of TdnAffect nodes, one for each attribute of a node or of the -t node types, an input connected to the \
             outputs it affects.",
            affected_net,
        )
        .positional("node")
        .flag(f("type", "t", A, "A node type to read in place of a node; give it again for more.").multi(1)),
    );
    c.register(
        crate::system_cmds::q_flags(Cmd::new(
            "backgroundEvaluationManager",
            "Pause and resume background evaluation: -pause, -resume, -mode serial|parallel, -interrupt on fast interruption. -q asks.",
            background_evaluation,
        ))
        .undoable(false)
        .flag(f("mode", "m", S, "serial or parallel."))
        .flag(f("pause", "p", B, "Pause background evaluation; with -q, whether it is paused."))
        .flag(f("resume", "r", B, "Resume it, unless someone else paused it too."))
        .flag(f("interrupt", "i", A, "Interrupt background work quickly while working interactively.")),
    );
    c.register(
        Cmd::new(
            "dagObjectCompare",
            "Compare two DAG objects by -t type, -r relatives, -c connections and -a attributes (all four when none is given), \
             printing the differences. -n names the baseline's namespace; -b first or category stops early; -s prints less.",
            dag_object_compare,
        )
        .positional("objects")
        .undoable(false)
        .flag(f("type", "t", B, "Compare their types."))
        .flag(f("relative", "r", B, "Compare their parents and children."))
        .flag(f("connection", "c", B, "Compare their connections."))
        .flag(f("attribute", "a", B, "Compare their attributes' values."))
        .flag(f("namespace", "n", S, "The baseline's namespace."))
        .flag(f("bail", "b", S, "never (the default), first or category: when to stop on a difference."))
        .flag(f("short", "s", B, "Shorter lines.")),
    );
    c.register(
        Cmd::new(
            "dbcount",
            "Counters of what the graph does: computes, dirty messages, nodes made and connections made, each by node type. \
             -e on|off turns them on; with no action the counts are printed (-f a file, -k a keyword, -q totals only, \
             -s as a spreadsheet, -md how deep); -r resets; -l lists them.",
            dbcount,
        )
        .undoable(false)
        .flag(f("enabled", "e", A, "Turn the counters on or off; returns those changed."))
        .flag(f("file", "f", S, "Where the counts go: a file, or stdout, stderr or msdev for the Script Editor."))
        .flag(f("keyword", "k", S, "Only counters whose names hold this."))
        .flag(f("list", "l", B, "List the counters and whether each is on."))
        .flag(f("maxdepth", "md", I, "How many levels down to print; 0 for all."))
        .flag(f("quick", "q", B, "Only each counter's total."))
        .flag(f("reset", "r", B, "Set the counters to 0 and drop all but the top ones; returns those reset."))
        .flag(f("spreadsheet", "s", B, "Print as a spreadsheet: a header row, then a counter and its count a line.")),
    );
    c.register(
        crate::system_cmds::q_flags(Cmd::new(
            "dbfootprint",
            "The memory parts of the scene hold, as JSON: of the objects named or selected, or all (-all). -t the kind of \
             footprint (nodes); -q lists the kinds, -q -t one describes it; -of writes it to a file.",
            dbfootprint,
        ))
        .positional("objects")
        .undoable(false)
        .flag(f("allObjects", "all", B, "Measure every object, whatever is named or selected."))
        .flag(f("outputFile", "of", S, "A file for the output, or stdout, cout, stderr or cerr."))
        .flag(f("type", "t", S, "The kind of footprint: nodes.")),
    );
    c.register(
        Cmd::new(
            "aaf2fcp",
            "Convert an AAF file to Final Cut Pro XML in the background: -src and -dst start it and return its handle; -pr its \
             progress, -gfn the file it writes, -wc waits for it, -t ends it (-df deletes the file).",
            aaf2fcp,
        )
        .undoable(false)
        .flag(f("srcFile", "src", S, "The AAF file."))
        .flag(f("dstPath", "dst", S, "The folder to write to."))
        .flag(f("getFileName", "gfn", I, "A conversion's output file."))
        .flag(f("progress", "pr", I, "How far a conversion is, in percent."))
        .flag(f("waitCompletion", "wc", I, "Wait for a conversion to finish."))
        .flag(f("terminate", "t", I, "End a conversion."))
        .flag(f("deleteFile", "df", B, "With -terminate: delete the file written.")),
    );
}
