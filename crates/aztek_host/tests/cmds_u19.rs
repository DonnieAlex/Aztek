//! Unit u19: windowPref, with every flag: kept on close, applied to the next window, -loadAll, -saveAll, -enableAll,
//! -maximized, -parentMain and the main window state files.

use aztek_host::scriptui::take_ops as take_ui_requests;
use aztek_host::Core;
use serde_json::{json, Value};

/// The user folder in a folder of the tests' own, set before any core is made.
fn home() {
    static P: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    P.get_or_init(|| {
        let d = std::env::temp_dir().join(format!("aztek_u19_home_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        std::env::set_var("AZTEK_PREFS", d.join("prefs"));
        std::env::set_var("AZTEK_APP_DIR", d.join("app"));
    });
}

fn core() -> Core {
    home();
    Core::new()
}

/// The preferences folder, as internalVar gives it.
fn prefs(c: &mut Core) -> String {
    ok(c, "internalVar -upd;").as_str().unwrap().to_string()
}

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

#[test]
fn create_query_edit_remove() {
    let mut c = core();
    ok(&mut c, "windowPref -removeAll;");
    assert_eq!(ok(&mut c, "windowPref -ex u19A;"), json!(false));
    ok(&mut c, "windowPref -wh 300 200 -tlc 10 20 u19A;");
    assert_eq!(ok(&mut c, "windowPref -exists u19A;"), json!(true));
    assert_eq!(ok(&mut c, "windowPref -q -wh u19A;"), json!([300, 200]));
    assert_eq!(ok(&mut c, "windowPref -q -topLeftCorner u19A;"), json!([10, 20]));
    assert_eq!(ok(&mut c, "windowPref -q -w u19A;"), json!(300));
    assert_eq!(ok(&mut c, "windowPref -q -h u19A;"), json!(200));
    assert_eq!(ok(&mut c, "windowPref -q -te u19A;"), json!(10));
    assert_eq!(ok(&mut c, "windowPref -q -le u19A;"), json!(20));
    assert_eq!(ok(&mut c, "windowPref -q -max u19A;"), json!(false));
    assert_eq!(ok(&mut c, "windowPref -q -pm u19A;"), json!(true));
    ok(&mut c, "windowPref -e -width 640 -leftEdge 5 -maximized true u19A;");
    assert_eq!(ok(&mut c, "windowPref -q -widthHeight u19A;"), json!([640, 200]));
    assert_eq!(ok(&mut c, "windowPref -q -tlc u19A;"), json!([10, 5]));
    assert_eq!(ok(&mut c, "windowPref -q -maximized u19A;"), json!(true));
    ok(&mut c, "windowPref -parentMain false u19A;");
    assert_eq!(ok(&mut c, "windowPref -q -parentMain u19A;"), json!(false));
    // Edit and query need one to be kept.
    bad(&mut c, "windowPref -e -w 10 u19None;");
    bad(&mut c, "windowPref -q -w u19None;");
    ok(&mut c, "windowPref -w 10 u19B;");
    ok(&mut c, "windowPref -r u19A;");
    assert_eq!(ok(&mut c, "windowPref -ex u19A;"), json!(false));
    assert_eq!(ok(&mut c, "windowPref -ex u19B;"), json!(true));
    ok(&mut c, "windowPref -ra;");
    assert_eq!(ok(&mut c, "windowPref -ex u19B;"), json!(false));
}

#[test]
fn kept_on_close_and_applied_to_next_window() {
    let mut c = core();
    ok(&mut c, "windowPref -enableAll true; if (`windowPref -ex u19W`) windowPref -remove u19W;");
    // A shown window leaves its size and place when deleted.
    ok(&mut c, "window -wh 410 220 -te 30 -le 40 u19W; showWindow u19W; deleteUI -window u19W;");
    assert_eq!(ok(&mut c, "windowPref -exists u19W;"), json!(true));
    assert_eq!(ok(&mut c, "windowPref -q -wh u19W;"), json!([410, 220]));
    assert_eq!(ok(&mut c, "windowPref -q -tlc u19W;"), json!([30, 40]));
    // The next window of that name takes them, and the shell is told.
    take_ui_requests();
    ok(&mut c, "window u19W;");
    assert_eq!(ok(&mut c, "window -q -wh u19W;"), json!([410, 220]));
    assert_eq!(ok(&mut c, "window -q -tlc u19W;"), json!([30, 40]));
    let reqs = take_ui_requests();
    let edit = reqs.iter().find(|r| r["op"] == "edit" && r["path"] == "u19W").expect("edit op");
    assert_eq!(edit["props"]["topLeftCorner"], json!([30, 40]));
    assert_eq!(edit["props"]["width"], json!(410));
    // Flags the call gives win over the preference.
    ok(&mut c, "deleteUI u19W; window -wh 100 90 u19W;");
    assert_eq!(ok(&mut c, "window -q -wh u19W;"), json!([100, 90]));
    assert_eq!(ok(&mut c, "window -q -te u19W;"), json!(30));
    // Closing it by its title bar keeps the place it then has.
    ok(&mut c, "window -e -wh 120 80 u19W; showWindow u19W;");
    c.ui_event(&json!({"path": "u19W", "closed": true}));
    assert_eq!(ok(&mut c, "window -ex u19W;"), json!(false));
    assert_eq!(ok(&mut c, "windowPref -q -wh u19W;"), json!([120, 80]));
    // -maximized goes to the shell with the next window.
    ok(&mut c, "windowPref -e -max 1 u19W;");
    take_ui_requests();
    ok(&mut c, "window u19W;");
    let reqs = take_ui_requests();
    assert!(reqs.iter().any(|r| r["op"] == "edit" && r["props"]["maximized"] == json!(true)), "{reqs:?}");
    // With -enableAll false windows ignore the preferences.
    ok(&mut c, "deleteUI u19W; windowPref -enableAll false;");
    assert_eq!(ok(&mut c, "windowPref -q -enableAll;"), json!(false));
    ok(&mut c, "window u19W;");
    assert_eq!(ok(&mut c, "window -q -wh u19W;"), json!([546, 350]));
    ok(&mut c, "deleteUI u19W; windowPref -enableAll true;");
    assert_eq!(ok(&mut c, "windowPref -q -ea;"), json!(true));
    // Removed, the next window is made at the default size.
    ok(&mut c, "windowPref -remove u19W; window u19W;");
    assert_eq!(ok(&mut c, "window -q -wh u19W;"), json!([546, 350]));
    ok(&mut c, "deleteUI u19W;");
    // A window never shown leaves none.
    ok(&mut c, "windowPref -remove u19W; window u19W; deleteUI u19W;");
    assert_eq!(ok(&mut c, "windowPref -ex u19W;"), json!(false));
}

#[test]
fn save_all_and_load_all() {
    let mut c = core();
    let p = prefs(&mut c);
    ok(&mut c, "windowPref -removeAll; windowPref -wh 333 222 -tlc 11 22 -max 1 -pm 0 \"u19 Saved\"; windowPref -w 50 u19Other;");
    ok(&mut c, "windowPref -saveAll;");
    let text = std::fs::read_to_string(format!("{p}windowPrefs.mel")).unwrap();
    assert!(text.contains("windowPref -topEdge 11 -leftEdge 22 -width 333 -height 222 -maximized 1 -parentMain 0 \"u19 Saved\";"), "{text}");
    ok(&mut c, "windowPref -ra;");
    assert_eq!(ok(&mut c, "windowPref -ex \"u19 Saved\";"), json!(false));
    ok(&mut c, "windowPref -la;");
    assert_eq!(ok(&mut c, "windowPref -q -wh \"u19 Saved\";"), json!([333, 222]));
    assert_eq!(ok(&mut c, "windowPref -q -tlc \"u19 Saved\";"), json!([11, 22]));
    assert_eq!(ok(&mut c, "windowPref -q -max \"u19 Saved\";"), json!(true));
    assert_eq!(ok(&mut c, "windowPref -q -pm \"u19 Saved\";"), json!(false));
    assert_eq!(ok(&mut c, "windowPref -q -w u19Other;"), json!(50));
    // The file is also MEL that runs.
    ok(&mut c, &format!("windowPref -ra; source \"{p}windowPrefs.mel\";"));
    assert_eq!(ok(&mut c, "windowPref -q -wh \"u19 Saved\";"), json!([333, 222]));
    // A file of the reference's two-value form loads too.
    std::fs::write(format!("{p}windowPrefs.mel"), "// prefs\nwindowPref -topLeftCorner 5 6 -widthHeight 70 80 u19Pair;\n").unwrap();
    ok(&mut c, "windowPref -loadAll;");
    assert_eq!(ok(&mut c, "windowPref -q -wh u19Pair;"), json!([70, 80]));
    assert_eq!(ok(&mut c, "windowPref -q -tlc u19Pair;"), json!([5, 6]));
}

#[test]
fn main_window_state() {
    let mut c = core();
    let p = prefs(&mut c);
    ok(&mut c, "workspaceControl -l \"U19\" -floating true u19Wc;");
    ok(&mut c, "workspaceControlState -tlc 15 25 -wh 300 250 u19Wc;");
    ok(&mut c, "windowPref -saveMainWindowState \"u19State\";");
    let text = std::fs::read_to_string(format!("{p}u19State")).unwrap();
    assert!(text.contains("u19Wc"), "{text}");
    ok(&mut c, "workspaceControlState -remove u19Wc;");
    assert_eq!(ok(&mut c, "workspaceControlState -ex u19Wc;"), json!(false));
    ok(&mut c, "windowPref -restoreMainWindowState \"u19State\";");
    assert_eq!(ok(&mut c, "workspaceControlState -q -tlc u19Wc;"), json!([15, 25]));
    assert_eq!(ok(&mut c, "workspaceControlState -q -wh u19Wc;"), json!([300, 250]));
    bad(&mut c, "windowPref -rms \"u19NoSuchState\";");
}
