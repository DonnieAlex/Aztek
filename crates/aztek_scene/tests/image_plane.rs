//! Image planes: on a camera at its depth and gate, free in the scene, sequences, shown only through their camera,
//! saved and opened, detached.

use aztek_scene::image_plane;
use aztek_scene::Session;
use glam::DVec3;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session() -> Session {
    let mut s = Session::new();
    aztek_scene::cameras::make_defaults(&mut s.graph);
    s
}

fn cam_shape(s: &Session, name: &str) -> u64 {
    let id = s.graph.find(name).unwrap().unwrap();
    aztek_scene::cameras::shape_of(&s.graph, id).unwrap()
}

#[test]
fn on_a_camera_at_its_depth_and_gate() {
    let mut s = session();
    let made = run(&mut s, "imagePlane", json!({"camera": "persp", "fileName": "/plates/a.png"}));
    assert_eq!(made, json!(["imagePlane1", "imagePlaneShape1"]));
    assert_eq!(run(&mut s, "listRelatives", json!({"args": ["imagePlane1"], "parent": true})), json!(["persp"]));
    s.graph.evaluate();
    let persp = cam_shape(&s, "persp");
    let d = image_plane::draws(&s.graph, persp, 1.0);
    assert_eq!(d.len(), 1);
    let cw = s.graph.node(persp).unwrap().data().matrix(aztek_scene::transform::DagNode::WORLD_MATRIX);
    let local = cw.inverse().transform_point3(d[0].center);
    assert!((local - DVec3::new(0.0, 0.0, -100.0)).length() < 1e-6, "{local}");
    // 35 mm lens on a 36 mm gate: half the gate at 100 is 100 * 1.8 / 3.5 across.
    assert!((d[0].right.length() - 100.0 * 3.6 / 2.0 / 3.5).abs() < 1e-2, "{}", d[0].right.length());
    assert_eq!(run(&mut s, "imagePlane", json!({"args": ["imagePlane1"], "query": true, "queried": ["camera"]})), json!("persp"));
    assert_eq!(run(&mut s, "imagePlane", json!({"args": ["imagePlane1"], "query": true, "queried": ["fileName"]})), json!("/plates/a.png"));
    // Only through its camera, when asked.
    run(&mut s, "imagePlane", json!({"args": ["imagePlane1"], "edit": true, "showInAllViews": false}));
    let top = cam_shape(&s, "top");
    assert!(image_plane::draws(&s.graph, top, 1.0).is_empty());
    assert_eq!(image_plane::draws(&s.graph, persp, 1.0).len(), 1);
}

#[test]
fn free_planes_and_sequences() {
    let mut s = session();
    let made = run(&mut s, "imagePlane", json!({"fileName": "/plates/shot.0001.jpg", "width": 4.0, "height": 2.0, "name": "ref"}));
    let shape = made[1].as_str().unwrap().to_string();
    run(&mut s, "setAttr", json!({"args": ["ref.translateY", 3.0]}));
    s.graph.evaluate();
    let d = image_plane::draws(&s.graph, cam_shape(&s, "persp"), 1.0);
    assert_eq!(d.len(), 1);
    assert!((d[0].center - DVec3::new(0.0, 3.0, 0.0)).length() < 1e-9);
    assert!((d[0].right.length() - 2.0).abs() < 1e-9 && (d[0].up.length() - 1.0).abs() < 1e-9);
    run(&mut s, "setAttr", json!({"args": [format!("{shape}.useFrameExtension"), true]}));
    let id = s.graph.find(&shape).unwrap().unwrap();
    assert_eq!(image_plane::file_at(&s.graph, id, 12.0), "/plates/shot.0012.jpg");
    run(&mut s, "setAttr", json!({"args": [format!("{shape}.frameOffset"), 3]}));
    assert_eq!(image_plane::file_at(&s.graph, id, 12.0), "/plates/shot.0015.jpg");
}

#[test]
fn saved_opened_and_detached() {
    let mut s = session();
    run(&mut s, "imagePlane", json!({"camera": "persp", "fileName": "/plates/a.png"}));
    run(&mut s, "setAttr", json!({"args": ["imagePlaneShape1.depth", 50.0]}));
    let path = std::env::temp_dir().join(format!("image_plane_{}.ma", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true}));
    let mut t = session();
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    assert_eq!(run(&mut t, "imagePlane", json!({"args": ["imagePlane1"], "query": true, "queried": ["camera"]})), json!("persp"));
    assert_eq!(run(&mut t, "getAttr", json!({"args": ["imagePlaneShape1.depth"]})), json!(50.0));
    run(&mut t, "imagePlane", json!({"args": ["imagePlane1"], "detach": true}));
    assert_eq!(run(&mut t, "imagePlane", json!({"args": ["imagePlane1"], "query": true, "queried": ["camera"]})), json!(""));
    assert_eq!(run(&mut t, "getAttr", json!({"args": ["imagePlaneShape1.lockedToCamera"]})), json!(false));
}
