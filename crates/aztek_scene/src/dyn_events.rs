//! Collision events on particles (`event`), as the reference's event command makes them: each event is an element of
//! the particles node's event arrays (eventName, eventCount, eventEmit, eventSplit, eventDie, eventRandom, eventSpread,
//! eventTarget, eventProc). After each solved frame, every particle that hit a collider (or the ground plane) counts the
//! collision; each event whose count is 0 or that collision's number acts on it: it emits particles into the target
//! (or its own object), splits into new ones there, dies, and has its procedure called as
//! `proc(object, particleId, objectHit)`. New particles appear at the next frame.

use std::collections::HashMap;
use std::sync::Mutex;

use aztek_ops::{fail, objects, ArgType, Args, CmdResult, CommandDef, Commands};
use glam::DVec3;
use serde_json::{json, Value};

use crate::dynamics as dy;
use crate::graph::Graph;
use crate::plug::{NodeId, PlugRef, PlugValue};
use crate::ref_core::{asked, f};
use crate::session::Session;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Float as Fl, Int as I, String as S};

/// The event arrays on the particles node: (attribute, short name, the flag that sets it).
pub const ATTRS: &[(&str, &str, &str)] = &[
    ("eventName", "evn", "name"),
    ("eventCount", "evc", "count"),
    ("eventEmit", "eve", "emit"),
    ("eventSplit", "evs", "split"),
    ("eventDie", "evd", "dieAtCollision"),
    ("eventRandom", "evr", "random"),
    ("eventSpread", "evp", "spread"),
    ("eventTarget", "evt", "target"),
    ("eventProc", "epr", "proc"),
];

/// One event as the arrays hold it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    pub index: i32,
    pub name: String,
    pub count: i64,
    pub emit: i64,
    pub split: i64,
    pub die: bool,
    pub random: bool,
    pub spread: f64,
    pub target: String,
    pub proc_name: String,
}

fn attr(g: &Graph, id: NodeId, name: &str) -> Option<usize> {
    g.node(id)?.ty().attr_index(name)
}

/// The events on a particles node, in index order.
pub fn events(g: &Graph, id: NodeId) -> Vec<Event> {
    let Some(n) = g.node(id) else { return Vec::new() };
    let Some(ni) = attr(g, id, "eventName") else { return Vec::new() };
    let el = |a: &str, i: i32| attr(g, id, a).and_then(|k| n.elements(k).iter().find(|e| e.0 == i).map(|e| e.1.clone()));
    let num = |a: &str, i: i32| el(a, i).and_then(|v| v.scalar()).unwrap_or(0.0);
    let text = |a: &str, i: i32| match el(a, i) {
        Some(PlugValue::String(t)) => t,
        _ => String::new(),
    };
    n.elements(ni)
        .iter()
        .map(|(i, v)| Event {
            index: *i,
            name: match v {
                PlugValue::String(t) => t.clone(),
                _ => String::new(),
            },
            count: num("eventCount", *i) as i64,
            emit: num("eventEmit", *i) as i64,
            split: num("eventSplit", *i) as i64,
            die: num("eventDie", *i) != 0.0,
            random: num("eventRandom", *i) != 0.0,
            spread: num("eventSpread", *i),
            target: text("eventTarget", *i),
            proc_name: text("eventProc", *i),
        })
        .collect()
}

fn set_el(g: &mut Graph, id: NodeId, a: &str, index: i32, v: PlugValue) -> CmdResult<()> {
    let Some(k) = attr(g, id, a) else { fail!("event: the particles have no {a}") };
    g.set_value(PlugRef { node: id, attr: k as i32, component: -1, element: index }, v)?;
    Ok(())
}

fn particles_of(g: &Graph, name: &str) -> CmdResult<NodeId> {
    crate::dyn_cmds::resolve(g, name, &[dy::PARTICLES]).ok_or_else(|| aztek_ops::CommandError::new(format!("event: {name} is not a particle object")))
}

fn as_bool(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true" | "yes" | "1")))
}

/// Writes what the flags give onto an event.
fn apply_flags(g: &mut Graph, id: NodeId, index: i32, a: &Args) -> CmdResult<()> {
    for (attr_name, _, flag) in ATTRS.iter().skip(1) {
        let Some(v) = a.get(flag) else { continue };
        let pv = match *flag {
            "count" | "emit" | "split" => {
                let n = v.as_i64().or_else(|| v.as_f64().map(|x| x as i64)).unwrap_or(0);
                if n < 0 {
                    fail!("event: -{flag} takes 0 or more");
                }
                PlugValue::Int(n)
            }
            "dieAtCollision" | "random" => PlugValue::Bool(as_bool(v)),
            "spread" => PlugValue::Float(v.as_f64().unwrap_or(0.0).clamp(0.0, 1.0)),
            _ => PlugValue::String(v.as_str().unwrap_or("").to_string()),
        };
        if *flag == "target" {
            let t = v.as_str().unwrap_or("");
            if !t.is_empty() {
                particles_of(g, t)?;
            }
        }
        set_el(g, id, attr_name, index, pv)?;
    }
    Ok(())
}

fn event(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let name_flag = a.get("name").and_then(Value::as_str).map(str::to_string);
    if a.bool("list") {
        let g = &s.graph;
        // Named, or the selected particle objects, or all of them.
        let names: Vec<String> = if a.named().is_empty() { names.into_iter().filter(|n| particles_of(g, n).is_ok()).collect() } else { names };
        if names.is_empty() {
            let mut out = Vec::new();
            for id in g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().name == dy::PARTICLES)) {
                for e in events(g, id) {
                    out.push(json!(g.path(id)));
                    out.push(json!(e.name));
                }
            }
            return Ok(Value::Array(out));
        }
        let mut out = Vec::new();
        for n in &names {
            let id = particles_of(g, n)?;
            out.extend(events(g, id).into_iter().map(|e| json!(e.name)));
        }
        return Ok(Value::Array(out));
    }
    // Query: event -count -q -name foo myCloud; the name may come as the first word.
    let mut words = names.clone();
    let mut ev_name = name_flag.clone();
    if a.bool("query") && ev_name.is_none() && words.len() > 1 {
        ev_name = Some(words.remove(0));
    }
    let Some(obj) = words.first().cloned() else { fail!("event: name a particle object or select one") };
    let id = particles_of(&s.graph, &obj)?;
    let list = events(&s.graph, id);
    let find = |n: &Option<String>| -> CmdResult<Event> {
        match n {
            Some(n) => list.iter().find(|e| e.name == *n).cloned().ok_or_else(|| aztek_ops::CommandError::new(format!("event: {obj} has no event named {n}"))),
            None => list.first().cloned().ok_or_else(|| aztek_ops::CommandError::new(format!("event: {obj} has no events"))),
        }
    };
    if a.bool("query") {
        let e = find(&ev_name)?;
        let q = crate::system_cmds::q_one(a).unwrap_or_default();
        return Ok(match q.as_str() {
            "count" => json!(e.count),
            "emit" => json!(e.emit),
            "split" => json!(e.split),
            "dieAtCollision" => json!(e.die),
            "random" => json!(e.random),
            "spread" => json!(e.spread),
            "target" => json!(e.target),
            "proc" => json!(e.proc_name),
            "name" | "rename" | "" => json!(e.name),
            other => fail!("event: can't query {other}"),
        });
    }
    if a.bool("delete") {
        let e = find(&ev_name)?;
        let g = &mut s.graph;
        for (attr_name, _, _) in ATTRS {
            if let Some(k) = attr(g, id, attr_name) {
                let p = PlugRef { node: id, attr: k as i32, component: -1, element: e.index };
                if g.node(id).is_some_and(|n| n.elements(k).iter().any(|x| x.0 == e.index)) {
                    g.remove_element(&p)?;
                }
            }
        }
        return Ok(json!(e.name));
    }
    if a.bool("edit") {
        let e = find(&ev_name)?;
        apply_flags(&mut s.graph, id, e.index, a)?;
        if let Some(new) = a.get("rename").and_then(Value::as_str) {
            if list.iter().any(|x| x.name == new && x.index != e.index) {
                fail!("event: {obj} already has an event named {new}");
            }
            set_el(&mut s.graph, id, "eventName", e.index, PlugValue::String(new.to_string()))?;
            return Ok(json!(new));
        }
        return Ok(json!(e.name));
    }
    // A new event: the next free index, named as asked or after its index.
    let index = list.iter().map(|e| e.index + 1).max().unwrap_or(0);
    let name = name_flag.unwrap_or_else(|| (0..).map(|k| format!("event{k}")).find(|n| !list.iter().any(|e| e.name == *n)).unwrap());
    if list.iter().any(|e| e.name == name) {
        fail!("event: {obj} already has an event named {name}");
    }
    let g = &mut s.graph;
    set_el(g, id, "eventName", index, PlugValue::String(name.clone()))?;
    for (attr_name, _, _) in ATTRS.iter().skip(1) {
        let v = match *attr_name {
            "eventDie" | "eventRandom" => PlugValue::Bool(false),
            "eventSpread" => PlugValue::Float(0.0),
            "eventTarget" | "eventProc" => PlugValue::String(String::new()),
            _ => PlugValue::Int(0),
        };
        set_el(g, id, attr_name, index, v)?;
    }
    apply_flags(g, id, index, a)?;
    Ok(json!(name))
}

// ---------------------------------------------------------------------------------------------------------------------
// After each frame

/// Procedure calls the events asked for, for the host to run: (procedure, object, particle id, object hit).
static CALLS: Mutex<Vec<(String, String, i64, String)>> = Mutex::new(Vec::new());

/// The procedure calls waiting, for the host to run.
pub fn take_calls() -> Vec<(String, String, i64, String)> {
    std::mem::take(&mut *CALLS.lock().unwrap_or_else(|e| e.into_inner()))
}

/// A simple random number in [0, 1) from a seed.
fn rand01(seed: u64) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(0x6A09_E667_F3BC_C909);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// A direction off `dir` by up to `spread` of 90 degrees.
fn spread_dir(dir: DVec3, spread: f64, seed: u64) -> DVec3 {
    let len = dir.length();
    if spread <= 0.0 || len < 1e-12 {
        return dir;
    }
    let d = dir / len;
    let up = if d.y.abs() < 0.9 { DVec3::Y } else { DVec3::X };
    let (u, v) = (d.cross(up).normalize(), d.cross(d.cross(up).normalize()));
    let angle = spread * std::f64::consts::FRAC_PI_2 * rand01(seed);
    let around = std::f64::consts::TAU * rand01(seed ^ 0x5555);
    (d * angle.cos() + (u * around.cos() + v * around.sin()) * angle.sin()) * len
}

/// What a collider stands for to a script: the object whose mesh it takes, or itself.
fn hit_object(g: &Graph, collider: NodeId) -> String {
    let mesh = attr(g, collider, "inputMesh").and_then(|i| g.source(&PlugRef::new(collider, i as i32))).map(|p| p.node);
    match mesh {
        Some(m) => g.path(crate::dag::transform_of(g, m)),
        None => g.path(collider),
    }
}

/// After a solved frame: counts each particle's collision and works the events on it.
pub(crate) fn after_step(g: &mut Graph, st: &mut crate::dyn_solve::State, mpu: f64) {
    let bodies: Vec<crate::dyn_solve::BodyRef> = st.bodies.iter().filter(|b| b.kind == crate::dyn_solve::Kind::Particles).cloned().collect();
    let with_events: Vec<(crate::dyn_solve::BodyRef, Vec<Event>)> = bodies.into_iter().map(|b| (b.clone(), events(g, b.node))).filter(|(_, e)| !e.is_empty()).collect();
    if with_events.is_empty() {
        return;
    }
    let Some(solver) = st.solver.as_mut() else { return };
    let names: HashMap<usize, NodeId> = st.bodies.iter().map(|b| (b.index, b.node)).collect();
    let frame = solver.frame;
    let mut born: Vec<(usize, Vec<(DVec3, DVec3)>)> = Vec::new();
    for (b, evs) in with_events {
        let Some(body) = solver.bodies.get(b.index) else { continue };
        let (start, count) = (body.start, body.count);
        let obj = g.path(b.node);
        for i in start..start + count {
            let hit = solver.frame_hits.get(i).copied().unwrap_or(0);
            if hit == 0 {
                continue;
            }
            let pid = solver.pt.id[i];
            let n = {
                let c = st.collisions.entry((b.node, pid)).or_insert(0);
                *c += 1;
                *c as i64
            };
            let hit_name = if hit == aztek_dyn::solver::HIT_GROUND { "groundPlane".to_string() } else { names.get(&((hit - 1) as usize)).map(|&id| hit_object(g, id)).unwrap_or_default() };
            let (x, v) = (solver.pt.x[i], solver.pt.v[i]);
            for e in &evs {
                if e.count != 0 && e.count != n {
                    continue;
                }
                let target = if e.target.is_empty() { Some(b.index) } else { particles_of(g, &e.target).ok().and_then(|t| st.bodies.iter().find(|x| x.node == t).map(|x| x.index)) };
                let seed = pid.wrapping_mul(31).wrapping_add(frame).wrapping_add(e.index as u64 * 7919);
                let how_many = |k: i64, salt: u64| if e.random && k > 1 { 1 + (rand01(seed ^ salt) * k as f64) as i64 } else { k };
                // Emit first, then split, as the reference does.
                if let Some(t) = target {
                    let mut pts = Vec::new();
                    for k in 0..how_many(e.emit, 1) {
                        pts.push((x, spread_dir(v, e.spread, seed.wrapping_add(k as u64 * 101))));
                    }
                    for k in 0..how_many(e.split, 2) {
                        pts.push((x, spread_dir(v, e.spread, seed.wrapping_add(k as u64 * 211 + 17))));
                    }
                    if !pts.is_empty() {
                        born.push((t, pts));
                    }
                }
                // A particle that splits leaves its object; one told to die does.
                if e.die || e.split > 0 {
                    solver.doomed.insert(pid);
                }
                if !e.proc_name.is_empty() {
                    CALLS.lock().unwrap_or_else(|e| e.into_inner()).push((e.proc_name.clone(), obj.clone(), pid as i64, hit_name.clone()));
                }
            }
        }
    }
    let _ = mpu;
    for (t, pts) in born {
        solver.add_particles_now(t, &pts);
    }
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        crate::system_cmds::q_flags(Cmd::new(
            "event",
            "Collision events on particles: each collision (or the -ct'th) of a particle emits -em particles into -t a target \
             (or its own object), splits it into -spl, kills it (-die), or calls -pr a procedure (object, id, objectHit). \
             Returns the event's name; -ls lists events, -d deletes one, -e -n edits one (-re renames it), -q -n asks.",
            event,
        ))
        .positional("object")
        .flag(f("count", "ct", I, "The collision it acts on, for each particle; 0 (the default) every one."))
        .flag(f("delete", "d", B, "Delete the event."))
        .flag(f("dieAtCollision", "die", A, "The particle dies at the collision -count names (the first, without it)."))
        .flag(f("emit", "em", I, "Emit this many new particles into the target; the colliding one lives on."))
        .flag(f("list", "ls", B, "The events of the object, or object and event pairs of every particle object."))
        .flag(f("name", "n", S, "The event's name: a new one's, or which one to edit, ask about or delete."))
        .flag(f("proc", "pr", S, "A global procedure called at each collision: proc(string obj, int id, string objHit)."))
        .flag(f("random", "r", A, "With -split or -emit: a random number of particles from 1 to that number."))
        .flag(f("rename", "re", S, "A new name for the event being edited."))
        .flag(f("select", "s", B, "Obsolete: use -name."))
        .flag(f("split", "spl", I, "Split the particle into this many new ones in the target."))
        .flag(f("spread", "sp", Fl, "How far new particles stray from the rebound direction, as a fraction (0 to 1) of 90 degrees."))
        .flag(f("target", "t", S, "The particle object new particles go into (the colliding one's own by default).")),
    );
    let _ = asked;
}
