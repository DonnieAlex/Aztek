//! pluginInfo: the plug-in registry, for every kind of plug-in (those that come with Aztek, compiled ones and Python
//! ones). What each registered is gathered into one record, which the query flags read.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;

use aztek_ops::{Args, CmdResult, CommandError};
use serde_json::{json, Map, Value};

use super::load::{autoload_list, find_native, key_of, list_plugins, python_call, python_file, python_started, set_autoload};
use super::*;
use aztek_scene::plug::NodeId;

thread_local! {
    /// Plug-ins loaded under a name loadPlugin -name gave.
    static USER_NAMED: RefCell<BTreeSet<String>> = const { RefCell::new(BTreeSet::new()) };
    /// Plug-ins whose records -remove took out of the registry.
    static REMOVED: RefCell<BTreeSet<String>> = const { RefCell::new(BTreeSet::new()) };
    /// Plug-ins -writeRequires turned on, loaded or not.
    static WRITE_REQUIRES: RefCell<BTreeSet<String>> = const { RefCell::new(BTreeSet::new()) };
    /// -changedCommand scripts, run whenever the registry changes.
    static CHANGED: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// What a loaded plug-in registered.
#[derive(Default, Debug)]
struct Record {
    name: String,
    path: String,
    vendor: String,
    version: String,
    api: String,
    commands: Vec<String>,
    nodes: Vec<String>,
    data: Vec<String>,
    translators: Vec<String>,
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn translators_of(owner: &str) -> Vec<String> {
    aztek_scene::ma_file::plugin_translators().into_iter().filter(|t| t.owner == owner).map(|t| t.name).collect()
}

fn record(s: &mut Session, name: &str) -> Option<Record> {
    if let Some(b) = crate::builtin_plugins::find(name) {
        if !crate::builtin_plugins::is_loaded(b.name) {
            return None;
        }
        let commands: Vec<String> = b.all_commands().into_iter().map(str::to_string).collect();
        let mut translators: Vec<String> =
            aztek_scene::ma_file::TRANSLATORS.iter().filter(|t| t.4.is_some_and(|c| commands.iter().any(|x| x == c))).map(|t| t.0.to_string()).collect();
        translators.extend(translators_of(b.name));
        return Some(Record {
            name: b.name.to_string(),
            path: b.name.to_string(),
            vendor: crate::builtin_plugins::VENDOR.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            api: aztek_core::REFERENCE_VERSION.to_string(),
            commands,
            nodes: b.nodes.iter().map(|n| n.to_string()).collect(),
            data: Vec::new(),
            translators,
        });
    }
    let key = key_of(name);
    if let Some(p) = plugin_named(&key).or_else(|| PLUGINS.with(|v| v.borrow_mut().iter_mut().find(|x| x.path == name).map(|b| &mut **b as *mut PluginState))) {
        let p = unsafe { &*p };
        return Some(Record {
            name: p.name.clone(),
            path: p.path.clone(),
            vendor: p.vendor.clone(),
            version: p.version.clone(),
            api: p.api.clone(),
            commands: p.commands.clone(),
            nodes: p.nodes.clone(),
            data: p.data.clone(),
            translators: translators_of(&p.name),
        });
    }
    if python_started() {
        let mut f = Map::new();
        f.insert("__record__".into(), json!(true));
        if let Ok(r) = python_call(s, "plugin_info", Some(name), &f) {
            if r.is_object() {
                let txt = |k: &str| r[k].as_str().unwrap_or("").to_string();
                return Some(Record {
                    name: txt("name"),
                    path: txt("path"),
                    vendor: txt("vendor"),
                    version: txt("version"),
                    api: txt("api"),
                    commands: strings(&r["commands"]),
                    nodes: strings(&r["nodes"]),
                    data: strings(&r["data"]),
                    translators: strings(&r["translators"]),
                });
            }
        }
    }
    None
}

/// The loaded plug-ins' names.
fn loaded_all(s: &mut Session) -> Vec<String> {
    let mut names = loaded_names();
    names.extend(crate::builtin_plugins::loaded());
    if python_started() {
        let mut f = Map::new();
        f.insert("listPlugins".into(), json!(true));
        if let Ok(Value::Array(l)) = python_call(s, "plugin_info", None, &f) {
            names.extend(l.into_iter().filter_map(|x| x.as_str().map(str::to_string)));
        }
    }
    names.sort();
    names.dedup();
    names
}

fn is_loaded(s: &mut Session, name: &str) -> bool {
    if let Some(b) = crate::builtin_plugins::find(name) {
        return crate::builtin_plugins::is_loaded(b.name);
    }
    record(s, name).is_some()
}

fn known(name: &str) -> bool {
    crate::builtin_plugins::find(name).is_some() || find_native(name).is_some() || python_file(name)
}

fn is_registered(s: &mut Session, name: &str) -> bool {
    let key = crate::builtin_plugins::find(name).map_or_else(|| key_of(name), |b| b.name.to_string());
    if REMOVED.with(|r| r.borrow().contains(&key)) {
        return false;
    }
    is_loaded(s, name) || known(name)
}

fn autoload_of(s: &mut Session, name: &str) -> bool {
    if crate::builtin_plugins::find(name).is_some() {
        return crate::builtin_plugins::autoload(name);
    }
    match plugin_file(s, name) {
        Some(path) => autoload_list().iter().any(|p| *p == path),
        None => false,
    }
}

/// A plug-in's file: the one loaded under that name, or the one that name finds on the path.
fn plugin_file(s: &mut Session, name: &str) -> Option<String> {
    let key = key_of(name);
    let rows = list_plugins(s);
    let row = rows.as_array().and_then(|r| r.iter().find(|r| r["path"] == name).or_else(|| r.iter().find(|r| r["name"] == key.as_str())));
    row.and_then(|r| r["path"].as_str()).map(str::to_string).or_else(|| Path::new(name).is_file().then(|| name.to_string()))
}

/// Node types of every loaded plug-in, with their owner.
fn all_plugin_types() -> Vec<(String, String)> {
    let mut v = aztek_scene::scripted::plugin_types();
    for b in crate::builtin_plugins::BUILTINS {
        if crate::builtin_plugins::is_loaded(b.name) {
            v.extend(b.nodes.iter().map(|n| (n.to_string(), b.name.to_string())));
        }
    }
    v
}

/// The reference's MPxNode type classes, as -dependNodeByType names them, a type belongs to.
fn type_class(name: &str) -> &'static str {
    use aztek_scene::scripted::NativeKind as K;
    if let Some(k) = aztek_scene::scripted::plugin_kind(name) {
        return match k {
            K::Depend => "kDependNode",
            K::Deformer => "kDeformerNode",
            K::Locator => "kLocatorNode",
            K::Transform => "kTransformNode",
            K::Constraint => "kConstraintNode",
            K::Shape => "kSurfaceShape",
            K::IkSolver => "kIkSolverNode",
        };
    }
    let Some(t) = aztek_scene::node::types::find(name) else { return "kDependNode" };
    if t.is_a("locator") {
        "kLocatorNode"
    } else if t.is_a("skinCluster") {
        "kSkinCluster"
    } else if t.is_a("blendShape") {
        "kBlendShape"
    } else if t.is_a("geometryFilter") {
        "kDeformerNode"
    } else if t.is_a("transform") {
        "kTransformNode"
    } else if t.is_a("objectSet") {
        "kObjectSet"
    } else if t.is_a("shape") {
        "kSurfaceShape"
    } else {
        "kDependNode"
    }
}

fn class_matches(class: &str, want: &str) -> bool {
    // The deformer classes sit under kGeometryFilter, and kDeformerNode takes in skin clusters and blend shapes.
    class == want || (want == "kGeometryFilter" && matches!(class, "kDeformerNode" | "kSkinCluster" | "kBlendShape")) || (want == "kDeformerNode" && matches!(class, "kSkinCluster" | "kBlendShape"))
}

/// The nodes -activeFile limits to: the selection and what is under it, as an export of the selection writes.
fn active_nodes(s: &Session) -> BTreeSet<NodeId> {
    let g = &s.graph;
    let mut out = BTreeSet::new();
    for &id in g.selection() {
        out.insert(id);
        out.extend(g.descendants(id));
    }
    out
}

/// Node types with nodes in the scene, or among `only`.
fn types_in_use(s: &Session, only: Option<&BTreeSet<NodeId>>) -> BTreeSet<String> {
    let g = &s.graph;
    g.nodes().into_iter().filter(|id| only.is_none_or(|o| o.contains(id))).filter_map(|id| g.node(id).map(|n| n.ty().name.clone())).collect()
}

fn plugins_in_use(s: &mut Session, active: bool, reference_translators: Option<bool>) -> Vec<String> {
    let only = active.then(|| active_nodes(s));
    let used = types_in_use(s, only.as_ref());
    let mut out: BTreeSet<String> = all_plugin_types().into_iter().filter(|(t, _)| used.contains(t)).map(|(_, o)| o).collect();
    let loaded = loaded_all(s);
    out.extend(WRITE_REQUIRES.with(|w| w.borrow().iter().filter(|p| loaded.contains(p)).cloned().collect::<Vec<_>>()));
    let mut out: Vec<String> = out.into_iter().collect();
    if let Some(want) = reference_translators {
        // The translators that read referenced files: those of the files' extensions.
        let refs = s.run("file", json!({"query": true, "reference": true})).ok().map(|v| strings(&v)).unwrap_or_default();
        let exts: BTreeSet<String> = refs.iter().filter_map(|r| Path::new(r.split('{').next().unwrap_or(r)).extension().map(|e| e.to_string_lossy().to_ascii_lowercase())).collect();
        let mut keep = Vec::new();
        for p in out {
            let rec = record(s, &p);
            let tr = rec.as_ref().map(|r| r.translators.clone()).unwrap_or_default();
            let reads_refs = tr.iter().any(|t| {
                let ext = aztek_scene::ma_file::TRANSLATORS.iter().find(|x| x.0 == t).map(|x| x.1.to_string()).or_else(|| aztek_scene::ma_file::plugin_translators().into_iter().find(|x| x.name == *t).map(|x| x.extension));
                ext.is_some_and(|e| exts.contains(&e.trim_start_matches('.').to_ascii_lowercase()))
            });
            if reads_refs == want {
                keep.push(p);
            }
        }
        out = keep;
    }
    out
}

/// After loadPlugin and unloadPlugin: the -changedCommand scripts run, and saved files require what -writeRequires names.
pub(crate) fn registry_changed(name: &str, loaded: bool, user_named: bool) {
    if loaded && user_named {
        USER_NAMED.with(|u| u.borrow_mut().insert(name.to_string()));
    }
    if !loaded {
        USER_NAMED.with(|u| u.borrow_mut().remove(name));
    }
    if loaded {
        REMOVED.with(|r| r.borrow_mut().remove(name));
    }
    let wants = WRITE_REQUIRES.with(|w| w.borrow().contains(name));
    aztek_scene::scripted::set_write_requires(name, wants && loaded);
    for cb in CHANGED.with(|c| c.borrow().clone()) {
        crate::script_jobs::defer(json!(cb));
    }
}

fn pref_file() -> std::path::PathBuf {
    crate::user_folder::user_pref_dir().join("pluginPrefs.mel")
}

fn quote(t: &str) -> String {
    format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\""))
}

/// -savePluginPrefs: each plug-in's autoload setting, as lines -loadPluginPrefs reads back.
fn save_prefs(s: &mut Session) -> CmdResult {
    let rows = list_plugins(s);
    let mut text = String::from("// The plug-ins loaded at start, written by pluginInfo -savePluginPrefs.\n");
    for r in rows.as_array().into_iter().flatten() {
        let (Some(path), Some(auto)) = (r["path"].as_str(), r["autoload"].as_bool()) else { continue };
        if r["kind"] == "aztek" || r["kind"] == "python" {
            text.push_str(&format!("pluginInfo -edit -autoload {} {};\n", if auto { "true" } else { "false" }, quote(path)));
        }
    }
    let file = pref_file();
    if let Some(d) = file.parent() {
        std::fs::create_dir_all(d).map_err(|e| CommandError::new(format!("pluginInfo: {e}")))?;
    }
    std::fs::write(&file, text).map_err(|e| CommandError::new(format!("pluginInfo: {}: {e}", file.display())))?;
    Ok(Value::Null)
}

/// The quoted strings of a line, unescaped.
fn quoted_words(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur: Option<String> = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match (&mut cur, c) {
            (None, '"') => cur = Some(String::new()),
            (Some(w), '\\') => {
                if let Some(n) = chars.next() {
                    w.push(n);
                }
            }
            (Some(_), '"') => out.push(cur.take().unwrap()),
            (Some(w), c) => w.push(c),
            _ => {}
        }
    }
    out
}

/// -loadPluginPrefs: the autoload settings in pluginPrefs.mel, ours or the reference's autoLoadPlugin lines.
fn load_prefs(s: &mut Session) -> CmdResult {
    let file = pref_file();
    let Ok(text) = std::fs::read_to_string(&file) else { return Ok(Value::Null) };
    for line in text.lines().map(str::trim).filter(|l| !l.starts_with("//")) {
        if line.starts_with("pluginInfo") && line.contains("-autoload") {
            let on = !line.contains("-autoload false") && !line.contains("-autoload 0");
            if let Some(path) = quoted_words(line).last() {
                set_one_autoload(s, path, on)?;
            }
        } else if line.contains("autoLoadPlugin") {
            // autoLoadPlugin("", "name", "file"): the escapes are one level deeper inside evalDeferred.
            let inner = line.replace("\\\"", "\"");
            let words = quoted_words(&inner);
            if let Some(name) = words.iter().rev().find(|w| !w.is_empty() && !w.contains("autoLoadPlugin")) {
                let _ = set_one_autoload(s, name, true);
            }
        }
    }
    Ok(Value::Null)
}

fn set_one_autoload(s: &mut Session, name: &str, on: bool) -> CmdResult<()> {
    if crate::builtin_plugins::find(name).is_some() {
        return crate::builtin_plugins::set_autoload(name, on).map_err(|e| CommandError::new(format!("pluginInfo: {e}")));
    }
    let Some(path) = plugin_file(s, name) else {
        return Err(CommandError::new(format!("Plug-in \"{name}\" was not found on the plug-in path.")));
    };
    set_autoload(&path, on).map_err(|e| CommandError::new(format!("pluginInfo: {e}")))
}

/// Whether a flag is asked about: set, or named in a query.
fn asks(a: &Args, k: &str) -> bool {
    a.bool(k) || a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x.as_str() == Some(k)))
}

pub(super) fn plugin_info(s: &mut Session, a: &Args) -> CmdResult {
    let query = a.bool("query");
    let edit = a.bool("edit");
    let name = a.positional.first().and_then(Value::as_str).map(str::to_string);
    let need_name = || name.clone().ok_or_else(|| CommandError::new("pluginInfo: name a plug-in"));

    // What doesn't need a plug-in named.
    if let Some(cb) = a.get("changedCommand").and_then(Value::as_str) {
        CHANGED.with(|c| {
            let mut c = c.borrow_mut();
            if !c.iter().any(|x| x == cb) {
                c.push(cb.to_string());
            }
        });
        return Ok(Value::Null);
    }
    if a.bool("savePluginPrefs") {
        return save_prefs(s);
    }
    if a.bool("loadPluginPrefs") {
        return load_prefs(s);
    }
    if asks(a, "listPluginsPath") {
        let names = loaded_all(s);
        let mut out = Vec::new();
        for n in names {
            out.push(record(s, &n).map(|r| r.path).unwrap_or(n));
        }
        return Ok(json!(out));
    }
    if asks(a, "listPlugins") {
        return Ok(json!(loaded_all(s)));
    }
    if asks(a, "allEvaluators") || asks(a, "cacheFormat") {
        // Aztek's plug-ins can't register evaluators or geometry cache formats.
        return Ok(json!(Vec::<String>::new()));
    }
    if let Some(class) = a.get("dependNodeByType").and_then(Value::as_str) {
        let mut v: Vec<String> = all_plugin_types().into_iter().map(|(t, _)| t).filter(|t| class_matches(type_class(t), class)).collect();
        v.sort();
        v.dedup();
        return Ok(json!(v));
    }
    if let Some(ty) = a.get("pluginForNodeType").and_then(Value::as_str) {
        return Ok(json!(all_plugin_types().into_iter().find(|(t, _)| t == ty).map(|(_, o)| o).unwrap_or_default()));
    }
    if asks(a, "pluginsInUse") {
        let rt = a.get("referenceTranslators").filter(|v| !v.is_null()).map(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
        return Ok(json!(plugins_in_use(s, a.bool("activeFile"), rt)));
    }

    // Settings of one plug-in.
    if a.get("autoload").is_some() || (query && asks(a, "autoload")) {
        let name = need_name()?;
        if query {
            return Ok(json!(autoload_of(s, &name)));
        }
        set_one_autoload(s, &name, a.bool("autoload"))?;
        return Ok(Value::Null);
    }
    if a.get("writeRequires").is_some() || (query && asks(a, "writeRequires")) {
        let name = need_name()?;
        let key = record(s, &name).map(|r| r.name).unwrap_or_else(|| crate::builtin_plugins::find(&name).map_or_else(|| key_of(&name), |b| b.name.to_string()));
        if query {
            return Ok(json!(WRITE_REQUIRES.with(|w| w.borrow().contains(&key))));
        }
        if !is_registered(s, &name) {
            return Err(CommandError::new(format!("Plug-in \"{name}\" is not registered.")));
        }
        let on = a.bool("writeRequires");
        WRITE_REQUIRES.with(|w| {
            let mut w = w.borrow_mut();
            if on {
                w.insert(key.clone());
            } else {
                w.remove(&key);
            }
        });
        let loaded = is_loaded(s, &name);
        aztek_scene::scripted::set_write_requires(&key, on && loaded);
        return Ok(Value::Null);
    }
    if a.bool("remove") {
        let name = need_name()?;
        if is_loaded(s, &name) {
            return Err(CommandError::new(format!("pluginInfo: \"{name}\" is loaded; unload it before removing its record")));
        }
        if !is_registered(s, &name) {
            return Err(CommandError::new(format!("Plug-in \"{name}\" is not registered.")));
        }
        let key = crate::builtin_plugins::find(&name).map_or_else(|| key_of(&name), |b| b.name.to_string());
        REMOVED.with(|r| r.borrow_mut().insert(key));
        for cb in CHANGED.with(|c| c.borrow().clone()) {
            crate::script_jobs::defer(json!(cb));
        }
        return Ok(Value::Null);
    }
    let _ = edit;

    let name = need_name()?;
    if asks(a, "loaded") {
        return Ok(json!(is_loaded(s, &name)));
    }
    if asks(a, "registered") {
        return Ok(json!(is_registered(s, &name)));
    }
    if asks(a, "settings") {
        let (l, au, r) = (is_loaded(s, &name), autoload_of(s, &name), is_registered(s, &name));
        return Ok(json!([l as i64, au as i64, r as i64]));
    }
    if asks(a, "userNamed") {
        let key = record(s, &name).map(|r| r.name).unwrap_or_else(|| key_of(&name));
        return Ok(json!(USER_NAMED.with(|u| u.borrow().contains(&key))));
    }
    let Some(r) = record(s, &name) else {
        if asks(a, "unloadOk") {
            return Ok(json!(true));
        }
        return Err(CommandError::new(format!("Plug-in \"{name}\" is not loaded.")));
    };
    let empty = || Ok(json!(Vec::<String>::new()));
    if asks(a, "version") {
        return Ok(json!(r.version));
    }
    if asks(a, "vendor") {
        return Ok(json!(r.vendor));
    }
    if asks(a, "path") {
        return Ok(json!(r.path));
    }
    if asks(a, "name") {
        return Ok(json!(r.name));
    }
    if asks(a, "apiVersion") {
        return Ok(json!(r.api));
    }
    if asks(a, "command") {
        return Ok(json!(r.commands));
    }
    // Aztek's plug-ins register only normal commands: none of these kinds.
    if asks(a, "constraintCommand") || asks(a, "controlCommand") || asks(a, "modelEditorCommand") || asks(a, "tool") {
        return empty();
    }
    if asks(a, "data") {
        return Ok(json!(r.data));
    }
    if asks(a, "dependNode") {
        if a.bool("activeFile") {
            let only = active_nodes(s);
            let used = types_in_use(s, Some(&only));
            return Ok(json!(r.nodes.into_iter().filter(|n| used.contains(n)).collect::<Vec<_>>()));
        }
        return Ok(json!(r.nodes));
    }
    if asks(a, "dependNodeId") {
        return Ok(json!(r.nodes.iter().map(|n| aztek_scene::scripted::type_id(n).unwrap_or(0) as i64).collect::<Vec<_>>()));
    }
    if asks(a, "iksolver") {
        return Ok(json!(r.nodes.into_iter().filter(|n| aztek_scene::scripted::is_plugin_ik_solver(n)).collect::<Vec<_>>()));
    }
    if asks(a, "translator") {
        return Ok(json!(r.translators));
    }
    if asks(a, "unloadOk") {
        let used = types_in_use(s, None);
        return Ok(json!(!r.nodes.iter().any(|n| used.contains(n))));
    }
    if ["device", "dragAndDropBehavior", "evaluator", "renderer", "animCurveInterp", "serviceDescriptions"].iter().any(|k| asks(a, k)) {
        // None of these can be registered by Aztek's plug-ins.
        return empty();
    }
    Err(CommandError::new("pluginInfo: nothing asked"))
}
