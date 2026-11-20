//! Reading the faces out of the solved fields: points whose lattice points coincide merge into one vertex, points
//! whose lattice points are one step apart give an edge, and the edges' cycles around the surface are the faces.

use super::field::{compat_orientation, compat_position, Level};
use glam::DVec3;
use std::collections::{HashMap, HashSet};

pub struct Extracted {
    pub v: Vec<DVec3>,
    pub n: Vec<DVec3>,
    pub faces: Vec<Vec<u32>>,
}

struct Dsu(Vec<u32>);
impl Dsu {
    fn find(&mut self, x: u32) -> u32 {
        let mut r = x;
        while self.0[r as usize] != r {
            r = self.0[r as usize];
        }
        let mut y = x;
        while self.0[y as usize] != r {
            let nx = self.0[y as usize];
            self.0[y as usize] = r;
            y = nx;
        }
        r
    }
    fn union(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.0[ra.max(rb) as usize] = ra.min(rb);
        }
    }
}

pub fn extract(l: &Level) -> Extracted {
    // Two points are read as the same lattice point only while their clusters lie within an edge length, and no
    // relation is read where the two lattices are torn apart by more than half of one.
    let merge_gap = 1.0;
    let tear = 0.5;
    let nv = l.v.len();
    let mut dsu = Dsu((0..nv as u32).collect());
    let mut links: Vec<(u32, u32)> = Vec::new();
    let mut merges: Vec<(f64, u32, u32)> = Vec::new();
    for i in 0..nv {
        for &(j, _) in &l.adj[i] {
            let j = j as usize;
            if j <= i {
                continue;
            }
            let (qi, qj) = compat_orientation(l.q[i], l.n[i], l.q[j], l.n[j]);
            let s = 0.5 * (l.scale[i] + l.scale[j]);
            let (a, pa, b, pb) = compat_position(l.v[i], l.n[i], qi, l.o[i], l.v[j], l.n[j], qj, l.o[j], s);
            // Both coordinates are relative to each own lattice point, the lattices' axes made to agree.
            let d = [a[0] - b[0], a[1] - b[1]];
            let manhattan = d[0].abs() + d[1].abs();
            // Where the two lattices disagree by much, the field is torn here: no relation is read from it.
            let gap = (pa - pb).length();
            if gap > s * tear {
                continue;
            }
            // The lattice points themselves must agree: the same point, or one step apart. A merge read wrong joins
            // two lattice points into one vertex (a triangle where a quad should be), and merges chain.
            let dist = (l.o[i] - l.o[j]).length();
            if manhattan == 0 {
                if dist < s * merge_gap {
                    merges.push((dist / s, i as u32, j as u32));
                }
            } else if manhattan == 1 && (dist - s).abs() < s * 0.5 {
                links.push((i as u32, j as u32));
            }
        }
    }
    // Merges closest first, each only while the two clusters' lattice points still agree: a chain of small
    // disagreements must not join two lattice points into one.
    merges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut sum: Vec<DVec3> = l.o.clone();
    let mut cnt: Vec<f64> = vec![1.0; nv];
    for &(_, i, j) in &merges {
        let (ri, rj) = (dsu.find(i), dsu.find(j));
        if ri == rj {
            continue;
        }
        let (ci, cj) = (sum[ri as usize] / cnt[ri as usize], sum[rj as usize] / cnt[rj as usize]);
        let s = 0.5 * (l.scale[i as usize] + l.scale[j as usize]);
        if (ci - cj).length() > s * merge_gap {
            continue;
        }
        dsu.union(ri, rj);
        let r = dsu.find(ri) as usize;
        let other = if r == ri as usize { rj as usize } else { ri as usize };
        let so = sum[other];
        sum[r] += so;
        let co = cnt[other];
        cnt[r] += co;
    }
    // Vertices: one per cluster, at the mean of its points' lattice points.
    let mut id = vec![u32::MAX; nv];
    let mut v = Vec::new();
    let mut n: Vec<DVec3> = Vec::new();
    let mut w: Vec<f64> = Vec::new();
    for i in 0..nv {
        let r = dsu.find(i as u32) as usize;
        if id[r] == u32::MAX {
            id[r] = v.len() as u32;
            v.push(DVec3::ZERO);
            n.push(DVec3::ZERO);
            w.push(0.0);
        }
        let k = id[r] as usize;
        let a = l.area[i].max(1e-30);
        v[k] += l.o[i] * a;
        n[k] += l.n[i] * a;
        w[k] += a;
    }
    for k in 0..v.len() {
        v[k] /= w[k];
        n[k] = n[k].try_normalize().unwrap_or(DVec3::Z);
    }
    let cluster = |dsu: &mut Dsu, i: u32| id[dsu.find(i) as usize];
    let mut edges: HashSet<(u32, u32)> = HashSet::new();
    for &(i, j) in &links {
        let (a, b) = (cluster(&mut dsu, i), cluster(&mut dsu, j));
        if a != b {
            edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }
    // Edges far longer than the edge length wanted are links between unrelated clusters.
    let mut scale_at = vec![0.0f64; v.len()];
    let mut cnt = vec![0.0f64; v.len()];
    for i in 0..nv {
        let k = cluster(&mut dsu, i as u32) as usize;
        scale_at[k] += l.scale[i];
        cnt[k] += 1.0;
    }
    for k in 0..v.len() {
        scale_at[k] /= cnt[k].max(1.0);
    }
    edges.retain(|&(a, b)| {
        let s = 0.5 * (scale_at[a as usize] + scale_at[b as usize]);
        (v[a as usize] - v[b as usize]).length() < 2.2 * s
    });
    let faces = trace_faces(&v, &n, &edges);
    Extracted { v, n, faces }
}

/// The faces of an embedded graph: around each vertex its neighbours in turn, each face walks from an edge to the
/// next one clockwise at its far end. Faces facing away from the surface (the outside of a hole) are dropped.
pub fn trace_faces(v: &[DVec3], n: &[DVec3], edges: &HashSet<(u32, u32)>) -> Vec<Vec<u32>> {
    let nv = v.len();
    let mut nb: Vec<Vec<u32>> = vec![Vec::new(); nv];
    for &(a, b) in edges {
        nb[a as usize].push(b);
        nb[b as usize].push(a);
    }
    // Counter-clockwise about the normal.
    for i in 0..nv {
        let (u, w) = super::tri::frame(n[i]);
        let p = v[i];
        nb[i].sort_by(|&a, &b| {
            let da = v[a as usize] - p;
            let db = v[b as usize] - p;
            let aa = da.dot(w).atan2(da.dot(u));
            let ab = db.dot(w).atan2(db.dot(u));
            aa.partial_cmp(&ab).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    let pos_of = |nb: &Vec<Vec<u32>>, at: u32, x: u32| nb[at as usize].iter().position(|&y| y == x);
    let mut used: HashSet<(u32, u32)> = HashSet::new();
    let mut faces = Vec::new();
    for a in 0..nv as u32 {
        for &b in &nb[a as usize].clone() {
            if used.contains(&(a, b)) {
                continue;
            }
            let mut face = vec![a];
            let (mut u, mut x) = (a, b);
            let mut ok = false;
            for _ in 0..12 {
                used.insert((u, x));
                if x == a {
                    ok = true;
                    break;
                }
                face.push(x);
                let k = match pos_of(&nb, x, u) {
                    Some(k) => k,
                    None => break,
                };
                let m = nb[x as usize].len();
                // The next neighbour clockwise from where we came from.
                let next = nb[x as usize][(k + m - 1) % m];
                u = x;
                x = next;
            }
            if !ok || face.len() < 3 || face.len() > 10 {
                continue;
            }
            // Newell normal against the vertices' normals.
            let mut fnrm = DVec3::ZERO;
            let mut avg = DVec3::ZERO;
            for i in 0..face.len() {
                let p = v[face[i] as usize];
                let q = v[face[(i + 1) % face.len()] as usize];
                fnrm += DVec3::new((p.y - q.y) * (p.z + q.z), (p.z - q.z) * (p.x + q.x), (p.x - q.x) * (p.y + q.y));
                avg += n[face[i] as usize];
            }
            if fnrm.dot(avg) <= 0.0 {
                continue;
            }
            faces.push(face);
        }
    }
    faces
}

/// Drops faces until no edge has more than two and no vertex is shared by unconnected fans, worst faces first.
pub fn make_manifold(v: &[DVec3], faces: &mut Vec<Vec<u32>>) {
    let quality = |f: &Vec<u32>| -> f64 {
        // Quads are best; then how square the corners are.
        let k = f.len();
        let mut q = if k == 4 { 1.0 } else { 0.6 };
        for i in 0..k {
            let a = v[f[(i + k - 1) % k] as usize];
            let b = v[f[i] as usize];
            let c = v[f[(i + 1) % k] as usize];
            let ang = (a - b).angle_between(c - b);
            if ang.is_finite() {
                let ideal = std::f64::consts::PI * (k as f64 - 2.0) / k as f64;
                q -= (ang - ideal).abs() / std::f64::consts::PI / k as f64;
            }
        }
        q
    };
    loop {
        let mut count: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
        for (fi, f) in faces.iter().enumerate() {
            for i in 0..f.len() {
                let (a, b) = (f[i], f[(i + 1) % f.len()]);
                count.entry(if a < b { (a, b) } else { (b, a) }).or_default().push(fi);
            }
        }
        let mut drop: HashSet<usize> = HashSet::new();
        for (_, fs) in count.iter() {
            if fs.len() > 2 {
                let mut s = fs.clone();
                s.sort_by(|&x, &y| quality(&faces[y]).partial_cmp(&quality(&faces[x])).unwrap_or(std::cmp::Ordering::Equal));
                for &x in &s[2..] {
                    drop.insert(x);
                }
            }
        }
        // The same directed edge twice means two faces disagree about which way round they go.
        let mut dir: HashMap<(u32, u32), usize> = HashMap::new();
        for (fi, f) in faces.iter().enumerate() {
            if drop.contains(&fi) {
                continue;
            }
            for i in 0..f.len() {
                let e = (f[i], f[(i + 1) % f.len()]);
                if let Some(&other) = dir.get(&e) {
                    let worse = if quality(&faces[other]) < quality(f) { other } else { fi };
                    drop.insert(worse);
                } else {
                    dir.insert(e, fi);
                }
            }
        }
        if drop.is_empty() {
            break;
        }
        let mut i = 0;
        faces.retain(|_| {
            let keep = !drop.contains(&i);
            i += 1;
            keep
        });
    }
}

/// Pairs of faces that make better quads together: two triangles become a quad, a triangle and a pentagon a hexagon
/// cut into two quads. Each pairing removes two irregular faces from the layout.
pub fn pair_irregular(v: &[DVec3], faces: &mut Vec<Vec<u32>>) {
    let corner_cost = |f: &[u32]| -> f64 {
        // How far the corners are from right angles, and whether the quad folds.
        let k = f.len();
        let mut c = 0.0;
        let mut nrm = DVec3::ZERO;
        for i in 0..k {
            let a = v[f[(i + k - 1) % k] as usize];
            let b = v[f[i] as usize];
            let d = v[f[(i + 1) % k] as usize];
            nrm += (d - b).cross(a - b);
        }
        for i in 0..k {
            let a = v[f[(i + k - 1) % k] as usize];
            let b = v[f[i] as usize];
            let d = v[f[(i + 1) % k] as usize];
            let cr = (d - b).cross(a - b);
            if cr.dot(nrm) <= 0.0 {
                return f64::MAX;
            }
            let ang = (a - b).angle_between(d - b);
            c += (ang - std::f64::consts::FRAC_PI_2).abs();
        }
        c
    };
    for _ in 0..3 {
        let mut edge_faces: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
        for (fi, f) in faces.iter().enumerate() {
            for i in 0..f.len() {
                let (a, b) = (f[i], f[(i + 1) % f.len()]);
                edge_faces.entry(if a < b { (a, b) } else { (b, a) }).or_default().push(fi);
            }
        }
        let mut gone = vec![false; faces.len()];
        let mut added: Vec<Vec<u32>> = Vec::new();
        let mut changed = false;
        for ti in 0..faces.len() {
            if gone[ti] || faces[ti].len() != 3 {
                continue;
            }
            // The best partner across one of the triangle's edges.
            let mut best: Option<(f64, usize, Vec<Vec<u32>>)> = None;
            let t = faces[ti].clone();
            for i in 0..3 {
                let (a, b) = (t[i], t[(i + 1) % 3]);
                let k = if a < b { (a, b) } else { (b, a) };
                let Some(list) = edge_faces.get(&k) else { continue };
                if list.len() != 2 {
                    continue;
                }
                let gi = if list[0] == ti { list[1] } else { list[0] };
                if gone[gi] || !(faces[gi].len() == 3 || faces[gi].len() == 5) {
                    continue;
                }
                let merged = merge_across(&t, &faces[gi], a, b);
                let Some(merged) = merged else { continue };
                let result: Vec<Vec<u32>> = if merged.len() == 4 {
                    vec![merged]
                } else {
                    // A hexagon: the best of its three cuts into two quads.
                    let mut bc: Option<(f64, Vec<Vec<u32>>)> = None;
                    for s in 0..3 {
                        let q1: Vec<u32> = (0..4).map(|j| merged[(s + j) % 6]).collect();
                        let q2: Vec<u32> = (0..4).map(|j| merged[(s + 3 + j) % 6]).collect();
                        let c = corner_cost(&q1) + corner_cost(&q2);
                        if c < f64::MAX && bc.as_ref().map(|x| c < x.0).unwrap_or(true) {
                            bc = Some((c, vec![q1, q2]));
                        }
                    }
                    match bc {
                        Some((_, qs)) => qs,
                        None => continue,
                    }
                };
                let cost: f64 = result.iter().map(|q| corner_cost(q)).sum::<f64>() / result.len() as f64;
                if cost < 2.0 && best.as_ref().map(|x| cost < x.0).unwrap_or(true) {
                    best = Some((cost, gi, result));
                }
            }
            if let Some((_, gi, result)) = best {
                gone[ti] = true;
                gone[gi] = true;
                added.extend(result);
                changed = true;
            }
        }
        if !changed {
            break;
        }
        let mut i = 0;
        faces.retain(|_| {
            let keep = !gone[i];
            i += 1;
            keep
        });
        faces.extend(added);
    }
}

/// The polygon of two faces joined across their shared edge a-b (f has it as a then b), or None if they share more.
fn merge_across(f: &[u32], g: &[u32], a: u32, b: u32) -> Option<Vec<u32>> {
    let shared = f.iter().filter(|x| g.contains(x)).count();
    if shared != 2 {
        return None;
    }
    // Walk f from b round to a, then g from a round to b.
    let fi = f.iter().position(|&x| x == b)?;
    let gi = g.iter().position(|&x| x == a)?;
    // g must have the edge as b then a.
    if g[(gi + 1) % g.len()] != b {
        return None;
    }
    let mut out = Vec::new();
    for k in 0..f.len() - 1 {
        out.push(f[(fi + k) % f.len()]);
    }
    for k in 0..g.len() - 1 {
        out.push(g[(gi + k) % g.len()]);
    }
    Some(out)
}
