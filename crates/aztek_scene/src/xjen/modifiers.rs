//! The Interactive Groom modifiers beyond Scale and Sculpt: Clump, Noise, Cut, Collide, Displace, Guide, Linear Wire
//! and Curve to Spline. Each takes the splines in, changes them by its settings times Magnitude and Magnitude Scale,
//! and passes them on; Mute passes them through.

use std::sync::Arc;

use glam::DVec3;

use super::{at, data_value, fl, getb, getf, geti, modifier_attrs, ranged, Rng, Spline, SplineData};
use crate::node::{types, NodeData, NodeType};
use crate::plug::{flags, AttrDef, PlugType, PlugValue};

/// (type, what it does).
pub const TYPES: &[(&str, &str)] = &[
    ("xgmModifierClump", "Gathers splines into clumps around guide splines chosen by density and seed."),
    ("xgmModifierNoise", "Wavers splines by a smooth noise along their length."),
    ("xgmModifierCut", "Cuts splines shorter, by an amount and a random part of it."),
    ("xgmModifierCollide", "Keeps splines outside the meshes connected to it."),
    ("xgmModifierDisplace", "Moves splines along their root normals, more toward the tip."),
    ("xgmModifierGuide", "Shapes splines after the curves connected to it, each spline following its nearest guides."),
    ("xgmModifierLinearWire", "Bends splines near each wire curve by how the wire has moved from its reference state."),
    ("xgmCurveToSpline", "Turns the curves connected to it into splines, rooted on the bound mesh, added to those coming in."),
];

fn n_of(p: &[DVec3]) -> f64 {
    (p.len().max(2) - 1) as f64
}

fn segments(p: &[DVec3]) -> Vec<f64> {
    p.windows(2).map(|w| w[0].distance(w[1])).collect()
}

fn keep_lengths(p: &mut [DVec3], lens: &[f64]) {
    for j in 1..p.len() {
        let dir = (p[j] - p[j - 1]).normalize_or_zero();
        if dir != DVec3::ZERO {
            p[j] = p[j - 1] + dir * lens[j - 1];
        }
    }
}

/// A polyline resampled to n points evenly along its length.
pub fn resample(p: &[DVec3], n: usize) -> Vec<DVec3> {
    if p.len() < 2 || n < 2 {
        return p.to_vec();
    }
    let lens = segments(p);
    let total: f64 = lens.iter().sum();
    (0..n)
        .map(|k| {
            let mut s = total * k as f64 / (n - 1) as f64;
            for (j, l) in lens.iter().enumerate() {
                if s <= *l || j + 1 == lens.len() {
                    let t = if *l > 1e-12 { (s / l).min(1.0) } else { 0.0 };
                    return p[j].lerp(p[j + 1], t);
                }
                s -= l;
            }
            p[p.len() - 1]
        })
        .collect()
}

fn mag(d: &NodeData) -> f64 {
    getf(d, "magnitude") * getf(d, "magnitudeScale")
}

/// Smooth 3D value noise in [-1, 1].
fn value_noise(p: DVec3, seed: u64) -> f64 {
    let c = p.floor();
    let f = p - c;
    let s = f * f * (DVec3::splat(3.0) - f * 2.0);
    let h = |x: f64, y: f64, z: f64| -> f64 {
        let k = (x as i64 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as i64 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F) ^ (z as i64 as u64).wrapping_mul(0x1656_67B1_9E37_79F9) ^ seed;
        Rng::new(k).next_f64() * 2.0 - 1.0
    };
    let mut v = 0.0;
    for corner in 0..8 {
        let (dx, dy, dz) = ((corner & 1) as f64, ((corner >> 1) & 1) as f64, ((corner >> 2) & 1) as f64);
        let w = (if dx > 0.0 { s.x } else { 1.0 - s.x }) * (if dy > 0.0 { s.y } else { 1.0 - s.y }) * (if dz > 0.0 { s.z } else { 1.0 - s.z });
        v += w * h(c.x + dx, c.y + dy, c.z + dz);
    }
    v
}

fn clump(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let amount = getf(d, "clump") * m;
    let (variance, density, seed) = (getf(d, "clumpVariance"), getf(d, "density"), geti(d, "seed").max(0) as u64);
    let (curl, keep) = (getf(d, "curl"), getb(d, "preserveLength"));
    if s.splines.is_empty() || amount == 0.0 {
        return;
    }
    // Guides: about density per face, picked by seed.
    let faces: std::collections::BTreeSet<u32> = s.splines.iter().map(|x| x.face).collect();
    let per_face = (s.splines.len() as f64 / faces.len().max(1) as f64).max(1.0);
    let p = (density / per_face).clamp(0.0, 1.0);
    let guides: Vec<usize> = (0..s.splines.len()).filter(|&k| super::hash01(s.splines[k].id ^ seed.wrapping_mul(0x51ED)) < p).collect();
    if guides.is_empty() {
        return;
    }
    let shapes: Vec<Vec<DVec3>> = guides.iter().map(|&k| s.splines[k].points.clone()).collect();
    for sp in &mut s.splines {
        let root = sp.root();
        let (gi, _) = (0..guides.len()).map(|i| (i, shapes[i][0].distance_squared(root))).fold((0, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
        let g = &shapes[gi];
        let lens = segments(&sp.points);
        let var = 1.0 - variance * super::hash01(sp.id ^ 0xC1);
        let n = n_of(&sp.points);
        let axis = (g[g.len() - 1] - g[0]).normalize_or_zero();
        for j in 1..sp.points.len() {
            let t = j as f64 / n;
            let target = g.get(j).copied().unwrap_or(g[g.len() - 1]);
            let mut q = sp.points[j].lerp(target, (amount * t * var).clamp(0.0, 1.0));
            if curl != 0.0 && axis != DVec3::ZERO {
                let rot = glam::DQuat::from_axis_angle(axis, curl * t * std::f64::consts::TAU * m);
                q = target + rot * (q - target);
            }
            sp.points[j] = q;
        }
        if keep {
            keep_lengths(&mut sp.points, &lens);
        }
    }
}

fn noise(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let (freq, amp, corr, keep, seed) = (getf(d, "frequency"), getf(d, "noiseMagnitude"), getf(d, "correlation"), getb(d, "preserveLength"), geti(d, "seed").max(0) as u64);
    for sp in &mut s.splines {
        let lens = segments(&sp.points);
        let total: f64 = lens.iter().sum();
        let n = n_of(&sp.points);
        // Correlation: 1 moves every spline alike, 0 each on its own.
        let own = sp.root() * freq + DVec3::splat((sp.id % 9973) as f64 * (1.0 - corr.clamp(0.0, 1.0)));
        for j in 1..sp.points.len() {
            let t = j as f64 / n;
            let q = own + sp.normal * (t * total * freq);
            let v = DVec3::new(value_noise(q, seed), value_noise(q + DVec3::splat(31.7), seed), value_noise(q + DVec3::splat(71.3), seed));
            sp.points[j] += v * amp * m * t * total;
        }
        if keep {
            keep_lengths(&mut sp.points, &lens);
        }
    }
}

fn cut(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let (amount, random, seed) = (getf(d, "amount"), getf(d, "randomness"), geti(d, "seed").max(0) as u64);
    for sp in &mut s.splines {
        let r = super::hash01(sp.id ^ seed.wrapping_mul(0xC07));
        let frac = (amount * (1.0 - random * r) * m).clamp(0.0, 1.0);
        if frac <= 0.0 {
            continue;
        }
        let len = sp.length() * (1.0 - frac);
        sp.points = super::groom::cut_to(&sp.points, len.max(1e-9));
    }
}

fn collide(d: &mut NodeData, s: &mut SplineData, m: f64) {
    if m <= 0.0 {
        return;
    }
    let dist = getf(d, "distance");
    let meshes: Vec<_> = d.elements(at(d, "collisionMesh")).iter().filter_map(|(_, v)| v.as_mesh().cloned()).collect();
    for mesh in meshes {
        let normals = aztek_geo::normals::point_normals(&mesh, aztek_geo::normals::NormalWeight::Angle);
        let b = mesh.bounds();
        for sp in &mut s.splines {
            // Only splines that reach near the mesh.
            let grow = DVec3::splat(dist.abs() + sp.length() + 1e-6);
            if sp.root().cmplt(b.min - grow).any() || sp.root().cmpgt(b.max + grow).any() {
                continue;
            }
            let grow = DVec3::splat(dist.abs() + 1e-6);
            for j in 1..sp.points.len() {
                let p = sp.points[j];
                // Beside the mesh, not under or over it.
                let flat = |v: DVec3, lo: DVec3, hi: DVec3| (0..3).filter(|&a| v[a] < lo[a] || v[a] > hi[a]).count();
                if flat(p, b.min - grow, b.max + grow) > 1 {
                    continue;
                }
                let Some((_, t, w, q)) = super::groom::closest(&mesh, p) else { continue };
                let (_, n, _) = super::frame(&mesh, &normals, t, w);
                if (p - q).dot(n) < dist {
                    sp.points[j] = p.lerp(q + n * dist, m.min(1.0));
                }
            }
        }
    }
}

fn displace(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let amount = getf(d, "displacement") * m;
    for sp in &mut s.splines {
        let n = n_of(&sp.points);
        let dir = sp.normal;
        for j in 1..sp.points.len() {
            sp.points[j] += dir * amount * (j as f64 / n);
        }
    }
}

/// Curves connected to an array, as polylines in world space.
fn curves_in(d: &NodeData, attr: &str) -> Vec<(i32, Vec<DVec3>)> {
    d.elements(at(d, attr))
        .iter()
        .filter_map(|(k, v)| crate::curves::curve_value(v).map(|c| (*k, c.tessellate(8))))
        .filter(|(_, p)| p.len() >= 2)
        .collect()
}

fn guide(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let blend = getf(d, "blend").clamp(0.0, 1.0) * m;
    let guides = curves_in(d, "guideCurves");
    if guides.is_empty() || blend == 0.0 {
        return;
    }
    for sp in &mut s.splines {
        let n = sp.points.len();
        let root = sp.root();
        // The three nearest guides, by their roots, weighted by inverse distance.
        let mut near: Vec<(f64, &Vec<DVec3>)> = guides.iter().map(|(_, g)| (g[0].distance(root), g)).collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0));
        near.truncate(3);
        let ws: Vec<f64> = near.iter().map(|(dd, _)| 1.0 / (dd + 1e-6)).collect();
        let total: f64 = ws.iter().sum();
        let mut shape = vec![DVec3::ZERO; n];
        for ((_, g), w) in near.iter().zip(&ws) {
            let r = resample(g, n);
            for j in 0..n {
                shape[j] += (r[j] - r[0]) * (w / total);
            }
        }
        for j in 1..n {
            let target = root + shape[j];
            sp.points[j] = sp.points[j].lerp(target, blend);
        }
    }
}

fn linear_wire(d: &mut NodeData, s: &mut SplineData, m: f64) {
    let wires = curves_in(d, "wireCurves");
    let rests = curves_in(d, "referenceWires");
    let smooth = getf(d, "smoothness").max(1e-6);
    if wires.is_empty() {
        return;
    }
    for sp in &mut s.splines {
        let n = sp.points.len();
        let root = sp.root();
        let mut delta = vec![DVec3::ZERO; n];
        let mut wsum = 0.0;
        for (k, w) in &wires {
            let Some((_, r)) = rests.iter().find(|(rk, _)| rk == k) else { continue };
            let (wp, rp) = (resample(w, n), resample(r, n));
            let dist = rp[0].distance(root);
            let f = (1.0 - dist / smooth).max(0.0);
            if f <= 0.0 {
                continue;
            }
            let f = f * f * (3.0 - 2.0 * f);
            for j in 0..n {
                delta[j] += (wp[j] - rp[j]) * f;
            }
            wsum += f;
        }
        if wsum <= 0.0 {
            continue;
        }
        // Roots stay; the pull grows linearly to the tip.
        let k = m / wsum.max(1.0);
        let last = n_of(&sp.points);
        for j in 1..n {
            sp.points[j] += delta[j] * k * (j as f64 / last);
        }
    }
}

fn curve_to_spline(d: &mut NodeData, s: &mut SplineData, m: f64) {
    if m <= 0.0 {
        return;
    }
    let curves = curves_in(d, "inputCurves");
    let mesh = d.mesh(at(d, "boundMesh")).cloned();
    let align = getb(d, "alignToNormal");
    let cv = geti(d, "cvCount").max(0) as usize;
    let normals = mesh.as_ref().map(|m| aztek_geo::normals::point_normals(m, aztek_geo::normals::NormalWeight::Angle)).unwrap_or_default();
    for (k, c) in curves {
        let n = if cv >= 2 { cv } else { s.splines.first().map_or(5, |x| x.points.len()) };
        let mut pts = resample(&c, n);
        let id = (u32::MAX as u64) << 32 | (1 << 30) | k as u64;
        let (face, corners, w, normal, tangent) = match mesh.as_ref().and_then(|mm| super::groom::closest(mm, pts[0]).map(|x| (mm, x))) {
            Some((mm, (face, corners, w, q))) => {
                let (_, nn, tt) = super::frame(mm, &normals, corners, w);
                let shift = q - pts[0];
                for p in &mut pts {
                    *p += shift;
                }
                if align {
                    // Turned so that its first segment leaves along the normal.
                    let first = (pts[1] - pts[0]).normalize_or_zero();
                    if first != DVec3::ZERO {
                        let rot = glam::DQuat::from_rotation_arc(first, nn);
                        let r = pts[0];
                        for p in &mut pts {
                            *p = r + rot * (*p - r);
                        }
                    }
                }
                (face, corners, w, nn, tt)
            }
            None => {
                let nn = (pts[1] - pts[0]).normalize_or_zero();
                (u32::MAX, [0; 3], [1.0, 0.0, 0.0], nn, nn.any_orthonormal_vector())
            }
        };
        s.splines.push(Spline { id, face, corners, w, uv: [0.0, 0.0], normal, tangent, points: pts, width: Vec::new(), width_scale: 1.0 });
    }
}

fn curve_array(name: &str, short: &str, doc: &str) -> AttrDef {
    let mut a = crate::curves::curve_attr(name, short, flags::INPUT | flags::HIDDEN, doc);
    a.flags |= flags::ARRAY;
    a
}

fn mesh_array(name: &str, short: &str, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Mesh, default_value: PlugValue::Mesh(None), flags: flags::INPUT | flags::HIDDEN | flags::ARRAY, doc: doc.into(), ..Default::default() }
}

fn b(name: &str, short: &str, dflt: bool, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Bool, default_value: PlugValue::Bool(dflt), flags: flags::INPUT | flags::STORABLE, doc: doc.into(), ..Default::default() }
}
fn i(name: &str, short: &str, dflt: i64, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(dflt), flags: flags::INPUT | flags::STORABLE, doc: doc.into(), ..Default::default() }
}

fn own_attrs(ty: &str) -> Vec<AttrDef> {
    match ty {
        "xgmModifierClump" => vec![
            ranged(fl("clump", "clp", 0.5, "How far splines are drawn to their clump's guide, at the tip."), 0.0, 1.0),
            ranged(fl("clumpVariance", "clv", 0.0, "A random part taken off each spline's clumping."), 0.0, 1.0),
            ranged(fl("density", "den", 1.0, "Clumps per face."), 0.0, 10.0),
            i("seed", "sd", 0, "Which splines become clump guides."),
            ranged(fl("curl", "crl", 0.0, "Turns about the guide, toward the tip."), -2.0, 2.0),
            b("preserveLength", "pl", false, "Keep each spline's length."),
        ],
        "xgmModifierNoise" => vec![
            ranged(fl("frequency", "frq", 1.0, "How quickly the noise changes along and across splines."), 0.0, 20.0),
            ranged(fl("noiseMagnitude", "nmag", 0.1, "How far the noise moves points, as a part of the spline's length."), 0.0, 1.0),
            ranged(fl("correlation", "cor", 0.0, "1 moves neighbouring splines alike, 0 each on its own."), 0.0, 1.0),
            b("preserveLength", "pl", true, "Keep each spline's length."),
            i("seed", "sd", 0, "Another noise."),
            AttrDef { enum_names: vec!["Live".into(), "Baked".into()], ..i("mode", "md", 0, "Live, or Baked to a cache (not yet: Live is used).") },
        ],
        "xgmModifierCut" => vec![
            ranged(fl("amount", "amt", 0.2, "The part of each spline's length cut off."), 0.0, 1.0),
            ranged(fl("randomness", "rnd", 0.0, "A random part of Amount kept per spline."), 0.0, 1.0),
            i("seed", "sd", 0, "Another random cut."),
        ],
        "xgmModifierCollide" => vec![mesh_array("collisionMesh", "cm", "Meshes to keep the splines out of, in world space."), fl("distance", "dst", 0.0, "How far above their surfaces.")],
        "xgmModifierDisplace" => vec![fl("displacement", "dsp", 0.1, "How far along the root normal, at the tip."), super::string("displacementMap", "dsm", "A map scaling it. Kept; not applied yet.")],
        "xgmModifierGuide" => vec![curve_array("guideCurves", "gc", "Guide curves, world space."), ranged(fl("blend", "bln", 1.0, "How much the splines take of their guides' shape."), 0.0, 1.0)],
        "xgmModifierLinearWire" => vec![
            curve_array("wireCurves", "wc", "Wire curves, world space."),
            curve_array("referenceWires", "rw", "Each wire as it was when the modifier was made: its reference state."),
            ranged(fl("smoothness", "smt", 1.0, "How far from a wire's root its pull reaches, in scene units."), 0.0, 10.0),
            ranged(fl("breakage", "brk", 0.0, "Kept; not applied yet."), 0.0, 1.0),
        ],
        "xgmCurveToSpline" => vec![
            curve_array("inputCurves", "ic", "Curves to turn into splines, world space."),
            AttrDef { name: "boundMesh".into(), short_name: "bm".into(), ty: PlugType::Mesh, default_value: PlugValue::Mesh(None), flags: flags::INPUT | flags::HIDDEN, doc: "The mesh their roots go onto.".into(), ..Default::default() },
            b("alignToNormal", "atn", false, "Turn each curve to leave the surface along its normal."),
            i("cvCount", "cvc", 0, "Points on each spline; 0 matches the splines coming in."),
        ],
        _ => Vec::new(),
    }
}

fn compute_for(ty: &str) -> fn(&mut NodeData, &mut SplineData, f64) {
    match ty {
        "xgmModifierClump" => clump,
        "xgmModifierNoise" => noise,
        "xgmModifierCut" => cut,
        "xgmModifierCollide" => collide,
        "xgmModifierDisplace" => displace,
        "xgmModifierGuide" => guide,
        "xgmModifierLinearWire" => linear_wire,
        _ => curve_to_spline,
    }
}

struct Modifier(fn(&mut NodeData, &mut SplineData, f64));

impl crate::node::NodeBehavior for Modifier {
    fn compute(&mut self, _ctx: &crate::node::EvalContext, d: &mut NodeData) {
        let Some((mut s, _)) = super::modifier_input(d) else { return };
        let m = mag(d);
        (self.0)(d, &mut s, m);
        d.set_output(at(d, "outSplineData"), data_value(s));
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn register_types() {
    for &(ty, doc) in TYPES {
        let mut attrs = modifier_attrs();
        attrs.extend(own_attrs(ty));
        let f = compute_for(ty);
        types::register_type(NodeType {
            name: ty.into(),
            icon: "grain".into(),
            doc: format!("Interactive Groom modifier. {doc}"),
            own_attrs: attrs,
            create: Some(Arc::new(move || Box::new(Modifier(f)))),
            ignores_added_attrs: true,
            ..Default::default()
        })
        .expect(ty);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resample_is_even() {
        let p = vec![DVec3::ZERO, DVec3::new(0.0, 1.0, 0.0), DVec3::new(0.0, 3.0, 0.0)];
        let r = resample(&p, 4);
        assert!((r[1].y - 1.0).abs() < 1e-12 && (r[2].y - 2.0).abs() < 1e-12 && (r[3].y - 3.0).abs() < 1e-12);
    }

    #[test]
    fn noise_is_smooth_and_bounded() {
        let a = value_noise(DVec3::new(0.3, 0.4, 0.5), 1);
        let b = value_noise(DVec3::new(0.3001, 0.4, 0.5), 1);
        assert!((a - b).abs() < 1e-2 && a.abs() <= 1.0);
    }
}
