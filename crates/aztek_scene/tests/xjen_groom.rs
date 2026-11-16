//! XJEN Interactive Groom core: a description grown on a mesh, its roots following the mesh, the Scale and Sculpt
//! modifiers, widths, xgmSplineQuery, scene files, and Harold rendering it.

use std::sync::Arc;

use aztek_scene::harold::{self, Request, Translator};
use aztek_scene::xjen::{self, sculpt};
use aztek_scene::Session;
use glam::DVec3;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session() -> Session {
    let mut s = Session::new();
    xjen::cmds::register(&mut s.commands);
    xjen::register_strands();
    s
}

/// A 4 x 4 plane, 2 units across, with a description of `density` splines per face on it.
fn groom(s: &mut Session, density: f64) -> String {
    run(s, "polyPlane", json!({"name": "skin", "width": 2.0, "height": 2.0, "subdivisionsX": 4, "subdivisionsY": 4}));
    let r = run(s, "xjenCreateGroom", json!({"args": ["skin"], "density": density, "length": 0.5, "seed": 3, "name": "fur"}));
    r[0].as_str().unwrap().to_string()
}

fn splines(s: &mut Session, desc: &str) -> xjen::SplineData {
    s.graph.evaluate();
    let tr = s.graph.find(desc).unwrap().unwrap();
    let sh = s.graph.children(tr)[0];
    xjen::render_splines(&s.graph, sh).cloned().expect("render data")
}

#[test]
fn grows_density_per_face_along_the_normal() {
    let mut s = session();
    let d = groom(&mut s, 5.0);
    assert_eq!(d, "fur");
    let sp = splines(&mut s, &d);
    assert_eq!(sp.splines.len(), 16 * 5);
    for x in &sp.splines {
        assert_eq!(x.points.len(), 5);
        let root = x.root();
        assert!(root.y.abs() < 1e-9 && root.x.abs() <= 1.0 + 1e-9 && root.z.abs() <= 1.0 + 1e-9, "{root}");
        let tip = *x.points.last().unwrap();
        assert!((tip - root - DVec3::new(0.0, 0.5, 0.0)).length() < 1e-9, "{tip} {root}");
    }
    assert_eq!(run(&mut s, "xgmSplineQuery", json!({"args": ["fur"], "splineCount": true})), json!(80));
    assert_eq!(run(&mut s, "xgmSplineQuery", json!({"listSplineDescriptions": true})), json!(["fur"]));
    assert_eq!(run(&mut s, "xgmSplineQuery", json!({"args": ["fur"], "isSplineDescription": true})), json!(true));
    assert_eq!(run(&mut s, "xgmSplineQuery", json!({"args": ["skin"], "isSplineDescription": true})), json!(false));
}

#[test]
fn fractional_density_and_seed() {
    let mut s = session();
    let d = groom(&mut s, 2.5);
    let n = splines(&mut s, &d).splines.len();
    assert!((32..=48).contains(&n), "{n}");
    let before: Vec<DVec3> = splines(&mut s, &d).splines.iter().map(|x| x.root()).collect();
    run(&mut s, "setAttr", json!({"args": ["fur_base.seed", 9]}));
    let after: Vec<DVec3> = splines(&mut s, &d).splines.iter().map(|x| x.root()).collect();
    assert_ne!(before, after);
}

#[test]
fn faces_named_bound_only_those() {
    let mut s = session();
    run(&mut s, "polyPlane", json!({"name": "skin", "subdivisionsX": 4, "subdivisionsY": 4}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin.f[0:1]", "skin.f[5]"], "density": 4}));
    let sp = splines(&mut s, "description1");
    assert_eq!(sp.splines.len(), 12);
    assert!(sp.splines.iter().all(|x| [0, 1, 5].contains(&x.face)));
}

#[test]
fn roots_follow_the_mesh_as_it_moves_and_deforms() {
    let mut s = session();
    let d = groom(&mut s, 3.0);
    let a = splines(&mut s, &d);
    run(&mut s, "setAttr", json!({"args": ["skin.translateY", 2.0]}));
    run(&mut s, "move", json!({"args": [0.0, 0.3, 0.0, "skin.vtx[12]"], "relative": true}));
    let b = splines(&mut s, &d);
    assert_eq!(a.splines.len(), b.splines.len());
    for (x, y) in a.splines.iter().zip(&b.splines) {
        assert_eq!(x.id, y.id);
        assert_eq!(x.face, y.face);
        assert!(y.root().y >= 2.0 - 1e-9, "{}", y.root());
    }
}

#[test]
fn scale_modifier_lengthens_by_magnitude() {
    let mut s = session();
    let d = groom(&mut s, 1.0);
    run(&mut s, "setAttr", json!({"args": ["fur_scale.scale", 2.0]}));
    assert!(splines(&mut s, &d).splines.iter().all(|x| (x.length() - 1.0).abs() < 1e-9));
    run(&mut s, "setAttr", json!({"args": ["fur_scale.magnitude", 0.5]}));
    assert!(splines(&mut s, &d).splines.iter().all(|x| (x.length() - 0.75).abs() < 1e-9));
    run(&mut s, "setAttr", json!({"args": ["fur_scale.mute", true]}));
    assert!(splines(&mut s, &d).splines.iter().all(|x| (x.length() - 0.5).abs() < 1e-9));
}

fn sculpt_one(s: &mut Session, id: u64) {
    let node = s.graph.find("fur_sculpt").unwrap().unwrap();
    let mut layer = sculpt::Layer { name: "Base".into(), ..Default::default() };
    layer.moves.insert(id, vec![[0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.4, 0.0, 0.0]]);
    let p = aztek_scene::harold::network::attr(&s.graph, node, "sculptData").unwrap();
    s.graph.set_value(p, aztek_scene::plug::PlugValue::Data(Some(Arc::new(sculpt::SculptData { layers: vec![layer], ..Default::default() })))).unwrap();
}

#[test]
fn sculpt_layers_move_points_by_weight_and_save() {
    let mut s = session();
    let d = groom(&mut s, 1.0);
    let first = splines(&mut s, &d).splines[0].clone();
    sculpt_one(&mut s, first.id);
    let moved = splines(&mut s, &d).splines[0].clone();
    let tip = |x: &xjen::Spline| *x.points.last().unwrap();
    assert!((tip(&moved).distance(tip(&first)) - 0.4).abs() < 1e-6);
    run(&mut s, "setAttr", json!({"args": ["fur_sculpt.layer[0].layerWeight", 0.5]}));
    let half = splines(&mut s, &d).splines[0].clone();
    assert!((tip(&half).distance(tip(&first)) - 0.2).abs() < 1e-6);
    run(&mut s, "setAttr", json!({"args": ["fur_sculpt.layer[0].layerVisibility", false]}));
    let off = splines(&mut s, &d).splines[0].clone();
    assert!(tip(&off).distance(tip(&first)) < 1e-9);
    run(&mut s, "setAttr", json!({"args": ["fur_sculpt.layer[0].layerVisibility", true]}));
    run(&mut s, "setAttr", json!({"args": ["furShape.taper", 0.5]}));

    let path = std::env::temp_dir().join(format!("xjen_groom_{}.ma", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true}));
    let mut t = session();
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    let back = splines(&mut t, &d);
    assert_eq!(back.splines.len(), 16);
    assert!((tip(&back.splines[0]).distance(tip(&half)) < 1e-5), "{} {}", tip(&back.splines[0]), tip(&half));
    assert!((back.splines[0].width[4] - 0.005).abs() < 1e-7);
    let sg = run(&mut t, "listConnections", json!({"args": ["furShape"], "type": "shadingEngine"}));
    assert_eq!(sg, json!(["fur_hairSG"]));
}

#[test]
fn width_and_taper() {
    let mut s = session();
    let d = groom(&mut s, 1.0);
    run(&mut s, "setAttr", json!({"args": ["furShape.widthScale", 0.04]}));
    run(&mut s, "setAttr", json!({"args": ["furShape.taper", 1.0]}));
    let x = splines(&mut s, &d).splines[0].clone();
    assert!((x.width[0] - 0.04).abs() < 1e-7);
    assert!(x.width[4].abs() < 1e-7);
    assert!((x.width[2] - 0.02).abs() < 1e-7);
}

#[test]
fn harold_renders_the_description_thinned_by_render_density() {
    let mut s = session();
    aztek_scene::cameras::make_defaults(&mut s.graph);
    let _ = groom(&mut s, 20.0);
    let count = |s: &mut Session| {
        let mut tr = Translator::default();
        let (scene, _) = harold::translate(s, &mut tr, &Request { camera: Some("persp".into()), width: Some(16), height: Some(16), ..Default::default() }).unwrap();
        scene.geo.meshes.iter().filter_map(|m| m.curves.as_ref().map(|_| m.prim_count())).sum::<usize>()
    };
    let all = count(&mut s);
    assert!(all > 0);
    run(&mut s, "setAttr", json!({"args": ["furShape.renderDensityMultiplier", 0.25]}));
    let some = count(&mut s);
    assert!(some * 3 < all && some > 0, "{some} of {all}");
    // The views see every spline.
    let v = aztek_scene::harold::strands::view_gather(&s.graph);
    assert_eq!(v.iter().map(|x| x.counts.len()).sum::<usize>(), 320);
}

#[test]
/// Binary files keep the description, its stack and settings. Sculpt moves are plug-in data, which binary files here
/// leave out (with a warning) until the reference's layout for them is known.
fn binary_files_keep_the_groom() {
    let mut s = session();
    let d = groom(&mut s, 1.0);
    run(&mut s, "setAttr", json!({"args": ["fur_scale.scale", 1.5]}));
    let want = splines(&mut s, &d).splines[0].clone();
    let path = std::env::temp_dir().join(format!("xjen_groom_{}.mb", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true, "type": "mayaBinary"}));
    let mut t = session();
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    let back = splines(&mut t, &d);
    assert_eq!(back.splines.len(), 16);
    assert!(back.splines[0].points.last().unwrap().distance(*want.points.last().unwrap()) < 1e-5);
}
