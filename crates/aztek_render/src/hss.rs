//! Harold scene files (`.hss`): a whole `Scene` as Harold renders it, written by the application's `haroldExportHss`
//! and rendered by the `harold` command-line program, as the reference's renderer writes and renders `.ass` files.
//!
//! The file is JSON: `{"harold": <version>, "scene": {...}}`, the scene's meshes, instances, shading network,
//! materials, lights, camera and options as `Scene` holds them. Textures are kept as their files and colour spaces and
//! loaded when rendered; hierarchies, light tables and texture caches are built again on reading.

use std::sync::Arc;

use crate::scene::Scene;
use crate::texture::TextureCache;

/// The layout version written; files of a later version are refused.
pub const VERSION: u64 = 1;

pub fn to_string(scene: &Scene) -> Result<String, String> {
    let body = serde_json::to_value(scene).map_err(|e| e.to_string())?;
    serde_json::to_string(&serde_json::json!({"harold": VERSION, "scene": body})).map_err(|e| e.to_string())
}

pub fn write(scene: &Scene, path: &std::path::Path) -> Result<(), String> {
    let text = to_string(scene)?;
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// A scene from a file's text, ready to render.
pub fn from_str(text: &str) -> Result<Scene, String> {
    let doc: serde_json::Value = serde_json::from_str(text).map_err(|e| format!("not a Harold scene file: {e}"))?;
    let version = doc.get("harold").and_then(serde_json::Value::as_u64).ok_or("not a Harold scene file: no \"harold\" version")?;
    if version > VERSION {
        return Err(format!("a Harold scene file of version {version}; this Harold reads up to {VERSION}"));
    }
    let body = doc.get("scene").cloned().ok_or("a Harold scene file with no scene")?;
    let cache = Arc::new(TextureCache::default());
    let mut scene: Scene = crate::texture::reading_into(cache.clone(), || serde_json::from_value(body)).map_err(|e| format!("Harold scene file: {e}"))?;
    cache.budget_mb.store(scene.options.texture_max_memory_mb, std::sync::atomic::Ordering::Relaxed);
    scene.textures = cache;
    for m in &mut scene.geo.meshes {
        Arc::make_mut(m).build();
    }
    scene.prepare();
    Ok(scene)
}

pub fn read(path: &std::path::Path) -> Result<Scene, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}
