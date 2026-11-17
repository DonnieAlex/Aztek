//! Constraints in the hierarchy (#120) with the reference's attribute sets (#121): a transform under the object it
//! drives, with only its own kind's settings shown.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn a_constraint_lives_under_what_it_drives() {
    let mut c = Core::new();
    mel(&mut c, "spaceLocator -n target; setAttr target.t 1 2 3; polyCube -n box;");
    let made = mel(&mut c, "pointConstraint target box;");
    assert_eq!(made, json!(["box_pointConstraint1"]));
    assert_eq!(mel(&mut c, "listRelatives -p box_pointConstraint1;"), json!(["box"]));
    assert_eq!(mel(&mut c, "listRelatives -type pointConstraint box;"), json!(["box_pointConstraint1"]));
    assert_eq!(mel(&mut c, "objectType -isAType transform box_pointConstraint1;"), json!(true));
    assert_eq!(mel(&mut c, "objectType -isAType constraint box_pointConstraint1;"), json!(true));
    assert_eq!(mel(&mut c, "listRelatives -type constraint box;"), json!(["box_pointConstraint1"]));
    assert!(c.mel_checked("createNode constraint;").is_err(), "nothing is made of the abstract type");
    // It still drives its parent, and follows it when that moves on.
    assert_eq!(mel(&mut c, "getAttr box.t;"), json!([1.0, 2.0, 3.0]));
    mel(&mut c, "setAttr target.tx 5;");
    assert_eq!(mel(&mut c, "getAttr box.tx;"), json!(5.0));
    // The transform's attributes, and the kind's own settings: a point constraint has an offset, no aim vector.
    assert_eq!(mel(&mut c, "attributeQuery -n box_pointConstraint1 -ex rotatePivot;"), json!(true));
    assert_eq!(mel(&mut c, "attributeQuery -n box_pointConstraint1 -h offset;"), json!(false));
    assert_eq!(mel(&mut c, "attributeQuery -n box_pointConstraint1 -h aimVector;"), json!(true));
    // Lock Output holds the result.
    mel(&mut c, "setAttr box_pointConstraint1.lockOutput 1; setAttr target.tx 7;");
    assert_eq!(mel(&mut c, "getAttr box.tx;"), json!(5.0));
    mel(&mut c, "setAttr box_pointConstraint1.lockOutput 0;");
    assert_eq!(mel(&mut c, "getAttr box.tx;"), json!(7.0));
    // A target weight driven by another attribute still counts the same pass.
    mel(&mut c, "spaceLocator -n other; setAttr other.t 0 0 0; pointConstraint other box; addAttr -ln blend -at double -dv 0 box; connectAttr box.blend box_pointConstraint1.otherW1; setAttr box_pointConstraint1.targetW0 0;");
    mel(&mut c, "setAttr box.blend 1; setAttr box_pointConstraint1.targetW0 0;");
    assert_eq!(mel(&mut c, "getAttr box.tx;"), json!(0.0));
    // Saved and opened, it is where it was; deleting the object takes it along.
    let p = std::env::temp_dir().join(format!("az_cdag_{}.ma", std::process::id())).to_string_lossy().replace('\\', "/");
    mel(&mut c, &format!("file -rename \"{p}\"; file -save -type \"mayaAscii\"; file -f -new; file -f -open \"{p}\";"));
    assert_eq!(mel(&mut c, "listRelatives -p box_pointConstraint1;"), json!(["box"]));
    assert_eq!(mel(&mut c, "ls -type pointConstraint;"), json!(["box_pointConstraint1"]));
    mel(&mut c, "delete box;");
    assert_eq!(mel(&mut c, "ls -type pointConstraint;"), json!([]));
    let _ = std::fs::remove_file(&p);
}
