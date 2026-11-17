//! Issues of 17/11/2026.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{gather, Viewport};
use aztek_host::Core;
use glam::{DVec2, DVec3};
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn top_view() -> Viewport {
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = DVec3::ZERO;
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 89.9;
    vp.cam.distance = 30.0;
    vp
}

/// #116: the live surface leaves the selection and can't be picked; what is under it can.
#[test]
fn a_live_surface_is_not_picked() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -n ground -w 20 -h 20; polyCube -n box; move 0 -5 0 box; select ground;");
    mel(&mut c, "makeLive;");
    assert_eq!(mel(&mut c, "makeLive -q;"), json!(["groundShape"]));
    assert_eq!(mel(&mut c, "ls -sl;"), json!([]));
    let mut vp = top_view();
    let items = gather(&mut c.s.graph);
    let at = vp.screen().project(DVec3::new(0.0, 0.0, 0.0)).unwrap();
    assert_eq!(vp.pick(&c.s, &items, at), vec!["box".to_string()]);
    let off: DVec2 = vp.screen().project(DVec3::new(8.0, 0.0, 8.0)).unwrap();
    assert!(vp.pick(&c.s, &items, off).is_empty());
    mel(&mut c, "makeLive -none;");
    assert_eq!(vp.pick(&c.s, &items, off), vec!["ground".to_string()]);
}

/// #114: Shift toggles, Ctrl removes, Ctrl+Shift adds, in objects and components.
#[test]
fn click_modifiers_are_the_references() {
    use aztek_host::view::{select_picked, CTRL, SHIFT};
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a; polyCube -n b; select -cl;");
    let p = |s: &str| vec![s.to_string()];
    select_picked(&mut c, p("a"), 0);
    select_picked(&mut c, p("b"), SHIFT);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["a", "b"]));
    select_picked(&mut c, p("a"), SHIFT);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["b"]));
    select_picked(&mut c, p("b"), CTRL);
    assert_eq!(mel(&mut c, "ls -sl;"), json!([]));
    select_picked(&mut c, p("b"), CTRL);
    assert_eq!(mel(&mut c, "ls -sl;"), json!([]));
    select_picked(&mut c, p("a"), CTRL | SHIFT);
    select_picked(&mut c, p("a"), CTRL | SHIFT);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["a"]));
    mel(&mut c, "select a; selectMode -component; selectType -v true; select a.vtx[0];");
    select_picked(&mut c, p("a.vtx[1]"), SHIFT);
    select_picked(&mut c, p("a.vtx[0]"), SHIFT);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["a.vtx[1]"]));
    select_picked(&mut c, p("a.vtx[2]"), CTRL | SHIFT);
    select_picked(&mut c, p("a.vtx[1]"), CTRL);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["a.vtx[2]"]));
}
