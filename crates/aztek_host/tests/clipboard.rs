//! Cut, Copy and Paste of objects, as the runtime commands CutSelected, CopySelected and PasteSelected run them: the
//! selection exported to the user's temp folder and imported back with the pasted_ prefix.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

const COPY: &str = "if (size(`ls -sl`)) { string $f = `internalVar -userTmpDir` + \"aztekClipboard.ma\"; file -force -exportSelected -type \"mayaAscii\" $f; }";
const CUT: &str = "if (size(`ls -sl`)) { string $f = `internalVar -userTmpDir` + \"aztekClipboard.ma\"; file -force -exportSelected -type \"mayaAscii\" $f; delete; }";
const PASTE: &str = "string $f = `internalVar -userTmpDir` + \"aztekClipboard.ma\"; if (`filetest -r $f`) { string $new[] = `file -import -type \"mayaAscii\" -renameAll true -renamingPrefix \"pasted_\" -returnNewNodes $f`; string $top[] = `ls -assemblies $new`; if (size($top)) select -r $top; }";

#[test]
fn copy_cut_and_paste() {
    let tmp = std::env::temp_dir().join(format!("aztek_clip_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    std::env::set_var("AZTEK_PREFS", tmp.join("prefs"));
    std::env::set_var("TMPDIR", &tmp);
    let mut c = Core::new();
    mel(&mut c, "polyCube -n box; move 1 2 3; group -n grp box; select grp;");
    mel(&mut c, COPY);
    mel(&mut c, PASTE);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["pasted__grp"]));
    assert_eq!(mel(&mut c, "listRelatives -c pasted__grp;"), json!(["pasted__box"]));
    assert_eq!(mel(&mut c, "xform -q -ws -t pasted__box;"), json!([1.0, 2.0, 3.0]));
    // A second paste takes the next number.
    mel(&mut c, PASTE);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["pasted__grp1"]));
    // Cut takes the original away; paste still works.
    mel(&mut c, "select grp;");
    mel(&mut c, CUT);
    assert_eq!(mel(&mut c, "objExists grp;"), json!(false));
    mel(&mut c, PASTE);
    assert_eq!(mel(&mut c, "ls -sl;"), json!(["pasted__grp2"]));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn import_into_a_namespace() {
    let tmp = std::env::temp_dir().join(format!("aztek_ns_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let f = tmp.join("box.ma").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, &format!("polyCube -n box; select box; file -force -exportSelected -type \"mayaAscii\" \"{f}\";"));
    assert_eq!(mel(&mut c, &format!("file -import -namespace \"lib\" -returnNewNodes \"{f}\";")), json!(["lib:box", "lib:boxShape"]));
    assert_eq!(mel(&mut c, &format!("file -import -namespace \"lib\" -returnNewNodes \"{f}\";")), json!(["lib1:box", "lib1:boxShape"]));
    assert_eq!(mel(&mut c, &format!("file -import -namespace \"lib\" -mergeNamespacesOnClash true -returnNewNodes \"{f}\";")), json!(["lib:box1", "lib:box1|lib:boxShape"]));
    assert_eq!(mel(&mut c, "namespaceInfo -lon;"), json!(["lib", "lib1"]));
    let _ = std::fs::remove_dir_all(&tmp);
}
