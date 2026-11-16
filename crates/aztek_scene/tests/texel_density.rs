//! texGetTexelDensity and texSetTexelDensity.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[test]
fn a_plane_reads_and_sets_its_density() {
    let mut s = Session::new();
    // A 2 x 2 plane with UVs over the unit square: 0.25 of the map per square unit, so 512 pixels a unit at 1024.
    run(&mut s, "polyPlane", json!({"name": "p", "width": 2.0, "height": 2.0, "subdivisionsX": 2, "subdivisionsY": 2}));
    let d = run(&mut s, "texGetTexelDensity", json!({"args": ["p"], "mapSize": 1024})).as_f64().unwrap();
    assert!((d - 512.0).abs() < 1e-6, "{d}");
    // Scaled up in the world, fewer pixels a unit.
    run(&mut s, "setAttr", json!({"args": ["p.scaleX", 2.0]}));
    run(&mut s, "setAttr", json!({"args": ["p.scaleZ", 2.0]}));
    let d = run(&mut s, "texGetTexelDensity", json!({"args": ["p"]})).as_f64().unwrap();
    assert!((d - 256.0).abs() < 1e-6, "{d}");
    run(&mut s, "texSetTexelDensity", json!({"args": ["p"], "texelDensity": 128.0, "mapSize": 1024}));
    let d = run(&mut s, "texGetTexelDensity", json!({"args": ["p"]})).as_f64().unwrap();
    assert!((d - 128.0).abs() < 1e-6, "{d}");
    // Faces alone.
    let f = run(&mut s, "texGetTexelDensity", json!({"args": ["p.f[0]"], "mapSize": 2048})).as_f64().unwrap();
    assert!((f - 256.0).abs() < 1e-6, "{f}");
    s.undo();
    let d = run(&mut s, "texGetTexelDensity", json!({"args": ["p"]})).as_f64().unwrap();
    assert!((d - 256.0).abs() < 1e-6, "{d}");
}
