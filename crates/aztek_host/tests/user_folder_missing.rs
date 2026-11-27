//! Start-up asks where the user folder is when there is none to use: on the first start, and again when Aztek's own
//! folder has been deleted or a linked one has gone. Choosing Aztek's own makes it.

use aztek_host::Core;
use serde_json::{json, Value};

// The paths and the choice are the process's, so everything runs in one test, in turn.
#[test]
fn a_missing_user_folder_is_asked_about_and_made_when_chosen() {
    let base = std::env::temp_dir().join(format!("aztek_user_folder_missing_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (theirs, own, prefs) = (base.join("maya"), base.join("aztek"), base.join("prefs"));
    std::env::set_var("HOME", &base);
    std::env::set_var("AZTEK_PREFS", &prefs);
    std::env::set_var("AZTEK_APP_DIR", &own);
    std::env::remove_var("MAYA_APP_DIR");
    let mut c = Core::new();
    let ask = |c: &mut Core| -> Value { c.exec("aztekUserFolder", json!({"query": true})).unwrap() };

    // First start, nothing anywhere: asked, with no folder of the reference's to offer.
    let q = ask(&mut c);
    assert_eq!(q["needsChoice"], json!(true), "{q}");
    assert!(q["detected"].is_null());

    // Aztek's own chosen: made, with the folders the paths look in, and not asked again.
    c.exec("aztekUserFolder", json!({"mode": "aztek"})).unwrap();
    for sub in ["scripts", "plug-ins", "modules", "prefs"] {
        assert!(own.join(sub).is_dir(), "{sub}");
    }
    assert_eq!(ask(&mut c)["needsChoice"], json!(false));

    // Deleted since: asked again.
    std::fs::remove_dir_all(&own).unwrap();
    assert_eq!(ask(&mut c)["needsChoice"], json!(true));

    // Copied from an empty folder of the reference's: the folder is made all the same.
    std::fs::create_dir_all(&theirs).unwrap();
    c.exec("aztekUserFolder", json!({"copyFrom": theirs.to_string_lossy()})).unwrap();
    assert!(own.is_dir());
    assert_eq!(ask(&mut c)["needsChoice"], json!(false));

    // Linked to the reference's folder, which then goes: asked again.
    c.exec("aztekUserFolder", json!({"mode": "link", "folder": theirs.to_string_lossy()})).unwrap();
    assert_eq!(ask(&mut c)["needsChoice"], json!(false));
    std::fs::remove_dir_all(&theirs).unwrap();
    assert_eq!(ask(&mut c)["needsChoice"], json!(true));
    let _ = std::fs::remove_dir_all(&base);
}
