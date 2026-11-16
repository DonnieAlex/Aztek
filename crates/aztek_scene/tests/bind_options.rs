//! The Bind Skin options: hidden joints and geometry left out unless asked, unused influences not bound, and a
//! skeleton out of its bind pose refused unless multiple bind poses are allowed.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, cmd: &str, a: Value) -> Value {
    s.run(cmd, a).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn rig(s: &mut Session) {
    run(s, "polyCube", json!({"height": 4.0, "subdivisionsHeight": 4}));
    run(s, "select", json!({"clear": true}));
    run(s, "joint", json!({"position": [0.0, -2.0, 0.0]}));
    run(s, "joint", json!({"position": [0.0, 0.0, 0.0]}));
    run(s, "joint", json!({"position": [0.0, 2.0, 0.0]}));
    // A joint far away: it gets no weight.
    run(s, "joint", json!({"position": [0.0, 200.0, 0.0]}));
}

fn influences(s: &mut Session) -> Vec<String> {
    serde_json::from_value(run(s, "skinCluster", json!({"args": ["skinCluster1"], "query": true, "influence": true}))).unwrap()
}

#[test]
fn hidden_joints_and_unused_influences_are_left_out() {
    let mut s = Session::new();
    rig(&mut s);
    run(&mut s, "setAttr", json!({"args": ["joint3.visibility", false]}));
    run(&mut s, "skinCluster", json!({"args": ["joint1", "pCube1"], "maximumInfluences": 3, "removeUnusedInfluence": true}));
    let inf = influences(&mut s);
    assert!(inf.iter().any(|j| j.ends_with("joint1")) && inf.iter().any(|j| j.ends_with("joint2")), "{inf:?}");
    // joint3 hidden (and joint4 under it), so neither is bound.
    assert!(!inf.iter().any(|j| j.ends_with("joint3") || j.ends_with("joint4")), "{inf:?}");
}

#[test]
fn hidden_selections_are_bound_when_asked_and_unused_ones_dropped() {
    let mut s = Session::new();
    rig(&mut s);
    run(&mut s, "skinCluster", json!({"args": ["joint1", "pCube1"], "includeHiddenSelections": true, "removeUnusedInfluence": true}));
    let inf = influences(&mut s);
    assert!(!inf.iter().any(|j| j.ends_with("joint4")), "{inf:?}");
    assert_eq!(inf.len(), 3, "{inf:?}");
}

#[test]
fn out_of_its_bind_pose_a_skeleton_binds_only_with_multiple_poses_allowed() {
    let mut s = Session::new();
    rig(&mut s);
    run(&mut s, "skinCluster", json!({"args": ["joint1", "pCube1"]}));
    run(&mut s, "polySphere", json!({}));
    run(&mut s, "setAttr", json!({"args": ["joint2.rotateZ", 30.0]}));
    let refused = s.run("skinCluster", json!({"args": ["joint1", "pSphere1"], "multipleBindPose": false}));
    assert!(refused.is_err());
    run(&mut s, "skinCluster", json!({"args": ["joint1", "pSphere1"], "multipleBindPose": true}));
    let poses: Vec<String> = serde_json::from_value(run(&mut s, "ls", json!({"type": "dagPose"}))).unwrap();
    assert_eq!(poses.len(), 2, "{poses:?}");
    // Back in the first pose, binding a third object is allowed with the option off.
    run(&mut s, "setAttr", json!({"args": ["joint2.rotateZ", 0.0]}));
    run(&mut s, "polyCylinder", json!({}));
    run(&mut s, "skinCluster", json!({"args": ["joint1", "pCylinder1"], "multipleBindPose": false}));
}
