//! Mesh > Unsmooth (polyUnsmooth): smoothing undone, live in the history.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn unsmooth_levels() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -w 2 -h 2 -d 2 -n box; polySmooth -dv 2 box;");
    assert_eq!(mel(&mut c, "polyEvaluate -f box;"), json!(96));
    let node = mel(&mut c, "polyUnsmooth -dlv 1 box;");
    let node = node.as_array().map(|a| a[0].clone()).unwrap_or(node).as_str().unwrap().to_string();
    assert_eq!(mel(&mut c, "polyEvaluate -f box;"), json!(24));
    mel(&mut c, &format!("setAttr {node}.divisionLevels 2;"));
    assert_eq!(mel(&mut c, "polyEvaluate -f box;"), json!(6));
    let bb = mel(&mut c, "exactWorldBoundingBox box;");
    assert!((bb[3].as_f64().unwrap() - 1.0).abs() < 1e-5, "{bb}");
    // Not a smoothed mesh: refused.
    mel(&mut c, "polyCylinder -n cyl;");
    assert!(c.mel_checked("polyUnsmooth cyl;").is_err());
}
