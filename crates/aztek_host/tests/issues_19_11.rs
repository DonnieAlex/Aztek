//! Issues of 19/11/2026.

use aztek_host::Core;
use serde_json::Value;

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

/// #169: a shiboken6 of another version than the PySide6 is an ImportError that says so, not an abort as PySide loads.
#[test]
fn mismatched_shiboken_is_an_import_error() {
    let dir = std::env::temp_dir().join(format!("aztek_169_{}", std::process::id()));
    for (pkg, v) in [("PySide6", "6.8.3"), ("shiboken6", "6.7.1")] {
        std::fs::create_dir_all(dir.join(pkg)).unwrap();
        std::fs::write(dir.join(pkg).join("__init__.py"), "").unwrap();
        std::fs::write(dir.join(pkg).join("_config.py"), format!("version = \"{v}\"\n")).unwrap();
    }
    let mut c = Core::new();
    let d = dir.to_string_lossy().replace('\\', "/");
    mel(&mut c, &format!("python(\"import sys; sys.path.insert(0, '{d}')\");"));
    mel(&mut c, "python(\"try:\\n    import PySide6\\n    why = ''\\nexcept ImportError as e:\\n    why = str(e)\");");
    let why = mel(&mut c, "python(\"why\");");
    let why = why.as_str().unwrap();
    assert!(why.contains("same version") && why.contains("6.7.1"), "{why}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// #167: HumanIK is the mayaHIK plug-in, loaded at start: pluginInfo lists its commands and nodes, unloading takes the
/// commands away and loading puts them back.
#[test]
fn humanik_is_the_mayahik_plugin() {
    let mut c = Core::new();
    assert_eq!(mel(&mut c, "pluginInfo -q -loaded mayaHIK;"), serde_json::json!(true));
    let cmds = mel(&mut c, "pluginInfo -q -command mayaHIK;");
    assert!(cmds.as_array().unwrap().iter().any(|v| v == "GetHIKNodeName"), "{cmds}");
    let nodes = mel(&mut c, "pluginInfo -q -dependNode mayaHIK;");
    assert!(nodes.as_array().unwrap().iter().any(|v| v == "HIKCharacterNode"), "{nodes}");
    assert_eq!(mel(&mut c, "hikGetNodeCount;"), mel(&mut c, "hikGetNodeCount;"));
    mel(&mut c, "unloadPlugin mayaHIK;");
    assert!(c.mel_checked("hikGetNodeCount;").is_err());
    mel(&mut c, "loadPlugin mayaHIK;");
    assert!(c.mel_checked("hikGetNodeCount;").is_ok());
}
