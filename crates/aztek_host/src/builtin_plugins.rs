//! Plug-ins that come with Aztek: features kept apart from the core, each registered when its plug-in loads and gone
//! when it unloads. They load at start unless turned off in the Plug-in Manager, and answer to loadPlugin, unloadPlugin
//! and pluginInfo by name like any other.

use std::cell::RefCell;

use aztek_scene::session::Session;
use serde_json::{json, Value};

pub struct Builtin {
    pub name: &'static str,
    pub doc: &'static str,
    /// The commands it registers, which unloading takes away.
    pub commands: &'static [&'static str],
    pub register: fn(&mut Session),
    /// Commands it registers besides, listed where they are made.
    pub more: fn() -> Vec<&'static str>,
    /// The node types it stands for, as pluginInfo -dependNode lists them.
    pub nodes: &'static [&'static str],
}

fn none() -> Vec<&'static str> {
    Vec::new()
}

impl Builtin {
    /// Every command it registers.
    pub fn all_commands(&self) -> Vec<&'static str> {
        let mut v = self.commands.to_vec();
        v.extend((self.more)());
        v
    }
}

pub const VENDOR: &str = "Aztek";
/// The folder the Plug-in Manager lists them under.
pub const FOLDER: &str = "Aztek (built in)";

pub const BUILTINS: &[Builtin] = &[
    // The reference keeps these nodes in plug-ins of these names, and tools load them by name before they make the
    // nodes. Aztek's are part of the core: the names are here so that loading them succeeds.
    Builtin { name: "matrixNodes", doc: "The matrix utility nodes: decomposeMatrix, composeMatrix, multMatrix, inverseMatrix, wtAddMatrix and the rest. Part of Aztek's core; listed so that tools which load matrixNodes find it.", commands: &[], register: |_| {}, more: none, nodes: &[] },
    Builtin { name: "quatNodes", doc: "The quaternion utility nodes: quatAdd, quatConjugate, quatInvert, quatNegate, quatNormalize, quatProd, quatSlerp, quatSub, quatToEuler and eulerToQuat. Part of Aztek's core; listed so that tools which load quatNodes find it.", commands: &[], register: |_| {}, more: none, nodes: &[] },
    Builtin {
        name: "fbx",
        doc: "FBX import and export: File > Import and File > Export with a .fbx name, file -import/-export -type \"FBX\", importFbx, exportFbx, FBXImport, FBXExport and the FBXImport and FBXExport settings commands.",
        commands: &["exportFbx", "FBXExport", "importFbx", "FBXImport"],
        register: |s| {
            aztek_scene::fbx_export::register(&mut s.commands);
            aztek_scene::fbx_import::register(&mut s.commands);
        },
        more: aztek_scene::fbx_options::command_names,
        nodes: &[],
    },
    Builtin {
        name: "objExport",
        doc: "OBJ import and export, as the reference's objExport plug-in: the OBJ and OBJexport file types (file -import, -open, -exportAll and -exportSelected with -type \"OBJ\" or \"OBJexport\", or a .obj name), importObj and exportObj.",
        commands: &["importObj", "exportObj"],
        register: |s| aztek_scene::mesh_nodes::register_obj(&mut s.commands),
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "mayaUsdPlugin",
        doc: "USD, as the reference's mayaUsdPlugin: the USD Import and USD Export file types (file with -type \"USD Import\" or \"USD Export\", or a .usd, .usda, .usdc or .usdz name), mayaUSDImport, mayaUSDExport, the proxy shape and the stage, layer and prim commands.",
        commands: &[],
        register: register_usd,
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "gltf",
        doc: "glTF 2.0 import and export: File > Import and File > Export with a .glb or .gltf name, importGltf and exportGltf.",
        commands: &["exportGltf", "importGltf"],
        register: |s| {
            aztek_scene::gltf_export::register(&mut s.commands);
            aztek_scene::gltf_import::register(&mut s.commands);
        },
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "AbcExport",
        doc: "Alembic export, as the reference's AbcExport plug-in: AbcExport -j with its job strings, writing Ogawa files.",
        commands: &["AbcExport"],
        register: |s| aztek_scene::alembic::register_export(&mut s.commands),
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "gpuCache",
        doc: "GPU caches, as the reference's gpuCache plug-in: the gpuCache command writes objects' geometry over a frame range to Alembic files, and the gpuCache shape plays such a file back in the views at the scene's time, a light stand-in for a heavy rig.",
        commands: &["gpuCache"],
        register: |s| aztek_scene::gpu_cache::register(&mut s.commands),
        more: none,
        nodes: &["gpuCache"],
    },
    Builtin {
        name: "AbcImport",
        doc: "Alembic import, as the reference's AbcImport plug-in: AbcImport and the AlembicNode that streams an animated file.",
        commands: &["AbcImport"],
        register: |s| aztek_scene::alembic::register_import(&mut s.commands),
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "xjenToolkit",
        doc: "XJEN hair, as the reference's xgenToolkit: Interactive Groom descriptions (xgmSplineDescription, xgmSplineBase, xgmModifierScale, xgmModifierSculpt) drawn in the views and rendered by Harold, Generate > Create Interactive Groom Splines (xjenCreateGroom) and xgmSplineQuery.",
        commands: aztek_scene::xjen::cmds::NAMES,
        register: |s| {
            aztek_scene::xjen::cmds::register(&mut s.commands);
            aztek_scene::xjen::register_strands();
        },
        more: none,
        nodes: &[],
    },
    Builtin {
        name: "mayaHIK",
        doc: "HumanIK, as the reference's mayaHIK plug-in (#167): the character definitions, the full body solve, retargeting between characters, the control rig and the bakes, with the hik commands the HumanIK window and scripts use.",
        commands: aztek_scene::hik_rig::NAMES,
        register: |s| aztek_scene::hik::register(&mut s.commands),
        more: none,
        nodes: &["HIKCharacterNode", "HIKSolverNode", "HIKState2SK", "HIKProperty2State"],
    },
];

/// The reference's name for it, which scene files require.
pub const XGEN_NAME: &str = "xgenToolkit";

/// USD's commands; the wrappers it puts round file, select, ls, move, rotate and scale go in once, and stay.
fn register_usd(s: &mut Session) {
    aztek_usd::install_commands(s);
    if !USD_WRAPPED.with(|w| w.replace(true)) {
        aztek_usd::install_wrappers(s);
    }
}

/// The names the two went by when they only exported.
const OLD_NAMES: &[(&str, &str)] = &[("fbxExport", "fbx"), ("gltfExport", "gltf"), ("fbxmaya", "fbx"), (XGEN_NAME, "xjenToolkit")];

thread_local! {
    static LOADED: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
    /// The commands each one added when it loaded, which unloading takes away.
    static ADDED: RefCell<Vec<(&'static str, Vec<String>)>> = const { RefCell::new(Vec::new()) };
    static USD_WRAPPED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn find(name: &str) -> Option<&'static Builtin> {
    let name = OLD_NAMES.iter().find(|o| o.0 == name).map_or(name, |o| o.1);
    BUILTINS.iter().find(|b| b.name == name)
}

pub fn is_loaded(name: &str) -> bool {
    let Some(b) = find(name) else { return false };
    LOADED.with(|l| l.borrow().contains(&b.name))
}

pub fn loaded() -> Vec<String> {
    LOADED.with(|l| l.borrow().iter().map(|n| n.to_string()).collect())
}

pub fn load(s: &mut Session, b: &'static Builtin) {
    if is_loaded(b.name) {
        return;
    }
    let before: std::collections::HashSet<String> = s.commands.all().map(|d| d.name.clone()).collect();
    (b.register)(s);
    let added: Vec<String> = s.commands.all().map(|d| d.name.clone()).filter(|n| !before.contains(n)).collect();
    ADDED.with(|a| a.borrow_mut().push((b.name, added)));
    LOADED.with(|l| l.borrow_mut().push(b.name));
}

pub fn unload(s: &mut Session, b: &'static Builtin) {
    if b.name == "xjenToolkit" {
        aztek_scene::xjen::unregister_strands();
    }
    for c in b.all_commands() {
        s.commands.unregister(c);
    }
    let added = ADDED.with(|a| {
        let mut a = a.borrow_mut();
        let i = a.iter().position(|(n, _)| *n == b.name);
        i.map(|i| a.remove(i).1).unwrap_or_default()
    });
    for c in added {
        s.commands.unregister(&c);
    }
    LOADED.with(|l| l.borrow_mut().retain(|n| *n != b.name));
}

fn off_file() -> std::path::PathBuf {
    crate::user_folder::prefs_base().join("builtinPlugins.json")
}

/// The ones turned off: every other loads at start.
fn off_list() -> Vec<String> {
    std::fs::read_to_string(off_file())
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v["off"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()))
        .unwrap_or_default()
}

pub fn autoload(name: &str) -> bool {
    let name = find(name).map_or(name, |b| b.name);
    !off_list().iter().any(|n| find(n).map_or(n.as_str(), |b| b.name) == name)
}

pub fn set_autoload(name: &str, on: bool) -> std::io::Result<()> {
    let name = find(name).map_or(name, |b| b.name);
    let mut off = off_list();
    off.retain(|n| find(n).map_or(n.as_str(), |b| b.name) != name);
    if !on {
        off.push(name.to_string());
    }
    std::fs::create_dir_all(crate::user_folder::prefs_base())?;
    std::fs::write(off_file(), json!({"off": off}).to_string())
}

/// At start: every one that has not been turned off.
pub fn load_defaults(s: &mut Session) {
    for b in BUILTINS {
        if autoload(b.name) {
            load(s, b);
        }
    }
}

/// The Plug-in Manager's rows for them.
pub fn rows() -> Vec<Value> {
    BUILTINS
        .iter()
        .map(|b| json!({"name": b.name, "file": b.name, "path": b.name, "folder": FOLDER, "kind": "aztek", "loaded": is_loaded(b.name), "autoload": autoload(b.name), "note": b.doc}))
        .collect()
}
