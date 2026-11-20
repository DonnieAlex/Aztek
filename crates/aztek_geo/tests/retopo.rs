//! Retopology: what each method must deliver on shapes whose answer is known.

use aztek_geo::attribute::{names, Index};
use aztek_geo::mesh::Mesh;
use aztek_geo::primitives::*;
use aztek_geo::retopo::*;
use aztek_geo::symmetry::axis_plane;
use glam::DVec3;

fn all_faces(m: &Mesh) -> Vec<Index> {
    (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect()
}

fn sphere() -> Mesh {
    make_sphere(&SphereOptions { radius: 1.0, subdiv_axis: 48, subdiv_height: 32, ..Default::default() })
}

fn border_edges(m: &Mesh) -> usize {
    (0..m.edge_capacity()).filter(|&e| m.edge_valid(e) && m.is_boundary_edge(e)).count()
}

fn run(m: &Mesh, method: Method, faces: usize) -> Mesh {
    let mut o = RetopoOptions::for_method(method);
    o.target_faces = faces;
    retopologize(m, &all_faces(m), &o)
}

#[test]
fn a_sphere_becomes_closed_even_quads_near_the_count() {
    let src = sphere();
    let out = run(&src, Method::Organic, 600);
    let q = quality(&out, &src);
    assert_eq!(q.quads, 1.0, "all quads");
    assert_eq!(border_edges(&out), 0, "closed");
    assert!((q.faces as f64 - 600.0).abs() < 600.0 * 0.25, "{} faces", q.faces);
    assert!(q.irregular < 0.12, "irregular {}", q.irregular);
    assert!(q.scaled_jacobian > 0.95, "scaled jacobian {}", q.scaled_jacobian);
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            let r = out.position(p).length();
            assert!((r - 1.0).abs() < 0.02, "point {p} at radius {r}");
        }
    }
}

#[test]
fn hard_surface_keeps_a_boxs_corners_and_sides() {
    let src = make_cube(&CubeOptions { width: 2.0, height: 2.0, depth: 2.0, ..Default::default() });
    let out = run(&src, Method::HardSurface, 300);
    let q = quality(&out, &src);
    assert_eq!(q.quads, 1.0);
    assert_eq!(border_edges(&out), 0);
    // Every corner of the box is a point of the result.
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                let c = DVec3::new(x, y, z);
                let near = (0..out.point_capacity()).filter(|&p| out.point_valid(p)).map(|p| (out.position(p) - c).length()).fold(f64::MAX, f64::min);
                assert!(near < 1e-6, "corner {c} missed by {near}");
            }
        }
    }
    // Every point on the box, and every face flat on one side.
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            let v = out.position(p);
            let m = v.abs().max_element();
            assert!((m - 1.0).abs() < 1e-6, "point {v} off the box");
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
    let mut o = RetopoOptions::for_method(Method::Character);
    o.target_faces = 500;
    o.symmetry = Some(axis_plane(0, 0.0));
    let out = retopologize(&src, &all_faces(&src), &o);
    assert!(out.num_faces() > 200);
    assert_eq!(border_edges(&out), 0, "the halves are joined");
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
    let a = run(&src, Method::Character, 400);
    let b = run(&src, Method::Character, 400);
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
fn segmenting_by_colour_puts_a_loop_on_the_colour_border() {
    // A sphere red above y = 0.3 and blue below: the result has a ring of points on that line.
    let mut src = sphere();
    let n = src.corner_capacity() as usize;
    let mut cols = vec![[0f32; 4]; n];
    for c in 0..src.corner_capacity() {
        if !src.corner_valid(c) {
            continue;
        }
        let f = src.corner_face(c);
        let y = src.face_centroid(f).y;
        cols[c as usize] = if y > 0.3 { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 1.0, 1.0] };
    }
    src.corner_attrs_mut().get_or_add::<[f32; 4]>(names::COLOR).copy_from_slice(&cols);
    let mut o = RetopoOptions::for_method(Method::Segmented);
    o.target_faces = 500;
    let out = retopologize(&src, &all_faces(&src), &o);
    // The colour border runs along the source's edges nearest y = 0.3.
    let border_y = {
        let rings: Vec<f64> = (0..src.point_capacity()).filter(|&p| src.point_valid(p)).map(|p| src.position(p).y).collect();
        rings.into_iter().filter(|y| *y > 0.3).fold(f64::MAX, f64::min)
    };
    let on_line = (0..out.point_capacity()).filter(|&p| out.point_valid(p) && (out.position(p).y - border_y).abs() < 0.01).count();
    assert!(on_line >= 16, "{on_line} points on the colour border at y {border_y}");
}

#[test]
fn a_selection_is_retopologized_as_a_piece() {
    let src = sphere();
    let top: Vec<Index> = all_faces(&src).into_iter().filter(|&f| src.face_centroid(f).y > 0.0).collect();
    let mut o = RetopoOptions::default();
    o.target_faces = 200;
    let out = retopologize(&src, &top, &o);
    assert!(out.num_faces() > 100);
    for p in 0..out.point_capacity() {
        if out.point_valid(p) {
            assert!(out.position(p).y > -0.05);
        }
    }
}
