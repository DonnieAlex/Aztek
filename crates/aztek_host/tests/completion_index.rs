//! The Script Editor's completion index: the command registry with flag types, the MEL procedures, and the Python
//! modules and classes Aztek actually ships.

#![cfg(feature = "python")]

use std::ffi::{CStr, CString};

use aztek_host::*;
use serde_json::Value;

fn take(p: *mut std::ffi::c_char) -> String {
    assert!(!p.is_null());
    let s = unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned();
    aztek_free(p);
    s
}

fn index() -> Value {
    serde_json::from_str(&take(aztek_completion_index())).unwrap()
}

fn find<'a>(list: &'a Value, name: &str) -> &'a Value {
    list.as_array().unwrap().iter().find(|e| e["name"] == name).unwrap_or_else(|| panic!("{name} missing"))
}

#[test]
fn commands_carry_flags_types_and_help() {
    aztek_init();
    let v = index();
    let cube = find(&v["commands"], "polyCube");
    assert!(cube["doc"].as_str().unwrap().to_lowercase().contains("cube"));
    let flags = cube["flags"].as_array().unwrap();
    let width = flags.iter().find(|f| f["name"] == "width").expect("polyCube -width");
    assert_eq!(width["short"], "w");
    assert_eq!(width["type"], "float");
    assert!(!width["doc"].as_str().unwrap().is_empty());
    let sub = flags.iter().find(|f| f["name"] == "subdivisionsWidth").unwrap();
    assert_eq!(sub["short"], "sw");
    assert_eq!(sub["type"], "int");
    // The shell's fabricated flag is not a command flag.
    assert!(!flags.iter().any(|f| f["name"] == "queried"));
}

#[test]
fn mel_procedures_and_builtins_are_listed() {
    aztek_init();
    let v = index();
    let add = find(&v["procedures"], "addRecentFile");
    assert!(add["signature"].as_str().unwrap().contains("string $theFile"), "{add}");
    assert_eq!(add["source"], "bundled");
    assert!(v["builtins"].as_array().unwrap().iter().any(|e| e["name"] == "sin"));
    assert!(v["builtins"].as_array().unwrap().iter().any(|e| e["name"] == "getenv"));
}

#[test]
fn python_modules_and_classes_are_the_ones_aztek_ships() {
    aztek_init();
    let v = index();
    let modules = &v["python"]["modules"];
    for name in ["maya", "maya.cmds", "maya.mel", "maya.api", "maya.api.OpenMaya", "maya.OpenMaya"] {
        assert!(!modules[name].is_null(), "{name} missing");
    }
    let maya = &modules["maya"];
    let kids = maya["children"].as_array().unwrap();
    for name in ["cmds", "mel", "OpenMaya", "api"] {
        assert!(kids.iter().any(|k| k == name), "maya.{name} not under maya: {kids:?}");
    }
    let api = &modules["maya.api"];
    let kids: Vec<&str> = api["children"].as_array().unwrap().iter().map(|k| k.as_str().unwrap()).collect();
    assert_eq!(kids, vec!["OpenMaya", "OpenMayaAnim", "OpenMayaRender", "OpenMayaUI"]);
    assert!(find(&modules["maya.mel"]["entries"], "eval")["kind"] == "function");
    assert!(find(&modules["maya.api.OpenMaya"]["entries"], "MFnMesh")["kind"] == "class");
    // maya.cmds is the command registry; the editor fills it from the commands above.
    assert!(modules["maya.cmds"]["entries"].as_array().unwrap().is_empty());
}

#[test]
fn a_class_is_loaded_with_its_constructor_and_members() {
    aztek_init();
    let module = CString::new("maya.api.OpenMaya").unwrap();
    let class = CString::new("MFnMesh").unwrap();
    let first = take(aztek_completion_class(module.as_ptr(), class.as_ptr()));
    let v: Value = serde_json::from_str(&first).unwrap();
    assert!(v["init"].as_str().unwrap().contains("obj"));
    let members = v["members"].as_array().unwrap();
    assert!(members.iter().any(|m| m["name"] == "allIntersections" && m["kind"] == "method"));
    assert!(members.iter().any(|m| m["name"] == "hasObj" && m["inherited"] == true));
    // The second load is the cached answer.
    let second = take(aztek_completion_class(module.as_ptr(), class.as_ptr()));
    assert_eq!(first, second);
    // A class that is not in the module comes back with the error rather than invented members.
    let missing = CString::new("NoSuchClass").unwrap();
    let v: Value = serde_json::from_str(&take(aztek_completion_class(module.as_ptr(), missing.as_ptr()))).unwrap();
    assert!(v["error"].is_string());
}
