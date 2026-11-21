//! dgtimer: wall-clock timers on each node, as the reference's. While on, the graph times every compute (self time and
//! count) and counts the dirty messages a node takes; the command turns timing on and off, resets it, and reports it
//! sorted, filtered and totalled. Draw, callback and fetch are measured nowhere here, so they report zero.

use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::NodeId;
use crate::session::Session;

#[derive(Default, Clone, Copy)]
struct NodeTimer {
    off: bool,
    compute: f64,
    compute_count: u64,
    dirty_count: u64,
}

#[derive(Default)]
struct Timers {
    /// By graph and node.
    nodes: HashMap<(u64, NodeId), NodeTimer>,
    since: Option<Instant>,
    overhead: bool,
    trace: Option<std::fs::File>,
}

static ON: AtomicBool = AtomicBool::new(false);
static TIMERS: Mutex<Option<Timers>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut Timers) -> R) -> R {
    let mut t = TIMERS.lock().unwrap_or_else(|e| e.into_inner());
    f(t.get_or_insert_with(Timers::default))
}

/// Whether the graph should time its computes.
pub fn active() -> bool {
    ON.load(Ordering::Relaxed)
}

/// A compute that took `secs`, on a node of graph `serial`.
pub fn record_compute(serial: u64, node: NodeId, name: &str, secs: f64) {
    with(|t| {
        let e = t.nodes.entry((serial, node)).or_default();
        if e.off {
            return;
        }
        e.compute += secs;
        e.compute_count += 1;
        if let Some(f) = t.trace.as_mut() {
            let _ = writeln!(f, "dgtimer:begin: compute 0 {name} {name}");
            let _ = writeln!(f, "dgtimer:end: compute 0 {name} {secs} {secs}");
        }
    });
}

/// A dirty message a node took.
pub fn record_dirty(serial: u64, node: NodeId) {
    with(|t| {
        let e = t.nodes.entry((serial, node)).or_default();
        if !e.off {
            e.dirty_count += 1;
        }
    });
}

const METRICS: &[&str] = &["callback", "compute", "dirty", "draw", "fetch"];

/// A node's figure for a metric and timer type.
fn figure(t: &NodeTimer, metric: &str, ty: &str) -> f64 {
    match (metric, ty) {
        ("compute", "count") => t.compute_count as f64,
        ("compute", _) => t.compute,
        ("dirty", "count") => t.dirty_count as f64,
        _ => 0.0,
    }
}

fn strings(a: &Args, k: &str) -> Vec<String> {
    match a.get(k) {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

/// The nodes -name (with -hierarchy) or -type pick, or None for all.
fn picked(g: &Graph, a: &Args) -> CmdResult<Option<Vec<NodeId>>> {
    if let Some(n) = a.get("name").and_then(Value::as_str) {
        let Some(id) = g.find(n)? else { fail!("dgtimer: no node named {n}") };
        let mut v = vec![id];
        if a.bool("hierarchy") {
            v.extend(g.descendants(id));
        }
        return Ok(Some(v));
    }
    if let Some(ty) = a.get("type").and_then(Value::as_str) {
        return Ok(Some(g.nodes().into_iter().filter(|&n| g.node(n).is_some_and(|x| x.ty().is_a(ty))).collect()));
    }
    Ok(None)
}

fn dgtimer(s: &mut Session, a: &Args) -> CmdResult {
    if a.has("name") && a.has("type") {
        fail!("dgtimer: -name and -type can't be used together");
    }
    let g = &s.graph;
    let serial = g.serial();
    let pick = picked(g, a)?;
    if a.has("overhead") {
        let on = a.bool("overhead");
        with(|t| t.overhead = on);
    }
    if a.bool("reset") {
        with(|t| match &pick {
            Some(ids) => {
                for id in ids {
                    if let Some(e) = t.nodes.get_mut(&(serial, *id)) {
                        *e = NodeTimer { off: e.off, ..Default::default() };
                    }
                }
            }
            None => {
                for e in t.nodes.values_mut() {
                    *e = NodeTimer { off: e.off, ..Default::default() };
                }
                t.since = Some(Instant::now());
            }
        });
    }
    let turn = |on: bool| {
        with(|t| match &pick {
            Some(ids) => {
                for id in ids {
                    t.nodes.entry((serial, *id)).or_default().off = !on;
                }
            }
            None => {
                for e in t.nodes.values_mut() {
                    e.off = !on;
                }
                if on && t.since.is_none() {
                    t.since = Some(Instant::now());
                }
            }
        });
    };
    if !a.bool("query") {
        if a.has("timerOn") && a.get("timerOn").is_none_or(|v| v.as_bool() != Some(false)) {
            turn(true);
            ON.store(true, Ordering::Relaxed);
        }
        if a.bool("timerOff") || a.get("timerOn").is_some_and(|v| v.as_bool() == Some(false)) {
            turn(false);
            // With every node off, the global timing goes off too.
            if pick.is_none() {
                ON.store(false, Ordering::Relaxed);
            }
        }
    }
    if let Some(v) = a.get("trace") {
        let on = v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0);
        let file = a.get("outputFile").and_then(Value::as_str).filter(|f| *f != "stdout" && *f != "MEL").map(str::to_string);
        with(|t| t.trace = if on { file.and_then(|f| std::fs::File::create(f).ok()) } else { None });
        if !a.bool("query") {
            return Ok(json!(0.0));
        }
    }
    if a.bool("query") && (crate::skin::asked(a, "timerOn") && a.flags.len() <= 3) {
        return Ok(json!(active()));
    }
    // Which columns, the sort and the totals.
    let mut shown: Vec<String> = Vec::new();
    for v in strings(a, "show") {
        match v.as_str() {
            "all" => shown = METRICS.iter().map(|m| m.to_string()).collect(),
            "compcb" | "compncb" => shown.push("callback".into()),
            m if METRICS.contains(&m) => shown.push(m.into()),
            m => fail!("dgtimer: -show {m} is not all, callback, compute, dirty, draw, compcb or compncb"),
        }
    }
    let hidden = strings(a, "hide");
    if !hidden.is_empty() {
        if shown.is_empty() {
            shown = METRICS.iter().map(|m| m.to_string()).collect();
        }
        shown.retain(|m| !hidden.contains(m) && !hidden.iter().any(|h| h == "all"));
    }
    if shown.is_empty() {
        shown.push("compute".into());
    }
    shown.dedup();
    // The deprecated -returnCode, for old scripts.
    let (mut metric, mut sort_type) = (a.string("sortMetric", &shown[0]), a.string("sortType", "self"));
    let mut return_type = a.string("returnType", "total");
    match a.get("returnCode").and_then(Value::as_str) {
        Some("nodecount") => sort_type = "none".into(),
        Some("count") => (metric, sort_type) = ("compute".into(), "count".into()),
        Some("selftime") => (metric, sort_type) = ("compute".into(), "self".into()),
        Some(o) => fail!("dgtimer: -returnCode {o} is not nodecount, count or selftime"),
        None => {}
    }
    if !METRICS.contains(&metric.as_str()) {
        fail!("dgtimer: -sortMetric {metric} is not callback, compute, dirty, draw or fetch");
    }
    if !["self", "inclusive", "count", "none"].contains(&sort_type.as_str()) {
        fail!("dgtimer: -sortType {sort_type} is not self, inclusive, count or none");
    }
    if !["total", "all"].contains(&return_type.as_str()) {
        return_type = "total".into();
    }
    let by = if sort_type == "none" { "self" } else { sort_type.as_str() };
    // Rows: (type, name, timer, on).
    let table: Vec<(String, String, NodeTimer)> = with(|t| {
        let mut rows = Vec::new();
        for id in g.nodes() {
            if pick.as_ref().is_some_and(|p| !p.contains(&id)) {
                continue;
            }
            let Some(n) = g.node(id) else { continue };
            let timer = t.nodes.get(&(serial, id)).copied().unwrap_or_default();
            let name = if a.bool("uniqueName") { g.path(id) } else { n.name().to_string() };
            rows.push((n.ty().name.clone(), name, timer));
        }
        rows
    });
    let mut rows: Vec<(String, String, NodeTimer)> = if a.bool("combineType") {
        let mut merged: Vec<(String, String, NodeTimer)> = Vec::new();
        for (ty, _, t) in table {
            match merged.iter_mut().find(|m| m.0 == ty) {
                Some(m) => {
                    m.2.compute += t.compute;
                    m.2.compute_count += t.compute_count;
                    m.2.dirty_count += t.dirty_count;
                    m.2.off &= t.off;
                }
                None => merged.push((ty.clone(), ty, t)),
            }
        }
        merged
    } else {
        table
    };
    rows.sort_by(|x, y| figure(&y.2, &metric, by).total_cmp(&figure(&x.2, &metric, by)).then(x.1.cmp(&y.1)));
    let total: f64 = rows.iter().map(|r| figure(&r.2, &metric, by)).sum();
    // Percentage cutoffs, an absolute threshold and the most rows shown.
    let (lo, hi) = (a.float("rangeLower", 0.0), a.float("rangeUpper", 100.0));
    let threshold = a.float("threshold", f64::MIN);
    let mut kept: Vec<(String, String, NodeTimer)> = rows
        .into_iter()
        .filter(|r| {
            let v = figure(&r.2, &metric, by);
            let pct = if total > 0.0 { v / total * 100.0 } else { 0.0 };
            v >= threshold && (total <= 0.0 || (pct >= lo && pct <= hi))
        })
        .collect();
    if let Some(m) = a.get("maxDisplay").and_then(Value::as_i64) {
        kept.truncate(m.max(0) as usize);
    }
    if let Some(div) = a.get("updateHeatMap").and_then(Value::as_i64) {
        // The heat map's top: the largest figure, which the divisions count up to.
        let _ = div;
        return Ok(json!(kept.iter().map(|r| figure(&r.2, &metric, by)).fold(0.0, f64::max)));
    }
    if !a.bool("query") {
        return Ok(json!(total));
    }
    // The report.
    let mut out = Vec::new();
    let no_header = a.bool("noHeader");
    if !no_header && a.get("name").is_none() {
        let real = with(|t| t.since.map_or(0.0, |s| s.elapsed().as_secs_f64()));
        out.push(format!("// Timing information: total real time {real:.6}"));
        let all_compute: f64 = kept.iter().map(|r| r.2.compute).sum();
        let all_count: u64 = kept.iter().map(|r| r.2.compute_count).sum();
        let dirty_count: u64 = kept.iter().map(|r| r.2.dirty_count).sum();
        out.push(format!("//   compute: {all_compute:.6} self, {all_count} count"));
        out.push(format!("//   dirty propagation: 0.000000 self, {dirty_count} count"));
        out.push("//   callbacks: 0.000000 self, 0 count; drawing: 0.000000 self, 0 count; fetching: 0.000000 self, 0 count".into());
        if with(|t| t.overhead) {
            out.push("//   timing overhead: measured, not taken off the figures".into());
        }
        let mut head = vec!["Rank".to_string(), "ON".to_string()];
        for m in &shown {
            let up = m.to_uppercase();
            head.push(format!("{up}-Self {up}-Inclusive {up}-Count"));
            if *m == metric && sort_type != "none" {
                head.push("Percent Cumulative".into());
            }
        }
        head.push("Type Name PluginName".into());
        out.push(head.join(" "));
    }
    let mut running = 0.0;
    let mut values = Vec::new();
    for (k, (ty, name, t)) in kept.iter().enumerate() {
        let mut line = vec![format!("{}", k + 1), (if t.off { "0" } else { "1" }).to_string()];
        for m in &shown {
            let (sf, inc, cnt) = (figure(t, m, "self"), figure(t, m, "inclusive"), figure(t, m, "count"));
            line.push(format!("{sf:.6} {inc:.6} {cnt}"));
            if *m == metric && sort_type != "none" {
                let v = figure(t, m, by);
                running += v;
                let pct = |x: f64| if total > 0.0 { x / total * 100.0 } else { 0.0 };
                line.push(format!("{:.2}% {:.2}%", pct(v), pct(running)));
            }
        }
        values.push(figure(t, &metric, by));
        line.push(format!("{ty} {name}"));
        out.push(line.join(" "));
    }
    if !no_header && a.get("name").is_none() {
        out.push(format!("// Total: {total:.6}"));
    }
    match a.get("outputFile").and_then(Value::as_str) {
        Some("MEL") => return Ok(json!(out)),
        Some("stdout") | None => {
            for l in &out {
                println!("{l}");
            }
        }
        Some(path) => {
            if let Err(e) = std::fs::write(path, out.join("\n") + "\n") {
                fail!("dgtimer: can't write {path}: {e}");
            }
        }
    }
    if sort_type == "none" && return_type == "total" {
        return Ok(json!(kept.len()));
    }
    if return_type == "all" {
        return Ok(json!(values));
    }
    Ok(json!(total))
}

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

pub(crate) fn register(c: &mut Commands<Session>) {
    use ArgType::{Bool as B, Float as F, Int as I, String as S};
    c.register(
        CommandDef::<Session>::new(
            "dgtimer",
            "Times each node's work: -on starts (all nodes, or -name or -type), -off stops, -reset zeroes, -q reports the timers sorted by \
             -sortMetric and -sortType, to -outputFile (a file, stdout, or MEL for a list). Compute is timed and dirty messages counted; \
             draw, callback and fetch report zero. Returns the total of the sorted column.",
            dgtimer,
        )
        .flag(f("query", "q", B, "Report the timers."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("timerOn", "on", ArgType::Any, "Turn timing on, for all nodes or those -name or -type pick."))
        .flag(f("timerOff", "off", B, "Turn timing off; with every node off, the global timing too."))
        .flag(f("reset", "r", B, "Zero the timers: all and the global ones, or those -name or -type pick."))
        .flag(f("outputFile", "o", S, "Where the report and trace go: a file, stdout, or MEL (the report comes back as a list)."))
        .flag(f("maxDisplay", "m", I, "Report only this many of the most expensive."))
        .flag(f("sortMetric", "sm", S, "The metric sorted on: callback, compute, dirty, draw or fetch (the first shown)."))
        .flag(f("sortType", "st", S, "The timer sorted on: self (the default), inclusive, count, or none (no percentages; returns the node count)."))
        .flag(f("threshold", "th", F, "Leave out rows whose sorted figure is under this."))
        .flag(f("uniqueName", "un", B, "Full DAG paths in the report."))
        .flag(f("combineType", "ct", B, "One row per node type."))
        .flag(f("hide", "hi", ArgType::List, "Show every column but these. Repeat it.").multi(1))
        .flag(f("show", "sh", ArgType::List, "The metric columns shown: all, callback, compute, dirty, draw, compcb or compncb (compute). Repeat it.").multi(1))
        .flag(f("hierarchy", "h", B, "With -name: its descendants too."))
        .flag(f("name", "n", S, "The node to reset, time or report."))
        .flag(f("type", "t", S, "The nodes of this type."))
        .flag(f("noHeader", "nh", B, "Only the rows: no global figures, header or totals."))
        .flag(f("overhead", "oh", ArgType::Any, "Measure the timing's own cost (reported, not taken off)."))
        .flag(f("rangeLower", "rgl", F, "Leave out rows under this percentage of the total (0)."))
        .flag(f("rangeUpper", "rgu", F, "Leave out rows over this percentage of the total (100)."))
        .flag(f("returnCode", "rc", S, "Old scripts' return choice: nodecount, count or selftime."))
        .flag(f("returnType", "rt", S, "total (the column's total) or all (each row's figure)."))
        .flag(f("trace", "tr", ArgType::Any, "Write a begin and end line for each operation to -outputFile while timing."))
        .flag(f("updateHeatMap", "uhm", I, "Rebuild the heat map with this many divisions; returns the largest figure."))
        .undoable(false),
    );
}
