//! Modify > Bake Pivot: the pivot's place and axes become the object's, nothing moving in the world.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn v3(v: &Value) -> [f64; 3] {
    let a = v.as_array().unwrap_or_else(|| panic!("{v}"));
    let a = if a.first().is_some_and(Value::is_array) { a[0].as_array().unwrap() } else { a };
    [a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()]
}

fn near(a: [f64; 3], b: [f64; 3]) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() < 1e-6)
}

#[test]
fn bake_pivot_moves_the_transform_not_the_geometry() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"name": "box"}));
    run(&mut s, "group", json!({"args": ["box"], "name": "g"}));
    run(&mut s, "spaceLocator", json!({"name": "kid"}));
    run(&mut s, "parent", json!({"args": ["kid", "box"]}));
    run(&mut s, "move", json!({"args": [3.0, 0.0, 0.0, "kid"], "absolute": true}));
    run(&mut s, "move", json!({"args": [1.0, 2.0, 0.0, "box"], "relative": true}));
    run(&mut s, "xform", json!({"args": ["box"], "worldSpace": true, "pivots": [1.5, 2.5, 0.5]}));
    run(&mut s, "setAttr", json!({"args": ["box.pivotOrient", 0.0, 0.0, 45.0], "type": "double3"}));
    let corner = |s: &mut Session| v3(&run(s, "xform", json!({"args": ["box.vtx[7]"], "query": true, "worldSpace": true, "translation": true})));
    let kid = |s: &mut Session| v3(&run(s, "xform", json!({"args": ["kid"], "query": true, "worldSpace": true, "translation": true})));
    let (c0, k0) = (corner(&mut s), kid(&mut s));
    run(&mut s, "bakeCustomToolPivot", json!({"args": ["box"]}));
    // The object sits at the pivot, turned 45 about Z; the points and the child are where they were.
    assert!(near(v3(&run(&mut s, "xform", json!({"args": ["box"], "query": true, "worldSpace": true, "translation": true}))), [1.5, 2.5, 0.5]));
    let rz = run(&mut s, "getAttr", json!({"args": ["box.rz"]})).as_f64().unwrap();
    assert!((rz - 45.0).abs() < 1e-6, "{rz}");
    assert!(near(v3(&run(&mut s, "getAttr", json!({"args": ["box.rotatePivot"]}))), [0.0; 3]));
    assert!(near(v3(&run(&mut s, "getAttr", json!({"args": ["box.pivotOrient"]}))), [0.0; 3]));
    assert!(near(corner(&mut s), c0), "{:?} {:?}", corner(&mut s), c0);
    assert!(near(kid(&mut s), k0));
    // One undo.
    s.undo();
    assert!(near(v3(&run(&mut s, "getAttr", json!({"args": ["box.rotatePivot"]}))), [0.5, 0.5, 0.5]));
    // Orientation only: the pivot stays where it is, the object turns to its axes.
    run(&mut s, "bakeCustomToolPivot", json!({"args": ["box"], "position": false, "orientation": true}));
    assert!(near(v3(&run(&mut s, "xform", json!({"args": ["box"], "query": true, "worldSpace": true, "rotatePivot": true}))), [1.5, 2.5, 0.5]));
    let rz = run(&mut s, "getAttr", json!({"args": ["box.rz"]})).as_f64().unwrap();
    assert!((rz - 45.0).abs() < 1e-6, "{rz}");
    assert!(near(corner(&mut s), c0));
}

/// Snap Pivot Orientation: the pivot's axes to the face, edge or vertex it sits on.
#[test]
fn pivot_orients_to_the_surface_under_it() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"name": "box"}));
    let z_axis = |s: &mut Session| -> [f64; 3] {
        // The pivot frame's Z, from pivotOrient (XYZ degrees) on an unrotated object.
        let o = v3(&run(s, "getAttr", json!({"args": ["box.pivotOrient"]})));
        let m = glam::DMat3::from_rotation_z(o[2].to_radians()) * glam::DMat3::from_rotation_y(o[1].to_radians()) * glam::DMat3::from_rotation_x(o[0].to_radians());
        let z = m * glam::DVec3::Z;
        [z.x, z.y, z.z]
    };
    let at = |s: &mut Session, p: [f64; 3]| {
        run(s, "setPivot", json!({"args": ["box"], "position": p}));
        run(s, "setPivot", json!({"args": ["box"], "orientToSurface": true}));
    };
    at(&mut s, [0.0, 0.5, 0.0]);
    assert!(near(z_axis(&mut s), [0.0, 1.0, 0.0]), "{:?}", z_axis(&mut s));
    at(&mut s, [0.5, 0.5, 0.0]);
    let h = std::f64::consts::FRAC_1_SQRT_2;
    assert!(near(z_axis(&mut s), [h, h, 0.0]), "{:?}", z_axis(&mut s));
    at(&mut s, [0.5, 0.5, 0.5]);
    let t = 1.0 / 3f64.sqrt();
    assert!(near(z_axis(&mut s), [t, t, t]), "{:?}", z_axis(&mut s));
    // Off the surface: left as it was.
    at(&mut s, [3.0, 3.0, 3.0]);
    assert!(near(z_axis(&mut s), [t, t, t]));
}
