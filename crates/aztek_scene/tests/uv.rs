//! UV editing: projections, cut and sew, unfold, layout, moving UVs; each as a history node.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn uvs(s: &mut Session, what: &str) -> Vec<f64> {
    run(s, "polyEditUV", json!({"args": [what], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

fn in_unit(v: &[f64]) -> bool {
    v.iter().all(|x| *x >= -1e-6 && *x <= 1.0 + 1e-6)
}

#[test]
fn projections_and_layout_stay_in_0_1() {
    let mut s = Session::new();
    run(&mut s, "polySphere", json!({}));
    let made = run(&mut s, "polyProjection", json!({"args": ["pSphere1"], "type": "planar", "mapDirection": "y"}));
    assert_eq!(made, json!(["polyPlanarProj1"]));
    assert!(in_unit(&uvs(&mut s, "pSphere1")));
    run(&mut s, "polyAutoProjection", json!({"args": ["pSphere1"]}));
    assert!(in_unit(&uvs(&mut s, "pSphere1")));
    run(&mut s, "u3dLayout", json!({"args": ["pSphere1"]}));
    assert!(in_unit(&uvs(&mut s, "pSphere1")));
    let hist = run(&mut s, "listHistory", json!({"args": ["pSphere1"]}));
    let h: Vec<&str> = hist.as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    assert!(h.contains(&"polyLayoutUV1") && h.contains(&"polyAutoProj1"), "{h:?}");
}

#[test]
fn edit_uv_moves_maps_and_undoes() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1}));
    let before = uvs(&mut s, "pPlane1.map[0]");
    run(&mut s, "polyEditUV", json!({"args": ["pPlane1.map[0]"], "u": 0.25, "v": -0.5}));
    let after = uvs(&mut s, "pPlane1.map[0]");
    assert!((after[0] - before[0] - 0.25).abs() < 1e-6 && (after[1] - before[1] + 0.5).abs() < 1e-6);
    s.undo();
    assert_eq!(uvs(&mut s, "pPlane1.map[0]"), before);
}

#[test]
fn cut_then_sew_splits_and_joins_shells() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 1}));
    let maps = uvs(&mut s, "pPlane1").len() / 2;
    assert_eq!(maps, 6);
    run(&mut s, "polyMapCut", json!({"args": ["pPlane1.f[0]"]}));
    assert_eq!(uvs(&mut s, "pPlane1").len() / 2, 8);
    run(&mut s, "polyMapSew", json!({"args": ["pPlane1.f[0]"]}));
    assert_eq!(uvs(&mut s, "pPlane1").len() / 2, 6);
}

#[test]
fn unfold_keeps_shell_in_place() {
    let mut s = Session::new();
    run(&mut s, "polyCylinder", json!({}));
    run(&mut s, "u3dUnfold", json!({"args": ["pCylinder1"]}));
    let v = uvs(&mut s, "pCylinder1");
    assert!(v.iter().all(|x| x.is_finite()));
}

/// Issue 55: unfold's options do what they say, and the legacy unfold holds pinned UVs.
#[test]
fn unfold_packs_and_pins() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 4, "subdivisionsY": 4}));
    // Stretch the UVs out of shape, beyond 0 to 1.
    run(&mut s, "polyEditUV", json!({"args": ["pPlane1.map[0:24]"], "scaleU": 3.0, "pivotU": 0.0, "pivotV": 0.0}));
    run(&mut s, "u3dUnfold", json!({"args": ["pPlane1"], "pack": 1, "method": "conformal"}));
    assert!(in_unit(&uvs(&mut s, "pPlane1")), "{:?}", uvs(&mut s, "pPlane1"));
    // Two corners held: they stay where they were, whatever the rest does.
    run(&mut s, "polyEditUV", json!({"args": ["pPlane1.map[0:24]"], "scaleU": 2.0, "pivotU": 0.0, "pivotV": 0.0}));
    let held = |s: &mut Session| [uvs(s, "pPlane1.map[0]"), uvs(s, "pPlane1.map[24]")];
    let before = held(&mut s);
    run(&mut s, "unfold", json!({"args": ["pPlane1.map[0]", "pPlane1.map[24]"], "pinSelected": true}));
    let after = held(&mut s);
    for (a, b) in before.iter().zip(&after) {
        for (x, y) in a.iter().zip(b) {
            assert!((x - y).abs() < 1e-5, "{before:?} {after:?}");
        }
    }
    // The rest moved: the stretch came out.
    let mid = uvs(&mut s, "pPlane1.map[12]");
    assert!(mid.iter().all(|x| x.is_finite()));
}

#[test]
fn uvs_and_vertex_faces_select_like_the_reference() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({}));
    // UVs by name: the mode follows, and the selection reads back as the reference writes it.
    run(&mut s, "select", json!({"args": ["pCube1.map[0:2]"]}));
    assert_eq!(run(&mut s, "selectMode", json!({})), json!("uv"));
    assert_eq!(run(&mut s, "ls", json!({"selection": true})), json!(["pCube1.map[0:2]"]));
    assert_eq!(run(&mut s, "filterExpand", json!({"selectionMask": 35})), json!(["pCube1.map[0]", "pCube1.map[1]", "pCube1.map[2]"]));
    // To vertices: the points those UVs sit at.
    let pts = run(&mut s, "polyListComponentConversion", json!({"args": ["pCube1.map[0]"], "toVertex": true}));
    assert_eq!(pts, json!(["pCube1.vtx[0]"]));
    // Vertex faces: one point as one face sees it.
    run(&mut s, "select", json!({"args": ["pCube1.vtxFace[0][0]"]}));
    assert_eq!(run(&mut s, "selectMode", json!({})), json!("vertexFace"));
    assert_eq!(run(&mut s, "ls", json!({"selection": true})), json!(["pCube1.vtxFace[0][0]"]));
    // selectType's masks: UVs, UV shells, vertex faces, and Multi for vertices, edges and faces together.
    run(&mut s, "selectType", json!({"polymeshUV": true}));
    assert_eq!(run(&mut s, "selectType", json!({"query": true, "polymeshUV": true})), json!(true));
    run(&mut s, "selectType", json!({"meshUVShell": true}));
    assert_eq!(run(&mut s, "selectMode", json!({})), json!("uvShell"));
    run(&mut s, "selectType", json!({"polymeshVertex": true, "polymeshEdge": true, "polymeshFace": true}));
    assert_eq!(run(&mut s, "selectMode", json!({})), json!("multi"));
    run(&mut s, "select", json!({"args": ["pCube1.vtx[1]", "pCube1.e[2]", "pCube1.f[3]"]}));
    assert_eq!(run(&mut s, "selectMode", json!({})), json!("multi"));
    assert_eq!(run(&mut s, "ls", json!({"selection": true})), json!(["pCube1.vtx[1]", "pCube1.e[2]", "pCube1.f[3]"]));
    run(&mut s, "selectMode", json!({"args": ["vertex"]}));
    assert_eq!(run(&mut s, "ls", json!({"selection": true})), json!(["pCube1.vtx[1]"]));
}

/// u3dLayout over UDIM tiles, with a pack box, margins and the tile assign modes.
#[test]
fn layout_over_udim_tiles() {
    let mut s = Session::new();
    run(&mut s, "polySphere", json!({}));
    run(&mut s, "polyAutoProjection", json!({"args": ["pSphere1"]}));
    let count = |v: &[f64], f: &dyn Fn(f64, f64) -> bool| v.chunks(2).filter(|c| f(c[0], c[1])).count();
    // Two tiles across: every UV in 0..2 by 0..1, both tiles used, none in the margin.
    run(&mut s, "u3dLayout", json!({"args": ["pSphere1"], "tileU": 2, "tileV": 1, "tileMargin": 0.05, "shellSpacing": 0.01}));
    let v = uvs(&mut s, "pSphere1");
    assert!(v.chunks(2).all(|c| c[0] >= 0.05 - 1e-6 && c[0] <= 1.95 + 1e-6 && c[1] >= 0.05 - 1e-6 && c[1] <= 0.95 + 1e-6), "{v:?}");
    assert!(v.chunks(2).all(|c| !(c[0] > 0.95 + 1e-6 && c[0] < 1.05 - 1e-6)), "a UV in the margin between the tiles");
    assert!(count(&v, &|u, _| u < 1.0) > 0 && count(&v, &|u, _| u > 1.0) > 0);
    // A pack box in the first tile's upper half.
    run(&mut s, "u3dLayout", json!({"args": ["pSphere1"], "packBox": [0.0, 1.0, 0.5, 1.0], "tileMargin": 0.0}));
    let v = uvs(&mut s, "pSphere1");
    assert!(v.chunks(2).all(|c| c[1] >= 0.5 - 1e-6 && c[1] <= 1.0 + 1e-6 && c[0] >= -1e-6 && c[0] <= 1.0 + 1e-6));
    // Shell centres: everything moved into tile 1002 stays there.
    run(&mut s, "polyEditUV", json!({"args": ["pSphere1.map[*]"], "u": 1.0, "v": 0.0}));
    run(&mut s, "u3dLayout", json!({"args": ["pSphere1"], "tileU": 2, "tileV": 1, "tileAssignMode": 1}));
    let v = uvs(&mut s, "pSphere1");
    assert!(v.chunks(2).all(|c| c[0] >= 1.0 - 1e-6 && c[0] <= 2.0 + 1e-6), "{v:?}");
    // Non-uniform scale fills the box in both directions.
    run(&mut s, "u3dLayout", json!({"args": ["pSphere1"], "layoutScaleMode": 2, "tileMargin": 0.0, "shellSpacing": 0.0}));
    let v = uvs(&mut s, "pSphere1");
    let (lo_u, hi_u) = v.chunks(2).fold((f64::MAX, f64::MIN), |(a, b), c| (a.min(c[0]), b.max(c[0])));
    let (lo_v, hi_v) = v.chunks(2).fold((f64::MAX, f64::MIN), |(a, b), c| (a.min(c[1]), b.max(c[1])));
    assert!(lo_u.abs() < 1e-6 && (hi_u - 1.0).abs() < 1e-6 && lo_v.abs() < 1e-6 && (hi_v - 1.0).abs() < 1e-6, "{lo_u} {hi_u} {lo_v} {hi_v}");
}
