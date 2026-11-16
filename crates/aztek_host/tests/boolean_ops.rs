//! Mesh > Booleans: Difference (B - A), Slice, Hole Punch, Cut Out and Split Edges, as polyCBoolOp -op 4 to 8.

use aztek_host::Core;
use serde_json::Value;

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn count(c: &mut Core, flag: &str, obj: &str) -> u64 {
    mel(c, &format!("polyEvaluate -{flag} {obj};")).as_u64().unwrap()
}

fn result(c: &mut Core, op: i64) -> String {
    mel(c, "polyCube -w 2 -h 2 -d 2 -n a; polyCube -n b; move 1 0 0 b;");
    let r = mel(c, &format!("polyCBoolOp -op {op} a b;"));
    r[0].as_str().unwrap().to_string()
}

#[test]
fn boolean_ops() {
    // Hole punch: the cube with the cutter's corner region gone and no walls of the cutter: open.
    let mut c = Core::new();
    let r = result(&mut c, 7);
    let border = mel(&mut c, &format!("polyListComponentConversion -te -bo {r};"));
    assert!(!border.as_array().unwrap().is_empty(), "hole punch is open");
    assert_eq!(count(&mut c, "s", &r), 1);

    // Cut out: only the first cube's surface inside the cutter, open: the patch of its +X side the cutter covers.
    let mut c = Core::new();
    let r = result(&mut c, 8);
    assert_eq!(count(&mut c, "s", &r), 1);
    let bb = mel(&mut c, &format!("exactWorldBoundingBox {r};"));
    assert!((bb[0].as_f64().unwrap() - 1.0).abs() < 1e-4 && (bb[1].as_f64().unwrap() + 0.5).abs() < 1e-4, "{bb}");

    // Slice: both parts, two shells.
    let mut c = Core::new();
    let r = result(&mut c, 5);
    assert_eq!(count(&mut c, "s", &r), 2);

    // Split edges: one closed shell, more faces than the cube.
    let mut c = Core::new();
    let r = result(&mut c, 6);
    assert_eq!(count(&mut c, "s", &r), 1);
    assert!(count(&mut c, "f", &r) > 6);
    let bb = mel(&mut c, &format!("exactWorldBoundingBox {r};"));
    assert!((bb[3].as_f64().unwrap() - 1.0).abs() < 1e-4, "{bb}");

    // B - A: the small cube minus the big one, from 1 to 1.5 in X.
    let mut c = Core::new();
    let r = result(&mut c, 4);
    let bb = mel(&mut c, &format!("exactWorldBoundingBox {r};"));
    assert!((bb[0].as_f64().unwrap() - 1.0).abs() < 1e-4 && (bb[3].as_f64().unwrap() - 1.5).abs() < 1e-4, "{bb}");
}
