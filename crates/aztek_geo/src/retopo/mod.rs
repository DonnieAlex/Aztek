//! Automatic retopology: a new mesh of quads over a surface, every face a quad, closed where the surface is closed.
//!
//! 1. The faces are triangulated and their features found: borders always, hard edges and edges sharper than an
//!    angle when asked, edges given as feature tags, and the cut of a symmetry plane.
//! 2. The surface is remeshed into even triangles, finer where it bends tightly, feature curves kept, then each
//!    triangle split into three quads.
//! 3. That all-quad mesh is simplified to a coarse layout of large quads (see `quad`): every step keeps it manifold
//!    and unfolded, and keeps the surface's own points near its faces, so thin parts and holes come through.
//! 4. Each row of the layout's quads gets a whole number of new rows, sized to the wanted edge length (following the
//!    curvature and stretching along it as asked) and scaled together to meet the face count; every coarse quad is
//!    filled with its grid. Inside each coarse quad every point has four neighbours, which is what makes the result
//!    regular, and neighbouring quads share their points, which is what keeps it closed.
//! 5. The points are relaxed along the surface, feature points along their curves, corners kept.
//!
//! With symmetry, half the surface is done this way with its cut as a border, then mirrored and joined along it.

pub mod quad;
pub mod surface;
pub mod tri;
pub mod trimesh;

use crate::attribute::Index;
use crate::bvh::MeshBvh;
use crate::mesh::Mesh;
use crate::symmetry::SymmetryPlane;
use glam::DVec3;
use quad::QMesh;
use std::collections::{HashMap, HashSet};
use surface::Surface;
use tri::{feature, key, Tri};
use trimesh::TMesh;

/// The reference's polyRetopo attributes.
#[derive(Clone, Debug)]
pub struct RetopoOptions {
    /// Faces wanted; 0 lets target_edge_deviation set the density.
    pub target_faces: usize,
    /// Percent the count may miss by.
    pub tolerance: f64,
    /// Follow the mesh's hard edges.
    pub preserve_hard_edges: bool,
    /// Follow edges whose faces bend more than `angle`.
    pub edges_by_angle: bool,
    /// Radians.
    pub angle: f64,
    /// 0 face size following the curvature, 1 one size everywhere.
    pub face_uniformity: f64,
    /// 0..1: how far quads stretch along the direction the surface bends least.
    pub anisotropy: f64,
    /// 0..1: higher for fewer irregular points and longer unbroken loops.
    pub topology_regularity: f64,
    /// When target_faces is 0: how far edges may stray from the surface, against its local radius.
    pub target_edge_deviation: f64,
    /// Mirror across this plane: the result is exactly symmetric.
    pub symmetry: Option<SymmetryPlane>,
    /// Clean the input first: edges shorter than short_edge_tolerance (of the size) collapsed, triangles thinner
    /// than thin_triangle_angle removed.
    pub preprocess: bool,
    pub short_edge_tolerance: f64,
    /// Radians.
    pub thin_triangle_angle: f64,
    /// Extra feature edges, as source mesh edges (the feature tags).
    pub feature_edges: Vec<Index>,
}

impl Default for RetopoOptions {
    fn default() -> Self {
        Self {
            target_faces: 1000,
            tolerance: 10.0,
            preserve_hard_edges: false,
            edges_by_angle: false,
            angle: 30f64.to_radians(),
            face_uniformity: 0.0,
            anisotropy: 0.5,
            topology_regularity: 0.5,
            target_edge_deviation: 0.2929,
            symmetry: None,
            preprocess: true,
            short_edge_tolerance: 0.0001,
            thin_triangle_angle: 1f64.to_radians(),
            feature_edges: Vec::new(),
        }
    }
}

/// Retopologizes the given faces of m as a new mesh.
pub fn retopologize(m: &Mesh, faces: &[Index], opts: &RetopoOptions) -> Mesh {
    if let Some(plane) = opts.symmetry {
        return symmetric(m, faces, opts, &plane);
    }
    run(m, faces, opts, None)
}

/// Half the mesh, retopologized with the cut held as a feature, then mirrored and joined along it.
fn symmetric(m: &Mesh, faces: &[Index], opts: &RetopoOptions, plane: &SymmetryPlane) -> Mesh {
    let mut half = crate::ops::extract_faces(m, faces);
    half.compact(None);
    let diag = half.bounds().size().length().max(1e-9);
    crate::ops::cut_at_plane(&mut half, plane, 1, diag * 1e-6, &glam::DMat4::IDENTITY);
    half.compact(None);
    if half.num_faces() == 0 {
        return run(m, faces, &RetopoOptions { symmetry: None, ..opts.clone() }, None);
    }
    let all: Vec<Index> = (0..half.face_capacity()).filter(|&f| half.face_valid(f)).collect();
    let mut o = opts.clone();
    o.symmetry = None;
    o.target_faces = (opts.target_faces + 1) / 2;
    // The half comes back already mirrored and joined along the cut.
    let mut out = run(&half, &all, &o, Some(plane));
    out.compact(None);
    out
}

fn m_point_key(m: &Mesh, p: Index) -> (u64, u64, u64) {
    let v = m.position(p);
    (v.x.to_bits(), v.y.to_bits(), v.z.to_bits())
}

fn run(m: &Mesh, faces: &[Index], opts: &RetopoOptions, plane: Option<&SymmetryPlane>) -> Mesh {
    let mut src = crate::ops::extract_faces(m, faces);
    src.compact(None);
    if src.num_faces() == 0 {
        return Mesh::new();
    }
    let target = opts.target_faces;
    // A far denser input is reduced first: the surface it leaves is what the result is held to.
    if opts.preprocess && opts.feature_edges.is_empty() {
        let tris: usize = (0..src.face_capacity()).filter(|&f| src.face_valid(f)).map(|f| src.face_size(f) as usize - 2).sum();
        let want = (target * 30).max(80_000);
        if tris > want * 2 {
            let ro = crate::reduce::ReduceOptions { target_triangles: want as Index, keep_hard_edges: true, ..Default::default() };
            src = crate::reduce::reduce_mesh(&src, &ro).mesh;
            src.compact(None);
        }
    }
    let all: Vec<Index> = (0..src.face_capacity()).filter(|&f| src.face_valid(f)).collect();
    let (mut t, orig) = Tri::from_mesh(&src, &all);
    if !opts.feature_edges.is_empty() {
        let mut point_map = HashMap::new();
        for (i, &p) in orig.iter().enumerate() {
            point_map.insert(m_point_key(&src, p), i as u32);
        }
        for &e in &opts.feature_edges {
            if e < m.edge_capacity() && m.edge_valid(e) {
                let (a, b) = (m_point_key(m, m.edge_point(e, 0)), m_point_key(m, m.edge_point(e, 1)));
                if let (Some(&x), Some(&y)) = (point_map.get(&a), point_map.get(&b)) {
                    *t.features.entry(key(x, y)).or_insert(0) |= feature::HARD;
                }
            }
        }
    }
    t.prepare();
    let area = t.total_area();
    if area <= 0.0 {
        return Mesh::new();
    }
    let mut keep = feature::BORDER | feature::SYMMETRY;
    if opts.preserve_hard_edges || !opts.feature_edges.is_empty() {
        keep |= feature::HARD;
    }
    t.features.retain(|_, f| *f & keep != 0);
    if opts.edges_by_angle {
        t.mark_angle_features(opts.angle);
    }
    if let Some(pl) = plane {
        let tol = area.sqrt() * 1e-4;
        for (k, f) in t.features.iter_mut() {
            if pl.distance(t.v[k.0 as usize]).abs() < tol && pl.distance(t.v[k.1 as usize]).abs() < tol {
                *f |= feature::SYMMETRY;
            }
        }
    }
    // The edge length: from the face count, or from the surface's radius when there's none.
    let edge = if target > 0 {
        (area / target as f64).sqrt()
    } else {
        // Edges about half the deviation times the surface's tighter radii, so a deviation of 0.29 puts some 40
        // edges round a sphere.
        let d = opts.target_edge_deviation.clamp(1e-3, 1.0);
        let k = tri::Curvature::new(&t, 2);
        let mut radii: Vec<f64> = (0..t.v.len()).map(|i| 1.0 / k.k_min[i].abs().max(k.k_max[i].abs()).max(1e-9)).collect();
        radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let r = radii[radii.len() / 4].min(area.sqrt());
        (0.5 * d * r).max(area.sqrt() * 1e-3)
    };
    // Sharp-edge features shorter than a few edges are noise the result can't follow anyway.
    prune_features(&mut t, edge * 4.0);
    let feat: HashSet<(u32, u32)> = t.features.keys().copied().collect();
    let segs: Vec<(DVec3, DVec3)> = {
        let mut k: Vec<_> = feat.iter().copied().collect();
        k.sort_unstable();
        k.iter().map(|&(a, b)| (t.v[a as usize], t.v[b as usize])).collect()
    };

    let est = (area / (edge * edge)).max(1.0);
    // The coarse layout: each of its quads becomes about m by m. Higher regularity, larger layout quads and so
    // fewer irregular points.
    let reg = opts.topology_regularity.clamp(0.0, 1.0);
    let m_fac = (1.5 + 3.0 * reg).min((est / 12.0).sqrt()).max(1.0);
    let coarse_n = (est / (m_fac * m_fac)).round().max(6.0) as usize;
    let coarse_edge = (area / coarse_n as f64).sqrt();
    let surf = Surface::new(&t, segs, coarse_edge);

    let mut radii: Vec<f64> = (0..t.v.len()).map(|i| surf.radius_at(t.v[i])).collect();
    radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let r_med = radii[radii.len() / 2].max(1e-12);
    let fun = opts.face_uniformity.clamp(0.0, 1.0);
    // Size against the even size where the surface bends: smaller where it bends tighter than usual, larger where
    // flatter, by up to two either way; none of that when uniform.
    let adapt = |p: DVec3| -> f64 { (surf.radius_at(p) / r_med).powf(0.35 * (1.0 - fun)).clamp(0.5, 2.0) };

    // Even triangles at a size that splits into about twelve times the layout's quads, finer where the surface
    // bends tightly so thin parts are there to keep.
    let l_start = (area / (0.433 * (12.0 * coarse_n as f64 / 3.0))).sqrt();
    let soft: HashSet<(u32, u32)> = t.features.iter().filter(|(_, f)| **f & !(feature::BORDER | feature::SYMMETRY) == 0).map(|(k, _)| *k).collect();
    let mut tm = TMesh::new(t.v.clone(), t.f.clone(), feat, &soft, 0.5 * coarse_edge);
    let plane_tol = area.sqrt() * 1e-5;
    tm.plane = plane.map(|p| (*p, plane_tol));
    if opts.preprocess {
        let diag = {
            let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
            for p in &t.v {
                lo = lo.min(*p);
                hi = hi.max(*p);
            }
            (hi - lo).length()
        };
        tm.cleanup(opts.short_edge_tolerance.max(0.0) * diag, opts.thin_triangle_angle.max(0.0));
    }
    // Feature curves at about the layout's spacing once each triangle edge is split in two, and the triangles near
    // them growing to match, so the layout's quads along a curve need not shrink toward it.
    let l_feat = 2.0 * coarse_edge;
    let tri_len = |p: DVec3, on: bool| {
        let curved = (surf.radius_at(p) * 0.7).clamp(0.25 * l_start, l_start);
        if on {
            return l_feat.min((surf.radius_at(p) * 2.0).max(0.25 * l_start)).max(curved);
        }
        match surf.on_feature(p) {
            Some(q) => {
                let t = ((q - p).length() / (1.5 * coarse_edge)).clamp(0.0, 1.0);
                (l_feat * (1.0 - t) + curved * t).max(curved)
            }
            None => curved,
        }
    };
    tm.remesh(&surf, tri_len, l_start, 10);
    tm.split_bridges(&surf, l_start);
    for _ in 0..2 {
        tm.smooth(&surf, l_start);
    }
    let (v, f, rank, feat) = tm.compact();
    let mut qm = QMesh::from_tris(&v, &f, &rank, &feat);
    qm.plane = tm.plane;
    for _ in 0..3 {
        qm.smooth(&surf, l_start, None, 0.5);
    }
    // The layout's quads may grow to about the layout edge, smaller where the surface bends tightly so thin parts
    // keep enough of them round them.
    let coarse_size = |p: DVec3| -> f64 {
        let r = surf.radius_at(p);
        (coarse_edge / 1.3).min((r * 1.5).max(edge))
    };
    qm.samples = qm.v.iter().map(|p| vec![*p]).collect();
    // Higher regularity lets the layout stray further from the surface's detail before it stops simplifying.
    qm.fit = 0.35 + 0.5 * reg;
    // Copies on the way down at the counts that a whole (or half) number of rows per layout quad turns into the face count:
    // where the layout's rows wind through all of it, every quad gets the same grid, and only these meet the count.
    let keep_at: Vec<usize> = if target > 0 { (2..=16).map(|k| (target as f64 / (k as f64 * 0.5).powi(2)) as usize).filter(|&c| c >= 6).collect() } else { Vec::new() };
    let mut layouts = qm.coarsen(&surf, 1, &coarse_size, coarse_edge, &keep_at);

    // The edge length wanted at a point along a direction.
    let aniso = opts.anisotropy.clamp(0.0, 1.0);
    let want = |p: DVec3, d: DVec3| -> f64 {
        let mut w = edge * adapt(p);
        w = w.min((surf.radius_at(p) * 0.8).max(edge * 0.4));
        if aniso > 0.0 {
            let (along, most) = surf.bend_along(p, d);
            if most * edge > 0.05 {
                w *= ((most + 1e-9) / (along + 1e-9)).powf(0.25 * aniso).clamp(1.0, 1.0 + aniso);
            }
        }
        w
    };
    // The coarsest layout that meets the count within the tolerance; else the one nearest it.
    layouts.push(qm);
    let tol = (opts.tolerance.max(1.0) / 100.0) * target as f64;
    let mut best: Option<(f64, QMesh)> = None;
    for l in layouts.iter().rev() {
        let f = l.refine(&want, target);
        let miss = (f.faces() as f64 - target as f64).abs();
        let better = best.as_ref().map(|b| miss < b.0).unwrap_or(true);
        if better {
            best = Some((miss, f));
        }
        if target == 0 || miss <= tol {
            break;
        }
    }
    let mut fine = best.map(|b| b.1).unwrap();
    fine.onto = true;
    fine.snap(&surf, coarse_edge);
    for _ in 0..12 {
        fine.smooth(&surf, edge * 2.0, None, 0.5);
    }
    fine.untangle(&surf, edge * 2.0);
    if let Some(pl) = plane {
        fine.clear_plane(&surf, pl, area.sqrt() * 1e-5, edge * 2.0);
    }

    let (v, f) = match plane {
        Some(pl) => fine.mirrored(pl, area.sqrt() * 1e-5),
        None => fine.compact(),
    };
    let mut out = Mesh::new();
    let pts: Vec<Index> = v.iter().map(|p| out.add_point(*p)).collect();
    for q in f {
        out.add_face(&q.map(|p| pts[p as usize]));
    }
    out
}

/// Sharp-edge features that are only bits: runs from a loose end shorter than min_len, and whole runs or loops shorter
/// than it, are dropped, until none are left. Borders and the symmetry cut stay.
fn prune_features(t: &mut Tri, min_len: f64) {
    if min_len <= 0.0 {
        return;
    }
    let fixed = feature::BORDER | feature::SYMMETRY;
    for _ in 0..8 {
        let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
        for &(a, b) in t.features.keys() {
            adj.entry(a).or_default().push(b);
            adj.entry(b).or_default().push(a);
        }
        let deg = |p: u32| adj.get(&p).map(|v| v.len()).unwrap_or(0);
        let mut seen: HashSet<(u32, u32)> = HashSet::new();
        let mut drop: Vec<(u32, u32)> = Vec::new();
        let mut keys: Vec<(u32, u32)> = t.features.keys().copied().collect();
        keys.sort_unstable();
        for k in keys {
            if seen.contains(&k) {
                continue;
            }
            // The run through k: out both ways while points have two feature edges.
            let mut run = vec![k];
            seen.insert(k);
            let mut ends = Vec::new();
            for (start, from) in [(k.1, k.0), (k.0, k.1)] {
                let (mut cur, mut prev) = (start, from);
                loop {
                    if deg(cur) != 2 {
                        ends.push(deg(cur));
                        break;
                    }
                    let next = adj[&cur].iter().copied().find(|&x| x != prev).unwrap_or(prev);
                    let e = key(cur, next);
                    if seen.contains(&e) {
                        ends.push(2);
                        break;
                    }
                    seen.insert(e);
                    run.push(e);
                    prev = cur;
                    cur = next;
                }
            }
            if run.iter().any(|e| t.features[e] & fixed != 0) {
                continue;
            }
            let len: f64 = run.iter().map(|&(a, b)| (t.v[a as usize] - t.v[b as usize]).length()).sum();
            // A run that turns back and forth at every point follows the triangulation, not a sharp edge.
            let mut turn = 0.0;
            let mut turns = 0.0;
            {
                let mut pts: HashMap<u32, Vec<u32>> = HashMap::new();
                for &(a, b) in &run {
                    pts.entry(a).or_default().push(b);
                    pts.entry(b).or_default().push(a);
                }
                for (p, n) in &pts {
                    if n.len() == 2 {
                        let d0 = (t.v[*p as usize] - t.v[n[0] as usize]).normalize_or_zero();
                        let d1 = (t.v[n[1] as usize] - t.v[*p as usize]).normalize_or_zero();
                        turn += d0.dot(d1).clamp(-1.0, 1.0).acos();
                        turns += 1.0;
                    }
                }
            }
            let ragged = turns >= 3.0 && turn / turns > 35f64.to_radians();
            if ragged || len < min_len {
                drop.extend(run);
            }
        }
        if drop.is_empty() {
            break;
        }
        for e in drop {
            t.features.remove(&e);
        }
    }
}

/// How good a quad mesh is: the share of quads, of irregular points, the mean scaled Jacobian of its corners, and
/// its mean and largest distance from a source surface.
#[derive(Clone, Debug, Default)]
pub struct Quality {
    pub faces: usize,
    pub quads: f64,
    pub irregular: f64,
    pub scaled_jacobian: f64,
    pub min_jacobian: f64,
    pub mean_distance: f64,
    pub max_distance: f64,
}

pub fn quality(m: &Mesh, source: &Mesh) -> Quality {
    let mut q = Quality { min_jacobian: 1.0, ..Default::default() };
    let faces: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
    q.faces = faces.len();
    if faces.is_empty() {
        return q;
    }
    q.quads = faces.iter().filter(|&&f| m.face_size(f) == 4).count() as f64 / faces.len() as f64;
    let mut irr = 0usize;
    let mut inner = 0usize;
    for p in 0..m.point_capacity() {
        if !m.point_valid(p) || m.is_boundary_point(p) {
            continue;
        }
        inner += 1;
        if m.point_valence(p) != 4 {
            irr += 1;
        }
    }
    q.irregular = irr as f64 / inner.max(1) as f64;
    let mut sj = 0.0;
    let mut n = 0.0;
    for &f in &faces {
        if m.face_size(f) != 4 {
            continue;
        }
        let pts = m.face_points(f);
        let nrm = m.face_normal(f);
        for i in 0..4 {
            let a = m.position(pts[(i + 3) % 4]);
            let b = m.position(pts[i]);
            let c = m.position(pts[(i + 1) % 4]);
            let j = (c - b).normalize_or_zero().cross((a - b).normalize_or_zero()).dot(nrm);
            sj += j;
            n += 1.0;
            q.min_jacobian = q.min_jacobian.min(j);
        }
    }
    q.scaled_jacobian = if n > 0.0 { sj / n } else { 0.0 };
    let bvh = MeshBvh::new(source);
    let mut sum = 0.0f64;
    let mut cnt = 0.0f64;
    for p in 0..m.point_capacity() {
        if m.point_valid(p) {
            if let Some(h) = bvh.closest_point(m.position(p)) {
                sum += h.distance;
                cnt += 1.0;
                q.max_distance = q.max_distance.max(h.distance);
            }
        }
    }
    q.mean_distance = sum / cnt.max(1.0);
    q
}
