//! The soft select falloff curve, its presets and the softSelectCurve command.

use aztek_core::{PrefScope, PrefStore};
use aztek_ops::{fail, ArgType, CommandDef, Commands, FlagDef};
use glam::DVec2;
use serde_json::{json, Value};

use crate::session::Session;

/// General prefs keys.
pub const ENABLED: &str = "/softSelect/enabled";
pub const RADIUS: &str = "/softSelect/radius";
pub const CURVE: &str = "/softSelect/curve";
pub const FALLOFF_MODE: &str = "/softSelect/falloffMode";
pub const USER_PRESETS: &str = "/softSelect/userPresets";
/// softSelect -ssf's numbers, in order.
pub const FALLOFFS: [&str; 4] = ["volume", "surface", "global", "object"];

const EPS: f64 = 1e-9;

/// How the segment that starts at a key reaches the next key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interp {
    Linear,
    /// Ease in and out, flat at both keys.
    Smooth,
    /// Monotone cubic through the neighbouring keys.
    #[default]
    Spline,
    /// Hold this key's value up to the next key.
    Flat,
}

pub fn interp_name(i: Interp) -> &'static str {
    match i {
        Interp::Linear => "linear",
        Interp::Smooth => "smooth",
        Interp::Flat => "flat",
        Interp::Spline => "spline",
    }
}

pub fn interp_from_name(name: &str, fallback: Interp) -> Interp {
    match name {
        "linear" => Interp::Linear,
        "smooth" => Interp::Smooth,
        "spline" => Interp::Spline,
        "flat" | "none" | "step" => Interp::Flat,
        _ => fallback,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// 0 at the selection, 1 at the radius.
    pub pos: f64,
    /// Weight, 0..1.
    pub value: f64,
    pub interp: Interp,
}

fn key(pos: f64, value: f64, interp: Interp) -> Key {
    Key { pos, value, interp }
}

fn default_keys() -> Vec<Key> {
    vec![key(0.0, 1.0, Interp::Smooth), key(1.0, 0.0, Interp::Smooth)]
}

/// The reference's soft select falloff curve: weight against normalised distance, through keys with per-key interpolation.
/// Spline segments use Fritsch-Carlson tangents so they never overshoot their keys; the curve's ends have flat
/// tangents. Results are clamped to 0..1. With no keys it is the default smooth falloff.
#[derive(Debug, Clone)]
pub struct Curve {
    keys: Vec<Key>,
    tangents: Vec<f64>,
}

impl PartialEq for Curve {
    fn eq(&self, o: &Self) -> bool {
        self.keys == o.keys
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::new(default_keys())
    }
}

impl Curve {
    pub fn new(keys: Vec<Key>) -> Self {
        let mut c = Curve { keys: if keys.is_empty() { default_keys() } else { keys }, tangents: Vec::new() };
        c.sort_and_clamp();
        c
    }

    pub fn keys(&self) -> &[Key] {
        &self.keys
    }

    fn sort_and_clamp(&mut self) {
        for k in &mut self.keys {
            k.pos = if k.pos.is_finite() { k.pos } else { 0.0 }.clamp(0.0, 1.0);
            k.value = if k.value.is_finite() { k.value } else { 0.0 }.clamp(0.0, 1.0);
        }
        self.keys.sort_by(|a, b| a.pos.partial_cmp(&b.pos).unwrap());
        self.update_tangents();
    }

    fn update_tangents(&mut self) {
        let n = self.keys.len();
        self.tangents = vec![0.0; n];
        // Interior tangents are the weighted harmonic mean of the neighbouring slopes (Fritsch-Butland), zero at
        // extrema.
        for k in 1..n.saturating_sub(1) {
            let (h0, h1) = (self.keys[k].pos - self.keys[k - 1].pos, self.keys[k + 1].pos - self.keys[k].pos);
            if h0 <= EPS || h1 <= EPS {
                continue;
            }
            let d0 = (self.keys[k].value - self.keys[k - 1].value) / h0;
            let d1 = (self.keys[k + 1].value - self.keys[k].value) / h1;
            if d0 * d1 <= 0.0 {
                continue;
            }
            self.tangents[k] = 3.0 * (h0 + h1) / ((2.0 * h1 + h0) / d0 + (h1 + 2.0 * h0) / d1);
        }
    }

    pub fn evaluate(&self, t: f64) -> f64 {
        let (first, last) = (self.keys[0], *self.keys.last().unwrap());
        if self.keys.len() == 1 || t <= first.pos {
            return first.value;
        }
        if t >= last.pos {
            return last.value;
        }
        let k = self.keys.partition_point(|key| key.pos <= t) - 1;
        let (a, b) = (self.keys[k], self.keys[k + 1]);
        let h = b.pos - a.pos;
        if h <= EPS {
            return b.value;
        }
        let u = (t - a.pos) / h;
        let v = match a.interp {
            Interp::Linear => a.value + (b.value - a.value) * u,
            Interp::Smooth => a.value + (b.value - a.value) * u * u * (3.0 - 2.0 * u),
            Interp::Flat => a.value,
            Interp::Spline => {
                let (u2, u3) = (u * u, u * u * u);
                (2.0 * u3 - 3.0 * u2 + 1.0) * a.value
                    + (u3 - 2.0 * u2 + u) * h * self.tangents[k]
                    + (-2.0 * u3 + 3.0 * u2) * b.value
                    + (u3 - u2) * h * self.tangents[k + 1]
            }
        };
        v.clamp(0.0, 1.0)
    }

    /// Index of the new key.
    pub fn add_key(&mut self, mut k: Key) -> usize {
        k.pos = k.pos.clamp(0.0, 1.0);
        k.value = k.value.clamp(0.0, 1.0);
        let at = self.keys.partition_point(|e| e.pos <= k.pos);
        self.keys.insert(at, k);
        self.update_tangents();
        at
    }

    pub fn remove_key(&mut self, index: usize) {
        if self.keys.len() <= 2 || index >= self.keys.len() {
            return;
        }
        self.keys.remove(index);
        self.update_tangents();
    }

    /// Moves a key, keeping the keys sorted; returns its new index.
    pub fn move_key(&mut self, index: usize, pos: f64, value: f64) -> usize {
        if index >= self.keys.len() {
            return index;
        }
        let mut k = self.keys.remove(index);
        k.pos = pos;
        k.value = value;
        self.add_key(k)
    }

    pub fn set_interp(&mut self, index: usize, i: Interp) {
        if let Some(k) = self.keys.get_mut(index) {
            k.interp = i;
        }
    }

    /// [[pos, value, "interp"], ...].
    pub fn to_json(&self) -> Value {
        Value::Array(self.keys.iter().map(|k| json!([k.pos, k.value, interp_name(k.interp)])).collect())
    }

    /// Accepts the to_json form or [{pos, value, interp}, ...]; anything unusable gives the default curve.
    pub fn from_json(j: &Value) -> Curve {
        let Some(list) = j.as_array() else { return Curve::default() };
        let mut keys = Vec::new();
        for e in list {
            let mut k = key(0.0, 0.0, Interp::Spline);
            if let Some(a) = e.as_array().filter(|a| a.len() >= 2 && a[0].is_number() && a[1].is_number()) {
                k.pos = a[0].as_f64().unwrap();
                k.value = a[1].as_f64().unwrap();
                if let Some(s) = a.get(2).and_then(Value::as_str) {
                    k.interp = interp_from_name(s, Interp::Spline);
                }
            } else if let Some(o) = e.as_object().filter(|o| o.get("pos").is_some_and(Value::is_number) && o.get("value").is_some_and(Value::is_number)) {
                k.pos = o["pos"].as_f64().unwrap();
                k.value = o["value"].as_f64().unwrap();
                if let Some(s) = o.get("interp").and_then(Value::as_str) {
                    k.interp = interp_from_name(s, Interp::Spline);
                }
            } else {
                continue;
            }
            keys.push(k);
        }
        if keys.len() >= 2 {
            Curve::new(keys)
        } else {
            Curve::default()
        }
    }
}

fn distance_to_segment(p: DVec2, a: DVec2, b: DVec2) -> f64 {
    let ab = b - a;
    let len2 = ab.length_squared();
    let t = if len2 > 0.0 { ((p - a).dot(ab) / len2).clamp(0.0, 1.0) } else { 0.0 };
    (p - (a + ab * t)).length()
}

/// Ramer-Douglas-Peucker: keeps the endpoints and every point further than epsilon from the simplified polyline.
pub fn simplify(points: &[DVec2], epsilon: f64) -> Vec<DVec2> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    *keep.last_mut().unwrap() = true;
    let mut stack = vec![(0, points.len() - 1)];
    while let Some((lo, hi)) = stack.pop() {
        let mut worst = -1.0;
        let mut at = lo;
        for i in lo + 1..hi {
            let d = distance_to_segment(points[i], points[lo], points[hi]);
            if d > worst {
                worst = d;
                at = i;
            }
        }
        if worst > epsilon {
            keep[at] = true;
            stack.push((lo, at));
            stack.push((at, hi));
        }
    }
    points.iter().zip(keep).filter(|(_, k)| *k).map(|(p, _)| *p).collect()
}

/// A freehand stroke (x = distance, y = weight, both 0..1) as spline keys: points that run backwards in x are
/// dropped, the stroke is extended to x = 0 and 1, then simplified.
pub fn curve_from_stroke(stroke: &[DVec2], epsilon: f64) -> Curve {
    if stroke.is_empty() {
        return Curve::default();
    }
    let mut pts = stroke.to_vec();
    if pts.last().unwrap().x < pts[0].x {
        pts.reverse();
    }
    let mut mono: Vec<DVec2> = Vec::new();
    for p in pts {
        let p = DVec2::new(p.x.clamp(0.0, 1.0), p.y.clamp(0.0, 1.0));
        if mono.last().is_none_or(|m| p.x > m.x + 1e-6) {
            mono.push(p);
        }
    }
    if mono[0].x > 0.0 {
        mono.insert(0, DVec2::new(0.0, mono[0].y));
    }
    if mono.last().unwrap().x < 1.0 {
        let y = mono.last().unwrap().y;
        mono.push(DVec2::new(1.0, y));
    }
    Curve::new(simplify(&mono, epsilon).into_iter().map(|p| key(p.x, p.y, Interp::Spline)).collect())
}

#[derive(Debug, Clone)]
pub struct Preset {
    pub name: String,
    pub curve: Curve,
}

fn spline(pts: &[(f64, f64)]) -> Curve {
    Curve::new(pts.iter().map(|&(p, v)| key(p, v, Interp::Spline)).collect())
}

/// Built-in presets; the first is the default curve.
pub fn builtin_presets() -> Vec<Preset> {
    let p = |name: &str, curve: Curve| Preset { name: name.into(), curve };
    vec![
        p("Smooth", Curve::default()),
        p("Linear", Curve::new(vec![key(0.0, 1.0, Interp::Linear), key(1.0, 0.0, Interp::Linear)])),
        p("Sharp", spline(&[(0.0, 1.0), (0.15, 0.55), (0.5, 0.12), (1.0, 0.0)])),
        p("Dome", spline(&[(0.0, 1.0), (0.5, 0.87), (0.85, 0.5), (1.0, 0.0)])),
        p("Bell", spline(&[(0.0, 1.0), (0.25, 0.8), (0.5, 0.3), (0.75, 0.06), (1.0, 0.0)])),
        p("Spike", spline(&[(0.0, 1.0), (0.08, 0.3), (0.5, 0.03), (1.0, 0.0)])),
        p("Plateau", spline(&[(0.0, 1.0), (0.6, 1.0), (1.0, 0.0)])),
        p("Inverse", Curve::new(vec![key(0.0, 0.0, Interp::Smooth), key(1.0, 1.0, Interp::Smooth)])),
        p("Ring", spline(&[(0.0, 1.0), (0.25, 0.15), (0.5, 0.7), (1.0, 0.0)])),
    ]
}

pub fn find_builtin(name: &str) -> Option<Preset> {
    builtin_presets().into_iter().find(|p| p.name == name)
}

/// The curve in prefs.
pub fn current_curve() -> Curve {
    let j = PrefStore::get().value("general", CURVE, Value::Null);
    if j.is_null() {
        Curve::default()
    } else {
        Curve::from_json(&j)
    }
}

pub fn set_current_curve(c: &Curve) {
    PrefStore::get().set("general", CURVE, c.to_json(), PrefScope::User);
}

fn user_presets_json() -> Vec<Value> {
    PrefStore::get().value("general", USER_PRESETS, json!([])).as_array().cloned().unwrap_or_default()
}

/// Saved user presets, in order.
pub fn user_presets() -> Vec<Preset> {
    user_presets_json()
        .iter()
        .filter_map(|e| {
            let name = e.get("name")?.as_str()?;
            Some(Preset { name: name.into(), curve: Curve::from_json(e.get("keys").unwrap_or(&Value::Null)) })
        })
        .collect()
}

fn name_of(e: &Value) -> Option<&str> {
    e.get("name").and_then(Value::as_str)
}

/// Adds or replaces the named preset.
pub fn save_user_preset(name: &str, c: &Curve) {
    let mut list = user_presets_json();
    let entry = json!({"name": name, "keys": c.to_json()});
    let mut replaced = false;
    for e in &mut list {
        if name_of(e) == Some(name) {
            *e = entry.clone();
            replaced = true;
        }
    }
    if !replaced {
        list.push(entry);
    }
    PrefStore::get().set("general", USER_PRESETS, Value::Array(list), PrefScope::User);
}

pub fn delete_user_preset(name: &str) -> bool {
    let list = user_presets_json();
    let kept: Vec<Value> = list.iter().filter(|e| name_of(e) != Some(name)).cloned().collect();
    if kept.len() == list.len() {
        return false;
    }
    PrefStore::get().set("general", USER_PRESETS, Value::Array(kept), PrefScope::User);
    true
}

pub fn rename_user_preset(from: &str, to: &str) -> bool {
    if to.is_empty() || from == to {
        return false;
    }
    let mut list = user_presets_json();
    if list.iter().any(|e| name_of(e) == Some(to)) {
        return false;
    }
    let mut found = false;
    for e in &mut list {
        if name_of(e) == Some(from) {
            e["name"] = json!(to);
            found = true;
        }
    }
    if found {
        PrefStore::get().set("general", USER_PRESETS, Value::Array(list), PrefScope::User);
    }
    found
}

/// The falloff mode's name.
pub fn falloff_name() -> String {
    PrefStore::get().string("general", FALLOFF_MODE, "volume")
}

/// What the falloff colours depend on past the radius: the mode, the curve, and for Global every selected component.
pub fn signature(g: &crate::graph::Graph, mode: crate::components::ComponentMode) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    let name = falloff_name();
    name.hash(&mut h);
    current_curve().to_json().to_string().hash(&mut h);
    let p = PrefStore::get();
    p.string("general", COLOR_CURVE, DEFAULT_COLOR_CURVE).hash(&mut h);
    p.bool("general", FALSE_COLOR, true).hash(&mut h);
    if name == "global" && mode != crate::components::ComponentMode::Object {
        for (node, ids) in crate::components::selected_all(g, mode) {
            node.hash(&mut h);
            ids.hash(&mut h);
        }
    }
    h.finish()
}

/// Whether the views show the weights in false colour (softSelect -enableFalseColor).
pub fn false_color_enabled() -> bool {
    PrefStore::get().bool("general", FALSE_COLOR, true)
}

/// The false colour ramp of softSelect -softSelectColorCurve: a weight's colour lies between its points by weight,
/// linear, or held where a point's interpolation is 0.
pub struct FalseColors(Vec<(f64, [f64; 3], i64)>);

impl FalseColors {
    pub fn current() -> FalseColors {
        let text = PrefStore::get().string("general", COLOR_CURVE, DEFAULT_COLOR_CURVE);
        let parse = |t: &str| -> Vec<(f64, [f64; 3], i64)> { numbers(t).unwrap_or_default().chunks_exact(5).map(|c| (c[3], [c[0], c[1], c[2]], c[4] as i64)).collect() };
        let mut pts = parse(&text);
        if pts.is_empty() {
            pts = parse(DEFAULT_COLOR_CURVE);
        }
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        FalseColors(pts)
    }

    pub fn at(&self, x: f64) -> [u8; 4] {
        let pts = &self.0;
        let rgb = if x <= pts[0].0 {
            pts[0].1
        } else if x >= pts[pts.len() - 1].0 {
            pts[pts.len() - 1].1
        } else {
            let k = pts.windows(2).position(|w| x >= w[0].0 && x <= w[1].0).unwrap_or(0);
            let (a, b) = (pts[k], pts[k + 1]);
            let t = if a.2 == 0 || b.0 - a.0 <= EPS { 0.0 } else { (x - a.0) / (b.0 - a.0) };
            [a.1[0] + (b.1[0] - a.1[0]) * t, a.1[1] + (b.1[1] - a.1[1]) * t, a.1[2] + (b.1[2] - a.1[2]) * t]
        };
        let c = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        [c(rgb[0]), c(rgb[1]), c(rgb[2]), 255]
    }
}

/// True when falloff distances are measured along the surface rather than straight through space.
pub fn surface_falloff() -> bool {
    PrefStore::get().string("general", FALLOFF_MODE, "volume") == "surface"
}

/// Whether soft selection is on, and its radius.
pub fn state() -> (bool, f64) {
    let p = PrefStore::get();
    (p.bool("general", ENABLED, false), p.f64("general", RADIUS, 5.0))
}

/// softSelect's other settings.
pub const UV_RADIUS: &str = "/softSelect/uvRadius";
pub const COLOR_CURVE: &str = "/softSelect/colorCurve";
pub const FALSE_COLOR: &str = "/softSelect/falseColor";
pub const COMPRESS_UNDO: &str = "/softSelect/compressUndo";
/// The false colour ramp to start with: black where nothing is picked, through red, to yellow at full weight.
pub const DEFAULT_COLOR_CURVE: &str = "0,0,0,0,1,1,0,0,0.5,1,1,1,0,1,1";

/// The reference's numbers for a key's interpolation: 0 none, 1 linear, 2 smooth, 3 spline.
fn interp_code(i: Interp) -> i64 {
    match i {
        Interp::Flat => 0,
        Interp::Linear => 1,
        Interp::Smooth => 2,
        Interp::Spline => 3,
    }
}

fn interp_of_code(c: i64) -> Interp {
    match c {
        0 => Interp::Flat,
        1 => Interp::Linear,
        2 => Interp::Smooth,
        _ => Interp::Spline,
    }
}

fn numbers(text: &str) -> Result<Vec<f64>, String> {
    text.split(',').map(str::trim).filter(|t| !t.is_empty()).map(|t| t.parse::<f64>().map_err(|_| format!("{t} is not a number"))).collect()
}

fn trim_num(v: f64) -> String {
    let t = format!("{v}");
    t
}

/// softSelect -ssc: three numbers per key, the weight, the distance (0 to 1) and the interpolation.
pub fn curve_from_string(text: &str) -> Result<Curve, String> {
    let n = numbers(text)?;
    if n.is_empty() || n.len() % 3 != 0 {
        return Err("three numbers per point: weight, distance, interpolation".into());
    }
    Ok(Curve::new(n.chunks(3).map(|c| key(c[1], c[0], interp_of_code(c[2] as i64))).collect()))
}

pub fn curve_to_string(c: &Curve) -> String {
    c.keys().iter().map(|k| format!("{},{},{}", trim_num(k.value), trim_num(k.pos), interp_code(k.interp))).collect::<Vec<_>>().join(",")
}

pub fn register(c: &mut Commands<Session>) {
    type T = ArgType;
    let f = |name: &str, short: &str, ty, doc: &str| FlagDef::new(name, short, ty, Value::Null, doc);
    c.register(
        CommandDef::new(
            "softSelect",
            "Soft selection (B): -sse 1 turns it on, -ssd sets the falloff radius. -q -sse / -q -ssd query them.",
            |_s, a| {
                let p = PrefStore::get();
                if a.bool("query") {
                    let q: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
                    if q.iter().any(|x| x == "softSelectDistance") {
                        return Ok(json!(p.f64("general", RADIUS, 5.0)));
                    }
                    if q.iter().any(|x| x == "softSelectFalloff") {
                        let m = p.string("general", FALLOFF_MODE, "volume");
                        return Ok(json!(FALLOFFS.iter().position(|x| *x == m).unwrap_or(0)));
                    }
                    if q.iter().any(|x| x == "softSelectUVDistance") {
                        return Ok(json!(p.f64("general", UV_RADIUS, 0.5)));
                    }
                    if q.iter().any(|x| x == "softSelectCurve") {
                        return Ok(json!(curve_to_string(&current_curve())));
                    }
                    if q.iter().any(|x| x == "softSelectColorCurve") {
                        return Ok(json!(p.string("general", COLOR_CURVE, DEFAULT_COLOR_CURVE)));
                    }
                    if q.iter().any(|x| x == "enableFalseColor") {
                        return Ok(json!(if p.bool("general", FALSE_COLOR, true) { 1 } else { 0 }));
                    }
                    if q.iter().any(|x| x == "compressUndo") {
                        return Ok(json!(p.f64("general", COMPRESS_UNDO, 0.0) as i64));
                    }
                    return Ok(json!(if p.bool("general", ENABLED, false) { 1 } else { 0 }));
                }
                // -softSelectReset: the settings go back to their defaults; whether it is on stays.
                if a.bool("softSelectReset") {
                    p.set("general", RADIUS, json!(5.0), PrefScope::User);
                    p.set("general", UV_RADIUS, json!(0.5), PrefScope::User);
                    p.set("general", FALLOFF_MODE, json!("volume"), PrefScope::User);
                    p.set("general", CURVE, Curve::default().to_json(), PrefScope::User);
                    p.set("general", COLOR_CURVE, json!(DEFAULT_COLOR_CURVE), PrefScope::User);
                    p.set("general", FALSE_COLOR, json!(true), PrefScope::User);
                }
                if let Some(t) = a.get("softSelectCurve").and_then(Value::as_str) {
                    match curve_from_string(t) {
                        Ok(c) => set_current_curve(&c),
                        Err(e) => fail!("softSelect: -ssc: {e}"),
                    }
                }
                if let Some(t) = a.get("softSelectColorCurve").and_then(Value::as_str) {
                    let n = numbers(t).map_err(|e| aztek_ops::CommandError::new(format!("softSelect: -scc: {e}")))?;
                    if n.is_empty() || n.len() % 5 != 0 {
                        fail!("softSelect: -scc: five numbers per point: red, green, blue, weight, interpolation");
                    }
                    p.set("general", COLOR_CURVE, json!(t), PrefScope::User);
                }
                if let Some(v) = a.get("enableFalseColor") {
                    let on = v.as_bool().or_else(|| v.as_i64().map(|i| i != 0)).unwrap_or(true);
                    p.set("general", FALSE_COLOR, json!(on), PrefScope::User);
                }
                if let Some(v) = a.get("compressUndo").and_then(Value::as_i64) {
                    if !(0..=2).contains(&v) {
                        fail!("softSelect: -cu is 0, 1 or 2");
                    }
                    p.set("general", COMPRESS_UNDO, json!(v), PrefScope::User);
                }
                if let Some(v) = a.get("softSelectUVDistance").and_then(Value::as_f64) {
                    p.set("general", UV_RADIUS, json!(v.max(1e-6)), PrefScope::User);
                }
                if let Some(v) = a.get("softSelectEnabled") {
                    let on = v.as_bool().or_else(|| v.as_i64().map(|i| i != 0)).or_else(|| v.as_str().map(|t| matches!(t, "true" | "on" | "yes" | "1"))).unwrap_or(false);
                    p.set("general", ENABLED, json!(on), PrefScope::User);
                }
                if let Some(v) = a.get("softSelectDistance").and_then(Value::as_f64) {
                    p.set("general", RADIUS, json!(v.max(1e-4)), PrefScope::User);
                }
                if let Some(v) = a.get("softSelectFalloff").and_then(Value::as_i64) {
                    let m = FALLOFFS.get(v.max(0) as usize).copied().unwrap_or("volume");
                    p.set("general", FALLOFF_MODE, json!(m), PrefScope::User);
                }
                Ok(Value::Null)
            },
        )
        .undoable(false)
        .flag(f("softSelectEnabled", "sse", T::Any, "1 on, 0 off."))
        .flag(f("softSelectDistance", "ssd", T::Float, "Falloff radius, world units."))
        .flag(f("softSelectFalloff", "ssf", T::Int, "0 volume, 1 surface, 2 global, 3 object."))
        .flag(f("softSelectCurve", "ssc", T::String, "The falloff curve: three numbers per point, the weight, the distance (0 at the selection, 1 at the radius) and the interpolation (0 none, 1 linear, 2 smooth, 3 spline), comma separated."))
        .flag(f("softSelectColorCurve", "scc", T::String, "The false colour ramp: five numbers per point, red, green and blue, the weight, and the interpolation, comma separated."))
        .flag(f("enableFalseColor", "efc", T::Int, "1 shows the false colours of the weights in the views, 0 doesn't (1)."))
        .flag(f("softSelectUVDistance", "sud", T::Float, "The falloff radius in UV space (0.5)."))
        .flag(f("softSelectReset", "ssr", T::Bool, "Put the settings back to their defaults."))
        .flag(f("compressUndo", "cu", T::Int, "0 every change undoes on its own, 1 changes in a row undo as one, 2 only interactive ones do (0)."))
        .flag(f("edit", "e", T::Bool, "Edit mode."))
        .flag(f("query", "q", T::Bool, "Query mode."))
        .flag(f("queried", "", T::List, "Flags asked about in query mode.")),
    );
    c.register(
        CommandDef::new(
            "softSelectCurve",
            "Set or query the soft select falloff curve (weight against distance from the selection, 0 at the selection, 1 \
             at the radius). With no flags it returns the current curve.",
            |_s, a| {
                if let Some(keys) = a.get("keys") {
                    set_current_curve(&Curve::from_json(keys));
                }
                if a.has("preset") {
                    let name = a.string("preset", "");
                    match find_builtin(&name).or_else(|| user_presets().into_iter().find(|u| u.name == name)) {
                        Some(p) => set_current_curve(&p.curve),
                        None => fail!("softSelectCurve: no preset named '{name}'"),
                    }
                }
                if a.has("falloffMode") {
                    let mode = a.string("falloffMode", "");
                    if !FALLOFFS.contains(&mode.as_str()) {
                        fail!("softSelectCurve: falloffMode is volume, surface, global or object");
                    }
                    PrefStore::get().set("general", FALLOFF_MODE, json!(mode), PrefScope::User);
                }
                if a.has("save") {
                    let name = a.string("save", "");
                    if name.is_empty() || find_builtin(&name).is_some() {
                        fail!("softSelectCurve: '{name}' can't be used as a preset name");
                    }
                    save_user_preset(&name, &current_curve());
                }
                if a.has("remove") && !delete_user_preset(&a.string("remove", "")) {
                    fail!("softSelectCurve: no user preset named '{}'", a.string("remove", ""));
                }
                if a.has("rename") && !rename_user_preset(&a.string("rename", ""), &a.string("to", "")) {
                    fail!("softSelectCurve: can't rename '{}' to '{}'", a.string("rename", ""), a.string("to", ""));
                }
                let users: Vec<String> = user_presets().into_iter().map(|p| p.name).collect();
                Ok(json!({
                    "keys": current_curve().to_json(),
                    "falloffMode": PrefStore::get().string("general", FALLOFF_MODE, "volume"),
                    "userPresets": users,
                }))
            },
        )
        .flag(f("keys", "k", T::List, "Keys as [[pos, value, interp], ...]; interp is linear, smooth, spline or flat."))
        .flag(f("preset", "p", T::String, "Apply a built-in or saved preset by name."))
        .flag(f("falloffMode", "fm", T::String, "Distance measured through space or along the surface.").choices(&FALLOFFS))
        .flag(f("save", "", T::String, "Save the current curve (after keys/preset) as a user preset with this name."))
        .flag(f("remove", "", T::String, "Delete the named user preset."))
        .flag(f("rename", "", T::String, "User preset to rename; the new name goes in to."))
        .flag(f("to", "", T::String, "New name for rename."))
        .undoable(false),
    );
}
