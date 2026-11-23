//! Issue 65: Freeze Transformations keeps a mesh's history, leaving a transformGeometry on top as the reference does.

use aztek_host::Core;
use serde_json::json;

#[test]
fn freezing_a_cube_keeps_its_poly_cube() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a -w 2; move 5 0 0 a; scale 1 3 1 a;").unwrap();
    let before = c.mel_checked("pointPosition -w a.vtx[2];").unwrap();
    c.mel_checked("makeIdentity -a 1 a;").unwrap();
    let hist = c.mel_checked("listHistory a;").unwrap();
    let names: Vec<&str> = hist.as_array().unwrap().iter().filter_map(|v| v.as_str()).collect();
    assert!(names.iter().any(|n| n.starts_with("transformGeometry")) && names.contains(&"polyCube1"), "{hist}");
    assert_eq!(c.mel_checked("getAttr a.translate;").unwrap(), json!([0.0, 0.0, 0.0]));
    assert_eq!(c.mel_checked("pointPosition -w a.vtx[2];").unwrap(), before);
    // The cube's settings still work, under the freeze: twice the width, twice as far from the middle.
    c.mel_checked("setAttr polyCube1.width 4;").unwrap();
    let p = c.mel_checked("pointPosition -w a.vtx[2];").unwrap();
    let x0 = before[0].as_f64().unwrap();
    assert!((p[0].as_f64().unwrap() - (5.0 + 2.0 * (x0 - 5.0))).abs() < 1e-6, "{p}");
    c.mel_checked("undo; undo;").unwrap();
    assert_eq!(c.mel_checked("getAttr a.translateX;").unwrap(), json!(5.0));
    let hist = c.mel_checked("listHistory a;").unwrap();
    assert!(!hist.to_string().contains("transformGeometry"), "{hist}");
    // Without history the points take the move, as before.
    c.mel_checked("polyCube -n b; move 1 0 0 b; delete -ch b; makeIdentity -a 1 b;").unwrap();
    assert!(!c.mel_checked("listHistory b;").unwrap().to_string().contains("transformGeometry"));
    let p = c.mel_checked("pointPosition -w b.vtx[0];").unwrap();
    assert!((p[0].as_f64().unwrap() - 0.5).abs() < 1e-6, "{p}");
    assert_eq!(c.mel_checked("getAttr b.translateX;").unwrap(), json!(0.0));
}

#[test]
fn the_freeze_survives_save_and_open() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; rotate 0 0 90 a; makeIdentity -a 1 a;").unwrap();
    let before = c.mel_checked("pointPosition -w a.vtx[2];").unwrap();
    let path = std::env::temp_dir().join(format!("aztek_freeze_{}.ma", std::process::id())).display().to_string().replace('\\', "/");
    c.mel_checked(&format!("file -rename \"{path}\"; file -save -type \"mayaAscii\";")).unwrap();
    let mut d = Core::new();
    d.mel_checked(&format!("file -f -open \"{path}\";")).unwrap();
    assert_eq!(d.mel_checked("pointPosition -w a.vtx[2];").unwrap(), before);
    // The freeze is saved as the history it is, as the reference keeps it (#209).
    assert!(d.mel_checked("listHistory a;").unwrap().to_string().contains("transformGeometry"));
    std::fs::remove_file(path).ok();
}

#[test]
fn freezing_a_circle_keeps_its_history() {
    let mut c = Core::new();
    c.mel_checked("circle -n k; move 3 0 0 k;").unwrap();
    let before = c.mel_checked("pointPosition -w k.cv[0];").unwrap();
    c.mel_checked("makeIdentity -a 1 k;").unwrap();
    assert_eq!(c.mel_checked("pointPosition -w k.cv[0];").unwrap(), before);
    assert!(c.mel_checked("listHistory k;").unwrap().to_string().contains("makeNurbCircle1"));
    c.mel_checked("setAttr makeNurbCircle1.radius 2;").unwrap();
    let after = c.mel_checked("pointPosition -w k.cv[0];").unwrap();
    let (x0, x1) = (before[0].as_f64().unwrap(), after[0].as_f64().unwrap());
    assert!((x1 - (3.0 + 2.0 * (x0 - 3.0))).abs() < 1e-6, "{after}");
}
