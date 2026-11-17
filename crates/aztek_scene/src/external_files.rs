//! The files a scene uses beside its own: references, file textures, image planes, sounds and caches. `file -q -list`
//! answers with them, and File > Archive Scene packs them.

use std::path::PathBuf;

use crate::session::Session;

/// (type, attribute) of each kind of node that names a file. A node of another type with a `cacheFile` attribute
/// (XJEN's caches) counts too.
const FILE_ATTRS: &[(&str, &str)] = &[("file", "fileTextureName"), ("psdFileTex", "fileTextureName"), ("imagePlane", "imageName"), ("audio", "filename"), ("AlembicNode", "abc_File")];

/// Each file: as written, and where it was found (None when it wasn't).
#[derive(Clone, Debug)]
pub struct External {
    pub written: String,
    pub found: Option<PathBuf>,
    /// What uses it: the node, or the reference node.
    pub user: String,
}

pub fn list(s: &mut Session) -> Vec<External> {
    let scene = s.scene_path.clone();
    let project = crate::shading::project_dir(s);
    let mut out: Vec<External> = Vec::new();
    for e in &s.extras.references.entries {
        let p = PathBuf::from(&e.path);
        out.push(External { written: e.unresolved.clone(), found: p.is_file().then_some(p), user: e.rfn.clone() });
    }
    let g = &s.graph;
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        let t = n.ty();
        let attr = FILE_ATTRS.iter().find(|(ty, _)| t.is_a(ty)).map(|x| x.1).or_else(|| t.attr_index("cacheFile").map(|_| "cacheFile"));
        let Some(attr) = attr else { continue };
        let Some(i) = t.attr_index(attr) else { continue };
        let written = n.data().value(i).as_str().to_string();
        if written.is_empty() {
            continue;
        }
        let found = crate::shading::resolve_texture(&written, scene.as_deref(), project.as_deref());
        out.push(External { written, found, user: g.path(id) });
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|e| seen.insert(e.found.clone().map_or(e.written.clone(), |p| p.to_string_lossy().to_string())));
    out
}
