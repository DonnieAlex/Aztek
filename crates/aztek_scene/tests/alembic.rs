//! AbcExport and AbcImport: transforms, meshes and hair written over a frame range, read back, animated ones streamed
//! by an AlembicNode.

use aztek_scene::Session;
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn session() -> Session {
    let mut s = Session::new();
    aztek_scene::alembic::register(&mut s.commands);
    aztek_scene::xjen::cmds::register(&mut s.commands);
    s
}

fn temp(name: &str) -> String {
    std::env::temp_dir().join(format!("{}_{name}", std::process::id())).to_string_lossy().to_string()
}

fn key(s: &mut Session, plug: &str, frame: f64, v: f64) {
    let (node, attr) = plug.split_once('.').unwrap();
    run(s, "setKeyframe", json!({"args": [node], "attribute": attr, "time": frame, "value": v}));
}

fn world(s: &mut Session, n: &str) -> DMat4 {
    let m = run(s, "xform", json!({"args": [n], "query": true, "matrix": true, "worldSpace": true}));
    let v: Vec<f64> = m.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    DMat4::from_cols_slice(&v)
}

fn points(s: &mut Session, n: &str) -> Vec<DVec3> {
    let v = run(s, "xform", json!({"args": [format!("{n}.vtx[*]")], "query": true, "translation": true, "worldSpace": true}));
    v.as_array().unwrap().chunks(3).map(|c| DVec3::new(c[0].as_f64().unwrap(), c[1].as_f64().unwrap(), c[2].as_f64().unwrap())).collect()
}

fn frame(s: &mut Session, f: f64) {
    run(s, "currentTime", json!({"args": [f]}));
}

#[test]
fn animated_transforms_and_meshes_stream_back() {
    let mut s = session();
    run(&mut s, "polyCube", json!({"name": "box"}));
    key(&mut s, "box.translateX", 1.0, 0.0);
    key(&mut s, "box.translateX", 10.0, 9.0);
    key(&mut s, "box.rotateY", 1.0, 0.0);
    key(&mut s, "box.rotateY", 10.0, 90.0);
    key(&mut s, "polyCube1.width", 1.0, 1.0);
    key(&mut s, "polyCube1.width", 10.0, 4.0);
    let mut want = Vec::new();
    for f in [1.0, 4.0, 10.0] {
        frame(&mut s, f);
        want.push((world(&mut s, "box"), points(&mut s, "box")));
    }
    let path = temp("anim.abc");
    run(&mut s, "AbcExport", json!({"jobArg": format!("-frameRange 1 10 -uvWrite -root box -file {path}")}));

    let mut t = session();
    let made = run(&mut t, "AbcImport", json!({"args": [path]}));
    let made: Vec<&str> = made.as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
    assert!(made.contains(&"box"));
    assert!(made.iter().any(|m| m.ends_with("_AlembicNode")));
    for (k, f) in [1.0, 4.0, 10.0].into_iter().enumerate() {
        frame(&mut t, f);
        let (m, p) = (world(&mut t, "box"), points(&mut t, "box"));
        assert!(m.abs_diff_eq(want[k].0, 1e-5), "frame {f}: {m} vs {}", want[k].0);
        assert_eq!(p.len(), want[k].1.len());
        for (a, b) in p.iter().zip(&want[k].1) {
            assert!(a.distance(*b) < 1e-4, "frame {f}: {a} vs {b}");
        }
    }
    // UVs came across.
    let uv = run(&mut t, "polyEvaluate", json!({"args": ["box"], "uvcoord": true}));
    assert_eq!(uv, json!(14));
}

#[test]
fn static_files_make_plain_nodes() {
    let mut s = session();
    run(&mut s, "polySphere", json!({"name": "ball", "subdivisionsX": 8, "subdivisionsY": 6}));
    run(&mut s, "setAttr", json!({"args": ["ball.translateY", 2.0]}));
    run(&mut s, "group", json!({"args": ["ball"], "name": "grp"}));
    run(&mut s, "setAttr", json!({"args": ["grp.translateZ", 1.0]}));
    let want = points(&mut s, "ball");
    let path = temp("static.abc");
    run(&mut s, "AbcExport", json!({"jobArg": [format!("-root grp -file {path}")]}));
    let mut t = session();
    run(&mut t, "AbcImport", json!({"args": [path]}));
    assert!(t.graph.find("AlembicNode").unwrap().is_none());
    let got = points(&mut t, "|grp|ball");
    assert_eq!(got.len(), want.len());
    assert!(got.iter().zip(&want).all(|(a, b)| a.distance(*b) < 1e-5));
    assert_eq!(run(&mut t, "polyEvaluate", json!({"args": ["ball"], "face": true})), json!(48));
}

#[test]
fn hair_exports_as_curves_with_widths() {
    let mut s = session();
    run(&mut s, "polyPlane", json!({"name": "skin", "subdivisionsX": 1, "subdivisionsY": 1}));
    run(&mut s, "xjenCreateGroom", json!({"args": ["skin"], "density": 3, "length": 1.0, "name": "fur"}));
    let path = temp("hair.abc");
    run(&mut s, "AbcExport", json!({"jobArg": format!("-root fur -file {path}")}));
    let a = aztek_abc::abc::read(std::fs::read(&path).unwrap()).unwrap();
    let fur = &a.top.children[0];
    assert_eq!(fur.name, "fur");
    let c = aztek_abc::geom::read_curves(&fur.children[0]).unwrap();
    assert_eq!(c.counts, vec![5, 5, 5]);
    assert_eq!(c.widths[0].len(), 15);
    let mut t = session();
    run(&mut t, "AbcImport", json!({"args": [path]}));
    let curves = run(&mut t, "ls", json!({"type": "nurbsCurve"}));
    assert_eq!(curves.as_array().unwrap().len(), 3);
}
