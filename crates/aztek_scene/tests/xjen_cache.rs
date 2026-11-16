//! XJEN caches: a description exported to Alembic and played back by a Spline Cache modifier, Noise baked, and Curve
//! to Spline reading a cache.

use aztek_scene::xjen;
use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session(name: &str) -> Session {
    let mut s = Session::new();
    aztek_scene::alembic::register(&mut s.commands);
    xjen::cmds::register(&mut s.commands);
    run(&mut s, "polyPlane", json!({"name": "skin", "width": 2.0, "height": 2.0, "subdivisionsX": 2, "subdivisionsY": 2}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin"], "density": 2, "length": 1.0, "seed": 7, "name": name}));
    s
}

fn splines(s: &mut Session, d: &str) -> Vec<xjen::Spline> {
    s.graph.evaluate();
    let id = xjen::cmds::description(s, d).unwrap();
    xjen::render_splines(&s.graph, id).unwrap().splines.clone()
}

fn temp(name: &str) -> String {
    std::env::temp_dir().join(format!("{}_{name}", std::process::id())).to_string_lossy().to_string()
}

#[test]
fn exported_cache_plays_back_on_another_description() {
    let mut s = session("fur");
    run(&mut s, "setKeyframe", json!({"args": ["fur_base"], "attribute": "initialLength", "time": 1, "value": 1.0}));
    run(&mut s, "setKeyframe", json!({"args": ["fur_base"], "attribute": "initialLength", "time": 5, "value": 2.0}));
    let path = temp("fur_cache.abc");
    run(&mut s, "xjenExportCache", json!({"args": ["fur"], "file": path.clone(), "frameRange": [1, 5]}));
    let mut t = session("copy");
    let node = run(&mut t, "xjenImportCache", json!({"args": ["copy"], "file": path}));
    assert!(node.as_str().unwrap().starts_with("copy_"));
    run(&mut t, "currentTime", json!({"args": [5]}));
    assert!(splines(&mut t, "copy").iter().all(|x| (x.length() - 2.0).abs() < 1e-4), "{:?}", splines(&mut t, "copy")[0].length());
    run(&mut t, "currentTime", json!({"args": [1]}));
    assert!(splines(&mut t, "copy").iter().all(|x| (x.length() - 1.0).abs() < 1e-4));
}

#[test]
fn baked_noise_stays_as_baked() {
    let mut s = session("fur");
    let n = run(&mut s, "xjenAddModifier", json!({"args": ["fur"], "type": "noise"}));
    let n = n.as_str().unwrap().to_string();
    let live = splines(&mut s, "fur");
    run(&mut s, "xjenBakeModifier", json!({"args": [n.clone()], "file": temp("noise.abc"), "frameRange": [1, 2]}));
    assert_eq!(run(&mut s, "getAttr", json!({"args": [format!("{n}.mode")]})), json!(1));
    let baked = splines(&mut s, "fur");
    assert!(baked.iter().zip(&live).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-5));
    run(&mut s, "setAttr", json!({"args": [format!("{n}.frequency"), 7.0]}));
    let still = splines(&mut s, "fur");
    assert!(still.iter().zip(&live).all(|(a, b)| a.points[4].distance(b.points[4]) < 1e-5));
    run(&mut s, "setAttr", json!({"args": [format!("{n}.mode"), 0]}));
    let changed = splines(&mut s, "fur");
    assert!(changed.iter().zip(&live).any(|(a, b)| a.points[4].distance(b.points[4]) > 1e-4));
}

#[test]
fn curve_to_spline_reads_a_cache() {
    let mut s = session("fur");
    let path = temp("c2s.abc");
    run(&mut s, "xjenExportCache", json!({"args": ["fur"], "file": path.clone()}));
    let n0 = splines(&mut s, "fur").len();
    let m = run(&mut s, "xjenAddModifier", json!({"args": ["fur"], "type": "curveToSpline"}));
    run(&mut s, "setAttr", json!({"args": [format!("{}.cacheFile", m.as_str().unwrap()), path], "type": "string"}));
    assert_eq!(splines(&mut s, "fur").len(), n0 * 2);
}
