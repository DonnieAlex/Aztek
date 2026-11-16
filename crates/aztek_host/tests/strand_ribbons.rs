//! Strands in the views: a ribbon a strand, turned to the eye, at least a pixel wide, lit as a fibre.

use aztek_host::viewport::strand_ribbons;
use aztek_scene::harold::strands::Strands;
use glam::{DMat4, DVec3, Vec3};

fn one_strand(radius: f32) -> Strands {
    Strands { points: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 2.0, 0.0)], radius: vec![radius; 3], counts: vec![3], world: DMat4::IDENTITY, color: [0.5, 0.4, 0.3], ..Default::default() }
}

#[test]
fn ribbons_face_the_eye_and_keep_a_pixel() {
    let eye = DVec3::new(0.0, 1.0, 10.0);
    let (v, i) = strand_ribbons(&[one_strand(0.1)], eye, None, (0.001, false));
    assert_eq!(v.len(), 6);
    assert_eq!(i.len(), 12);
    // Across the view: the two vertices of a point differ along X only, by the diameter.
    let (a, b) = (Vec3::from(v[2].pos), Vec3::from(v[3].pos));
    assert!(((b - a).length() - 0.2).abs() < 1e-5, "{}", (b - a).length());
    assert!((b - a).z.abs() < 1e-6);
    // Far finer than a pixel: still a pixel wide at its distance (10 units, a pixel 0.001 a unit).
    let (v, _) = strand_ribbons(&[one_strand(1e-6)], eye, None, (0.001, false));
    let w = (Vec3::from(v[3].pos) - Vec3::from(v[2].pos)).length();
    assert!((w - 0.01).abs() < 1e-3, "{w}");
    // Lit across the view, darker at the root.
    assert!(v[5].color[0] > v[1].color[0]);
}
