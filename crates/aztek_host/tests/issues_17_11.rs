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

/// #123: setting the ViewCube's preferences, as Alt+B's preference pass did, leaves the camera where it is.
#[test]
fn view_cube_settings_leave_the_camera() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -w 10 -h 10 -d 10; select -cl; setAttr persp.t 3 4 5; setAttr persp.r -20 30 0;");
    let before = mel(&mut c, "getAttr persp.t;");
    mel(&mut c, "viewManip -visible true -topRight true -zoomToFitScene true -preserveSceneUp true;");
    assert_eq!(mel(&mut c, "getAttr persp.t;"), before);
    assert_eq!(mel(&mut c, "viewManip -q -zoomToFitScene;"), json!(true));
}

/// #124: in a component mode, a click on another object takes it, in object mode; a click on nothing keeps the mode.
#[test]
fn a_click_on_another_object_leaves_component_mode() {
    use aztek_host::view::select_picked;
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a -w 2 -h 2 -d 2; move -4 0 0; polyCube -n b -w 2 -h 2 -d 2; move 4 0 0; select a; selectMode -component; selectType -v true; select a.vtx[0];");
    let mut vp = top_view();
    let items = gather(&mut c.s.graph);
    let on_b = vp.screen().project(DVec3::new(4.0, 1.0, 0.0)).unwrap();
    let nothing = vp.screen().project(DVec3::new(0.0, 0.0, 9.0)).unwrap();
    let picked = vp.pick(&c.s, &items, nothing);
    assert!(picked.is_empty());
    let picked = vp.pick(&c.s, &items, on_b);
    assert_eq!(picked, vec!["b".to_string()]);
    select_picked(&mut c, picked, 0);
    assert_eq!(mel(&mut c, "selectMode -q -object;"), json!(true));
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["b"]));
}

/// #89: referenced nodes and startup cameras among those deleted are passed over too; the rest go.
#[test]
fn delete_passes_over_referenced_nodes_and_startup_cameras() {
    let dir = std::env::temp_dir().join(format!("az_del_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let prop = dir.join("prop.ma").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, &format!("polyCube -n crate; file -rename \"{prop}\"; file -save -type \"mayaAscii\"; file -f -new;"));
    mel(&mut c, &format!("file -r -ns \"rd\" \"{prop}\"; polySphere -n ball; shadingNode -asShader lambert -n mat;"));
    mel(&mut c, "select -r rd:crate ball persp mat defaultRenderGlobals; delete;");
    assert_eq!(mel(&mut c, "objExists ball;"), json!(false));
    assert_eq!(mel(&mut c, "objExists mat;"), json!(false));
    assert_eq!(mel(&mut c, "objExists rd:crate;"), json!(true));
    assert_eq!(mel(&mut c, "objExists persp;"), json!(true));
    assert!(c.mel_checked("delete rd:crate;").is_err());
    let _ = std::fs::remove_dir_all(&dir);
}
