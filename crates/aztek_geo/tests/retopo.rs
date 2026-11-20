//! Retopology: what it must deliver on shapes whose answer is known.

use aztek_geo::attribute::Index;
use aztek_geo::mesh::Mesh;
use aztek_geo::primitives::*;
use aztek_geo::retopo::*;
use aztek_geo::symmetry::axis_plane;
use glam::DVec3;
use std::collections::HashMap;

fn all_faces(m: &Mesh) -> Vec<Index> {
    (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect()
}

fn sphere() -> Mesh {
    make_sphere(&SphereOptions { radius: 1.0, subdiv_axis: 48, subdiv_height: 32, ..Default::default() })
}

fn run(m: &Mesh, faces: usize) -> Mesh {
    let o = RetopoOptions { target_faces: faces, ..Default::default() };
    retopologize(m, &all_faces(m), &o)
}

/// Every edge between exactly two faces, each running it once each way, and every face a quad.
fn assert_closed_quads(m: &Mesh) {
    let mut uses: HashMap<(Index, Index), usize> = HashMap::new();
    for f in all_faces(m) {
        let p = m.face_points(f);
        assert_eq!(p.len(), 4, "face {f} has {} sides", p.len());
        for i in 0..4 {
            *uses.entry((p[i], p[(i + 1) % 4])).or_default() += 1;
        }
    }
    for (&(a, b), &n) in &uses {
        assert_eq!(n, 1, "edge {a}-{b} runs the same way {n} times");
        assert_eq!(uses.get(&(b, a)), Some(&1), "edge {a}-{b} has one face");
    }
}

fn euler(m: &Mesh) -> i64 {
    let mut edges = std::collections::HashSet::new();
    let mut pts = std::collections::HashSet::new();
    for f in all_faces(m) {
        let p = m.face_points(f);
        for i in 0..p.len() {
            pts.insert(p[i]);
            let (a, b) = (p[i], p[(i + 1) % p.len()]);
            edges.insert((a.min(b), a.max(b)));
        }
    }
    pts.len() as i64 - edges.len() as i64 + m.num_faces() as i64
}

#[test]
fn a_sphere_becomes_closed_even_quads_near_the_count() {
    let src = sphere();
    let out = run(&src, 600);
    assert_closed_quads(&out);
    assert_eq!(euler(&out), 2);
    let q = quality(&out, &src);
    assert!((q.faces as f64 - 600.0).abs() < 600.0 * 0.1, "{} faces", q.faces);
    assert!(q.irregular < 0.06, "irregular {}", q.irregular);
    assert!(q.scaled_jacobian > 0.95, "scaled jacobian {}", q.scaled_jacobian);
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            let r = out.position(p).length();
            assert!((r - 1.0).abs() < 0.01, "point {p} at radius {r}");
        }
    }
}

#[test]
fn a_thin_torus_keeps_its_hole_and_stays_closed() {
    let src = make_torus(&TorusOptions { radius: 2.0, section_radius: 0.15, subdiv_axis: 120, subdiv_height: 16, ..Default::default() });
    for faces in [200, 800, 3000] {
        let out = run(&src, faces);
        assert_closed_quads(&out);
        assert_eq!(euler(&out), 0, "at {faces} faces");
        // Every point on the tube.
        for p in 0..out.point_capacity() {
            if out.point_valid(p) {
                let v = out.position(p);
                let ring = DVec3::new(v.x, 0.0, v.z).normalize() * 2.0;
                let d = (v - ring).length();
                assert!((d - 0.15).abs() < 0.03, "point {v} {d} from the tube's middle at {faces} faces");
            }
        }
    }
}

#[test]
fn hard_edges_keep_a_boxs_corners_and_sides() {
    let src = make_cube(&CubeOptions { width: 2.0, height: 2.0, depth: 2.0, ..Default::default() });
    let o = RetopoOptions { target_faces: 300, edges_by_angle: true, ..Default::default() };
    let out = retopologize(&src, &all_faces(&src), &o);
    assert_closed_quads(&out);
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                let c = DVec3::new(x, y, z);
                let near = (0..out.point_capacity()).filter(|&p| out.point_valid(p)).map(|p| (out.position(p) - c).length()).fold(f64::MAX, f64::min);
                assert!(near < 1e-6, "corner {c} missed by {near}");
            }
        }
    }
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            let v = out.position(p);
            assert!((v.abs().max_element() - 1.0).abs() < 1e-6, "point {v} off the box");
        }
    }
    for f in all_faces(&out) {
        let n = out.face_normal(f);
        assert!(n.abs().max_element() > 0.999, "face {f} not on one side: {n}");
    }
}

#[test]
fn symmetry_mirrors_every_point() {
    let src = make_torus(&TorusOptions { radius: 2.0, section_radius: 0.6, subdiv_axis: 60, subdiv_height: 24, ..Default::default() });
    let o = RetopoOptions { target_faces: 500, symmetry: Some(axis_plane(0, 0.0)), ..Default::default() };
    let out = retopologize(&src, &all_faces(&src), &o);
    assert!(out.num_faces() > 400);
    assert_closed_quads(&out);
    let pts: Vec<DVec3> = (0..out.point_capacity()).filter(|&p| out.point_valid(p)).map(|p| out.position(p)).collect();
    for v in &pts {
        let m = DVec3::new(-v.x, v.y, v.z);
        let near = pts.iter().map(|w| (*w - m).length()).fold(f64::MAX, f64::min);
        assert!(near < 1e-6, "{v} has no mirror ({near})");
    }
}

#[test]
fn the_same_input_gives_the_same_result() {
    let src = sphere();
    let a = run(&src, 400);
    let b = run(&src, 400);
    assert_eq!(a.num_faces(), b.num_faces());
    assert_eq!(a.positions(), b.positions());
}

#[test]
fn with_no_count_the_density_follows_the_deviation() {
    let src = sphere();
    let faces = all_faces(&src);
    let count = |ted: f64| {
        let o = RetopoOptions { target_faces: 0, target_edge_deviation: ted, ..RetopoOptions::default() };
        retopologize(&src, &faces, &o).num_faces()
    };
    let (fine, coarse) = (count(0.2), count(0.5));
    assert!(fine > 0 && coarse > 0);
    assert!(fine > coarse * 3, "{fine} against {coarse}");
}

#[test]
fn a_selection_is_retopologized_as_a_piece_with_its_border_kept() {
    let src = sphere();
    let top: Vec<Index> = all_faces(&src).into_iter().filter(|&f| src.face_centroid(f).y > 0.0).collect();
    let o = RetopoOptions { target_faces: 200, ..Default::default() };
    let out = retopologize(&src, &top, &o);
    assert!(out.num_faces() > 150);
    let border: Vec<Index> = (0..out.point_capacity()).filter(|&p| out.point_valid(p) && out.is_boundary_point(p)).collect();
    assert!(border.len() >= 12);
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            assert!(out.position(p).y > -1e-6);
        }
    }
    // The border stays on the sphere's equator.
    for p in border {
        assert!(out.position(p).y.abs() < 1e-6, "border point at {}", out.position(p));
    }
}
