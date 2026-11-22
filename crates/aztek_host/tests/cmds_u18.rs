//! Unit u18: selectPref, selectType, setNodeTypeFlag, shapeCompare, suitePrefs, switchTable, tolerance, unknownPlugin
//! and vectorize.

use aztek_host::camera::CamKind;
use aztek_host::viewport::{gather, Viewport};
use aztek_host::Core;
use glam::{DVec2, DVec3};
use serde_json::{json, Value};

/// Selection masks, selection preferences, tolerances and node type flags are the application's, shared by every
/// core in the process: one test touching them at a time.
static SHARED: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    SHARED.lock().unwrap_or_else(|e| e.into_inner())
}

fn core() -> Core {
    let mut c = Core::new();
    aztek_scene::cameras::make_defaults(&mut c.s.graph);
    c
}

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

fn dir(tag: &str) -> String {
    let d = std::env::temp_dir().join(format!("aztek_u18_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.to_string_lossy().replace('\\', "/")
}

fn truthy(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_i64().is_some_and(|x| x != 0))
}

fn view() -> Viewport {
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = DVec3::new(0.0, 2.0, 0.0);
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 0.0;
    vp.cam.distance = 30.0;
    vp
}

fn marquee(c: &mut Core, vp: &Viewport) -> Vec<String> {
    let items = gather(&mut c.s.graph);
    vp.pick_rect(&c.s, &items, DVec2::new(0.0, 0.0), DVec2::new(800.0, 800.0))
}

fn reset_masks(c: &mut Core) {
    ok(c, "selectType -allObjects 1; selectType -objectComponent -allComponents 0;");
    ok(c, "selectPref -isp 0 -sbs 0 -stc 0 -cbs 4 -mcb 8 -ud 0 -aud 0;");
}

// ---------------------------------------------------------------------------------------------------------------------
// selectPref

#[test]
fn select_pref_sets_and_reads_every_setting_with_its_type() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    // Defaults.
    assert_eq!(ok(&mut c, "selectPref -q -clickBoxSize;"), json!(4));
    assert_eq!(ok(&mut c, "selectPref -q -mcb;"), json!(8));
    assert!(!truthy(&ok(&mut c, "selectPref -q -popupMenuSelection;")));
    assert!(truthy(&ok(&mut c, "selectPref -q -allowHiliteSelection;")));
    assert_eq!(ok(&mut c, "selectPref -q -selectionChildHighlightMode;"), json!(0));
    assert_eq!(ok(&mut c, "selectPref -q -preSelectHiliteSize;").as_f64(), Some(7.0));
    // Set by long and short names, read back by either.
    ok(&mut c, "selectPref -popupMenuSelection true -disableComponentPopups true;");
    assert!(truthy(&ok(&mut c, "selectPref -q -pms;")));
    assert!(truthy(&ok(&mut c, "selectPref -q -dcp;")));
    ok(&mut c, "selectPref -cbs 6 -phs 3.5 -sch 2 -pds 10 -pdt 50 -epl 1 -asc 1 -asm 1 -aud 1 -ccs 1 -ps 1 -psd 1 -psc 1 -sld 1 -tso 1 -xns 1 -ahs 0;");
    assert_eq!(ok(&mut c, "selectPref -q -clickBoxSize;"), json!(6));
    assert_eq!(ok(&mut c, "selectPref -q -preSelectHiliteSize;").as_f64(), Some(3.5));
    assert_eq!(ok(&mut c, "selectPref -q -selectionChildHighlightMode;"), json!(2));
    assert_eq!(ok(&mut c, "selectPref -q -preSelectDeadSpace;"), json!(10));
    assert_eq!(ok(&mut c, "selectPref -q -preSelectTweakDeadSpace;"), json!(50));
    for flag in ["expandPopupList", "autoSelectContainer", "autoSelectOutlinerSetMembers", "autoUseDepth", "containerCentricSelection", "paintSelect", "paintSelectWithDepth", "preSelectClosest", "straightLineDistance", "trackSelectionOrder", "xformNoSelect"] {
        assert!(truthy(&ok(&mut c, &format!("selectPref -q -{flag};"))), "{flag}");
    }
    assert!(!truthy(&ok(&mut c, "selectPref -q -allowHiliteSelection;")));
    // The click boxes must be positive; the child highlight mode is 0 to 2.
    assert!(bad(&mut c, "selectPref -clickBoxSize 0;").contains("positive"));
    assert!(bad(&mut c, "selectPref -manipClickBoxSize -2;").contains("positive"));
    assert!(bad(&mut c, "selectPref -sch 3;").contains("0, 1 or 2"));
    // The app's JSON form.
    assert_eq!(c.s.run("selectPref", json!({"query": true, "clickBoxSize": true})).unwrap(), json!(6));
    ok(&mut c, "selectPref -epl 0 -asc 0 -asm 0 -aud 0 -ccs 0 -ps 0 -psd 0 -psc 0 -sld 0 -tso 0 -xns 0 -ahs 1 -pms 0 -dcp 0 -sch 0 -phs 7 -pds 8 -pdt 120;");
    reset_masks(&mut c);
}

#[test]
fn select_pref_priority_single_box_and_lock() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    ok(&mut c, "select -cl; joint -p 0 0 0; joint -p 0 4 1; joint -p 0 8 0; curve -d 1 -p 3 0 0 -p 3 6 0; polySphere -r 1; move -3 2 0; select -cl;");
    c.s.run("getAttr", json!({"args": ["joint2.worldMatrix"]})).ok();
    let vp = view();
    // Selection priority: the joints win over the curve and the sphere.
    assert_eq!(marquee(&mut c, &vp), vec!["joint1".to_string()]);
    // Ignoring it, the marquee takes every kind it covers.
    ok(&mut c, "selectPref -ignoreSelectionPriority 1;");
    let all = marquee(&mut c, &vp);
    assert!(all.contains(&"joint1".to_string()) && all.contains(&"curve1".to_string()) && all.contains(&"pSphere1".to_string()), "{all:?}");
    // Single box selection: one only.
    ok(&mut c, "selectPref -singleBoxSelection 1;");
    assert_eq!(marquee(&mut c, &vp).len(), 1);
    ok(&mut c, "selectPref -lockSelection 1;");
    assert!(aztek_host::ui_cmds::selection_locked());
    assert!(truthy(&ok(&mut c, "selectPref -q -lockSelection;")));
    ok(&mut c, "selectPref -lockSelection 0;");
    assert!(!aztek_host::ui_cmds::selection_locked());
    reset_masks(&mut c);
}

// ---------------------------------------------------------------------------------------------------------------------
// selectType

#[test]
fn select_type_object_masks_gate_picking() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    ok(&mut c, "select -cl; joint -p 0 0 0; joint -p 0 4 1; joint -p 0 8 0; ikHandle -sj joint1 -ee joint3; curve -d 1 -p 3 0 0 -p 3 6 0; polySphere -r 1; move -3 2 0; select -cl;");
    c.s.run("getAttr", json!({"args": ["joint3.worldMatrix"]})).ok();
    let vp = view();
    assert_eq!(marquee(&mut c, &vp), vec!["ikHandle1".to_string()]);
    ok(&mut c, "selectType -ikHandle 0;");
    assert!(!truthy(&ok(&mut c, "selectType -q -ikh;")));
    assert_eq!(marquee(&mut c, &vp), vec!["joint1".to_string()]);
    ok(&mut c, "selectType -j 0;");
    assert_eq!(marquee(&mut c, &vp), vec!["curve1".to_string()]);
    // The curve mask covers NURBS curves.
    ok(&mut c, "selectType -curve 0;");
    assert!(!truthy(&ok(&mut c, "selectType -q -nurbsCurve;")));
    assert_eq!(marquee(&mut c, &vp), vec!["pSphere1".to_string()]);
    ok(&mut c, "selectType -p 0;");
    assert!(!truthy(&ok(&mut c, "selectType -q -polymesh;")));
    assert!(!truthy(&ok(&mut c, "selectType -q -allObjects;")));
    assert!(marquee(&mut c, &vp).is_empty());
    // Everything back on, then all off but one.
    ok(&mut c, "selectType -allObjects 1;");
    assert!(truthy(&ok(&mut c, "selectType -q -allObjects;")));
    ok(&mut c, "selectType -allObjects 0 -polymesh 1;");
    assert_eq!(marquee(&mut c, &vp), vec!["pSphere1".to_string()]);
    reset_masks(&mut c);
}

#[test]
fn select_type_masks_by_kind() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    ok(&mut c, "polyCube -n box; select box; lattice -n lat; select box; cluster -n clu; spaceLocator -n loc; pointLight -n lamp; select -cl;");
    let pick = |c: &mut Core, n: &str| {
        let id = c.s.graph.find(n).unwrap().unwrap();
        aztek_host::select_cmds::pickable(&c.s.graph, id)
    };
    let lat = ok(&mut c, "ls -type lattice;")[0].as_str().unwrap().to_string();
    for (name, mask) in [("loc", "locator"), ("lamp", "light"), ("persp", "camera"), ("box", "polymesh"), (lat.as_str(), "lattice")] {
        let id = c.s.graph.find(name).unwrap().unwrap();
        assert_eq!(aztek_host::select_cmds::node_mask(&c.s.graph, id), mask, "{name}");
        assert!(pick(&mut c, name));
        ok(&mut c, &format!("selectType -{mask} 0;"));
        assert!(!pick(&mut c, name), "{name}");
        ok(&mut c, &format!("selectType -{mask} 1;"));
    }
    // The locator mask sets every kind of locator.
    ok(&mut c, "selectType -locator 0;");
    assert!(!truthy(&ok(&mut c, "selectType -q -locatorXYZ;")));
    assert!(!truthy(&ok(&mut c, "selectType -q -orientationLocator;")));
    ok(&mut c, "selectType -lc 1;");
    // Cluster handles and lattices have their own masks.
    let clu = c.s.graph.find("cluHandle").unwrap().unwrap();
    assert_eq!(aztek_host::select_cmds::node_mask(&c.s.graph, clu), "cluster");
    // Masks with no Aztek object kind still hold their state.
    for flag in ["fluid", "stroke", "subdiv", "texture", "emitter", "field", "follicle", "hairSystem", "nCloth", "nRigid", "nParticleShape", "particleShape", "rigidBody", "rigidConstraint", "dynamicConstraint", "spring", "plane", "sculpt", "dimension", "implicitGeometry", "collisionModel", "curveOnSurface", "handle", "ikEndEffector", "locatorUV"] {
        ok(&mut c, &format!("selectType -{flag} 0;"));
        assert!(!truthy(&ok(&mut c, &format!("selectType -q -{flag};"))), "{flag}");
        ok(&mut c, &format!("selectType -{flag} 1;"));
        assert!(truthy(&ok(&mut c, &format!("selectType -q -{flag};"))), "{flag}");
    }
    reset_masks(&mut c);
}

#[test]
fn select_type_components_by_name_and_object_component() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    ok(&mut c, "polyCube -n a; select a; selectMode -component;");
    // The old mesh names switch modes as the polymesh ones do.
    ok(&mut c, "selectType -facet 1;");
    assert!(truthy(&ok(&mut c, "selectType -q -facet;")));
    assert!(truthy(&ok(&mut c, "selectType -q -polymeshFace;")));
    ok(&mut c, "selectType -eg 1;");
    assert!(truthy(&ok(&mut c, "selectType -q -edge;")));
    assert!(!truthy(&ok(&mut c, "selectType -q -fc;")));
    // Component masks without a mode of their own hold their state.
    for flag in ["hull", "isoparm", "curveKnot", "surfaceKnot", "surfaceUV", "surfaceEdge", "surfaceFace", "surfaceRange", "surfaceParameterPoint", "latticePoint", "particle", "nParticle", "rotatePivot", "scalePivot", "selectHandle", "localRotationAxis", "jointPivot", "imagePlane", "springComponent", "subdivMeshEdge", "subdivMeshFace", "subdivMeshPoint", "subdivMeshUV", "polymeshFreeEdge", "motionTrailPoint", "motionTrailTangent", "animBreakdown", "animCurve", "animInTangent", "animKeyframe", "animOutTangent"] {
        ok(&mut c, &format!("selectType -{flag} 1;"));
        assert!(truthy(&ok(&mut c, &format!("selectType -q -{flag};"))), "{flag}");
        ok(&mut c, &format!("selectType -{flag} 0;"));
        assert!(!truthy(&ok(&mut c, &format!("selectType -q -{flag};"))), "{flag}");
    }
    // -allComponents off: every component mask off, the mode's too.
    ok(&mut c, "selectType -allObjects 1 -allComponents 0;");
    assert!(!truthy(&ok(&mut c, "selectType -q -allComponents;")));
    assert!(!truthy(&ok(&mut c, "selectType -q -edge;")));
    ok(&mut c, "selectType -pv 1;");
    assert!(truthy(&ok(&mut c, "selectType -q -polymeshVertex;")));
    // Object mode component masks are their own.
    assert!(!truthy(&ok(&mut c, "selectType -q -ocm -selectHandle;")));
    ok(&mut c, "selectType -ocm -selectHandle 1 -rotatePivot 1;");
    assert!(truthy(&ok(&mut c, "selectType -q -ocm -selectHandle;")));
    assert!(truthy(&ok(&mut c, "selectType -q -objectComponent -rp;")));
    assert!(!truthy(&ok(&mut c, "selectType -q -selectHandle;")));
    assert!(aztek_host::select_cmds::component_mask("selectHandle", true));
    // User masks by name.
    assert!(truthy(&ok(&mut c, "selectType -q -queryByName \"myShape\";")));
    ok(&mut c, "selectType -byName \"myShape\" 0 -bn \"other\" 1;");
    assert!(!truthy(&ok(&mut c, "selectType -q -qbn \"myShape\";")));
    assert!(truthy(&ok(&mut c, "selectType -q -qbn \"other\";")));
    ok(&mut c, "selectType -bn \"myShape\" 1;");
    reset_masks(&mut c);
}

#[test]
fn select_type_change_affects_active_drops_what_cannot_be_picked() {
    let _l = lock();
    let mut c = core();
    reset_masks(&mut c);
    ok(&mut c, "polyCube -n box; spaceLocator -n loc; select -r box loc;");
    ok(&mut c, "selectType -locator 0;");
    // Off by default: the selection stays.
    assert_eq!(c.s.graph.selection().len(), 2);
    ok(&mut c, "selectType -locator 1; selectPref -selectTypeChangeAffectsActive 1; selectType -locator 0;");
    let sel: Vec<String> = c.s.graph.selection().iter().map(|&id| c.s.graph.path(id)).collect();
    assert_eq!(sel, vec!["box".to_string()]);
    ok(&mut c, "selectPref -stc 0;");
    reset_masks(&mut c);
}

// ---------------------------------------------------------------------------------------------------------------------
// setNodeTypeFlag

#[test]
fn set_node_type_flag_hides_a_type_from_the_editors() {
    let _l = lock();
    let mut c = core();
    assert!(truthy(&ok(&mut c, "setNodeTypeFlag -q -display \"multiplyDivide\";")));
    assert_eq!(ok(&mut c, "setNodeTypeFlag -display false \"multiplyDivide\";"), json!(true));
    assert!(!truthy(&ok(&mut c, "setNodeTypeFlag -q -display \"multiplyDivide\";")));
    // By its type tag too.
    let tag = ok(&mut c, "objectType -tagFromType \"multiplyDivide\";").as_i64().unwrap();
    assert!(tag != 0);
    assert!(!truthy(&ok(&mut c, &format!("setNodeTypeFlag -q -dsp {tag};"))));
    // The Node Editor leaves it out, unless asked for by name.
    ok(&mut c, "polyCube -n box; createNode multiplyDivide -n md; connectAttr box.tx md.input1X; connectAttr md.outputX box.ty;");
    let graph = aztek_host::query::node_graph(&c, r#"{"roots": ["box"], "depth": 1}"#);
    let names: Vec<String> = graph["nodes"].as_array().unwrap().iter().map(|n| n["name"].as_str().unwrap().to_string()).collect();
    assert!(!names.contains(&"md".to_string()), "{names:?}");
    let graph = aztek_host::query::node_graph(&c, r#"{"roots": ["md"], "depth": 1}"#);
    assert!(graph["nodes"].as_array().unwrap().iter().any(|n| n["name"] == "md"));
    ok(&mut c, &format!("setNodeTypeFlag -display true {tag};"));
    assert!(truthy(&ok(&mut c, "setNodeTypeFlag -q -display multiplyDivide;")));
    let graph = aztek_host::query::node_graph(&c, r#"{"roots": ["box"], "depth": 1}"#);
    assert!(graph["nodes"].as_array().unwrap().iter().any(|n| n["name"] == "md"));
    // Thread safety is obsolete: accepted, no effect.
    assert_eq!(ok(&mut c, "setNodeTypeFlag -threadSafe true multiplyDivide;"), json!(true));
    assert!(bad(&mut c, "setNodeTypeFlag -display false noSuchType;").contains("Unknown node type"));
}

// ---------------------------------------------------------------------------------------------------------------------
// shapeCompare

#[test]
fn shape_compare_meshes_curves_and_surfaces() {
    let mut c = core();
    ok(&mut c, "polySphere; polySphere; select -cl; select -r pSphere1 pSphere2;");
    assert_eq!(ok(&mut c, "shapeCompare;"), json!(0));
    // A transform doesn't count: the shapes are compared.
    ok(&mut c, "move 5 0 0 pSphere2;");
    assert_eq!(ok(&mut c, "shapeCompare pSphere1 pSphere2;"), json!(0));
    ok(&mut c, "move -r 0 1 0 pSphere2.vtx[3];");
    assert_eq!(ok(&mut c, "shapeCompare pSphere1 pSphere2;"), json!(1));
    ok(&mut c, "polySphere -sx 10;");
    assert_eq!(ok(&mut c, "shapeCompare pSphere1 pSphere3;"), json!(1));
    ok(&mut c, "curve -n c1 -d 1 -p 0 0 0 -p 1 0 0; curve -n c2 -d 1 -p 0 0 0 -p 1 0 0; curve -n c3 -d 1 -p 0 0 0 -p 2 0 0;");
    assert_eq!(ok(&mut c, "shapeCompare c1 c2;"), json!(0));
    assert_eq!(ok(&mut c, "shapeCompare c1 c3;"), json!(1));
    assert_eq!(ok(&mut c, "shapeCompare c1 pSphere1;"), json!(1));
    ok(&mut c, "sphere -n s1; sphere -n s2;");
    assert_eq!(ok(&mut c, "shapeCompare s1 s2;"), json!(0));
    assert!(bad(&mut c, "select -r pSphere1; shapeCompare;").contains("two shapes"));
}

// ---------------------------------------------------------------------------------------------------------------------
// suitePrefs

#[test]
fn suite_prefs_answer_as_a_standalone_install() {
    let mut c = core();
    assert!(!truthy(&ok(&mut c, "suitePrefs -q -installedAsSuite;")));
    assert!(!truthy(&ok(&mut c, "suitePrefs -q -isCompleteSuite;")));
    assert!(!truthy(&ok(&mut c, "suitePrefs -installedAsSuite;")));
    ok(&mut c, "suitePrefs -applyToSuite \"3dsMax\";");
    assert_eq!(ok(&mut c, "suitePrefs -q -applyToSuite;"), json!("3dsMax"));
    ok(&mut c, "suitePrefs -applyToSuite 1;");
    assert_eq!(ok(&mut c, "suitePrefs -q -ats;"), json!("Maya"));
    ok(&mut c, "suitePrefs -ats \"undefined\";");
    assert_eq!(ok(&mut c, "suitePrefs -q -applyToSuite;"), json!("undefined"));
    assert!(bad(&mut c, "suitePrefs -applyToSuite \"Blender\";").contains("applyToSuite"));
}

// ---------------------------------------------------------------------------------------------------------------------
// switchTable

#[test]
fn switch_table_shows_a_shading_switch() {
    let mut c = core();
    ok(&mut c, "polySphere -n ball; polyCube -n box; shadingNode -asUtility tripleShadingSwitch -n sw; shadingNode -asTexture checker -n chk; shadingNode -asTexture ramp -n rmp;");
    ok(&mut c, "connectAttr ballShape.message sw.input[0].inShape; connectAttr chk.outColor sw.input[0].inTriple;");
    ok(&mut c, "connectAttr boxShape.message sw.input[1].inShape; connectAttr rmp.outColor sw.input[1].inTriple;");
    ok(&mut c, "window -width 200 w1; formLayout theForm;");
    let _ = aztek_host::scriptui::take_ops();
    let made = ok(&mut c, "switchTable theSwitch;");
    assert!(made.as_str().unwrap().ends_with("theSwitch"));
    ok(&mut c, "formLayout -e -af theSwitch top 0 -af theSwitch left 0 -af theSwitch bottom 0 -af theSwitch right 0 theForm;");
    assert!(truthy(&ok(&mut c, "switchTable -exists theSwitch;")));
    assert_eq!(ok(&mut c, "switchTable -q -label1 theSwitch;"), json!("Shape"));
    assert_eq!(ok(&mut c, "switchTable -q -selectedRow theSwitch;"), json!(0));
    assert_eq!(ok(&mut c, "switchTable -q -underPointerRow theSwitch;"), json!(0));
    ok(&mut c, "switchTable -e -switchNode sw -label1 \"Object\" -label2 \"Texture\" theSwitch;");
    assert_eq!(ok(&mut c, "switchTable -q -switchNode theSwitch;"), json!("sw"));
    assert_eq!(ok(&mut c, "switchTable -q -l2 theSwitch;"), json!("Texture"));
    // The rows go to the shell: each shape and what feeds its value.
    let ops = aztek_host::scriptui::take_ops();
    let cells = ops.iter().rev().find_map(|o| o["props"].get("cells").cloned()).expect("cells sent");
    assert_eq!(cells, json!([["ballShape", "chk.outColor"], ["boxShape", "rmp.outColor"]]));
    // What the shell reports back.
    let path = made.as_str().unwrap().to_string();
    aztek_host::scriptui::shell_event(&json!({"path": path, "set": {"selectedRow": 2}}));
    assert_eq!(ok(&mut c, "switchTable -q -sr theSwitch;"), json!(2));
    // Common control flags.
    ok(&mut c, "switchTable -e -ann \"pick a row\" -w 300 -h 120 theSwitch;");
    assert_eq!(ok(&mut c, "switchTable -q -annotation theSwitch;"), json!("pick a row"));
    assert_eq!(ok(&mut c, "switchTable -q -width theSwitch;"), json!(300));
    assert!(bad(&mut c, "switchTable -e -switchNode ball theSwitch;").contains("shading switch"));
    // The switch node's output is its default.
    ok(&mut c, "setAttr sw.default 0.2 0.3 0.4;");
    let out = ok(&mut c, "getAttr sw.output;");
    let v: Vec<f64> = out.as_array().unwrap().iter().flat_map(|x| x.as_array().cloned().unwrap_or_else(|| vec![x.clone()])).filter_map(|x| x.as_f64()).collect();
    assert_eq!(v.len(), 3);
    assert!((v[0] - 0.2).abs() < 1e-9 && (v[2] - 0.4).abs() < 1e-9, "{v:?}");
}

// ---------------------------------------------------------------------------------------------------------------------
// tolerance

#[test]
fn tolerance_sets_what_modelling_takes_by_default() {
    let _l = lock();
    let mut c = core();
    let lin = ok(&mut c, "tolerance -q -linear;").as_f64().unwrap();
    assert!((lin - 0.01).abs() < 1e-12);
    let ang = ok(&mut c, "tolerance -q -angular;").as_f64().unwrap();
    assert!((ang - 0.1).abs() < 1e-9, "{ang}");
    ok(&mut c, "tolerance -linear 0.02;");
    assert!((ok(&mut c, "tolerance -q -l;").as_f64().unwrap() - 0.02).abs() < 1e-12);
    // In the current unit, and with a unit given.
    ok(&mut c, "currentUnit -l mm;");
    assert!((ok(&mut c, "tolerance -q -linear;").as_f64().unwrap() - 0.2).abs() < 1e-9);
    ok(&mut c, "tolerance -linear 0.05cm;");
    assert!((ok(&mut c, "tolerance -q -linear;").as_f64().unwrap() - 0.5).abs() < 1e-9);
    ok(&mut c, "currentUnit -l cm;");
    assert!((ok(&mut c, "tolerance -q -linear;").as_f64().unwrap() - 0.05).abs() < 1e-9);
    ok(&mut c, "tolerance -angular 0.02rad;");
    let deg = ok(&mut c, "tolerance -q -a;").as_f64().unwrap();
    assert!((deg - 0.02_f64.to_degrees()).abs() < 1e-9, "{deg}");
    // A bevel with no tolerance of its own takes the linear one.
    ok(&mut c, "circle -n crv; bevel -n bv crv;");
    let t = ok(&mut c, "getAttr bevel1.tolerance;").as_f64().unwrap();
    assert!((t - 0.05).abs() < 1e-9, "{t}");
    assert!(bad(&mut c, "tolerance -linear 0;").contains("greater than 0"));
    ok(&mut c, "tolerance -linear 0.01 -angular 0.1;");
}

// ---------------------------------------------------------------------------------------------------------------------
// unknownPlugin

#[test]
fn unknown_plugins_are_listed_kept_and_removed() {
    let mut c = core();
    let d = dir("unknown");
    let file = format!("{d}/scene.ma");
    std::fs::write(
        &file,
        "//Maya ASCII 2027 scene\nrequires maya \"2027\";\nrequires \"fooPlug\" \"1.5\";\nrequires -nodeType \"barNode\" -dataType \"barData\" \"barPlug\" \"2.0\";\n\
         currentUnit -l centimeter -a degree -t film;\ncreateNode transform -n \"pCube1\";\ncreateNode barNode -n \"bar1\";\n\tsetAttr \".k\" 3;\n",
    )
    .unwrap();
    ok(&mut c, &format!("file -f -o \"{file}\";"));
    let list = ok(&mut c, "unknownPlugin -q -list;");
    assert_eq!(list, json!(["barPlug", "fooPlug"]));
    assert_eq!(ok(&mut c, "unknownPlugin -q -v \"fooPlug\";"), json!("1.5"));
    assert_eq!(ok(&mut c, "unknownPlugin -q -version \"barPlug\";"), json!("2.0"));
    assert_eq!(ok(&mut c, "unknownPlugin -q -nodeTypes \"barPlug\";"), json!(["barNode"]));
    assert_eq!(ok(&mut c, "unknownPlugin -q -dt \"barPlug\";"), json!(["barData"]));
    assert_eq!(ok(&mut c, "unknownPlugin -q -nt \"fooPlug\";"), json!([]));
    // Written back as they came.
    let out = format!("{d}/saved.ma");
    ok(&mut c, &format!("file -rename \"{out}\"; file -f -type \"mayaAscii\" -save;"));
    let text = std::fs::read_to_string(&out).unwrap();
    assert!(text.contains("requires \"fooPlug\" \"1.5\";"), "{text}");
    assert!(text.contains("-nodeType \"barNode\"") && text.contains("-dataType \"barData\""), "{text}");
    // barPlug's node is in the scene: it stays.
    assert!(bad(&mut c, "unknownPlugin -r \"barPlug\";").contains("in the scene"));
    ok(&mut c, "unknownPlugin -remove \"fooPlug\";");
    assert_eq!(ok(&mut c, "unknownPlugin -q -l;"), json!(["barPlug"]));
    ok(&mut c, "delete bar1; unknownPlugin -r \"barPlug\";");
    assert_eq!(ok(&mut c, "unknownPlugin -q -l;"), json!([]));
    assert!(bad(&mut c, "unknownPlugin -q -v \"nope\";").contains("not an unknown plug-in"));
    ok(&mut c, "file -f -new;");
}

// ---------------------------------------------------------------------------------------------------------------------
// vectorize

fn scene(c: &mut Core) {
    ok(c, "polySphere -r 2 -n ball; polyCube -n box; move 4 0 0 box; setAttr persp.translate 0 3 20; setAttr persp.rotate -8 0 0;");
}

#[test]
fn vectorize_writes_svg_eps_and_illustrator() {
    let mut c = core();
    scene(&mut c);
    let d = dir("vec");
    ok(&mut c, &format!("vectorize -imageFormat svg -w 320 -h 240 -fs SingleColor -c persp -cf -ff \"name.ext\" -of \"{d}/untitled\";"));
    let svg = std::fs::read_to_string(format!("{d}/untitled.svg")).unwrap();
    assert!(svg.contains("<svg") && svg.contains("width=\"320\"") && svg.contains("height=\"240\""));
    let polys = svg.matches("<polygon").count();
    assert!(polys > 50, "{polys}");
    // Outline edges by default; EntireMesh draws far more; None draws none.
    let outline = svg.matches("<line").count();
    assert!(outline > 0);
    ok(&mut c, &format!("vectorize -if svg -es EntireMesh -of \"{d}/all\";"));
    let all = std::fs::read_to_string(format!("{d}/all.svg")).unwrap();
    assert!(all.matches("<line").count() > outline * 2);
    ok(&mut c, &format!("vectorize -if svg -es None -fs None -of \"{d}/none\";"));
    let none = std::fs::read_to_string(format!("{d}/none.svg")).unwrap();
    assert_eq!(none.matches("<line").count() + none.matches("<polygon").count(), 0);
    // Back faces add more faces; gradients add gradient fills; edge colour and weight.
    ok(&mut c, &format!("vectorize -if svg -sb -fs AreaGradient -ec 255 0 0 -ew 2.5 -of \"{d}/grad\";"));
    let grad = std::fs::read_to_string(format!("{d}/grad.svg")).unwrap();
    assert!(grad.matches("<polygon").count() > polys);
    assert!(grad.contains("<linearGradient") && grad.contains("stroke=\"#ff0000\"") && grad.contains("stroke-width=\"2.5\""));
    // Two colours: at most a handful of distinct fills.
    ok(&mut c, &format!("vectorize -if svg -fs TwoColor -es None -of \"{d}/two\";"));
    let two = std::fs::read_to_string(format!("{d}/two.svg")).unwrap();
    let fills: std::collections::HashSet<&str> = two.match_indices("fill=\"#").map(|(i, _)| &two[i + 6..i + 13]).collect();
    assert!(fills.len() <= 2, "{fills:?}");
    // Compressed SVG.
    ok(&mut c, &format!("vectorize -if svg -svgCompression -of \"{d}/zip\";"));
    let z = std::fs::read(format!("{d}/zip.svgz")).unwrap();
    assert_eq!(&z[..2], &[0x1f, 0x8b]);
    // EPS, one file a frame.
    ok(&mut c, &format!("vectorize -if eps -sf 1 -ef 2 -of \"{d}/frames\";"));
    let eps = std::fs::read_to_string(format!("{d}/frames.1.eps")).unwrap();
    assert!(eps.starts_with("%!PS-Adobe-3.0 EPSF-3.0") && eps.contains("%%BoundingBox: 0 0 ") && eps.contains("fill"));
    assert!(std::path::Path::new(&format!("{d}/frames.2.eps")).exists());
    ok(&mut c, &format!("vectorize -if eps -sf 1 -ef 2 -ff \"name_#.ext\" -ce \"ps1\" -of \"{d}/named\";"));
    assert!(std::path::Path::new(&format!("{d}/named_1.ps1")).exists());
    // Illustrator: its paths read back as curves.
    ok(&mut c, &format!("vectorize -if ai -es None -of \"{d}/art\";"));
    let ai = format!("{d}/art.ai");
    let text = std::fs::read_to_string(&ai).unwrap();
    assert!(text.contains("%%AI5_FileFormat") && text.contains(" m\n") && text.contains(" L\n"));
    ok(&mut c, &format!("illustratorCurves -ifn \"{ai}\";"));
    let curves = ok(&mut c, "ls -type nurbsCurve;");
    assert!(curves.as_array().is_some_and(|v| v.len() > 50), "{curves}");
}

#[test]
fn vectorize_writes_flash_and_images() {
    let mut c = core();
    scene(&mut c);
    ok(&mut c, "setKeyframe -t 1 -v 0 -at tx ball; setKeyframe -t 3 -v -4 -at tx ball;");
    let d = dir("swf");
    ok(&mut c, &format!("vectorize -imageFormat swf -w 320 -h 240 -sf 1 -ef 3 -fr 24 -fv 4 -of \"{d}/anim\";"));
    let swf = std::fs::read(format!("{d}/anim.swf")).unwrap();
    assert_eq!(&swf[..3], b"FWS");
    assert_eq!(swf[3], 4);
    assert_eq!(u32::from_le_bytes([swf[4], swf[5], swf[6], swf[7]]) as usize, swf.len());
    // After the frame rectangle: rate 24.0 and three frames.
    let nbits = (swf[8] >> 3) as usize;
    let rect_bytes = (5 + 4 * nbits).div_ceil(8);
    let at = 8 + rect_bytes;
    assert_eq!(u16::from_le_bytes([swf[at], swf[at + 1]]), 24 * 256);
    assert_eq!(u16::from_le_bytes([swf[at + 2], swf[at + 3]]), 3);
    assert!(bad(&mut c, "vectorize -if swf -fv 7;").contains("flashVersion"));
    // The current time is back where it was.
    assert_eq!(ok(&mut c, "currentTime -q;").as_f64(), Some(1.0));
    // Combined fills and edges make fewer shapes.
    ok(&mut c, &format!("vectorize -if swf -cf -of \"{d}/split\"; vectorize -if swf -cf -cfe -of \"{d}/combined\";"));
    let split = std::fs::read(format!("{d}/split.swf")).unwrap();
    let combined = std::fs::read(format!("{d}/combined.swf")).unwrap();
    assert!(combined.len() < split.len());
    // SVG animation: one file, a group a frame.
    ok(&mut c, &format!("vectorize -if svg -sf 1 -ef 3 -of \"{d}/anim\"; vectorize -if svg -sf 1 -ef 3 -sa HTMLScript -of \"{d}/script\";"));
    let svg = std::fs::read_to_string(format!("{d}/anim.svg")).unwrap();
    assert_eq!(svg.matches("<g id=\"frame").count(), 3);
    assert!(svg.contains("<set attributeName=\"display\""));
    let script = std::fs::read_to_string(format!("{d}/script.svg")).unwrap();
    assert!(script.contains("<script") && !script.contains("<set "));
    // Images.
    ok(&mut c, &format!("vectorize -if png -w 160 -h 120 -fs FullColor -of \"{d}/pic\";"));
    let img = image::open(format!("{d}/pic.png")).unwrap().to_rgb8();
    assert_eq!(img.dimensions(), (160, 120));
    let lit = img.pixels().filter(|p| p.0 != [0, 0, 0]).count();
    assert!(lit > 500, "{lit}");
    ok(&mut c, &format!("vectorize -if tga -w 64 -h 48 -of \"{d}/pic\"; vectorize -if bmp -w 64 -h 48 -of \"{d}/pic\"; vectorize -if jpg -w 64 -h 48 -of \"{d}/pic\"; vectorize -if tif -w 64 -h 48 -of \"{d}/pic\"; vectorize -if sgi -w 64 -h 48 -of \"{d}/pic\";"));
    for e in ["tga", "bmp", "jpg", "tif"] {
        let i = image::open(format!("{d}/pic.{e}")).unwrap();
        assert_eq!((i.width(), i.height()), (64, 48), "{e}");
    }
    let sgi = std::fs::read(format!("{d}/pic.sgi")).unwrap();
    assert_eq!(&sgi[..2], &474u16.to_be_bytes());
    assert_eq!(sgi.len(), 512 + 64 * 48 * 3);
    // Render optimisation drops tiny faces.
    ok(&mut c, &format!("vectorize -if svg -w 40 -h 30 -ro Safe -of \"{d}/safe\"; vectorize -if svg -w 40 -h 30 -ro Aggressive -of \"{d}/aggr\";"));
    let safe = std::fs::read_to_string(format!("{d}/safe.svg")).unwrap().matches("<polygon").count();
    let aggr = std::fs::read_to_string(format!("{d}/aggr.svg")).unwrap().matches("<polygon").count();
    assert!(aggr < safe, "{aggr} {safe}");
    // Render layers: each into its own file, named for it.
    ok(&mut c, &format!("vectorize -if svg -cf -rl -of \"{d}/layers\"; vectorize -if svg -cf -l defaultRenderLayer -of \"{d}/one\";"));
    assert!(std::path::Path::new(&format!("{d}/layers.svg")).exists() || std::path::Path::new(&format!("{d}/layers_defaultRenderLayer.svg")).exists());
    assert!(std::path::Path::new(&format!("{d}/one_defaultRenderLayer.svg")).exists());
    assert!(bad(&mut c, "vectorize -if gif;").contains("isn't written here"));
    assert!(bad(&mut c, "vectorize -if svg -fs Spotty;").contains("fillStyle"));
    assert!(bad(&mut c, "vectorize -if svg -l ball;").contains("not a render layer"));
}

#[test]
fn every_flag_of_the_reference_is_there() {
    let c = core();
    let want: &[(&str, &str)] = &[
        ("selectPref", "ahs allowHiliteSelection asc asm aud autoSelectContainer autoSelectOutlinerSetMembers autoUseDepth ccs containerCentricSelection dcp disableComponentPopups epl expandPopupList ignoreSelectionPriority isp manipClickBoxSize mcb paintSelect paintSelectWithDepth pds pdt phs pms popupMenuSelection preSelectClosest preSelectDeadSpace preSelectHiliteSize preSelectTweakDeadSpace ps psc psd sbs sch selectTypeChangeAffectsActive selectionChildHighlightMode singleBoxSelection sld stc straightLineDistance"),
        ("selectType", "abd ac ait ak alc allComponents animBreakdown animCurve animInTangent animKeyframe animOutTangent aot bn byName c ck clm collisionModel cos curve curveKnot curveOnSurface dc dim dimension dynamicConstraint edge eg em emitter facet fc fi field fl fluid fo follicle hairSystem hl hs hull iee ig ikEndEffector imagePlane implicitGeometry ip iso isoparm jointPivot jp la lattice latticePoint light localRotationAxis locatorUV locatorXYZ lp lt luv motionTrailPoint motionTrailTangent mtp mtt nCloth nParticle nParticleShape nRigid ncl npr nps nr objectComponent ocm ol orientationLocator p particle particleShape pfe pl plane polymeshFreeEdge pr ps qbn queryByName ra rb rc rigidBody rigidConstraint rotatePivot rp sc scalePivot sculpt sd se selectHandle sf sh sk sme smf smp smu sp spc spp spr spring springComponent sr str stroke subdiv subdivMeshEdge subdivMeshFace subdivMeshPoint subdivMeshUV surfaceEdge surfaceFace surfaceKnot surfaceParameterPoint surfaceRange surfaceUV suv texture tx xyz"),
        ("setNodeTypeFlag", "display dsp q query threadSafe ts"),
        ("shapeCompare", ""),
        ("suitePrefs", "applyToSuite ats ias ics installedAsSuite isCompleteSuite"),
        ("switchTable", "ann annotation backgroundColor bgc defineTemplate dgc docTag dpc dragCallback dropCallback dt dtg e ebg edit ekf en enable enableBackground enableKeyboardFocus ex exists fpn fullPathName h height highlightColor hlc io isObscured l1 l2 label1 label2 m manage nbg noBackground npm numberOfPopupMenus p parent pma po popupMenuArray preventOverride q query sbm selectedRow sn sr statusBarMessage switchNode underPointerRow upr useTemplate ut vcc vis visible visibleChangeCommand w width"),
        ("tolerance", "a angular l linear q query"),
        ("unknownPlugin", "dataTypes dt l list nodeTypes nt q query r remove v version"),
        ("vectorize", "bf browserView bv byFrame c camera ce cf cfe combineFillsEdges ct currentFrame curveTolerance customExtension detailLevel dl ec ed edgeColor edgeDetail edgeStyle edgeWeight ef endFrame es ew ff filenameFormat fillStyle flashVersion fr frameRate fs fv h he height hi hiddenEdges highlightLevel highlights hl imageFormat l layer mea minEdgeAngle oai of outlinesAtIntersections outputFileName par pixelAspectRatio rd reflectionDepth reflections renderLayers renderOptimization renderView rf rl ro rv sa sb sc scf secondaryCurveFitting sf sh shadows showBackFaces startFrame svgAnimation svgCompression w width"),
    ];
    for (name, flags) in want {
        let def = c.s.commands.find(name).unwrap_or_else(|| panic!("{name} missing"));
        for fl in flags.split_whitespace() {
            assert!(def.flags.iter().any(|d| d.name == fl || d.short_name == fl) || matches!(fl, "q" | "query" | "e" | "edit"), "{name} -{fl}");
        }
    }
}
