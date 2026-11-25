//! Script UI commands existing tools rely on: nameField, timeField, timeFieldGrp, toolBar, keyframeTangentControl,
//! layoutDialog, menuSet, saveShelf, saveAllShelves, selectContext and assignCommand.

use aztek_host::Core;
use serde_json::json;

fn ok(c: &mut Core, src: &str) -> serde_json::Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    match c.mel_checked(src) {
        Ok(v) => panic!("{src} should fail, gave {v}"),
        Err(e) => e.to_string(),
    }
}

#[test]
fn name_field_follows_its_object() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    ok(&mut c, "window w; columnLayout; nameField -object box nf;");
    assert_eq!(ok(&mut c, "nameField -q -object nf;"), json!("box"));
    assert_eq!(ok(&mut c, "textField -q -text nf;"), json!("box"));
    // Renamed elsewhere: the field shows the new name and -object follows.
    ok(&mut c, "rename box crate;");
    c.idle();
    assert_eq!(ok(&mut c, "textField -q -text nf;"), json!("crate"));
    assert_eq!(ok(&mut c, "nameField -q -object nf;"), json!("crate"));
    // Another object.
    ok(&mut c, "polySphere -n ball;");
    ok(&mut c, "nameField -e -object ball nf;");
    assert_eq!(ok(&mut c, "textField -q -text nf;"), json!("ball"));
    // Typed into: the object is renamed and -nameChangeCommand runs.
    ok(&mut c, "global int $renamed = 0; nameField -e -nameChangeCommand \"$renamed = 1\" nf;");
    let path = ok(&mut c, "nameField -q -fullPathName nf;");
    c.ui_event(&json!({"path": path, "set": {"text": "orb"}, "fire": ["changeCommand"], "args": ["orb"]}));
    assert_eq!(ok(&mut c, "objExists orb;"), json!(true));
    assert_eq!(ok(&mut c, "objExists ball;"), json!(false));
    assert_eq!(ok(&mut c, "global int $renamed; $renamed;"), json!(1));
    assert_eq!(ok(&mut c, "textField -q -text nf;"), json!("orb"));
}

#[test]
fn time_fields_hold_values_as_the_float_fields_do() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout; timeField -value 12 -precision 1 tf; timeFieldGrp -numberOfFields 2 -label \"Range\" -value1 1 -value2 24 tg;");
    assert_eq!(ok(&mut c, "timeField -q -value tf;"), json!(12.0));
    assert_eq!(ok(&mut c, "timeField -q -precision tf;"), json!(1));
    ok(&mut c, "timeField -e -value 30 tf;");
    assert_eq!(ok(&mut c, "timeField -q -v tf;"), json!(30.0));
    assert_eq!(ok(&mut c, "timeFieldGrp -q -value2 tg;"), json!(24.0));
    assert_eq!(ok(&mut c, "timeFieldGrp -q -label tg;"), json!("Range"));
}

#[test]
fn tool_bar_holds_a_layout() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout col; button;");
    let made = ok(&mut c, "toolBar -area \"top\" -content col -label \"Tools\" tb;");
    assert_eq!(made, json!("tb"));
    assert_eq!(ok(&mut c, "toolBar -q -area tb;"), json!("top"));
    assert_eq!(ok(&mut c, "toolBar -q -label tb;"), json!("Tools"));
    assert_eq!(ok(&mut c, "toolBar -exists tb;"), json!(true));
    ok(&mut c, "deleteUI tb;");
    assert_eq!(ok(&mut c, "toolBar -exists tb;"), json!(false));
}

#[test]
fn keyframe_tangent_control_is_a_control() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout; keyframeTangentControl ktc;");
    assert_eq!(ok(&mut c, "keyframeTangentControl -exists ktc;"), json!(true));
    ok(&mut c, "keyframeTangentControl -e -enable 0 ktc;");
    assert_eq!(ok(&mut c, "keyframeTangentControl -q -enable ktc;"), json!(false));
}

#[test]
fn layout_dialog_builds_under_a_form_and_answers_dismiss_without_an_interface() {
    let mut c = Core::new();
    ok(&mut c, "global string $gForm = \"\"; global proc buildIt() { global string $gForm; $gForm = `setParent -q`; button -label \"OK\" -command \"layoutDialog -dismiss \\\"OK\\\"\" okB; }");
    let r = ok(&mut c, "layoutDialog -title \"Pick\" -uiScript \"buildIt\";");
    assert_eq!(r, json!("dismiss"));
    // The script ran with a formLayout as the parent; the dialog is gone afterwards.
    let form = ok(&mut c, "global string $gForm; $gForm;");
    assert!(form.as_str().unwrap_or("").contains("formLayout"), "{form}");
    assert_eq!(ok(&mut c, "control -exists okB;"), json!(false));
    // Nothing to dismiss.
    bad(&mut c, "layoutDialog -dismiss \"x\";");
}

#[test]
fn menu_sets() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "menuSet -q -currentMenuSet;"), json!("modelingMenuSet"));
    assert_eq!(ok(&mut c, "menuSet -q -numberOfMenuSets;"), json!(5));
    assert_eq!(ok(&mut c, "menuSet -q -allMenuSets;"), json!(["modelingMenuSet", "riggingMenuSet", "animationMenuSet", "fxMenuSet", "renderingMenuSet"]));
    assert_eq!(ok(&mut c, "menuSet -q -label animationMenuSet;"), json!("Animation"));
    // A set of a script's own, with menus in it.
    assert_eq!(ok(&mut c, "menuSet -label \"Mine\" mySet;"), json!("mySet"));
    ok(&mut c, "menu -label \"One\" -parent MayaWindow m1; menu -label \"Two\" -parent MayaWindow m2;");
    ok(&mut c, "menuSet -currentMenuSet mySet;");
    assert_eq!(ok(&mut c, "menuSet -q -cms;"), json!("mySet"));
    ok(&mut c, "menuSet -addMenu m1; menuSet -addMenu m2;");
    assert_eq!(ok(&mut c, "menuSet -q -numberOfMenus;"), json!(2));
    ok(&mut c, "menuSet -moveMenu m2 0;");
    let menus = ok(&mut c, "menuSet -q -menuArray mySet;");
    assert!(menus[0].as_str().unwrap().ends_with("m2"), "{menus}");
    ok(&mut c, "menuSet -removeMenu m2;");
    assert_eq!(ok(&mut c, "menuSet -q -nm mySet;"), json!(1));
    assert_eq!(ok(&mut c, "menuSet -exists mySet;"), json!(true));
    assert_eq!(ok(&mut c, "menuSet -q -permanent mySet;"), json!(false));
    ok(&mut c, "menuSet -moveMenuSet mySet 0;");
    assert_eq!(ok(&mut c, "menuSet -q -allMenuSets;")[0], json!("mySet"));
    // The reference's five stay.
    bad(&mut c, "menuSet -removeMenuSet modelingMenuSet;");
    ok(&mut c, "menuSet -removeMenuSet mySet;");
    assert_eq!(ok(&mut c, "menuSet -exists mySet;"), json!(false));
    // Removing the current set falls back to Modeling.
    assert_eq!(ok(&mut c, "menuSet -q -cms;"), json!("modelingMenuSet"));
    ok(&mut c, "menuSet -cms riggingMenuSet;");
    assert_eq!(ok(&mut c, "menuSet -q -cms;"), json!("riggingMenuSet"));
    bad(&mut c, "menuSet -cms nothingMenuSet;");
}

#[test]
fn save_shelf_writes_the_reference_shelf_file() {
    let mut c = Core::new();
    let dir = std::env::temp_dir().join(format!("aztek_shelf_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let base = dir.join("shelf_Mine");
    ok(&mut c, "window w; shelfTabLayout tabs; shelfLayout Mine;");
    ok(&mut c, "shelfButton -label \"Cube\" -annotation \"A cube\" -image \"cube.png\" -command \"polyCube\" -sourceType \"mel\";");
    ok(&mut c, "shelfButton -label \"Hi\" -command \"print 'hi'\" -sourceType \"python\";");
    let p = base.to_string_lossy().replace('\\', "/");
    assert_eq!(ok(&mut c, &format!("saveShelf Mine \"{p}\";")), json!(true));
    let text = std::fs::read_to_string(dir.join("shelf_Mine.mel")).unwrap();
    assert!(text.starts_with("global proc shelf_Mine () {"), "{text}");
    assert_eq!(text.matches("shelfButton").count(), 2, "{text}");
    assert!(text.contains("-command \"polyCube\""), "{text}");
    assert!(text.contains("-sourceType \"python\""), "{text}");
    assert!(text.contains("-annotation \"A cube\""), "{text}");
    // The file sources back into the same buttons.
    ok(&mut c, "deleteUI w; window w2; shelfTabLayout; shelfLayout Again;");
    ok(&mut c, &format!("source \"{}/shelf_Mine.mel\"; shelf_Mine;", dir.to_string_lossy().replace('\\', "/")));
    assert_eq!(ok(&mut c, "shelfLayout -q -numberOfChildren Again;"), json!(2));
    bad(&mut c, "saveShelf noSuchShelf \"/tmp/x\";");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn select_context_is_made_and_set() {
    let mut c = Core::new();
    let made = ok(&mut c, "selectContext;");
    assert!(made.as_str().unwrap().starts_with("selectContext"), "{made}");
    ok(&mut c, &format!("setToolTo {};", made.as_str().unwrap()));
    assert_eq!(ok(&mut c, "currentCtx;"), made);
    ok(&mut c, "selectContext mySel;");
    assert_eq!(ok(&mut c, "selectContext -exists mySel;"), json!(true));
}

#[test]
fn assign_command_numbers_name_commands() {
    let mut c = Core::new();
    let before = ok(&mut c, "assignCommand -q -numElements;").as_i64().unwrap();
    ok(&mut c, "nameCommand -annotation \"Make a cube\" -command \"polyCube\" aaCubeCommand;");
    assert_eq!(ok(&mut c, "assignCommand -q -numElements;"), json!(before + 1));
    let i = ok(&mut c, "assignCommand -q -index aaCubeCommand;").as_i64().unwrap();
    assert!(i >= 1);
    assert_eq!(ok(&mut c, &format!("assignCommand -q -name {i};")), json!("aaCubeCommand"));
    assert_eq!(ok(&mut c, &format!("assignCommand -q -command {i};")), json!("polyCube"));
    assert_eq!(ok(&mut c, &format!("assignCommand -q -annotation {i};")), json!("Make a cube"));
    assert_eq!(ok(&mut c, &format!("assignCommand -q -keyString {i};")), json!(["NONE", "0", "0", "0", "0"]));
    ok(&mut c, &format!("assignCommand -e -alt -keyString \"k\" {i};"));
    assert_eq!(ok(&mut c, &format!("assignCommand -q -keyString {i};")), json!(["k", "1", "0", "0", "0"]));
    assert_eq!(ok(&mut c, "hotkey -q -alt -name k;"), json!("aaCubeCommand"));
    ok(&mut c, &format!("assignCommand -e -annotation \"Cube\" {i};"));
    assert_eq!(ok(&mut c, &format!("assignCommand -q -ann {i};")), json!("Cube"));
    ok(&mut c, &format!("assignCommand -delete {i};"));
    assert_eq!(ok(&mut c, "assignCommand -q -numElements;"), json!(before));
    bad(&mut c, "assignCommand -q -name 99999;");
}
