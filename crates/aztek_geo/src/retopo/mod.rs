//! Automatic retopology: a new mesh of even, well-placed quads over a surface.
//!
//! The pipeline, after Instant Field-Aligned Meshes (Jakob et al. 2015) with the feature handling of feature-line
//! driven remeshing (QuadWild, Pietroni et al. 2021) and curvature-adapted density as in QuadriFlow (Huang et al. 2018):
//!
//! 1. The faces are triangulated, too-long edges split so the fields have enough points to work with, and the
//!    features found: borders, hard edges, edges sharper than an angle, UV seams, material borders, and the borders
//!    of regions segmented by shape (Variational Shape Approximation) or by colour.
//! 2. A 4-way orientation field is smoothed over a hierarchy, held to the features and drawn toward the principal
//!    curvature directions where the surface bends one way more than the other.
//! 3. A 4-way position field places a lattice of the wanted edge length on it, its lines held along the features and
//!    its points at feature corners; the edge length can follow curvature.
//! 4. The lattice is read out as a mesh, made manifold, holes filled, then optionally every face split into quads,
//!    and the points relaxed and put back on the surface, feature points onto their features.
//!
//! Methods weigh these differently: Organic (smooth flow, density following curvature), Hard Surface (sharp edges and
//! flat regions followed exactly, even density), Character (edge loops along the curvature, symmetric, denser where
//! the shape is detailed) and Segmented (regions by shape and colour, each meshed to its border).

pub mod extract;
pub mod field;
pub mod segment;
pub mod tri;

use crate::attribute::{Index, INVALID};
use crate::bvh::MeshBvh;
use crate::mesh::Mesh;
use crate::symmetry::SymmetryPlane;
use field::Level;
use glam::DVec3;
use std::collections::{HashMap, HashSet};
use tri::{feature, key, Curvature, Tri};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Method {
    /// Smooth flow and density following curvature: sculpts, scans, organic shapes.
    #[default]
    Organic,
    /// Sharp edges and flat regions followed exactly, even density: machined and modelled hard surfaces.
    HardSurface,
    /// Loops along the curvature, symmetric where the shape is, more faces where the shape is detailed.
    Character,
    /// Regions by shape and colour, each meshed to its border.
    Segmented,
}

impl Method {
    pub fn from_index(i: i64) -> Method {
        match i {
            1 => Method::HardSurface,
            2 => Method::Character,
            3 => Method::Segmented,
            _ => Method::Organic,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RetopoOptions {
    pub method: Method,
    /// Faces wanted; 0 lets target_edge_deviation set the density.
    pub target_faces: usize,
    /// Percent the count may miss by before the solve is redone with a corrected edge length.
    pub tolerance: f64,
    /// Follow the mesh's hard edges.
    pub preserve_hard_edges: bool,
    /// Follow edges whose faces bend more than `angle`.
    pub edges_by_angle: bool,
    /// Radians.
    pub angle: f64,
    /// 1 even density everywhere, 0 density following curvature fully.
    pub face_uniformity: f64,
    /// 0..1: how far quads stretch along the direction the surface bends least.
    pub anisotropy: f64,
    /// 0..1: low for fewer singularities and smoother flow, high to follow detail.
    pub topology_regularity: f64,
    /// When target_faces is 0: edges about half this times the surface's smaller radii.
    pub target_edge_deviation: f64,
    /// Mirror across this plane: the result is exactly symmetric.
    pub symmetry: Option<SymmetryPlane>,
    pub segment_by_shape: bool,
    pub segment_by_color: bool,
    /// Follow UV seams and material borders.
    pub follow_seams: bool,
    /// Split every face into quads (the lattice is solved at twice the edge length).
    pub pure_quads: bool,
    /// Reduce a dense input first.
    pub preprocess: bool,
    /// Extra feature edges, as source mesh edges.
    pub feature_edges: Vec<Index>,
    /// Per source face colour sampled from a texture, linear RGB; empty when none.
    pub face_colors: Vec<[f32; 3]>,
}

impl Default for RetopoOptions {
    fn default() -> Self {
        Self {
            method: Method::Organic,
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
            segment_by_shape: false,
            segment_by_color: false,
            follow_seams: false,
            pure_quads: true,
            preprocess: true,
            feature_edges: Vec::new(),
            face_colors: Vec::new(),
        }
    }
}

impl RetopoOptions {
    /// The defaults of a method: what it follows and how it weighs density and flow.
    pub fn for_method(method: Method) -> Self {
        let mut o = RetopoOptions { method, ..Default::default() };
        match method {
            Method::Organic => {
                o.face_uniformity = 0.6;
                o.topology_regularity = 0.4;
            }
            Method::HardSurface => {
                o.edges_by_angle = true;
                o.preserve_hard_edges = true;
                o.face_uniformity = 1.0;
                o.anisotropy = 0.0;
                o.topology_regularity = 0.3;
                o.segment_by_shape = true;
                o.follow_seams = true;
            }
            Method::Character => {
                o.face_uniformity = 0.35;
                o.topology_regularity = 0.7;
                o.anisotropy = 0.5;
            }
            Method::Segmented => {
                o.segment_by_shape = true;
                o.segment_by_color = true;
                o.follow_seams = true;
                o.edges_by_angle = true;
                o.face_uniformity = 0.7;
            }
        }
        o
    }
}

/// Retopologizes the given faces of m as a new mesh. Faces of other shells than these are not part of it.
pub fn retopologize(m: &Mesh, faces: &[Index], opts: &RetopoOptions) -> Mesh {
    if let Some(plane) = opts.symmetry {
        return symmetric(m, faces, opts, &plane);
    }
    run(m, faces, opts, None)
}

/// Half the mesh, retopologized with the cut held as a feature, then mirrored and joined.
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
    let mut out = run(&half, &all, &o, Some(plane));
    let mo = crate::ops::MirrorOptions {
        plane: *plane,
        direction: crate::ops::MirrorDirection::Both,
        weld: true,
        tolerance: diag * 1e-4,
        ..Default::default()
    };
    crate::ops::mirror_mesh(&mut out, &mo);
    out.compact(None);
    out
}

/// What one solve made, with the edge length it was solved for.
struct Attempt {
    mesh: Mesh,
    faces: usize,
}

fn run(m: &Mesh, faces: &[Index], opts: &RetopoOptions, plane: Option<&SymmetryPlane>) -> Mesh {
    // The source, reduced first when it's far denser than needed.
    let mut src = crate::ops::extract_faces(m, faces);
    src.compact(None);
    if src.num_faces() == 0 {
        return Mesh::new();
    }
    let target = opts.target_faces;
    if opts.preprocess && target > 0 {
        let tris: usize = (0..src.face_capacity()).filter(|&f| src.face_valid(f)).map(|f| src.face_size(f) as usize - 2).sum();
        let want = (target * 40).max(60_000);
        if tris > want * 2 {
            let ro = crate::reduce::ReduceOptions { target_triangles: want as Index, keep_hard_edges: true, ..Default::default() };
            src = crate::reduce::reduce_mesh(&src, &ro).mesh;
            src.compact(None);
        }
    }
    let all: Vec<Index> = (0..src.face_capacity()).filter(|&f| src.face_valid(f)).collect();
    let (mut t, orig) = Tri::from_mesh(&src, &all);
    // Extra feature edges given by the caller, as pairs of source points.
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
    // Which features to keep.
    let mut keep_flags = feature::BORDER | feature::SEGMENT | feature::SYMMETRY;
    if opts.preserve_hard_edges || !opts.feature_edges.is_empty() {
        keep_flags |= feature::HARD;
    }
    if opts.follow_seams {
        keep_flags |= feature::MATERIAL;
    }
    t.features.retain(|_, f| *f & keep_flags != 0);
    if opts.follow_seams {
        let seams: Vec<(u32, u32)> = t.seams.keys().copied().collect();
        for k in seams {
            *t.features.entry(k).or_insert(0) |= feature::SEAM;
        }
    }
    if opts.edges_by_angle {
        t.mark_angle_features(opts.angle);
    }
    if let Some(pl) = plane {
        // The cut: border edges on the plane.
        let tol = (area.sqrt()) * 1e-4;
        for (k, f) in t.features.iter_mut() {
            if pl.distance(t.v[k.0 as usize]).abs() < tol && pl.distance(t.v[k.1 as usize]).abs() < tol {
                *f |= feature::SYMMETRY;
            }
        }
    }
    if opts.segment_by_shape {
        let k = ((target.max(100) as f64).sqrt() * 0.6).clamp(6.0, 60.0) as usize;
        let label = segment::by_shape(&t, k, 6);
        // Only borders between regions that really turn: a cylinder's segments meet at shallow angles.
        let mut avg = HashMap::<u32, (DVec3, f64)>::new();
        for (f, &l) in label.iter().enumerate() {
            let e = avg.entry(l).or_insert((DVec3::ZERO, 0.0));
            e.0 += t.fnormal[f] * t.farea[f];
            e.1 += t.farea[f];
        }
        let normal_of = |l: u32| avg[&l].0.try_normalize().unwrap_or(DVec3::Z);
        let cos = (opts.angle * 0.5).cos();
        let pairs: Vec<((u32, u32), Vec<u32>)> = t.edge_faces.iter().map(|(k, v)| (*k, v.clone())).collect();
        let mut keep_pair = HashSet::new();
        for (_, fs) in &pairs {
            if fs.len() == 2 {
                let (a, b) = (label[fs[0] as usize], label[fs[1] as usize]);
                if a != b && normal_of(a).dot(normal_of(b)) < cos {
                    keep_pair.insert((a.min(b), a.max(b)));
                }
            }
        }
        for (k, fs) in &pairs {
            if fs.len() == 2 {
                let (a, b) = (label[fs[0] as usize], label[fs[1] as usize]);
                if a != b && keep_pair.contains(&(a.min(b), a.max(b))) {
                    *t.features.entry(*k).or_insert(0) |= feature::SEGMENT;
                }
            }
        }
    }
    if opts.segment_by_color {
        if let Some(label) = segment::by_color(&t, 8) {
            t.mark_label_boundaries(&label, feature::COLOR);
        }
    }

    // Edge length: from the face count, or from the surface's radius when there's no count.
    // With every face split into quads the lattice is solved at twice this, so its cells come out a quarter as many.
    let mut edge = if target > 0 {
        (area / target as f64).sqrt()
    } else {
        // Edges about half the deviation times the surface's smaller radii (its tightest quarter), so tighter
        // shapes get smaller faces and a deviation of 0.29 puts some 40 edges round a sphere.
        let d = opts.target_edge_deviation.clamp(1e-3, 1.0);
        let k = Curvature::new(&t, 2);
        let mut radii: Vec<f64> = (0..t.v.len()).map(|i| 1.0 / t.k_abs_max(&k, i).max(1e-9)).collect();
        radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let r = radii[radii.len() / 4].min(area.sqrt());
        (0.5 * d * r).max(area.sqrt() * 1e-3)
    };
    let tol = (opts.tolerance.max(1.0)) / 100.0;
    let mut best: Option<Attempt> = None;
    for attempt in 0..4 {
        let mesh = solve_once(&t, opts, edge, plane);
        let n = mesh.num_faces() as usize;
        let better = match &best {
            None => true,
            Some(b) => target > 0 && (n as f64 - target as f64).abs() < (b.faces as f64 - target as f64).abs(),
        };
        if better {
            best = Some(Attempt { mesh, faces: n });
        }
        if target == 0 || n == 0 {
            break;
        }
        let ratio = n as f64 / target as f64;
        if (ratio - 1.0).abs() <= tol || attempt == 3 {
            break;
        }
        // Faces go with the inverse square of the edge length.
        edge *= ratio.sqrt().clamp(0.5, 2.0);
    }
    best.map(|b| b.mesh).unwrap_or_default()
}

fn m_point_key(m: &Mesh, p: Index) -> (u64, u64, u64) {
    let v = m.position(p);
    (v.x.to_bits(), v.y.to_bits(), v.z.to_bits())
}

impl Tri {
    fn k_abs_max(&self, k: &Curvature, i: usize) -> f64 {
        k.k_min[i].abs().max(k.k_max[i].abs())
    }
}

/// The feature curves: per point its feature edges, which points are corners, and the segments for snapping.
/// Borders between shape or colour regions alone are soft: the flow leans along them but nothing is pinned to them.
struct Features {
    /// Per point: the directions of its feature edges.
    dirs: Vec<Vec<DVec3>>,
    /// Per point: all its feature edges are soft.
    soft: Vec<bool>,
    corner: Vec<bool>,
    segments: Vec<(DVec3, DVec3)>,
    corners: Vec<DVec3>,
}

fn features(t: &Tri) -> Features {
    let nv = t.v.len();
    let mut dirs: Vec<Vec<DVec3>> = vec![Vec::new(); nv];
    let mut hard_count = vec![0usize; nv];
    let mut segments = Vec::new();
    let mut list: Vec<(&(u32, u32), &u8)> = t.features.iter().collect();
    list.sort_by_key(|x| *x.0);
    for (&(a, b), &flags) in list {
        let d = (t.v[b as usize] - t.v[a as usize]).try_normalize().unwrap_or(DVec3::X);
        let soft = flags & !feature::SEGMENT == 0;
        if !soft {
            hard_count[a as usize] += 1;
            hard_count[b as usize] += 1;
            segments.push((t.v[a as usize], t.v[b as usize]));
            dirs[a as usize].insert(0, d);
            dirs[b as usize].insert(0, -d);
        } else {
            dirs[a as usize].push(d);
            dirs[b as usize].push(-d);
        }
    }
    let soft: Vec<bool> = (0..nv).map(|i| hard_count[i] == 0).collect();
    let mut corner = vec![false; nv];
    let mut corners = Vec::new();
    for i in 0..nv {
        let c = match hard_count[i] {
            0 | 1 => false,
            // A bend sharper than 45 degrees along the curve is a corner.
            2 => dirs[i][0].dot(-dirs[i][1]) < 45f64.to_radians().cos(),
            _ => true,
        };
        if c {
            corner[i] = true;
            corners.push(t.v[i]);
        }
    }
    Features { dirs, soft, corner, segments, corners }
}

fn solve_once(t0: &Tri, opts: &RetopoOptions, edge: f64, plane: Option<&SymmetryPlane>) -> Mesh {
    let lattice = if opts.pure_quads { edge * 2.0 } else { edge };
    // Enough points under each lattice cell: edges at most a third of it.
    let mut t = t0.clone();
    t.refine(lattice / 3.0, 1_500_000);
    t.prepare();
    let nv = t.v.len();
    let feats = features(&t);
    let curv = Curvature::new(&t, 4 + (6.0 * (1.0 - opts.topology_regularity)) as usize);

    // Edge length per point: curvature-adapted, then scaled to keep the count.
    let mut scale = vec![lattice; nv];
    // Half strength at most: sizing fully to the curvature tears the lattice where the size changes.
    let adapt = 0.5 * (1.0 - opts.face_uniformity).clamp(0.0, 1.0);
    if adapt > 0.0 {
        // Each point's local radius against the median, edge length following the radius' square root.
        let kk: Vec<f64> = (0..nv).map(|i| t.k_abs_max(&curv, i)).collect();
        let mut sorted = kk.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let med = sorted[sorted.len() / 2].max(1e-12);
        for i in 0..nv {
            let rel = (med / kk[i].max(med * 0.05)).clamp(0.25, 4.0);
            scale[i] = lattice * rel.powf(0.5 * adapt);
        }
        // Smooth it so neighbouring cells don't jump, then normalize so the cell count stays the same.
        for _ in 0..8 {
            let prev = scale.clone();
            for i in 0..nv {
                let mut s = prev[i];
                for &j in &t.adj[i] {
                    s += prev[j as usize];
                }
                scale[i] = s / (t.adj[i].len() + 1) as f64;
            }
        }
        let want: f64 = t.area.iter().sum::<f64>() / (lattice * lattice);
        let got: f64 = (0..nv).map(|i| t.area[i] / (scale[i] * scale[i])).sum();
        let fix = (got / want).sqrt();
        for s in &mut scale {
            *s *= fix;
        }
    }

    // The finest level and its constraints.
    let mut l = Level {
        v: t.v.clone(),
        n: t.n.clone(),
        area: t.area.clone(),
        adj: t.adj.iter().map(|a| a.iter().map(|&j| (j, 1.0)).collect()).collect(),
        scale,
        q: vec![DVec3::ZERO; nv],
        o: vec![DVec3::ZERO; nv],
        cq: vec![DVec3::ZERO; nv],
        cq_w: vec![0.0; nv],
        co: vec![DVec3::ZERO; nv],
        co_w: vec![0.0; nv],
        pin: vec![false; nv],
        to_coarse: Vec::new(),
    };
    // Curvature: lean toward the principal directions where the surface bends one way more than the other.
    let curv_weight = match opts.method {
        Method::HardSurface => 0.0,
        Method::Character => 0.2 + 0.4 * opts.topology_regularity,
        Method::Organic => 0.15 + 0.4 * opts.topology_regularity,
        Method::Segmented => 0.1 + 0.3 * opts.topology_regularity,
    };
    if curv_weight > 0.0 {
        // Kept light: each point's own principal direction is noisy, and a strong pull makes singularities.
        let cw = 0.5;
        for i in 0..nv {
            // How much more the surface bends one way than the other across one cell: flat or round places, where
            // the principal directions are noise, get no pull.
            let bend = (curv.k_max[i].abs() - curv.k_min[i].abs()) * lattice;
            let w = ((bend - 0.15) / 0.5).clamp(0.0, 1.0) * curv.anisotropy(i);
            if w > 0.0 {
                l.cq[i] = curv.dir_min[i];
                l.cq_w[i] = (curv_weight * cw * w).min(0.9);
            }
        }
    }
    for i in 0..nv {
        let d = &feats.dirs[i];
        if d.is_empty() {
            continue;
        }
        if feats.soft[i] {
            // A region border: lean along it.
            let dir = if d.len() >= 2 { (d[0] - d[1]).try_normalize().unwrap_or(d[0]) } else { d[0] };
            l.cq[i] = dir;
            l.cq_w[i] = l.cq_w[i].max(0.6);
        } else if feats.corner[i] {
            l.cq[i] = d[0];
            l.cq_w[i] = 1.0;
            l.co[i] = t.v[i];
            l.co_w[i] = 1.0;
            l.pin[i] = true;
        } else {
            // Along the curve: the average of its two directions, the second turned to agree with the first.
            let dir = if d.len() >= 2 { (d[0] - d[1]).try_normalize().unwrap_or(d[0]) } else { d[0] };
            l.cq[i] = dir;
            l.cq_w[i] = 1.0;
            l.co[i] = t.v[i];
            l.co_w[i] = 1.0;
        }
    }
    let mut levels = field::build_hierarchy(l);
    let iters = 10 + (14.0 * (1.0 - opts.topology_regularity)) as usize;
    let soft_from = 2;
    field::solve(&mut levels, iters, iters, soft_from);
    let ex = extract::extract(&levels[0]);
    let mut faces = ex.faces;
    extract::make_manifold(&ex.v, &mut faces);
    extract::pair_irregular(&ex.v, &mut faces);
    // To a mesh.
    let mut out = Mesh::new();
    let mut map = vec![INVALID; ex.v.len()];
    let mut used = vec![false; ex.v.len()];
    for f in &faces {
        for &p in f {
            used[p as usize] = true;
        }
    }
    for (i, p) in ex.v.iter().enumerate() {
        if used[i] {
            map[i] = out.add_point(*p);
        }
    }
    for f in &faces {
        let pts: Vec<Index> = f.iter().map(|&p| map[p as usize]).collect();
        out.add_face(&pts);
    }
    out.compact(None);
    let src_bvh = {
        let mut sm = Mesh::new();
        let pts: Vec<Index> = t.v.iter().map(|p| sm.add_point(*p)).collect();
        for f in &t.f {
            sm.add_face(&[pts[f[0] as usize], pts[f[1] as usize], pts[f[2] as usize]]);
        }
        MeshBvh::new(&sm)
    };
    let source_closed = t.edge_faces.values().all(|v| v.len() == 2);
    fill_holes(&mut out, source_closed, &feats, edge, &src_bvh);
    // Faces of five or more sides become quads and triangles; then optionally all quads.
    split_large(&mut out);
    if opts.pure_quads {
        out = split_to_quads(&out);
    }
    relax(&mut out, &src_bvh, &feats, plane, edge, if opts.method == Method::HardSurface { 3 } else { 6 });
    drop_crumbs(&mut out);
    out
}

/// Pieces of a few faces cut off from the rest, where the reading tore: gone, unless the mesh is all such pieces.
fn drop_crumbs(m: &mut Mesh) {
    let faces: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
    if faces.len() < 100 {
        return;
    }
    let mut comp: HashMap<Index, usize> = HashMap::new();
    let mut parts: Vec<Vec<Index>> = Vec::new();
    for &f in &faces {
        if comp.contains_key(&f) {
            continue;
        }
        let id = parts.len();
        let mut stack = vec![f];
        comp.insert(f, id);
        let mut part = Vec::new();
        while let Some(x) = stack.pop() {
            part.push(x);
            for c in m.face_corners(x) {
                let e = m.corner_edge(c);
                m.for_each_edge_corner(e, |c2| {
                    let g = m.corner_face(c2);
                    if let std::collections::hash_map::Entry::Vacant(v) = comp.entry(g) {
                        v.insert(id);
                        stack.push(g);
                    }
                });
            }
        }
        parts.push(part);
    }
    let biggest = parts.iter().map(|p| p.len()).max().unwrap_or(0);
    for p in &parts {
        if p.len() < 8 && p.len() * 50 < biggest {
            for &f in p {
                m.remove_face(f);
            }
        }
    }
    m.remove_all_loose();
    m.compact(None);
}

/// Every face of n sides becomes n quads about its centre, through its edges' middles. Positions stay where they
/// were; the relaxation after puts them on the surface.
fn split_to_quads(m: &Mesh) -> Mesh {
    let mut out = Mesh::new();
    let mut pmap = vec![INVALID; m.point_capacity() as usize];
    for p in 0..m.point_capacity() {
        if m.point_valid(p) {
            pmap[p as usize] = out.add_point(m.position(p));
        }
    }
    let mut mid: HashMap<(Index, Index), Index> = HashMap::new();
    for f in 0..m.face_capacity() {
        if !m.face_valid(f) {
            continue;
        }
        let pts = m.face_points(f);
        let k = pts.len();
        let c = out.add_point(m.face_centroid(f));
        let mids: Vec<Index> = (0..k)
            .map(|i| {
                let (a, b) = (pts[i], pts[(i + 1) % k]);
                let kk = if a < b { (a, b) } else { (b, a) };
                *mid.entry(kk).or_insert_with(|| out.add_point((m.position(a) + m.position(b)) * 0.5))
            })
            .collect();
        for i in 0..k {
            out.add_face(&[pmap[pts[i] as usize], mids[i], c, mids[(i + k - 1) % k]]);
        }
    }
    out
}

/// Holes the reading left: on a closed source all of them, else those away from its borders. A hole of up to six
/// sides becomes one face; a larger one a fan of triangles about a point on the surface, which the split into quads
/// then evens out.
fn fill_holes(m: &mut Mesh, source_closed: bool, feats: &Features, edge: f64, bvh: &MeshBvh) {
    let border: Vec<Index> = (0..m.edge_capacity()).filter(|&e| m.edge_valid(e) && m.is_boundary_edge(e)).collect();
    if border.is_empty() {
        return;
    }
    let mut next: HashMap<Index, Vec<Index>> = HashMap::new();
    for &e in &border {
        let c = m.edge_corner(e);
        if c == INVALID {
            continue;
        }
        let a = m.corner_point(c);
        let b = m.corner_next_point(c);
        // Holes run the other way round from the faces about them.
        next.entry(b).or_default().push(a);
    }
    // Walk border edges, each once; where a point has two ways on, the one turning least sharply.
    for v in next.values_mut() {
        v.sort_unstable();
    }
    let mut starts: Vec<Index> = next.keys().copied().collect();
    starts.sort_unstable();
    let mut used: HashSet<(Index, Index)> = HashSet::new();
    let grid = SegmentGrid::new(&feats.segments, edge);
    let near_source_border = |p: DVec3| grid.closest(p).map(|x| x.0 < edge * 0.6).unwrap_or(false);
    let mut fills = Vec::new();
    for start in starts {
        for &first in next[&start].clone().iter() {
            if used.contains(&(start, first)) {
                continue;
            }
            let mut lp = vec![start];
            used.insert((start, first));
            let (mut prev, mut cur) = (start, first);
            let mut ok = false;
            for _ in 0..4096 {
                if cur == start {
                    ok = true;
                    break;
                }
                lp.push(cur);
                let Some(cands) = next.get(&cur) else { break };
                let free: Vec<Index> = cands.iter().copied().filter(|&n| !used.contains(&(cur, n))).collect();
                if free.is_empty() {
                    break;
                }
                let n = if free.len() == 1 {
                    free[0]
                } else {
                    let din = (m.position(cur) - m.position(prev)).normalize_or_zero();
                    *free
                        .iter()
                        .max_by(|&&a, &&b| {
                            let da = (m.position(a) - m.position(cur)).normalize_or_zero().dot(din);
                            let db = (m.position(b) - m.position(cur)).normalize_or_zero().dot(din);
                            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .unwrap()
                };
                used.insert((cur, n));
                prev = cur;
                cur = n;
            }
            if !ok || lp.len() < 3 {
                continue;
            }
            // A loop through a point twice is two holes touching: fill it as it is, the fan handles it.
            // A hole along the source's own border (or a symmetry cut) is that border: left open.
        if !source_closed && lp.iter().filter(|&&p| near_source_border(m.position(p))).count() * 10 >= lp.len() * 7 {
                continue;
            }
            fills.push(lp);
        }
    }
    for lp in fills {
        if lp.len() <= 6 {
            m.add_face(&lp);
        } else {
            let c: DVec3 = lp.iter().map(|&p| m.position(p)).sum::<DVec3>() / lp.len() as f64;
            let c = bvh.closest_point(c).map(|h| h.point).unwrap_or(c);
            let mid = m.add_point(c);
            for i in 0..lp.len() {
                m.add_face(&[lp[i], lp[(i + 1) % lp.len()], mid]);
            }
        }
    }
}

/// Faces of five or more sides split into quads and at most one triangle, along their shortest diagonals.
fn split_large(m: &mut Mesh) {
    let big: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f) && m.face_size(f) > 4).collect();
    if big.is_empty() {
        return;
    }
    let o = crate::ops::NgonSplitOptions::default();
    crate::ops::split_ngons(m, &big, &o);
}

/// Points relaxed along the surface and put back on it; points near a feature onto it, near a corner onto it.
fn relax(m: &mut Mesh, bvh: &MeshBvh, feats: &Features, plane: Option<&SymmetryPlane>, edge: f64, passes: usize) {
    let np = m.point_capacity();
    let grid = SegmentGrid::new(&feats.segments, edge);
    // Which points sit on a feature, decided once from where they start.
    let mut on_feature = vec![false; np as usize];
    let mut on_corner: Vec<Option<DVec3>> = vec![None; np as usize];
    // Each feature corner takes the point nearest it, if one is within most of an edge: the lattice can't always
    // land a point on every corner, and the nearest one moving there keeps the corner sharp.
    let live: Vec<Index> = (0..np).filter(|&p| m.point_valid(p)).collect();
    let mut taken = HashSet::new();
    for c in &feats.corners {
        let best = live
            .iter()
            .filter(|p| !taken.contains(*p))
            .map(|&p| ((m.position(p) - *c).length(), p))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if let Some((d, p)) = best {
            if d < edge * 1.5 {
                on_corner[p as usize] = Some(*c);
                taken.insert(p);
            }
        }
    }
    for &p in &live {
        if on_corner[p as usize].is_none() {
            if let Some((d, _)) = grid.closest(m.position(p)) {
                on_feature[p as usize] = d < edge * 0.5;
            }
        }
    }
    let on_plane: Vec<bool> = (0..np)
        .map(|p| {
            // Only the border along the cut goes onto the plane: inner points near it keep their shape.
            plane.map(|pl| m.point_valid(p) && m.is_boundary_point(p) && pl.distance(m.position(p)).abs() < edge * 1.6).unwrap_or(false)
        })
        .collect();
    for pass in 0..passes + 4 {
        // The last passes only untangle: points at a folded corner move to their neighbours' middle.
        let untangle_only = pass >= passes;
        let folded: HashSet<Index> = if untangle_only { folded_points(m) } else { HashSet::new() };
        if untangle_only && folded.is_empty() {
            break;
        }
        let mut newp = m.positions().to_vec();
        for p in 0..np {
            if !m.point_valid(p) {
                continue;
            }
            if let Some(c) = on_corner[p as usize] {
                newp[p as usize] = c;
                continue;
            }
            if untangle_only && !folded.contains(&p) {
                continue;
            }
            let x = m.position(p);
            let mut sum = DVec3::ZERO;
            let mut cnt = 0.0;
            m.for_each_point_edge(p, |e| {
                let q = m.edge_other_point(e, p);
                // A feature point is pulled only by its neighbours along the feature.
                if !on_feature[p as usize] || on_feature[q as usize] || on_corner[q as usize].is_some() {
                    sum += m.position(q);
                    cnt += 1.0;
                }
            });
            let mut y = if cnt > 0.0 { x.lerp(sum / cnt, if untangle_only { 1.0 } else { 0.5 }) } else { x };
            if on_feature[p as usize] {
                if let Some((_, c)) = grid.closest(y) {
                    y = c;
                }
            } else if let Some(h) = bvh.closest_point(y) {
                y = h.point;
            }
            if on_plane[p as usize] {
                if let Some(pl) = plane {
                    y = pl.project(y);
                }
            }
            newp[p as usize] = y;
        }
        for p in 0..np {
            if m.point_valid(p) {
                m.set_position(p, newp[p as usize]);
            }
        }
    }
}

/// Points at a corner of a face that folds over: the corner turns the other way from the face's normal.
fn folded_points(m: &Mesh) -> HashSet<Index> {
    let mut out = HashSet::new();
    for f in 0..m.face_capacity() {
        if !m.face_valid(f) {
            continue;
        }
        let pts = m.face_points(f);
        let k = pts.len();
        let n = m.face_normal(f);
        for i in 0..k {
            let a = m.position(pts[(i + k - 1) % k]);
            let b = m.position(pts[i]);
            let c = m.position(pts[(i + 1) % k]);
            if (c - b).cross(a - b).dot(n) <= 0.0 {
                out.insert(pts[i]);
            }
        }
    }
    out
}

fn point_segment(p: DVec3, a: DVec3, b: DVec3) -> (f64, DVec3) {
    let ab = b - a;
    let l2 = ab.length_squared();
    let t = if l2 > 0.0 { ((p - a).dot(ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    let c = a + ab * t;
    ((p - c).length(), c)
}

/// Feature segments in a grid of cells the edge length wide, for the nearest point on any of them.
struct SegmentGrid<'a> {
    segs: &'a [(DVec3, DVec3)],
    cell: f64,
    map: HashMap<(i64, i64, i64), Vec<u32>>,
}

impl<'a> SegmentGrid<'a> {
    fn new(segs: &'a [(DVec3, DVec3)], cell: f64) -> Self {
        let cell = cell.max(1e-9);
        let mut map: HashMap<(i64, i64, i64), Vec<u32>> = HashMap::new();
        for (i, (a, b)) in segs.iter().enumerate() {
            let lo = a.min(*b);
            let hi = a.max(*b);
            let c0 = (lo / cell).floor();
            let c1 = (hi / cell).floor();
            for x in c0.x as i64..=c1.x as i64 {
                for y in c0.y as i64..=c1.y as i64 {
                    for z in c0.z as i64..=c1.z as i64 {
                        map.entry((x, y, z)).or_default().push(i as u32);
                    }
                }
            }
        }
        SegmentGrid { segs, cell, map }
    }
    fn closest(&self, p: DVec3) -> Option<(f64, DVec3)> {
        let c = (p / self.cell).floor();
        let mut best: Option<(f64, DVec3)> = None;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(list) = self.map.get(&(c.x as i64 + dx, c.y as i64 + dy, c.z as i64 + dz)) {
                        for &i in list {
                            let (a, b) = self.segs[i as usize];
                            let r = point_segment(p, a, b);
                            if best.map(|x| r.0 < x.0).unwrap_or(true) {
                                best = Some(r);
                            }
                        }
                    }
                }
            }
        }
        best
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
