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
