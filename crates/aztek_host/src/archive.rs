//! File > Archive Scene: the saved scene and the files it uses (references, textures, image planes, sounds, caches),
//! packed into a zip beside the scene, as the reference's Archive Scene does. The scene goes in at the top; a file in
//! the scene's folder or the project keeps its path relative to it, anything else goes under `external/`.

use std::io::Write;
use std::path::{Path, PathBuf};

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, FlagDef};
use aztek_scene::session::Session;
use serde_json::{json, Value};

fn entry_name(file: &Path, scene_dir: &Path, project: Option<&Path>) -> String {
    let rel = file.strip_prefix(scene_dir).ok().or_else(|| project.and_then(|p| file.strip_prefix(p).ok()));
    let name = match rel {
        Some(r) => r.to_path_buf(),
        None => Path::new("external").join(file.file_name().unwrap_or_default()),
    };
    name.to_string_lossy().replace('\\', "/")
}

pub fn archive(s: &mut Session, to: Option<&str>) -> CmdResult<Value> {
    let Some(scene) = s.scene_path.clone() else { fail!("archiveScene: save the scene first") };
    if s.is_modified() {
        fail!("archiveScene: the scene has unsaved changes; save it first");
    }
    let scene_path = PathBuf::from(&scene);
    let dir = scene_path.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = scene_path.file_stem().and_then(|x| x.to_str()).unwrap_or("scene").to_string();
    let zip_path = to.map(PathBuf::from).unwrap_or_else(|| dir.join(format!("{stem}.zip")));
    let project = aztek_scene::shading::project_dir(s).filter(|p| !p.is_empty()).map(PathBuf::from);
    let mut files = vec![(scene_path.clone(), scene_path.file_name().unwrap_or_default().to_string_lossy().to_string())];
    let mut missing = Vec::new();
    for e in aztek_scene::external_files::list(s) {
        match e.found {
            Some(p) => {
                let name = entry_name(&p, &dir, project.as_deref());
                if !files.iter().any(|(_, n)| *n == name) {
                    files.push((p, name));
                }
            }
            None => missing.push(format!("{} ({})", e.written, e.user)),
        }
    }
    let shown = zip_path.display().to_string();
    let err = |e: &dyn std::fmt::Display| aztek_ops::CommandError::new(format!("archiveScene: {shown}: {e}"));
    let out = std::fs::File::create(&zip_path).map_err(|e| err(&e))?;
    let mut zip = zip::ZipWriter::new(out);
    for (path, name) in &files {
        let data = std::fs::read(path).map_err(|e| aztek_ops::CommandError::new(format!("archiveScene: {}: {e}", path.display())))?;
        let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).large_file(data.len() as u64 >= u32::MAX as u64);
        zip.start_file(name.as_str(), options).map_err(|e| err(&e))?;
        zip.write_all(&data).map_err(|e| err(&e))?;
    }
    zip.finish().map_err(|e| err(&e))?;
    for m in &missing {
        crate::script_jobs::warn(&format!("archiveScene: not found, left out: {m}"));
    }
    Ok(json!(zip_path.to_string_lossy().replace('\\', "/")))
}

pub fn register(s: &mut Session) {
    s.register(
        CommandDef::<Session>::new(
            "archiveScene",
            "File > Archive Scene: the saved scene and every file it uses (references, file textures, image planes, sounds, \
             Alembic and XJEN caches) zipped into <scene>.zip beside it, or -file. Files that can't be found are left out \
             with a warning. Aztek's name for the menu item. Returns the zip.",
            |s, a| archive(s, a.get("file").and_then(Value::as_str)),
        )
        .undoable(false)
        .flag(FlagDef::new("file", "f", ArgType::String, Value::Null, "Where the zip goes.")),
    );
}
