//! The names the reference's renderer gives its settings nodes and itself. Scenes and scripts written for the
//! reference use them, so each one answers for Harold's own: looking a node up, creating one with `-s` from a file,
//! `renderer -exists`. Saving a scene writes these names again, so the file keeps its settings when the reference
//! opens it. Everything else in Aztek uses Harold's names. This file is the only place the old names are kept.

/// (the reference's name, Harold's) for the settings nodes every scene has.
pub const NODE_NAMES: &[(&str, &str)] = &[
    ("defaultArnoldRenderOptions", "defaultHaroldRenderOptions"),
    ("defaultArnoldFilter", "defaultHaroldFilter"),
    ("defaultArnoldDriver", "defaultHaroldDriver"),
    ("defaultArnoldDisplayDriver", "defaultHaroldDisplayDriver"),
];

/// Harold's name in the renderer registry and in defaultRenderGlobals.currentRenderer.
pub const RENDERER: &str = "harold";
/// The same renderer under the reference's name.
pub const FILE_RENDERER: &str = "arnold";

/// The render command under the reference's name: hrender with the same flags.
pub const RENDER_COMMAND: &str = "arnoldRender";

/// The renderer name a script or file gave, as Harold's: the reference's name becomes "harold".
pub fn renderer(name: &str) -> &str {
    if name == FILE_RENDERER { RENDERER } else { name }
}

/// Harold's name for a node name or path, keeping any namespace and parents: `ns:defaultArnoldFilter` is
/// `ns:defaultHaroldFilter`. Any other name comes back as it is.
pub fn node_name(name: &str) -> std::borrow::Cow<'_, str> {
    let (ns, leaf) = match name.rfind([':', '|']) {
        Some(i) => name.split_at(i + 1),
        None => ("", name),
    };
    match NODE_NAMES.iter().find(|(old, _)| *old == leaf) {
        Some((_, new)) => std::borrow::Cow::Owned(format!("{ns}{new}")),
        None => std::borrow::Cow::Borrowed(name),
    }
}

/// A scene file's text with Harold's names put back as the reference's: the settings nodes, and the renderer
/// defaultRenderGlobals names.
pub fn to_file_names(text: &str) -> String {
    let mut out = text.to_string();
    for (old, new) in NODE_NAMES {
        out = out.replace(new, old);
    }
    out.replace(
        &format!("\".ren\" -type \"string\" \"{RENDERER}\""),
        &format!("\".ren\" -type \"string\" \"{FILE_RENDERER}\""),
    )
}
