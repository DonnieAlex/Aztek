//! The rotate manipulator's Gimbal axes: the rotate channels' own axes, which line up in gimbal lock.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{Handle, Tool, Viewport};
use aztek_host::Core;
use glam::DVec3;

fn close(a: DVec3, b: DVec3) -> bool {
    (a - b).length() < 1e-6
}

#[test]
fn rotate_picks_the_visible_side_of_each_axis() {
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 1000.0, 800.0];
    vp.tool = Tool::Rotate;
    let pivot = DVec3::ZERO;
    let len = vp.handle_len(pivot, 1.0);
    let direction = DVec3::new(0.0, 0.6, 0.8);
    let pick = |v: &Viewport, point| v.manip_hit(pivot, v.screen().project(point).unwrap(), 1.0);
    assert_eq!(pick(&vp, direction * len), Some(Handle::Axis(0)));
    assert_eq!(pick(&vp, -direction * len), Some(Handle::Trackball));

    vp.cam.look_from(DVec3::Z * 40.0);
    for orthographic in [false, true] {
        vp.cam.orthographic = orthographic;
        let radius = vp.handle_len(pivot, 1.0);
        assert_eq!(pick(&vp, DVec3::new(0.6, 0.8, 0.0) * radius), Some(Handle::Axis(2)));
    }
}

#[test]
fn gimbal_axes_follow_the_rotate_order() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n box; setAttr box.rotate 0 90 0; select box;").unwrap();
    let mut vp = Viewport::new(CamKind::Persp);
    vp.tool = Tool::Rotate;
    vp.axis_mode = "gimbal".into();
    vp.update_frame(&mut c.s);
    // xyz: Z is the parent's, Y turned by Z (none), X turned by Y's 90: along -Z, on top of Z.
    assert!(close(vp.frame.z_axis, DVec3::Z), "{:?}", vp.frame);
    assert!(close(vp.frame.y_axis, DVec3::Y), "{:?}", vp.frame);
    assert!(close(vp.frame.x_axis, -DVec3::Z), "{:?}", vp.frame);
    // zyx: X outermost; Z turned by Y's 90 lies along X.
    c.mel_checked("setAttr box.rotateOrder 5;").unwrap();
    vp.update_frame(&mut c.s);
    assert!(close(vp.frame.x_axis, DVec3::X) && close(vp.frame.z_axis, DVec3::X), "{:?}", vp.frame);
    // Under a turned parent, all turn with it.
    c.mel_checked("setAttr box.rotateOrder 0; setAttr box.rotate 0 0 0; group -n g box; setAttr g.ry 90; select box;").unwrap();
    vp.update_frame(&mut c.s);
    assert!(close(vp.frame.x_axis, -DVec3::Z) && close(vp.frame.z_axis, DVec3::X), "{:?}", vp.frame);
}
