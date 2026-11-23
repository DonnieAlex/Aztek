//! channelBox: the channel boxes scripts make, and the application's own, mainChannelBox. The model in scriptui holds
//! each box's flags; this module answers what depends on the scene (the objects in each section and the picked
//! channels), keeps the colour rules, runs -execute and -rebuildCommand, and dresses the snapshot the shell draws from.

use std::cell::RefCell;
use std::collections::HashMap;

use aztek_ops::{Args, CmdResult, CommandError};
use aztek_scene::plug::NodeId;
use aztek_scene::Session;
use serde_json::{json, Map, Value};

use crate::scriptui::{control_full_path, control_kind, control_prop, control_props, controls_of, wildcard_match};

/// The application's own channel box, under the reference's name.
pub const MAIN: &str = "mainChannelBox";

/// The box's own flags, past the ones every control has.
pub(crate) const FLAGS: &str = "attrBgColor:bc:a attrColor:ac:a attrFilter:af:a attrRegex:ar:a attributeEditorMode:aem:b containerAtTop:cat:b enableLabelSelection:els:b execute:exe:a2 fieldWidth:fw:a fixedAttrList:fal:l historyObjectList:hol:b hyperbolic:hyp:b inputs:in:b labelWidth:lw:a longNames:ln:b mainListConnection:mlc:a mainObjectList:mol:b maxHeight:mh:a maxWidth:mw:a niceNames:nn:b nodeRegex:nr:a outputObjectList:ool:b outputs:out:b precision:pre:a rebuildCommand:rbc:a select:s:a1 selectedHistoryAttributes:sha:b selectedMainAttributes:sma:b selectedOutputAttributes:soa:b selectedShapeAttributes:ssa:b shapeObjectList:sol:b shapes:shp:b showNamespace:sn:b showTransforms:st:b speed:spd:a takeFocus:tf:b ufeFixedAttrList:ual:l update:u:b useManips:mnp:a";

/// The queries answered from the scene rather than the box's stored flags.
const LIVE: &[&str] = &["mainObjectList", "shapeObjectList", "historyObjectList", "outputObjectList", "inputs", "outputs", "shapes", "selectedMainAttributes", "selectedShapeAttributes", "selectedHistoryAttributes", "selectedOutputAttributes"];

/// The channels picked, as the shell has them: attribute names (the object's and its shapes'), and whole plugs for the
/// channels of input and output nodes. `serial` moves when a script picks, so the shell takes the script's picks.
#[derive(Default, Clone)]
struct Picks {
    attrs: Vec<String>,
    plugs: Vec<String>,
    serial: u64,
}

/// One -attrColor call: attribute and node patterns, text and background colours.
#[derive(Clone)]
struct Rule {
    attr: String,
    node: String,
    fg: Value,
    bg: Value,
}

#[derive(Default)]
struct BoxState {
    rules: Vec<Rule>,
    last_lead: Option<String>,
    rebuild_due: bool,
}

thread_local! {
    static PICKS: RefCell<Picks> = RefCell::new(Picks::default());
    static BOXES: RefCell<HashMap<String, BoxState>> = RefCell::new(HashMap::new());
}

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        o => o.to_string(),
    }
}

fn flat(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a.iter().flat_map(flat).collect(),
        o => vec![o.clone()],
    }
}

fn truth(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || matches!(v.as_str(), Some("true" | "on" | "yes" | "1")))
}

fn with_box<R>(path: &str, f: impl FnOnce(&mut BoxState) -> R) -> R {
    BOXES.with(|b| f(b.borrow_mut().entry(path.to_string()).or_default()))
}

/// A channel box's full path, by any of its names.
fn box_path(name: &str) -> Option<String> {
    (control_kind(name).as_deref() == Some("channelBox")).then(|| control_full_path(name)).flatten()
}

fn prop(path: &str, key: &str) -> Option<Value> {
    control_prop(path, key).filter(|v| !v.is_null())
}

// ---------------------------------------------------------------------------------------------------------------------
// What each section shows

/// The objects a box follows: its -mainListConnection's, or the selection.
fn objects(s: &Session, path: &str) -> Vec<NodeId> {
    let g = &s.graph;
    let conn = prop(path, "mainListConnection").map(|v| text(&v)).filter(|c| !c.is_empty());
    match conn.and_then(|c| crate::script_objects::connection_members(&c)).flatten() {
        Some(list) => list.iter().filter_map(|n| g.find(n).ok().flatten()).collect(),
        None => g.selection().to_vec(),
    }
}

fn type_of(s: &Session, id: NodeId) -> String {
    s.graph.node(id).map(|n| n.ty().name.clone()).unwrap_or_default()
}

fn shapes_of(s: &Session, id: NodeId) -> Vec<NodeId> {
    let g = &s.graph;
    let Some(n) = g.node(id) else { return Vec::new() };
    if n.ty().is_a("transform") {
        aztek_scene::dag::shapes(g, id, false)
    } else if aztek_scene::dag::is_shape(g, id) {
        vec![id]
    } else {
        Vec::new()
    }
}

/// The INPUTS: the construction history of the object's meshes, as the box lists it.
fn inputs_of(s: &Session, id: NodeId) -> Vec<NodeId> {
    let g = &s.graph;
    let mut out = Vec::new();
    for sh in shapes_of(s, id) {
        if g.node(sh).is_some_and(|n| n.ty().is_a("mesh")) {
            for h in aztek_scene::history::history_of(g, sh) {
                if g.node(h).is_some_and(|n| !n.ty().dag) && !out.contains(&h) {
                    out.push(h);
                }
            }
        }
    }
    out
}

/// The OUTPUTS: the nodes the object and its shapes drive, sets, layers and editor bookkeeping aside.
fn outputs_of(s: &Session, id: NodeId) -> Vec<NodeId> {
    let g = &s.graph;
    let mut from = shapes_of(s, id);
    from.push(id);
    let inputs = inputs_of(s, id);
    let mut out = Vec::new();
    for (dst, src) in g.connections() {
        if !from.contains(&src.node) || from.contains(&dst.node) || out.contains(&dst.node) || inputs.contains(&dst.node) {
            continue;
        }
        let Some(n) = g.node(dst.node) else { continue };
        let ty = n.ty();
        if ty.dag || ty.is_a("objectSet") || ["displayLayer", "renderLayer", "nodeGraphEditorInfo", "hyperLayout", "groupId", "groupParts", "displayLayerManager", "renderLayerManager"].iter().any(|t| ty.is_a(t)) {
            continue;
        }
        out.push(dst.node);
    }
    out
}

struct Sections {
    lead: Option<NodeId>,
    main: Vec<NodeId>,
    shapes: Vec<NodeId>,
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}

fn sections(s: &Session, path: &str) -> Sections {
    let objs = objects(s, path);
    let Some(&lead) = objs.last() else { return Sections { lead: None, main: Vec::new(), shapes: Vec::new(), inputs: Vec::new(), outputs: Vec::new() } };
    let ty = type_of(s, lead);
    let main: Vec<NodeId> = objs.iter().copied().filter(|&o| type_of(s, o) == ty).collect();
    Sections { lead: Some(lead), main, shapes: shapes_of(s, lead), inputs: inputs_of(s, lead), outputs: outputs_of(s, lead) }
}

fn names(s: &Session, ids: &[NodeId]) -> Value {
    json!(ids.iter().map(|&i| s.graph.path(i)).collect::<Vec<_>>())
}

/// Across the main objects, the nodes of `each` whose types are among the lead's own.
fn matching(s: &Session, sec: &Sections, of_lead: &[NodeId], each: impl Fn(&Session, NodeId) -> Vec<NodeId>) -> Vec<NodeId> {
    let types: Vec<String> = of_lead.iter().map(|&i| type_of(s, i)).collect();
    let mut out = Vec::new();
    for &o in &sec.main {
        for n in each(s, o) {
            if types.contains(&type_of(s, n)) && !out.contains(&n) {
                out.push(n);
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// Picked channels

fn attr_query(s: &mut Session, node: &str, attr: &str, flag: &str) -> Value {
    s.run("attributeQuery", json!({"args": [attr], "node": node, flag: true})).unwrap_or(Value::Null)
}

fn has_attr(s: &mut Session, node: &str, attr: &str) -> bool {
    truth(&attr_query(s, node, attr, "exists"))
}

fn short_of(s: &mut Session, node: &str, attr: &str) -> String {
    let v = attr_query(s, node, attr, "shortName");
    let t = text(&v);
    if t.is_empty() {
        attr.to_string()
    } else {
        t
    }
}

fn long_of(s: &mut Session, node: &str, attr: &str) -> String {
    let t = text(&attr_query(s, node, attr, "longName"));
    if t.is_empty() {
        attr.to_string()
    } else {
        t
    }
}

/// The picked channels of one section, by short name.
fn selected(s: &mut Session, path: &str, which: &str) -> Vec<String> {
    let sec = sections(s, path);
    let picks = PICKS.with(|p| p.borrow().clone());
    let Some(lead) = sec.lead else { return Vec::new() };
    let lead_path = s.graph.path(lead);
    let from_plugs: Vec<String> = picks.plugs.iter().filter_map(|p| p.rsplit_once('.').map(|x| x.1.to_string())).collect();
    let mut out = Vec::new();
    match which {
        "main" => {
            for a in picks.attrs.iter().filter(|a| !from_plugs.contains(a)) {
                if has_attr(s, &lead_path, a) {
                    out.push(short_of(s, &lead_path, a));
                }
            }
        }
        "shape" => {
            let shapes: Vec<String> = sec.shapes.iter().map(|&i| s.graph.path(i)).collect();
            for a in picks.attrs.iter().filter(|a| !from_plugs.contains(a)) {
                if has_attr(s, &lead_path, a) {
                    continue;
                }
                if let Some(sh) = shapes.iter().find(|sh| has_attr(s, sh, a)).cloned() {
                    out.push(short_of(s, &sh, a));
                }
            }
        }
        _ => {
            let nodes: Vec<String> = if which == "history" { &sec.inputs } else { &sec.outputs }.iter().map(|&i| s.graph.path(i)).collect();
            for p in &picks.plugs {
                let Some((n, a)) = p.rsplit_once('.') else { continue };
                let n = s.graph.find(n).ok().flatten().map(|i| s.graph.path(i)).unwrap_or_default();
                if nodes.contains(&n) {
                    out.push(short_of(s, &n, a));
                }
            }
        }
    }
    let mut seen = Vec::new();
    out.retain(|x| if seen.contains(x) { false } else { seen.push(x.clone()); true });
    out
}

/// What the shell picked: attribute names, and the plugs of input and output nodes.
pub fn shell_picks(arg: &str) {
    let v: Value = serde_json::from_str(arg).unwrap_or(Value::Null);
    let list = |k: &str| v[k].as_array().map(|a| a.iter().map(text).collect::<Vec<_>>()).unwrap_or_default();
    PICKS.with(|p| {
        let mut p = p.borrow_mut();
        p.attrs = list("attrs");
        p.plugs = list("plugs");
    });
}

/// -select: picks the named plugs, as a click on each would, among what the box shows. A node alone picks nothing.
fn select(s: &mut Session, path: &str, items: &[String]) {
    let sec = sections(s, path);
    let shown_nodes: Vec<NodeId> = sec.main.iter().chain(&sec.shapes).copied().collect();
    let side: Vec<NodeId> = sec.inputs.iter().chain(&sec.outputs).copied().collect();
    let (mut attrs, mut plugs) = (Vec::new(), Vec::new());
    for item in items {
        let Some((n, a)) = item.rsplit_once('.') else { continue };
        let Ok(Some(id)) = s.graph.find(n) else { continue };
        let node = s.graph.path(id);
        if !has_attr(s, &node, a) {
            continue;
        }
        let long = long_of(s, &node, a);
        if shown_nodes.contains(&id) {
            attrs.push(long);
        } else if side.contains(&id) {
            plugs.push(format!("{node}.{long}"));
            attrs.push(long);
        }
    }
    PICKS.with(|p| {
        let mut p = p.borrow_mut();
        p.attrs = attrs;
        p.plugs = plugs;
        p.serial += 1;
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// The command

fn asked_of(a: &Args) -> Option<String> {
    a.get("queried").and_then(Value::as_array).and_then(|q| q.first()).and_then(Value::as_str).map(str::to_string).or_else(|| a.flags.keys().find(|k| !matches!(k.as_str(), "query" | "queried")).cloned())
}

fn live_query(s: &mut Session, path: &str, asked: &str) -> Value {
    if asked.starts_with("selected") {
        let which = match asked {
            "selectedMainAttributes" => "main",
            "selectedShapeAttributes" => "shape",
            "selectedHistoryAttributes" => "history",
            _ => "output",
        };
        return json!(selected(s, path, which));
    }
    let sec = sections(s, path);
    match asked {
        "mainObjectList" => names(s, &sec.main),
        "shapes" => names(s, &sec.shapes),
        "inputs" => names(s, &sec.inputs),
        "outputs" => names(s, &sec.outputs),
        "shapeObjectList" => names(s, &matching(s, &sec, &sec.shapes, shapes_of)),
        "historyObjectList" => names(s, &matching(s, &sec, &sec.inputs, inputs_of)),
        "outputObjectList" => names(s, &matching(s, &sec, &sec.outputs, outputs_of)),
        _ => Value::Null,
    }
}

fn lead_name(s: &Session, path: &str) -> Option<String> {
    objects(s, path).last().map(|&i| s.graph.path(i))
}

/// The channelBox command past the generic control: live queries, colour rules, -select, -update and -rebuildCommand.
pub(crate) fn command(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        if let (Some(asked), Some(name)) = (asked_of(a), a.pos_str(0)) {
            if LIVE.contains(&asked.as_str()) {
                let Some(path) = box_path(name) else { return Err(CommandError::new(format!("Object '{name}' not found."))) };
                return Ok(live_query(s, &path, &asked));
            }
        }
        return crate::scriptui::command_for("channelBox", a);
    }
    let mut plain = a.clone();
    for k in ["execute", "select", "update"] {
        plain.flags.remove(k);
    }
    if a.has("niceNames") && a.bool("niceNames") {
        plain.flags.insert("longNames".into(), json!(false));
    }
    let r = crate::scriptui::command_for("channelBox", &plain)?;
    if a.bool("exists") && !a.bool("edit") {
        return Ok(r);
    }
    let Some(path) = r.as_str().and_then(box_path) else { return Ok(r) };
    if a.has("attrColor") || a.has("attrBgColor") {
        let pat = |k: &str| a.get(k).map(text).filter(|t| !t.is_empty()).unwrap_or_else(|| "*".to_string());
        let rule = Rule { attr: pat("attrRegex"), node: pat("nodeRegex"), fg: a.get("attrColor").cloned().unwrap_or(Value::Null), bg: a.get("attrBgColor").cloned().unwrap_or(Value::Null) };
        with_box(&path, |b| b.rules.push(rule));
    }
    if let Some(items) = a.get("select") {
        let list: Vec<String> = flat(items).iter().map(text).collect();
        select(s, &path, &list);
    }
    if a.bool("update") {
        with_box(&path, |b| b.rebuild_due = true);
    }
    if a.has("rebuildCommand") || !a.bool("edit") {
        let lead = lead_name(s, &path);
        with_box(&path, |b| b.last_lead = lead);
    }
    Ok(r)
}

/// The rebuild scripts due after a command: of each box whose object changed, or that -update asked to rebuild.
pub(crate) fn rebuild_callbacks(s: &mut Session) -> Vec<Value> {
    let boxes = controls_of("channelBox");
    let mut out = Vec::new();
    for (path, props) in boxes {
        let lead = lead_name(s, &path);
        let due = with_box(&path, |b| {
            let due = b.rebuild_due || b.last_lead != lead;
            b.rebuild_due = false;
            b.last_lead = lead.clone();
            due
        });
        if due {
            if let Some(cb) = props.get("rebuildCommand").filter(|v| !v.is_null() && v.as_str() != Some("")) {
                out.push(cb.clone());
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// -execute

/// The commands -execute runs: one per cell (or per picked cell), on every object a change to that cell would reach, with
/// #A the attribute, #N the node's name and #P its full path.
fn execute_commands(s: &mut Session, path: &str, script: &str, only_picked: bool) -> Vec<String> {
    let sec = sections(s, path);
    let Some(lead) = sec.lead else { return Vec::new() };
    let mut cells: Vec<(NodeId, String)> = Vec::new();
    let main_attrs: Vec<String> = if only_picked {
        Vec::new()
    } else {
        displayed(s, path, lead).into_iter().map(|c| text(&c["attr"])).collect()
    };
    let shape_attrs: Vec<(NodeId, Vec<String>)> = if only_picked { Vec::new() } else { sec.shapes.iter().map(|&sh| (sh, displayed(s, path, sh).into_iter().map(|c| text(&c["attr"])).collect())).collect() };
    if only_picked {
        let lead_path = s.graph.path(lead);
        let mains = selected(s, path, "main");
        for a in mains {
            let long = long_of(s, &lead_path, &a);
            for &o in &sec.main {
                cells.push((o, long.clone()));
            }
        }
        let shapes = selected(s, path, "shape");
        let others = matching(s, &sec, &sec.shapes, shapes_of);
        for a in shapes {
            for &o in &others {
                let p = s.graph.path(o);
                if has_attr(s, &p, &a) {
                    let long = long_of(s, &p, &a);
                    cells.push((o, long));
                }
            }
        }
        let picks = PICKS.with(|p| p.borrow().plugs.clone());
        for plug in picks {
            let Some((n, a)) = plug.rsplit_once('.') else { continue };
            if let Ok(Some(id)) = s.graph.find(n) {
                cells.push((id, a.to_string()));
            }
        }
    } else {
        for a in main_attrs {
            for &o in &sec.main {
                cells.push((o, a.clone()));
            }
        }
        for (sh, attrs) in shape_attrs {
            for a in attrs {
                cells.push((sh, a));
            }
        }
    }
    cells
        .into_iter()
        .filter_map(|(id, attr)| {
            let n = s.graph.node(id)?;
            Some(script.replace("#A", &attr).replace("#N", n.name()).replace("#P", &s.graph.full_path(id)))
        })
        .collect()
}

/// The keys of a call by long name, whichever form it came in.
fn long_keys(s: &Session, call: &Value) -> Map<String, Value> {
    let mut out = Map::new();
    let def = s.commands.find("channelBox");
    for (k, v) in call.as_object().into_iter().flatten() {
        let long = def.and_then(|d| d.flags.iter().find(|f| f.name == *k || f.short_name == *k)).map(|f| f.name.clone()).unwrap_or_else(|| k.clone());
        out.insert(long, v.clone());
    }
    out
}

/// channelBox -e -execute: the commands run now, through MEL, before the rest of the edit.
pub(crate) fn intercept(core: &mut crate::core::Core, name: &str, call: &Value) -> Option<Result<Value, String>> {
    if name != "channelBox" {
        return None;
    }
    let o = long_keys(&core.s, call);
    let exe = o.get("execute").filter(|v| !v.is_null())?;
    let parts = flat(exe);
    let script = parts.first().map(text).unwrap_or_default();
    let only = parts.get(1).is_some_and(truth);
    let box_name = o.get("args").and_then(|a| flat(a).first().map(text)).unwrap_or_else(|| MAIN.to_string());
    let Some(path) = box_path(&box_name) else { return Some(Err(format!("channelBox: Object '{box_name}' not found."))) };
    for line in execute_commands(&mut core.s, &path, &script, only) {
        if let Err(e) = core.mel_checked(&line) {
            return Some(Err(e));
        }
    }
    None
}

// ---------------------------------------------------------------------------------------------------------------------
// What the shell draws

/// A node's channels as the box shows them: -fixedAttrList's in its order, through -attrFilter.
fn displayed(s: &mut Session, path: &str, node: NodeId) -> Vec<Value> {
    let all_inputs = s.graph.node(node).is_some_and(|n| !n.ty().dag);
    let chans = crate::query::channels(&s.graph, node, all_inputs);
    filter_channels(s, path, node, chans)
}

fn filter_channels(s: &mut Session, path: &str, node: NodeId, mut chans: Vec<Value>) -> Vec<Value> {
    let fixed: Vec<String> = prop(path, "fixedAttrList").map(|v| flat(&v).iter().map(text).filter(|t| !t.is_empty()).collect()).unwrap_or_default();
    if !fixed.is_empty() {
        let mut out = Vec::new();
        for f in &fixed {
            if let Some(c) = chans.iter().find(|c| text(&c["attr"]) == *f || text(&c["short"]) == *f) {
                out.push(c.clone());
            }
        }
        chans = out;
    }
    if let Some(filter) = prop(path, "attrFilter").map(|v| text(&v)).filter(|f| !f.is_empty() && f != "0") {
        let node_path = s.graph.path(node);
        if crate::script_objects::is_attr_filter(&filter) {
            let passed: Vec<String> = s.run("lsThroughFilter", json!({"args": [filter], "item": [node_path]})).ok().map(|v| flat(&v).iter().map(text).collect()).unwrap_or_default();
            let leaves: Vec<String> = passed.iter().filter_map(|p| p.rsplit_once('.').map(|x| x.1.to_string())).collect();
            chans.retain(|c| leaves.contains(&text(&c["attr"])) || leaves.contains(&text(&c["short"])));
        }
    }
    chans
}

fn colour(path: &str, node_name: &str, attr: &str, short: &str, c: &mut Value) {
    let rules = with_box(path, |b| b.rules.clone());
    // Nodes in namespaces match as if they were in none.
    let bare = node_name.rsplit('|').next().unwrap_or(node_name);
    let bare = bare.rsplit(':').next().unwrap_or(bare);
    for r in rules {
        if wildcard_match(&r.node, bare) && (wildcard_match(&r.attr, attr) || wildcard_match(&r.attr, short)) {
            if !r.fg.is_null() {
                c["fg"] = r.fg.clone();
            }
            if !r.bg.is_null() {
                c["bg"] = r.bg.clone();
            }
        }
    }
}

fn dress(s: &mut Session, path: &str, node_path: &str, chans: &Value) -> Value {
    let Ok(Some(id)) = s.graph.find(node_path) else { return chans.clone() };
    let list = chans.as_array().cloned().unwrap_or_default();
    let mut list = filter_channels(s, path, id, list);
    let name = s.graph.node(id).map(|n| n.name().to_string()).unwrap_or_default();
    for c in &mut list {
        let (a, sh) = (text(&c["attr"]), text(&c["short"]));
        colour(path, &name, &a, &sh, c);
    }
    Value::Array(list)
}

/// The settings the box was given, for the shell to draw with; what wasn't given stays the shell's own.
fn options(path: &str) -> Value {
    let p = control_props(path).unwrap_or_default();
    let get = |k: &str| p.get(k).filter(|v| !v.is_null()).cloned();
    let mut o = Map::new();
    if get("niceNames").is_some() || get("longNames").is_some() {
        let nice = get("niceNames").is_none_or(|v| truth(&v));
        let long = get("longNames").is_some_and(|v| truth(&v));
        o.insert("names".into(), json!(if nice { 0 } else if long { 1 } else { 2 }));
    }
    if let Some(v) = get("showNamespace") {
        o.insert("namespace".into(), json!(truth(&v)));
    }
    if let Some(v) = get("precision") {
        o.insert("precision".into(), json!(v.as_f64().unwrap_or(3.0) as i64));
    }
    if let Some(v) = get("hyperbolic") {
        o.insert("hyperbolic".into(), json!(truth(&v)));
    }
    if let Some(v) = get("speed") {
        let x = v.as_f64().unwrap_or(1.0);
        o.insert("speed".into(), json!(if x < 0.05 { 0 } else if x < 0.5 { 1 } else if x < 5.0 { 2 } else { 3 }));
        o.insert("speedValue".into(), json!(x));
    }
    if let Some(v) = get("useManips") {
        o.insert("manipulators".into(), json!(match text(&v).as_str() { "none" => 0, "invisible" => 1, _ => 2 }));
    }
    for k in ["labelWidth", "fieldWidth", "maxWidth", "maxHeight"] {
        if let Some(v) = get(k) {
            o.insert(k.into(), v);
        }
    }
    Value::Object(o)
}

/// The snapshot the shell draws a box from: the box's object (its -mainListConnection's last, or the selection's), its
/// channels through -fixedAttrList and -attrFilter and coloured by the -attrColor rules, its OUTPUTS, the box's settings,
/// and the picks a script made.
pub fn decorate(s: &mut Session, name: &str, snapshot: Value) -> Value {
    let name = if name.is_empty() { MAIN } else { name };
    let Some(path) = box_path(name) else { return snapshot };
    let mut v = snapshot;
    let mut objs: Vec<NodeId> = objects(s, &path);
    if prop(&path, "mainListConnection").is_some_and(|c| !text(&c).is_empty()) {
        v = match objs.last() {
            Some(&lead) => crate::query::channel_box_of(&s.graph, lead, objs.len()),
            None => Value::Null,
        };
    }
    if v.is_null() {
        return v;
    }
    if let Some(p) = v["path"].as_str().map(str::to_string) {
        let c = dress(s, &path, &p, &v["channels"]);
        v["channels"] = c;
    }
    for key in ["shapes", "inputs"] {
        let n = v[key].as_array().map(Vec::len).unwrap_or(0);
        for i in 0..n {
            let p = text(&v[key][i]["path"]);
            let c = dress(s, &path, &p, &v[key][i]["channels"]);
            v[key][i]["channels"] = c;
        }
    }
    // OUTPUTS, for the object shown.
    if let Some(Ok(Some(lead))) = v["path"].as_str().map(|p| s.graph.find(p)) {
        let outs: Vec<Value> = outputs_of(s, lead)
            .into_iter()
            .filter_map(|o| {
                let n = s.graph.node(o)?;
                Some(json!({"name": n.name(), "path": s.graph.path(o), "type": n.ty().name, "channels": crate::query::channels(&s.graph, o, true)}))
            })
            .collect();
        let mut dressed = Vec::new();
        for mut o in outs {
            let p = text(&o["path"]);
            o["channels"] = dress(s, &path, &p, &o["channels"]);
            dressed.push(o);
        }
        v["outputs"] = Value::Array(dressed);
    }
    // Every selected object, so the shell can stack them as the reference's Channel Box does. The lead's fields
    // above stay for scripts and for a single selection.
    if v.is_object() {
        let mut all = Vec::new();
        for &id in &objs {
            let mut snap = crate::query::channel_box_of(&s.graph, id, objs.len());
            if snap.is_null() {
                continue;
            }
            let p = text(&snap["path"]);
            let c = dress(s, &path, &p, &snap["channels"]);
            snap["channels"] = c;
            for key in ["shapes", "inputs"] {
                let n = snap[key].as_array().map(Vec::len).unwrap_or(0);
                for i in 0..n {
                    let pp = text(&snap[key][i]["path"]);
                    let c = dress(s, &path, &pp, &snap[key][i]["channels"]);
                    snap[key][i]["channels"] = c;
                }
            }
            if let Ok(Some(lead)) = s.graph.find(&p) {
                let outs: Vec<Value> = outputs_of(s, lead)
                    .into_iter()
                    .filter_map(|o| {
                        let n = s.graph.node(o)?;
                        Some(json!({"name": n.name(), "path": s.graph.path(o), "type": n.ty().name, "channels": crate::query::channels(&s.graph, o, true)}))
                    })
                    .collect();
                let mut dressed = Vec::new();
                for mut o in outs {
                    let op = text(&o["path"]);
                    o["channels"] = dress(s, &path, &op, &o["channels"]);
                    dressed.push(o);
                }
                snap["outputs"] = Value::Array(dressed);
            }
            all.push(snap);
        }
        v["objects"] = Value::Array(all);
    }
    if prop(&path, "showTransforms").is_some_and(|x| truth(&x)) {
        v["shapes"] = json!([]);
        v["inputs"] = json!([]);
        v["outputs"] = json!([]);
    }
    v["options"] = options(&path);
    let picks = PICKS.with(|p| p.borrow().clone());
    v["picks"] = json!({"serial": picks.serial, "attrs": picks.attrs, "plugs": picks.plugs});
    v
}

/// The defaults a box answers before it is given a value.
pub(crate) fn default_value(key: &str) -> Option<Value> {
    Some(match key {
        "precision" => json!(3),
        "niceNames" | "showNamespace" => json!(true),
        "longNames" | "hyperbolic" | "showTransforms" | "attributeEditorMode" | "enableLabelSelection" | "containerAtTop" => json!(false),
        "speed" => json!(1.0),
        "useManips" => json!("standard"),
        "mainListConnection" | "attrFilter" | "rebuildCommand" => json!(""),
        "attrRegex" | "nodeRegex" => json!("*"),
        "fixedAttrList" | "ufeFixedAttrList" => json!([]),
        "labelWidth" => json!(110),
        "fieldWidth" => json!(65),
        "maxWidth" | "maxHeight" => json!(0),
        _ => return None,
    })
}
