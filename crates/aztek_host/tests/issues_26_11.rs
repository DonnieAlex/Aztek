//! Issues of 26/11/2026.

use aztek_host::tools::ik_spline_click;
use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    let v = c.mel(src);
    let errors: Vec<String> = c.take_log().as_array().unwrap().iter().filter(|l| l["kind"] == 2).map(|l| l["text"].as_str().unwrap().to_string()).collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    v
}

fn spine(c: &mut Core) {
    mel(c, "select -cl;");
    for (i, y) in [0.0, 2.0, 4.0, 6.0].iter().enumerate() {
        mel(c, &format!("joint -p 0 {y} {} -n spine{};", if i == 2 { 0.5 } else { 0.0 }, i + 1));
    }
    mel(c, "select -cl;");
}

#[test]
fn the_spline_handle_tool_makes_the_handle_and_its_curve_on_the_end_joint_click() {
    // #317: click the start joint, then the end joint, as in the reference.
    let mut c = Core::new();
    spine(&mut c);
    let mut picked = Vec::new();
    assert!(ik_spline_click(&mut c, &mut picked, Some("spine1".into()), None));
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["spine1"]));
    assert_eq!(mel(&mut c, "ls -type ikHandle;"), json!([]));
    assert!(ik_spline_click(&mut c, &mut picked, Some("spine4".into()), None));
    assert!(picked.is_empty(), "the tool is ready for the next handle");
    assert_eq!(mel(&mut c, "ls -type ikHandle;"), json!(["ikHandle1"]));
    assert_eq!(mel(&mut c, "ikHandle -q -sj ikHandle1;"), json!("spine1"));
    let curve = mel(&mut c, "ikHandle -q -c ikHandle1;");
    assert!(curve.as_str().is_some_and(|s| !s.is_empty()), "a curve was made: {curve}");
    // The tool's Number of spans: 1 by default, a curve of 4 CVs.
    let shape = curve.as_str().unwrap().to_string();
    assert_eq!(mel(&mut c, &format!("getAttr {shape}.spans;")), json!(1));
}

#[test]
fn with_auto_create_curve_off_the_next_click_picks_the_curve() {
    // #317: Auto create curve off: start joint, end joint, then the curve to follow.
    let mut c = Core::new();
    spine(&mut c);
    mel(&mut c, "curve -n path -d 3 -p 0 0 0 -p 1 2 0 -p 1 4 0 -p 0 6 0;");
    mel(&mut c, "ikSplineHandleCtx -e -ccv false ikSplineHandleContext;");
    let mut picked = Vec::new();
    ik_spline_click(&mut c, &mut picked, Some("spine1".into()), None);
    ik_spline_click(&mut c, &mut picked, Some("spine4".into()), None);
    assert_eq!(mel(&mut c, "ls -type ikHandle;"), json!([]));
    // A click on nothing waits for the curve.
    assert!(!ik_spline_click(&mut c, &mut picked, None, None));
    assert!(ik_spline_click(&mut c, &mut picked, None, Some("path".into())));
    assert_eq!(mel(&mut c, "ls -type ikHandle;"), json!(["ikHandle1"]));
    let followed = mel(&mut c, "ikHandle -q -c ikHandle1;");
    assert!(followed.as_str().is_some_and(|s| s.contains("path")), "{followed}");
}

#[test]
fn the_spline_tool_settings_have_the_references_defaults() {
    // #317: the IK Spline Handle Tool's settings, as ikSplineHandleCtx documents them.
    let mut c = Core::new();
    for (flag, want) in [("roc", json!(true)), ("cra", json!(false)), ("pcv", json!(true)), ("snc", json!(false)), ("ccv", json!(true)), ("scv", json!(true)), ("ns", json!(1)), ("rtm", json!(false)), ("tws", json!("linear"))] {
        assert_eq!(mel(&mut c, &format!("ikSplineHandleCtx -q -{flag} ikSplineHandleContext;")), want, "{flag}");
    }
}
