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

/// #91: an OBJ's exporter "default" group doesn't name the mesh; `file -i -rnn` returns the new nodes.
#[test]
fn obj_import_names_and_returns_new_nodes() {
    let mut c = Core::new();
    let dir = std::env::temp_dir().join("aztek_issue_91");
    std::fs::create_dir_all(&dir).unwrap();
    let a = dir.join("a.obj");
    std::fs::write(&a, "g default\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n").unwrap();
    let b = dir.join("b.obj");
    std::fs::write(&b, "o Thing\nv 0 0 0\nv 1 0 0\nv 0 1 0\nusemtl default\nf 1 2 3\n").unwrap();
    mel(&mut c, "loadPlugin objExport;");
    let n = mel(&mut c, &format!("file -i -rnn \"{}\";", a.display()));
    assert!(n.as_array().unwrap().iter().any(|x| x.as_str().unwrap().ends_with("polySurface1")), "{n}");
    let n = mel(&mut c, &format!("file -i -rnn \"{}\";", b.display()));
    assert!(n.as_array().unwrap().iter().any(|x| x == "Thing"), "{n}");
    assert_eq!(mel(&mut c, "ls -type transform \"default*\";"), json!([]));
}

/// #92: every type an importer reads can be referenced, its nodes named as the file has them inside the namespace.
#[test]
fn every_imported_type_can_be_referenced() {
    let mut c = Core::new();
    let dir = std::env::temp_dir().join("aztek_issue_92");
    std::fs::create_dir_all(&dir).unwrap();
    let d = dir.display().to_string();
    mel(&mut c, "loadPlugin objExport; loadPlugin fbx; loadPlugin gltf; loadPlugin AbcImport; loadPlugin AbcExport;");
    mel(&mut c, "polyCube -n c;");
    mel(&mut c, &format!("file -f -type \"FBX export\" -exportAll \"{d}/c.fbx\";"));
    mel(&mut c, &format!("exportGltf \"{d}/c.glb\";"));
    mel(&mut c, &format!("AbcExport -j \"-root |c -file {d}/c.abc\";"));
    mel(&mut c, &format!("file -f -exportAll -type OBJexport \"{d}/c.obj\";"));
    mel(&mut c, &format!("file -f -exportAll -type STL \"{d}/c.stl\";"));
    for ext in ["fbx", "glb", "abc", "obj", "stl"] {
        mel(&mut c, &format!("file -r -ns r{ext} \"{d}/c.{ext}\";"));
        let rn = mel(&mut c, &format!("referenceQuery -rfn \"{d}/c.{ext}\";"));
        assert_eq!(rn, json!(format!("r{ext}RN")), "{ext}");
        let nodes = mel(&mut c, &format!("ls -type transform \"r{ext}:*\";"));
        assert!(!nodes.as_array().unwrap().is_empty(), "{ext}: {nodes}");
        if matches!(ext, "fbx" | "glb" | "abc") {
            assert!(nodes.as_array().unwrap().contains(&json!(format!("r{ext}:c"))), "{ext}: {nodes}");
        }
    }
    // Nothing merged into the scene's own cube.
    assert_eq!(mel(&mut c, "ls -type transform \"c*\";"), json!(["c"]));
}

/// #94: Combine keeps each mesh's materials face by face, and Separate gives them to the pieces.
#[test]
fn combine_and_separate_keep_materials() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a; polyCube -n b; move 3 0 0 b;");
    mel(&mut c, "shadingNode -asShader lambert -n red; sets -renderable true -noSurfaceShader true -empty -n redSG; connectAttr red.outColor redSG.surfaceShader;");
    mel(&mut c, "shadingNode -asShader lambert -n blue; sets -renderable true -noSurfaceShader true -empty -n blueSG; connectAttr blue.outColor blueSG.surfaceShader;");
    mel(&mut c, "sets -e -forceElement redSG a; sets -e -forceElement blueSG b.f[0:2]; sets -e -forceElement redSG b.f[3:5];");
    mel(&mut c, "polyUnite -n u a b;");
    let red = mel(&mut c, "sets -q redSG;");
    assert!(red.as_array().unwrap().contains(&json!("u.f[0:5]")) && red.as_array().unwrap().contains(&json!("u.f[9:11]")), "{red}");
    assert!(mel(&mut c, "sets -q blueSG;").as_array().unwrap().contains(&json!("u.f[6:8]")));
    let pieces = mel(&mut c, "polySeparate u;");
    let first = pieces[0].as_str().unwrap().to_string();
    let second = pieces[1].as_str().unwrap().to_string();
    let shading = |c: &mut Core, x: &str| mel(c, &format!("listConnections -type shadingEngine `listRelatives -s {x}`;"));
    assert_eq!(shading(&mut c, &first), json!(["redSG"]));
    let both = shading(&mut c, &second);
    assert!(both.as_array().unwrap().contains(&json!("redSG")) && both.as_array().unwrap().contains(&json!("blueSG")), "{both}");
}

/// #96: emitter makes only the emitter; the reference's type names find these nodes; connectDynamic says what it got.
#[test]
fn emitter_alone_reference_type_names_and_clear_errors() {
    let mut c = Core::new();
    assert_eq!(mel(&mut c, "emitter -type volume -n em1;"), json!(["em1"]));
    assert_eq!(mel(&mut c, "ls -type particles;"), json!([]));
    assert_eq!(mel(&mut c, "ls -type nucleus;"), json!([]));
    assert!(!mel(&mut c, "ls -type pointEmitter;").as_array().unwrap().is_empty());
    let p = mel(&mut c, "nParticle -n np;");
    assert_eq!(p, json!(["np", "npShape"]));
    assert_eq!(mel(&mut c, "nParticle;"), json!(["nParticle1", "nParticleShape1"]));
    assert_eq!(mel(&mut c, "particle;")[0], json!("particle1"));
    assert_eq!(mel(&mut c, "ls -type nParticle;")[0], json!("npShape"));
    assert_eq!(mel(&mut c, "ls -type particle;")[0], json!("npShape"));
    assert_eq!(mel(&mut c, "size(`ls -type nucleus`);"), json!(1));
    mel(&mut c, "connectDynamic -em em1 np;");
    // Again: nothing doubled.
    mel(&mut c, "connectDynamic -em em1 np;");
    assert_eq!(mel(&mut c, "size(`listConnections -s 1 -d 0 npShape.newParticles`);"), json!(1));
    let e = c.mel_checked("connectDynamic -em np np;").unwrap_err().to_string();
    assert!(e.contains("takes an emitter") && e.contains("np is a transform of a particles") && e.contains("em1"), "{e}");
    mel(&mut c, "polySphere -n ball;");
    assert_eq!(mel(&mut c, "emitter -type surface ball;"), json!(["ball", "emitter1"]));
}
