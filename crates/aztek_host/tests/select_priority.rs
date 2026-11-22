//! A marquee over several kinds of object takes only the highest in selection priority, as the reference's; joints
//! pick on their bones, and a marquee takes a chain by its top joint.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{gather, Viewport};
use aztek_host::Core;
use glam::{DVec2, DVec3};

// The selection masks are the UI's, shared by every Core in the process: one test at a time, or a mask one test turns
// off is off for the other.
static MASKS: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn view() -> Viewport {
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = DVec3::new(0.0, 2.0, 0.0);
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 0.0;
    vp.cam.distance = 30.0;
    vp
}

fn scene(c: &mut Core) {
    c.mel(
        "select -cl; joint -p 0 0 0; joint -p 0 4 1; joint -p 0 8 0; ikHandle -sj joint1 -ee joint3;\
         curve -d 1 -p 3 0 0 -p 3 6 0; polySphere -r 1; move -3 2 0; select -cl;",
    );
    c.s.run("getAttr", serde_json::json!({"args": ["joint3.worldMatrix"]})).ok();
}

fn marquee(c: &mut Core, vp: &Viewport) -> Vec<String> {
    let items = gather(&mut c.s.graph);
    vp.pick_rect(&c.s, &items, DVec2::new(0.0, 0.0), DVec2::new(800.0, 800.0))
}

#[test]
fn a_marquee_takes_the_highest_priority_kind_and_a_joint_chain_by_its_top() {
    let _one = MASKS.lock().unwrap_or_else(|e| e.into_inner());
    let mut c = Core::new();
    scene(&mut c);
    let vp = view();
    assert_eq!(marquee(&mut c, &vp), vec!["ikHandle1".to_string()]);
    c.mel("selectType -ikHandle 0;");
    assert_eq!(marquee(&mut c, &vp), vec!["joint1".to_string()]);
    c.mel("selectType -joint 0;");
    assert_eq!(marquee(&mut c, &vp), vec!["curve1".to_string()]);
    c.mel("selectType -nurbsCurve 0;");
    assert_eq!(marquee(&mut c, &vp), vec!["pSphere1".to_string()]);
}

#[test]
fn a_click_inside_a_bone_takes_its_joint() {
    let _one = MASKS.lock().unwrap_or_else(|e| e.into_inner());
    let mut c = Core::new();
    c.mel("selectType -allObjects 1;");
    c.mel("select -cl; joint -p 0 0 0 -rad 3; joint -p 0 8 0; select -cl;");
    c.s.run("getAttr", serde_json::json!({"args": ["joint2.worldMatrix"]})).ok();
    let mut vp = view();
    vp.cam.distance = 20.0;
    let items = gather(&mut c.s.graph);
    // Halfway along the bone, between its edges: the body of the bone, not a line of it.
    let p = vp.screen().project(DVec3::new(0.0, 4.0, 0.0)).unwrap();
    vp.click_box = 1.0;
    assert_eq!(vp.pick(&c.s, &items, p), vec!["joint1".to_string()]);
    // The middle of a ball picks too.
    let q = vp.screen().project(DVec3::new(0.0, 8.0, 0.0)).unwrap();
    assert_eq!(vp.pick(&c.s, &items, q), vec!["joint2".to_string()]);
}

