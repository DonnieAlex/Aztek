use aztek_host::camera::CamKind;
use aztek_host::viewport::{Handle, Tool, Viewport};
use glam::{DMat3, DVec3};

fn check_plane_pick(tool: Tool, offset: f64) {
    let mut view = Viewport::new(CamKind::Persp);
    view.rect = [0.0, 0.0, 1000.0, 800.0];
    view.cam.look_from(DVec3::Z * 40.0);
    view.cam.orthographic = true;
    let pivot = DVec3::ZERO;

    // #256: check the XY plane in a face-on view so other handles don't overlap it.
    view.tool = tool;
    let len = view.handle_len(pivot, 1.0);
    let pick = |point| view.manip_hit(pivot, view.screen().project(point).unwrap(), 1.0);
    // unverified: the exact distance from the pivot in a front view of the reference.
    assert_eq!(pick((DVec3::X + DVec3::Y) * len * offset), Some(Handle::Plane(2)));
    assert_eq!(pick((DVec3::X + DVec3::Y) * len * 0.30), None);
    assert_eq!(pick(pivot), Some(Handle::Center));
    assert_eq!(pick(DVec3::X * len * 0.8), Some(Handle::Axis(0)));
}

#[test]
fn move_picks_plane_handles_farther_from_the_pivot() {
    check_plane_pick(Tool::Move, 0.62);
}

#[test]
fn scale_picks_plane_handles_farther_from_the_pivot() {
    check_plane_pick(Tool::Scale, 0.76);
}

#[test]
fn an_angled_plane_picks_even_if_the_third_axis_is_edge_on() {
    let mut view = Viewport::new(CamKind::Top);
    view.rect = [0.0, 0.0, 1000.0, 800.0];
    view.tool = Tool::Move;
    let y = (DVec3::X + DVec3::Y).normalize();
    view.frame = DMat3::from_cols(DVec3::X, y, DVec3::Z);
    let pivot = DVec3::ZERO;
    let center = (y + DVec3::Z) * view.handle_len(pivot, 1.0) * 0.62;
    let pixel = view.screen().project(center).unwrap();
    assert_eq!(view.manip_hit(pivot, pixel, 1.0), Some(Handle::Plane(0)));
}
