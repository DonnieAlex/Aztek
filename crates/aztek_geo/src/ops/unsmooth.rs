//! Unsmooth: one level of Catmull-Clark subdivision undone. The fine mesh's points fall into three kinds, the cage's
//! own points, edge points and face points: edges join edge points to the other two, so the point graph is two-coloured;
//! each quad holds a cage point and a face point on one diagonal. The cage is read off the face points (each one a cage
//! face, its quads round it giving the cage points in order), and its positions are solved so that subdividing it again
//! lands its points where the fine mesh has them.

use std::collections::HashMap;

use glam::DVec3;

use crate::ops::subdivide::{subdivide, SubdivideMap, SubdivideOptions};
use crate::{Index, Mesh, INVALID};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Unset,
    Cage,
    Edge,
    Face,
}

/// Points joined by an edge.
fn neighbours(m: &Mesh, p: Index) -> Vec<Index> {
    let mut out = Vec::new();
    m.for_each_point_edge(p, |e| {
        let (a, b) = (m.edge_point(e, 0), m.edge_point(e, 1));
        out.push(if a == p { b } else { a });
    });
    out
}

/// The kinds of one connected part's points, the part given by its points; None when they can't be read as a
/// subdivided mesh. Of the readings that work, the one whose face points have four quads round them most often wins
/// (a quad cage), then the one with fewer face points.
fn classify(m: &Mesh, part: &[Index], colour: &HashMap<Index, bool>) -> Option<HashMap<Index, Kind>> {
    let mut best: Option<(usize, usize, HashMap<Index, Kind>)> = None;
    for edge_colour in [false, true] {
        // Edge points: four quads round each inside, two on a border.
        let ok = part.iter().filter(|p| colour[p] == edge_colour).all(|&p| {
            let n = m.point_faces(p).len();
            if m.is_boundary_point(p) { n == 2 } else { n == 4 }
        });
        if !ok {
            continue;
        }
        // The rest split along the quads' diagonals.
        let mut diag: HashMap<Index, Vec<Index>> = HashMap::new();
        for &p in part {
            if colour[&p] == edge_colour {
                continue;
            }
            for f in m.point_faces(p) {
                let pts = m.face_points(f);
                let i = pts.iter().position(|&x| x == p)?;
                diag.entry(p).or_default().push(pts[(i + 2) % 4]);
            }
        }
        let seed = part.iter().copied().find(|p| colour[p] != edge_colour)?;
        for seed_kind in [Kind::Cage, Kind::Face] {
            let mut kind: HashMap<Index, Kind> = part.iter().map(|&p| (p, if colour[&p] == edge_colour { Kind::Edge } else { Kind::Unset })).collect();
            kind.insert(seed, seed_kind);
            let mut stack = vec![seed];
            let mut bad = false;
            while let Some(p) = stack.pop() {
                let other = if kind[&p] == Kind::Cage { Kind::Face } else { Kind::Cage };
                for &q in diag.get(&p).into_iter().flatten() {
                    match kind[&q] {
                        Kind::Unset => {
                            kind.insert(q, other);
                            stack.push(q);
                        }
                        k if k != other => bad = true,
                        _ => {}
                    }
                }
            }
            // A face point is inside a cage face: never on a border, and with three quads or more round it.
            bad |= kind.iter().any(|(&p, &k)| k == Kind::Unset || (k == Kind::Face && (m.is_boundary_point(p) || m.point_faces(p).len() < 3)));
            if bad {
                continue;
            }
            let faces: Vec<Index> = kind.iter().filter(|(_, &k)| k == Kind::Face).map(|(&p, _)| p).collect();
            let off = faces.iter().filter(|&&p| m.point_faces(p).len() != 4).count();
            if best.as_ref().is_none_or(|(o, n, _)| (off, faces.len()) < (*o, *n)) {
                best = Some((off, faces.len(), kind));
            }
        }
    }
    best.map(|b| b.2)
}

/// One level of Catmull-Clark undone, with borders kept sharp as the subdivision keeps them. None when the mesh
/// isn't all quads or can't be read as subdivided.
pub fn unsmooth(m: &Mesh) -> Option<Mesh> {
    let faces: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
    if faces.is_empty() || faces.iter().any(|&f| m.face_size(f) != 4) {
        return None;
    }
    // Two colours over the point graph, part by part.
    let mut colour: HashMap<Index, bool> = HashMap::new();
    let mut kind: HashMap<Index, Kind> = HashMap::new();
    for start in 0..m.point_capacity() {
        if !m.point_valid(start) || colour.contains_key(&start) || m.point_faces(start).is_empty() {
            continue;
        }
        let mut part = vec![start];
        colour.insert(start, false);
        let mut i = 0;
        while i < part.len() {
            let p = part[i];
            i += 1;
            for q in neighbours(m, p) {
                match colour.get(&q) {
                    None => {
                        colour.insert(q, !colour[&p]);
                        part.push(q);
                    }
                    Some(&c) if c == colour[&p] => return None,
                    _ => {}
                }
            }
        }
        kind.extend(classify(m, &part, &colour)?);
    }

    // The cage: its points, and a face per face point, its quads round it giving the cage points in order.
    let mut index: HashMap<Index, Index> = HashMap::new();
    let mut fine_of: Vec<Index> = Vec::new();
    for p in 0..m.point_capacity() {
        if kind.get(&p) == Some(&Kind::Cage) {
            index.insert(p, fine_of.len() as Index);
            fine_of.push(p);
        }
    }
    let (mut sizes, mut corners) = (Vec::new(), Vec::new());
    let mut centres: Vec<Index> = kind.iter().filter(|(_, &k)| k == Kind::Face).map(|(&p, _)| p).collect();
    centres.sort_unstable();
    for fp in centres {
        // Each quad round it, read from it: fp, a, cage point, b. The next cage point is in the quad starting with b.
        let mut next: HashMap<Index, (Index, Index)> = HashMap::new();
        for f in m.point_faces(fp) {
            let pts = m.face_points(f);
            let i = pts.iter().position(|&x| x == fp)?;
            next.insert(pts[(i + 1) % 4], (pts[(i + 2) % 4], pts[(i + 3) % 4]));
        }
        let first = *next.keys().min()?;
        let mut at = first;
        let mut ring = Vec::new();
        for _ in 0..next.len() {
            let (o, b) = *next.get(&at)?;
            ring.push(*index.get(&o)?);
            at = b;
            if at == first {
                break;
            }
        }
        if at != first || ring.len() != next.len() {
            return None;
        }
        sizes.push(ring.len() as Index);
        corners.extend(ring);
    }
    let target: Vec<DVec3> = fine_of.iter().map(|&p| m.position(p)).collect();
    let mut cage = Mesh::from_polygons(&target, &sizes, &corners);

    // Positions: subdividing the cage should put its points back where they were.
    let opts = SubdivideOptions { levels: 1, sharp_boundary: true, keep_hard_edges: false };
    for _ in 0..60 {
        let mut map = SubdivideMap::default();
        let fine = subdivide(&cage, &opts, Some(&mut map));
        let mut worst: f64 = 0.0;
        for (c, &t) in target.iter().enumerate() {
            let at = map.points.get(c).copied().unwrap_or(INVALID);
            if at == INVALID {
                continue;
            }
            let r = t - fine.position(at);
            worst = worst.max(r.length());
            cage.set_position(c as Index, cage.position(c as Index) + r);
        }
        if worst < 1e-10 {
            break;
        }
    }
    Some(cage)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cube() -> Mesh {
        let p: Vec<DVec3> = (0..8).map(|i| DVec3::new((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64) * 2.0 - 1.0).collect();
        let f: [[Index; 4]; 6] = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
        Mesh::from_polygons(&p, &[4; 6], &f.concat())
    }

    #[test]
    fn undoes_one_and_two_levels() {
        let c = cube();
        let o = SubdivideOptions { levels: 1, sharp_boundary: true, keep_hard_edges: false };
        let once = subdivide(&c, &o, None);
        let back = unsmooth(&once).expect("unsmooth");
        assert_eq!((back.num_points(), back.num_faces()), (8, 6));
        let mut ps: Vec<[i64; 3]> = (0..8).map(|p| (back.position(p) * 1e6).round().as_i64vec3().to_array()).collect();
        let mut want: Vec<[i64; 3]> = (0..8).map(|p| (c.position(p) * 1e6).round().as_i64vec3().to_array()).collect();
        ps.sort_unstable();
        want.sort_unstable();
        assert_eq!(ps, want);
        let twice = subdivide(&once, &o, None);
        let two = unsmooth(&unsmooth(&twice).unwrap()).unwrap();
        assert_eq!((two.num_points(), two.num_faces()), (8, 6));
        assert!(unsmooth(&c).is_none() || unsmooth(&c).unwrap().num_faces() < 6);
    }

    #[test]
    fn open_grid() {
        let p: Vec<DVec3> = (0..9).map(|i| DVec3::new((i % 3) as f64, (i / 3) as f64, ((i * 7) % 5) as f64 * 0.1)).collect();
        let m = Mesh::from_polygons(&p, &[4; 4], &[0, 1, 4, 3, 1, 2, 5, 4, 3, 4, 7, 6, 4, 5, 8, 7]);
        let o = SubdivideOptions { levels: 1, sharp_boundary: true, keep_hard_edges: false };
        let back = unsmooth(&subdivide(&m, &o, None)).expect("unsmooth");
        assert_eq!((back.num_points(), back.num_faces()), (9, 4));
        for q in 0..9 {
            let pos = back.position(q);
            assert!((0..9).any(|i| (p[i] - pos).length() < 1e-6), "{pos}");
        }
    }
}
