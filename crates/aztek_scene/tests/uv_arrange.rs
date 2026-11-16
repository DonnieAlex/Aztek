//! The UV Toolkit's shell arranging (#108): orient, stack, unstack, gather, distribute, align, snap, randomize, linear align.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// A mesh's UV bounds: (umin, vmin, umax, vmax).
fn bounds(s: &mut Session, mesh: &str) -> [f64; 4] {
    let v: Vec<f64> = run(s, "polyEditUV", json!({"args": [format!("{mesh}.map[*]")], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let mut b = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
    for p in v.chunks(2) {
        b[0] = b[0].min(p[0]);
        b[1] = b[1].min(p[1]);
        b[2] = b[2].max(p[0]);
        b[3] = b[3].max(p[1]);
    }
    b
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-4
}

/// Two one-face planes, each one shell of 0 to 1; b's shrunk to half and moved off.
fn two(s: &mut Session) {
    run(s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1, "name": "a"}));
    run(s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1, "name": "b"}));
    run(s, "polyEditUV", json!({"args": ["b.map[*]"], "scaleU": 0.5, "scaleV": 0.5, "pivotU": 0.0, "pivotV": 0.0}));
    run(s, "polyEditUV", json!({"args": ["b.map[*]"], "u": 2.3, "v": -1.6}));
}

#[test]
fn gather_stack_distribute_align_snap() {
    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "texGatherShells", json!({"args": ["b.map[*]"]}));
    let b = bounds(&mut s, "b");
    assert!(b[0] >= 0.0 && b[2] <= 1.0 && b[1] >= 0.0 && b[3] <= 1.0, "{b:?}");

    run(&mut s, "texStackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    assert!(close((a[0] + a[2]) / 2.0, (b[0] + b[2]) / 2.0) && close((a[1] + a[3]) / 2.0, (b[1] + b[3]) / 2.0), "{a:?} {b:?}");

    run(&mut s, "texDistributeShells", json!({"args": ["a.map[*]", "b.map[*]"], "direction": "u", "spacing": 0.1}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    let (first, second) = if a[0] < b[0] { (a, b) } else { (b, a) };
    assert!(close(second[0], first[2] + 0.1), "{a:?} {b:?}");

    run(&mut s, "texAlignShells", json!({"args": ["a.map[*]", "b.map[*]"], "alignTo": "top"}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    assert!(close(a[3], b[3]), "{a:?} {b:?}");

    run(&mut s, "texSnapShells", json!({"args": ["b.map[*]"], "position": 8}));
    let b = bounds(&mut s, "b");
    assert!(close(b[2], 1.0) && close(b[1], 0.0), "{b:?}");
    run(&mut s, "undo", json!({}));
    let b2 = bounds(&mut s, "b");
    assert!(!close(b2[2], 1.0) || !close(b2[1], 0.0), "{b2:?}");
}

#[test]
fn orient_and_unstack() {
    let mut s = Session::new();
    two(&mut s);
    // Turned 30 degrees, a square shell's box grows; oriented, it is square to U and V again.
    run(&mut s, "polyEditUV", json!({"args": ["a.map[*]"], "angle": 30.0, "pivotU": 0.5, "pivotV": 0.5}));
    let t = bounds(&mut s, "a");
    assert!(t[2] - t[0] > 1.2);
    run(&mut s, "texOrientShells", json!({"args": ["a.map[*]"]}));
    let o = bounds(&mut s, "a");
    assert!(close(o[2] - o[0], 1.0) && close(o[3] - o[1], 1.0), "{o:?}");

    // Stacked, then unstacked: they no longer overlap.
    run(&mut s, "texStackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    run(&mut s, "texUnstackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    let overlap = a[0] < b[2] && b[0] < a[2] && a[1] < b[3] && b[1] < a[3];
    assert!(!overlap, "{a:?} {b:?}");
}

#[test]
fn orient_to_edge_randomize_linear_align() {
    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "polyEditUV", json!({"args": ["a.map[*]"], "angle": 20.0, "pivotU": 0.5, "pivotV": 0.5}));
    run(&mut s, "texOrientEdge", json!({"args": ["a.e[0]"]}));
    let o = bounds(&mut s, "a");
    assert!(close(o[2] - o[0], 1.0), "{o:?}");

    let before = bounds(&mut s, "b");
    run(&mut s, "texRandomizeShells", json!({"args": ["b.map[*]"], "translate": 0.5, "rotate": 30.0, "seed": 7}));
    let after = bounds(&mut s, "b");
    assert!(before != after);

    // Three UVs of a corner onto one line.
    run(&mut s, "texLinearAlignUVs", json!({"args": ["a.map[0:2]"]}));
    let p: Vec<f64> = run(&mut s, "polyEditUV", json!({"args": ["a.map[0:2]"], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let (x0, y0, x1, y1, x2, y2) = (p[0], p[1], p[2], p[3], p[4], p[5]);
    let cross = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
    assert!(cross.abs() < 1e-5, "{p:?}");
}
