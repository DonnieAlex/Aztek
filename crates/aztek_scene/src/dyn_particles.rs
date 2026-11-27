//! Particle, emitter, field and cache commands, under the old systems' names so their scripts run: `particle`,
//! `nParticle`, `emitter`, `gravity`, `air`, `drag`, `newton`, `radial`, `turbulence`, `uniform`, `vortex`,
//! `volumeAxis`, `connectDynamic`, `emit`, `getParticleAttr`, `setParticleAttr`, `dynExpression`; and `dynCache` for disk
//! caches.

use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use glam::DVec3;
use serde_json::{json, Value};

use crate::deform::{f, Cmd};
use crate::dyn_cmds::{attr_plug, connect, connect_next, join, make_shape, q, resolve, set, solver_for, solver_of};
use crate::dynamics::{self as dy};
use crate::graph::Graph;
use crate::plug::{NodeId, PlugValue};
use crate::session::Session;

fn particles_named(g: &Graph, name: &str) -> CmdResult<NodeId> {
    let base = name.split('.').next().unwrap_or(name);
    resolve(g, base, &[dy::PARTICLES]).ok_or_else(|| aztek_ops::CommandError::new(format!("{name} is not particles")))
}

/// `particle1.pt[2:4]` -> [2, 3, 4]; None for the whole object.
fn particle_ids(name: &str) -> Option<Vec<usize>> {
    let at = name.find(".pt[")?;
    let inner = name[at + 4..].trim_end_matches(']');
    let mut out = Vec::new();
    for part in inner.split(',') {
        match part.split_once(':') {
            Some((a, b)) => {
                let (a, b): (usize, usize) = (a.trim().parse().ok()?, b.trim().parse().ok()?);
                out.extend(a..=b);
            }
            None => out.push(part.trim().parse().ok()?),
        }
    }
    Some(out)
}

fn particle(s: &mut Session, a: &Args) -> CmdResult {
    make_particles(s, a, "particle1")
}

/// nParticle names its particles as the reference does: nParticle1, nParticleShape1.
fn n_particle(s: &mut Session, a: &Args) -> CmdResult {
    make_particles(s, a, "nParticle1")
}

/// The place in the particle array a call names: -order, or the particle of -particleId.
fn particle_index(st: &dy::ParticleState, a: &Args) -> Option<usize> {
    if let Some(o) = a.get("order").and_then(Value::as_i64) {
        return (o >= 0 && (o as usize) < st.position.len()).then_some(o as usize);
    }
    let id = a.get("particleId").and_then(Value::as_i64).unwrap_or(-1);
    st.id.iter().position(|&x| x as i64 == id).or((id < 0 && !st.position.is_empty()).then_some(0))
}

fn make_particles(s: &mut Session, a: &Args, default_name: &str) -> CmdResult {
    let cmd = if default_name.starts_with('n') { "nParticle" } else { "particle" };
    let names = a.named();
    let names = if names.is_empty() && (a.bool("query") || a.bool("edit")) { s.graph.selection().iter().map(|&i| s.graph.path(i)).collect() } else { names };
    let g = &mut s.graph;
    if a.bool("query") {
        let Some(name) = names.into_iter().next() else { fail!("{cmd}: name the particles") };
        let p = particles_named(g, &name)?;
        g.evaluate();
        let st = dy::particle_state(g, p).unwrap_or_default();
        if crate::skin::asked(a, "count") {
            return Ok(json!(st.position.len()));
        }
        if a.get("attribute").is_some_and(Value::is_string) && !crate::skin::asked(a, "dynamicAttrList") {
            let at = a.string("attribute", "");
            let Some(i) = particle_index(&st, a) else { fail!("{cmd}: no such particle") };
            return Ok(particle_value(&st, &at, i));
        }
        let ty = g.node(p).unwrap().type_arc().clone();
        if crate::skin::asked(a, "dynamicAttrList") {
            return Ok(json!(ty.attrs.iter().filter(|x| x.flags & crate::plug::flags::USER != 0).map(|x| x.name.clone()).collect::<Vec<_>>()));
        }
        if crate::skin::asked(a, "perParticleDouble") {
            return Ok(json!(PER_PARTICLE_DOUBLE));
        }
        if crate::skin::asked(a, "perParticleVector") {
            return Ok(json!(PER_PARTICLE_VECTOR));
        }
        if crate::skin::asked(a, "conserve") {
            return Ok(json!(crate::dyn_solve::num(g, p, "conserve", 1.0)));
        }
        if crate::skin::asked(a, "inherit") {
            return Ok(json!(crate::dyn_solve::num(g, p, "inheritFactor", 0.0)));
        }
        if crate::skin::asked(a, "cache") {
            return Ok(json!(crate::dyn_solve::num(g, p, "cacheData", 1.0) != 0.0));
        }
        if crate::skin::asked(a, "name") {
            return Ok(json!(g.path(crate::dag::transform_of(g, p))));
        }
        if crate::skin::asked(a, "shapeName") {
            return Ok(json!(g.node(p).unwrap().name()));
        }
        fail!("{cmd}: query what?");
    }
    if a.bool("edit") {
        let Some(name) = names.into_iter().next() else { fail!("{cmd}: name the particles") };
        let p = particles_named(g, &name)?;
        // A per particle value: -attribute with -particleId or -order, and -floatValue or -vectorValue.
        if a.has("attribute") {
            let at = a.string("attribute", "");
            g.evaluate();
            let st = dy::particle_state(g, p).unwrap_or_default();
            let Some(i) = particle_index(&st, a) else { fail!("{cmd}: no such particle") };
            let value = match (a.vec3("vectorValue"), a.get("floatValue").and_then(Value::as_f64)) {
                (Some(v), _) => DVec3::from(v),
                (None, Some(x)) => DVec3::splat(x),
                _ => fail!("{cmd}: give -floatValue or -vectorValue"),
            };
            write_particles(g, p, &at, value, Some(vec![i]), cmd)?;
            return Ok(Value::Null);
        }
        if let Some(c) = a.get("conserve").and_then(Value::as_f64) {
            set(g, p, "conserve", c.clamp(0.0, 1.0))?;
        }
        if let Some(c) = a.get("inherit").and_then(Value::as_f64) {
            set(g, p, "inheritFactor", c.clamp(0.0, 1.0))?;
        }
        if a.has("cache") {
            set(g, p, "cacheData", a.bool("cache"))?;
        }
        if let Some(n) = a.get("shapeName").and_then(Value::as_str) {
            g.rename(p, n)?;
        }
        if let Some(n) = a.get("name").and_then(Value::as_str) {
            let xf = crate::dag::transform_of(g, p);
            g.rename(xf, n)?;
        }
        if a.bool("deleteCache") {
            crate::dyn_solve::clear_cache(g, p);
        }
        return Ok(Value::Null);
    }
    if a.bool("deleteCache") {
        // On its own it acts on the particles named.
        if let Some(name) = names.first() {
            let p = particles_named(g, name)?;
            crate::dyn_solve::clear_cache(g, p);
            return Ok(Value::Null);
        }
    }
    let mut pts: Vec<f64> = Vec::new();
    if let Some(Value::Array(list)) = a.get("position") {
        for v in list {
            match v {
                Value::Array(xyz) if xyz.len() == 3 => pts.extend(xyz.iter().map(|x| x.as_f64().unwrap_or(0.0))),
                Value::Number(n) => pts.push(n.as_f64().unwrap_or(0.0)),
                _ => {}
            }
        }
    }
    // A grid: -ll lower left, -ur upper right, -grs spacing.
    if let (Some(lo), Some(hi)) = (a.vec3("lowerLeft"), a.vec3("upperRight")) {
        let step = a.float("gridSpacing", 1.0).max(1e-6);
        let (lo, hi) = (DVec3::from(lo), DVec3::from(hi));
        let n = ((hi - lo) / step).floor();
        for x in 0..=n.x as i64 {
            for y in 0..=n.y as i64 {
                for z in 0..=n.z as i64 {
                    let p = lo + DVec3::new(x as f64, y as f64, z as f64) * step;
                    pts.extend([p.x, p.y, p.z]);
                }
            }
        }
    }
    pts.extend(jitter_points(a));
    pts.truncate(pts.len() / 3 * 3);
    let solver = solver_for(g, a)?;
    let name = a.string("name", default_name);
    let (xf, shape) = make_shape(g, dy::PARTICLES, &name)?;
    if !pts.is_empty() {
        crate::deform::set_floats(g, shape, attr_plug(g, shape, "startPositions")?.attr as usize, pts)?;
    }
    if let Some(c) = a.get("conserve").and_then(Value::as_f64) {
        set(g, shape, "conserve", c.clamp(0.0, 1.0))?;
    }
    if let Some(c) = a.get("inherit").and_then(Value::as_f64) {
        set(g, shape, "inheritFactor", c.clamp(0.0, 1.0))?;
    }
    if a.has("cache") {
        set(g, shape, "cacheData", a.bool("cache"))?;
    }
    if let Some(n) = a.get("shapeName").and_then(Value::as_str) {
        g.rename(shape, n)?;
    }
    join(g, shape, solver)?;
    Ok(json!([g.path(xf), g.path(shape)]))
}

fn particle_value(st: &dy::ParticleState, at: &str, i: usize) -> Value {
    let v3 = |v: DVec3| json!([v.x, v.y, v.z]);
    match at {
        "position" | "worldPosition" => v3(st.position[i]),
        "velocity" | "worldVelocity" => v3(st.velocity[i]),
        "rgbPP" => v3(st.rgb[i]),
        "age" => json!(st.age[i]),
        "lifespanPP" => json!(st.lifespan[i]),
        "opacityPP" => json!(st.opacity[i]),
        "radiusPP" => json!(st.radius[i]),
        "particleId" | "id" => json!(st.id[i]),
        "mass" => json!(1.0),
        _ => Value::Null,
    }
}

/// getParticleAttr -at position -a true particle1: every particle's value; without -a, the mean and the spread
/// (largest minus smallest), as the old command gives.
fn get_particle_attr(s: &mut Session, a: &Args) -> CmdResult {
    // -object is the reference's older way of naming them; the selection when nothing is named.
    let named = a.named().into_iter().next().or_else(|| a.get("object").and_then(Value::as_str).map(str::to_string)).or_else(|| s.graph.selection().first().map(|&id| s.graph.path(id)));
    let Some(name) = named else { fail!("getParticleAttr: name particles") };
    let g = &mut s.graph;
    let p = particles_named(g, &name)?;
    g.evaluate();
    let st = dy::particle_state(g, p).unwrap_or_default();
    let at = a.string("attribute", "position");
    let ids: Vec<usize> = particle_ids(&name).unwrap_or_else(|| (0..st.position.len()).collect()).into_iter().filter(|&i| i < st.position.len()).collect();
    let values: Vec<Vec<f64>> = ids
        .iter()
        .map(|&i| {
            // Lengths and speeds read in the current unit.
            let k = if aztek_ops::units::user_call() && matches!(at.as_str(), "position" | "velocity" | "radiusPP") { 1.0 / aztek_ops::units::cm_per_unit() } else { 1.0 };
            match particle_value(&st, &at, i) {
                Value::Array(v) => v.iter().filter_map(Value::as_f64).map(|x| x * k).collect(),
                Value::Number(n) => vec![n.as_f64().unwrap_or(0.0) * k],
                _ => Vec::new(),
            }
        })
        .filter(|v| !v.is_empty())
        .collect();
    if a.bool("array") {
        return Ok(Value::Array(values.into_iter().flatten().map(|x| json!(x)).collect()));
    }
    let Some(width) = values.first().map(Vec::len) else { return Ok(json!([])) };
    let mut out = Vec::new();
    for k in 0..width {
        let col: Vec<f64> = values.iter().map(|v| v[k]).collect();
        out.push(col.iter().sum::<f64>() / col.len() as f64);
    }
    for k in 0..width {
        let col: Vec<f64> = values.iter().map(|v| v[k]).collect();
        out.push(col.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - col.iter().cloned().fold(f64::INFINITY, f64::min));
    }
    Ok(json!(out))
}

/// setParticleAttr -at velocity -vv 0 1 0 particle1.pt[3]: changes particles in the solver now.
fn set_particle_attr(s: &mut Session, a: &Args) -> CmdResult {
    // -object: that object alone, every particle of it, whatever is named or selected.
    let names: Vec<String> = match a.get("object").and_then(Value::as_str) {
        Some(o) => vec![o.to_string()],
        None => aztek_ops::objects(s, a),
    };
    if names.is_empty() {
        fail!("setParticleAttr: name or select particles");
    }
    let at = a.string("attribute", "position");
    let vector = matches!(at.as_str(), "position" | "velocity" | "rgbPP");
    // A float attribute given a vector takes its length; a vector one given a float takes it on each axis.
    let given = match a.vec3("vectorValue").map(DVec3::from) {
        Some(v) if !vector => DVec3::splat(v.length()),
        Some(v) => v,
        None => DVec3::splat(a.float("floatValue", 0.0)),
    };
    let rf = a.float("randomFloat", 0.0);
    let rv = a.vec3("randomVector").map(DVec3::from).unwrap_or(DVec3::ZERO);
    let relative = a.bool("relative");
    // xorshift64*: -1 to 1.
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15 ^ names.iter().map(|n| n.len() as u64).sum::<u64>();
    let mut unit = move || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        (state.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    let mut changed = 0;
    let g = &mut s.graph;
    for name in &names {
        let p = particles_named(g, name)?;
        let ids = if a.has("object") { None } else { particle_ids(name) };
        changed += write_particles_with(g, p, &at, ids, "setParticleAttr", &mut |now: DVec3| {
            // The random part: within -rf on every axis for a float, within -rv per axis for a vector.
            let jitter = if vector { DVec3::new(unit() * rv.x, unit() * rv.y, unit() * rv.z) + DVec3::splat(unit() * rf) } else { DVec3::splat(unit() * rf) };
            let v = given + jitter;
            if relative { now + v } else { v }
        })?;
    }
    Ok(json!(changed))
}

/// Sets a per particle value on the particles at these places in the array (all when None), in the solver now.
fn write_particles(g: &mut Graph, p: NodeId, at: &str, value: DVec3, ids: Option<Vec<usize>>, cmd: &str) -> CmdResult<usize> {
    write_particles_with(g, p, at, ids, cmd, &mut |_| value)
}

/// As write_particles, each particle's new value worked from its value now (in the scene's units).
fn write_particles_with(g: &mut Graph, p: NodeId, at: &str, ids: Option<Vec<usize>>, cmd: &str, value: &mut dyn FnMut(DVec3) -> DVec3) -> CmdResult<usize> {
    let name = g.path(p);
    let Some(sv) = solver_of(g, p) else { fail!("{cmd}: {name} is in no solver") };
    // The solver works in metres: lengths and speeds given in the scene's units go in as such.
    let mpu = crate::dyn_solve::metres(g, sv);
    let scale = if matches!(at, "position" | "velocity" | "radiusPP") { mpu } else { 1.0 };
    let n = g.node(sv).and_then(|n| n.behavior().as_any().downcast_ref::<dy::SolverNode>());
    let Some(n) = n else { fail!("{cmd}: no solver") };
    let mut st = n.state.lock().unwrap();
    let body = st.bodies.iter().find(|b| b.node == p).map(|b| b.index);
    let (Some(b), Some(solver)) = (body, st.solver.as_mut()) else { fail!("{cmd}: {name} has not been solved yet") };
    let range = solver.bodies[b].start..solver.bodies[b].start + solver.bodies[b].count;
    let mut changed = 0;
    for (k, i) in range.enumerate() {
        if ids.as_ref().is_some_and(|ids| !ids.contains(&k)) {
            continue;
        }
        let pt = &mut solver.pt;
        let now = match at {
            "position" => pt.x[i],
            "velocity" => pt.v[i],
            "rgbPP" => pt.rgb[i],
            "opacityPP" => DVec3::splat(pt.opacity[i]),
            "lifespanPP" => DVec3::splat(pt.life[i]),
            "radiusPP" => DVec3::splat(pt.radius[i]),
            other => fail!("{cmd}: {other} is not a particle attribute here"),
        } / scale;
        let v = value(now) * scale;
        match at {
            "position" => {
                pt.x[i] = v;
                pt.p[i] = v;
            }
            "velocity" => pt.v[i] = v,
            "rgbPP" => pt.rgb[i] = v,
            "opacityPP" => pt.opacity[i] = v.x,
            "lifespanPP" => pt.life[i] = v.x,
            _ => pt.radius[i] = v.x,
        }
        changed += 1;
    }
    let last = st.last;
    if let Some(l) = last {
        st.cache.retain(|&k, _| (k as f64) < l * 1000.0);
    }
    drop(st);
    Ok(changed)
}

/// One value given to emit: a number or a vector.
#[derive(Clone, Copy)]
enum Given {
    Float(f64),
    Vector(DVec3),
}

/// The per particle attributes emit can set here, and whether each is a vector.
const EMIT_ATTRS: &[(&str, bool)] = &[("position", true), ("velocity", true), ("rgbPP", true), ("opacityPP", false), ("lifespanPP", false), ("radiusPP", false), ("mass", false)];

/// emit -o particle1 -pos 1 1 1 -pos 2 2 2 -at velocity -vv 1 2 3: particles added to particles that exist, at the
/// positions given, one each. The values after an -attribute go to the new particles in turn; the rest take the last.
/// An -attribute given again picks up where it left off. Returns the new particles' ids.
fn emit(s: &mut Session, a: &Args) -> CmdResult {
    // The object is named with -object; the selection is not used.
    let Some(object) = a.get("object").and_then(Value::as_str).map(str::to_string) else { fail!("emit: name the particles with -object") };
    let g = &mut s.graph;
    let p = particles_named(g, &object)?;
    let Some(sv) = solver_of(g, p) else { fail!("emit: {object} is in no solver") };
    let positions = uses(a, "position", 3);
    let attrs: Vec<String> = match a.get("attribute") {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(list)) => list.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        _ => Vec::new(),
    };
    let floats: Vec<f64> = uses(a, "floatValue", 1).into_iter().map(|v| v[0]).collect();
    let vectors: Vec<DVec3> = uses(a, "vectorValue", 3).into_iter().map(|v| DVec3::new(v[0], v[1], v[2])).collect();
    // Each attribute's values in the order given. MEL passes the flags' order; without it (Python) one attribute
    // takes the vectors, then the numbers.
    let mut streams: Vec<(String, Vec<Given>)> = Vec::new();
    let order: Vec<String> = match a.get("flagOrder") {
        Some(Value::Array(list)) => list.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        _ => Vec::new(),
    };
    let stream = |streams: &mut Vec<(String, Vec<Given>)>, at: &str| -> usize {
        match streams.iter().position(|(n, _)| n == at) {
            Some(k) => k,
            None => {
                streams.push((at.to_string(), Vec::new()));
                streams.len() - 1
            }
        }
    };
    if !order.is_empty() {
        let (mut na, mut nf, mut nv) = (0, 0, 0);
        let mut current: Option<usize> = None;
        for flag in &order {
            match flag.as_str() {
                "attribute" => {
                    if let Some(at) = attrs.get(na) {
                        current = Some(stream(&mut streams, at));
                    }
                    na += 1;
                }
                "floatValue" | "vectorValue" => {
                    let given = if flag == "floatValue" {
                        nf += 1;
                        floats.get(nf - 1).map(|&x| Given::Float(x))
                    } else {
                        nv += 1;
                        vectors.get(nv - 1).map(|&v| Given::Vector(v))
                    };
                    let Some(k) = current else { fail!("emit: -{flag} needs an -attribute before it") };
                    if let Some(v) = given {
                        streams[k].1.push(v);
                    }
                }
                _ => {}
            }
        }
    } else if !floats.is_empty() || !vectors.is_empty() {
        if attrs.len() != 1 {
            fail!("emit: without the flags' order, give one -attribute with its values");
        }
        let k = stream(&mut streams, &attrs[0]);
        streams[k].1.extend(vectors.iter().map(|&v| Given::Vector(v)).chain(floats.iter().map(|&x| Given::Float(x))));
    } else {
        for at in &attrs {
            stream(&mut streams, at);
        }
    }
    for (at, _) in &streams {
        if !EMIT_ATTRS.iter().any(|(n, _)| n == at) {
            fail!("emit: {at} is not a per particle attribute of {object} that emit can set");
        }
    }
    // Lengths and speeds come in the current unit from a script; the solver works in metres.
    let mpu = crate::dyn_solve::metres(g, sv);
    let user = if aztek_ops::units::user_call() { aztek_ops::units::cm_per_unit() } else { 1.0 };
    let list: Vec<crate::dyn_solve::Emitted> = positions
        .iter()
        .enumerate()
        .map(|(k, pos)| {
            let values = streams
                .iter()
                .filter_map(|(at, vals)| {
                    let v = vals.get(k).or(vals.last())?;
                    let vector = EMIT_ATTRS.iter().any(|(n, vec)| n == at && *vec);
                    // A number for a vector goes on each axis; a vector for a number gives its length.
                    let v = match (*v, vector) {
                        (Given::Float(x), _) => DVec3::splat(x),
                        (Given::Vector(v), true) => v,
                        (Given::Vector(v), false) => DVec3::splat(v.length()),
                    };
                    let scale = if matches!(at.as_str(), "position" | "velocity" | "radiusPP") { user * mpu } else { 1.0 };
                    Some((at.clone(), v * scale))
                })
                .collect();
            crate::dyn_solve::Emitted { body: p, position: DVec3::new(pos[0], pos[1], pos[2]) * mpu, values }
        })
        .collect();
    if list.is_empty() {
        return Ok(json!([]));
    }
    match crate::dyn_solve::emit(g, p, list) {
        Ok(Some(ids)) => Ok(json!(ids)),
        Ok(None) => {
            log::warn!("emit: {object} takes particles only from its start frame on");
            Ok(json!([]))
        }
        Err(e) => fail!("emit: {e}"),
    }
}

/// Per particle attributes the reference lists for -perParticleDouble and -perParticleVector, those this solver keeps.
const PER_PARTICLE_DOUBLE: &[&str] = &["mass", "age", "lifespanPP", "opacityPP", "radiusPP"];
const PER_PARTICLE_VECTOR: &[&str] = &["position", "velocity", "rgbPP"];

/// The values of a multi-use flag, one per use.
fn uses(a: &Args, flag: &str, width: usize) -> Vec<Vec<f64>> {
    let mut flat: Vec<f64> = Vec::new();
    let mut out = Vec::new();
    let mut walk = |v: &Value| match v {
        Value::Array(x) if width > 1 && x.len() == width && x.iter().all(Value::is_number) => out.push(x.iter().filter_map(Value::as_f64).collect()),
        Value::Number(n) => flat.push(n.as_f64().unwrap_or(0.0)),
        _ => {}
    };
    match a.get(flag) {
        Some(Value::Array(list)) if width > 1 && list.len() == width && list.iter().all(Value::is_number) => return vec![list.iter().filter_map(Value::as_f64).collect()],
        Some(Value::Array(list)) => list.iter().for_each(&mut walk),
        Some(v) => walk(v),
        None => {}
    }
    out.extend(flat.chunks_exact(width.max(1)).map(|c| c.to_vec()));
    out
}

/// -jitterBasePoint, -jitterRadius, -numJitters: for each base point, that many particles at random within the radius.
/// The radius and count pair with the base points in order; when there are fewer, the last is used again.
fn jitter_points(a: &Args) -> Vec<f64> {
    let bases = uses(a, "jitterBasePoint", 3);
    let radii: Vec<f64> = uses(a, "jitterRadius", 1).into_iter().map(|v| v[0]).collect();
    let counts: Vec<f64> = uses(a, "numJitters", 1).into_iter().map(|v| v[0]).collect();
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    for (k, b) in bases.iter().enumerate() {
        let r = radii.get(k).or(radii.last()).copied().unwrap_or(0.0);
        let n = counts.get(k).or(counts.last()).copied().unwrap_or(1.0).max(0.0) as usize;
        for _ in 0..n {
            // Uniform in the ball: a direction, and a distance by the cube root.
            let (u, v, w) = (next() * 2.0 - 1.0, next() * std::f64::consts::TAU, next().cbrt() * r);
            let s = (1.0 - u * u).max(0.0).sqrt();
            out.extend([b[0] + w * s * v.cos(), b[1] + w * s * v.sin(), b[2] + w * u]);
        }
    }
    out
}

fn emitter(s: &mut Session, a: &Args) -> CmdResult {
    let g = &mut s.graph;
    if a.bool("query") || a.bool("edit") {
        let Some(name) = a.named().into_iter().next() else { fail!("emitter: name an emitter") };
        let Some(e) = resolve(g, &name, &[dy::EMITTER]) else { fail!("emitter: {name} is not an emitter") };
        if a.bool("query") {
            for (flag, at) in [("rate", "rate"), ("speed", "speed"), ("speedRandom", "speedRandom"), ("spread", "spread")] {
                if a.has(flag) {
                    return Ok(json!(crate::dyn_solve::num(g, e, at, 0.0)));
                }
            }
            if a.has("type") {
                return Ok(json!(dy::EMITTER_KINDS[crate::dyn_solve::num(g, e, "emitterType", 1.0) as usize]));
            }
            fail!("emitter: query what?");
        }
        emitter_flags(g, e, a)?;
        return Ok(json!(g.path(e)));
    }
    // A mesh named or selected: emits off its surface.
    let named = a.named();
    let source = named.first().and_then(|n| g.find(n).ok().flatten()).or_else(|| g.selection().first().copied()).and_then(|id| crate::components::as_mesh(g, id));
    let name = a.string("name", "emitter1");
    let (xf, shape) = make_shape(g, dy::EMITTER, &name)?;
    if let Some(p) = a.vec3("position") {
        let tp = attr_plug(g, xf, "translate")?;
        g.set_value(tp, PlugValue::Vec3(DVec3::from(p)))?;
    }
    if let Some(m) = source {
        set(g, shape, "emitterType", 2i64)?;
        let out = g.node(m).unwrap().ty().attr_index("outMesh").unwrap();
        let input = attr_plug(g, shape, "inputMesh")?;
        g.connect(&crate::plug::PlugRef::new(m, out as i32), &input, true)?;
        connect(g, m, "worldMatrix", shape, "inputMatrix")?;
    }
    emitter_flags(g, shape, a)?;
    // As the reference's command, only the emitter: particles to emit into are made apart (particle, nParticle) and
    // joined with connectDynamic -em, or named here with -particles. The Create Emitter menu does all three (#96).
    if a.has("particles") {
        let particles = particles_named(g, &a.string("particles", ""))?;
        connect_next(g, shape, "message", particles, "newParticles")?;
        if let Some(sv) = solver_of(g, particles) {
            crate::dyn_solve::invalidate(g, sv);
        }
    }
    // An emitter on an object comes back after it, as the reference returns them.
    Ok(match source {
        Some(m) => json!([g.path(crate::dag::transform_of(g, m)), g.path(xf)]),
        None => json!([g.path(xf)]),
    })
}

fn emitter_flags(g: &mut Graph, e: NodeId, a: &Args) -> CmdResult<()> {
    if a.has("type") {
        let t = a.string("type", "omni").to_lowercase();
        let k = match t.as_str() {
            "dir" | "direction" | "directional" => 0,
            "omni" => 1,
            "surf" | "surface" => 2,
            "curv" | "curve" => 3,
            "vol" | "volume" => 4,
            other => fail!("emitter: no type {other}"),
        };
        set(g, e, "emitterType", k as i64)?;
    }
    for (flag, at) in [("rate", "rate"), ("speed", "speed"), ("speedRandom", "speedRandom"), ("spread", "spread")] {
        if a.has(flag) {
            set(g, e, at, a.float(flag, 0.0))?;
        }
    }
    let d = DVec3::new(a.float("directionX", f64::NAN), a.float("directionY", f64::NAN), a.float("directionZ", f64::NAN));
    if d.is_finite() || a.has("directionX") || a.has("directionY") || a.has("directionZ") {
        let old = crate::dyn_solve::num(g, e, "emitterType", 1.0);
        let _ = old;
        let cur = g.node(e).unwrap().vec3(g.node(e).unwrap().ty().attr_index("direction").unwrap());
        let v = DVec3::new(if d.x.is_finite() { d.x } else { cur.x }, if d.y.is_finite() { d.y } else { cur.y }, if d.z.is_finite() { d.z } else { cur.z });
        set(g, e, "direction", PlugValue::Vec3(v))?;
    }
    if a.has("volumeShape") {
        set(g, e, "volumeShape", if a.string("volumeShape", "cube") == "sphere" { 1i64 } else { 0 })?;
    }
    Ok(())
}

const FIELD_COMMANDS: &[(&str, &str)] = &[("gravity", "gravityField"), ("air", "airField"), ("drag", "dragField"), ("newton", "newtonField"), ("radial", "radialField"), ("turbulence", "turbulenceField"), ("uniform", "uniformField"), ("vortex", "vortexField"), ("volumeAxis", "volumeAxisField")];

fn field_command(s: &mut Session, a: &Args, kind: usize) -> CmdResult {
    let g = &mut s.graph;
    if a.bool("query") || a.bool("edit") {
        let Some(name) = a.named().into_iter().next() else { fail!("{}: name a field", FIELD_COMMANDS[kind].0) };
        let Some(fl) = resolve(g, &name, &[dy::FIELD]) else { fail!("{name} is not a field") };
        if a.bool("query") {
            // From a script the flag asked about comes as queried, its value left out.
            let asked = |flag: &str| a.has(flag) || a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == flag));
            for (flag, at) in FIELD_FLAGS {
                if asked(flag) {
                    return Ok(json!(crate::dyn_solve::num(g, fl, at, 0.0)));
                }
            }
            // A vortex's axis is the field's direction, one component a flag.
            let dir = g.node(fl).unwrap().vec3(g.node(fl).unwrap().ty().attr_index("direction").unwrap());
            for (flag, v) in [("axisX", dir.x), ("axisY", dir.y), ("axisZ", dir.z)] {
                if kind == VORTEX && asked(flag) {
                    return Ok(json!(v));
                }
            }
            fail!("{}: query what?", FIELD_COMMANDS[kind].0);
        }
        field_flags(g, fl, a, kind)?;
        return Ok(json!(g.path(fl)));
    }
    let name = a.string("name", &format!("{}1", FIELD_COMMANDS[kind].1));
    let (xf, shape) = make_shape(g, dy::FIELD, &name)?;
    set(g, shape, "fieldType", kind as i64)?;
    let def = aztek_dyn::Field::new(match kind {
        1 => aztek_dyn::FieldKind::Air,
        2 => aztek_dyn::FieldKind::Drag,
        3 => aztek_dyn::FieldKind::Newton,
        4 => aztek_dyn::FieldKind::Radial,
        5 => aztek_dyn::FieldKind::Turbulence,
        6 => aztek_dyn::FieldKind::Uniform,
        7 => aztek_dyn::FieldKind::Vortex,
        8 => aztek_dyn::FieldKind::VolumeAxis,
        _ => aztek_dyn::FieldKind::Gravity,
    });
    set(g, shape, "magnitude", def.magnitude)?;
    set(g, shape, "attenuation", def.attenuation)?;
    set(g, shape, "useMaxDistance", def.use_max_distance)?;
    set(g, shape, "direction", PlugValue::Vec3(def.direction))?;
    if let Some(p) = a.vec3("position") {
        let tp = attr_plug(g, xf, "translate")?;
        g.set_value(tp, PlugValue::Vec3(DVec3::from(p)))?;
    }
    field_flags(g, shape, a, kind)?;
    // Acts on the bodies selected.
    let named = a.named();
    let targets: Vec<NodeId> = if named.is_empty() { g.selection().to_vec() } else { named.iter().filter_map(|n| g.find(n).ok().flatten()).collect() };
    for t in targets {
        if let Some(b) = resolve(g, &g.path(t), &[dy::CLOTH, dy::PARTICLES, dy::RIGID]) {
            connect_next(g, shape, "message", b, "inputForce")?;
        }
    }
    Ok(json!([g.path(xf)]))
}

const FIELD_FLAGS: &[(&str, &str)] = &[("magnitude", "magnitude"), ("attenuation", "attenuation"), ("maxDistance", "maxDistance"), ("speed", "speed"), ("inheritVelocity", "inheritVelocity"), ("frequency", "frequency"), ("minDistance", "minDistance")];

/// The vortex's place in FIELD_COMMANDS.
const VORTEX: usize = 7;

fn field_flags(g: &mut Graph, fl: NodeId, a: &Args, kind: usize) -> CmdResult<()> {
    for (flag, at) in FIELD_FLAGS {
        if a.has(flag) {
            set(g, fl, at, a.float(flag, 0.0))?;
            if *flag == "maxDistance" {
                set(g, fl, "useMaxDistance", true)?;
            }
        }
    }
    if a.has("useMaxDistance") {
        set(g, fl, "useMaxDistance", a.bool("useMaxDistance"))?;
    }
    if a.has("directionX") || a.has("directionY") || a.has("directionZ") {
        let cur = g.node(fl).unwrap().vec3(g.node(fl).unwrap().ty().attr_index("direction").unwrap());
        let v = DVec3::new(a.float("directionX", cur.x), a.float("directionY", cur.y), a.float("directionZ", cur.z));
        set(g, fl, "direction", PlugValue::Vec3(v))?;
    }
    // vortex -ax -ay -az: the axis it turns about, kept as the direction.
    if kind == VORTEX && (a.has("axisX") || a.has("axisY") || a.has("axisZ")) {
        let cur = g.node(fl).unwrap().vec3(g.node(fl).unwrap().ty().attr_index("direction").unwrap());
        let v = DVec3::new(a.float("axisX", cur.x), a.float("axisY", cur.y), a.float("axisZ", cur.z));
        set(g, fl, "direction", PlugValue::Vec3(v))?;
    }
    Ok(())
}

/// connectDynamic -f gravity1 -em emitter1 -c collider1 particles1: what acts on the bodies named. -d disconnects.
fn connect_dynamic(s: &mut Session, a: &Args) -> CmdResult {
    if a.has("addScriptHandler") || a.has("removeScriptHandler") {
        // The application keeps the handlers, as they run Python; a scene on its own has none.
        fail!("connectDynamic: script handlers are kept by the application");
    }
    let g = &mut s.graph;
    let list = |a: &Args, flag: &str| -> Vec<String> {
        match a.get(flag) {
            Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
            Some(Value::String(x)) => vec![x.clone()],
            _ => Vec::new(),
        }
    };
    let delete = a.bool("delete");
    let mut done = Vec::new();
    for name in a.named() {
        let Some(b) = resolve(g, &name, &[dy::CLOTH, dy::PARTICLES, dy::RIGID]) else { fail!("connectDynamic: {name} is not a dynamic body") };
        // A body from a system with no solver node joins one.
        if solver_of(g, b).is_none() {
            let sv = solver_for(g, a)?;
            join(g, b, sv)?;
        }
        for fname in list(a, "fields") {
            let Some(fl) = resolve(g, &fname, &[dy::FIELD]) else { fail!("connectDynamic: {fname} is not a field") };
            if delete {
                let arr = attr_plug(g, b, "inputForce")?;
                for e in g.elements(&arr) {
                    if g.source(&e).is_some_and(|s| s.node == fl) {
                        g.disconnect(&e);
                    }
                }
            } else {
                connect_next(g, fl, "message", b, "inputForce")?;
            }
        }
        for ename in list(a, "emitters") {
            let Some(em) = resolve(g, &ename, &[dy::EMITTER]) else {
                // Say what it is when it is something: a particle object given as an emitter is the usual slip (#96).
                let found = g.find(&ename).ok().flatten();
                let what = found.map(|id| {
                    // A transform says what its shape is.
                    let shape = crate::dag::shapes(g, id, false).into_iter().next();
                    match shape.and_then(|sh| g.node(sh)) {
                        Some(sh) if g.node(id).is_some_and(|n| n.ty().name == "transform") => format!("transform of a {} ({})", sh.ty().name, sh.name()),
                        _ => g.node(id).map(|n| n.ty().name.clone()).unwrap_or_default(),
                    }
                });
                let emitters: Vec<String> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().name == dy::EMITTER)).map(|id| g.path(crate::dag::transform_of(g, id))).take(3).collect();
                let hint = if emitters.is_empty() { "there are no emitters; make one with emitter".to_string() } else { format!("emitters here: {}", emitters.join(", ")) };
                match what {
                    Some(t) => fail!("connectDynamic: -emitters takes an emitter; {ename} is a {t} ({hint})"),
                    None => fail!("connectDynamic: -emitters {ename}: no such node ({hint})"),
                }
            };
            let arr = attr_plug(g, b, "newParticles")?;
            let already = g.elements(&arr).iter().any(|e| g.source(e).is_some_and(|s| s.node == em));
            if already && !delete {
                log::warn!("connectDynamic: {ename} already emits into {name}");
                continue;
            }
            if delete {
                let arr = attr_plug(g, b, "newParticles")?;
                for e in g.elements(&arr) {
                    if g.source(&e).is_some_and(|s| s.node == em) {
                        g.disconnect(&e);
                    }
                }
            } else {
                connect_next(g, em, "message", b, "newParticles")?;
            }
        }
        for cname in list(a, "collisions") {
            // A collider in the same solver collides with everything in it already; joining it to the body's solver is
            // all there is to do.
            let Some(sv) = solver_of(g, b) else { continue };
            let c = match resolve(g, &cname, &[dy::COLLIDER, dy::CLOTH, dy::RIGID, dy::PARTICLES]) {
                Some(c) => c,
                None => {
                    let made = s_collider(g, &cname)?;
                    made
                }
            };
            if !delete && solver_of(g, c) != Some(sv) {
                join(g, c, sv)?;
            }
        }
        if let Some(sv) = solver_of(g, b) {
            crate::dyn_solve::invalidate(g, sv);
        }
        done.push(json!(g.path(b)));
    }
    Ok(Value::Array(done))
}

fn s_collider(g: &mut Graph, name: &str) -> CmdResult<NodeId> {
    let Some(id) = g.find(name)? else { fail!("connectDynamic: no object named {name}") };
    let Some(m) = crate::components::as_mesh(g, id) else { fail!("connectDynamic: {name} is not a mesh") };
    let t = crate::node::types::find(dy::COLLIDER).unwrap();
    let c = g.create_node(&t, "collider1", crate::plug::NO_NODE, true)?;
    let out = g.node(m).unwrap().ty().attr_index("outMesh").unwrap();
    let input = attr_plug(g, c, "inputMesh")?;
    g.connect(&crate::plug::PlugRef::new(m, out as i32), &input, true)?;
    connect(g, m, "worldMatrix", c, "inputMatrix")?;
    Ok(c)
}

/// dynExpression -s "rgbPP = <<1,0,0>>;" -c particlesShape1: kept on the particles; -q -s reads it back.
fn dyn_expression(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.named().into_iter().next() else { fail!("dynExpression: name particles") };
    let g = &mut s.graph;
    let p = particles_named(g, &name)?;
    let at = if a.bool("creation") { "creationExpression" } else { "runtimeExpression" };
    if a.bool("query") {
        return Ok(json!(g.node(p).unwrap().value(g.node(p).unwrap().ty().attr_index(at).unwrap()).as_str()));
    }
    set(g, p, at, PlugValue::String(a.string("string", "")))?;
    Ok(json!(g.path(p)))
}

/// Disk caches: -create solves the bodies from the start frame to the end and writes every frame; -append adds frames
/// from the end of the cache to the current frame; -replace writes a new cache over the old; -delete removes it.
fn dyn_cache(s: &mut Session, a: &Args) -> CmdResult {
    let mut bodies = Vec::new();
    for n in a.named() {
        match resolve(&s.graph, &n, &[dy::CLOTH]) {
            Some(b) => bodies.push(b),
            None => fail!("dynCache: {n} is not cloth"),
        }
    }
    if bodies.is_empty() {
        for id in s.graph.selection().to_vec() {
            if let Some(b) = resolve(&s.graph, &s.graph.path(id), &[dy::CLOTH]) {
                bodies.push(b);
            }
        }
    }
    if bodies.is_empty() {
        fail!("dynCache: select cloth");
    }
    let g = &mut s.graph;
    if a.bool("delete") {
        for &b in &bodies {
            let path = crate::dyn_solve::num(g, b, "", 0.0);
            let _ = path;
            let file = g.node(b).unwrap().value(g.node(b).unwrap().ty().attr_index("diskCache").unwrap()).as_str().to_string();
            if !file.is_empty() && a.bool("deleteFile") {
                let _ = std::fs::remove_file(&file);
            }
            set(g, b, "diskCache", PlugValue::String(String::new()))?;
        }
        crate::dyn_solve::forget_disk(g);
        return Ok(json!(bodies.iter().map(|&b| g.path(b)).collect::<Vec<_>>()));
    }
    let (lo, hi) = crate::dyn_cmds::playback_range();
    let start = a.float("startTime", lo);
    let end = a.float("endTime", hi);
    let append = a.bool("append");
    let file_for = |g: &Graph, b: NodeId| -> String {
        if a.has("file") {
            return a.string("file", "");
        }
        let dir = std::env::temp_dir();
        dir.join(format!("{}.azdc", g.node(b).unwrap().name())).to_string_lossy().into_owned()
    };
    let files: Vec<(NodeId, String)> = bodies.iter().map(|&b| (b, file_for(g, b))).collect();
    // Solving to write: the bodies play live while the cache is made.
    for (b, _) in &files {
        set(g, *b, "diskCache", PlugValue::String(String::new()))?;
    }
    crate::dyn_solve::forget_disk(g);
    if !append {
        for (_, f) in &files {
            let _ = std::fs::remove_file(f);
        }
    }
    let first = if append { crate::anim::frame(g).max(start) } else { start };
    let was = crate::anim::frame(g);
    let mut frame = first;
    let mut written = 0;
    while frame <= end + 1e-6 {
        crate::anim::set_frame(g, frame);
        g.evaluate();
        for (b, f) in &files {
            let Some(mesh) = g.node(*b).and_then(|n| n.data().mesh(crate::deform::OUT).cloned()) else { continue };
            let name = g.path(*b);
            aztek_dyn::cache::Cache::append(f, frame, &[(name, mesh.positions().to_vec())]).map_err(aztek_ops::CommandError::new)?;
        }
        written += 1;
        frame += 1.0;
    }
    for (b, f) in &files {
        set(g, *b, "diskCache", PlugValue::String(f.clone()))?;
    }
    crate::dyn_solve::forget_disk(g);
    crate::anim::set_frame(g, was);
    Ok(json!({"frames": written, "files": files.iter().map(|x| x.1.clone()).collect::<Vec<_>>()}))
}

pub fn register(c: &mut Commands<Session>) {
    for (name, run) in [("particle", particle as fn(&mut Session, &Args) -> CmdResult), ("nParticle", n_particle)] {
        c.register(
            q(Cmd::new(name, "Make particles: -p x y z for each one, or a grid with -ll, -ur and -grs. -q -ct counts them; -q -at position -id 3 reads one. Returns [transform, shape].", run))
                .positional("particles (query)")
                .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
                .flag(f("position", "p", ArgType::List, "A particle there: x y z. Repeat it.").multi(3))
                .flag(f("lowerLeft", "ll", ArgType::Vec3, "Grid: one corner."))
                .flag(f("upperRight", "ur", ArgType::Vec3, "Grid: the other corner."))
                .flag(f("gridSpacing", "grs", ArgType::Float, "Grid: spacing."))
                .flag(f("conserve", "c", ArgType::Float, "How much momentum is kept from frame to frame, 0 to 1 (1)."))
                .flag(f("inherit", "i", ArgType::Float, "How much of an emitter's velocity new particles take, 0 to 1."))
                .flag(f("cache", "ch", ArgType::Bool, "Keep solved frames for scrubbing back (on); off, going back solves again from the start."))
                .flag(f("deleteCache", "dc", ArgType::Bool, "Throw away the frames kept."))
                .flag(f("dynamicAttrList", "dal", ArgType::Bool, "Query: the attributes added to the particles."))
                .flag(f("perParticleDouble", "ppd", ArgType::Bool, "Query: the per particle number attributes."))
                .flag(f("perParticleVector", "ppv", ArgType::Bool, "Query: the per particle vector attributes."))
                .flag(f("count", "ct", ArgType::Bool, "Query: how many."))
                .flag(f("attribute", "at", ArgType::String, "A per particle attribute, queried or with -e set."))
                .flag(f("particleId", "id", ArgType::Int, "The particle by its id, for -attribute."))
                .flag(f("order", "or", ArgType::Int, "The particle by its place in the array, from 0, for -attribute."))
                .flag(f("floatValue", "fv", ArgType::Float, "With -e -attribute: a number."))
                .flag(f("vectorValue", "vv", ArgType::Vec3, "With -e -attribute: a vector."))
                .flag(f("shapeName", "sn", ArgType::String, "The particle shape's name."))
                .flag(f("jitterBasePoint", "jbp", ArgType::List, "A centre to scatter particles round: x y z. Repeat it.").multi(3))
                .flag(f("jitterRadius", "jr", ArgType::Any, "How far from the centre they scatter, for each -jbp in turn.").multi(1))
                .flag(f("numJitters", "nj", ArgType::Any, "How many particles round each -jbp in turn.").multi(1))
                .flag(f("solver", "sv", ArgType::String, "The solver to join.")),
        );
    }
    c.register(
        q(Cmd::new("emitter", "Make an emitter: -type omni, dir, surf, curve or vol, -r rate, -spd speed. With a mesh named or selected it emits off its surface. Only the emitter, as the reference makes it: connectDynamic -em (or -particles here) gives it particles to emit into. Returns [emitter], or [object, emitter] on an object.", emitter))
            .positional("a mesh to emit from")
            .flag(f("type", "typ", ArgType::String, "omni, dir, surf, curve or vol."))
            .flag(f("rate", "r", ArgType::Float, "Particles per second."))
            .flag(f("speed", "spd", ArgType::Float, "Speed."))
            .flag(f("speedRandom", "srn", ArgType::Float, "How much speeds vary."))
            .flag(f("spread", "sp", ArgType::Float, "Directional: 0 to 1."))
            .flag(f("directionX", "dx", ArgType::Float, "Direction X."))
            .flag(f("directionY", "dy", ArgType::Float, "Direction Y."))
            .flag(f("directionZ", "dz", ArgType::Float, "Direction Z."))
            .flag(f("position", "pos", ArgType::Vec3, "Where it is."))
            .flag(f("volumeShape", "vsh", ArgType::String, "Volume: cube or sphere."))
            .flag(f("particles", "pt", ArgType::String, "Particles to feed (new ones by default)."))
            .flag(f("solver", "sv", ArgType::String, "The solver new particles join.")),
    );
    for (k, (name, _)) in FIELD_COMMANDS.iter().enumerate() {
        let mut cmd = q(Cmd::new(name, &format!("Make a {name} field acting on the selected bodies, or -q/-e one. Returns [field]."), move |s, a| field_command(s, a, k)));
        if k == VORTEX {
            cmd = cmd
                .flag(f("axisX", "ax", ArgType::Float, "The axis it turns about, X (0 by default)."))
                .flag(f("axisY", "ay", ArgType::Float, "The axis it turns about, Y (1 by default)."))
                .flag(f("axisZ", "az", ArgType::Float, "The axis it turns about, Z (0 by default)."));
        }
        c.register(
            cmd
                .positional("bodies to act on")
                .flag(f("magnitude", "m", ArgType::Float, "Strength."))
                .flag(f("attenuation", "att", ArgType::Float, "How fast it weakens with distance."))
                .flag(f("maxDistance", "mxd", ArgType::Float, "How far it reaches (turns the limit on)."))
                .flag(f("useMaxDistance", "umd", ArgType::Bool, "Limit its reach."))
                .flag(f("directionX", "dx", ArgType::Float, "Direction X."))
                .flag(f("directionY", "dy", ArgType::Float, "Direction Y."))
                .flag(f("directionZ", "dz", ArgType::Float, "Direction Z."))
                .flag(f("speed", "s", ArgType::Float, "Air: wind speed."))
                .flag(f("inheritVelocity", "iv", ArgType::Float, "Air: kept motion."))
                .flag(f("frequency", "f", ArgType::Float, "Turbulence: noise frequency."))
                .flag(f("minDistance", "mnd", ArgType::Float, "Newton: least distance."))
                .flag(f("position", "pos", ArgType::Vec3, "Where it is."))
                .flag(f("queried", "", ArgType::List, "The flags asked about in query mode.")),
        );
    }
    c.register(
        Cmd::new("connectDynamic", "Connect fields (-f), emitters (-em) and colliders (-c) to the bodies named; -d disconnects.", connect_dynamic)
            .positional("bodies")
            .flag(f("fields", "f", ArgType::List, "Fields."))
            .flag(f("emitters", "em", ArgType::List, "Emitters."))
            .flag(f("collisions", "c", ArgType::List, "Colliders."))
            .flag(f("delete", "d", ArgType::Bool, "Disconnect instead."))
            .flag(f("solver", "sv", ArgType::String, "The solver a body with none joins."))
            .flag(f("addScriptHandler", "ash", ArgType::Any, "A Python function that may handle the calls instead, given fields, emitters, collisionObjects and objects as keywords; True says it did. Alone; returns a number for -rsh."))
            .flag(f("removeScriptHandler", "rsh", ArgType::Int, "Take away the handler of this number; an unknown one does nothing. Alone.")),
    );
    c.register(
        Cmd::new("emit", "Add particles to particles that exist: -o the particles, -pos x y z for each new one, then -at an attribute followed by its values, -vv x y z or -fv n, one per new particle in turn. Returns their ids.", emit)
            .flag(f("object", "o", ArgType::String, "The particle shape, or its transform, to add to. Needed: the selection is not used."))
            .flag(f("position", "pos", ArgType::List, "Where one new particle goes, in the particles' space (world): x y z. Repeat it, once for each.").multi(3))
            .flag(f("attribute", "at", ArgType::Any, "The per particle attribute the values after it set: position, velocity, rgbPP, opacityPP, lifespanPP, radiusPP or mass. Repeat it for another.").multi(1))
            .flag(f("floatValue", "fv", ArgType::Any, "A number for the next new particle's attribute; on each axis of a vector attribute.").multi(1))
            .flag(f("vectorValue", "vv", ArgType::List, "A vector for the next new particle's attribute; its length for a number attribute.").multi(3))
            .flag(f("flagOrder", "", ArgType::List, "The flags' order, one name per use: MEL passes it, as the values pair with the attributes by it."))
            .undoable(false),
    );
    c.register(
        Cmd::new("getParticleAttr", "Per particle values: -at position -a true gives every one; without -a, the mean then the spread.", get_particle_attr)
            .positional("particles or particle1.pt[0:3]")
            .flag(f("attribute", "at", ArgType::String, "position, velocity, rgbPP, opacityPP, lifespanPP, age, radiusPP, particleId."))
            .flag(f("array", "a", ArgType::Bool, "Every value."))
            .flag(f("object", "o", ArgType::String, "Obsolete: the particle object, when none is named."))
            .undoable(false),
    );
    c.register(
        Cmd::new("setParticleAttr", "Set per particle values now: -at velocity -vv 0 1 0 particle1.pt[3].", set_particle_attr)
            .positional("particles or particle1.pt[0:3]")
            .flag(f("attribute", "at", ArgType::String, "position, velocity, rgbPP, opacityPP, lifespanPP, radiusPP."))
            .flag(f("vectorValue", "vv", ArgType::Vec3, "A vector."))
            .flag(f("floatValue", "fv", ArgType::Float, "A number: on each axis of a vector attribute."))
            .flag(f("object", "o", ArgType::String, "Only this particle object, every particle of it; what is named or selected is passed over."))
            .flag(f("randomFloat", "rf", ArgType::Float, "Add a random amount from -this to +this to each particle's value (0)."))
            .flag(f("randomVector", "rv", ArgType::Vec3, "Add a random amount within -x -y -z to x y z to each particle's value (0 0 0)."))
            .flag(f("relative", "r", ArgType::Bool, "Add the value to each particle's value rather than replace it (off)."))
            .undoable(false),
    );
    c.register(
        Cmd::new("dynExpression", "Keep a creation (-c) or runtime (-rbd, -rad) expression on particles: -s the text.", dyn_expression)
            .positional("particles")
            .flag(f("string", "s", ArgType::String, "The expression."))
            .flag(f("creation", "c", ArgType::Bool, "Run for each new particle."))
            .flag(f("runtimeBeforeDynamics", "rbd", ArgType::Bool, "Run every frame."))
            .flag(f("runtimeAfterDynamics", "rad", ArgType::Bool, "Run every frame."))
            .flag(f("query", "q", ArgType::Bool, "Read it back.")),
    );
    c.register(
        Cmd::new("dynCache", "Disk caches for cloth: -create (cr) solves the range and writes it, -append (ap) adds frames to the current one, -replace (rp) writes over, -delete (d) stops using it.", dyn_cache)
            .positional("cloth")
            .flag(f("create", "cr", ArgType::Bool, "Make a cache over the range."))
            .flag(f("replace", "rp", ArgType::Bool, "Write a new cache over the old."))
            .flag(f("append", "ap", ArgType::Bool, "Add frames from the cache's end to now."))
            .flag(f("delete", "d", ArgType::Bool, "Stop using the cache."))
            .flag(f("deleteFile", "df", ArgType::Bool, "With -d: remove the file too."))
            .flag(f("file", "f", ArgType::String, "The file (in the temporary folder by default)."))
            .flag(f("startTime", "st", ArgType::Float, "First frame (the playback start)."))
            .flag(f("endTime", "et", ArgType::Float, "Last frame (the playback end)."))
            .undoable(false),
    );
}
