//! The two fields a quad layout is read from, after Instant Field-Aligned Meshes (Jakob, Tarini, Panozzo and
//! Sorkine-Hornung, 2015): a 4-way orientation field (where edges run) and a 4-way position field (where vertices
//! sit), both smoothed extrinsically over a hierarchy of ever coarser point sets, coarse to fine. On top: hard
//! constraints along features, a pull toward the principal curvature directions, and a per point edge length.

use glam::DVec3;

/// One level of the hierarchy.
#[derive(Clone, Default)]
pub struct Level {
    pub v: Vec<DVec3>,
    pub n: Vec<DVec3>,
    pub area: Vec<f64>,
    /// Neighbours and the weight of each link.
    pub adj: Vec<Vec<(u32, f64)>>,
    /// Edge length wanted at each point.
    pub scale: Vec<f64>,
    /// The orientation field: one of the four directions, tangent.
    pub q: Vec<DVec3>,
    /// The position field: a lattice point near the point, on its tangent plane.
    pub o: Vec<DVec3>,
    /// Orientation constraints: a direction and how strongly (1 = fixed).
    pub cq: Vec<DVec3>,
    pub cq_w: Vec<f64>,
    /// Position constraints: a point an iso-line (or with `pin`, a lattice point) must pass through.
    pub co: Vec<DVec3>,
    pub co_w: Vec<f64>,
    /// The constraint fixes the lattice point itself (a feature corner), not just a line through it.
    pub pin: Vec<bool>,
    /// Coarse to fine: which point of this level each point of the next finer one merged into.
    pub to_coarse: Vec<u32>,
}

pub fn middle_point(p0: DVec3, n0: DVec3, p1: DVec3, n1: DVec3) -> DVec3 {
    let n0p0 = n0.dot(p0);
    let n0p1 = n0.dot(p1);
    let n1p0 = n1.dot(p0);
    let n1p1 = n1.dot(p1);
    let n0n1 = n0.dot(n1);
    let denom = 1.0 / (1.0 - n0n1 * n0n1 + 1e-4);
    let l0 = 2.0 * (n0p1 - n0p0 - n0n1 * (n1p0 - n1p1)) * denom;
    let l1 = 2.0 * (n1p0 - n1p1 - n0n1 * (n0p1 - n0p0)) * denom;
    0.5 * (p0 + p1) - 0.25 * (n0 * l0 + n1 * l1)
}

/// Of the four rotations of q0 and of q1, the pair that agrees best, q1's possibly negated.
pub fn compat_orientation(q0: DVec3, n0: DVec3, q1: DVec3, n1: DVec3) -> (DVec3, DVec3) {
    let a = [q0, n0.cross(q0)];
    let b = [q1, n1.cross(q1)];
    let mut best = (f64::MIN, 0, 0);
    for i in 0..2 {
        for j in 0..2 {
            let s = a[i].dot(b[j]).abs();
            if s > best.0 {
                best = (s, i, j);
            }
        }
    }
    let dp = a[best.1].dot(b[best.2]);
    (a[best.1], b[best.2] * dp.signum())
}

/// The lattice point at or below p in the lattice through o along q and t with spacing s: the integer steps and the point.
pub fn floor4(o: DVec3, q: DVec3, t: DVec3, p: DVec3, s: f64) -> ([i64; 2], DVec3) {
    let d = p - o;
    let a = (q.dot(d) / s).floor();
    let b = (t.dot(d) / s).floor();
    ([a as i64, b as i64], o + q * (a * s) + t * (b * s))
}

pub fn round4(o: DVec3, q: DVec3, t: DVec3, p: DVec3, s: f64) -> DVec3 {
    let d = p - o;
    let a = (q.dot(d) / s).round();
    let b = (t.dot(d) / s).round();
    o + q * (a * s) + t * (b * s)
}

/// The two lattice points, one from each point's lattice, nearest each other around the points' middle, with their
/// integer coordinates in each own lattice.
#[allow(clippy::too_many_arguments)]
pub fn compat_position(
    p0: DVec3,
    n0: DVec3,
    q0: DVec3,
    o0: DVec3,
    p1: DVec3,
    n1: DVec3,
    q1: DVec3,
    o1: DVec3,
    s: f64,
) -> ([i64; 2], DVec3, [i64; 2], DVec3) {
    let t0 = n0.cross(q0);
    let t1 = n1.cross(q1);
    let mid = middle_point(p0, n0, p1, n1);
    let (i0, b0) = floor4(o0, q0, t0, mid, s);
    let (i1, b1) = floor4(o1, q1, t1, mid, s);
    let mut best = (f64::MAX, 0, 0);
    for i in 0..4 {
        let a = b0 + (q0 * (i & 1) as f64 + t0 * ((i >> 1) & 1) as f64) * s;
        for j in 0..4 {
            let b = b1 + (q1 * (j & 1) as f64 + t1 * ((j >> 1) & 1) as f64) * s;
            let c = (a - b).length_squared();
            if c < best.0 {
                best = (c, i, j);
            }
        }
    }
    let (i, j) = (best.1, best.2);
    let a = b0 + (q0 * (i & 1) as f64 + t0 * ((i >> 1) & 1) as f64) * s;
    let b = b1 + (q1 * (j & 1) as f64 + t1 * ((j >> 1) & 1) as f64) * s;
    (
        [i0[0] + (i & 1) as i64, i0[1] + ((i >> 1) & 1) as i64],
        a,
        [i1[0] + (j & 1) as i64, i1[1] + ((j >> 1) & 1) as i64],
        b,
    )
}

/// Builds coarser levels until one has few points. Each coarse point is a merged pair of neighbours facing alike.
pub fn build_hierarchy(fine: Level) -> Vec<Level> {
    let mut levels = vec![fine];
    loop {
        let cur = levels.last().unwrap();
        let nv = cur.v.len();
        if nv <= 64 || levels.len() >= 20 {
            break;
        }
        // Pair each point with its best unpaired neighbour, best pairs first.
        let mut cand: Vec<(f64, u32, u32)> = Vec::new();
        for i in 0..nv {
            for &(j, _) in &cur.adj[i] {
                if (j as usize) > i {
                    let (a, b) = (cur.area[i].max(1e-30), cur.area[j as usize].max(1e-30));
                    let ratio = if a > b { b / a } else { a / b };
                    cand.push((cur.n[i].dot(cur.n[j as usize]) * ratio, i as u32, j));
                }
            }
        }
        cand.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut to_coarse = vec![u32::MAX; nv];
        let mut next = Level::default();
        let push = |next: &mut Level, members: &[usize]| {
            let mut a = 0.0;
            let mut p = DVec3::ZERO;
            let mut n = DVec3::ZERO;
            let mut s = 0.0;
            let mut cqv = DVec3::ZERO;
            let mut cqw: f64 = 0.0;
            let mut cov = DVec3::ZERO;
            let mut cow: f64 = 0.0;
            let mut pin = false;
            for &m in members {
                let w = cur.area[m].max(1e-30);
                a += cur.area[m];
                p += cur.v[m] * w;
                n += cur.n[m] * w;
                s += cur.scale[m] * w;
                if cur.cq_w[m] > 0.0 {
                    let d = if cqw > 0.0 { compat_orientation(cqv, cur.n[m], cur.cq[m], cur.n[m]).1 } else { cur.cq[m] };
                    cqv = if cqw > 0.0 { cqv + d * cur.cq_w[m] } else { d * cur.cq_w[m] };
                    cqw = cqw.max(cur.cq_w[m]);
                }
                if cur.co_w[m] > 0.0 && (cur.pin[m] || !pin) {
                    if cur.pin[m] && !pin {
                        cov = DVec3::ZERO;
                        cow = 0.0;
                    }
                    pin |= cur.pin[m];
                    cov += cur.co[m];
                    cow += 1.0;
                }
            }
            let wsum: f64 = members.iter().map(|&m| cur.area[m].max(1e-30)).sum();
            let nn = n.try_normalize().unwrap_or(cur.n[members[0]]);
            next.v.push(p / wsum);
            next.n.push(nn);
            next.area.push(a);
            next.scale.push(s / wsum);
            next.cq.push((cqv - nn * nn.dot(cqv)).try_normalize().unwrap_or(DVec3::ZERO));
            next.cq_w.push(if cqv.length_squared() > 0.0 { cqw } else { 0.0 });
            next.co.push(if cow > 0.0 { cov / cow } else { DVec3::ZERO });
            next.co_w.push(if cow > 0.0 { members.iter().map(|&m| cur.co_w[m]).fold(0.0, f64::max) } else { 0.0 });
            next.pin.push(pin);
        };
        for &(_, i, j) in &cand {
            if to_coarse[i as usize] == u32::MAX && to_coarse[j as usize] == u32::MAX {
                let c = next.v.len() as u32;
                to_coarse[i as usize] = c;
                to_coarse[j as usize] = c;
                push(&mut next, &[i as usize, j as usize]);
            }
        }
        for i in 0..nv {
            if to_coarse[i] == u32::MAX {
                to_coarse[i] = next.v.len() as u32;
                push(&mut next, &[i]);
            }
        }
        if next.v.len() as f64 > nv as f64 * 0.9 {
            break;
        }
        let nc = next.v.len();
        let mut adj: Vec<std::collections::HashMap<u32, f64>> = vec![Default::default(); nc];
        for i in 0..nv {
            for &(j, w) in &cur.adj[i] {
                let (a, b) = (to_coarse[i], to_coarse[j as usize]);
                if a != b {
                    *adj[a as usize].entry(b).or_insert(0.0) += w;
                }
            }
        }
        next.adj = adj.into_iter().map(|m| {
            let mut v: Vec<(u32, f64)> = m.into_iter().collect();
            v.sort_by_key(|x| x.0);
            v
        }).collect();
        next.q = vec![DVec3::ZERO; nc];
        next.o = vec![DVec3::ZERO; nc];
        next.to_coarse = Vec::new();
        levels.last_mut().unwrap().to_coarse = to_coarse;
        levels.push(next);
    }
    levels
}

/// Gauss-Seidel sweeps of the orientation field on one level, each point leaning toward its constraint by its weight.
pub fn smooth_orientation(l: &mut Level, iterations: usize, use_soft: bool) {
    let nv = l.v.len();
    for _ in 0..iterations {
        for i in 0..nv {
            let n = l.n[i];
            if l.cq_w[i] >= 1.0 {
                l.q[i] = fit_constraint(l.q[i], n, l.cq[i]);
                continue;
            }
            let mut q = l.q[i];
            let mut wsum = 0.0;
            for &(j, w) in &l.adj[i] {
                let (a, b) = compat_orientation(q, n, l.q[j as usize], l.n[j as usize]);
                q = a * wsum + b * w;
                wsum += w;
                q = (q - n * n.dot(q)).try_normalize().unwrap_or(l.q[i]);
            }
            if use_soft && l.cq_w[i] > 0.0 {
                let c = fit_constraint(q, n, l.cq[i]);
                let (a, b) = compat_orientation(q, n, c, n);
                q = (a * (1.0 - l.cq_w[i]) + b * l.cq_w[i]).try_normalize().unwrap_or(q);
            }
            l.q[i] = (q - n * n.dot(q)).try_normalize().unwrap_or(l.q[i]);
        }
    }
}

/// The constraint direction, on the tangent plane: the field takes it as one of its four directions.
fn fit_constraint(q: DVec3, n: DVec3, c: DVec3) -> DVec3 {
    (c - n * n.dot(c)).try_normalize().unwrap_or(q)
}

/// Gauss-Seidel sweeps of the position field on one level.
pub fn smooth_position(l: &mut Level, iterations: usize) {
    let nv = l.v.len();
    for _ in 0..iterations {
        for i in 0..nv {
            let (v, n, q) = (l.v[i], l.n[i], l.q[i]);
            let t = n.cross(q);
            let s = l.scale[i];
            if l.pin[i] && l.co_w[i] >= 1.0 {
                l.o[i] = l.co[i];
                continue;
            }
            let mut o = l.o[i];
            let mut wsum = 0.0;
            for &(j, w) in &l.adj[i] {
                let j = j as usize;
                let (qa, qb) = compat_orientation(q, n, l.q[j], l.n[j]);
                let sj = 0.5 * (s + l.scale[j]);
                let (_, a, _, b) = compat_position(v, n, qa, o, l.v[j], l.n[j], qb, l.o[j], sj);
                o = (a * wsum + b * w) / (wsum + w);
                wsum += w;
                o -= n * n.dot(o - v);
            }
            if l.co_w[i] > 0.0 {
                let c = l.co[i];
                let mut d = c - o;
                d -= n * n.dot(d);
                if !l.pin[i] && l.cq_w[i] > 0.0 {
                    // Only across the feature: the lattice line along it passes through c, and slides freely along.
                    let along = (l.cq[i] - n * n.dot(l.cq[i])).try_normalize().unwrap_or(q);
                    d -= along * along.dot(d);
                }
                o += d * l.co_w[i];
            }
            l.o[i] = round4(o, q, t, v, s);
        }
    }
}

/// Gives the finer level its starting values from the coarser one.
pub fn prolong(coarse: &Level, fine: &mut Level, what: u8) {
    for i in 0..fine.v.len() {
        let c = fine.to_coarse[i] as usize;
        let n = fine.n[i];
        if what & 1 != 0 {
            let q = coarse.q[c];
            fine.q[i] = (q - n * n.dot(q)).try_normalize().unwrap_or(fine.q[i]);
        }
        if what & 2 != 0 {
            let o = coarse.o[c];
            fine.o[i] = o - n * n.dot(o - fine.v[i]);
        }
    }
}

/// Random-free starting values: a tangent direction from the point's frame and the point itself.
pub fn init(l: &mut Level) {
    let nv = l.v.len();
    l.q = (0..nv).map(|i| super::tri::frame(l.n[i]).0).collect();
    l.o = l.v.clone();
}

/// Solves both fields over the hierarchy, coarse to fine.
/// Soft constraints (curvature, region borders) steer only the levels from `soft_from` up: they set the coarse
/// layout, and the finer levels smooth it out rather than follow each point's noisy direction.
pub fn solve(levels: &mut [Level], orient_iters: usize, pos_iters: usize, soft_from: usize) {
    let last = levels.len() - 1;
    for l in levels.iter_mut() {
        init(l);
    }
    for k in (0..=last).rev() {
        if k < last {
            let (fine, coarse) = levels.split_at_mut(k + 1);
            prolong(&coarse[0], &mut fine[k], 1);
        }
        smooth_orientation(&mut levels[k], if k == 0 { orient_iters * 2 } else { orient_iters }, k >= soft_from);
    }
    for k in (0..=last).rev() {
        if k < last {
            let (fine, coarse) = levels.split_at_mut(k + 1);
            prolong(&coarse[0], &mut fine[k], 2);
        }
        smooth_position(&mut levels[k], if k == 0 { pos_iters * 2 } else { pos_iters });
    }
}
