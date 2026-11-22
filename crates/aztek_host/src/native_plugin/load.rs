//! loadPlugin, unloadPlugin and pluginInfo for compiled plug-ins, handing Python ones to the Python host.

use std::ffi::{c_char, c_void};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use aztek_ops::{ArgType, Args, CmdResult, CommandDef, CommandError, FlagDef};
use serde_json::Map;

use super::table::TABLE;
use super::*;

#[cfg(windows)]
const EXTENSIONS: &[&str] = &["dll", "mll"];
#[cfg(not(windows))]
const EXTENSIONS: &[&str] = &["so"];

pub(super) fn search_path() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for var in ["AZTEK_PLUG_IN_PATH", "MAYA_PLUG_IN_PATH"] {
        if let Some(v) = std::env::var_os(var) {
            out.extend(std::env::split_paths(&v).filter(|p| !p.as_os_str().is_empty()));
        }
    }
    // Last, the plug-ins that ship with Aztek: one of the same name on the path comes first.
    for d in shipped_dirs() {
        if !out.contains(&d) {
            out.push(d);
        }
    }
    out
}

/// Where the plug-ins built with Aztek are (plugins/ in the source): plug-ins/ beside the program (a build, Windows),
/// lib/aztek/plug-ins beside its bin/ (Linux installs, the AppImage), PlugIns/aztek in the Mac app. Those that exist.
pub fn shipped_dirs() -> Vec<PathBuf> {
    let Some(bin) = std::env::current_exe().ok().and_then(|e| e.canonicalize().ok()).and_then(|e| e.parent().map(Path::to_path_buf)) else { return Vec::new() };
    [bin.join("plug-ins"), bin.join("../lib/aztek/plug-ins"), bin.join("../PlugIns/aztek")].into_iter().filter(|d| d.is_dir()).filter_map(|d| d.canonicalize().ok()).collect()
}

fn is_native(p: &Path) -> bool {
    p.extension().and_then(|e| e.to_str()).is_some_and(|e| ["so", "dll", "mll", "bundle"].contains(&e.to_ascii_lowercase().as_str()))
}

/// What a plug-in file is, told without loading it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Built with Aztek's kit.
    Aztek,
    /// A Python plug-in.
    Python,
    /// A compiled plug-in built against the reference's libraries: it can't load here.
    Foreign,
    /// A library that isn't a plug-in of either.
    Other,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Aztek => "aztek",
            Kind::Python => "python",
            Kind::Foreign => "foreign",
            Kind::Other => "other",
        }
    }
}

fn holds(hay: &[u8], needle: &[u8]) -> bool {
    // By the first byte, then the rest: most positions are passed over at once.
    let (first, rest) = (needle[0], &needle[1..]);
    let mut from = 0;
    while let Some(p) = hay[from..].iter().position(|&b| b == first) {
        let at = from + p + 1;
        if hay[at..].starts_with(rest) {
            return true;
        }
        from = at;
    }
    false
}

/// Reads a plug-in file's bytes for what it exports and links: Aztek's kit gives every plug-in `aztek_plugin_abi`;
/// one built for the reference has `initializePlugin` and names its libraries. Nothing of the file is run.
pub fn kind_of(path: &Path) -> Kind {
    if path.extension().is_some_and(|e| e == "py") {
        return Kind::Python;
    }
    // Remembered by size and time, so a list made again does not read a large library again.
    let stamp = std::fs::metadata(path).ok().map(|m| (m.len(), m.modified().ok()));
    if let Some(k) = KINDS.with(|c| c.borrow().get(path).filter(|(s, _)| *s == stamp).map(|(_, k)| *k)) {
        return k;
    }
    let Ok(bytes) = std::fs::read(path) else { return Kind::Other };
    let kind = if holds(&bytes, b"aztek_plugin_abi") {
        Kind::Aztek
    } else if holds(&bytes, b"initializePlugin") || holds(&bytes, b"OpenMaya") {
        Kind::Foreign
    } else {
        Kind::Other
    };
    KINDS.with(|c| c.borrow_mut().insert(path.to_path_buf(), (stamp, kind)));
    kind
}

type Stamp = Option<(u64, Option<std::time::SystemTime>)>;
thread_local! {
    static KINDS: std::cell::RefCell<std::collections::HashMap<PathBuf, (Stamp, Kind)>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

pub const FOREIGN_NOTE: &str = "This plug-in was compiled for Maya. It needs to be recompiled for Aztek from its source, with Aztek's plug-in kit (docs/PLUGINS.md).";

/// Where the names of the plug-ins loaded at start are kept.
fn autoload_file() -> PathBuf {
    crate::user_folder::prefs_base().join("pluginPrefs.json")
}

/// The plug-ins loaded at start, by file path.
pub fn autoload_list() -> Vec<String> {
    std::fs::read_to_string(autoload_file())
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v["autoload"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()))
        .unwrap_or_default()
}

pub(super) fn set_autoload(path: &str, on: bool) -> std::io::Result<()> {
    let mut list = autoload_list();
    list.retain(|p| p != path);
    if on {
        list.push(path.to_string());
    }
    std::fs::create_dir_all(crate::user_folder::prefs_base())?;
    std::fs::write(autoload_file(), json!({"autoload": list}).to_string())
}

/// Every plug-in file in the folders of the plug-in path, in the path's order, then what is loaded from elsewhere.
pub(super) fn list_plugins(s: &mut Session) -> Value {
    let mut loaded: Vec<(String, String)> = PLUGINS.with(|p| p.borrow().iter().map(|x| (x.name.clone(), x.path.clone())).collect());
    if python_started() {
        let mut f = Map::new();
        f.insert("listPlugins".into(), json!(true));
        if let Ok(Value::Array(names)) = python_call(s, "plugin_info", None, &f) {
            for n in names.iter().filter_map(Value::as_str) {
                let mut f = Map::new();
                f.insert("path".into(), json!(true));
                let path = python_call(s, "plugin_info", Some(n), &f).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
                loaded.push((n.to_string(), path));
            }
        }
    }
    let auto = autoload_list();
    let same = |a: &str, b: &Path| !a.is_empty() && (Path::new(a) == b || std::fs::canonicalize(a).ok() == std::fs::canonicalize(b).ok());
    // The ones that come with Aztek first.
    let mut out = crate::builtin_plugins::rows();
    let mut seen: Vec<PathBuf> = Vec::new();
    let row = |path: &Path, folder: &Path, loaded: &[(String, String)]| {
        let kind = kind_of(path);
        if kind == Kind::Other {
            return None;
        }
        let name = key_of(&path.to_string_lossy());
        let is_loaded = loaded.iter().any(|(n, p)| same(p, path) || (p.is_empty() && *n == name));
        Some(json!({
            "name": name,
            "file": path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default(),
            "path": path.to_string_lossy(),
            "folder": folder.to_string_lossy(),
            "kind": kind.name(),
            "loaded": is_loaded,
            "autoload": auto.iter().any(|a| same(a, path)),
            "note": if kind == Kind::Foreign { FOREIGN_NOTE } else { "" },
        }))
    };
    for dir in search_path() {
        if seen.contains(&dir) {
            continue;
        }
        seen.push(dir.clone());
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.is_file() && (is_native(p) || p.extension().is_some_and(|e| e == "py"))).collect();
        files.sort();
        for f in files {
            if let Some(r) = row(&f, &dir, &loaded) {
                out.push(r);
            }
        }
    }
    // Loaded by path from somewhere off the plug-in path.
    for (name, path) in &loaded {
        let p = Path::new(path);
        if path.is_empty() || out.iter().any(|r| same(r["path"].as_str().unwrap_or(""), p)) {
            continue;
        }
        let folder = p.parent().unwrap_or(Path::new("")).to_path_buf();
        match row(p, &folder, &loaded) {
            Some(r) => out.push(r),
            None => out.push(json!({"name": name, "file": name, "path": path, "folder": folder.to_string_lossy(), "kind": "aztek", "loaded": true, "autoload": false, "note": ""})),
        }
    }
    Value::Array(out)
}

/// Loads the plug-ins marked to load at start. One that fails is reported and the rest go on.
pub fn load_autoload(s: &mut Session) -> Vec<String> {
    let mut errors = Vec::new();
    for path in autoload_list() {
        if !Path::new(&path).is_file() {
            errors.push(format!("Plug-in {path} is marked to load at start and is no longer there."));
            continue;
        }
        let a = Args { positional: vec![json!(path)], flags: Map::new() };
        if let Err(e) = load_plugin(s, &a) {
            errors.push(format!("{path}: {e}"));
        }
    }
    errors
}

/// A compiled plug-in's file: a path, or a name looked for on the plug-in path.
pub(crate) fn find_native(name: &str) -> Option<PathBuf> {
    let p = Path::new(name);
    if p.is_file() && is_native(p) {
        return Some(std::fs::canonicalize(p).unwrap_or(p.to_path_buf()));
    }
    if p.components().count() > 1 {
        return None;
    }
    // The first one built with Aztek's kit: one built for the reference earlier on the path (an mGear install's own
    // binaries, say) can't load, and the one Aztek ships with may come after it.
    let mut first = None;
    for dir in search_path() {
        let mut candidates = Vec::new();
        if is_native(p) {
            candidates.push(dir.join(name));
        }
        candidates.extend(EXTENSIONS.iter().map(|ext| dir.join(format!("{name}.{ext}"))));
        for f in candidates.into_iter().filter(|f| f.is_file()) {
            if kind_of(&f) == Kind::Aztek {
                return Some(f);
            }
            first.get_or_insert(f);
        }
    }
    first
}

/// Whether a Python plug-in by that name is there to load.
pub(super) fn python_file(name: &str) -> bool {
    let p = Path::new(name);
    if p.extension().is_some_and(|e| e == "py") && p.is_file() {
        return true;
    }
    let file = if name.ends_with(".py") { name.to_string() } else { format!("{name}.py") };
    search_path().iter().any(|d| d.join(&file).is_file())
}

pub(crate) fn key_of(name: &str) -> String {
    let p = Path::new(name);
    let base = p.file_name().and_then(|b| b.to_str()).unwrap_or(name);
    for ext in ["so", "dll", "mll", "bundle", "py"] {
        if let Some(stem) = base.strip_suffix(&format!(".{ext}")) {
            return stem.to_string();
        }
    }
    base.to_string()
}

fn open(path: &Path) -> Result<libloading::Library, String> {
    // openMayaPref -lazyLoad: unresolved references fail when called rather than when loaded.
    #[cfg(unix)]
    let r = {
        let bind = if crate::sys_info::lazy_load() { libc::RTLD_LAZY } else { libc::RTLD_NOW };
        unsafe { libloading::os::unix::Library::open(Some(path), bind | libc::RTLD_LOCAL) }.map(libloading::Library::from)
    };
    #[cfg(windows)]
    let r = unsafe { libloading::os::windows::Library::load_with_flags(path, libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH) }.map(libloading::Library::from);
    r.map_err(|e| {
        let text = e.to_string();
        let foreign = ["libOpenMaya", "OpenMaya.dll", "libFoundation", "Foundation.dll"].iter().any(|n| text.contains(n));
        if foreign {
            format!("{} was built against another application's libraries and can't load here. Rebuild it from source against Aztek's kit (see PLUGINS.md): {text}", path.display())
        } else {
            text
        }
    })
}

fn load_native(s: &mut Session, key: &str, path: &Path) -> Result<(), String> {
    let lib = open(path)?;
    let abi = unsafe { lib.get::<unsafe extern "C" fn() -> i32>(b"aztek_plugin_abi\0") }
        .map_err(|_| format!("{} was not built with Aztek's kit: it has no aztek_plugin_abi. A plug-in built for another application can't load; rebuild it from source with aztek_add_plugin (see PLUGINS.md).", path.display()))?;
    let version = unsafe { abi() };
    if version < 1 || version as u32 > ABI_VERSION {
        return Err(format!("{} was built with a newer kit (ABI {version}) than this Aztek has ({ABI_VERSION}).", path.display()));
    }
    let load = unsafe { lib.get::<unsafe extern "C" fn(*const c_void, *mut c_void, *mut *mut c_char) -> i32>(b"aztek_plugin_load\0") }.map_err(|e| e.to_string())?;
    let load = *load;
    let unload = unsafe { lib.get::<unsafe extern "C" fn(*mut *mut c_char) -> i32>(b"aztek_plugin_unload\0") }.ok().map(|f| *f);
    let lib = Arc::new(Lib(lib));
    let state = Box::new(PluginState {
        name: key.to_string(),
        path: path.to_string_lossy().into_owned(),
        vendor: "Unknown".into(),
        version: "Unknown".into(),
        api: "Any".into(),
        abi: version as u32,
        lib,
        commands: Vec::new(),
        nodes: Vec::new(),
        data: Vec::new(),
        callbacks: Vec::new(),
        unload,
    });
    let ptr = PLUGINS.with(|p| {
        let mut v = p.borrow_mut();
        v.push(state);
        &mut **v.last_mut().unwrap() as *mut PluginState
    });
    let mut err: *mut c_char = std::ptr::null_mut();
    let rc = nested(s, || unsafe { load(&TABLE as *const _ as *const c_void, ptr as *mut c_void, &mut err) });
    let err = take(err);
    if rc != AZ_OK {
        take_back(s, ptr);
        PLUGINS.with(|p| p.borrow_mut().retain(|x| x.name != key));
        return Err(match rc {
            AZ_CRASHED => format!("initializePlugin of {key} crashed ({err})"),
            _ if err.is_empty() => format!("initializePlugin of {key} failed"),
            _ => format!("initializePlugin of {key} failed: {err}"),
        });
    }
    // openMayaPref -oldPluginWarning: built with an older kit, or for an older release than the reference's.
    if crate::sys_info::old_plugin_warning() {
        let api = PLUGINS.with(|p| p.borrow().iter().find(|x| x.name == key).map(|x| x.api.clone())).unwrap_or_default();
        if let Some(w) = crate::sys_info::old_plugin_text(key, version as u32, ABI_VERSION, &api) {
            defer(w, LineKind::Warning);
        }
    }
    Ok(())
}

/// What a plug-in still has registered goes when it does.
fn take_back(s: &mut Session, p: *mut PluginState) {
    let p = unsafe { &mut *p };
    for name in std::mem::take(&mut p.commands) {
        s.commands.unregister(&name);
    }
    keep_commands(s);
    for name in std::mem::take(&mut p.nodes) {
        aztek_scene::scripted::deregister_type(&name);
    }
    for name in std::mem::take(&mut p.data) {
        super::nodes::forget_data(&name);
    }
    for id in std::mem::take(&mut p.callbacks) {
        CALLBACKS.with(|c| c.borrow_mut().remove(&id));
        super::messages::unwatch(id);
    }
}

fn unload_native(s: &mut Session, key: &str, force: bool) -> Result<(), String> {
    let p = plugin_named(key).ok_or_else(|| format!("Plug-in \"{key}\" is not loaded."))?;
    let types: Vec<String> = unsafe { (*p).nodes.clone() };
    let in_use = s.graph.nodes().into_iter().filter(|&id| s.graph.node(id).is_some_and(|n| types.contains(&n.ty().name))).count();
    if in_use > 0 && !force {
        return Err(format!("Plug-in \"{key}\" is still in use: the scene has {in_use} of its nodes. Delete them first, or unload with -force."));
    }
    let mut err: *mut c_char = std::ptr::null_mut();
    let rc = match unsafe { (*p).unload } {
        Some(f) => nested(s, || unsafe { f(&mut err) }),
        None => AZ_OK,
    };
    let err = take(err);
    take_back(s, p);
    // Nodes, undo steps and commands still holding the library keep it mapped until they go.
    PLUGINS.with(|v| v.borrow_mut().retain(|x| x.name != key));
    match rc {
        AZ_OK => Ok(()),
        AZ_CRASHED => Err(format!("uninitializePlugin of {key} crashed ({err})")),
        _ => Err(format!("uninitializePlugin of {key} failed: {err}")),
    }
}

#[cfg(feature = "python")]
pub(super) fn python_call(s: &mut Session, method: &'static str, name: Option<&str>, flags: &Map<String, Value>) -> CmdResult {
    let a = Args { positional: name.map(|n| vec![json!(n)]).unwrap_or_default(), flags: flags.clone() };
    crate::python::plugin_command(s, method, &a)
}

#[cfg(feature = "python")]
pub(super) fn python_started() -> bool {
    crate::python::started()
}

#[cfg(not(feature = "python"))]
pub(super) fn python_call(_s: &mut Session, _method: &'static str, name: Option<&str>, _flags: &Map<String, Value>) -> CmdResult {
    Err(CommandError::new(format!("Plug-in \"{}\": this build has no Python.", name.unwrap_or(""))))
}

#[cfg(not(feature = "python"))]
pub(super) fn python_started() -> bool {
    false
}

thread_local! {
    /// loadPlugin -addCallback: scripts run with the name of each plug-in loaded after.
    static LOAD_CALLBACKS: std::cell::RefCell<Vec<Value>> = const { std::cell::RefCell::new(Vec::new()) };
}

thread_local! {
    /// unloadPlugin -addCallback: scripts run with the name of each plug-in about to unload.
    static UNLOAD_CALLBACKS: std::cell::RefCell<Vec<Value>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// The callbacks -addCallback gave, each due with the plug-in's name: a MEL global proc by name, a Python string with %s
/// for the name, or a Python callable.
fn queue_load_callbacks(name: &str) {
    let list = LOAD_CALLBACKS.with(|c| c.borrow().clone());
    for cb in list {
        let call = match &cb {
            Value::String(src) if src.contains("%s") => json!({ "__pysrc__": src.replace("%s", name) }),
            Value::String(src) => json!(format!("{src} \"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))),
            Value::Object(o) if o.contains_key("__pysrc__") => json!({ "__pysrc__": o["__pysrc__"].as_str().unwrap_or("").replace("%s", name) }),
            other => json!({ "__cb__": other, "__args__": [name] }),
        };
        crate::script_jobs::defer(call);
    }
}

/// Every plug-in file on the plug-in path that can load here and isn't loaded, for -allPlugins.
fn all_plugin_files(s: &mut Session) -> Vec<Value> {
    let rows = list_plugins(s);
    rows.as_array()
        .into_iter()
        .flatten()
        .filter(|r| matches!(r["kind"].as_str(), Some("aztek") | Some("python")) && !r["loaded"].as_bool().unwrap_or(false))
        .filter_map(|r| r["path"].as_str().filter(|p| !p.is_empty()).map(|p| json!(p)))
        .collect()
}

fn load_plugin(s: &mut Session, a: &Args) -> CmdResult {
    let quiet = a.bool("quiet");
    if let Some(cb) = a.get("addCallback") {
        LOAD_CALLBACKS.with(|c| {
            let mut c = c.borrow_mut();
            if !c.contains(cb) {
                c.push(cb.clone());
            }
        });
    }
    if let Some(cb) = a.get("removeCallback") {
        LOAD_CALLBACKS.with(|c| c.borrow_mut().retain(|x| x != cb));
    }
    let names: Vec<Value> = if a.bool("allPlugins") { all_plugin_files(s) } else { a.positional.clone() };
    let mut out = Vec::new();
    for v in &names {
        let Some(name) = v.as_str() else { continue };
        // -name: the plug-in registers under this name, numbered when it is taken.
        let key = match a.get("name").and_then(Value::as_str) {
            Some(n) if !n.is_empty() => {
                let mut k = n.to_string();
                let mut i = 1;
                while plugin_named(&k).is_some() {
                    k = format!("{n}{i}");
                    i += 1;
                }
                k
            }
            _ => key_of(name),
        };
        // -pluginPath: only where the file is.
        if a.bool("pluginPath") {
            let path = find_native(name).map(|p| p.to_string_lossy().into_owned()).or_else(|| {
                let p = Path::new(name);
                if p.is_file() {
                    return Some(std::fs::canonicalize(p).unwrap_or(p.to_path_buf()).to_string_lossy().into_owned());
                }
                let file = if name.ends_with(".py") { name.to_string() } else { format!("{name}.py") };
                search_path().into_iter().map(|d| d.join(&file)).find(|f| f.is_file()).map(|f| f.to_string_lossy().into_owned())
            });
            out.push(json!(path.map(|p| aztek_core::paths::script_spelling(Path::new(&p))).unwrap_or_default()));
            continue;
        }
        let before = out.len();
        load_one(s, a, name, &key, quiet, &mut out)?;
        let named = a.get("name").and_then(Value::as_str).is_some_and(|n| !n.is_empty());
        for loaded in out[before..].iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>() {
            super::info::registry_changed(&loaded, true, named && loaded == key);
            queue_load_callbacks(&loaded);
        }
    }
    Ok(Value::Array(out))
}

fn load_one(s: &mut Session, a: &Args, name: &str, key: &str, quiet: bool, out: &mut Vec<Value>) -> CmdResult<()> {
    let key = key.to_string();
    {
        // One that comes with Aztek.
        if let Some(b) = crate::builtin_plugins::find(name) {
            crate::builtin_plugins::load(s, b);
            out.push(json!(b.name));
            return Ok(());
        }
        if plugin_named(&key).is_some() {
            out.push(json!(key));
            return Ok(());
        }
        if let Some(path) = find_native(name) {
            if kind_of(&path) == Kind::Foreign {
                return Err(CommandError::new(format!("{}: {FOREIGN_NOTE}", path.display())));
            }
            let file = path.to_string_lossy().into_owned();
            messages::scene(s, messages::BEFORE_PLUGIN_LOAD, json!({ "strings": [file] }));
            load_native(s, &key, &path).map_err(CommandError::new)?;
            messages::scene(s, messages::AFTER_PLUGIN_LOAD, json!({ "strings": [file, key] }));
            out.push(json!(key));
            return Ok(());
        }
        if python_file(name) {
            let mut f = Map::new();
            f.insert("quiet".into(), json!(quiet));
            if let Some(n) = a.get("name") {
                f.insert("name".into(), n.clone());
            }
            messages::scene(s, messages::BEFORE_PLUGIN_LOAD, json!({ "strings": [name] }));
            if let Value::Array(l) = python_call(s, "load_plugin", Some(name), &f)? {
                messages::scene(s, messages::AFTER_PLUGIN_LOAD, json!({ "strings": [name, key] }));
                out.extend(l);
            }
            return Ok(());
        }
        // Not found anywhere.
        if !quiet {
            return Err(CommandError::new(format!("Plug-in \"{name}\" was not found on the plug-in path.")));
        }
    }
    Ok(())
}

fn unload_plugin(s: &mut Session, a: &Args) -> CmdResult {
    let force = a.bool("force");
    if let Some(cb) = a.get("addCallback") {
        UNLOAD_CALLBACKS.with(|c| {
            let mut c = c.borrow_mut();
            if !c.contains(cb) {
                c.push(cb.clone());
            }
        });
    }
    if let Some(cb) = a.get("removeCallback") {
        UNLOAD_CALLBACKS.with(|c| c.borrow_mut().retain(|x| x != cb));
    }
    let mut done = Vec::new();
    for v in &a.positional {
        let Some(name) = v.as_str() else { continue };
        let key = key_of(name);
        let loaded = crate::builtin_plugins::find(name).is_some() || plugin_named(&key).is_some() || python_started();
        if loaded {
            // Run as soon as the command returns, first among what it leaves to run.
            for cb in UNLOAD_CALLBACKS.with(|c| c.borrow().clone()) {
                let (call, args) = match &cb {
                    Value::String(src) if src.contains("%s") => (json!({ "__pysrc__": src.replace("%s", &key) }), Vec::new()),
                    Value::String(src) => (json!(format!("{src} \"{}\"", key.replace('\\', "\\\\").replace('"', "\\\""))), Vec::new()),
                    Value::Object(o) if o.contains_key("__pysrc__") => (json!({ "__pysrc__": o["__pysrc__"].as_str().unwrap_or("").replace("%s", &key) }), Vec::new()),
                    other => (other.clone(), vec![json!(key)]),
                };
                crate::scriptui::run_soon(call, args);
            }
        }
        if let Some(b) = crate::builtin_plugins::find(name) {
            crate::builtin_plugins::unload(s, b);
        } else if let Some(p) = plugin_named(&key) {
            let file = unsafe { (*p).path.clone() };
            messages::scene(s, messages::BEFORE_PLUGIN_UNLOAD, json!({ "strings": [key] }));
            unload_native(s, &key, force).map_err(CommandError::new)?;
            messages::scene(s, messages::AFTER_PLUGIN_UNLOAD, json!({ "strings": [key, file] }));
        } else if python_started() {
            messages::scene(s, messages::BEFORE_PLUGIN_UNLOAD, json!({ "strings": [key] }));
            python_call(s, "unload_plugin", Some(name), &a.flags)?;
            messages::scene(s, messages::AFTER_PLUGIN_UNLOAD, json!({ "strings": [key, name] }));
        } else {
            return Err(CommandError::new(format!("Plug-in \"{name}\" is not loaded.")));
        }
        super::info::registry_changed(&key, false, false);
        done.push(json!(key));
    }
    // The internal names of the plug-ins unloaded.
    Ok(Value::Array(done))
}

/// loadPlugin, unloadPlugin and pluginInfo for every kind of plug-in, and the command callbacks arrive by.
pub fn register_commands(s: &mut Session) {
    let b = |name: &str, short: &str, doc: &str| FlagDef::new(name, short, ArgType::Bool, Value::Null, doc);
    s.register(
        CommandDef::<Session>::new(
            "loadPlugin",
            "Load a plug-in: a compiled one (.so, .dll) built with Aztek's kit or a Python one (.py), by path or by name from the folders in AZTEK_PLUG_IN_PATH and MAYA_PLUG_IN_PATH. Returns its name in a list.",
            load_plugin,
        )
        .positional("plug-in file or name")
        .flag(b("quiet", "qt", "Don't complain when it can't be found."))
        .flag(b("allPlugins", "a", "Load every plug-in on the plug-in path."))
        .flag(FlagDef::new("name", "n", ArgType::String, Value::Null, "The name the plug-in registers under, numbered when it is taken."))
        .flag(b("pluginPath", "pp", "The full path of the plug-in file found, or \"\" when there is none; nothing loads."))
        .flag(FlagDef::new("addCallback", "ac", ArgType::Any, Value::Null, "A script run after each plug-in loads, given its name: a MEL global proc taking string $pluginName, or Python (a string with %s, or a callable)."))
        .flag(FlagDef::new("removeCallback", "rc", ArgType::Any, Value::Null, "Take back a script -addCallback gave."))
        .undoable(false),
    );
    s.register(
        CommandDef::<Session>::new("unloadPlugin", "Unload a plug-in: its uninitializePlugin runs and its commands and node types go. A compiled one in use by the scene needs -force.", unload_plugin)
            .positional("plug-in name")
            .flag(b("force", "f", "Unload even with its nodes in the scene."))
            .flag(FlagDef::new("addCallback", "ac", ArgType::Any, Value::Null, "A script run as each plug-in unloads, given its name: a MEL global proc taking string $pluginName, or Python (a string with %s, or a callable)."))
            .flag(FlagDef::new("removeCallback", "rc", ArgType::Any, Value::Null, "Take back a script -addCallback gave."))
            .undoable(false),
    );
    let st = |name: &str, short: &str, doc: &str| FlagDef::new(name, short, ArgType::String, Value::Null, doc);
    let mut info = CommandDef::<Session>::new(
        "pluginInfo",
        "The plug-in registry: what each plug-in registered (-q -command, -dependNode, -translator, ...), whether it is loaded, loads at start or is in use, and the plug-in preferences.",
        super::info::plugin_info,
    )
    .positional("plug-in name or path")
    .flag(b("query", "q", "Query."))
    .flag(b("edit", "e", "Edit."))
    .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What is asked."));
    for (n, sh, doc) in [
        ("activeFile", "af", "-dependNode and -pluginsInUse look only at the selection, as an export of the selection writes it."),
        ("allEvaluators", "ae", "The evaluator types every plug-in registered."),
        ("apiVersion", "av", "The API version it was built against."),
        ("autoload", "a", "Whether it loads at every start: -e -autoload true name sets it, -q -autoload name asks."),
        ("cacheFormat", "cf", "The geometry cache formats plug-ins registered."),
        ("constraintCommand", "cnc", "The constraint commands it registered."),
        ("controlCommand", "ctc", "The control commands it registered."),
        ("dependNode", "dn", "The node types it registered."),
        ("device", "dv", "The devices it registered."),
        ("dragAndDropBehavior", "ddb", "The drag and drop behaviours it registered."),
        ("evaluator", "e", "The evaluator types it registered."),
        ("iksolver", "ik", "The IK solvers it registered."),
        ("listPlugins", "ls", "The loaded plug-ins."),
        ("listPluginsPath", "lsp", "The full paths of the loaded plug-ins."),
        ("loadPluginPrefs", "lpp", "Read the plug-in preferences (what loads at start) from pluginPrefs.mel."),
        ("loaded", "l", "Whether it is loaded."),
        ("modelEditorCommand", "mec", "The model editor commands it registered."),
        ("pluginsInUse", "pu", "The plug-ins the scene uses."),
        ("referenceTranslators", "rtr", "With -pluginsInUse: true lists only those with translators that read the scene's referenced files, false only the others."),
        ("registered", "r", "Whether the registry knows it."),
        ("remove", "rm", "Edit: take its record out of the registry."),
        ("renderer", "rdr", "The renderers it registered."),
        ("savePluginPrefs", "spp", "Write the plug-in preferences (what loads at start) to pluginPrefs.mel."),
        ("serviceDescriptions", "sd", "Short descriptions of the services it uses."),
        ("settings", "set", "Whether it is loaded, loads at start and is registered."),
        ("translator", "tr", "The file translators it registered."),
        ("unloadOk", "uo", "Whether it can be unloaded safely: false while the scene has nodes of its types."),
        ("userNamed", "u", "Whether it was given its name by loadPlugin -name."),
        ("version", "v", "Its version."),
        ("writeRequires", "wr", "Whether saved files require it, so opening them loads it."),
    ] {
        info = info.flag(b(n, sh, doc));
    }
    for (n, sh, doc) in [
        ("animCurveInterp", "aci", "The animation curve interpolators it registered."),
        ("changedCommand", "cc", "A script run whenever the registry changes."),
        ("command", "c", "The commands it registered."),
        ("data", "d", "The data types it registered."),
        ("dependNodeByType", "dnt", "The plug-in node types of an MPxNode type class, such as kLocatorNode."),
        ("dependNodeId", "dni", "The ids of the node types it registered."),
        ("name", "n", "The name it is registered under."),
        ("path", "p", "Its file."),
        ("pluginForNodeType", "pnt", "The plug-in that registered a node type, or \"\"."),
        ("tool", "t", "The tool contexts it registered."),
        ("vendor", "vd", "Its vendor."),
    ] {
        info = info.flag(st(n, sh, doc));
    }
    s.register(info.undoable(false));
    s.register(
        CommandDef::<Session>::new(
            "aztekPlugins",
            "Every plug-in file in the folders of the plug-in path, for the Plug-in Manager: its name, file, folder, kind (aztek, python, or foreign for one compiled for another application), and whether it is loaded and loads at start.",
            |s, _a| Ok(list_plugins(s)),
        )
        .undoable(false),
    );
    s.register(
        CommandDef::<Session>::new("aztekPluginCallback", "Runs a compiled plug-in's message callback by its id; scriptJobs the kit makes call it.", |s, a| {
            let id = a.positional.first().and_then(Value::as_i64).unwrap_or(0);
            let payload = a.positional.get(1).map(|v| v.as_str().map(str::to_string).unwrap_or(v.to_string())).unwrap_or_default();
            super::table::fire_callback(s, id, &payload).map_err(CommandError::new)?;
            Ok(Value::Null)
        })
        .positional("callback id, then what it is told")
        .undoable(false),
    );
}
