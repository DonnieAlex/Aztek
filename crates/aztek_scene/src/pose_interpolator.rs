//! Pose interpolators, as the reference's poseInterpolator node (a shape) and command: driver joints' rotations (and
//! translations) compared with stored poses, giving each pose a weight in output[], for corrective shapes. The node's
//! attributes and defaults are the 2027 release's (PARITY.md says where they were read); its arithmetic is not
//! published, so the solver here is Aztek's own (PARITY.md says what it does).

use std::sync::Arc;

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use aztek_ops::units::Unit;
use glam::{DMat3, DMat4, DQuat, DVec3};
use serde_json::{json, Value};

use crate::array_data::{self, ArrayData};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::Shape;

pub const TYPE: &str = "poseInterpolator";

const STORE: u32 = flags::INPUT | flags::STORABLE;
const KEY: u32 = STORE | flags::KEYABLE;
const POSE_TYPES: [&str; 3] = ["Swing and Twist", "Swing Only", "Twist Only"];

fn a(name: &str, ty: PlugType, d: PlugValue, fl: u32, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: name.into(), ty, default_value: d, flags: fl, doc: doc.into(), ..Default::default() }
}
fn b(name: &str, d: bool, fl: u32, doc: &str) -> AttrDef {
    a(name, PlugType::Bool, PlugValue::Bool(d), fl, doc)
}
fn f(name: &str, d: f64, lo: Option<f64>, hi: Option<f64>, fl: u32, doc: &str) -> AttrDef {
    AttrDef { min_value: lo, max_value: hi, ..a(name, PlugType::Float, PlugValue::Float(d), fl, doc) }
}
fn e(name: &str, d: i64, names: &[&str], fl: u32, doc: &str) -> AttrDef {
    AttrDef { enum_names: names.iter().map(|x| x.to_string()).collect(), ..a(name, PlugType::Enum, PlugValue::Int(d), fl, doc) }
}
fn doubles(name: &str, doc: &str) -> AttrDef {
    array_data::register();
    a(name, PlugType::Data, ArrayData::empty("doubleArray").value(), STORE | flags::HIDDEN, doc)
}
fn angles(name: &str, doc: &str) -> AttrDef {
    let mut x = a(name, PlugType::Vec3, PlugValue::Vec3(DVec3::ZERO), STORE | flags::HIDDEN, doc);
    x.unit = Unit::Angle;
    for (k, c) in ["X", "Y", "Z"].into_iter().enumerate() {
        x.child_names[k] = format!("{name}{c}");
        x.child_short[k] = format!("{name}{c}");
    }
    x
}
/// A child of a multi compound: pose[3].poseName.
fn kid(group: &str, mut x: AttrDef) -> AttrDef {
    x.name = format!("{group}.{}", x.name);
    x.short_name = format!("{group}.{}", x.short_name);
    x.flags |= flags::ARRAY;
    x
}

fn attrs() -> Vec<AttrDef> {
    vec![
        b("allowNegativeWeights", true, KEY, "Weights may go below 0; off, they are held at 0."),
        b("displayOutput", false, KEY, "Shows the weights in the views (not drawn yet)."),
        f("displaySpacing", 1.0, None, None, KEY, "Spacing of the shown weights."),
        b("enableRotation", true, KEY, "The drivers' rotations count."),
        b("enableTranslation", false, KEY, "The drivers' translations count."),
        e("interpolation", 0, &["Linear", "Gaussian"], KEY, "How weight falls away from a pose."),
        f("outputSmoothing", 0.0, Some(0.0), Some(1.0), KEY, "Eases each weight towards a smooth curve."),
        f("regularization", 0.0, Some(0.0), None, KEY, "Smooths the solve when poses crowd; at 0 each pose is met exactly."),
        kid("driver", b("driverEulerTwist", false, STORE, "Twist read from the rotate channel about the twist axis.")),
        kid("driver", a("driverMatrix", PlugType::Matrix, PlugValue::Matrix(DMat4::IDENTITY), STORE, "The driver's local matrix.")),
        kid("driver", angles("driverOrient", "The driver's joint orient, as it was.")),
        kid("driver", angles("driverRotateAxis", "The driver's rotate axis, as it was.")),
        kid("driver", e("driverRotateOrder", 0, &["xyz", "yzx", "zxy", "xzy", "yxz", "zyx"], STORE, "The driver's rotate order.")),
        kid("driver", e("driverTwistAxis", 0, &["X Axis", "Y Axis", "Z Axis"], STORE, "The driver's twist axis.")),
        kid("pose", b("isEnabled", true, STORE, "The pose counts.")),
        kid("pose", b("isIndependent", false, STORE, "Weighted by its own falloffs only, outside the solve.")),
        kid("pose", f("poseFalloff", 1.0, Some(0.0), Some(1.0), STORE, "Width of a Gaussian pose, in radians of the drivers' change.")),
        kid("pose", a("poseName", PlugType::String, PlugValue::String(String::new()), STORE, "The pose's name.")),
        kid("pose", doubles("poseRotation", "The drivers' rotations at the pose: a quaternion (x, y, z, w) for each driver.")),
        kid("pose", f("poseRotationFalloff", 180.0, Some(0.001), Some(180.0), STORE, "An independent pose's reach, in degrees.")),
        kid("pose", doubles("poseTranslation", "The drivers' translations at the pose: three numbers for each driver.")),
        kid("pose", f("poseTranslationFalloff", 0.0, Some(0.0), None, STORE, "An independent pose's reach in distance; 0 none.")),
        kid("pose", e("poseType", 0, &POSE_TYPES, STORE, "What of the drivers' rotation is compared.")),
        AttrDef { flags: flags::OUTPUT | flags::ARRAY, ..a("output", PlugType::Float, PlugValue::Float(0.0), 0, "Each pose's weight, by the pose's index.") },
    ]
}

// ---------------------------------------------------------------------------------------------------------------------
// The solve

#[derive(Clone)]
struct Driver {
    rot: DQuat,
    pos: DVec3,
    axis: DVec3,
}

#[derive(Clone)]
struct Pose {
    index: i32,
    independent: bool,
    kind: i64,
    falloff: f64,
    rot_falloff: f64,
    tr_falloff: f64,
    rots: Vec<DQuat>,
    pos: Vec<DVec3>,
}

/// Angle between two directions.
fn angle(a: DVec3, b: DVec3) -> f64 {
    a.normalize_or_zero().dot(b.normalize_or_zero()).clamp(-1.0, 1.0).acos()
}

/// The twist about an axis of the rotation taking b to a.
fn twist(a: DQuat, b: DQuat, axis: DVec3) -> f64 {
    let r = b.inverse() * a;
    let v = DVec3::new(r.x, r.y, r.z);
    let along = v.dot(axis);
    let t = 2.0 * along.abs().atan2(r.w.abs());
    t.min(std::f64::consts::TAU - t)
}

/// Squared rotation distance between the drivers now and a pose, by the pose's type.
fn rot_d2(now: &[(DQuat, DVec3)], rots: &[DQuat], kind: i64) -> f64 {
    let mut d2 = 0.0;
    for (k, (q, axis)) in now.iter().enumerate() {
        let p = rots.get(k).copied().unwrap_or(DQuat::IDENTITY);
        let sw = angle(*q * *axis, p * *axis);
        let tw = twist(*q, p, *axis);
        d2 += match kind {
            1 => sw * sw,
            2 => tw * tw,
            _ => sw * sw + tw * tw,
        };
    }
    d2
}

fn tr_d2(now: &[DVec3], pos: &[DVec3]) -> f64 {
    now.iter().enumerate().map(|(k, p)| (*p - pos.get(k).copied().unwrap_or(DVec3::ZERO)).length_squared()).sum()
}

struct Settings {
    rotation: bool,
    translation: bool,
    gaussian: bool,
    negative: bool,
    smoothing: f64,
    lambda: f64,
}

fn dist(st: &Settings, rots: &[(DQuat, DVec3)], pos: &[DVec3], p: &Pose) -> f64 {
    let mut d2 = 0.0;
    if st.rotation {
        d2 += rot_d2(rots, &p.rots, p.kind);
    }
    if st.translation {
        d2 += tr_d2(pos, &p.pos);
    }
    d2.sqrt()
}

fn kernel(st: &Settings, d: f64, p: &Pose) -> f64 {
    if st.gaussian {
        let s = p.falloff.max(1e-3);
        (-(d / s) * (d / s)).exp()
    } else {
        d
    }
}

/// Solves A x = b for each column of b (n x n), with partial pivoting. None when A is singular.
fn solve(mut a: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut inv: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect()).collect();
    for c in 0..n {
        let piv = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if a[piv][c].abs() < 1e-12 {
            return None;
        }
        a.swap(c, piv);
        inv.swap(c, piv);
        let d = a[c][c];
        for j in 0..n {
            a[c][j] /= d;
            inv[c][j] /= d;
        }
        for i in 0..n {
            if i != c {
                let k = a[i][c];
                if k != 0.0 {
                    for j in 0..n {
                        a[i][j] -= k * a[c][j];
                        inv[i][j] -= k * inv[c][j];
                    }
                }
            }
        }
    }
    Some(inv)
}

fn smooth(w: f64, s: f64) -> f64 {
    let c = w.clamp(0.0, 1.0);
    w + s * (c * c * (3.0 - 2.0 * c) - w)
}

/// The weight of each pose, by index.
fn weights(st: &Settings, drivers: &[Driver], poses: &[Pose]) -> Vec<(i32, f64)> {
    let rots: Vec<(DQuat, DVec3)> = drivers.iter().map(|d| (d.rot, d.axis)).collect();
    let pos: Vec<DVec3> = drivers.iter().map(|d| d.pos).collect();
    let mut out = Vec::new();
    // Poses of one type are solved together: what a swing pose and a twist pose measure can't be set against each
    // other, and neutral poses of each type keep their own type's poses at 0 at rest.
    for kind in 0..3 {
        let solved: Vec<&Pose> = poses.iter().filter(|p| !p.independent && p.kind.clamp(0, 2) == kind).collect();
        if solved.is_empty() {
            continue;
        }
        let n = solved.len();
        let as_rots = |p: &Pose| -> Vec<(DQuat, DVec3)> { rots.iter().enumerate().map(|(k, (_, axis))| (p.rots.get(k).copied().unwrap_or(DQuat::IDENTITY), *axis)).collect() };
        let a: Vec<Vec<f64>> = solved
            .iter()
            .map(|pi| {
                let (ri, ti) = (as_rots(pi), pi.pos.clone());
                solved.iter().map(|pj| kernel(st, dist(st, &ri, &ti, pj), pj) + if std::ptr::eq(*pi, *pj) { st.lambda } else { 0.0 }).collect()
            })
            .collect();
        let phi: Vec<f64> = solved.iter().map(|p| kernel(st, dist(st, &rots, &pos, p), p)).collect();
        let ws: Vec<f64> = match solve(a) {
            Some(inv) => (0..n).map(|k| (0..n).map(|j| phi[j] * inv[j][k]).sum()).collect(),
            None => {
                // One pose, or poses on top of each other: the nearest takes it all.
                let ds: Vec<f64> = solved.iter().map(|p| dist(st, &rots, &pos, p)).collect();
                let best = (0..n).min_by(|&i, &j| ds[i].total_cmp(&ds[j])).unwrap_or(0);
                (0..n).map(|k| if k == best { 1.0 } else { 0.0 }).collect()
            }
        };
        for (p, w) in solved.iter().zip(ws) {
            let w = if st.negative { w } else { w.max(0.0) };
            out.push((p.index, smooth(w, st.smoothing)));
        }
    }
    for p in poses.iter().filter(|p| p.independent) {
        let mut w = 1.0;
        if st.rotation {
            let d = rot_d2(&rots, &p.rots, p.kind).sqrt().to_degrees();
            w *= (1.0 - d / p.rot_falloff.max(1e-3)).max(0.0);
        }
        if st.translation && p.tr_falloff > 0.0 {
            w *= (1.0 - tr_d2(&pos, &p.pos).sqrt() / p.tr_falloff).max(0.0);
        }
        out.push((p.index, smooth(w, st.smoothing)));
    }
    out.sort_by_key(|x| x.0);
    out
}

fn at(d: &NodeData, n: &str) -> usize {
    d.ty().attr_index(n).unwrap_or_else(|| panic!("poseInterpolator has no {n}"))
}

fn axis_of(k: i64) -> DVec3 {
    match k {
        1 => DVec3::Y,
        2 => DVec3::Z,
        _ => DVec3::X,
    }
}

fn quats(v: &PlugValue) -> Vec<DQuat> {
    array_data::of(v).map(|a| a.nums.chunks_exact(4).map(|c| DQuat::from_xyzw(c[0], c[1], c[2], c[3]).normalize()).collect()).unwrap_or_default()
}
fn points(v: &PlugValue) -> Vec<DVec3> {
    array_data::of(v).map(|a| a.nums.chunks_exact(3).map(|c| DVec3::new(c[0], c[1], c[2])).collect()).unwrap_or_default()
}

/// A matrix's rotation, scale taken out, and translation.
pub fn rot_pos(m: &DMat4) -> (DQuat, DVec3) {
    let (t, r, _) = crate::transform::decompose(m);
    (DQuat::from_mat3(&DMat3::from_cols(r.x_axis.normalize_or_zero(), r.y_axis.normalize_or_zero(), r.z_axis.normalize_or_zero())).normalize(), t)
}

fn compute_weights(d: &mut NodeData) {
    let b = |n: &str| d.value(at(d, n)).as_bool();
    let st = Settings {
        rotation: b("enableRotation"),
        translation: b("enableTranslation"),
        gaussian: d.value(at(d, "interpolation")).as_int() == 1,
        negative: b("allowNegativeWeights"),
        smoothing: d.value(at(d, "outputSmoothing")).as_float().clamp(0.0, 1.0),
        lambda: d.value(at(d, "regularization")).as_float().max(0.0),
    };
    let (dm, dt) = (at(d, "driver.driverMatrix"), at(d, "driver.driverTwistAxis"));
    let drivers: Vec<Driver> = d
        .elements(dm)
        .iter()
        .map(|(k, v)| {
            let (rot, pos) = rot_pos(&v.as_matrix());
            Driver { rot, pos, axis: axis_of(d.element_or(dt, *k).as_int()) }
        })
        .collect();
    let names = ["pose.isEnabled", "pose.isIndependent", "pose.poseFalloff", "pose.poseRotation", "pose.poseRotationFalloff", "pose.poseTranslation", "pose.poseTranslationFalloff", "pose.poseType", "pose.poseName"];
    let ids: Vec<usize> = names.iter().map(|n| at(d, n)).collect();
    let mut idx: Vec<i32> = ids.iter().flat_map(|&i| d.elements(i).iter().map(|(k, _)| *k)).collect();
    idx.sort_unstable();
    idx.dedup();
    let poses: Vec<Pose> = idx
        .into_iter()
        .filter(|&k| d.element_or(ids[0], k).as_bool())
        .map(|k| Pose {
            index: k,
            independent: d.element_or(ids[1], k).as_bool(),
            falloff: d.element_or(ids[2], k).as_float(),
            rots: quats(d.element_or(ids[3], k)),
            rot_falloff: d.element_or(ids[4], k).as_float(),
            pos: points(d.element_or(ids[5], k)),
            tr_falloff: d.element_or(ids[6], k).as_float(),
            kind: d.element_or(ids[7], k).as_int(),
        })
        .collect();
    let out = if drivers.is_empty() { Vec::new() } else { weights(&st, &drivers, &poses) };
    let oa = at(d, "output");
    d.set_output_elements(oa, out.into_iter().map(|(k, w)| (k, PlugValue::Float(w))).collect());
}

struct PoseInterpolator;

impl NodeBehavior for PoseInterpolator {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
        compute_weights(d);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let shape = types::find("shape").expect("shape");
        types::register_type(NodeType {
            name: TYPE.into(),
            base: Some(shape),
            dag: true,
            icon: "pose".into(),
            doc: "A pose interpolator: weights for stored poses of driver joints, for corrective shapes.".into(),
            own_attrs: attrs(),
            create: Some(Arc::new(|| Box::new(PoseInterpolator))),
            ..Default::default()
        })
        .expect(TYPE);
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands

fn shape_of(s: &mut Session, name: &str, cmd: &str) -> CmdResult<NodeId> {
    let Some(id) = s.graph.find(name)? else { fail!("{cmd}: no {name}") };
    if s.graph.node(id).is_some_and(|n| n.ty().is_a(TYPE)) {
        return Ok(id);
    }
    if let Some(k) = s.graph.children(id).iter().copied().find(|&k| s.graph.node(k).is_some_and(|n| n.ty().is_a(TYPE))) {
        return Ok(k);
    }
    fail!("{cmd}: {name} is not a pose interpolator")
}

/// The indices in use of a multi compound, over the children named.
fn indices(s: &mut Session, sh: NodeId, kids: &[&str]) -> CmdResult<Vec<i32>> {
    let Some(n) = s.graph.node(sh) else { return Ok(Vec::new()) };
    let mut out: Vec<i32> = kids.iter().filter_map(|k| n.ty().attr_index(k)).flat_map(|i| n.data().elements(i).iter().map(|(k, _)| *k).collect::<Vec<_>>()).collect();
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

/// (index, name) of each pose.
fn poses(s: &mut Session, sh: NodeId) -> CmdResult<Vec<(i32, String)>> {
    let path = s.graph.path(sh);
    let mut out = Vec::new();
    for k in indices(s, sh, &["pose.poseName", "pose.poseRotation", "pose.poseType", "pose.isEnabled"])? {
        let p = s.graph.plug(&format!("{path}.pose[{k}].poseName"))?;
        out.push((k, s.graph.value(&p)?.as_str().to_string()));
    }
    Ok(out)
}

fn pose_index(s: &mut Session, sh: NodeId, name: &str, cmd: &str) -> CmdResult<i32> {
    poses(s, sh)?.into_iter().find(|x| x.1 == name).map(|x| x.0).ok_or_else(|| aztek_ops::CommandError::new(format!("{cmd}: no pose {name}")))
}

/// Each driver: (index, the node feeding driverMatrix, its matrix now).
fn drivers(s: &mut Session, sh: NodeId) -> CmdResult<Vec<(i32, NodeId, DMat4)>> {
    s.graph.evaluate();
    let path = s.graph.path(sh);
    let mut out = Vec::new();
    for k in indices(s, sh, &["driver.driverMatrix", "driver.driverTwistAxis"])? {
        let p = s.graph.plug(&format!("{path}.driver[{k}].driverMatrix"))?;
        let m = s.graph.value(&p)?.as_matrix();
        let src = s.graph.source(&p).map_or(NO_NODE, |x| x.node);
        out.push((k, src, m));
    }
    Ok(out)
}

/// Stores the drivers as they are now into pose k.
fn store(s: &mut Session, sh: NodeId, k: i32) -> CmdResult<()> {
    let ds = drivers(s, sh)?;
    let (mut r, mut t) = (ArrayData::empty("doubleArray"), ArrayData::empty("doubleArray"));
    for (_, _, m) in &ds {
        let (q, p) = rot_pos(m);
        r.nums.extend([q.x, q.y, q.z, q.w]);
        t.nums.extend([p.x, p.y, p.z]);
    }
    let path = s.graph.path(sh);
    let pr = s.graph.plug(&format!("{path}.pose[{k}].poseRotation"))?;
    s.graph.set_value(pr, r.value())?;
    let pt = s.graph.plug(&format!("{path}.pose[{k}].poseTranslation"))?;
    s.graph.set_value(pt, t.value())?;
    Ok(())
}

fn add_pose(s: &mut Session, sh: NodeId, name: &str, kind: i64) -> CmdResult<i32> {
    let have = poses(s, sh)?;
    if have.iter().any(|x| x.1 == name) {
        fail!("poseInterpolator: there is already a pose {name}");
    }
    let k = have.iter().map(|x| x.0 + 1).max().unwrap_or(0);
    let path = s.graph.path(sh);
    s.run("setAttr", json!({"args": [format!("{path}.pose[{k}].poseName"), name], "type": "string"}))?;
    s.run("setAttr", json!({"args": [format!("{path}.pose[{k}].poseType"), kind]}))?;
    store(s, sh, k)?;
    Ok(k)
}

/// The default name of a new pose: the driver's name and its rotate channels, rounded.
fn default_name(s: &mut Session, sh: NodeId) -> CmdResult<String> {
    let ds = drivers(s, sh)?;
    let Some(&(_, src, _)) = ds.first() else { return Ok("pose".into()) };
    if src == NO_NODE {
        return Ok("pose".into());
    }
    let n = s.graph.node(src).map(|x| x.name().to_string()).unwrap_or_default();
    let p = s.graph.path(src);
    let mut parts = vec![n];
    for c in ["X", "Y", "Z"] {
        let v = s.run("getAttr", json!({"args": [format!("{p}.rotate{c}")]}))?.as_f64().unwrap_or(0.0);
        parts.push(format!("{}", v.to_degrees().round() as i64).replace('-', "n"));
    }
    Ok(parts.join("_"))
}

fn go_to(s: &mut Session, sh: NodeId, k: i32) -> CmdResult<()> {
    let path = s.graph.path(sh);
    let pr = s.graph.plug(&format!("{path}.pose[{k}].poseRotation"))?;
    let rots = quats(&s.graph.value(&pr)?);
    let pt = s.graph.plug(&format!("{path}.pose[{k}].poseTranslation"))?;
    let pos = points(&s.graph.value(&pt)?);
    let et = s.graph.plug(&format!("{path}.enableTranslation"))?;
    let with_t = s.graph.value(&et)?.as_bool();
    for (j, (_, src, m)) in drivers(s, sh)?.into_iter().enumerate() {
        if src == NO_NODE {
            continue;
        }
        let Some(q) = rots.get(j).copied() else { continue };
        let (t, r, sc) = crate::transform::decompose(&m);
        let _ = r;
        let t = if with_t { pos.get(j).copied().unwrap_or(t) } else { t };
        let mx = DMat4::from_scale_rotation_translation(sc, q, t);
        let sp = s.graph.path(src);
        s.run("xform", json!({"args": [sp], "objectSpace": true, "matrix": mx.to_cols_array().to_vec()}))?;
    }
    Ok(())
}

fn chunk<T>(s: &mut Session, name: &str, f: impl FnOnce(&mut Session) -> CmdResult<T>) -> CmdResult<T> {
    s.graph.open_chunk(name);
    let r = f(s);
    s.graph.close_chunk();
    if r.is_err() {
        s.undo();
    }
    r
}

/// Makes a pose interpolator on a joint: its transform and shape beside the scene's other top-level nodes, the joint's
/// matrix as driver 0, with neutral poses when asked.
pub fn create(s: &mut Session, joint: NodeId, neutral: bool) -> CmdResult<NodeId> {
    let jn = s.graph.node(joint).map(|n| n.name().to_string()).unwrap_or_default();
    let jp = s.graph.path(joint);
    let (tr, sh) = crate::dag::create_with_shape(&mut s.graph, TYPE, &format!("{jn}_poseInterpolator"), NO_NODE)?;
    let tn = s.graph.node(tr).map(|n| n.name().to_string()).unwrap_or_default();
    s.graph.rename(sh, &format!("{tn}Shape"))?;
    let sp = s.graph.path(sh);
    s.run("connectAttr", json!({"args": [format!("{jp}.matrix"), format!("{sp}.driver[0].driverMatrix")]}))?;
    if s.graph.node(joint).is_some_and(|n| n.ty().is_a("joint")) {
        for (from, to) in [("jointOrient", "driverOrient")] {
            let v = s.run("getAttr", json!({"args": [format!("{jp}.{from}")]}))?;
            if let Some(v) = v.as_array().and_then(|x| x.first()).and_then(Value::as_array) {
                let n: Vec<f64> = v.iter().filter_map(Value::as_f64).collect();
                if n.len() == 3 {
                    s.run("setAttr", json!({"args": [format!("{sp}.driver[0].{to}"), n[0], n[1], n[2]], "type": "double3"}))?;
                }
            }
        }
    }
    let ro = s.run("getAttr", json!({"args": [format!("{jp}.rotateOrder")]})).ok().and_then(|v| v.as_i64()).unwrap_or(0);
    s.run("setAttr", json!({"args": [format!("{sp}.driver[0].driverRotateOrder"), ro]}))?;
    if neutral {
        for (name, kind) in [("neutral", 0), ("neutralSwing", 1), ("neutralTwist", 2)] {
            add_pose(s, sh, name, kind)?;
        }
    }
    Ok(sh)
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        CommandDef::<Session>::new(
            "poseInterpolator",
            "Works on a pose interpolator: -addPose name (the drivers as they are now; -poseType 0 swing and twist, 1 swing, \
             2 twist) returns the pose's index, -deletePose, -updatePose (stores the drivers now), -goToPose (sets the \
             drivers to it), -rename old new. With -query: -poseNames, -drivers, -index with -pose name.",
            |s, a| {
                let names = a.named();
                let Some(n) = names.first().cloned() else { fail!("poseInterpolator: name a pose interpolator") };
                let sh = shape_of(s, &n, "poseInterpolator")?;
                let queried: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
                let asked = |f: &str| a.bool(f) || queried.iter().any(|x| x == f);
                if a.bool("query") {
                    if asked("poseNames") {
                        return Ok(json!(poses(s, sh)?.into_iter().map(|x| x.1).collect::<Vec<_>>()));
                    }
                    if asked("drivers") {
                        let ds = drivers(s, sh)?;
                        return Ok(json!(ds.into_iter().filter(|x| x.1 != NO_NODE).map(|x| s.graph.path(x.1)).collect::<Vec<_>>()));
                    }
                    if asked("index") {
                        let p = a.string("pose", "");
                        return Ok(json!(pose_index(s, sh, &p, "poseInterpolator")?));
                    }
                    fail!("poseInterpolator: query what?");
                }
                if let Some(name) = a.get("addPose").and_then(Value::as_str) {
                    let name = if name.is_empty() { default_name(s, sh)? } else { name.to_string() };
                    let kind = a.int("poseType", 0).clamp(0, 2);
                    return Ok(json!(chunk(s, "Add Pose", |s| add_pose(s, sh, &name, kind))?));
                }
                if let Some(name) = a.get("deletePose").and_then(Value::as_str) {
                    let k = pose_index(s, sh, name, "poseInterpolator")?;
                    let path = s.graph.path(sh);
                    s.run("removeMultiInstance", json!({"args": [format!("{path}.pose[{k}]")]}))?;
                    return Ok(Value::Null);
                }
                if let Some(name) = a.get("updatePose").and_then(Value::as_str) {
                    let k = pose_index(s, sh, name, "poseInterpolator")?;
                    chunk(s, "Update Pose", |s| store(s, sh, k))?;
                    return Ok(Value::Null);
                }
                if let Some(name) = a.get("goToPose").and_then(Value::as_str) {
                    let k = pose_index(s, sh, name, "poseInterpolator")?;
                    chunk(s, "Go to Pose", |s| go_to(s, sh, k))?;
                    return Ok(Value::Null);
                }
                if let Some(r) = a.get("rename").and_then(Value::as_array) {
                    let (Some(old), Some(new)) = (r.first().and_then(Value::as_str), r.get(1).and_then(Value::as_str)) else { fail!("poseInterpolator: -rename old new") };
                    let k = pose_index(s, sh, old, "poseInterpolator")?;
                    let path = s.graph.path(sh);
                    s.run("setAttr", json!({"args": [format!("{path}.pose[{k}].poseName"), new], "type": "string"}))?;
                    return Ok(Value::Null);
                }
                fail!("poseInterpolator: nothing to do")
            },
        )
        .positional("pose interpolator")
        .flag(FlagDef::new("addPose", "ap", ArgType::String, Value::Null, "Adds a pose of the drivers as they are now; an empty name takes the default."))
        .flag(FlagDef::new("poseType", "pt", ArgType::Int, Value::Null, "With -addPose: 0 swing and twist, 1 swing only, 2 twist only."))
        .flag(FlagDef::new("deletePose", "dp", ArgType::String, Value::Null, "Removes a pose."))
        .flag(FlagDef::new("updatePose", "up", ArgType::String, Value::Null, "Stores the drivers as they are now in a pose."))
        .flag(FlagDef::new("goToPose", "gtp", ArgType::String, Value::Null, "Sets the drivers to a pose."))
        .flag(FlagDef::new("rename", "rn", ArgType::List, Value::Null, "Renames a pose: old new."))
        .flag(FlagDef::new("poseNames", "pn", ArgType::Bool, Value::Null, "With -query: the poses' names, by index."))
        .flag(FlagDef::new("drivers", "d", ArgType::Bool, Value::Null, "With -query: the driver nodes."))
        .flag(FlagDef::new("index", "i", ArgType::Bool, Value::Null, "With -query and -pose: the pose's index."))
        .flag(FlagDef::new("pose", "p", ArgType::String, Value::Null, "The pose -index asks for."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Reads."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What -q asks for.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "createPoseInterpolator",
            "Deform > Pose Space Deformation > Create Pose Interpolator: a pose interpolator for each joint named or \
             selected (<joint>_poseInterpolator), driven by the joint's matrix, with the neutral, neutralSwing and \
             neutralTwist poses unless -neutralPoses is off. Returns the shapes. Aztek's name for the menu item.",
            |s, a| {
                let mut targets: Vec<NodeId> = Vec::new();
                for n in a.named() {
                    match s.graph.find(&n)? {
                        Some(id) => targets.push(id),
                        None => fail!("createPoseInterpolator: no {n}"),
                    }
                }
                if targets.is_empty() {
                    targets = s.graph.selection().to_vec();
                }
                targets.retain(|&t| crate::dag::is_transform(&s.graph, t));
                if targets.is_empty() {
                    fail!("createPoseInterpolator: select a joint");
                }
                let neutral = a.get("neutralPoses").map_or(true, |v| v.as_bool().unwrap_or(true));
                let out = chunk(s, "Create Pose Interpolator", |s| {
                    let mut out = Vec::new();
                    for t in targets {
                        let sh = create(s, t, neutral)?;
                        out.push(s.graph.path(sh));
                    }
                    Ok(out)
                })?;
                Ok(json!(out))
            },
        )
        .positional("joints")
        .flag(FlagDef::new("neutralPoses", "np", ArgType::Bool, Value::Null, "Adds the neutral poses; on by default.")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_inverts() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let inv = solve(a.clone()).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let x: f64 = (0..2).map(|k| a[i][k] * inv[k][j]).sum();
                assert!((x - if i == j { 1.0 } else { 0.0 }).abs() < 1e-12);
            }
        }
        assert!(solve(vec![vec![1.0, 1.0], vec![1.0, 1.0]]).is_none());
    }

    #[test]
    fn twist_about_an_axis() {
        let q = DQuat::from_axis_angle(DVec3::X, 0.5);
        assert!((twist(q, DQuat::IDENTITY, DVec3::X) - 0.5).abs() < 1e-9);
        assert!(twist(q, DQuat::IDENTITY, DVec3::Y) < 1e-9);
        let sw = DQuat::from_axis_angle(DVec3::Z, 0.7);
        assert!((angle(sw * DVec3::X, DVec3::X) - 0.7).abs() < 1e-9);
    }
}
