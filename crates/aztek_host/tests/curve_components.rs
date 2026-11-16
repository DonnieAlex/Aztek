//! Curve Edit Point and Curve Point modes: selectType -ep and -cpp, picking `curve1.ep[i]` and `curve1.u[t]` in a view,
//! a box taking edit points, and moving an edit point.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{gather, Viewport};
use aztek_host::Core;
use glam::{DVec2, DVec3};
use serde_json::json;

fn mel(c: &mut Core, src: &str) -> serde_json::Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn edit_points_and_curve_points_pick_in_a_view() {
    let mut c = Core::new();
    // A straight degree 3 curve along X through 0, 2, 4, 6: four edit points.
    mel(&mut c, "curve -n crv -d 3 -ep 0 0 0 -ep 2 0 0 -ep 4 0 0 -ep 6 0 0; select crv;");
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = DVec3::new(3.0, 0.0, 0.0);
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 89.9;
    vp.cam.distance = 12.0;
    let items = gather(&mut c.s.graph);
    let at = |vp: &Viewport, p: DVec3| -> DVec2 { vp.screen().project(p).unwrap() };

    mel(&mut c, "selectMode vertex; selectType -ep true;");
    assert_eq!(mel(&mut c, "selectType -q -ep;"), json!(true));
    assert_eq!(mel(&mut c, "selectType -q -cv;"), json!(false));
    assert_eq!(vp.pick(&c.s, &items, at(&vp, DVec3::new(4.05, 0.0, 0.0))), vec!["crv.ep[2]".to_string()]);
    // A box over the first two.
    let boxed = vp.pick_rect(&c.s, &items, at(&vp, DVec3::new(-0.5, 0.0, -0.5)), at(&vp, DVec3::new(2.5, 0.0, 0.5)));
    assert_eq!(boxed, vec!["crv.ep[0]".to_string(), "crv.ep[1]".to_string()]);

    // Curve Point: a click on the curve at x = 3 is half way along.
    mel(&mut c, "selectType -cpp true;");
    let picked = vp.pick(&c.s, &items, at(&vp, DVec3::new(3.0, 0.0, 0.0)));
    assert_eq!(picked.len(), 1, "{picked:?}");
    let u: f64 = picked[0].trim_start_matches("crv.u[").trim_end_matches(']').parse().unwrap();
    let (lo, hi) = (mel(&mut c, "getAttr crv.minValue;").as_f64().unwrap(), mel(&mut c, "getAttr crv.maxValue;").as_f64().unwrap());
    assert!((u - (lo + hi) / 2.0).abs() < 0.02, "{u} in {lo}..{hi}");

    // Back to CVs: F9 and the marking menu's Control Vertex.
    mel(&mut c, "selectType -cv true;");
    assert_eq!(mel(&mut c, "selectType -q -ep;"), json!(false));
    assert!(vp.pick(&c.s, &items, at(&vp, DVec3::new(0.0, 0.0, 0.0)))[0].starts_with("crv.cv["));

    // Moving an edit point: the curve passes through its new place.
    mel(&mut c, "selectType -ep true; select -r crv.ep[1]; move -r 0 0 1;");
    let p = mel(&mut c, "pointPosition crv.ep[1];");
    assert!((p[2].as_f64().unwrap() - 1.0).abs() < 1e-3, "{p}");
}
