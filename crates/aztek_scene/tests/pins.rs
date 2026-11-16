//! Constrain > UV Pin and Proximity Pin.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn pos(s: &mut Session, n: &str) -> [f64; 3] {
    let v = run(s, "xform", json!({"args": [n], "query": true, "worldSpace": true, "translation": true}));
    let a = v.as_array().unwrap();
    [a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()]
}

fn near(a: [f64; 3], b: [f64; 3]) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() < 1e-4)
}

fn setup(s: &mut Session) {
    run(s, "polyPlane", json!({"name": "p", "width": 4.0, "height": 4.0, "subdivisionsX": 4, "subdivisionsY": 4}));
    run(s, "spaceLocator", json!({"name": "a"}));
    run(s, "move", json!({"args": [0.5, 0.0, 0.5, "a"], "absolute": true}));
    run(s, "spaceLocator", json!({"name": "b"}));
    run(s, "move", json!({"args": [-1.0, 0.2, 1.0, "b"], "absolute": true}));
    run(s, "select", json!({"args": ["a", "b", "p"]}));
}

#[test]
fn uv_pin_keeps_objects_in_place_and_follows_the_mesh() {
    let mut s = Session::new();
    setup(&mut s);
    let (a0, b0) = (pos(&mut s, "a"), pos(&mut s, "b"));
    let node = run(&mut s, "UVPin", json!({})).as_str().unwrap().to_string();
    assert_eq!(run(&mut s, "nodeType", json!({"args": [node.clone()]})), json!("uvPin"));
    assert!(near(pos(&mut s, "a"), a0), "{:?}", pos(&mut s, "a"));
    assert!(near(pos(&mut s, "b"), b0), "{:?}", pos(&mut s, "b"));
    // The mesh moves; the pinned objects go with it.
    run(&mut s, "move", json!({"args": [0.0, 3.0, 0.0, "p"], "relative": true}));
    let a1 = pos(&mut s, "a");
    assert!(near(a1, [a0[0], a0[1] + 3.0, a0[2]]), "{a1:?}");
    let b1 = pos(&mut s, "b");
    assert!(near(b1, [b0[0], b0[1] + 3.0, b0[2]]), "{b1:?}");
}

#[test]
fn proximity_pin_keeps_objects_in_place_and_follows_the_mesh() {
    let mut s = Session::new();
    setup(&mut s);
    let (a0, b0) = (pos(&mut s, "a"), pos(&mut s, "b"));
    let node = run(&mut s, "ProximityPin", json!({})).as_str().unwrap().to_string();
    assert_eq!(run(&mut s, "nodeType", json!({"args": [node]})), json!("proximityPin"));
    assert!(near(pos(&mut s, "a"), a0), "{:?}", pos(&mut s, "a"));
    assert!(near(pos(&mut s, "b"), b0), "{:?}", pos(&mut s, "b"));
    run(&mut s, "move", json!({"args": [1.0, 2.0, 0.0, "p"], "relative": true}));
    let a1 = pos(&mut s, "a");
    assert!(near(a1, [a0[0] + 1.0, a0[1] + 2.0, a0[2]]), "{a1:?}");
    // Lifting the middle rows, where b sits, moves b too.
    run(&mut s, "move", json!({"args": [0.0, 1.0, 0.0, "p.vtx[5:19]"], "relative": true}));
    let b2 = pos(&mut s, "b");
    assert!(b2[1] > b0[1] + 2.0 + 0.1, "{b2:?}");
}

#[test]
fn pins_need_a_mesh_last() {
    let mut s = Session::new();
    setup(&mut s);
    run(&mut s, "select", json!({"args": ["p", "a"]}));
    assert!(s.run("UVPin", json!({})).is_err());
    run(&mut s, "select", json!({"args": ["p"]}));
    assert!(s.run("ProximityPin", json!({})).is_err());
}

#[test]
fn a_pin_is_one_undo_and_survives_saving() {
    let mut s = Session::new();
    setup(&mut s);
    let b0 = pos(&mut s, "b");
    run(&mut s, "UVPin", json!({}));
    let dir = std::env::temp_dir().join(format!("aztek-pins-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("pins.ma").to_string_lossy().replace('\\', "/");
    run(&mut s, "file", json!({"rename": f.clone()}));
    run(&mut s, "file", json!({"save": true, "type": "mayaAscii", "force": true}));
    run(&mut s, "file", json!({"args": [f], "open": true, "force": true}));
    run(&mut s, "move", json!({"args": [0.0, 1.0, 0.0, "p"], "relative": true}));
    let b1 = pos(&mut s, "b");
    assert!(near(b1, [b0[0], b0[1] + 1.0, b0[2]]), "{b1:?}");
    let _ = std::fs::remove_dir_all(&dir);

    let mut s = Session::new();
    setup(&mut s);
    run(&mut s, "ProximityPin", json!({}));
    s.undo();
    assert_eq!(run(&mut s, "ls", json!({"type": "proximityPin"})), json!([]));
    assert_eq!(run(&mut s, "ls", json!({"args": ["pPlane1PinOrig", "pPinOrig", "pPlanePinOrig"]})), json!([]));
}
