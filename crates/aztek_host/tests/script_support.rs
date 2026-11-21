//! Commands the reference's bundled scripts lean on: renderer and preferredRenderer, listAnimatable, sets' notes,
//! filterExpand on mixed selections and groups, polyProjection into a named UV set, hide -clearLastHidden.

use aztek_host::Core;
use serde_json::json;

fn ok(c: &mut Core, src: &str) -> serde_json::Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn renderers_register_and_answer() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "renderer -exists arnold;"), json!(true));
    assert_eq!(ok(&mut c, "renderer -q -rendererUIName arnold;"), json!("Harold"));
    ok(&mut c, "renderer -rendererUIName \"Mine\" -renderProcedure \"myRender\" -addGlobalsNode \"myGlobals\" -addGlobalsTab \"Common\" \"makeCommon\" \"updateCommon\" mine;");
    assert_eq!(ok(&mut c, "renderer -q -namesOfAvailableRenderers arnold;"), json!(["arnold", "mine"]));
    assert_eq!(ok(&mut c, "renderer -q -renderProcedure mine;"), json!("myRender"));
    assert_eq!(ok(&mut c, "renderer -q -globalsNodes mine;"), json!(["myGlobals"]));
    assert_eq!(ok(&mut c, "renderer -q -globalsTabCreateProcNames mine;"), json!(["makeCommon"]));
    ok(&mut c, "preferredRenderer mine;");
    assert_eq!(ok(&mut c, "preferredRenderer -q;"), json!("mine"));
    ok(&mut c, "renderer -unregisterRenderer mine;");
    assert_eq!(ok(&mut c, "renderer -exists mine;"), json!(false));
}

#[test]
fn animatable_attributes_and_types() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    let all = ok(&mut c, "listAnimatable box;");
    let list: Vec<String> = all.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert!(list.contains(&"|box.translateX".to_string()), "{list:?}");
    assert!(list.iter().any(|p| p.starts_with("|box|boxShape.")), "{list:?}");
    assert_eq!(ok(&mut c, "listAnimatable -type box;"), json!(["transform", "mesh"]));
}

#[test]
fn sets_keep_a_note() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n b; sets -em -text \"gCharacterSet\" -n chans;");
    assert_eq!(ok(&mut c, "sets -q -text chans;"), json!("gCharacterSet"));
    ok(&mut c, "sets -add chans a;");
    assert_eq!(ok(&mut c, "sets -anyMember chans a b;"), json!(true));
    assert_eq!(ok(&mut c, "sets -isMember chans a b;"), json!(false));
    ok(&mut c, "sets -e -text \"other\" chans;");
    assert_eq!(ok(&mut c, "sets -q -text chans;"), json!("other"));
    ok(&mut c, "sets -copy chans -n chans2;");
    assert_eq!(ok(&mut c, "sets -q chans2;"), json!(["a"]));
}

#[test]
fn filter_expand_reads_the_whole_selection() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; curve -d 1 -p 0 0 0 -p 1 0 0 -n crv; select -cl;");
    assert_eq!(ok(&mut c, "filterExpand -sm 31;"), json!([]));
    assert_eq!(ok(&mut c, "size(`filterExpand -sm 31 -sm 32 box.vtx[0] box.e[1] box.f[2]`);"), json!(2));
    ok(&mut c, "select -r box.vtx[0] crv;");
    assert_eq!(ok(&mut c, "filterExpand -sm 9;"), json!(["crv"]));
    ok(&mut c, "select -cl; group -n grp box; select -r grp;");
    assert_eq!(ok(&mut c, "filterExpand -sm 12;"), json!(["box"]));
}

#[test]
fn projection_into_a_named_uv_set() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; polyProjection -type Planar -md y -cm 1 -uvs \"second\" -icx 0.25 box.f[*];");
    assert_eq!(ok(&mut c, "polyUVSet -q -allUVSets box;"), json!(["map1", "second"]));
    assert_eq!(ok(&mut c, "polyUVSet -q -currentUVSet box;"), json!(["map1"]));
    assert_eq!(ok(&mut c, "getAttr polyPlanarProj1.imageCenterX;"), json!(0.25));
}

#[test]
fn hide_can_forget_what_it_hid() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; hide -clearLastHidden box;");
    assert_eq!(ok(&mut c, "getAttr box.visibility;"), json!(false));
    assert_eq!(ok(&mut c, "showLastHidden;"), json!([]));
}

#[test]
fn run_time_commands_keep_their_fields() {
    let mut c = Core::new();
    ok(&mut c, "runTimeCommand -ann \"Say hi\" -cat \"Custom\" -l \"Hello\" -c \"polyCube -n fromRtc\" -k \"greet\" SayHi;");
    assert_eq!(ok(&mut c, "runTimeCommand -q -label SayHi;"), json!("Hello"));
    assert_eq!(ok(&mut c, "runTimeCommand -q -category SayHi;"), json!("Custom"));
    ok(&mut c, "runTimeCommand -e -addKeyword \"wave\" SayHi;");
    assert_eq!(ok(&mut c, "runTimeCommand -q -keywords SayHi;"), json!("greet;wave"));
    assert_eq!(ok(&mut c, "runTimeCommand -q -userCommandArray;"), json!(["SayHi"]));
    assert!(c.mel_checked("runTimeCommand -c \"x\" SayHi;").is_err());
    ok(&mut c, "runTimeCommand -e -delete SayHi;");
    assert_eq!(ok(&mut c, "runTimeCommand -exists SayHi;"), json!(false));
}

#[test]
fn repeat_last_keeps_recent_commands() {
    let mut c = Core::new();
    ok(&mut c, "repeatLast -addCommand \"polyCube\" -addCommandLabel \"Cube\"; repeatLast -ac \"polySphere\" -acl \"Sphere\";");
    assert_eq!(ok(&mut c, "repeatLast -q -nhi;"), json!(2));
    assert_eq!(ok(&mut c, "repeatLast -q -cnl 2;"), json!(["Sphere", "Cube"]));
    ok(&mut c, "repeatLast -ac \"polyCube\" -acl \"Cube\";");
    assert_eq!(ok(&mut c, "repeatLast -q -cl 2;"), json!(["polyCube", "polySphere"]));
    ok(&mut c, "repeatLast -hl 1;");
    assert_eq!(ok(&mut c, "repeatLast -q -nhi;"), json!(1));
}

#[test]
fn ls_kinds_add_up_and_install_clears() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    let both = ok(&mut c, "ls -transforms -shapes box box|boxShape;");
    assert_eq!(both, json!(["box", "boxShape"]));
    ok(&mut c, "select -r box.e[0]; polyInstallAction polyBevel; polyInstallAction;");
    assert_eq!(ok(&mut c, "polyInstallAction -q -cn;"), json!(""));
}

#[test]
fn editors_unlock_their_main_connection() {
    let mut c = Core::new();
    ok(&mut c, "window w; formLayout; outlinerEditor oe; outlinerEditor -e -lockMainConnection oe;");
    assert_eq!(ok(&mut c, "outlinerEditor -q -lockMainConnection oe;"), json!(true));
    ok(&mut c, "outlinerEditor -e -unlockMainConnection oe;");
    assert_eq!(ok(&mut c, "outlinerEditor -q -lockMainConnection oe;"), json!(false));
}

#[test]
fn key_commands_take_a_selection_connection() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -at tx box; setKeyframe -t 10 -v 5 -at tx box; selectionConnection -object box conn;");
    assert_eq!(ok(&mut c, "keyframe -q -kc conn;"), json!(2));
    ok(&mut c, "select -r box;");
    assert_eq!(ok(&mut c, "keyframe -q -kc modelList;"), json!(2));
}

#[test]
fn time_display_and_slider_flags() {
    let mut c = Core::new();
    ok(&mut c, "animDisplay -e -timeCode fulltimecode;");
    assert_eq!(ok(&mut c, "animDisplay -q -timeCode;"), json!("fulltimecode"));
    assert_eq!(ok(&mut c, "timeControl -q -showKeysCombined slider;"), json!(false));
    ok(&mut c, "timeControl -e -skc 1 -asw 1 slider;");
    assert_eq!(ok(&mut c, "timeControl -q -showKeysCombined slider;"), json!(true));
    assert_eq!(ok(&mut c, "timeControl -q -animLayerShowWeight slider;"), json!(true));
    ok(&mut c, "window -minimizeButton false -maximizeButton false w1;");
    assert_eq!(ok(&mut c, "window -q -mnb w1;"), json!(false));
}
