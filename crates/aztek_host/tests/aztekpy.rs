//! aztekpy, the reference's mayapy: Python's command line with Aztek's modules, its exit status, and Aztek's own
//! packages folder ahead of the system's.

#[test]
fn aztekpy_runs_python_with_the_commands_and_its_own_packages() {
    let tmp = std::env::temp_dir().join(format!("aztekpy_{}", std::process::id()));
    let pkgs = tmp.join("site-packages");
    std::fs::create_dir_all(&pkgs).unwrap();
    std::fs::write(pkgs.join("aztek_probe_pkg.py"), "WHERE = 'aztek packages'\n").unwrap();
    std::env::set_var("AZTEK_PYTHON_PACKAGES", &pkgs);
    let out = tmp.join("out.txt").to_string_lossy().replace('\\', "/");
    let src = format!(
        "import maya.standalone; maya.standalone.initialize()\n\
         import maya.cmds as cmds, aztek_probe_pkg, sys\n\
         open({out:?}, 'w').write(repr((cmds.polyCube()[0], aztek_probe_pkg.WHERE, sys.argv)))\n\
         sys.exit(7)"
    );
    let args: Vec<std::ffi::CString> = ["aztekpy", "-c", &src, "one"].iter().map(|a| std::ffi::CString::new(*a).unwrap()).collect();
    let ptrs: Vec<*const std::ffi::c_char> = args.iter().map(|a| a.as_ptr()).collect();
    let rc = aztek_host::aztek_python_main(ptrs.len() as i32, ptrs.as_ptr());
    assert_eq!(rc, 7);
    assert_eq!(std::fs::read_to_string(&out).unwrap(), "('pCube1', 'aztek packages', ['-c', 'one'])");
    let _ = std::fs::remove_dir_all(&tmp);
}
