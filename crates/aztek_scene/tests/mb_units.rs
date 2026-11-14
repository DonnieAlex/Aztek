//! A binary scene file keeps the scene's units and frame rate. Its own file: the frame rate is process-wide, and the
//! other tests' new scenes would change it under this one.

use aztek_scene::{ma_file, Session};
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn temp(name: &str) -> String {
    let dir = std::env::temp_dir().join(format!("aztek_mb_units_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name).to_str().unwrap().to_string()
}

#[test]
fn units_and_frame_rate_come_back() {
    let mut s = Session::new();
    run(&mut s, "currentUnit", json!({"linear": "mm", "angle": "rad", "time": "ntsc"}));
    run(&mut s, "polyCube", json!({"name": "box"}));
    run(&mut s, "setAttr", json!({"args": ["box.translate", 10, 20, 30], "type": "double3"}));
    run(&mut s, "setAttr", json!({"args": ["box.rotateX", 0.5]}));
    run(&mut s, "setKeyframe", json!({"args": ["box"], "attribute": "translateZ", "time": 12, "value": 4}));
    let out = temp("units.mb");
    ma_file::save_scene(&mut s, &out, ma_file::Format::Mb).unwrap();
    let mut t = Session::new();
    ma_file::open_scene(&mut t, &out).unwrap();
    assert_eq!(run(&mut t, "currentUnit", json!({"query": true, "queried": ["linear"]})), json!("mm"));
    assert_eq!(run(&mut t, "currentUnit", json!({"query": true, "queried": ["angle"]})), json!("rad"));
    assert_eq!(run(&mut t, "currentUnit", json!({"query": true, "queried": ["time"]})), json!("ntsc"));
    let v = run(&mut t, "getAttr", json!({"args": ["box.translate"]}));
    let v: Vec<f64> = v[0].as_array().map_or_else(|| v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect(), |a| a.iter().map(|x| x.as_f64().unwrap()).collect());
    // Z is keyed.
    for (a, b) in v.iter().zip([10.0, 20.0, 4.0]) {
        assert!((a - b).abs() < 1e-9, "{v:?}");
    }
    let r = run(&mut t, "getAttr", json!({"args": ["box.rotateX"]})).as_f64().unwrap();
    assert!((r - 0.5).abs() < 1e-12, "{r}");
    let keys = run(&mut t, "keyframe", json!({"args": ["box.translateZ"], "query": true, "queried": ["timeChange"]}));
    assert_eq!(keys, json!([12.0]));
}
