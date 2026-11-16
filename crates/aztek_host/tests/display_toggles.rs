//! Display > Transform Display toggles live on the node (displayLocalAxis and the like), so they are undone and saved
//! with the scene in both formats, with template and drawing overrides.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn toggles_are_attributes_and_survive_both_formats() {
    let tmp = std::env::temp_dir().join(format!("aztek_toggles_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let mut c = Core::new();
    mel(&mut c, "polyCube -n box; select box; toggle -localAxis; toggle -rotatePivot -state true; toggle -selectHandle; toggle -template;");
    mel(&mut c, "setAttr box.overrideEnabled 1; setAttr box.overrideColor 13;");
    assert_eq!(mel(&mut c, "getAttr box.displayLocalAxis;"), json!(true));
    assert_eq!(mel(&mut c, "getAttr box.displayRotatePivot;"), json!(true));
    assert_eq!(mel(&mut c, "getAttr box.displayHandle;"), json!(true));
    assert_eq!(mel(&mut c, "getAttr box.displayScalePivot;"), json!(false));
    for ext in ["ma", "mb"] {
        let f = tmp.join(format!("t.{ext}")).to_string_lossy().replace('\\', "/");
        let ty = if ext == "ma" { "mayaAscii" } else { "mayaBinary" };
        mel(&mut c, &format!("file -rename \"{f}\"; file -save -type \"{ty}\";"));
        let mut d = Core::new();
        mel(&mut d, &format!("file -open -force \"{f}\";"));
        for (attr, want) in [("displayLocalAxis", true), ("displayRotatePivot", true), ("displayHandle", true), ("displayScalePivot", false), ("template", true), ("overrideEnabled", true)] {
            assert_eq!(mel(&mut d, &format!("getAttr box.{attr};")), json!(want), "{ext} {attr}");
        }
        assert_eq!(mel(&mut d, "getAttr box.overrideColor;"), json!(13), "{ext}");
    }
    mel(&mut c, "select box; toggle -localAxis;");
    assert_eq!(mel(&mut c, "getAttr box.displayLocalAxis;"), json!(false));
    mel(&mut c, "undo;");
    assert_eq!(mel(&mut c, "getAttr box.displayLocalAxis;"), json!(true));
    let _ = std::fs::remove_dir_all(&tmp);
}
