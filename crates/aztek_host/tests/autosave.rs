//! autoSave: settings kept, -perform writing numbered copies that leave the scene's own name alone, the limit on how
//! many are kept, and the newest found for recovery.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn performs_numbered_copies_and_keeps_the_limit() {
    let tmp = std::env::temp_dir().join(format!("aztek_autosave_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    std::env::set_var("AZTEK_PREFS", tmp.join("prefs"));
    let dir = tmp.join("saves").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, &format!("autoSave -enable true -interval 30 -destination 1 -folder \"{dir}\" -limitBackups true -maxBackups 2;"));
    assert_eq!(mel(&mut c, "autoSave -q -enable;"), json!(true));
    assert_eq!(mel(&mut c, "autoSave -q -interval;"), json!(30.0));
    assert_eq!(mel(&mut c, "autoSave -q -destinationFolder;"), json!(dir));
    let scene = tmp.join("shot.ma").to_string_lossy().replace('\\', "/");
    mel(&mut c, &format!("polyCube; file -rename \"{scene}\"; file -save -type \"mayaAscii\";"));
    let a = mel(&mut c, "autoSave -perform;");
    assert_eq!(a, json!(format!("{dir}/shot.0001.ma")));
    mel(&mut c, "polySphere;");
    mel(&mut c, "autoSave -perform;");
    let third = mel(&mut c, "autoSave -perform;");
    assert_eq!(third, json!(format!("{dir}/shot.0003.ma")));
    // Two kept; the scene is still shot.ma.
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
    names.sort();
    assert_eq!(names, vec!["shot.0002.ma", "shot.0003.ma"]);
    assert_eq!(mel(&mut c, "file -q -sceneName;"), json!(scene));
    assert_eq!(mel(&mut c, "autoSave -q -latestFile;"), third);
    // The copy holds the scene.
    let text = std::fs::read_to_string(format!("{dir}/shot.0003.ma")).unwrap();
    assert!(text.contains("pSphere1") && text.contains("pCube1"));
    // Settings outlive the session.
    let mut d = Core::new();
    assert_eq!(mel(&mut d, "autoSave -q -maxBackups;"), json!(2));
    let _ = std::fs::remove_dir_all(&tmp);
}
