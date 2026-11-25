//! Script UI commands that existing tools lean on, past what `scriptui` holds as flags:
//!
//! - nameField: a field showing an object's name; typing in it renames the object. Drawn as a textField, kept up to
//!   date with the object as it is renamed elsewhere.
//! - layoutDialog: a modal dialog whose inside a script builds, under a formLayout; `layoutDialog -dismiss` ends it
//!   with the string the call gives back.
//! - menuSet: the main window's menu sets, the reference's five and any a script makes, each with the menus scripts put
//!   in it. Making one of the five current switches the main window's menus.
//! - saveShelf and saveAllShelves: shelves written as the reference's `shelf_Name.mel` files.

use std::cell::RefCell;
use std::path::PathBuf;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Map, Value};

use crate::core::Core;
use crate::scriptui::{call_hook, callback_value, controls_of, model_edit, with_model};

type Cmd = CommandDef<Session>;

/// nameField's own flags, beside the ones every control has.
pub(crate) const NAME_FIELD_FLAGS: &str = "object:o:a nameChangeCommand:ncc:a changeCommand:cc:a receiveFocusCommand:rfc:a drawInactiveFrame:dif:b";

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Bool(b) => (*b as i32).to_string(),
        Value::Array(a) => a.first().map(text).unwrap_or_default(),
        o => o.to_string(),
    }
}

fn truth(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Value::String(s) => matches!(s.as_str(), "1" | "true" | "on" | "yes"),
        _ => false,
    }
}

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

/// The flag asked about in a query.
fn asked(a: &Args) -> Option<String> {
    a.get("queried").and_then(Value::as_array).and_then(|q| q.first()).and_then(Value::as_str).map(str::to_string).or_else(|| {
        a.flags.keys().find(|k| !matches!(k.as_str(), "query" | "queried" | "edit")).cloned()
    })
}

// ---------------------------------------------------------------------------------------------------------------------
// nameField

/// The nameFields there are: drawn as text fields, made by nameField.
fn name_fields() -> Vec<(String, Map<String, Value>)> {
    controls_of("textField").into_iter().filter(|(_, p)| p.get("__cmd").and_then(Value::as_str) == Some("nameField")).collect()
}

/// Keeps each nameField on its object. The node is found by name once, then followed by its id, so a rename made
/// anywhere shows; a new -object is found again.
fn refresh_name_fields(s: &mut Session) {
    for (path, props) in name_fields() {
        let object = props.get("object").map(text).unwrap_or_default();
        let mut edit = Map::new();
        let resolved_from = props.get("__nodeOf").map(text);
        let node = if resolved_from.as_deref() == Some(object.as_str()) {
            props.get("__node").and_then(Value::as_u64)
        } else {
            let id = if object.is_empty() { None } else { s.graph.find(&object).ok().flatten() };
            edit.insert("__node".into(), id.map_or(Value::Null, |i| json!(i)));
            edit.insert("__nodeOf".into(), json!(object));
            id
        };
        let shown = match node.filter(|id| s.graph.node(*id).is_some()) {
            Some(id) => {
                let full = s.graph.path(id);
                let short = full.rsplit('|').next().unwrap_or("").to_string();
                // Renamed elsewhere: -object follows, by its short name where that is unique.
                if s.graph.find(&object).ok().flatten() != Some(id) {
                    let now = if s.graph.find(&short).ok().flatten() == Some(id) { short.clone() } else { full };
                    edit.insert("object".into(), json!(now));
                    edit.insert("__nodeOf".into(), json!(now));
                }
                short
            }
            None => String::new(),
        };
        if props.get("text").map(text).as_deref() != Some(shown.as_str()) {
            edit.insert("text".into(), json!(shown));
        }
        if !edit.is_empty() {
            let _ = model_edit(&path, &edit);
        }
    }
}

/// The user typed a name into a nameField: the object takes it, then -nameChangeCommand runs.
fn name_field_changed(core: &mut Core, ev: &Value) {
    let Some(path) = ev["path"].as_str().and_then(crate::scriptui::control_full_path) else { return };
    let Some(props) = crate::scriptui::control_props(&path) else { return };
    if props.get("__cmd").and_then(Value::as_str) != Some("nameField") {
        return;
    }
    let fired: Vec<&str> = ev["fire"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
    if !fired.contains(&"changeCommand") {
        return;
    }
    let typed = props.get("text").map(text).unwrap_or_default();
    let object = props.get("object").map(text).unwrap_or_default();
    let node = props.get("__node").and_then(Value::as_u64);
    let current = match node {
        Some(id) if core.s.graph.node(id).is_some() => core.s.graph.path(id),
        _ => object.clone(),
    };
    let old_short = current.rsplit('|').next().unwrap_or("").to_string();
    if typed.is_empty() || current.is_empty() || typed == old_short {
        refresh_name_fields(&mut core.s);
        return;
    }
    match core.exec("rename", json!({"args": [current, typed]})) {
        Ok(v) => {
            let now = text(&v);
            let full = now;
            let _ = model_edit(&path, &Map::from_iter([("object".to_string(), json!(full.clone())), ("__nodeOf".to_string(), json!(full))]));
            if let Some(cb) = props.get("nameChangeCommand") {
                core.run_callback(cb, &[]);
            }
        }
        Err(_) => {}
    }
    refresh_name_fields(&mut core.s);
}

// ---------------------------------------------------------------------------------------------------------------------
// layoutDialog

struct OpenDialog {
    window: String,
    answer: Option<String>,
}

thread_local! {
    static DIALOGS: RefCell<Vec<OpenDialog>> = const { RefCell::new(Vec::new()) };
}

/// The answer a dialog gives when closed without -dismiss, and with no interface to show it.
const DISMISSED: &str = "dismiss";

fn layout_dialog(core: &mut Core, call: &Value) -> Result<Value, String> {
    let o = call.as_object().cloned().unwrap_or_default();
    if let Some(d) = o.get("dismiss").filter(|v| !v.is_null()) {
        let window = DIALOGS.with(|ds| {
            let mut ds = ds.borrow_mut();
            ds.last_mut().map(|top| {
                top.answer = Some(text(d));
                top.window.clone()
            })
        });
        let Some(window) = window else { return Err("layoutDialog: no layout dialog is open to dismiss.".into()) };
        call_hook("layoutDialogDismiss", &json!({"window": window}));
        return Ok(Value::Null);
    }
    let title = o.get("title").map(text).unwrap_or_default();
    let mut win_flags = json!({"title": title});
    if let Some(r) = o.get("resizable").filter(|v| !v.is_null()) {
        win_flags["sizeable"] = json!(truth(r));
    }
    if let Some(bg) = o.get("backgroundColor").filter(|v| !v.is_null()) {
        win_flags["backgroundColor"] = bg.clone();
    }
    let was_parent = with_model(|m| m.parent);
    let window = core.s.run("window", win_flags).map_err(|e| e.to_string())?;
    let window = text(&window);
    let form = core.s.run("formLayout", json!({})).map_err(|e| e.to_string());
    let form = match form {
        Ok(v) => text(&v),
        Err(e) => {
            let _ = core.s.run("deleteUI", json!({"args": [window], "window": true}));
            with_model(|m| m.parent = was_parent);
            return Err(e);
        }
    };
    let _ = core.s.run("setParent", json!({"args": [form]}));
    if let Some(ui) = o.get("uiScript").filter(|v| !v.is_null()) {
        core.run_callback(&callback_value(ui), &[]);
    }
    DIALOGS.with(|ds| ds.borrow_mut().push(OpenDialog { window: window.clone(), answer: None }));
    let mut payload = json!({"window": window, "title": title});
    if let Some(p) = o.get("parent").filter(|v| !v.is_null()) {
        payload["parent"] = json!(text(p));
    }
    let shown = call_hook("layoutDialog", &payload).is_some();
    let answer = DIALOGS.with(|ds| ds.borrow_mut().pop()).and_then(|d| d.answer);
    let _ = core.s.run("deleteUI", json!({"args": [window], "window": true}));
    with_model(|m| m.parent = was_parent.filter(|p| m.controls.contains_key(p)));
    // With no interface the dialog can't be answered: it comes back as closed.
    Ok(json!(if shown { answer.unwrap_or_else(|| DISMISSED.to_string()) } else { DISMISSED.to_string() }))
}

/// Commands that need the whole core: layoutDialog runs its script and waits.
pub(crate) fn intercept(core: &mut Core, name: &str, call: &Value) -> Option<Result<Value, String>> {
    match name {
        "layoutDialog" => Some(layout_dialog(core, call)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// menuSet

#[derive(Clone)]
struct MenuSet {
    name: String,
    label: String,
    menus: Vec<String>,
    permanent: bool,
    hotbox: bool,
}

struct MenuSets {
    sets: Vec<MenuSet>,
    current: String,
}

/// The reference's five, by name and label; their labels are what the main window's menu set box shows.
const BUILT_IN: &[(&str, &str)] = &[("modelingMenuSet", "Modeling"), ("riggingMenuSet", "Rigging"), ("animationMenuSet", "Animation"), ("fxMenuSet", "FX"), ("renderingMenuSet", "Rendering")];

thread_local! {
    static SETS: RefCell<Option<MenuSets>> = const { RefCell::new(None) };
}

fn with_sets<R>(f: impl FnOnce(&mut MenuSets) -> R) -> R {
    SETS.with(|s| {
        let mut s = s.borrow_mut();
        let sets = s.get_or_insert_with(|| MenuSets {
            sets: BUILT_IN.iter().map(|(n, l)| MenuSet { name: n.to_string(), label: l.to_string(), menus: Vec::new(), permanent: true, hotbox: true }).collect(),
            current: BUILT_IN[0].0.to_string(),
        });
        f(sets)
    })
}

/// A menu set by name, or by its label as the main window shows it (Dynamics is FX's older name).
fn find_set(sets: &MenuSets, name: &str) -> Option<usize> {
    let name = if name == "Dynamics" || name == "dynamicsMenuSet" { "fxMenuSet" } else { name };
    sets.sets.iter().position(|s| s.name == name).or_else(|| sets.sets.iter().position(|s| s.label == name))
}

/// A menu's full path, as the menus were made, or the name as given.
fn menu_path(name: &str) -> String {
    crate::scriptui::control_full_path(name).unwrap_or_else(|| name.to_string())
}

fn index_arg(v: &Value) -> Result<(String, usize), String> {
    let a = match v {
        Value::Array(a) if a.len() >= 2 => a.clone(),
        _ => return Err("give a name and an index.".into()),
    };
    let idx = a[1].as_f64().or_else(|| a[1].as_str().and_then(|s| s.trim().parse().ok())).ok_or("the index is not a number.")?;
    if idx < 0.0 {
        return Err("the index can't be negative.".into());
    }
    Ok((text(&a[0]), idx as usize))
}

fn menu_set(a: &Args) -> CmdResult {
    let target = a.pos_str(0).map(str::to_string);
    if a.bool("query") {
        let q = asked(a).unwrap_or_default();
        return with_sets(|ms| {
            let set = |ms: &MenuSets| -> Result<usize, aztek_ops::CommandError> {
                let n = target.clone().unwrap_or_else(|| ms.current.clone());
                find_set(ms, &n).ok_or_else(|| aztek_ops::CommandError::new(format!("menuSet: Object '{n}' not found.")))
            };
            Ok(match q.as_str() {
                "currentMenuSet" => json!(ms.current),
                "allMenuSets" => json!(ms.sets.iter().map(|s| s.name.clone()).collect::<Vec<_>>()),
                "numberOfMenuSets" => json!(ms.sets.len()),
                "label" => json!(ms.sets[set(ms)?].label),
                "menuArray" => json!(ms.sets[set(ms)?].menus),
                "numberOfMenus" => json!(ms.sets[set(ms)?].menus.len()),
                "permanent" => json!(ms.sets[set(ms)?].permanent),
                "hotBoxVisible" => json!(ms.sets[set(ms)?].hotbox),
                "exists" => json!(target.as_deref().or(a.get("exists").and_then(Value::as_str)).is_some_and(|n| find_set(ms, n).is_some())),
                o => fail!("menuSet: -{o} can't be queried."),
            })
        });
    }
    // -exists without -q, as the reference takes it: the name follows the flag.
    if let Some(n) = a.get("exists").filter(|v| v.is_string()).map(text).or_else(|| if a.bool("exists") { target.clone() } else { None }) {
        return Ok(json!(with_sets(|ms| find_set(ms, &n).is_some())));
    }
    if let Some(n) = a.get("removeMenuSet").filter(|v| !v.is_null()).map(text) {
        return with_sets(|ms| {
            let Some(i) = find_set(ms, &n) else { fail!("menuSet: Object '{n}' not found.") };
            if ms.sets[i].permanent {
                fail!("menuSet: {n} is permanent and can't be removed.");
            }
            let gone = ms.sets.remove(i);
            if ms.current == gone.name {
                ms.current = BUILT_IN[0].0.to_string();
            }
            Ok(Value::Null)
        });
    }
    if let Some(v) = a.get("moveMenuSet").filter(|v| !v.is_null()) {
        let (n, at) = index_arg(v).map_err(|e| aztek_ops::CommandError::new(format!("menuSet: -moveMenuSet: {e}")))?;
        return with_sets(|ms| {
            let Some(i) = find_set(ms, &n) else { fail!("menuSet: Object '{n}' not found.") };
            let s = ms.sets.remove(i);
            let at = at.min(ms.sets.len());
            ms.sets.insert(at, s);
            Ok(Value::Null)
        });
    }
    if let Some(n) = a.get("currentMenuSet").filter(|v| !v.is_null()).map(text) {
        let switch = with_sets(|ms| -> Result<Option<String>, aztek_ops::CommandError> {
            let Some(i) = find_set(ms, &n) else { fail!("menuSet: Object '{n}' not found.") };
            let name = ms.sets[i].name.clone();
            let changed = ms.current != name;
            ms.current = name.clone();
            Ok(BUILT_IN.iter().find(|(b, _)| *b == name).filter(|_| changed).map(|(_, l)| l.to_string()))
        })?;
        // The main window shows the reference's five; a set of a script's own is kept as current here.
        if let Some(label) = switch {
            let cmd = if label == "FX" { "MenuSetDynamics".to_string() } else { format!("MenuSet{label}") };
            crate::core::push_ui_request(cmd);
        }
        if !a.has("label") && !a.has("addMenu") && !a.has("insertMenu") && !a.has("removeMenu") && !a.has("moveMenu") && !a.has("menuArray") {
            return Ok(Value::Null);
        }
    }
    // Making a set: a new name. Otherwise the edits go to the named set, or the current one.
    let made = with_sets(|ms| -> Result<Option<String>, aztek_ops::CommandError> {
        let i = match &target {
            Some(n) => match find_set(ms, n) {
                Some(i) => {
                    if !a.bool("edit") && (a.has("label") && !a.has("addMenu")) && ms.sets[i].name == *n && !a.has("permanent") {
                        // Made again under a name already there: the reference says the name is taken.
                        fail!("menuSet: Object's name '{n}' is not unique.");
                    }
                    i
                }
                None => {
                    if a.bool("edit") {
                        fail!("menuSet: Object '{n}' not found.");
                    }
                    ms.sets.push(MenuSet { name: n.clone(), label: n.clone(), menus: Vec::new(), permanent: false, hotbox: true });
                    ms.sets.len() - 1
                }
            },
            None if a.has("label") && !a.bool("edit") && !a.has("addMenu") && !a.has("insertMenu") && !a.has("removeMenu") && !a.has("moveMenu") && !a.has("menuArray") => {
                // No name: one is made up, as the reference's menuSet1.
                let mut k = 1;
                while find_set(ms, &format!("menuSet{k}")).is_some() {
                    k += 1;
                }
                let n = format!("menuSet{k}");
                ms.sets.push(MenuSet { name: n.clone(), label: n.clone(), menus: Vec::new(), permanent: false, hotbox: true });
                ms.sets.len() - 1
            }
            None => find_set(ms, &ms.current.clone()).unwrap_or(0),
        };
        let set = &mut ms.sets[i];
        if let Some(l) = a.get("label").filter(|v| !v.is_null()) {
            set.label = text(l);
        }
        if a.has("permanent") {
            set.permanent = a.bool("permanent");
        }
        if a.has("hotBoxVisible") {
            set.hotbox = a.bool("hotBoxVisible");
        }
        if let Some(v) = a.get("menuArray").filter(|v| !v.is_null()) {
            set.menus = match v {
                Value::Array(l) => l.iter().map(|x| menu_path(&text(x))).collect(),
                o => vec![menu_path(&text(o))],
            };
        }
        for v in a.get("addMenu").filter(|v| !v.is_null()).into_iter() {
            let items: Vec<Value> = match v {
                Value::Array(l) => l.clone(),
                o => vec![o.clone()],
            };
            for m in items {
                let p = menu_path(&text(&m));
                if !set.menus.contains(&p) {
                    set.menus.push(p);
                }
            }
        }
        if let Some(v) = a.get("insertMenu").filter(|v| !v.is_null()) {
            let (m, at) = index_arg(v).map_err(|e| aztek_ops::CommandError::new(format!("menuSet: -insertMenu: {e}")))?;
            let p = menu_path(&m);
            set.menus.retain(|x| *x != p);
            let at = at.min(set.menus.len());
            set.menus.insert(at, p);
        }
        if let Some(v) = a.get("moveMenu").filter(|v| !v.is_null()) {
            let (m, at) = index_arg(v).map_err(|e| aztek_ops::CommandError::new(format!("menuSet: -moveMenu: {e}")))?;
            let p = menu_path(&m);
            let Some(from) = set.menus.iter().position(|x| *x == p) else { fail!("menuSet: -moveMenu: {m} is not in {}.", set.name) };
            let it = set.menus.remove(from);
            let at = at.min(set.menus.len());
            set.menus.insert(at, it);
        }
        if let Some(v) = a.get("removeMenu").filter(|v| !v.is_null()) {
            let p = menu_path(&text(v));
            let before = set.menus.len();
            set.menus.retain(|x| *x != p && x.rsplit('|').next() != Some(text(v).as_str()));
            if set.menus.len() == before {
                fail!("menuSet: -removeMenu: {} is not in {}.", text(v), set.name);
            }
        }
        Ok(if a.bool("edit") { None } else { Some(set.name.clone()) })
    })?;
    Ok(made.map_or(Value::Null, |n| json!(n)))
}

/// The main window switched its menus: the set it now shows becomes current, without asking it to switch again.
fn menu_set_shown(a: &Args) -> CmdResult {
    let label = a.pos_str(0).unwrap_or("").to_string();
    with_sets(|ms| {
        if let Some(i) = find_set(ms, &label) {
            ms.current = ms.sets[i].name.clone();
        }
    });
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// saveShelf, saveAllShelves

fn mel_quote(t: &str) -> String {
    format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n"))
}

/// The shelf file a path names: `.mel` added when it isn't there, as the reference does.
fn shelf_file(path: &str) -> PathBuf {
    if path.ends_with(".mel") {
        PathBuf::from(path)
    } else {
        PathBuf::from(format!("{path}.mel"))
    }
}

/// The flags of a shelfButton as saved, in the reference's order; what the button doesn't have is left out.
const SAVED_BUTTON_FLAGS: &[(&str, &str)] = &[
    ("enableCommandRepeat", "b"),
    ("enable", "b"),
    ("width", "n"),
    ("height", "n"),
    ("manage", "b"),
    ("visible", "b"),
    ("preventOverride", "b"),
    ("annotation", "s"),
    ("enableBackground", "b"),
    ("backgroundColor", "v"),
    ("highlightColor", "v"),
    ("align", "s"),
    ("label", "s"),
    ("labelOffset", "n"),
    ("rotation", "n"),
    ("flipX", "b"),
    ("flipY", "b"),
    ("useAlpha", "b"),
    ("font", "s"),
    ("imageOverlayLabel", "s"),
    ("overlayLabelColor", "v"),
    ("overlayLabelBackColor", "v"),
    ("image", "s"),
    ("image1", "s"),
    ("style", "s"),
    ("marginWidth", "n"),
    ("marginHeight", "n"),
    ("command", "s"),
    ("sourceType", "s"),
    ("commandRepeatable", "b"),
    ("flat", "b"),
];

fn button_mel(props: &Map<String, Value>) -> String {
    let mut out = String::from("    shelfButton\n");
    for (flag, kind) in SAVED_BUTTON_FLAGS {
        let Some(v) = props.get(*flag).filter(|v| !v.is_null()) else {
            // What every saved button carries, as the reference writes them.
            match *flag {
                "enable" | "manage" | "visible" => out.push_str(&format!("        -{flag} 1\n")),
                "sourceType" => out.push_str("        -sourceType \"mel\"\n"),
                "style" => out.push_str("        -style \"iconOnly\"\n"),
                _ => {}
            }
            continue;
        };
        // A Python callable can't be written down: its source isn't known.
        if *flag == "command" && v.is_object() {
            if let Some(src) = v.get("__pysrc__").and_then(Value::as_str) {
                out.push_str(&format!("        -command {}\n", mel_quote(src)));
            }
            continue;
        }
        let val = match *kind {
            "b" => (truth(v) as i32).to_string(),
            "n" => text(v),
            "v" => match v {
                Value::Array(a) => a.iter().map(text).collect::<Vec<_>>().join(" "),
                o => text(o),
            },
            _ => mel_quote(&text(v)),
        };
        out.push_str(&format!("        -{flag} {val}\n"));
    }
    out.push_str("    ;\n");
    out
}

/// A shelf of the script UI, as the reference's shelf file: one global proc named after the file, a shelfButton call
/// for each button.
fn shelf_mel(proc_name: &str, buttons: &[Map<String, Value>]) -> String {
    let mut out = format!("global proc {proc_name} () {{\n    global string $gBuffStr;\n    global string $gBuffStr0;\n    global string $gBuffStr1;\n\n\n");
    for b in buttons {
        out.push_str(&button_mel(b));
    }
    out.push_str("\n} \n");
    out
}

/// The shelfButtons of a shelfLayout in the script UI, in order; None when there is no such shelf.
fn shelf_buttons(shelf: &str) -> Option<Vec<Map<String, Value>>> {
    with_model(|m| {
        let id = m.find(shelf)?;
        let c = m.controls.get(&id)?;
        if c.kind != "shelfLayout" {
            return None;
        }
        Some(c.children.iter().filter_map(|k| m.controls.get(k)).filter(|k| k.kind == "shelfButton").map(|k| k.props.clone()).collect())
    })
}

fn write_shelf(shelf: &str, path: &str) -> Result<bool, String> {
    let file = shelf_file(path);
    let proc_name = file.file_stem().and_then(|s| s.to_str()).unwrap_or("shelf").to_string();
    match shelf_buttons(shelf) {
        Some(buttons) => {
            if let Some(dir) = file.parent().filter(|d| !d.as_os_str().is_empty()) {
                std::fs::create_dir_all(dir).map_err(|e| format!("saveShelf: {}: {e}", dir.display()))?;
            }
            std::fs::write(&file, shelf_mel(&proc_name, &buttons)).map_err(|e| format!("saveShelf: {}: {e}", file.display()))?;
            Ok(true)
        }
        // One of the main window's own shelves: the window writes it.
        None => {
            if call_hook("shelfExists", &json!({"shelf": shelf})).is_some_and(|v| truth(&v)) {
                crate::core::push_ui_request(format!("saveShelf|{}", json!({"shelf": shelf, "path": aztek_core::paths::to_utf8(&file)})));
                Ok(true)
            } else {
                Err(format!("saveShelf: Object '{shelf}' not found."))
            }
        }
    }
}

fn save_shelf(a: &Args) -> CmdResult {
    let (Some(shelf), Some(path)) = (a.pos_str(0), a.pos_str(1)) else { fail!("saveShelf: give the shelf and the file to write.") };
    write_shelf(shelf, path).map(|b| json!(b)).map_err(aztek_ops::CommandError::new)
}

/// The prefs folder's shelves, where the reference keeps shelf_Name.mel.
fn shelves_folder() -> PathBuf {
    crate::user_folder::prefs_base().join("prefs").join("shelves")
}

fn save_all_shelves(a: &Args) -> CmdResult {
    let Some(tabs) = a.pos_str(0) else { fail!("saveAllShelves: give the shelf tab layout.") };
    let shelves: Option<Vec<(String, String)>> = with_model(|m| {
        let id = m.find(tabs)?;
        let c = m.controls.get(&id)?;
        if !matches!(c.kind.as_str(), "tabLayout") || c.props.get("__cmd").and_then(Value::as_str) != Some("shelfTabLayout") && !c.children.iter().any(|k| m.controls.get(k).is_some_and(|x| x.kind == "shelfLayout")) {
            return None;
        }
        Some(c.children.iter().filter_map(|k| m.controls.get(k)).filter(|k| k.kind == "shelfLayout").map(|k| (k.path.clone(), k.name.clone())).collect())
    });
    match shelves {
        Some(list) => {
            let dir = shelves_folder();
            for (path, name) in list {
                let file = dir.join(format!("shelf_{name}"));
                write_shelf(&path, &aztek_core::paths::to_utf8(&file)).map_err(aztek_ops::CommandError::new)?;
            }
            Ok(json!(true))
        }
        // The main window's shelves: it keeps them written, and writes them now.
        None => {
            crate::core::push_ui_request("saveAllShelves|".to_string());
            Ok(json!(true))
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Hooks from the core

pub(crate) fn shell_changed(core: &mut Core, ev: &Value) {
    name_field_changed(core, ev);
}

pub(crate) fn idle(core: &mut Core) {
    refresh_name_fields(&mut core.s);
}

/// After a nameField is made or edited: it shows its object at once.
pub(crate) fn after_command(s: &mut Session, cmd: &str) {
    if cmd == "nameField" {
        refresh_name_fields(s);
    }
}

pub fn register(s: &mut Session) {
    s.register(
        Cmd::new(
            "layoutDialog",
            "A modal dialog whose inside -uiScript builds, under a formLayout it is given as the parent. layoutDialog \
             -dismiss ends it with a string, which the call gives back; closed any other way it gives \"dismiss\", as \
             it does with no interface to show it in.",
            |_s, _a| Ok(json!(DISMISSED)),
        )
        .undoable(false)
        .flag(f("title", "t", ArgType::String, "The window's title."))
        .flag(f("uiScript", "ui", ArgType::Any, "What builds the dialog's inside: MEL, or a Python callable."))
        .flag(f("dismiss", "dis", ArgType::String, "Ends the dialog that is open, which gives this string back."))
        .flag(f("parent", "p", ArgType::String, "The window the dialog belongs to."))
        .flag(f("backgroundColor", "bgc", ArgType::Any, "The dialog's background, red green blue 0 to 1."))
        .flag(f("resizable", "res", ArgType::Bool, "Whether it can be resized.")),
    );
    s.register(
        Cmd::new(
            "menuSet",
            "The main window's menu sets: the reference's five (modelingMenuSet, riggingMenuSet, animationMenuSet, fxMenuSet, \
             renderingMenuSet) and any made with a new name and -label. -currentMenuSet makes one current, which switches \
             the main window's menus for the five; -addMenu, -insertMenu, -moveMenu, -removeMenu and -menuArray hold \
             the menus scripts put in a set (the five's own menus are the window's and aren't listed).",
            |_s, a| menu_set(a),
        )
        .positional("name")
        .undoable(false)
        .flag(f("query", "q", ArgType::Bool, "Read."))
        .flag(f("edit", "e", ArgType::Bool, "Change."))
        .flag(f("queried", "", ArgType::Any, "What a query asks for."))
        .flag(f("addMenu", "am", ArgType::Any, "A menu to put at the end of the set."))
        .flag(f("allMenuSets", "ams", ArgType::Bool, "With -q: every menu set's name."))
        .flag(f("currentMenuSet", "cms", ArgType::String, "Makes a set current; with -q, the current one."))
        .flag(f("exists", "ex", ArgType::Any, "Whether a menu set of the name is there."))
        .flag(f("hotBoxVisible", "hbv", ArgType::Bool, "Whether the set shows in the hotbox."))
        .flag(FlagDef::new("insertMenu", "im", ArgType::Any, Value::Null, "A menu and the place to put it in the set.").multi(2))
        .flag(f("label", "l", ArgType::String, "The set's label."))
        .flag(f("menuArray", "ma", ArgType::Any, "The set's menus: given, they replace them; with -q, the list."))
        .flag(FlagDef::new("moveMenu", "mm", ArgType::Any, Value::Null, "A menu of the set and the place to move it to.").multi(2))
        .flag(FlagDef::new("moveMenuSet", "mms", ArgType::Any, Value::Null, "A set and the place to move it to among the sets.").multi(2))
        .flag(f("numberOfMenuSets", "nms", ArgType::Bool, "With -q: how many sets there are."))
        .flag(f("numberOfMenus", "nm", ArgType::Bool, "With -q: how many menus the set has."))
        .flag(f("permanent", "p", ArgType::Bool, "A set that can't be removed."))
        .flag(f("removeMenu", "rm", ArgType::String, "Takes a menu out of the set."))
        .flag(f("removeMenuSet", "rms", ArgType::String, "Removes a set made by a script.")),
    );
    s.register(
        Cmd::new("aztekMenuSetShown", "For the main window: the menu set it now shows, by label, made current.", |_s, a| menu_set_shown(a))
            .positional("label")
            .undoable(false),
    );
    s.register(
        Cmd::new(
            "saveShelf",
            "Writes a shelf as the reference's shelf file: the shelf's name, then the file (.mel added). A shelf a script \
             made is written from its buttons; one of the main window's by the window. Gives true once written.",
            |_s, a| save_shelf(a),
        )
        .positional("shelf")
        .positional("file")
        .undoable(false),
    );
    s.register(
        Cmd::new(
            "saveAllShelves",
            "Writes every shelf of a shelf tab layout to the prefs shelves folder as shelf_Name.mel. The main window's \
             shelves are kept written as they change; this writes them now.",
            |_s, a| save_all_shelves(a),
        )
        .positional("shelfTabLayout")
        .undoable(false),
    );
}
