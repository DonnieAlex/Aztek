//! Tests: conform.

use aztek_geo::{Index, Mesh};
use aztek_ops::{execute, CallSource};
use aztek_scene::conform_commands::last_sphere;
use aztek_scene::last_op;
use aztek_scene::mesh_nodes::MeshNode;
use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn run_ui(s: &mut Session, name: &str, call: Value) -> Value {
    execute(s, name, &call, CallSource::Ui).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn rerun(s: &mut Session, flags: Value) {
    last_op::rerun(s, &flags, CallSource::Script, false).unwrap();
}

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-5 * (1.0 + a.abs().max(b.abs()))
}

fn approx_eps(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps * (1.0 + a.abs().max(b.abs()))
}

fn mesh_of(s: &mut Session, name: &str) -> Mesh {
    s.graph.evaluate();
    let id = s.graph.find(name).unwrap().unwrap();
    (**s.graph.node(aztek_scene::components::as_mesh(&s.graph, id).unwrap()).unwrap().data().mesh(MeshNode::OUT_MESH).unwrap()).clone()
}

fn first(v: &Value) -> String {
    v[0].as_str().unwrap().to_string()
}

/// Plane point id at column i, row j (rows run toward -Z).
fn grid_id(sx: u32, i: u32, j: u32) -> Index {
    j * (sx + 1) + i
}

fn spread(v: &[f64]) -> f64 {
    v.iter().copied().fold(f64::MIN, f64::max) - v.iter().copied().fold(f64::MAX, f64::min)
}

fn all_points(m: &Mesh) -> Vec<Index> {
    (0..m.point_capacity()).filter(|&p| m.point_valid(p)).collect()
}

#[test]
fn conform_commands_undoable_and_adjustable() {
    let mut s = Session::new();
    let plane = first(&run(&mut s, "polyPlane", json!({"subdivisionsX": 4, "subdivisionsZ": 2})));
    // Bend the first and last rows, then straighten both chains at once.
    run(&mut s, "selectComponents", json!({"args": [plane], "vertices": [grid_id(4, 2, 0), grid_id(4, 1, 2)]}));
    run(&mut s, "moveComponents", json!({"by": [0, 10, 0]}));
    let mut rows = Vec::new();
    for i in 0..=4 {
        rows.push(grid_id(4, i, 0));
        rows.push(grid_id(4, i, 2));
    }
    run(&mut s, "selectComponents", json!({"args": [plane], "vertices": rows}));
    run_ui(&mut s, "polyStraighten", json!({}));
    let m = mesh_of(&mut s, &plane);
    for &p in &rows {
        assert!(approx(m.position(p).y, 0.0));
    }
    assert!(approx(m.position(grid_id(4, 2, 0)).x, 0.0));
    assert!(last_op::live(&s));
    rerun(&mut s, json!({"strength": 0.5}));
    assert!(approx(mesh_of(&mut s, &plane).position(grid_id(4, 2, 0)).y, 5.0));
    run(&mut s, "undo", json!({}));
    assert!(approx(mesh_of(&mut s, &plane).position(grid_id(4, 2, 0)).y, 0.0));
    run(&mut s, "undo", json!({}));
    assert!(approx(mesh_of(&mut s, &plane).position(grid_id(4, 2, 0)).y, 10.0));

    // Circularize a face region, then square it, then adjust the radius live.
    run(&mut s, "newScene", json!({}));
    let plane = first(&run(&mut s, "polyPlane", json!({"subdivisionsX": 4, "subdivisionsZ": 4})));
    run(&mut s, "selectComponents", json!({"args": [plane], "faces": (0..16).collect::<Vec<_>>()}));
    run_ui(&mut s, "polyCircularize", json!({}));
    let radius_of = |s: &mut Session, p: Index| {
        let at = mesh_of(s, &plane).position(p);
        at.x.hypot(at.z)
    };
    assert!(approx(radius_of(&mut s, grid_id(4, 0, 0)), radius_of(&mut s, grid_id(4, 2, 0))));
    assert!(last_op::live(&s));
    rerun(&mut s, json!({"radius": 30.0}));
    assert!(approx(radius_of(&mut s, grid_id(4, 0, 0)), 30.0));
    assert!(approx(radius_of(&mut s, grid_id(4, 3, 4)), 30.0));
    assert!(radius_of(&mut s, grid_id(4, 1, 1)) < 30.0);
    run_ui(&mut s, "polyConformShape", json!({"shape": "square"}));
    run(&mut s, "undo", json!({}));
    assert!(approx(radius_of(&mut s, grid_id(4, 3, 4)), 30.0));

    run(&mut s, "newScene", json!({}));
    let cube = first(&run(&mut s, "polyCube", json!({"subdivisionsX": 2, "subdivisionsY": 2, "subdivisionsZ": 2})));
    run(&mut s, "selectComponents", json!({"args": [cube], "faces": [0]}));
    run(&mut s, "convertSelection", json!({"args": ["vertex"]}));
    run(&mut s, "selectAllComponents", json!({}));
    run_ui(&mut s, "polySpherize", json!({}));
    let m = mesh_of(&mut s, &cube);
    let radii: Vec<f64> = all_points(&m).iter().map(|&p| m.position(p).length()).collect();
    assert!(spread(&radii) < 1e-6);

    // Conform to object: the other selected mesh is the target.
    run(&mut s, "newScene", json!({}));
    let cyl = first(&run(&mut s, "polyCylinder", json!({"radius": 25, "height": 100, "subdivisionsAxis": 8, "caps": false})));
    let plane = first(&run(&mut s, "polyPlane", json!({"width": 60, "depth": 60, "subdivisionsX": 2, "subdivisionsZ": 2})));
    run(&mut s, "move", json!({"args": [plane], "to": [0, 48, 0]}));
    run(&mut s, "selectComponents", json!({"args": [plane], "faces": [0, 1, 2, 3]}));
    run_ui(&mut s, "polyConformToObject", json!({"target": cyl}));
    let data = run(&mut s, "meshData", json!({"args": [plane], "worldSpace": true}));
    let on_rim = data["points"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| {
            let (x, y, z) = (p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap());
            (y - 50.0).abs() < 1e-6 && (x.hypot(z) - 25.0).abs() < 1e-6
        })
        .count();
    assert_eq!(on_rim, 8);
    assert!(s.run("polyConformToObject", json!({"target": plane})).is_err());
}

#[test]
fn conform_commands_follow_symmetry() {
    let mut s = Session::new();
    let plane = first(&run(&mut s, "polyPlane", json!({"subdivisionsX": 4, "subdivisionsZ": 4})));
    run(&mut s, "setSymmetry", json!({"args": [plane], "axis": "x"}));
    // The +X half's middle 2x2 block of faces.
    run(&mut s, "selectComponents", json!({"args": [plane], "faces": [6, 7, 10, 11]}));
    run(&mut s, "polyCircularize", json!({"radius": 20.0}));
    let m = mesh_of(&mut s, &plane);
    for p in all_points(&m) {
        let a = m.position(p);
        let mirrored = glam::DVec3::new(-a.x, a.y, a.z);
        let best = all_points(&m).iter().map(|&q| (m.position(q) - mirrored).length()).fold(f64::MAX, f64::min);
        assert!(best < 1e-6, "point {p} has no mirror");
    }
}

#[test]
fn spherize_command_overrides_ellipsoid_result_and_undo() {
    let mut s = Session::new();
    let cube = first(&run(&mut s, "polyCube", json!({"subdivisionsX": 2, "subdivisionsY": 2, "subdivisionsZ": 2})));
    run(&mut s, "selectComponents", json!({"args": [cube], "faces": [0]}));
    run(&mut s, "convertSelection", json!({"args": ["vertex"]}));
    run(&mut s, "selectAllComponents", json!({}));
    let before = mesh_of(&mut s, &cube);
    let pts = all_points(&before);

    let result = run(&mut s, "polySpherize", json!({"center": [0, 0, 0], "radius": 30.0}));
    assert!(approx(result["radius"].as_f64().unwrap(), 30.0));
    let m = mesh_of(&mut s, &cube);
    for &p in &pts {
        assert!(approx_eps(m.position(p).length(), 30.0, 1e-9));
    }
    assert!(approx(last_sphere(&s.graph).radius, 30.0));
    run(&mut s, "undo", json!({}));
    let m = mesh_of(&mut s, &cube);
    for &p in &pts {
        assert_eq!(m.position(p), before.position(p));
    }

    run(&mut s, "polySpherize", json!({"center": [0, 0, 0], "radius": 30.0, "scale": [2, 1, 1]}));
    let m = mesh_of(&mut s, &cube);
    for &p in &pts {
        let q = m.position(p);
        assert!(approx_eps((q.x / 2.0).powi(2) + q.y * q.y + q.z * q.z, 900.0, 1e-9));
    }
    run(&mut s, "undo", json!({}));

    run(&mut s, "polySpherize", json!({"center": [0, 0, 0], "radius": 30.0, "amount": 0.0}));
    let m = mesh_of(&mut s, &cube);
    for &p in &pts {
        assert_eq!(m.position(p), before.position(p));
    }
    assert!(s.run("polySpherize", json!({"alongNormals": true, "radiusMode": "max"})).is_ok());
}

#[test]
fn spherize_in_object_mode_moves_the_whole_mesh() {
    let mut s = Session::new();
    let cube = first(&run(&mut s, "polyCube", json!({"subdivisionsX": 2, "subdivisionsY": 2, "subdivisionsZ": 2})));
    run(&mut s, "select", json!({"args": [cube]}));
    run(&mut s, "selectMode", json!({"args": ["object"]}));
    let before = mesh_of(&mut s, &cube);
    run(&mut s, "polySpherize", json!({"center": [0, 0, 0], "radius": 30.0}));
    let m = mesh_of(&mut s, &cube);
    for p in all_points(&m) {
        assert!(approx_eps(m.position(p).length(), 30.0, 1e-9));
    }
    run(&mut s, "undo", json!({}));
    let m = mesh_of(&mut s, &cube);
    for p in all_points(&before) {
        assert_eq!(m.position(p), before.position(p));
    }
    run(&mut s, "redo", json!({}));
    let first_point = all_points(&before)[0];
    assert!(approx_eps(mesh_of(&mut s, &cube).position(first_point).length(), 30.0, 1e-9));
}

/// Mesh > Conform as the reference's (#165): the selection onto the live surface along the points' normals or to the
/// closest point, an offset off it along its normal, and a clear message with nothing live.
#[test]
fn conform_onto_the_live_surface() {
    let mut s = aztek_scene::Session::new();
    let r = |s: &mut aztek_scene::Session, n: &str, v: serde_json::Value| s.run(n, v);
    r(&mut s, "polySphere", serde_json::json!({"name": "ball", "radius": 2.0, "subdivisionsAxis": 48, "subdivisionsHeight": 24})).unwrap();
    r(&mut s, "polyPlane", serde_json::json!({"name": "sheet", "width": 1.0, "height": 1.0, "subdivisionsX": 4, "subdivisionsY": 4})).unwrap();
    r(&mut s, "move", serde_json::json!({"args": [0.0, 3.0, 0.0, "sheet"], "absolute": true})).unwrap();
    // Nothing live: it says so.
    r(&mut s, "select", serde_json::json!({"args": ["sheet.vtx[*]"], "replace": true})).unwrap();
    let e = r(&mut s, "polyConformToLive", serde_json::json!({})).unwrap_err().to_string();
    assert!(e.contains("live surface"), "{e}");
    r(&mut s, "makeLive", serde_json::json!({"args": ["ball"]})).unwrap();
    let radii = |s: &mut aztek_scene::Session| -> Vec<f64> {
        let v = s.run("xform", serde_json::json!({"args": ["sheet.vtx[*]"], "query": true, "worldSpace": true, "translation": true})).unwrap();
        v.as_array().unwrap().chunks(3).map(|p| (p[0].as_f64().unwrap().powi(2) + p[1].as_f64().unwrap().powi(2) + p[2].as_f64().unwrap().powi(2)).sqrt()).collect()
    };
    for (projection, offset) in [("normals", 0.0), ("closest", 0.0), ("closest", 0.5)] {
        r(&mut s, "select", serde_json::json!({"args": ["sheet.vtx[*]"], "replace": true})).unwrap();
        r(&mut s, "polyConformToLive", serde_json::json!({"projection": projection, "surfaceOffset": offset})).unwrap();
        for d in radii(&mut s) {
            assert!((d - 2.0 - offset).abs() < 0.03, "{projection} {offset}: {d}");
        }
        r(&mut s, "undo", serde_json::json!({})).unwrap();
    }
}
