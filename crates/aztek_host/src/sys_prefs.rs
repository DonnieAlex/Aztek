//! System commands about preferences, the Script Editor and the scene: savePrefs, savePrefObjects, resourceManager,
//! scriptEditorInfo (with what the history pane shows and writes to a file), sceneLint and scmh.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, CommandError, FlagDef};
use aztek_scene::plug::NodeId;
use aztek_scene::Session;
use serde_json::{json, Value};

use crate::core::LineKind;

fn f(n: &str, sh: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(n, sh, ty, Value::Null, doc)
}

fn queried(a: &Args) -> Vec<String> {
    a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or(v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|s| matches!(s, "true" | "on" | "yes" | "1")))
}

fn err(e: impl std::fmt::Display) -> CommandError {
    CommandError::new(e.to_string())
}

/// The first line of the preference files Aztek writes, so it knows to read them back at start.
pub const MARK: &str = "// Aztek preferences";

// ---------------------------------------------------------------------------------------------------------------------
// scriptEditorInfo

#[derive(Default)]
struct Editor {
    errors: bool,
    info: bool,
    results: bool,
    stack_window: bool,
    warnings: bool,
    write: bool,
    file: String,
}

thread_local! {
    static EDITOR: std::cell::RefCell<Editor> = std::cell::RefCell::new(Editor::default());
}

fn editor<R>(f: impl FnOnce(&mut Editor) -> R) -> R {
    EDITOR.with(|e| f(&mut e.borrow_mut()))
}

/// The name scriptEditorInfo answers with.
const COMMAND_WINDOW: &str = "CommandWindow";

/// Where a history file goes: a relative name is in the user folder.
fn history_path(name: &str) -> PathBuf {
    let name = if name.is_empty() { "scriptEditorHistory.txt" } else { name };
    let p = Path::new(name);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        crate::user_folder::applied().root.join(p)
    }
}

/// Whether the Script Editor leaves a line of this kind out (scriptEditorInfo -suppress...).
pub fn hides(kind: LineKind) -> bool {
    editor(|e| match kind {
        LineKind::Error => e.errors,
        LineKind::Warning => e.warnings,
        LineKind::Info => e.info,
        LineKind::Result => e.results,
        LineKind::Echo => false,
    })
}

/// Whether a stack trace goes to the output window rather than a window of its own.
pub fn stack_window_suppressed() -> bool {
    editor(|e| e.stack_window)
}

/// A line the history shows, also written to the history file while -writeHistory is on.
pub fn history_line(text: &str) {
    let path = editor(|e| e.write.then(|| history_path(&e.file)));
    if let Some(p) = path {
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
            let _ = writeln!(file, "{text}");
        }
    }
}

fn script_editor_info(_s: &mut Session, a: &Args) -> CmdResult {
    let q = queried(a);
    let bools: [(&str, fn(&mut Editor) -> &mut bool); 6] = [
        ("suppressErrors", |e| &mut e.errors),
        ("suppressInfo", |e| &mut e.info),
        ("suppressResults", |e| &mut e.results),
        ("suppressStackWindow", |e| &mut e.stack_window),
        ("suppressWarnings", |e| &mut e.warnings),
        ("writeHistory", |e| &mut e.write),
    ];
    if a.bool("query") {
        let wants = |n: &str| q.iter().any(|x| x == n) || a.get(n).and_then(Value::as_bool) == Some(true);
        if wants("historyFilename") {
            return Ok(json!(editor(|e| history_path(&e.file)).to_string_lossy().replace('\\', "/")));
        }
        for (n, field) in bools {
            if wants(n) {
                return Ok(json!(editor(|e| *field(e))));
            }
        }
        fail!("scriptEditorInfo: name what to query");
    }
    if let Some(name) = a.get("historyFilename").and_then(Value::as_str) {
        editor(|e| e.file = name.to_string());
    }
    for (n, field) in bools {
        if let Some(v) = a.get(n).filter(|v| !v.is_null()) {
            let b = on(v);
            editor(|e| *field(e) = b);
        }
    }
    if a.bool("clearHistoryFile") {
        let p = editor(|e| e.write.then(|| history_path(&e.file)));
        if let Some(p) = p {
            std::fs::write(&p, "").map_err(|e| err(format!("scriptEditorInfo: {}: {e}", p.display())))?;
        }
    }
    if a.bool("clearHistory") {
        crate::core::push_ui_request("scriptEditorClear".into());
    }
    if let Some(text) = a.get("input").and_then(Value::as_str) {
        crate::core::push_ui_request(format!("scriptEditorInput|{text}"));
    }
    Ok(json!(COMMAND_WINDOW))
}

// ---------------------------------------------------------------------------------------------------------------------
// savePrefs

fn mel_quote(t: &str) -> String {
    format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n"))
}

/// One optionVar as the statement that sets it; arrays set the first value and append the rest.
fn option_var_line(name: &str, v: &Value) -> Option<String> {
    let one = |x: &Value, append: bool| -> Option<String> {
        let (flag, val) = match x {
            Value::Bool(b) => ("i", (*b as i32).to_string()),
            Value::Number(n) if n.is_i64() || n.is_u64() => ("i", n.to_string()),
            Value::Number(n) => ("f", n.as_f64().unwrap_or(0.0).to_string()),
            Value::String(s) => ("s", mel_quote(s)),
            _ => return None,
        };
        Some(format!("-{flag}v{} {} {val}", if append { "a" } else { "" }, mel_quote(name)))
    };
    match v {
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().enumerate().filter_map(|(i, x)| one(x, i > 0)).collect();
            (!parts.is_empty()).then(|| format!("optionVar {};", parts.join(" ")))
        }
        x => one(x, false).map(|p| format!("optionVar {p};")),
    }
}

fn write_pref_file(path: &Path, lines: &[String]) -> CmdResult<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| err(format!("savePrefs: {}: {e}", d.display())))?;
    }
    let mut text = format!("{MARK}, written by savePrefs.\n");
    for l in lines {
        text.push_str(l);
        text.push('\n');
    }
    std::fs::write(path, text).map_err(|e| err(format!("savePrefs: {}: {e}", path.display())))
}

/// The saved window sizes and places, as windowPrefs.mel: for savePrefs and windowPref -saveAll.
pub(crate) fn write_window_prefs(path: &Path) -> CmdResult<()> {
    write_pref_file(path, &crate::scriptui::window_pref_lines())
}

/// The optionVars that are kept between sessions, without those a category saves to a file of its own.
fn kept_vars(s: &Session, file: Option<&str>) -> BTreeMap<String, Value> {
    let meta = &s.extras.option_var_meta;
    s.option_vars
        .iter()
        .filter(|(k, v)| meta.saved(k) && v.as_array().is_none_or(|a| !a.is_empty()))
        .filter(|(k, _)| {
            let own = meta.category.get(*k).and_then(|c| meta.pref_file.get(c));
            match file {
                Some(f) => own.is_some_and(|o| o == f),
                None => own.is_none(),
            }
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

fn var_lines(vars: &BTreeMap<String, Value>) -> Vec<String> {
    vars.iter().filter_map(|(k, v)| option_var_line(k, v)).collect()
}

fn save_prefs(s: &mut Session, a: &Args) -> CmdResult {
    let dir = crate::user_folder::user_pref_dir();
    let kinds = ["colors", "general", "hotkeys", "menuSets", "plugins", "uiLayout"];
    let file = a.get("file").and_then(Value::as_str).map(str::to_string);
    let picked: Vec<&str> = kinds.iter().copied().filter(|k| a.bool(k)).collect();
    // With no flags, everything is saved.
    let all = picked.is_empty() && file.is_none();
    let want = |k: &str| all || picked.contains(&k);
    if let Some(name) = &file {
        let vars = kept_vars(s, Some(name));
        if vars.is_empty() && !s.extras.option_var_meta.pref_file.values().any(|f| f == name) {
            fail!("savePrefs: no preferences are saved to {name} (optionVar -category -prefFile names the file)");
        }
        write_pref_file(&dir.join(name), &var_lines(&vars))?;
    }
    if want("general") {
        let vars = kept_vars(s, None);
        crate::ui_cmds::write_option_var_map(&vars).map_err(|e| err(format!("savePrefs: {e}")))?;
        let general: BTreeMap<String, Value> = vars.into_iter().filter(|(k, _)| !is_menu_set_var(k)).collect();
        write_pref_file(&dir.join("userPrefs.mel"), &var_lines(&general))?;
    }
    if want("colors") {
        let lines: Vec<String> = aztek_scene::draw_style::display_rgb_all()
            .into_iter()
            .map(|(n, c)| format!("displayRGBColor -create {} {} {} {} {};", mel_quote(&n), c[0], c[1], c[2], c[3]))
            .collect();
        write_pref_file(&dir.join("userRGBColors2.mel"), &lines)?;
    }
    if want("hotkeys") {
        let (set, text) = crate::ui_cmds::current_hotkey_set_file();
        let path = dir.join("hotkeys").join(format!("{set}.mhk"));
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d).map_err(|e| err(format!("savePrefs: {}: {e}", d.display())))?;
        }
        std::fs::write(&path, text).map_err(|e| err(format!("savePrefs: {}: {e}", path.display())))?;
    }
    if want("menuSets") {
        let vars: BTreeMap<String, Value> = kept_vars(s, None).into_iter().filter(|(k, _)| is_menu_set_var(k)).collect();
        write_pref_file(&dir.join("menuSetPrefs.mel"), &var_lines(&vars))?;
    }
    if want("plugins") {
        crate::native_plugin::info::save_prefs(s)?;
    }
    if want("uiLayout") {
        write_window_prefs(&dir.join("windowPrefs.mel"))?;
    }
    Ok(Value::Null)
}

fn is_menu_set_var(name: &str) -> bool {
    let l = name.to_ascii_lowercase();
    l.contains("menuset") || l.contains("menumode")
}

/// The preference files savePrefs writes that are run at start: colours and window places, when Aztek wrote them.
pub fn startup_pref_files() -> Vec<PathBuf> {
    let dir = crate::user_folder::user_pref_dir();
    ["userRGBColors2.mel", "windowPrefs.mel"]
        .iter()
        .map(|n| dir.join(n))
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| t.starts_with(MARK)))
        .collect()
}

// ---------------------------------------------------------------------------------------------------------------------
// savePrefObjects

thread_local! {
    /// The preference nodes: those loadPrefObjects read, by name.
    static PREF_OBJECTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// loadPrefObjects read these nodes: they are the preference nodes savePrefObjects writes.
pub fn note_pref_objects(names: Vec<String>) {
    PREF_OBJECTS.with(|p| {
        let mut p = p.borrow_mut();
        for n in names {
            if !p.contains(&n) {
                p.push(n);
            }
        }
    });
}

fn save_pref_objects(s: &mut Session, _a: &Args) -> CmdResult {
    let file = crate::user_folder::user_pref_dir().join("userPrefObjects.ma");
    if let Some(d) = file.parent() {
        std::fs::create_dir_all(d).map_err(|e| err(format!("savePrefObjects: {e}")))?;
    }
    let names: Vec<String> = PREF_OBJECTS.with(|p| p.borrow().clone()).into_iter().filter(|n| matches!(s.graph.find(n), Ok(Some(_)))).collect();
    PREF_OBJECTS.with(|p| *p.borrow_mut() = names.clone());
    let path = file.to_string_lossy().replace('\\', "/");
    if names.is_empty() {
        let v = aztek_core::REFERENCE_VERSION;
        std::fs::write(&file, format!("//Maya ASCII {v} scene\n//Name: userPrefObjects.ma\nrequires maya \"{v}\";\n// End of userPrefObjects.ma\n")).map_err(|e| err(format!("savePrefObjects: {e}")))?;
        return Ok(json!(true));
    }
    let saved: Vec<NodeId> = s.graph.selection().to_vec();
    s.run("select", json!({"args": names, "replace": true, "noExpand": true}))?;
    let r = s.run("file", json!({"args": [path], "exportSelected": true, "type": "mayaAscii", "force": true}));
    s.graph.select(&saved, aztek_scene::graph::SelectMode::Replace);
    r?;
    Ok(json!(true))
}

// ---------------------------------------------------------------------------------------------------------------------
// resourceManager

/// Where the application's images are: its own icons, then each folder on the icon path.
fn resource_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let exe = aztek_core::paths::executable_dir();
    for d in [exe.join("icons"), exe.join("../share/aztek/icons"), Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/icons")] {
        if d.is_dir() {
            dirs.push(d);
        }
    }
    for d in aztek_core::env::icon_dirs() {
        if d.is_dir() && !dirs.contains(&d) {
            dirs.push(d);
        }
    }
    dirs
}

fn is_image(p: &Path) -> bool {
    p.extension().and_then(|e| e.to_str()).is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "png" | "svg" | "xpm" | "bmp" | "jpg" | "jpeg" | "ico" | "gif" | "tif" | "tiff"))
}

/// Every resource by name, the first found of each.
fn resources() -> Vec<(String, PathBuf)> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    for d in resource_dirs() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.is_file() && is_image(p)).collect();
        files.sort();
        for p in files {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if !out.iter().any(|(n, _)| *n == name) {
                out.push((name, p));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn resource_manager(_s: &mut Session, a: &Args) -> CmdResult {
    if let Some(v) = a.get("saveAs").and_then(Value::as_array) {
        let words: Vec<String> = v.iter().map(|x| x.as_str().map(str::to_string).unwrap_or_else(|| x.to_string())).collect();
        let [res, to] = words.as_slice() else { fail!("resourceManager: -saveAs takes the resource and the file to save it as") };
        let name = res.trim_start_matches(":/").to_string();
        let Some((_, from)) = resources().into_iter().find(|(n, _)| *n == name) else { fail!("resourceManager: no resource named {res}") };
        std::fs::copy(&from, to).map_err(|e| err(format!("resourceManager: {to}: {e}")))?;
        return Ok(Value::Null);
    }
    let pat = a.get("nameFilter").and_then(Value::as_str).unwrap_or("*").to_string();
    let names: Vec<String> = resources().into_iter().map(|(n, _)| n).filter(|n| crate::scriptui::wildcard_match(&pat, n)).collect();
    Ok(json!(names))
}

// ---------------------------------------------------------------------------------------------------------------------
// sceneLint

/// The checks: name, what it finds, and what fixing it gains.
const ISSUES: &[(&str, &str, &str)] = &[
    ("EMPTY_TRANSFORMS", "Transforms with no children, no shape and no connections.", "Less clutter, and fewer nodes to evaluate and draw."),
    ("UNKNOWN_NODES", "Nodes of a type no loaded plug-in defines, kept only to be saved again.", "Smaller files and no warnings on load."),
    ("UNUSED_ANIM_CURVES", "Animation curves that drive nothing.", "Less memory and smaller files."),
    ("UNUSED_SHADERS", "Materials assigned through no shading group.", "Less clutter and smaller files."),
];

const DELETE_CODE: &str = "import maya.cmds as cmds\nfor obj in OBJECTS:\n    if cmds.objExists(obj):\n        cmds.delete(obj)\n";

fn is_a(t: &aztek_scene::node::NodeType, name: &str) -> bool {
    let mut cur = Some(t);
    while let Some(x) = cur {
        if x.name == name {
            return true;
        }
        cur = x.base.as_deref();
    }
    false
}

fn find_issue(s: &Session, code: &str) -> Vec<String> {
    let g = &s.graph;
    let conns = g.connections();
    let has_out = |id: NodeId| conns.iter().any(|(a, _)| a.node == id);
    let connected = |id: NodeId| conns.iter().any(|(a, b)| a.node == id || b.node == id);
    let mut out = Vec::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if n.reference_id() != 0 || aztek_scene::defaults::undeletable(g, id) {
            continue;
        }
        let t = n.ty();
        let hit = match code {
            "EMPTY_TRANSFORMS" => t.name == "transform" && g.children(id).is_empty() && !connected(id),
            "UNKNOWN_NODES" => matches!(t.name.as_str(), "unknown" | "unknownDag" | "unknownTransform"),
            "UNUSED_ANIM_CURVES" => is_a(t, "animCurve") && !has_out(id),
            "UNUSED_SHADERS" => aztek_scene::shading::is_material_type(t) && !conns.iter().any(|(a, b)| a.node == id && g.node(b.node).is_some_and(|x| is_a(x.ty(), "shadingEngine"))),
            _ => false,
        };
        if hit {
            out.push(if t.dag { g.full_path(id) } else { g.path(id) });
        }
    }
    out
}

fn scene_lint(s: &mut Session, a: &Args) -> CmdResult {
    let asked: Vec<String> = match a.get("issueType") {
        Some(Value::Array(v)) => v.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        Some(Value::String(t)) => vec![t.clone()],
        _ => Vec::new(),
    };
    for i in &asked {
        if !ISSUES.iter().any(|x| x.0 == i) {
            fail!("sceneLint: no issue type {i}; sceneLint -q lists them");
        }
    }
    let picked: Vec<&(&str, &str, &str)> = ISSUES.iter().filter(|x| asked.is_empty() || asked.iter().any(|i| i == x.0)).collect();
    if a.bool("query") {
        let verbose = a.bool("verbose") || queried(a).iter().any(|x| x == "verbose");
        let mut v = Vec::new();
        for x in picked {
            v.push(x.0.to_string());
            if verbose || !asked.is_empty() {
                v.push(x.1.to_string());
            }
        }
        return Ok(json!(v));
    }
    let mut found = serde_json::Map::new();
    for x in picked {
        let objects = find_issue(s, x.0);
        if objects.is_empty() {
            continue;
        }
        found.insert(
            x.0.to_string(),
            json!({"description": x.1, "mitigation": [{"objects": objects, "benefit": x.2, "description": "Delete them.", "code": DELETE_CODE}]}),
        );
    }
    Ok(json!(serde_json::to_string_pretty(&json!({"sceneLint": found})).unwrap_or_default()))
}

// ---------------------------------------------------------------------------------------------------------------------
// scmh

fn scmh_run(s: &mut Session, a: &Args) -> CmdResult {
    let vals: Vec<f64> = a.positional.iter().filter_map(|v| v.as_f64().or_else(|| v.as_str().and_then(|t| t.parse().ok()))).collect();
    if vals.is_empty() {
        fail!("scmh: give the handle's values");
    }
    let ignore: Vec<usize> = match a.get("ignore") {
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_f64()).map(|x| x as usize).collect(),
        Some(v) => v.as_f64().map(|x| vec![x as usize]).unwrap_or_default(),
        None => Vec::new(),
    };
    let relative = a.bool("relative") && !a.bool("absolute");
    let ctx = crate::ctx_cmds::current();
    let kind = crate::ctx_cmds::kind_of(&ctx).unwrap_or_default();
    let (tool, attr) = match kind.as_str() {
        "manipMoveContext" | "moveSuperContext" => (crate::viewport::Tool::Move, "translate"),
        "manipRotateContext" | "RotateSuperContext" => (crate::viewport::Tool::Rotate, "rotate"),
        "manipScaleContext" | "scaleSuperContext" => (crate::viewport::Tool::Scale, "scale"),
        _ => fail!("scmh: the current tool ({ctx}) has no manipulator handle"),
    };
    let handle = crate::viewport::active_handle(tool);
    // The axes the handle drives: one axis, the two of a plane, or all three.
    let axes: Vec<usize> = match handle {
        Some(crate::viewport::Handle::Axis(i)) => vec![i],
        Some(crate::viewport::Handle::Plane(n)) => (0..3).filter(|&i| i != n).collect(),
        _ => vec![0, 1, 2],
    };
    // One value on the centre of the scale manipulator scales evenly.
    let vals: Vec<f64> = if axes.len() == 3 && vals.len() == 1 && attr == "scale" { vec![vals[0]; 3] } else { vals };
    // The values are in the interface's units; the scene keeps centimetres and radians.
    let unit = match attr {
        "translate" => aztek_ops::units::cm_per_unit(),
        "rotate" => aztek_ops::units::rad_per_unit(),
        _ => 1.0,
    };
    let vals: Vec<f64> = vals.into_iter().map(|v| v * unit).collect();
    let mut set: Vec<(usize, f64)> = Vec::new();
    for (k, &axis) in axes.iter().enumerate() {
        if ignore.contains(&(k + 1)) {
            continue;
        }
        if let Some(&v) = vals.get(k) {
            set.push((axis, v));
        }
    }
    let objects: Vec<String> = s.graph.selection().iter().filter(|&&id| s.graph.node(id).is_some_and(|n| n.ty().dag)).map(|&id| s.graph.full_path(id)).collect();
    if objects.is_empty() {
        fail!("scmh: nothing is selected for the manipulator to act on");
    }
    let names = ["X", "Y", "Z"];
    for o in &objects {
        if attr == "translate" && !relative {
            // Where the object is in world space, with the handle's axes set.
            let mut p: Vec<f64> = s.run("xform", json!({"args": [o], "query": true, "worldSpace": true, "translation": true}))?.as_array().map(|v| v.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
            p.resize(3, 0.0);
            for &(axis, v) in &set {
                p[axis] = v;
            }
            s.run("xform", json!({"args": [o], "worldSpace": true, "translation": p}))?;
            continue;
        }
        if attr == "translate" {
            let mut d = [0.0; 3];
            for &(axis, v) in &set {
                d[axis] = v;
            }
            s.run("move", json!({"args": [o], "by": d}))?;
            continue;
        }
        for &(axis, v) in &set {
            let plug = format!("{o}.{attr}{}", names[axis]);
            let now = s.run("getAttr", json!({"args": [plug.clone()]}))?.as_f64().unwrap_or(if attr == "scale" { 1.0 } else { 0.0 });
            let to = match (relative, attr) {
                (false, _) => v,
                (true, "scale") => now * v / unit,
                (true, _) => now + v,
            };
            s.run("setAttr", json!({"args": [plug, to]}))?;
        }
    }
    Ok(Value::Null)
}

fn scmh(s: &mut Session, a: &Args) -> CmdResult {
    match scmh_run(s, a) {
        Err(_) if a.bool("quiet") => Ok(Value::Null),
        r => r,
    }
}

pub fn register(s: &mut Session) {
    let b = ArgType::Bool;
    s.register(
        CommandDef::new("aztekPreloadRefEdRows", "For the Preload Reference Editor: the load settings as rows (id, parent, node, file, unloaded).", |_s, _a| Ok(aztek_scene::file_more::load_settings_rows()))
            .undoable(false),
    );
    s.register(
        CommandDef::new("aztekProfilerView", "For the profiler panel: the recorded events and the view's state; -select picks events by index.", |_s, a| {
            if let Some(v) = a.get("select") {
                let idx: Vec<usize> = match v {
                    Value::Array(x) => x.iter().filter_map(Value::as_u64).map(|i| i as usize).collect(),
                    x => x.as_u64().map(|i| vec![i as usize]).unwrap_or_default(),
                };
                crate::profiler::select_events(idx);
                return Ok(Value::Null);
            }
            Ok(crate::profiler::view_json())
        })
        .undoable(false)
        .flag(f("select", "", ArgType::Any, "The events picked.")),
    );
    s.register(
        CommandDef::new(
            "scriptEditorInfo",
            "The Script Editor's history: which kinds of line it leaves out, the file its lines are written to, and clearing it. Returns the Command Window's name.",
            script_editor_info,
        )
        .flag(f("query", "q", b, "Query mode."))
        .flag(f("edit", "e", b, "Edit mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("clearHistory", "ch", b, "Clear the history pane."))
        .flag(f("clearHistoryFile", "chf", b, "Empty the history file, while -writeHistory is on; it is otherwise added to."))
        .flag(f("historyFilename", "hfn", ArgType::String, "The file the history is written to; a relative name is in the user folder. Empty: scriptEditorHistory.txt there."))
        .flag(f("input", "i", ArgType::String, "Set the text of the input pane; empty clears it."))
        .flag(f("suppressErrors", "se", b, "Leave error messages out of the history."))
        .flag(f("suppressInfo", "si", b, "Leave information messages out of the history."))
        .flag(f("suppressResults", "sr", b, "Leave command results out of the history."))
        .flag(f("suppressStackWindow", "ssw", b, "With stack traces on, show them in the output window rather than a window of their own."))
        .flag(f("suppressWarnings", "sw", b, "Leave warnings out of the history."))
        .flag(f("writeHistory", "wh", b, "Write the history's lines to the history file as well.")),
    );
    s.register(
        CommandDef::new(
            "savePrefs",
            "Save preferences to the preferences folder. With no flags, all of them: optionVars (userPrefs.mel), colours (userRGBColors2.mel), hotkeys (hotkeys/<set>.mhk), menu sets (menuSetPrefs.mel), plug-ins (pluginPrefs.mel) and window sizes and places (windowPrefs.mel).",
            save_prefs,
        )
        .flag(f("colors", "c", b, "Save the colour preferences."))
        .flag(f("file", "f", ArgType::String, "Save the optionVars whose category optionVar -prefFile sends to this file."))
        .flag(f("general", "g", b, "Save the general preferences (optionVars)."))
        .flag(f("hotkeys", "hk", b, "Save the hotkeys."))
        .flag(f("menuSets", "ms", b, "Save the menu set preferences."))
        .flag(f("plugins", "pl", b, "Save the plug-in preferences."))
        .flag(f("uiLayout", "ui", b, "Save each window's size and place.")),
    );
    s.register(CommandDef::new("savePrefObjects", "Save the preference nodes to userPrefObjects.ma in the preferences folder. Returns true when it is written.", save_pref_objects));
    s.register(
        CommandDef::new("resourceManager", "List the application's resources (its images and those on the icon path) by name, or save a copy of one as a file.", resource_manager)
            .undoable(false)
            .flag(f("nameFilter", "nf", ArgType::String, "Only the resources whose names match; ? and * are wildcards."))
            .flag(f("saveAs", "s", ArgType::Any, "Save a copy of the resource (first) as the file (second).").multi(2)),
    );
    s.register(
        CommandDef::new(
            "sceneLint",
            "Look through the scene for what costs speed, memory or clutter. Returns JSON: {\"sceneLint\": {issue: {description, mitigation: [{objects, benefit, description, code}]}}}; each mitigation's Python runs with exec(code, {}, {'OBJECTS': objects}). Query: the issue types.",
            scene_lint,
        )
        .undoable(false)
        .flag(f("query", "q", b, "Query mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("issueType", "i", ArgType::Any, "Check only this issue type; query: its name and description.").multi(1))
        .flag(f("verbose", "v", b, "Query: each issue type's description after its name.")),
    );
    s.register(
        CommandDef::new("scmh", "Set the current manipulator handle's values: the active handle of the move, rotate or scale tool, on the selected objects, in the current units.", scmh)
            .positional("values")
            .flag(f("absolute", "a", b, "The values are what the handle is set to (the default)."))
            .flag(f("ignore", "i", ArgType::Any, "Leave the value of this entry (from 1) as it is.").multi(1))
            .flag(f("quiet", "q", b, "Say nothing when it fails."))
            .flag(f("relative", "r", b, "The values are added to the handle's (multiplied, for scale).")),
    );
}

/// Where loadPrefObjects' nodes are noted, so savePrefObjects writes them back.
pub(crate) fn load_pref_objects(s: &mut Session) -> CmdResult {
    let before: std::collections::HashSet<NodeId> = s.graph.nodes().into_iter().collect();
    let file = crate::user_folder::user_pref_dir().join("userPrefObjects.ma");
    if !file.is_file() {
        return Ok(json!(false));
    }
    let path = file.to_string_lossy().replace('\\', "/");
    s.run("file", json!({"args": [path], "import": true}))?;
    let new: Vec<String> = s.graph.nodes().into_iter().filter(|id| !before.contains(id)).filter_map(|id| s.graph.node(id).map(|n| (n.ty().dag, id))).map(|(dag, id)| if dag { s.graph.full_path(id) } else { s.graph.path(id) }).collect();
    note_pref_objects(new);
    Ok(json!(true))
}
