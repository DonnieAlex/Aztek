//! Menus built from the scene: attributeMenu (the connection menus of the node editors), buildBookmarkMenu (the sets of
//! one bookmark type) and buildKeyframeMenu (what can be keyed on the selection).

use std::sync::Mutex;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Map, Value};

use crate::scriptui::{control_full_path, control_kind, create_as, current_menu, model_edit, set_from_python};

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        o => o.to_string(),
    }
}

fn strings(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().flat_map(strings).collect(),
        Value::Null => Vec::new(),
        o => vec![text(o)],
    }
}

fn quote(t: &str) -> String {
    format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\""))
}

/// A menu by name, as a full path: one of the menu kinds.
fn menu_path(name: &str, cmd: &str) -> Result<String, aztek_ops::CommandError> {
    match control_kind(name).as_deref() {
        Some("menu" | "popupMenu" | "optionMenu" | "optionMenuGrp" | "menuItem") => Ok(control_full_path(name).unwrap_or_else(|| name.to_string())),
        Some(_) => Err(aztek_ops::CommandError::new(format!("{cmd}: '{name}' is not a menu."))),
        None => Err(aztek_ops::CommandError::new(format!("{cmd}: Object '{name}' not found."))),
    }
}

/// Adds an item to a menu: its label and MEL command, or a divider when the label is empty.
fn add_item(menu: &str, label: &str, command: Option<String>) {
    let mut flags = Map::new();
    flags.insert("parent".into(), json!(menu));
    if label.is_empty() {
        flags.insert("divider".into(), json!(true));
    } else {
        flags.insert("label".into(), json!(label));
    }
    if let Some(c) = command {
        flags.insert("command".into(), json!(c));
    }
    // The commands are MEL whoever builds the menu.
    let saved = set_from_python(false);
    let _ = create_as("menuItem", "menuItem", &Args { positional: Vec::new(), flags });
    set_from_python(saved);
}

fn clear_menu(menu: &str) {
    let _ = model_edit(menu, &Map::from_iter([("deleteAllItems".to_string(), json!(true))]));
}

// ---------------------------------------------------------------------------------------------------------------------
// attributeMenu

/// -regPulldownMenuCommand's callbacks, by the id each was given.
static PULLDOWNS: Mutex<Vec<(i64, String)>> = Mutex::new(Vec::new());
static NEXT_PULLDOWN: Mutex<i64> = Mutex::new(1);

/// The kind of connection an attribute takes: numbers go with numbers, three with three, the rest with their own type.
fn connection_kind(at: &str) -> String {
    match at {
        "bool" | "long" | "short" | "byte" | "char" | "enum" | "float" | "double" | "doubleLinear" | "doubleAngle" | "time" | "int" => "number".to_string(),
        "double3" | "float3" | "long3" | "short3" => "triple".to_string(),
        "double2" | "float2" | "long2" | "short2" => "pair".to_string(),
        o => o.to_string(),
    }
}

fn attr_type(s: &mut Session, node: &str, attr: &str) -> String {
    s.run("attributeQuery", json!({"args": [attr], "node": node, "attributeType": true})).map(|v| text(&v)).unwrap_or_default()
}

/// The attributes a connection menu lists: the node's inputs (writable) or outputs (readable) that can connect, and with
/// a plug at the other end, only those of a kind that takes it.
fn connection_attrs(s: &mut Session, node: &str, inputs: bool, other: Option<&str>) -> Vec<String> {
    let flag = if inputs { "write" } else { "read" };
    let attrs = s.run("listAttr", json!({"args": [node], flag: true, "connectable": true})).map(|v| strings(&v)).unwrap_or_default();
    let want = other.and_then(|p| p.rsplit_once('.')).map(|(n, a)| {
        let (n, a) = (n.to_string(), a.split('[').next().unwrap_or(a).to_string());
        connection_kind(&attr_type(s, &n, &a))
    });
    let mut out = Vec::new();
    for a in attrs {
        if a.contains('.') || a.contains('[') {
            continue;
        }
        if let Some(w) = &want {
            if connection_kind(&attr_type(s, node, &a)) != *w {
                continue;
            }
        }
        out.push(a);
    }
    out
}

/// The menu's items: on -beginMenu each picks the start of a connection (kept in $gAttributeMenuPlug); to complete one
/// (with -plug), each connects the plug at the other end. -finishMenu ends the menu with Other..., the Connection Editor.
fn attribute_menu(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(cb) = a.get("regPulldownMenuCommand").map(text).filter(|t| !t.is_empty()) {
        let mut next = NEXT_PULLDOWN.lock().unwrap_or_else(|e| e.into_inner());
        let id = *next;
        *next += 1;
        PULLDOWNS.lock().unwrap_or_else(|e| e.into_inner()).push((id, cb));
        return Ok(json!(id));
    }
    if a.has("unregPulldownMenuCommand") {
        let id = a.int("unregPulldownMenuCommand", -1);
        let mut list = PULLDOWNS.lock().unwrap_or_else(|e| e.into_inner());
        let before = list.len();
        list.retain(|(i, _)| *i != id);
        if list.len() == before {
            fail!("attributeMenu: no pulldown menu command with id {id}.");
        }
        return Ok(Value::Null);
    }
    let Some(menu) = current_menu() else { fail!("attributeMenu: No menu found.") };
    let inputs = a.get("inputs").is_none_or(|v| v.as_bool().unwrap_or(v.as_f64().is_some_and(|x| x != 0.0)));
    let begin = a.bool("beginMenu");
    let plug = a.get("plug").map(text).filter(|t| !t.is_empty());
    if let Some(node) = a.pos_str(0).map(str::to_string) {
        if !matches!(s.graph.find(&node), Ok(Some(_))) {
            fail!("attributeMenu: Object '{node}' not found.");
        }
        if !begin && !inputs && plug.is_none() {
            fail!("attributeMenu: -plug is needed to complete a connection.");
        }
        let other = if begin { None } else { plug.as_deref() };
        for attr in connection_attrs(s, &node, inputs, other) {
            let here = format!("{node}.{attr}");
            let command = if begin {
                format!("global string $gAttributeMenuPlug; $gAttributeMenuPlug = {};", quote(&here))
            } else if let Some(p) = &plug {
                if inputs {
                    format!("connectAttr -f {} {};", quote(p), quote(&here))
                } else {
                    format!("connectAttr -f {} {};", quote(&here), quote(p))
                }
            } else {
                format!("global string $gAttributeMenuPlug; if ($gAttributeMenuPlug != \"\") connectAttr -f $gAttributeMenuPlug {};", quote(&here))
            };
            add_item(&menu, &attr, Some(command));
        }
    } else if !a.bool("finishMenu") {
        fail!("attributeMenu: name a node.");
    }
    if a.bool("finishMenu") {
        add_item(&menu, "", None);
        add_item(&menu, "Other...", Some("ConnectionEditor;".to_string()));
    }
    Ok(json!(menu))
}

/// The keys of a call by long name, whichever form it came in.
fn long_keys(s: &Session, name: &str, call: &Value) -> Map<String, Value> {
    let mut out = Map::new();
    let def = s.commands.find(name);
    for (k, v) in call.as_object().into_iter().flatten() {
        let long = def.and_then(|d| d.flags.iter().find(|f| f.name == *k || f.short_name == *k)).map(|f| f.name.clone()).unwrap_or_else(|| k.clone());
        out.insert(long, v.clone());
    }
    out
}

/// Before attributeMenu builds a menu, each registered callback is asked
/// `proc(editor, node, plug, mode, menuType)`: one that answers 1 has built the menu itself, and nothing more is added.
pub(crate) fn intercept(core: &mut crate::core::Core, name: &str, call: &Value) -> Option<Result<Value, String>> {
    if name != "attributeMenu" {
        return None;
    }
    let procs: Vec<String> = PULLDOWNS.lock().unwrap_or_else(|e| e.into_inner()).iter().map(|x| x.1.clone()).collect();
    if procs.is_empty() {
        return None;
    }
    let o = long_keys(&core.s, name, call);
    if o.contains_key("regPulldownMenuCommand") || o.contains_key("unregPulldownMenuCommand") {
        return None;
    }
    let node = o.get("args").map(strings).and_then(|v| v.first().cloned())?;
    let editor = o.get("editor").map(text).unwrap_or_default();
    let plug = o.get("plug").map(text).unwrap_or_default();
    let begin = o.get("beginMenu").is_some_and(|v| v.as_bool().unwrap_or(false) || v.as_f64().is_some_and(|x| x != 0.0));
    let inputs = o.get("inputs").is_none_or(|v| v.as_bool().unwrap_or(v.as_f64().is_some_and(|x| x != 0.0)));
    let mode = if begin { "begin" } else { "complete" };
    let kind = if inputs { "inputs" } else { "outputs" };
    for p in procs {
        let line = format!("{p}({}, {}, {}, {}, {});", quote(&editor), quote(&node), quote(&plug), quote(mode), quote(kind));
        match core.mel_checked(&line) {
            Ok(v) if v.as_i64() == Some(1) || v.as_bool() == Some(true) => return Some(Ok(json!(current_menu().unwrap_or_default()))),
            Ok(_) => {}
            Err(e) => return Some(Err(e)),
        }
    }
    None
}

// ---------------------------------------------------------------------------------------------------------------------
// buildBookmarkMenu, buildKeyframeMenu

/// The menu filled again with the sets whose text (sets -text) is -type; picking one selects what it holds.
fn build_bookmark_menu(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.pos_str(0) else { fail!("buildBookmarkMenu: name a menu.") };
    let menu = menu_path(name, "buildBookmarkMenu")?;
    let ty = a.get("type").map(text).unwrap_or_default();
    clear_menu(&menu);
    let sets = s.run("ls", json!({"type": "objectSet"})).map(|v| strings(&v)).unwrap_or_default();
    for set in sets {
        let t = s.run("sets", json!({"args": [set.clone()], "query": true, "queried": ["text"]})).map(|v| text(&v)).unwrap_or_default();
        if ty.is_empty() || t == ty {
            add_item(&menu, &set, Some(format!("select -r {};", quote(&set))));
        }
    }
    Ok(Value::Null)
}

/// The menu filled again with what can be keyed on the selection, as the current tool narrows it: translate with Move,
/// rotate with Rotate, scale with Scale. Picking one keys it on the selected objects.
fn build_keyframe_menu(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.pos_str(0) else { fail!("buildKeyframeMenu: name a menu.") };
    let menu = menu_path(name, "buildKeyframeMenu")?;
    clear_menu(&menu);
    let tool = crate::ctx_cmds::current();
    let kind = crate::ctx_cmds::kind_of(&tool).unwrap_or_else(|| tool.clone()).to_lowercase();
    let only = if kind.contains("move") {
        Some("translate")
    } else if kind.contains("rotate") {
        Some("rotate")
    } else if kind.contains("scale") {
        Some("scale")
    } else {
        None
    };
    let selected: Vec<String> = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
    let mut attrs: Vec<String> = Vec::new();
    for n in &selected {
        for at in s.run("listAttr", json!({"args": [n], "keyable": true})).map(|v| strings(&v)).unwrap_or_default() {
            if only.is_some_and(|p| !at.starts_with(p)) || attrs.contains(&at) {
                continue;
            }
            attrs.push(at);
        }
    }
    for at in attrs {
        add_item(&menu, &at, Some(format!("setKeyframe -attribute {};", quote(&at))));
    }
    Ok(Value::Null)
}

pub fn register(s: &mut Session) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    s.register(
        CommandDef::new("attributeMenu", "Builds a connection menu in the current menu: a node's inputs or outputs, to start a connection or to complete one to -plug.", attribute_menu)
            .positional("node")
            .undoable(false)
            .flag(f("beginMenu", "beg", ArgType::Bool, "Start a connection: every input or output of the node."))
            .flag(f("editor", "edt", ArgType::String, "The editor the menu is for."))
            .flag(f("finishMenu", "fsh", ArgType::Bool, "End the menu."))
            .flag(f("inputs", "inp", ArgType::Bool, "List inputs (true) or outputs (false)."))
            .flag(f("plug", "p", ArgType::String, "The plug at the other end of the connection being completed."))
            .flag(f("regPulldownMenuCommand", "rpm", ArgType::String, "Register a procedure that builds the menu for some nodes; gives its id."))
            .flag(f("unregPulldownMenuCommand", "upm", ArgType::Int, "Unregister the procedure with this id.")),
    );
    s.register(
        CommandDef::new("buildBookmarkMenu", "Fills a menu with the bookmarks of one type: the sets whose text (sets -text) is -type.", build_bookmark_menu)
            .positional("menu")
            .flag(f("editor", "ed", ArgType::String, "The editor the menu belongs to."))
            .flag(f("type", "typ", ArgType::String, "The bookmark type, as sets -text gives it.")),
    );
    s.register(CommandDef::new("buildKeyframeMenu", "Fills a menu with what can be keyed on the selection, as the current tool narrows it.", build_keyframe_menu).positional("menu"));
}
