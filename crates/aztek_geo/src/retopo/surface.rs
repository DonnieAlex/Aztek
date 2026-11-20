//! The source surface the new mesh is held to: nearest points on it, its normal and how tightly it bends near a point,
//! and the feature curves points on them slide along.

use super::tri::{Curvature, Tri};
use crate::attribute::Index;
use crate::bvh::MeshBvh;
use crate::mesh::Mesh;
use glam::DVec3;
use std::collections::HashMap;

pub struct Surface {
    bvh: MeshBvh,
    tris: Vec<[u32; 3]>,
    fnormal: Vec<DVec3>,
    /// Per source point: the radius of its tighter bend, and its least-bending direction with how much more it bends
    /// across that than along it (0..1).
    radius: Vec<f64>,
    dir_min: Vec<DVec3>,
    k_min: Vec<f64>,
    k_max: Vec<f64>,
    segs: Vec<(DVec3, DVec3)>,
    cell: f64,
    grid: HashMap<(i64, i64, i64), Vec<u32>>,
}

fn segs_keys(t: &Tri) -> Vec<(u32, u32)> {
    t.features.keys().copied().collect()
}

impl Surface {
    /// `segs` are the feature edges; `cell` about the spacing points will have along them.
    pub fn new(t: &Tri, segs: Vec<(DVec3, DVec3)>, cell: f64) -> Surface {
        let mut sm = Mesh::new();
        let pts: Vec<Index> = t.v.iter().map(|p| sm.add_point(*p)).collect();
        for f in &t.f {
            sm.add_face(&[pts[f[0] as usize], pts[f[1] as usize], pts[f[2] as usize]]);
        }
        let bvh = MeshBvh::new(&sm);
        let k = Curvature::new(t, 3);
        let mut radius: Vec<f64> = (0..t.v.len()).map(|i| 1.0 / k.k_min[i].abs().max(k.k_max[i].abs()).max(1e-12)).collect();
        // A sharp edge bends its points' fit without the surface bending there: they take their neighbours' radius.
        let mut on = vec![false; t.v.len()];
        for (a, b) in &segs_keys(t) {
            on[*a as usize] = true;
            on[*b as usize] = true;
        }
        for _ in 0..3 {
            let prev = radius.clone();
            for i in 0..t.v.len() {
                if on[i] {
                    let r = t.adj[i].iter().filter(|&&j| !on[j as usize]).map(|&j| prev[j as usize]).fold(0.0, f64::max);
                    radius[i] = if r > 0.0 { r } else { f64::MAX.sqrt() };
                }
            }
        }
        let cell = cell.max(1e-9);
        let mut grid: HashMap<(i64, i64, i64), Vec<u32>> = HashMap::new();
        for (i, (a, b)) in segs.iter().enumerate() {
            let c0 = (a.min(*b) / cell).floor();
            let c1 = (a.max(*b) / cell).floor();
            for x in c0.x as i64..=c1.x as i64 {
                for y in c0.y as i64..=c1.y as i64 {
                    for z in c0.z as i64..=c1.z as i64 {
                        grid.entry((x, y, z)).or_default().push(i as u32);
                    }
                }
            }
        }
        Surface {
            bvh,
            tris: t.f.clone(),
            fnormal: t.fnormal.clone(),
            radius,
            dir_min: k.dir_min,
            k_min: k.k_min,
            k_max: k.k_max,
            segs,
            cell,
            grid,
        }
    }

    /// The nearest surface point within r whose side faces the same way as n; None when there is none.
    pub fn project(&self, p: DVec3, n: DVec3, r: f64) -> Option<DVec3> {
        let h = self.bvh.closest_point_within(p, r)?;
        if n == DVec3::ZERO || self.fnormal[h.face as usize].dot(n) > 0.0 {
            Some(h.point)
        } else {
            None
        }
    }

    /// Where the line through p along n meets the surface nearest p, facing n's way; else the nearest point
    /// facing that way within reach; else the nearest point.
    pub fn onto(&self, p: DVec3, n: DVec3, reach: f64) -> DVec3 {
        let mut best: Option<(f64, DVec3)> = None;
        if n != DVec3::ZERO {
            for d in [n, -n] {
                if let Some(h) = self.bvh.raycast_with(p, d, reach, true) {
                    if self.fnormal[h.face as usize].dot(n) > 0.0 && best.map(|b| h.t < b.0).unwrap_or(true) {
                        best = Some((h.t, h.point));
                    }
                }
            }
        }
        if let Some(b) = best {
            return b.1;
        }
        self.project(p, n, reach).unwrap_or_else(|| self.closest(p))
    }

    pub fn closest(&self, p: DVec3) -> DVec3 {
        self.bvh.closest_point(p).map(|h| h.point).unwrap_or(p)
    }

    pub fn normal_at(&self, p: DVec3) -> DVec3 {
        self.bvh.closest_point(p).map(|h| self.fnormal[h.face as usize]).unwrap_or(DVec3::Z)
    }

    /// Per point values at the nearest triangle, by its barycentric weights.
    fn at<T: Copy + Default + std::ops::Mul<f64, Output = T> + std::ops::Add<Output = T>>(&self, p: DVec3, v: &[T]) -> T {
        match self.bvh.closest_point(p) {
            Some(h) => {
                let t = self.tris[h.face as usize];
                v[t[0] as usize] * h.bary.x + v[t[1] as usize] * h.bary.y + v[t[2] as usize] * h.bary.z
            }
            None => T::default(),
        }
    }

    /// The radius of the surface's tighter bend near p.
    pub fn radius_at(&self, p: DVec3) -> f64 {
        self.at(p, &self.radius)
    }

    /// The normal curvature near p along direction d, and the larger of the two principal curvatures there.
    pub fn bend_along(&self, p: DVec3, d: DVec3) -> (f64, f64) {
        let Some(h) = self.bvh.closest_point(p) else { return (0.0, 0.0) };
        let t = self.tris[h.face as usize];
        let mut along = 0.0;
        let mut most = 0.0;
        for k in 0..3 {
            let i = t[k] as usize;
            let w = h.bary[k];
            let c = self.dir_min[i].dot(d).powi(2) / d.length_squared().max(1e-30);
            let (a, b) = (self.k_min[i].abs(), self.k_max[i].abs());
            along += w * (a * c + b * (1.0 - c));
            most += w * b;
        }
        (along, most)
    }

    pub fn has_features(&self) -> bool {
        !self.segs.is_empty()
    }

    /// The nearest point on a feature curve near p; with a direction, on a curve running that way if there's one
    /// near, so a point doesn't jump to another curve where two meet.
    pub fn on_feature(&self, p: DVec3) -> Option<DVec3> {
        self.on_feature_along(p, DVec3::ZERO)
    }

    /// Whether p lies on a feature curve, within tol.
    pub fn on_curve(&self, p: DVec3, tol: f64) -> bool {
        self.on_feature(p).map(|q| (q - p).length() <= tol).unwrap_or(false)
    }

    /// A free point whose nearest surface point lies on a feature curve was pushed past a border onto it: it goes
    /// a little back toward `inner` instead.
    pub fn off_curves(&self, q: DVec3, inner: DVec3, n: DVec3, reach: f64) -> DVec3 {
        if !self.on_curve(q, reach * 1e-4) {
            return q;
        }
        for t in [0.25, 0.5] {
            if let Some(y) = self.project(q.lerp(inner, t), n, reach) {
                if !self.on_curve(y, reach * 1e-4) {
                    return y;
                }
            }
        }
        q
    }

    pub fn on_feature_along(&self, p: DVec3, dir: DVec3) -> Option<DVec3> {
        let dir = dir.normalize_or_zero();
        let mut aligned: Option<(f64, DVec3)> = None;
        let c = (p / self.cell).floor();
        let mut best: Option<(f64, DVec3)> = None;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(list) = self.grid.get(&(c.x as i64 + dx, c.y as i64 + dy, c.z as i64 + dz)) {
                        for &i in list {
                            let (a, b) = self.segs[i as usize];
                            let ab = b - a;
                            let l2 = ab.length_squared();
                            let t = if l2 > 0.0 { ((p - a).dot(ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
                            let q = a + ab * t;
                            let d = (q - p).length_squared();
                            if best.map(|x| d < x.0).unwrap_or(true) {
                                best = Some((d, q));
                            }
                            if dir != DVec3::ZERO && l2 > 0.0 && (ab.dot(dir).abs() / l2.sqrt()) > 0.7 && aligned.map(|x| d < x.0).unwrap_or(true) {
                                aligned = Some((d, q));
                            }
                        }
                    }
                }
            }
        }
        aligned.or(best).map(|b| b.1)
    }
}
