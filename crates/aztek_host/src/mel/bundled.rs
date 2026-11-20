//! The global MEL procedures that come with Aztek (#171), from the repository's scripts/ folder, built in so every
//! package has them. Laid out as the reference's installation: startup/ and others/.

use std::path::PathBuf;

/// Path under scripts/, and the text.
const SCRIPTS: &[(&str, &str)] = &[
    ("others/MTprocs.mel", include_str!("../../../../scripts/others/MTprocs.mel")),
    ("others/PolySelectConvert.mel", include_str!("../../../../scripts/others/PolySelectConvert.mel")),
    ("others/artAttrSkinToolScript.mel", include_str!("../../../../scripts/others/artAttrSkinToolScript.mel")),
    ("others/artAttrToolScript.mel", include_str!("../../../../scripts/others/artAttrToolScript.mel")),
    ("others/bevelToolScript.mel", include_str!("../../../../scripts/others/bevelToolScript.mel")),
    ("others/doBakeNonDefHistory.mel", include_str!("../../../../scripts/others/doBakeNonDefHistory.mel")),
    ("others/expressionEditor.mel", include_str!("../../../../scripts/others/expressionEditor.mel")),
    ("others/extrudeToolScript.mel", include_str!("../../../../scripts/others/extrudeToolScript.mel")),
    ("others/fileOptions.mel", include_str!("../../../../scripts/others/fileOptions.mel")),
    ("others/fitAllPanels.mel", include_str!("../../../../scripts/others/fitAllPanels.mel")),
    ("others/fitPanel.mel", include_str!("../../../../scripts/others/fitPanel.mel")),
    ("others/nextOrPreviousFrame.mel", include_str!("../../../../scripts/others/nextOrPreviousFrame.mel")),
    ("others/nodeEditorWindow.mel", include_str!("../../../../scripts/others/nodeEditorWindow.mel")),
    ("others/openSelectionConstraintWindow.mel", include_str!("../../../../scripts/others/openSelectionConstraintWindow.mel")),
    ("others/performBakeNonDefHistory.mel", include_str!("../../../../scripts/others/performBakeNonDefHistory.mel")),
    ("others/performDuplicate.mel", include_str!("../../../../scripts/others/performDuplicate.mel")),
    ("others/performDuplicateSpecial.mel", include_str!("../../../../scripts/others/performDuplicateSpecial.mel")),
    ("others/performGroup.mel", include_str!("../../../../scripts/others/performGroup.mel")),
    ("others/performJointOrient.mel", include_str!("../../../../scripts/others/performJointOrient.mel")),
    ("others/performParent.mel", include_str!("../../../../scripts/others/performParent.mel")),
    ("others/performPlayblast.mel", include_str!("../../../../scripts/others/performPlayblast.mel")),
    ("others/performPolyDeleteElements.mel", include_str!("../../../../scripts/others/performPolyDeleteElements.mel")),
    ("others/performSetKeyframeArgList.mel", include_str!("../../../../scripts/others/performSetKeyframeArgList.mel")),
    ("others/performUnparent.mel", include_str!("../../../../scripts/others/performUnparent.mel")),
    ("others/pluginWin.mel", include_str!("../../../../scripts/others/pluginWin.mel")),
    ("others/projectViewer.mel", include_str!("../../../../scripts/others/projectViewer.mel")),
    ("others/renderSequence.mel", include_str!("../../../../scripts/others/renderSequence.mel")),
    ("others/renderSequenceOptionsWindow.mel", include_str!("../../../../scripts/others/renderSequenceOptionsWindow.mel")),
    ("others/renderWindowPanel.mel", include_str!("../../../../scripts/others/renderWindowPanel.mel")),
    ("others/setSubdivDisplayLevelAndFilter.mel", include_str!("../../../../scripts/others/setSubdivDisplayLevelAndFilter.mel")),
    ("others/togglePlayback.mel", include_str!("../../../../scripts/others/togglePlayback.mel")),
    ("others/unifiedRenderGlobalsWindow.mel", include_str!("../../../../scripts/others/unifiedRenderGlobalsWindow.mel")),
    ("others/uvSetEditor.mel", include_str!("../../../../scripts/others/uvSetEditor.mel")),
    ("startup/FileMenu.mel", include_str!("../../../../scripts/startup/FileMenu.mel")),
    ("startup/UIComponents.mel", include_str!("../../../../scripts/startup/UIComponents.mel")),
    ("startup/WindowMenu.mel", include_str!("../../../../scripts/startup/WindowMenu.mel")),
    ("startup/buildDisplayMenu.mel", include_str!("../../../../scripts/startup/buildDisplayMenu.mel")),
    ("startup/colorManagementUtilities.mel", include_str!("../../../../scripts/startup/colorManagementUtilities.mel")),
    ("startup/hotkeyEditorWindow.mel", include_str!("../../../../scripts/startup/hotkeyEditorWindow.mel")),
    ("startup/incrementAndSaveScene.mel", include_str!("../../../../scripts/startup/incrementAndSaveScene.mel")),
    ("startup/initAfter.mel", include_str!("../../../../scripts/startup/initAfter.mel")),
    ("startup/initHUDScripts.mel", include_str!("../../../../scripts/startup/initHUDScripts.mel")),
    ("startup/initHelp.mel", include_str!("../../../../scripts/startup/initHelp.mel")),
    ("startup/initOutliner.mel", include_str!("../../../../scripts/startup/initOutliner.mel")),
    ("startup/performNewScene.mel", include_str!("../../../../scripts/startup/performNewScene.mel")),
    ("startup/preferencesWnd.mel", include_str!("../../../../scripts/startup/preferencesWnd.mel")),
    ("startup/scriptEditorPanel.mel", include_str!("../../../../scripts/startup/scriptEditorPanel.mel")),
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

/// Scripts outside startup/ whose procedures the reference's own commands call by name before anything sources them:
/// its polygon UI sources MTprocs at start, and Render View and file options procedures are run from its runtime
/// commands.
const SOURCED_AT_START: &[&str] = &["others/MTprocs.mel", "others/renderWindowPanel.mel", "others/fileOptions.mel"];

/// The global procedures each script sourced at start defines, by name: the reference has them all from the start,
/// whatever file they sit in, so calling one finds its file.
fn startup_procs() -> &'static std::collections::HashMap<String, usize> {
    static MAP: std::sync::OnceLock<std::collections::HashMap<String, usize>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        for (i, (path, text)) in SCRIPTS.iter().enumerate() {
            if !(path.starts_with("startup/") || SOURCED_AT_START.contains(path)) {
                continue;
            }
            for line in text.lines() {
                let Some(rest) = line.trim_start().strip_prefix("global proc ") else { continue };
                let Some(head) = rest.split('(').next() else { continue };
                if let Some(name) = head.split_whitespace().last() {
                    map.entry(name.to_string()).or_insert(i);
                }
            }
        }
        map
    })
}

fn shown(i: usize) -> (String, &'static str) {
    let (p, text) = SCRIPTS[i];
    (shown_root().join(p).to_string_lossy().into_owned(), text)
}

/// The bundled script for `name`: `name.mel`, else the script sourced at start that defines it. The file it shows as,
/// and its text.
pub fn script(name: &str) -> Option<(String, &'static str)> {
    let file = format!("{name}.mel");
    SCRIPTS
        .iter()
        .position(|(p, _)| p.rsplit('/').next() == Some(file.as_str()))
        .or_else(|| startup_procs().get(name).copied())
        .map(shown)
}

/// The bundled script `name.mel` itself, for `source`: the file it shows as, and its text.
pub fn file(name: &str) -> Option<(String, &'static str)> {
    let file = format!("{name}.mel");
    SCRIPTS.iter().position(|(p, _)| p.rsplit('/').next() == Some(file.as_str())).map(shown)
}

/// Whether `name` is a procedure of a bundled script the reference sources at start, so it is already known.
pub fn at_startup(name: &str) -> bool {
    let file = format!("startup/{name}.mel");
    SCRIPTS.iter().any(|(p, _)| *p == file) || startup_procs().contains_key(name)
}
