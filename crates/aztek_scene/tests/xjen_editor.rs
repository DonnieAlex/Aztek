//! XJEN step 5: what the Interactive Groom Editor reads, converting to polygons, presets and transfer.

use aztek_scene::xjen;
use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session() -> Session {
    let mut s = Session::new();
    xjen::cmds::register(&mut s.commands);
    run(&mut s, "polyPlane", json!({"name": "skin", "width": 2.0, "height": 2.0, "subdivisionsX": 2, "subdivisionsY": 2}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin"], "density": 3, "length": 1.0, "seed": 4, "name": "fur"}));
    s
}

fn splines(s: &mut Session, d: &str) -> Vec<xjen::Spline> {
    s.graph.evaluate();
    let id = xjen::cmds::description(s, d).unwrap();
    xjen::render_splines(&s.graph, id).unwrap().splines.clone()
}

#[test]
fn info_lists_descriptions_stacks_and_layers() {
    let mut s = session();
    run(&mut s, "xjenAddModifier", json!({"args": ["fur"], "type": "clump"}));
    run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "add": "Detail"}));
    let info = run(&mut s, "xjenGroomInfo", json!({}));
    let d = &info[0];
    assert_eq!(d["transform"], json!("fur"));
    assert_eq!(d["splines"], json!(12));
    let types: Vec<&str> = d["stack"].as_array().unwrap().iter().map(|x| x["type"].as_str().unwrap()).collect();
    assert_eq!(types, ["xgmModifierClump", "xgmModifierSculpt", "xgmModifierScale", "xgmSplineBase"]);
    assert_eq!(d["sculpt"][0]["active"], json!(1));
    assert_eq!(d["sculpt"][0]["layers"][1]["name"], json!("Detail"));
    run(&mut s, "xjenRemoveModifier", json!({"args": ["fur_scale"]}));
    let info = run(&mut s, "xjenGroomInfo", json!({}));
    let types: Vec<&str> = info[0]["stack"].as_array().unwrap().iter().map(|x| x["type"].as_str().unwrap()).collect();
    assert_eq!(types, ["xgmModifierClump", "xgmModifierSculpt", "xgmSplineBase"]);
    assert_eq!(info[0]["splines"], json!(12));
    assert!(s.run("xjenRemoveModifier", json!({"args": ["fur_base"]})).is_err());
}

#[test]
fn converts_to_ribbons_and_tubes() {
    let mut s = session();
    let made = run(&mut s, "xjenConvertToPolygons", json!({"args": ["fur"]}));
    assert_eq!(made, json!(["fur_mesh"]));
    let faces = run(&mut s, "polyEvaluate", json!({"args": ["fur_mesh"], "face": true}));
    assert_eq!(faces, json!(12 * 4));
    let made = run(&mut s, "xjenConvertToPolygons", json!({"args": ["fur"], "tube": true, "sides": 5}));
    let faces = run(&mut s, "polyEvaluate", json!({"args": [made[0].clone()], "face": true}));
    assert_eq!(faces, json!(12 * 4 * 5));
}

#[test]
fn presets_carry_settings_layers_and_edits() {
    let mut s = session();
    run(&mut s, "setAttr", json!({"args": ["furShape.taper", 0.6]}));
    run(&mut s, "setAttr", json!({"args": ["fur_base.cvCount", 7]}));
    let c = run(&mut s, "xjenAddModifier", json!({"args": ["fur"], "type": "cut"}));
    run(&mut s, "setAttr", json!({"args": [format!("{}.amount", c.as_str().unwrap()), 0.5]}));
    run(&mut s, "xjenGroomCtx", json!({"tool": "comb", "strength": 1.0}));
    run(&mut s, "xjenGroomStroke", json!({"args": ["fur"], "points": [[0.0, 0.3, 0.0], [0.3, 0.3, 0.0]], "radius": 1.0}));
    let want = splines(&mut s, "fur");
    let path = std::env::temp_dir().join(format!("xjen_preset_{}.json", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "xjenPreset", json!({"args": ["fur"], "export": path.clone()}));
    run(&mut s, "polyPlane", json!({"name": "skin2", "width": 2.0, "height": 2.0, "subdivisionsX": 2, "subdivisionsY": 2}));
    let made = run(&mut s, "xjenPreset", json!({"args": ["skin2"], "import": path, "name": "copy"}));
    assert_eq!(made, json!("copy"));
    let got = splines(&mut s, "copy");
    assert_eq!(got.len(), want.len());
    assert_eq!(got[0].points.len(), 7);
    assert!(got.iter().zip(&want).all(|(a, b)| a.points[6].distance(b.points[6]) < 1e-6));
    assert!((got[0].width[6] - want[0].width[6]).abs() < 1e-9);
}

#[test]
fn transfer_moves_a_description_to_another_mesh() {
    let mut s = session();
    run(&mut s, "polyPlane", json!({"name": "skin2", "width": 2.0, "height": 2.0, "subdivisionsX": 2, "subdivisionsY": 2}));
    run(&mut s, "setAttr", json!({"args": ["skin2.translateY", 3.0]}));
    run(&mut s, "xjenTransfer", json!({"args": ["fur"], "mesh": "skin2"}));
    assert!(splines(&mut s, "fur").iter().all(|x| (x.root().y - 3.0).abs() < 1e-9));
}
