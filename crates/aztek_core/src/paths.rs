
use std::path::{Component, Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::workspace::Workspace;

fn env_path(name: &str) -> PathBuf {
    std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from).unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn linux_config_home() -> PathBuf {
    let xdg = env_path("XDG_CONFIG_HOME");
    if xdg.is_absolute() {
        xdg
    } else {
        env_path("HOME").join(".config")
    }
}

#[cfg(target_os = "linux")]
fn copy_missing(from: &Path, to: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            if !target.exists() {
                std::fs::create_dir_all(&target)?;
            }
            if target.symlink_metadata()?.is_dir() {
                copy_missing(&entry.path(), &target)?;
            }
        } else if kind.is_file() {
            let mut input = std::fs::File::open(entry.path())?;
            let mut output = match std::fs::OpenOptions::new().write(true).create_new(true).open(&target) {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            };
            if let Err(error) = std::io::copy(&mut input, &mut output) {
                drop(output);
                let _ = std::fs::remove_file(&target);
                return Err(error);
            }
        } else if kind.is_symlink() && !target.exists() {
            let link = std::fs::read_link(entry.path())?;
            let link = if link.is_absolute() { link } else { from.join(link) };
            match std::os::unix::fs::symlink(link, target) {
                Ok(()) => (),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(error) => return Err(error),
            }
        }
    }
    Ok(())
}

/// Linux configuration shared by the shell and host. Earlier settings are copied once without overwriting files.
#[cfg(target_os = "linux")]
pub fn linux_config_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let base = linux_config_home();
        let path = base.join("aztek");
        let marker = path.join(".legacy-config-copied");
        if !marker.exists() {
            let migrate = || -> std::io::Result<()> {
                std::fs::create_dir_all(&path)?;
                for old in [base.join("Aztek"), env_path("HOME").join(".aztek")] {
                    if old.is_dir() && old.canonicalize()? != path.canonicalize()? {
                        copy_missing(&old, &path)?;
                    }
                }
                std::fs::File::create(marker)?;
                Ok(())
            };
            if let Err(error) = migrate() {
                eprintln!("Aztek could not copy its old settings: {error}");
            }
        }
        path
    })
    .clone()
}

/// Aztek's default preference directory, overridden by AZTEK_PREFS.
pub fn user_prefs_dir() -> PathBuf {
    let over = env_path("AZTEK_PREFS");
    if !over.as_os_str().is_empty() {
        return over;
    }
    #[cfg(target_os = "linux")]
    return linux_config_dir();
    #[cfg(not(target_os = "linux"))]
    PathBuf::from(std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).unwrap_or_default()).join(".aztek")
}

/// %APPDATA%/Aztek, ~/.config/aztek on Linux, ~/Library/Application Support/Aztek, or AZTEK_CONFIG_DIR.
pub fn user_config_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let over = env_path("AZTEK_CONFIG_DIR");
        if !over.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(&over);
            return over;
        }
        #[cfg(target_os = "linux")]
        return linux_config_dir();
        #[cfg(not(target_os = "linux"))]
        {
            let base = if cfg!(windows) {
                env_path("APPDATA")
            } else if cfg!(target_os = "macos") {
                env_path("HOME").join("Library").join("Application Support")
            } else {
                let x = env_path("XDG_CONFIG_HOME");
                if x.as_os_str().is_empty() {
                    env_path("HOME").join(".config")
                } else {
                    x
                }
            };
            let d = base.join("Aztek");
            let _ = std::fs::create_dir_all(&d);
            d
        }
    })
    .clone()
}

pub fn executable_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.canonicalize().ok())
            .and_then(|p| p.parent().map(Path::to_path_buf))
            .unwrap_or_default()
    })
    .clone()
}

/// Installed layout first, then the source tree for dev builds.
fn find_data_dir(name: &str) -> PathBuf {
    let installed = executable_dir().join(name);
    if installed.exists() {
        return installed;
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join(name)
}

/// Read-only app data: resources/ and python/. Source tree in dev builds, next to the exe when installed.
pub fn resources_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| find_data_dir("resources")).clone()
}

pub fn python_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| find_data_dir("python")).clone()
}

/// Generic (forward slash) UTF-8 form.
pub fn to_utf8(p: &Path) -> String {
    let s = p.to_string_lossy().to_string();
    if cfg!(windows) {
        s.replace('\\', "/")
    } else {
        s
    }
}

static SCENE_DIR: Mutex<PathBuf> = Mutex::new(PathBuf::new());

/// Folder of the open scene file, empty for an untitled scene. io keeps it current.
pub fn current_scene_dir() -> PathBuf {
    SCENE_DIR.lock().unwrap().clone()
}

pub fn set_current_scene_dir(dir: &Path) {
    *SCENE_DIR.lock().unwrap() = dir.to_path_buf();
}

fn is_url(s: &str) -> bool {
    s.find("://").is_some_and(|c| c > 1)
}

fn same_element(a: &str, b: &str) -> bool {
    if cfg!(windows) {
        a.len() == b.len()
            && a.chars().zip(b.chars()).all(|(x, y)| {
                let n = |c: char| if c == '\\' { '/' } else { c.to_lowercase().next().unwrap_or(c) };
                n(x) == n(y)
            })
    } else {
        a == b
    }
}

/// The path's lexically normal elements below its root.
fn elements(p: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for c in p.components() {
        match c {
            Component::Normal(s) => out.push(s.to_string_lossy().to_string()),
            Component::ParentDir => {
                if out.last().is_some_and(|l| l != "..") {
                    out.pop();
                } else {
                    out.push("..".into());
                }
            }
            _ => {}
        }
    }
    out
}

fn root_name(p: &Path) -> String {
    match p.components().next() {
        Some(Component::Prefix(pre)) => pre.as_os_str().to_string_lossy().to_string(),
        _ => String::new(),
    }
}

/// A path as commands hand it to scripts: forward slashes, and without the `\\?\` prefix Windows' canonical form adds,
/// as the reference spells paths on every platform.
pub fn script_spelling(p: &Path) -> String {
    let s = p.to_string_lossy();
    let s = s.strip_prefix(r"\\?\UNC\").map(|rest| format!(r"\\{rest}")).unwrap_or_else(|| s.strip_prefix(r"\\?\").unwrap_or(&s).to_string());
    s.replace('\\', "/")
}

/// Lexically normal form.
pub fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(out.components().next_back(), Some(Component::Normal(_))) {
                    out.pop();
                } else {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// path relative to base, lexically, with ".." where it leaves base and case blind on Windows. None when no relative
/// form exists: either path is relative, or they sit on different drives / root names.
pub fn relative_to(path: &Path, base: &Path) -> Option<PathBuf> {
    if !path.is_absolute() || !base.is_absolute() || !same_element(&root_name(path), &root_name(base)) {
        return None;
    }
    let (p, b) = (elements(path), elements(base));
    let mut common = 0;
    while common < p.len() && common < b.len() && same_element(&p[common], &b[common]) {
        common += 1;
    }
    let mut rel = PathBuf::new();
    for _ in common..b.len() {
        rel.push("..");
    }
    for e in &p[common..] {
        rel.push(e);
    }
    Some(if rel.as_os_str().is_empty() { PathBuf::from(".") } else { rel })
}

/// How a file path is written to a scene or workspace file: relative to the workspace root when one is open and on
/// the path's drive, else to scene_dir, ".." allowed, forward slashes. Absolute only when neither has a relative form
/// (another drive) or there is no base. A relative path is taken as stored for the open scene, so it is resolved
/// first and re-expressed against scene_dir, which is where the file being written lives. Empty and URL-like strings
/// pass through.
pub fn stored_path(path: &Path, scene_dir: &Path) -> String {
    let text = to_utf8(path);
    if text.is_empty() || is_url(&text) {
        return text;
    }
    let abs = if path.is_absolute() { path.to_path_buf() } else { resolve_path(&text, &current_scene_dir()) };
    // A workspace on another drive can't hold the path, but the scene folder still can.
    if let Some(root) = Workspace::root() {
        if let Some(rel) = relative_to(&abs, &root) {
            return to_utf8(&rel);
        }
    }
    if let Some(rel) = relative_to(&abs, scene_dir) {
        return to_utf8(&rel);
    }
    to_utf8(&normalize(&abs))
}

/// A stored path made usable: absolute ones as written, relative ones against the workspace root, then scene_dir.
/// The first base where the file exists wins, then the first where its folder does (a sequence pattern names no
/// file), then the first base.
pub fn resolve_path(stored: &str, scene_dir: &Path) -> PathBuf {
    let p = PathBuf::from(stored);
    if stored.is_empty() || is_url(stored) || p.is_absolute() {
        return p;
    }
    let mut bases = Vec::new();
    if let Some(root) = Workspace::root() {
        bases.push(root);
    }
    if !scene_dir.as_os_str().is_empty() {
        bases.push(scene_dir.to_path_buf());
    }
    for b in &bases {
        if b.join(&p).exists() {
            return normalize(&b.join(&p));
        }
    }
    for b in &bases {
        if b.join(&p).parent().is_some_and(Path::is_dir) {
            return normalize(&b.join(&p));
        }
    }
    match bases.first() {
        Some(b) => normalize(&b.join(&p)),
        None => p,
    }
}
