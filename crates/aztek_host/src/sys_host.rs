//! The reference's system commands that need the host: callbacks (hooks scripts and plug-ins extend the interface at),
//! commandLogging (the commands run, kept in memory and written to a log), crashInfo (the files the last crash left) and
//! appHome (the welcome screen, which is Aztek's application home).

use std::collections::VecDeque;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Value};

use crate::core::{push_ui_request, Core};

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Int as I, List as L, String as S};

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true" | "yes" | "1")))
}

fn asked(a: &Args, k: &str) -> bool {
    a.get("queried").and_then(Value::as_array).is_some_and(|v| v.iter().any(|x| x.as_str() == Some(k))) || a.bool(k)
}

// ---------------------------------------------------------------------------------------------------------------------
// callbacks

/// (hook, owner, callback): a MEL procedure's name or a Python callable.
static CALLBACKS: Mutex<Vec<(String, String, Value)>> = Mutex::new(Vec::new());

/// The standard hooks: name, arguments, what a callback returns.
const HOOKS: &[(&str, &str, &str)] = &[
    ("hyperShadePanelBuildCreateMenu", "", "adds items to the create menu, ending with a divider"),
    ("hyperShadePanelBuildCreateSubMenu", "", "string: a classification"),
    ("hyperShadePanelPluginChange", "string $classification, string $changeType", "int"),
    ("renderNodeClassification", "", "string: a classification"),
    ("provideClassificationStrings", "", "string: classifications separated by colons"),
    ("provideClassificationExclusionStrings", "string $classification", "string: classifications separated by colons"),
    ("provideClassificationStringsForFilteredTreeLister", "string $currentFilterString", "string: classifications separated by white space"),
    ("nodeCanBeUsedAsMaterial", "string $nodeId, string $nodeOwner", "int"),
    ("createRenderNodeSelectNodeCategories", "string $flag, string $treeLister", ""),
    ("createRenderNodePluginChange", "string $classification", "int"),
    ("createRenderNodeCommand", "string $postCommand, string $type", "string: a MEL command that creates the node"),
    ("buildRenderNodeTreeListerContent", "string $renderNodeTreeLister, string $postCommand, string $filterString", ""),
    ("AETemplateCustomContent", "string $nodeName", ""),
    ("provideAETemplateForNodeType", "string $nodeType", "string: a template command or procedure name"),
    ("AEnewMultiHandler", "string $nodeName, string $attributeName, string $uiName, string $changedCommand, string $elementIndexString", "string"),
    ("AEreplaceMultiHandler", "string $layoutName, string $nodeName, string $attributeName, string $changedCommand, string $elementIndexString", "int"),
    ("AEnewAttributeHandler", "string $nodeName, string $attributeName, string $uiName, string $changedCommand", "string"),
    ("AEreplaceAttributeHandler", "string $nodeName, string $attributeName, string $changedCommand", "int"),
    ("addHeaderContentToMayaLambertianShadersAE", "string $nodeName", ""),
    ("updateAE", "", ""),
    ("firstConnectedShader", "string $nodeName", "string"),
    ("allConnectedShaders", "string $nodeName", "string: shaders separated by colons"),
    ("provideNodeToAttrConnection", "string $nodeType, string $attributeType", "string"),
    ("provideNodeToNodeConnection", "string $srcType, string $dstType", "string: source and destination attributes separated by colons"),
    ("provideOutputAttributeNameForTextureNode", "string $nodeName", "string"),
    ("connectNodeToNodeOverrideCallback", "string $srcNode, string $dstNode", "int"),
    ("preventMaterialDeletionFromCleanUpSceneCommand", "string $shader, string $plug, string $connection", ""),
    ("addBakingMenuItems", "string $menuItemAnchor", ""),
    ("addVertexBakingMenuItems", "", ""),
    ("addPrelightMenuItems", "", ""),
    ("addRMBBakingMenuItems", "string $objectName", ""),
    ("addRMBNewMaterialMenuItems", "string $objectName", ""),
    ("addRMBExistingMaterialMenuItems", "string $objectName", ""),
    ("addRMBEditMaterialMenuItems", "string $objectName", ""),
    ("addMayaMuscleMenuItems", "string $menuItemAnchor", ""),
    ("addMayaMuscleShelfButtons", "", ""),
    ("addBackburnerRendererMenuItems", "", "a menu item named the renderer's short name"),
    ("addItemsToHypergraphNodePopupMenu", "string $nodeName", ""),
    ("addItemsToOutlinerNodePopupMenu", "string $nodeName", ""),
    ("addItemsToRenderLayerEditorPopupMenu", "string $layerName", ""),
    ("addItemsToRMBMenuOfTreeLister", "string $nodeType, string $scriptCommand", "int"),
    ("addItemToFileMenu", "", ""),
    ("addItemToCreateLightMenu", "", ""),
    ("renderLayerPresetMenu", "string $nodeName", ""),
    ("addMayaRenderingPreferences", "", ""),
    ("updateMayaRenderingPreferences", "", ""),
    ("renderSettingsBuilt", "", ""),
    ("rendererAddOneTabToGlobalsWindowCreateProc", "string $createProc", ""),
    ("enableRenderPassMenuOfRenderView", "", "int"),
    ("addItemsToRenderPassMenuOfRenderView", "", ""),
    ("prepareRenderChanged", "", ""),
    ("textureReload", "string $file", ""),
    ("saveCustomNodePresetAttributes", "string $presetNodeToSave", "string: a procedure name"),
    ("shouldEarlyReturnFromUpdateMultiCameraBufferNamingMenu", "", "string: \"true\" to return early"),
    ("shouldEarlyReturnFromUpdateMayaSoftwareImageFormatControl", "", "string: \"true\" to return early"),
    ("shouldEarlyReturnFromUpdateDefaultTraversalSetMenu", "", "string: \"true\" to return early"),
    ("shouldEarlyReturnFromShouldAppearInNodeCreateUI", "", "string: \"true\" to return early"),
    ("sequencerAddMenus", "string $editor", ""),
    ("sequencerAddToolButtons", "string $editor", ""),
    ("sequencerAddRMBMenuItems", "string $editor, string $parent_menu, string $context, string $id, string $track_type", ""),
];

fn shown(cb: &Value) -> String {
    match cb {
        Value::String(t) => t.clone(),
        Value::Object(o) => o.get("__pysrc__").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| format!("<python callable {}>", o.get("__pycb__").cloned().unwrap_or(Value::Null))),
        o => o.to_string(),
    }
}

/// Everything but -executeCallbacks, which runs scripts and so is the host's (`execute_callbacks`).
fn callbacks(_s: &mut Session, a: &Args) -> CmdResult {
    let hook = a.get("hook").and_then(Value::as_str).map(str::to_string);
    let owner = a.get("owner").and_then(Value::as_str).map(str::to_string);
    let mut list = CALLBACKS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(cb) = a.get("addCallback").filter(|v| !v.is_null()) {
        let (Some(h), Some(o)) = (&hook, &owner) else { fail!("callbacks: -addCallback needs -hook and -owner") };
        if !list.iter().any(|(x, y, z)| x == h && y == o && z == cb) {
            list.push((h.clone(), o.clone(), cb.clone()));
        }
        return Ok(Value::Null);
    }
    if let Some(cb) = a.get("removeCallback").filter(|v| !v.is_null()) {
        let (Some(h), Some(o)) = (&hook, &owner) else { fail!("callbacks: -removeCallback needs -hook and -owner") };
        list.retain(|(x, y, z)| !(x == h && y == o && shown(z) == shown(cb)));
        return Ok(Value::Null);
    }
    if a.bool("clearAllCallbacks") {
        list.clear();
        return Ok(Value::Null);
    }
    if a.bool("clearCallbacks") {
        let Some(o) = &owner else { fail!("callbacks: -clearCallbacks needs -owner") };
        list.retain(|(x, y, _)| !(y == o && hook.as_ref().is_none_or(|h| h == x)));
        return Ok(Value::Null);
    }
    if a.bool("listCallbacks") {
        let Some(h) = &hook else { fail!("callbacks: -listCallbacks needs -hook") };
        return Ok(json!(list.iter().filter(|(x, y, _)| x == h && owner.as_ref().is_none_or(|o| o == y)).map(|(_, _, z)| shown(z)).collect::<Vec<_>>()));
    }
    if a.bool("dumpCallbacks") {
        return Ok(json!(list.iter().map(|(x, y, z)| format!("{x} {y} {}", shown(z))).collect::<Vec<_>>()));
    }
    if a.bool("describeHooks") {
        return Ok(json!(HOOKS
            .iter()
            .map(|(n, args, ret)| if ret.is_empty() { format!("{n}({args})") } else { format!("{n}({args}) returns {ret}") })
            .collect::<Vec<_>>()));
    }
    if a.bool("executeCallbacks") {
        fail!("callbacks: -executeCallbacks runs in the host");
    }
    fail!("callbacks: nothing to do: -addCallback, -removeCallback, -listCallbacks, -executeCallbacks ...")
}

/// -executeCallbacks -hook name [args...]: each callback on the hook in turn, given the arguments; what each returns, ""
/// for those that return nothing.
pub(crate) fn execute_callbacks(core: &mut Core, call: &Value) -> Option<Result<Value, String>> {
    let o = call.as_object()?;
    if !o.get("executeCallbacks").is_some_and(on) {
        return None;
    }
    let Some(hook) = o.get("hook").and_then(Value::as_str).map(str::to_string) else { return Some(Err("callbacks: -executeCallbacks needs -hook".into())) };
    let owner = o.get("owner").and_then(Value::as_str).map(str::to_string);
    let args: Vec<Value> = o.get("args").and_then(Value::as_array).cloned().unwrap_or_default();
    let cbs: Vec<Value> = CALLBACKS.lock().unwrap_or_else(|e| e.into_inner()).iter().filter(|(h, w, _)| *h == hook && owner.as_ref().is_none_or(|x| x == w)).map(|x| x.2.clone()).collect();
    let mut out = Vec::new();
    for cb in cbs {
        let v = match &cb {
            Value::String(src) => {
                let word = src.trim();
                let plain = !word.is_empty() && word.chars().all(|c| c.is_alphanumeric() || c == '_');
                let text = if plain {
                    let quoted: Vec<String> = args.iter().map(|x| serde_json::to_string(&text_of(x)).unwrap_or_default()).collect();
                    format!("{word}({});", quoted.join(", "))
                } else {
                    src.clone()
                };
                crate::mel::call_for_value(core, &text)
            }
            Value::Object(m) if m.get("__pycb__").is_some() => crate::core::python_value(core, m["__pycb__"].as_i64().unwrap_or(0), &args),
            _ => Ok(Value::Null),
        };
        match v {
            Ok(Value::Null) => out.push(json!("")),
            Ok(x) => out.push(json!(text_of(&x))),
            Err(e) => return Some(Err(format!("callbacks: {e}"))),
        }
    }
    Some(Ok(Value::Array(out)))
}

fn text_of(v: &Value) -> String {
    match v {
        Value::String(t) => t.clone(),
        Value::Bool(b) => (*b as i32).to_string(),
        Value::Null => String::new(),
        o => o.to_string(),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// commandLogging

struct Logging {
    size: usize,
    record: bool,
    to_disk: bool,
    file: Option<PathBuf>,
    history: VecDeque<String>,
}

static LOGGING: Mutex<Logging> = Mutex::new(Logging { size: 200, record: true, to_disk: false, file: None, history: VecDeque::new() });

/// The log file the reference's default names: in the application folder, beside the environment file.
fn default_log_file() -> PathBuf {
    crate::user_folder::applied().root.join("aztekCommandLog.txt")
}

/// A command run, as its MEL: kept in memory and written to the log as asked.
pub(crate) fn log_command(name: &str, call: &Value) {
    if name == "commandLogging" {
        return;
    }
    let mut l = LOGGING.lock().unwrap_or_else(|e| e.into_inner());
    if !l.record && !l.to_disk {
        return;
    }
    let line = crate::mel::echo(name, call);
    if l.record {
        l.history.push_back(line.clone());
        while l.history.len() > l.size {
            l.history.pop_front();
        }
    }
    if l.to_disk {
        let file = l.file.clone().unwrap_or_else(default_log_file);
        if let Ok(mut fh) = std::fs::OpenOptions::new().create(true).append(true).open(file) {
            let _ = writeln!(fh, "{line}");
        }
    }
}

fn command_logging(_s: &mut Session, a: &Args) -> CmdResult {
    let mut l = LOGGING.lock().unwrap_or_else(|e| e.into_inner());
    if a.bool("query") {
        return Ok(if asked(a, "historySize") {
            json!(l.size)
        } else if asked(a, "logCommands") {
            json!(l.to_disk)
        } else if asked(a, "recordCommands") {
            json!(l.record)
        } else if asked(a, "logFile") {
            json!(aztek_core::paths::to_utf8(&l.file.clone().unwrap_or_else(default_log_file)))
        } else {
            fail!("commandLogging: query -historySize, -logCommands, -recordCommands or -logFile")
        });
    }
    if let Some(n) = a.get("historySize").and_then(Value::as_i64) {
        if n < 0 {
            fail!("commandLogging: the history size can't be negative");
        }
        l.size = n as usize;
        while l.history.len() > l.size {
            l.history.pop_front();
        }
    }
    if let Some(v) = a.get("recordCommands") {
        l.record = on(v);
        if !l.record {
            l.history.clear();
        }
    }
    if a.bool("resetLogFile") {
        l.file = None;
    }
    if let Some(p) = a.get("logFile").and_then(Value::as_str) {
        // Logging goes on in the new file: the old one is closed with the line after its last.
        l.file = Some(PathBuf::from(p));
    }
    if let Some(v) = a.get("logCommands") {
        l.to_disk = on(v);
    }
    if l.to_disk {
        let file = l.file.clone().unwrap_or_else(default_log_file);
        if let Some(dir) = file.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Err(e) = std::fs::OpenOptions::new().create(true).append(true).open(&file) {
            fail!("commandLogging: can't write {}: {e}", file.display());
        }
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// crashInfo

/// Where the record of the last crash is kept between sessions.
fn crash_record() -> PathBuf {
    crate::user_folder::prefs_base().join("lastCrash.json")
}

/// Days since 1970 to a civil date.
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + if m <= 2 { 1 } else { 0 }, m, d)
}

/// The scene open now, for the crash record: the thread's own (the core's) first, then the last any core noted.
static SCENE: Mutex<String> = Mutex::new(String::new());

thread_local! {
    static OWN_SCENE: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

pub fn note_scene(path: Option<&str>) {
    let p = path.unwrap_or("");
    OWN_SCENE.with(|s| {
        if *s.borrow() != p {
            *s.borrow_mut() = p.to_string();
        }
    });
    if let Ok(mut s) = SCENE.lock() {
        if *s != p {
            *s = p.to_string();
        }
    }
}

/// On a panic in the application: a crash log in the temporary folder (what went wrong and the commands before it), and
/// a record of it for crashInfo in the next session.
pub fn install_crash_handler() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        let (y, m, d) = civil(secs.div_euclid(86_400));
        let hm = secs.rem_euclid(86_400) / 60;
        let log = std::env::temp_dir().join(format!("aztek.{y:04}{m:02}{d:02}.{:02}{:02}.log", hm / 60, hm % 60));
        if let Ok(mut fh) = std::fs::File::create(&log) {
            let _ = writeln!(fh, "Aztek crashed: {info}");
            let _ = writeln!(fh, "\nThe commands before it:");
            if let Ok(l) = LOGGING.try_lock() {
                for c in &l.history {
                    let _ = writeln!(fh, "{c}");
                }
            }
        }
        let own = OWN_SCENE.try_with(|s| s.try_borrow().map(|x| x.clone()).unwrap_or_default()).unwrap_or_default();
        let scene = if own.is_empty() { SCENE.try_lock().map(|s| s.clone()).unwrap_or_default() } else { own };
        let rec = json!({"crashLog": aztek_core::paths::to_utf8(&log), "savedBeforeCrash": scene, "time": secs});
        let _ = std::fs::create_dir_all(crate::user_folder::prefs_base());
        let _ = std::fs::write(crash_record(), rec.to_string());
        previous(info);
    }));
}

fn crash_info(s: &mut Session, a: &Args) -> CmdResult {
    if !a.bool("query") {
        fail!("crashInfo: query -crashFile, -crashLog or -savedBeforeCrash");
    }
    let rec: Value = std::fs::read_to_string(crash_record()).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null);
    let text = |k: &str| rec.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    Ok(if asked(a, "crashLog") {
        json!(text("crashLog"))
    } else if asked(a, "savedBeforeCrash") {
        json!(text("savedBeforeCrash"))
    } else if asked(a, "crashFile") {
        // The scene kept from the crashed session: the newest autosave made before it, which is what Aztek offers to
        // recover.
        if rec.is_null() {
            json!("")
        } else {
            let when = rec.get("time").and_then(Value::as_i64).unwrap_or(i64::MAX);
            let latest = crate::autosave::latest(s).filter(|p| {
                std::fs::metadata(p).and_then(|m| m.modified()).ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).is_some_and(|d| (d.as_secs() as i64) <= when)
            });
            json!(latest.map(|p| aztek_core::paths::to_utf8(&p)).unwrap_or_default())
        }
    } else {
        fail!("crashInfo: query -crashFile, -crashLog or -savedBeforeCrash")
    })
}

// ---------------------------------------------------------------------------------------------------------------------
// appHome

struct Home {
    visible: bool,
    tab: String,
    instrument: String,
}

static HOME: Mutex<Home> = Mutex::new(Home { visible: false, tab: String::new(), instrument: String::new() });

const TABS: &[&str] = &["Recent", "GettingStarted", "Learning", "WhatsNew", "Community"];

const ICON_VAR: &str = "appHomeIconVisible";

fn app_home(s: &mut Session, a: &Args) -> CmdResult {
    let icon_locked = aztek_core::env::get("MAYA_NO_HOME_ICON").is_some();
    if a.bool("query") {
        return Ok(if asked(a, "iconVisible") {
            json!(!icon_locked && s.option_vars.get(ICON_VAR).map(on).unwrap_or(true))
        } else if asked(a, "visible") {
            json!(HOME.lock().unwrap().visible)
        } else {
            fail!("appHome: query -visible or -iconVisible")
        });
    }
    let mut req = serde_json::Map::new();
    if let Some(i) = a.get("instrument").and_then(Value::as_str) {
        HOME.lock().unwrap().instrument = i.to_string();
    }
    if let Some(v) = a.get("iconVisible") {
        if icon_locked {
            fail!("appHome: the home icon is turned off by MAYA_NO_HOME_ICON and can't be shown");
        }
        s.option_vars.insert(ICON_VAR.into(), json!(on(v) as i32));
        req.insert("iconVisible".into(), json!(on(v)));
    }
    let mut h = HOME.lock().unwrap();
    if let Some(t) = a.get("setTab").and_then(Value::as_str) {
        if !TABS.contains(&t) {
            fail!("appHome: no tab '{t}': {}", TABS.join(", "));
        }
        h.tab = t.to_string();
        req.insert("tab".into(), json!(t));
    }
    if let Some(v) = a.get("visible") {
        h.visible = on(v);
        req.insert("visible".into(), json!(h.visible));
    }
    if a.bool("toggleVisibility") {
        h.visible = !h.visible;
        req.insert("visible".into(), json!(h.visible));
    }
    if a.bool("updateRecentFiles") {
        req.insert("updateRecentFiles".into(), json!(true));
    }
    if !req.is_empty() {
        push_ui_request(format!("appHome|{}", Value::Object(req)));
    }
    Ok(Value::Null)
}

/// The last string appHome was instrumented with.
pub fn app_home_instrument() -> String {
    HOME.lock().map(|h| h.instrument.clone()).unwrap_or_default()
}

pub fn register(s: &mut Session) {
    s.register(
        Cmd::new(
            "callbacks",
            "Callbacks at the interface's hooks, so scripts and plug-ins can extend it: -ac adds one for a -hook and -owner, -rc \
             removes one, -cc clears an owner's, -cac all; -lc lists a hook's, -dc all of them, -dh the standard hooks; -ec runs \
             a hook's callbacks with the arguments given and returns what each returns.",
            callbacks,
        )
        .positional("arguments for -executeCallbacks")
        .undoable(false)
        .flag(f("addCallback", "ac", A, "A callback to add: a MEL procedure's name or a Python callable."))
        .flag(f("removeCallback", "rc", A, "A callback to remove."))
        .flag(f("clearCallbacks", "cc", B, "Remove the -owner's callbacks, on the -hook if one is given."))
        .flag(f("clearAllCallbacks", "cac", B, "Remove every callback of every hook and owner."))
        .flag(f("listCallbacks", "lc", B, "The callbacks on the -hook, the -owner's if one is given."))
        .flag(f("dumpCallbacks", "dc", B, "Every callback: hook, owner and callback."))
        .flag(f("describeHooks", "dh", B, "The standard hooks with their arguments and what they return."))
        .flag(f("executeCallbacks", "ec", B, "Run the -hook's callbacks with the arguments given; what each returns."))
        .flag(f("hook", "h", S, "The hook."))
        .flag(f("owner", "o", S, "Who adds the callback, typically a plug-in.")),
    );
    s.register(
        Cmd::new(
            "commandLogging",
            "The commands run, kept in memory for crash logs (-rc, -hs entries) and written to a log file (-lc, -lf; -rl puts \
             the default file back). -q asks.",
            command_logging,
        )
        .flag(f("historySize", "hs", I, "How many commands memory keeps."))
        .flag(f("recordCommands", "rc", A, "Keep the commands in memory."))
        .flag(f("logCommands", "lc", A, "Write the commands to the log file."))
        .flag(f("logFile", "lf", S, "The log file; logging moves to it."))
        .flag(f("resetLogFile", "rl", B, "Log to the default file, aztekCommandLog.txt in the application folder."))
        .flag(f("query", "q", B, "Query mode."))
        .flag(f("queried", "", L, "Flags asked about in query mode.")),
    );
    s.register(
        Cmd::new("crashInfo", "The files the last crash left: -q -cf the scene kept from it, -cl its log, -sbc the scene open when it happened.", crash_info)
            .flag(f("crashFile", "cf", B, "The scene kept from the crashed session."))
            .flag(f("crashLog", "cl", B, "The crash's log."))
            .flag(f("savedBeforeCrash", "sbc", B, "The scene that was open."))
            .flag(f("query", "q", B, "Query mode."))
            .flag(f("queried", "", L, "Flags asked about in query mode.")),
    );
    s.register(
        Cmd::new(
            "appHome",
            "Show and hide the application home, the welcome screen: -v on or off, -tv toggles, -t a tab (Recent, GettingStarted, \
             Learning, WhatsNew, Community), -urf refreshes its recent files, -iv the status line's home icon. -q asks.",
            app_home,
        )
        .undoable(false)
        .flag(f("visible", "v", A, "Show or hide it."))
        .flag(f("toggleVisibility", "tv", B, "Show it if hidden, hide it if shown."))
        .flag(f("setTab", "t", S, "The tab to show: Recent, GettingStarted, Learning, WhatsNew or Community."))
        .flag(f("updateRecentFiles", "urf", B, "Refresh its recent files."))
        .flag(f("iconVisible", "iv", A, "Show the home icon in the status line (a preference)."))
        .flag(f("instrument", "i", S, "A string to instrument the command with."))
        .flag(f("query", "q", B, "Query mode."))
        .flag(f("edit", "e", B, "Edit mode."))
        .flag(f("queried", "", L, "Flags asked about in query mode.")),
    );
}
