//! Key > IK/FK Keys: Set IK/FK Key, Enable IK Solver and Move IK to FK, as the runtime commands run them.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

pub const SET_IKFK_KEY: &str = "{ string $h[] = `ls -sl -type ikHandle`; if (!size($h)) error \"Select an IK handle\"; for ($x in $h) { setKeyframe $x; setKeyframe -at ikBlend $x; } }";
pub const TOGGLE_SOLVER: &str = "{ string $h[] = `ls -sl -type ikHandle`; if (!size($h)) error \"Select an IK handle\"; for ($x in $h) setAttr ($x + \".solverEnable\") (!`getAttr ($x + \".solverEnable\")`); }";
pub const MOVE_IK_TO_FK: &str = "{ string $h[] = `ls -sl -type ikHandle`; if (!size($h)) error \"Select an IK handle\"; for ($x in $h) ikHandle -e -see $x; }";

#[test]
fn ikfk_keys() {
    let mut c = Core::new();
    mel(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 2 0 1 -n j2; joint -p 4 0 0 -n j3; select -cl;");
    let h = mel(&mut c, "ikHandle -sj j1 -ee j3 -n arm;")[0].as_str().unwrap().to_string();
    mel(&mut c, &format!("select {h}; currentTime 1;"));
    mel(&mut c, SET_IKFK_KEY);
    assert!(mel(&mut c, &format!("keyframe -q -kc {h}.translateX;")).as_i64().unwrap() >= 1);
    assert_eq!(mel(&mut c, &format!("keyframe -q -kc {h}.ikBlend;")), json!(1));
    mel(&mut c, TOGGLE_SOLVER);
    assert_eq!(mel(&mut c, &format!("getAttr {h}.solverEnable;")), json!(false));
    // FK: turn the root, then bring the handle to the chain's end.
    mel(&mut c, "setAttr j1.rotateY 30;");
    mel(&mut c, MOVE_IK_TO_FK);
    let hp = mel(&mut c, &format!("xform -q -ws -t {h};"));
    let jp = mel(&mut c, "xform -q -ws -t j3;");
    for i in 0..3 {
        assert!((hp[i].as_f64().unwrap() - jp[i].as_f64().unwrap()).abs() < 1e-4, "{hp} {jp}");
    }
    mel(&mut c, TOGGLE_SOLVER);
    assert_eq!(mel(&mut c, &format!("getAttr {h}.solverEnable;")), json!(true));
}
