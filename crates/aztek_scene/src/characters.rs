//! Character sets, clips and poses. A `character` node lists channels. `clip` moves their keys into a source clip kept
//! on a `clipLibrary` and schedules it on a `clipScheduler`; each member channel then reads the schedule through a small
//! node of its own (`clipChannel`). `pose` keeps values on the library.
//!
//! The reference routes these through the character node's value arrays and one scheduler output per channel. Array elements
//! can't drive plugs in this graph, hence the per-channel node.

use std::any::Any;
use std::sync::Arc;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, Commands};
use serde_json::{json, Map, Value};

use crate::anim::{self, f, query_flags, queried, Cmd, Keys};
use crate::graph::Graph;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::rig::{data_attr, json_of, set_json};
use crate::session::Session;
use crate::transform::DagNode;

// character: which node each member is on comes from a connection, so it follows renames; which attr from the list.
const MEMBER_OBJECTS: usize = 0;
const MEMBER_ATTRS: usize = 1;
// The links run from the library and scheduler to the character: they drive its members, and the members feed it.
const LINK_LIBRARY: usize = 2;
const LINK_SCHEDULER: usize = 3;
/// Sub-characters: each one's message, connected in.
const SUB_CHARACTERS: usize = 4;
/// The members marked as offset objects (character -addOffsetObject), by name.
const OFFSET_OBJECTS: usize = 5;
/// character -text.
const ANNOTATION: usize = 6;
/// The characterOffset node's message, when the character has a root.
const OFFSET_LINK: usize = 7;

const LIB_SOURCES: usize = 0;
const LIB_POSES: usize = 1;

const SCHED_CLIPS: usize = 0;

const CH_SCHEDULE: usize = 0;
const CH_SOURCES: usize = 1;
const CH_MEMBER: usize = 2;
const CH_BASE: usize = 3;
const CH_REST: usize = 4;
const CH_OUTPUT: usize = 5;

/// A clip's value for a member at a frame, with its scale and cycles.
fn clip_value(clip: &Value, keys: &Keys, t: f64) -> f64 {
    let n = |k: &str, d: f64| clip.get(k).and_then(Value::as_f64).unwrap_or(d);
    let (start, ss, se, scale) = (n("start", 1.0), n("sourceStart", 1.0), n("sourceEnd", 1.0), n("scale", 1.0).abs().max(1e-6));
    let (pre, post) = (n("preCycle", 0.0), n("postCycle", 1.0).max(1.0));
    let span = (se - ss).max(1e-9);
    let local = (t - start) / scale;
    let wrapped = if span <= 1e-9 {
        0.0
    } else if local < 0.0 {
        if pre > 0.0 {
            local.max(-pre * span).rem_euclid(span)
        } else {
            0.0
        }
    } else if local >= span * post {
        // Holds its last value past the end of its last cycle.
        span
    } else if post > 1.0 {
        local.rem_euclid(span)
    } else {
        local.min(span)
    };
    keys.eval_inside(ss + wrapped, keys.weighted) * n("weight", 1.0)
}

fn clip_span(clip: &Value) -> (f64, f64) {
    let n = |k: &str, d: f64| clip.get(k).and_then(Value::as_f64).unwrap_or(d);
    let (start, scale, span) = (n("start", 1.0), n("scale", 1.0).abs(), n("sourceEnd", 1.0) - n("sourceStart", 1.0));
    (start - n("preCycle", 0.0) * span * scale, start + span * scale * n("postCycle", 1.0).max(1.0))
}

fn usable(clip: &Value) -> bool {
    clip.get("enabled").and_then(Value::as_bool).unwrap_or(true) && !clip.get("mute").and_then(Value::as_bool).unwrap_or(false)
}

struct ClipChannel;

impl NodeBehavior for ClipChannel {
    fn compute(&mut self, ctx: &EvalContext, d: &mut NodeData) {
        let t = ctx.time * anim::fps();
        let r = t.round();
        let t = if (t - r).abs() < 1e-6 { r } else { t };
        let schedule = json_of(d, CH_SCHEDULE);
        let sources = json_of(d, CH_SOURCES);
        let member = d.int(CH_MEMBER).to_string();
        let clips: Vec<&Value> = schedule.as_array().map(|a| a.iter().filter(|c| usable(c)).collect()).unwrap_or_default();
        let keys_for = |c: &Value| -> Option<Keys> {
            let src = c.get("source").and_then(Value::as_str)?;
            sources.get(src)?.get("curves")?.get(&member).map(Keys::from_json)
        };
        let soloed = clips.iter().any(|c| c.get("solo").and_then(Value::as_bool).unwrap_or(false));
        let playing: Vec<&&Value> = clips.iter().filter(|c| !soloed || c.get("solo").and_then(Value::as_bool).unwrap_or(false)).collect();
        // Absolute clips: the one playing on the highest track; between clips the last one to have finished holds, and
        // before them all the first. Relative clips add what they have moved from their start.
        let track = |c: &Value| c.get("track").and_then(Value::as_i64).unwrap_or(0);
        let relative = |c: &Value| c.get("relative").and_then(Value::as_bool).unwrap_or(false);
        let abs: Vec<&&&Value> = playing.iter().filter(|c| !relative(c)).collect();
        let active = abs.iter().filter(|c| {
            let (a, b) = clip_span(c);
            t >= a - 1e-9 && t <= b + 1e-9
        });
        let chosen = active
            .max_by(|x, y| (track(x), clip_span(x).0).partial_cmp(&(track(y), clip_span(y).0)).unwrap_or(std::cmp::Ordering::Equal))
            .or_else(|| abs.iter().filter(|c| clip_span(c).1 < t).max_by(|x, y| clip_span(x).1.total_cmp(&clip_span(y).1)))
            .or_else(|| abs.iter().min_by(|x, y| clip_span(x).0.total_cmp(&clip_span(y).0)));
        let mut value = match chosen.and_then(|c| keys_for(c).map(|k| (c, k))) {
            Some((c, k)) if !k.keys.is_empty() => Some(clip_value(c, &k, t)),
            _ => None,
        };
        for c in playing.iter().filter(|c| relative(c)) {
            let (a, b) = clip_span(c);
            if t < a {
                continue;
            }
            if let Some(k) = keys_for(c).filter(|k| !k.keys.is_empty()) {
                let start = c.get("start").and_then(Value::as_f64).unwrap_or(a);
                value = Some(value.unwrap_or_else(|| d.float(CH_BASE).max(d.float(CH_REST))) + clip_value(c, &k, t.min(b)) - clip_value(c, &k, start));
            }
        }
        // Keys left outside a clip carry on where no clip has anything; otherwise the value it had.
        let out = value.unwrap_or_else(|| if d.bool(CH_REST + 2) { d.float(CH_BASE) } else { d.float(CH_REST) });
        d.set_output(CH_OUTPUT, PlugValue::Float(out));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct PlainNode;

impl NodeBehavior for PlainNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn is_character(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.ty().name == "character")
}

pub fn is_clip_channel(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.ty().name == "clipChannel")
}

fn member_list(g: &Graph, c: NodeId) -> Vec<(i32, String, i32)> {
    let Some(n) = g.node(c) else { return Vec::new() };
    json_of(n.data(), MEMBER_ATTRS)
        .as_array()
        .map(|v| v.iter().filter_map(|e| Some((e.get(0)?.as_i64()? as i32, e.get(1)?.as_str()?.to_string(), e.get(2)?.as_i64()? as i32))).collect())
        .unwrap_or_default()
}

fn member_plug(g: &Graph, c: NodeId, m: &(i32, String, i32)) -> Option<PlugRef> {
    let owner = g.source(&PlugRef::element(c, MEMBER_OBJECTS as i32, m.0))?.node;
    let at = g.node(owner)?.ty().attr_index(&m.1)? as i32;
    Some(if m.2 >= 0 { PlugRef::component(owner, at, m.2) } else { PlugRef::new(owner, at) })
}

/// The channels of a character. Members whose nodes have been deleted drop out.
pub fn member_plugs(g: &Graph, c: NodeId) -> Vec<PlugRef> {
    member_list(g, c).iter().filter_map(|m| member_plug(g, c, m)).collect()
}

/// Whether a plug is what an -at flag names: tx, translateX or translate.
pub fn plug_matches(g: &Graph, p: &PlugRef, name: &str) -> bool {
    let a = g.attr(p);
    if p.component >= 0 && a.ty == PlugType::Vec3 {
        let c = p.component as usize;
        return a.child_names[c] == name || a.child_short[c] == name || a.name == name || a.short_name == name;
    }
    a.name == name || a.short_name == name
}

fn character_arg(g: &Graph, name: &str, cmd: &str) -> CmdResult<NodeId> {
    match g.find(name)? {
        Some(id) if is_character(g, id) => Ok(id),
        _ => fail!("{cmd}: no character named {name}"),
    }
}

fn add_members(g: &mut Graph, c: NodeId, plugs: &[PlugRef]) -> CmdResult<usize> {
    let mut list = member_list(g, c);
    let existing = member_plugs(g, c);
    let mut n = 0;
    for p in plugs {
        if existing.contains(p) {
            continue;
        }
        if !g.node(p.node).is_some_and(|x| x.ty().dag) {
            fail!("character: {} is not on a node in the hierarchy", g.plug_name(p));
        }
        let elem = g.connect(&PlugRef::new(p.node, DagNode::WORLD_MATRIX as i32), &PlugRef::new(c, MEMBER_OBJECTS as i32), false)?;
        let name = g.attr(p).name.clone();
        list.push((elem.element, name, p.component));
        n += 1;
    }
    let v: Vec<Value> = list.iter().map(|m| json!([m.0, m.1, m.2])).collect();
    set_json(g, c, MEMBER_ATTRS, json!(v))?;
    Ok(n)
}

fn remove_members(g: &mut Graph, c: NodeId, plugs: &[PlugRef]) -> CmdResult<usize> {
    let list = member_list(g, c);
    let mut keep = Vec::new();
    let mut n = 0;
    for m in list {
        if member_plug(g, c, &m).is_some_and(|p| plugs.contains(&p)) {
            let e = PlugRef::element(c, MEMBER_OBJECTS as i32, m.0);
            g.disconnect(&e);
            g.remove_element(&e)?;
            n += 1;
        } else {
            keep.push(json!([m.0, m.1, m.2]));
        }
    }
    set_json(g, c, MEMBER_ATTRS, json!(keep))?;
    Ok(n)
}

/// The channels the command's names mean: plugs as named, objects as their keyable channels less those excluded.
fn named_plugs(s: &Session, names: &[String], a: &Args) -> CmdResult<Vec<PlugRef>> {
    let g = &s.graph;
    let mut out = Vec::new();
    for n in names {
        if n.contains('.') {
            let p = g.plug(n)?;
            if g.attr(&p).ty == PlugType::Vec3 && p.component < 0 {
                out.extend((0..3).map(|c| PlugRef::component(p.node, p.attr, c)));
            } else {
                out.push(p);
            }
        } else {
            let Some(id) = g.find(n)? else { fail!("character: no object named {n}") };
            out.extend(anim::keyable_plugs(g, crate::dag::transform_of(g, id)));
        }
    }
    out.retain(|p| {
        let name = g.attr(p).name.clone();
        !((a.bool("excludeTranslate") && name.starts_with("translate")) || (a.bool("excludeRotate") && name.starts_with("rotate")) || (a.bool("excludeScale") && name.starts_with("scale")) || (a.bool("excludeVisibility") && name == "visibility"))
    });
    Ok(out)
}

/// The characters directly inside one.
pub fn sub_characters(g: &Graph, c: NodeId) -> Vec<NodeId> {
    g.elements(&PlugRef::new(c, SUB_CHARACTERS as i32)).iter().filter_map(|e| g.source(e)).map(|p| p.node).filter(|&n| is_character(g, n)).collect()
}

/// The character a character is inside, if any.
fn parent_character(g: &Graph, c: NodeId) -> Option<NodeId> {
    let msg = g.node(c)?.ty().attr_index("message")? as i32;
    g.destinations(&PlugRef::new(c, msg)).into_iter().find(|d| is_character(g, d.node) && d.attr == SUB_CHARACTERS as i32).map(|d| d.node)
}

fn add_sub(g: &mut Graph, c: NodeId, sub: NodeId) -> CmdResult<()> {
    if sub == c || sub_characters(g, c).contains(&sub) {
        return Ok(());
    }
    // A character can't hold one it is inside.
    let mut up = Some(c);
    while let Some(x) = up {
        if x == sub {
            fail!("character: {} is inside {}", g.path(c), g.path(sub));
        }
        up = parent_character(g, x);
    }
    if let Some(old) = parent_character(g, sub) {
        remove_sub(g, old, sub)?;
    }
    let msg = g.node(sub).and_then(|n| n.ty().attr_index("message")).unwrap() as i32;
    g.connect(&PlugRef::new(sub, msg), &PlugRef::new(c, SUB_CHARACTERS as i32), false)?;
    Ok(())
}

fn remove_sub(g: &mut Graph, c: NodeId, sub: NodeId) -> CmdResult<()> {
    for e in g.elements(&PlugRef::new(c, SUB_CHARACTERS as i32)) {
        if g.source(&e).is_some_and(|p| p.node == sub) {
            g.disconnect(&e);
            g.remove_element(&e)?;
        }
    }
    Ok(())
}

/// Every channel a character holds, its sub-characters' too.
pub fn all_member_plugs(g: &Graph, c: NodeId) -> Vec<PlugRef> {
    let mut out = member_plugs(g, c);
    for sub in sub_characters(g, c) {
        out.extend(all_member_plugs(g, sub));
    }
    out
}

/// What a character's query lists: its channels as plugs, then its sub-characters.
fn member_names(g: &Graph, c: NodeId) -> Vec<String> {
    let mut out: Vec<String> = member_plugs(g, c).iter().map(|p| g.plug_name(p)).collect();
    out.extend(sub_characters(g, c).iter().map(|&x| g.path(x)));
    out
}

/// The character's own plug for a member, as the reference names it: linearValues, angularValues or unitlessValues
/// by the member's unit, numbered from 1 in the order members were added.
fn character_plug(g: &Graph, c: NodeId, p: &PlugRef) -> Option<(String, String)> {
    let mut counts = [0usize; 3];
    for m in member_plugs(g, c) {
        let (kind, long, short) = match g.attr(&m).unit {
            aztek_ops::units::Unit::Linear => (1, "linearValues", "lv"),
            aztek_ops::units::Unit::Angle => (2, "angularValues", "av"),
            _ => (0, "unitlessValues", "uv"),
        };
        counts[kind] += 1;
        if m == *p {
            return Some((format!("{long}[{}]", counts[kind]), format!("{short}[{}]", counts[kind])));
        }
    }
    None
}

/// The names given as members: characters as themselves, the rest as channels.
fn split_names(s: &Session, names: &[String], a: &Args) -> CmdResult<(Vec<PlugRef>, Vec<NodeId>)> {
    let g = &s.graph;
    let mut chars = Vec::new();
    let mut rest = Vec::new();
    for n in names {
        match g.find(n).ok().flatten() {
            Some(id) if !n.contains('.') && is_character(g, id) => chars.push(id),
            _ => rest.push(n.clone()),
        }
    }
    Ok((named_plugs(s, &rest, a)?, chars))
}

fn set_lists(s: &Session, first: &str, others: &[String]) -> CmdResult<Vec<Vec<String>>> {
    let g = &s.graph;
    let mut out = Vec::new();
    for n in std::iter::once(first.to_string()).chain(others.iter().cloned()) {
        let c = character_arg(g, &n, "character")?;
        out.push(all_member_plugs(g, c).iter().map(|p| g.plug_name(p)).collect());
    }
    Ok(out)
}

fn character(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    // The set operations: -union, -intersection, -subtract and -isIntersecting take the first character, the others
    // follow.
    for key in ["union", "intersection", "subtract", "isIntersecting"] {
        // After -q the flag is a switch and the characters are all named after it.
        let (first, others) = match a.get(key).and_then(Value::as_str) {
            Some(f) => (f.to_string(), names.clone()),
            None if a.bool("query") && queried(a, &[key]) && !names.is_empty() => (names[0].clone(), names[1..].to_vec()),
            None => continue,
        };
        let lists = set_lists(s, &first, &others)?;
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
            return Ok(json!(lists.len() > 1 && !acc.is_empty()));
        }
        return Ok(json!(acc));
    }
    if a.bool("query") {
        // -q -mi before the character: the member at that index, or with a member named, its index.
        let index_value = a.get("memberIndex").filter(|v| !matches!(v, Value::Bool(_)));
        if index_value.is_some() || queried(a, &["memberIndex"]) {
            let cname = names.iter().find(|n| s.graph.find(n).ok().flatten().is_some_and(|id| is_character(&s.graph, id))).cloned();
            let Some(cname) = cname else { fail!("character: name the character") };
            let c = character_arg(&s.graph, &cname, "character")?;
            let list = member_names(&s.graph, c);
            if let Some(i) = index_value.and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|x| x.parse().ok()))) {
                return Ok(list.get(i as usize).map(|x| json!(x)).unwrap_or(Value::Null));
            }
            let named = names.iter().find(|n| **n != cname).cloned();
            let want = index_value.and_then(Value::as_str).map(str::to_string).or(named).unwrap_or_default();
            let want = want.as_str();
            let full = s.graph.plug(want).map(|p| s.graph.plug_name(&p)).unwrap_or_else(|_| want.to_string());
            return Ok(list.iter().position(|x| *x == full || x == want).map(|i| json!(i)).unwrap_or(json!(-1)));
        }
        // -q -cp member: the character's own plug for it; -q -ua member: that plug's alias, if one was given.
        if queried(a, &["characterPlug"]) || queried(a, &["userAlias"]) {
            let g = &s.graph;
            let mut out = Vec::new();
            for n in names.iter().chain(a.get("userAlias").and_then(Value::as_str).map(str::to_string).iter()) {
                let Ok(p) = g.plug(n) else { continue };
                let p = if g.attr(&p).ty == PlugType::Vec3 && p.component < 0 { PlugRef::component(p.node, p.attr, 0) } else { p };
                let Some(c) = g.nodes().into_iter().find(|&c| is_character(g, c) && member_plugs(g, c).contains(&p)) else { continue };
                let (long, short) = character_plug(g, c, &p).unwrap();
                if queried(a, &["userAlias"]) {
                    let alias = g.node(c).and_then(|n| n.aliases().iter().find(|(_, at)| **at == long || **at == short).map(|(al, _)| al.clone()));
                    out.push(alias.unwrap_or_default());
                } else {
                    out.push(format!("{}.{long}", g.path(c)));
                }
            }
            return Ok(if out.len() == 1 { json!(out[0]) } else { json!(out) });
        }
        let Some(name) = names.first() else { fail!("character: name the character") };
        let c = character_arg(&s.graph, name, "character")?;
        let g = &s.graph;
        let plugs = member_plugs(g, c);
        if queried(a, &["offsetNode"]) {
            return Ok(g.source(&PlugRef::new(c, OFFSET_LINK as i32)).map(|p| json!(g.path(p.node))).unwrap_or(json!("")));
        }
        if queried(a, &["addOffsetObject"]) {
            return Ok(json_of(g.node(c).unwrap().data(), OFFSET_OBJECTS).as_array().cloned().map(Value::Array).unwrap_or(json!([])));
        }
        if queried(a, &["text"]) {
            return Ok(json!(g.node(c).map(|n| n.data().string(ANNOTATION).to_string()).unwrap_or_default()));
        }
        if a.bool("nodesOnly") || queried(a, &["nodesOnly"]) {
            let mut nodes: Vec<String> = Vec::new();
            for p in &plugs {
                let n = g.path(p.node);
                if !nodes.contains(&n) {
                    nodes.push(n);
                }
            }
            for sub in sub_characters(g, c) {
                let n = g.path(sub);
                if !nodes.contains(&n) {
                    nodes.push(n);
                }
            }
            return Ok(json!(nodes));
        }
        if queried(a, &["attributes"]) {
            return Ok(json!(plugs.iter().map(|p| anim::channel_name(g, p)).collect::<Vec<_>>()));
        }
        if queried(a, &["library"]) {
            return Ok(library_of(g, c).map(|l| json!(g.path(l))).unwrap_or(Value::Null));
        }
        if queried(a, &["scheduler"]) {
            return Ok(scheduler_of(g, c).map(|l| json!(g.path(l))).unwrap_or(Value::Null));
        }
        return Ok(json!(member_names(g, c)));
    }
    if let Some(n) = a.get("isMember").and_then(Value::as_str) {
        let c = character_arg(&s.graph, n, "character")?;
        let (plugs, chars) = split_names(s, &names, a)?;
        let members = member_plugs(&s.graph, c);
        let subs = sub_characters(&s.graph, c);
        return Ok(json!((!plugs.is_empty() || !chars.is_empty()) && plugs.iter().all(|p| members.contains(p)) && chars.iter().all(|x| subs.contains(x))));
    }
    if let Some(n) = a.get("anyMember").and_then(Value::as_str) {
        let c = character_arg(&s.graph, n, "character")?;
        let (plugs, chars) = split_names(s, &objects(s, a), a)?;
        let members = member_plugs(&s.graph, c);
        let subs = sub_characters(&s.graph, c);
        return Ok(json!(plugs.iter().any(|p| members.contains(p)) || chars.iter().any(|x| subs.contains(x))));
    }
    if a.bool("edit") || a.has("addElement") || a.has("forceElement") || a.has("include") || a.has("remove") || a.has("clear") || a.has("flatten") || a.has("removeOffsetObject") || a.has("addOffsetObject") {
        // -text with -e names the character as the object.
        if let (Some(t), true) = (a.get("text").and_then(Value::as_str), a.bool("edit")) {
            let Some(n) = names.first() else { fail!("character: name the character") };
            let c = character_arg(&s.graph, n, "character")?;
            s.graph.set_value(PlugRef::new(c, ANNOTATION as i32), PlugValue::String(t.to_string()))?;
            return Ok(json!(s.graph.path(c)));
        }
    }
    for (flag, force, lenient) in [("addElement", false, false), ("forceElement", true, false), ("include", false, true)] {
        if let Some(n) = a.get(flag).and_then(Value::as_str) {
            let c = character_arg(&s.graph, n, "character")?;
            let (plugs, chars) = split_names(s, &objects(s, a), a)?;
            let g = &mut s.graph;
            let others: Vec<NodeId> = g.nodes().into_iter().filter(|&x| is_character(g, x) && x != c).collect();
            let mut take = Vec::new();
            for p in plugs {
                let holder = others.iter().copied().find(|&o| member_plugs(g, o).contains(&p));
                match holder {
                    // Characters share no channels: forcing takes them from the other one.
                    Some(o) if force => {
                        remove_members(g, o, &[p])?;
                        take.push(p);
                    }
                    Some(o) if lenient => {
                        if !a.bool("noWarnings") {
                            log::warn!("{} is already in {}; not added to {}.", g.plug_name(&p), g.path(o), g.path(c));
                        }
                    }
                    Some(o) => fail!("character: {} is already in {}", g.plug_name(&p), g.path(o)),
                    None => take.push(p),
                }
            }
            add_members(g, c, &take)?;
            for sub in chars {
                match add_sub(g, c, sub) {
                    Err(e) if lenient => {
                        if !a.bool("noWarnings") {
                            log::warn!("{e}");
                        }
                    }
                    r => r?,
                }
            }
            return Ok(json!(s.graph.path(c)));
        }
    }
    if let Some(n) = a.get("remove").and_then(Value::as_str) {
        let c = character_arg(&s.graph, n, "character")?;
        let (plugs, chars) = split_names(s, &objects(s, a), a)?;
        remove_members(&mut s.graph, c, &plugs)?;
        for sub in chars {
            remove_sub(&mut s.graph, c, sub)?;
        }
        return Ok(json!(s.graph.path(c)));
    }
    if let Some(n) = a.get("clear").and_then(Value::as_str) {
        let c = character_arg(&s.graph, n, "character")?;
        let plugs = member_plugs(&s.graph, c);
        remove_members(&mut s.graph, c, &plugs)?;
        for sub in sub_characters(&s.graph, c) {
            remove_sub(&mut s.graph, c, sub)?;
        }
        return Ok(json!(s.graph.path(c)));
    }
    // -flatten: the characters inside give up their members to this one.
    if let Some(n) = a.get("flatten").and_then(Value::as_str) {
        let c = character_arg(&s.graph, n, "character")?;
        let g = &mut s.graph;
        let mut stack = sub_characters(g, c);
        while let Some(sub) = stack.pop() {
            stack.extend(sub_characters(g, sub));
            let plugs = member_plugs(g, sub);
            remove_members(g, sub, &plugs)?;
            add_members(g, c, &plugs)?;
            for inner in sub_characters(g, sub) {
                remove_sub(g, sub, inner)?;
            }
            remove_sub(g, c, sub)?;
        }
        return Ok(json!(g.path(c)));
    }
    // -addOffsetObject / -removeOffsetObject: the selected members (or those named) as the character's offset objects.
    for (flag, add) in [("addOffsetObject", true), ("removeOffsetObject", false)] {
        let Some(n) = a.get(flag).and_then(Value::as_str) else { continue };
        let c = character_arg(&s.graph, n, "character")?;
        let g = &mut s.graph;
        let picked: Vec<String> = if names.is_empty() { g.selection().iter().map(|&i| g.path(crate::dag::transform_of(g, i))).collect() } else { names.clone() };
        let mut list: Vec<String> = json_of(g.node(c).unwrap().data(), OFFSET_OBJECTS).as_array().map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let members: Vec<NodeId> = all_member_plugs(g, c).iter().map(|p| p.node).collect();
        for o in picked {
            let Some(id) = g.find(&o)? else { fail!("character: no object named {o}") };
            let path = g.path(id);
            if add {
                if !members.contains(&id) {
                    fail!("character: {path} is not a member of {}", g.path(c));
                }
                if !list.contains(&path) {
                    list.push(path);
                }
            } else {
                list.retain(|x| *x != path);
            }
        }
        set_json(g, c, OFFSET_OBJECTS, json!(list))?;
        return Ok(json!(g.path(c)));
    }
    // -split: the items leave the named character for a new one.
    if let Some(n) = a.get("split").and_then(Value::as_str) {
        let from = character_arg(&s.graph, n, "character")?;
        let (plugs, chars) = split_names(s, &objects(s, a), a)?;
        let g = &mut s.graph;
        remove_members(g, from, &plugs)?;
        let t = types::find("character").unwrap();
        let c = g.create_node(&t, a.get("name").and_then(Value::as_str).unwrap_or("character1"), NO_NODE, true)?;
        add_members(g, c, &plugs)?;
        for sub in chars {
            remove_sub(g, from, sub)?;
            add_sub(g, c, sub)?;
        }
        return Ok(json!(g.path(c)));
    }
    let (plugs, chars) = if a.bool("empty") { (Vec::new(), Vec::new()) } else { split_names(s, &objects(s, a), a)? };
    if plugs.is_empty() && chars.is_empty() && !a.bool("empty") {
        fail!("character: name or select what goes in it, or use -em");
    }
    let t = types::find("character").unwrap();
    let g = &mut s.graph;
    let c = g.create_node(&t, a.get("name").and_then(Value::as_str).unwrap_or("character1"), NO_NODE, true)?;
    add_members(g, c, &plugs)?;
    for sub in chars {
        add_sub(g, c, sub)?;
    }
    if let Some(t) = a.get("text").and_then(Value::as_str) {
        g.set_value(PlugRef::new(c, ANNOTATION as i32), PlugValue::String(t.to_string()))?;
    }
    if let Some(root) = a.get("root").and_then(Value::as_str) {
        let Some(r) = g.find(root)? else { fail!("character: no object named {root}") };
        make_offset(g, c, r)?;
    }
    Ok(json!(g.path(c)))
}

// ---------------------------------------------------------------------------------------------------------------------
// characterOffset

const OFS_ACPT: usize = 0;
const OFS_ENABLE: usize = 1;
const OFS_IN_R: usize = 2;
const OFS_IN_T: usize = 3;
const OFS_INIT_T: usize = 4;
const OFS_OFF_R: usize = 5;
const OFS_OFF_ORDER: usize = 6;
const OFS_PIVOT: usize = 7;
const OFS_OFF_T: usize = 8;
const OFS_OUT_R: usize = 9;
const OFS_OUT_T: usize = 10;
const OFS_JOINT_ORIENT: usize = 11;
const OFS_PARENT_INV: usize = 12;
const OFS_PARENT: usize = 13;
const OFS_ORDER: usize = 14;
const OFS_CONTROL_PARENT: usize = 15;
const OFS_CONTROL_SCALE: usize = 16;

/// Redirects a root's motion: its rotation turned by the offset rotation about the pivot, then moved by the offset
/// translation less where the translation's driver started. Matrices taken as the root's parent space.
struct CharacterOffset;

impl NodeBehavior for CharacterOffset {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        use crate::transform::{euler_near, euler_to_matrix, RotateOrder};
        let (rin, tin) = (d.vec3(OFS_IN_R), d.vec3(OFS_IN_T));
        if !d.bool(OFS_ENABLE) {
            d.set_output(OFS_OUT_R, PlugValue::Vec3(rin));
            d.set_output(OFS_OUT_T, PlugValue::Vec3(tin));
            return;
        }
        let order = RotateOrder::from_index(d.int(OFS_ORDER));
        let jo = glam::DMat3::from_mat4(euler_to_matrix(d.vec3(OFS_JOINT_ORIENT), RotateOrder::from_index(0)));
        let off = glam::DMat3::from_mat4(euler_to_matrix(d.vec3(OFS_OFF_R), RotateOrder::from_index(d.int(OFS_OFF_ORDER))));
        // Into the parent's space and back: the offset is in world space.
        let parent = d.matrix(OFS_PARENT);
        let parent_inv = d.matrix(OFS_PARENT_INV);
        let control = if d.bool(OFS_ACPT) { d.matrix(OFS_CONTROL_PARENT) } else { glam::DMat4::IDENTITY };
        let scale = d.vec3(OFS_CONTROL_SCALE);
        let world_t = parent.transform_point3(tin);
        let pivot = control.transform_point3(d.vec3(OFS_PIVOT) * scale);
        let moved = off * (world_t - pivot) + pivot + control.transform_vector3(d.vec3(OFS_OFF_T) - d.vec3(OFS_INIT_T));
        let tout = parent_inv.transform_point3(moved);
        let pr = glam::DMat3::from_mat4(parent);
        let rot = jo * glam::DMat3::from_mat4(euler_to_matrix(rin, order));
        let world_r = pr * rot;
        let out_r = pr.inverse() * off * world_r;
        let local = jo.inverse() * out_r;
        let rout = euler_near(&local, order, rin);
        d.set_output(OFS_OUT_R, PlugValue::Vec3(rout));
        d.set_output(OFS_OUT_T, PlugValue::Vec3(tout));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn register_offset_type() {
    use aztek_ops::units::Unit;
    let v3 = |name: &str, short: &str, shorts: [&str; 3], default: f64, fl: u32, unit: Unit, doc: &str| {
        let mut a = crate::node::vec3_attr(name, short, glam::DVec3::splat(default), fl, doc);
        for (i, s) in shorts.iter().enumerate() {
            a.child_short[i] = s.to_string();
        }
        a.unit = unit;
        a
    };
    let io = flags::INPUT | flags::STORABLE;
    let ax = |b: &str| [format!("{b}x"), format!("{b}y"), format!("{b}z")];
    let sh = |b: &str| {
        let v = ax(b);
        [v[0].clone(), v[1].clone(), v[2].clone()]
    };
    fn s3(v: &[String; 3]) -> [&str; 3] {
        [v[0].as_str(), v[1].as_str(), v[2].as_str()]
    }
    let (rri, rti, itf, rrf, rpf, rtf, rro, rto, rjo, rcs) = (sh("rri"), sh("rti"), sh("itf"), sh("rrf"), [String::from("rppfx"), String::from("rppfy"), String::from("rppfz")], sh("rtf"), sh("rro"), sh("rto"), sh("rjo"), sh("rcs"));
    let mat = |name: &str, short: &str, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Matrix, default_value: PlugValue::Matrix(glam::DMat4::IDENTITY), flags: io, doc: doc.into(), ..Default::default() };
    let enm = |name: &str, short: &str, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Enum, default_value: PlugValue::Int(0), flags: io, enum_names: ["xyz", "yzx", "zxy", "xzy", "yxz", "zyx"].iter().map(|x| x.to_string()).collect(), doc: doc.into(), ..Default::default() };
    let boolean = |name: &str, short: &str, d: bool, fl: u32, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Bool, default_value: PlugValue::Bool(d), flags: fl, doc: doc.into(), ..Default::default() };
    types::register_type(NodeType {
        name: "characterOffset".into(),
        icon: "chart-line".into(),
        doc: "Offsets a character's root: turns and moves its motion, as when redirecting a walk.".into(),
        own_attrs: vec![
            boolean("applyControlParentTransform", "acpx", false, io, "Take the offset control's parent matrix into account."),
            boolean("enable", "nabl", true, io | flags::KEYABLE, "The offset is on."),
            v3("inRootRotate", "rri", s3(&rri), 0.0, io, Unit::Angle, "The root's rotation before the offset."),
            v3("inRootTranslate", "rti", s3(&rti), 0.0, io, Unit::Linear, "The root's translation before the offset."),
            v3("initialOffsetRootTranslate", "itf", s3(&itf), 0.0, io, Unit::Linear, "Where the translation offset's driver starts."),
            v3("offsetRootRotate", "rrf", s3(&rrf), 0.0, io | flags::KEYABLE, Unit::Angle, "The rotation offset."),
            enm("offsetRootRotateOrder", "rfor", "The order the rotation offset applies in."),
            v3("offsetRootRotatePivot", "rpf", s3(&rpf), 0.0, io | flags::KEYABLE, Unit::Linear, "What the rotation offset turns about."),
            v3("offsetRootTranslate", "rtf", s3(&rtf), 0.0, io | flags::KEYABLE, Unit::Linear, "The translation offset."),
            v3("outRootRotate", "rro", s3(&rro), 0.0, flags::OUTPUT, Unit::Angle, "The root's rotation after the offset."),
            v3("outRootTranslate", "rto", s3(&rto), 0.0, flags::OUTPUT, Unit::Linear, "The root's translation after the offset."),
            v3("rootJointOrient", "rjo", s3(&rjo), 0.0, io, Unit::Angle, "The root joint's orient."),
            mat("rootParentInverseMatrix", "rpim", "The inverse of the root's parent matrix."),
            mat("rootParentMatrix", "rpm", "The root's parent matrix."),
            enm("rootRotateOrder", "rror", "The root's rotate order."),
            mat("rotateControlParentMatrix", "rcpm", "The rotate control's parent matrix."),
            v3("rotateControlScale", "rcs", s3(&rcs), 1.0, io, Unit::None, "The rotate control's scale."),
        ],
        create: Some(Arc::new(|| Box::new(CharacterOffset))),
        ..Default::default()
    })
    .expect("characterOffset");
}

/// character -root: a characterOffset between the root's channels and what drives them.
fn make_offset(g: &mut Graph, c: NodeId, root: NodeId) -> CmdResult<NodeId> {
    let t = types::find("characterOffset").unwrap();
    let o = g.create_node(&t, "characterOffset1", NO_NODE, true)?;
    let msg = g.node(o).and_then(|n| n.ty().attr_index("message")).unwrap() as i32;
    g.connect(&PlugRef::new(o, msg), &PlugRef::new(c, OFFSET_LINK as i32), false)?;
    let ty = g.node(root).unwrap().ty().clone();
    let at = |n: &str| ty.attr_index(n).map(|i| i as i32);
    let pairs = [("rotate", OFS_IN_R, OFS_OUT_R), ("translate", OFS_IN_T, OFS_OUT_T)];
    for (name, input, output) in pairs {
        let Some(a) = at(name) else { continue };
        let whole = PlugRef::new(root, a);
        let v = g.value(&whole)?;
        g.set_value(PlugRef::new(o, input as i32), v)?;
        // Whatever drove the channel now drives the offset's input.
        if let Some(src) = g.source(&whole) {
            g.disconnect(&whole);
            g.connect(&src, &PlugRef::new(o, input as i32), false)?;
        }
        for k in 0..3 {
            let comp = PlugRef::component(root, a, k);
            if let Some(src) = g.source(&comp) {
                g.disconnect(&comp);
                g.connect(&src, &PlugRef::component(o, input as i32, k), false)?;
            }
        }
        g.connect(&PlugRef::new(o, output as i32), &whole, true)?;
    }
    if let Some(a) = at("rotateOrder") {
        let v = g.value(&PlugRef::new(root, a))?;
        g.set_value(PlugRef::new(o, OFS_ORDER as i32), v)?;
    }
    if let Some(a) = at("jointOrient") {
        let v = g.value(&PlugRef::new(root, a))?;
        g.set_value(PlugRef::new(o, OFS_JOINT_ORIENT as i32), v)?;
    }
    for (name, dst) in [("parentMatrix", OFS_PARENT), ("parentInverseMatrix", OFS_PARENT_INV)] {
        if let Some(a) = at(name) {
            g.connect(&PlugRef::new(root, a), &PlugRef::new(o, dst as i32), false)?;
        }
    }
    Ok(o)
}

// ---------------------------------------------------------------------------------------------------------------------
// Clips

fn library_of(g: &Graph, c: NodeId) -> Option<NodeId> {
    g.source(&PlugRef::new(c, LINK_LIBRARY as i32)).map(|s| s.node)
}

fn scheduler_of(g: &Graph, c: NodeId) -> Option<NodeId> {
    g.source(&PlugRef::new(c, LINK_SCHEDULER as i32)).map(|s| s.node)
}

/// The library and scheduler of a character, made the first time.
fn ensure_clip_nodes(g: &mut Graph, c: NodeId) -> CmdResult<(NodeId, NodeId)> {
    let base = g.node(c).map(|n| n.name.clone()).unwrap_or_default();
    let lib = match library_of(g, c) {
        Some(l) => l,
        None => {
            let t = types::find("clipLibrary").unwrap();
            let l = g.create_node(&t, &format!("{base}Clips"), NO_NODE, true)?;
            g.connect(&PlugRef::new(l, LIB_SOURCES as i32), &PlugRef::new(c, LINK_LIBRARY as i32), false)?;
            set_json(g, l, LIB_SOURCES, json!({}))?;
            set_json(g, l, LIB_POSES, json!({}))?;
            l
        }
    };
    let sched = match scheduler_of(g, c) {
        Some(x) => x,
        None => {
            let t = types::find("clipScheduler").unwrap();
            let x = g.create_node(&t, &format!("{base}Scheduler"), NO_NODE, true)?;
            g.connect(&PlugRef::new(x, SCHED_CLIPS as i32), &PlugRef::new(c, LINK_SCHEDULER as i32), false)?;
            set_json(g, x, SCHED_CLIPS, json!([]))?;
            x
        }
    };
    Ok((lib, sched))
}

fn node_json(g: &Graph, id: NodeId, attr: usize) -> Value {
    g.node(id).map(|n| json_of(n.data(), attr)).unwrap_or(Value::Null)
}

fn unique_in(names: &[String], wanted: &str) -> String {
    if !names.iter().any(|n| n == wanted) {
        return wanted.to_string();
    }
    let stem = wanted.trim_end_matches(|c: char| c.is_ascii_digit());
    (1..).map(|i| format!("{stem}{i}")).find(|n| !names.iter().any(|x| x == n)).unwrap()
}

fn clip_names(g: &Graph, sched: NodeId) -> Vec<String> {
    node_json(g, sched, SCHED_CLIPS).as_array().map(|a| a.iter().filter_map(|c| c.get("name")?.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

/// Puts a channel node on each member that has none yet.
fn wire_channels(g: &mut Graph, c: NodeId, lib: NodeId, sched: NodeId) -> CmdResult<()> {
    let t = types::find("clipChannel").unwrap();
    for m in member_list(g, c) {
        let Some(p) = member_plug(g, c, &m) else { continue };
        if g.source(&p).is_some_and(|s| is_clip_channel(g, s.node)) {
            continue;
        }
        let name = format!("{}_{}_clip", g.node(p.node).map(|n| n.name.clone()).unwrap_or_default(), anim::channel_name(g, &p));
        let ch = g.create_node(&t, &name, NO_NODE, true)?;
        g.connect(&PlugRef::new(sched, SCHED_CLIPS as i32), &PlugRef::new(ch, CH_SCHEDULE as i32), false)?;
        g.connect(&PlugRef::new(lib, LIB_SOURCES as i32), &PlugRef::new(ch, CH_SOURCES as i32), false)?;
        g.set_value(PlugRef::new(ch, CH_MEMBER as i32), PlugValue::Int(m.0 as i64))?;
        let v = anim::plug_scalar(g, &p);
        g.set_value(PlugRef::new(ch, CH_REST as i32), PlugValue::Float(v))?;
        // Keys the clip didn't take stay in play under it.
        if let Some(src) = g.source(&p) {
            g.disconnect(&p);
            g.connect(&src, &PlugRef::new(ch, CH_BASE as i32), false)?;
            g.set_value(PlugRef::new(ch, CH_REST as i32 + 2), PlugValue::Bool(true))?;
        }
        g.connect(&PlugRef::new(ch, CH_OUTPUT as i32), &p, true)?;
    }
    Ok(())
}

/// A member's keys: on it, or under the clip channel that now reads through to them.
fn member_curve(g: &Graph, p: &PlugRef) -> Option<NodeId> {
    anim::curve_of(g, p).or_else(|| {
        let ch = g.source(p).or_else(|| g.source(&p.whole())).filter(|x| is_clip_channel(g, x.node))?;
        g.source(&PlugRef::new(ch.node, CH_BASE as i32)).map(|x| x.node).filter(|&c| anim::is_curve(g, c))
    })
}

fn clip(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let char_name = a.get("character").and_then(Value::as_str).map(str::to_string).or_else(|| names.iter().find(|n| s.graph.find(n).ok().flatten().is_some_and(|id| is_character(&s.graph, id))).cloned());
    let char_name = char_name.or_else(|| s.current_characters.first().map(|&c| s.graph.path(c)));
    let Some(char_name) = char_name else { fail!("clip: name the character") };
    let c = character_arg(&s.graph, &char_name, "clip")?;

    if a.bool("query") {
        let g = &s.graph;
        let Some(sched) = scheduler_of(g, c) else { return Ok(json!([])) };
        let clips = node_json(g, sched, SCHED_CLIPS);
        let lib = library_of(g, c);
        if queried(a, &["allSourceClips"]) {
            let sources = lib.map(|l| node_json(g, l, LIB_SOURCES)).unwrap_or(Value::Null);
            return Ok(json!(sources.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()).unwrap_or_default()));
        }
        if queried(a, &["allClips"]) {
            return Ok(json!(clip_names(g, sched)));
        }
        let which = a.get("name").and_then(Value::as_str).unwrap_or("");
        let one = clips.as_array().and_then(|v| v.iter().find(|x| x.get("name").and_then(Value::as_str) == Some(which)).cloned());
        let Some(one) = one else { return Ok(json!(clip_names(g, sched))) };
        return Ok(if queried(a, &["startTime", "st"]) {
            one["sourceStart"].clone()
        } else if queried(a, &["endTime", "et"]) {
            one["sourceEnd"].clone()
        } else if queried(a, &["sourceClipName"]) {
            one["source"].clone()
        } else {
            one
        });
    }

    let (lib, sched) = ensure_clip_nodes(&mut s.graph, c)?;
    let g = &mut s.graph;
    if a.bool("remove") {
        let Some(name) = a.get("name").and_then(Value::as_str) else { fail!("clip: -rm needs -n") };
        let mut clips = node_json(g, sched, SCHED_CLIPS);
        if let Some(v) = clips.as_array_mut() {
            v.retain(|x| x.get("name").and_then(Value::as_str) != Some(name));
        }
        set_json(g, sched, SCHED_CLIPS, clips)?;
        return Ok(Value::Null);
    }

    let wanted = a.get("name").and_then(Value::as_str).unwrap_or("clip1").to_string();
    let times = |flag: &str, alt: &str| a.get(flag).or(a.get(alt)).and_then(Value::as_f64);
    let (mut lo, mut hi) = (times("startTime", "st"), times("endTime", "et"));
    let members = member_list(g, c);
    let plugs: Vec<((i32, String, i32), PlugRef)> = members.into_iter().filter_map(|m| member_plug(g, c, &m).map(|p| (m, p))).collect();
    if plugs.is_empty() {
        fail!("clip: {char_name} has no members");
    }
    if lo.is_none() || hi.is_none() {
        // The range of all the members' keys.
        let t: Vec<f64> = plugs.iter().filter_map(|(_, p)| member_curve(g, p)).flat_map(|c| anim::keys_of(g, c).keys.into_iter().map(|k| k.t)).collect();
        lo = lo.or(t.iter().copied().reduce(f64::min));
        hi = hi.or(t.iter().copied().reduce(f64::max));
    }
    let (Some(lo), Some(hi)) = (lo, hi) else { fail!("clip: no keys to make a clip of: give -s and -end") };

    // The keys in the range move to the source clip; a channel without any holds its value.
    let mut sources = node_json(g, lib, LIB_SOURCES);
    let existing: Vec<String> = sources.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
    let source_name = unique_in(&existing, a.get("sourceClipName").and_then(Value::as_str).map(str::to_string).unwrap_or(format!("{wanted}Source")).as_str());
    let mut curves = Map::new();
    let leave = a.bool("leaveKeys");
    for (m, p) in &plugs {
        let curve = member_curve(g, p);
        let mut taken = Keys::default();
        if let Some(cu) = curve {
            let mut k = anim::keys_of(g, cu);
            taken.weighted = k.weighted;
            taken.keys = k.keys.iter().filter(|x| x.t >= lo - 1e-6 && x.t <= hi + 1e-6).copied().collect();
            if !leave {
                k.keys.retain(|x| x.t < lo - 1e-6 || x.t > hi + 1e-6);
                if k.keys.is_empty() {
                    let v = anim::plug_scalar(g, p);
                    g.delete_node(cu)?;
                    if g.source(p).is_none() {
                        let _ = g.set_value(*p, PlugValue::Float(v));
                    }
                } else {
                    anim::set_keys(g, cu, k)?;
                }
            }
        }
        if taken.keys.is_empty() {
            let v = anim::plug_scalar(g, p);
            let r = anim::range();
            taken.keys.push(anim::Key::new(lo, v, r.default_in, r.default_out));
        }
        curves.insert(m.0.to_string(), crate::plug::Data::to_json(&taken));
    }
    if let Some(o) = sources.as_object_mut() {
        o.insert(source_name.clone(), json!({"start": lo, "end": hi, "curves": Value::Object(curves)}));
    }
    set_json(g, lib, LIB_SOURCES, sources)?;

    let scheduled = a.get("scheduleClip").and_then(Value::as_bool).unwrap_or(true);
    if !scheduled {
        return Ok(json!([source_name]));
    }
    wire_channels(g, c, lib, sched)?;
    let mut clips = node_json(g, sched, SCHED_CLIPS);
    let name = unique_in(&clip_names(g, sched), &wanted);
    let entry = json!({"name": name, "source": source_name, "start": a.get("scheduleStart").and_then(Value::as_f64).unwrap_or(lo), "sourceStart": lo, "sourceEnd": hi, "scale": 1.0, "preCycle": 0.0, "postCycle": 1.0, "weight": 1.0, "enabled": true, "track": 0, "relative": a.bool("allRelative")});
    clips.as_array_mut().unwrap().push(entry);
    set_json(g, sched, SCHED_CLIPS, clips)?;
    Ok(json!([source_name, name]))
}

// ---------------------------------------------------------------------------------------------------------------------
// bakeClip and clipMatching

/// bakeClip: the clips picked (all by default) sampled frame by frame into one new source clip, scheduled in their place.
fn bake_clip(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let char_name = names.first().cloned().or_else(|| s.current_characters.first().map(|&c| s.graph.path(c)));
    let Some(char_name) = char_name else { fail!("bakeClip: name the character") };
    let g = &mut s.graph;
    let c = character_arg(g, &char_name, "bakeClip")?;
    let (Some(lib), Some(sched)) = (library_of(g, c), scheduler_of(g, c)) else { fail!("bakeClip: {char_name} has no clips") };
    // -ci and -b together: the clips named and the blend's two.
    let mut picked: Vec<usize> = Vec::new();
    for flag in ["clipIndex", "blend"] {
        let idx: Vec<usize> = match a.get(flag) {
            Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_u64().or_else(|| x.as_str().and_then(|s| s.parse().ok()))).map(|x| x as usize).collect(),
            Some(v) => v.as_u64().map(|x| x as usize).into_iter().collect(),
            None => Vec::new(),
        };
        for i in idx {
            if !picked.contains(&i) {
                picked.push(i);
            }
        }
    }
    let clips = node_json(g, sched, SCHED_CLIPS);
    let list = clips.as_array().cloned().unwrap_or_default();
    if picked.is_empty() {
        picked = (0..list.len()).collect();
    }
    if picked.iter().any(|&i| i >= list.len()) || picked.is_empty() {
        fail!("bakeClip: no such clip");
    }
    let spans: Vec<(f64, f64)> = picked.iter().map(|&i| clip_span(&list[i])).collect();
    let lo = spans.iter().map(|x| x.0).fold(f64::MAX, f64::min);
    let hi = spans.iter().map(|x| x.1).fold(f64::MIN, f64::max);
    let members = member_list(g, c);
    let plugs: Vec<((i32, String, i32), PlugRef)> = members.into_iter().filter_map(|m| member_plug(g, c, &m).map(|p| (m, p))).collect();
    let now = anim::frame(g);
    g.open_chunk("Bake Clip");
    let r = (|| -> CmdResult<Value> {
        // Only the clips asked for count while sampling.
        let mut sampling = list.clone();
        for (i, one) in sampling.iter_mut().enumerate() {
            if !picked.contains(&i) {
                one["enabled"] = json!(false);
            }
        }
        set_json(g, sched, SCHED_CLIPS, Value::Array(sampling))?;
        let mut curves = Map::new();
        let rg = anim::range();
        let mut samples: Vec<Keys> = vec![Keys::default(); plugs.len()];
        let mut t = lo;
        while t <= hi + 1e-9 {
            anim::set_frame(g, t);
            g.evaluate();
            for (k, (_, p)) in plugs.iter().enumerate() {
                let v = anim::plug_scalar(g, p);
                samples[k].keys.push(anim::Key::new(t, v, rg.default_in, rg.default_out));
            }
            t += 1.0;
        }
        anim::set_frame(g, now);
        for ((m, _), k) in plugs.iter().zip(&samples) {
            curves.insert(m.0.to_string(), crate::plug::Data::to_json(k));
        }
        let mut sources = node_json(g, lib, LIB_SOURCES);
        let existing: Vec<String> = sources.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
        let wanted = a.get("name").and_then(Value::as_str).unwrap_or("bakedClip1").to_string();
        let source = unique_in(&existing, &format!("{wanted}Source"));
        sources.as_object_mut().map(|o| o.insert(source.clone(), json!({"start": lo, "end": hi, "curves": Value::Object(curves)})));
        set_json(g, lib, LIB_SOURCES, sources)?;
        let mut kept: Vec<Value> = if a.bool("keepOriginals") { list.iter().map(|x| { let mut x = x.clone(); x["enabled"] = json!(false); x }).collect() } else { list.iter().enumerate().filter(|(i, _)| !picked.contains(i)).map(|(_, x)| x.clone()).collect() };
        let taken: Vec<String> = kept.iter().filter_map(|x| x.get("name").and_then(Value::as_str).map(str::to_string)).collect();
        let name = unique_in(&taken, &wanted);
        kept.push(json!({"name": name, "source": source, "start": lo, "sourceStart": lo, "sourceEnd": hi, "scale": 1.0, "preCycle": 0.0, "postCycle": 1.0, "weight": 1.0, "enabled": true, "track": 0, "relative": false}));
        set_json(g, sched, SCHED_CLIPS, Value::Array(kept))?;
        Ok(json!(name))
    })();
    g.close_chunk();
    r
}

/// clipMatching: -cs clip time, -cd clip time: shifts the destination's channels so it starts from where the source is.
fn clip_matching(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let char_name = names.first().cloned().or_else(|| s.current_characters.first().map(|&c| s.graph.path(c)));
    let Some(char_name) = char_name else { fail!("clipMatching: name the character") };
    let g = &mut s.graph;
    let c = character_arg(g, &char_name, "clipMatching")?;
    let (Some(lib), Some(sched)) = (library_of(g, c), scheduler_of(g, c)) else { fail!("clipMatching: {char_name} has no clips") };
    let pair = |flag: &str| -> Option<(String, f64)> {
        let l = a.get(flag)?.as_array()?;
        Some((l.first()?.as_str()?.to_string(), l.get(1)?.as_f64()?))
    };
    let (Some(src), Some(dst)) = (pair("clipSrc"), pair("clipDst")) else { fail!("clipMatching: give -cs and -cd") };
    let clips = node_json(g, sched, SCHED_CLIPS);
    let find_clip = |n: &str| clips.as_array().and_then(|v| v.iter().find(|x| x.get("name").and_then(Value::as_str) == Some(n)).cloned());
    let (Some(sc), Some(dc)) = (find_clip(&src.0), find_clip(&dst.0)) else { fail!("clipMatching: no such clip") };
    let mut sources = node_json(g, lib, LIB_SOURCES);
    let (ss, ds) = (sc["source"].as_str().unwrap_or("").to_string(), dc["source"].as_str().unwrap_or("").to_string());
    let (match_t, match_r) = (a.get("matchTranslation").and_then(Value::as_i64).unwrap_or(1) != 0, a.get("matchRotation").and_then(Value::as_i64).unwrap_or(0) != 0);
    for m in member_list(g, c) {
        let attr = m.1.as_str();
        let translate = attr.starts_with("translate") || attr.starts_with("t");
        let rotate = attr.starts_with("rotate") || attr.starts_with("r");
        if !(translate && match_t || rotate && match_r) {
            continue;
        }
        let key = m.0.to_string();
        let (Some(a_keys), Some(b_keys)) = (sources[&ss]["curves"].get(&key).map(Keys::from_json), sources[&ds]["curves"].get(&key).map(Keys::from_json)) else { continue };
        if a_keys.keys.is_empty() || b_keys.keys.is_empty() {
            continue;
        }
        let d = clip_value(&sc, &a_keys, src.1) - clip_value(&dc, &b_keys, dst.1);
        let mut moved = b_keys;
        for k in &mut moved.keys {
            k.v += d;
        }
        sources[&ds]["curves"][&key] = crate::plug::Data::to_json(&moved);
    }
    set_json(g, lib, LIB_SOURCES, sources)?;
    g.evaluate();
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// clipSchedule

fn scheduler_arg(g: &Graph, a: &Args) -> CmdResult<NodeId> {
    let Some(name) = a.named().first().cloned() else { fail!("clipSchedule: name the scheduler") };
    match g.find(&name)? {
        Some(id) if g.node(id).is_some_and(|n| n.ty().name == "clipScheduler") => Ok(id),
        Some(id) if is_character(g, id) => match scheduler_of(g, id) {
            Some(s) => Ok(s),
            None => fail!("clipSchedule: {name} has no clips"),
        },
        _ => fail!("clipSchedule: no scheduler named {name}"),
    }
}

fn clip_schedule(s: &mut Session, a: &Args) -> CmdResult {
    let g = &mut s.graph;
    let sched = scheduler_arg(g, a)?;
    let mut clips = node_json(g, sched, SCHED_CLIPS);
    let list = clips.as_array_mut().unwrap_or_else(|| unreachable!("the schedule is a list"));
    let by_name = a.get("name").and_then(Value::as_str).map(str::to_string);
    let by_index = a.get("clipIndex").and_then(Value::as_u64).map(|i| i as usize);
    let index = by_index.or_else(|| by_name.as_ref().and_then(|n| list.iter().position(|c| c.get("name").and_then(Value::as_str) == Some(n))));
    if a.bool("query") {
        let line = |i: usize, c: &Value| {
            let n = |k: &str| c.get(k).and_then(Value::as_f64).unwrap_or(0.0);
            // The reference's order: name, index, start, source start, source end, scale, pre cycle, post cycle, hold.
            format!("{},{},{},{},{},{},{},{},{}", c.get("name").and_then(Value::as_str).unwrap_or(""), i, n("start"), n("sourceStart"), n("sourceEnd"), n("scale"), n("preCycle"), n("postCycle"), n("hold"))
        };
        let Some(i) = index else {
            return Ok(json!(list.iter().enumerate().map(|(i, c)| line(i, c)).collect::<Vec<_>>()));
        };
        let Some(c) = list.get(i) else { fail!("clipSchedule: no clip {i}") };
        for (flag, key) in [("start", "start"), ("sourceStart", "sourceStart"), ("sourceEnd", "sourceEnd"), ("scale", "scale"), ("cycle", "postCycle"), ("weight", "weight"), ("track", "track"), ("enable", "enabled"), ("mute", "mute"), ("solo", "solo"), ("hold", "hold")] {
            if queried(a, &[flag]) {
                return Ok(c.get(key).cloned().unwrap_or(Value::Null));
            }
        }
        return Ok(json!(line(i, c)));
    }
    if a.bool("removeEmptyTracks") {
        // Tracks close up, keeping their order.
        let mut used: Vec<i64> = list.iter().filter_map(|c| c.get("track").and_then(Value::as_i64)).collect();
        used.sort();
        used.dedup();
        for c in list.iter_mut() {
            let t = c.get("track").and_then(Value::as_i64).unwrap_or(0);
            c["track"] = json!(used.iter().position(|x| *x == t).unwrap_or(0));
        }
    }
    if let Some(t) = a.get("insertTrack").and_then(Value::as_i64) {
        for c in list.iter_mut() {
            let x = c.get("track").and_then(Value::as_i64).unwrap_or(0);
            if x >= t {
                c["track"] = json!(x + 1);
            }
        }
    }
    if let Some(t) = a.get("removeTrack").and_then(Value::as_i64) {
        list.retain(|c| c.get("track").and_then(Value::as_i64).unwrap_or(0) != t);
        for c in list.iter_mut() {
            let x = c.get("track").and_then(Value::as_i64).unwrap_or(0);
            if x > t {
                c["track"] = json!(x - 1);
            }
        }
    }
    let mut result = Value::Null;
    if let Some(src) = a.get("instance").and_then(Value::as_str) {
        // Schedules a source clip again.
        let characters = g.destinations(&PlugRef::new(sched, SCHED_CLIPS as i32)).into_iter().map(|d| d.node).find(|&n| is_character(g, n));
        let lib = characters.and_then(|c| library_of(g, c));
        let sources = lib.map(|l| node_json(g, l, LIB_SOURCES)).unwrap_or(Value::Null);
        let Some(info) = sources.get(src) else { fail!("clipSchedule: no source clip named {src}") };
        let names: Vec<String> = list.iter().filter_map(|c| c.get("name")?.as_str().map(str::to_string)).collect();
        let name = unique_in(&names, by_name.as_deref().unwrap_or(src.trim_end_matches("Source")));
        let (ss, se) = (info["start"].as_f64().unwrap_or(1.0), info["end"].as_f64().unwrap_or(1.0));
        list.push(json!({"name": name, "source": src, "start": a.float("start", ss), "sourceStart": ss, "sourceEnd": se, "scale": a.float("scale", 1.0), "preCycle": 0.0, "postCycle": a.float("cycle", 1.0), "weight": a.float("weight", 1.0), "enabled": true, "track": a.int("track", 0), "relative": false}));
        result = json!(name);
    } else if let Some(i) = index {
        let Some(c) = list.get_mut(i) else { fail!("clipSchedule: no clip {i}") };
        if a.bool("remove") {
            list.remove(i);
        } else {
            for (flag, key) in [("start", "start"), ("sourceStart", "sourceStart"), ("sourceEnd", "sourceEnd"), ("scale", "scale"), ("cycle", "postCycle"), ("preCycle", "preCycle"), ("weight", "weight"), ("hold", "hold"), ("postHold", "postHold"), ("preHold", "preHold")] {
                if let Some(v) = a.get(flag).and_then(Value::as_f64) {
                    c[key] = json!(v);
                }
            }
            if let Some(v) = a.get("track").and_then(Value::as_i64) {
                c["track"] = json!(v);
            }
            if let Some(v) = a.get("shift").and_then(Value::as_i64) {
                let t = c.get("track").and_then(Value::as_i64).unwrap_or(0);
                c["track"] = json!((t + v).max(0));
            }
            for (flag, key) in [("enable", "enabled"), ("mute", "mute"), ("solo", "solo"), ("lock", "lock")] {
                if let Some(v) = a.get(flag).and_then(Value::as_bool) {
                    c[key] = json!(v);
                }
            }
            if a.bool("allRelative") {
                c["relative"] = json!(true);
            }
            if a.bool("allAbsolute") {
                c["relative"] = json!(false);
            }
        }
    } else if a.bool("remove") {
        fail!("clipSchedule: -rm needs -n or -ci");
    }
    set_json(g, sched, SCHED_CLIPS, clips)?;
    Ok(result)
}

// ---------------------------------------------------------------------------------------------------------------------
// Poses

fn pose(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let Some(cn) = names.first().cloned().or_else(|| s.current_characters.first().map(|&c| s.graph.path(c))) else { fail!("pose: name the character") };
    let c = character_arg(&s.graph, &cn, "pose")?;
    let g = &mut s.graph;
    if a.bool("query") {
        let poses = library_of(g, c).map(|l| node_json(g, l, LIB_POSES)).unwrap_or(json!({}));
        return Ok(json!(poses.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()).unwrap_or_default()));
    }
    let (lib, _) = ensure_clip_nodes(g, c)?;
    let mut poses = node_json(g, lib, LIB_POSES);
    if a.bool("apply") {
        let Some(name) = a.get("name").and_then(Value::as_str) else { fail!("pose: -a needs -n") };
        let Some(values) = poses.get(name).cloned() else { fail!("pose: no pose named {name}") };
        for m in member_list(g, c) {
            let (Some(p), Some(v)) = (member_plug(g, c, &m), values.get(m.0.to_string()).and_then(Value::as_f64)) else { continue };
            // A channel that is driven holds the value until the time changes, as a set does.
            let _ = g.set_value(p, PlugValue::Float(v));
        }
        return Ok(json!(name));
    }
    let existing: Vec<String> = poses.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
    let name = unique_in(&existing, a.get("name").and_then(Value::as_str).unwrap_or("pose1"));
    let mut values = Map::new();
    for m in member_list(g, c) {
        if let Some(p) = member_plug(g, c, &m) {
            values.insert(m.0.to_string(), json!(anim::plug_scalar(g, &p)));
        }
    }
    if let Some(o) = poses.as_object_mut() {
        o.insert(name.clone(), Value::Object(values));
    }
    set_json(g, lib, LIB_POSES, poses)?;
    Ok(json!(name))
}

// ---------------------------------------------------------------------------------------------------------------------

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let plain = |name: &str, doc: &str, attrs: Vec<AttrDef>| {
            types::register_type(NodeType {
                name: name.into(),
                icon: "chart-line".into(),
                doc: doc.into(),
                own_attrs: attrs,
                create: Some(Arc::new(|| Box::new(PlainNode))),
                ..Default::default()
            })
            .expect(name);
        };
        plain(
            "character",
            "A character set: channels gathered to key and clip together.",
            vec![
                AttrDef { name: "memberObjects".into(), short_name: "mo".into(), ty: PlugType::Matrix, default_value: PlugValue::Matrix(glam::DMat4::IDENTITY), flags: flags::INPUT | flags::ARRAY | flags::HIDDEN, doc: "The members' nodes.".into(), ..Default::default() },
                data_attr("memberAttrs", "ma", "Each member's attr and component."),
                data_attr("clipLibrary", "cli", "From its library."),
                data_attr("clipScheduler", "cls", "From its scheduler."),
                AttrDef { name: "subCharacters".into(), short_name: "sch".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::INPUT | flags::ARRAY | flags::HIDDEN | flags::MESSAGE, doc: "The characters inside this one.".into(), ..Default::default() },
                data_attr("offsetObjects", "ofo", "The members used to work out and apply offsets."),
                AttrDef { name: "annotation".into(), short_name: "an".into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: flags::INPUT | flags::STORABLE, doc: "A note kept with the character.".into(), ..Default::default() },
                AttrDef { name: "offsetNode".into(), short_name: "ofn".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::INPUT | flags::HIDDEN | flags::MESSAGE, doc: "Its characterOffset node.".into(), ..Default::default() },
            ],
        );
        register_offset_type();
        plain(
            "clipLibrary",
            "A character's source clips and poses.",
            vec![data_attr("sources", "src", "Source clips by name: their range and keys."), data_attr("poses", "ps", "Poses by name.")],
        );
        plain("clipScheduler", "When a character's clips play.", vec![data_attr("clips", "cl", "The scheduled clips.")]);
        let float = |name: &str, short: &str, fl: u32, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Float, default_value: PlugValue::Float(0.0), flags: fl, doc: doc.into(), ..Default::default() };
        types::register_type(NodeType {
            name: "clipChannel".into(),
            icon: "chart-line".into(),
            time_dependent: true,
            doc: "One member channel of a character, reading its schedule.".into(),
            own_attrs: vec![
                data_attr("schedule", "sc", "From the scheduler."),
                data_attr("sources", "src", "From the library."),
                AttrDef { name: "member".into(), short_name: "mb".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::INPUT | flags::STORABLE | flags::HIDDEN, doc: "Which member, by its place on the character.".into(), ..Default::default() },
                float("base", "b", flags::INPUT | flags::STORABLE, "Keys the clips didn't take."),
                float("rest", "r", flags::INPUT | flags::STORABLE, "The value before there were clips."),
                float("output", "o", flags::OUTPUT, "What the channel takes."),
                AttrDef { name: "hasBase".into(), short_name: "hb".into(), ty: PlugType::Bool, default_value: PlugValue::Bool(false), flags: flags::INPUT | flags::STORABLE | flags::HIDDEN, doc: "Whether base is connected.".into(), ..Default::default() },
            ],
            create: Some(Arc::new(|| Box::new(ClipChannel))),
            ..Default::default()
        })
        .expect("clipChannel");
    });
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        Cmd::new("bakeClip", "Bake clips into one, frame by frame: -ci which (all by default), -b a blend's two, -k keeps the originals disabled, -n its name. Returns the new clip.", bake_clip)
            .positional("character")
            .flag(f("clipIndex", "ci", ArgType::Any, "A clip's place on the scheduler.").multi(1))
            .flag(f("blend", "b", ArgType::List, "The two clips of a blend, by index.").multi(2))
            .flag(f("keepOriginals", "k", ArgType::Bool, "Keep the clips, disabled."))
            .flag(f("name", "n", ArgType::String, "The new clip's name.")),
    );
    c.register(
        Cmd::new("clipMatching", "Line a clip up with another at a frame each: -cs clip frame, -cd clip frame, -mt and -mr which channels (translate by default).", clip_matching)
            .positional("character")
            .flag(f("clipSrc", "cs", ArgType::List, "The clip to match and its frame.").multi(2))
            .flag(f("clipDst", "cd", ArgType::List, "The clip to shift and its frame.").multi(2))
            .flag(f("matchTranslation", "mt", ArgType::Int, "Match translates (on by default)."))
            .flag(f("matchRotation", "mr", ArgType::Int, "Match rotates.")),
    );

    c.register(
        query_flags(Cmd::new(
            "character",
            "Create a character set from channels or objects (-n names it, -em for an empty one), add to one (-add), remove (-rm), clear (-cl). \
             -q lists its channels, -q -no its nodes, -q -at its attribute names; -im asks whether channels are in it. Returns its name.",
            character,
        ))
        .positional("channels or objects (the selection when none); the character when querying")
        .flag(f("name", "n", ArgType::String, "The new character's name."))
        .flag(f("addElement", "add", ArgType::String, "The character to add to."))
        .flag(f("forceElement", "fe", ArgType::String, "The character to move them to, from any other."))
        .flag(f("remove", "rm", ArgType::String, "The character to remove from."))
        .flag(f("clear", "cl", ArgType::String, "The character to empty."))
        .flag(f("isMember", "im", ArgType::String, "The character to test membership of."))
        .flag(f("empty", "em", ArgType::Bool, "Create it empty."))
        .flag(f("nodesOnly", "no", ArgType::Bool, "Query: the nodes rather than the channels."))
        .flag(f("attributes", "at", ArgType::Bool, "Query: the channels' attribute names."))
        .flag(f("library", "lib", ArgType::Bool, "Query: its clip library."))
        .flag(f("scheduler", "sc", ArgType::Bool, "Query: its clip scheduler."))
        .flag(f("excludeTranslate", "et", ArgType::Bool, "Leave out translate channels."))
        .flag(f("excludeRotate", "er", ArgType::Bool, "Leave out rotate channels."))
        .flag(f("excludeScale", "es", ArgType::Bool, "Leave out scale channels."))
        .flag(f("excludeVisibility", "ev", ArgType::Bool, "Leave out visibility."))
        .flag(f("excludeDynamic", "ed", ArgType::Bool, "Accepted for existing scripts: objects give their keyable channels."))
        .flag(f("include", "in", ArgType::String, "The character to add to; what can't go in is left out with a warning."))
        .flag(f("anyMember", "am", ArgType::String, "Whether any of the items is in this character."))
        .flag(f("flatten", "fl", ArgType::String, "The character whose sub-characters give it their members."))
        .flag(f("union", "un", ArgType::String, "Query: the members of this character and the others named."))
        .flag(f("intersection", "int", ArgType::String, "Query: the members this character shares with all the others named."))
        .flag(f("subtract", "sub", ArgType::String, "Query: the members of this character not in the others named."))
        .flag(f("isIntersecting", "ii", ArgType::String, "Query: whether this character shares members with the others named."))
        .flag(f("split", "sp", ArgType::String, "The character the items leave for a new one."))
        .flag(f("noWarnings", "nw", ArgType::Bool, "No warnings for items that can't be added."))
        .flag(f("text", "t", ArgType::String, "A note kept with the character."))
        .flag(f("memberIndex", "mi", ArgType::Any, "Query: the member at this index, or the index of the member named."))
        .flag(f("characterPlug", "cp", ArgType::Bool, "Query: the character's plug for the member named."))
        .flag(f("userAlias", "ua", ArgType::Any, "Query: the alias given to the character's plug for a member, or an empty string."))
        .flag(f("root", "rt", ArgType::String, "A transform to be the character's root, with a characterOffset node."))
        .flag(f("offsetNode", "ofs", ArgType::Bool, "Query: the characterOffset node."))
        .flag(f("addOffsetObject", "aoo", ArgType::String, "The character whose offset objects the selected members join; -q lists them."))
        .flag(f("removeOffsetObject", "roo", ArgType::String, "The character whose offset objects the selected members leave.")),
    );

    c.register(
        query_flags(Cmd::new(
            "currentCharacters",
            "The characters keys go to when nothing is selected. Names set them, -cl clears them, -q lists them.",
            |s, a| {
                if a.bool("query") {
                    return Ok(json!(s.current_characters.iter().map(|&c| s.graph.path(c)).collect::<Vec<_>>()));
                }
                let mut ids = Vec::new();
                if !a.bool("clear") {
                    for n in a.named() {
                        ids.push(character_arg(&s.graph, &n, "currentCharacters")?);
                    }
                }
                s.current_characters = ids;
                Ok(json!(s.current_characters.iter().map(|&c| s.graph.path(c)).collect::<Vec<_>>()))
            },
        ))
        .positional("characters")
        .flag(f("clear", "cl", ArgType::Bool, "No current character."))
        .undoable(false),
    );

    c.register(
        query_flags(Cmd::new(
            "clip",
            "Make a clip of a character's keys between -s and -end (all of them when not given): the keys move to a source clip and, unless -sc is off, it is scheduled. \
             Returns the source clip and the clip. -q -ac and -q -asc list clips, -rm -n removes one from the schedule.",
            clip,
        ))
        .positional("the character (the current one when none)")
        .flag(f("name", "n", ArgType::String, "The clip's name."))
        .flag(f("character", "ch", ArgType::String, "The character."))
        .flag(f("startTime", "s", ArgType::Float, "Start of the keys taken."))
        .flag(f("endTime", "end", ArgType::Float, "End of the keys taken."))
        .flag(f("st", "st", ArgType::Float, "As -s."))
        .flag(f("et", "et", ArgType::Float, "As -end."))
        .flag(f("scheduleClip", "sc", ArgType::Bool, "Schedule it as well as making the source (on by default)."))
        .flag(f("scheduleStart", "ss", ArgType::Float, "Where to schedule it (where the keys were)."))
        .flag(f("sourceClipName", "scn", ArgType::String, "The source clip's name (the clip's plus Source)."))
        .flag(f("leaveKeys", "lk", ArgType::Bool, "Copy the keys rather than move them."))
        .flag(f("allRelative", "rel", ArgType::Bool, "The clip adds to what is under it."))
        .flag(f("allAbsolute", "abs", ArgType::Bool, "The clip replaces what is under it (the default)."))
        .flag(f("allClips", "ac", ArgType::Bool, "Query: the scheduled clips."))
        .flag(f("allSourceClips", "asc", ArgType::Bool, "Query: the source clips."))
        .flag(f("remove", "rm", ArgType::Bool, "Take the clip off the schedule.")),
    );

    c.register(
        query_flags(Cmd::new(
            "clipSchedule",
            "A character's schedule. -q gives one line per clip: name, index, start, source start, source end, scale, pre cycle, post cycle, hold. -e with -n or -ci changes a clip \
             (-s, -sc, -ss, -se, -c, -w, -t, -en, -m, -so) or -rm removes it; -ins schedules a source clip again.",
            clip_schedule,
        ))
        .positional("the scheduler, or its character")
        .flag(f("name", "n", ArgType::String, "The clip."))
        .flag(f("clipIndex", "ci", ArgType::Int, "The clip, by index."))
        .flag(f("start", "s", ArgType::Float, "When it starts."))
        .flag(f("sourceStart", "ss", ArgType::Float, "The first of its source used."))
        .flag(f("sourceEnd", "se", ArgType::Float, "The last of its source used."))
        .flag(f("scale", "sc", ArgType::Float, "Its speed: 2 takes twice as long."))
        .flag(f("cycle", "c", ArgType::Float, "How many times it plays."))
        .flag(f("preCycle", "prc", ArgType::Float, "How many times it plays before its start."))
        .flag(f("weight", "w", ArgType::Float, "Its weight."))
        .flag(f("track", "t", ArgType::Int, "Its track."))
        .flag(f("shift", "sh", ArgType::Int, "Move it this many tracks."))
        .flag(f("hold", "h", ArgType::Float, "Frames held at the end."))
        .flag(f("postHold", "ph", ArgType::Float, "Frames held after."))
        .flag(f("preHold", "prh", ArgType::Float, "Frames held before."))
        .flag(f("enable", "en", ArgType::Bool, "Plays or not."))
        .flag(f("mute", "m", ArgType::Bool, "Muted or not."))
        .flag(f("solo", "so", ArgType::Bool, "Soloed or not."))
        .flag(f("lock", "l", ArgType::Bool, "Locked or not."))
        .flag(f("remove", "rm", ArgType::Bool, "Take it off the schedule."))
        .flag(f("instance", "ins", ArgType::String, "Schedule this source clip."))
        .flag(f("insertTrack", "it", ArgType::Int, "Open a track here."))
        .flag(f("removeTrack", "rt", ArgType::Int, "Remove this track and its clips."))
        .flag(f("removeEmptyTracks", "ret", ArgType::Bool, "Close up empty tracks."))
        .flag(f("allRelative", "rel", ArgType::Bool, "It adds to what is under it."))
        .flag(f("allAbsolute", "abs", ArgType::Bool, "It replaces what is under it.")),
    );

    c.register(
        query_flags(Cmd::new(
            "pose",
            "Keep a character's current values as a pose (-n names it), or apply one (-a -n). -q -ap lists them. Returns the pose's name.",
            pose,
        ))
        .positional("the character (the current one when none)")
        .flag(f("name", "n", ArgType::String, "The pose."))
        .flag(f("apply", "a", ArgType::Bool, "Apply the pose."))
        .flag(f("allPoses", "ap", ArgType::Bool, "Query: all poses.")),
    );
}

// ---------------------------------------------------------------------------------------------------------------------
// Scene files: what a character holds, by member position, for the file's layout (see ma_file/characters.rs).

/// A character as a file keeps it. Source clips' curves are keyed by member position, not by element.
pub(crate) struct Saved {
    pub members: Vec<PlugRef>,
    /// The library's name and its source clips: {name: {start, end, curves: {position: keys}}}.
    pub library: Option<(String, Value)>,
    /// The scheduler's name and its clips, as `clipSchedule` keeps them.
    pub scheduler: Option<(String, Value)>,
    pub has_poses: bool,
}

pub(crate) fn saved(g: &Graph, c: NodeId) -> Saved {
    let list: Vec<((i32, String, i32), PlugRef)> = member_list(g, c).into_iter().filter_map(|m| member_plug(g, c, &m).map(|p| (m, p))).collect();
    let position: std::collections::HashMap<String, usize> = list.iter().enumerate().map(|(k, (m, _))| (m.0.to_string(), k)).collect();
    let library = library_of(g, c).map(|l| {
        let mut sources = node_json(g, l, LIB_SOURCES);
        if let Some(o) = sources.as_object_mut() {
            for src in o.values_mut() {
                let curves: Map<String, Value> = src.get("curves").and_then(Value::as_object).map(|cs| cs.iter().filter_map(|(e, k)| Some((position.get(e)?.to_string(), k.clone()))).collect()).unwrap_or_default();
                src["curves"] = Value::Object(curves);
            }
        }
        (g.path(l), sources)
    });
    let has_poses = library_of(g, c).is_some_and(|l| node_json(g, l, LIB_POSES).as_object().is_some_and(|o| !o.is_empty()));
    let scheduler = scheduler_of(g, c).map(|x| (g.path(x), node_json(g, x, SCHED_CLIPS)));
    Saved { members: list.into_iter().map(|(_, p)| p).collect(), library, scheduler, has_poses }
}


/// Whether a plug is a clip channel's base: the keys left outside the clips.
pub(crate) fn is_channel_base(g: &Graph, p: &PlugRef) -> bool {
    is_clip_channel(g, p.node) && p.attr == CH_BASE as i32
}

/// Makes a character from a file: its members, then its library and scheduler under the file's names, with the
/// channels that play the clips. Source clips' curves come keyed by member position.
pub(crate) fn restore(g: &mut Graph, name: &str, plugs: &[PlugRef], library: Option<(String, Value)>, scheduler: Option<(String, Value)>) -> CmdResult<NodeId> {
    let t = types::find("character").unwrap();
    let c = g.create_node(&t, name, NO_NODE, true)?;
    add_members(g, c, plugs)?;
    if library.is_none() && scheduler.is_none() {
        return Ok(c);
    }
    let (lib, sched) = ensure_clip_nodes(g, c)?;
    let elements: Vec<String> = member_list(g, c).iter().map(|m| m.0.to_string()).collect();
    if let Some((lname, mut sources)) = library {
        if let Some(o) = sources.as_object_mut() {
            for src in o.values_mut() {
                let curves: Map<String, Value> = src.get("curves").and_then(Value::as_object).map(|cs| cs.iter().filter_map(|(k, keys)| Some((elements.get(k.parse::<usize>().ok()?)?.clone(), keys.clone()))).collect()).unwrap_or_default();
                src["curves"] = Value::Object(curves);
            }
        }
        set_json(g, lib, LIB_SOURCES, sources)?;
        let _ = g.rename(lib, &lname);
    }
    let has_clips = scheduler.as_ref().is_some_and(|(_, c)| c.as_array().is_some_and(|a| !a.is_empty()));
    if let Some((sname, clips)) = scheduler {
        set_json(g, sched, SCHED_CLIPS, clips)?;
        let _ = g.rename(sched, &sname);
    }
    if has_clips {
        wire_channels(g, c, lib, sched)?;
    }
    Ok(c)
}
