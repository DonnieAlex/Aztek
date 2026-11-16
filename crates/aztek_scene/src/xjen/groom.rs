//! The Interactive Groom tools: Density, Place, Length, Cut, Width, Twist, Comb, Grab, Smooth, Noise, Clump, Part,
//! Freeze and Select. A stroke works on one description: shaping tools write moves (and Width, width multipliers) into
//! the Sculpt modifier's active layer, Density and Place change the base, Freeze and Select mark splines. Each dab
//! replaces the stroke's last change, so a stroke undoes as one step.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex};

use aztek_geo::Mesh;
use glam::{DMat4, DVec3};

use super::edits::{EditData, Placed};
use super::sculpt::{self, SculptData};
use super::{spline_data, Rng, SplineData, BASE, SCULPT};
use crate::graph::Graph;
use crate::plug::{NodeId, PlugValue};
use crate::session::Session;

pub const TOOLS: &[&str] = &["density", "place", "length", "cut", "width", "twist", "comb", "grab", "smooth", "noise", "clump", "part", "freeze", "select"];

/// The tools' shared settings, as the reference's Tool Settings show them. Size is in pixels on screen.
#[derive(Clone, Debug)]
pub struct GroomCtx {
    pub tool: String,
    pub size: f64,
    pub strength: f64,
    pub collide: bool,
    pub collision_distance: f64,
    pub invert_frozen: bool,
    /// N toggles each tool's other mode (Comb: preserve length off; Clump: spread; Length and Width: shrink).
    pub alt_mode: bool,
    /// Per description, the splines the Select tool picked: the other tools work only on these when there are any.
    pub selection: HashMap<NodeId, HashSet<u64>>,
}

impl Default for GroomCtx {
    fn default() -> Self {
        GroomCtx { tool: "comb".into(), size: 60.0, strength: 0.5, collide: false, collision_distance: 0.1, invert_frozen: false, alt_mode: false, selection: HashMap::new() }
    }
}

/// The settings, one set per thread: the interface's, or each test's.
pub fn ctx() -> &'static Mutex<GroomCtx> {
    thread_local! {
        static C: &'static Mutex<GroomCtx> = Box::leak(Box::new(Mutex::new(GroomCtx::default())));
    }
    C.with(|c| *c)
}

/// 1 at the centre, 0 at the rim, smooth.
fn falloff(t: f64) -> f64 {
    if t >= 1.0 {
        0.0
    } else {
        let u = 1.0 - t * t;
        u * u * (3.0 - 2.0 * u).max(0.0)
    }
}

fn node_value(g: &Graph, id: NodeId, attr: &str) -> Option<PlugValue> {
    let n = g.node(id)?;
    Some(n.data().value(n.ty().attr_index(attr)?).clone())
}

/// Closest place on a mesh to a point: (face, triangle corners, weights, position).
pub fn closest(mesh: &Mesh, p: DVec3) -> Option<(u32, [u32; 3], [f64; 3], DVec3)> {
    let mut best: Option<(f64, (u32, [u32; 3], [f64; 3], DVec3))> = None;
    for f in 0..mesh.face_capacity() {
        if !mesh.face_valid(f) {
            continue;
        }
        let c = mesh.face_corners(f);
        for k in 1..c.len().saturating_sub(1) {
            let t = [c[0], c[k], c[k + 1]];
            let [a, b, cc] = t.map(|x| mesh.position(mesh.corner_point(x)));
            let q = crate::utility_nodes::closest_on_tri(p, a, b, cc);
            let d = q.distance_squared(p);
            if best.as_ref().is_some_and(|x| x.0 <= d) {
                continue;
            }
            let (v0, v1, v2) = (b - a, cc - a, q - a);
            let (d00, d01, d11, d20, d21) = (v0.dot(v0), v0.dot(v1), v1.dot(v1), v2.dot(v0), v2.dot(v1));
            let den = d00 * d11 - d01 * d01;
            if den.abs() < 1e-20 {
                continue;
            }
            let w1 = (d11 * d20 - d01 * d21) / den;
            let w2 = (d00 * d21 - d01 * d20) / den;
            best = Some((d, (f as u32, t, [1.0 - w1 - w2, w1, w2], q)));
        }
    }
    best.map(|b| b.1)
}

/// Lengths of a polyline's segments.
fn segments(p: &[DVec3]) -> Vec<f64> {
    p.windows(2).map(|w| w[0].distance(w[1])).collect()
}

/// Chains points back to segment lengths, from the root out, keeping each segment's direction.
fn keep_lengths(p: &mut [DVec3], lens: &[f64]) {
    for j in 1..p.len() {
        let dir = (p[j] - p[j - 1]).normalize_or_zero();
        if dir != DVec3::ZERO {
            p[j] = p[j - 1] + dir * lens[j - 1];
        }
    }
}

/// The polyline cut to a length from its root, with the same number of points, evenly along what is kept.
fn cut_to(p: &[DVec3], length: f64) -> Vec<DVec3> {
    let n = p.len();
    let lens = segments(p);
    let at = |s: f64| -> DVec3 {
        let mut s = s;
        for (j, l) in lens.iter().enumerate() {
            if s <= *l || j + 1 == lens.len() {
                let t = if *l > 1e-12 { (s / l).min(1.0) } else { 0.0 };
                return p[j].lerp(p[j + 1], t);
            }
            s -= l;
        }
        p[n - 1]
    };
    (0..n).map(|j| at(length * j as f64 / (n - 1).max(1) as f64)).collect()
}

/// A stroke on one description.
pub struct Stroke {
    pub desc: NodeId,
    sculpt: NodeId,
    base: NodeId,
    layer: usize,
    /// Splines going into the Sculpt modifier, in the description's space.
    pre: SplineData,
    /// The moves at the stroke's start, and as the stroke has them now.
    start: SculptData,
    data: SculptData,
    edits: EditData,
    /// Points as the description has them, with every layer but the active one, then with all of them.
    others: Vec<Vec<DVec3>>,
    cur: Vec<Vec<DVec3>>,
    /// The active layer's weight times the modifier's magnitude: what a move in it is worth.
    worth: f64,
    world: DMat4,
    inv: DMat4,
    mesh: Option<Arc<Mesh>>,
    applied: usize,
    last: Option<DVec3>,
    pub invert: bool,
    /// Shift: the tool's other action.
    pub shift: bool,
    dabs: u64,
    /// Grab: each spline's pull at the stroke's start.
    grab: Option<Vec<f64>>,
}

/// The Sculpt modifier and base of a description, if it has them.
fn parts(g: &Graph, desc: NodeId) -> Option<(NodeId, NodeId)> {
    let st = super::stack(g, desc);
    let sculpt = st.iter().rev().copied().find(|&n| g.node(n).is_some_and(|x| x.ty().is_a(SCULPT)))?;
    let base = st.iter().copied().find(|&n| g.node(n).is_some_and(|x| x.ty().is_a(BASE)))?;
    Some((sculpt, base))
}

/// Points of the splines in, with the layers applied (all of them, or all but one), in the order of `pre`.
fn with_layers(g: &Graph, sculpt: NodeId, pre: &SplineData, data: &SculptData, skip: Option<usize>, mag: f64) -> Vec<Vec<DVec3>> {
    let index: HashMap<u64, usize> = pre.splines.iter().enumerate().map(|(k, s)| (s.id, k)).collect();
    let mut out: Vec<super::Spline> = pre.splines.clone();
    for (li, l) in data.layers.iter().enumerate() {
        if Some(li) == skip || !layer_on(g, sculpt, li) {
            continue;
        }
        let w = layer_weight(g, sculpt, li) * mag;
        for (id, m) in &l.moves {
            if let Some(&k) = index.get(id) {
                for (j, v) in m.iter().enumerate() {
                    sculpt::apply(&mut out[k], j, *v, w);
                }
            }
        }
    }
    out.into_iter().map(|s| s.points).collect()
}

fn element(g: &Graph, id: NodeId, attr: &str, k: usize) -> Option<PlugValue> {
    let n = g.node(id)?;
    let a = n.ty().attr_index(attr)?;
    Some(n.data().element_or(a, k as i32).clone())
}
fn layer_weight(g: &Graph, sculpt: NodeId, k: usize) -> f64 {
    element(g, sculpt, "layer.layerWeight", k).map_or(1.0, |v| v.as_float())
}
fn layer_on(g: &Graph, sculpt: NodeId, k: usize) -> bool {
    element(g, sculpt, "layer.layerVisibility", k).is_none_or(|v| v.as_bool())
}
fn modifier_worth(g: &Graph, n: NodeId) -> f64 {
    if node_value(g, n, "mute").is_some_and(|v| v.as_bool()) {
        return 0.0;
    }
    node_value(g, n, "magnitude").map_or(1.0, |v| v.as_float()) * node_value(g, n, "magnitudeScale").map_or(1.0, |v| v.as_float())
}

impl Stroke {
    /// Starts a stroke on a description; None if it has no Sculpt modifier and base.
    pub fn begin(s: &mut Session, desc: NodeId, invert: bool, shift: bool) -> Option<Stroke> {
        s.graph.evaluate();
        let g = &s.graph;
        let (sculpt, base) = parts(g, desc)?;
        let pre = node_value(g, sculpt, "inSplineData").and_then(|v| spline_data(&v).cloned()).unwrap_or_default();
        let data = node_value(g, sculpt, "sculptData").and_then(|v| sculpt::sculpt_data(&v).cloned()).unwrap_or_default();
        let edits = node_value(g, base, "editData").and_then(|v| super::edits::edit_data(&v).cloned()).unwrap_or_default();
        let mut data = data;
        if data.layers.is_empty() {
            data.layers.push(sculpt::Layer { name: "Base".into(), ..Default::default() });
        }
        let layer = (node_value(g, sculpt, "activeLayer").map_or(0, |v| v.as_int()).max(0) as usize).min(data.layers.len() - 1);
        let mag = modifier_worth(g, sculpt);
        let worth = layer_weight(g, sculpt, layer) * mag * if layer_on(g, sculpt, layer) { 1.0 } else { 0.0 };
        let others = with_layers(g, sculpt, &pre, &data, Some(layer), mag);
        let cur = with_layers(g, sculpt, &pre, &data, None, mag);
        let world = g.node(desc).map(|n| n.data().matrix(crate::transform::DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY);
        let mesh = node_value(g, base, "boundMesh").and_then(|v| v.as_mesh().cloned());
        let start = data.clone();
        Some(Stroke { desc, sculpt, base, layer, pre, start, data, edits, others, cur, worth, world, inv: world.inverse(), mesh, applied: 0, last: None, invert, shift, dabs: 0, grab: None })
    }

    pub fn sculpt_node(&self) -> NodeId {
        self.sculpt
    }

    /// How much a spline takes of a dab at center (description space): falloff from its nearest point, times
    /// strength, less what is frozen, nothing for splines outside the selection when there is one.
    fn weights(&self, center: DVec3, radius: f64, c: &GroomCtx, by_root: bool) -> Vec<f64> {
        let sel = c.selection.get(&self.desc).filter(|s| !s.is_empty());
        self.pre
            .splines
            .iter()
            .zip(&self.cur)
            .map(|(sp, pts)| {
                if sel.is_some_and(|s| !s.contains(&sp.id)) {
                    return 0.0;
                }
                let d = if by_root { pts[0].distance(center) } else { pts.iter().map(|p| p.distance(center)).fold(f64::INFINITY, f64::min) };
                let f = self.data.frozen.get(&sp.id).copied().unwrap_or(0.0) as f64;
                let free = if c.invert_frozen { f } else { 1.0 - f };
                falloff(d / radius.max(1e-9)) * c.strength * free.clamp(0.0, 1.0)
            })
            .collect()
    }

    /// One dab: center and normal in world space, radius in world units.
    pub fn dab(&mut self, s: &mut Session, center_w: DVec3, normal_w: DVec3, radius: f64) {
        let c = ctx().lock().unwrap().clone();
        let center = self.inv.transform_point3(center_w);
        let normal = self.inv.transform_vector3(normal_w).normalize_or_zero();
        let pull = self.last.map(|l| center - l).unwrap_or(DVec3::ZERO);
        self.last = Some(center);
        self.dabs += 1;
        let sign = if self.invert { -1.0 } else { 1.0 };
        let tool = c.tool.as_str();
        match tool {
            "freeze" | "select" => {
                // Every spline in the brush, frozen or not, picked or not.
                let w: Vec<f64> = self.cur.iter().map(|p| falloff(p.iter().map(|x| x.distance(center)).fold(f64::INFINITY, f64::min) / radius.max(1e-9))).collect();
                if tool == "freeze" {
                    for (sp, w) in self.pre.splines.iter().zip(&w) {
                        if *w <= 0.0 {
                            continue;
                        }
                        let f = self.data.frozen.entry(sp.id).or_insert(0.0);
                        *f = (*f + (w * c.strength * 2.0 * sign) as f32).clamp(0.0, 1.0);
                    }
                    self.data.frozen.retain(|_, f| *f > 0.0);
                    self.commit(s, false);
                } else {
                    let mut g = ctx().lock().unwrap();
                    let set = g.selection.entry(self.desc).or_default();
                    for (sp, w) in self.pre.splines.iter().zip(&w) {
                        if *w > 0.05 {
                            if self.invert {
                                set.remove(&sp.id);
                            } else {
                                set.insert(sp.id);
                            }
                        }
                    }
                }
                return;
            }
            "density" | "place" => {
                self.grow(s, center_w, normal_w, radius, &c, tool == "place");
                return;
            }
            _ => {}
        }
        let by_root = matches!(tool, "length" | "width" | "twist" | "noise");
        let w = self.weights(center, radius, &c, by_root);
        let up = if normal != DVec3::ZERO { normal } else { DVec3::Y };
        let n_pts = |p: &Vec<DVec3>| (p.len().max(2) - 1) as f64;
        match tool {
            "length" => {
                let grow = if self.invert != c.alt_mode { -1.0 } else { 1.0 };
                for (p, w) in self.cur.iter_mut().zip(&w) {
                    if *w > 0.0 {
                        let k = (1.0 + 0.1 * w * grow).max(0.05);
                        let r = p[0];
                        for q in p.iter_mut() {
                            *q = r + (*q - r) * k;
                        }
                    }
                }
            }
            "cut" => {
                for (p, w) in self.cur.iter_mut().zip(&w) {
                    if *w <= 0.0 {
                        continue;
                    }
                    // Where the spline passes nearest the brush: cut there.
                    let (mut best, mut along, mut acc) = (f64::INFINITY, 0.0, 0.0);
                    for k in 0..p.len() - 1 {
                        let (a, b) = (p[k], p[k + 1]);
                        let d = b - a;
                        let t = if d.length_squared() > 1e-20 { ((center - a).dot(d) / d.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
                        let q = a + d * t;
                        let dist = q.distance(center);
                        if dist < best {
                            best = dist;
                            along = acc + d.length() * t;
                        }
                        acc += d.length();
                    }
                    if best <= radius && along > 1e-6 {
                        *p = cut_to(p, along);
                    }
                }
            }
            "width" => {
                let grow = if self.invert != c.alt_mode { -1.0 } else { 1.0 };
                let layer = &mut self.data.layers[self.layer];
                for (sp, w) in self.pre.splines.iter().zip(&w) {
                    if *w > 0.0 {
                        let x = layer.widths.entry(sp.id).or_insert(1.0);
                        *x = (*x * (1.0 + 0.1 * w * grow) as f32).max(0.0);
                    }
                }
            }
            "twist" => {
                for (p, w) in self.cur.iter_mut().zip(&w) {
                    if *w > 0.0 {
                        let r = p[0];
                        let q = glam::DQuat::from_axis_angle(up, 0.3 * w * sign);
                        for x in p.iter_mut() {
                            *x = r + q * (*x - r);
                        }
                    }
                }
            }
            "comb" | "grab" => {
                let along = pull - up * pull.dot(up);
                let d = if tool == "grab" { pull } else { along };
                if tool == "grab" && self.grab.is_none() {
                    self.grab = Some(w.clone());
                }
                let ws = if tool == "grab" { self.grab.clone().unwrap_or(w) } else { w };
                let keep = tool == "comb" && !c.alt_mode;
                for (p, w) in self.cur.iter_mut().zip(&ws) {
                    if *w <= 0.0 || d == DVec3::ZERO {
                        continue;
                    }
                    let lens = segments(p);
                    let n = n_pts(p);
                    for (j, x) in p.iter_mut().enumerate().skip(1) {
                        let t = j as f64 / n;
                        *x += d * w * if tool == "grab" { t } else { t.sqrt() * 2.0 };
                    }
                    if keep {
                        keep_lengths(p, &lens);
                    }
                }
            }
            "smooth" => {
                // Along each spline, and toward the shape of its neighbours in the brush.
                let mut mean: Vec<DVec3> = Vec::new();
                let mut total = 0.0;
                for (p, w) in self.cur.iter().zip(&w) {
                    if *w > 0.0 {
                        if mean.len() < p.len() {
                            mean.resize(p.len(), DVec3::ZERO);
                        }
                        for (j, x) in p.iter().enumerate() {
                            mean[j] += (*x - p[0]) * *w;
                        }
                        total += w;
                    }
                }
                for (p, w) in self.cur.iter_mut().zip(&w) {
                    if *w <= 0.0 {
                        continue;
                    }
                    let lens = segments(p);
                    let old = p.clone();
                    for j in 1..p.len() - 1 {
                        p[j] = p[j].lerp((old[j - 1] + old[j + 1]) * 0.5, w * 0.5);
                    }
                    if total > 0.0 {
                        for j in 1..p.len().min(mean.len()) {
                            let target = p[0] + mean[j] / total;
                            p[j] = p[j].lerp(target, w * 0.5);
                        }
                    }
                    keep_lengths(p, &lens);
                }
            }
            "noise" => {
                for ((p, w), sp) in self.cur.iter_mut().zip(&w).zip(&self.pre.splines) {
                    if *w <= 0.0 {
                        continue;
                    }
                    let lens = segments(p);
                    let mut rng = Rng::new(sp.id ^ self.dabs.wrapping_mul(0x9E37_79B9));
                    let n = n_pts(p);
                    for (j, x) in p.iter_mut().enumerate().skip(1) {
                        let v = DVec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
                        *x += v * lens.get(j - 1).copied().unwrap_or(0.0) * w * (j as f64 / n);
                    }
                    keep_lengths(p, &lens);
                }
            }
            "clump" | "part" => {
                // Clump draws points toward the weighted middle of those in the brush, level by level (spread
                // pushes them out); Part pushes them away from the brush's centre across the surface.
                let mut mid: Vec<DVec3> = Vec::new();
                let mut total = 0.0;
                for (p, w) in self.cur.iter().zip(&w) {
                    if *w > 0.0 {
                        if mid.len() < p.len() {
                            mid.resize(p.len(), DVec3::ZERO);
                        }
                        for (j, x) in p.iter().enumerate() {
                            mid[j] += *x * *w;
                        }
                        total += w;
                    }
                }
                let spread = if tool == "clump" && (self.invert != c.alt_mode) { -1.0 } else { 1.0 };
                for (p, w) in self.cur.iter_mut().zip(&w) {
                    if *w <= 0.0 {
                        continue;
                    }
                    let lens = segments(p);
                    let n = n_pts(p);
                    for j in 1..p.len() {
                        let t = j as f64 / n;
                        if tool == "clump" {
                            if total > 0.0 && j < mid.len() {
                                let m = mid[j] / total;
                                let here = p[j];
                                p[j] = here + (m - here) * (w * t * spread).clamp(-1.0, 1.0);
                            }
                        } else {
                            let away = p[j] - center;
                            let away = (away - up * away.dot(up)).normalize_or_zero();
                            p[j] += away * lens.get(j - 1).copied().unwrap_or(0.0) * w * t * 2.0 * sign;
                        }
                    }
                    if tool == "part" {
                        keep_lengths(p, &lens);
                    }
                }
            }
            _ => return,
        }
        if c.collide {
            self.collide(c.collision_distance);
        }
        self.write_moves();
        self.commit(s, false);
    }

    /// Points that went inside the bound mesh are put back out to the collision distance above its surface.
    fn collide(&mut self, dist: f64) {
        let Some(mesh) = self.mesh.clone() else { return };
        let normals = aztek_geo::normals::point_normals(&mesh, aztek_geo::normals::NormalWeight::Angle);
        for (p, before) in self.cur.iter_mut().zip(&self.others) {
            for j in 1..p.len() {
                if before.get(j).is_some_and(|b| b.distance_squared(p[j]) < 1e-18) {
                    continue;
                }
                let wp = self.world.transform_point3(p[j]);
                let Some((_, t, w, q)) = closest(&mesh, wp) else { continue };
                let (_, n, _) = super::frame(&mesh, &normals, t, w);
                if (wp - q).dot(n) < dist {
                    p[j] = self.inv.transform_point3(q + n * dist);
                }
            }
        }
    }

    /// The active layer's moves: what makes the points as they are now, from what the other layers make.
    fn write_moves(&mut self) {
        if self.worth.abs() < 1e-9 {
            return;
        }
        let layer = &mut self.data.layers[self.layer];
        for ((sp, now), other) in self.pre.splines.iter().zip(&self.cur).zip(&self.others) {
            let changed = now.iter().zip(other).any(|(a, b)| a.distance_squared(*b) > 1e-18);
            if !changed {
                layer.moves.remove(&sp.id);
                continue;
            }
            let mv: Vec<[f32; 3]> = now.iter().zip(other).map(|(a, b)| sculpt::to_frame(sp, (*a - *b) / self.worth)).collect();
            layer.moves.insert(sp.id, mv);
        }
    }

    /// Density adds splines in the brush (Ctrl takes them away); Place adds one where the brush is.
    fn grow(&mut self, s: &mut Session, center_w: DVec3, normal_w: DVec3, radius: f64, c: &GroomCtx, place: bool) {
        let Some(mesh) = self.mesh.clone() else { return };
        if self.invert {
            let w = self.weights(self.inv.transform_point3(center_w), radius, c, true);
            let mut rng = Rng::new(0x5EED ^ self.dabs);
            for (sp, w) in self.pre.splines.iter().zip(&w) {
                if *w > 0.0 && rng.next_f64() < *w {
                    self.edits.removed.insert(sp.id);
                }
            }
        } else {
            let count = if place { 1 } else { (c.strength * 6.0).ceil() as usize };
            let mut rng = Rng::new(0xD0_5EED ^ self.dabs.wrapping_mul(7919) ^ (center_w.x.to_bits() >> 12));
            let n = normal_w.normalize_or_zero();
            let t = n.any_orthonormal_vector();
            let b = n.cross(t);
            let roots: Vec<DVec3> = self.cur.iter().map(|p| self.world.transform_point3(p[0])).collect();
            let spacing = radius * if place { 0.1 } else { 0.15 };
            let mut added: Vec<DVec3> = Vec::new();
            for _ in 0..count {
                let (r, a) = if place { (0.0, 0.0) } else { (radius * rng.next_f64().sqrt(), rng.next_f64() * std::f64::consts::TAU) };
                let at = center_w + (t * a.cos() + b * a.sin()) * r;
                let Some((face, corners, w, q)) = closest(&mesh, at) else { continue };
                if roots.iter().chain(&added).any(|x| x.distance(q) < spacing) {
                    continue;
                }
                added.push(q);
                self.edits.placed.push(Placed { face, corners, w });
            }
        }
        self.commit(s, true);
    }

    /// Replaces the stroke's last change with what it has now.
    fn commit(&mut self, s: &mut Session, base: bool) {
        for _ in 0..self.applied {
            s.undo();
        }
        self.applied = 0;
        let sp = crate::harold::network::attr(&s.graph, self.sculpt, "sculptData");
        if let Some(p) = sp {
            if s.graph.set_value(p, PlugValue::Data(Some(Arc::new(self.data.clone())))).is_ok() {
                self.applied += 1;
            }
        }
        if base || !self.edits.removed.is_empty() || !self.edits.placed.is_empty() {
            if let Some(p) = crate::harold::network::attr(&s.graph, self.base, "editData") {
                if s.graph.set_value(p, PlugValue::Data(Some(Arc::new(self.edits.clone())))).is_ok() {
                    self.applied += 1;
                }
            }
        }
        if base {
            // New or fewer splines: start over from what the stack makes now, keeping this change.
            let (applied, last, dabs) = (self.applied, self.last, self.dabs);
            if let Some(mut st) = Stroke::begin(s, self.desc, self.invert, self.shift) {
                st.applied = applied;
                st.last = last;
                st.dabs = dabs;
                st.start = self.start.clone();
                *self = st;
            }
        }
    }

    /// Whether the stroke changed anything.
    pub fn changed(&self) -> bool {
        self.applied > 0 && (self.data != self.start || !self.edits.removed.is_empty() || !self.edits.placed.is_empty())
    }
}

/// Splines of a description on faces: the ids of those rooted there.
pub fn splines_on_faces(g: &Graph, desc: NodeId, faces: &HashSet<u32>) -> HashSet<u64> {
    super::render_splines(g, desc).map_or(HashSet::new(), |d| d.splines.iter().filter(|s| faces.contains(&s.face)).map(|s| s.id).collect())
}

/// Freezes (fully) the selected splines of a description, as xgmSplineSelect -convertToFreeze does.
pub fn freeze_selected(s: &mut Session, desc: NodeId) -> usize {
    let sel: Vec<u64> = ctx().lock().unwrap().selection.get(&desc).map(|x| x.iter().copied().collect()).unwrap_or_default();
    let Some((sculpt, _)) = parts(&s.graph, desc) else { return 0 };
    let mut data = node_value(&s.graph, sculpt, "sculptData").and_then(|v| sculpt::sculpt_data(&v).cloned()).unwrap_or_default();
    for id in &sel {
        data.frozen.insert(*id, 1.0);
    }
    if let Some(p) = crate::harold::network::attr(&s.graph, sculpt, "sculptData") {
        let _ = s.graph.set_value(p, PlugValue::Data(Some(Arc::new(data))));
    }
    sel.len()
}

/// Layers of a Sculpt modifier as (name, weight, visible).
pub fn layers(g: &Graph, sculpt: NodeId) -> Vec<(String, f64, bool)> {
    let data = node_value(g, sculpt, "sculptData").and_then(|v| sculpt::sculpt_data(&v).cloned()).unwrap_or_default();
    data.layers.iter().enumerate().map(|(k, l)| (l.name.clone(), layer_weight(g, sculpt, k), layer_on(g, sculpt, k))).collect()
}

/// Merges layers into the first of them, each scaled by its weight and dropped where hidden; the merged layer's
/// weight becomes 1. Returns the moves to set.
pub fn merged(g: &Graph, sculpt: NodeId, which: &[usize]) -> Option<SculptData> {
    let mut data = node_value(g, sculpt, "sculptData").and_then(|v| sculpt::sculpt_data(&v).cloned())?;
    let mut which: Vec<usize> = which.iter().copied().filter(|&k| k < data.layers.len()).collect();
    which.sort_unstable();
    which.dedup();
    let first = *which.first()?;
    let mut moves: BTreeMap<u64, Vec<[f32; 3]>> = BTreeMap::new();
    let mut widths: BTreeMap<u64, f32> = BTreeMap::new();
    for &k in &which {
        if !layer_on(g, sculpt, k) {
            continue;
        }
        let w = layer_weight(g, sculpt, k) as f32;
        for (id, m) in &data.layers[k].moves {
            let e = moves.entry(*id).or_default();
            if e.len() < m.len() {
                e.resize(m.len(), [0.0; 3]);
            }
            for (j, v) in m.iter().enumerate() {
                for a in 0..3 {
                    e[j][a] += v[a] * w;
                }
            }
        }
        for (id, x) in &data.layers[k].widths {
            *widths.entry(*id).or_insert(1.0) *= 1.0 + (x - 1.0) * w;
        }
    }
    data.layers[first].moves = moves;
    data.layers[first].widths = widths;
    for &k in which.iter().rev().filter(|&&k| k != first) {
        data.layers.remove(k);
    }
    Some(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_keeps_point_count() {
        let p: Vec<DVec3> = (0..5).map(|j| DVec3::new(0.0, j as f64, 0.0)).collect();
        let c = cut_to(&p, 2.0);
        assert_eq!(c.len(), 5);
        assert!((c[4] - DVec3::new(0.0, 2.0, 0.0)).length() < 1e-12);
        assert!((c[1] - DVec3::new(0.0, 0.5, 0.0)).length() < 1e-12);
    }
}
