//! An all-quad mesh and the changes that keep it one: every change replaces some faces with others and is checked
//! before it is made, so the mesh stays manifold, closed where it was closed, and unfolded.
//!
//! The coarse layout comes from simplifying a fine all-quad mesh (Tarini et al., Practical quad mesh simplification,
//! 2010): quads collapsed across a diagonal, smallest first, two-sided points removed, edges turned to bring points
//! toward four neighbours, and points slid along the surface between. Its rows of quads (chords) then each get a
//! whole number of new rows, and every coarse quad is filled with that grid, so neighbouring quads share their points.

use super::surface::Surface;
use super::tri::key;
use glam::DVec3;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
pub struct QMesh {
    pub v: Vec<DVec3>,
    pub f: Vec<[u32; 4]>,
    pub dead: Vec<bool>,
    pub vf: Vec<Vec<u32>>,
    /// 0 free, 1 on a feature curve, 2 a corner of features (fixed).
    pub rank: Vec<u8>,
    pub feat: HashSet<(u32, u32)>,
    live: usize,
    stamp: Vec<u32>,
    /// Per point: the positions of the fine mesh's points it has absorbed, which its faces must stay near.
    pub samples: Vec<Vec<DVec3>>,
    /// How far from its faces, against the wanted size, a sample may lie.
    pub fit: f64,
    /// A symmetry plane and how near it counts as on it: no edge may join two points on it except along its cut,
    /// as the mirror would give that edge four faces.
    pub plane: Option<(crate::symmetry::SymmetryPlane, f64)>,
    /// Points merging in the change being checked, gone to kept.
    remap: Vec<(u32, u32)>,
    /// Smoothing puts a point it can't project near by onto the surface along its normal.
    pub onto: bool,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct Ord64(f64);
impl Eq for Ord64 {}
impl Ord for Ord64 {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&o.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn newell(p: [DVec3; 4]) -> DVec3 {
    let mut n = DVec3::ZERO;
    for i in 0..4 {
        n += p[i].cross(p[(i + 1) % 4]);
    }
    n
}

impl QMesh {
    pub fn new(v: Vec<DVec3>, f: Vec<[u32; 4]>, rank: Vec<u8>, feat: HashSet<(u32, u32)>) -> QMesh {
        let mut vf = vec![Vec::new(); v.len()];
        for (i, q) in f.iter().enumerate() {
            for &p in q {
                vf[p as usize].push(i as u32);
            }
        }
        QMesh { dead: vec![false; f.len()], live: f.len(), stamp: vec![0; f.len()], v, f, vf, rank, feat, onto: false, samples: Vec::new(), fit: 0.6, plane: None, remap: Vec::new() }
    }

    /// Every triangle as three quads about its middle, through its edges' middles.
    pub fn from_tris(v: &[DVec3], f: &[[u32; 3]], rank: &[u8], feat: &HashSet<(u32, u32)>) -> QMesh {
        let mut pv = v.to_vec();
        let mut pr = rank.to_vec();
        let mut pf = Vec::new();
        let mut nf = HashSet::new();
        let mut mid: HashMap<(u32, u32), u32> = HashMap::new();
        for t in f {
            let c = pv.len() as u32;
            pv.push((v[t[0] as usize] + v[t[1] as usize] + v[t[2] as usize]) / 3.0);
            pr.push(0);
            let mut m = [0u32; 3];
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                m[k] = *mid.entry(key(a, b)).or_insert_with(|| {
                    let i = pv.len() as u32;
                    pv.push((v[a as usize] + v[b as usize]) * 0.5);
                    let on = feat.contains(&key(a, b));
                    pr.push(on as u8);
                    if on {
                        nf.insert(key(a, i));
                        nf.insert(key(i, b));
                    }
                    i
                });
            }
            for k in 0..3 {
                pf.push([t[k], m[k], c, m[(k + 2) % 3]]);
            }
        }
        QMesh::new(pv, pf, pr, nf)
    }

    pub fn faces(&self) -> usize {
        self.live
    }

    fn pts(&self, q: [u32; 4]) -> [DVec3; 4] {
        q.map(|p| self.v[p as usize])
    }

    fn fnormal(&self, fi: u32) -> DVec3 {
        newell(self.pts(self.f[fi as usize]))
    }

    /// Next and previous point of p in face q.
    fn around(q: &[u32; 4], p: u32) -> (u32, u32) {
        let i = q.iter().position(|&x| x == p).unwrap();
        (q[(i + 1) % 4], q[(i + 3) % 4])
    }

    pub fn neighbors(&self, p: u32) -> Vec<u32> {
        let mut n = Vec::new();
        for &fi in &self.vf[p as usize] {
            let (a, b) = Self::around(&self.f[fi as usize], p);
            n.push(a);
            n.push(b);
        }
        n.sort_unstable();
        n.dedup();
        n
    }

    pub fn is_border(&self, p: u32) -> bool {
        let mut nx = Vec::new();
        let mut pv = Vec::new();
        for &fi in &self.vf[p as usize] {
            let (a, b) = Self::around(&self.f[fi as usize], p);
            nx.push(a);
            pv.push(b);
        }
        nx.iter().any(|a| !pv.contains(a)) || pv.iter().any(|b| !nx.contains(b))
    }

    fn energy(&self, p: u32, delta: i32) -> i32 {
        if self.rank[p as usize] == 2 {
            return 0;
        }
        let val = self.neighbors(p).len() as i32 + delta;
        let ideal = if self.is_border(p) { 3 } else { 4 };
        (val - ideal) * (val - ideal)
    }

    /// Replaces faces `remove` with `add`, points in `gone` disappearing and `moves` moving, if after it every point
    /// still has one fan of faces about it and no face turns over. `refs` is a normal per added face to compare with.
    fn try_replace(&mut self, remove: &[u32], add: &[[u32; 4]], refs: &[DVec3], gone: &[u32], moves: &[(u32, DVec3)], convex: bool) -> bool {
        for q in add {
            for i in 0..4 {
                for j in i + 1..4 {
                    if q[i] == q[j] {
                        return false;
                    }
                }
            }
        }
        for &g in gone {
            if self.vf[g as usize].iter().any(|f| !remove.contains(f)) {
                return false;
            }
        }
        let mut aff: Vec<u32> = add.iter().flatten().copied().chain(remove.iter().flat_map(|&f| self.f[f as usize])).collect();
        aff.sort_unstable();
        aff.dedup();
        aff.retain(|p| !gone.contains(p));
        for &p in &aff {
            let mut fan: Vec<(u32, u32)> = Vec::new();
            for &fi in &self.vf[p as usize] {
                if !remove.contains(&fi) {
                    fan.push(Self::around(&self.f[fi as usize], p));
                }
            }
            for q in add {
                if q.contains(&p) {
                    fan.push(Self::around(q, p));
                }
            }
            if fan.is_empty() {
                return false;
            }
            let k = fan.len();
            for i in 0..k {
                for j in i + 1..k {
                    if fan[i].0 == fan[j].0 || fan[i].1 == fan[j].1 {
                        return false;
                    }
                }
            }
            // Face j follows face i about p when j's previous point is i's next.
            let starts: Vec<usize> = (0..k).filter(|&i| !fan.iter().any(|x| x.0 == fan[i].1)).collect();
            if starts.len() > 1 {
                return false;
            }
            let s = starts.first().copied().unwrap_or(0);
            let mut cur = s;
            let mut steps = 1;
            loop {
                match (0..k).find(|&j| fan[j].1 == fan[cur].0) {
                    Some(j) if j != s => {
                        cur = j;
                        steps += 1;
                        if steps > k {
                            return false;
                        }
                    }
                    _ => break,
                }
            }
            if steps != k {
                return false;
            }
        }
        let pos = |p: u32| moves.iter().find(|m| m.0 == p).map(|m| m.1).unwrap_or(self.v[p as usize]);
        if let Some((pl, tol)) = self.plane.as_ref() {
            for q in add {
                for i in 0..4 {
                    let (a, b) = (q[i], q[(i + 1) % 4]);
                    // A feature edge here may still be named by the points about to go into a and b.
                    let srcs = |x: u32| -> Vec<u32> {
                        let mut v = vec![x];
                        v.extend(self.remap.iter().filter(|r| r.1 == x).map(|r| r.0));
                        v
                    };
                    let is_feat = srcs(a).iter().any(|&x| srcs(b).iter().any(|&y| self.feat.contains(&key(x, y))));
                    if !is_feat && pl.distance(pos(a)).abs() < *tol && pl.distance(pos(b)).abs() < *tol {
                        return false;
                    }
                }
            }
        }
        let good = |q: &[u32; 4], r: DVec3, convex: bool| {
            let p = q.map(pos);
            let n = newell(p);
            let rn = r.normalize_or_zero();
            if n.dot(rn) <= 0.25 * n.length() || n.length() < 1e-20 {
                return false;
            }
            if convex {
                for i in 0..4 {
                    let c = (p[(i + 1) % 4] - p[i]).cross(p[(i + 3) % 4] - p[i]);
                    if c.dot(rn) <= 0.0 {
                        return false;
                    }
                }
            }
            true
        };
        for (q, r) in add.iter().zip(refs) {
            if !good(q, *r, convex) {
                return false;
            }
        }
        for &(p, _) in moves {
            for &fi in &self.vf[p as usize] {
                if !remove.contains(&fi) && !good(&self.f[fi as usize], self.fnormal(fi), false) {
                    return false;
                }
            }
        }
        for &(p, x) in moves {
            self.v[p as usize] = x;
        }
        for &fi in remove {
            self.dead[fi as usize] = true;
            self.live -= 1;
            self.stamp[fi as usize] += 1;
            for p in self.f[fi as usize] {
                self.vf[p as usize].retain(|&g| g != fi);
            }
        }
        for q in add {
            let i = self.f.len() as u32;
            self.f.push(*q);
            self.dead.push(false);
            self.stamp.push(0);
            self.live += 1;
            for &p in q {
                self.vf[p as usize].push(i);
            }
        }
        for &g in gone {
            self.vf[g as usize].clear();
        }
        true
    }

    /// Whether every sample lies within `fit` of the wanted size of one of the faces.
    fn fits(&self, samples: &[DVec3], faces: &[[u32; 4]], at: &dyn Fn(u32) -> DVec3, size: &dyn Fn(DVec3) -> f64) -> bool {
        if samples.is_empty() {
            return true;
        }
        let quads: Vec<[DVec3; 4]> = faces.iter().map(|f| f.map(at)).collect();
        for &x in samples {
            let tol = self.fit * size(x);
            let mut ok = false;
            for p in &quads {
                for (a, b, c) in [(p[0], p[1], p[2]), (p[0], p[2], p[3])] {
                    if (crate::bvh::closest_on_triangle(x, a, b, c).0 - x).length() <= tol {
                        ok = true;
                        break;
                    }
                }
                if ok {
                    break;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    fn take_samples(&mut self, from: u32, to: u32) {
        if self.samples.is_empty() {
            return;
        }
        let s = std::mem::take(&mut self.samples[from as usize]);
        self.samples[to as usize].extend(s);
    }

    /// Collapses quad q across its diagonal from corner w (0 or 1). Returns the point that stays.
    fn diag_collapse(&mut self, q: u32, w: usize, size: &dyn Fn(DVec3) -> f64) -> Option<u32> {
        let t = self.f[q as usize];
        let (p0, p1, p2, p3) = (t[w], t[w + 1], t[(w + 2) % 4], t[(w + 3) % 4]);
        let (r0, r2) = (self.rank[p0 as usize], self.rank[p2 as usize]);
        if r0 >= 1 && r2 >= 1 {
            return None;
        }
        let fe = |a: u32, b: u32| self.feat.contains(&key(a, b));
        if (fe(p0, p1) && fe(p1, p2)) || (fe(p2, p3) && fe(p3, p0)) {
            return None;
        }
        // No point gathers more than five edges: a fan of quads about one point never evens out.
        if self.neighbors(p0).len() + self.neighbors(p2).len() > 7 {
            return None;
        }
        // Points facing apart lie on two sides of something thin: joining them would flatten it.
        if self.vnormal(p0).dot(self.vnormal(p2)) < 0.3 {
            return None;
        }
        let (keep, gone) = if r2 > r0 { (p2, p0) } else { (p0, p2) };
        let pos = if r0 == r2 { (self.v[p0 as usize] + self.v[p2 as usize]) * 0.5 } else { self.v[keep as usize] };
        let remove = self.vf[gone as usize].clone();
        let mut add = Vec::new();
        let mut refs = Vec::new();
        for &fi in &remove {
            if fi != q {
                add.push(self.f[fi as usize].map(|p| if p == gone { keep } else { p }));
                refs.push(self.fnormal(fi));
            }
        }
        // No face grows past the size wanted where it lies.
        let at = |p: u32| if p == keep { pos } else { self.v[p as usize] };
        for f in &add {
            let p = f.map(at);
            let c = (p[0] + p[1] + p[2] + p[3]) * 0.25;
            if (p[0] - p[2]).length().max((p[1] - p[3]).length()) > 2.2 * size(c) {
                return None;
            }
        }
        if !self.samples.is_empty() {
            let mut around = add.clone();
            for &fi in &self.vf[keep as usize] {
                if !remove.contains(&fi) {
                    around.push(self.f[fi as usize]);
                }
            }
            let smp: Vec<DVec3> = self.samples[p0 as usize].iter().chain(self.samples[p2 as usize].iter()).copied().collect();
            if !self.fits(&smp, &around, &at, size) {
                return None;
            }
        }
        self.remap = vec![(gone, keep)];
        let ok = self.try_replace(&remove, &add, &refs, &[gone], &[(keep, pos)], false);
        self.remap.clear();
        if !ok {
            return None;
        }
        self.take_samples(gone, keep);
        let old: Vec<(u32, u32)> = self.feat.iter().copied().filter(|k| k.0 == gone || k.1 == gone).collect();
        for k in old {
            self.feat.remove(&k);
            let o = if k.0 == gone { k.1 } else { k.0 };
            self.feat.insert(key(o, keep));
        }
        Some(keep)
    }

    /// A free point with only two faces: they become one.
    fn doublet(&mut self, p: u32) -> bool {
        if self.rank[p as usize] != 0 || self.vf[p as usize].len() != 2 || self.is_border(p) {
            return false;
        }
        let (f1, f2) = (self.vf[p as usize][0], self.vf[p as usize][1]);
        let rot = |q: [u32; 4]| {
            let i = q.iter().position(|&x| x == p).unwrap();
            [q[i], q[(i + 1) % 4], q[(i + 2) % 4], q[(i + 3) % 4]]
        };
        let (a1, a2) = (rot(self.f[f1 as usize]), rot(self.f[f2 as usize]));
        // a1 = p a x b, a2 = p b y a.
        if a2[1] != a1[3] || a2[3] != a1[1] || a1[2] == a2[2] {
            return false;
        }
        let r = self.fnormal(f1) + self.fnormal(f2);
        if !self.try_replace(&[f1, f2], &[[a1[1], a1[2], a1[3], a2[2]]], &[r], &[p], &[], false) {
            return false;
        }
        self.take_samples(p, a1[1]);
        true
    }

    /// Turns the edge a-b inside the hexagon its two quads make, when that brings points nearer four neighbours.
    fn rotate(&mut self, a: u32, b: u32) -> bool {
        if self.feat.contains(&key(a, b)) {
            return false;
        }
        let fs: Vec<u32> = self.vf[a as usize].iter().copied().filter(|&f| self.f[f as usize].contains(&b)).collect();
        if fs.len() != 2 {
            return false;
        }
        let rot = |q: [u32; 4], x: u32| {
            let i = q.iter().position(|&p| p == x).unwrap();
            [q[i], q[(i + 1) % 4], q[(i + 2) % 4], q[(i + 3) % 4]]
        };
        let (mut g1, mut g2) = (rot(self.f[fs[0] as usize], a), rot(self.f[fs[1] as usize], b));
        let (mut f1, mut f2) = (fs[0], fs[1]);
        if g1[1] != b {
            g1 = rot(self.f[fs[1] as usize], a);
            g2 = rot(self.f[fs[0] as usize], b);
            std::mem::swap(&mut f1, &mut f2);
        }
        // g1 = a b c d, g2 = b a e f.
        if g1[1] != b || g2[1] != a {
            return false;
        }
        let (c, d, e, f) = (g1[2], g1[3], g2[2], g2[3]);
        let before = self.energy(a, 0) + self.energy(b, 0) + self.energy(c, 0) + self.energy(d, 0) + self.energy(e, 0) + self.energy(f, 0);
        let base = self.energy(a, -1) + self.energy(b, -1);
        let opt_a = base + self.energy(c, 1) + self.energy(e, 1) + self.energy(d, 0) + self.energy(f, 0);
        let opt_b = base + self.energy(d, 1) + self.energy(f, 1) + self.energy(c, 0) + self.energy(e, 0);
        let r = self.fnormal(f1) + self.fnormal(f2);
        let mut opts = vec![(opt_a, [[c, d, a, e], [e, f, b, c]]), (opt_b, [[d, a, e, f], [f, b, c, d]])];
        opts.sort_by_key(|o| o.0);
        for (en, add) in opts {
            if en < before && self.try_replace(&[f1, f2], &add, &[r, r], &[], &[], true) {
                return true;
            }
        }
        false
    }

    fn edges(&self) -> Vec<(u32, u32)> {
        let mut e = Vec::new();
        for (i, q) in self.f.iter().enumerate() {
            if !self.dead[i] {
                for k in 0..4 {
                    e.push(key(q[k], q[(k + 1) % 4]));
                }
            }
        }
        e.sort_unstable();
        e.dedup();
        e
    }

    pub fn rotate_all(&mut self) -> usize {
        let mut n = 0;
        for (a, b) in self.edges() {
            if !self.vf[a as usize].is_empty() && self.rotate(a, b) {
                n += 1;
            }
        }
        n
    }

    pub fn doublets_all(&mut self) -> usize {
        let mut n = 0;
        for p in 0..self.v.len() as u32 {
            if self.doublet(p) {
                n += 1;
            }
        }
        // A point along a curve with one quad: that quad's far corner joins it, so the curve's points each have a
        // quad on both sides.
        for p in 0..self.v.len() as u32 {
            if self.rank[p as usize] == 1 && self.vf[p as usize].len() == 1 && self.is_border(p) {
                let q = self.vf[p as usize][0];
                let w = self.f[q as usize].iter().position(|&x| x == p).unwrap();
                let any = |_: DVec3| f64::MAX;
                if self.diag_collapse(q, w % 2, &any).is_some() {
                    n += 1;
                }
            }
        }
        n
    }

    fn vnormal(&self, p: u32) -> DVec3 {
        self.vf[p as usize].iter().map(|&f| self.fnormal(f)).sum::<DVec3>().normalize_or_zero()
    }

    /// Points slid toward the middle of their faces along the surface and put back on it; feature points along their
    /// curve.
    pub fn smooth(&mut self, surf: &Surface, reach: f64, only: Option<&[u32]>, rate: f64) {
        let list: Vec<u32> = match only {
            Some(l) => l.to_vec(),
            None => (0..self.v.len() as u32).collect(),
        };
        let mut moves = Vec::with_capacity(list.len());
        for &p in &list {
            if self.vf[p as usize].is_empty() || self.rank[p as usize] == 2 {
                continue;
            }
            let x = self.v[p as usize];
            if self.rank[p as usize] == 1 {
                let nb: Vec<u32> = self.neighbors(p).into_iter().filter(|&q| self.feat.contains(&key(p, q))).collect();
                if nb.len() != 2 {
                    continue;
                }
                let c = (self.v[nb[0] as usize] + self.v[nb[1] as usize]) * 0.5;
                let dir = self.v[nb[1] as usize] - self.v[nb[0] as usize];
                if let Some(y) = surf.on_feature_along(x.lerp(c, rate), dir) {
                    moves.push((p, y));
                }
                continue;
            }
            let k = self.vf[p as usize].len() as f64;
            let c = self.vf[p as usize].iter().map(|&f| self.pts(self.f[f as usize]).iter().sum::<DVec3>() / 4.0).sum::<DVec3>() / k;
            let nb = self.neighbors(p);
            let c2 = nb.iter().map(|&q| self.v[q as usize]).sum::<DVec3>() / nb.len().max(1) as f64;
            let c = (c + c2) * 0.5;
            let n = self.vnormal(p);
            let d = c - x;
            let y = x + (d - n * n.dot(d)) * rate;
            let q = match surf.project(y, n, reach) {
                Some(q) => Some(q),
                None if self.onto => Some(surf.onto(y, n, reach)),
                None => None,
            };
            if let Some(q) = q {
                moves.push((p, surf.off_curves(q, c2, n, reach)));
            }
        }
        for (p, y) in moves {
            self.v[p as usize] = y;
        }
    }

    /// Free points put on the surface, feature points on their curves.
    pub fn snap(&mut self, surf: &Surface, reach: f64) {
        for p in 0..self.v.len() as u32 {
            if self.vf[p as usize].is_empty() {
                continue;
            }
            let x = self.v[p as usize];
            let y = match self.rank[p as usize] {
                0 => {
                    let nb = self.neighbors(p);
                    let c = nb.iter().map(|&q| self.v[q as usize]).sum::<DVec3>() / nb.len().max(1) as f64;
                    let n = self.vnormal(p);
                    Some(surf.off_curves(surf.onto(x, n, reach), c, n, reach))
                }
                1 => {
                    let nb: Vec<u32> = self.neighbors(p).into_iter().filter(|&q| self.feat.contains(&key(p, q))).collect();
                    let dir = if nb.len() == 2 { self.v[nb[1] as usize] - self.v[nb[0] as usize] } else { DVec3::ZERO };
                    surf.on_feature_along(x, dir)
                }
                _ => None,
            };
            if let Some(y) = y {
                self.v[p as usize] = y;
            }
        }
    }

    /// Points at a corner that turns the wrong way move to their neighbours' middle, a few times over.
    pub fn untangle(&mut self, surf: &Surface, reach: f64) {
        for _ in 0..8 {
            let mut bad = HashSet::new();
            for (fi, q) in self.f.iter().enumerate() {
                if self.dead[fi] {
                    continue;
                }
                let p = self.pts(*q);
                let n = surf.normal_at((p[0] + p[1] + p[2] + p[3]) * 0.25);
                for i in 0..4 {
                    let c = (p[(i + 1) % 4] - p[i]).cross(p[(i + 3) % 4] - p[i]);
                    if c.dot(n) <= 0.0 {
                        bad.insert(q[i]);
                    }
                }
            }
            if bad.is_empty() {
                return;
            }
            let list: Vec<u32> = bad.into_iter().flat_map(|p| std::iter::once(p).chain(self.neighbors(p))).collect();
            self.smooth(surf, reach, Some(&list), 1.0);
        }
    }

    /// Simplifies to about `target` faces: the quad smallest against `size` at its middle collapsed first.
    /// Copies are kept as it passes each of `keep_at` faces, cleaned up first; they're returned with the face count
    /// they were taken at.
    pub fn coarsen(&mut self, surf: &Surface, target: usize, size: &dyn Fn(DVec3) -> f64, reach: f64, keep_at: &[usize]) -> Vec<QMesh> {
        let mut kept: Vec<QMesh> = Vec::new();
        let mut next_keep = 0usize;
        let mut keep_at: Vec<usize> = keep_at.to_vec();
        keep_at.sort_unstable_by(|a, b| b.cmp(a));
        let prio = |m: &QMesh, q: u32| {
            let p = m.pts(m.f[q as usize]);
            let d = (p[0] - p[2]).length().min((p[1] - p[3]).length());
            d / size((p[0] + p[1] + p[2] + p[3]) * 0.25).max(1e-12)
        };
        let mut heap: BinaryHeap<Reverse<(Ord64, u32, u32)>> = BinaryHeap::new();
        let fill = |m: &QMesh, heap: &mut BinaryHeap<Reverse<(Ord64, u32, u32)>>| {
            heap.clear();
            for q in 0..m.f.len() as u32 {
                if !m.dead[q as usize] {
                    heap.push(Reverse((Ord64(prio(m, q)), q, m.stamp[q as usize])));
                }
            }
        };
        fill(self, &mut heap);
        let mut since = 0usize;
        let mut period = 0usize;
        let mut stuck = 0;
        while self.live > target {
            while next_keep < keep_at.len() && self.live <= keep_at[next_keep] {
                let mut c = self.clone();
                c.rotate_all();
                c.doublets_all();
                for _ in 0..3 {
                    c.smooth(surf, reach, None, 0.5);
                }
                kept.push(c);
                next_keep += 1;
            }
            let Some(Reverse((_, q, st))) = heap.pop() else {
                since += self.chord_pass(size);
                if since == 0 {
                    stuck += 1;
                    if stuck > 2 {
                        break;
                    }
                }
                self.rotate_all();
                self.doublets_all();
                self.smooth(surf, reach, None, 0.5);
                fill(self, &mut heap);
                since = 0;
                continue;
            };
            if self.dead[q as usize] || self.stamp[q as usize] != st {
                continue;
            }
            let p = self.pts(self.f[q as usize]);
            let order = if (p[0] - p[2]).length() <= (p[1] - p[3]).length() { [0, 1] } else { [1, 0] };
            let mut kept = None;
            for w in order {
                if let Some(k) = self.diag_collapse(q, w, size) {
                    kept = Some(k);
                    break;
                }
            }
            let Some(k) = kept else { continue };
            since += 1;
            stuck = 0;
            for n in self.neighbors(k) {
                self.doublet(n);
            }
            let mut ring: Vec<u32> = vec![k];
            ring.extend(self.neighbors(k));
            for _ in 0..2 {
                self.smooth(surf, reach, Some(&ring), 0.5);
            }
            let mut touched: Vec<u32> = ring.iter().flat_map(|&p| self.vf[p as usize].clone()).collect();
            touched.sort_unstable();
            touched.dedup();
            for fi in touched {
                self.stamp[fi as usize] += 1;
                heap.push(Reverse((Ord64(prio(self, fi)), fi, self.stamp[fi as usize])));
            }
            period += 1;
            if period >= (self.live / 6).max(64) {
                period = 0;
                self.rotate_all();
                self.doublets_all();
                self.smooth(surf, reach, None, 0.5);
                fill(self, &mut heap);
            }
        }
        for _ in 0..4 {
            let n = self.rotate_all() + self.doublets_all();
            for _ in 0..3 {
                self.smooth(surf, reach, None, 0.5);
            }
            if n == 0 {
                break;
            }
        }
        kept
    }

    /// The row of quads through quad q across its edge k, both ways: each quad with the corner its first crossed
    /// edge starts at. None when the row crosses itself.
    fn chord(&self, q: u32, k: usize) -> Option<Vec<(u32, usize)>> {
        let other = |fi: u32, a: u32, b: u32| -> Option<(u32, usize)> {
            for &g in &self.vf[a as usize] {
                if g != fi {
                    let t = self.f[g as usize];
                    for i in 0..4 {
                        if (t[i] == a && t[(i + 1) % 4] == b) || (t[i] == b && t[(i + 1) % 4] == a) {
                            return Some((g, i));
                        }
                    }
                }
            }
            None
        };
        let mut out = vec![(q, k)];
        let mut seen = HashSet::new();
        seen.insert(q);
        let mut closed = false;
        // Forward across edge k+2, then backward across edge k.
        for dir in 0..2 {
            if closed {
                break;
            }
            let (mut cur, mut e) = (q, if dir == 0 { (k + 2) % 4 } else { k });
            loop {
                let t = self.f[cur as usize];
                let Some((g, i)) = other(cur, t[e], t[(e + 1) % 4]) else { break };
                if g == q {
                    closed = true;
                    break;
                }
                if !seen.insert(g) {
                    return None;
                }
                out.push((g, i));
                cur = g;
                e = (i + 2) % 4;
            }
        }
        Some(out)
    }

    /// Splits the row of quads through quad q across its edge k down the middle: each crossing edge gets a point.
    /// Returns the new points, or nothing when the row crosses itself.
    pub fn split_chord(&mut self, q: u32, k: usize) -> Vec<u32> {
        let Some(ch) = self.chord(q, k) else { return Vec::new() };
        let mut mids: HashMap<(u32, u32), u32> = HashMap::new();
        let mut made = Vec::new();
        let mut adds = Vec::new();
        for &(g, i) in &ch {
            let t = self.f[g as usize];
            let r = [t[i], t[(i + 1) % 4], t[(i + 2) % 4], t[(i + 3) % 4]];
            let mut mid = |a: u32, b: u32, m: &mut QMesh| -> u32 {
                *mids.entry(key(a, b)).or_insert_with(|| {
                    let x = m.v.len() as u32;
                    m.v.push((m.v[a as usize] + m.v[b as usize]) * 0.5);
                    m.vf.push(Vec::new());
                    if !m.samples.is_empty() {
                        m.samples.push(Vec::new());
                    }
                    let on = m.feat.remove(&key(a, b));
                    m.rank.push(on as u8);
                    if on {
                        m.feat.insert(key(a, x));
                        m.feat.insert(key(x, b));
                    }
                    made.push(x);
                    x
                })
            };
            let m0 = mid(r[0], r[1], self);
            let m1 = mid(r[2], r[3], self);
            adds.push([r[0], m0, m1, r[3]]);
            adds.push([m0, r[1], r[2], m1]);
        }
        for &(g, _) in &ch {
            self.dead[g as usize] = true;
            self.live -= 1;
            for p in self.f[g as usize] {
                self.vf[p as usize].retain(|&x| x != g);
            }
        }
        for q in adds {
            let i = self.f.len() as u32;
            self.f.push(q);
            self.dead.push(false);
            self.stamp.push(0);
            self.live += 1;
            for &p in &q {
                self.vf[p as usize].push(i);
            }
        }
        made
    }

    /// Inner edges with both ends on the plane, which a mirror would give four faces: each gets its row split,
    /// and the new point between them moved off the plane onto the surface.
    pub fn clear_plane(&mut self, surf: &Surface, plane: &crate::symmetry::SymmetryPlane, tol: f64, reach: f64) {
        // Free points that ended on the plane (the surface's nearest point to them was on its cut) move back
        // toward their neighbours off it.
        for p in 0..self.v.len() as u32 {
            if self.vf[p as usize].is_empty() || self.rank[p as usize] != 0 || plane.distance(self.v[p as usize]).abs() >= tol {
                continue;
            }
            let off: Vec<DVec3> = self.neighbors(p).iter().map(|&n| self.v[n as usize]).filter(|y| plane.distance(*y).abs() >= tol).collect();
            if off.is_empty() {
                continue;
            }
            let c = off.iter().sum::<DVec3>() / off.len() as f64;
            let x = self.v[p as usize];
            let y = x.lerp(c, 0.3);
            let n = self.vnormal(p);
            self.v[p as usize] = surf.project(y, n, reach).filter(|q| plane.distance(*q).abs() >= tol).unwrap_or(y);
        }
        for _ in 0..8 {
            let on = |m: &QMesh, p: u32| plane.distance(m.v[p as usize]).abs() < tol;
            let mut found = None;
            'outer: for (fi, t) in self.f.iter().enumerate() {
                if self.dead[fi] {
                    continue;
                }
                for k in 0..4 {
                    let (a, b) = (t[k], t[(k + 1) % 4]);
                    if on(self, a) && on(self, b) && self.is_border(a) && self.is_border(b) {
                        let shared = self.vf[a as usize].iter().filter(|&&g| self.f[g as usize].contains(&b)).count();
                        if shared == 2 {
                            found = Some((fi as u32, k));
                            break 'outer;
                        }
                    }
                }
            }
            let Some((q, k)) = found else { return };
            let made = self.split_chord(q, k);
            if made.is_empty() {
                return;
            }
            for p in made {
                let x = self.v[p as usize];
                if plane.distance(x).abs() < tol && self.rank[p as usize] == 0 {
                    // Toward its neighbours off the plane.
                    let off: Vec<DVec3> = self.neighbors(p).iter().map(|&n| self.v[n as usize]).filter(|y| plane.distance(*y).abs() >= tol).collect();
                    if !off.is_empty() {
                        let c = off.iter().sum::<DVec3>() / off.len() as f64;
                        let y = x.lerp(c, 0.5);
                        self.v[p as usize] = surf.onto(y, self.vnormal(p), reach);
                    }
                } else if self.rank[p as usize] == 0 {
                    self.v[p as usize] = surf.onto(x, self.vnormal(p), reach);
                }
            }
        }
    }

    /// Removes a row of quads, each of its crossing edges collapsing to a point.
    fn chord_collapse(&mut self, chord: &[(u32, usize)], size: &dyn Fn(DVec3) -> f64) -> bool {
        let quads: Vec<u32> = chord.iter().map(|c| c.0).collect();
        let mut rungs: Vec<(u32, u32)> = Vec::new();
        for &(q, i) in chord {
            let t = self.f[q as usize];
            for j in [i, (i + 2) % 4] {
                let k = key(t[j], t[(j + 1) % 4]);
                if !rungs.contains(&k) {
                    rungs.push(k);
                }
            }
        }
        // Each point in one crossing edge at most.
        let mut owner: HashMap<u32, usize> = HashMap::new();
        for (r, &(a, b)) in rungs.iter().enumerate() {
            if owner.insert(a, r).is_some() || owner.insert(b, r).is_some() {
                return false;
            }
        }
        let mut gone = Vec::new();
        let mut moves = Vec::new();
        let mut map: HashMap<u32, u32> = HashMap::new();
        for &(a, b) in &rungs {
            let (ra, rb) = (self.rank[a as usize], self.rank[b as usize]);
            let fe = self.feat.contains(&key(a, b));
            if ra >= 1 && rb >= 1 && (!fe || (ra == 2 && rb == 2)) {
                return false;
            }
            let (keep, go) = if rb > ra { (b, a) } else { (a, b) };
            let pos = if ra == rb { (self.v[a as usize] + self.v[b as usize]) * 0.5 } else { self.v[keep as usize] };
            gone.push(go);
            moves.push((keep, pos));
            map.insert(go, keep);
        }
        let mut remove: Vec<u32> = quads.clone();
        for &g in &gone {
            for &fi in &self.vf[g as usize] {
                if !remove.contains(&fi) {
                    remove.push(fi);
                }
            }
        }
        let mut add = Vec::new();
        let mut refs = Vec::new();
        for &fi in &remove {
            if quads.contains(&fi) {
                continue;
            }
            add.push(self.f[fi as usize].map(|p| *map.get(&p).unwrap_or(&p)));
            refs.push(self.fnormal(fi));
        }
        let at = |p: u32| moves.iter().find(|m| m.0 == p).map(|m| m.1).unwrap_or(self.v[p as usize]);
        for f in &add {
            let p = f.map(at);
            let c = (p[0] + p[1] + p[2] + p[3]) * 0.25;
            if (p[0] - p[2]).length().max((p[1] - p[3]).length()) > 2.2 * size(c) {
                return false;
            }
        }
        for &(keep, _) in &moves {
            for &fi in &self.vf[keep as usize] {
                if remove.contains(&fi) {
                    continue;
                }
                let p = self.f[fi as usize].map(at);
                let c = (p[0] + p[1] + p[2] + p[3]) * 0.25;
                if (p[0] - p[2]).length().max((p[1] - p[3]).length()) > 2.2 * size(c) {
                    return false;
                }
            }
        }
        if !self.samples.is_empty() {
            let mut around = add.clone();
            for &(keep, _) in &moves {
                for &fi in &self.vf[keep as usize] {
                    if !remove.contains(&fi) {
                        around.push(self.f[fi as usize]);
                    }
                }
            }
            let smp: Vec<DVec3> = moves.iter().map(|m| m.0).chain(gone.iter().copied()).flat_map(|p| self.samples[p as usize].clone()).collect();
            if !self.fits(&smp, &around, &at, size) {
                return false;
            }
        }
        self.remap = map.iter().map(|(&g, &k)| (g, k)).collect();
        let ok = self.try_replace(&remove, &add, &refs, &gone, &moves, false);
        self.remap.clear();
        if !ok {
            return false;
        }
        for (&go, &keep) in &map {
            self.take_samples(go, keep);
        }
        for (&go, &keep) in &map {
            let old: Vec<(u32, u32)> = self.feat.iter().copied().filter(|k| k.0 == go || k.1 == go).collect();
            for k in old {
                self.feat.remove(&k);
                let o = if k.0 == go { k.1 } else { k.0 };
                let o = *map.get(&o).unwrap_or(&o);
                if o != keep {
                    self.feat.insert(key(o, keep));
                }
            }
        }
        true
    }

    /// Rows of quads narrow against the wanted size removed, narrowest first. Returns how many.
    pub fn chord_pass(&mut self, size: &dyn Fn(DVec3) -> f64) -> usize {
        let mut cands: Vec<(f64, u32, usize)> = Vec::new();
        let mut done: HashSet<(u32, usize)> = HashSet::new();
        for q in 0..self.f.len() as u32 {
            if self.dead[q as usize] {
                continue;
            }
            for k in 0..2 {
                if done.contains(&(q, k)) {
                    continue;
                }
                let Some(ch) = self.chord(q, k) else { continue };
                let mut sum = 0.0;
                let mut n = 0.0;
                for &(g, i) in &ch {
                    done.insert((g, i % 2));
                    let t = self.f[g as usize];
                    for j in [i, (i + 2) % 4] {
                        let (a, b) = (self.v[t[j] as usize], self.v[t[(j + 1) % 4] as usize]);
                        sum += (a - b).length() / size((a + b) * 0.5).max(1e-12);
                        n += 1.0;
                    }
                }
                let r = sum / n;
                if r < 0.75 {
                    cands.push((r, q, k));
                }
            }
        }
        cands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
        let mut count = 0;
        for (_, q, k) in cands {
            if self.dead[q as usize] {
                continue;
            }
            if let Some(ch) = self.chord(q, k) {
                if self.chord_collapse(&ch, size) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Each row of quads gets a whole number of new rows: the row's edges against `want` (the edge length wanted at
    /// a point along a direction), scaled together so the faces come near `target` (0 keeps `want` as it is). Then
    /// every quad becomes its grid, neighbours sharing the points along their common edge.
    pub fn refine(&self, want: &dyn Fn(DVec3, DVec3) -> f64, target: usize) -> QMesh {
        let mut eid: HashMap<(u32, u32), usize> = HashMap::new();
        let mut elist: Vec<(u32, u32)> = Vec::new();
        let mut qe: Vec<(u32, [usize; 4])> = Vec::new();
        for (fi, q) in self.f.iter().enumerate() {
            if self.dead[fi] {
                continue;
            }
            let mut ids = [0usize; 4];
            for k in 0..4 {
                let kk = key(q[k], q[(k + 1) % 4]);
                ids[k] = *eid.entry(kk).or_insert_with(|| {
                    elist.push(kk);
                    elist.len() - 1
                });
            }
            qe.push((fi as u32, ids));
        }
        let ne = elist.len();
        let mut parent: Vec<usize> = (0..ne).collect();
        fn find(p: &mut [usize], x: usize) -> usize {
            let mut r = x;
            while p[r] != r {
                r = p[r];
            }
            let mut y = x;
            while p[y] != r {
                let n = p[y];
                p[y] = r;
                y = n;
            }
            r
        }
        for (_, ids) in &qe {
            for (a, b) in [(ids[0], ids[2]), (ids[1], ids[3])] {
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                if ra != rb {
                    parent[ra.max(rb)] = ra.min(rb);
                }
            }
        }
        let mut sum = vec![0.0f64; ne];
        let mut cnt = vec![0.0f64; ne];
        for (i, &(a, b)) in elist.iter().enumerate() {
            let (pa, pb) = (self.v[a as usize], self.v[b as usize]);
            let r = (pb - pa).length() / want((pa + pb) * 0.5, pb - pa).max(1e-12);
            let c = find(&mut parent, i);
            sum[c] += r;
            cnt[c] += 1.0;
        }
        let class: Vec<usize> = (0..ne).map(|i| find(&mut parent, i)).collect();
        let mean: Vec<f64> = (0..ne).map(|c| if cnt[c] > 0.0 { sum[c] / cnt[c] } else { 0.0 }).collect();
        let n_of = |s: f64, e: usize| ((mean[class[e]] / s).round() as usize).max(1);
        let count = |s: f64| qe.iter().map(|(_, ids)| n_of(s, ids[0]) * n_of(s, ids[1])).sum::<usize>();
        let mut s = 1.0;
        if target > 0 {
            let (mut lo, mut hi) = (1e-3f64, 1e3f64);
            let mut best = (usize::MAX, 1.0);
            for _ in 0..60 {
                let mid = (lo * hi).sqrt();
                let c = count(mid);
                let miss = (c as i64 - target as i64).unsigned_abs() as usize;
                if miss < best.0 {
                    best = (miss, mid);
                }
                if c > target {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            s = best.1;
        }
        // Then single rows one more or one fewer, those rounded furthest the other way first, while that brings the
        // count nearer.
        let mut n: Vec<usize> = (0..ne).map(|c| if cnt[c] > 0.0 { ((mean[c] / s).round() as usize).max(1) } else { 1 }).collect();
        if target > 0 {
            // Per row: the quads it crosses, as the other direction's row.
            let mut across: HashMap<usize, Vec<usize>> = HashMap::new();
            for (_, ids) in &qe {
                across.entry(class[ids[0]]).or_default().push(class[ids[1]]);
                across.entry(class[ids[1]]).or_default().push(class[ids[0]]);
            }
            let rows: Vec<usize> = {
                let mut r: Vec<usize> = across.keys().copied().collect();
                r.sort_unstable();
                r
            };
            let total = |n: &[usize]| qe.iter().map(|(_, ids)| n[class[ids[0]]] * n[class[ids[1]]]).sum::<usize>();
            let mut cur = total(&n) as i64;
            for _ in 0..rows.len() * 20 + 100 {
                let miss = cur - target as i64;
                if miss.unsigned_abs() as f64 <= target as f64 * 0.02 {
                    break;
                }
                let mut best: Option<(f64, usize, i64)> = None;
                for &c in &rows {
                    let step: i64 = if miss > 0 { -1 } else { 1 };
                    if step < 0 && n[c] <= 1 {
                        continue;
                    }
                    let delta: i64 = across[&c].iter().map(|&o| if o == c { 2 * n[c] as i64 + step } else { n[o] as i64 }).sum::<i64>() * step;
                    if (miss + delta).abs() >= miss.abs() {
                        continue;
                    }
                    // How far this row's count would sit from its ideal.
                    let off = ((n[c] as i64 + step) as f64 - mean[c] / s).abs();
                    if best.map(|b| off < b.0).unwrap_or(true) {
                        best = Some((off, c, delta));
                    }
                }
                let Some((_, c, delta)) = best else { break };
                n[c] = (n[c] as i64 + if miss > 0 { -1 } else { 1 }) as usize;
                cur += delta;
            }
        }
        let n_of = |_s: f64, e: usize| n[class[e]];
        // Points: the coarse ones, then those along each edge from its smaller point.
        let mut v = self.v.clone();
        let mut rank = self.rank.clone();
        let mut feat = HashSet::new();
        let mut along: Vec<Vec<u32>> = Vec::with_capacity(ne);
        for (i, &(a, b)) in elist.iter().enumerate() {
            let n = n_of(s, i);
            let on = self.feat.contains(&(a, b));
            let mut pts = vec![a];
            for k in 1..n {
                pts.push(v.len() as u32);
                v.push(self.v[a as usize].lerp(self.v[b as usize], k as f64 / n as f64));
                rank.push(on as u8);
            }
            pts.push(b);
            if on {
                for w in pts.windows(2) {
                    feat.insert(key(w[0], w[1]));
                }
            }
            along.push(pts);
        }
        let mut f = Vec::new();
        for (fi, ids) in &qe {
            let q = self.f[*fi as usize];
            let (nu, nv) = (n_of(s, ids[0]), n_of(s, ids[1]));
            // Points of edge k running from q[k] to q[k+1].
            let run = |k: usize| -> Vec<u32> {
                let e = &along[ids[k]];
                if e[0] == q[k] {
                    e.clone()
                } else {
                    e.iter().rev().copied().collect()
                }
            };
            let bottom = run(0); // q0 -> q1, i = 0..nu
            let right = run(1); // q1 -> q2, j = 0..nv
            let top: Vec<u32> = run(2).into_iter().rev().collect(); // q3 -> q2
            let left: Vec<u32> = run(3).into_iter().rev().collect(); // q0 -> q3
            let mut g = vec![vec![0u32; nv + 1]; nu + 1];
            for i in 0..=nu {
                g[i][0] = bottom[i];
                g[i][nv] = top[i];
            }
            for j in 0..=nv {
                g[0][j] = left[j];
                g[nu][j] = right[j];
            }
            let c = [q[0], q[1], q[2], q[3]].map(|p| self.v[p as usize]);
            for i in 1..nu {
                for j in 1..nv {
                    let (u, w) = (i as f64 / nu as f64, j as f64 / nv as f64);
                    let p = v[bottom[i] as usize] * (1.0 - w) + v[top[i] as usize] * w + v[left[j] as usize] * (1.0 - u) + v[right[j] as usize] * u
                        - (c[0] * (1.0 - u) * (1.0 - w) + c[1] * u * (1.0 - w) + c[2] * u * w + c[3] * (1.0 - u) * w);
                    g[i][j] = v.len() as u32;
                    v.push(p);
                    rank.push(0);
                }
            }
            for i in 0..nu {
                for j in 0..nv {
                    f.push([g[i][j], g[i + 1][j], g[i + 1][j + 1], g[i][j + 1]]);
                }
            }
        }
        QMesh::new(v, f, rank, feat)
    }

    /// Live points and faces with their mirror across the plane added, joined along the border edges that lie on
    /// the plane: only those border points are shared, so no other edge can end up with four faces.
    pub fn mirrored(&self, plane: &crate::symmetry::SymmetryPlane, tol: f64) -> (Vec<DVec3>, Vec<[u32; 4]>) {
        let (v, f) = self.compact();
        let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
        for q in &f {
            for i in 0..4 {
                *uses.entry(key(q[i], q[(i + 1) % 4])).or_default() += 1;
            }
        }
        let on = |p: u32| plane.distance(v[p as usize]).abs() < tol;
        let mut shared = vec![false; v.len()];
        for (&(a, b), &n) in &uses {
            if n == 1 && on(a) && on(b) {
                shared[a as usize] = true;
                shared[b as usize] = true;
            }
        }
        let mut out_v = v.clone();
        for (i, p) in out_v.iter_mut().enumerate() {
            if shared[i] {
                *p = plane.project(*p);
            }
        }
        let mut map = vec![0u32; v.len()];
        for i in 0..v.len() {
            if shared[i] {
                map[i] = i as u32;
            } else {
                map[i] = out_v.len() as u32;
                out_v.push(plane.reflect(v[i]));
            }
        }
        let mut out_f = f.clone();
        for q in &f {
            out_f.push([map[q[3] as usize], map[q[2] as usize], map[q[1] as usize], map[q[0] as usize]]);
        }
        (out_v, out_f)
    }

    /// Live points and faces, renumbered from 0.
    pub fn compact(&self) -> (Vec<DVec3>, Vec<[u32; 4]>) {
        let mut map = vec![u32::MAX; self.v.len()];
        let mut v = Vec::new();
        let mut f = Vec::new();
        for (i, q) in self.f.iter().enumerate() {
            if self.dead[i] {
                continue;
            }
            f.push(q.map(|p| {
                if map[p as usize] == u32::MAX {
                    map[p as usize] = v.len() as u32;
                    v.push(self.v[p as usize]);
                }
                map[p as usize]
            }));
        }
        (v, f)
    }
}
