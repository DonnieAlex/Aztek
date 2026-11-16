//! XJEN modifiers: Clump, Noise, Cut, Collide, Displace, Guide, Linear Wire and Curve to Spline in a description's
//! stack, Magnitude and Mute on them, and the stack saved and opened.

use aztek_scene::xjen;
use aztek_scene::Session;
use glam::DVec3;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session() -> Session {
    let mut s = Session::new();
    xjen::cmds::register(&mut s.commands);
    run(&mut s, "polyPlane", json!({"name": "skin", "width": 2.0, "height": 2.0, "subdivisionsX": 4, "subdivisionsY": 4}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin"], "density": 6, "length": 1.0, "seed": 2, "name": "fur"}));
    s
}

fn splines(s: &mut Session) -> Vec<xjen::Spline> {
    s.graph.evaluate();
    let id = xjen::cmds::description(s, "fur").unwrap();
    xjen::render_splines(&s.graph, id).unwrap().splines.clone()
}

fn add(s: &mut Session, ty: &str, extra: Value) -> String {
    let mut call = json!({"args": ["fur"], "type": ty});
    if let Value::Object(m) = extra {
        for (k, v) in m {
            call[k] = v;
        }
    }
    run(s, "xjenAddModifier", call).as_str().unwrap().to_string()
}

fn tip_spread(v: &[xjen::Spline]) -> f64 {
    let tips: Vec<DVec3> = v.iter().map(|x| *x.points.last().unwrap()).collect();
    let roots: Vec<DVec3> = v.iter().map(|x| x.root()).collect();
    let mt = tips.iter().copied().sum::<DVec3>() / tips.len() as f64;
    let mr = roots.iter().copied().sum::<DVec3>() / roots.len() as f64;
    // How spread the tips are about their centre, against the roots' spread.
    tips.iter().map(|t| (*t - mt).length()).sum::<f64>() / roots.iter().map(|r| (*r - mr).length()).sum::<f64>()
}

#[test]
fn clump_gathers_and_magnitude_and_mute_scale_it() {
    let mut s = session();
    let before = splines(&mut s);
    let m = add(&mut s, "clump", json!({}));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.clump"), 1.0]}));
    let c = splines(&mut s);
    assert!(tip_spread(&c) < tip_spread(&before) * 0.8, "{} {}", tip_spread(&c), tip_spread(&before));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.magnitude"), 0.0]}));
    let off = splines(&mut s);
    assert!(off.iter().zip(&before).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.magnitude"), 1.0]}));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.preserveLength"), true]}));
    assert!(splines(&mut s).iter().all(|x| (x.length() - 1.0).abs() < 1e-6));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.mute"), true]}));
    let muted = splines(&mut s);
    assert!(muted.iter().zip(&before).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
}

#[test]
fn noise_wavers_keeping_length_and_seed_changes_it() {
    let mut s = session();
    let before = splines(&mut s);
    let m = add(&mut s, "noise", json!({}));
    let a = splines(&mut s);
    assert!(a.iter().zip(&before).any(|(x, y)| x.points[4].distance(y.points[4]) > 1e-3));
    assert!(a.iter().all(|x| (x.length() - 1.0).abs() < 1e-6));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.seed"), 5]}));
    let b = splines(&mut s);
    assert!(a.iter().zip(&b).any(|(x, y)| x.points[4].distance(y.points[4]) > 1e-4));
}

#[test]
fn cut_shortens_by_amount() {
    let mut s = session();
    let m = add(&mut s, "cut", json!({}));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.amount"), 0.25]}));
    assert!(splines(&mut s).iter().all(|x| (x.length() - 0.75).abs() < 1e-6 && x.points.len() == 5));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.randomness"), 1.0]}));
    let v = splines(&mut s);
    assert!(v.iter().all(|x| x.length() >= 0.75 - 1e-6 && x.length() <= 1.0 + 1e-6));
    assert!(v.iter().any(|x| x.length() > 0.8));
}

#[test]
fn collide_keeps_splines_above_a_mesh() {
    let mut s = session();
    run(&mut s, "polyPlane", json!({"name": "roof", "width": 1.0, "height": 1.0, "subdivisionsX": 1, "subdivisionsY": 1}));
    run(&mut s, "setAttr", json!({"args": ["roof.translateY", 0.5]}));
    add(&mut s, "collide", json!({"meshes": ["roof"]}));
    let v = splines(&mut s);
    let under: Vec<_> = v.iter().filter(|x| x.root().x.abs() < 0.45 && x.root().z.abs() < 0.45).collect();
    assert!(!under.is_empty());
    for x in under {
        assert!(x.points.iter().skip(1).all(|p| p.y >= 0.5 - 1e-6), "{:?}", x.points);
    }
}

#[test]
fn displace_moves_along_the_normal() {
    let mut s = session();
    let m = add(&mut s, "displace", json!({}));
    run(&mut s, "setAttr", json!({"args": [format!("{m}.displacement"), 0.3]}));
    assert!(splines(&mut s).iter().all(|x| (x.points[4].y - 1.3).abs() < 1e-9));
}

#[test]
fn guides_shape_the_splines() {
    let mut s = session();
    run(&mut s, "curve", json!({"degree": 1, "point": [[0.0, 0.0, 0.0], [0.5, 0.5, 0.0], [1.0, 1.0, 0.0]], "name": "g1"}));
    add(&mut s, "guide", json!({"curves": ["g1"]}));
    for x in splines(&mut s) {
        let d = x.points[4] - x.root();
        assert!((d - DVec3::new(1.0, 1.0, 0.0)).length() < 1e-6, "{d}");
    }
    // Made from the splines when no curves are given: a tenth of them.
    let mut t = session();
    add(&mut t, "guide", json!({"fraction": 0.1}));
    let made = run(&mut t, "ls", json!({"args": ["fur_guide*"], "type": "transform"}));
    let n = made.as_array().unwrap().len();
    assert!((3..=20).contains(&n), "{n}");
}

#[test]
fn linear_wires_bend_splines_near_them() {
    let mut s = session();
    let before = splines(&mut s);
    add(&mut s, "linearWire", json!({"fraction": 0.05}));
    // Unmoved wires change nothing.
    let same = splines(&mut s);
    assert!(same.iter().zip(&before).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-6));
    let wires = run(&mut s, "ls", json!({"args": ["fur_wire*"], "type": "transform"}));
    let wires: Vec<String> = wires.as_array().unwrap().iter().filter_map(Value::as_str).filter(|n| !n.ends_with("_reference")).map(str::to_string).collect();
    assert!(!wires.is_empty());
    for w in &wires {
        run(&mut s, "setAttr", json!({"args": [format!("{w}.translateX"), 0.5]}));
    }
    let moved = splines(&mut s);
    assert!(moved.iter().zip(&before).any(|(a, b)| a.points[4].x - b.points[4].x > 0.1));
    // Roots stay.
    assert!(moved.iter().zip(&before).all(|(a, b)| a.root().distance(b.root()) < 1e-9));
}

#[test]
fn curves_become_splines_rooted_on_the_mesh() {
    let mut s = session();
    let n = splines(&mut s).len();
    run(&mut s, "curve", json!({"degree": 1, "point": [[0.3, 0.2, 0.3], [0.3, 1.2, 0.3], [0.8, 1.8, 0.3]], "name": "c1"}));
    let m = add(&mut s, "curveToSpline", json!({"curves": ["c1"]}));
    let v = splines(&mut s);
    assert_eq!(v.len(), n + 1);
    let last = v.last().unwrap();
    assert!(last.root().y.abs() < 1e-9 && (last.root().x - 0.3).abs() < 1e-9, "{}", last.root());
    assert_eq!(last.points.len(), 5);
    run(&mut s, "setAttr", json!({"args": [format!("{m}.alignToNormal"), true]}));
    let v = splines(&mut s);
    let last = v.last().unwrap();
    let first = (last.points[1] - last.points[0]).normalize();
    assert!(first.y > 0.999, "{first}");
}

#[test]
fn the_stack_saves_and_opens() {
    let mut s = session();
    let c = add(&mut s, "clump", json!({}));
    add(&mut s, "cut", json!({"after": c}));
    run(&mut s, "setAttr", json!({"args": [format!("{c}.clump"), 0.8]}));
    let want = splines(&mut s);
    let path = std::env::temp_dir().join(format!("xjen_mods_{}.ma", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true}));
    let mut t = Session::new();
    xjen::cmds::register(&mut t.commands);
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    let back = splines(&mut t);
    assert_eq!(back.len(), want.len());
    assert!(back.iter().zip(&want).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-6));
    let d = xjen::cmds::description(&t, "fur").unwrap();
    let kinds: Vec<String> = xjen::stack(&t.graph, d).into_iter().map(|n| t.graph.node(n).unwrap().ty().name.clone()).collect();
    assert_eq!(kinds, ["xgmSplineBase", "xgmModifierScale", "xgmModifierSculpt", "xgmModifierClump", "xgmModifierCut"]);
}
