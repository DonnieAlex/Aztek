//! XJEN grooming tools: each tool's effect through scripted strokes, undo per stroke, freezing, picking splines,
//! sculpt layers and collision.

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
    xjen::register_strands();
    run(&mut s, "xjenGroomCtx", json!({"strength": 1.0, "collide": false, "invertFrozen": false, "altMode": false, "clearSelection": true}));
    run(&mut s, "polyPlane", json!({"name": "skin", "width": 2.0, "height": 2.0, "subdivisionsX": 4, "subdivisionsY": 4}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin"], "density": 6, "length": 1.0, "seed": 1, "name": "fur"}));
    s
}

fn splines(s: &mut Session) -> Vec<xjen::Spline> {
    s.graph.evaluate();
    let id = xjen::cmds::description(s, "fur").unwrap();
    xjen::render_splines(&s.graph, id).unwrap().splines.clone()
}

fn stroke(s: &mut Session, tool: &str, pts: &[[f64; 3]], radius: f64, invert: bool) -> Value {
    run(s, "xjenGroomCtx", json!({"tool": tool}));
    run(s, "xjenGroomStroke", json!({"args": ["fur"], "points": pts, "radius": radius, "invert": invert}))
}

fn near(x: &xjen::Spline, c: DVec3, r: f64) -> bool {
    x.root().distance(DVec3::new(c.x, 0.0, c.z)) < r
}

#[test]
fn comb_bends_keeps_length_and_undoes_as_one() {
    let mut s = session();
    let before = splines(&mut s);
    let pts: Vec<[f64; 3]> = (0..6).map(|k| [k as f64 * 0.06, 0.5, 0.0]).collect();
    assert_eq!(stroke(&mut s, "comb", &pts, 0.5, false), json!(true));
    let after = splines(&mut s);
    let moved: Vec<_> = after.iter().zip(&before).filter(|(a, b)| a.points[4].distance(b.points[4]) > 1e-3).collect();
    assert!(!moved.is_empty());
    for (a, b) in &moved {
        assert!(a.points[4].x > b.points[4].x, "combed along +x");
        assert!((a.length() - 1.0).abs() < 1e-6, "{}", a.length());
        assert!(a.root().distance(b.root()) < 1e-9);
    }
    // Far ones untouched.
    assert!(after.iter().zip(&before).filter(|(_, b)| b.root().length() > 1.0).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
    s.undo();
    let back = splines(&mut s);
    assert!(back.iter().zip(&before).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
}

#[test]
fn length_cut_and_width() {
    let mut s = session();
    let c = DVec3::new(0.5, 0.0, 0.5);
    stroke(&mut s, "length", &[[c.x, 0.0, c.z]; 4], 0.4, false);
    let a = splines(&mut s);
    assert!(a.iter().filter(|x| near(x, c, 0.2)).all(|x| x.length() > 1.1));
    assert!(a.iter().filter(|x| !near(x, c, 0.45)).all(|x| (x.length() - 1.0).abs() < 1e-9));
    stroke(&mut s, "length", &[[c.x, 0.0, c.z]; 4], 0.4, true);

    let cut = DVec3::new(-0.5, 0.4, -0.5);
    stroke(&mut s, "cut", &[cut.to_array()], 0.3, false);
    let b = splines(&mut s);
    let hit: Vec<_> = b.iter().filter(|x| near(x, cut, 0.15)).collect();
    assert!(!hit.is_empty());
    assert!(hit.iter().all(|x| (x.length() - 0.4).abs() < 1e-6), "{:?}", hit.iter().map(|x| x.length()).collect::<Vec<_>>());
    assert!(hit.iter().all(|x| x.points.len() == 5));

    stroke(&mut s, "width", &[[0.0, 0.0, 0.0]; 3], 0.3, false);
    let w = splines(&mut s);
    assert!(w.iter().filter(|x| near(x, DVec3::ZERO, 0.1)).all(|x| x.width_scale > 1.1));
    assert!(w.iter().filter(|x| !near(x, DVec3::ZERO, 0.35)).all(|x| x.width_scale == 1.0));
}

#[test]
fn freeze_shields_and_invert_frozen_reverses() {
    let mut s = session();
    let before = splines(&mut s);
    stroke(&mut s, "freeze", &[[0.5, 0.5, 0.5]; 3], 0.6, false);
    let pts: Vec<[f64; 3]> = (0..5).map(|k| [0.5 + k as f64 * 0.05, 0.5, 0.5]).collect();
    stroke(&mut s, "comb", &pts, 0.6, false);
    let after = splines(&mut s);
    let frozen = |x: &xjen::Spline| near(x, DVec3::new(0.5, 0.0, 0.5), 0.2);
    assert!(after.iter().zip(&before).filter(|(_, b)| frozen(b)).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
    run(&mut s, "xjenGroomCtx", json!({"invertFrozen": true}));
    stroke(&mut s, "comb", &pts, 0.6, false);
    let inv = splines(&mut s);
    assert!(inv.iter().zip(&after).filter(|(_, b)| frozen(b)).any(|(a, b)| a.points[4].distance(b.points[4]) > 1e-3));
    run(&mut s, "xjenGroomCtx", json!({"invertFrozen": false}));
    // Ctrl unfreezes.
    stroke(&mut s, "freeze", &[[0.5, 0.5, 0.5]; 6], 0.6, true);
    let again = splines(&mut s);
    stroke(&mut s, "comb", &pts, 0.6, false);
    let last = splines(&mut s);
    assert!(last.iter().zip(&again).filter(|(_, b)| frozen(b)).any(|(a, b)| a.points[4].distance(b.points[4]) > 1e-3));
}

#[test]
fn picked_splines_limit_the_tools_and_freeze() {
    let mut s = session();
    run(&mut s, "select", json!({"args": ["skin.f[0]"], "replace": true}));
    assert_eq!(run(&mut s, "xgmSplineSelect", json!({"args": ["fur"], "replaceBySelectedFaces": true})), json!(6));
    let before = splines(&mut s);
    let pts: Vec<[f64; 3]> = (0..5).map(|k| [k as f64 * 0.1 - 1.0, 0.5, -1.0]).collect();
    stroke(&mut s, "comb", &pts, 3.0, false);
    let after = splines(&mut s);
    for (a, b) in after.iter().zip(&before) {
        let moved = a.points[4].distance(b.points[4]) > 1e-6;
        assert_eq!(moved, b.face == 0, "face {}", b.face);
    }
    assert_eq!(run(&mut s, "xgmSplineSelect", json!({"args": ["fur"], "convertToFreeze": true})), json!(6));
    stroke(&mut s, "comb", &pts, 3.0, false);
    let last = splines(&mut s);
    for (a, b) in last.iter().zip(&after) {
        let moved = a.points[4].distance(b.points[4]) > 1e-6;
        assert_eq!(moved, b.face != 0, "face {}", b.face);
    }
}

#[test]
fn density_place_and_remove_save_with_the_scene() {
    let mut s = session();
    let n0 = splines(&mut s).len();
    assert_eq!(n0, 96);
    stroke(&mut s, "density", &[[0.25, 0.0, 0.25]; 3], 0.4, false);
    let n1 = splines(&mut s).len();
    assert!(n1 > n0, "{n1}");
    stroke(&mut s, "place", &[[-0.7, 0.0, 0.66]], 0.2, false);
    let all = splines(&mut s);
    assert_eq!(all.len(), n1 + 1);
    let placed = all.last().unwrap();
    assert!(placed.root().distance(DVec3::new(-0.7, 0.0, 0.66)) < 1e-6);
    assert!(placed.id & xjen::edits::PLACED != 0);
    stroke(&mut s, "density", &[[-0.5, 0.0, -0.5]; 8], 0.3, true);
    let n2 = splines(&mut s).len();
    assert!(n2 < n1 + 1, "{n2}");
    let path = std::env::temp_dir().join(format!("xjen_tools_{}.ma", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true}));
    let mut t = session();
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    let back = splines(&mut t);
    assert_eq!(back.len(), n2);
    // Undo takes the removal back.
    s.undo();
    assert_eq!(splines(&mut s).len(), n1 + 1);
}

#[test]
fn layers_weight_merge_and_remove() {
    let mut s = session();
    let straight = splines(&mut s);
    assert_eq!(run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "add": "Combed"})), json!(1));
    let pts: Vec<[f64; 3]> = (0..5).map(|k| [k as f64 * 0.08, 0.5, 0.0]).collect();
    stroke(&mut s, "comb", &pts, 0.6, false);
    let combed = splines(&mut s);
    run(&mut s, "setAttr", json!({"args": ["fur_sculpt.layer[1].layerWeight", 0.0]}));
    let off = splines(&mut s);
    assert!(off.iter().zip(&straight).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-9));
    run(&mut s, "setAttr", json!({"args": ["fur_sculpt.layer[1].layerWeight", 1.0]}));
    let list = run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "list": true}));
    assert_eq!(list, json!([["Base", 1.0, true], ["Combed", 1.0, true]]));
    run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "mergeVisible": true}));
    assert_eq!(run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "list": true})), json!([["Base", 1.0, true]]));
    let merged = splines(&mut s);
    assert!(merged.iter().zip(&combed).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-5));
    run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "add": "Two"}));
    run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "remove": 1}));
    assert_eq!(run(&mut s, "xjenSculptLayer", json!({"args": ["fur"], "list": true})).as_array().unwrap().len(), 1);
}

#[test]
fn collide_keeps_points_above_the_surface() {
    let mut s = session();
    run(&mut s, "xjenGroomCtx", json!({"collide": true, "collisionDistance": 0.05}));
    let pts: Vec<[f64; 3]> = (0..8).map(|k| [0.0, 0.5 - k as f64 * 0.15, 0.0]).collect();
    stroke(&mut s, "grab", &pts, 0.8, false);
    let a = splines(&mut s);
    assert!(a.iter().all(|x| x.points.iter().skip(1).all(|p| p.y >= 0.05 - 1e-6)), "{:?}", a.iter().map(|x| x.points[4].y).fold(f64::INFINITY, f64::min));
    run(&mut s, "xjenGroomCtx", json!({"collide": false}));
    s.undo();
    stroke(&mut s, "grab", &pts, 0.8, false);
    let b = splines(&mut s);
    assert!(b.iter().any(|x| x.points[4].y < 0.0));
}

#[test]
fn clump_gathers_part_spreads_twist_smooth_noise_change() {
    let spread = |v: &[xjen::Spline], c: DVec3| {
        let tips: Vec<DVec3> = v.iter().filter(|x| near(x, c, 0.3)).map(|x| x.points[4]).collect();
        let m = tips.iter().copied().sum::<DVec3>() / tips.len() as f64;
        tips.iter().map(|t| t.distance(m)).sum::<f64>() / tips.len() as f64
    };
    let c = DVec3::new(0.0, 0.0, 0.0);
    let mut s = session();
    let before = splines(&mut s);
    stroke(&mut s, "clump", &[[0.0, 0.8, 0.0]; 3], 0.5, false);
    assert!(spread(&splines(&mut s), c) < spread(&before, c) * 0.8);
    let mut s = session();
    stroke(&mut s, "part", &[[0.0, 0.5, 0.0]; 3], 0.5, false);
    let p = splines(&mut s);
    assert!(spread(&p, c) > spread(&before, c));
    assert!(p.iter().all(|x| (x.length() - 1.0).abs() < 1e-6));
    for tool in ["twist", "smooth", "noise"] {
        let mut s = session();
        if tool != "noise" {
            stroke(&mut s, "comb", &[[0.0, 0.5, 0.0], [0.2, 0.5, 0.0], [0.4, 0.5, 0.0]], 0.6, false);
        }
        let a = splines(&mut s);
        stroke(&mut s, tool, &[[0.0, 0.5, 0.0]; 3], 0.6, false);
        let b = splines(&mut s);
        assert!(a.iter().zip(&b).any(|(x, y)| x.points[4].distance(y.points[4]) > 1e-4), "{tool} changed nothing");
        assert!(b.iter().all(|x| (x.length() - 1.0).abs() < 1e-3), "{tool} changed length");
    }
}
