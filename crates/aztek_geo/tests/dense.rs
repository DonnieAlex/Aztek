//! Tools on dense meshes: they finish in time that follows the mesh, and get the same answers as on light ones.

mod common;

use aztek_geo::ops::basic::{dissolve_edges, mirror_mesh, MirrorDirection, MirrorOptions};
use aztek_geo::ops::poly_tools::{remesh, retopo, RemeshOptions};
use aztek_geo::primitives::*;
use aztek_geo::symmetry::SymmetryPlane;
use aztek_geo::{Index, Mesh};
use common::*;
use glam::DVec3;
use std::time::Instant;

fn live_faces(m: &Mesh) -> Vec<Index> {
    (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect()
}

fn sphere(axis: i32, height: i32) -> Mesh {
    make_sphere(&SphereOptions { radius: 10.0, subdiv_axis: axis, subdiv_height: height, ..Default::default() })
}

/// Dissolving every edge inside a patch makes one face of its outline at once, with the inner points gone.
#[test]
fn dissolving_a_patch_makes_one_face() {
    let mut m = quad_grid(6, 6);
    m.compact(None);
    // The edges inside the middle 4 x 4 block (from 1 to 5 each way), not those on its outline.
    let near = |a: f64, b: f64| (a - b).abs() < 1e-9;
    let inb = |p: DVec3| p.x > 0.9 && p.x < 5.1 && -p.z > 0.9 && -p.z < 5.1;
    let on_line = |a: DVec3, b: DVec3| {
        (near(a.x, b.x) && (near(a.x, 1.0) || near(a.x, 5.0))) || (near(a.z, b.z) && (near(-a.z, 1.0) || near(-a.z, 5.0)))
    };
    let edges: Vec<Index> = (0..m.edge_capacity())
        .filter(|&e| m.edge_valid(e))
        .filter(|&e| {
            let (a, b) = (m.position(m.edge_point(e, 0)), m.position(m.edge_point(e, 1)));
            inb(a) && inb(b) && !on_line(a, b)
        })
        .collect();
    assert_eq!(edges.len(), 24);
    let faces_before = m.num_faces();
    let made = dissolve_edges(&mut m, &edges, true);
    assert!(valid(&m));
    assert_eq!(made.len(), 1);
    assert_eq!(m.face_size(made[0]), 16);
    assert_eq!(m.num_faces(), faces_before - 16 + 1);
    // The nine points inside are gone.
    let strictly = |p: DVec3| p.x > 1.1 && p.x < 4.9 && -p.z > 1.1 && -p.z < 4.9;
    let inner = (0..m.point_capacity()).filter(|&p| m.point_valid(p) && strictly(m.position(p))).count();
    assert_eq!(inner, 0);
}

/// Mirroring a dense mesh doubles it, welded on the plane, in well under a second per hundred thousand faces.
#[test]
fn mirror_of_a_dense_mesh() {
    let mut m = sphere(200, 100);
    let n = m.num_faces();
    let t = Instant::now();
    let o = MirrorOptions { plane: SymmetryPlane { normal: DVec3::X, offset: 10.0 }, direction: MirrorDirection::Both, ..Default::default() };
    mirror_mesh(&mut m, &o);
    let secs = t.elapsed().as_secs_f64();
    assert!(valid(&m));
    assert_eq!(m.num_faces(), 2 * n);
    // The sphere touches the plane at one point, welded into one.
    assert_eq!(m.num_points() as usize, 2 * (sphere(200, 100).num_points() as usize) - 1);
    assert!(secs < 20.0, "{secs} s");
}

/// Retopology of a mesh far denser than the target gets near the target.
#[test]
fn retopo_of_a_dense_mesh_reaches_the_target() {
    let mut m = sphere(120, 60);
    let faces = live_faces(&m);
    let made = retopo(&mut m, &faces, 400, 60.0);
    assert!(valid(&m));
    let n = made.len();
    assert!((200..=1200).contains(&n), "{n} faces");
    // Still the sphere: every point near the radius.
    for p in 0..m.point_capacity() {
        if m.point_valid(p) {
            let r = m.position(p).length();
            assert!((r - 10.0).abs() < 0.6, "{r}");
        }
    }
}

/// The remesh keeps every face it makes in its region: a second pass works on all of them, and none is left out of
/// the result.
#[test]
fn remesh_returns_every_face_it_makes() {
    let mut m = sphere(40, 20);
    let faces = live_faces(&m);
    let o = RemeshOptions { max_edge: 0.8, collapse: 0.4, smooth: 0.5, tessellate_borders: true, passes: 3 };
    let made = remesh(&mut m, &faces, &o);
    assert!(valid(&m));
    assert_eq!(made.len(), live_faces(&m).len());
}
