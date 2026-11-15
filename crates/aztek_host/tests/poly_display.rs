//! Issue 47: Polygon Display's switches, kept on each mesh as the reference keeps them.

use aztek_host::Core;
use serde_json::json;

#[test]
fn poly_options_sets_the_selected_meshes_switches() {
    let mut c = Core::new();
    c.mel_checked("polySphere -n s; polyCube -n b; select -r s;").unwrap();
    c.mel_checked("polyOptions -dmb 1 -dc 1;").unwrap();
    assert_eq!(c.mel_checked("getAttr sShape.displayMapBorders;").unwrap(), json!(true));
    assert_eq!(c.mel_checked("getAttr sShape.displayCenter;").unwrap(), json!(true));
    assert_eq!(c.mel_checked("getAttr bShape.displayCenter;").unwrap(), json!(false), "only the selection");
    // -r turns each one the other way.
    c.mel_checked("select -r s b; polyOptions -r -dc 1;").unwrap();
    assert_eq!(c.mel_checked("getAttr sShape.displayCenter;").unwrap(), json!(false));
    assert_eq!(c.mel_checked("getAttr bShape.displayCenter;").unwrap(), json!(true));
    c.mel_checked("undo;").unwrap();
    assert_eq!(c.mel_checked("getAttr bShape.displayCenter;").unwrap(), json!(false));
    c.mel_checked("select -cl; polyOptions -gl -dv 1 -dt 1 -db 1;").unwrap();
    for a in ["sShape.displayVertices", "bShape.displayTriangles", "bShape.displayBorders"] {
        assert_eq!(c.mel_checked(&format!("getAttr {a};")).unwrap(), json!(true), "{a}");
    }
}

#[test]
fn the_switches_are_saved_with_the_scene() {
    let mut c = Core::new();
    c.mel_checked("polySphere -n s; select -r s; polyOptions -dmb 1 -dt 1;").unwrap();
    let path = std::env::temp_dir().join(format!("aztek_poly_display_{}.ma", std::process::id())).display().to_string().replace('\\', "/");
    c.mel_checked(&format!("file -rename \"{path}\"; file -save -type \"mayaAscii\";")).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("setAttr \".dmb\" yes;"));
    let mut d = Core::new();
    d.mel_checked(&format!("file -f -open \"{path}\";")).unwrap();
    assert_eq!(d.mel_checked("getAttr sShape.displayMapBorders;").unwrap(), json!(true));
    assert_eq!(d.mel_checked("getAttr sShape.displayTriangles;").unwrap(), json!(true));
    assert_eq!(d.mel_checked("getAttr sShape.displayCenter;").unwrap(), json!(false));
    std::fs::remove_file(path).ok();
}
