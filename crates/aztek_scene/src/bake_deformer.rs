//! bakeDeformer: linear blend skin weights that best match whatever deforms a mesh. The source skeleton is taken through
//! a range of motion (each influence turned 45 degrees about each axis, or the frames of -customRangeOfMotion); the
//! source mesh's points in every pose, against the influences' matrices, give each point's weights by a least squares
//! fit with the weights kept positive and summing to one. The destination mesh (the source by default) is then bound
//! to the destination skeleton's matching joints with those weights.

use aztek_geo::Mesh;
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::anim;
use crate::graph::Graph;
use crate::mesh_nodes::MeshNode;
use crate::plug::{NodeId, PlugRef, PlugValue};
use crate::session::Session;
use crate::transform::{DagNode, Transform};

type Cmd = CommandDef<Session>;

const RANGE_ANGLE: f64 = std::f64::consts::FRAC_PI_4;

/// The joints under a root, itself first, depth first in child order.
fn joints_below(g: &Graph, root: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(j) = stack.pop() {
        if crate::rig::is_joint(g, j) {
            out.push(j);
        }
        let kids: Vec<NodeId> = g.children(j).to_vec();
        for &k in kids.iter().rev() {
            if g.node(k).is_some_and(|n| n.ty().is_a("transform")) {
                stack.push(k);
            }
        }
    }
    out
}

fn mesh_shape(g: &Graph, name: &str) -> CmdResult<NodeId> {
    let Some(id) = g.find(name)? else { fail!("bakeDeformer: no object named {name}") };
    match crate::components::as_mesh(g, id) {
        Some(m) => Ok(m),
        None => fail!("bakeDeformer: {name} is not a mesh"),
    }
}

fn joint(g: &Graph, name: &str) -> CmdResult<NodeId> {
    match g.find(name)? {
        Some(id) if crate::rig::is_joint(g, id) => Ok(id),
        _ => fail!("bakeDeformer: no joint named {name}"),
    }
}

/// The mesh's points in world space as it evaluates now.
fn world_points(g: &mut Graph, shape: NodeId) -> Vec<DVec3> {
    g.evaluate();
    let mesh: Option<std::sync::Arc<Mesh>> = match g.value(&PlugRef::new(shape, MeshNode::OUT_MESH as i32)) {
        Ok(PlugValue::Mesh(Some(m))) => Some(m.clone()),
        _ => None,
    };
    let world = g.value(&PlugRef::new(shape, DagNode::WORLD_MATRIX as i32)).ok().and_then(|v| if let PlugValue::Matrix(m) = v { Some(m) } else { None }).unwrap_or(DMat4::IDENTITY);
    mesh.map(|m| m.positions().iter().map(|p| world.transform_point3(*p)).collect()).unwrap_or_default()
}

fn world_matrix(g: &mut Graph, id: NodeId) -> DMat4 {
    match g.value(&PlugRef::new(id, DagNode::WORLD_MATRIX as i32)) {
        Ok(PlugValue::Matrix(m)) => m,
        _ => DMat4::IDENTITY,
    }
}

fn local_trs(g: &Graph, id: NodeId) -> (DVec3, DVec3, DVec3) {
    let n = g.node(id).unwrap();
    (n.data().vec3(Transform::TRANSLATE), n.data().vec3(Transform::ROTATE), n.data().vec3(Transform::SCALE))
}

fn set_rotate(g: &mut Graph, id: NodeId, r: DVec3) -> CmdResult<()> {
    g.set_value(PlugRef::new(id, Transform::ROTATE as i32), PlugValue::Vec3(r))?;
    Ok(())
}

/// The default range of motion: the rest pose, then each influence turned 45 degrees each way about each axis.
fn range_poses(n: usize) -> Vec<Option<(usize, DVec3)>> {
    let mut out = vec![None];
    for j in 0..n {
        for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
            for sign in [1.0, -1.0] {
                out.push(Some((j, axis * RANGE_ANGLE * sign)));
            }
        }
    }
    out
}

/// Non-negative least squares on the normal equations: minimise w'Hw - 2f'w with w >= 0 (Lawson and Hanson's active set).
fn nnls(h: &[Vec<f64>], f: &[f64]) -> Vec<f64> {
    let n = f.len();
    let mut w = vec![0.0; n];
    let mut passive = vec![false; n];
    let tol = 1e-12 * h.iter().enumerate().map(|(i, r)| r[i].abs()).fold(0.0, f64::max).max(1e-30);
    for _ in 0..(3 * n + 10) {
        let grad: Vec<f64> = (0..n).map(|i| f[i] - (0..n).map(|k| h[i][k] * w[k]).sum::<f64>()).collect();
        let Some((j, g)) = (0..n).filter(|&i| !passive[i]).map(|i| (i, grad[i])).max_by(|a, b| a.1.total_cmp(&b.1)) else { break };
        if g <= tol {
            break;
        }
        passive[j] = true;
        loop {
            let set: Vec<usize> = (0..n).filter(|&i| passive[i]).collect();
            let z = solve_sub(h, f, &set);
            if set.iter().zip(&z).all(|(_, &v)| v > 0.0) {
                for (k, &i) in set.iter().enumerate() {
                    w[i] = z[k];
                }
                break;
            }
            // Step back to where the first weight reaches zero, and let it go.
            let mut alpha = 1.0f64;
            for (k, &i) in set.iter().enumerate() {
                if z[k] <= 0.0 {
                    let d = w[i] - z[k];
                    if d > 1e-300 {
                        alpha = alpha.min(w[i] / d);
                    }
                }
            }
            for (k, &i) in set.iter().enumerate() {
                w[i] += alpha * (z[k] - w[i]);
                if w[i] <= 1e-14 {
                    w[i] = 0.0;
                    passive[i] = false;
                }
            }
            if !passive.iter().any(|&p| p) {
                break;
            }
        }
    }
    w
}

/// Solves H z = f on a subset of the unknowns (Cholesky, with a little damping when it is singular).
fn solve_sub(h: &[Vec<f64>], f: &[f64], set: &[usize]) -> Vec<f64> {
    let m = set.len();
    let mut a: Vec<Vec<f64>> = set.iter().map(|&i| set.iter().map(|&k| h[i][k]).collect()).collect();
    let mut b: Vec<f64> = set.iter().map(|&i| f[i]).collect();
    let scale = (0..m).map(|i| a[i][i].abs()).fold(0.0, f64::max).max(1e-30);
    // Gaussian elimination with partial pivoting.
    for c in 0..m {
        let p = (c..m).max_by(|&x, &y| a[x][c].abs().total_cmp(&a[y][c].abs())).unwrap();
        a.swap(c, p);
        b.swap(c, p);
        if a[c][c].abs() < 1e-12 * scale {
            a[c][c] += 1e-9 * scale;
        }
        for r in c + 1..m {
            let k = a[r][c] / a[c][c];
            if k != 0.0 {
                for x in c..m {
                    a[r][x] -= k * a[c][x];
                }
                b[r] -= k * b[c];
            }
        }
    }
    let mut z = vec![0.0; m];
    for r in (0..m).rev() {
        let s: f64 = (r + 1..m).map(|x| a[r][x] * z[x]).sum();
        z[r] = (b[r] - s) / a[r][r];
    }
    z
}

/// The weights of one point: rest position, its positions in each pose and each influence's pose-from-rest matrices.
fn solve_point(rest: DVec3, posed: &[DVec3], mats: &[Vec<DMat4>], max_inf: usize) -> Vec<(usize, f64)> {
    let n = mats.len();
    let p = posed.len();
    // y[j][k]: where influence j alone puts the point in pose k.
    let y: Vec<Vec<DVec3>> = mats.iter().map(|m| (0..p).map(|k| m[k].transform_point3(rest)).collect()).collect();
    let mut h = vec![vec![0.0; n]; n];
    let mut f = vec![0.0; n];
    for i in 0..n {
        for k in i..n {
            let v: f64 = (0..p).map(|q| y[i][q].dot(y[k][q])).sum();
            h[i][k] = v;
            h[k][i] = v;
        }
        f[i] = (0..p).map(|q| y[i][q].dot(posed[q])).sum();
    }
    // The weights sum to one: a heavy penalty on the sum's distance from it.
    let scale = (0..n).map(|i| h[i][i]).fold(0.0, f64::max).max(1e-12);
    let lambda = scale * 1e3;
    let ridge = scale * 1e-9;
    let fit = |set: &[usize]| -> Vec<f64> {
        let hh: Vec<Vec<f64>> = set.iter().map(|&i| set.iter().map(|&k| h[i][k] + lambda + if i == k { ridge } else { 0.0 }).collect()).collect();
        let ff: Vec<f64> = set.iter().map(|&i| f[i] + lambda).collect();
        nnls(&hh, &ff)
    };
    let all: Vec<usize> = (0..n).collect();
    let w = fit(&all);
    let mut order: Vec<usize> = (0..n).filter(|&i| w[i] > 1e-9).collect();
    order.sort_by(|&a, &b| w[b].total_cmp(&w[a]));
    let (set, w) = if order.len() > max_inf {
        order.truncate(max_inf);
        let w2 = fit(&order);
        (order.clone(), w2)
    } else {
        (all.clone(), w)
    };
    let mut out: Vec<(usize, f64)> = set.iter().enumerate().map(|(k, &i)| (i, if set.len() == n { w[i] } else { w[k] })).filter(|(_, x)| *x > 1e-9).collect();
    let sum: f64 = out.iter().map(|x| x.1).sum();
    if sum > 1e-12 {
        for x in &mut out {
            x.1 /= sum;
        }
    } else {
        out = vec![(0, 1.0)];
    }
    out
}

/// Each point's neighbours, by the mesh's edges.
fn neighbours(m: &Mesh) -> Vec<Vec<u32>> {
    let n = m.num_points() as usize;
    let mut out = vec![Vec::new(); n];
    for (p, list) in out.iter_mut().enumerate() {
        if !m.point_valid(p as aztek_geo::Index) {
            continue;
        }
        m.for_each_point_edge(p as aztek_geo::Index, |e| {
            let o = m.edge_other_point(e, p as aztek_geo::Index);
            if !list.contains(&o) {
                list.push(o);
            }
        });
    }
    out
}

fn bake_deformer(s: &mut Session, a: &Args) -> CmdResult {
    let src_mesh_name = a.get("srcMeshName").and_then(Value::as_str).map(str::to_string);
    let src_skel_name = a.get("srcSkeletonName").and_then(Value::as_str).map(str::to_string);
    let Some(src_skel_name) = src_skel_name else { fail!("bakeDeformer: give the source skeleton with -ss") };
    let g = &mut s.graph;
    let src_root = joint(g, &src_skel_name)?;
    let dst_root = match a.get("dstSkeletonName").and_then(Value::as_str) {
        Some(n) => joint(g, n)?,
        None => src_root,
    };
    // The influences: those named (with their children for -hi), or the whole source skeleton.
    let src_joints = joints_below(g, src_root);
    let mut infl: Vec<NodeId> = Vec::new();
    match a.get("influences") {
        Some(v) => {
            let list: Vec<String> = match v {
                Value::Array(x) => x.iter().filter_map(|e| e.as_str().map(str::to_string)).collect(),
                Value::String(x) => vec![x.clone()],
                _ => Vec::new(),
            };
            for n in list {
                let j = joint(g, &n)?;
                let more = if a.bool("hierarchy") { joints_below(g, j) } else { vec![j] };
                for x in more {
                    if !infl.contains(&x) {
                        infl.push(x);
                    }
                }
            }
        }
        None => infl = src_joints.clone(),
    }
    if infl.is_empty() {
        fail!("bakeDeformer: no influences");
    }
    let n = infl.len();

    // -bakeRangeOfMotion: key the range of motion on the source skeleton, one pose a frame, and say how many.
    if a.bool("bakeRangeOfMotion") {
        let poses = range_poses(n);
        let start = anim::frame(g);
        g.open_chunk("bakeDeformer");
        let r = (|| -> CmdResult<()> {
            for (k, pose) in poses.iter().enumerate() {
                for (i, &j) in infl.iter().enumerate() {
                    let r = match pose {
                        Some((pj, rot)) if *pj == i => *rot,
                        _ => DVec3::ZERO,
                    };
                    for c in 0..3 {
                        anim::key_plug(g, &PlugRef::component(j, Transform::ROTATE as i32, c), start + k as f64, Some(r[c as usize]), None, None)?;
                    }
                }
            }
            Ok(())
        })();
        g.close_chunk();
        r?;
        return Ok(json!(poses.len()));
    }

    let Some(src_mesh_name) = src_mesh_name else { fail!("bakeDeformer: give the source mesh with -sm") };
    let src_mesh = mesh_shape(g, &src_mesh_name)?;
    let dst_mesh = match a.get("dstMeshName").and_then(Value::as_str) {
        Some(n) => mesh_shape(g, n)?,
        None => src_mesh,
    };
    // The destination skeleton's joint for each influence: the one at the same place in its hierarchy.
    let dst_joints_all = joints_below(g, dst_root);
    let dst_infl: Vec<NodeId> = if dst_root == src_root {
        infl.clone()
    } else {
        if dst_joints_all.len() != src_joints.len() {
            fail!("bakeDeformer: the destination skeleton has {} joints, the source {}", dst_joints_all.len(), src_joints.len());
        }
        infl.iter().map(|j| dst_joints_all[src_joints.iter().position(|x| x == j).unwrap_or(0)]).collect()
    };
    let max_inf = a.get("maxInfluences").and_then(Value::as_i64).map(|x| x.max(1) as usize).unwrap_or(4).min(n);

    // Sample the poses. The rest pose is the influences with no rotation.
    let saved: Vec<DVec3> = infl.iter().map(|&j| local_trs(g, j).1).collect();
    let now = anim::frame(g);
    for &j in &infl {
        set_rotate(g, j, DVec3::ZERO)?;
    }
    let rest_points = world_points(g, src_mesh);
    let rest_mats: Vec<DMat4> = infl.iter().map(|&j| world_matrix(g, j)).collect();
    let rest_inv: Vec<DMat4> = rest_mats.iter().map(|m| m.inverse()).collect();
    let mut posed: Vec<Vec<DVec3>> = Vec::new();
    let mut mats: Vec<Vec<DMat4>> = vec![Vec::new(); n];
    let mut still: Vec<String> = Vec::new();
    let record = |g: &mut Graph, posed: &mut Vec<Vec<DVec3>>, mats: &mut Vec<Vec<DMat4>>| {
        posed.push(world_points(g, src_mesh));
        for (i, &j) in infl.iter().enumerate() {
            mats[i].push(world_matrix(g, j) * rest_inv[i]);
        }
    };
    let rom = anim::range_flag(a, "customRangeOfMotion")?;
    if let Some((lo, hi)) = rom {
        // Each frame of the range a pose; open ends are the animation range's.
        let r = anim::range();
        let (lo, hi) = (if lo.is_finite() { lo } else { r.anim_start }, if hi.is_finite() { hi } else { r.anim_end });
        for &j in &infl {
            set_rotate(g, j, DVec3::ZERO)?;
        }
        let mut seen: Vec<Vec<(DVec3, DVec3, DVec3)>> = vec![Vec::new(); n];
        let mut t = lo;
        while t <= hi + 1e-9 {
            anim::set_frame(g, t);
            g.evaluate();
            record(g, &mut posed, &mut mats);
            for (i, &j) in infl.iter().enumerate() {
                seen[i].push(local_trs(g, j));
            }
            t += 1.0;
        }
        // Influences that hardly move over the range: under 5 degrees, 1% of the largest bounding box, or 1% scale.
        let bbox = {
            let (mut lo3, mut hi3) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
            for p in posed.iter().flatten() {
                lo3 = lo3.min(*p);
                hi3 = hi3.max(*p);
            }
            (hi3 - lo3).max_element().max(1e-9)
        };
        for (i, &j) in infl.iter().enumerate() {
            let v = &seen[i];
            let spread = |f: &dyn Fn(&(DVec3, DVec3, DVec3)) -> DVec3| -> f64 {
                let (mut lo3, mut hi3) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
                for x in v {
                    lo3 = lo3.min(f(x));
                    hi3 = hi3.max(f(x));
                }
                if v.is_empty() { 0.0 } else { (hi3 - lo3).max_element() }
            };
            let moves = spread(&|x| x.1) >= 5f64.to_radians() || spread(&|x| x.0) >= 0.01 * bbox || spread(&|x| x.2) >= 0.01;
            if !moves {
                still.push(g.path(j));
            }
        }
        anim::set_frame(g, now);
    } else {
        for pose in range_poses(n) {
            for (i, &j) in infl.iter().enumerate() {
                let r = match pose {
                    Some((pj, rot)) if pj == i => rot,
                    _ => DVec3::ZERO,
                };
                set_rotate(g, j, r)?;
            }
            record(g, &mut posed, &mut mats);
        }
    }
    // Back to the rest pose to bind.
    for &j in &infl {
        set_rotate(g, j, DVec3::ZERO)?;
    }
    g.evaluate();
    let count = rest_points.len();
    if posed.iter().any(|p| p.len() != count) || count == 0 {
        fail!("bakeDeformer: the source mesh changes its point count between poses");
    }
    let dst_count = world_points(g, dst_mesh).len();
    if dst_count != count {
        fail!("bakeDeformer: the destination mesh has {dst_count} points, the source {count}");
    }

    // The fit, point by point.
    let mut weights: Vec<Vec<(usize, f64)>> = Vec::with_capacity(count);
    for v in 0..count {
        let pts: Vec<DVec3> = posed.iter().map(|p| p[v]).collect();
        weights.push(solve_point(rest_points[v], &pts, &mats, max_inf));
    }

    // Smoothing: each pass averages a point's weights with its neighbours', renormalised.
    let smooth = a.get("smoothWeights").and_then(Value::as_i64).unwrap_or(0).max(0);
    if smooth > 0 {
        let m: Option<std::sync::Arc<Mesh>> = match g.value(&PlugRef::new(dst_mesh, MeshNode::OUT_MESH as i32)) {
            Ok(PlugValue::Mesh(Some(m))) => Some(m.clone()),
            _ => None,
        };
        if let Some(m) = m {
            let nb = neighbours(&m);
            for _ in 0..smooth {
                let dense: Vec<Vec<f64>> = weights.iter().map(|w| {
                    let mut d = vec![0.0; n];
                    for &(i, x) in w {
                        d[i] = x;
                    }
                    d
                }).collect();
                for v in 0..count {
                    let mut acc = dense[v].clone();
                    let k = nb.get(v).map(|l| l.len()).unwrap_or(0);
                    for &o in nb.get(v).into_iter().flatten() {
                        for i in 0..n {
                            acc[i] += dense[o as usize][i];
                        }
                    }
                    let mut list: Vec<(usize, f64)> = acc.iter().enumerate().map(|(i, x)| (i, x / (k + 1) as f64)).filter(|x| x.1 > 1e-9).collect();
                    list.sort_by(|x, y| y.1.total_cmp(&x.1));
                    list.truncate(max_inf);
                    let s: f64 = list.iter().map(|x| x.1).sum();
                    for x in &mut list {
                        x.1 /= s.max(1e-12);
                    }
                    weights[v] = list;
                }
            }
        }
    }

    // Bind the destination. The same mesh loses what deformed it, in its rest pose, to the new skin.
    g.open_chunk("bakeDeformer");
    let result = (|| -> CmdResult<String> {
        if dst_mesh == src_mesh {
            let xf = crate::dag::transform_of(&s.graph, dst_mesh);
            let p = s.graph.path(xf);
            s.run("delete", json!({"args": [p], "constructionHistory": true}))?;
        }
        let mut args: Vec<String> = dst_infl.iter().map(|&j| s.graph.path(j)).collect();
        args.push(s.graph.path(dst_mesh));
        let r = s.run("skinCluster", json!({"args": args, "toSelectedBones": true, "maximumInfluences": max_inf}))?;
        let name = match &r {
            Value::Array(v) => v.first().and_then(Value::as_str).unwrap_or("").to_string(),
            Value::String(x) => x.clone(),
            _ => String::new(),
        };
        let g = &mut s.graph;
        let Some(sk) = crate::skin::skin_of(g, dst_mesh) else { fail!("bakeDeformer: the bind made no skinCluster") };
        let index_of: Vec<u32> = {
            let list = crate::skin::influences(g, sk);
            dst_infl.iter().map(|j| list.iter().find(|(_, x)| x == j).map(|(i, _)| *i as u32).unwrap_or(0)).collect()
        };
        let Some(bind) = crate::skin::bind_of(g, sk) else { fail!("bakeDeformer: no weights on {name}") };
        let mut b = (*bind).clone();
        b.weights = weights.iter().map(|w| w.iter().map(|&(i, x)| (index_of[i], x)).collect()).collect();
        crate::skin::commit_paint(g, sk, b)?;
        Ok(if name.is_empty() { g.path(sk) } else { name })
    })();
    // The source skeleton goes back to how it was posed.
    for (&j, r) in infl.iter().zip(&saved) {
        if !dst_infl.contains(&j) || dst_mesh != src_mesh {
            let _ = set_rotate(&mut s.graph, j, *r);
        }
    }
    s.graph.close_chunk();
    let skin = result?;
    if let Some(pw) = a.get("pruneWeights").and_then(Value::as_f64) {
        s.run("skinPercent", json!({"args": [skin.clone(), s.graph.path(dst_mesh)], "pruneWeights": pw}))?;
    }
    if a.bool("colorizeSkeleton") {
        if !a.has("srcSkeletonName") || !a.has("dstSkeletonName") {
            fail!("bakeDeformer: -colorizeSkeleton needs -srcSkeletonName and -dstSkeletonName");
        }
        let g = &mut s.graph;
        for (i, &j) in dst_infl.iter().enumerate() {
            let ty = g.node(j).unwrap().ty().clone();
            if let (Some(u), Some(c)) = (ty.attr_index("useObjectColor"), ty.attr_index("objectColor")) {
                g.set_value(PlugRef::new(j, u as i32), PlugValue::Int(1))?;
                g.set_value(PlugRef::new(j, c as i32), PlugValue::Int((i % 8) as i64))?;
            }
        }
    }
    if rom.is_some() {
        return Ok(json!(still));
    }
    Ok(json!(skin))
}

pub fn register(c: &mut Commands<Session>) {
    let f = |name: &str, short: &str, ty: ArgType, doc: &str| FlagDef::new(name, short, ty, Value::Null, doc);
    c.register(
        Cmd::new(
            "bakeDeformer",
            "Fit linear blend skin weights to what deforms a mesh: the source skeleton (-ss) goes through a range of motion, the source \
             mesh (-sm) is sampled in each pose, and the destination mesh (-dm, the source by default) is bound to the destination \
             skeleton's (-ds) matching joints with the weights found. -mi caps influences per point, -sw smooths, -pw prunes. \
             -brm keys the range of motion and returns the number of poses; -rom uses the frames of a range and returns the \
             influences that hardly move. Returns the skinCluster.",
            bake_deformer,
        )
        .flag(f("srcMeshName", "sm", ArgType::String, "The mesh whose deformation is matched."))
        .flag(f("srcSkeletonName", "ss", ArgType::String, "The source skeleton's root joint."))
        .flag(f("dstMeshName", "dm", ArgType::String, "The mesh bound with the weights found (the source mesh)."))
        .flag(f("dstSkeletonName", "ds", ArgType::String, "The destination skeleton's root joint (the source skeleton)."))
        .flag(f("influences", "i", ArgType::Any, "The joints that become influences (the whole source skeleton).").multi(1))
        .flag(f("hierarchy", "hi", ArgType::Bool, "With -i: the children of the joints named too."))
        .flag(f("maxInfluences", "mi", ArgType::Int, "The most influences a point has (4)."))
        .flag(f("pruneWeights", "pw", ArgType::Float, "Afterwards, weights below this become zero."))
        .flag(f("smoothWeights", "sw", ArgType::Int, "Afterwards, smooth the weights this many times."))
        .flag(f("bakeRangeOfMotion", "brm", ArgType::Bool, "Key the range of motion on the source skeleton, one pose a frame from the current one; returns how many poses."))
        .flag(f("customRangeOfMotion", "rom", ArgType::Any, "Use the frames of this range as the poses, \"1:40\"; open ends are the animation range's."))
        .flag(f("colorizeSkeleton", "cs", ArgType::Bool, "Give the new skin's joints colours of their own (needs -ss and -ds).")),
    );
}
