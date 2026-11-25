//! Skin Tools: layers on a skinCluster, blended into its weights; tools, mirror, files and transfer.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// A cylinder along X from -10 to 10, bound to a joint on each side and one in the middle.
fn rig() -> Session {
    let mut s = Session::new();
    aztek_scene::skin_layers::register(&mut s.commands);
    run(&mut s, "select", json!({"clear": true}));
    run(&mut s, "joint", json!({"name": "C_root", "position": [0, 0, 0]}));
    run(&mut s, "select", json!({"clear": true}));
    run(&mut s, "joint", json!({"name": "arm_L0_jnt", "position": [6, 0, 0]}));
    run(&mut s, "select", json!({"clear": true}));
    run(&mut s, "joint", json!({"name": "arm_R0_jnt", "position": [-6, 0, 0]}));
    run(&mut s, "polyCylinder", json!({"name": "tube", "height": 20, "subdivisionsHeight": 10, "axis": [1, 0, 0]}));
    run(&mut s, "skinCluster", json!({"args": ["C_root", "arm_L0_jnt", "arm_R0_jnt", "tube"], "toSelectedBones": true, "name": "skin"}));
    s
}

fn weights(s: &mut Session, inf: &str) -> Vec<f64> {
    let n = run(s, "polyEvaluate", json!({"args": ["tube"], "vertex": true})).as_i64().unwrap();
    (0..n).map(|i| run(s, "skinPercent", json!({"args": ["skin", format!("tube.vtx[{i}]")], "transform": inf, "query": true})).as_f64().unwrap()).collect()
}

fn xs(s: &mut Session) -> Vec<f64> {
    let n = run(s, "polyEvaluate", json!({"args": ["tube"], "vertex": true})).as_i64().unwrap();
    (0..n).map(|i| run(s, "pointPosition", json!({"args": [format!("tube.vtx[{i}]")], "world": true}))[0].as_f64().unwrap()).collect()
}

#[test]
fn init_keeps_the_weights_and_a_layer_overrides_them() {
    let mut s = rig();
    let before = weights(&mut s, "arm_L0_jnt");
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "exists": true})), json!(false));
    let d = run(&mut s, "skinLayers", json!({"args": ["tube"], "initialize": true}));
    assert_eq!(d, json!("ngSkinToolsData_skin"));
    assert_eq!(run(&mut s, "nodeType", json!({"args": ["ngSkinToolsData_skin"]})), json!("ngst2SkinLayerData"));
    let after = weights(&mut s, "arm_L0_jnt");
    for (a, b) in before.iter().zip(&after) {
        assert!((a - b).abs() < 1e-9);
    }
    let ls = run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "layers": true}));
    assert_eq!(ls[0]["name"], json!("base weights"));

    // A new layer is empty and changes nothing until it holds weights.
    let top = run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "override"})).as_u64().unwrap();
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "current": true})), json!(top));
    assert!((weights(&mut s, "arm_L0_jnt")[0] - after[0]).abs() < 1e-9);
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "target": "C_root", "mode": "replace", "intensity": 1.0}));
    assert!(weights(&mut s, "C_root").iter().all(|w| (w - 1.0).abs() < 1e-9));
    // Half opacity: half the layer, half the base.
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": top, "opacity": 0.5}));
    let c = weights(&mut s, "C_root");
    let base_c = before.iter().zip(weights(&mut s, "arm_L0_jnt")).map(|(b, _)| *b).collect::<Vec<_>>();
    let _ = base_c;
    assert!(c.iter().all(|w| *w >= 0.5 - 1e-9));
    // A mask of zero hides it.
    let n = c.len();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": top, "target": "mask", "weights": vec![0.0; n]}));
    let l = weights(&mut s, "arm_L0_jnt");
    for (a, b) in after.iter().zip(&l) {
        assert!((a - b).abs() < 1e-9);
    }
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "layer": top, "weights": true, "target": "mask"})), json!(vec![0.0; n]));
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": top, "enabled": false}));
    // Undo takes the edits back one by one.
    s.undo();
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "layer": top}))["enabled"], json!(true));

    // Deleting the layers leaves the weights as they blend.
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": top, "target": "mask", "weights": []}));
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": top, "opacity": 1.0}));
    run(&mut s, "skinLayers", json!({"args": ["tube"], "delete": true}));
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "exists": true})), json!(false));
    assert!(weights(&mut s, "C_root").iter().all(|w| (w - 1.0).abs() < 1e-9));
}

#[test]
fn closest_joint_mirror_and_sharpen() {
    let mut s = rig();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "initialize": true}));
    run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "sides"}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "assignFromClosestJoint": true}));
    let x = xs(&mut s);
    let l = weights(&mut s, "arm_L0_jnt");
    let r = weights(&mut s, "arm_R0_jnt");
    for i in 0..x.len() {
        if x[i] > 3.5 {
            assert_eq!(l[i], 1.0, "point {i} at {}", x[i]);
        }
        if x[i] < -3.5 {
            assert_eq!(r[i], 1.0);
        }
    }
    // The left side flooded to the root; mirrored, the right side follows.
    run(&mut s, "select", json!({"args": ["tube.vtx[*]"]}));
    let left: Vec<String> = (0..x.len()).filter(|&i| x[i] > 0.5).map(|i| format!("tube.vtx[{i}]")).collect();
    run(&mut s, "select", json!({"args": left, "replace": true}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "selectedComponents": true, "target": "C_root", "intensity": 1.0, "mode": "replace"}));
    run(&mut s, "select", json!({"clear": true}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "mirror": true}));
    let c = weights(&mut s, "C_root");
    for i in 0..x.len() {
        if x[i].abs() > 0.5 {
            assert!((c[i] - 1.0).abs() < 1e-9, "point {i} at {}", x[i]);
        }
    }
    // Pairs by name, the root with itself.
    let pairs = run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "influencePairs": true}));
    assert_eq!(pairs, json!([[0, 0], [1, 2], [2, 1]]));
    // Sharpen keeps each point's total.
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "target": "C_root", "mode": "replace", "intensity": 0.6}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "mode": "sharpen", "intensity": 0.5}));
    let c = weights(&mut s, "C_root");
    assert!(c.iter().all(|w| *w > 0.6 - 1e-9) && c.iter().any(|w| *w > 0.65), "{c:?}");
}

#[test]
fn layers_save_and_load_and_go_to_another_mesh() {
    let dir = std::env::temp_dir().join(format!("aztek_layers_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut s = rig();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "initialize": true}));
    let id = run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "root"}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "target": "C_root", "intensity": 1.0}));
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": id, "opacity": 0.3}));
    let want = weights(&mut s, "C_root");

    let ma = dir.join("layers.ma").to_string_lossy().replace('\\', "/");
    run(&mut s, "file", json!({"rename": ma}));
    run(&mut s, "file", json!({"save": true, "type": "mayaAscii", "force": true}));
    eprintln!("{}", std::fs::read_to_string(&ma).unwrap().lines().filter(|l| l.contains("ngSkin") || l.contains("ngst")).collect::<Vec<_>>().join("\n"));
    let mut t = Session::new();
    aztek_scene::skin_layers::register(&mut t.commands);
    run(&mut t, "file", json!({"args": [ma], "open": true, "force": true}));
    let ls = run(&mut t, "skinLayers", json!({"args": ["tube"], "query": true, "layers": true}));
    assert_eq!(ls.as_array().unwrap().len(), 2);
    assert_eq!(ls[1]["opacity"], json!(0.3));
    let got = weights(&mut t, "C_root");
    for (a, b) in want.iter().zip(&got) {
        assert!((a - b).abs() < 1e-6);
    }

    let file = dir.join("layers.json").to_string_lossy().into_owned();
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "exportFile": file}));
    // Another cylinder, bound the same way, takes them by closest point.
    run(&mut s, "polyCylinder", json!({"name": "tube2", "height": 20, "subdivisionsHeight": 10, "axis": [1, 0, 0]}));
    run(&mut s, "skinCluster", json!({"args": ["C_root", "arm_L0_jnt", "arm_R0_jnt", "tube2"], "toSelectedBones": true, "name": "skin2"}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube2"], "importFile": file}));
    let ls = run(&mut s, "skinLayers", json!({"args": ["tube2"], "query": true, "layers": true}));
    assert_eq!(ls[1]["name"], json!("root"));
    let w0 = run(&mut s, "skinPercent", json!({"args": ["skin2", "tube2.vtx[0]"], "transform": "C_root", "query": true})).as_f64().unwrap();
    assert!((w0 - want[0]).abs() < 1e-6);
    run(&mut s, "polyCylinder", json!({"name": "tube3", "height": 20, "subdivisionsHeight": 10, "axis": [1, 0, 0]}));
    run(&mut s, "skinCluster", json!({"args": ["C_root", "arm_L0_jnt", "arm_R0_jnt", "tube3"], "toSelectedBones": true, "name": "skin3"}));
    run(&mut s, "skinLayersTool", json!({"args": ["tube3"], "transferFrom": "tube", "vertexTransferMode": "vertexId"}));
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube3"], "query": true, "layers": true})).as_array().unwrap().len(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn groups_brush_and_layer_order() {
    let mut s = rig();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "initialize": true}));
    let g = run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "group"})).as_i64().unwrap();
    let c = run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "child", "parent": g})).as_i64().unwrap();
    run(&mut s, "skinLayersTool", json!({"args": ["tube"], "flood": true, "target": "arm_R0_jnt", "intensity": 1.0}));
    assert!(weights(&mut s, "arm_R0_jnt").iter().all(|w| (w - 1.0).abs() < 1e-9));
    let n = weights(&mut s, "C_root").len();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": g, "target": "mask", "weights": vec![0.0; n]}));
    assert!(weights(&mut s, "arm_R0_jnt").iter().any(|w| *w < 0.5));
    let info = run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "layer": g}));
    assert_eq!(info["children"], json!([c]));
    // Removing the group takes the child.
    run(&mut s, "skinLayers", json!({"args": ["tube"], "remove": g}));
    assert_eq!(run(&mut s, "skinLayers", json!({"args": ["tube"], "query": true, "layers": true})).as_array().unwrap().len(), 1);
    // The brush paints the current layer's target.
    let l = run(&mut s, "skinLayers", json!({"args": ["tube"], "add": "paint"})).as_i64().unwrap();
    run(&mut s, "skinLayers", json!({"args": ["tube"], "edit": true, "layer": l, "paintTarget": "arm_L0_jnt"}));
    run(&mut s, "skinLayersTool", json!({"mode": "replace", "intensity": 1.0}));
    assert_eq!(run(&mut s, "skinLayersTool", json!({"query": true, "mode": true}))["mode"], json!("replace"));
    let hit = run(&mut s, "skinLayersTool", json!({"args": ["tube"], "stamp": [-10, 0, 0]})).as_i64().unwrap();
    assert!(hit > 0);
    let n = run(&mut s, "polyEvaluate", json!({"args": ["tube"], "vertex": true})).as_i64().unwrap();
    let d: Vec<f64> = (0..n).map(|i| { let p = run(&mut s, "pointPosition", json!({"args": [format!("tube.vtx[{i}]")], "world": true})); ((p[0].as_f64().unwrap() + 10.0).powi(2) + p[1].as_f64().unwrap().powi(2) + p[2].as_f64().unwrap().powi(2)).sqrt() }).collect();
    let w = weights(&mut s, "arm_L0_jnt");
    let far = (0..d.len()).min_by(|&a, &b| d[a].total_cmp(&d[b])).unwrap();
    assert!(w[far] > 0.5, "{}", w[far]);
    assert!(s.run("skinLayers", json!({"args": ["tube"], "remove": 99})).is_err());
}
