//! alignCurve and alignSurface: the reference's align tool. The end of the first curve (or surface, along U or V) meets the
//! start of the second with positional continuity (moving either or both, or changing their end CVs), tangent continuity
//! (the second CV of one lined up with the other's tangent) and curvature continuity (the third CV set so the curvature
//! matches), as live alignCurve and alignSurface nodes whose attributes are the reference's.

use std::any::Any;
use std::sync::Arc;

use aztek_geo::nurbs::{Form, NurbsCurve, NurbsSurface};
use aztek_ops::{fail, objects, ArgType, Args, CmdResult, Commands};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::attr_defs::matrix_attr;
use crate::curves::{curve_attr, curve_value, f_in, int_in, local_curve, store_bool, CurveShape};
use crate::graph::{Graph, SelectMode};
use crate::history_qe::{apply_common, f, node_flags, node_state, qe, qe_flags, Cmd};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, NodeId, PlugRef, PlugValue, NO_NODE};
use crate::rig::plug;
use crate::session::Session;
use crate::surfaces::{surface_attr, surface_value, SurfaceShape};

/// The reference's joinParameter default: no parameter given, the ends meet.
pub(crate) const NO_JOIN: f64 = 123456.0;

/// The settings both align nodes share.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Align {
    pub pc: bool,
    pub pct: i64,
    pub tc: bool,
    pub tct: i64,
    pub cc: bool,
    pub cs1: f64,
    pub cs2: f64,
    pub ts1: f64,
    pub ts2: f64,
    pub jnp: f64,
}

impl Default for Align {
    fn default() -> Self {
        Align { pc: true, pct: 1, tc: true, tct: 1, cc: false, cs1: 0.0, cs2: 0.0, ts1: 1.0, ts2: 1.0, jnp: NO_JOIN }
    }
}

/// A point on a curve with its first and second derivatives, from differences that stay inside the domain.
fn ders(c: &NurbsCurve, u: f64) -> (DVec3, DVec3, DVec3) {
    let (lo, hi) = c.domain();
    let h = ((hi - lo) * 1e-3).max(1e-7);
    let u = u.clamp(lo, hi);
    let e = |x: f64| c.eval(x);
    if u - 3.0 * h < lo {
        let (f0, f1, f2, f3) = (e(u), e(u + h), e(u + 2.0 * h), e(u + 3.0 * h));
        (f0, (-3.0 * f0 + 4.0 * f1 - f2) / (2.0 * h), (2.0 * f0 - 5.0 * f1 + 4.0 * f2 - f3) / (h * h))
    } else if u + 3.0 * h > hi {
        let (f0, f1, f2, f3) = (e(u), e(u - h), e(u - 2.0 * h), e(u - 3.0 * h));
        (f0, (3.0 * f0 - 4.0 * f1 + f2) / (2.0 * h), (2.0 * f0 - 5.0 * f1 + 4.0 * f2 - f3) / (h * h))
    } else {
        let (fm, f0, fp) = (e(u - h), e(u), e(u + h));
        (f0, (fp - fm) / (2.0 * h), (fp - 2.0 * f0 + fm) / (h * h))
    }
}

/// What a curve offers at the join: its unit tangent forward along it, and curvature vector.
#[derive(Debug, Clone, Copy)]
struct JoinRef {
    t: DVec3,
    k: DVec3,
}

fn join_ref(c: &NurbsCurve, u: f64) -> JoinRef {
    let (_, d1, d2) = ders(c, u);
    let s = d1.length();
    if s < 1e-12 {
        return JoinRef { t: DVec3::ZERO, k: DVec3::ZERO };
    }
    let t = d1 / s;
    let k = (d2 - t * d2.dot(t)) / (s * s);
    JoinRef { t, k }
}

/// End CV, the one inside it, and the third, at the start or the end.
fn end_ids(n: usize, at_start: bool) -> (usize, usize, usize) {
    if at_start {
        (0, 1, 2)
    } else {
        (n - 1, n.saturating_sub(2), n.saturating_sub(3))
    }
}

fn end_param(c: &NurbsCurve, at_start: bool) -> f64 {
    let (lo, hi) = c.domain();
    if at_start {
        lo
    } else {
        hi
    }
}

fn set_end_point(c: &mut NurbsCurve, at_start: bool, p: DVec3) {
    if c.cvs.is_empty() {
        return;
    }
    let (e, _, _) = end_ids(c.cvs.len(), at_start);
    c.cvs[e] = p;
}

/// Lines the second CV up with `t` (forward along the curve at the join), its distance from the end scaled.
fn set_end_tangent(c: &mut NurbsCurve, at_start: bool, t: DVec3, scale: f64) {
    let n = c.cvs.len();
    if n < 2 || t == DVec3::ZERO {
        return;
    }
    let (e, i, _) = end_ids(n, at_start);
    let len = (c.cvs[i] - c.cvs[e]).length().max(1e-6) * scale;
    c.cvs[i] = if at_start { c.cvs[e] + t * len } else { c.cvs[e] - t * len };
}

/// Moves the third CV so the curvature at the end is `k`; `scale` sets the second derivative's part along the tangent,
/// which leaves the curvature as it is.
fn set_end_curvature(c: &mut NurbsCurve, at_start: bool, k: DVec3, scale: f64) {
    let n = c.cvs.len();
    if n < 3 || c.degree < 2 {
        return;
    }
    let (_, _, third) = end_ids(n, at_start);
    let u = end_param(c, at_start);
    let (_, d1, d2) = ders(c, u);
    let s = d1.length();
    if s < 1e-12 {
        return;
    }
    let t = d1 / s;
    // The second derivative is linear in the CVs: how much it moves for a unit move of the third CV.
    let mut probe = c.clone();
    probe.cvs[third] += DVec3::X;
    let coef = (ders(&probe, u).2 - d2).x;
    if coef.abs() < 1e-9 {
        return;
    }
    let want = k * s * s + t * scale * s * s;
    c.cvs[third] += (want - d2) / coef;
}

fn translated(c: &NurbsCurve, by: DVec3) -> NurbsCurve {
    let mut out = c.clone();
    out.cvs.iter_mut().for_each(|p| *p += by);
    out
}

/// The parameter on the reference curve where the join is: joinParameter when given, else its end at the join.
fn ref_param(c: &NurbsCurve, at_start: bool, jnp: f64) -> f64 {
    if (jnp - NO_JOIN).abs() > 1e-9 {
        let (lo, hi) = c.domain();
        jnp.clamp(lo, hi)
    } else {
        end_param(c, at_start)
    }
}

/// Aligns the end of `a` to the start of `b`, both in one space. Returns them changed.
pub(crate) fn align_curves(a: &NurbsCurve, b: &NurbsCurve, o: &Align) -> (NurbsCurve, NurbsCurve) {
    let (mut a, mut b) = (a.clone(), b.clone());
    if !a.is_valid() || !b.is_valid() {
        return (a, b);
    }
    // Curvature continuity takes tangent continuity with it.
    let tangent = o.tc || o.cc;
    if o.pc {
        let pa = a.eval(end_param(&a, false));
        let pb = b.eval(end_param(&b, true));
        match o.pct {
            2 => {
                let target = a.eval(ref_param(&a, false, o.jnp));
                b = translated(&b, target - pb);
            }
            3 => {
                let mid = (pa + pb) * 0.5;
                a = translated(&a, mid - pa);
                b = translated(&b, mid - pb);
            }
            4 => {
                let target = b.eval(ref_param(&b, true, o.jnp));
                set_end_point(&mut a, false, target);
            }
            5 => {
                let target = a.eval(ref_param(&a, false, o.jnp));
                set_end_point(&mut b, true, target);
            }
            6 => {
                let mid = (pa + pb) * 0.5;
                set_end_point(&mut a, false, mid);
                set_end_point(&mut b, true, mid);
            }
            _ => {
                let target = b.eval(ref_param(&b, true, o.jnp));
                a = translated(&a, target - pa);
            }
        }
    }
    if tangent {
        if o.tct == 2 {
            let r = join_ref(&a, ref_param(&a, false, o.jnp));
            set_end_tangent(&mut b, true, r.t, o.ts2);
            if o.cc {
                set_end_curvature(&mut b, true, r.k, o.cs2);
            }
        } else {
            let r = join_ref(&b, ref_param(&b, true, o.jnp));
            set_end_tangent(&mut a, false, r.t, o.ts1);
            if o.cc {
                set_end_curvature(&mut a, false, r.k, o.cs1);
            }
        }
    }
    (a, b)
}

/// Two curves end to end as one. Where they touch and keep is on, the join keeps its multiple knot and the curves stay
/// exactly as they were; otherwise the curve is rebuilt smooth through both.
pub(crate) fn join_curves(a: &NurbsCurve, b: &NurbsCurve, keep: bool) -> NurbsCurve {
    let touching = a.is_valid() && b.is_valid() && (a.eval(a.domain().1) - b.eval(b.domain().0)).length() < 1e-6;
    if keep && touching && a.degree == b.degree && a.form == Form::Open && b.form == Form::Open {
        let d = a.degree;
        let (_, ea) = a.domain();
        let (sb, _) = b.domain();
        let mut cvs = a.cvs.clone();
        let mut b_cvs = b.cvs.clone();
        // The joint is the first curve's end; the second's start moves there.
        b_cvs[0] = *cvs.last().unwrap();
        cvs.extend(b_cvs.into_iter().skip(1));
        let mut knots = a.knots.clone();
        knots.extend(b.knots.iter().skip(d).map(|k| k - sb + ea));
        let joined = NurbsCurve { degree: d, form: Form::Open, cvs, knots };
        if joined.is_valid() {
            return joined;
        }
    }
    a.attached(b)
}

fn bools(d: &NodeData, at: usize) -> bool {
    d.bool(at)
}

// ---------------------------------------------------------------------------------------------------------------------
// alignCurve

const C_IN1: usize = 0;
const C_OUT1: usize = 1;
const C_IN2: usize = 2;
const C_OUT2: usize = 3;
/// positionalContinuity, positionalContinuityType, tangentContinuity, tangentContinuityType, curvatureContinuity,
/// curvatureScale1, curvatureScale2, tangentScale1, tangentScale2, joinParameter: from here, in that order.
const C_SETTINGS: usize = 4;
const C_REV1: usize = 14;
const C_REV2: usize = 15;
const C_ATTACH: usize = 16;
const C_KMK: usize = 17;
const C_SPACE1: usize = 18;
const C_SPACE2: usize = 19;

fn settings_of(d: &NodeData, at: usize) -> Align {
    Align {
        pc: d.bool(at),
        pct: d.int(at + 1),
        tc: d.bool(at + 2),
        tct: d.int(at + 3),
        cc: d.bool(at + 4),
        cs1: d.float(at + 5),
        cs2: d.float(at + 6),
        ts1: d.float(at + 7),
        ts2: d.float(at + 8),
        jnp: d.float(at + 9),
    }
}

fn settings_attrs() -> Vec<crate::plug::AttrDef> {
    use crate::plug::AttrDef;
    vec![
        AttrDef { short_name: "pc".into(), ..store_bool("positionalContinuity", true, "Make the ends meet.") },
        int_in("positionalContinuityType", "pct", 1, "1 move the first, 2 move the second, 3 move both, 4 change the first's end, 5 the second's, 6 both."),
        AttrDef { short_name: "tc".into(), ..store_bool("tangentContinuity", true, "Make the tangents line up where they meet.") },
        int_in("tangentContinuityType", "tct", 1, "1 change the first, 2 the second."),
        AttrDef { short_name: "cc".into(), ..store_bool("curvatureContinuity", false, "Match the curvature too (tangents line up with it).") },
        f_in("curvatureScale1", "cs1", 0.0, "Shapes the first's curvature continuity."),
        f_in("curvatureScale2", "cs2", 0.0, "Shapes the second's curvature continuity."),
        f_in("tangentScale1", "ts1", 1.0, "Scales the first's tangent."),
        f_in("tangentScale2", "ts2", 1.0, "Scales the second's tangent."),
        f_in("joinParameter", "jnp", NO_JOIN, "Where on the curve kept as it is the other meets it; 123456 for its end."),
    ]
}

struct AlignCurveNode;

impl NodeBehavior for AlignCurveNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let state = node_state(d);
        if state == 2 || state == 5 {
            return;
        }
        let (Some(a), Some(b)) = (curve_value(d.value(C_IN1)).cloned(), curve_value(d.value(C_IN2)).cloned()) else {
            d.set_output(C_OUT1, d.value(C_IN1).clone());
            d.set_output(C_OUT2, d.value(C_IN2).clone());
            return;
        };
        if state == 1 || state == 4 {
            d.set_output(C_OUT1, crate::curves::to_value(a));
            d.set_output(C_OUT2, crate::curves::to_value(b));
            return;
        }
        let (m1, m2) = (d.matrix(C_SPACE1), d.matrix(C_SPACE2));
        let (r1, r2) = (bools(d, C_REV1), bools(d, C_REV2));
        let mut wa = a.transformed(&m1);
        let mut wb = b.transformed(&m2);
        if r1 {
            wa = wa.reversed();
        }
        if r2 {
            wb = wb.reversed();
        }
        let (mut oa, mut ob) = align_curves(&wa, &wb, &settings_of(d, C_SETTINGS));
        if d.bool(C_ATTACH) {
            oa = join_curves(&oa, &ob, d.bool(C_KMK));
        }
        // The curves keep their direction: reversing only chose the end that meets.
        if r1 && !d.bool(C_ATTACH) {
            oa = oa.reversed();
        }
        if r2 {
            ob = ob.reversed();
        }
        d.set_output(C_OUT1, crate::curves::to_value(oa.transformed(&m1.inverse())));
        d.set_output(C_OUT2, crate::curves::to_value(ob.transformed(&m2.inverse())));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// alignSurface

const S_IN1: usize = 0;
const S_OUT1: usize = 1;
const S_IN2: usize = 2;
const S_OUT2: usize = 3;
const S_SETTINGS: usize = 4;
const S_REV1: usize = 14;
const S_REV2: usize = 15;
const S_DIR_U: usize = 16;
const S_SWAP1: usize = 17;
const S_SWAP2: usize = 18;
const S_TWIST: usize = 19;
const S_ATTACH: usize = 20;
const S_KMK: usize = 21;
const S_SPACE1: usize = 22;
const S_SPACE2: usize = 23;

/// The options on how the two surfaces are taken before they align.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SurfaceTake {
    pub dir_u: bool,
    pub rev1: bool,
    pub rev2: bool,
    pub swap1: bool,
    pub swap2: bool,
    pub twist: bool,
}

fn col_curve(s: &NurbsSurface, v: usize) -> NurbsCurve {
    NurbsCurve { degree: s.degree_u, form: s.form_u, cvs: (0..s.nu).map(|u| s.cv(u, v)).collect(), knots: s.knots_u.clone() }
}

fn put_col(s: &mut NurbsSurface, v: usize, c: &NurbsCurve) {
    for (u, p) in c.cvs.iter().enumerate() {
        s.cvs[u * s.nv + v] = *p;
    }
}

/// The surface's U curve through parameter v, sampled from its rows: used as the reference at a column.
fn iso_at(s: &NurbsSurface, v: f64) -> NurbsCurve {
    s.iso_v(v)
}

/// Maps a parameter along V of one surface to the other's by where it falls in the domain.
fn map_v(v: f64, from: (f64, f64), to: (f64, f64)) -> f64 {
    let t = if (from.1 - from.0).abs() < 1e-12 { 0.0 } else { (v - from.0) / (from.1 - from.0) };
    to.0 + t * (to.1 - to.0)
}

/// Aligns the end of `a` along U to the start of `b`, both in one space, a column of CVs at a time.
pub(crate) fn align_surfaces_u(a: &NurbsSurface, b: &NurbsSurface, o: &Align) -> (NurbsSurface, NurbsSurface) {
    let (mut a, mut b) = (a.clone(), b.clone());
    if !a.is_valid() || !b.is_valid() {
        return (a, b);
    }
    let (dva, dvb) = (a.domain_v(), b.domain_v());
    let (ua_hi, ub_lo) = (a.domain_u().1, b.domain_u().0);
    // Moves: the mean gap along the edge.
    if o.pc && matches!(o.pct, 1..=3) {
        let samples = 9;
        let mut gap = DVec3::ZERO;
        for i in 0..samples {
            let t = i as f64 / (samples - 1) as f64;
            let va = dva.0 + t * (dva.1 - dva.0);
            let vb = dvb.0 + t * (dvb.1 - dvb.0);
            let given = (o.jnp - NO_JOIN).abs() > 1e-9;
            let ua_at = if o.pct == 2 && given { o.jnp } else { ua_hi };
            let ub_at = if o.pct == 1 && given { o.jnp } else { ub_lo };
            gap += b.eval(ub_at, vb) - a.eval(ua_at, va);
        }
        gap /= samples as f64;
        let shift = |s: &mut NurbsSurface, by: DVec3| s.cvs.iter_mut().for_each(|p| *p += by);
        match o.pct {
            2 => shift(&mut b, -gap),
            3 => {
                shift(&mut a, gap * 0.5);
                shift(&mut b, -gap * 0.5);
            }
            _ => shift(&mut a, gap),
        }
    }
    // Changes to the ends, a column at a time against the other surface's U curve at that column's V.
    let col_params = |s: &NurbsSurface| s.cv_params_v();
    let per_column = o.pc && matches!(o.pct, 4..=6) || o.tc || o.cc;
    if per_column {
        let pa = col_params(&a);
        let pb = col_params(&b);
        let (a0, b0) = (a.clone(), b.clone());
        for (j, &va) in pa.iter().enumerate().take(a.nv) {
            let other = iso_at(&b0, map_v(va, dva, dvb));
            let mut col = col_curve(&a, j);
            let (na, _) = align_curves(&col, &other, &Align { pct: if o.pct == 6 { 4 } else { o.pct }, ..only_a(o) });
            col = na;
            put_col(&mut a, j, &col);
        }
        for (j, &vb) in pb.iter().enumerate().take(b.nv) {
            let other = iso_at(&a0, map_v(vb, dvb, dva));
            let col = col_curve(&b, j);
            let (_, nb) = align_curves(&other, &col, &Align { pct: if o.pct == 6 { 5 } else { o.pct }, ..only_b(o) });
            put_col(&mut b, j, &nb);
        }
        if o.pct == 6 && o.pc {
            // Both ends to halfway: the columns above went all the way, so take each back half.
            for j in 0..a.nv {
                let was = col_curve(&a0, j);
                let mut now = col_curve(&a, j);
                let e = now.cvs.len() - 1;
                now.cvs[e] = (now.cvs[e] + was.cvs[e]) * 0.5;
                put_col(&mut a, j, &now);
            }
            for j in 0..b.nv {
                let was = col_curve(&b0, j);
                let mut now = col_curve(&b, j);
                now.cvs[0] = (now.cvs[0] + was.cvs[0]) * 0.5;
                put_col(&mut b, j, &now);
            }
        }
    }
    (a, b)
}

/// The settings for changing only the first surface's columns: the moves are done already.
fn only_a(o: &Align) -> Align {
    let pc = o.pc && matches!(o.pct, 4 | 6);
    let tang = o.tct != 2;
    Align { pc, tc: o.tc && tang, cc: o.cc && tang, ..*o }
}

fn only_b(o: &Align) -> Align {
    let pc = o.pc && matches!(o.pct, 5 | 6);
    let tang = o.tct == 2;
    Align { pc, tc: o.tc && tang, cc: o.cc && tang, ..*o }
}

/// Takes the surfaces as the options say, aligns them, and gives them back in their own parameterisation.
pub(crate) fn align_surfaces(a: &NurbsSurface, b: &NurbsSurface, o: &Align, take: &SurfaceTake, attach: bool, keep: bool) -> (NurbsSurface, NurbsSurface) {
    let (mut a, mut b) = (a.clone(), b.clone());
    if take.swap1 {
        a = a.swapped();
    }
    if take.swap2 {
        b = b.swapped();
    }
    // Work along U.
    if !take.dir_u {
        a = a.swapped();
        b = b.swapped();
    }
    if take.rev1 {
        a = a.reversed_u();
    }
    if take.rev2 {
        b = b.reversed_u();
    }
    if take.twist {
        b = b.reversed_v();
    }
    let (mut oa, mut ob) = align_surfaces_u(&a, &b, o);
    if attach {
        let _ = keep;
        oa = oa.attached_u(&ob, false, 0.5);
    }
    if take.twist {
        ob = ob.reversed_v();
    }
    if take.rev2 {
        ob = ob.reversed_u();
    }
    if take.rev1 && !attach {
        oa = oa.reversed_u();
    }
    if !take.dir_u {
        oa = oa.swapped();
        ob = ob.swapped();
    }
    if take.swap2 {
        ob = ob.swapped();
    }
    if take.swap1 {
        oa = oa.swapped();
    }
    (oa, ob)
}

struct AlignSurfaceNode;

impl NodeBehavior for AlignSurfaceNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let state = node_state(d);
        if state == 2 || state == 5 {
            return;
        }
        let (Some(a), Some(b)) = (surface_value(d.value(S_IN1)).cloned(), surface_value(d.value(S_IN2)).cloned()) else {
            d.set_output(S_OUT1, d.value(S_IN1).clone());
            d.set_output(S_OUT2, d.value(S_IN2).clone());
            return;
        };
        if state == 1 || state == 4 {
            d.set_output(S_OUT1, crate::surfaces::to_value(a));
            d.set_output(S_OUT2, crate::surfaces::to_value(b));
            return;
        }
        let (m1, m2) = (d.matrix(S_SPACE1), d.matrix(S_SPACE2));
        let take = SurfaceTake { dir_u: d.bool(S_DIR_U), rev1: d.bool(S_REV1), rev2: d.bool(S_REV2), swap1: d.bool(S_SWAP1), swap2: d.bool(S_SWAP2), twist: d.bool(S_TWIST) };
        let (oa, ob) = align_surfaces(&a.transformed(&m1), &b.transformed(&m2), &settings_of(d, S_SETTINGS), &take, d.bool(S_ATTACH), d.bool(S_KMK));
        d.set_output(S_OUT1, crate::surfaces::to_value(oa.transformed(&m1.inverse())));
        d.set_output(S_OUT2, crate::surfaces::to_value(ob.transformed(&m2.inverse())));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        use crate::plug::AttrDef;
        let hidden_in = flags::INPUT | flags::STORABLE | flags::HIDDEN;
        let mut curve = vec![
            curve_attr("inputCurve1", "ic1", hidden_in, "The first curve."),
            curve_attr("outputCurve1", "oc1", flags::OUTPUT | flags::HIDDEN, "The first curve aligned."),
            curve_attr("inputCurve2", "ic2", hidden_in, "The second curve."),
            curve_attr("outputCurve2", "oc2", flags::OUTPUT | flags::HIDDEN, "The second curve aligned."),
        ];
        curve.extend(settings_attrs());
        curve.extend([
            AttrDef { short_name: "rv1".into(), ..store_bool("reverse1", false, "Reverse the first curve before aligning.") },
            AttrDef { short_name: "rv2".into(), ..store_bool("reverse2", false, "Reverse the second curve before aligning.") },
            AttrDef { short_name: "at".into(), flags: hidden_in, ..store_bool("attach", false, "Join the curves into the first once aligned.") },
            AttrDef { short_name: "kmk".into(), flags: hidden_in, ..store_bool("keepMultipleKnots", true, "Joined, keep the multiple knot where they meet.") },
            matrix_attr("space1", hidden_in, "The first curve's world matrix: it aligns in world space."),
            matrix_attr("space2", hidden_in, "The second curve's world matrix."),
        ]);
        let mut surface = vec![
            surface_attr("inputSurface1", "is1", hidden_in, "The first surface."),
            surface_attr("outputSurface1", "os1", flags::OUTPUT | flags::HIDDEN, "The first surface aligned."),
            surface_attr("inputSurface2", "is2", hidden_in, "The second surface."),
            surface_attr("outputSurface2", "os2", flags::OUTPUT | flags::HIDDEN, "The second surface aligned."),
        ];
        surface.extend(settings_attrs());
        surface.extend([
            AttrDef { short_name: "rv1".into(), ..store_bool("reverse1", false, "Reverse the first surface along the direction first.") },
            AttrDef { short_name: "rv2".into(), ..store_bool("reverse2", false, "Reverse the second surface along the direction first.") },
            AttrDef { short_name: "du".into(), ..store_bool("directionU", true, "Align along U; off, along V.") },
            AttrDef { short_name: "sw1".into(), ..store_bool("swap1", false, "Swap the first surface's U and V first.") },
            AttrDef { short_name: "sw2".into(), ..store_bool("swap2", false, "Swap the second surface's U and V first.") },
            AttrDef { short_name: "tw".into(), ..store_bool("twist", false, "Reverse the second surface across the direction first, against a twist.") },
            AttrDef { short_name: "at".into(), flags: hidden_in, ..store_bool("attach", false, "Join the surfaces into the first once aligned.") },
            AttrDef { short_name: "kmk".into(), flags: hidden_in, ..store_bool("keepMultipleKnots", true, "Joined, keep the multiple knots where they meet.") },
            matrix_attr("space1", hidden_in, "The first surface's world matrix: it aligns in world space."),
            matrix_attr("space2", hidden_in, "The second surface's world matrix."),
        ]);
        for t in [
            NodeType { name: "alignCurve".into(), icon: "chart-line".into(), doc: "Aligns the end of one curve to the start of another.".into(), own_attrs: curve, create: Some(Arc::new(|| Box::new(AlignCurveNode))), ..Default::default() },
            NodeType { name: "alignSurface".into(), icon: "chart-line".into(), doc: "Aligns the edge of one surface to the edge of another.".into(), own_attrs: surface, create: Some(Arc::new(|| Box::new(AlignSurfaceNode))), ..Default::default() },
        ] {
            let name = t.name.clone();
            types::register_type(t).expect(&name);
        }
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// The commands

fn on(a: &Args, flag: &str, def: bool) -> bool {
    match a.get(flag) {
        Some(Value::Bool(b)) => *b,
        Some(v) => v.as_i64().map_or(def, |i| i != 0),
        None => def,
    }
}

fn align_args(a: &Args) -> Align {
    Align {
        pc: on(a, "positionalContinuity", true),
        pct: a.int("positionalContinuityType", 1),
        tc: on(a, "tangentContinuity", true),
        tct: a.int("tangentContinuityType", 1),
        cc: on(a, "curvatureContinuity", false),
        cs1: a.float("curvatureScale1", 0.0),
        cs2: a.float("curvatureScale2", 0.0),
        ts1: a.float("tangentScale1", 1.0),
        ts2: a.float("tangentScale2", 1.0),
        jnp: a.float("joinParameter", NO_JOIN),
    }
}

fn set_settings(g: &mut Graph, n: NodeId, at: usize, o: &Align) -> CmdResult<()> {
    let vals = [
        PlugValue::Bool(o.pc),
        PlugValue::Int(o.pct),
        PlugValue::Bool(o.tc),
        PlugValue::Int(o.tct),
        PlugValue::Bool(o.cc),
        PlugValue::Float(o.cs1),
        PlugValue::Float(o.cs2),
        PlugValue::Float(o.ts1),
        PlugValue::Float(o.ts2),
        PlugValue::Float(o.jnp),
    ];
    for (i, v) in vals.into_iter().enumerate() {
        g.set_value(plug(n, at + i), v)?;
    }
    Ok(())
}

/// What a shape's create plug gives, fed into a node's input: its upstream when it has no tweaks, else its own geometry.
fn feed(g: &mut Graph, shape: NodeId, create: usize, tweak: usize, into: PlugRef, current: PlugValue) -> CmdResult<()> {
    let c = plug(shape, create);
    match g.source(&c) {
        Some(src) if g.node(shape).is_some_and(|n| matches!(n.data().value(tweak), PlugValue::Data(None))) => {
            g.connect(&src, &into, false)?;
        }
        _ => {
            g.set_value(into, current)?;
            let _ = g.set_value(plug(shape, tweak), PlugValue::Data(None));
        }
    }
    Ok(())
}

fn world_of(g: &mut Graph, shape: NodeId) -> DMat4 {
    let tr = crate::dag::transform_of(g, shape);
    crate::rig::world(g, tr)
}

fn curve_flags(c: Cmd) -> Cmd {
    c.flag(f("positionalContinuity", "pc", ArgType::Bool, "Make the ends meet (on)."))
        .flag(f("positionalContinuityType", "pct", ArgType::Int, "1 move the first (the default), 2 move the second, 3 move both, 4 change the first's end, 5 the second's, 6 both."))
        .flag(f("tangentContinuity", "tc", ArgType::Bool, "Line the tangents up where they meet (on)."))
        .flag(f("tangentContinuityType", "tct", ArgType::Int, "1 change the first (the default), 2 the second."))
        .flag(f("curvatureContinuity", "cc", ArgType::Bool, "Match the curvature as well (off); takes tangent continuity with it."))
        .flag(f("curvatureScale1", "cs1", ArgType::Float, "Shapes the first's curvature continuity (0)."))
        .flag(f("curvatureScale2", "cs2", ArgType::Float, "Shapes the second's curvature continuity (0)."))
        .flag(f("tangentScale1", "ts1", ArgType::Float, "Scales the first's tangent (1)."))
        .flag(f("tangentScale2", "ts2", ArgType::Float, "Scales the second's tangent (1)."))
        .flag(f("joinParameter", "jnp", ArgType::Float, "Where on the one kept as it is the other meets it (123456: its end)."))
        .flag(f("reverse1", "rv1", ArgType::Bool, "Reverse the first before aligning (off)."))
        .flag(f("reverse2", "rv2", ArgType::Bool, "Reverse the second before aligning (off)."))
        .flag(f("attach", "at", ArgType::Bool, "Join them into the first once aligned."))
        .flag(f("keepMultipleKnots", "kmk", ArgType::Bool, "Joined, keep the multiple knots where they meet (on)."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Keep the node (on by default)."))
        .flag(f("replaceOriginal", "rpo", ArgType::Bool, "Change the originals (on); off, new objects."))
        .flag(f("object", "o", ArgType::Bool, "Make the result (on); off, only the node."))
        .flag(f("name", "n", ArgType::String, "Name of the new node."))
}

fn history_on(g: &Graph, a: &Args) -> bool {
    a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history)
}

const CURVE_MAP: &[(&str, &str)] = &[];

fn align_curve_cmd(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(r) = qe(s, a, "alignCurve", &["alignCurve"], CURVE_MAP) {
        return r;
    }
    let names = objects(s, a);
    if names.len() < 2 {
        fail!("alignCurve: name or select two curves");
    }
    let g = &mut s.graph;
    let s1 = crate::curves::curve_arg(g, &names[0], "alignCurve")?;
    let s2 = crate::curves::curve_arg(g, &names[1], "alignCurve")?;
    let o = align_args(a);
    let (rv1, rv2, attach, kmk) = (on(a, "reverse1", false), on(a, "reverse2", false), on(a, "attach", false), on(a, "keepMultipleKnots", true));
    let rpo = on(a, "replaceOriginal", true);
    let object = on(a, "object", true);
    let ch = history_on(g, a);
    g.evaluate();
    if !ch {
        // Worked out once; the curves keep the result.
        let (w1, w2) = (world_of(g, s1), world_of(g, s2));
        let (Some(c1), Some(c2)) = (local_curve(g, s1), local_curve(g, s2)) else { fail!("alignCurve: no curve data") };
        let (mut wa, mut wb) = (c1.transformed(&w1), c2.transformed(&w2));
        if rv1 {
            wa = wa.reversed();
        }
        if rv2 {
            wb = wb.reversed();
        }
        let (mut oa, mut ob) = align_curves(&wa, &wb, &o);
        if attach {
            oa = join_curves(&oa, &ob, kmk);
        }
        if rv1 && !attach {
            oa = oa.reversed();
        }
        if rv2 {
            ob = ob.reversed();
        }
        g.open_chunk("alignCurve");
        let r = (|| -> CmdResult<Vec<Value>> {
            if rpo {
                crate::curves::bake_history(g, s1)?;
                crate::curves::set_cached(g, s1, oa.transformed(&w1.inverse()))?;
                let mut out = vec![json!(g.path(crate::dag::transform_of(g, s1)))];
                if !attach {
                    crate::curves::bake_history(g, s2)?;
                    crate::curves::set_cached(g, s2, ob.transformed(&w2.inverse()))?;
                    out.push(json!(g.path(crate::dag::transform_of(g, s2))));
                }
                Ok(out)
            } else {
                let (t1, _) = crate::curves::create_curve(g, "alignedCurve1", oa, false)?;
                let mut out = vec![json!(g.path(t1))];
                if !attach {
                    let (t2, _) = crate::curves::create_curve(g, "alignedCurve2", ob, false)?;
                    out.push(json!(g.path(t2)));
                }
                Ok(out)
            }
        })();
        g.close_chunk();
        let out = r?;
        if let Some(first) = out.first().and_then(Value::as_str) {
            if let Ok(Some(id)) = g.find(first) {
                g.select(&[id], SelectMode::Replace);
            }
        }
        return Ok(Value::Array(out));
    }
    g.open_chunk("alignCurve");
    let r = (|| -> CmdResult<(Vec<Value>, NodeId)> {
        let t = types::find("alignCurve").expect("alignCurve");
        let node = g.create_node(&t, &a.string("name", "alignCurve1"), NO_NODE, true)?;
        set_settings(g, node, C_SETTINGS, &o)?;
        g.set_value(plug(node, C_REV1), PlugValue::Bool(rv1))?;
        g.set_value(plug(node, C_REV2), PlugValue::Bool(rv2))?;
        g.set_value(plug(node, C_ATTACH), PlugValue::Bool(attach))?;
        g.set_value(plug(node, C_KMK), PlugValue::Bool(kmk))?;
        apply_common(g, node, a)?;
        let mut out = Vec::new();
        if rpo {
            let (c1, c2) = (local_curve(g, s1).unwrap_or_default(), local_curve(g, s2).unwrap_or_default());
            feed(g, s1, CurveShape::CREATE, CurveShape::TWEAK, plug(node, C_IN1), crate::curves::to_value(c1))?;
            feed(g, s2, CurveShape::CREATE, CurveShape::TWEAK, plug(node, C_IN2), crate::curves::to_value(c2))?;
            let (w1, w2) = (world_of(g, s1), world_of(g, s2));
            g.set_value(plug(node, C_SPACE1), PlugValue::Matrix(w1))?;
            g.set_value(plug(node, C_SPACE2), PlugValue::Matrix(w2))?;
            if object {
                g.connect(&plug(node, C_OUT1), &plug(s1, CurveShape::CREATE), true)?;
                out.push(json!(g.path(crate::dag::transform_of(g, s1))));
                if !attach {
                    g.connect(&plug(node, C_OUT2), &plug(s2, CurveShape::CREATE), true)?;
                }
            }
        } else {
            g.connect(&plug(s1, CurveShape::WORLD_SPACE), &plug(node, C_IN1), false)?;
            g.connect(&plug(s2, CurveShape::WORLD_SPACE), &plug(node, C_IN2), false)?;
            if object {
                for (k, from) in [C_OUT1, C_OUT2].into_iter().enumerate() {
                    if k == 1 && attach {
                        break;
                    }
                    let (tr, sh) = crate::curves::create_curve(g, &format!("alignedCurve{}", k + 1), NurbsCurve::default(), false)?;
                    g.connect(&plug(node, from), &plug(sh, CurveShape::CREATE), true)?;
                    out.push(json!(g.path(tr)));
                }
            }
        }
        Ok((out, node))
    })();
    g.close_chunk();
    let (mut out, node) = r?;
    g.evaluate();
    if let Some(first) = out.first().and_then(Value::as_str) {
        if let Ok(Some(id)) = g.find(first) {
            g.select(&[id], SelectMode::Replace);
        }
    }
    out.push(json!(g.path(node)));
    Ok(Value::Array(out))
}

fn align_surface_cmd(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(r) = qe(s, a, "alignSurface", &["alignSurface"], CURVE_MAP) {
        return r;
    }
    let names = objects(s, a);
    if names.len() < 2 {
        fail!("alignSurface: name or select two surfaces");
    }
    let g = &mut s.graph;
    let s1 = crate::surfaces::surface_arg(g, &names[0], "alignSurface")?;
    let s2 = crate::surfaces::surface_arg(g, &names[1], "alignSurface")?;
    let o = align_args(a);
    let take = SurfaceTake {
        dir_u: on(a, "directionU", true),
        rev1: on(a, "reverse1", false),
        rev2: on(a, "reverse2", false),
        swap1: on(a, "swap1", false),
        swap2: on(a, "swap2", false),
        twist: on(a, "twist", false),
    };
    let (attach, kmk) = (on(a, "attach", false), on(a, "keepMultipleKnots", true));
    let rpo = on(a, "replaceOriginal", true);
    let object = on(a, "object", true);
    let ch = history_on(g, a);
    g.evaluate();
    if !ch {
        let (w1, w2) = (world_of(g, s1), world_of(g, s2));
        let (Some(a1), Some(a2)) = (crate::surfaces::local_surface(g, s1), crate::surfaces::local_surface(g, s2)) else { fail!("alignSurface: no surface data") };
        let (oa, ob) = align_surfaces(&a1.transformed(&w1), &a2.transformed(&w2), &o, &take, attach, kmk);
        g.open_chunk("alignSurface");
        let r = (|| -> CmdResult<Vec<Value>> {
            if rpo {
                crate::surfaces::bake_history(g, s1)?;
                crate::surfaces::set_cached(g, s1, oa.transformed(&w1.inverse()))?;
                let mut out = vec![json!(g.path(crate::dag::transform_of(g, s1)))];
                if !attach {
                    crate::surfaces::bake_history(g, s2)?;
                    crate::surfaces::set_cached(g, s2, ob.transformed(&w2.inverse()))?;
                    out.push(json!(g.path(crate::dag::transform_of(g, s2))));
                }
                Ok(out)
            } else {
                let (t1, _) = crate::surfaces::create_surface(g, "alignedSurface1", oa)?;
                let mut out = vec![json!(g.path(t1))];
                if !attach {
                    let (t2, _) = crate::surfaces::create_surface(g, "alignedSurface2", ob)?;
                    out.push(json!(g.path(t2)));
                }
                Ok(out)
            }
        })();
        g.close_chunk();
        return Ok(Value::Array(r?));
    }
    g.open_chunk("alignSurface");
    let r = (|| -> CmdResult<(Vec<Value>, NodeId)> {
        let t = types::find("alignSurface").expect("alignSurface");
        let node = g.create_node(&t, &a.string("name", "alignSurface1"), NO_NODE, true)?;
        set_settings(g, node, S_SETTINGS, &o)?;
        for (at, v) in [(S_REV1, take.rev1), (S_REV2, take.rev2), (S_DIR_U, take.dir_u), (S_SWAP1, take.swap1), (S_SWAP2, take.swap2), (S_TWIST, take.twist), (S_ATTACH, attach), (S_KMK, kmk)] {
            g.set_value(plug(node, at), PlugValue::Bool(v))?;
        }
        apply_common(g, node, a)?;
        let mut out = Vec::new();
        if rpo {
            let (c1, c2) = (crate::surfaces::local_surface(g, s1).unwrap_or_default(), crate::surfaces::local_surface(g, s2).unwrap_or_default());
            feed(g, s1, SurfaceShape::CREATE, SurfaceShape::TWEAK, plug(node, S_IN1), crate::surfaces::to_value(c1))?;
            feed(g, s2, SurfaceShape::CREATE, SurfaceShape::TWEAK, plug(node, S_IN2), crate::surfaces::to_value(c2))?;
            let (w1, w2) = (world_of(g, s1), world_of(g, s2));
            g.set_value(plug(node, S_SPACE1), PlugValue::Matrix(w1))?;
            g.set_value(plug(node, S_SPACE2), PlugValue::Matrix(w2))?;
            if object {
                g.connect(&plug(node, S_OUT1), &plug(s1, SurfaceShape::CREATE), true)?;
                out.push(json!(g.path(crate::dag::transform_of(g, s1))));
                if !attach {
                    g.connect(&plug(node, S_OUT2), &plug(s2, SurfaceShape::CREATE), true)?;
                }
            }
        } else {
            g.connect(&plug(s1, SurfaceShape::WORLD_SPACE), &plug(node, S_IN1), false)?;
            g.connect(&plug(s2, SurfaceShape::WORLD_SPACE), &plug(node, S_IN2), false)?;
            if object {
                for (k, from) in [S_OUT1, S_OUT2].into_iter().enumerate() {
                    if k == 1 && attach {
                        break;
                    }
                    let (tr, sh) = crate::dag::create_with_shape(g, crate::surfaces::SHAPE_TYPE, &format!("alignedSurface{}", k + 1), NO_NODE)?;
                    g.connect(&plug(node, from), &plug(sh, SurfaceShape::CREATE), true)?;
                    out.push(json!(g.path(tr)));
                }
            }
        }
        Ok((out, node))
    })();
    g.close_chunk();
    let (mut out, node) = r?;
    g.evaluate();
    if let Some(first) = out.first().and_then(Value::as_str) {
        if let Ok(Some(id)) = g.find(first) {
            g.select(&[id], SelectMode::Replace);
        }
    }
    out.push(json!(g.path(node)));
    Ok(Value::Array(out))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(node_flags(qe_flags(curve_flags(
        Cmd::new(
            "alignCurve",
            "Align the end of the first curve to the start of the second through an alignCurve node: positional continuity (-pc, \
             -pct), tangent (-tc, -tct) and curvature (-cc). With no flags, positional and tangent on the two selected. Returns \
             [curve, node]; -rpo off makes new curves, -ch off keeps no node.",
            align_curve_cmd,
        )
        .positional("two curves (the selection when none); the node or a curve with -q and -e"),
    ))));
    c.register(node_flags(qe_flags(
        curve_flags(Cmd::new(
            "alignSurface",
            "Align the edge of the first surface to the edge of the second through an alignSurface node: along U (-du) or V, \
             positional (-pc, -pct), tangent (-tc, -tct) and curvature (-cc) continuity. Returns [surface, node].",
            align_surface_cmd,
        ))
        .positional("two surfaces (the selection when none); the node or a surface with -q and -e")
        .flag(f("directionU", "du", ArgType::Bool, "Along U (on); off, along V."))
        .flag(f("swap1", "sw1", ArgType::Bool, "Swap the first's U and V before aligning."))
        .flag(f("swap2", "sw2", ArgType::Bool, "Swap the second's U and V before aligning."))
        .flag(f("twist", "tw", ArgType::Bool, "Reverse the second across the direction before aligning, against a twist.")),
    )));
}
