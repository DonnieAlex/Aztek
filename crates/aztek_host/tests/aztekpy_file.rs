//! aztekpy running a script file: the script is __main__, as under the reference's mayapy, and the API modules
//! still import (they used to reach Aztek's own setup through __main__).

#[test]
fn a_script_file_is_main_and_the_api_still_imports() {
    let tmp = std::env::temp_dir().join(format!("aztekpy_file_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let out = tmp.join("out.txt").to_string_lossy().replace('\\', "/");
    let script = tmp.join("tool.py");
    std::fs::write(
        &script,
        format!(
            "import maya.standalone; maya.standalone.initialize()\n\
             import maya.OpenMaya as om1, maya.api.OpenMaya as om\n\
             sel = om.MSelectionList()\n\
             open({out:?}, 'w').write(repr((__name__, om1.MSelectionList.__name__, sel.length())))\n"
        ),
    )
    .unwrap();
    let path = script.to_string_lossy().to_string();
    let args: Vec<std::ffi::CString> = ["aztekpy", path.as_str()].iter().map(|a| std::ffi::CString::new(*a).unwrap()).collect();
    let ptrs: Vec<*const std::ffi::c_char> = args.iter().map(|a| a.as_ptr()).collect();
    assert_eq!(aztek_host::aztek_python_main(ptrs.len() as i32, ptrs.as_ptr()), 0);
    assert_eq!(std::fs::read_to_string(&out).unwrap(), "('__main__', 'MSelectionList', 0)");
    let _ = std::fs::remove_dir_all(&tmp);
}
