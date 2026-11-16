//! hotkeySet: sets, the current one, importing a hotkey set file of the reference's kind and exporting one.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn sets_import_and_export() {
    let tmp = std::env::temp_dir().join(format!("aztek_hotkeys_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    std::env::set_var("AZTEK_PREFS", tmp.join("prefs"));
    let mut c = Core::new();
    assert_eq!(mel(&mut c, "hotkeySet -q -current;"), json!("Aztek_Default"));
    assert_eq!(mel(&mut c, "hotkeySet -q -exists Maya_Default;"), json!(true));
    assert_eq!(mel(&mut c, "hotkeySet -q -exists Mine;"), json!(false));
    // A file as the reference writes one (hand-written here: none from the reference is in the repo).
    let mhk = tmp.join("Mine.mhk");
    std::fs::write(
        &mhk,
        r#"//Maya Preference 2027 (Release 1)
//
//
hotkeySet "Mine" -source "Maya_Default" -current;
runTimeCommand
	-annotation "Print hello"
	-category "Custom Scripts"
	-commandLanguage "mel"
	-command ("print \"hello\\n\";")
	SayHello;

nameCommand
	-annotation "SayHelloNameCommand"
	-sourceType "mel"
	-command ("SayHello")
	SayHelloNameCommand;

hotkeyCtx -type "Editor" -addClient "hyperShadePanel";
hotkeyCtx -type "Editor" -addClient "outlinerPanel";
hotkey -keyShortcut "h" -alt -name ("SayHelloNameCommand");
hotkey -keyShortcut "b" -ctl -sht -name ("SayHelloNameCommand");
hotkey -keyShortcut "x" -name ("") -releaseName ("") -ctxClient "hyperShadePanel";
"#,
    )
    .unwrap();
    let p = mhk.to_string_lossy().replace('\\', "/");
    assert_eq!(mel(&mut c, &format!("hotkeySet -import \"{p}\";")), json!("Mine"));
    assert_eq!(mel(&mut c, "hotkeySet -q -current;"), json!("Mine"));
    assert_eq!(mel(&mut c, "hotkeySet -q -hotkeySetArray;"), json!(["Aztek_Default", "Mine"]));
    assert_eq!(mel(&mut c, "hotkey -q -k \"h\" -alt -name;"), json!("SayHelloNameCommand"));
    assert_eq!(mel(&mut c, "hotkeyCheck -k \"h\" -alt;"), json!("SayHelloNameCommand"));
    // Exported and read back.
    let out = tmp.join("out.mhk").to_string_lossy().replace('\\', "/");
    mel(&mut c, &format!("hotkeySet -export \"{out}\";"));
    let text = std::fs::read_to_string(&out).unwrap();
    assert!(text.contains("hotkeySet \"Mine\" -current;"), "{text}");
    assert!(text.contains("hotkey -keyShortcut \"b\" -ctl -sht -name (\"SayHelloNameCommand\");"), "{text}");
    assert!(text.contains("nameCommand -annotation \"SayHelloNameCommand\" -sourceType \"mel\" -command (\"SayHello\") SayHelloNameCommand;"), "{text}");
    mel(&mut c, "hotkeySet -current Maya_Default;");
    assert_eq!(mel(&mut c, "hotkeySet -q -current;"), json!("Aztek_Default"));
    mel(&mut c, "hotkeySet -delete Mine;");
    assert_eq!(mel(&mut c, "hotkeySet -q -hotkeySetArray;"), json!(["Aztek_Default"]));
    let _ = std::fs::remove_dir_all(&tmp);
}
