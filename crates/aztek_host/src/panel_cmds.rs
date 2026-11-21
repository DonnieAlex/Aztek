//! Panels and views from scripts: getPanel, modelPanel, modelEditor, lookThru, the camera moves and playblast.
//!
//! The model holds what each panel shows, so the commands answer with no interface. The shell tells it what the user
//! changed (`shell_sync`) and takes edits as "panelSet" requests. Camera moves work on the camera node; the views follow
//! camera nodes on their own.

use std::cell::RefCell;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, CommandError, FlagDef};
use aztek_scene::cameras as cam;
use aztek_scene::plug::{PlugRef, PlugValue};
use aztek_scene::transform::Transform;
use aztek_scene::world;
use aztek_scene::Session;
use glam::{DMat3, DVec3};
use serde_json::{json, Map, Value};

use crate::core::push_ui_request;

type Cmd = CommandDef<Session>;

#[derive(Clone, Debug)]
struct Panel {
    name: String,
    kind: String,
    label: String,
    camera: String,
    /// modelEditor flags by long name.
    editor: Map<String, Value>,
    visible: bool,
    parent: String,
    scripted_type: String,
}

fn model_panel(name: &str, label: &str, camera: &str, visible: bool) -> Panel {
    Panel { name: name.into(), kind: "modelPanel".into(), label: label.into(), camera: camera.into(), editor: Map::new(), visible, parent: String::new(), scripted_type: String::new() }
}

fn other_panel(name: &str, kind: &str, label: &str, scripted: &str) -> Panel {
    Panel { name: name.into(), kind: kind.into(), label: label.into(), camera: String::new(), editor: Map::new(), visible: false, parent: String::new(), scripted_type: scripted.into() }
}

#[derive(Default)]
struct Panels {
    all: Vec<Panel>,
    focus: String,
    under_pointer: String,
    types: Vec<(String, Map<String, Value>)>,
    counter: u32,
    isolate: std::collections::HashMap<String, bool>,
}

thread_local! {
    static PANELS: RefCell<Panels> = RefCell::new(Panels::default());
}

fn with_panels<R>(f: impl FnOnce(&mut Panels) -> R) -> R {
    PANELS.with(|p| {
        let mut p = p.borrow_mut();
        if p.all.is_empty() {
            // As the reference names them: top, side, front, persp.
            p.all = vec![
                model_panel("modelPanel1", "Top View", "top", false),
                model_panel("modelPanel2", "Side View", "side", false),
                model_panel("modelPanel3", "Front View", "front", false),
                model_panel("modelPanel4", "Persp View", "persp", true),
                other_panel("outlinerPanel1", "outlinerPanel", "Outliner", ""),
                other_panel("graphEditor1", "scriptedPanel", "Graph Editor", "graphEditor"),
                other_panel("dopeSheetPanel1", "scriptedPanel", "Dope Sheet", "dopeSheetPanel"),
                other_panel("nodeEditorPanel1", "scriptedPanel", "Node Editor", "nodeEditorPanel"),
                other_panel("polyTexturePlacementPanel1", "scriptedPanel", "UV Editor", "polyTexturePlacementPanel"),
                other_panel("hyperGraphPanel1", "scriptedPanel", "Hypergraph", "hyperGraphPanel"),
            ];
            p.focus = "modelPanel4".into();
            p.under_pointer = "modelPanel4".into();
        }
        f(&mut p)
    })
}

pub fn panel_type(name: &str) -> Option<String> {
    with_panels(|p| p.all.iter().find(|x| x.name == name).map(|x| x.kind.clone()))
}

pub fn delete_panel(name: &str) {
    with_panels(|p| p.all.retain(|x| x.name != name || (1..=4).any(|i| x.name == format!("modelPanel{i}"))));
}

/// The camera the panel in focus looks through.
pub(crate) fn active_camera() -> String {
    with_panels(|p| {
        let focus = p.focus.clone();
        p.all.iter().find(|x| x.name == focus && x.kind == "modelPanel").or_else(|| p.all.iter().find(|x| x.name == "modelPanel4")).map(|x| x.camera.clone()).unwrap_or_else(|| "persp".to_string())
    })
}

fn focus_model_panel() -> String {
    with_panels(|p| {
        let focus = p.focus.clone();
        if p.all.iter().any(|x| x.name == focus && x.kind == "modelPanel") {
            focus
        } else {
            "modelPanel4".to_string()
        }
    })
}

/// What the shell's views show now, with the panel in focus. The display flags come as the view's own "disp" keys.
pub fn shell_sync(v: &Value) {
    with_panels(|p| {
        if let Some(f) = v["focus"].as_str() {
            p.focus = f.to_string();
            p.under_pointer = v["underPointer"].as_str().unwrap_or(f).to_string();
        }
        for item in v["panels"].as_array().into_iter().flatten() {
            let Some(name) = item["name"].as_str() else { continue };
            let Some(x) = p.all.iter_mut().find(|x| x.name == name) else { continue };
            if let Some(c) = item["camera"].as_str() {
                x.camera = c.to_string();
            }
            if let Some(b) = item["visible"].as_bool() {
                x.visible = b;
            }
            if let Some(d) = item["disp"].as_object() {
                for (key, flag) in DISP {
                    if let Some(b) = d.get(*key).and_then(Value::as_bool) {
                        x.editor.insert(flag.to_string(), json!(b));
                    }
                }
                if let Some(m) = d.get("displayLights").and_then(Value::as_str) {
                    x.editor.insert("displayLights".into(), json!(m));
                }
                let appearance = if item["wireframe"].as_bool() == Some(true) {
                    "wireframe"
                } else if d.get("boundingBox").and_then(Value::as_bool) == Some(true) {
                    "boundingBox"
                } else if d.get("flat").and_then(Value::as_bool) == Some(true) {
                    "flatShaded"
                } else {
                    "smoothShaded"
                };
                x.editor.insert("displayAppearance".into(), json!(appearance));
            }
            if let Some(b) = item["wireOnShaded"].as_bool() {
                x.editor.insert("wireframeOnShaded".into(), json!(b));
            }
            if let Some(b) = item["grid"].as_bool() {
                x.editor.insert("grid".into(), json!(b));
            }
        }
    });
}

/// The view's "disp" keys and the modelEditor flags they answer to.
const DISP: &[(&str, &str)] = &[
    ("polygons", "polymeshes"),
    ("surfaces", "nurbsSurfaces"),
    ("curves", "nurbsCurves"),
    ("joints", "joints"),
    ("ikHandles", "ikHandles"),
    ("deformers", "deformers"),
    ("locators", "locators"),
    ("cameras", "cameras"),
    ("lights", "lights"),
    ("twoSided", "twoSidedLighting"),
    ("shadows", "shadows"),
    ("manipulators", "manipulators"),
    ("xray", "xray"),
    ("xrayJoints", "jointXray"),
    ("xrayActive", "activeComponentsXray"),
    ("backface", "backfaceCulling"),
    ("highlight", "selectionHiliteDisplay"),
    ("textured", "displayTextures"),
    ("defaultMaterial", "useDefaultMaterial"),
];

fn f(name: &str, short: &str, ty: ArgType) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, "")
}

fn asked(a: &Args) -> Option<String> {
    a.get("queried").and_then(Value::as_array).and_then(|q| q.first()).and_then(Value::as_str).map(str::to_string).or_else(|| a.flags.keys().find(|k| !matches!(k.as_str(), "query" | "queried")).cloned())
}

fn panel_set(panel: &str, key: &str, value: Value) {
    push_ui_request(format!("panelSet|{}", json!({"panel": panel, "key": key, "value": value})));
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or(v.as_i64().is_some_and(|i| i != 0))
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands on panels

fn get_panel(a: &Args) -> CmdResult {
    with_panels(|p| {
        if a.bool("withFocus") {
            return Ok(json!(p.focus));
        }
        if a.bool("underPointer") {
            return Ok(json!(p.under_pointer));
        }
        if let Some(l) = a.get("withLabel").and_then(Value::as_str) {
            return Ok(json!(p.all.iter().find(|x| x.label == l).map(|x| x.name.clone()).unwrap_or_default()));
        }
        if let Some(n) = a.get("typeOf").and_then(Value::as_str) {
            return Ok(json!(p.all.iter().find(|x| x.name == n).map(|x| x.kind.clone()).unwrap_or_else(|| "unknown".to_string())));
        }
        if let Some(t) = a.get("type").and_then(Value::as_str) {
            let v: Vec<String> = p.all.iter().filter(|x| x.kind == t).map(|x| x.name.clone()).collect();
            return Ok(json!(v));
        }
        if let Some(t) = a.get("scriptType").and_then(Value::as_str) {
            let v: Vec<String> = p.all.iter().filter(|x| x.scripted_type == t).map(|x| x.name.clone()).collect();
            return Ok(json!(v));
        }
        if a.bool("visiblePanels") {
            return Ok(json!(p.all.iter().filter(|x| x.visible).map(|x| x.name.clone()).collect::<Vec<_>>()));
        }
        if a.bool("invisiblePanels") {
            return Ok(json!(p.all.iter().filter(|x| !x.visible).map(|x| x.name.clone()).collect::<Vec<_>>()));
        }
        if a.bool("allTypes") {
            let mut v: Vec<String> = p.all.iter().map(|x| x.kind.clone()).collect();
            v.sort();
            v.dedup();
            return Ok(json!(v));
        }
        if a.bool("allScriptedTypes") {
            let mut v: Vec<String> = p.all.iter().filter(|x| !x.scripted_type.is_empty()).map(|x| x.scripted_type.clone()).collect();
            v.sort();
            v.dedup();
            return Ok(json!(v));
        }
        if a.bool("allConfigs") {
            return Ok(json!(crate::script_objects::all_configs()));
        }
        if let Some(l) = a.get("configWithLabel").map(text) {
            return Ok(json!(crate::script_objects::config_with_label(&l).unwrap_or_default()));
        }
        if a.has("atPosition") {
            return Ok(json!(p.focus));
        }
        if let Some(c) = a.get("containing").and_then(Value::as_str) {
            return Ok(json!(p.all.iter().find(|x| x.name == c || x.camera == c).map(|x| x.name.clone()).unwrap_or_default()));
        }
        Ok(json!(p.all.iter().map(|x| x.name.clone()).collect::<Vec<_>>()))
    })
}

const EDITOR_FLAGS: &str = "activeOnly:ao:b activeView:av:b allObjects:alo:b backfaceCulling:bfc:b camera:cam:a cameras:ca:b clipGhosts:cg:b controlVertices:cv:b controllers:ctrl:b deformers:df:b dimensions:dim:b displayAppearance:da:a displayLights:dl:a displayTextures:dtx:b dynamicConstraints:dc:b dynamics:dy:b fluids:fl:b follicles:fo:b fogging:fg:b grid:gr:b hairSystems:hs:b handles:ha:b headsUpDisplay:hud:b hulls:hu:b ikHandles:ikh:b imagePlane:imp:b joints:j:b jointXray:jx:b lights:lt:b locators:lc:b lockMainConnection:lck:b lowQualityLighting:lql:b manipulators:m:b motionTrails:mt:b nCloths:ncl:b nParticles:npa:b nRigids:nr:b nurbsCurves:nc:b nurbsSurfaces:ns:b objectFilterShowInHUD:ofs:b panel:pnl:a parent:p:a pivots:pv:b planes:pl:b pluginObjects:po:a2 pluginShapes:ps:b polymeshes:pm:b rendererName:rnm:a selectionHiliteDisplay:sel:b shadows:sdw:b smoothWireframe:swf:b sortTransparent:st:b strokes:str:b subdivSurfaces:sds:b textures:tx:b twoSidedLighting:tsl:b useDefaultMaterial:udm:b viewObjects:vo:b viewSelected:vs:b wireframeBackingStore:wbs:b wireframeOnShaded:wos:b xray:xr:b activeComponentsXray:acx:b exists:ex:b query:q:b edit:e:b queried::a";

fn add_spec(mut c: Cmd, spec: &str) -> Cmd {
    for t in spec.split_whitespace() {
        let mut it = t.split(':');
        let (Some(long), Some(short), Some(ty)) = (it.next(), it.next(), it.next()) else { continue };
        if c.flags.iter().any(|x| x.name == long) {
            continue;
        }
        let mut d = f(long, short, if ty == "b" { ArgType::Bool } else { ArgType::Any });
        if let Some(n) = ty.strip_prefix('a').and_then(|n| n.parse::<u8>().ok()) {
            d = d.multi(n);
        }
        c = c.flag(d);
    }
    c
}

/// A modelEditor flag to the view setting it changes.
fn editor_to_view(panel: &str, flag: &str, v: &Value) {
    match flag {
        "camera" => panel_set(panel, "camera", json!(text(v))),
        "displayAppearance" => {
            let da = text(v);
            panel_set(panel, "wireframe", json!(da == "wireframe"));
            panel_set(panel, "disp", json!({"flat": da == "flatShaded", "boundingBox": da == "boundingBox"}));
        }
        "wireframeOnShaded" => panel_set(panel, "wireOnShaded", json!(on(v))),
        "displayLights" => panel_set(panel, "disp", json!({"displayLights": text(v)})),
        "grid" => panel_set(panel, "grid", json!(on(v))),
        _ => {
            if let Some((key, _)) = DISP.iter().find(|(_, fl)| *fl == flag) {
                panel_set(panel, "disp", json!({ *key: on(v) }));
            }
        }
    }
}

/// Nested lists as one flat list.
fn flat_values(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a.iter().flat_map(flat_values).collect(),
        x => vec![x.clone()],
    }
}

fn text(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), str::to_string)
}

const OBJECT_FLAGS: &[&str] = &["cameras", "controlVertices", "controllers", "deformers", "dimensions", "dynamicConstraints", "dynamics", "fluids", "follicles", "grid", "hairSystems", "handles", "hulls", "ikHandles", "imagePlane", "joints", "lights", "locators", "manipulators", "motionTrails", "nCloths", "nParticles", "nRigids", "nurbsCurves", "nurbsSurfaces", "pivots", "planes", "pluginShapes", "polymeshes", "strokes", "subdivSurfaces", "textures"];

fn editor_default(flag: &str) -> Value {
    match flag {
        "displayAppearance" => json!("smoothShaded"),
        "displayLights" => json!("default"),
        "rendererName" => json!("vp2Renderer"),
        "headsUpDisplay" | "selectionHiliteDisplay" | "twoSidedLighting" => json!(true),
        "displayTextures" | "wireframeOnShaded" | "xray" | "jointXray" | "activeComponentsXray" | "backfaceCulling" | "useDefaultMaterial" | "activeOnly" | "viewSelected" | "shadows" => json!(false),
        k if OBJECT_FLAGS.contains(&k) => json!(true),
        _ => Value::Null,
    }
}

fn model_editor(a: &Args) -> CmdResult {
    // No name: the panel in focus, as most scripts mean.
    let name = a.pos_str(0).map(str::to_string);
    let exists = |n: &str| with_panels(|p| p.all.iter().any(|x| x.name == n && x.kind == "modelPanel"));
    if a.bool("exists") {
        return Ok(json!(name.as_deref().is_some_and(exists)));
    }
    let Some(name) = name else {
        if !a.bool("query") && !a.bool("edit") {
            return Ok(json!(focus_model_panel()));
        }
        fail!("modelEditor: name an editor.")
    };
    if !exists(&name) {
        fail!("modelEditor: Object '{name}' not found.");
    }
    if a.bool("query") {
        let Some(q) = asked(a) else { fail!("modelEditor: name what to ask about.") };
        return with_panels(|p| {
            let x = p.all.iter().find(|x| x.name == name).unwrap();
            Ok(match q.as_str() {
                "camera" => json!(x.camera),
                "panel" => json!(x.name),
                "activeView" => json!(p.focus == name),
                _ => x.editor.get(&q).cloned().unwrap_or_else(|| editor_default(&q)),
            })
        });
    }
    let mut flags = a.flags.clone();
    flags.retain(|k, v| !matches!(k.as_str(), "query" | "edit" | "queried" | "exists") && !v.is_null());
    // -allObjects first, so the flags naming kinds beside it win.
    if let Some(v) = flags.get("allObjects").cloned() {
        let o = on(&v);
        with_panels(|p| {
            if let Some(x) = p.all.iter_mut().find(|x| x.name == name) {
                for k in OBJECT_FLAGS {
                    x.editor.insert(k.to_string(), json!(o));
                }
            }
        });
        let all: Map<String, Value> = DISP.iter().filter(|(_, fl)| OBJECT_FLAGS.contains(fl)).map(|(k, _)| (k.to_string(), json!(o))).collect();
        panel_set(&name, "disp", Value::Object(all));
    }
    for (k, v) in &flags {
        if k == "allObjects" {
            continue;
        }
        if k == "activeView" {
            if on(v) {
                with_panels(|p| p.focus = name.clone());
                push_ui_request(format!("panelFocus|{name}"));
            }
            continue;
        }
        with_panels(|p| {
            if let Some(x) = p.all.iter_mut().find(|x| x.name == name) {
                if k == "camera" {
                    x.camera = text(v);
                } else {
                    x.editor.insert(k.clone(), v.clone());
                }
            }
        });
        editor_to_view(&name, k, v);
    }
    Ok(json!(name))
}

/// -menuBarRepeatLast, held for -q; -tearOffRestore, the panel brought back and given the focus.
fn panel_extras(n: &str, a: &Args) {
    if let Some(v) = a.get("menuBarRepeatLast").and_then(Value::as_bool) {
        with_panels(|p| {
            if let Some(x) = p.all.iter_mut().find(|x| x.name == n) {
                x.editor.insert("menuBarRepeatLast".into(), json!(v));
            }
        });
    }
    if a.bool("tearOffRestore") {
        push_ui_request(format!("panelFocus|{n}"));
    }
}

fn panel_common(kind: &'static str, a: &Args) -> CmdResult {
    let name = a.pos_str(0).map(str::to_string);
    if a.bool("exists") {
        return Ok(json!(name.is_some_and(|n| with_panels(|p| p.all.iter().any(|x| x.name == n && (kind == "panel" || x.kind == kind))))));
    }
    if a.bool("query") {
        let Some(n) = name else { fail!("{kind}: name a panel.") };
        let Some(q) = asked(a) else { fail!("{kind}: name what to ask about.") };
        return with_panels(|p| {
            let Some(x) = p.all.iter().find(|x| x.name == n) else { fail!("{kind}: Object '{n}' not found.") };
            Ok(match q.as_str() {
                "camera" => json!(x.camera),
                "label" => json!(x.label),
                "modelEditor" | "outlinerEditor" | "blendShapeEditor" | "poseEditor" => json!(x.name),
                "control" => json!(x.parent),
                "parent" => json!(x.parent),
                "type" => json!(x.scripted_type),
                "menuBarVisible" | "barLayout" => json!(true),
                "menuBarRepeatLast" => x.editor.get("menuBarRepeatLast").cloned().unwrap_or(json!(false)),
                "exists" => json!(true),
                _ => x.editor.get(&q).cloned().unwrap_or(Value::Null),
            })
        });
    }
    if a.bool("edit") {
        let Some(n) = name else { fail!("{kind}: name a panel.") };
        if !with_panels(|p| p.all.iter().any(|x| x.name == n)) {
            fail!("{kind}: Object '{n}' not found.");
        }
        if let Some(c) = a.get("camera").map(text) {
            with_panels(|p| {
                if let Some(x) = p.all.iter_mut().find(|x| x.name == n) {
                    x.camera = c.clone();
                }
            });
            panel_set(&n, "camera", json!(c));
        }
        if let Some(l) = a.get("label").map(text) {
            with_panels(|p| {
                if let Some(x) = p.all.iter_mut().find(|x| x.name == n) {
                    x.label = l.clone();
                }
            });
        }
        if let Some(pr) = a.get("parent").map(text) {
            with_panels(|p| {
                if let Some(x) = p.all.iter_mut().find(|x| x.name == n) {
                    x.parent = pr.clone();
                }
            });
        }
        panel_extras(&n, a);
        return Ok(json!(n));
    }
    // Create, in the layout being built or the one named. Model panels show a real view there.
    let n = match name {
        Some(n) => n,
        None => with_panels(|p| loop {
            p.counter += 1;
            let base = if kind == "panel" { "panel" } else { kind };
            let n = format!("{base}{}", p.counter + if kind == "modelPanel" { 4 } else { 0 });
            if !p.all.iter().any(|x| x.name == n) {
                break n;
            }
        }),
    };
    if with_panels(|p| p.all.iter().any(|x| x.name == n)) {
        fail!("{kind}: Object's name '{n}' is not unique.");
    }
    let label = a.get("label").map(text).unwrap_or_else(|| n.clone());
    let cam = a.get("camera").map(text).unwrap_or_else(|| "persp".to_string());
    let parent = a.get("parent").map(text).unwrap_or_default();
    let mut panel = if kind == "modelPanel" { model_panel(&n, &label, &cam, true) } else { other_panel(&n, kind, &label, &a.string("type", "")) };
    panel.parent = parent.clone();
    with_panels(|p| p.all.push(panel));
    panel_extras(&n, a);
    match kind {
        "modelPanel" | "scriptedPanel" => crate::scriptui::embed_panel(kind, &n, Some(&parent), &cam),
        "outlinerPanel" => crate::scriptui::embed_panel("outlinerEditor", &n, Some(&parent), ""),
        "blendShapePanel" => crate::scriptui::embed_panel("blendShapeEditor", &n, Some(&parent), ""),
        "posePanel" => crate::scriptui::embed_panel("poseEditor", &n, Some(&parent), ""),
        _ => {}
    }
    Ok(json!(n))
}

/// The settings a scripted panel type keeps: everything but the mode flags.
const TYPE_META: &[&str] = &["query", "edit", "queried", "exists", "defineTemplate", "useTemplate", "args"];

fn scripted_panel_type(a: &Args) -> CmdResult {
    let Some(n) = a.pos_str(0).map(str::to_string) else { fail!("scriptedPanelType: give a name.") };
    if a.bool("exists") {
        return Ok(json!(with_panels(|p| p.types.iter().any(|(t, _)| *t == n))));
    }
    if a.bool("query") {
        let Some(q) = asked(a) else { fail!("scriptedPanelType: name what to ask about.") };
        let Some(m) = with_panels(|p| p.types.iter().find(|(t, _)| *t == n).map(|(_, m)| m.clone())) else { fail!("scriptedPanelType: Object '{n}' not found.") };
        return Ok(match m.get(&q) {
            Some(v) => v.clone(),
            // The hotkey context is the type's own name unless one is given.
            None if q == "hotkeyCtxClient" => json!(n),
            None if matches!(q.as_str(), "customView" | "obsolete" | "retainOnFileOpen" | "unique") => json!(false),
            None => json!(""),
        });
    }
    let mut m: Map<String, Value> = a.flags.iter().filter(|(k, _)| !TYPE_META.contains(&k.as_str())).map(|(k, v)| (k.clone(), v.clone())).collect();
    with_panels(|p| {
        if a.bool("edit") {
            match p.types.iter_mut().find(|(t, _)| *t == n) {
                Some((_, old)) => {
                    old.append(&mut m);
                    return Ok(());
                }
                None => fail!("scriptedPanelType: Object '{n}' not found."),
            }
        }
        p.types.retain(|(t, _)| *t != n);
        p.types.push((n.clone(), m));
        Ok(())
    })?;
    Ok(json!(n))
}

/// refresh with -filename: a snapshot of the current view (with -currentView, or the view in focus standing for all of
/// them) written as an image, of the type -fileExtension names.
fn refresh_cmd(s: &mut Session) {
    let Some(old) = s.commands.find("refresh").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = add_spec(old, "fileExtension:fe:a filename:fn:a");
    cmd.doc = "Bring the scene up to date and redraw the views; -fn writes a snapshot of the view to a file (-fe its type, -cv the current view only).".into();
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let r = run(s, a)?;
        let Some(file) = a.get("filename").map(text).filter(|f| !f.is_empty()) else { return Ok(r) };
        let ext = a.get("fileExtension").map(text).filter(|e| !e.is_empty());
        let has_ext = std::path::Path::new(&file).extension().is_some();
        let file = match (&ext, has_ext) {
            (Some(e), false) => format!("{file}.{e}"),
            _ => file,
        };
        let time = aztek_scene::anim::frame(&s.graph);
        let payload = json!({
            "frames": [time], "files": [file.clone()], "passes": [{"setup": "", "files": [file.clone()]}], "width": 0, "height": 0, "percent": 100.0,
            "quality": 90, "panel": focus_model_panel(), "viewer": false, "ornaments": true, "exposure": 0.0, "gamma": 1.0, "format": ext,
        });
        if crate::scriptui::call_hook("playblast", &payload).is_none() {
            crate::script_jobs::warn("refresh: no view to capture without the interface; nothing was written.");
        }
        aztek_scene::anim::set_frame(&mut s.graph, time);
        Ok(r)
    });
    s.register(cmd);
}

/// saveViewportSettings: each model panel's camera and model editor settings, one optionVar a panel holding them.
fn save_viewport_settings(s: &mut Session) -> CmdResult {
    let panels: Vec<(String, String, Map<String, Value>)> = with_panels(|p| p.all.iter().filter(|x| x.kind == "modelPanel").map(|x| (x.name.clone(), x.camera.clone(), x.editor.clone())).collect());
    for (name, camera, editor) in panels {
        let mut state = editor;
        state.insert("camera".into(), json!(camera));
        s.run("optionVar", json!({"stringValue": [format!("{name}ViewportSettings"), Value::Object(state).to_string()]}))?;
    }
    Ok(Value::Null)
}

fn scripted_panel(a: &Args) -> CmdResult {
    if a.bool("query") || a.bool("exists") {
        return panel_common("scriptedPanel", a);
    }
    if a.bool("edit") {
        return panel_common("scriptedPanel", a);
    }
    // A torn off copy is of its original's type.
    let original = a.get("tearOffCopy").or_else(|| a.get("copy")).map(text).filter(|t| !t.is_empty());
    let mut ty = a.string("type", "");
    if ty.is_empty() {
        if let Some(o) = &original {
            ty = with_panels(|p| p.all.iter().find(|x| x.name == *o).map(|x| x.scripted_type.clone())).unwrap_or_default();
        }
    }
    let script = with_panels(|p| p.types.iter().find(|(n, _)| *n == ty).map(|(_, m)| m.clone()));
    if script.is_none() {
        fail!("scriptedPanel: Object '{ty}' not found: declare it with scriptedPanelType.");
    }
    let made = panel_common("scriptedPanel", a)?;
    let name = made.as_str().unwrap_or("").to_string();
    // The type's callbacks build it, with the panel's name, as the reference calls them.
    if let Some(m) = script {
        for flag in ["createCallback", "initCallback", "addCallback"] {
            if let Some(proc_name) = m.get(flag).and_then(Value::as_str).filter(|t| !t.is_empty()) {
                crate::scriptui::run_soon(json!(format!("{proc_name}(\"{name}\");")), Vec::new());
            }
        }
        // A copy: once the add callback has run, the copy callback gives it the original's state.
        if let (Some(o), Some(proc_name)) = (&original, m.get("copyStateCallback").and_then(Value::as_str).filter(|t| !t.is_empty())) {
            crate::scriptui::run_soon(json!(format!("{proc_name}(\"{o}\", \"{name}\");")), Vec::new());
        }
    }
    Ok(made)
}

// ---------------------------------------------------------------------------------------------------------------------
// Camera moves

struct Camera {
    tr: aztek_scene::plug::NodeId,
    sh: aztek_scene::plug::NodeId,
    eye: DVec3,
    /// Local axes in the world: x right, y up, z backward.
    axes: DMat3,
    coi: f64,
    ortho: bool,
    width: f64,
}

fn camera_of(s: &mut Session, name: Option<&str>, cmd: &str) -> CmdResult<Camera> {
    let name = name.map(str::to_string).unwrap_or_else(active_camera);
    let g = &mut s.graph;
    let Ok(Some(id)) = g.find(&name) else { fail!("{cmd}: Object '{name}' not found.") };
    let Some(sh) = cam::shape_of(g, id) else { fail!("{cmd}: {name} is not a camera.") };
    let tr = aztek_scene::dag::transform_of(g, sh);
    g.evaluate();
    let w = aztek_scene::rig::world(g, tr);
    let d = g.node(sh).unwrap().data();
    let (coi, ortho, width) = (d.float(cam::COI), d.bool(cam::ORTHOGRAPHIC), d.float(cam::ORTHO_WIDTH));
    let (x, y, z) = (w.x_axis.truncate().normalize_or_zero(), w.y_axis.truncate().normalize_or_zero(), w.z_axis.truncate().normalize_or_zero());
    Ok(Camera { tr, sh, eye: w.w_axis.truncate(), axes: DMat3::from_cols(x, y, z), coi, ortho, width })
}

impl Camera {
    fn forward(&self) -> DVec3 {
        -self.axes.z_axis
    }
    fn center(&self) -> DVec3 {
        self.eye + self.forward() * self.coi
    }
}

fn set_plug(s: &mut Session, id: aztek_scene::plug::NodeId, index: usize, v: PlugValue) -> CmdResult<()> {
    s.graph.set_value(PlugRef::new(id, index as i32), v).map_err(|e| CommandError::new(e.to_string()))
}

fn put_eye(s: &mut Session, c: &Camera, eye: DVec3) -> CmdResult<()> {
    // Parented cameras take the move in their parent's space.
    let p = s.graph.parent(c.tr);
    let local = if p != aztek_scene::plug::NO_NODE { aztek_scene::rig::world(&mut s.graph, p).inverse().transform_point3(eye) } else { eye };
    set_plug(s, c.tr, Transform::TRANSLATE, PlugValue::Vec3(local))
}

/// Euler XYZ, degrees, for a camera looking along `forward` with `up` as near its up as can be.
fn look_rotation(forward: DVec3, up: DVec3) -> DVec3 {
    let z = (-forward).normalize_or_zero();
    let mut x = up.cross(z);
    if x.length_squared() < 1e-10 {
        x = DVec3::X.cross(z);
        if x.length_squared() < 1e-10 {
            x = DVec3::Y.cross(z);
        }
    }
    let x = x.normalize();
    let y = z.cross(x);
    // R = Rz Ry Rx in column terms, the transform's XYZ order.
    let sy = (-x.z).clamp(-1.0, 1.0);
    let ry = sy.asin();
    let (rx, rz) = if ry.cos().abs() > 1e-6 { (y.z.atan2(z.z), x.y.atan2(x.x)) } else { ((-z.y).atan2(y.y), 0.0) };
    DVec3::new(rx, ry, rz)
}

fn aim(s: &mut Session, c: &Camera, eye: DVec3, target: DVec3) -> CmdResult<()> {
    let fwd = target - eye;
    put_eye(s, c, eye)?;
    set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(look_rotation(fwd, world::up())))?;
    set_plug(s, c.sh, cam::COI, PlugValue::Float(fwd.length().max(1e-3)))
}

fn vec3_of(v: &Value) -> Option<DVec3> {
    let a = v.as_array()?;
    Some(DVec3::new(a.first()?.as_f64()?, a.get(1)?.as_f64()?, a.get(2)?.as_f64()?))
}

fn bounds(s: &mut Session, names: Vec<String>) -> CmdResult<Option<(DVec3, DVec3)>> {
    if names.is_empty() {
        return Ok(None);
    }
    let v = s.run("exactWorldBoundingBox", json!({"args": names}))?;
    let b: Vec<f64> = v.as_array().map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
    if b.len() < 6 {
        return Ok(None);
    }
    let (lo, hi) = (DVec3::new(b[0], b[1], b[2]), DVec3::new(b[3], b[4], b[5]));
    if (hi - lo).length() < 1e-9 && lo.length() < 1e-9 {
        return Ok(None);
    }
    Ok(Some((lo, hi)))
}

fn every_object(s: &mut Session) -> Vec<String> {
    let g = &s.graph;
    g.nodes().into_iter().filter(|&i| g.parent(i) == aztek_scene::plug::NO_NODE && g.node(i).is_some_and(|n| n.ty().dag) && !cam::is_default_camera(g, i)).map(|i| g.path(i)).collect()
}

fn fit_camera(s: &mut Session, c: &Camera, b: (DVec3, DVec3), fraction: f64) -> CmdResult<()> {
    let center = (b.0 + b.1) * 0.5;
    let radius = ((b.1 - b.0).length() * 0.5).max(1e-3);
    let d = s.graph.node(c.sh).unwrap().data();
    let (fl, hfa, vfa) = (d.float(cam::FOCAL_LENGTH), d.float(cam::H_APERTURE), d.float(cam::V_APERTURE));
    let fraction = fraction.max(1e-3);
    if c.ortho {
        let eye = center - c.forward() * c.coi;
        put_eye(s, c, eye)?;
        return set_plug(s, c.sh, cam::ORTHO_WIDTH, PlugValue::Float(2.0 * radius / fraction));
    }
    // The tighter of the two field angles decides.
    let half = (hfa.min(vfa) * 25.4 * 0.5 / fl.max(1e-6)).atan();
    let dist = radius / (half.sin().max(1e-6) * fraction);
    put_eye(s, c, center - c.forward() * dist)?;
    set_plug(s, c.sh, cam::COI, PlugValue::Float(dist))
}

fn cam_arg(a: &Args) -> Option<String> {
    a.named().into_iter().next()
}

/// The reference's home for a default camera, turned with the world when Z is up; other cameras go to the
/// perspective's.
fn home_of(name: &str) -> (DVec3, DVec3, f64) {
    cam::home(name)
}

thread_local! {
    /// viewSet's views of each camera, older first, and where in them the camera is: -previousView and -nextView step
    /// through them.
    static VIEWS: std::cell::RefCell<std::collections::HashMap<String, (Vec<(DVec3, DVec3, f64)>, usize)>> = Default::default();
    /// viewSet -fit and -keepRenderSettings as last given, for -q.
    static VIEW_SET_FLAGS: std::cell::RefCell<(bool, bool)> = const { std::cell::RefCell::new((false, false)) };
}

fn view_state(s: &Session, c: &Camera) -> (DVec3, DVec3, f64) {
    let d = s.graph.node(c.tr).unwrap().data();
    (d.vec3(Transform::TRANSLATE), d.vec3(Transform::ROTATE), c.coi)
}

fn view_set(s: &mut Session, a: &Args) -> CmdResult {
    let name = cam_arg(a).unwrap_or_else(active_camera);
    let c = camera_of(s, Some(&name), "viewSet")?;
    let path = s.graph.path(c.tr);
    if a.bool("query") {
        let asked = |k: &str| a.bool(k) || a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == k));
        let (fit, keep) = VIEW_SET_FLAGS.with(|v| *v.borrow());
        let (n, at) = VIEWS.with(|v| v.borrow().get(&path).map(|(l, i)| (l.len(), *i)).unwrap_or((0, 0)));
        return Ok(json!(if asked("previousView") {
            at > 0
        } else if asked("nextView") {
            at + 1 < n
        } else if asked("keepRenderSettings") {
            keep
        } else if asked("fit") {
            fit
        } else {
            fail!("viewSet: name what to query: -fit, -keepRenderSettings, -previousView or -nextView.")
        }));
    }
    VIEW_SET_FLAGS.with(|v| {
        let mut v = v.borrow_mut();
        if a.has("fit") {
            v.0 = a.bool("fit");
        }
        if a.has("keepRenderSettings") {
            v.1 = a.bool("keepRenderSettings");
        }
    });
    if a.bool("previousView") || a.bool("nextView") {
        // The camera goes back or on to a view it had; the panel's own history follows.
        let back = a.bool("previousView");
        let now = view_state(s, &c);
        let step = VIEWS.with(|v| {
            let mut v = v.borrow_mut();
            let (list, at) = v.entry(path.clone()).or_default();
            if *at == list.len() {
                list.push(now);
            } else {
                list[*at] = now;
            }
            let to = if back { at.checked_sub(1) } else { Some(*at + 1).filter(|i| *i < list.len()) };
            to.map(|i| {
                *at = i;
                list[i]
            })
        });
        push_ui_request(if back { "panelFocusUndo" } else { "panelFocusRedo" }.to_string());
        if let Some((t, r, coi)) = step {
            set_plug(s, c.tr, Transform::TRANSLATE, PlugValue::Vec3(t))?;
            set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(r))?;
            set_plug(s, c.sh, cam::COI, PlugValue::Float(coi))?;
        }
        return Ok(Value::Null);
    }
    // The view the camera leaves is kept to go back to.
    let before = view_state(s, &c);
    VIEWS.with(|v| {
        let mut v = v.borrow_mut();
        let (list, at) = v.entry(path.clone()).or_default();
        list.truncate(*at);
        list.push(before);
        *at = list.len();
    });
    let center = c.center();
    let dist = c.coi;
    // Directions in the Y up frame, turned with the world: -top looks down world up whichever axis that is.
    let side = |s: &mut Session, dir: DVec3| -> CmdResult<()> {
        let up = world::to_world(if dir.y.abs() > 0.9 { -DVec3::Z * dir.y.signum() } else { DVec3::Y });
        let dir = world::to_world(dir);
        let eye = center + dir * dist;
        put_eye(s, &c, eye)?;
        set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(look_rotation(-dir, up)))
    };
    let mut fit = a.bool("fit");
    if a.bool("home") {
        let (t, r, coi) = home_of(&name);
        put_eye(s, &c, t)?;
        set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(r))?;
        set_plug(s, c.sh, cam::COI, PlugValue::Float(coi))?;
    } else if a.bool("persp") {
        let (_, r, _) = home_of("persp");
        let m = glam::DMat3::from_euler(glam::EulerRot::ZYX, r.z, r.y, r.x);
        put_eye(s, &c, center + m.z_axis * dist)?;
        // r is already turned with the world.
        set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(r))?;
    } else if a.bool("top") {
        side(s, DVec3::Y)?;
    } else if a.bool("bottom") {
        side(s, -DVec3::Y)?;
    } else if a.bool("front") {
        side(s, DVec3::Z)?;
    } else if a.bool("back") {
        side(s, -DVec3::Z)?;
    } else if a.bool("side") || a.bool("rightSide") {
        side(s, DVec3::X)?;
    } else if a.bool("leftSide") {
        side(s, -DVec3::X)?;
    }
    if a.bool("fit") {
        fit = true;
    }
    if fit {
        let names = every_object(s);
        if let Some(b) = bounds(s, names)? {
            let c = camera_of(s, Some(&name), "viewSet")?;
            fit_camera(s, &c, b, a.float("fitFactor", 1.0))?;
        }
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// Playblast

/// A program on the PATH, by name; Windows' .exe is tried too.
pub fn find_program(name: &str) -> Option<std::path::PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).find_map(|dir| [dir.join(name), dir.join(format!("{name}.exe"))].into_iter().find(|p| p.is_file()))
}

/// The file a movie goes in: the name given, with the format's extension when it has none.
pub fn movie_file_name(base: &str, format: &str) -> String {
    let stem = base.rsplit(['/', '\\']).next().unwrap_or(base);
    if stem.contains('.') {
        return base.to_string();
    }
    format!("{base}.{}", if format == "avi" { "avi" } else { "mov" })
}

/// What ffmpeg is given for a movie from a numbered frame sequence. Sizes are made even, as yuv420p needs.
pub fn movie_arguments(pattern: &str, fps: f64, out: &str) -> Vec<String> {
    ["-y", "-loglevel", "error", "-framerate", &fps.to_string(), "-i", pattern, "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2", "-c:v", "libx264", "-pix_fmt", "yuv420p", out].iter().map(|x| x.to_string()).collect()
}

/// The sounds a playblast takes: -s the one named, -uts every sound on the timeline. Each as (file, start frame, mute).
fn playblast_sounds(s: &Session, a: &Args, first_frame: f64) -> Vec<(String, f64)> {
    let g = &s.graph;
    let ty = aztek_scene::sound::TYPE;
    let mut ids: Vec<aztek_scene::plug::NodeId> = Vec::new();
    if let Some(n) = a.get("sound").and_then(Value::as_str).filter(|n| !n.is_empty()) {
        if let Ok(Some(id)) = g.find(n) {
            ids.push(id);
        }
    }
    if a.bool("useTraxSounds") || a.bool("useSequencerSounds") {
        for id in g.nodes() {
            if g.node(id).is_some_and(|n| n.ty().name == ty) && !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    let attr = |id, k: &str| g.node(id).and_then(|n| n.ty().attr_index(k).map(|i| n.data().value(i).clone()));
    ids.into_iter()
        .filter(|&id| !attr(id, "mute").is_some_and(|v| v.as_bool()))
        .filter_map(|id| {
            let file = attr(id, "filename")?.as_str().to_string();
            let off = attr(id, "offset").map(|v| v.as_float()).unwrap_or(0.0);
            (!file.is_empty()).then_some((file, off - first_frame))
        })
        .collect()
}

/// ffmpeg's arguments adding sounds to a movie with no sound: each delayed to where it starts, one track each, or with
/// combine all mixed into one.
fn sound_arguments(movie: &str, sounds: &[(String, f64)], fps: f64, combine: bool, out: &str) -> Vec<String> {
    let mut v: Vec<String> = ["-y", "-loglevel", "error", "-i", movie].iter().map(|x| x.to_string()).collect();
    for (file, _) in sounds {
        v.extend(["-i".to_string(), file.clone()]);
    }
    let mut filter = Vec::new();
    for (i, (_, start)) in sounds.iter().enumerate() {
        let ms = ((start / fps.max(1e-6)) * 1000.0).max(0.0).round() as i64;
        filter.push(format!("[{}:a]adelay={ms}|{ms}[a{i}]", i + 1));
    }
    if combine && sounds.len() > 1 {
        let ins: String = (0..sounds.len()).map(|i| format!("[a{i}]")).collect();
        filter.push(format!("{ins}amix=inputs={}[mix]", sounds.len()));
    }
    v.extend(["-filter_complex".to_string(), filter.join(";"), "-map".to_string(), "0:v".to_string()]);
    if combine && sounds.len() > 1 {
        v.extend(["-map".to_string(), "[mix]".to_string()]);
    } else {
        for i in 0..sounds.len() {
            v.extend(["-map".to_string(), format!("[a{i}]")]);
        }
    }
    v.extend(["-c:v", "copy", "-c:a", "aac", "-shortest", out].iter().map(|x| x.to_string()));
    v
}

fn run_ffmpeg(args: Vec<String>, what: &str) -> CmdResult<()> {
    match std::process::Command::new("ffmpeg").args(args).output() {
        Ok(o) if o.status.success() => Ok(()),
        Ok(o) => {
            let said = String::from_utf8_lossy(&o.stderr);
            let last = said.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("").to_string();
            fail!("playblast: ffmpeg could not make {what}: {last}")
        }
        Err(e) => fail!("playblast: could not run ffmpeg: {e}"),
    }
}

fn playblast(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let q = asked(a).unwrap_or_default();
        return Ok(match q.as_str() {
            "activeEditor" => json!(focus_model_panel()),
            "format" => json!(["image", "movie", "qt", "avi"]),
            "compression" => json!(["png", "jpg", "bmp", "tif"]),
            _ => Value::Null,
        });
    }
    if a.bool("activeEditor") {
        return Ok(json!(focus_model_panel()));
    }
    // -options opens the Playblast options and -codecOptions the movie settings, which are those options here: no
    // playblast either way.
    if a.bool("options") || a.bool("codecOptions") {
        crate::scriptui::run_soon(json!("PlayblastOptions;"), Vec::new());
        return Ok(Value::Null);
    }
    let throw = a.bool("throwOnError");
    let time = aztek_scene::anim::frame(&s.graph);
    let range = |s: &mut Session, f: &str, d: f64| s.run("playbackOptions", json!({"query": true, "queried": [f]})).ok().and_then(|v| v.as_f64()).unwrap_or(d);
    let replacing = a.get("replaceFilename").and_then(Value::as_str).filter(|x| !x.is_empty()).map(str::to_string);
    let frames: Vec<f64> = if let Some(l) = a.get("frame").and_then(Value::as_array) {
        l.iter().filter_map(Value::as_f64).collect()
    } else if let Some(v) = a.get("frame").and_then(Value::as_f64) {
        vec![v]
    } else {
        let (sk, ek) = if replacing.is_some() { ("replaceStartTime", "replaceEndTime") } else { ("startTime", "endTime") };
        let st = if a.has(sk) { a.float(sk, time) } else if a.has("startTime") { a.float("startTime", time) } else { range(s, "minTime", time) };
        let et = if a.has(ek) { a.float(ek, time) } else if a.has("endTime") { a.float("endTime", time) } else { range(s, "maxTime", time) };
        if et < st {
            fail!("playblast: the end time is before the start time.");
        }
        let step = 1.0;
        let mut v = Vec::new();
        let mut t = st;
        while t <= et + 1e-6 {
            v.push(t);
            t += step;
        }
        v
    };
    let fmt = a.string("format", "image");
    let movie = fmt != "image" && fmt != "iff";
    if movie && find_program("ffmpeg").is_none() {
        fail!("playblast: format \"{fmt}\" needs ffmpeg, and there is none on the PATH. Install it, or use -format image for an image sequence.");
    }
    let compression = match a.string("compression", "png").to_lowercase().as_str() {
        "jpg" | "jpeg" => "jpg",
        "bmp" => "bmp",
        "tif" | "tiff" => "tif",
        _ => "png",
    };
    let fps = s.run("playbackOptions", json!({"query": true, "queried": ["framesPerSecond"]})).ok().and_then(|v| v.as_f64()).unwrap_or(24.0);
    let sounds = playblast_sounds(s, a, frames.first().copied().unwrap_or(time));
    // -replaceAudioOnly: the movie keeps its pictures, the sound is put in again.
    if a.bool("replaceAudioOnly") {
        let Some(file) = replacing.clone().or_else(|| a.get("filename").and_then(Value::as_str).map(str::to_string)) else { fail!("playblast: -replaceAudioOnly names the movie with -replaceFilename.") };
        if !std::path::Path::new(&file).exists() {
            fail!("playblast: {file} doesn't exist to replace the sound of.");
        }
        if sounds.is_empty() {
            fail!("playblast: -replaceAudioOnly needs a sound: -s, or -uts for the timeline's.");
        }
        if find_program("ffmpeg").is_none() {
            fail!("playblast: replacing a movie's sound needs ffmpeg, and there is none on the PATH.");
        }
        let tmp = format!("{file}.sound.{}", std::path::Path::new(&file).extension().and_then(|e| e.to_str()).unwrap_or("mov"));
        run_ffmpeg(sound_arguments(&file, &sounds, fps, a.bool("combineSound"), &tmp), &file)?;
        std::fs::rename(&tmp, &file).map_err(|e| CommandError::new(format!("playblast: {e}")))?;
        return Ok(json!(file));
    }
    let base = match a.get("filename").and_then(Value::as_str) {
        Some(p) if !p.is_empty() => p.to_string(),
        _ => match &replacing {
            Some(r) if !movie => r.clone(),
            _ => std::env::temp_dir().join("playblast").display().to_string(),
        },
    };
    let padding = a.int("framePadding", 4).clamp(1, 16) as usize;
    // -cameraSetup: a procedure run before each set of frames, its camera's name on the end of the file name.
    let setups: Vec<(String, String)> = a.get("cameraSetup").map(flat_values).unwrap_or_default().chunks(2).filter(|c| c.len() == 2).map(|c| (text(&c[0]), text(&c[1]))).collect();
    if a.has("cameraSetup") && setups.is_empty() {
        fail!("playblast: -cameraSetup takes a procedure and a camera name.");
    }
    let passes: Vec<(String, String)> = if setups.is_empty() { vec![(String::new(), base.clone())] } else { setups.iter().map(|(p, cam)| (p.clone(), format!("{base}{cam}"))).collect() };
    let index_from_zero = a.bool("indexFromZero");
    let mut payload_passes = Vec::new();
    let mut outs: Vec<(Option<std::path::PathBuf>, Option<String>, String)> = Vec::new();
    for (k, (proc_name, stem)) in passes.iter().enumerate() {
        // A movie's frames go in a folder of their own, numbered from 0 whatever frames they are, for ffmpeg to read.
        let movie_dir = movie.then(|| std::env::temp_dir().join(format!("aztek_playblast_{}_{k}", std::process::id())));
        let movie_out = movie.then(|| if replacing.is_some() && setups.is_empty() { replacing.clone().unwrap() } else { movie_file_name(stem, &fmt) });
        if let Some(out) = &movie_out {
            if !a.bool("forceOverwrite") && replacing.is_none() && std::path::Path::new(out).exists() {
                fail!("playblast: {out} exists; use -forceOverwrite.");
            }
        }
        let names: Vec<String> = match &movie_dir {
            Some(d) => (0..frames.len()).map(|i| d.join(format!("frame.{i:06}.png")).display().to_string()).collect(),
            None => frames.iter().enumerate().map(|(i, t)| format!("{stem}.{:0padding$}.{compression}", if index_from_zero { i as i64 } else { t.round() as i64 })).collect(),
        };
        if !a.bool("forceOverwrite") && !movie && replacing.is_none() {
            if let Some(n) = names.iter().find(|n| std::path::Path::new(n).exists()) {
                fail!("playblast: {n} exists; use -forceOverwrite.");
            }
        }
        payload_passes.push(json!({"setup": proc_name, "files": names}));
        outs.push((movie_dir, movie_out, format!("{stem}.{}.{compression}", "#".repeat(padding))));
    }
    let wh = a.get("widthHeight").map(|v| v.as_array().map(|l| (l.first().and_then(Value::as_i64).unwrap_or(0), l.get(1).and_then(Value::as_i64).unwrap_or(0))).unwrap_or((0, 0))).unwrap_or((a.int("width", 0), a.int("height", 0)));
    let panel = a.get("editorPanelName").map(text).unwrap_or_else(focus_model_panel);
    let payload = json!({
        "frames": frames, "files": payload_passes[0]["files"], "passes": payload_passes, "width": wh.0, "height": wh.1, "percent": a.float("percent", 50.0),
        "quality": a.int("quality", 90), "panel": panel, "viewer": a.has("viewer") && a.bool("viewer"), "ornaments": !a.has("showOrnaments") || a.bool("showOrnaments"),
        "exposure": a.float("exposure", 0.0), "gamma": a.float("gamma", 1.0),
    });
    // Each frame is set and drawn by the shell. With none there are no pixels to write.
    let written = crate::scriptui::call_hook("playblast", &payload);
    if written.is_none() {
        if throw {
            fail!("playblast: no view to capture without the interface.");
        }
        crate::script_jobs::warn("playblast: no view to capture without the interface; nothing was written.");
    }
    if throw && written.as_ref().and_then(Value::as_array).is_some_and(|w| w.len() < frames.len() * passes.len()) {
        fail!("playblast: not every frame could be written.");
    }
    aztek_scene::anim::set_frame(&mut s.graph, time);
    let mut first: Option<Value> = None;
    for (movie_dir, movie_out, pattern) in outs {
        let (Some(dir), Some(out)) = (movie_dir, movie_out) else {
            first.get_or_insert(json!(pattern));
            continue;
        };
        if written.is_none() {
            first.get_or_insert(json!(out));
            continue;
        }
        let frames_in = dir.join("frame.%06d.png").display().to_string();
        let replace_range = replacing.is_some() && std::path::Path::new(&out).exists();
        let fresh = if replace_range || !sounds.is_empty() { format!("{}.new.{}", dir.display(), std::path::Path::new(&out).extension().and_then(|e| e.to_str()).unwrap_or("mov")) } else { out.clone() };
        let r = (|| -> CmdResult<()> {
            run_ffmpeg(movie_arguments(&frames_in, fps, &fresh), &out)?;
            if replace_range {
                // The frames from the replace start to its end swapped into the movie there: the movie before them, the
                // new frames, the movie after.
                let start = (frames.first().copied().unwrap_or(0.0) - range(s, "minTime", 0.0)).max(0.0) / fps;
                let len = frames.len() as f64 / fps;
                let joined = format!("{fresh}.joined.{}", std::path::Path::new(&out).extension().and_then(|e| e.to_str()).unwrap_or("mov"));
                let filter = format!("[0:v]trim=0:{start},setpts=PTS-STARTPTS[a];[1:v]setpts=PTS-STARTPTS[b];[0:v]trim=start={},setpts=PTS-STARTPTS[c];[a][b][c]concat=n=3:v=1:a=0[v]", start + len);
                let args: Vec<String> = ["-y", "-loglevel", "error", "-i", &out, "-i", &fresh, "-filter_complex", &filter, "-map", "[v]", "-c:v", "libx264", "-pix_fmt", "yuv420p", &joined].iter().map(|x| x.to_string()).collect();
                run_ffmpeg(args, &out)?;
                let _ = std::fs::remove_file(&fresh);
                std::fs::rename(&joined, &fresh).map_err(|e| CommandError::new(format!("playblast: {e}")))?;
            }
            if !sounds.is_empty() {
                run_ffmpeg(sound_arguments(&fresh, &sounds, fps, a.bool("combineSound"), &out), &out)?;
                let _ = std::fs::remove_file(&fresh);
            } else if fresh != out {
                std::fs::rename(&fresh, &out).map_err(|e| CommandError::new(format!("playblast: {e}")))?;
            }
            Ok(())
        })();
        let _ = std::fs::remove_dir_all(&dir);
        r?;
        first.get_or_insert(json!(out));
    }
    Ok(first.unwrap_or(Value::Null))
}

// ---------------------------------------------------------------------------------------------------------------------

pub fn register(s: &mut Session) {
    s.register(
        Cmd::new(
            "excludeObjectDisplayPreset",
            "Named presets of which kinds of object a view leaves out, for modelEditor: -listPresets gives their names, All (every kind shown) and None.",
            |_s, a| {
                if a.bool("listPresets") || a.flags.is_empty() || a.bool("query") {
                    return Ok(serde_json::json!(["All", "None"]));
                }
                Ok(Value::Null)
            },
        )
        .undoable(false)
        .flag(f("listPresets", "", ArgType::Bool))
        .flag(f("query", "q", ArgType::Bool))
        .flag(f("edit", "e", ArgType::Bool)),
    );
    let mut gp = Cmd::new("getPanel", "The panels: -wf in focus, -up under the pointer, -wl by label, -typ of a type, -to the type of one, -vis visible, -all.", |_s, a| get_panel(a)).undoable(false);
    for (n, sh) in [("withLabel", "wl"), ("type", "typ"), ("typeOf", "to"), ("scriptType", "sty"), ("containing", "con"), ("configWithLabel", "cwl")] {
        gp = gp.flag(f(n, sh, ArgType::Any));
    }
    for (n, sh) in [("withFocus", "wf"), ("underPointer", "up"), ("allPanels", "all"), ("visiblePanels", "vis"), ("invisiblePanels", "inv"), ("allTypes", "at"), ("allScriptedTypes", "ast"), ("allConfigs", "ac")] {
        gp = gp.flag(f(n, sh, ArgType::Bool));
    }
    s.register(gp.flag(f("atPosition", "ap", ArgType::Any).multi(2)));

    let spec = "camera:cam:a modelEditor:me:b label:l:a parent:p:a control:ctl:b copy:cp:a createString:cs:b defineTemplate:dt:a docTag:dtg:a editString:es:b exists:ex:b init:in:b isUnique:iu:b menuBarVisible:mbv:b needsInit:ni:b popupMenuProcedure:pmp:a replacePanel:rp:a tearOff:to:b tearOffCopy:toc:a tearOffRestore:tor:b menuBarRepeatLast:mrl:b unParent:up:b useTemplate:ut:a barLayout:bl:b query:q:b edit:e:b queried::a";
    s.register(add_spec(Cmd::new("modelPanel", "A panel holding a view: -q -cam, -e -cam, -me its editor.", |_s, a| panel_common("modelPanel", a)).positional("name").undoable(false), spec));
    for (name, editor) in [("outlinerPanel", "outlinerEditor:oe:b"), ("blendShapePanel", "blendShapeEditor:be:b"), ("posePanel", "poseEditor:pe:b")] {
        let kind: &'static str = name;
        let c = Cmd::new(kind, "A panel holding an editor of its kind: the editor has the panel's name.", move |_s, a| panel_common(kind, a)).positional("name").undoable(false);
        s.register(add_spec(add_spec(c, editor), "label:l:a parent:p:a control:ctl:b copy:cp:a createString:cs:b defineTemplate:dt:a docTag:dtg:a editString:es:b exists:ex:b init:in:b isUnique:iu:b menuBarVisible:mbv:b needsInit:ni:b popupMenuProcedure:pmp:a replacePanel:rp:a tearOff:to:b tearOffCopy:toc:a tearOffRestore:tor:b menuBarRepeatLast:mrl:b unParent:up:b useTemplate:ut:a query:q:b edit:e:b queried::a"));
    }
    s.register(add_spec(Cmd::new("panel", "Any panel: -ex, -l, -ctl, -p.", |_s, a| panel_common("panel", a)).positional("name").undoable(false), "label:l:a parent:p:a control:ctl:b copy:cp:a createString:cs:b defineTemplate:dt:a docTag:dtg:a editString:es:b exists:ex:b init:in:b isUnique:iu:b menuBarVisible:mbv:b needsInit:ni:b popupMenuProcedure:pmp:a replacePanel:rp:a tearOff:to:b tearOffCopy:toc:a tearOffRestore:tor:b menuBarRepeatLast:mrl:b unParent:up:b useTemplate:ut:a query:q:b edit:e:b queried::a"));
    s.register(add_spec(
        Cmd::new("scriptedPanel", "A panel of a scripted type: -typ the type, -l, -p. The type's callbacks build it.", |_s, a| scripted_panel(a)).positional("name").undoable(false),
        "type:typ:a label:l:a parent:p:a control:ctl:b copy:cp:a createString:cs:b defineTemplate:dt:a docTag:dtg:a editString:es:b exists:ex:b init:in:b isUnique:iu:b menuBarVisible:mbv:b needsInit:ni:b popupMenuProcedure:pmp:a replacePanel:rp:a tearOff:to:b tearOffCopy:toc:a tearOffRestore:tor:b menuBarRepeatLast:mrl:b unParent:up:b useTemplate:ut:a query:q:b edit:e:b queried::a",
    ));
    s.register(add_spec(
        Cmd::new(
            "scriptedPanelType",
            "Declare a scripted panel type: -ccb -icb -acb -rcb -dcb -scb -ocb the procedures that build, keep and copy it, -hkc its hotkey \
             context. -q and -e read and change a type.",
            |_s, a| scripted_panel_type(a),
        )
        .positional("name")
        .undoable(false),
        "createCallback:ccb:a initCallback:icb:a addCallback:acb:a removeCallback:rcb:a deleteCallback:dcb:a saveStateCallback:scb:a copyStateCallback:ocb:a hotkeyCtxClient:hkc:a customView:cv:b defineTemplate:dt:a exists:ex:b label:l:a obsolete:o:b retainOnFileOpen:rfo:b unique:u:b useTemplate:ut:a query:q:b edit:e:b queried::a",
    ));

    refresh_cmd(s);
    s.register(
        Cmd::new(
            "saveViewportSettings",
            "Every 3D view keeps its settings as optionVars, as the reference does when it quits: its camera and what its model editor shows.",
            |s, _a| save_viewport_settings(s),
        )
        .undoable(false),
    );

    s.register(add_spec(Cmd::new("modelEditor", "What a model panel shows: -cam, -da, -wos, -grid, -joints, -nurbsCurves, -polymeshes, -locators, -deformers, -ikHandles, -cameras, -manipulators, -dtx and more; -q and -e.", |_s, a| model_editor(a)).positional("panel").undoable(false), EDITOR_FLAGS));

    s.register(
        Cmd::new("lookThru", "Make a view look through a camera: lookThru camera [panel]. -q gives the camera.", |_s, a| {
            let names = a.named();
            let is_panel = |n: &str| with_panels(|p| p.all.iter().any(|x| x.name == n && x.kind == "modelPanel"));
            let panel = names.iter().find(|n| is_panel(n)).cloned();
            let camera = names.iter().find(|n| !is_panel(n)).cloned().or_else(|| a.get("objects").and_then(Value::as_str).map(str::to_string));
            let target = panel.clone().unwrap_or_else(focus_model_panel);
            if a.bool("query") || camera.is_none() {
                return Ok(with_panels(|p| json!(p.all.iter().find(|x| x.name == target).map(|x| x.camera.clone()).unwrap_or_default())));
            }
            let camera = camera.unwrap();
            with_panels(|p| {
                if let Some(x) = p.all.iter_mut().find(|x| x.name == target) {
                    x.camera = camera.clone();
                }
            });
            match panel {
                Some(pn) => panel_set(&pn, "camera", json!(camera)),
                None => push_ui_request(format!("lookThru|{camera}")),
            }
            Ok(Value::Null)
        })
        .positional("camera [panel]")
        .undoable(false)
        .flag(f("query", "q", ArgType::Bool))
        .flag(f("objects", "o", ArgType::String))
        .flag(f("nearClip", "nc", ArgType::Any))
        .flag(f("farClip", "fc", ArgType::Any)),
    );

    s.register(
        Cmd::new("isolateSelect", "Isolate Select on a panel: -state on or off, -addSelected, -removeSelected, -update. -q -state reads it.", |_s, a| {
            let panel = a.pos_str(0).map(str::to_string).unwrap_or_else(focus_model_panel);
            if a.bool("query") {
                return Ok(json!(with_panels(|p| p.isolate.get(&panel).copied().unwrap_or(false))));
            }
            let mut m = Map::new();
            for k in ["state", "addSelected", "removeSelected", "update"] {
                if let Some(v) = a.get(k) {
                    m.insert(k.to_string(), v.clone());
                }
            }
            if a.has("state") {
                let on = a.bool("state");
                with_panels(|p| {
                    p.isolate.insert(panel.clone(), on);
                });
            }
            crate::core::push_ui_request(format!("isolate|{}", Value::Object(m)));
            Ok(Value::Null)
        })
        .positional("panel")
        .undoable(false)
        .flag(f("query", "q", ArgType::Bool))
        .flag(f("state", "s", ArgType::Bool))
        .flag(f("addSelected", "as", ArgType::Bool))
        .flag(f("removeSelected", "rs", ArgType::Bool))
        .flag(f("update", "u", ArgType::Bool))
        .flag(f("loadSelected", "ls", ArgType::Bool))
        .flag(f("addDagObject", "ado", ArgType::Any))
        .flag(f("removeDagObject", "rdo", ArgType::Any))
        .flag(f("viewObjects", "vo", ArgType::Bool)),
    );

    s.register(
        Cmd::new("setFocus", "Give a panel or control the focus.", |_s, a| {
            let Some(n) = a.pos_str(0) else { fail!("setFocus: name it.") };
            if with_panels(|p| {
                let known = p.all.iter().any(|x| x.name == n);
                if known {
                    p.focus = n.to_string();
                }
                known
            }) {
                push_ui_request(format!("panelFocus|{n}"));
            }
            Ok(Value::Null)
        })
        .positional("name")
        .undoable(false),
    );

    s.register(
        Cmd::new("listCameras", "The cameras: -p perspective only, -o orthographic only.", |s, a| {
            let (po, oo) = (a.bool("perspective"), a.bool("orthographic"));
            let g = &s.graph;
            let mut out = Vec::new();
            for id in g.nodes() {
                if g.node(id).is_none_or(|n| n.ty().name != cam::TYPE) {
                    continue;
                }
                let ortho = g.node(id).unwrap().data().bool(cam::ORTHOGRAPHIC);
                if (po && ortho) || (oo && !ortho) {
                    continue;
                }
                let tr = aztek_scene::dag::transform_of(g, id);
                out.push(g.path(tr));
            }
            Ok(json!(out))
        })
        .undoable(false)
        .flag(f("perspective", "p", ArgType::Bool))
        .flag(f("orthographic", "o", ArgType::Bool)),
    );

    s.register(
        Cmd::new("viewFit", "Frame the selection, or -all, in a camera: -f the fraction of the view to fill.", |s, a| {
            let c = camera_of(s, cam_arg_filtered(a).as_deref(), "viewFit")?;
            let prims_selected = !aztek_usd::prims::selected().is_empty();
            let names: Vec<String> = if a.bool("allObjects") { every_object(s) } else {
                let sel: Vec<String> = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
                if sel.is_empty() && !prims_selected { every_object(s) } else { sel }
            };
            // -noChildren: each object by its own shapes, without what is parented under it.
            let names = if a.bool("noChildren") {
                let g = &s.graph;
                names
                    .iter()
                    .filter_map(|n| g.find(n).ok().flatten())
                    .flat_map(|id| if g.node(id).is_some_and(|x| x.ty().is_a("transform")) { aztek_scene::dag::shapes(g, id, false) } else { vec![id] })
                    .map(|id| g.path(id))
                    .collect()
            } else {
                names
            };
            let nodes = bounds(s, names)?;
            let stage = crate::usd_view::bounds(s, a.bool("allObjects"));
            let b = match (nodes, stage) {
                (Some(x), Some(y)) => Some((x.0.min(y.0), x.1.max(y.1))),
                (x, y) => x.or(y),
            };
            if let Some(b) = b {
                fit_camera(s, &c, b, a.float("fitFactor", 1.0))?;
                // -fitClipPlanes: an orthographic camera's near and far planes take in the whole box.
                if a.bool("fitClipPlanes") && c.ortho {
                    let c = camera_of(s, cam_arg_filtered(a).as_deref(), "viewFit")?;
                    let (lo, hi) = b;
                    let depths: Vec<f64> = (0..8)
                        .map(|k| DVec3::new(if k & 1 == 0 { lo.x } else { hi.x }, if k & 2 == 0 { lo.y } else { hi.y }, if k & 4 == 0 { lo.z } else { hi.z }))
                        .map(|p| (p - c.eye).dot(c.forward()))
                        .collect();
                    let (near, far) = depths.iter().fold((f64::MAX, f64::MIN), |(n, f), d| (n.min(*d), f.max(*d)));
                    let pad = ((far - near) * 0.01).max(1e-3);
                    set_plug(s, c.sh, cam::NEAR_CLIP, PlugValue::Float((near - pad).max(1e-3)))?;
                    set_plug(s, c.sh, cam::FAR_CLIP, PlugValue::Float((far + pad).max(near.max(1e-3) + 2.0 * pad)))?;
                }
            }
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("allObjects", "all", ArgType::Bool))
        .flag(f("fitFactor", "f", ArgType::Float))
        .flag(f("fitClipPlanes", "fcp", ArgType::Bool))
        .flag(f("noChildren", "noc", ArgType::Bool))
        .flag(f("animate", "an", ArgType::Bool))
        .flag(f("center", "c", ArgType::Bool))
        .flag(f("namespace", "ns", ArgType::Any))
        .flag(f("panel", "p", ArgType::Any)),
    );

    let mut vs = Cmd::new("viewSet", "Put a camera on an axis, or back: -p -t -f -s -bo -bk -ls -rs, -home, -fit, -fitFactor.", view_set).positional("[camera]");
    for (n, sh) in [("persp", "p"), ("top", "t"), ("front", "f"), ("side", "s"), ("bottom", "bo"), ("back", "b"), ("leftSide", "ls"), ("rightSide", "rs"), ("home", "h"), ("fit", "fit"), ("previousView", "pv"), ("nextView", "nv"), ("viewNegativeX", "vnx"), ("viewNegativeY", "vny"), ("viewNegativeZ", "vnz"), ("viewX", "vx"), ("viewY", "vy"), ("viewZ", "vz"), ("keepRenderSettings", "krs")] {
        vs = vs.flag(f(n, sh, ArgType::Bool));
    }
    s.register(vs.flag(f("animate", "an", ArgType::Bool)).flag(f("fitFactor", "ff", ArgType::Float)).flag(f("namespace", "ns", ArgType::Any)).flag(f("query", "q", ArgType::Bool)).flag(f("queried", "", ArgType::Any)));

    s.register(
        Cmd::new("viewLookAt", "Turn a camera to a point, or to the selection. The camera stays where it is.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "viewLookAt")?;
            let target = match a.get("position").and_then(vec3_of) {
                Some(p) => p,
                None => {
                    let sel: Vec<String> = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
                    match bounds(s, sel)? {
                        Some((lo, hi)) => (lo + hi) * 0.5,
                        None => DVec3::ZERO,
                    }
                }
            };
            aim(s, &c, c.eye, target)?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("position", "pos", ArgType::Vec3)),
    );

    s.register(
        Cmd::new("viewCamera", "Put a camera square on to another: -m the camera to move, -t on its top, -s on its side.", |s, a| {
            let target = camera_of(s, cam_arg(a).as_deref(), "viewCamera")?;
            let Some(mover) = a.get("move").map(text) else { fail!("viewCamera: -move names the camera to move.") };
            let c = camera_of(s, Some(&mover), "viewCamera")?;
            let dist = target.coi;
            let eye = if a.bool("topView") { target.eye + target.axes.y_axis * dist } else { target.eye + target.axes.x_axis * dist };
            aim(s, &c, eye, target.eye)?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("move", "m", ArgType::Any))
        .flag(f("topView", "t", ArgType::Bool))
        .flag(f("sideView", "s", ArgType::Bool)),
    );

    s.register(
        Cmd::new("viewPlace", "Place a camera: -eye, -la a point to look at or -vd a direction to look in, -up, -fov degrees, -o orthographic, -p perspective, -ow its width.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "viewPlace")?;
            if a.has("lookAt") && a.has("viewDirection") {
                fail!("viewPlace: -lookAt and -viewDirection can't both be given.");
            }
            if a.bool("ortho") && a.bool("perspective") {
                fail!("viewPlace: -ortho and -perspective can't both be given.");
            }
            let eye = a.get("eyePoint").and_then(vec3_of).unwrap_or(c.eye);
            let target = match (a.get("lookAt").and_then(vec3_of), a.get("viewDirection").and_then(vec3_of)) {
                (Some(t), _) => t,
                (None, Some(d)) if d.length() > 1e-12 => eye + d.normalize() * c.coi,
                _ => eye + c.forward() * c.coi,
            };
            let up = a.get("upDirection").and_then(vec3_of).unwrap_or_else(world::up);
            put_eye(s, &c, eye)?;
            set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(look_rotation(target - eye, up)))?;
            set_plug(s, c.sh, cam::COI, PlugValue::Float((target - eye).length().max(1e-3)))?;
            if a.bool("ortho") {
                set_plug(s, c.sh, cam::ORTHOGRAPHIC, PlugValue::Bool(true))?;
            }
            if a.bool("perspective") {
                set_plug(s, c.sh, cam::ORTHOGRAPHIC, PlugValue::Bool(false))?;
            }
            if a.has("orthoWidth") {
                set_plug(s, c.sh, cam::ORTHO_WIDTH, PlugValue::Float(a.float("orthoWidth", c.width)))?;
            }
            if a.has("fieldOfView") {
                let hfa = s.graph.node(c.sh).unwrap().data().float(cam::H_APERTURE);
                let fov = a.float("fieldOfView", 54.0).to_radians();
                let fl = hfa * 25.4 * 0.5 / (fov * 0.5).tan().max(1e-6);
                set_plug(s, c.sh, cam::FOCAL_LENGTH, PlugValue::Float(fl))?;
            }
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("eyePoint", "eye", ArgType::Vec3))
        .flag(f("lookAt", "la", ArgType::Vec3))
        .flag(f("upDirection", "up", ArgType::Vec3))
        .flag(f("viewDirection", "vd", ArgType::Vec3))
        .flag(f("animate", "an", ArgType::Bool))
        .flag(f("fieldOfView", "fov", ArgType::Float))
        .flag(f("ortho", "o", ArgType::Bool))
        .flag(f("perspective", "p", ArgType::Bool))
        .flag(f("orthoWidth", "ow", ArgType::Float)),
    );

    s.register(
        Cmd::new("viewHeadOn", "Turn a camera to face the selected faces along their normal, or the selection along the view.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "viewHeadOn")?;
            let frame = aztek_scene::mesh_edit::transform::selection_normal_frame(&mut s.graph);
            let normal = frame.z_axis;
            let sel: Vec<String> = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
            let center = bounds(s, sel)?.map_or(c.center(), |(lo, hi)| (lo + hi) * 0.5);
            let dir = if normal.length_squared() > 0.5 { normal.normalize() } else { c.axes.z_axis };
            aim(s, &c, center + dir * c.coi, center)?;
            Ok(Value::Null)
        })
        .positional("[camera]"),
    );

    s.register(
        Cmd::new("viewClipPlane", "A camera's clip planes: -ncp near, -fcp far, -acp set from the scene. -q reads.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "viewClipPlane")?;
            if a.bool("query") {
                let q = asked(a).unwrap_or_default();
                let d = s.graph.node(c.sh).unwrap().data();
                return Ok(match q.as_str() {
                    "nearClipPlane" => json!(d.float(cam::NEAR_CLIP)),
                    "farClipPlane" => json!(d.float(cam::FAR_CLIP)),
                    "autoClipPlane" | "surfacesOnly" => json!(false),
                    _ => Value::Null,
                });
            }
            if a.has("nearClipPlane") {
                set_plug(s, c.sh, cam::NEAR_CLIP, PlugValue::Float(a.float("nearClipPlane", 0.1)))?;
            }
            if a.has("farClipPlane") {
                set_plug(s, c.sh, cam::FAR_CLIP, PlugValue::Float(a.float("farClipPlane", 10000.0)))?;
            }
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("query", "q", ArgType::Bool))
        .flag(f("edit", "e", ArgType::Bool))
        .flag(f("queried", "", ArgType::Any))
        .flag(f("nearClipPlane", "ncp", ArgType::Any))
        .flag(f("farClipPlane", "fcp", ArgType::Any))
        .flag(f("autoClipPlane", "acp", ArgType::Bool))
        .flag(f("surfacesOnly", "so", ArgType::Bool)),
    );

    s.register(
        Cmd::new("dolly", "Move a camera along its view: -d how far, positive in; -abs sets the distance to its centre; -os scales an orthographic width.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "dolly")?;
            if a.has("orthoScale") || (c.ortho && a.has("distance")) {
                let k = if a.has("orthoScale") { a.float("orthoScale", 1.0) } else { (c.width - a.float("distance", 0.0)).max(1e-3) / c.width.max(1e-6) };
                set_plug(s, c.sh, cam::ORTHO_WIDTH, PlugValue::Float((c.width * k).max(1e-6)))?;
                return Ok(Value::Null);
            }
            let d = a.float("distance", 0.0);
            let center = c.center();
            let (eye, coi) = if a.bool("absolute") { (center - c.forward() * d.max(1e-3), d.max(1e-3)) } else { (c.eye + c.forward() * d, (c.coi - d).max(1e-3)) };
            put_eye(s, &c, eye)?;
            set_plug(s, c.sh, cam::COI, PlugValue::Float(coi))?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("distance", "d", ArgType::Float))
        .flag(f("absolute", "abs", ArgType::Bool))
        .flag(f("relative", "rel", ArgType::Bool))
        .flag(f("dollyTowardsCenter", "dtc", ArgType::Bool))
        .flag(f("orthoScale", "os", ArgType::Float)),
    );

    s.register(
        Cmd::new("orbit", "Turn a camera about its centre: -rv right and up degrees, -ha -va, -pv the point to turn about.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "orbit")?;
            let (mut h, mut v) = (a.float("horizontalAngle", 0.0), a.float("verticalAngle", 0.0));
            if let Some(l) = a.get("rotationAngles").and_then(Value::as_array) {
                h = l.first().and_then(Value::as_f64).unwrap_or(h);
                v = l.get(1).and_then(Value::as_f64).unwrap_or(v);
            }
            let pivot = a.get("pivotPoint").and_then(vec3_of).unwrap_or(c.center());
            let turn = glam::DMat3::from_axis_angle(world::up(), -h) * glam::DMat3::from_axis_angle(c.axes.x_axis, -v);
            let eye = pivot + turn * (c.eye - pivot);
            let fwd = turn * c.forward();
            let up = turn * c.axes.y_axis;
            put_eye(s, &c, eye)?;
            set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(look_rotation(fwd, up)))?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("rotationAngles", "ra", ArgType::Any))
        .flag(f("horizontalAngle", "ha", ArgType::Float))
        .flag(f("verticalAngle", "va", ArgType::Float))
        .flag(f("pivotPoint", "pp", ArgType::Vec3)),
    );

    s.register(add_spec(
        Cmd::new("tumble", "Turn a camera about a point: -aa azimuth, -ea elevation, -ra both, -pp the point.", |s, a| {
            let mut call = Map::new();
            if let Some(c) = cam_arg(a) {
                call.insert("args".to_string(), json!([c]));
            }
            let (mut h, mut v) = (a.float("azimuthAngle", 0.0), a.float("elevationAngle", 0.0));
            if let Some(l) = a.get("rotationAngles").and_then(Value::as_array) {
                h = l.first().and_then(Value::as_f64).unwrap_or(h);
                v = l.get(1).and_then(Value::as_f64).unwrap_or(v);
            }
            call.insert("rotationAngles".to_string(), json!([h, v]));
            if let Some(p) = a.get("pivotPoint") {
                call.insert("pivotPoint".to_string(), p.clone());
            }
            s.run("orbit", Value::Object(call))?;
            Ok(Value::Null)
        })
        .positional("[camera]"),
        "azimuthAngle:aa:a elevationAngle:ea:a localTumble:lt:a pivotPoint:pp:a rotationAngles:ra:a",
    ));

    s.register(
        Cmd::new("roll", "Roll a camera about its view: -d degrees; -abs sets the roll.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "roll")?;
            let r = s.graph.node(c.tr).unwrap().data().vec3(Transform::ROTATE);
            let d = a.float("degree", 0.0);
            let z = if a.bool("absolute") { d } else { r.z + d };
            set_plug(s, c.tr, Transform::ROTATE, PlugValue::Vec3(DVec3::new(r.x, r.y, z)))?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("degree", "d", ArgType::Float))
        .flag(f("absolute", "abs", ArgType::Bool))
        .flag(f("relative", "rel", ArgType::Bool)),
    );

    s.register(
        Cmd::new("track", "Slide a camera sideways: -l left, -r right, -u or -up up, -d down, in units.", |s, a| {
            let c = camera_of(s, cam_arg(a).as_deref(), "track")?;
            let side = a.float("right", 0.0) - a.float("left", 0.0);
            // -upDistance01 (-u) and -upDistance02 (-up) are the same.
            let up = a.float("upDistance01", 0.0) + a.float("upDistance02", 0.0) - a.float("down", 0.0);
            let eye = c.eye + c.axes.x_axis * side + c.axes.y_axis * up;
            put_eye(s, &c, eye)?;
            Ok(Value::Null)
        })
        .positional("[camera]")
        .flag(f("left", "l", ArgType::Float))
        .flag(f("right", "r", ArgType::Float))
        .flag(f("upDistance01", "u", ArgType::Float))
        .flag(f("upDistance02", "up", ArgType::Float))
        .flag(f("down", "d", ArgType::Float)),
    );

    let mut pb = Cmd::new("playblast", "Write the view's frames as an image sequence or a movie: -f name, -fmt image or movie, -st -et or -fr, -wh, -p percent, -fp padding, -fo overwrite, -c png jpg bmp tif, -exp -g exposure and gamma, -cs a setup procedure per camera, -s or -uts sound into a movie (-csd as one track), -rf -rst -ret to replace frames of a movie, -rao its sound only. Returns the file pattern or the movie.", playblast).undoable(false);
    for (n, sh) in [("filename", "f"), ("format", "fmt"), ("startTime", "st"), ("endTime", "et"), ("widthHeight", "wh"), ("width", "w"), ("height", "h"), ("framePadding", "fp"), ("percent", "p"), ("compression", "c"), ("quality", "qlt"), ("frame", "fr"), ("editorPanelName", "epn"), ("sound", "s"), ("sequenceTime", "sqt"), ("completeFilename", "cf"), ("replaceFilename", "rf"), ("replaceStartTime", "rst"), ("replaceEndTime", "ret"), ("exposure", "exp"), ("gamma", "g")] {
        pb = pb.flag(f(n, sh, ArgType::Any));
    }
    for (n, sh) in [("viewer", "v"), ("forceOverwrite", "fo"), ("clearCache", "cc"), ("showOrnaments", "orn"), ("offScreen", "os"), ("offScreenViewportUpdate", "osv"), ("activeEditor", "ae"), ("rawFrameNumbers", "rfn"), ("saveFilmGate", "sfg"), ("indexFromZero", "ifz"), ("options", "o"), ("codecOptions", "co"), ("combineSound", "csd"), ("useTraxSounds", "uts"), ("useSequencerSounds", "uss"), ("replaceAudioOnly", "rao"), ("throwOnError", "toe"), ("query", "q")] {
        pb = pb.flag(f(n, sh, ArgType::Bool));
    }
    pb = pb.flag(f("cameraSetup", "cs", ArgType::Any).multi(2));
    s.register(pb.flag(f("queried", "", ArgType::Any)));
}

/// A positional that names a camera: the panel argument of viewFit is left to -panel.
fn cam_arg_filtered(a: &Args) -> Option<String> {
    if let Some(p) = a.get("panel").map(text) {
        return with_panels(|x| x.all.iter().find(|y| y.name == p).map(|y| y.camera.clone()));
    }
    cam_arg(a)
}
