//! The global MEL procedures that come with Aztek (#171), from the repository's scripts/ folder, built in so every
//! package has them. Laid out as the reference's installation: startup/ and others/.

use std::path::PathBuf;

/// Path under scripts/, and the text.
const SCRIPTS: &[(&str, &str)] = &[
    ("startup/preferencesWnd.mel", include_str!("../../../../scripts/startup/preferencesWnd.mel")),
];

/// The folder the scripts show as coming from: scripts/ in the installation where it has one (the Linux packages put a
/// copy in share/aztek/scripts), else beside the program.
fn shown_root() -> PathBuf {
    let exe_dir = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_path_buf())).unwrap_or_default();
    for d in [exe_dir.join("scripts"), exe_dir.join("../share/aztek/scripts"), exe_dir.join("../Resources/scripts")] {
        if d.is_dir() {
            return d.canonicalize().unwrap_or(d);
        }
    }
    exe_dir.join("scripts")
}

/// `name.mel` among the bundled scripts: the file it shows as, and its text.
pub fn script(name: &str) -> Option<(String, &'static str)> {
    let file = format!("{name}.mel");
    SCRIPTS
        .iter()
        .find(|(p, _)| p.rsplit('/').next() == Some(file.as_str()))
        .map(|(p, text)| (shown_root().join(p).to_string_lossy().into_owned(), *text))
}

/// Whether the bundled script for `name` is one the reference sources at start, so its procedure is already known.
pub fn at_startup(name: &str) -> bool {
    let file = format!("startup/{name}.mel");
    SCRIPTS.iter().any(|(p, _)| *p == file)
}
