//! Alembic in a stage (#100), as the reference's usdAbc plug-in gives it: a layer, a reference or a payload naming a
//! .abc file reads it as USD. openusd reads only its own formats, so the resolver hands the stage a USD layer made
//! from the Alembic file instead: the file imported into a scene of its own (transforms, meshes with UVs, curves, its
//! animation) and written out as USDA, kept beside the other temporary files and made again when the .abc changes.
//!
//! As the reference's: face sets, NURBS patches and lights aren't carried, and transforms come as matrices.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use aztek_ops::Args;
use aztek_scene::Session;
use serde_json::{json, Value};

/// The USD layer standing for an Alembic file, made if needed. None when the file can't be read.
pub fn usd_for_abc(abc: &Path) -> Option<PathBuf> {
    let stamp = std::fs::metadata(abc).and_then(|m| m.modified()).ok()?;
    let mut h = DefaultHasher::new();
    abc.to_string_lossy().hash(&mut h);
    stamp.hash(&mut h);
    let stem = abc.file_stem().and_then(|x| x.to_str()).unwrap_or("alembic");
    let dir = std::env::temp_dir().join("aztek-usdabc").join(format!("{:016x}", h.finish()));
    let out = dir.join(format!("{stem}.usda"));
    if out.is_file() {
        return Some(out);
    }
    std::fs::create_dir_all(&dir).ok()?;
    let path = abc.to_string_lossy().replace('\\', "/");
    let mut s = Session::new();
    aztek_scene::alembic::import(&mut s, &path).ok()?;
    let mut flags = serde_json::Map::new();
    flags.insert("file".into(), json!(out.to_string_lossy().replace('\\', "/")));
    flags.insert("defaultUSDFormat".into(), json!("usda"));
    flags.insert("exportMaterials".into(), json!(false));
    flags.insert("mergeTransformAndShape".into(), json!(true));
    if let Some((a, b)) = aztek_scene::alembic::frame_range(&path) {
        if b > a {
            flags.insert("frameRange".into(), json!([a, b]));
        }
    }
    let args = Args { positional: Vec::new(), flags: flags.into_iter().collect::<serde_json::Map<String, Value>>() };
    crate::export::run(&mut s, &args).ok()?;
    out.is_file().then_some(out)
}
