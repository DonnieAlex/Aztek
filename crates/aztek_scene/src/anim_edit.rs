//! Key editing beyond the keys themselves: the key clipboard (copyKey, pasteKey), key selection (selectKey), scaling and
//! snapping keys, infinities, baking, and set driven keys (animCurveUL/UA/UU with their input, and blendWeighted to sum
//! several drivers).

use std::any::Any;
use std::sync::Arc;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::anim::{self, frame, key_plug, keys_of, set_keys, Key, Keys, Tangent, OUTPUT, POST, PRE};
use crate::graph::Graph;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::session::Session;

type Cmd = CommandDef<Session>;

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

fn modes(c: Cmd) -> Cmd {
    c.flag(f("query", "q", ArgType::Bool, "Query mode."))
        .flag(f("edit", "e", ArgType::Bool, "Edit mode."))
        .flag(f("queried", "", ArgType::List, "The flags asked about in query mode."))
}

/// The key clipboard: one entry per channel copied.
#[derive(Clone, Debug, Default)]
pub struct KeyClipboard {
    pub entries: Vec<ClipEntry>,
    pub start: f64,
    pub end: f64,
}

#[derive(Clone, Debug)]
pub struct ClipEntry {
    /// The object the keys came from, its short name (pasteKey -matchByName).
    pub node: String,
    pub attr: String,
    pub keys: Vec<Key>,
    pub pre: i64,
    pub post: i64,
    /// A rotation curve's interpolation as copied (anim::ROT_INTERP_NAMES): 2 and up paste as quaternion curves.
    pub rot_interp: i64,
}

// ---------------------------------------------------------------------------------------------------------------------
// Driven curves

/// The input of animCurveUL/UA/UU: what a driven key curve evaluates at.
pub const INPUT: usize = anim::BASE_ATTRS;

pub struct DrivenCurve;

impl NodeBehavior for DrivenCurve {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let at = d.float(INPUT);
        let v = match d.value(anim::KEYS) {
            PlugValue::Data(Some(k)) => k.as_any().downcast_ref::<Keys>().map(|k| k.eval_mode(at, anim::infinity(d.int(PRE)), anim::infinity(d.int(POST)), d.bool(anim::WEIGHTED))),
            _ => None,
        };
        d.set_output(OUTPUT, PlugValue::Float(v.unwrap_or(0.0)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BlendWeighted;

impl NodeBehavior for BlendWeighted {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let sum: f64 = d.elements(0).iter().map(|(_, v)| v.scalar().unwrap_or(0.0)).sum();
        d.set_output(1, PlugValue::Float(sum));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        anim::register_types();
        let base = types::find("animCurve").expect("animCurve");
        for (name, doc) in [
            ("animCurveUL", "Driven keys on a distance channel."),
            ("animCurveUA", "Driven keys on an angle channel."),
            ("animCurveUU", "Driven keys on a unitless channel."),
            ("animCurveUT", "Driven keys on a time channel, in frames."),
        ] {
            types::register_type(NodeType {
                name: name.into(),
                base: Some(base.clone()),
                time_dependent: false,
                doc: doc.into(),
                own_attrs: vec![AttrDef {
                    name: "input".into(),
                    short_name: "i".into(),
                    ty: PlugType::Float,
                    default_value: PlugValue::Float(0.0),
                    flags: flags::INPUT | flags::STORABLE,
                    doc: "The driver's value.".into(),
                    ..Default::default()
                }],
                create: Some(Arc::new(|| Box::new(DrivenCurve))),
                ..Default::default()
            })
            .expect(name);
        }
        types::register_type(NodeType {
            name: "blendWeighted".into(),
            icon: "chart-line".into(),
            doc: "Sums its inputs: several driven key curves on one channel.".into(),
            own_attrs: vec![
                AttrDef {
                    name: "input".into(),
                    short_name: "i".into(),
                    ty: PlugType::Float,
                    default_value: PlugValue::Float(0.0),
                    flags: flags::INPUT | flags::ARRAY,
                    doc: "The values summed.".into(),
                    ..Default::default()
                },
                AttrDef {
                    name: "output".into(),
                    short_name: "o".into(),
                    ty: PlugType::Float,
                    default_value: PlugValue::Float(0.0),
                    flags: flags::OUTPUT,
                    doc: "The sum.".into(),
                    ..Default::default()
                },
            ],
            create: Some(Arc::new(|| Box::new(BlendWeighted))),
            ..Default::default()
        })
        .expect("blendWeighted");
    });
}

pub fn is_driven_curve(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| matches!(n.ty().name.as_str(), "animCurveUL" | "animCurveUA" | "animCurveUU"))
}

/// A driven curve on a plug coming straight or through a blendWeighted, from this driver.
fn driven_curve_for(g: &Graph, driven: &PlugRef, driver: &PlugRef) -> Option<NodeId> {
    let matches = |g: &Graph, c: NodeId| is_driven_curve(g, c) && g.source(&PlugRef::new(c, INPUT as i32)).is_some_and(|s| s == *driver);
    let s = g.source(driven)?;
    if matches(g, s.node) {
        return Some(s.node);
    }
    if g.node(s.node).is_some_and(|n| n.ty().name == "blendWeighted") {
        for e in g.elements(&PlugRef::new(s.node, 0)) {
            if let Some(src) = g.source(&e) {
                if matches(g, src.node) {
                    return Some(src.node);
                }
            }
        }
    }
    None
}

fn scalar(g: &mut Graph, p: &PlugRef) -> f64 {
    g.value(p).ok().and_then(|v| v.scalar()).unwrap_or(0.0)
}

fn split_vec(g: &Graph, p: PlugRef) -> Vec<PlugRef> {
    if g.attr(&p).ty == PlugType::Vec3 && p.component < 0 {
        (0..3).map(|c| PlugRef::component(p.node, p.attr, c)).collect()
    } else {
        vec![p]
    }
}

#[allow(dead_code)]
fn set_driven_key(g: &mut Graph, driven: PlugRef, driver: PlugRef, at: f64, value: f64, itt: Tangent, ott: Tangent) -> CmdResult<NodeId> {
    set_driven_key_as(g, driven, &driven, driver, at, value, itt, ott)
}

/// A driven key on `driven` (the channel, or the pairBlend input its keys go to), its curve named and typed for `channel`.
#[allow(clippy::too_many_arguments)]
fn set_driven_key_as(g: &mut Graph, driven: PlugRef, channel: &PlugRef, driver: PlugRef, at: f64, value: f64, itt: Tangent, ott: Tangent) -> CmdResult<NodeId> {
    let curve = match driven_curve_for(g, &driven, &driver) {
        Some(c) => c,
        None => {
            let a = g.attr(channel);
            let ty = if a.name.starts_with("rotate") { "animCurveUA" } else if a.name.starts_with("translate") { "animCurveUL" } else { "animCurveUU" };
            let t = types::find(ty).unwrap();
            let name = format!("{}_{}", g.node(channel.node).map(|n| n.name.clone()).unwrap_or_default(), anim::channel_name(g, channel));
            let c = g.create_node(&t, &name, NO_NODE, true)?;
            g.connect(&driver, &PlugRef::new(c, INPUT as i32), false)?;
            let out = PlugRef::new(c, OUTPUT as i32);
            match g.source(&driven) {
                None => {
                    g.connect(&out, &driven, false)?;
                }
                Some(s) if g.node(s.node).is_some_and(|n| n.ty().name == "blendWeighted") => {
                    let i = g.elements(&PlugRef::new(s.node, 0)).iter().map(|e| e.element + 1).max().unwrap_or(0);
                    g.connect(&out, &PlugRef::element(s.node, 0, i), false)?;
                }
                Some(s) if is_driven_curve(g, s.node) => {
                    // A second driver: blendWeighted sums the curves, as the reference makes it.
                    let bt = types::find("blendWeighted").unwrap();
                    let b = g.create_node(&bt, "blendWeighted1", NO_NODE, true)?;
                    g.disconnect(&driven);
                    g.connect(&s, &PlugRef::element(b, 0, 0), false)?;
                    g.connect(&out, &PlugRef::element(b, 0, 1), false)?;
                    g.connect(&PlugRef::new(b, 1), &driven, false)?;
                }
                Some(_) => fail!("setDrivenKeyframe: {} is already driven", g.plug_name(&driven)),
            }
            c
        }
    };
    let mut k = keys_of(g, curve);
    match k.keys.iter_mut().find(|e| (e.t - at).abs() < 1e-6) {
        Some(e) => e.v = value,
        None => k.set(Key::new(at, value, itt, ott)),
    }
    set_keys(g, curve, k)?;
    Ok(curve)
}

fn drivers_of(g: &Graph, driven: &PlugRef) -> Vec<PlugRef> {
    let driven = &anim::pair_blend_slot(g, driven).unwrap_or(*driven);
    let Some(s) = g.source(driven) else { return Vec::new() };
    let mut curves = Vec::new();
    if is_driven_curve(g, s.node) {
        curves.push(s.node);
    } else if g.node(s.node).is_some_and(|n| n.ty().name == "blendWeighted") {
        for e in g.elements(&PlugRef::new(s.node, 0)) {
            if let Some(x) = g.source(&e) {
                if is_driven_curve(g, x.node) {
                    curves.push(x.node);
                }
            }
        }
    }
    curves.into_iter().filter_map(|c| g.source(&PlugRef::new(c, INPUT as i32))).collect()
}

/// Plugs driven by keys from a driver plug.
fn driven_by_plug(g: &Graph, driver: &PlugRef) -> Vec<PlugRef> {
    let mut out = Vec::new();
    for d in g.destinations(driver) {
        if !is_driven_curve(g, d.node) {
            continue;
        }
        for x in g.destinations(&PlugRef::new(d.node, OUTPUT as i32)) {
            let ends = if g.node(x.node).is_some_and(|n| n.ty().name == "blendWeighted") { g.destinations(&PlugRef::new(x.node, 1)) } else { vec![x] };
            for e in ends {
                // Through a pairBlend's first input: the channel it drives.
                if g.node(e.node).is_some_and(|n| n.ty().name == "pairBlend") {
                    let out_name = if g.attr(&e).name.starts_with("inRotate") { "outRotate" } else { "outTranslate" };
                    if let Some(oi) = g.node(e.node).and_then(|n| n.ty().attr_index(out_name)) {
                        let o = PlugRef::component(e.node, oi as i32, e.component);
                        let mut fin = g.destinations(&o);
                        if fin.is_empty() {
                            fin = g.destinations(&PlugRef::new(e.node, oi as i32)).into_iter().map(|w| PlugRef::component(w.node, w.attr, e.component)).collect();
                        }
                        out.extend(fin);
                    }
                } else {
                    out.push(e);
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands

fn asked(a: &Args, name: &str) -> bool {
    a.bool(name) || a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == name))
}

/// Curves named by the usual arguments, or the selected keys' curves with -sl.
fn curves_for(s: &Session, a: &Args, cmd: &str) -> CmdResult<Vec<(PlugRef, NodeId)>> {
    anim::target_curves(s, a, cmd)
}

/// The key times of every animation curve upstream of a plug, sorted.
fn upstream_key_times(g: &Graph, p: &PlugRef) -> Vec<f64> {
    let mut times = Vec::new();
    let mut seen: Vec<NodeId> = Vec::new();
    let mut stack: Vec<NodeId> = g.source(p).or_else(|| g.source(&p.whole())).map(|x| x.node).into_iter().collect();
    while let Some(id) = stack.pop() {
        if seen.contains(&id) || seen.len() > 4096 {
            continue;
        }
        seen.push(id);
        if anim::is_curve(g, id) {
            times.extend(keys_of(g, id).keys.iter().map(|k| k.t));
            continue;
        }
        let Some(n) = g.node(id) else { continue };
        for i in 0..n.ty().attrs.len() {
            let at = PlugRef::new(id, i as i32);
            if let Some(src) = g.source(&at) {
                stack.push(src.node);
            }
            for e in g.elements(&at) {
                if let Some(src) = g.source(&e) {
                    stack.push(src.node);
                }
            }
            for c in 0..3 {
                if let Some(src) = g.source(&PlugRef::component(id, i as i32, c)) {
                    stack.push(src.node);
                }
            }
        }
    }
    times.sort_by(f64::total_cmp);
    times.dedup_by(|x, y| (*x - *y).abs() < 1e-6);
    times
}

/// Smart bake: the samples at the wanted times and the range's ends, then, with a tolerance, at the samples where the
/// result between them strays from the full one by more than tol of the channel's range.
fn smart_keys(samples: &[(f64, f64)], wanted: &[f64], tol: Option<f64>) -> Vec<(f64, f64)> {
    if samples.len() < 3 {
        return samples.to_vec();
    }
    let mut keep: Vec<bool> = samples.iter().enumerate().map(|(i, (t, _))| i == 0 || i + 1 == samples.len() || wanted.iter().any(|w| (w - t).abs() < 1e-6)).collect();
    if let Some(tol) = tol {
        let (lo, hi) = samples.iter().fold((f64::MAX, f64::MIN), |(a, b), (_, v)| (a.min(*v), b.max(*v)));
        let limit = tol.abs() * (hi - lo).max(1e-9);
        loop {
            let kept: Vec<usize> = (0..samples.len()).filter(|&i| keep[i]).collect();
            let mut worst: Option<(usize, f64)> = None;
            for w in kept.windows(2) {
                let ((ta, va), (tb, vb)) = (samples[w[0]], samples[w[1]]);
                for j in w[0] + 1..w[1] {
                    let (t, v) = samples[j];
                    let lin = va + (vb - va) * (t - ta) / (tb - ta).max(1e-12);
                    let e = (v - lin).abs();
                    if e > limit && worst.is_none_or(|(_, x)| e > x) {
                        worst = Some((j, e));
                    }
                }
            }
            match worst {
                Some((j, _)) => keep[j] = true,
                None => break,
            }
        }
    }
    samples.iter().zip(keep).filter(|(_, k)| *k).map(|(s, _)| *s).collect()
}

fn copy_keys(s: &mut Session, a: &Args) -> CmdResult {
    let mut curves = curves_for(s, a, "copyKey")?;
    // -animLayer: only the curves the channels have on that layer; none when they have no keys there.
    if let Some(name) = a.get("animLayer").and_then(Value::as_str) {
        let g = &s.graph;
        let layer = match g.find(name)? {
            Some(l) if crate::anim_layers::is_layer(g, l) => l,
            _ => fail!("copyKey: no animation layer named {name}"),
        };
        let plugs: Vec<PlugRef> = match anim::targets(s, a, "copyKey") {
            Ok(p) => p,
            Err(_) => curves.iter().map(|(p, _)| *p).collect(),
        };
        let mut on_layer = Vec::new();
        for p in plugs {
            let found = crate::anim_layers::plug_layer_curves(g, &p).into_iter().find(|(l, _)| *l == layer).and_then(|(_, c)| c);
            // A channel in no layer has its keys on the base layer.
            let found = found.or_else(|| (crate::anim_layers::base_layer(g) == Some(layer) && crate::anim_layers::chain(g, &p).is_empty()).then(|| anim::curve_of(g, &p)).flatten());
            if let Some(c) = found {
                on_layer.push((p, c));
            }
        }
        curves = on_layer;
    }
    let independent = a.bool("forceIndependentEulerAngles");
    let range = anim::time_range(a)?;
    let pick = crate::anim_keys::KeyPick::new(s, a)?;
    let mut clip = KeyClipboard::default();
    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
    let mut n = 0;
    for (p, c) in &curves {
        let k = keys_of(&s.graph, *c);
        let picked: Vec<Key> = k.keys.iter().enumerate().filter(|(i, x)| pick.hits(*c, *i, x)).map(|(_, x)| *x).collect();
        if picked.is_empty() {
            continue;
        }
        n += picked.len();
        for x in &picked {
            lo = lo.min(x.t);
            hi = hi.max(x.t);
        }
        let g = &s.graph;
        let (pre, post) = (g.node(*c).map(|x| x.data().int(PRE)).unwrap_or(0), g.node(*c).map(|x| x.data().int(POST)).unwrap_or(0));
        // Rotation curves under quaternion interpolation stay so on the clipboard unless -fea asks for plain Euler curves.
        let rot_interp = if independent { 0 } else { g.node(*c).map(|x| x.data().int(anim::ROT_INTERP)).unwrap_or(0) };
        clip.entries.push(ClipEntry { node: short_name(g, p.node), attr: anim::channel_name(g, p), keys: picked, pre, post, rot_interp });
    }
    if clip.entries.is_empty() {
        if a.has("animLayer") {
            return Ok(json!(0));
        }
        fail!("copyKey: no keys to copy");
    }
    clip.start = match range {
        Some((a, _)) if a.is_finite() => a,
        _ => lo,
    };
    clip.end = match range {
        Some((_, b)) if b.is_finite() => b,
        _ => hi,
    };
    s.key_clipboard = clip;
    Ok(json!(n))
}

pub(crate) fn short_name(g: &Graph, id: NodeId) -> String {
    g.node(id).map(|n| n.name.rsplit('|').next().unwrap_or("").to_string()).unwrap_or_default()
}

/// The paste modes of pasteKey -option: what happens to the target's keys, and how the clipboard fits a target range.
#[derive(Clone, Copy, PartialEq)]
enum PasteBase {
    Insert,
    Replace,
    ReplaceCompletely,
    Merge,
}

fn paste_mode(o: &str) -> CmdResult<(PasteBase, u8)> {
    // The second value: 0 as copied, 1 scaled to fit the target range, 2 cut to the target range's length.
    Ok(match o {
        "insert" => (PasteBase::Insert, 0),
        "replace" => (PasteBase::Replace, 0),
        "replaceCompletely" => (PasteBase::ReplaceCompletely, 0),
        "merge" => (PasteBase::Merge, 0),
        "scaleInsert" => (PasteBase::Insert, 1),
        "scaleReplace" => (PasteBase::Replace, 1),
        "scaleMerge" => (PasteBase::Merge, 1),
        "fitInsert" => (PasteBase::Insert, 2),
        "fitReplace" => (PasteBase::Replace, 2),
        "fitMerge" => (PasteBase::Merge, 2),
        _ => fail!("pasteKey: unknown option {o}"),
    })
}

/// Whether a curve's input is something other than time: a driven key curve.
fn unitless_input(g: &Graph, c: NodeId) -> bool {
    g.node(c).is_some_and(|n| n.ty().name.starts_with("animCurveU"))
}

fn paste_keys(s: &mut Session, a: &Args) -> CmdResult {
    let clip = s.key_clipboard.clone();
    if clip.entries.is_empty() {
        fail!("pasteKey: the clipboard is empty");
    }
    let (base, fit) = paste_mode(&a.string("option", "insert"))?;
    // Where to paste: the targets' channels, or with -an keys the curves of the active keys.
    let use_keys = a.named().is_empty() && a.string("animation", "keysOrObjects") == "keys";
    let mut targets: Vec<(PlugRef, Option<NodeId>)> = if use_keys {
        let mut b = a.clone();
        b.flags.insert("selected".into(), json!(true));
        curves_for(s, &b, "pasteKey")?.into_iter().map(|(p, c)| (p, Some(c))).collect()
    } else {
        anim::targets(s, a, "pasteKey")?.into_iter().map(|p| (p, None)).collect()
    };
    if targets.is_empty() {
        if use_keys || a.has("animLayer") {
            return Ok(json!(0));
        }
        fail!("pasteKey: name or select where to paste");
    }
    // -animLayer: only the channels' curves on that layer.
    if let Some(name) = a.get("animLayer").and_then(Value::as_str) {
        let g = &s.graph;
        let layer = match g.find(name)? {
            Some(l) if crate::anim_layers::is_layer(g, l) => l,
            _ => fail!("pasteKey: no animation layer named {name}"),
        };
        let mut kept = Vec::new();
        for (p, _) in targets {
            let g = &mut s.graph;
            let in_layer = crate::anim_layers::plug_layer_curves(g, &p).into_iter().find(|(l, _)| *l == layer);
            let found = match in_layer {
                Some((_, Some(c))) => Some(c),
                // In the layer with no keys there yet: a curve on it to paste into.
                Some((_, None)) => {
                    let made = crate::anim_layers::layer_curve_for_key(g, &p, frame(g), Some(layer), 0.0, true)?.map(|x| x.0);
                    if let Some(c) = made {
                        let mut k = keys_of(g, c);
                        k.keys.clear();
                        set_keys(g, c, k)?;
                    }
                    made
                }
                None => (crate::anim_layers::base_layer(g) == Some(layer) && crate::anim_layers::chain(g, &p).is_empty()).then(|| anim::curve_of(g, &p)).flatten(),
            };
            if let Some(c) = found {
                kept.push((p, Some(c)));
            }
        }
        targets = kept;
    }
    let time_range = anim::range_flag(a, "time")?;
    let float_range = anim::range_flag(a, "float")?;
    let index: Option<usize> = match a.get("index") {
        Some(Value::Number(n)) => n.as_u64().map(|x| x as usize),
        Some(Value::String(x)) => x.split(':').next().and_then(|v| v.trim().parse().ok()),
        Some(Value::Array(v)) => v.first().and_then(|x| x.as_u64().or_else(|| x.as_str().and_then(|t| t.split(':').next()?.trim().parse().ok()))).map(|x| x as usize),
        _ => None,
    };
    let upper = a.get("includeUpperBound").and_then(Value::as_bool).unwrap_or(true);
    let (to, fo, vo) = (a.float("timeOffset", 0.0), a.float("floatOffset", 0.0), a.float("valueOffset", 0.0));
    let copies = a.int("copies", 1).max(1);
    let connect = a.bool("connect");
    let (by_name, by_attr) = (a.bool("matchByName"), a.bool("matchByAttr"));
    let span = (clip.end - clip.start).max(0.0);
    let now = frame(&s.graph);
    let mut pasted_curves = 0;
    let mut pasted_keys: Vec<(NodeId, f64)> = Vec::new();
    let mut quat_curves: Vec<(NodeId, i64)> = Vec::new();
    for (i, (p, layer_curve)) in targets.iter().enumerate() {
        let g = &mut s.graph;
        let (node, attr) = (short_name(g, p.node), anim::channel_name(g, p));
        // Which clipboard entry goes here.
        let entry = if by_name {
            clip.entries.iter().find(|e| e.node == node && e.attr == attr)
        } else if by_attr {
            clip.entries.iter().find(|e| e.node == node && e.attr == attr).or_else(|| clip.entries.iter().find(|e| e.attr == attr))
        } else {
            clip.entries.iter().find(|e| e.attr == attr).or_else(|| if clip.entries.len() == 1 { clip.entries.first() } else { None }).or(clip.entries.get(i))
        };
        let Some(entry) = entry else { continue };
        if entry.keys.is_empty() {
            continue;
        }
        let existing = layer_curve.or_else(|| anim::curve_of(g, p));
        let unitless = existing.is_some_and(|c| unitless_input(g, c));
        let (range, offset) = if unitless { (float_range, fo) } else { (time_range, to) };
        // The paste time: the range's start, the key at -index, or the current time.
        let keys_before = existing.map(|c| keys_of(g, c)).unwrap_or_default();
        let start = match (range, index) {
            (Some((lo, _)), _) if lo.is_finite() => lo,
            (_, Some(ix)) => match keys_before.keys.get(ix) {
                Some(k) => k.t,
                None => continue,
            },
            _ if unitless => clip.start,
            _ => now,
        };
        let target_len = range.filter(|(lo, hi)| lo.is_finite() && hi.is_finite() && hi > lo).map(|(lo, hi)| hi - lo);
        // How the clipboard's times land: scaled to the range, or cut to its length.
        let (scale, cut) = match (fit, target_len) {
            (1, Some(l)) if span > 1e-9 => (l / span, None),
            (2, Some(l)) => (1.0, Some(l)),
            _ => (1.0, None),
        };
        let seg = match (fit, target_len) {
            (1 | 2, Some(l)) => l,
            _ => span,
        };
        let block = seg + 1.0;
        let first_t = start + offset;
        // -connect: the pasted keys carry on from the target's value at the paste time.
        let shift_v = if connect {
            let at = existing.map(|c| {
                let d = g.node(c).map(|n| (n.data().int(PRE), n.data().int(POST))).unwrap_or((0, 0));
                keys_of(g, c).eval(first_t, anim::infinity(d.0), anim::infinity(d.1))
            });
            let at = match at {
                Some(v) if !keys_before.keys.is_empty() => v,
                _ => anim::plug_scalar(g, p),
            };
            at - entry.keys[0].v
        } else {
            0.0
        };
        let fresh = existing.is_none();
        if fresh {
            let (t0, v0) = (first_t, entry.keys[0].v + vo + shift_v);
            if !key_plug(g, p, t0, Some(v0), Some(entry.keys[0].itt), Some(entry.keys[0].ott))? {
                continue;
            }
        }
        let Some(curve) = layer_curve.or_else(|| anim::curve_of(g, p)) else { continue };
        let mut k = keys_of(g, curve);
        if fresh {
            k.keys.clear();
        }
        match base {
            PasteBase::ReplaceCompletely => k.keys.clear(),
            PasteBase::Replace => {
                let (lo, hi) = match range.filter(|(lo, hi)| lo.is_finite() && hi.is_finite() && hi > lo) {
                    Some((lo, hi)) => (lo + offset, hi + offset),
                    None => (first_t, first_t + block * copies as f64 - 1.0),
                };
                k.keys.retain(|x| x.t < lo - 1e-6 || (if upper { x.t > hi + 1e-6 } else { x.t >= hi - 1e-6 }));
            }
            PasteBase::Insert => {
                // Keys at and after the paste time make room for what is pasted.
                let shift = block * copies as f64;
                for x in k.keys.iter_mut() {
                    if x.t >= first_t - 1e-6 {
                        x.t += shift;
                    }
                }
            }
            PasteBase::Merge => {}
        }
        for c in 0..copies {
            let off = first_t + c as f64 * block;
            for key in &entry.keys {
                let rel = key.t - clip.start;
                if cut.is_some_and(|l| rel > l + 1e-6) {
                    continue;
                }
                let mut nk = *key;
                nk.t = off + rel * scale;
                nk.v += vo + shift_v;
                if scale != 1.0 {
                    nk.in_slope /= scale;
                    nk.out_slope /= scale;
                }
                k.keys.retain(|x| (x.t - nk.t).abs() > 1e-6);
                k.keys.push(nk);
                pasted_keys.push((curve, nk.t));
            }
        }
        k.keys.sort_by(|x, y| x.t.total_cmp(&y.t));
        set_keys(g, curve, k)?;
        pasted_curves += 1;
        if entry.rot_interp >= 2 && g.attr(p).name.starts_with("rotate") {
            quat_curves.push((curve, entry.rot_interp));
        }
    }
    for (c, mode) in quat_curves {
        let name = s.graph.path(c);
        s.run("rotationInterpolation", json!({"args": [name], "convert": anim::ROT_INTERP_NAMES[mode as usize]}))?;
    }
    if a.bool("selectPasted") {
        s.key_selection = pasted_keys;
    }
    Ok(json!(pasted_curves))
}

/// scaleKey: time curves on the time flags, driven key curves on the float flags, values on both.
fn scale_keys(s: &mut Session, a: &Args) -> CmdResult {
    let curves = curves_for(s, a, "scaleKey")?;
    let pick = crate::anim_keys::KeyPick::new(s, a)?;
    let (vs, vp) = (a.float("valueScale", 1.0), a.float("valuePivot", 0.0));
    let specified = a.get("scaleSpecifiedKeys").and_then(Value::as_bool).unwrap_or(true);
    let snap = match a.get("autoSnap").and_then(Value::as_bool) {
        Some(b) => b,
        None => s.option_vars.get("scaleKeyAutoSnap").is_some_and(|v| v.as_f64().is_some_and(|x| x != 0.0) || v.as_bool() == Some(true)),
    };
    let mut n = 0;
    for (_, c) in curves {
        let unitless = unitless_input(&s.graph, c);
        let (sc, pv, new_lo, new_hi, given) = if unitless {
            (a.float("floatScale", 1.0), a.float("floatPivot", 0.0), a.get("newStartFloat").and_then(Value::as_f64), a.get("newEndFloat").and_then(Value::as_f64), pick.float_range())
        } else {
            (a.float("timeScale", 1.0), a.float("timePivot", 0.0), a.get("newStartTime").and_then(Value::as_f64), a.get("newEndTime").and_then(Value::as_f64), pick.time_range())
        };
        let mut k = keys_of(&s.graph, c);
        let picked: Vec<usize> = (0..k.keys.len()).filter(|&i| pick.hits(c, i, &k.keys[i])).collect();
        if picked.is_empty() {
            continue;
        }
        // The range being scaled: the -t or -f range when bounded, else the picked keys' own.
        let (klo, khi) = (k.keys[picked[0]].t, k.keys[picked[picked.len() - 1]].t);
        let (lo, hi) = match given {
            Some((x, y)) if x.is_finite() && y.is_finite() => (x, y),
            _ => (klo, khi),
        };
        // Into a new range, or by a scale about a pivot.
        let (scale, pivot, to_pivot) = match (new_lo, new_hi) {
            (None, None) => (sc, pv, pv),
            (nl, nh) => {
                let (nl, nh) = (nl.unwrap_or(lo), nh.unwrap_or(hi));
                let s = if hi - lo > 1e-9 { (nh - nl) / (hi - lo) } else { 1.0 };
                (s, lo, nl)
            }
        };
        let place = |t: f64| {
            let t = to_pivot + (t - pivot) * scale;
            if snap {
                t.round()
            } else {
                t
            }
        };
        let (moved_lo, moved_hi) = (place(lo), place(hi));
        let before = k.keys.clone();
        for &i in &picked {
            let key = &mut k.keys[i];
            key.t = place(key.t);
            key.v = vp + (key.v - vp) * vs;
            let ratio = vs / scale.abs().max(1e-9);
            key.in_slope *= ratio;
            key.out_slope *= ratio;
        }
        if !specified {
            // The keys around the range keep their distance from its ends.
            for i in 0..k.keys.len() {
                if picked.contains(&i) {
                    continue;
                }
                let t = before[i].t;
                if t > hi + 1e-6 {
                    k.keys[i].t = t + (moved_hi - hi);
                } else if t < lo - 1e-6 {
                    k.keys[i].t = t + (moved_lo - lo);
                }
            }
        }
        // Scaled keys replace the keys they land on.
        let landed: Vec<f64> = picked.iter().map(|&i| k.keys[i].t).collect();
        let mut i = 0;
        k.keys.retain(|x| {
            let keep = picked.contains(&i) || !landed.iter().any(|t| (t - x.t).abs() < 1e-6);
            i += 1;
            keep
        });
        k.keys.sort_by(|x, y| x.t.total_cmp(&y.t));
        k.keys.dedup_by(|x, y| (x.t - y.t).abs() < 1e-6);
        set_keys(&mut s.graph, c, k)?;
        n += 1;
    }
    Ok(json!(n))
}

/// snapKey: keys never pass a neighbour; two landing on one frame stay apart unless -md merges them.
fn snap_keys(s: &mut Session, a: &Args) -> CmdResult {
    let curves = curves_for(s, a, "snapKey")?;
    let pick = crate::anim_keys::KeyPick::new(s, a)?;
    let tm = a.float("timeMultiple", 1.0);
    let vm = a.float("valueMultiple", 0.0);
    let merge = a.bool("mergeDuplicate");
    let mut clashed = 0;
    for (_, c) in curves {
        let mut k = keys_of(&s.graph, c);
        let mut clash = false;
        let mut drop: Vec<usize> = Vec::new();
        for i in 0..k.keys.len() {
            if !pick.hits(c, i, &k.keys[i]) {
                continue;
            }
            if vm > 0.0 {
                k.keys[i].v = (k.keys[i].v / vm).round() * vm;
            }
            if tm <= 0.0 {
                continue;
            }
            let t = (k.keys[i].t / tm).round() * tm;
            // The neighbours as they stand now: earlier keys are already snapped.
            let prev = (0..i).rev().find(|j| !drop.contains(j)).map(|j| k.keys[j].t);
            let next = k.keys.get(i + 1).map(|x| x.t);
            let past_prev = prev.is_some_and(|p| t < p - 1e-6);
            let past_next = next.is_some_and(|q| t > q + 1e-6);
            let on_prev = prev.is_some_and(|p| (t - p).abs() < 1e-6);
            let on_next = next.is_some_and(|q| (t - q).abs() < 1e-6 && (q / tm - (q / tm).round()).abs() < 1e-9);
            if past_prev || past_next {
                clash = true;
                continue;
            }
            if on_prev || on_next {
                if merge {
                    k.keys[i].t = t;
                    if on_prev {
                        // The later key wins, as with keys moved onto others.
                        if let Some(j) = (0..i).rev().find(|j| !drop.contains(j)) {
                            drop.push(j);
                        }
                    }
                } else {
                    clash = true;
                }
                continue;
            }
            k.keys[i].t = t;
        }
        let mut i = 0;
        k.keys.retain(|_| {
            let keep = !drop.contains(&i);
            i += 1;
            keep
        });
        k.sort();
        set_keys(&mut s.graph, c, k)?;
        if clash {
            clashed += 1;
        }
    }
    Ok(json!(clashed))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();

    c.register(
        crate::anim_keys::pick_flags(Cmd::new("copyKey", "Copy keys of objects or channels to the key clipboard; -t limits them. Returns how many.", copy_keys))
            .undoable(false)
            .positional("objects, plugs or curves (the selection when none)")
            .flag(f("selected", "sl", ArgType::Bool, "Copy the keys selectKey selected."))
            .flag(f("animLayer", "al", ArgType::String, "Only the keys on this animation layer."))
            .flag(f("forceIndependentEulerAngles", "fea", ArgType::Bool, "Rotation curves go to the clipboard as independent Euler curves (off)."))
            .flag(f("option", "o", ArgType::String, "keys (the default) or curve."))
            .flag(f("clipboard", "cb", ArgType::String, "Accepted for existing scripts.")),
    );

    c.register(
        Cmd::new(
            "pasteKey",
            "Paste the key clipboard onto channels at -t (the current time by default). -o insert (the default) moves later keys \
             along, replace clears the pasted range, replaceCompletely clears the curve, merge overlays; the scale and fit forms \
             first scale the clipboard to the -t range or cut it to the range's length. -to and -vo offset, -cp repeats. Returns \
             how many curves took keys.",
            paste_keys,
        )
        .positional("objects or plugs (the selection when none)")
        .flag(f("attribute", "at", ArgType::Any, "Channel name(s).").multi(1))
        .flag(f("time", "t", ArgType::Any, "Where to paste: a time, or a range the scale and fit options fit to."))
        .flag(f("float", "f", ArgType::Any, "Where to paste on driven key curves: a driver value or range, as -t is for time."))
        .flag(f("index", "in", ArgType::Any, "Paste at the time of the key with this index on each curve."))
        .flag(f("includeUpperBound", "iub", ArgType::Bool, "Whether replace takes out keys at the end of the -t range (on)."))
        .flag(f("animation", "an", ArgType::String, "Where the targets come from: objects, keys (the curves of the active keys) or keysOrObjects (the default).").choices(&["objects", "keys", "keysOrObjects"]))
        .flag(f("animLayer", "al", ArgType::String, "Paste only onto the curves the channels have on this layer; none when they have none there."))
        .flag(
            f("option", "o", ArgType::String, "insert, replace, replaceCompletely, merge, scaleInsert, scaleReplace, scaleMerge, fitInsert, fitReplace or fitMerge.")
                .choices(&["insert", "replace", "replaceCompletely", "merge", "scaleInsert", "scaleReplace", "scaleMerge", "fitInsert", "fitReplace", "fitMerge"]),
        )
        .flag(f("timeOffset", "to", ArgType::Float, "Added to the paste time, on time curves."))
        .flag(f("floatOffset", "fo", ArgType::Float, "Added to the paste place, on driven key curves."))
        .flag(f("valueOffset", "vo", ArgType::Float, "Added to the values."))
        .flag(f("copies", "cp", ArgType::Int, "How many times to paste."))
        .flag(f("connect", "c", ArgType::Bool, "Shift the pasted values so they start from the target's value at the paste time (off)."))
        .flag(f("matchByName", "mn", ArgType::Bool, "Paste only where the object and channel names match a clipboard entry's exactly (off)."))
        .flag(f("matchByAttr", "ma", ArgType::Bool, "Paste where object and channel names match, or failing that the channel name alone."))
        .flag(f("selectPasted", "sp", ArgType::Bool, "Select the pasted keys."))
        .flag(f("clipboard", "cb", ArgType::String, "anim (the default) or api: there is one key clipboard.")),
    );

    c.register(modes(crate::anim_keys::pick_flags(
        Cmd::new(
            "selectKey",
            "Select keys, for the commands that take -sl. -t limits them; -add, -rm and -tgl change the selection, -r (the default) \
             replaces it; -cl clears it. -uk picks keys off multiples of a time. Returns how many keys it touched. -q -kc counts them.",
            |s, a| {
                if a.bool("clear") {
                    s.key_selection.clear();
                    return Ok(json!(0));
                }
                if a.bool("query") {
                    return Ok(json!(s.key_selection.len()));
                }
                let curves = curves_for(s, a, "selectKey")?;
                let pick = crate::anim_keys::KeyPick::new(s, a)?;
                let unsnapped = a.get("unsnappedKeys").and_then(Value::as_f64);
                let (add, rm, tgl) = (a.bool("addTo"), a.bool("remove"), a.bool("toggle"));
                if !(add || rm || tgl) {
                    s.key_selection.clear();
                }
                let mut n = 0;
                for (_, c) in curves {
                    for (i, k) in keys_of(&s.graph, c).keys.iter().enumerate() {
                        if !pick.hits(c, i, k) {
                            continue;
                        }
                        if let Some(m) = unsnapped {
                            let m = if m > 0.0 { m } else { 1.0 };
                            if ((k.t / m) - (k.t / m).round()).abs() < 1e-6 {
                                continue;
                            }
                        }
                        let has = s.key_selection.iter().position(|x| x.0 == c && (x.1 - k.t).abs() < 1e-6);
                        match (has, rm, tgl) {
                            (Some(i), true, _) | (Some(i), _, true) => {
                                s.key_selection.remove(i);
                            }
                            (None, false, _) => s.key_selection.push((c, k.t)),
                            _ => {}
                        }
                        n += 1;
                    }
                }
                Ok(json!(n))
            },
        ))
        .undoable(false)
        .positional("objects, plugs or curves (the selection when none)")
        .flag(f("addTo", "add", ArgType::Bool, "Add to the selection."))
        .flag(f("remove", "rm", ArgType::Bool, "Remove from the selection."))
        .flag(f("toggle", "tgl", ArgType::Bool, "Toggle in the selection."))
        .flag(f("replace", "r", ArgType::Bool, "Replace the selection (the default)."))
        .flag(f("clear", "cl", ArgType::Bool, "Clear the selection."))
        .flag(f("keyframe", "k", ArgType::Bool, "Select the keys themselves (the default)."))
        .flag(f("inTangent", "it", ArgType::Bool, "Accepted for existing scripts: selects the keys."))
        .flag(f("outTangent", "ot", ArgType::Bool, "Accepted for existing scripts: selects the keys."))
        .flag(f("unsnappedKeys", "uk", ArgType::Float, "Only keys not on a multiple of this time (1 when 0)."))
        .flag(f("selected", "sl", ArgType::Bool, "Work on the keys already selected."))
        .flag(f("keyframeCount", "kc", ArgType::Bool, "Query: how many selected.")),
    ));

    c.register(
        crate::anim_keys::pick_flags(Cmd::new(
            "scaleKey",
            "Scale keys in time about -tp (-ts), or into -nst to -net; driven key curves about -fp (-fs), or into -nsf to -nef; and \
             values about -vp (-vs). -t, -f and -in pick the keys. Returns how many curves were scaled.",
            scale_keys,
        ))
        .positional("objects, plugs or curves (the selection when none)")
        .flag(f("selected", "sl", ArgType::Bool, "Scale the keys selectKey selected."))
        .flag(f("timeScale", "ts", ArgType::Float, "Time scale."))
        .flag(f("timePivot", "tp", ArgType::Float, "Time pivot."))
        .flag(f("floatScale", "fs", ArgType::Float, "Scale along the driver axis of driven key curves."))
        .flag(f("floatPivot", "fp", ArgType::Float, "Pivot along the driver axis of driven key curves."))
        .flag(f("valueScale", "vs", ArgType::Float, "Value scale."))
        .flag(f("valuePivot", "vp", ArgType::Float, "Value pivot."))
        .flag(f("newStartTime", "nst", ArgType::Float, "Where the picked keys' time range starts after scaling."))
        .flag(f("newEndTime", "net", ArgType::Float, "Where the picked keys' time range ends after scaling."))
        .flag(f("newStartFloat", "nsf", ArgType::Float, "Where the picked keys' driver range starts after scaling, on driven key curves."))
        .flag(f("newEndFloat", "nef", ArgType::Float, "Where the picked keys' driver range ends after scaling, on driven key curves."))
        .flag(f("scaleSpecifiedKeys", "ssk", ArgType::Bool, "Only the picked keys move (on). Off, the keys after them move with the range's end, and those before with its start."))
        .flag(f("autoSnap", "asp", ArgType::Bool, "Snap scaled keys to whole frames (the scaleKeyAutoSnap option var by default).")),
    );

    c.register(
        crate::anim_keys::pick_flags(Cmd::new(
            "snapKey",
            "Move keys to whole multiples of -tm (1 by default), and values to multiples of -vm if given. Keys don't pass their \
             neighbours; -md merges keys that land on one frame. Returns how many curves had keys left unsnapped by a clash.",
            snap_keys,
        ))
        .positional("objects, plugs or curves (the selection when none)")
        .flag(f("selected", "sl", ArgType::Bool, "Snap the keys selectKey selected."))
        .flag(f("timeMultiple", "tm", ArgType::Float, "Snap time to multiples of this."))
        .flag(f("valueMultiple", "vm", ArgType::Float, "Snap value to multiples of this."))
        .flag(f("mergeDuplicate", "md", ArgType::Bool, "Keys that snap onto one frame become one (off).")),
    );

    c.register(modes(
        Cmd::new(
            "setInfinity",
            "What curves do before and after their keys: -pri and -poi constant, linear, cycle, cycleRelative or oscillate.",
            |s, a| {
                // Shapes' channels are taken in unless -s 0.
                let mut b = a.clone();
                if !a.has("shape") {
                    b.flags.insert("shape".to_string(), json!(true));
                }
                let a = &b;
                let curves = curves_for(s, a, "setInfinity")?;
                let index = |n: &str| anim::INFINITY_NAMES.iter().enumerate().find(|(i, x)| **x == n && *i != 2).map(|(i, _)| i as i64);
                if a.bool("query") {
                    let at = if asked(a, "postInfinite") { POST } else { PRE };
                    let v: Vec<Value> = curves
                        .iter()
                        .map(|(_, c)| {
                            let i = s.graph.node(*c).map(|n| n.data().int(at)).unwrap_or(0) as usize;
                            json!(anim::INFINITY_NAMES.get(i).copied().unwrap_or("constant"))
                        })
                        .collect();
                    return Ok(Value::Array(v));
                }
                for (_, c) in curves {
                    for (flag, at) in [("preInfinite", PRE), ("postInfinite", POST)] {
                        if let Some(n) = a.get(flag).and_then(Value::as_str) {
                            let Some(i) = index(n) else { fail!("setInfinity: unknown infinity {n}") };
                            s.graph.set_value(PlugRef::new(c, at as i32), PlugValue::Int(i))?;
                        }
                    }
                }
                Ok(Value::Null)
            },
        )
        .positional("objects, plugs or curves (the selection when none)")
        .flag(f("attribute", "at", ArgType::Any, "Channel name(s)."))
        .flag(f("preInfinite", "pri", ArgType::String, "Before the first key."))
        .flag(f("postInfinite", "poi", ArgType::String, "After the last key."))
        .flag(f("controlPoints", "cp", ArgType::Bool, "Take in the shapes' control points too (off)."))
        .flag(f("hierarchy", "hi", ArgType::String, "Take in the hierarchy: above, below, both or none (none)."))
        .flag(f("shape", "s", ArgType::Bool, "Take in the channels of the shapes under the transforms, not their control points (on).")),
    ));

    c.register(
        Cmd::new(
            "bakeResults",
            "Key channels at every -sb frames over -t with the values they evaluate to, replacing what drove them. -hi below includes \
             the hierarchy, -at the channels; -pok keeps keys outside the range. Returns how many channels.",
            |s, a| {
                let names = objects(s, a);
                // keysOrObjects (the default): the active keys' channels when nothing is named and keys are active.
                let anim_mode = a.string("animation", "keysOrObjects");
                let from_keys = anim_mode == "keys" || (anim_mode == "keysOrObjects" && a.named().is_empty() && !a.has("attribute") && !s.key_selection.is_empty());
                if anim_mode == "keys" && s.key_selection.is_empty() {
                    return Ok(json!(0));
                }
                let (mut plugs, key_span) = if from_keys {
                    let mut call = a.clone();
                    call.flags.insert("animation".into(), json!("keys"));
                    let curves = anim::target_curves(s, &call, "bakeResults")?;
                    let times: Vec<f64> = s.key_selection.iter().filter(|(c, _)| curves.iter().any(|(_, x)| x == c)).map(|(_, t)| *t).collect();
                    let span = times.iter().copied().reduce(f64::min).zip(times.iter().copied().reduce(f64::max));
                    (curves.into_iter().map(|(p, _)| p).collect::<Vec<_>>(), span)
                } else {
                    (anim::targets(s, a, "bakeResults")?, None)
                };
                if a.string("hierarchy", "none") == "below" {
                    let g = &s.graph;
                    for n in &names {
                        if let Ok(Some(id)) = g.find(n.split('.').next().unwrap_or(n)) {
                            for d in g.descendants(id) {
                                if g.node(d).is_some_and(|x| x.ty().is_a("transform")) {
                                    plugs.extend(anim::keyable_plugs(g, d));
                                }
                            }
                        }
                    }
                }
                let r = anim::range();
                // The keyset gives the range: -t, or -f on curves driven by time, or the keys -index picks, or the active
                // keys; the playback range otherwise.
                let index_span = match a.get("index") {
                    Some(_) => {
                        let pick = crate::anim_keys::KeyPick::new(s, a)?;
                        let g = &s.graph;
                        let mut t: Vec<f64> = Vec::new();
                        for p in &plugs {
                            if let Some(c) = anim::curve_of(g, p) {
                                t.extend(keys_of(g, c).keys.iter().enumerate().filter(|(i, k)| pick.hits(c, *i, k)).map(|(_, k)| k.t));
                            }
                        }
                        t.iter().copied().reduce(f64::min).zip(t.iter().copied().reduce(f64::max))
                    }
                    None => None,
                };
                let g = &mut s.graph;
                let given = anim::time_range(a)?.or(anim::range_flag(a, "float")?);
                let (t0, t1) = given
                    .map(|(lo, hi)| (if lo.is_finite() { lo } else { r.min }, if hi.is_finite() { hi } else { r.max }))
                    .or(index_span)
                    .or(key_span)
                    .unwrap_or((r.min, r.max));
                let upper = a.get("includeUpperBound").and_then(Value::as_bool).unwrap_or(true) || (t1 - t0).abs() < 1e-9;
                let by = a.float("sampleBy", 1.0).max(1e-3);
                // -oversamplingRate: the scene steps this many times per -sampleBy; keys go on the -sampleBy frames.
                let over = a.int("oversamplingRate", 1).max(1);
                let now = frame(g);
                let mut samples: Vec<Vec<(f64, f64)>> = vec![Vec::new(); plugs.len()];
                let mut step = 0i64;
                loop {
                    let t = t0 + step as f64 * by / over as f64;
                    if t > t1 + 1e-9 || (!upper && t > t1 - 1e-9 && step > 0) {
                        break;
                    }
                    anim::set_frame(g, t);
                    g.evaluate();
                    if step % over == 0 {
                        for (i, p) in plugs.iter().enumerate() {
                            samples[i].push((t, scalar(g, p)));
                        }
                    }
                    step += 1;
                }
                anim::set_frame(g, now);
                // -smart: keys only where what drove the channel had keys (and the range's ends); a tolerance adds keys
                // where the result strays from that by more than that fraction of the channel's range.
                let smart = match a.get("smart") {
                    Some(Value::Bool(b)) => b.then_some(None),
                    Some(Value::Array(v)) if !v.is_empty() => {
                        let on = v[0].as_bool().unwrap_or_else(|| v[0].as_f64().map(|x| x != 0.0).unwrap_or_else(|| matches!(v[0].as_str(), Some("true" | "on" | "yes" | "1"))));
                        on.then(|| v.get(1).and_then(|x| x.as_f64().or_else(|| x.as_str().and_then(|s| s.parse().ok()))))
                    }
                    Some(v) if v.as_f64().is_some() => (v.as_f64() != Some(0.0)).then_some(None),
                    _ => None,
                };
                if let Some(tol) = smart {
                    for (i, p) in plugs.iter().enumerate() {
                        let mut wanted = upstream_key_times(g, p);
                        wanted.retain(|t| *t >= t0 - 1e-6 && *t <= t1 + 1e-6);
                        if wanted.is_empty() {
                            continue;
                        }
                        samples[i] = smart_keys(&samples[i], &wanted, tol);
                    }
                }
                let keep = a.bool("preserveOutsideKeys");
                let mut done = 0;
                g.open_chunk("Bake");
                let result = (|| -> CmdResult<()> {
                    let layer = crate::anim_layers::prepare_bake(g, &plugs, a)?;
                    for (p, sampled) in plugs.iter().zip(samples) {
                        // A channel in layers: the result goes on the destination, solved under what stays above it.
                        if !crate::anim_layers::chain(g, p).is_empty() || layer.is_some() {
                            let target = layer.or_else(|| crate::anim_layers::base_layer(g));
                            if let Some(t) = target {
                                crate::anim_layers::add_plug(g, t, p)?;
                            }
                            if crate::anim_layers::chain(g, p).is_empty() {
                                continue;
                            }
                            crate::anim_layers::bake_onto(g, p, target, &sampled, keep, (t0, t1))?;
                            done += 1;
                            continue;
                        }
                        // Whatever drove it now gives way to keys.
                        let old = anim::curve_of(g, p);
                        if old.is_none() && g.source(p).is_some() {
                            g.disconnect(p);
                        }
                        if old.is_none() && p.component >= 0 && g.source(&p.whole()).is_some() {
                            // A vector driven whole: split off to per-axis curves.
                            g.disconnect(&p.whole());
                        }
                        if let Some(&(t, v)) = sampled.first() {
                            if !key_plug(g, p, t, Some(v), None, None)? {
                                continue;
                            }
                        }
                        let Some(c) = anim::curve_of(g, p) else { continue };
                        let mut k = keys_of(g, c);
                        if !keep {
                            k.keys.clear();
                        } else {
                            k.keys.retain(|x| x.t < t0 - 1e-6 || x.t > t1 + 1e-6);
                        }
                        let r = anim::range();
                        for (t, v) in sampled {
                            k.keys.push(Key::new(t, v, r.default_in, r.default_out));
                        }
                        k.sort();
                        set_keys(g, c, k)?;
                        done += 1;
                    }
                    Ok(())
                })();
                g.close_chunk();
                result?;
                Ok(json!(done))
            },
        )
        .positional("objects or plugs (the selection when none)")
        .flag(f("attribute", "at", ArgType::Any, "Channel name(s)."))
        .flag(f("time", "t", ArgType::Any, "The range to bake: \"1:100\" (the playback range by default)."))
        .flag(f("sampleBy", "sb", ArgType::Float, "Frames between keys (1)."))
        .flag(f("hierarchy", "hi", ArgType::String, "none or below."))
        .flag(f("preserveOutsideKeys", "pok", ArgType::Bool, "Keep keys outside the baked range."))
        .flag(f("simulation", "sm", ArgType::Bool, "Accepted for existing scripts (always evaluated in time order)."))
        .flag(f("disableImplicitControl", "dic", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("sparseAnimCurveBake", "sac", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("destinationLayer", "dl", ArgType::String, "The layer that takes the result."))
        .flag(f("bakeOnOverrideLayer", "bol", ArgType::Bool, "The result goes on a new override layer on top."))
        .flag(f("removeBakedAttributeFromLayer", "ral", ArgType::Bool, "Baked channels leave their layers, rather than those layers being muted."))
        .flag(f("removeBakedAnimFromLayer", "rba", ArgType::Bool, "Baked animation leaves its layers, rather than those layers being muted."))
        .flag(f("resolveWithoutLayer", "rwl", ArgType::Any, "Layers merged in the bake: only these give way to the result.").multi(1))
        .flag(f("minimizeRotation", "mr", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("controlPoints", "cp", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("shape", "s", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("animation", "an", ArgType::String, "keysOrObjects (the default), keys or objects: keys bakes the active keys' channels over their span.").choices(&["objects", "keys", "keysOrObjects"]))
        .flag(f("float", "f", ArgType::Any, "A range of driver values, as -t is for time."))
        .flag(f("index", "in", ArgType::Any, "Keys by index: the bake spans them."))
        .flag(f("includeUpperBound", "iub", ArgType::Bool, "Whether the range's end is baked (on)."))
        .flag(f("oversamplingRate", "osr", ArgType::Int, "How many times the scene steps per -sampleBy (1)."))
        .flag(f("smart", "sr", ArgType::List, "Smart bake: keys only where the source had keys; an optional tolerance adds keys where the result strays.")),
    );

    c.register(modes(
        Cmd::new(
            "setDrivenKeyframe",
            "Set a driven key: at the driver's value (-cd plug, -dv value, or its current one) the channel takes its value (-v, or \
             its current one). -i inserts a key keeping the curve's shape. -q -dr lists a channel's drivers, -q -cd its current \
             driver, -q -dn a driver's driven channels; with -cnt, how many.",
            set_driven_keyframe,
        )
        .positional("the driven object or channels (the selection when none)")
        .flag(f("currentDriver", "cd", ArgType::String, "The driving attribute."))
        .flag(f("driverValue", "dv", ArgType::Any, "The driver's value at this key (its current one by default); used more than once, a key at each.").multi(1))
        .flag(f("value", "v", ArgType::Float, "The channel's value at this key (its current one by default)."))
        .flag(f("attribute", "at", ArgType::Any, "Channel name(s).").multi(1))
        .flag(f("inTangentType", "itt", ArgType::String, "Tangent into the key.").choices(Tangent::NAMES))
        .flag(f("outTangentType", "ott", ArgType::String, "Tangent out of the key.").choices(Tangent::NAMES))
        .flag(f("driver", "dr", ArgType::Bool, "Query: the channel's drivers."))
        .flag(f("driven", "dn", ArgType::Bool, "Query: the driver's driven channels."))
        .flag(f("count", "cnt", ArgType::Bool, "Query: with -dr, -dn or -cd, how many instead of their names."))
        .flag(f("insert", "i", ArgType::Bool, "Insert a key at the driver value keeping the curve's shape; the tangents around it become fixed."))
        .flag(f("insertBlend", "ib", ArgType::Bool, "A translate or rotate channel driven by something other than driven keys gets a pairBlend between that driver and its keys; off, such channels get no key."))
        .flag(f("hierarchy", "hi", ArgType::String, "Take in the hierarchy: above, below, both or none (none).").choices(&["none", "below", "above", "both"]))
        .flag(f("shape", "s", ArgType::Bool, "Take in the channels of the shapes under the transforms, not their control points (on)."))
        .flag(f("controlPoints", "cp", ArgType::Bool, "Take in the shapes' control points too (off).")),
    ));
}

fn set_driven_keyframe(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    if a.bool("query") {
        let g = &s.graph;
        let n = names.first().cloned().or_else(|| g.selection().first().map(|&i| g.path(i))).unwrap_or_default();
        let plugs: Vec<PlugRef> = if n.contains('.') { split_vec(g, g.plug(&n)?) } else { Vec::new() };
        let plugs = if plugs.is_empty() {
            let Some(id) = g.find(&n)? else { fail!("setDrivenKeyframe: no object named {n}") };
            anim::keyable_plugs(g, id)
        } else {
            plugs
        };
        let list: Vec<String> = if asked(a, "driven") {
            plugs.iter().flat_map(|p| driven_by_plug(g, p)).map(|x| g.plug_name(&x)).collect()
        } else {
            // -dr and -cd: the attributes driving the channel.
            let mut v: Vec<String> = Vec::new();
            for p in &plugs {
                for x in drivers_of(g, p) {
                    let name = g.plug_name(&x);
                    if !v.contains(&name) {
                        v.push(name);
                    }
                }
            }
            v
        };
        return Ok(if asked(a, "count") { json!(list.len()) } else { json!(list) });
    }
    let g = &mut s.graph;
    let Some(driver) = a.get("currentDriver").and_then(Value::as_str) else { fail!("setDrivenKeyframe: -cd driver.attr") };
    let driver = g.plug(driver)?;
    g.evaluate();
    let ats: Vec<f64> = match a.get("driverValue") {
        Some(Value::Array(v)) => v.iter().filter_map(Value::as_f64).collect(),
        Some(v) => v.as_f64().into_iter().collect(),
        None => vec![scalar(g, &driver)],
    };
    // The driven channels: as setKeyframe takes them, shapes' channels included unless -s 0.
    let mut b = a.clone();
    if !a.has("shape") {
        b.flags.insert("shape".to_string(), json!(true));
    }
    let plugs = anim::targets(s, &b, "setDrivenKeyframe")?;
    if plugs.is_empty() {
        fail!("setDrivenKeyframe: name the driven channels");
    }
    let g = &mut s.graph;
    let itt = anim::tangent_flag(a, "inTangentType")?.unwrap_or(Tangent::Clamped);
    let ott = anim::tangent_flag(a, "outTangentType")?.unwrap_or(Tangent::Clamped);
    let insert = a.bool("insert");
    let blend = a.get("insertBlend").and_then(Value::as_bool);
    let mut out = 0;
    g.open_chunk("Set Driven Key");
    let r = (|| -> CmdResult<()> {
        for p in plugs {
            // A channel some other node drives: a pairBlend takes the keys beside it with -ib, else no key.
            let slot = match pair_blend_slot_for_driven(g, &p) {
                Some(x) => x,
                None => {
                    let src = g.source(&p).or_else(|| if p.component >= 0 { g.source(&p.whole()) } else { None });
                    let other = src.is_some_and(|x| !is_driven_curve(g, x.node) && !anim::is_curve(g, x.node) && !g.node(x.node).is_some_and(|n| n.ty().name == "blendWeighted"));
                    if other {
                        if blend != Some(true) {
                            continue;
                        }
                        match anim::insert_pair_blend(g, &p)? {
                            Some(x) => x,
                            None => continue,
                        }
                    } else {
                        p
                    }
                }
            };
            for &at in &ats {
                if insert {
                    // Keeps the curve as it is; a channel with no driven curve from this driver has nothing to insert into.
                    if let Some(c) = driven_curve_for(g, &slot, &driver) {
                        let mut k = keys_of(g, c);
                        if k.insert(at) {
                            set_keys(g, c, k)?;
                            out += 1;
                        }
                    }
                    continue;
                }
                let v = match a.get("value").and_then(Value::as_f64) {
                    Some(v) if aztek_ops::units::user_call() => aztek_ops::units::to_stored(g.attr(&p).unit, v),
                    Some(v) => v,
                    None => scalar(g, &p),
                };
                set_driven_key_as(g, slot, &p, driver, at, v, itt, ott)?;
                out += 1;
            }
        }
        Ok(())
    })();
    g.close_chunk();
    r?;
    Ok(json!(out))
}

/// A channel whose keys already go through a pairBlend: that input.
fn pair_blend_slot_for_driven(g: &Graph, p: &PlugRef) -> Option<PlugRef> {
    anim::pair_blend_slot(g, p)
}

/// Moving keys carries the breakdowns between them along in proportion, as the reference does: breakdowns between `moved` keys
/// retime to keep their place. `before` is the curve before the move, `after` the moved one, matched by index.
pub fn retime_breakdowns(before: &[Key], after: &mut [Key]) {
    if before.len() != after.len() {
        return;
    }
    let plain: Vec<usize> = (0..before.len()).filter(|&i| !before[i].breakdown).collect();
    for w in plain.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (t0, t1) = (before[a].t, before[b].t);
        let (n0, n1) = (after[a].t, after[b].t);
        if (t1 - t0).abs() < 1e-9 || ((t0 - n0).abs() < 1e-9 && (t1 - n1).abs() < 1e-9) {
            continue;
        }
        for i in a + 1..b {
            // Only breakdowns the user did not move themselves.
            if (after[i].t - before[i].t).abs() < 1e-9 {
                let s = (before[i].t - t0) / (t1 - t0);
                after[i].t = n0 + (n1 - n0) * s;
            }
        }
    }
}
