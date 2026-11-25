//! The global MEL procedures that come with Aztek (#171), from the repository's scripts/ folder, built in so every
//! package has them. Laid out as the reference's installation: startup/ and others/.

use std::path::PathBuf;

/// Path under scripts/, and the text.
const SCRIPTS: &[(&str, &str)] = &[
    ("others/MTprocs.mel", include_str!("../../../../scripts/others/MTprocs.mel")),
    ("others/OutlinerEdMenu.mel", include_str!("../../../../scripts/others/OutlinerEdMenu.mel")),
    ("others/PolySelectConvert.mel", include_str!("../../../../scripts/others/PolySelectConvert.mel")),
    ("others/TimeSliderMenu.mel", include_str!("../../../../scripts/others/TimeSliderMenu.mel")),
    ("others/addRecentFile.mel", include_str!("../../../../scripts/others/addRecentFile.mel")),
    ("others/addStandardButtons.mel", include_str!("../../../../scripts/others/addStandardButtons.mel")),
    ("others/aiffFileOptions.mel", include_str!("../../../../scripts/others/aiffFileOptions.mel")),
    ("others/artAttrCallback.mel", include_str!("../../../../scripts/others/artAttrCallback.mel")),
    ("others/artAttrFilterLabel.mel", include_str!("../../../../scripts/others/artAttrFilterLabel.mel")),
    ("others/artAttrInitPaintableAttr.mel", include_str!("../../../../scripts/others/artAttrInitPaintableAttr.mel")),
    ("others/artAttrSkinToolScript.mel", include_str!("../../../../scripts/others/artAttrSkinToolScript.mel")),
    ("others/artAttrToolScript.mel", include_str!("../../../../scripts/others/artAttrToolScript.mel")),
    ("others/artPuttyToolScript.mel", include_str!("../../../../scripts/others/artPuttyToolScript.mel")),
    ("others/artisanCallback.mel", include_str!("../../../../scripts/others/artisanCallback.mel")),
    ("others/artisanNamedCommandSetup.mel", include_str!("../../../../scripts/others/artisanNamedCommandSetup.mel")),
    ("others/bevelToolScript.mel", include_str!("../../../../scripts/others/bevelToolScript.mel")),
    ("others/birailToolScript.mel", include_str!("../../../../scripts/others/birailToolScript.mel")),
    ("others/clipEditorNameFromPanel.mel", include_str!("../../../../scripts/others/clipEditorNameFromPanel.mel")),
    ("others/convertHairSelection.mel", include_str!("../../../../scripts/others/convertHairSelection.mel")),
    ("others/createObjectTypeFilters.mel", include_str!("../../../../scripts/others/createObjectTypeFilters.mel")),
    ("others/createOutlinerPanelMenu.mel", include_str!("../../../../scripts/others/createOutlinerPanelMenu.mel")),
    ("others/customChannelSets.mel", include_str!("../../../../scripts/others/customChannelSets.mel")),
    ("others/doBakeNonDefHistory.mel", include_str!("../../../../scripts/others/doBakeNonDefHistory.mel")),
    ("others/doGroup.mel", include_str!("../../../../scripts/others/doGroup.mel")),
    ("others/doSetFullBodyIKKeysArgList.mel", include_str!("../../../../scripts/others/doSetFullBodyIKKeysArgList.mel")),
    ("others/doSetKeyframeArgList.mel", include_str!("../../../../scripts/others/doSetKeyframeArgList.mel")),
    ("others/duplicatePreset.mel", include_str!("../../../../scripts/others/duplicatePreset.mel")),
    ("others/editorNameFromPanel.mel", include_str!("../../../../scripts/others/editorNameFromPanel.mel")),
    ("others/expandPolyGroupSelection.mel", include_str!("../../../../scripts/others/expandPolyGroupSelection.mel")),
    ("others/expandSelectionConnection.mel", include_str!("../../../../scripts/others/expandSelectionConnection.mel")),
    ("others/expressionEdTextEditor.mel", include_str!("../../../../scripts/others/expressionEdTextEditor.mel")),
    ("others/expressionEditor.mel", include_str!("../../../../scripts/others/expressionEditor.mel")),
    ("others/extrudeToolScript.mel", include_str!("../../../../scripts/others/extrudeToolScript.mel")),
    ("others/fileBrowser.mel", include_str!("../../../../scripts/others/fileBrowser.mel")),
    ("others/fileDialogFilterTypes.mel", include_str!("../../../../scripts/others/fileDialogFilterTypes.mel")),
    ("others/fileOptions.mel", include_str!("../../../../scripts/others/fileOptions.mel")),
    ("others/fitAllPanels.mel", include_str!("../../../../scripts/others/fitAllPanels.mel")),
    ("others/fitPanel.mel", include_str!("../../../../scripts/others/fitPanel.mel")),
    ("others/getOptionBox.mel", include_str!("../../../../scripts/others/getOptionBox.mel")),
    ("others/getStandardWindow.mel", include_str!("../../../../scripts/others/getStandardWindow.mel")),
    ("others/hideKeepSelection.mel", include_str!("../../../../scripts/others/hideKeepSelection.mel")),
    ("others/hikKeyingModeOperation.mel", include_str!("../../../../scripts/others/hikKeyingModeOperation.mel")),
    ("others/ikFK.mel", include_str!("../../../../scripts/others/ikFK.mel")),
    ("others/initRecentFilesTypeList.mel", include_str!("../../../../scripts/others/initRecentFilesTypeList.mel")),
    ("others/jointOrientUtils.mel", include_str!("../../../../scripts/others/jointOrientUtils.mel")),
    ("others/loadAnimMenuLibrary.mel", include_str!("../../../../scripts/others/loadAnimMenuLibrary.mel")),
    ("others/lookThroughModelPanel.mel", include_str!("../../../../scripts/others/lookThroughModelPanel.mel")),
    ("others/namespaceTreeView.mel", include_str!("../../../../scripts/others/namespaceTreeView.mel")),
    ("others/nextOrPreviousFrame.mel", include_str!("../../../../scripts/others/nextOrPreviousFrame.mel")),
    ("others/nodeEditorWindow.mel", include_str!("../../../../scripts/others/nodeEditorWindow.mel")),
    ("others/nodePresetConfirmDeleteDialog.mel", include_str!("../../../../scripts/others/nodePresetConfirmDeleteDialog.mel")),
    ("others/numSelectedObjects.mel", include_str!("../../../../scripts/others/numSelectedObjects.mel")),
    ("others/nurbsBooleanToolScript.mel", include_str!("../../../../scripts/others/nurbsBooleanToolScript.mel")),
    ("others/nurbsToPolyAddOptions.mel", include_str!("../../../../scripts/others/nurbsToPolyAddOptions.mel")),
    ("others/nurbsToPolyCallback.mel", include_str!("../../../../scripts/others/nurbsToPolyCallback.mel")),
    ("others/nurbsToPolySetup.mel", include_str!("../../../../scripts/others/nurbsToPolySetup.mel")),
    ("others/nurbsToSubdivSetup.mel", include_str!("../../../../scripts/others/nurbsToSubdivSetup.mel")),
    ("others/openSelectionConstraintWindow.mel", include_str!("../../../../scripts/others/openSelectionConstraintWindow.mel")),
    ("others/parentPreset.mel", include_str!("../../../../scripts/others/parentPreset.mel")),
    ("others/performBakeNonDefHistory.mel", include_str!("../../../../scripts/others/performBakeNonDefHistory.mel")),
    ("others/performBevel.mel", include_str!("../../../../scripts/others/performBevel.mel")),
    ("others/performCreateDynamicConstraint.mel", include_str!("../../../../scripts/others/performCreateDynamicConstraint.mel")),
    ("others/performCreateGeometryCache.mel", include_str!("../../../../scripts/others/performCreateGeometryCache.mel")),
    ("others/performDuplicate.mel", include_str!("../../../../scripts/others/performDuplicate.mel")),
    ("others/performDuplicateClip.mel", include_str!("../../../../scripts/others/performDuplicateClip.mel")),
    ("others/performDuplicateSpecial.mel", include_str!("../../../../scripts/others/performDuplicateSpecial.mel")),
    ("others/performDynamics.mel", include_str!("../../../../scripts/others/performDynamics.mel")),
    ("others/performExtrude.mel", include_str!("../../../../scripts/others/performExtrude.mel")),
    ("others/performFileDropAction.mel", include_str!("../../../../scripts/others/performFileDropAction.mel")),
    ("others/performFileImportAction.mel", include_str!("../../../../scripts/others/performFileImportAction.mel")),
    ("others/performGroup.mel", include_str!("../../../../scripts/others/performGroup.mel")),
    ("others/performJointOrient.mel", include_str!("../../../../scripts/others/performJointOrient.mel")),
    ("others/performNormalizeSkinWeights.mel", include_str!("../../../../scripts/others/performNormalizeSkinWeights.mel")),
    ("others/performParent.mel", include_str!("../../../../scripts/others/performParent.mel")),
    ("others/performPlayblast.mel", include_str!("../../../../scripts/others/performPlayblast.mel")),
    ("others/performPolyAutoUnwrap.mel", include_str!("../../../../scripts/others/performPolyAutoUnwrap.mel")),
    ("others/performPolyCopyPaste.mel", include_str!("../../../../scripts/others/performPolyCopyPaste.mel")),
    ("others/performPolyDeleteElements.mel", include_str!("../../../../scripts/others/performPolyDeleteElements.mel")),
    ("others/performPolyExtrude.mel", include_str!("../../../../scripts/others/performPolyExtrude.mel")),
    ("others/performPolyForceUVArgList.mel", include_str!("../../../../scripts/others/performPolyForceUVArgList.mel")),
    ("others/performPolyMerge.mel", include_str!("../../../../scripts/others/performPolyMerge.mel")),
    ("others/performPolyPrimitive.mel", include_str!("../../../../scripts/others/performPolyPrimitive.mel")),
    ("others/performPolyProjectionArgList.mel", include_str!("../../../../scripts/others/performPolyProjectionArgList.mel")),
    ("others/performPolySubdivide.mel", include_str!("../../../../scripts/others/performPolySubdivide.mel")),
    ("others/performPolySuperShape.mel", include_str!("../../../../scripts/others/performPolySuperShape.mel")),
    ("others/performSetKeyframeArgList.mel", include_str!("../../../../scripts/others/performSetKeyframeArgList.mel")),
    ("others/performUnparent.mel", include_str!("../../../../scripts/others/performUnparent.mel")),
    ("others/pluginWin.mel", include_str!("../../../../scripts/others/pluginWin.mel")),
    ("others/polyCheckSelection.mel", include_str!("../../../../scripts/others/polyCheckSelection.mel")),
    ("others/polyNextSelectionBatch.mel", include_str!("../../../../scripts/others/polyNextSelectionBatch.mel")),
    ("others/polyPerformAction.mel", include_str!("../../../../scripts/others/polyPerformAction.mel")),
    ("others/polySelConstraintPanel.mel", include_str!("../../../../scripts/others/polySelConstraintPanel.mel")),
    ("others/polygonConstraint.mel", include_str!("../../../../scripts/others/polygonConstraint.mel")),
    ("others/projectViewer.mel", include_str!("../../../../scripts/others/projectViewer.mel")),
    ("others/proxyTags.mel", include_str!("../../../../scripts/others/proxyTags.mel")),
    ("others/proxyUtils.mel", include_str!("../../../../scripts/others/proxyUtils.mel")),
    ("others/pv_performAction.mel", include_str!("../../../../scripts/others/pv_performAction.mel")),
    ("others/pythonRunTimeCommand.mel", include_str!("../../../../scripts/others/pythonRunTimeCommand.mel")),
    ("others/renderSequence.mel", include_str!("../../../../scripts/others/renderSequence.mel")),
    ("others/renderSequenceOptionsWindow.mel", include_str!("../../../../scripts/others/renderSequenceOptionsWindow.mel")),
    ("others/renderWindowPanel.mel", include_str!("../../../../scripts/others/renderWindowPanel.mel")),
    ("others/retainWorkingDirectory.mel", include_str!("../../../../scripts/others/retainWorkingDirectory.mel")),
    ("others/saveNodePresetDialog.mel", include_str!("../../../../scripts/others/saveNodePresetDialog.mel")),
    ("others/sceneIsEmpty.mel", include_str!("../../../../scripts/others/sceneIsEmpty.mel")),
    ("others/scriptCtxCommonProperties.mel", include_str!("../../../../scripts/others/scriptCtxCommonProperties.mel")),
    ("others/scriptCtxCommonValues.mel", include_str!("../../../../scripts/others/scriptCtxCommonValues.mel")),
    ("others/scriptToolScript.mel", include_str!("../../../../scripts/others/scriptToolScript.mel")),
    ("others/sequenceEditorNameFromPanel.mel", include_str!("../../../../scripts/others/sequenceEditorNameFromPanel.mel")),
    ("others/setArtAttrFilterLabel.mel", include_str!("../../../../scripts/others/setArtAttrFilterLabel.mel")),
    ("others/setDefaultTemplates.mel", include_str!("../../../../scripts/others/setDefaultTemplates.mel")),
    ("others/setMeshSculptTool.mel", include_str!("../../../../scripts/others/setMeshSculptTool.mel")),
    ("others/setSubdivDisplayLevelAndFilter.mel", include_str!("../../../../scripts/others/setSubdivDisplayLevelAndFilter.mel")),
    ("others/setWorkingDirectory.mel", include_str!("../../../../scripts/others/setWorkingDirectory.mel")),
    ("others/setupNewMesh.mel", include_str!("../../../../scripts/others/setupNewMesh.mel")),
    ("others/skinClusterList.mel", include_str!("../../../../scripts/others/skinClusterList.mel")),
    ("others/supportRenderers.mel", include_str!("../../../../scripts/others/supportRenderers.mel")),
    ("others/texturePanelCommon.mel", include_str!("../../../../scripts/others/texturePanelCommon.mel")),
    ("others/togglePlayback.mel", include_str!("../../../../scripts/others/togglePlayback.mel")),
    ("others/toolPropertyWindow1.mel", include_str!("../../../../scripts/others/toolPropertyWindow1.mel")),
    ("others/uiToMelMsg.mel", include_str!("../../../../scripts/others/uiToMelMsg.mel")),
    ("others/unifiedRenderGlobalsWindow.mel", include_str!("../../../../scripts/others/unifiedRenderGlobalsWindow.mel")),
    ("others/useRenderRegion.mel", include_str!("../../../../scripts/others/useRenderRegion.mel")),
    ("others/uvSetEditCmd.mel", include_str!("../../../../scripts/others/uvSetEditCmd.mel")),
    ("others/uvSetEditor.mel", include_str!("../../../../scripts/others/uvSetEditor.mel")),
    ("others/workspaceHelperProcs.mel", include_str!("../../../../scripts/others/workspaceHelperProcs.mel")),
    ("startup/ChaSkeletonsMenu.mel", include_str!("../../../../scripts/startup/ChaSkeletonsMenu.mel")),
    ("startup/ContextHelp.mel", include_str!("../../../../scripts/startup/ContextHelp.mel")),
    ("startup/FileMenu.mel", include_str!("../../../../scripts/startup/FileMenu.mel")),
    ("startup/UIComponents.mel", include_str!("../../../../scripts/startup/UIComponents.mel")),
    ("startup/WindowMenu.mel", include_str!("../../../../scripts/startup/WindowMenu.mel")),
    ("startup/buildDisplayMenu.mel", include_str!("../../../../scripts/startup/buildDisplayMenu.mel")),
    ("startup/colorManagementNonLinearSliderUtilities.mel", include_str!("../../../../scripts/startup/colorManagementNonLinearSliderUtilities.mel")),
    ("startup/colorManagementRenderViewUtilities.mel", include_str!("../../../../scripts/startup/colorManagementRenderViewUtilities.mel")),
    ("startup/colorManagementUtilities.mel", include_str!("../../../../scripts/startup/colorManagementUtilities.mel")),
    ("startup/doEnableNodeItems.mel", include_str!("../../../../scripts/startup/doEnableNodeItems.mel")),
    ("startup/doHelpMenu.mel", include_str!("../../../../scripts/startup/doHelpMenu.mel")),
    ("startup/hotkeyEditorWindow.mel", include_str!("../../../../scripts/startup/hotkeyEditorWindow.mel")),
    ("startup/incrementAndSaveScene.mel", include_str!("../../../../scripts/startup/incrementAndSaveScene.mel")),
    ("startup/initAfter.mel", include_str!("../../../../scripts/startup/initAfter.mel")),
    ("startup/initAuxiliary.mel", include_str!("../../../../scripts/startup/initAuxiliary.mel")),
    ("startup/initHUDScripts.mel", include_str!("../../../../scripts/startup/initHUDScripts.mel")),
    ("startup/initHelp.mel", include_str!("../../../../scripts/startup/initHelp.mel")),
    ("startup/initOutliner.mel", include_str!("../../../../scripts/startup/initOutliner.mel")),
    ("startup/initPolygonsUI.mel", include_str!("../../../../scripts/startup/initPolygonsUI.mel")),
    ("startup/localizedPanelLabel.mel", include_str!("../../../../scripts/startup/localizedPanelLabel.mel")),
    ("startup/localizedUIComponentLabel.mel", include_str!("../../../../scripts/startup/localizedUIComponentLabel.mel")),
    ("startup/performNewScene.mel", include_str!("../../../../scripts/startup/performNewScene.mel")),
    ("startup/polysDisplaySetup.mel", include_str!("../../../../scripts/startup/polysDisplaySetup.mel")),
    ("startup/preferencesWnd.mel", include_str!("../../../../scripts/startup/preferencesWnd.mel")),
    ("startup/scriptEditorPanel.mel", include_str!("../../../../scripts/startup/scriptEditorPanel.mel")),
    ("startup/setNamedPanelLayout.mel", include_str!("../../../../scripts/startup/setNamedPanelLayout.mel")),
    ("startup/setSelectMode.mel", include_str!("../../../../scripts/startup/setSelectMode.mel")),
    ("startup/switchPanes.mel", include_str!("../../../../scripts/startup/switchPanes.mel")),
    ("startup/tearOffRestorePanel.mel", include_str!("../../../../scripts/startup/tearOffRestorePanel.mel")),
    ("startup/uiRes.mel", include_str!("../../../../scripts/startup/uiRes.mel")),
    ("startup/updateMainWindowComponentState.mel", include_str!("../../../../scripts/startup/updateMainWindowComponentState.mel")),
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
