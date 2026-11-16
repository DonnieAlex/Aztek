//! Key > IK/FK Keys: Set IK/FK Key, Enable IK Solver and Move IK to FK, as the runtime commands run them.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

pub const SET_IKFK_KEY: &str = "{ string $h[] = `ls -sl -type ikHandle`; if (!size($h)) error \"Select an IK handle\"; for ($x in $h) { setKeyframe $x; setKeyframe -at ikBlend $x; setKeyframe -at rotate `ikHandle -q -jl $x`; } }";
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

#[test]
fn fk_keys_survive_a_handle_and_play_where_ik_is_off() {
    let mut c = Core::new();
    mel(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 2 0 1 -n j2; joint -p 4 0 0 -n j3; select -cl;");
    mel(&mut c, "currentTime 1; setKeyframe -at rotateY -v 0 j1; currentTime 11; setKeyframe -at rotateY -v 40 j1;");
    let h = mel(&mut c, "ikHandle -sj j1 -ee j3 -n arm;")[0].as_str().unwrap().to_string();
    // The keys stay with the joint: queried on it, and playing where IK is off.
    assert_eq!(mel(&mut c, "keyframe -q -kc j1.rotateY;"), json!(2));
    mel(&mut c, &format!("setAttr {h}.ikBlend 0; currentTime 6;"));
    let ry = mel(&mut c, "getAttr j1.rotateY;").as_f64().unwrap();
    assert!(ry > 5.0 && ry < 35.0, "FK keys play: {ry}");
    // A key set on a joint under IK lands on its FK rotate, at what it shows.
    mel(&mut c, "currentTime 20; setKeyframe -at rotateZ -v 15 j2;");
    assert_eq!(mel(&mut c, "keyframe -q -kc j2.rotateZ;"), json!(1));
    assert!((mel(&mut c, "getAttr j2.rotateZ;").as_f64().unwrap() - 15.0).abs() < 1e-6);
    // Set IK/FK Key keys the joints too now.
    mel(&mut c, &format!("setAttr {h}.ikBlend 1; currentTime 30; select {h};"));
    mel(&mut c, SET_IKFK_KEY);
    assert_eq!(mel(&mut c, "keyframe -q -kc j2.rotateX;"), json!(1));
}

#[test]
fn fk_keys_under_ik_save_on_the_joint() {
    let tmp = std::env::temp_dir().join(format!("aztek_ikfk_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let mut c = Core::new();
    mel(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 2 0 1 -n j2; joint -p 4 0 0 -n j3; select -cl;");
    mel(&mut c, "currentTime 1; setKeyframe -at rotateY -v 0 j1; currentTime 11; setKeyframe -at rotateY -v 40 j1;");
    mel(&mut c, "ikHandle -sj j1 -ee j3 -n arm; setAttr arm.ikBlend 0;");
    for (ext, ty) in [("ma", "mayaAscii"), ("mb", "mayaBinary")] {
        let f = tmp.join(format!("t.{ext}")).to_string_lossy().replace('\\', "/");
        mel(&mut c, &format!("file -rename \"{f}\"; file -save -type \"{ty}\";"));
        if ext == "ma" {
            let text = std::fs::read_to_string(&f).unwrap();
            assert!(text.contains("j1_rotateY.o\" \"|j1.rotateY\"") || text.contains("j1_rotateY.o\" \"j1.rotateY\"") || text.contains("j1_rotateY.o\" \"|j1.ry\""), "the key is written on the joint");
        }
        let mut d = Core::new();
        mel(&mut d, &format!("file -open -force \"{f}\";"));
        assert_eq!(mel(&mut d, "keyframe -q -kc j1.rotateY;"), json!(2), "{ext}");
        mel(&mut d, "currentTime 11;");
        let ry = mel(&mut d, "getAttr j1.rotateY;").as_f64().unwrap();
        assert!((ry - 40.0).abs() < 1e-4, "{ext}: {ry}");
    }
}
