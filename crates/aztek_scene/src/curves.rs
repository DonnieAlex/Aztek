//! NURBS curves: the nurbsCurve shape with its CVs as components, makeNurbCircle and the curve commands (curve, circle,
//! rebuildCurve, reverseCurve, closeCurve, ...), each edit a history node as in the reference; the info nodes (curveInfo,
//! nearestPointOnCurve, pointOnCurveInfo); and building surfaces from curves (extrude, revolve, loft, planar): NURBS by
//! default, polygons with -po 1, as in the reference.

use std::any::Any;
use std::sync::Arc;

use aztek_geo::nurbs::{self, Form, NurbsCurve};
use aztek_geo::Mesh;
use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat3, DMat4, DQuat, DVec3};
use serde_json::{json, Value};

use crate::attr_defs::{bool_attr, float_attr_short, mesh_attr};
use crate::components::{self, PointProvider};
use crate::graph::{Graph, SelectMode};
use crate::node::{types, vec3_attr, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::rig::{find, plug};
use crate::session::Session;
use crate::transform::{DagNode, Shape};

/// A curve on a plug.
#[derive(Debug, Clone, Default)]
pub struct CurveData(pub NurbsCurve);

impl crate::plug::Data for CurveData {
    fn kind(&self) -> &str {
        "nurbsCurveData"
    }
    fn to_json(&self) -> Value {
        let c = &self.0;
        json!({"kind": "nurbsCurveData", "degree": c.degree, "form": c.form.index(),
               "cvs": c.cvs.iter().flat_map(|p| [p.x, p.y, p.z]).collect::<Vec<f64>>(), "knots": c.knots})
    }
    fn bytes(&self) -> usize {
        self.0.cvs.len() * 24 + self.0.knots.len() * 8
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn curve_from_json(j: &Value) -> NurbsCurve {
    let nums = |k: &str| -> Vec<f64> { j.get(k).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default() };
    NurbsCurve {
        degree: j.get("degree").and_then(Value::as_u64).unwrap_or(1) as usize,
        form: Form::from_index(j.get("form").and_then(Value::as_i64).unwrap_or(0)),
        cvs: nums("cvs").chunks(3).filter(|c| c.len() == 3).map(|c| DVec3::new(c[0], c[1], c[2])).collect(),
        knots: nums("knots"),
    }
}

pub(crate) fn curve_value(v: &PlugValue) -> Option<&NurbsCurve> {
    match v {
        PlugValue::Data(Some(x)) => x.as_any().downcast_ref::<CurveData>().map(|c| &c.0),
        _ => None,
    }
}

pub fn to_value(c: NurbsCurve) -> PlugValue {
    PlugValue::Data(Some(Arc::new(CurveData(c))))
}

pub(crate) fn curve_attr(name: &str, short: &str, fl: u32, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Data, default_value: PlugValue::Data(None), flags: fl, doc: doc.into(), ..Default::default() }
}

pub(crate) fn int_out(name: &str, short: &str, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::OUTPUT, doc: doc.into(), ..Default::default() }
}

pub(crate) fn float_out(name: &str, short: &str, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Float, default_value: PlugValue::Float(0.0), flags: flags::OUTPUT, doc: doc.into(), ..Default::default() }
}

pub(crate) fn int_in(name: &str, short: &str, def: i64, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(def), flags: flags::INPUT | flags::STORABLE | flags::KEYABLE, doc: doc.into(), ..Default::default() }
}

pub(crate) fn store_bool(name: &str, def: bool, doc: &str) -> AttrDef {
    AttrDef { flags: flags::INPUT | flags::STORABLE, ..bool_attr(name, def, doc) }
}

// ---------------------------------------------------------------------------------------------------------------------
// The shape

pub const SHAPE_TYPE: &str = "nurbsCurve";

pub struct CurveShape;

impl CurveShape {
    pub const CREATE: usize = Shape::COUNT;
    pub const CACHED: usize = Shape::COUNT + 1;
    pub const TWEAK: usize = Shape::COUNT + 2;
    pub const LOCAL: usize = Shape::COUNT + 3;
    pub const WORLD_SPACE: usize = Shape::COUNT + 4;
    pub const DEGREE: usize = Shape::COUNT + 5;
    pub const SPANS: usize = Shape::COUNT + 6;
    pub const FORM: usize = Shape::COUNT + 7;
    pub const MIN_VALUE: usize = Shape::COUNT + 8;
    pub const MAX_VALUE: usize = Shape::COUNT + 9;
    pub const DISP_CV: usize = Shape::COUNT + 10;
    pub const DISP_HULL: usize = Shape::COUNT + 11;
    pub const DISP_EP: usize = Shape::COUNT + 12;
    pub const LINE_WIDTH: usize = Shape::COUNT + 13;
    pub const CONTROL_POINTS: usize = Shape::COUNT + 14;
    /// editPoints: xValueEp, yValueEp, zValueEp, three arrays.
    pub const EDIT_POINTS: usize = Shape::COUNT + 15;
}

impl NodeBehavior for CurveShape {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
        let base = curve_value(d.value(Self::CREATE)).or(curve_value(d.value(Self::CACHED))).cloned();
        let Some(mut c) = base else {
            d.set_output(Self::LOCAL, PlugValue::Data(None));
            d.set_output(Self::WORLD_SPACE, PlugValue::Data(None));
            return;
        };
        if let PlugValue::Data(Some(t)) = d.value(Self::TWEAK) {
            if let Some(t) = t.as_any().downcast_ref::<crate::rig::JsonData>() {
                for e in t.0.as_array().into_iter().flatten() {
                    let i = e.get(0).and_then(Value::as_u64).unwrap_or(u64::MAX) as usize;
                    let f = |k: usize| e.get(k).and_then(Value::as_f64).unwrap_or(0.0);
                    if let Some(p) = c.cvs.get_mut(i) {
                        *p += DVec3::new(f(1), f(2), f(3));
                    }
                }
                // The wrapped CVs of a periodic curve follow their first lap.
                if c.form == Form::Periodic {
                    let lap = c.spans();
                    for i in 0..c.degree {
                        c.cvs[lap + i] = c.cvs[i % lap];
                    }
                }
            }
        }
        // With history feeding the shape a control point is a move on top of it, as the reference's tweaks are; with none it is the CV.
        let moves = curve_value(d.value(Self::CREATE)).is_some();
        for (i, v) in d.elements(Self::CONTROL_POINTS) {
            if let (Some(p), PlugValue::Vec3(x)) = (c.cvs.get_mut(*i as usize), v) {
                *p = if moves { *p + *x } else { *x };
            }
        }
        if c.form == Form::Periodic && !d.elements(Self::CONTROL_POINTS).is_empty() {
            let lap = c.spans();
            for i in 0..c.degree {
                c.cvs[lap + i] = c.cvs[i % lap];
            }
        }
        let world = d.matrix(DagNode::WORLD_MATRIX);
        d.set_output(Self::DEGREE, PlugValue::Int(c.degree as i64));
        d.set_output(Self::SPANS, PlugValue::Int(c.spans() as i64));
        d.set_output(Self::FORM, PlugValue::Int(c.form.index()));
        let (lo, hi) = c.domain();
        d.set_output(Self::MIN_VALUE, PlugValue::Float(lo));
        d.set_output(Self::MAX_VALUE, PlugValue::Float(hi));
        // Where the curve is at each knot, in object space.
        if d.elements.len() <= Self::EDIT_POINTS + 2 {
            d.elements.resize_with(d.values.len(), Vec::new);
        }
        let mut us: Vec<f64> = c.knots.iter().copied().filter(|u| *u >= lo && *u <= hi).collect();
        us.dedup();
        for k in 0..3 {
            d.elements[Self::EDIT_POINTS + k] = us.iter().enumerate().map(|(i, u)| (i as i32, PlugValue::Float(c.eval(*u)[k]))).collect();
        }
        d.set_output(Self::WORLD_SPACE, to_value(c.transformed(&world)));
        d.set_output(Self::LOCAL, to_value(c));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn as_curve(g: &Graph, id: NodeId) -> Option<NodeId> {
    crate::dag::shape_of_type(g, id, SHAPE_TYPE)
}

/// The evaluated curve of a shape, object space.
pub fn local_curve(g: &mut Graph, shape: NodeId) -> Option<NurbsCurve> {
    g.evaluate();
    g.node(shape).and_then(|n| curve_value(n.data().value(CurveShape::LOCAL)).cloned())
}

/// The evaluated curve of a shape, world space.
pub fn world_curve(g: &mut Graph, shape: NodeId) -> Option<NurbsCurve> {
    g.evaluate();
    g.node(shape).and_then(|n| curve_value(n.data().value(CurveShape::WORLD_SPACE)).cloned())
}

/// The curve as last evaluated, for drawing.
pub fn drawn_curve(n: &crate::node::Node) -> Option<&NurbsCurve> {
    curve_value(n.data().value(CurveShape::WORLD_SPACE))
}

/// Line segments of an evaluated shape, world space.
pub fn curve_lines(n: &crate::node::Node) -> Vec<(DVec3, DVec3)> {
    let Some(c) = drawn_curve(n) else { return Vec::new() };
    let pts = c.tessellate(12);
    pts.windows(2).map(|w| (w[0], w[1])).collect()
}

/// Replaces a curve's geometry; history above it stops counting.
pub(crate) fn set_curve_data(g: &mut Graph, shape: NodeId, c: NurbsCurve) -> CmdResult<()> {
    set_cached(g, shape, c)
}

pub(crate) fn set_cached(g: &mut Graph, shape: NodeId, c: NurbsCurve) -> CmdResult<()> {
    g.set_value(plug(shape, CurveShape::CACHED), to_value(c))?;
    Ok(())
}

/// Points moved in object space: into the shape's own curve, or into its tweaks when history feeds it.
fn move_cvs(g: &mut Graph, shape: NodeId, moves: &[(u32, DVec3)]) {
    let Some(cur) = local_curve(g, shape) else { return };
    let has_history = g.source(&plug(shape, CurveShape::CREATE)).is_some();
    if !has_history {
        let mut c = cur;
        for &(i, p) in moves {
            if let Some(cv) = c.cvs.get_mut(i as usize) {
                *cv = p;
            }
        }
        if c.form == Form::Periodic {
            let lap = c.spans();
            for i in 0..c.degree {
                c.cvs[lap + i] = c.cvs[i % lap];
            }
        }
        let _ = set_cached(g, shape, c);
        return;
    }
    // Tweaks: offsets from what history gives.
    let upstream = g.node(shape).and_then(|n| curve_value(n.data().value(CurveShape::CREATE)).cloned()).unwrap_or_default();
    let mut all = cur.cvs.clone();
    for &(i, p) in moves {
        if let Some(cv) = all.get_mut(i as usize) {
            *cv = p;
        }
    }
    let list: Vec<Value> = all
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let d = *p - upstream.cvs.get(i).copied().unwrap_or(*p);
            (d.length_squared() > 1e-18).then(|| json!([i, d.x, d.y, d.z]))
        })
        .collect();
    let _ = crate::rig::set_json(g, shape, CurveShape::TWEAK, json!(list));
}

fn register_provider() {
    components::register_point_provider(PointProvider {
        node_type: SHAPE_TYPE.into(),
        points: Some(Arc::new(|g: &mut Graph, node: NodeId| {
            let Some(c) = local_curve(g, node) else { return Vec::new() };
            let mut out: Vec<(u32, DVec3)> = c.distinct_cvs().iter().enumerate().map(|(i, p)| (i as u32, *p)).collect();
            out.extend(c.edit_points().into_iter().enumerate().map(|(i, p)| (crate::nurbs_components::EP | i as u32, p)));
            out
        })),
        set_points: Some(Arc::new(|g: &mut Graph, node: NodeId, pts: &[(u32, DVec3)]| {
            let Some(c) = local_curve(g, node) else { return };
            let moves = crate::nurbs_components::curve_moves(&c, pts);
            move_cvs(g, node, &moves)
        })),
        edit_plug: None,
        frames: None,
        bake: Some(Arc::new(|g: &mut Graph, node: NodeId, m: &DMat4| {
            if let Some(c) = local_curve(g, node) {
                let _ = set_cached(g, node, c.transformed(m));
                let _ = crate::rig::set_json(g, node, CurveShape::TWEAK, Value::Null);
                g.disconnect(&plug(node, CurveShape::CREATE));
                let _ = g.set_value(plug(node, CurveShape::CREATE), PlugValue::Data(None));
            }
        })),
        soft: None,
        grid: None,
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// History

/// makeNurbCircle's curve. fixCenter off: the circle starts at the first point, its centre a radius back toward the centre
/// given. useTolerance: as few sections as keep every point within tolerance of the true circle.
#[allow(clippy::too_many_arguments)]
pub(crate) fn circle_from(normal: DVec3, center: DVec3, first: DVec3, r: f64, degree: usize, sections: usize, sweep_deg: f64, use_tol: bool, tol: f64, fix_center: bool) -> NurbsCurve {
    let n = normal.normalize_or_zero();
    let n = if n == DVec3::ZERO { DVec3::Z } else { n };
    let (center, dir) = if fix_center {
        (center, first)
    } else {
        let radial = first - center;
        let radial = radial - n * radial.dot(n);
        let x = if radial.length() < 1e-9 { n.any_orthonormal_vector() } else { radial.normalize() };
        (first - x * r, x)
    };
    if !use_tol || tol <= 0.0 {
        return nurbs::circle(center, n, dir, r, degree, sections, sweep_deg);
    }
    let error = |c: &NurbsCurve| {
        let (lo, hi) = c.domain();
        (0..=64).map(|i| ((c.eval(lo + (hi - lo) * i as f64 / 64.0) - center).length() - r).abs()).fold(0.0, f64::max)
    };
    let mut best = nurbs::circle(center, n, dir, r, degree, 3, sweep_deg);
    for k in 3..=512 {
        let c = nurbs::circle(center, n, dir, r, degree, k, sweep_deg);
        let ok = error(&c) <= tol;
        best = c;
        if ok {
            break;
        }
    }
    best
}

/// A node making a curve from settings, or changing the curve at its input.
pub struct CurveNode {
    kind: Kind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Circle,
    Square,
    Rebuild,
    Reverse,
    Close,
    Smooth,
    Fit,
    Extend,
    Offset,
    InsertKnot,
}

const IN: usize = 0;
const OUT: usize = 1;
const P0: usize = 2;

impl NodeBehavior for CurveNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let input = curve_value(d.value(IN)).cloned();
        // nodeState: 1 (has no effect) passes the input through, 2 (blocking) keeps what the node last gave.
        if !matches!(self.kind, Kind::Circle | Kind::Square) {
            match crate::history_qe::node_state(d) {
                1 | 4 => {
                    d.set_output(OUT, input.map(to_value).unwrap_or(PlugValue::Data(None)));
                    return;
                }
                2 | 5 => return,
                _ => {}
            }
        }
        let c = match self.kind {
            Kind::Circle => {
                let state = crate::history_qe::node_state(d);
                if state == 2 || state == 5 {
                    return;
                }
                let (normal, center, first) = (d.vec3(P0), d.vec3(P0 + 1), d.vec3(P0 + 2));
                let (r, sw) = (d.float(P0 + 3), d.float(P0 + 6));
                Some(circle_from(normal, center, first, r, d.int(P0 + 4).max(1) as usize, d.int(P0 + 5).max(3) as usize, sw.to_degrees(), d.bool(P0 + 7), d.float(P0 + 8), d.bool(P0 + 9)))
            }
            Kind::Square => {
                if matches!(crate::history_qe::node_state(d), 2 | 5) {
                    return;
                }
                let (normal, center) = (d.vec3(P0), d.vec3(P0 + 1));
                let (sl, deg) = (d.float(P0 + 2), d.int(P0 + 3).max(1) as usize);
                let (sl2, spans) = (d.float(P0 + 4), d.int(P0 + 5).max(1) as usize);
                let n = normal.normalize_or_zero();
                let n = if n == DVec3::ZERO { DVec3::Z } else { n };
                // Up in the square's plane: world Y laid into it, or -Z for a square facing up or down.
                let up = DVec3::Y - n * DVec3::Y.dot(n);
                let v = if up.length() > 1e-6 { up.normalize() } else { DVec3::new(0.0, 0.0, -n.y.signum()) };
                let u = v.cross(n).normalize_or_zero();
                let (a, b) = (u * sl * 0.5, v * sl2 * 0.5);
                // Top, left, bottom and right, each a curve of its own with its CVs evenly along it, one knot span per
                // span.
                let sides = [(center - a + b, center + a + b), (center - a - b, center - a + b), (center + a - b, center - a - b), (center + a + b, center + a - b)];
                for (k, (from, to)) in sides.iter().enumerate() {
                    let c = NurbsCurve::line(*from, *to, spans, deg);
                    let c = c.with_domain(0.0, spans as f64);
                    d.set_output(P0 + 6 + k, to_value(c));
                }
                // outputCurve: the four sides as one closed outline.
                let corners = [center - a + b, center + a + b, center + a - b, center - a - b];
                let mut pts: Vec<DVec3> = corners.to_vec();
                pts.push(corners[0]);
                Some(NurbsCurve::open(1, pts))
            }
            Kind::Rebuild => input.map(|c| {
                let spans = d.int(P0);
                let deg = d.int(P0 + 1).max(1) as usize;
                let keep_cvs = d.bool(P0 + 2);
                if keep_cvs {
                    NurbsCurve::open(deg, c.distinct_cvs().to_vec())
                } else {
                    c.rebuilt(if spans <= 0 { c.spans() } else { spans as usize }, deg)
                }
            }),
            Kind::Reverse => input.map(|c| c.reversed()),
            Kind::Close => input.map(|c| close_curve(&c, d.int(P0), d.float(P0 + 1), d.bool(P0 + 2), d.float(P0 + 3))),
            Kind::Smooth => input.map(|c| c.smoothed(d.float(P0), d.int(P0 + 1).max(1) as usize)),
            Kind::Fit => input.map(|c| fit_bspline(c.distinct_cvs(), d.float(P0))),
            Kind::Extend => input.map(|c| {
                if d.bool(P0 + 7) {
                    return c;
                }
                let o = ExtendOptions {
                    distance: d.float(P0),
                    start: d.int(P0 + 1),
                    method: d.int(P0 + 2),
                    kind: d.int(P0 + 3),
                    point: d.vec3(P0 + 4),
                    join: d.bool(P0 + 5),
                    remove_knots: d.bool(P0 + 6),
                };
                extend_curve(&c, &o)
            }),
            Kind::Offset => input.map(|c| {
                let n = d.vec3(P0 + 1);
                let given = d.bool(P0 + 9);
                let o = OffsetOptions {
                    distance: d.float(P0),
                    normal: (given && n != DVec3::ZERO).then_some(n),
                    connect_breaks: d.int(P0 + 2),
                    cut_loop: d.bool(P0 + 3),
                    cut_radius: d.float(P0 + 4),
                    reparameterize: d.bool(P0 + 5),
                    subdivision_density: d.int(P0 + 7),
                    tolerance: d.float(P0 + 8),
                    check_points: 4,
                };
                offset_curve(&c, &o)
            }),
            Kind::InsertKnot => input.map(|c| {
                let mut out = c;
                let u = d.float(P0);
                for _ in 0..d.int(P0 + 1).max(1) {
                    out = out.insert_knot(u);
                }
                out
            }),
        };
        d.set_output(OUT, c.map(to_value).unwrap_or(PlugValue::Data(None)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The reference's closeCurve: `ps` 0 makes the curve periodic on its own CVs (the shape moves), 1 keeps the shape, the
/// curve passing through its edit points with a span over the gap, 2 blends the ends to meet at a point `bias` of the
/// way from the end to the start, with knots added at `param` of the end spans first when `bki`.
pub(crate) fn close_curve(c: &NurbsCurve, ps: i64, bias: f64, bki: bool, param: f64) -> NurbsCurve {
    if c.form == aztek_geo::nurbs::Form::Periodic || !c.is_valid() {
        return c.clone();
    }
    if ps == 0 {
        return c.closed();
    }
    let (lo, hi) = c.domain();
    // The edit points and two more on each span between them, so even a single span keeps its shape.
    let knots = c.distinct_params();
    let mut us: Vec<f64> = Vec::new();
    for w in knots.windows(2) {
        us.extend([w[0], w[0] + (w[1] - w[0]) / 3.0, w[0] + (w[1] - w[0]) * 2.0 / 3.0]);
    }
    if let Some(&last) = knots.last() {
        us.push(last);
    }
    if ps == 2 && bki && us.len() >= 2 {
        let p = param.clamp(0.0, 1.0);
        let (a, b) = (us[0] + (us[1] - us[0]) * p, us[us.len() - 1] - (us[us.len() - 1] - us[us.len() - 2]) * p);
        us.insert(1, a);
        let at = us.len() - 1;
        us.insert(at, b);
    }
    let mut pts: Vec<DVec3> = us.iter().map(|&u| c.eval(u.clamp(lo, hi))).collect();
    if pts.len() >= 2 && pts[0].distance(*pts.last().unwrap()) < 1e-9 {
        pts.pop();
    } else if ps == 2 && pts.len() >= 2 {
        let meet = pts.last().unwrap().lerp(pts[0], bias.clamp(0.0, 1.0));
        pts[0] = meet;
        pts.pop();
    }
    if pts.len() < 3 {
        return c.closed();
    }
    NurbsCurve::interpolated_periodic(c.degree, &pts)
}

/// The reference's fitBspline: a cubic with as few spans as keep it within `tol` of every point; through them all at worst.
pub(crate) fn fit_bspline(pts: &[DVec3], tol: f64) -> NurbsCurve {
    let exact = NurbsCurve::through(3, pts);
    if pts.len() < 5 || tol <= 0.0 {
        return exact;
    }
    let within = |c: &NurbsCurve| pts.iter().all(|p| c.eval(c.closest_param(*p)).distance(*p) <= tol);
    for spans in 1..pts.len() - 2 {
        let c = exact.rebuilt(spans, 3);
        if within(&c) {
            return c;
        }
    }
    exact
}

/// extendCurve's settings.
pub(crate) struct ExtendOptions {
    pub distance: f64,
    /// 0 the end, 1 the start, 2 both.
    pub start: i64,
    /// 0 by distance, 2 to `point`.
    pub method: i64,
    /// 0 linear, 1 circular, 2 extrapolated.
    pub kind: i64,
    pub point: DVec3,
    pub join: bool,
    pub remove_knots: bool,
}

/// Points of an extension leaving `e` along unit tangent `t`, with `k` the curvature vector there and `a` the second
/// derivative per unit length squared: reaching `dist` along it, or `target` when given.
fn extension_points(e: DVec3, t: DVec3, k: DVec3, kind: i64, dist: f64, target: Option<DVec3>) -> Vec<DVec3> {
    const STEPS: usize = 8;
    match (kind, target) {
        (1, Some(p)) => {
            // The arc leaving e along t that passes through p.
            let d = p - e;
            let side = d - t * d.dot(t);
            if side.length() < 1e-9 || d.length() < 1e-12 {
                return vec![e, p];
            }
            let n = side.normalize();
            let r = d.length_squared() / (2.0 * d.dot(n));
            let c = e + n * r;
            let axis = t.cross(n);
            let (u, w) = (e - c, p - c);
            let angle = u.cross(w).dot(axis).atan2(u.dot(w)).rem_euclid(std::f64::consts::TAU);
            (0..=STEPS).map(|i| c + glam::DQuat::from_axis_angle(axis, angle * i as f64 / STEPS as f64) * u).collect()
        }
        (1, None) if k.length() > 1e-9 => {
            let r = 1.0 / k.length();
            let n = k.normalize();
            let c = e + n * r;
            let axis = t.cross(n);
            let u = e - c;
            (0..=STEPS).map(|i| c + glam::DQuat::from_axis_angle(axis, dist / r * i as f64 / STEPS as f64) * u).collect()
        }
        (2, _) => {
            // The end's shape carried on: a parabola matching position, tangent and curvature.
            let len = target.map_or(dist, |p| (p - e).length());
            let mut pts: Vec<DVec3> = (0..=STEPS)
                .map(|i| {
                    let s = len * i as f64 / STEPS as f64;
                    e + t * s + k * (0.5 * s * s)
                })
                .collect();
            if let Some(p) = target {
                // Bend the tail onto the point.
                let miss = p - *pts.last().unwrap();
                for (i, q) in pts.iter_mut().enumerate() {
                    let f = i as f64 / STEPS as f64;
                    *q += miss * f * f;
                }
            }
            pts
        }
        _ => {
            let end = target.unwrap_or(e + t * dist);
            (0..=STEPS).map(|i| e.lerp(end, i as f64 / STEPS as f64)).collect()
        }
    }
}

/// The extension at one end, running away from the curve, as a curve of the curve's degree.
fn extension_at(c: &NurbsCurve, at_start: bool, o: &ExtendOptions) -> Option<NurbsCurve> {
    let (lo, hi) = c.domain();
    let u = if at_start { lo } else { hi };
    let e = c.eval(u);
    let h = (hi - lo).max(1e-6) * 1e-3;
    let sign = if at_start { -1.0 } else { 1.0 };
    let d1 = c.tangent(u) * sign;
    let speed = d1.length();
    if speed < 1e-12 {
        return None;
    }
    let t = d1 / speed;
    let _ = h;
    // Curvature from the circle through the end and two points just inside it.
    let step = (hi - lo) / c.spans().max(1) as f64 * 0.05;
    let (p1, p2) = if at_start { (c.eval(u + step), c.eval(u + 2.0 * step)) } else { (c.eval(u - step), c.eval(u - 2.0 * step)) };
    let k = {
        let (ab, ac) = (p1 - e, p2 - e);
        let n = ab.cross(ac);
        if n.length_squared() < 1e-18 {
            DVec3::ZERO
        } else {
            let centre = e + n.cross(ab * ac.length_squared() - ac * ab.length_squared()) / (2.0 * n.length_squared());
            let r2 = (centre - e).length_squared();
            let towards = centre - e;
            (towards - t * towards.dot(t)).normalize_or_zero() / r2.sqrt()
        }
    };
    let target = (o.method == 2).then_some(o.point);
    let pts = extension_points(e, t, k, o.kind, o.distance, target);
    if pts.len() < 2 || pts[0].distance(*pts.last().unwrap()) < 1e-12 {
        return None;
    }
    Some(if o.kind == 0 { NurbsCurve::line(pts[0], *pts.last().unwrap(), 1, c.degree) } else { NurbsCurve::through(c.degree, &pts) })
}

/// Two clamped curves of one degree, the second starting where the first ends: one curve, the first exactly as it was,
/// a knot of full multiplicity where they meet.
fn concatenated(a: &NurbsCurve, b: &NurbsCurve) -> NurbsCurve {
    let d = a.degree;
    if b.degree != d || a.form == Form::Periodic {
        return a.attached(b);
    }
    let (_, hi) = a.domain();
    let (blo, _) = b.domain();
    let mut cvs = a.cvs.clone();
    cvs.extend(b.cvs.iter().skip(1));
    let mut knots = a.knots.clone();
    knots.extend(b.knots.iter().skip(d).map(|k| k - blo + hi));
    let c = NurbsCurve::new(d, Form::Open, cvs, knots);
    if c.is_valid() { c } else { a.attached(b) }
}

/// The reference's extendCurve: an end (or both) carried on by a line, an arc or the curve's own shape, by a distance
/// or to a point; joined to the curve or on its own.
pub(crate) fn extend_curve(c: &NurbsCurve, o: &ExtendOptions) -> NurbsCurve {
    if !c.is_valid() || c.form == Form::Periodic {
        return c.clone();
    }
    let c = c.clamped();
    let (at_start, at_end) = match o.start {
        0 => (false, true),
        2 => (true, true),
        _ => (true, false),
    };
    let start = if at_start { extension_at(&c, true, o) } else { None };
    let end = if at_end { extension_at(&c, false, o) } else { None };
    if !o.join {
        // The extension alone, running out from the curve.
        return end.or(start).unwrap_or(c);
    }
    if o.remove_knots {
        // One smooth curve through the original and its extensions.
        let mut pts: Vec<DVec3> = Vec::new();
        if let Some(s) = &start {
            pts.extend(s.reversed().tessellate(4));
            pts.pop();
        }
        pts.extend(c.tessellate(8));
        if let Some(e) = &end {
            pts.extend(e.tessellate(4).into_iter().skip(1));
        }
        let extra = usize::from(start.is_some()) + usize::from(end.is_some());
        return NurbsCurve::through(c.degree, &pts).rebuilt(c.spans() + extra, c.degree);
    }
    let mut out = c.clone();
    if let Some(s) = start {
        let r = s.reversed();
        out = concatenated(&r, &out);
    }
    if let Some(e) = end {
        out = concatenated(&out, &e);
    }
    out
}

fn curve_op_type(name: &str, kind: Kind, doc: &str, params: Vec<AttrDef>) -> NodeType {
    let mut attrs = vec![
        curve_attr("inputCurve", "ic", flags::INPUT | flags::STORABLE | flags::HIDDEN, "The curve changed."),
        curve_attr("outputCurve", "oc", flags::OUTPUT | flags::HIDDEN, "The result."),
    ];
    attrs.extend(params);
    NodeType {
        name: name.into(),
        icon: "chart-line".into(),
        doc: doc.into(),
        own_attrs: attrs,
        create: Some(Arc::new(move || Box::new(CurveNode { kind }))),
        ..Default::default()
    }
}

/// offsetCurve's settings.
pub(crate) struct OffsetOptions {
    pub distance: f64,
    pub normal: Option<DVec3>,
    pub connect_breaks: i64,
    pub cut_loop: bool,
    pub cut_radius: f64,
    pub reparameterize: bool,
    pub subdivision_density: i64,
    pub tolerance: f64,
    /// Samples a span starts with, where the fit is checked.
    pub check_points: usize,
}

/// The offset of a curve in its plane. A curve of straight spans is offset span by span and the breaks at its corners
/// joined as connect_breaks says; a smooth one is sampled, each span halved (up to subdivision_density times) until the
/// samples between keep the distance to the tolerance. Loops where the offset crosses itself are cut with cut_loop.
pub(crate) fn offset_curve(c: &NurbsCurve, o: &OffsetOptions) -> NurbsCurve {
    let n = o.normal.unwrap_or_else(|| c.plane_normal()).normalize_or_zero();
    let n = if n == DVec3::ZERO { DVec3::Y } else { n };
    let d = o.distance;
    let side = |t: DVec3| t.normalize_or_zero().cross(n).normalize_or_zero();
    let mut pts: Vec<DVec3>;
    let straight = c.degree == 1;
    if straight {
        let cv = c.distinct_cvs().to_vec();
        if cv.len() < 2 {
            return c.clone();
        }
        pts = Vec::new();
        for i in 0..cv.len() - 1 {
            let off = side(cv[i + 1] - cv[i]) * d;
            let (a, b) = (cv[i] + off, cv[i + 1] + off);
            if i == 0 {
                pts.push(a);
            } else {
                let prev = *pts.last().unwrap();
                let into = cv[i] - cv[i - 1];
                if prev.distance(a) > 1e-9 {
                    match o.connect_breaks {
                        // Joined at a sharp corner: the two offset lines met.
                        2 => {
                            let (p0, t0) = (prev, into.normalize_or_zero());
                            let (p1, t1) = (a, (cv[i + 1] - cv[i]).normalize_or_zero());
                            let w = t0.cross(t1);
                            if w.length() > 1e-9 {
                                let s0 = (p1 - p0).cross(t1).dot(w) / w.length_squared();
                                *pts.last_mut().unwrap() = p0 + t0 * s0;
                            } else {
                                pts.push(a);
                            }
                        }
                        // An arc round the corner, the distance out.
                        1 => {
                            let (u0, u1) = ((prev - cv[i]).normalize_or_zero(), (a - cv[i]).normalize_or_zero());
                            let ang = u0.dot(u1).clamp(-1.0, 1.0).acos();
                            let steps = ((ang / 0.2).ceil() as usize).max(2);
                            let axis = u0.cross(u1).normalize_or_zero();
                            for k in 1..steps {
                                let q = DQuat::from_axis_angle(if axis == DVec3::ZERO { n } else { axis }, ang * k as f64 / steps as f64);
                                pts.push(cv[i] + q * u0 * d.abs());
                            }
                            pts.push(a);
                        }
                        _ => pts.push(a),
                    }
                }
            }
            pts.push(b);
        }
    } else {
        let (lo, hi) = c.domain();
        let spans = c.spans().max(1);
        let at = |u: f64| c.eval(u) + side(c.tangent(u)) * d;
        let span_ends: Vec<f64> = (0..=spans).map(|i| lo + (hi - lo) * i as f64 / spans as f64).collect();
        pts = vec![at(lo)];
        for w in span_ends.windows(2) {
            let mut per = o.check_points.max(1);
            for _ in 0..o.subdivision_density.max(0) {
                // Between two samples, how far the chord's middle is from the distance wanted.
                let mut worst = 0.0_f64;
                for k in 0..per {
                    let (u0, u1) = (w[0] + (w[1] - w[0]) * k as f64 / per as f64, w[0] + (w[1] - w[0]) * (k + 1) as f64 / per as f64);
                    let mid = (at(u0) + at(u1)) * 0.5;
                    let near = c.eval(c.closest_param(mid));
                    worst = worst.max((mid.distance(near) - d.abs()).abs());
                }
                if worst <= o.tolerance.max(1e-6) {
                    break;
                }
                per *= 2;
            }
            for k in 1..=per {
                pts.push(at(w[0] + (w[1] - w[0]) * k as f64 / per as f64));
            }
        }
    }
    if o.cut_loop {
        pts = cut_loops(&pts, n, o.cut_radius);
    }
    if straight {
        return NurbsCurve::open(1, pts);
    }
    let fit = NurbsCurve::through(c.degree, &pts);
    if o.reparameterize {
        // A parameterization of its own: even in length along the offset.
        let spans = (pts.len() - 1).max(1).min(c.spans().max(1) * 4);
        return fit.rebuilt(spans, c.degree);
    }
    fit.rebuilt(c.spans().max(1), c.degree)
}

/// Where a polyline crosses itself in the plane of n, the loop between is cut out, leaving a corner (or an arc of
/// radius r) at the crossing.
fn cut_loops(pts: &[DVec3], n: DVec3, r: f64) -> Vec<DVec3> {
    let (x, y) = n.any_orthonormal_pair();
    let flat = |p: DVec3| glam::DVec2::new(p.dot(x), p.dot(y));
    let mut out: Vec<DVec3> = pts.to_vec();
    let mut guard = 0;
    'again: while guard < 64 {
        guard += 1;
        let m = out.len();
        for i in 0..m.saturating_sub(1) {
            for j in (i + 2)..m.saturating_sub(1) {
                let (a, b, c2, d2) = (flat(out[i]), flat(out[i + 1]), flat(out[j]), flat(out[j + 1]));
                let r1 = b - a;
                let r2 = d2 - c2;
                let den = r1.perp_dot(r2);
                if den.abs() < 1e-12 {
                    continue;
                }
                let t = (c2 - a).perp_dot(r2) / den;
                let u = (c2 - a).perp_dot(r1) / den;
                if t > 1e-9 && t < 1.0 - 1e-9 && u > 1e-9 && u < 1.0 - 1e-9 {
                    let hit = out[i].lerp(out[i + 1], t);
                    let mut next: Vec<DVec3> = out[..=i].to_vec();
                    if r > 1e-9 {
                        // An arc tangent to both sides, of radius r.
                        let (t0, t1) = ((out[i + 1] - out[i]).normalize_or_zero(), (out[j + 1] - out[j]).normalize_or_zero());
                        let ang = t0.dot(t1).clamp(-1.0, 1.0).acos();
                        let back = r * (ang * 0.5).tan();
                        let (p0, p1) = (hit - t0 * back, hit + t1 * back);
                        let bis = (t1 - t0).normalize_or_zero();
                        let centre = hit + bis * (r * r + back * back).sqrt();
                        let (v0, v1) = ((p0 - centre).normalize_or_zero(), (p1 - centre).normalize_or_zero());
                        let sweep = v0.dot(v1).clamp(-1.0, 1.0).acos();
                        let axis = v0.cross(v1).normalize_or_zero();
                        next.push(p0);
                        let steps = ((sweep / 0.2).ceil() as usize).max(2);
                        for k in 1..steps {
                            next.push(centre + DQuat::from_axis_angle(if axis == DVec3::ZERO { n } else { axis }, sweep * k as f64 / steps as f64) * v0 * r);
                        }
                        next.push(p1);
                    } else {
                        next.push(hit);
                    }
                    next.extend_from_slice(&out[j + 1..]);
                    out = next;
                    continue 'again;
                }
            }
        }
        break;
    }
    out
}

/// The facts pointOnCurve and pointOnCurveInfo give at a parameter: position, tangent, unit tangent, normal, unit
/// normal, centre and radius of curvature. The normal is the curve's bend, its second derivative square to the tangent;
/// a straight curve has none, and a radius of 0.
pub(crate) fn curve_facts(c: &NurbsCurve, u: f64) -> [Value; 7] {
    let (lo, hi) = c.domain();
    let u = u.clamp(lo, hi);
    let p = c.eval(u);
    let t = c.tangent(u);
    let h = ((hi - lo) * 1e-4).max(1e-7);
    let (a, b) = ((u - h).max(lo), (u + h).min(hi));
    let dd = if b > a { (c.tangent(b) - c.tangent(a)) / (b - a) } else { DVec3::ZERO };
    let tu = t.normalize_or_zero();
    let normal = dd - tu * dd.dot(tu);
    let speed = t.length();
    let k = if speed > 1e-12 { t.cross(dd).length() / speed.powi(3) } else { 0.0 };
    let (radius, centre) = if k > 1e-9 { (1.0 / k, p + normal.normalize_or_zero() / k) } else { (0.0, p) };
    let v = |x: DVec3| json!([x.x, x.y, x.z]);
    [v(p), v(t), v(tu), v(normal), v(normal.normalize_or_zero()), v(centre), json!(radius)]
}

fn point_on_curve(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(r) = crate::history_qe::qe(s, a, "pointOnCurve", &["pointOnCurveInfo"], &[]) {
        return r;
    }
    let names = objects(s, a);
    let Some(n) = names.first() else { fail!("pointOnCurve: name or select a curve") };
    let g = &mut s.graph;
    let shape = curve_arg(g, n, "pointOnCurve")?;
    let on = |k: &str| a.get(k).is_some_and(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
    if on("constructionHistory") {
        let t = types::find("pointOnCurveInfo").expect("pointOnCurveInfo");
        let node = g.create_node(&t, "pointOnCurveInfo1", NO_NODE, true)?;
        g.connect(&plug(shape, CurveShape::WORLD_SPACE), &plug(node, 0), false)?;
        g.set_value(plug(node, 1), PlugValue::Float(a.float("parameter", 0.0)))?;
        g.set_value(plug(node, 2), PlugValue::Bool(on("turnOnPercentage")))?;
        crate::history_qe::apply_common(g, node, a)?;
        g.evaluate();
        return Ok(json!(g.path(node)));
    }
    let Some(c) = world_curve(g, shape) else { fail!("pointOnCurve: {n} has no curve") };
    let u = a.float("parameter", 0.0);
    let u = if on("turnOnPercentage") {
        let (lo, hi) = c.domain();
        lo + (hi - lo) * u
    } else {
        u
    };
    let facts = curve_facts(&c, u);
    let pick = [("position", 0), ("tangent", 1), ("normalizedTangent", 2), ("normal", 3), ("normalizedNormal", 4), ("curvatureCenter", 5), ("curvatureRadius", 6)];
    let asked: Vec<usize> = pick.iter().filter(|(k, _)| on(k)).map(|(_, i)| *i).collect();
    match asked.as_slice() {
        [] => Ok(facts[0].clone()),
        [one] => Ok(facts[*one].clone()),
        many => Ok(Value::Array(many.iter().flat_map(|&i| match &facts[i] {
            Value::Array(x) => x.clone(),
            x => vec![x.clone()],
        }).collect())),
    }
}

/// A node reading curves: the info nodes.
struct InfoNode;

impl NodeBehavior for InfoNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let state = crate::history_qe::node_state(d);
        if state == 2 || state == 5 {
            return;
        }
        let Some(c) = curve_value(d.value(0)).cloned() else { return };
        d.set_output(1, PlugValue::Float(c.length()));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// pointOnCurveInfo: position, tangent and normal at a parameter.
struct PointOnCurveNode;

impl NodeBehavior for PointOnCurveNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let Some(c) = curve_value(d.value(0)).cloned() else { return };
        let u = if d.bool(2) {
            let (lo, hi) = c.domain();
            lo + (hi - lo) * d.float(1)
        } else {
            d.float(1)
        };
        if matches!(crate::history_qe::node_state(d), 2 | 5) {
            return;
        }
        let facts = curve_facts(&c, u);
        let vec = |v: &Value| {
            let x: Vec<f64> = v.as_array().map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
            DVec3::new(x[0], x[1], x[2])
        };
        for (i, k) in [0, 1, 2, 3, 4, 5].iter().enumerate() {
            d.set_output(3 + i, PlugValue::Vec3(vec(&facts[*k])));
        }
        d.set_output(9, PlugValue::Float(facts[6].as_f64().unwrap_or(0.0)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// nearestPointOnCurve.
struct NearestNode;

impl NodeBehavior for NearestNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let Some(c) = curve_value(d.value(0)).cloned() else { return };
        let p = d.vec3(1);
        let u = c.closest_param(p);
        d.set_output(2, PlugValue::Float(u));
        d.set_output(3, PlugValue::Vec3(c.eval(u)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Meshes from curves

#[derive(Clone, Copy, PartialEq, Eq)]
enum SurfKind {
    Extrude,
    Revolve,
    Loft,
    Planar,
    Sweep,
}

struct SurfNode {
    kind: SurfKind,
}

const S_CURVES: usize = 0;
const S_OUT_SURFACE: usize = 1;
const S_OUT: usize = 2;
const S_POLYGON: usize = 3;
const S_P: usize = 4;

fn grid_mesh(rows: &[Vec<DVec3>], close_u: bool, close_v: bool) -> Mesh {
    let mut m = Mesh::default();
    let nv = rows.len();
    if nv == 0 {
        return m;
    }
    let nu = rows[0].len();
    let ids: Vec<Vec<u32>> = rows.iter().map(|r| r.iter().map(|p| m.add_point(*p)).collect()).collect();
    let umax = if close_u { nu } else { nu.saturating_sub(1) };
    let vmax = if close_v { nv } else { nv.saturating_sub(1) };
    for v in 0..vmax {
        for u in 0..umax {
            let (a, b, c, e) = (ids[v][u], ids[v][(u + 1) % nu], ids[(v + 1) % nv][(u + 1) % nu], ids[(v + 1) % nv][u]);
            m.add_face(&[a, b, c, e]);
        }
    }
    m
}

/// Rotation frames along a path, carrying the first along without twist.
fn frames_along(path: &[DVec3]) -> Vec<DMat3> {
    let mut out = Vec::with_capacity(path.len());
    let tangent = |i: usize| -> DVec3 {
        let a = path[i.saturating_sub(1)];
        let b = path[(i + 1).min(path.len() - 1)];
        (b - a).normalize_or_zero()
    };
    let mut last_t = tangent(0);
    let mut x = last_t.any_orthonormal_vector();
    for i in 0..path.len() {
        let t = tangent(i);
        let q = if t != DVec3::ZERO && last_t != DVec3::ZERO { DQuat::from_rotation_arc(last_t, t) } else { DQuat::IDENTITY };
        x = (q * x).normalize_or_zero();
        out.push(DMat3::from_cols(x, t.cross(x), t));
        last_t = if t != DVec3::ZERO { t } else { last_t };
    }
    out
}

fn is_closed(c: &NurbsCurve) -> bool {
    c.form != Form::Open || (c.eval(c.domain().0) - c.eval(c.domain().1)).length() < 1e-6
}

fn open_points(c: &NurbsCurve, per_span: usize) -> (Vec<DVec3>, bool) {
    let mut pts = c.tessellate(per_span);
    let closed = is_closed(c);
    if closed && pts.len() > 1 {
        pts.pop();
    }
    (pts, closed)
}

impl NodeBehavior for SurfNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        if matches!(crate::history_qe::node_state(d), 2 | 5) {
            return;
        }
        let curves: Vec<NurbsCurve> = d.elements(S_CURVES).iter().filter_map(|(_, v)| curve_value(v).cloned()).collect();
        if self.kind != SurfKind::Sweep && d.int(S_POLYGON) != 1 {
            let surface = match self.kind {
                SurfKind::Extrude => curves.first().and_then(|profile| {
                    let o = crate::surfaces::ExtrudeOptions {
                        extrude_type: d.int(S_P + 2),
                        direction: d.vec3(S_P + 4),
                        length: d.float(S_P + 5),
                        scale: d.float(S_P),
                        rotation: d.float(S_P + 1),
                        fixed_path: d.bool(S_P + 6),
                        profile_normal: d.bool(S_P + 7),
                        pivot_mode: d.int(S_P + 8),
                        degree_along: d.int(S_P + 9).clamp(1, 7) as usize,
                        pivot: d.vec3(S_P + 10),
                        reverse_if_reversed: d.bool(S_P + 11),
                    };
                    crate::surfaces::extrude_surface(profile, curves.get(1), o)
                }),
                SurfKind::Revolve => curves.first().filter(|c| c.is_valid()).map(|c| {
                    let (start, end) = (d.float(S_P + 4), d.float(S_P + 5));
                    nurbs::revolved(c, d.vec3(S_P), d.vec3(S_P + 1), start.to_degrees(), end.to_degrees(), d.int(S_P + 2).max(3) as usize, d.int(S_P + 6).clamp(1, 3) as usize)
                }),
                SurfKind::Loft => {
                    // Per curve, by the curve's place: reverse it (with autoReverse off) and break the tangent there.
                    let per = |attr: usize| -> Vec<bool> {
                        let mut v = vec![false; curves.len()];
                        for (k, x) in d.elements(attr) {
                            if let (Some(slot), PlugValue::Bool(b)) = (v.get_mut(*k as usize), x) {
                                *slot = *b;
                            }
                        }
                        v
                    };
                    let (rev, cusp) = (per(S_P + 7), per(S_P + 8));
                    let auto = d.bool(S_P + 5);
                    let list: Vec<NurbsCurve> = curves.iter().enumerate().map(|(k, c)| if !auto && rev[k] { c.reversed() } else { c.clone() }).collect();
                    crate::surfaces::loft_surface_cusps(
                        &list,
                        crate::surfaces::LoftOptions {
                            close: d.bool(S_P),
                            uniform: d.bool(S_P + 2),
                            degree: d.int(S_P + 3).max(1) as usize,
                            section_spans: d.int(S_P + 4).max(1) as usize,
                            auto_reverse: auto,
                            reverse_normals: d.bool(S_P + 6),
                        },
                        &cusp,
                    )
                }
                SurfKind::Planar => crate::nurbs_ops::planar_surface(&curves, d.int(S_P).max(1) as usize, d.bool(S_P + 1), d.float(S_P + 2)),
                SurfKind::Sweep => None,
            };
            d.set_output(S_OUT_SURFACE, surface.map(crate::surfaces::to_value).unwrap_or(PlugValue::Data(None)));
            d.set_output(S_OUT, PlugValue::Mesh(None));
            return;
        }
        d.set_output(S_OUT_SURFACE, PlugValue::Data(None));
        let mesh = match self.kind {
            SurfKind::Extrude => (|| {
                let profile = curves.first()?;
                let (pts, closed_u) = open_points(profile, d.int(S_P + 3).max(1) as usize);
                let scale = d.float(S_P);
                let rotation = d.float(S_P + 1);
                let extrude_type = d.int(S_P + 2);
                let rows: Vec<Vec<DVec3>> = match curves.get(1) {
                    Some(path) if extrude_type != 0 => {
                        let spans = (path.spans() * 8).max(2);
                        let path_pts = path.tessellate(8);
                        let _ = spans;
                        let fr = frames_along(&path_pts);
                        let start = path_pts[0];
                        let f0 = fr[0];
                        // At path (tube): the profile's first placement is carried along; the profile stays where it
                        // sits relative to the path's start.
                        let local: Vec<DVec3> = pts.iter().map(|p| f0.transpose() * (*p - start)).collect();
                        let n = path_pts.len();
                        (0..n)
                            .map(|i| {
                                let s = i as f64 / (n - 1).max(1) as f64;
                                let (sc, ang) = (1.0 + (scale - 1.0) * s, rotation * s);
                                let turn = DMat3::from_rotation_z(ang);
                                local.iter().map(|l| path_pts[i] + fr[i] * (turn * (*l * DVec3::new(sc, sc, 1.0)))).collect()
                            })
                            .collect()
                    }
                    Some(path) => {
                        // Distance: straight along the path's overall direction.
                        let (lo, hi) = path.domain();
                        let off = path.eval(hi) - path.eval(lo);
                        (0..=8).map(|i| pts.iter().map(|p| *p + off * (i as f64 / 8.0)).collect()).collect()
                    }
                    None => {
                        let dir = d.vec3(S_P + 4);
                        let len = d.float(S_P + 5);
                        (0..=1).map(|i| pts.iter().map(|p| *p + dir.normalize_or_zero() * len * i as f64).collect()).collect()
                    }
                };
                Some(grid_mesh(&rows, closed_u, false))
            })(),
            SurfKind::Revolve => (|| {
                let c = curves.first()?;
                let (pts, closed_u) = open_points(c, d.int(S_P + 3).max(1) as usize);
                let axis = d.vec3(S_P).normalize_or_zero();
                let axis = if axis == DVec3::ZERO { DVec3::Y } else { axis };
                let pivot = d.vec3(S_P + 1);
                let sections = d.int(S_P + 2).max(3) as usize;
                let (start, end) = (d.float(S_P + 4), d.float(S_P + 5));
                let full = ((end - start).abs() - std::f64::consts::TAU).abs() < 1e-6;
                let rows_n = if full { sections } else { sections + 1 };
                let rows: Vec<Vec<DVec3>> = (0..rows_n)
                    .map(|i| {
                        let a = start + (end - start) * i as f64 / sections as f64;
                        let q = DQuat::from_axis_angle(axis, a);
                        pts.iter().map(|p| pivot + q * (*p - pivot)).collect()
                    })
                    .collect();
                Some(grid_mesh(&rows, closed_u, full))
            })(),
            SurfKind::Loft => (|| {
                if curves.len() < 2 {
                    return None;
                }
                let per = d.int(S_P + 1).max(1) as usize;
                let close_v = d.bool(S_P);
                let (first, closed_u) = open_points(&curves[0], per);
                let n = first.len();
                // Each section sampled at the same number of points.
                let rows: Vec<Vec<DVec3>> = curves
                    .iter()
                    .map(|c| {
                        let (lo, hi) = c.domain();
                        (0..n)
                            .map(|i| {
                                let t = i as f64 / if closed_u { n as f64 } else { (n - 1).max(1) as f64 };
                                c.eval(lo + (hi - lo) * t)
                            })
                            .collect()
                    })
                    .collect();
                Some(grid_mesh(&rows, closed_u, close_v))
            })(),
            SurfKind::Planar => (|| {
                let c = curves.first()?;
                let (pts, _) = open_points(c, 8);
                let mut m = Mesh::default();
                let ids: Vec<u32> = pts.iter().map(|p| m.add_point(*p)).collect();
                if ids.len() >= 3 {
                    m.add_face(&ids);
                }
                Some(m)
            })(),
            SurfKind::Sweep => (|| {
                let path = curves.first()?;
                let radius = d.float(S_P);
                let sides = d.int(S_P + 1).max(3) as usize;
                let per = d.int(S_P + 2).max(1) as usize;
                let scale_end = d.float(S_P + 3);
                let twist = d.float(S_P + 4);
                let path_pts = path.tessellate(per);
                let fr = frames_along(&path_pts);
                let closed_v = is_closed(path);
                let n = if closed_v { path_pts.len() - 1 } else { path_pts.len() };
                let rows: Vec<Vec<DVec3>> = (0..n)
                    .map(|i| {
                        let s = i as f64 / (n - 1).max(1) as f64;
                        let (r, a0) = (radius * (1.0 + (scale_end - 1.0) * s), twist.to_radians() * s);
                        (0..sides)
                            .map(|k| {
                                let a = a0 + k as f64 / sides as f64 * std::f64::consts::TAU;
                                path_pts[i] + fr[i] * DVec3::new(a.cos() * r, a.sin() * r, 0.0)
                            })
                            .collect()
                    })
                    .collect();
                Some(grid_mesh(&rows, true, closed_v))
            })(),
        };
        d.set_output(S_OUT, PlugValue::Mesh(mesh.map(Arc::new)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn surf_type(name: &str, kind: SurfKind, doc: &str, params: Vec<AttrDef>) -> NodeType {
    let mut attrs = vec![
        curve_attr("inputCurve", "ic", flags::INPUT | flags::ARRAY | flags::HIDDEN, "The curves it's made from."),
        curve_attr("outputSurface", "os", flags::OUTPUT | flags::HIDDEN, "The NURBS surface."),
        mesh_attr("outputMesh", "om", flags::OUTPUT | flags::HIDDEN, "The mesh, with polygon on."),
        int_in("polygon", "po", 0, "0 NURBS, 1 polygons."),
    ];
    attrs.extend(params);
    NodeType { name: name.into(), icon: "chart-line".into(), doc: doc.into(), own_attrs: attrs, create: Some(Arc::new(move || Box::new(SurfNode { kind }))), ..Default::default() }
}

pub(crate) fn f_in(name: &str, short: &str, def: f64, doc: &str) -> AttrDef {
    float_attr_short(name, short, def, doc, None, None)
}

pub(crate) fn v_in(name: &str, short: &str, def: DVec3, doc: &str) -> AttrDef {
    vec3_attr(name, short, def, flags::INPUT | flags::STORABLE | flags::KEYABLE, doc)
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        types::register_builtins();
        crate::plug::data::register_kind("nurbsCurveData", Arc::new(|j| Ok(Arc::new(CurveData(curve_from_json(j))) as Arc<dyn crate::plug::Data>)));
        let shape = types::find("shape").expect("shape");
        let stored = flags::INPUT | flags::STORABLE;
        types::register_type(NodeType {
            name: SHAPE_TYPE.into(),
            base: Some(shape),
            dag: true,
            icon: "chart-line".into(),
            doc: "A NURBS curve: its CVs are components, curve1.cv[0].".into(),
            own_attrs: vec![
                curve_attr("create", "cr", flags::INPUT | flags::HIDDEN, "From history."),
                curve_attr("cached", "cc", stored | flags::HIDDEN, "The curve itself, when no history feeds it."),
                curve_attr("cvTweaks", "cvtw", stored | flags::HIDDEN, "CV offsets on top of history."),
                curve_attr("local", "l", flags::OUTPUT | flags::HIDDEN, "The curve in object space."),
                curve_attr("worldSpace", "ws", flags::OUTPUT | flags::HIDDEN, "The curve in world space."),
                int_out("degree", "d", "The degree."),
                int_out("spans", "sp", "How many spans."),
                int_out("form", "f", "0 open, 1 closed, 2 periodic."),
                float_out("minValue", "min", "First parameter."),
                float_out("maxValue", "max", "Last parameter."),
                store_bool("dispCV", false, "Show CVs when not in component mode."),
                store_bool("dispHull", false, "Show the hull."),
                store_bool("dispEP", false, "Show edit points."),
                AttrDef { flags: stored, ..f_in("lineWidth", "lw", -1.0, "Line width; -1 uses the default.") },
                AttrDef {
                    // The reference's names for a control point's parts.
                    child_names: ["xValue".into(), "yValue".into(), "zValue".into()],
                    child_short: ["xv".into(), "yv".into(), "zv".into()],
                    ..crate::attr_defs::vec3_attr("controlPoints", "cp", DVec3::ZERO, flags::INPUT | flags::HIDDEN | flags::ARRAY, "A CV's place in object space, when something drives it.")
                },
                AttrDef { flags: flags::OUTPUT | flags::HIDDEN | flags::ARRAY, ..float_out("editPoints.xValueEp", "eps.xve", "An edit point's X.") },
                AttrDef { flags: flags::OUTPUT | flags::HIDDEN | flags::ARRAY, ..float_out("editPoints.yValueEp", "eps.yve", "An edit point's Y.") },
                AttrDef { flags: flags::OUTPUT | flags::HIDDEN | flags::ARRAY, ..float_out("editPoints.zValueEp", "eps.zve", "An edit point's Z.") },
                AttrDef { short_name: "ado".into(), ..store_bool("alwaysDrawOnTop", false, "Draw the curve over everything else, as rig controls are. Kept and saved; the views do not act on it yet.") },
            ]
            .into_iter()
            .chain(crate::harold_nodes::curve_render_attrs())
            .collect(),
            create: Some(Arc::new(|| Box::new(CurveShape))),
            ..Default::default()
        })
        .expect(SHAPE_TYPE);
        for t in [
            curve_op_type(
                "makeNurbCircle",
                Kind::Circle,
                "A circle or arc.",
                vec![
                    v_in("normal", "nr", DVec3::Z, "The normal."),
                    v_in("center", "c", DVec3::ZERO, "The centre."),
                    v_in("firstPoint", "fp", DVec3::X, "Where the first section starts."),
                    f_in("radius", "r", 1.0, "The radius."),
                    int_in("degree", "d", 3, "The degree."),
                    int_in("sections", "s", 8, "Sections around."),
                    f_in("sweep", "sw", 360.0, "Degrees covered."),
                    AttrDef { short_name: "ut".into(), ..store_bool("useTolerance", false, "Take the sections from the tolerance.") },
                    f_in("tolerance", "tol", 0.01, "How far from the true circle its points may be, with useTolerance."),
                    AttrDef { short_name: "fc".into(), ..store_bool("fixCenter", true, "The centre stays; off, the circle starts at the first point.") },
                ],
            ),
            curve_op_type(
                "makeNurbsSquare",
                Kind::Square,
                "A square.",
                vec![
                    v_in("normal", "nr", DVec3::Y, "The normal."),
                    v_in("center", "c", DVec3::ZERO, "The centre."),
                    f_in("sideLength1", "sl1", 1.0, "The side."),
                    int_in("degree", "d", 3, "The degree of each side."),
                    f_in("sideLength2", "sl2", 1.0, "The other side."),
                    int_in("spansPerSide", "sps", 1, "Spans along each side."),
                    curve_attr("outputCurve1", "oc1", flags::OUTPUT | flags::HIDDEN, "A side."),
                    curve_attr("outputCurve2", "oc2", flags::OUTPUT | flags::HIDDEN, "A side."),
                    curve_attr("outputCurve3", "oc3", flags::OUTPUT | flags::HIDDEN, "A side."),
                    curve_attr("outputCurve4", "oc4", flags::OUTPUT | flags::HIDDEN, "A side."),
                ],
            ),
            curve_op_type(
                "rebuildCurve",
                Kind::Rebuild,
                "Rebuilt with even spans.",
                vec![int_in("spans", "s", 4, "Spans; 0 keeps the count."), int_in("degree", "d", 3, "The degree."), store_bool("keepControlPoints", false, "Keep the CVs as they are.")],
            ),
            curve_op_type("reverseCurve", Kind::Reverse, "Direction reversed.", vec![]),
            curve_op_type(
                "closeCurve",
                Kind::Close,
                "Closed: made periodic.",
                vec![
                    int_in("preserveShape", "ps", 1, "0 the CVs as they are make the closed curve, 1 the curve keeps its shape and a span closes the gap, 2 the ends blend together."),
                    f_in("blendBias", "bb", 0.5, "With a blend: toward the start (under 0.5) or the end (over 0.5)."),
                    AttrDef { short_name: "bki".into(), ..store_bool("blendKnotInsertion", false, "With a blend: a knot goes in at the parameter near each end first.") },
                    f_in("parameter", "p", 0.1, "Where the blend's knot goes, as a fraction of the end span."),
                ],
            ),
            curve_op_type("smoothCurve", Kind::Smooth, "CVs smoothed.", vec![f_in("smoothness", "s", 0.5, "How far each pass moves."), int_in("iterations", "i", 1, "Passes.")]),
            curve_op_type("fitBspline", Kind::Fit, "A cubic fitted to the CVs.", vec![f_in("tolerance", "tol", 0.1, "How far the fitted curve may stray from the CVs.")]),
            curve_op_type(
                "extendCurve",
                Kind::Extend,
                "Lengthened past its ends.",
                vec![
                    f_in("distance", "d", 1.0, "How far, extending by distance."),
                    int_in("start", "s", 1, "Which end: 0 the end, 1 the start, 2 both."),
                    int_in("extendMethod", "em", 0, "0 by distance, 2 to inputPoint."),
                    int_in("extensionType", "et", 0, "0 linear, 1 circular, 2 extrapolated."),
                    {
                        let mut p = v_in("inputPoint", "ip", DVec3::ZERO, "The point extended to, with extendMethod 2.");
                        p.child_names = ["pointX".into(), "pointY".into(), "pointZ".into()];
                        p.child_short = ["px".into(), "py".into(), "pz".into()];
                        p
                    },
                    AttrDef { short_name: "jn".into(), ..store_bool("join", true, "The extension joins the curve; off, it is a curve of its own.") },
                    AttrDef { short_name: "rmk".into(), ..store_bool("removeMultipleKnots", false, "With join: the knots at the join are removed, the curve refitted smooth through it.") },
                    AttrDef { short_name: "nc".into(), ..store_bool("noChanges", false, "Pass the curve through unchanged.") },
                ],
            ),
            curve_op_type(
                "offsetCurve",
                Kind::Offset,
                "Offset in its plane.",
                vec![
                    f_in("distance", "d", 1.0, "How far."),
                    v_in("normal", "nr", DVec3::ZERO, "The plane's normal; zero works it out."),
                    int_in("connectBreaks", "cb", 2, "Where the offset breaks apart at a corner: 0 left as a straight jump, 1 joined by an arc, 2 joined at a sharp corner."),
                    AttrDef { short_name: "cl".into(), ..store_bool("cutLoop", false, "Cut the loops out of the offset where it crosses itself.") },
                    f_in("cutRadius", "cr", 0.0, "With cutLoop: 0 a sharp corner where a loop is cut, more an arc of this radius."),
                    AttrDef { short_name: "rp".into(), ..store_bool("reparameterize", false, "Give the offset a parameterization of its own, even along its length.") },
                    AttrDef { short_name: "st".into(), ..store_bool("stitch", true, "Join the offset's pieces into one curve.") },
                    int_in("subdivisionDensity", "sd", 5, "How many times a span may be halved while the offset is brought within tolerance."),
                    f_in("tolerance", "tol", 0.01, "How closely the offset keeps the distance."),
                    AttrDef { short_name: "ugn".into(), ..store_bool("useGivenNormal", true, "Use the normal given; off, the curve's own plane.") },
                ],
            ),
            curve_op_type("insertKnotCurve", Kind::InsertKnot, "A knot added.", vec![f_in("parameter", "p", 0.0, "Where."), int_in("numberOfKnots", "nk", 1, "How many.")]),
        ] {
            let name = t.name.clone();
            types::register_type(t).expect(&name);
        }
        types::register_type(NodeType {
            name: "curveInfo".into(),
            icon: "chart-line".into(),
            doc: "Facts about a curve: its arc length.".into(),
            own_attrs: vec![curve_attr("inputCurve", "ic", flags::INPUT | flags::HIDDEN, "The curve."), float_out("arcLength", "al", "Length along the curve.")],
            create: Some(Arc::new(|| Box::new(InfoNode))),
            ..Default::default()
        })
        .expect("curveInfo");
        types::register_type(NodeType {
            name: "pointOnCurveInfo".into(),
            icon: "chart-line".into(),
            doc: "A point on a curve at a parameter.".into(),
            own_attrs: vec![
                curve_attr("inputCurve", "ic", flags::INPUT | flags::HIDDEN, "The curve."),
                f_in("parameter", "pr", 0.0, "The parameter, or a fraction of the length with turnOnPercentage."),
                AttrDef { short_name: "top".into(), ..store_bool("turnOnPercentage", false, "parameter is a fraction 0 to 1 of the curve's parameter range.") },
                vec3_attr("position", "p", DVec3::ZERO, flags::OUTPUT, "The point."),
                vec3_attr("tangent", "t", DVec3::ZERO, flags::OUTPUT, "The tangent."),
                vec3_attr("normalizedTangent", "nt", DVec3::ZERO, flags::OUTPUT, "The unit tangent."),
                vec3_attr("normal", "n", DVec3::ZERO, flags::OUTPUT, "The normal: the curve's bend, square to the tangent."),
                vec3_attr("normalizedNormal", "nn", DVec3::ZERO, flags::OUTPUT, "The unit normal."),
                vec3_attr("curvatureCenter", "cc", DVec3::ZERO, flags::OUTPUT, "The centre of curvature."),
                float_out("curvatureRadius", "cr", "The radius of curvature; 0 where the curve is straight."),
            ],
            create: Some(Arc::new(|| Box::new(PointOnCurveNode))),
            ..Default::default()
        })
        .expect("pointOnCurveInfo");
        types::register_type(NodeType {
            name: "nearestPointOnCurve".into(),
            icon: "chart-line".into(),
            doc: "The point of a curve nearest a position.".into(),
            own_attrs: vec![
                curve_attr("inputCurve", "ic", flags::INPUT | flags::HIDDEN, "The curve."),
                v_in("inPosition", "ip", DVec3::ZERO, "The position."),
                float_out("parameter", "pr", "The parameter of the nearest point."),
                vec3_attr("position", "p", DVec3::ZERO, flags::OUTPUT, "The nearest point."),
            ],
            create: Some(Arc::new(|| Box::new(NearestNode))),
            ..Default::default()
        })
        .expect("nearestPointOnCurve");
        for t in [
            surf_type(
                "extrude",
                SurfKind::Extrude,
                "A profile swept along a path or a direction.",
                vec![
                    f_in("scale", "sc", 1.0, "Scale at the end of the path."),
                    f_in("rotation", "ro", 0.0, "Twist at the end of the path, degrees."),
                    int_in("extrudeType", "et", 2, "0 distance, 1 flat, 2 tube."),
                    int_in("divisions", "dv", 8, "Mesh points per span of the profile."),
                    {
                        let mut d = v_in("direction", "d", DVec3::Y, "Direction of a distance extrude.");
                        d.child_names = ["directionX".into(), "directionY".into(), "directionZ".into()];
                        d.child_short = ["dx".into(), "dy".into(), "dz".into()];
                        d
                    },
                    f_in("length", "l", 1.0, "Length of a distance extrude."),
                    AttrDef { short_name: "fpt".into(), ..store_bool("fixedPath", false, "The surface sits on the path, not where the profile is.") },
                    AttrDef { short_name: "upn".into(), ..store_bool("useProfileNormal", false, "The profile's normal is turned along the path's start.") },
                    int_in("useComponentPivot", "ucp", 0, "The pivot a tube turns and scales the profile about: 0 the path's end nearest the profile, 1 pivot, 2 the middle of the profile's bounding box."),
                    int_in("degreeAlongLength", "dl", 1, "The surface's degree along a distance extrude."),
                    v_in("pivot", "p", DVec3::ZERO, "The profile's pivot, with useComponentPivot 1."),
                    AttrDef { short_name: "rsp".into(), ..store_bool("reverseSurfaceIfPathReversed", false, "A tube along a path taken from its far end is reversed to run with the path.") },
                    AttrDef { short_name: "scs".into(), ..store_bool("subCurveSubSurface", false, "The path's curve range applies to the surface rather than cutting the path first.") },
                ],
            ),
            surf_type(
                "revolve",
                SurfKind::Revolve,
                "A profile turned about an axis.",
                vec![
                    v_in("axis", "ax", DVec3::Y, "The axis."),
                    v_in("pivot", "p", DVec3::ZERO, "A point on the axis."),
                    int_in("sections", "s", 8, "Sections around."),
                    int_in("divisions", "dv", 4, "Mesh points per span of the profile."),
                    f_in("startSweep", "ssw", 0.0, "Start, degrees."),
                    f_in("endSweep", "esw", 360.0, "End, degrees."),
                    int_in("degree", "d", 3, "Degree around."),
                ],
            ),
            surf_type(
                "loft",
                SurfKind::Loft,
                "A surface through curves in order.",
                vec![
                    store_bool("close", false, "Back to the first curve."),
                    int_in("divisions", "dv", 8, "Mesh points per span of the first curve."),
                    store_bool("uniform", false, "Even parameters between the curves, rather than by their distance."),
                    int_in("degree", "d", 3, "Degree across the curves."),
                    int_in("sectionSpans", "ss", 1, "Spans between each pair of curves."),
                    store_bool("autoReverse", true, "Turn curves round to run alike."),
                    store_bool("reverseSurfaceNormals", false, "Flip the normals."),
                    AttrDef { short_name: "r".into(), flags: flags::INPUT | flags::STORABLE | flags::ARRAY, ..bool_attr("reverse", false, "Per curve: turn it round first, when autoReverse is off.") },
                    AttrDef { short_name: "cc".into(), flags: flags::INPUT | flags::STORABLE | flags::ARRAY, ..bool_attr("createCusp", false, "Per curve: a tangent break in the surface at that section.") },
                ],
            ),
            surf_type(
                "planarTrim",
                SurfKind::Planar,
                "A plane trimmed to closed boundary curves.",
                vec![
                    int_in("degree", "d", 3, "The surface's degree, 1 linear or 3 cubic, where it is a trimmed plane."),
                    AttrDef { short_name: "ko".into(), ..store_bool("keepOutside", false, "Keep the regions outside the curves rather than inside.") },
                    f_in("tolerance", "tol", 0.01, "How far from one plane the curves may lie."),
                ],
            ),
            surf_type(
                "sweepMeshCreator",
                SurfKind::Sweep,
                "A tube along a curve.",
                vec![
                    f_in("scaleProfileX", "spx", 0.1, "Radius of the profile."),
                    int_in("profilePolySides", "pps", 8, "Sides of the profile."),
                    int_in("interpolationSteps", "is", 8, "Rings per span of the curve."),
                    f_in("taper", "tp", 1.0, "Radius scale at the end."),
                    f_in("twist", "tw", 0.0, "Twist at the end, degrees."),
                ],
            ),
        ] {
            let name = t.name.clone();
            types::register_type(t).expect(&name);
        }
        register_provider();
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands

type Cmd = CommandDef<Session>;

pub(crate) fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

pub(crate) fn flat_numbers(v: &Value, out: &mut Vec<f64>) {
    match v {
        Value::Array(a) => a.iter().for_each(|x| flat_numbers(x, out)),
        x => out.extend(x.as_f64()),
    }
}

/// The rational curve of `c`'s CVs with these weights, as a plain curve through points sampled along it: eight to a
/// span, so it follows the weighted shape closely.
fn rational_fit(c: &NurbsCurve, weights: &[f64]) -> NurbsCurve {
    let lap = c.distinct_cvs().len();
    let w_of = |i: usize| weights.get(i % lap.max(1)).copied().unwrap_or(1.0);
    // Homogeneous: the weighted points and the weights, on the same knots.
    let num = NurbsCurve::new(c.degree, c.form, c.cvs.iter().enumerate().map(|(i, p)| *p * w_of(i)).collect(), c.knots.clone());
    let den = NurbsCurve::new(c.degree, c.form, (0..c.cvs.len()).map(|i| DVec3::new(w_of(i), 0.0, 0.0)).collect(), c.knots.clone());
    let (lo, hi) = c.domain();
    let n = c.spans().max(1) * 8;
    let at = |u: f64| {
        let w = den.eval(u).x;
        if w.abs() < 1e-12 { num.eval(u) } else { num.eval(u) / w }
    };
    if c.form == Form::Periodic {
        let pts: Vec<DVec3> = (0..n).map(|k| at(lo + (hi - lo) * k as f64 / n as f64)).collect();
        return NurbsCurve::interpolated_periodic(c.degree, &pts);
    }
    let pts: Vec<DVec3> = (0..=n).map(|k| at(lo + (hi - lo) * k as f64 / n as f64)).collect();
    NurbsCurve::through(c.degree, &pts)
}

fn points_flag(a: &Args, name: &str) -> Vec<DVec3> {
    let mut nums = Vec::new();
    if let Some(v) = a.get(name) {
        flat_numbers(v, &mut nums);
    }
    nums.chunks(3).filter(|c| c.len() == 3).map(|c| DVec3::new(c[0], c[1], c[2])).collect()
}

pub(crate) fn create_curve(g: &mut Graph, name: &str, c: NurbsCurve, select: bool) -> CmdResult<(NodeId, NodeId)> {
    let (tr, sh) = crate::dag::create_with_shape(g, SHAPE_TYPE, name, NO_NODE)?;
    set_cached(g, sh, c)?;
    if select {
        g.select(&[tr], SelectMode::Replace);
    }
    Ok((tr, sh))
}

/// A new curve object, selection untouched.
pub fn create_curve_node(g: &mut Graph, name: &str, c: NurbsCurve) -> CmdResult<(NodeId, NodeId)> {
    create_curve(g, name, c, false)
}

pub(crate) fn curve_arg(g: &Graph, name: &str, cmd: &str) -> CmdResult<NodeId> {
    let id = find(g, name, cmd)?;
    match as_curve(g, id) {
        Some(c) => Ok(c),
        None => fail!("{cmd}: {name} is not a curve"),
    }
}

/// Puts a history node above a curve shape; returns the node. The shape's tweaks go into the input so they aren't lost.
pub(crate) fn insert_op(g: &mut Graph, shape: NodeId, ty: &str, set: &[(usize, PlugValue)]) -> CmdResult<NodeId> {
    let current = local_curve(g, shape).unwrap_or_default();
    let t = types::find(ty).ok_or_else(|| aztek_ops::CommandError::new(format!("no node type {ty}")))?;
    let node = g.create_node(&t, &format!("{ty}1"), NO_NODE, true)?;
    let create = plug(shape, CurveShape::CREATE);
    match g.source(&create) {
        Some(src) if g.node(shape).is_some_and(|n| matches!(n.data().value(CurveShape::TWEAK), PlugValue::Data(None))) => {
            g.connect(&src, &plug(node, IN), false)?;
        }
        _ => {
            g.set_value(plug(node, IN), to_value(current))?;
            let _ = g.set_value(plug(shape, CurveShape::TWEAK), PlugValue::Data(None));
        }
    }
    for (attr, v) in set {
        g.set_value(plug(node, *attr), v.clone())?;
    }
    g.connect(&plug(node, OUT), &create, true)?;
    Ok(node)
}

/// circle's flags that name makeNurbCircle attributes of other names.
const CIRCLE_MAP: &[(&str, &str)] = &[("first", "firstPoint")];

pub(crate) fn with_common(c: Cmd) -> Cmd {
    c.flag(f("constructionHistory", "ch", ArgType::Bool, "Keep a history node (on by default)."))
        .flag(f("replaceOriginal", "rpo", ArgType::Bool, "Change the curve itself (on by default)."))
        .flag(f("name", "n", ArgType::String, "Name of a new curve."))
        .flag(f("object", "o", ArgType::Bool, "Make an object (accepted for existing scripts)."))
}

/// Runs a curve op on the curves named or selected, with the reference's -ch and -rpo. Returns [curve, node].
pub(crate) fn run_op(s: &mut Session, a: &Args, cmd: &str, ty: &str, set: &[(usize, PlugValue)]) -> CmdResult {
    let names = objects(s, a);
    if names.is_empty() {
        fail!("{cmd}: name or select a curve");
    }
    let g = &mut s.graph;
    let ch = a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history);
    let rpo = a.get("replaceOriginal").and_then(Value::as_bool).unwrap_or(true);
    let mut out = Vec::new();
    for n in names {
        let shape = curve_arg(g, &n, cmd)?;
        let target = if rpo {
            shape
        } else {
            let c = local_curve(g, shape).unwrap_or_default();
            let tr = crate::dag::transform_of(g, shape);
            let wm = crate::rig::world(g, tr);
            create_curve(g, &a.string("name", ""), c.transformed(&wm), false)?.1
        };
        let node = insert_op(g, target, ty, set)?;
        g.evaluate();
        out.push(json!(g.path(crate::dag::transform_of(g, target))));
        if ch {
            out.push(json!(g.path(node)));
        } else {
            bake_history(g, target)?;
        }
    }
    Ok(Value::Array(out))
}

/// The reference's delete -ch on a curve: the shape keeps what history gives, the nodes go.
pub fn bake_history(g: &mut Graph, shape: NodeId) -> CmdResult<()> {
    let Some(c) = local_curve(g, shape) else { return Ok(()) };
    let create = plug(shape, CurveShape::CREATE);
    let mut src = g.source(&create);
    g.disconnect(&create);
    // A disconnected plug keeps its last value, which would hide the shape's own curve.
    g.set_value(create, PlugValue::Data(None))?;
    set_cached(g, shape, c)?;
    let _ = g.set_value(plug(shape, CurveShape::TWEAK), PlugValue::Data(None));
    while let Some(p) = src {
        let up = g.source(&plug(p.node, IN));
        // Stop at nodes other things hang off.
        if g.destinations(&p).is_empty() {
            g.delete_node(p.node)?;
        }
        src = up;
    }
    Ok(())
}

/// History nodes feeding a curve, nearest first.
pub fn curve_history(g: &Graph, shape: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut src = g.source(&plug(shape, CurveShape::CREATE));
    while let Some(p) = src {
        out.push(p.node);
        src = g.source(&plug(p.node, IN));
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn surf_output(g: &mut Graph, ty: &str, name: &str, node_name: &str, inputs: &[NodeId], set: &[(usize, PlugValue)], polygon: bool) -> CmdResult<(NodeId, NodeId)> {
    let t = types::find(ty).ok_or_else(|| aztek_ops::CommandError::new(format!("no node type {ty}")))?;
    let node = g.create_node(&t, node_name, NO_NODE, true)?;
    for (i, &shape) in inputs.iter().enumerate() {
        g.connect(&plug(shape, CurveShape::WORLD_SPACE), &PlugRef::element(node, S_CURVES as i32, i as i32), false)?;
    }
    for (attr, v) in set {
        g.set_value(plug(node, *attr), v.clone())?;
    }
    g.set_value(plug(node, S_POLYGON), PlugValue::Int(polygon as i64))?;
    let tr = if polygon {
        let (tr, sh) = crate::mesh_nodes::create_mesh_node(g, name)?;
        g.connect(&plug(node, S_OUT), &plug(sh, crate::mesh_nodes::MeshNode::IN_MESH), true)?;
        tr
    } else {
        let (tr, sh) = crate::dag::create_with_shape(g, crate::surfaces::SHAPE_TYPE, name, NO_NODE)?;
        g.connect(&plug(node, S_OUT_SURFACE), &plug(sh, crate::surfaces::SurfaceShape::CREATE), false)?;
        tr
    };
    g.select(&[tr], SelectMode::Replace);
    Ok((tr, node))
}

fn surf_flags(c: Cmd) -> Cmd {
    c.flag(f("polygon", "po", ArgType::Int, "0 NURBS (the default), 1 polygons."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Keep the node (on by default)."))
        .flag(f("range", "rn", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("name", "n", ArgType::String, "Name of the new object."))
}

fn finish_surf(g: &mut Graph, a: &Args, tr: NodeId, node: NodeId) -> CmdResult {
    let ch = a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history);
    if !ch {
        if let Some(m) = components::as_mesh(g, tr) {
            crate::history::delete_history(g, m)?;
        } else if let Some(s) = crate::surfaces::as_surface(g, tr) {
            crate::surfaces::bake_history(g, s)?;
        }
        return Ok(json!([g.path(tr)]));
    }
    Ok(json!([g.path(tr), g.path(node)]))
}

pub(crate) fn v3(a: &Args, name: &str, def: DVec3) -> DVec3 {
    a.vec3(name).map(DVec3::from_array).unwrap_or(def)
}

/// nurbsSquare: the reference's group of four sides, topnurbsSquare1, leftnurbsSquare1, bottomnurbsSquare1 and
/// rightnurbsSquare1, each fed by one of the makeNurbsSquare node's outputCurve1 to 4.
fn nurbs_square(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(r) = crate::history_qe::qe(s, a, "nurbsSquare", &["makeNurbsSquare"], &[]) {
        return r;
    }
    let g = &mut s.graph;
    let ch = a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history);
    let object = a.get("object").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64() != Some(0)));
    let vec_of = |whole: &str, parts: [&str; 3], def: DVec3| {
        let mut v = v3(a, whole, def);
        for (i, p) in parts.iter().enumerate() {
            if let Some(x) = a.get(p).and_then(Value::as_f64) {
                v[i] = x;
            }
        }
        v
    };
    let normal = vec_of("normal", ["normalX", "normalY", "normalZ"], DVec3::Z);
    let center = vec_of("center", ["centerX", "centerY", "centerZ"], DVec3::ZERO);
    let name = a.string("name", "nurbsSquare1");
    g.open_chunk("nurbsSquare");
    let r = (|| -> CmdResult<(Option<(NodeId, Vec<NodeId>)>, NodeId)> {
        let t = types::find("makeNurbsSquare").unwrap();
        let n = g.create_node(&t, "makeNurbsSquare1", NO_NODE, true)?;
        g.set_value(plug(n, P0), PlugValue::Vec3(normal))?;
        g.set_value(plug(n, P0 + 1), PlugValue::Vec3(center))?;
        g.set_value(plug(n, P0 + 2), PlugValue::Float(a.float("sideLength1", 1.0)))?;
        g.set_value(plug(n, P0 + 3), PlugValue::Int(a.int("degree", 3)))?;
        g.set_value(plug(n, P0 + 4), PlugValue::Float(a.float("sideLength2", 1.0)))?;
        g.set_value(plug(n, P0 + 5), PlugValue::Int(a.int("spansPerSide", 1)))?;
        crate::history_qe::apply_common(g, n, a)?;
        if !object {
            return Ok((None, n));
        }
        let group = g.create_node(&types::find("transform").unwrap(), &name, NO_NODE, true)?;
        let stem = g.node(group).map(|x| x.name().to_string()).unwrap_or_default();
        let mut shapes = Vec::new();
        for (k, side) in ["top", "left", "bottom", "right"].iter().enumerate() {
            let (_, sh) = crate::dag::create_with_shape(g, SHAPE_TYPE, &format!("{side}{stem}"), group)?;
            g.connect(&plug(n, P0 + 6 + k), &plug(sh, CurveShape::CREATE), false)?;
            shapes.push(sh);
        }
        Ok((Some((group, shapes)), n))
    })();
    g.close_chunk();
    let (made, n) = r?;
    let Some((group, shapes)) = made else { return Ok(json!([g.path(n)])) };
    g.select(&[group], SelectMode::Replace);
    g.evaluate();
    if !ch {
        for sh in shapes {
            bake_history(g, sh)?;
        }
        return Ok(json!([g.path(group)]));
    }
    Ok(json!([g.path(group), g.path(n)]))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    crate::curve_history::register(c);

    c.register(
        Cmd::new(
            "curve",
            "Create a NURBS curve: curve -d 3 -p 0 0 0 -p 1 0 0 -p 2 1 0 -p 3 1 0. -k gives knots, -per a periodic curve, -ep edit \
             points to pass through, -bezier takes Bezier points. Given a curve: -a appends the points, -r replaces it. Returns its \
             transform.",
            |s, a| {
                let names = a.named();
                let g = &mut s.graph;
                let degree = a.int("degree", 3).max(1) as usize;
                let mut pts = points_flag(a, "point");
                // -pointWeight: CVs with weights, x y z w.
                let mut weights: Vec<f64> = Vec::new();
                if let Some(v) = a.get("pointWeight") {
                    let mut nums = Vec::new();
                    flat_numbers(v, &mut nums);
                    for c in nums.chunks(4).filter(|c| c.len() == 4) {
                        pts.push(DVec3::new(c[0], c[1], c[2]));
                        weights.push(c[3]);
                    }
                    if !weights.is_empty() && weights.len() != pts.len() {
                        fail!("curve: give every CV with -pw, or none");
                    }
                }
                let eps = points_flag(a, "editPoint");
                let mut knots = Vec::new();
                if let Some(k) = a.get("knot") {
                    flat_numbers(k, &mut knots);
                }
                let periodic = a.bool("periodic");
                let existing = match names.first() {
                    Some(n) => Some(curve_arg(g, n, "curve")?),
                    None => None,
                };
                if a.bool("append") {
                    let Some(shape) = existing else { fail!("curve: -a needs a curve") };
                    let old = local_curve(g, shape).unwrap_or_default();
                    let mut all = old.cvs.clone();
                    all.extend(pts);
                    let next = NurbsCurve::open(a.get("degree").map(|_| degree).unwrap_or(old.degree), all);
                    bake_history(g, shape)?;
                    set_cached(g, shape, next)?;
                    return Ok(json!(g.path(crate::dag::transform_of(g, shape))));
                }
                let build = if !eps.is_empty() {
                    NurbsCurve::through_edit_points(degree, &eps)
                } else if pts.len() < 2 {
                    fail!("curve: give at least two points with -p");
                } else if a.bool("bezier") {
                    // Bezier points: a CV with every third point on the curve, knots with multiplicity 3.
                    let spans = (pts.len() - 1) / 3;
                    let mut k = Vec::new();
                    for i in 0..=spans {
                        for _ in 0..3 {
                            k.push(i as f64);
                        }
                    }
                    NurbsCurve::new(3, Form::Open, pts, k)
                } else if periodic {
                    let degree = degree.min(pts.len().saturating_sub(1)).max(1);
                    if knots.len() == pts.len() + degree - 1 {
                        NurbsCurve::new(degree, Form::Periodic, std::mem::take(&mut pts), knots)
                    } else {
                        // The CVs given are a lap, with or without their wrap.
                        let lap = if pts.len() > degree && pts[..degree] == pts[pts.len() - degree..] { pts[..pts.len() - degree].to_vec() } else { pts };
                        NurbsCurve::periodic(degree, lap)
                    }
                } else {
                    let degree = degree.min(pts.len().saturating_sub(1)).max(1);
                    if !knots.is_empty() {
                        if knots.len() != pts.len() + degree - 1 {
                            fail!("curve: {} CVs of degree {degree} take {} knots, not {}", pts.len(), pts.len() + degree - 1, knots.len());
                        }
                        NurbsCurve::new(degree, Form::Open, pts, knots)
                    } else {
                        NurbsCurve::open(degree, pts)
                    }
                };
                // Weighted CVs make a rational curve; it is kept as a plain one following it closely: sampled along its
                // spans and fitted again.
                let build = if weights.iter().any(|w| (w - 1.0).abs() > 1e-12) { rational_fit(&build, &weights) } else { build };
                if let Some(shape) = existing {
                    if !a.bool("replace") {
                        fail!("curve: naming a curve needs -a or -r");
                    }
                    bake_history(g, shape)?;
                    set_cached(g, shape, build)?;
                    return Ok(json!(g.path(crate::dag::transform_of(g, shape))));
                }
                let (tr, _) = create_curve(g, &a.string("name", ""), build, true)?;
                Ok(json!(g.path(tr)))
            },
        )
        .positional("a curve, with -a or -r")
        .flag(f("degree", "d", ArgType::Int, "Degree (3)."))
        .flag(f("point", "p", ArgType::List, "A CV: x y z. Repeat it.").multi(3))
        .flag(f("editPoint", "ep", ArgType::List, "An edit point to pass through: x y z. Repeat it.").multi(3))
        .flag(f("pointWeight", "pw", ArgType::List, "A CV with its weight: x y z w. Repeat it; a weight over 1 pulls the curve to the CV.").multi(4))
        .flag(f("knot", "k", ArgType::List, "A knot. Repeat it; CVs + degree - 1 of them.").multi(1))
        .flag(f("periodic", "per", ArgType::Bool, "A closed, periodic curve."))
        .flag(f("bezier", "bez", ArgType::Bool, "The points are Bezier points."))
        .flag(f("append", "a", ArgType::Bool, "Add the points to the curve named."))
        .flag(f("replace", "r", ArgType::Bool, "Replace the curve named."))
        .flag(f("worldSpace", "ws", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("objectSpace", "os", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("name", "n", ArgType::String, "Name of the transform.")),
    );

    c.register(
        Cmd::new(
            "circle",
            "Create a NURBS circle or arc, with a makeNurbCircle node (-nr normal, -c centre, -r radius, -d degree, -s sections, -sw sweep, \
             -fp first point; -fc off starts it at the first point; -ut sets the sections from -tol). Returns [nurbsCircle1, makeNurbCircle1]. \
             -q and -e read and change the node.",
            |s, a| {
                if let Some(r) = crate::history_qe::qe(s, a, "circle", &["makeNurbCircle"], CIRCLE_MAP) {
                    return r;
                }
                let g = &mut s.graph;
                let ch = a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history);
                let object = a.get("object").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64() != Some(0)));
                let name = a.string("name", "nurbsCircle1");
                let vec_of = |whole: &str, parts: [&str; 3], def: DVec3| {
                    let mut v = v3(a, whole, def);
                    for (i, p) in parts.iter().enumerate() {
                        if let Some(x) = a.get(p).and_then(Value::as_f64) {
                            v[i] = x;
                        }
                    }
                    v
                };
                let normal = vec_of("normal", ["normalX", "normalY", "normalZ"], DVec3::Z);
                let center = vec_of("center", ["centerX", "centerY", "centerZ"], DVec3::ZERO);
                let first = vec_of("first", ["firstPointX", "firstPointY", "firstPointZ"], DVec3::X);
                let on = |k: &str, def: bool| a.get(k).map_or(def, |v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
                g.open_chunk("Circle");
                let r = (|| -> CmdResult<(Option<(NodeId, NodeId)>, NodeId)> {
                    let t = types::find("makeNurbCircle").unwrap();
                    let n = g.create_node(&t, "makeNurbCircle1", NO_NODE, true)?;
                    g.set_value(plug(n, P0), PlugValue::Vec3(normal))?;
                    g.set_value(plug(n, P0 + 1), PlugValue::Vec3(center))?;
                    g.set_value(plug(n, P0 + 2), PlugValue::Vec3(first))?;
                    g.set_value(plug(n, P0 + 3), PlugValue::Float(a.float("radius", 1.0)))?;
                    g.set_value(plug(n, P0 + 4), PlugValue::Int(a.int("degree", 3)))?;
                    g.set_value(plug(n, P0 + 5), PlugValue::Int(a.int("sections", 8)))?;
                    g.set_value(plug(n, P0 + 6), PlugValue::Float(a.float("sweep", std::f64::consts::TAU)))?;
                    g.set_value(plug(n, P0 + 7), PlugValue::Bool(on("useTolerance", false)))?;
                    g.set_value(plug(n, P0 + 8), PlugValue::Float(a.float("tolerance", 0.01)))?;
                    g.set_value(plug(n, P0 + 9), PlugValue::Bool(on("fixCenter", true)))?;
                    crate::history_qe::apply_common(g, n, a)?;
                    if !object {
                        return Ok((None, n));
                    }
                    let (tr, sh) = crate::dag::create_with_shape(g, SHAPE_TYPE, &name, NO_NODE)?;
                    g.connect(&plug(n, OUT), &plug(sh, CurveShape::CREATE), false)?;
                    Ok((Some((tr, sh)), n))
                })();
                g.close_chunk();
                let (made, n) = r?;
                let Some((tr, sh)) = made else { return Ok(json!([g.path(n)])) };
                g.select(&[tr], SelectMode::Replace);
                g.evaluate();
                if !ch {
                    bake_history(g, sh)?;
                    return Ok(json!([g.path(tr)]));
                }
                Ok(json!([g.path(tr), g.path(n)]))
            },
        )
        .positional("the makeNurbCircle node or the circle, with -q and -e")
        .flag(f("normal", "nr", ArgType::Vec3, "The normal of the circle's plane (0 0 1)."))
        .flag(f("normalX", "nrx", ArgType::Float, "The normal's X (0)."))
        .flag(f("normalY", "nry", ArgType::Float, "The normal's Y (0)."))
        .flag(f("normalZ", "nrz", ArgType::Float, "The normal's Z (1)."))
        .flag(f("center", "c", ArgType::Vec3, "The centre (0 0 0)."))
        .flag(f("centerX", "cx", ArgType::Float, "The centre's X (0)."))
        .flag(f("centerY", "cy", ArgType::Float, "The centre's Y (0)."))
        .flag(f("centerZ", "cz", ArgType::Float, "The centre's Z (0)."))
        .flag(f("first", "fp", ArgType::Vec3, "The start point with -fc off; with -fc on, which way the first section starts (1 0 0)."))
        .flag(f("firstPointX", "fpx", ArgType::Float, "The first point's X (1)."))
        .flag(f("firstPointY", "fpy", ArgType::Float, "The first point's Y (0)."))
        .flag(f("firstPointZ", "fpz", ArgType::Float, "The first point's Z (0)."))
        .flag(f("fixCenter", "fc", ArgType::Bool, "Keep the centre (on); off, the circle starts at the first point."))
        .flag(f("radius", "r", ArgType::Float, "Radius (1)."))
        .flag(f("degree", "d", ArgType::Int, "Degree: 1 linear or 3 cubic (3)."))
        .flag(f("sections", "s", ArgType::Int, "Sections, when -ut is off (8)."))
        .flag(f("sweep", "sw", ArgType::Float, "Degrees covered (360)."))
        .flag(f("useTolerance", "ut", ArgType::Bool, "Take the sections from -tol (off)."))
        .flag(f("tolerance", "tol", ArgType::Float, "How far the points may be from the true circle, with -ut (0.01)."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Keep the node (on by default)."))
        .flag(f("object", "o", ArgType::Bool, "Make the circle (on); off, only the node."))
        .flag(f("name", "n", ArgType::String, "Name of the transform."))
        .flag(f("caching", "cch", ArgType::Bool, "The node's caching."))
        .flag(f("nodeState", "nds", ArgType::Int, "The node's state: 0 normal, 2 blocking (it keeps what it gave)."))
        .flag(f("query", "q", ArgType::Bool, "Query mode: read the node's settings."))
        .flag(f("edit", "e", ArgType::Bool, "Edit mode: change the node's settings."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );

    c.register(crate::history_qe::node_flags(crate::history_qe::qe_flags(
        Cmd::new(
            "nurbsSquare",
            "Create a square of four curves, one per side, under a group, with a makeNurbsSquare node: -nr normal, -c centre, -sl1 and \
             -sl2 the sides, -d degree, -sps spans per side. Returns [nurbsSquare1, makeNurbsSquare1]. -q and -e read and change the node.",
            nurbs_square,
        )
        .positional("the makeNurbsSquare node or the square, with -q and -e")
        .flag(f("normal", "nr", ArgType::Vec3, "The normal of the square's plane (0 0 1)."))
        .flag(f("normalX", "nrx", ArgType::Float, "The normal's X (0)."))
        .flag(f("normalY", "nry", ArgType::Float, "The normal's Y (0)."))
        .flag(f("normalZ", "nrz", ArgType::Float, "The normal's Z (1)."))
        .flag(f("center", "c", ArgType::Vec3, "The centre (0 0 0)."))
        .flag(f("centerX", "cx", ArgType::Float, "The centre's X (0)."))
        .flag(f("centerY", "cy", ArgType::Float, "The centre's Y (0)."))
        .flag(f("centerZ", "cz", ArgType::Float, "The centre's Z (0)."))
        .flag(f("sideLength1", "sl1", ArgType::Float, "The length of one side (1)."))
        .flag(f("sideLength2", "sl2", ArgType::Float, "The length of the side next to it, for a rectangle (1)."))
        .flag(f("spansPerSide", "sps", ArgType::Int, "Spans along each side (1)."))
        .flag(f("degree", "d", ArgType::Int, "Degree of each side: 1, 2, 3, 5 or 7 (3)."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Keep the node (on by default)."))
        .flag(f("object", "o", ArgType::Bool, "Make the curves (on); off, only the node."))
        .flag(f("name", "n", ArgType::String, "Name of the group.")),
    )));

    c.register(with_common(
        Cmd::new("rebuildCurve", "Rebuild with even spans: -s spans (0 keeps the count), -d degree, -kcp keeps the CVs. Returns [curve, node].", |s, a| {
            let set = vec![
                (P0, PlugValue::Int(a.int("spans", 4))),
                (P0 + 1, PlugValue::Int(a.int("degree", 3))),
                (P0 + 2, PlugValue::Bool(a.bool("keepControlPoints"))),
            ];
            run_op(s, a, "rebuildCurve", "rebuildCurve", &set)
        })
        .positional("curves (the selection when none)")
        .flag(f("spans", "s", ArgType::Int, "Spans."))
        .flag(f("degree", "d", ArgType::Int, "Degree."))
        .flag(f("keepControlPoints", "kcp", ArgType::Bool, "Keep the CVs."))
        .flag(f("rebuildType", "rt", ArgType::Int, "Accepted for existing scripts (uniform)."))
        .flag(f("endKnots", "end", ArgType::Int, "Accepted for existing scripts."))
        .flag(f("keepRange", "kr", ArgType::Int, "Accepted for existing scripts."))
        .flag(f("keepEndPoints", "kep", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("keepTangents", "kt", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("tolerance", "tol", ArgType::Float, "Accepted for existing scripts.")),
    ));
    c.register(with_common(Cmd::new("reverseCurve", "Reverse the direction. Returns [curve, node].", |s, a| run_op(s, a, "reverseCurve", "reverseCurve", &[])).positional("curves (the selection when none)")));
    c.register(with_common(
        Cmd::new("closeCurve", "Close a curve, making it periodic: -ps 0 on its CVs, 1 keeping its shape (the default), 2 blending the ends at -bb. Returns [curve, node].", |s, a| {
            let set = vec![
                (P0, PlugValue::Int(a.int("preserveShape", 1))),
                (P0 + 1, PlugValue::Float(a.float("blendBias", 0.5))),
                (P0 + 2, PlugValue::Bool(a.bool("blendKnotInsertion"))),
                (P0 + 3, PlugValue::Float(a.float("parameter", 0.1))),
            ];
            run_op(s, a, "closeCurve", "closeCurve", &set)
        })
            .positional("curves (the selection when none)")
            .flag(f("preserveShape", "ps", ArgType::Int, "0 on the CVs as they are, 1 keep the shape (1), 2 blend the ends."))
            .flag(f("blendBias", "bb", ArgType::Float, "Where the ends of a blend meet, from the end (0) to the start (1) (0.5)."))
            .flag(f("blendKnotInsertion", "bki", ArgType::Bool, "With a blend: a knot goes in near each end first (off)."))
            .flag(f("parameter", "p", ArgType::Float, "Where that knot goes, a fraction of the end span (0.1)."))
            .flag(f("curveOnSurface", "cos", ArgType::Bool, "A curve on a surface stays one where it can; other curves close in 3D.")),
    ));
    c.register(with_common(
        Cmd::new("smoothCurve", "Smooth the CVs: -s how far, -i passes. Returns [curve, node].", |s, a| {
            let set = vec![(P0, PlugValue::Float(a.float("smoothness", 0.5))), (P0 + 1, PlugValue::Int(a.int("iterations", 1)))];
            run_op(s, a, "smoothCurve", "smoothCurve", &set)
        })
        .positional("curves (the selection when none)")
        .flag(f("smoothness", "s", ArgType::Float, "How far each pass moves."))
        .flag(f("iterations", "i", ArgType::Int, "Passes.")),
    ));
    c.register(with_common(
        Cmd::new("fitBspline", "A cubic fitted to the curve's CVs, within -tol of them (0.1), with as few spans as that allows. Returns [curve, node].", |s, a| {
            // A new curve, as the reference makes: the input stays.
            let mut b = a.clone();
            b.flags.insert("replaceOriginal".to_string(), json!(false));
            run_op(s, &b, "fitBspline", "fitBspline", &[(P0, PlugValue::Float(a.float("tolerance", 0.1)))])
        })
        .positional("curves (the selection when none)")
        .flag(f("tolerance", "tol", ArgType::Float, "How far the fit may stray from the CVs (0.1).")),
    ));
    c.register(with_common(
        Cmd::new(
            "extendCurve",
            "Extend a curve past its ends: -s 0 the end, 1 the start (the default), 2 both; -em 0 by -d distance, 2 to the point -ip (or \
             -px -py -pz); -et 0 linear, 1 circular, 2 extrapolated; -jn joins the extension to the curve (on), -rmk smooths the join. \
             Returns [curve, node].",
            |s, a| {
                let mut pt = a.vec3("inputPoint").map(DVec3::from_array).unwrap_or(DVec3::ZERO);
                for (i, k) in ["pointX", "pointY", "pointZ"].iter().enumerate() {
                    if a.has(k) {
                        pt[i] = a.float(k, 0.0);
                    }
                }
                let start = match a.get("start") {
                    Some(Value::Bool(b)) => *b as i64,
                    _ => a.int("start", 1),
                };
                let set = vec![
                    (P0, PlugValue::Float(a.float("distance", 1.0))),
                    (P0 + 1, PlugValue::Int(start)),
                    (P0 + 2, PlugValue::Int(a.int("extendMethod", 0))),
                    (P0 + 3, PlugValue::Int(a.int("extensionType", 0))),
                    (P0 + 4, PlugValue::Vec3(pt)),
                    (P0 + 5, PlugValue::Bool(a.get("join").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64() != Some(0))))),
                    (P0 + 6, PlugValue::Bool(a.bool("removeMultipleKnots"))),
                    (P0 + 7, PlugValue::Bool(a.bool("noChanges"))),
                ];
                run_op(s, a, "extendCurve", "extendCurve", &set)
            },
        )
        .positional("curves (the selection when none)")
        .flag(f("distance", "d", ArgType::Float, "How far, with -em 0 (1)."))
        .flag(f("start", "s", ArgType::Int, "0 the end, 1 the start (1), 2 both."))
        .flag(f("extendMethod", "em", ArgType::Int, "0 by distance (0), 2 to a point."))
        .flag(f("extensionType", "et", ArgType::Int, "0 linear (0), 1 circular, 2 extrapolated."))
        .flag(f("inputPoint", "ip", ArgType::Vec3, "The point extended to, with -em 2."))
        .flag(f("pointX", "px", ArgType::Float, "Its X (0)."))
        .flag(f("pointY", "py", ArgType::Float, "Its Y (0)."))
        .flag(f("pointZ", "pz", ArgType::Float, "Its Z (0)."))
        .flag(f("join", "jn", ArgType::Bool, "Join the extension to the curve (on); off, the result is the extension alone."))
        .flag(f("removeMultipleKnots", "rmk", ArgType::Bool, "With -jn: remove the multiple knots at the join, refitting the curve smooth (off)."))
        .flag(f("noChanges", "nc", ArgType::Bool, "The node passes the curve through unchanged (off)."))
        .flag(f("curveOnSurface", "cos", ArgType::Bool, "Make a curve on a surface where the input is one; curves here are 3D, so the result is too."))
        .flag(f("range", "rn", ArgType::Bool, "Use the whole input curve as its range: curves here are always taken whole.")),
    ));
    c.register(with_common(
        Cmd::new(
            "offsetCurve",
            "A copy offset in the curve's plane: -d distance, -nr normal, -cb how corners join, -cl -cr cutting loops, -sd -tol how closely. \
             Returns [curve, node]; -q and -e on the node.",
            |s, a| {
                let on = |k: &str, def: bool| a.get(k).map_or(def, |v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
                let set = vec![
                    (P0, PlugValue::Float(a.float("distance", 1.0))),
                    (P0 + 1, PlugValue::Vec3(v3(a, "normal", DVec3::ZERO))),
                    (P0 + 2, PlugValue::Int(a.int("connectBreaks", 2))),
                    (P0 + 3, PlugValue::Bool(on("cutLoop", false))),
                    (P0 + 4, PlugValue::Float(a.float("cutRadius", 0.0))),
                    (P0 + 5, PlugValue::Bool(on("reparameterize", false))),
                    (P0 + 6, PlugValue::Bool(on("stitch", true))),
                    (P0 + 7, PlugValue::Int(a.int("subdivisionDensity", 5))),
                    (P0 + 8, PlugValue::Float(a.float("tolerance", 0.01))),
                    (P0 + 9, PlugValue::Bool(on("useGivenNormal", true))),
                ];
                let mut b = a.clone();
                b.flags.entry("replaceOriginal".to_string()).or_insert(json!(false));
                if a.get("object").is_some_and(|v| v.as_bool() == Some(false) || v.as_i64() == Some(0)) {
                    // Only the node, fed by the curve.
                    let names = objects(s, a);
                    let g = &mut s.graph;
                    let mut out = Vec::new();
                    for n in names {
                        let shape = curve_arg(g, &n, "offsetCurve")?;
                        let t = types::find("offsetCurve").unwrap();
                        let node = g.create_node(&t, "offsetCurve1", NO_NODE, true)?;
                        g.connect(&plug(shape, CurveShape::LOCAL), &plug(node, IN), false)?;
                        for (i, v) in &set {
                            g.set_value(plug(node, *i), v.clone())?;
                        }
                        crate::history_qe::apply_common(g, node, a)?;
                        out.push(json!(g.path(node)));
                    }
                    return Ok(Value::Array(out));
                }
                run_op(s, &b, "offsetCurve", "offsetCurve", &set)
            },
        )
        .positional("curves (the selection when none); the node or the offset with -q and -e")
        .flag(f("distance", "d", ArgType::Float, "How far (1)."))
        .flag(f("normal", "nr", ArgType::Vec3, "The plane's normal."))
        .flag(f("useGivenNormal", "ugn", ArgType::Bool, "Use the -nr normal (on); off, the curve's own plane."))
        .flag(f("connectBreaks", "cb", ArgType::Int, "How the offset joins where it breaks apart at a corner: 0 off (a straight jump), 1 an arc, 2 a sharp corner (2)."))
        .flag(f("cutLoop", "cl", ArgType::Bool, "Cut out the loops where the offset crosses itself (off)."))
        .flag(f("cutRadius", "cr", ArgType::Float, "With -cl: 0 a sharp corner at each cut, more an arc of this radius (0)."))
        .flag(f("reparameterize", "rp", ArgType::Bool, "Give the offset a parameterization of its own (off)."))
        .flag(f("stitch", "st", ArgType::Bool, "Join the pieces into one curve (on)."))
        .flag(f("subdivisionDensity", "sd", ArgType::Int, "How many times a span may be halved to meet the tolerance (5)."))
        .flag(f("range", "rn", ArgType::Bool, "Offset the whole curve, as always here."))
        .flag(f("tolerance", "tol", ArgType::Float, "How closely the offset keeps the distance (0.01).")),
    ));
    c.register(with_common(
        Cmd::new("insertKnotCurve", "Add knots at -p: curve keeps its shape. Returns [curve, node].", |s, a| {
            let set = vec![(P0, PlugValue::Float(a.float("parameter", 0.0))), (P0 + 1, PlugValue::Int(a.int("numberOfKnots", 1)))];
            run_op(s, a, "insertKnotCurve", "insertKnotCurve", &set)
        })
        .positional("curves (the selection when none)")
        .flag(f("parameter", "p", ArgType::Float, "Where."))
        .flag(f("numberOfKnots", "nk", ArgType::Int, "How many."))
        .flag(f("addKnots", "add", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("insertBetween", "ib", ArgType::Bool, "Accepted for existing scripts.")),
    ));

    c.register(
        Cmd::new("setCurvePoints", "Move a curve's CVs at once, in object space: -p [[x, y, z], ...], from the first CV. A periodic curve's wrapped CVs follow their first lap.", |s, a| {
            let names = objects(s, a);
            let Some(n) = names.first() else { fail!("setCurvePoints: name or select a curve") };
            let g = &mut s.graph;
            let shape = curve_arg(g, n, "setCurvePoints")?;
            let pts = points_flag(a, "points");
            let Some(cur) = local_curve(g, shape) else { fail!("setCurvePoints: {n} has no curve") };
            if pts.len() > cur.cvs.len() {
                fail!("setCurvePoints: {n} has {} CVs, got {}", cur.cvs.len(), pts.len());
            }
            let moves: Vec<(u32, DVec3)> = pts.iter().enumerate().map(|(i, p)| (i as u32, *p)).collect();
            move_cvs(g, shape, &moves);
            Ok(json!(pts.len()))
        })
        .positional("a curve (the selection when none)")
        .flag(f("points", "p", ArgType::List, "[[x, y, z], ...].")),
    );

    c.register(
        Cmd::new(
            "arclen",
            "The length of a curve along its arc, world space. -ch on makes a curveInfo node computing it, connected to the curve, and returns the node, \
             whose arcLength follows the curve. -q and -e on the node: -cch, -nds.",
            |s, a| {
                if let Some(r) = crate::history_qe::qe(s, a, "arclen", &["curveInfo"], &[]) {
                    return r;
                }
                let names = objects(s, a);
                let Some(n) = names.first() else { fail!("arclen: name or select a curve") };
                let g = &mut s.graph;
                let shape = curve_arg(g, n, "arclen")?;
                if a.get("constructionHistory").is_some_and(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0)) {
                    let t = types::find("curveInfo").expect("curveInfo");
                    let node = g.create_node(&t, "curveInfo1", NO_NODE, true)?;
                    g.connect(&plug(shape, CurveShape::WORLD_SPACE), &plug(node, 0), false)?;
                    crate::history_qe::apply_common(g, node, a)?;
                    g.evaluate();
                    return Ok(json!(g.path(node)));
                }
                // In the current unit: the result isn't tagged, as a query or the node's name may come back instead.
                let len = world_curve(g, shape).map(|c| c.length()).unwrap_or(0.0);
                Ok(json!(if aztek_ops::units::user_call() { aztek_ops::units::to_user(aztek_ops::units::Unit::Linear, len) } else { len }))
            },
        )
        .positional("a curve (the selection when none); the curveInfo node with -q and -e")
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Make a curveInfo node computing the length and return it (off)."))
        .flag(f("caching", "cch", ArgType::Bool, "With -ch: the node's caching."))
        .flag(f("nodeState", "nds", ArgType::Int, "With -ch: the node's state; 2 blocking keeps the length it gave."))
        .flag(f("query", "q", ArgType::Bool, "Query mode, on the node."))
        .flag(f("edit", "e", ArgType::Bool, "Edit mode, on the node."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );

    c.register(crate::history_qe::node_flags(crate::history_qe::qe_flags(
        Cmd::new(
            "pointOnCurve",
            "Facts at a parameter of a curve, world space: -pr parameter (-top a fraction of its parameter range), asking -p position, -t \
             tangent, -nt unit tangent, -no normal, -nn unit normal, -cc centre and -cr radius of curvature (position when none). -ch on \
             makes a pointOnCurveInfo node, connected to the curve, and returns it; -q and -e on the node.",
            point_on_curve,
        )
        .undoable(false)
        .positional("a curve (the selection when none); the pointOnCurveInfo node with -q and -e")
        .flag(f("parameter", "pr", ArgType::Float, "The parameter (0)."))
        .flag(f("turnOnPercentage", "top", ArgType::Bool, "-pr is a fraction 0 to 1 of the parameter range (off)."))
        .flag(f("position", "p", ArgType::Bool, "Ask for the point."))
        .flag(f("tangent", "t", ArgType::Bool, "Ask for the tangent."))
        .flag(f("normalizedTangent", "nt", ArgType::Bool, "Ask for the unit tangent."))
        .flag(f("normal", "no", ArgType::Bool, "Ask for the normal: the bend of the curve, square to the tangent."))
        .flag(f("normalizedNormal", "nn", ArgType::Bool, "Ask for the unit normal."))
        .flag(f("curvatureCenter", "cc", ArgType::Bool, "Ask for the centre of curvature."))
        .flag(f("curvatureRadius", "cr", ArgType::Bool, "Ask for the radius of curvature."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Make a pointOnCurveInfo node, connected to the curve, and return it (off).")),
    )));

    c.register(surf_flags(
        Cmd::new(
            "extrude",
            "Sweep a profile curve along a path curve (or -d -l a direction and length): extrude profile path -et 2 -sc 1 -ro 0. Returns \
             [extrudedSurface1, extrude1].",
            |s, a| {
                let names = objects(s, a);
                let g = &mut s.graph;
                if names.is_empty() {
                    fail!("extrude: name or select a profile and a path");
                }
                let mut inputs: Vec<NodeId> = names.iter().map(|n| curve_arg(g, n, "extrude")).collect::<CmdResult<_>>()?;
                if a.bool("rebuild") {
                    // The inputs rebuilt first: uniform knots, keeping each curve's span count and degree.
                    let mut rebuilt = Vec::new();
                    for &sh in &inputs {
                        let c = world_curve(g, sh).unwrap_or_default();
                        let (_, copy) = create_curve(g, "rebuiltCurve1", c.rebuilt(c.spans().max(1), c.degree), false)?;
                        g.set_value(plug(crate::dag::transform_of(g, copy), DagNode::VISIBILITY), PlugValue::Bool(false))?;
                        rebuilt.push(copy);
                    }
                    inputs = rebuilt;
                }
                let mut dir = v3(a, "direction", DVec3::Y);
                for (i, k) in ["directionX", "directionY", "directionZ"].iter().enumerate() {
                    if a.has(k) {
                        dir[i] = a.float(k, 0.0);
                    }
                }
                let pivot = match a.vec3("pivot") {
                    Some(p) => DVec3::from_array(p),
                    None => world_curve(g, inputs[0]).map(|c| {
                        let lap = c.distinct_cvs();
                        lap.iter().copied().sum::<DVec3>() / lap.len().max(1) as f64
                    }).unwrap_or(DVec3::ZERO),
                };
                let set = vec![
                    (S_P, PlugValue::Float(a.float("scale", 1.0))),
                    (S_P + 1, PlugValue::Float(a.float("rotation", 0.0))),
                    (S_P + 2, PlugValue::Int(a.int("extrudeType", 2))),
                    (S_P + 3, PlugValue::Int(a.int("divisions", 8))),
                    (S_P + 4, PlugValue::Vec3(dir)),
                    (S_P + 5, PlugValue::Float(a.float("length", 1.0))),
                    (S_P + 6, PlugValue::Bool(a.bool("fixedPath"))),
                    (S_P + 7, PlugValue::Bool(a.bool("useProfileNormal"))),
                    (S_P + 8, PlugValue::Int(a.int("useComponentPivot", 0))),
                    (S_P + 9, PlugValue::Int(a.int("degreeAlongLength", 1))),
                    (S_P + 10, PlugValue::Vec3(pivot)),
                    (S_P + 11, PlugValue::Bool(a.bool("reverseSurfaceIfPathReversed"))),
                    (S_P + 12, PlugValue::Bool(a.bool("subCurveSubSurface"))),
                ];
                if a.get("object").is_some_and(|v| v.as_bool() == Some(false) || v.as_i64() == Some(0)) {
                    // Only the node.
                    let t = types::find("extrude").expect("extrude");
                    let node = g.create_node(&t, "extrude1", NO_NODE, true)?;
                    for (i, &shape) in inputs.iter().enumerate() {
                        g.connect(&plug(shape, CurveShape::WORLD_SPACE), &PlugRef::element(node, S_CURVES as i32, i as i32), false)?;
                    }
                    for (attr, v) in &set {
                        g.set_value(plug(node, *attr), v.clone())?;
                    }
                    g.evaluate();
                    return Ok(json!([g.path(node)]));
                }
                let (tr, node) = surf_output(g, "extrude", &a.string("name", "extrudedSurface1"), "extrude1", &inputs, &set, a.int("polygon", 0) == 1)?;
                finish_surf(g, a, tr, node)
            },
        )
        .positional("the profile, then the path (the selection when none)")
        .flag(f("extrudeType", "et", ArgType::Int, "0 distance, 1 flat, 2 tube (2)."))
        .flag(f("scale", "sc", ArgType::Float, "Scale at the path's end."))
        .flag(f("rotation", "ro", ArgType::Float, "Twist at the path's end, degrees."))
        .flag(f("direction", "d", ArgType::Vec3, "Direction, with no path."))
        .flag(f("length", "l", ArgType::Float, "Length, with no path."))
        .flag(f("divisions", "dv", ArgType::Int, "Mesh points per span of the profile."))
        .flag(f("fixedPath", "fpt", ArgType::Bool, "The surface sits on the path, not where the profile is."))
        .flag(f("useComponentPivot", "ucp", ArgType::Int, "A tube's pivot: 0 the path's end nearest the profile (0), 1 -p, 2 the middle of the profile's bounding box."))
        .flag(f("useProfileNormal", "upn", ArgType::Bool, "Turn the profile's plane square to the path's start (off)."))
        .flag(f("reverseSurfaceIfPathReversed", "rsp", ArgType::Bool, "A tube along a path taken from its far end is reversed to run with the path (off)."))
        .flag(f("degreeAlongLength", "dl", ArgType::Int, "The surface's degree along a distance extrude (1)."))
        .flag(f("directionX", "dx", ArgType::Float, "The direction's X (0)."))
        .flag(f("directionY", "dy", ArgType::Float, "The direction's Y (1)."))
        .flag(f("directionZ", "dz", ArgType::Float, "The direction's Z (0)."))
        .flag(f("pivot", "p", ArgType::Vec3, "The profile's pivot, with -ucp 1 (the middle of its CVs)."))
        .flag(f("subCurveSubSurface", "scs", ArgType::Bool, "The path's curve range applies to the surface rather than cutting the path first (off)."))
        .flag(f("rebuild", "rb", ArgType::Bool, "Rebuild the input curves first, with uniform knots (off)."))
        .flag(f("mergeItems", "mi", ArgType::Bool, "Merge component results where it can: the result here is whole objects."))
        .flag(f("object", "o", ArgType::Bool, "Make the surface (on); off, only the node.")),
    ));

    c.register(surf_flags(
        Cmd::new(
            "revolve",
            "Turn a curve about an axis: revolve curve1 -ax 0 1 0 -p 0 0 0 -s 8 -ssw 0 -esw 360. Returns [revolvedSurface1, revolve1].",
            |s, a| {
                let names = objects(s, a);
                let g = &mut s.graph;
                let Some(n) = names.first() else { fail!("revolve: name or select a curve") };
                let shape = curve_arg(g, n, "revolve")?;
                let set = vec![
                    (S_P, PlugValue::Vec3(v3(a, "axis", DVec3::Y))),
                    (S_P + 1, PlugValue::Vec3(v3(a, "pivot", DVec3::ZERO))),
                    (S_P + 2, PlugValue::Int(a.int("sections", 8))),
                    (S_P + 3, PlugValue::Int(a.int("divisions", 4))),
                    (S_P + 4, PlugValue::Float(a.float("startSweep", 0.0))),
                    (S_P + 5, PlugValue::Float(a.float("endSweep", std::f64::consts::TAU))),
                    (S_P + 6, PlugValue::Int(a.int("degree", 3))),
                ];
                let (tr, node) = surf_output(g, "revolve", &a.string("name", "revolvedSurface1"), "revolve1", &[shape], &set, a.int("polygon", 0) == 1)?;
                finish_surf(g, a, tr, node)
            },
        )
        .positional("a curve (the selection when none)")
        .flag(f("axis", "ax", ArgType::Vec3, "The axis."))
        .flag(f("pivot", "p", ArgType::Vec3, "A point on the axis."))
        .flag(f("sections", "s", ArgType::Int, "Sections (8)."))
        .flag(f("divisions", "dv", ArgType::Int, "Mesh points per span of the curve."))
        .flag(f("startSweep", "ssw", ArgType::Float, "Start, degrees."))
        .flag(f("endSweep", "esw", ArgType::Float, "End, degrees."))
        .flag(f("degree", "d", ArgType::Int, "Degree around (3)."))
        .flag(f("useTolerance", "ut", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("tolerance", "tol", ArgType::Float, "Accepted for existing scripts.")),
    ));

    c.register(surf_flags(
        Cmd::new(
            "loft",
            "A surface through curves in order: loft c1 c2 c3 -c 0. Returns [loftedSurface1, loft1].",
            |s, a| {
                let names = objects(s, a);
                let g = &mut s.graph;
                if names.len() < 2 {
                    fail!("loft: name or select two curves or more");
                }
                let inputs: Vec<NodeId> = names.iter().map(|n| curve_arg(g, n, "loft")).collect::<CmdResult<_>>()?;
                let bools = |flag: &str| -> Vec<bool> {
                    match a.get(flag) {
                        Some(Value::Array(v)) => v.iter().map(|x| x.as_bool().unwrap_or(x.as_i64().unwrap_or(0) != 0)).collect(),
                        Some(x) => vec![x.as_bool().unwrap_or(x.as_i64().unwrap_or(0) != 0)],
                        None => Vec::new(),
                    }
                };
                let (rev, cusp) = (bools("reverse"), bools("createCusp"));
                let set = vec![
                    (S_P, PlugValue::Bool(a.bool("close"))),
                    (S_P + 1, PlugValue::Int(a.int("divisions", 8))),
                    (S_P + 2, PlugValue::Bool(a.bool("uniform"))),
                    (S_P + 3, PlugValue::Int(a.int("degree", 3))),
                    (S_P + 4, PlugValue::Int(a.int("sectionSpans", 1))),
                    (S_P + 5, PlugValue::Bool(a.get("autoReverse").and_then(Value::as_bool).unwrap_or(true))),
                    (S_P + 6, PlugValue::Bool(a.bool("reverseSurfaceNormals"))),
                ];
                let (tr, node) = surf_output(g, "loft", &a.string("name", "loftedSurface1"), "loft1", &inputs, &set, a.int("polygon", 0) == 1)?;
                for (k, (r, c)) in rev.iter().chain(std::iter::repeat(&false)).zip(cusp.iter().chain(std::iter::repeat(&false))).take(inputs.len()).enumerate() {
                    if *r {
                        g.set_value(PlugRef::element(node, (S_P + 7) as i32, k as i32), PlugValue::Bool(true))?;
                    }
                    if *c {
                        g.set_value(PlugRef::element(node, (S_P + 8) as i32, k as i32), PlugValue::Bool(true))?;
                    }
                }
                // -rebuild: each curve goes through a rebuildCurve node first.
                if a.bool("rebuild") {
                    let t = types::find("rebuildCurve").expect("rebuildCurve");
                    for (k, &shape) in inputs.iter().enumerate() {
                        let rb = g.create_node(&t, "rebuildCurve1", NO_NODE, true)?;
                        g.connect(&plug(shape, CurveShape::WORLD_SPACE), &plug(rb, IN), false)?;
                        g.connect(&plug(rb, OUT), &PlugRef::element(node, S_CURVES as i32, k as i32), true)?;
                    }
                }
                // -object off: only the node, its output going nowhere.
                if a.get("object").and_then(Value::as_bool) == Some(false) {
                    let path = g.path(node);
                    g.delete_node(tr)?;
                    g.evaluate();
                    return Ok(json!([path]));
                }
                finish_surf(g, a, tr, node)
            },
        )
        .positional("curves in order (the selection when none)")
        .flag(f("close", "c", ArgType::Bool, "Back to the first curve."))
        .flag(f("divisions", "dv", ArgType::Int, "Mesh points per span of the first curve."))
        .flag(f("uniform", "u", ArgType::Bool, "Even parameters between the curves, rather than by their distance."))
        .flag(f("autoReverse", "ar", ArgType::Bool, "Turn curves round to run alike (on)."))
        .flag(f("degree", "d", ArgType::Int, "Degree across the curves, 1 to 3 (3)."))
        .flag(f("sectionSpans", "ss", ArgType::Int, "Spans between each pair of curves (1)."))
        .flag(f("reverseSurfaceNormals", "rsn", ArgType::Bool, "Flip the normals."))
        .flag(f("reverse", "r", ArgType::Any, "Per curve, in order: turn it round first, when -ar is off. Repeat it.").multi(1))
        .flag(f("createCusp", "cc", ArgType::Any, "Per curve, in order: a tangent break in the surface at that section. Repeat it.").multi(1))
        .flag(f("rebuild", "rb", ArgType::Bool, "Rebuild the curves first, each through a rebuildCurve node."))
        .flag(f("object", "o", ArgType::Bool, "Make the surface (on); off, only the node.")),
    ));

    c.register(surf_flags(
        Cmd::new(
            "planar",
            "A surface filling a closed curve: a rectangle makes a plain patch, anything else a plane trimmed to it; -po 1 a polygon \
             face. Returns [planarTrimmedSurface1, planarTrim1].",
            |s, a| {
                if let Some(r) = crate::history_qe::qe(s, a, "planar", &["planarTrim"], &[]) {
                    return r;
                }
                let names = objects(s, a);
                let g = &mut s.graph;
                if names.is_empty() {
                    fail!("planar: name or select closed boundary curves");
                }
                // Each curve named, and the curves under a group named (a square's four sides).
                let mut shapes = Vec::new();
                for n in &names {
                    let id = find(g, n, "planar")?;
                    if let Some(c) = as_curve(g, id) {
                        shapes.push(c);
                        continue;
                    }
                    let under: Vec<NodeId> = g.descendants(id).into_iter().filter(|&d| g.node(d).is_some_and(|x| x.ty().name == SHAPE_TYPE) && !g.node(d).is_some_and(|x| x.bool(DagNode::INTERMEDIATE_OBJECT))).collect();
                    if under.is_empty() {
                        fail!("planar: {n} is not a curve");
                    }
                    shapes.extend(under);
                }
                let polygon = a.int("polygon", 0) == 1;
                let set = vec![
                    (S_P, PlugValue::Int(a.int("degree", 3))),
                    (S_P + 1, PlugValue::Bool(a.bool("keepOutside"))),
                    (S_P + 2, PlugValue::Float(a.float("tolerance", 0.01))),
                ];
                if a.get("object").is_some_and(|v| v.as_bool() == Some(false) || v.as_i64() == Some(0)) {
                    let t = types::find("planarTrim").expect("planarTrim");
                    let node = g.create_node(&t, "planarTrim1", NO_NODE, true)?;
                    for (i, &shape) in shapes.iter().enumerate() {
                        g.connect(&plug(shape, CurveShape::WORLD_SPACE), &PlugRef::element(node, S_CURVES as i32, i as i32), false)?;
                    }
                    for (attr, v) in &set {
                        g.set_value(plug(node, *attr), v.clone())?;
                    }
                    crate::history_qe::apply_common(g, node, a)?;
                    return Ok(json!([g.path(node)]));
                }
                let (tr, node) = surf_output(g, "planarTrim", &a.string("name", "planarTrimmedSurface1"), "planarTrim1", &shapes, &set, polygon)?;
                crate::history_qe::apply_common(g, node, a)?;
                finish_surf(g, a, tr, node)
            },
        )
        .positional("closed boundary curves, or groups of them (the selection when none); with -q and -e the planarTrim node or the surface")
        .flag(f("degree", "d", ArgType::Int, "The surface's degree, 1 linear or 3 cubic (3), where it is a trimmed plane."))
        .flag(f("keepOutside", "ko", ArgType::Bool, "Keep the regions outside the curves, making a hole (off)."))
        .flag(f("tolerance", "tol", ArgType::Float, "How far from one plane the curves' CVs may lie (0.01)."))
        .flag(f("object", "o", ArgType::Bool, "Make the surface (on); off, only the node.")),
    ));
    if let Some(d) = c.find("planar").cloned() {
        c.register(crate::history_qe::node_flags(crate::history_qe::qe_flags(d)));
    }

    c.register(surf_flags(
        Cmd::new(
            "sweepMeshFromCurve",
            "A tube along a curve with a sweepMeshCreator node: -scaleProfileX radius, -profilePolySides, -interpolationSteps, -taper, \
             -twist. Returns [sweep1, sweepMeshCreator1].",
            |s, a| {
                let names = objects(s, a);
                let g = &mut s.graph;
                let Some(n) = names.first() else { fail!("sweepMeshFromCurve: name or select a curve") };
                let shape = curve_arg(g, n, "sweepMeshFromCurve")?;
                let set = vec![
                    (S_P, PlugValue::Float(a.float("scaleProfileX", 0.1))),
                    (S_P + 1, PlugValue::Int(a.int("profilePolySides", 8))),
                    (S_P + 2, PlugValue::Int(a.int("interpolationSteps", 8))),
                    (S_P + 3, PlugValue::Float(a.float("taper", 1.0))),
                    (S_P + 4, PlugValue::Float(a.float("twist", 0.0))),
                ];
                let (tr, node) = surf_output(g, "sweepMeshCreator", &a.string("name", "sweep1"), "sweepMeshCreator1", &[shape], &set, true)?;
                finish_surf(g, a, tr, node)
            },
        )
        .positional("a curve (the selection when none)")
        .flag(f("scaleProfileX", "spx", ArgType::Float, "Radius of the profile."))
        .flag(f("profilePolySides", "pps", ArgType::Int, "Sides of the profile."))
        .flag(f("interpolationSteps", "is", ArgType::Int, "Rings per span."))
        .flag(f("taper", "tp", ArgType::Float, "Radius scale at the end."))
        .flag(f("twist", "tw", ArgType::Float, "Twist at the end, degrees.")),
    ));
}
