//! The triangle mesh the fields are solved on: points, triangles, what lies along each edge, and per point the normal,
//! area, neighbours and curvature.

use crate::attribute::{names, Index, INVALID};
use crate::mesh::Mesh;
use crate::triangulate::{triangulate_face, TriangulateOptions};
use glam::DVec3;
use std::collections::HashMap;

/// An undirected edge, its smaller point first.
pub fn key(a: u32, b: u32) -> (u32, u32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Why an edge is one the result should follow.
pub mod feature {
    pub const BORDER: u8 = 1;
    pub const ANGLE: u8 = 2;
    pub const HARD: u8 = 4;
    pub const SEAM: u8 = 8;
    pub const MATERIAL: u8 = 16;
    pub const SEGMENT: u8 = 32;
    pub const SYMMETRY: u8 = 64;
    /// A border between colours: followed exactly, unlike a shape segment's border.
    pub const COLOR: u8 = 128;
}

#[derive(Clone, Default)]
pub struct Tri {
    pub v: Vec<DVec3>,
    pub f: Vec<[u32; 3]>,
    /// Per point colour, linear RGB, for segmenting by colour; empty when the mesh has none.
    pub color: Vec<[f32; 3]>,
    /// Per triangle colour, from a texture the caller sampled; empty when none.
    pub face_color: Vec<[f32; 3]>,
    /// Per triangle: the source face's material.
    pub material: Vec<i32>,
    /// Undirected edges that are features, and why.
    pub features: HashMap<(u32, u32), u8>,
    /// Edges whose two triangles' UVs don't meet.
    pub seams: HashMap<(u32, u32), ()>,

    // Derived by `prepare`.
    pub fnormal: Vec<DVec3>,
    pub farea: Vec<f64>,
    pub n: Vec<DVec3>,
    pub area: Vec<f64>,
    pub adj: Vec<Vec<u32>>,
    /// Triangles on each undirected edge.
    pub edge_faces: HashMap<(u32, u32), Vec<u32>>,
}

impl Tri {
    /// The faces of a mesh as triangles. Points no face uses are left out.
    pub fn from_mesh(m: &Mesh, faces: &[Index]) -> (Tri, Vec<Index>) {
        let mut map = vec![INVALID; m.point_capacity() as usize];
        let mut t = Tri::default();
        let colors = m.corner_attrs().get::<[f32; 4]>(names::COLOR);
        let point_colors = m.point_attrs().get::<[f32; 4]>(names::COLOR);
        let uv = m.corner_attrs().get::<[f32; 2]>(&names::uv(0));
        let mat = m.face_attrs().get::<i32>(names::MATERIAL);
        let mut col_sum: Vec<([f32; 3], f32)> = Vec::new();
        let opts = TriangulateOptions::default();
        let mut tris = Vec::new();
        let mut orig = Vec::new();
        for &f in faces {
            if !m.face_valid(f) || m.face_size(f) < 3 {
                continue;
            }
            for c in m.face_corners(f) {
                let p = m.corner_point(c);
                if map[p as usize] == INVALID {
                    map[p as usize] = t.v.len() as Index;
                    t.v.push(m.position(p));
                    orig.push(p);
                    col_sum.push(([0.0; 3], 0.0));
                }
                let i = map[p as usize] as usize;
                let col = if !colors.is_empty() {
                    Some(colors[c as usize])
                } else if !point_colors.is_empty() {
                    Some(point_colors[p as usize])
                } else {
                    None
                };
                if let Some(c4) = col {
                    for k in 0..3 {
                        col_sum[i].0[k] += c4[k];
                    }
                    col_sum[i].1 += 1.0;
                }
            }
            tris.clear();
            triangulate_face(m, f, &mut tris, &opts);
            for k in tris.chunks(3) {
                if k.len() < 3 {
                    continue;
                }
                let p = [0, 1, 2].map(|i| map[m.corner_point(k[i]) as usize]);
                if p[0] == p[1] || p[1] == p[2] || p[0] == p[2] {
                    continue;
                }
                t.f.push(p);
                t.material.push(if mat.is_empty() { 0 } else { mat[f as usize] });
            }
            // The source face's own edges: borders, hard edges, seams and material changes.
            let n = m.face_size(f) as usize;
            let corners = m.face_corners(f);
            for i in 0..n {
                let c = corners[i];
                let e = m.corner_edge(c);
                let a = map[m.corner_point(c) as usize];
                let b = map[m.corner_point(corners[(i + 1) % n]) as usize];
                let k = key(a, b);
                let mut flags = 0u8;
                let face_count = m.edge_face_count(e);
                if face_count != 2 {
                    flags |= feature::BORDER;
                }
                let hard = m.edge_attrs().get::<bool>(names::HARD);
                if !hard.is_empty() && hard[e as usize] {
                    flags |= feature::HARD;
                }
                let seam = m.edge_attrs().get::<bool>(names::SEAM);
                if !seam.is_empty() && seam[e as usize] {
                    t.seams.insert(k, ());
                }
                if face_count == 2 {
                    let other = m.opposite_face(e, f);
                    if other != INVALID {
                        if !mat.is_empty() && mat[other as usize] != mat[f as usize] {
                            flags |= feature::MATERIAL;
                        }
                        if !uv.is_empty() {
                            // The UVs at this edge's ends, seen from both faces.
                            let pa = m.corner_point(c);
                            let pb = m.corner_point(corners[(i + 1) % n]);
                            let ca = m.find_corner(other, pa);
                            let cb = m.find_corner(other, pb);
                            if ca != INVALID && cb != INVALID {
                                let near = |x: [f32; 2], y: [f32; 2]| (x[0] - y[0]).abs() + (x[1] - y[1]).abs() < 1e-5;
                                if !near(uv[c as usize], uv[ca as usize])
                                    || !near(uv[corners[(i + 1) % n] as usize], uv[cb as usize])
                                {
                                    t.seams.insert(k, ());
                                }
                            }
                        }
                    }
                }
                if flags != 0 {
                    *t.features.entry(k).or_insert(0) |= flags;
                }
            }
        }
        if col_sum.iter().any(|c| c.1 > 0.0) {
            t.color = col_sum.iter().map(|(s, n)| if *n > 0.0 { s.map(|x| x / n) } else { [0.0; 3] }).collect();
        }
        (t, orig)
    }

    /// Normals, areas, neighbours and the triangles on each edge.
    pub fn prepare(&mut self) {
        let nv = self.v.len();
        self.fnormal = Vec::with_capacity(self.f.len());
        self.farea = Vec::with_capacity(self.f.len());
        self.n = vec![DVec3::ZERO; nv];
        self.area = vec![0.0; nv];
        let mut adj: Vec<Vec<u32>> = vec![Vec::new(); nv];
        self.edge_faces.clear();
        for (fi, t) in self.f.iter().enumerate() {
            let (a, b, c) = (self.v[t[0] as usize], self.v[t[1] as usize], self.v[t[2] as usize]);
            let cr = (b - a).cross(c - a);
            let ar = cr.length() * 0.5;
            let fnrm = if ar > 0.0 { cr / (2.0 * ar) } else { DVec3::Z };
            self.fnormal.push(fnrm);
            self.farea.push(ar);
            for k in 0..3 {
                let p = t[k] as usize;
                // Angle-weighted normals keep a coarse triangulation from tilting them.
                let (e1, e2) = (self.v[t[(k + 1) % 3] as usize] - self.v[p], self.v[t[(k + 2) % 3] as usize] - self.v[p]);
                let ang = e1.angle_between(e2);
                if ang.is_finite() {
                    self.n[p] += fnrm * ang;
                }
                self.area[p] += ar / 3.0;
                let q = t[(k + 1) % 3];
                adj[p].push(q);
                adj[q as usize].push(t[k]);
                self.edge_faces.entry(key(t[k], q)).or_default().push(fi as u32);
            }
        }
        for n in &mut self.n {
            *n = n.try_normalize().unwrap_or(DVec3::Z);
        }
        for a in &mut adj {
            a.sort_unstable();
            a.dedup();
        }
        self.adj = adj;
    }

    pub fn total_area(&self) -> f64 {
        self.farea.iter().sum()
    }

    pub fn mean_edge(&self) -> f64 {
        let mut s = 0.0;
        let mut n = 0usize;
        for (i, a) in self.adj.iter().enumerate() {
            for &j in a {
                if (j as usize) > i {
                    s += (self.v[i] - self.v[j as usize]).length();
                    n += 1;
                }
            }
        }
        if n == 0 {
            0.0
        } else {
            s / n as f64
        }
    }

    /// Splits every edge longer than max_len until none is, keeping features and colours along. Stops at max_tris.
    pub fn refine(&mut self, max_len: f64, max_tris: usize) {
        for _ in 0..12 {
            if self.f.len() >= max_tris {
                break;
            }
            let mut mid: HashMap<(u32, u32), u32> = HashMap::new();
            let max2 = max_len * max_len;
            for t in &self.f {
                for k in 0..3 {
                    let (a, b) = (t[k], t[(k + 1) % 3]);
                    let kk = key(a, b);
                    if mid.contains_key(&kk) {
                        continue;
                    }
                    if (self.v[a as usize] - self.v[b as usize]).length_squared() > max2 {
                        let m = self.v.len() as u32;
                        self.v.push((self.v[a as usize] + self.v[b as usize]) * 0.5);
                        if !self.color.is_empty() {
                            let (ca, cb) = (self.color[a as usize], self.color[b as usize]);
                            self.color.push([0, 1, 2].map(|i| (ca[i] + cb[i]) * 0.5));
                        }
                        mid.insert(kk, m);
                    }
                }
            }
            if mid.is_empty() {
                break;
            }
            for (kk, &m) in &mid {
                if let Some(fl) = self.features.remove(kk) {
                    self.features.insert(key(kk.0, m), fl);
                    self.features.insert(key(m, kk.1), fl);
                }
                if self.seams.remove(kk).is_some() {
                    self.seams.insert(key(kk.0, m), ());
                    self.seams.insert(key(m, kk.1), ());
                }
            }
            let old = std::mem::take(&mut self.f);
            let old_mat = std::mem::take(&mut self.material);
            let old_col = std::mem::take(&mut self.face_color);
            for (fi, t) in old.into_iter().enumerate() {
                let ms = [0, 1, 2].map(|k| mid.get(&key(t[k], t[(k + 1) % 3])).copied());
                let mut out: Vec<[u32; 3]> = Vec::with_capacity(4);
                match ms.iter().filter(|x| x.is_some()).count() {
                    0 => out.push(t),
                    3 => {
                        let (m0, m1, m2) = (ms[0].unwrap(), ms[1].unwrap(), ms[2].unwrap());
                        out.extend([[t[0], m0, m2], [m0, t[1], m1], [m2, m1, t[2]], [m0, m1, m2]]);
                    }
                    1 => {
                        let k = ms.iter().position(|x| x.is_some()).unwrap();
                        let m = ms[k].unwrap();
                        let (a, b, c) = (t[k], t[(k + 1) % 3], t[(k + 2) % 3]);
                        out.extend([[a, m, c], [m, b, c]]);
                    }
                    _ => {
                        // Two split edges: the unsplit one is k.
                        let k = ms.iter().position(|x| x.is_none()).unwrap();
                        let (a, b, c) = (t[k], t[(k + 1) % 3], t[(k + 2) % 3]);
                        let mbc = ms[(k + 1) % 3].unwrap();
                        let mca = ms[(k + 2) % 3].unwrap();
                        out.push([mca, mbc, c]);
                        // The quad a b mbc mca: split along its shorter diagonal.
                        let d1 = (self.v[a as usize] - self.v[mbc as usize]).length_squared();
                        let d2 = (self.v[b as usize] - self.v[mca as usize]).length_squared();
                        if d1 < d2 {
                            out.extend([[a, b, mbc], [a, mbc, mca]]);
                        } else {
                            out.extend([[a, b, mca], [b, mbc, mca]]);
                        }
                    }
                }
                for tt in out {
                    self.f.push(tt);
                    self.material.push(old_mat.get(fi).copied().unwrap_or(0));
                    if !old_col.is_empty() {
                        self.face_color.push(old_col[fi]);
                    }
                }
            }
        }
    }

    /// Edges whose faces bend by more than `angle` radians become features.
    pub fn mark_angle_features(&mut self, angle: f64) {
        let cos = angle.cos();
        for (k, faces) in &self.edge_faces {
            if faces.len() == 2 && self.fnormal[faces[0] as usize].dot(self.fnormal[faces[1] as usize]) < cos {
                *self.features.entry(*k).or_insert(0) |= feature::ANGLE;
            }
        }
    }

    /// Edges between triangles of different labels become features.
    pub fn mark_label_boundaries(&mut self, label: &[u32], flag: u8) {
        for (k, faces) in &self.edge_faces {
            if faces.len() == 2 && label[faces[0] as usize] != label[faces[1] as usize] {
                *self.features.entry(*k).or_insert(0) |= flag;
            }
        }
    }

    /// Triangle neighbours across edges, for growing regions.
    pub fn face_adjacency(&self) -> Vec<Vec<u32>> {
        let mut out = vec![Vec::new(); self.f.len()];
        for faces in self.edge_faces.values() {
            if faces.len() == 2 {
                out[faces[0] as usize].push(faces[1]);
                out[faces[1] as usize].push(faces[0]);
            }
        }
        for a in &mut out {
            a.sort_unstable();
        }
        out
    }
}

/// Per point: the curvature tensor as two principal directions (unit, tangent) and curvatures, fitted to the normal
/// curvatures of the point's edges and smoothed over a ring.
pub struct Curvature {
    pub dir_min: Vec<DVec3>,
    pub k_min: Vec<f64>,
    pub k_max: Vec<f64>,
}

impl Curvature {
    pub fn new(t: &Tri, smooth_rings: usize) -> Curvature {
        let nv = t.v.len();
        // The tensor as a symmetric 3x3 matrix in world space, tangent to the point.
        let mut tensor = vec![[0.0f64; 6]; nv];
        for i in 0..nv {
            let n = t.n[i];
            let (u, w) = frame(n);
            // Least squares of k(d) = a du^2 + 2 b du dv + c dv^2.
            let mut ata = [[0.0f64; 3]; 3];
            let mut atb = [0.0f64; 3];
            for &j in &t.adj[i] {
                let d = t.v[j as usize] - t.v[i];
                let l2 = d.length_squared();
                if l2 <= 0.0 {
                    continue;
                }
                let k = 2.0 * n.dot(d) / l2;
                let dt = d - n * n.dot(d);
                let len = dt.length();
                if len <= 0.0 {
                    continue;
                }
                let (x, y) = (dt.dot(u) / len, dt.dot(w) / len);
                let row = [x * x, 2.0 * x * y, y * y];
                for r in 0..3 {
                    for c in 0..3 {
                        ata[r][c] += row[r] * row[c];
                    }
                    atb[r] += row[r] * k;
                }
            }
            for r in 0..3 {
                ata[r][r] += 1e-9;
            }
            let s = solve3(ata, atb).unwrap_or([0.0; 3]);
            // Back to world: K = a u u^T + b (u w^T + w u^T) + c w w^T.
            let m = |x: DVec3, y: DVec3| [x.x * y.x, x.y * y.y, x.z * y.z, x.x * y.y, x.x * y.z, x.y * y.z];
            let uu = m(u, u);
            let ww = m(w, w);
            let uw = m(u, w);
            let wu = m(w, u);
            let mut k6 = [0.0; 6];
            for q in 0..6 {
                // Off-diagonal entries are symmetric: average u w^T and w u^T.
                let sym = if q < 3 { 2.0 * uw[q] } else { uw[q] + wu[q] };
                k6[q] = s[0] * uu[q] + s[1] * sym + s[2] * ww[q];
            }
            tensor[i] = k6;
        }
        for _ in 0..smooth_rings {
            let prev = tensor.clone();
            for i in 0..nv {
                let mut acc = prev[i];
                let mut w = 1.0;
                for &j in &t.adj[i] {
                    for q in 0..6 {
                        acc[q] += prev[j as usize][q];
                    }
                    w += 1.0;
                }
                tensor[i] = acc.map(|x| x / w);
            }
        }
        let mut out = Curvature { dir_min: vec![DVec3::X; nv], k_min: vec![0.0; nv], k_max: vec![0.0; nv] };
        for i in 0..nv {
            let n = t.n[i];
            let (u, w) = frame(n);
            let k = tensor[i];
            let apply = |x: DVec3| {
                DVec3::new(k[0] * x.x + k[3] * x.y + k[4] * x.z, k[3] * x.x + k[1] * x.y + k[5] * x.z, k[4] * x.x + k[5] * x.y + k[2] * x.z)
            };
            let (ku, kw) = (apply(u), apply(w));
            let (a, b, c) = (u.dot(ku), u.dot(kw), w.dot(kw));
            // Eigen of [[a b][b c]].
            let tr = 0.5 * (a + c);
            let disc = (0.25 * (a - c) * (a - c) + b * b).sqrt();
            let (l1, l2) = (tr - disc, tr + disc);
            // Eigenvector of the eigenvalue smaller in magnitude: the direction the surface bends least.
            let (kmin, kmax) = if l1.abs() < l2.abs() { (l1, l2) } else { (l2, l1) };
            let ev = if b.abs() > 1e-12 {
                DVec3::new(b, kmin - a, 0.0)
            } else if (a - kmin).abs() < (c - kmin).abs() {
                DVec3::new(1.0, 0.0, 0.0)
            } else {
                DVec3::new(0.0, 1.0, 0.0)
            };
            let d = (u * ev.x + w * ev.y).try_normalize().unwrap_or(u);
            out.dir_min[i] = d;
            out.k_min[i] = kmin;
            out.k_max[i] = kmax;
        }
        out
    }

    /// 0 where the surface bends the same way everywhere (plane, sphere), 1 where it bends one way only (cylinder).
    pub fn anisotropy(&self, i: usize) -> f64 {
        let (a, b) = (self.k_min[i].abs(), self.k_max[i].abs());
        if a + b < 1e-12 {
            0.0
        } else {
            (b - a) / (a + b)
        }
    }
}

pub fn frame(n: DVec3) -> (DVec3, DVec3) {
    let u = if n.x.abs() < 0.9 { DVec3::X } else { DVec3::Y };
    let u = (u - n * n.dot(u)).normalize();
    (u, n.cross(u))
}

fn solve3(a: [[f64; 3]; 3], b: [f64; 3]) -> Option<[f64; 3]> {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if det.abs() < 1e-18 {
        return None;
    }
    let mut out = [0.0; 3];
    for col in 0..3 {
        let mut m = a;
        for r in 0..3 {
            m[r][col] = b[r];
        }
        let d = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
        out[col] = d / det;
    }
    Some(out)
}
