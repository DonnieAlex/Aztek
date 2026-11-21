//! Smaller animation commands: timeWarp, setKeyPath, selectKeyframe, tickDrawSpecial, recordAttr, bakeSimulation,
//! animDisplay, keyingGroup, characterMap. bakeClip and clipMatching are in characters.rs, beside what they read.
//!
//! Differs from the reference: recordAttr marks attributes for scripts to read back; nothing records them, as there's no `record`.
//! A time warp's frames interpolate linearly, whatever -it asks for.

use std::any::Any;
use std::sync::Arc;

use aztek_geo::nurbs::NurbsCurve;
use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use glam::DVec3;
use serde_json::{json, Value};

use crate::anim;
use crate::graph::Graph;
use crate::ref_core::{asked, f};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::rig::{find, plug};
use crate::session::Session;
use crate::system_cmds::{q_flags, setting_store, Def};
use crate::transform::Transform;

type Cmd = CommandDef<Session>;
use ArgType::{Bool as B, Int as I, String as S};

// ---------------------------------------------------------------------------------------------------------------------
// timeWarp

const TW_APPLY: usize = 0;
const TW_ORIG: usize = 1;
const TW_WARPED: usize = 2;
const TW_OUT: usize = 4;

struct TimeWarpNode;

impl NodeBehavior for TimeWarpNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let list: Vec<Value> = if d.bool(TW_APPLY) {
            let warped = d.elements(TW_WARPED).to_vec();
            d.elements(TW_ORIG)
                .iter()
                .filter_map(|(i, o)| {
                    let w = warped.iter().find(|(j, _)| j == i).map(|(_, x)| x)?;
                    match (o, w) {
                        (PlugValue::Float(a), PlugValue::Float(b)) => Some(json!([a, b])),
                        _ => None,
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        d.set_output(TW_OUT, PlugValue::Data(Some(Arc::new(crate::rig::JsonData(Value::Array(list))))));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The frame a curve reads at scene frame t: what the warp shows at t. Frames it marks move from orig to warped, so the
/// key at orig plays at warped. Past either end the offset of that end carries on.
pub fn warped_frame(warp: &Value, t: f64) -> f64 {
    let Some(list) = warp.as_array().filter(|l| l.len() >= 2) else { return t };
    let mut pts: Vec<(f64, f64)> = list.iter().filter_map(|p| Some((p.get(0)?.as_f64()?, p.get(1)?.as_f64()?))).collect();
    if pts.len() < 2 {
        return t;
    }
    pts.sort_by(|a, b| a.1.total_cmp(&b.1));
    let (first, last) = (pts[0], pts[pts.len() - 1]);
    if t <= first.1 {
        return t - first.1 + first.0;
    }
    if t >= last.1 {
        return t - last.1 + last.0;
    }
    for w in pts.windows(2) {
        let ((o0, w0), (o1, w1)) = (w[0], w[1]);
        if t <= w1 && w1 > w0 {
            return o0 + (o1 - o0) * (t - w0) / (w1 - w0);
        }
    }
    t
}

fn is_warp(g: &crate::graph::Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.ty().name == "timeWarp")
}

fn frames_of(g: &crate::graph::Graph, tw: NodeId, attr: usize) -> Vec<f64> {
    g.node(tw).unwrap().elements(attr).iter().filter_map(|(_, v)| if let PlugValue::Float(x) = v { Some(*x) } else { None }).collect()
}

fn set_frames(g: &mut crate::graph::Graph, tw: NodeId, attr: usize, list: &[f64]) -> CmdResult<()> {
    g.set_elements(&PlugRef::new(tw, attr as i32), &list.iter().map(|&x| PlugValue::Float(x)).collect::<Vec<_>>())?;
    Ok(())
}

fn time_warp(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let g = &mut s.graph;
    let warp_named = |g: &crate::graph::Graph, list: &[String]| -> CmdResult<NodeId> {
        for n in list {
            let id = find(g, n, "timeWarp")?;
            if is_warp(g, id) {
                return Ok(id);
            }
        }
        fail!("timeWarp: name a timeWarp node")
    };
    if a.bool("query") {
        let tw = warp_named(g, &names)?;
        if asked(a, "frame") {
            return Ok(json!(frames_of(g, tw, TW_ORIG)));
        }
        fail!("timeWarp: query what?");
    }
    if a.bool("edit") {
        let tw = warp_named(g, &names)?;
        let (mut orig, mut warped) = (frames_of(g, tw, TW_ORIG), frames_of(g, tw, TW_WARPED));
        if let Some(fr) = a.get("frame") {
            for x in nums(fr) {
                orig.push(x);
                warped.push(x);
            }
            let mut pairs: Vec<(f64, f64)> = orig.iter().copied().zip(warped.iter().copied()).collect();
            pairs.sort_by(|x, y| x.0.total_cmp(&y.0));
            orig = pairs.iter().map(|p| p.0).collect();
            warped = pairs.iter().map(|p| p.1).collect();
        }
        if let Some(l) = a.get("moveFrame") {
            let v = nums(l);
            let [i, to] = v.as_slice() else { fail!("timeWarp: -mf takes an index and a frame") };
            let Some(w) = warped.get_mut(*i as usize) else { fail!("timeWarp: no warp frame {i}") };
            *w = *to;
        }
        if a.has("deleteFrame") {
            let i = a.int("deleteFrame", -1) as usize;
            if i >= orig.len() {
                fail!("timeWarp: no warp frame {i}");
            }
            orig.remove(i);
            warped.remove(i);
        }
        g.open_chunk("timeWarp");
        let r = set_frames(g, tw, TW_ORIG, &orig).and_then(|_| set_frames(g, tw, TW_WARPED, &warped));
        g.close_chunk();
        r?;
        return Ok(json!(g.path(tw)));
    }
    // The curves: named, keyed on what's named, or with -g every one in the scene.
    let mut curves: Vec<NodeId> = Vec::new();
    if a.bool("global") {
        curves.extend(g.nodes().into_iter().filter(|&n| anim::is_curve(g, n)));
    }
    for n in &names {
        let id = find(g, n, "timeWarp")?;
        if anim::is_curve(g, id) {
            curves.push(id);
        } else {
            curves.extend(anim::curves_on(g, id).into_iter().map(|(_, c)| c));
        }
    }
    if curves.is_empty() {
        fail!("timeWarp: name animated objects or curves");
    }
    let frames: Vec<f64> = a.get("frame").map(nums).unwrap_or_default();
    let frames = if frames.is_empty() {
        let r = anim::range();
        vec![r.min, r.max]
    } else {
        frames
    };
    let t = types::find("timeWarp").unwrap();
    g.open_chunk("timeWarp");
    let r = (|| -> CmdResult<NodeId> {
        let tw = g.create_node(&t, &a.string("name", "timeWarp1"), NO_NODE, true)?;
        set_frames(g, tw, TW_ORIG, &frames)?;
        set_frames(g, tw, TW_WARPED, &frames)?;
        for c in curves {
            g.connect(&plug(tw, TW_OUT), &plug(c, anim::TIME_WARP), true)?;
        }
        Ok(tw)
    })();
    g.close_chunk();
    let tw = r?;
    g.evaluate();
    Ok(json!(g.path(tw)))
}

fn nums(v: &Value) -> Vec<f64> {
    match v {
        Value::Array(l) => l.iter().filter_map(Value::as_f64).collect(),
        x => x.as_f64().into_iter().collect(),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// setKeyPath

/// The curve a motion path follows.
fn path_curve(g: &crate::graph::Graph, mp: NodeId) -> Option<NodeId> {
    g.connections().into_iter().find(|(d, s)| d.node == mp && g.node(s.node).is_some_and(|n| n.ty().is_a("nurbsCurve"))).map(|(_, s)| s.node)
}

fn set_key_path(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    if names.is_empty() {
        fail!("setKeyPath: name or select an object");
    }
    let frame = anim::frame(&s.graph);
    let mut out = Vec::new();
    s.graph.open_chunk("Set Key Path");
    let r = (|| -> CmdResult<()> {
        for n in &names {
            let g = &mut s.graph;
            let id = find(g, n, "setKeyPath")?;
            g.evaluate();
            let at = g.node(id).map_or(DVec3::ZERO, |x| x.matrix(Transform::WORLD_MATRIX).w_axis.truncate());
            let mp = g.source(&plug(id, Transform::TRANSLATE)).filter(|p| g.node(p.node).is_some_and(|x| x.ty().name == "motionPath")).map(|p| p.node);
            let (curve_shape, mp) = match mp {
                Some(mp) => (path_curve(g, mp), mp),
                None => {
                    let line = NurbsCurve::open(1, vec![at, at + DVec3::X * 1e-3]);
                    let (tr, sh) = crate::curves::create_curve(g, "curve1", line, false)?;
                    let path = g.path(tr);
                    let name = g.path(id);
                    let made = s.run("pathAnimation", json!({"args": [name], "curve": path, "fractionMode": false, "follow": false}))?;
                    let g = &mut s.graph;
                    let Some(mp) = made.as_str().and_then(|m| g.find(m).ok().flatten()) else { fail!("setKeyPath: no motion path") };
                    (Some(sh), mp)
                }
            };
            let g = &mut s.graph;
            let Some(sh) = curve_shape else { fail!("setKeyPath: the motion path on {n} has no curve") };
            let u_plug = crate::motion_path::u_value_plug(mp);
            // The points so far: the path at each key, with this one in time order.
            let old = crate::curves::world_curve(g, sh).unwrap_or_default();
            let mut pts: Vec<(f64, DVec3)> = Vec::new();
            if let Some(c) = anim::curve_of(g, &u_plug) {
                for k in anim::keys_of(g, c).keys {
                    pts.push((k.t, old.eval(k.v)));
                }
            }
            pts.retain(|p| (p.0 - frame).abs() > 1e-6);
            pts.push((frame, at));
            pts.sort_by(|x, y| x.0.total_cmp(&y.0));
            let positions: Vec<DVec3> = pts.iter().map(|p| p.1).collect();
            let curve = match positions.len() {
                1 => NurbsCurve::open(1, vec![at, at + DVec3::X * 1e-3]),
                n => NurbsCurve::through(if n >= 4 { 3 } else { 1 }, &positions),
            };
            let params: Vec<f64> = positions.iter().map(|p| curve.closest_param(*p)).collect();
            crate::curves::set_curve_data(g, sh, curve)?;
            if let Some(c) = anim::curve_of(g, &u_plug) {
                g.delete_node(c)?;
            }
            for ((t, _), u) in pts.iter().zip(&params) {
                anim::key_plug(g, &u_plug, *t, Some(*u), Some(anim::Tangent::Linear), Some(anim::Tangent::Linear))?;
            }
            out.push(json!(g.path(crate::dag::transform_of(g, sh))));
            out.push(json!(g.path(mp)));
        }
        Ok(())
    })();
    s.graph.close_chunk();
    r?;
    s.graph.evaluate();
    Ok(Value::Array(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// recordAttr, tickDrawSpecial

fn record_attr(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let attrs: Vec<String> = match a.get("attribute") {
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Some(Value::String(t)) => vec![t.clone()],
        _ => Vec::new(),
    };
    let mut plugs = Vec::new();
    for n in &names {
        if let Some((obj, attr)) = n.split_once('.') {
            plugs.push(format!("{obj}.{attr}"));
            continue;
        }
        if attrs.is_empty() {
            let g = &s.graph;
            let id = find(g, n, "recordAttr")?;
            plugs.extend(anim::keyable_plugs(g, id).iter().map(|p| g.plug_name(p)));
        }
        plugs.extend(attrs.iter().map(|x| format!("{n}.{x}")));
    }
    let store = &mut s.extras.recording;
    if a.bool("query") {
        return Ok(json!(store));
    }
    if a.bool("delete") {
        store.retain(|p| !plugs.contains(p));
    } else {
        for p in plugs {
            if !store.contains(&p) {
                store.push(p);
            }
        }
    }
    Ok(Value::Null)
}

fn tick_draw_special(s: &mut Session, a: &Args) -> CmdResult {
    let mut call = json!({"args": a.named()});
    let on = a.get("special").is_none_or(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
    call["tickDrawSpecial"] = json!(on);
    if let Some(t) = a.get("time") {
        call["time"] = t.clone();
    }
    s.run("keyframe", call)?;
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// keyingGroup

/// keyingGroup -setActiveFilter: which groups keying an activator keys. AllKeyingGroups (the default), NoKeyingGroups,
/// or a category.
static ACTIVE_FILTER: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn active_filter() -> String {
    let f = ACTIVE_FILTER.lock().unwrap().clone();
    if f.is_empty() {
        "AllKeyingGroups".into()
    } else {
        f
    }
}

pub(crate) fn is_keying_group(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.ty().name == "keyingGroup")
}

fn kg_attr(g: &Graph, id: NodeId, name: &str) -> PlugRef {
    PlugRef::new(id, g.node(id).and_then(|n| n.ty().attr_index(name)).unwrap_or(0) as i32)
}

fn activators(g: &Graph, kg: NodeId) -> Vec<NodeId> {
    g.elements(&kg_attr(g, kg, "activators")).iter().filter_map(|e| g.source(e)).map(|p| p.node).collect()
}

/// The channels a keying group keys: its members' keyable channels less those it was made without, and its
/// sub-groups' (keyed with it).
pub(crate) fn keying_group_plugs(g: &Graph, kg: NodeId) -> Vec<PlugRef> {
    let mut out = Vec::new();
    let mut seen = Vec::new();
    let mut stack = vec![kg];
    while let Some(k) = stack.pop() {
        if seen.contains(&k) {
            continue;
        }
        seen.push(k);
        let excluded: Vec<String> = g.node(k).map(|n| n.data().string(n.ty().attr_index("excludedChannels").unwrap_or(0)).split_whitespace().map(str::to_string).collect()).unwrap_or_default();
        for m in crate::object_sets::members(g, k) {
            if is_keying_group(g, m.node) {
                stack.push(m.node);
                continue;
            }
            for p in anim::keyable_plugs(g, crate::dag::transform_of(g, m.node)) {
                let name = g.attr(&p).name.clone();
                let dynamic = g.attr(&p).flags & flags::USER != 0;
                let skip = excluded.iter().any(|x| match x.as_str() {
                    "translate" => name.starts_with("translate"),
                    "rotate" => name.starts_with("rotate"),
                    "scale" => name.starts_with("scale"),
                    "visibility" => name == "visibility",
                    "dynamic" => dynamic,
                    _ => false,
                });
                if !skip && !out.contains(&p) {
                    out.push(p);
                }
            }
        }
    }
    out
}

/// The channels keying these nodes also keys: those of the active keying groups they activate.
pub(crate) fn activated_plugs(g: &Graph, nodes: &[NodeId]) -> Vec<PlugRef> {
    let filter = active_filter();
    if filter == "NoKeyingGroups" {
        return Vec::new();
    }
    let mut out = Vec::new();
    for kg in g.nodes().into_iter().filter(|&k| is_keying_group(g, k)) {
        let cat = g.node(kg).map(|n| n.data().string(n.ty().attr_index("category").unwrap()).to_string()).unwrap_or_default();
        if filter != "AllKeyingGroups" && cat != filter {
            continue;
        }
        if activators(g, kg).iter().any(|a| nodes.contains(a)) {
            for p in keying_group_plugs(g, kg) {
                if !out.contains(&p) {
                    out.push(p);
                }
            }
        }
    }
    out
}

const KG_OWN: &[&str] = &["activator", "removeActivator", "category", "setActiveFilter", "minimizeRotation", "excludeDynamic", "excludeRotate", "excludeScale", "excludeTranslate", "excludeVisibility", "copy", "split", "name", "empty", "layer"];

fn keying_group(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let queried: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let asked = |f: &str| queried.iter().any(|x| x == f) || a.get(f) == Some(&json!(true));
    // The global filter, in any mode.
    if a.bool("query") && asked("setActiveFilter") {
        return Ok(json!(active_filter()));
    }
    if let Some(f) = a.get("setActiveFilter").and_then(Value::as_str) {
        *ACTIVE_FILTER.lock().unwrap() = f.to_string();
        if names.is_empty() {
            return Ok(Value::Null);
        }
    }
    let group = |s: &Session, n: &str| -> CmdResult<NodeId> {
        match s.graph.find(n)? {
            Some(id) if is_keying_group(&s.graph, id) => Ok(id),
            _ => fail!("keyingGroup: no keying group named {n}"),
        }
    };
    if a.bool("query") {
        let Some(first) = names.first() else { fail!("keyingGroup: name a keying group") };
        let kg = group(s, first)?;
        let g = &s.graph;
        if asked("activator") {
            return Ok(json!(activators(g, kg).iter().map(|&x| g.path(x)).collect::<Vec<_>>()));
        }
        if asked("category") {
            return Ok(json!(g.node(kg).map(|n| n.data().string(n.ty().attr_index("category").unwrap()).to_string()).unwrap_or_default()));
        }
        if asked("minimizeRotation") {
            return Ok(json!(g.node(kg).map(|n| n.data().bool(n.ty().attr_index("minimizeRotation").unwrap())).unwrap_or(false)));
        }
        let mut call = a.flags.clone();
        call.insert("args".into(), json!(names));
        return s.run("sets", Value::Object(call.into_iter().collect()));
    }
    if a.bool("edit") {
        // Flags naming the group take it; the other names are what they work on.
        let kg_name = ["addElement", "forceElement", "include", "remove", "clear", "flatten", "activator", "removeActivator"].iter().find_map(|f| a.get(f).and_then(Value::as_str).map(str::to_string));
        let Some(target) = kg_name.clone().or_else(|| names.first().cloned()) else { fail!("keyingGroup: name a keying group") };
        let kg = group(s, &target)?;
        let rest: Vec<String> = if kg_name.is_some() { names.clone() } else { names[1..].to_vec() };
        let mut done = false;
        for (flag, add) in [("activator", true), ("removeActivator", false)] {
            if !a.has(flag) {
                continue;
            }
            done = true;
            let picked: Vec<NodeId> = if rest.is_empty() {
                s.graph.selection().to_vec()
            } else {
                rest.iter().filter_map(|n| s.graph.find(n).ok().flatten()).collect()
            };
            let g = &mut s.graph;
            let have = activators(g, kg);
            for id in picked {
                let msg = g.node(id).and_then(|n| n.ty().attr_index("message")).unwrap() as i32;
                if add && !have.contains(&id) {
                    g.connect(&PlugRef::new(id, msg), &kg_attr(g, kg, "activators"), false)?;
                } else if !add {
                    for e in g.elements(&kg_attr(g, kg, "activators")) {
                        if g.source(&e).is_some_and(|p| p.node == id) {
                            g.disconnect(&e);
                            g.remove_element(&e)?;
                        }
                    }
                }
            }
        }
        let g = &mut s.graph;
        if let Some(c) = a.get("category").and_then(Value::as_str) {
            g.set_value(kg_attr(g, kg, "category"), PlugValue::String(c.to_string()))?;
            done = true;
        }
        if let Some(m) = a.get("minimizeRotation") {
            g.set_value(kg_attr(g, kg, "minimizeRotation"), PlugValue::Bool(m.as_bool().unwrap_or_else(|| m.as_f64().unwrap_or(0.0) != 0.0)))?;
            done = true;
        }
        let set_flags: Vec<(String, Value)> = a.flags.iter().filter(|(k, _)| !KG_OWN.contains(&k.as_str()) && !matches!(k.as_str(), "args" | "edit" | "query" | "queried")).map(|(k, v)| (k.clone(), v.clone())).collect();
        if !set_flags.is_empty() {
            let mut call = serde_json::Map::new();
            for (k, v) in set_flags {
                call.insert(k, v);
            }
            call.insert("edit".into(), json!(true));
            call.insert("args".into(), json!(names));
            s.run("sets", Value::Object(call))?;
            done = true;
        }
        let _ = done;
        return Ok(json!(s.graph.path(kg)));
    }
    // Set operations that make nothing: as sets does them.
    if ["anyMember", "isMember", "union", "intersection", "subtract", "isIntersecting"].iter().any(|f| a.has(f)) {
        let mut call = a.flags.clone();
        call.insert("args".into(), json!(names));
        return s.run("sets", Value::Object(call.into_iter().collect()));
    }
    // Create: from the objects named or selected, a copy of a group, or the items split off one.
    let mut members = objects(s, a);
    if let Some(src) = a.get("copy").and_then(Value::as_str) {
        let id = group(s, src).or_else(|_| s.graph.find(src)?.ok_or_else(|| aztek_ops::CommandError::new(format!("keyingGroup: no object named {src}"))))?;
        members = crate::object_sets::format(&s.graph, &crate::object_sets::members(&s.graph, id));
    }
    if a.bool("empty") {
        members.clear();
    }
    let split_from = match a.get("split").and_then(Value::as_str) {
        Some(n) => Some(group(s, n)?),
        None => None,
    };
    let g = &mut s.graph;
    let list = crate::object_sets::parse(g, &members).map_err(aztek_ops::CommandError::new)?;
    let t = types::find("keyingGroup").unwrap();
    let mut excluded = Vec::new();
    for (flag, word) in [("excludeTranslate", "translate"), ("excludeRotate", "rotate"), ("excludeScale", "scale"), ("excludeVisibility", "visibility"), ("excludeDynamic", "dynamic")] {
        if a.bool(flag) {
            excluded.push(word);
        }
    }
    g.open_chunk("keyingGroup");
    let r = (|| -> CmdResult<NodeId> {
        if let Some(from) = split_from {
            crate::object_sets::remove(g, from, &list)?;
        }
        let n = g.create_node(&t, &a.string("name", "keyingGroup1"), NO_NODE, true)?;
        crate::object_sets::add(g, n, &list)?;
        if !excluded.is_empty() {
            g.set_value(kg_attr(g, n, "excludedChannels"), PlugValue::String(excluded.join(" ")))?;
        }
        if let Some(c) = a.get("category").and_then(Value::as_str) {
            g.set_value(kg_attr(g, n, "category"), PlugValue::String(c.to_string()))?;
        }
        if let Some(m) = a.get("minimizeRotation") {
            g.set_value(kg_attr(g, n, "minimizeRotation"), PlugValue::Bool(m.as_bool().unwrap_or_else(|| m.as_f64().unwrap_or(0.0) != 0.0)))?;
        }
        if let Some(t) = a.get("text").and_then(Value::as_str) {
            let p = kg_attr(g, n, "annotation");
            g.set_value(p, PlugValue::String(t.to_string()))?;
        }
        Ok(n)
    })();
    g.close_chunk();
    let n = r?;
    // Colours and the like, as sets keeps them.
    if a.has("channelSetColor") || a.has("channelSetColorIndex") {
        let mut call = serde_json::Map::new();
        call.insert("edit".into(), json!(true));
        call.insert("args".into(), json!([s.graph.path(n)]));
        for k in ["channelSetColor", "channelSetColorIndex"] {
            if let Some(v) = a.get(k) {
                call.insert(k.into(), v.clone());
            }
        }
        s.run("sets", Value::Object(call))?;
    }
    Ok(json!(s.graph.path(n)))
}

// ---------------------------------------------------------------------------------------------------------------------
// characterMap

const MAP_LIST: usize = 0;

fn character_map(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let map_node = names.iter().filter_map(|n| s.graph.find(n).ok().flatten()).find(|&i| s.graph.node(i).is_some_and(|n| n.ty().name == "characterMap"));
    let member_attrs = |s: &Session, id: NodeId| -> Vec<String> {
        let g = &s.graph;
        crate::characters::member_plugs(g, id).iter().map(|p| g.plug_name(p)).collect()
    };
    // Names matched attribute to attribute, for -proposedMapping and a new map.
    let proposed = |s: &Session, one: NodeId, two: NodeId| -> Vec<String> {
        let (x, y) = (member_attrs(s, one), member_attrs(s, two));
        let attr = |p: &str| p.rsplit('.').next().unwrap_or(p).to_string();
        let mut out = Vec::new();
        for p in &x {
            if let Some(q) = y.iter().find(|q| attr(q) == attr(p)) {
                out.push(format!("{p} {q}"));
            }
        }
        out
    };
    let chars: Vec<NodeId> = names.iter().filter_map(|n| s.graph.find(n).ok().flatten()).filter(|&i| crate::characters::is_character(&s.graph, i)).collect();
    if a.bool("proposedMapping") {
        let [one, two] = chars.as_slice() else { fail!("characterMap: name two characters") };
        return Ok(json!(proposed(s, *one, *two)));
    }
    let list = |s: &Session, m: NodeId| -> Vec<String> {
        s.graph.node(m).unwrap().elements(MAP_LIST).iter().filter_map(|(_, v)| if let PlugValue::String(t) = v { Some(t.clone()) } else { None }).collect()
    };
    // Two characters named: the map already pairing their members, if there is one.
    let map_node = map_node.or_else(|| {
        let [one, two] = chars.as_slice() else { return None };
        let (x, y) = (member_attrs(s, *one), member_attrs(s, *two));
        s.graph.nodes().into_iter().filter(|&i| s.graph.node(i).is_some_and(|n| n.ty().name == "characterMap")).find(|&m| {
            let l = list(s, m);
            !l.is_empty() && l.iter().all(|e| {
                let mut it = e.split(' ');
                let (p, q) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
                x.iter().any(|v| v == p) && y.iter().any(|v| v == q)
            })
        })
    });
    if a.bool("query") {
        let Some(m) = map_node else {
            if chars.len() == 2 {
                return Ok(json!([]));
            }
            fail!("characterMap: name a characterMap")
        };
        // -q -m attr: what that attribute is paired with.
        if let Some(at) = a.get("mapping").and_then(Value::as_str).filter(|_| crate::anim::queried(a, &["mapping"]) || a.has("mapping")) {
            let full = s.graph.plug(at).map(|p| s.graph.plug_name(&p)).unwrap_or_else(|_| at.to_string());
            if full.contains('.') {
                let out: Vec<String> = list(s, m).into_iter().filter(|e| e.split(' ').any(|x| x == full)).collect();
                return Ok(json!(out));
            }
        }
        return Ok(json!(list(s, m)));
    }
    let edits = a.get("mapAttr").is_some() || a.get("unmapAttr").is_some() || a.get("mapNode").is_some() || a.get("unmapNode").is_some();
    let m = match map_node {
        Some(m) => m,
        None => {
            let [one, two] = chars.as_slice() else { fail!("characterMap: name two characters") };
            let first = proposed(s, *one, *two);
            let g = &mut s.graph;
            let t = types::find("characterMap").unwrap();
            g.open_chunk("characterMap");
            let n = g.create_node(&t, &a.string("mapping", "characterMap1"), NO_NODE, true);
            let n = n.map_err(aztek_ops::CommandError::from).and_then(|n| {
                g.set_elements(&PlugRef::new(n, MAP_LIST as i32), &first.into_iter().map(PlugValue::String).collect::<Vec<_>>())?;
                Ok(n)
            });
            g.close_chunk();
            n?
        }
    };
    if edits {
        let mut now = list(s, m);
        let pair = |v: Option<&Value>| -> Option<String> {
            let l = v?.as_array()?;
            Some(format!("{} {}", l.first()?.as_str()?, l.get(1)?.as_str()?))
        };
        if let Some(p) = pair(a.get("mapAttr")) {
            if !now.contains(&p) {
                now.push(p);
            }
        }
        if let Some(p) = pair(a.get("unmapAttr")) {
            now.retain(|x| *x != p);
        }
        // -mapNode and -unmapNode: every member attribute of the first node with its namesake on the second. Pairs whose
        // names differ are left by -unmapNode.
        let attr = |p: &str| p.rsplit('.').next().unwrap_or(p).to_string();
        let node = |p: &str| p.rsplit_once('.').map(|x| x.0.to_string()).unwrap_or_default();
        let short = |g: &Graph, n: &str| g.find(n).ok().flatten().map(|i| g.path(i)).unwrap_or_else(|| n.to_string());
        if let Some(l) = a.get("mapNode").and_then(Value::as_array) {
            let (src, dst) = (short(&s.graph, l.first().and_then(Value::as_str).unwrap_or("")), short(&s.graph, l.get(1).and_then(Value::as_str).unwrap_or("")));
            // The characters named, or else every character's members.
            let pool: Vec<NodeId> = if chars.is_empty() { s.graph.nodes().into_iter().filter(|&i| crate::characters::is_character(&s.graph, i)).collect() } else { chars.clone() };
            let all: Vec<String> = pool.iter().flat_map(|&c| member_attrs(s, c)).collect();
            for p in all.iter().filter(|p| node(p) == src) {
                if let Some(q) = all.iter().find(|q| node(q) == dst && attr(q) == attr(p)) {
                    now.retain(|x| x.split(' ').next() != Some(p.as_str()));
                    now.push(format!("{p} {q}"));
                }
            }
        }
        if let Some(l) = a.get("unmapNode").and_then(Value::as_array) {
            let (src, dst) = (short(&s.graph, l.first().and_then(Value::as_str).unwrap_or("")), short(&s.graph, l.get(1).and_then(Value::as_str).unwrap_or("")));
            now.retain(|x| {
                let mut it = x.split(' ');
                let (p, q) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
                !(node(p) == src && node(q) == dst && attr(p) == attr(q))
            });
        }
        s.graph.set_elements(&PlugRef::new(m, MAP_LIST as i32), &now.into_iter().map(PlugValue::String).collect::<Vec<_>>())?;
    }
    Ok(json!([s.graph.path(m)]))
}

// ---------------------------------------------------------------------------------------------------------------------

const ANIM_DISPLAY: &[(&str, &str, ArgType, Def)] = &[
    ("modelUpdate", "upd", S, Def::S("interactive")),
    ("refAnimCurvesEditable", "rae", B, Def::B(false)),
    ("timeCode", "tc", S, Def::S("frame")),
    ("timeCodeOffset", "tco", S, Def::S("00:00:00:00")),
];

fn real_attr(name: &str, short: &str, ty: PlugType, default: PlugValue, fl: u32) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: default, flags: fl, doc: "A setting.".into(), ..Default::default() }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let store = flags::INPUT | flags::STORABLE;
        types::register_type(NodeType {
            name: "timeWarp".into(),
            icon: "chart-line".into(),
            doc: "Moves time for the animation curves it drives: the key at each original frame plays at its warped frame.".into(),
            own_attrs: vec![
                real_attr("apply", "ap", PlugType::Bool, PlugValue::Bool(true), store | flags::KEYABLE),
                real_attr("origFrames", "of", PlugType::Float, PlugValue::Float(0.0), store | flags::ARRAY),
                real_attr("warpedFrames", "wf", PlugType::Float, PlugValue::Float(0.0), store | flags::ARRAY),
                real_attr("interpType", "it", PlugType::Int, PlugValue::Int(1), store | flags::ARRAY),
                crate::curves::curve_attr("warpOutput", "wo", flags::OUTPUT | flags::HIDDEN, "The frames, for the curves."),
            ],
            create: Some(Arc::new(|| Box::new(TimeWarpNode))),
            ..Default::default()
        })
        .expect("timeWarp");
        types::register_type(NodeType {
            name: "characterMap".into(),
            doc: "Pairs the attributes of two characters.".into(),
            own_attrs: vec![real_attr("mapping", "mp", PlugType::String, PlugValue::String(String::new()), store | flags::ARRAY)],
            create: Some(Arc::new(|| Box::new(crate::node::Plain))),
            ..Default::default()
        })
        .expect("characterMap");
        let set = types::find(crate::object_sets::TYPE_NAME).expect("objectSet");
        types::register_type(NodeType {
            name: "keyingGroup".into(),
            base: Some(set.clone()),
            doc: "A set of objects keyed together.".into(),
            own_attrs: vec![
                real_attr("minimizeRotation", "mr", PlugType::Bool, PlugValue::Bool(false), store),
                real_attr("category", "cat", PlugType::String, PlugValue::String(String::new()), store),
                AttrDef { name: "activators".into(), short_name: "act".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::INPUT | flags::ARRAY | flags::MESSAGE | flags::HIDDEN, doc: "The nodes whose keying keys this group.".into(), ..Default::default() },
                real_attr("excludedChannels", "exc", PlugType::String, PlugValue::String(String::new()), store | flags::HIDDEN),
            ],
            create: set.create.clone(),
            ..Default::default()
        })
        .expect("keyingGroup");
    });
}

fn copy_of(c: &mut Commands<Session>, from: &str, name: &str, doc: &str, also: &[(&str, Value)]) {
    let mut def = c.find(from).unwrap_or_else(|| panic!("no command {from}")).clone();
    def.name = name.to_string();
    def.doc = doc.to_string();
    let run = def.run.clone();
    let also: Vec<(String, Value)> = also.iter().map(|(k, v)| (k.to_string(), v.clone())).collect();
    def.run = Arc::new(move |s: &mut Session, a: &Args| {
        let mut call = a.clone();
        for (k, v) in &also {
            if !call.has(k) {
                call.flags.insert(k.clone(), v.clone());
            }
        }
        run(s, &call)
    });
    c.register(def);
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        q_flags(Cmd::new(
            "timeWarp",
            "Warp time for animation curves: names objects or curves, -f the frames to warp at (the playback range by default), -g every curve. With -e: -mf index frame moves one, -df index removes one. -q -f reads them. Returns the node.",
            time_warp,
        ))
        .positional("objects or curves, or the node with -e")
        .flag(f("frame", "f", ArgType::Any, "Frames to warp at, or with -q ask."))
        .flag(f("moveFrame", "mf", ArgType::List, "A warp frame's index and where it goes.").multi(2))
        .flag(f("deleteFrame", "df", I, "A warp frame's index to remove."))
        .flag(f("global", "g", B, "Warp every curve in the scene."))
        .flag(f("interpType", "it", ArgType::List, "Accepted: warps interpolate linearly here.").multi(2))
        .flag(f("name", "n", S, "The node's name.")),
    );
    c.register(Cmd::new("setKeyPath", "Key an object's position on a motion path at the current time: makes the path and curve the first time, adds a point after that. Returns [curve, motionPath].", set_key_path).positional("objects"));
    copy_of(c, "selectKey", "selectKeyframe", "Select keys, as selectKey.", &[]);
    copy_of(c, "bakeResults", "bakeSimulation", "Key channels with what they evaluate to, stepping through time: bakeResults -simulation.", &[("simulation", json!(true))]);
    c.register(
        Cmd::new("tickDrawSpecial", "Draw keys' ticks in the special colour on the time slider: -sp on or off (on), -t which keys.", tick_draw_special)
            .positional("objects or plugs (the selection when none)")
            .flag(f("special", "sp", ArgType::Any, "On or off."))
            .flag(f("time", "t", ArgType::Any, "Which keys.")),
    );
    c.register(
        q_flags(Cmd::new("recordAttr", "Mark attributes for recording, kept for scripts: -at names them, all keyable ones when none; -d takes them off; -q lists them.", record_attr))
            .positional("objects or plugs")
            .flag(f("attribute", "at", ArgType::Any, "The attributes.").multi(1))
            .flag(f("delete", "d", B, "Take them off."))
            .undoable(false),
    );
    setting_store(c, "animDisplay", "How animation shows, kept for scripts: -upd, -tc, -tco, -rae; -q reads.", ANIM_DISPLAY, |s| &mut s.extras.anim_display);
    c.register(
        q_flags(Cmd::new(
            "keyingGroup",
            "A keying group: objects (and groups inside it) keyed as one. Keying an activator (-act) keys the group when the \
             global filter (-fil) lets its category (-cat) through. The set flags work as the sets command's. Returns its name; \
             -q lists it.",
            keying_group,
        ))
        .positional("objects; the group with -e and -q")
        .flag(f("name", "n", S, "The group's name."))
        .flag(f("empty", "em", B, "Make it empty."))
        .flag(f("activator", "act", ArgType::Any, "With -e: the selected nodes become activators; -q lists them."))
        .flag(f("removeActivator", "rac", ArgType::Any, "With -e: the selected nodes stop being activators."))
        .flag(f("category", "cat", S, "The category the active filter matches."))
        .flag(f("setActiveFilter", "fil", S, "AllKeyingGroups (the default), NoKeyingGroups, or the category whose groups are active."))
        .flag(f("minimizeRotation", "mr", B, "Keys on the group's rotations take the shortest way."))
        .flag(f("excludeDynamic", "ed", B, "Made without dynamic attributes."))
        .flag(f("excludeRotate", "er", B, "Made without rotate channels."))
        .flag(f("excludeScale", "es", B, "Made without scale channels."))
        .flag(f("excludeTranslate", "et", B, "Made without translate channels."))
        .flag(f("excludeVisibility", "ev", B, "Made without visibility."))
        .flag(f("addElement", "add", ArgType::Any, "With -e: what to add."))
        .flag(f("include", "in", ArgType::Any, "With -e: what to add, with a warning for what can't go in."))
        .flag(f("forceElement", "fe", ArgType::Any, "With -e: what to add, taken from any other set of its partition."))
        .flag(f("remove", "rm", ArgType::Any, "With -e: what to remove."))
        .flag(f("clear", "cl", ArgType::Any, "With -e: empty the group."))
        .flag(f("flatten", "fl", ArgType::Any, "With -e: groups inside give it their members."))
        .flag(f("anyMember", "am", S, "Whether any of the items is in the group."))
        .flag(f("isMember", "im", S, "Whether all the items are in the group."))
        .flag(f("union", "un", S, "The members of this group and the others."))
        .flag(f("intersection", "int", S, "The members this group shares with the others."))
        .flag(f("subtract", "sub", S, "The members of this group not in the others."))
        .flag(f("isIntersecting", "ii", S, "Whether this group shares members with the others."))
        .flag(f("copy", "cp", S, "A new group holding this one's members."))
        .flag(f("split", "sp", S, "The items leave this group for a new one."))
        .flag(f("text", "t", S, "A note kept with the group."))
        .flag(f("size", "s", B, "Query: how many members."))
        .flag(f("nodesOnly", "no", B, "Query: each node once."))
        .flag(f("ordered", "o", B, "Members in the order they were added."))
        .flag(f("noIntermediate", "ni", B, "Leave out intermediate objects."))
        .flag(f("noWarnings", "nw", B, "No warnings for items that can't be added."))
        .flag(f("afterFilters", "af", B, "With -e on a deformer's set: change the set, not what the deformer acts on."))
        .flag(f("channelSetColor", "csc", ArgType::Vec3, "Its dope sheet colour, used when its colour index is -1."))
        .flag(f("channelSetColorIndex", "coi", I, "Its dope sheet colour by index; -1 uses -channelSetColor."))
        .flag(f("color", "co", I, "Highlight colour, -1 to 7."))
        .flag(f("edges", "eg", B, "Edges only."))
        .flag(f("editPoints", "ep", B, "Edit points only."))
        .flag(f("facets", "fc", B, "Faces only."))
        .flag(f("vertices", "v", B, "Vertices only."))
        .flag(f("renderable", "r", B, "Renderable."))
        .flag(f("noSurfaceShader", "nss", B, "With -r: no default surface shader."))
        .flag(f("layer", "l", B, "Obsolete.")),
    );
    c.register(
        q_flags(Cmd::new("characterMap", "Pair two characters' attributes: name them for a map matched by attribute name, -pm for the pairs alone, -ma and -ua with -e to change it. Returns the node; -q the pairs.", character_map))
            .positional("two characters, or a map")
            .flag(f("mapping", "m", S, "The node's name."))
            .flag(f("proposedMapping", "pm", B, "The pairs it would make."))
            .flag(f("mapAttr", "ma", ArgType::List, "Two plugs to pair.").multi(2))
            .flag(f("unmapAttr", "ua", ArgType::List, "Two plugs to part.").multi(2))
            .flag(f("mapMethod", "mm", S, "Accepted: by name is what it does."))
            .flag(f("mapNode", "mn", ArgType::List, "Two nodes: pair every member attribute of the first with its namesake on the second.").multi(2))
            .flag(f("unmapNode", "umn", ArgType::List, "Two nodes: part the pairs between them whose attribute names match.").multi(2)),
    );
}
