//! Dynamic hair, as the reference's nHair: follicles hold the start curves and hand them to a hairSystem, which moves
//! each strand's points a frame at a time (gravity, damping, drag, stretch and bend resistance, start curve attract)
//! and hands the moved curves back to the follicles, whose output curves draw them. Made with makeCurvesDynamic.
//!
//! Differs from the reference: a strand moves by its curve's CVs, not by points sampled along it; the hairSystem has
//! its own gravity and start frame rather than a nucleus's, and doesn't collide.

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Arc;

use aztek_geo::nurbs::NurbsCurve;
use aztek_ops::{fail, Args, CmdResult, CommandDef, Commands};
use glam::DVec3;
use serde_json::{json, Value};

use crate::curves;
use crate::graph::Graph;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue};
use crate::session::Session;
use crate::transform::Shape;

pub const HAIR_SYSTEM: &str = "hairSystem";

const IN: u32 = flags::INPUT | flags::STORABLE | flags::KEYABLE;
const HIDDEN_IN: u32 = flags::INPUT | flags::HIDDEN;
const HIDDEN_OUT: u32 = flags::OUTPUT | flags::HIDDEN;

fn float_attr(name: &str, short: &str, d: f64, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Float, default_value: PlugValue::Float(d), flags: IN, doc: doc.into(), ..Default::default() }
}

fn array(mut a: AttrDef) -> AttrDef {
    a.flags |= flags::ARRAY;
    a
}

// The hairSystem's attributes, after the shape's.
const H_INPUT: usize = Shape::COUNT;
const H_LOCK: usize = Shape::COUNT + 1;
const H_OUTPUT: usize = Shape::COUNT + 2;
const H_ACTIVE: usize = Shape::COUNT + 3;
const H_START: usize = Shape::COUNT + 4;
const H_STRETCH: usize = Shape::COUNT + 5;
const H_BEND: usize = Shape::COUNT + 6;
const H_ATTRACT: usize = Shape::COUNT + 7;
const H_DAMP: usize = Shape::COUNT + 8;
const H_DRAG: usize = Shape::COUNT + 9;
const H_GRAVITY: usize = Shape::COUNT + 10;
const H_SUBSTEPS: usize = Shape::COUNT + 11;
const H_ITERATIONS: usize = Shape::COUNT + 12;

fn hair_attrs() -> Vec<AttrDef> {
    vec![
        array(curves::curve_attr("inputHair", "ih", HIDDEN_IN, "Each follicle's start curve, world space.")),
        array(AttrDef { name: "inputHairPin".into(), short_name: "ihp".into(), ty: PlugType::Int, default_value: PlugValue::Int(1), flags: HIDDEN_IN, doc: "Each follicle's point lock: 0 none, 1 base, 2 tip, 3 both ends.".into(), ..Default::default() }),
        array(curves::curve_attr("outputHair", "oh", HIDDEN_OUT, "Each strand as it moved, world space.")),
        AttrDef { flags: IN, ..crate::attr_defs::bool_attr("active", true, "Simulate; off, the strands are their start curves.") },
        float_attr("startFrame", "stf", 1.0, "The frame the strands start from their start curves."),
        float_attr("stretchResistance", "sr", 10.0, "How hard the strands resist stretching."),
        float_attr("bendResistance", "br", 0.1, "How hard they resist bending."),
        float_attr("startCurveAttract", "sct", 0.0, "How strongly each strand keeps its start curve's shape, 0 to 1."),
        float_attr("damp", "dmp", 0.0, "Damping of the strands' own motion."),
        float_attr("drag", "drg", 0.05, "Air drag: speed lost to the air."),
        float_attr("gravity", "grv", 9.8, "Gravity, scene units a second squared, down Y."),
        AttrDef { name: "substeps".into(), short_name: "sbs".into(), ty: PlugType::Int, default_value: PlugValue::Int(4), flags: IN, doc: "Steps a frame.".into(), ..Default::default() },
        AttrDef { name: "iterations".into(), short_name: "itr".into(), ty: PlugType::Int, default_value: PlugValue::Int(8), flags: IN, doc: "Constraint passes a step.".into(), ..Default::default() },
    ]
}

#[derive(Clone)]
struct Strand {
    pos: Vec<DVec3>,
    prev: Vec<DVec3>,
    /// The start curve's CVs the frame was solved against.
    target: Vec<DVec3>,
}

#[derive(Default)]
struct HairSystem {
    frames: BTreeMap<i64, Vec<Strand>>,
    key: Vec<u64>,
}

const KEEP: usize = 2048;

struct Settings {
    stretch: f64,
    bend: f64,
    attract: f64,
    damp: f64,
    drag: f64,
    gravity: f64,
    substeps: usize,
    iterations: usize,
}

/// Moves a strand from where it was to the next frame. `from` and `to` are its start curve at the frame before and
/// this one: the locked points ride between them.
fn step(s: &mut Strand, from: &[DVec3], to: &[DVec3], lock: i64, k: &Settings, dt: f64) {
    let n = s.pos.len().min(to.len()).min(from.len());
    if n == 0 {
        return;
    }
    let rest: Vec<f64> = (1..n).map(|i| (to[i] - to[i - 1]).length()).collect();
    let rest2: Vec<f64> = (2..n).map(|i| (to[i] - to[i - 2]).length()).collect();
    let pinned = |i: usize| (lock == 1 || lock == 3) && i == 0 || (lock == 2 || lock == 3) && i + 1 == n;
    let h = dt / k.substeps as f64;
    // Resistances 0 to 10 and more map to how much of the error each pass takes.
    let w_stretch = (k.stretch / (k.stretch + 1.0)).clamp(0.0, 1.0);
    let w_bend = (k.bend / (k.bend + 0.5)).clamp(0.0, 1.0);
    let keep = (1.0 - k.damp.clamp(0.0, 1.0) * h * 10.0).clamp(0.0, 1.0) * (1.0 - k.drag.clamp(0.0, 1.0) * h * 10.0).clamp(0.0, 1.0);
    for sub in 1..=k.substeps {
        let t = sub as f64 / k.substeps as f64;
        let goal: Vec<DVec3> = (0..n).map(|i| from[i] + (to[i] - from[i]) * t).collect();
        for i in 0..n {
            if pinned(i) {
                s.prev[i] = s.pos[i];
                s.pos[i] = goal[i];
                continue;
            }
            let v = (s.pos[i] - s.prev[i]) * keep;
            s.prev[i] = s.pos[i];
            s.pos[i] += v + DVec3::new(0.0, -k.gravity, 0.0) * h * h;
        }
        for _ in 0..k.iterations {
            for i in 1..n {
                let (a, b) = (i - 1, i);
                solve_distance(&mut s.pos, a, b, rest[i - 1], w_stretch, pinned(a), pinned(b));
            }
            for i in 2..n {
                solve_distance(&mut s.pos, i - 2, i, rest2[i - 2], w_bend, pinned(i - 2), pinned(i));
            }
            if k.attract > 0.0 {
                // The start curve's shape, carried with the strand's root.
                let shift = s.pos[0] - goal[0];
                let a = (k.attract.clamp(0.0, 1.0) / k.iterations as f64).min(1.0);
                for i in 0..n {
                    if !pinned(i) {
                        let want = goal[i] + shift;
                        { let p = s.pos[i]; s.pos[i] = p + (want - p) * a; }
                    }
                }
            }
        }
    }
    s.target = to.to_vec();
}

fn solve_distance(p: &mut [DVec3], a: usize, b: usize, rest: f64, w: f64, fa: bool, fb: bool) {
    let d = p[b] - p[a];
    let len = d.length();
    if len < 1e-12 || (fa && fb) {
        return;
    }
    let err = (len - rest) / len * w;
    let (wa, wb) = match (fa, fb) {
        (true, false) => (0.0, 1.0),
        (false, true) => (1.0, 0.0),
        _ => (0.5, 0.5),
    };
    p[a] += d * err * wa;
    p[b] -= d * err * wb;
}

impl NodeBehavior for HairSystem {
    fn compute(&mut self, ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
        let inputs: Vec<(i32, Option<NurbsCurve>)> = d.elements(H_INPUT).iter().map(|(k, v)| (*k, curves::curve_value(v).cloned())).collect();
        let locks: BTreeMap<i32, i64> = d.elements(H_LOCK).iter().map(|(k, v)| (*k, v.scalar().unwrap_or(1.0) as i64)).collect();
        let lock_of = |k: i32| locks.get(&k).copied().unwrap_or(1);
        let starts: Vec<Vec<DVec3>> = inputs.iter().map(|(_, c)| c.as_ref().map(|c| c.cvs.clone()).unwrap_or_default()).collect();
        let write = |d: &mut NodeData, strands: &[Vec<DVec3>]| {
            let els: Vec<(i32, PlugValue)> = inputs
                .iter()
                .zip(strands)
                .filter_map(|((k, c), p)| c.as_ref().map(|c| (*k, curves::to_value(NurbsCurve { cvs: p.clone(), ..c.clone() }))))
                .collect();
            d.set_output_elements(H_OUTPUT, els);
        };
        let start = d.float(H_START).round() as i64;
        let frame = (ctx.time * crate::anim::fps()).round() as i64;
        if !d.bool(H_ACTIVE) || frame <= start {
            self.frames.clear();
            write(d, &starts);
            return;
        }
        let k = Settings {
            stretch: d.float(H_STRETCH).max(0.0),
            bend: d.float(H_BEND).max(0.0),
            attract: d.float(H_ATTRACT),
            damp: d.float(H_DAMP),
            drag: d.float(H_DRAG),
            gravity: d.float(H_GRAVITY),
            substeps: (d.int(H_SUBSTEPS).clamp(1, 64)) as usize,
            iterations: (d.int(H_ITERATIONS).clamp(1, 100)) as usize,
        };
        // Settings or strands changed: what was solved goes.
        let mut key: Vec<u64> = [k.stretch, k.bend, k.attract, k.damp, k.drag, k.gravity].iter().map(|x| x.to_bits()).collect();
        key.extend([k.substeps as u64, k.iterations as u64]);
        key.extend(inputs.iter().map(|(i, _)| (*i as u64) << 32 | lock_of(*i) as u64));
        key.extend(starts.iter().map(|s| s.len() as u64));
        if key != self.key {
            self.frames.clear();
            self.key = key;
        }
        let fresh = || -> Vec<Strand> { starts.iter().map(|s| Strand { pos: s.clone(), prev: s.clone(), target: s.clone() }).collect() };
        let same_targets = |f: &Vec<Strand>| f.iter().zip(&starts).all(|(s, t)| s.target == *t);
        let state = if let Some(f) = self.frames.get(&frame).filter(|f| same_targets(f)) {
            f.clone()
        } else {
            let _ = self.frames.split_off(&frame);
            let (from, mut st) = match self.frames.iter().next_back() {
                Some((f, s)) => (*f, s.clone()),
                None => (start, fresh()),
            };
            let dt = 1.0 / crate::anim::fps();
            let steps = (frame - from).clamp(0, KEEP as i64);
            for n in 0..steps {
                // The start curves between the last frame solved and this one, in a straight line.
                let t0 = n as f64 / steps as f64;
                let t1 = (n + 1) as f64 / steps as f64;
                for (i, s) in st.iter_mut().enumerate() {
                    let was = s.target.clone();
                    let now = &starts[i];
                    if was.len() != now.len() {
                        *s = Strand { pos: now.clone(), prev: now.clone(), target: now.clone() };
                        continue;
                    }
                    let a: Vec<DVec3> = was.iter().zip(now).map(|(p, q)| *p + (*q - *p) * t0).collect();
                    let b: Vec<DVec3> = was.iter().zip(now).map(|(p, q)| *p + (*q - *p) * t1).collect();
                    let lock = lock_of(inputs[i].0);
                    step(s, &a, &b, lock, &k, dt);
                    s.target = if n + 1 == steps { now.clone() } else { was };
                }
            }
            st
        };
        if self.frames.len() >= KEEP {
            let first = *self.frames.keys().next().unwrap();
            self.frames.remove(&first);
        }
        self.frames.insert(frame, state.clone());
        let pts: Vec<Vec<DVec3>> = state.into_iter().map(|s| s.pos).collect();
        write(d, &pts);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_types() {
    if types::find(HAIR_SYSTEM).is_some() {
        return;
    }
    let shape = types::find("shape").expect("shape");
    let _ = types::register_type(NodeType {
        name: HAIR_SYSTEM.into(),
        base: Some(shape),
        dag: true,
        icon: "wave-sine".into(),
        doc: "Dynamic hair: moves its follicles' curves a frame at a time.".into(),
        own_attrs: hair_attrs(),
        create: Some(Arc::new(|| Box::<HairSystem>::default())),
        time_dependent: true,
        ..Default::default()
    });
}

fn run(s: &mut Session, cmd: &str, call: Value) -> CmdResult {
    s.run(cmd, call)
}

fn first_name(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(a) => a.first().and_then(Value::as_str).unwrap_or("").to_string(),
        _ => String::new(),
    }
}

fn unique(g: &Graph, base: &str) -> String {
    let mut i = 1;
    loop {
        let n = format!("{base}{i}");
        if g.find(&n).ok().flatten().is_none() {
            return n;
        }
        i += 1;
    }
}

/// The hair system shape named or selected, if any.
fn hair_shape_of(g: &Graph, id: NodeId) -> Option<NodeId> {
    if g.node(id).is_some_and(|n| n.ty().name == HAIR_SYSTEM) {
        return Some(id);
    }
    crate::dag::shapes(g, id, false).into_iter().find(|&c| g.node(c).is_some_and(|n| n.ty().name == HAIR_SYSTEM))
}

fn make_curves_dynamic(s: &mut Session, a: &Args) -> CmdResult {
    let mut names: Vec<String> = a.named().into_iter().filter(|n| s.graph.find(n).ok().flatten().is_some()).collect();
    if names.is_empty() {
        names = s.graph.selection().iter().map(|&id| s.graph.path(id)).collect();
    }
    let mut curves_in = Vec::new();
    let mut system = None;
    for n in &names {
        let Some(id) = s.graph.find(n)? else { continue };
        if let Some(h) = hair_shape_of(&s.graph, id) {
            system = Some(h);
        } else if let Some(c) = curves::as_curve(&s.graph, id) {
            curves_in.push(c);
        }
    }
    if curves_in.is_empty() {
        fail!("makeCurvesDynamic: select curves (and a hair system to add them to)");
    }
    s.graph.open_chunk("Make Curves Dynamic");
    let r = (|| -> CmdResult {
        let (hs_shape, hs_name) = match system {
            Some(h) => {
                let t = crate::dag::transform_of(&s.graph, h);
                (s.graph.path(h), s.graph.node(t).map(|n| n.name().to_string()).unwrap_or_default())
            }
            None => {
                let t = unique(&s.graph, "hairSystem");
                let shape_name = t.replace("hairSystem", "hairSystemShape");
                let made = run(s, "createNode", json!({"args": ["transform"], "name": t}))?;
                let t = first_name(&made);
                let sh = run(s, "createNode", json!({"args": [HAIR_SYSTEM], "name": shape_name, "parent": t}))?;
                (first_name(&sh), t)
            }
        };
        let group = |s: &mut Session, name: String| -> CmdResult<String> {
            if s.graph.find(&name)?.is_some() {
                return Ok(name);
            }
            Ok(first_name(&run(s, "createNode", json!({"args": ["transform"], "name": name}))?))
        };
        let follicles = group(s, format!("{hs_name}Follicles"))?;
        let outputs = group(s, format!("{hs_name}OutputCurves"))?;
        let hs_id = s.graph.find(&hs_shape)?.ok_or_else(|| aztek_ops::CommandError::new("makeCurvesDynamic: the hair system went"))?;
        let mut next = s.graph.node(hs_id).map(|n| n.data().elements(H_INPUT).iter().map(|(k, _)| *k + 1).max().unwrap_or(0)).unwrap_or(0);
        for c in curves_in {
            let curve_shape = s.graph.path(c);
            let curve_t = s.graph.path(crate::dag::transform_of(&s.graph, c));
            let fname = unique(&s.graph, "follicle");
            let ft = first_name(&run(s, "createNode", json!({"args": ["transform"], "name": fname, "parent": follicles}))?);
            let fs = first_name(&run(s, "createNode", json!({"args": [crate::nurbs_nodes::FOLLICLE], "name": fname.replace("follicle", "follicleShape"), "parent": ft}))?);
            // The start curve goes under its follicle, keeping where it is, as the reference puts it.
            let _ = run(s, "parent", json!({"args": [curve_t, ft]}));
            let curve_shape = s.graph.find(&curve_shape)?.map(|id| s.graph.path(id)).unwrap_or(curve_shape);
            run(s, "connectAttr", json!({"args": [format!("{curve_shape}.worldSpace"), format!("{fs}.startPosition")]}))?;
            run(s, "connectAttr", json!({"args": [format!("{fs}.outHair"), format!("{hs_shape}.inputHair[{next}]")]}))?;
            run(s, "connectAttr", json!({"args": [format!("{fs}.pointLock"), format!("{hs_shape}.inputHairPin[{next}]")]}))?;
            run(s, "connectAttr", json!({"args": [format!("{hs_shape}.outputHair[{next}]"), format!("{fs}.currentPosition")]}))?;
            run(s, "setAttr", json!({"args": [format!("{fs}.pointLock"), 1]}))?;
            let oname = unique(&s.graph, "curve");
            let ot = first_name(&run(s, "createNode", json!({"args": ["transform"], "name": oname, "parent": outputs}))?);
            let os = first_name(&run(s, "createNode", json!({"args": [curves::SHAPE_TYPE], "name": format!("{oname}Shape"), "parent": ot}))?);
            run(s, "connectAttr", json!({"args": [format!("{fs}.outCurve"), format!("{os}.create")]}))?;
            next += 1;
        }
        Ok(json!([hs_shape]))
    })();
    s.graph.close_chunk();
    r
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "makeCurvesDynamic",
            "Make Selected Curves Dynamic: each selected curve gets a follicle on a hair system (a new one, or the one selected) and a dynamic output curve. The reference's arguments (surface attach and its options) are accepted. Returns the hair system.",
            make_curves_dynamic,
        )
        .positional("the reference's options, curves or a hair system (the selection when none)"),
    );
}
