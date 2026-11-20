//! The MEL scripts that come with Aztek (scripts/): each one sources cleanly, their procedures are found by name as the
//! reference finds its own, and the ones that do work on the scene do it.

use aztek_host::Core;
use std::path::PathBuf;

fn scripts() -> Vec<(String, String)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts");
    let mut out = Vec::new();
    for dir in ["startup", "others"] {
        for e in std::fs::read_dir(root.join(dir)).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "mel") {
                out.push((dir.to_string(), p.file_stem().unwrap().to_string_lossy().into_owned()));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn every_script_sources() {
    for (dir, name) in scripts() {
        let mut c = Core::new();
        if let Err(e) = c.mel_checked(&format!("source \"{name}.mel\";")) {
            panic!("{dir}/{name}.mel: {e}");
        }
    }
}

#[test]
fn procedures_are_found_by_name() {
    let mut c = Core::new();
    // A script's own name, a procedure in a startup script under another name, and those the reference's commands
    // call from scripts it sources at start.
    for name in ["performDuplicate", "toggleUIComponentVisibility", "FileMenu_SaveItem", "setPolygonDisplaySettings", "fileCmdCallback", "MTtoggleCmd", "renderIntoNewWindow", "fileOptions2"] {
        let v = c.mel_checked(&format!("exists {name}")).unwrap();
        assert_eq!(format!("{v:?}"), "Number(1)", "{name}");
    }
}

#[test]
fn perform_procedures_do_their_work() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; polyCube -n b; select -r a b; performGroup 0;").unwrap();
    let parent = c.mel_checked("listRelatives -p a").unwrap();
    assert!(format!("{parent:?}").contains("group1"), "{parent:?}");
    c.mel_checked("select -r a; performDuplicate 0;").unwrap();
    assert_eq!(format!("{:?}", c.mel_checked("objExists a1").unwrap()), "Bool(true)");
    let cmd = c.mel_checked("performParent 2").unwrap();
    assert!(format!("{cmd:?}").contains("parent"), "{cmd:?}");
}
