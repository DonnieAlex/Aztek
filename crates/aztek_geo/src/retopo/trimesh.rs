//! Even triangles over the source: edges split when too long, collapsed when too short, flipped toward six per
//! point, and points slid along the surface (Botsch and Kobbelt 2004). Feature curves are kept: their points only
//! slide along them, and their corners never move. Every collapse keeps the mesh manifold (the link condition).

use super::surface::Surface;
use super::tri::key;
use glam::DVec3;
use std::collections::HashSet;

pub struct TMesh {
    pub v: Vec<DVec3>,
    pub f: Vec<[u32; 3]>,
    pub dead: Vec<bool>,
    pub vf: Vec<Vec<u32>>,
    /// 0 free, 1 on a feature curve, 2 a corner of features (fixed).
    pub rank: Vec<u8>,
    pub feat: HashSet<(u32, u32)>,
    live: usize,
    /// As for QMesh: no edge joins two points on this plane except along its cut.
    pub plane: Option<(crate::symmetry::SymmetryPlane, f64)>,
    /// Per point: the source points it has absorbed, which its triangles must stay near.
    samples: Vec<Vec<DVec3>>,
}

/// Rank of each point from its feature edges: none 0, two running on 1, else 2. Whether a point with two is a
/// corner is judged over `span` either way along its curve, so a curve that wiggles at the triangles' scale (a
/// border cut through a scan) isn't all corners.
pub fn ranks(v: &[DVec3], feat: &HashSet<(u32, u32)>, soft: &HashSet<(u32, u32)>, span: f64) -> Vec<u8> {
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); v.len()];
    for &(a, b) in feat {
        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }
    let reach = |p: u32, first: u32, span: f64| -> DVec3 {
        let (mut prev, mut cur) = (p, first);
        let mut len = (v[cur as usize] - v[p as usize]).length();
        while len < span && adj[cur as usize].len() == 2 && cur != p {
            let next = if adj[cur as usize][0] == prev { adj[cur as usize][1] } else { adj[cur as usize][0] };
            len += (v[next as usize] - v[cur as usize]).length();
            prev = cur;
            cur = next;
        }
        (v[cur as usize] - v[p as usize]).normalize_or_zero()
    };
    (0..v.len())
        .map(|i| match adj[i].len() {
            0 => 0,
            2 => {
                // Borders and the symmetry cut are judged over the span; sharp edges at their own points.
                let sp = if soft.contains(&key(i as u32, adj[i][0])) && soft.contains(&key(i as u32, adj[i][1])) { span } else { 0.0 };
                let (d0, d1) = (reach(i as u32, adj[i][0], sp), reach(i as u32, adj[i][1], sp));
                // A border may bend a long way round before it's a corner; a sharp edge 45 degrees.
                let limit: f64 = if sp > 0.0 { 100.0 } else { 45.0 };
                if d0.dot(-d1) > limit.to_radians().cos() {
                    1
                } else {
                    2
                }
            }
            _ => 2,
        })
        .collect()
}

impl TMesh {
    pub fn new(v: Vec<DVec3>, f: Vec<[u32; 3]>, feat: HashSet<(u32, u32)>, soft: &HashSet<(u32, u32)>, span: f64) -> TMesh {
        let mut vf = vec![Vec::new(); v.len()];
        for (i, t) in f.iter().enumerate() {
            for &p in t {
                vf[p as usize].push(i as u32);
            }
        }
        let rank = ranks(&v, &feat, soft, span);
        let samples = v.iter().map(|p| vec![*p]).collect();
        TMesh { dead: vec![false; f.len()], live: f.len(), v, f, vf, rank, feat, plane: None, samples }
    }

    pub fn alive_faces(&self) -> usize {
        self.live
    }

    fn edge_faces(&self, a: u32, b: u32) -> Vec<u32> {
        self.vf[a as usize].iter().copied().filter(|&f| self.f[f as usize].contains(&b)).collect()
    }

    fn neighbors(&self, a: u32) -> Vec<u32> {
        let mut n: Vec<u32> = self.vf[a as usize].iter().flat_map(|&f| self.f[f as usize]).filter(|&p| p != a).collect();
        n.sort_unstable();
        n.dedup();
        n
    }

    fn edges(&self) -> Vec<(u32, u32)> {
        let mut e = Vec::new();
        for (i, t) in self.f.iter().enumerate() {
            if self.dead[i] {
                continue;
            }
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                e.push(key(a, b));
            }
        }
        e.sort_unstable();
        e.dedup();
        e
    }

    fn normal(&self, t: [u32; 3]) -> DVec3 {
        let (a, b, c) = (self.v[t[0] as usize], self.v[t[1] as usize], self.v[t[2] as usize]);
        (b - a).cross(c - a)
    }

    fn vnormal(&self, p: u32) -> DVec3 {
        self.vf[p as usize].iter().map(|&f| self.normal(self.f[f as usize])).sum::<DVec3>().normalize_or_zero()
    }

    fn border_point(&self, p: u32) -> bool {
        self.neighbors(p).iter().any(|&q| self.edge_faces(p, q).len() == 1)
    }

    fn add_face(&mut self, t: [u32; 3]) -> u32 {
        let i = self.f.len() as u32;
        self.f.push(t);
        self.dead.push(false);
        self.live += 1;
        for &p in &t {
            self.vf[p as usize].push(i);
        }
        i
    }

    fn split(&mut self, a: u32, b: u32) {
        if self.edge_faces(a, b).is_empty() {
            return;
        }
        let m = self.v.len() as u32;
        self.v.push((self.v[a as usize] + self.v[b as usize]) * 0.5);
        self.vf.push(Vec::new());
        self.samples.push(Vec::new());
        let on_feat = self.feat.remove(&key(a, b));
        self.rank.push(if on_feat { 1 } else { 0 });
        if on_feat {
            self.feat.insert(key(a, m));
            self.feat.insert(key(m, b));
        }
        for fi in self.edge_faces(a, b) {
            let t = self.f[fi as usize];
            // Turn the triangle so its first edge is this one, in its own direction.
            let r = (0..3).find(|&k| key(t[k], t[(k + 1) % 3]) == key(a, b)).unwrap();
            let (x, y, z) = (t[r], t[(r + 1) % 3], t[(r + 2) % 3]);
            self.f[fi as usize] = [x, m, z];
            self.vf[y as usize].retain(|&g| g != fi);
            self.vf[m as usize].push(fi);
            self.add_face([m, y, z]);
        }
    }

    /// Collapses a into b at pos when that keeps the mesh manifold, unfolded and no edge longer than max_len.
    fn collapse(&mut self, a: u32, b: u32, pos: DVec3, max_len: f64) -> bool {
        let ab = self.edge_faces(a, b);
        if ab.is_empty() || self.alive_faces() <= 8 {
            return false;
        }
        if self.vnormal(a).dot(self.vnormal(b)) < 0.3 {
            return false;
        }
        let na = self.neighbors(a);
        let nb = self.neighbors(b);
        let common: Vec<u32> = na.iter().copied().filter(|x| nb.contains(x)).collect();
        let mut opp: Vec<u32> = ab.iter().map(|&f| *self.f[f as usize].iter().find(|&&p| p != a && p != b).unwrap()).collect();
        opp.sort_unstable();
        if common != opp {
            return false;
        }
        // A border edge's ends both on the border, or an inner edge joining two border points, would pinch.
        if ab.len() == 2 && self.border_point(a) && self.border_point(b) {
            return false;
        }
        for &q in na.iter().chain(nb.iter()) {
            if q != a && q != b && !self.feat.contains(&key(a, q)) && self.bridge(b, q, pos, self.v[q as usize]) {
                return false;
            }
            if q != a && q != b && (self.v[q as usize] - pos).length() > max_len {
                return false;
            }
        }
        for &p in &[a, b] {
            for &fi in &self.vf[p as usize] {
                if ab.contains(&fi) {
                    continue;
                }
                let t = self.f[fi as usize];
                let before = self.normal(t);
                let after = {
                    let q = t.map(|x| if x == a || x == b { pos } else { self.v[x as usize] });
                    (q[1] - q[0]).cross(q[2] - q[0])
                };
                if after.length() < 1e-12 * before.length().max(1e-30) || before.normalize_or_zero().dot(after.normalize_or_zero()) < 0.3 {
                    return false;
                }
            }
        }
        // The source points under a and b stay near the triangles about the joined point.
        {
            let tol = 0.2 * max_len.min(1e300);
            let mut tris: Vec<[DVec3; 3]> = Vec::new();
            for &p in &[a, b] {
                for &fi in &self.vf[p as usize] {
                    if !ab.contains(&fi) {
                        tris.push(self.f[fi as usize].map(|x| if x == a || x == b { pos } else { self.v[x as usize] }));
                    }
                }
            }
            if max_len.is_finite() {
                for x in self.samples[a as usize].iter().chain(self.samples[b as usize].iter()) {
                    if !tris.iter().any(|t| (crate::bvh::closest_on_triangle(*x, t[0], t[1], t[2]).0 - *x).length() <= tol) {
                        return false;
                    }
                }
            }
        }
        let moved_samples = std::mem::take(&mut self.samples[a as usize]);
        self.samples[b as usize].extend(moved_samples);
        self.v[b as usize] = pos;
        for &fi in &ab {
            self.dead[fi as usize] = true;
            self.live -= 1;
            for p in self.f[fi as usize] {
                self.vf[p as usize].retain(|&g| g != fi);
            }
        }
        let moved = std::mem::take(&mut self.vf[a as usize]);
        for fi in moved {
            for p in self.f[fi as usize].iter_mut() {
                if *p == a {
                    *p = b;
                }
            }
            self.vf[b as usize].push(fi);
        }
        let old: Vec<(u32, u32)> = self.feat.iter().copied().filter(|k| k.0 == a || k.1 == a).collect();
        for k in old {
            self.feat.remove(&k);
            let o = if k.0 == a { k.1 } else { k.0 };
            if o != b {
                self.feat.insert(key(o, b));
            }
        }
        true
    }

    fn try_collapse(&mut self, a: u32, b: u32, max_len: f64) -> bool {
        let fe = self.feat.contains(&key(a, b));
        for (x, y) in [(a, b), (b, a)] {
            let (rx, ry) = (self.rank[x as usize], self.rank[y as usize]);
            let ok = rx == 0 || (rx == 1 && fe && ry >= 1);
            if !ok {
                continue;
            }
            let pos = if ry > rx { self.v[y as usize] } else { (self.v[x as usize] + self.v[y as usize]) * 0.5 };
            if self.collapse(x, y, pos, max_len) {
                return true;
            }
        }
        false
    }

    fn bridge(&self, a: u32, b: u32, pa: DVec3, pb: DVec3) -> bool {
        match &self.plane {
            Some((pl, tol)) => !self.feat.contains(&key(a, b)) && pl.distance(pa).abs() < *tol && pl.distance(pb).abs() < *tol,
            None => false,
        }
    }

    /// Edges joining two points on the symmetry plane other than along its cut get a point between, which the
    /// smoothing then moves off it.
    pub fn split_bridges(&mut self, surf: &Surface, reach: f64) {
        let Some((pl, tol)) = self.plane else { return };
        for (a, b) in self.edges() {
            if self.bridge(a, b, self.v[a as usize], self.v[b as usize]) {
                let m = self.v.len() as u32;
                self.split(a, b);
                if self.v.len() as u32 == m {
                    continue;
                }
                let off: Vec<DVec3> = self.neighbors(m).iter().map(|&q| self.v[q as usize]).filter(|y| pl.distance(*y).abs() >= tol).collect();
                if !off.is_empty() {
                    let c = off.iter().sum::<DVec3>() / off.len() as f64;
                    let y = self.v[m as usize].lerp(c, 0.5);
                    let n = self.vnormal(m);
                    self.v[m as usize] = surf.project(y, n, reach).unwrap_or(y);
                }
            }
        }
    }

    fn flip(&mut self, a: u32, b: u32) -> bool {
        if self.feat.contains(&key(a, b)) {
            return false;
        }
        let ab = self.edge_faces(a, b);
        if ab.len() != 2 {
            return false;
        }
        // f1 runs a to b, f2 b to a.
        let dir = |t: [u32; 3], x: u32, y: u32| (0..3).any(|k| t[k] == x && t[(k + 1) % 3] == y);
        let (f1, f2) = if dir(self.f[ab[0] as usize], a, b) { (ab[0], ab[1]) } else { (ab[1], ab[0]) };
        if !dir(self.f[f1 as usize], a, b) || !dir(self.f[f2 as usize], b, a) {
            return false;
        }
        let c = *self.f[f1 as usize].iter().find(|&&p| p != a && p != b).unwrap();
        let d = *self.f[f2 as usize].iter().find(|&&p| p != a && p != b).unwrap();
        if c == d || self.neighbors(c).contains(&d) || self.bridge(c, d, self.v[c as usize], self.v[d as usize]) {
            return false;
        }
        let ideal = |m: &TMesh, p: u32| if m.border_point(p) { 4i32 } else { 6 };
        let val = |m: &TMesh, p: u32| m.neighbors(p).len() as i32;
        let dev = |x: i32| x * x;
        let (ia, ib, ic, id) = (ideal(self, a), ideal(self, b), ideal(self, c), ideal(self, d));
        let (va, vb, vc, vd) = (val(self, a), val(self, b), val(self, c), val(self, d));
        let before = dev(va - ia) + dev(vb - ib) + dev(vc - ic) + dev(vd - id);
        let after = dev(va - 1 - ia) + dev(vb - 1 - ib) + dev(vc + 1 - ic) + dev(vd + 1 - id);
        if after >= before {
            return false;
        }
        let (n1, n2) = (self.normal([c, a, d]), self.normal([d, b, c]));
        let old = (self.normal(self.f[f1 as usize]) + self.normal(self.f[f2 as usize])).normalize_or_zero();
        if n1.normalize_or_zero().dot(old) < 0.2 || n2.normalize_or_zero().dot(old) < 0.2 {
            return false;
        }
        self.f[f1 as usize] = [c, a, d];
        self.f[f2 as usize] = [d, b, c];
        self.vf[b as usize].retain(|&g| g != f1);
        self.vf[a as usize].retain(|&g| g != f2);
        self.vf[d as usize].push(f1);
        self.vf[c as usize].push(f2);
        true
    }

    /// Points slid toward their neighbours' middle along the surface, and put back on it.
    pub fn smooth(&mut self, surf: &Surface, reach: f64) {
        let mut newv = self.v.clone();
        for p in 0..self.v.len() as u32 {
            if self.vf[p as usize].is_empty() || self.rank[p as usize] == 2 {
                continue;
            }
            let x = self.v[p as usize];
            let nb: Vec<u32> = if self.rank[p as usize] == 1 {
                self.neighbors(p).into_iter().filter(|&q| self.feat.contains(&key(p, q))).collect()
            } else {
                self.neighbors(p)
            };
            if nb.is_empty() {
                continue;
            }
            let c = nb.iter().map(|&q| self.v[q as usize]).sum::<DVec3>() / nb.len() as f64;
            if self.rank[p as usize] == 1 {
                let dir = if nb.len() == 2 { self.v[nb[1] as usize] - self.v[nb[0] as usize] } else { DVec3::ZERO };
                if let Some(y) = surf.on_feature_along(x.lerp(c, 0.5), dir) {
                    newv[p as usize] = y;
                }
                continue;
            }
            let n = self.vnormal(p);
            let d = c - x;
            let y = x + (d - n * n.dot(d)) * 0.8;
            if let Some(q) = surf.project(y, n, reach) {
                newv[p as usize] = surf.off_curves(q, c, n, reach);
            }
        }
        self.v = newv;
    }

    /// Edges toward `len` (scaled down where `size` says the surface bends tightly), over `iters` rounds.
    pub fn remesh(&mut self, surf: &Surface, len: impl Fn(DVec3, bool) -> f64, reach: f64, iters: usize) {
        for _ in 0..iters {
            let target = |m: &TMesh, a: u32, b: u32| len((m.v[a as usize] + m.v[b as usize]) * 0.5, m.feat.contains(&key(a, b)));
            // Split long edges.
            for _ in 0..4 {
                let mut any = false;
                for (a, b) in self.edges() {
                    if (self.v[a as usize] - self.v[b as usize]).length() > 4.0 / 3.0 * target(self, a, b) {
                        self.split(a, b);
                        any = true;
                    }
                }
                if !any {
                    break;
                }
            }
            // Collapse short ones, shortest first.
            let mut es = self.edges();
            es.sort_by(|x, y| {
                let lx = (self.v[x.0 as usize] - self.v[x.1 as usize]).length();
                let ly = (self.v[y.0 as usize] - self.v[y.1 as usize]).length();
                lx.partial_cmp(&ly).unwrap()
            });
            for (a, b) in es {
                if self.vf[a as usize].is_empty() || self.vf[b as usize].is_empty() {
                    continue;
                }
                let t = target(self, a, b);
                if (self.v[a as usize] - self.v[b as usize]).length() < 0.8 * t && self.edge_faces(a, b).len() > 0 {
                    self.try_collapse(a, b, 4.0 / 3.0 * t);
                }
            }
            for (a, b) in self.edges() {
                self.flip(a, b);
            }
            for _ in 0..2 {
                self.smooth(surf, reach);
            }
        }
    }

    /// Edges shorter than min_len collapsed, and triangles with an angle under min_angle lose their shortest edge.
    pub fn cleanup(&mut self, min_len: f64, min_angle: f64) {
        if min_len > 0.0 {
            for (a, b) in self.edges() {
                if !self.vf[a as usize].is_empty() && !self.vf[b as usize].is_empty() && (self.v[a as usize] - self.v[b as usize]).length() < min_len {
                    self.try_collapse(a, b, f64::MAX);
                }
            }
        }
        if min_angle > 0.0 {
            for fi in 0..self.f.len() {
                if self.dead[fi] {
                    continue;
                }
                let t = self.f[fi];
                let p = t.map(|x| self.v[x as usize]);
                let mut smallest = f64::MAX;
                for k in 0..3 {
                    let ang = (p[(k + 1) % 3] - p[k]).angle_between(p[(k + 2) % 3] - p[k]);
                    smallest = smallest.min(if ang.is_finite() { ang } else { 0.0 });
                }
                if smallest < min_angle {
                    let k = (0..3)
                        .min_by(|&i, &j| {
                            let li = (p[(i + 1) % 3] - p[i]).length();
                            let lj = (p[(j + 1) % 3] - p[j]).length();
                            li.partial_cmp(&lj).unwrap()
                        })
                        .unwrap();
                    self.try_collapse(t[k], t[(k + 1) % 3], f64::MAX);
                }
            }
        }
    }

    /// Live points renumbered from 0, and live faces.
    pub fn compact(&self) -> (Vec<DVec3>, Vec<[u32; 3]>, Vec<u8>, HashSet<(u32, u32)>) {
        let mut map = vec![u32::MAX; self.v.len()];
        let mut v = Vec::new();
        let mut rank = Vec::new();
        let mut f = Vec::new();
        for (i, t) in self.f.iter().enumerate() {
            if self.dead[i] {
                continue;
            }
            let t2 = t.map(|p| {
                if map[p as usize] == u32::MAX {
                    map[p as usize] = v.len() as u32;
                    v.push(self.v[p as usize]);
                    rank.push(self.rank[p as usize]);
                }
                map[p as usize]
            });
            f.push(t2);
        }
        let feat = self
            .feat
            .iter()
            .filter(|k| map[k.0 as usize] != u32::MAX && map[k.1 as usize] != u32::MAX)
            .map(|k| key(map[k.0 as usize], map[k.1 as usize]))
            .collect();
        (v, f, rank, feat)
    }
}
