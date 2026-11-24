//! A plug-in in two folders of the plug-in path (an mGear module's and another copy on the path) is listed once in
//! the Plug-in Manager: the copy loadPlugin takes by name.

use std::path::Path;

use aztek_host::Core;
use serde_json::Value;

#[test]
fn a_plugin_in_two_folders_is_listed_once() {
    let root = std::env::temp_dir().join(format!("aztek-plugin-once-{}", std::process::id()));
    let (a, b) = (root.join("a"), root.join("b"));
    for d in [&a, &b] {
        std::fs::create_dir_all(d).unwrap();
        // What the listing reads of a file built with the kit: the entry point's name.
        std::fs::write(d.join("dupPlugin.so"), b"\x7fELF aztek_plugin_abi").unwrap();
    }
    std::env::set_var("AZTEK_PLUG_IN_PATH", std::env::join_paths([&a, &b]).unwrap());
    let mut c = Core::new();
    let rows = c.mel_checked("aztekPlugins;").unwrap();
    let dups: Vec<&Value> = rows.as_array().unwrap().iter().filter(|r| r["name"] == "dupPlugin").collect();
    assert_eq!(dups.len(), 1, "{dups:?}");
    assert!(Path::new(dups[0]["folder"].as_str().unwrap()).ends_with("a"), "{dups:?}");
    let _ = std::fs::remove_dir_all(&root);
}
