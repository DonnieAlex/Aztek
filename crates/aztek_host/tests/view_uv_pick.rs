//! Picking in a view in the reference's UV, UV Shell, Vertex Face and Multi modes.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{gather, Viewport};
use aztek_host::Core;
use glam::{DVec2, DVec3};
use serde_json::json;

fn setup(c: &mut Core) -> (Viewport, Vec<aztek_host::viewport::DrawItem>) {
    c.mel("polyPlane -w 2 -h 2 -sx 2 -sy 2; select pPlane1;");
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    // Straight down on the plane.
    vp.cam.coi = DVec3::ZERO;
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 89.9;
    vp.cam.distance = 6.0;
    let items = gather(&mut c.s.graph);
    (vp, items)
}

fn at(vp: &Viewport, p: DVec3) -> DVec2 {
    vp.screen().project(p).unwrap()
}

#[test]
fn uv_vertex_face_and_multi_picks() {
    let mut c = Core::new();
    let (mut vp, items) = setup(&mut c);
    // UV: the UV at the point clicked.
    c.mel("selectMode uv;");
    let centre = at(&vp, DVec3::ZERO);
    let picked = vp.pick(&c.s, &items, centre);
    assert_eq!(picked.len(), 1, "{picked:?}");
    assert!(picked[0].starts_with("pPlane1.map["), "{picked:?}");
    // UV Shell: the whole plane is one shell, so all nine UVs.
    c.mel("selectType -msh true;");
    assert_eq!(c.s.run("selectMode", json!({})).unwrap(), json!("uvShell"));
    assert_eq!(vp.pick(&c.s, &items, centre), vec!["pPlane1.map[0:8]".to_string()]);
    // Vertex face: the corner nearest the click, inside a face.
    c.mel("selectMode vertexFace;");
    let near = at(&vp, DVec3::new(0.15, 0.0, 0.15));
    let picked = vp.pick(&c.s, &items, near);
    assert_eq!(picked.len(), 1, "{picked:?}");
    assert!(picked[0].starts_with("pPlane1.vtxFace[4]["), "{picked:?}");
    // Multi: near a point the vertex, near an edge's middle the edge, in a face's middle the face.
    c.mel("selectMode multi;");
    assert!(vp.pick(&c.s, &items, centre)[0].starts_with("pPlane1.vtx[4]"));
    assert!(vp.pick(&c.s, &items, at(&vp, DVec3::new(0.5, 0.0, 0.0)))[0].starts_with("pPlane1.e["));
    assert!(vp.pick(&c.s, &items, at(&vp, DVec3::new(0.5, 0.0, 0.5)))[0].starts_with("pPlane1.f["));
    // Picking in Multi keeps Multi.
    let picked = vp.pick(&c.s, &items, centre);
    c.s.run("select", json!({"args": picked})).unwrap();
    assert_eq!(c.s.run("selectMode", json!({})).unwrap(), json!("multi"));
    // A box in Multi takes the vertices, edges and faces inside.
    let (a, b) = (at(&vp, DVec3::new(-1.1, 0.0, -1.1)), at(&vp, DVec3::new(0.1, 0.0, 0.1)));
    let boxed = vp.pick_rect(&c.s, &items, a, b);
    for k in [".vtx[", ".e[", ".f["] {
        assert!(boxed.iter().any(|x| x.contains(k)), "{k} {boxed:?}");
    }
}

#[test]
fn a_click_or_a_short_drag_takes_a_locator_through_the_click_box() {
    use aztek_host::viewport::Viewport;
    let mut c = Core::new();
    c.mel("spaceLocator -n loc; scale 5 5 5 loc; select -cl;");
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = DVec3::ZERO;
    vp.cam.yaw = 30.0;
    vp.cam.pitch = -30.0;
    vp.cam.distance = 30.0;
    let items = gather(&mut c.s.graph);
    let on = vp.screen().project(DVec3::new(4.5, 0.0, 0.0)).unwrap();
    // Four pixels off the line: inside a click box of 4 either way it is picked; with 2, not.
    let off = on + DVec2::new(0.0, 3.5);
    vp.click_box = 4.0;
    assert_eq!(vp.pick(&c.s, &items, off), vec!["loc".to_string()]);
    vp.click_box = 2.0;
    assert!(vp.pick(&c.s, &items, off).is_empty());
    // selectPref keeps the size.
    c.mel("selectPref -clickBoxSize 7;");
    assert_eq!(c.s.run("selectPref", serde_json::json!({"query": true, "clickBoxSize": true})).unwrap(), serde_json::json!(7));
}
