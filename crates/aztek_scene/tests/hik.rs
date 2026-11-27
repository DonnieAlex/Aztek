//! Characters: skeleton, definition, control rig, retargeting.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn pos(s: &mut Session, j: &str) -> [f64; 3] {
    let v = run(s, "xform", json!({"args": [j], "query": true, "worldSpace": true, "translation": true}));
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap(), v[2].as_f64().unwrap()]
}

#[test]
fn skeleton_and_definition() {
    let mut s = Session::new();
    let j = run(&mut s, "hikCreateSkeleton", json!({"name": "Hero", "height": 170.0, "character": true}));
    assert!(j.as_array().unwrap().len() > 20);
    assert!((pos(&mut s, "Hero_Hips")[1] - 95.0).abs() < 1e-6);
    assert!((pos(&mut s, "Hero_LeftHand")[0] - 70.0).abs() < 1e-6);
    assert!((pos(&mut s, "Hero_RightHand")[0] + 70.0).abs() < 1e-6);
    assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Hero", 1]})), json!("Hero_Hips"));
    assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Hero", "LeftArm"]})), json!("Hero_LeftArm"));
}

#[test]
fn names_map_to_slots() {
    use aztek_scene::hik::slot_for_name as f;
    for (name, want) in [
        ("mixamorig:LeftArm", "LeftArm"),
        ("Character1_RightFoot", "RightFoot"),
        ("L_UpperArm", "LeftArm"),
        ("r_thigh", "RightUpLeg"),
        ("pelvis", "Hips"),
        ("spine_02", "Spine2"),
        ("Right_Shoulder", "RightShoulder"),
    ] {
        assert_eq!(f(name).as_deref(), Some(want), "{name}");
    }
    assert_eq!(f("prop_sword"), None);
}

#[test]
fn definition_from_a_named_skeleton() {
    let mut s = Session::new();
    run(&mut s, "joint", json!({"position": [0, 10, 0], "name": "pelvis"}));
    run(&mut s, "joint", json!({"position": [0, 12, 0], "name": "spine_01"}));
    run(&mut s, "hikCreateCharacter", json!({"args": ["Bob", "pelvis"]}));
    assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Bob", "Hips"]})), json!("pelvis"));
    run(&mut s, "joint", json!({"position": [0, 14, 0], "name": "oddName"}));
    run(&mut s, "setCharacterObject", json!({"args": ["oddName", "Bob", 15, 0]}));
    assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Bob", "Head"]})), json!("oddName"));
}

#[test]
fn control_rig_drives_the_skeleton() {
    let mut s = Session::new();
    run(&mut s, "hikCreateSkeleton", json!({"name": "Hero", "character": true}));
    let rest = pos(&mut s, "Hero_LeftForeArm");
    let made = run(&mut s, "hikCreateControlRig", json!({"args": ["Hero"]}));
    assert!(made.as_array().unwrap().iter().any(|x| x == "Hero_Ctrl_LeftHandEffector"));
    let near = |a: [f64; 3], b: [f64; 3], t: f64| (0..3).all(|i| (a[i] - b[i]).abs() < t);
    // Building it moves nothing.
    assert!(near(pos(&mut s, "Hero_LeftForeArm"), rest, 1e-3), "{:?} {rest:?}", pos(&mut s, "Hero_LeftForeArm"));
    // The hand effector pulls the hand.
    let h = pos(&mut s, "Hero_LeftHand");
    run(&mut s, "move", json!({"args": ["Hero_Ctrl_LeftHandEffector"], "to": [h[0] - 15.0, h[1] + 15.0, h[2]]}));
    let p = pos(&mut s, "Hero_LeftHand");
    assert!(near(p, [h[0] - 15.0, h[1] + 15.0, h[2]], 1e-2), "{p:?}");
    // The hips control moves the hips, and the feet stay.
    let foot = pos(&mut s, "Hero_LeftFoot");
    run(&mut s, "move", json!({"args": ["Hero_Ctrl_Hips"], "by": [0, -10, 0]}));
    assert!((pos(&mut s, "Hero_Hips")[1] - 85.0).abs() < 1e-2, "{:?}", pos(&mut s, "Hero_Hips"));
    assert!(near(pos(&mut s, "Hero_LeftFoot"), foot, 1e-2));
    run(&mut s, "hikDeleteControlRig", json!({"args": ["Hero"]}));
    assert_eq!(run(&mut s, "ls", json!({"args": ["Hero_Ctrl_*"]})), json!([]));
}

#[test]
fn retarget_and_bake() {
    let mut s = Session::new();
    run(&mut s, "hikCreateSkeleton", json!({"name": "Src", "character": true}));
    run(&mut s, "hikCreateSkeleton", json!({"name": "Dst", "character": true}));
    assert!(run(&mut s, "hikSetSource", json!({"args": ["Dst", "Src"]})).as_i64().unwrap() > 10);
    run(&mut s, "setKeyframe", json!({"args": ["Src_LeftArm.rz"], "time": 1, "value": 0.0}));
    run(&mut s, "setKeyframe", json!({"args": ["Src_LeftArm.rz"], "time": 10, "value": 60.0}));
    run(&mut s, "currentTime", json!({"args": [10]}));
    let (a, b) = (pos(&mut s, "Src_LeftHand"), pos(&mut s, "Dst_LeftHand"));
    assert!((a[1] - b[1]).abs() < 1e-3, "{a:?} {b:?}");
    assert!(b[1] > 150.0, "{b:?}");
    run(&mut s, "hikBake", json!({"args": ["Dst"], "time": "1:10"}));
    run(&mut s, "cutKey", json!({"args": ["Src_LeftArm.rz"], "clear": true}));
    run(&mut s, "currentTime", json!({"args": [10]}));
    assert!(pos(&mut s, "Dst_LeftHand")[1] > 150.0, "{:?}", pos(&mut s, "Dst_LeftHand"));
}

/// The Skeleton tab's choices (#168): neck and shoulder joints, roll bones, which fingers and toes and how many bones
/// each, the base and in-hand bones; each in its slot of the definition.
#[test]
fn skeleton_tab_choices() {
    let mut s = Session::new();
    let call = json!({"name": "Hero", "character": true, "characterScale": 2.0, "spineCount": 4, "neckCount": 2, "shoulderCount": 2,
        "upperArmRolls": 2, "lowerArmRolls": 1, "upperLegRolls": 3, "lowerLegRolls": 0, "thumb": true, "index": true, "middle": false,
        "ring": false, "pinky": true, "fingerBones": 2, "fingerBase": true, "inHandBones": true, "footThumb": true, "toeBones": 1, "toeBase": true});
    run(&mut s, "hikCreateSkeleton", call);
    assert!((pos(&mut s, "Hero_Hips")[1] - 190.0).abs() < 1e-6);
    for (slot, joint) in [
        ("Spine3", "Hero_Spine3"), ("Neck1", "Hero_Neck1"), ("LeftShoulderExtra", "Hero_LeftShoulderExtra"),
        ("LeafLeftArmRoll2", "Hero_LeafLeftArmRoll2"), ("LeafRightForeArmRoll1", "Hero_LeafRightForeArmRoll1"), ("LeafLeftUpLegRoll3", "Hero_LeafLeftUpLegRoll3"),
        ("LeftHandPinky2", "Hero_LeftHandPinky2"), ("LeftFingerBase", "Hero_LeftFingerBase"), ("RightInHandThumb", "Hero_RightInHandThumb"),
        ("LeftFootThumb1", "Hero_LeftFootThumb1"), ("RightToeBase", "Hero_RightToeBase"),
    ] {
        assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Hero", slot]})), json!(joint), "{slot}");
    }
    for missing in ["Hero_LeftHandMiddle1", "Hero_LeftHandPinky3", "Hero_LeafLeftLegRoll1", "Hero_LeftFootThumb2"] {
        assert_eq!(run(&mut s, "objExists", json!({"args": [missing]})), json!(false), "{missing}");
    }
}

/// The calls mGear's Humanoid IK mapper makes: a source set and read back by name, a joint taken out of the definition,
/// and the bake bracketed by its pre and post steps.
#[test]
fn mapper_calls() {
    let mut s = Session::new();
    run(&mut s, "hikCreateSkeleton", json!({"name": "Src", "character": true}));
    run(&mut s, "hikCreateSkeleton", json!({"name": "Dst", "character": true}));
    assert_eq!(run(&mut s, "hikGetCharacterInputString", json!({"args": ["Dst"]})), json!("None"));
    run(&mut s, "mayaHIKsetCharacterInput", json!({"args": ["Dst", "Src"]}));
    assert_eq!(run(&mut s, "hikGetCharacterInputString", json!({"args": ["Dst"]})), json!("Src"));
    run(&mut s, "hikSetCharacterInput", json!({"args": ["Dst", ""]}));
    assert_eq!(run(&mut s, "hikGetCharacterInputString", json!({"args": ["Dst"]})), json!("None"));
    run(&mut s, "hikRemoveSkFromCharacter", json!({"args": ["Dst", 15]}));
    assert_eq!(run(&mut s, "hikGetSkNode", json!({"args": ["Dst", "Head"]})), json!(""));
    run(&mut s, "hikReadStancePoseTRSOffsets", json!({"args": ["Dst"]}));
    let picked = run(&mut s, "hikBakeCharacterPre", json!({"args": ["Dst"]}));
    assert!(picked.as_array().unwrap().iter().any(|v| v == "Dst_Hips"));
    assert_eq!(run(&mut s, "ls", json!({"selection": true})).as_array().unwrap().len(), picked.as_array().unwrap().len());
    run(&mut s, "hikBakeCharacterPost", json!({"args": ["Dst"]}));
    assert!(s.run("hikGetCharacterInputString", json!({"args": ["Nobody"]})).is_err());
}
