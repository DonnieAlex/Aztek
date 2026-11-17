//! Dynamic hair (XJEN step 8, nHair): makeCurvesDynamic puts curves on follicles of a hair system, which moves them
//! a frame at a time.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn y_of(c: &mut Core, what: &str) -> f64 {
    mel(c, &format!("pointPosition -w {what};"))[1].as_f64().unwrap()
}

#[test]
fn curves_made_dynamic_fall_from_their_roots() {
    let mut c = Core::new();
    // A strand standing up, sideways a little so it bends one way.
    mel(&mut c, "curve -d 3 -p 0 0 0 -p 0.1 1 0 -p 0.2 2 0 -p 0.3 3 0 -n strand;");
    let made = mel(&mut c, "select strand; makeCurvesDynamic 2 { \"0\", \"0\", \"1\", \"1\", \"0\"};");
    assert_eq!(made, json!(["hairSystemShape1"]));
    assert_eq!(mel(&mut c, "nodeType hairSystemShape1;"), json!("hairSystem"));
    assert_eq!(mel(&mut c, "objExists follicle1;"), json!(true));
    assert_eq!(mel(&mut c, "listRelatives -p strand;"), json!(["follicle1"]));
    // At the start frame the output curve is the start curve.
    mel(&mut c, "currentTime 1;");
    assert!((y_of(&mut c, "curve1.cv[3]") - 3.0).abs() < 1e-6);
    // Later the tip has fallen, the root holds.
    mel(&mut c, "currentTime 40;");
    let tip = y_of(&mut c, "curve1.cv[3]");
    let root = y_of(&mut c, "curve1.cv[0]");
    assert!(tip < 2.0, "{tip}");
    assert!(root.abs() < 1e-6, "{root}");
    // The strand keeps its length, near enough.
    let p: Vec<Vec<f64>> = (0..4).map(|i| mel(&mut c, &format!("pointPosition -w curve1.cv[{i}];")).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
    let len: f64 = (1..4).map(|i| ((p[i][0] - p[i - 1][0]).powi(2) + (p[i][1] - p[i - 1][1]).powi(2) + (p[i][2] - p[i - 1][2]).powi(2)).sqrt()).sum();
    assert!((len - 3.015).abs() < 0.3, "{len}");
    // Both ends held: the tip stays.
    mel(&mut c, "setAttr follicleShape1.pointLock 3; currentTime 1; currentTime 40;");
    assert!((y_of(&mut c, "curve1.cv[3]") - 3.0).abs() < 1e-6);
    // Off: the start curve.
    mel(&mut c, "setAttr follicleShape1.pointLock 1; setAttr hairSystemShape1.active 0; currentTime 41;");
    assert!((y_of(&mut c, "curve1.cv[3]") - 3.0).abs() < 1e-6);
}

#[test]
fn hair_saves_and_opens() {
    let dir = std::env::temp_dir().join(format!("az_hair_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("h.ma").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, "curve -d 3 -p 0 0 0 -p 0.1 1 0 -p 0.2 2 0 -p 0.3 3 0 -n strand; select strand; makeCurvesDynamic 2 { \"0\", \"0\", \"1\", \"1\", \"0\"};");
    mel(&mut c, "setAttr hairSystemShape1.gravity 20;");
    mel(&mut c, &format!("file -rename \"{p}\"; file -save -type \"mayaAscii\";"));
    mel(&mut c, "file -f -new;");
    mel(&mut c, &format!("file -f -open \"{p}\";"));
    assert_eq!(mel(&mut c, "nodeType hairSystemShape1;"), json!("hairSystem"));
    assert_eq!(mel(&mut c, "getAttr hairSystemShape1.gravity;"), json!(20.0));
    mel(&mut c, "currentTime 1; currentTime 30;");
    assert!(y_of(&mut c, "curve1.cv[3]") < 2.0);
    let _ = std::fs::remove_dir_all(&dir);
}
