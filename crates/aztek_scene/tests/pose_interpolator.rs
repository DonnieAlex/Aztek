//! poseInterpolator: the node, its command and Create Pose Interpolator.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn w(s: &mut Session, sh: &str, k: i32) -> f64 {
    run(s, "getAttr", json!({"args": [format!("{sh}.output[{k}]")]})).as_f64().unwrap()
}

fn elbow(s: &mut Session) -> String {
    run(s, "joint", json!({"name": "shoulder", "position": [0.0, 0.0, 0.0]}));
    run(s, "joint", json!({"name": "elbow", "position": [4.0, 0.0, 0.0]}));
    run(s, "joint", json!({"name": "wrist", "position": [8.0, 0.0, 0.0]}));
    let made = run(s, "createPoseInterpolator", json!({"args": ["elbow"]}));
    made[0].as_str().unwrap().to_string()
}

#[test]
fn neutral_poses_and_a_bent_pose() {
    let mut s = Session::new();
    let sh = elbow(&mut s);
    assert!(sh.ends_with("elbow_poseInterpolatorShape"), "{sh}");
    assert_eq!(run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "query": true, "poseNames": true})), json!(["neutral", "neutralSwing", "neutralTwist"]));
    assert_eq!(run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "query": true, "drivers": true})), json!(["elbow"]));
    // Bent 90 degrees about Z: a swing of the X axis, as a swing pose.
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 90.0]}));
    let k = run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "addPose": "bend", "poseType": 1})).as_i64().unwrap() as i32;
    assert_eq!(k, 3);
    // At the pose: 1, and 0 at rest.
    assert!((w(&mut s, &sh, 3) - 1.0).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 0.0]}));
    assert!(w(&mut s, &sh, 3).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    assert!((w(&mut s, &sh, 0) - 1.0).abs() < 1e-6);
    // Half way: in between, rising with the bend.
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 45.0]}));
    let half = w(&mut s, &sh, 3);
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 70.0]}));
    let more = w(&mut s, &sh, 3);
    assert!(half > 0.05 && half < 0.95 && more > half, "{half} {more}");
    // A twist about X alone leaves a swing pose at 0.
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 0.0]}));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateX", 60.0]}));
    assert!(w(&mut s, &sh, 3).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    // Bent half way while twisted: the swing pose sees the bend only.
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 45.0]}));
    assert!((w(&mut s, &sh, 3) - half).abs() < 1e-6, "{} {half}", w(&mut s, &sh, 3));
    // Go to the pose sets the joint.
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "goToPose": "bend"}));
    assert!((w(&mut s, &sh, 3) - 1.0).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    let rz = run(&mut s, "getAttr", json!({"args": ["elbow.rotateZ"]})).as_f64().unwrap();
    assert!((rz - 90.0).abs() < 1e-4, "{rz}");
}

#[test]
fn independent_gaussian_and_editing() {
    let mut s = Session::new();
    let sh = elbow(&mut s);
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateY", -60.0]}));
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "addPose": "up"}));
    run(&mut s, "setAttr", json!({"args": [format!("{sh}.pose[3].isIndependent"), true]}));
    run(&mut s, "setAttr", json!({"args": [format!("{sh}.pose[3].poseRotationFalloff"), 60.0]}));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateY", -30.0]}));
    assert!((w(&mut s, &sh, 3) - 0.5).abs() < 1e-4, "{}", w(&mut s, &sh, 3));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateY", 10.0]}));
    assert_eq!(w(&mut s, &sh, 3), 0.0);
    // Gaussian, solved: at the poses each is met.
    run(&mut s, "setAttr", json!({"args": [format!("{sh}.pose[3].isIndependent"), false]}));
    run(&mut s, "setAttr", json!({"args": [format!("{sh}.interpolation"), 1]}));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateY", -60.0]}));
    assert!((w(&mut s, &sh, 3) - 1.0).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    // Rename, update, index, delete.
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "rename": ["up", "raise"]}));
    assert_eq!(run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "query": true, "index": true, "pose": "raise"})), json!(3));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateY", -80.0]}));
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "updatePose": "raise"}));
    assert!((w(&mut s, &sh, 3) - 1.0).abs() < 1e-6);
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "deletePose": "raise"}));
    assert_eq!(run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "query": true, "poseNames": true})), json!(["neutral", "neutralSwing", "neutralTwist"]));
}

#[test]
fn a_default_name_and_saving() {
    let mut s = Session::new();
    let sh = elbow(&mut s);
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 90.0]}));
    run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "addPose": ""}));
    let names = run(&mut s, "poseInterpolator", json!({"args": [sh.clone()], "query": true, "poseNames": true}));
    assert_eq!(names[3], json!("elbow_0_0_90"));
    let dir = std::env::temp_dir().join(format!("aztek-pi-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("pi.ma").to_string_lossy().replace('\\', "/");
    run(&mut s, "file", json!({"rename": f.clone()}));
    run(&mut s, "file", json!({"save": true, "type": "mayaAscii", "force": true}));
    run(&mut s, "file", json!({"args": [f], "open": true, "force": true}));
    let sh = "elbow_poseInterpolatorShape".to_string();
    assert!((w(&mut s, &sh, 3) - 1.0).abs() < 1e-6, "{}", w(&mut s, &sh, 3));
    run(&mut s, "setAttr", json!({"args": ["elbow.rotateZ", 0.0]}));
    assert!(w(&mut s, &sh, 3).abs() < 1e-6);
    assert_eq!(run(&mut s, "poseInterpolator", json!({"args": [sh], "query": true, "poseNames": true}))[3], json!("elbow_0_0_90"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// Typed arrays given an element at a time, as pose[i].poseRotation[driver]: hand-written, the layout assumed.
#[test]
fn reads_poses_given_by_driver() {
    let ma = r#"//Maya ASCII 2019 scene
requires maya "2019";
requires "poseInterpolator" "1.0";
currentUnit -l centimeter -a degree -t film;
createNode joint -n "elbow";
	setAttr ".r" -type "double3" 0 0 90 ;
createNode transform -n "elbow_poseInterpolator";
createNode poseInterpolator -n "elbow_poseInterpolatorShape" -p "elbow_poseInterpolator";
	setAttr -k off ".v";
	setAttr -s 2 ".pose";
	setAttr ".pose[0].poseName" -type "string" "neutral";
	setAttr ".pose[0].poseRotation[0]" -type "doubleArray" 4 0 0 0 1 ;
	setAttr ".pose[0].poseTranslation[0]" -type "doubleArray" 3 0 0 0 ;
	setAttr ".pose[1].poseName" -type "string" "bend";
	setAttr ".pose[1].poseRotation[0]" -type "doubleArray" 4 0 0 0.70710678 0.70710678 ;
	setAttr ".pose[1].poseTranslation[0]" -type "doubleArray" 3 0 0 0 ;
connectAttr "elbow.m" "elbow_poseInterpolatorShape.driver[0].driverMatrix";
"#;
    let dir = std::env::temp_dir().join(format!("aztek-pi-ref-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("ref.ma");
    std::fs::write(&f, ma).unwrap();
    let mut s = Session::new();
    run(&mut s, "file", json!({"args": [f.to_string_lossy().replace('\\', "/")], "open": true, "force": true}));
    let pr = run(&mut s, "getAttr", json!({"args": ["elbow_poseInterpolatorShape.pose[1].poseRotation"]}));
    assert_eq!(pr.as_array().unwrap().len(), 4);
    assert!((w(&mut s, "elbow_poseInterpolatorShape", 1) - 1.0).abs() < 1e-6);
    let _ = std::fs::remove_dir_all(&dir);
}
