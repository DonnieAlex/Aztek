//! Autosave, as the reference's autoSave command: its settings kept as optionVars (enable, interval, a limit on how
//! many autosaves are kept, where they go, a prompt), and -perform writing a numbered copy of the scene now. The timer
//! that calls it and the recovery offer after a crash are the interface's.

use std::path::{Path, PathBuf};

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, FlagDef};
use aztek_scene::session::Session;
use serde_json::{json, Value};

use crate::ui_cmds::{option_var, set_option_vars};

type Cmd = CommandDef<Session>;

/// The settings: (flag, short, optionVar, default).
const SETTINGS: &[(&str, &str, &str, fn() -> Value)] = &[
    ("enable", "en", "autoSaveEnable", || json!(false)),
    ("interval", "int", "autoSaveInterval", || json!(600.0)),
    ("limitBackups", "lim", "autoSaveLimitBackups", || json!(false)),
    ("maxBackups", "max", "autoSaveMaxBackups", || json!(10)),
    ("destination", "dst", "autoSaveDestination", || json!(0)),
    ("folder", "fol", "autoSaveFolder", || json!("")),
    ("prompt", "prm", "autoSavePrompt", || json!(false)),
];

fn get(flag: &str) -> Value {
    let (_, _, var, def) = SETTINGS.iter().find(|x| x.0 == flag).unwrap();
    option_var(var).unwrap_or_else(def)
}

fn truthy(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0))
}

/// Where autosaves go: the project's autosave folder (0), the folder named (1), or the folder MAYA_AUTOSAVE_FOLDER or
/// AZTEK_AUTOSAVE_FOLDER names (2); the first when the others give nothing.
pub fn folder(s: &mut Session) -> PathBuf {
    match get("destination").as_i64().unwrap_or(0) {
        1 => {
            let f = get("folder").as_str().unwrap_or("").to_string();
            if !f.is_empty() {
                return PathBuf::from(f);
            }
        }
        2 => {
            for v in ["AZTEK_AUTOSAVE_FOLDER", "MAYA_AUTOSAVE_FOLDER"] {
                if let Some(f) = std::env::var_os(v).filter(|f| !f.is_empty()) {
                    return PathBuf::from(f);
                }
            }
        }
        _ => {}
    }
    let root = s.run("workspace", json!({"query": true, "queried": ["rootDirectory"]})).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
    let root = if root.is_empty() { std::env::temp_dir().to_string_lossy().to_string() } else { root };
    Path::new(&root).join("autosave")
}

/// The scene's stem: its file's name without the extension, or "untitled".
fn stem(s: &mut Session) -> String {
    let name = s.run("file", json!({"query": true, "sceneName": true})).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
    Path::new(&name).file_stem().and_then(|x| x.to_str()).filter(|x| !x.is_empty()).unwrap_or("untitled").to_string()
}

/// The autosaves of a stem in a folder, oldest first: stem.0001.ma, stem.0002.ma...
fn existing(dir: &Path, stem: &str) -> Vec<(u32, PathBuf)> {
    let mut out: Vec<(u32, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            let name = p.file_name()?.to_str()?.to_string();
            let rest = name.strip_prefix(&format!("{stem}."))?.strip_suffix(".ma")?;
            Some((rest.parse::<u32>().ok()?, p))
        })
        .collect();
    out.sort_by_key(|x| x.0);
    out
}

/// Writes a numbered copy of the scene now; the scene keeps its own name. Returns the file written.
pub fn perform(s: &mut Session) -> CmdResult<String> {
    let dir = folder(s);
    std::fs::create_dir_all(&dir).map_err(|e| aztek_ops::CommandError::new(format!("autoSave: {}: {e}", dir.display())))?;
    let stem = stem(s);
    let have = existing(&dir, &stem);
    let next = have.last().map_or(1, |x| x.0 + 1);
    let path = dir.join(format!("{stem}.{next:04}.ma"));
    let p = path.to_string_lossy().replace('\\', "/");
    s.run("file", json!({"args": [p.clone()], "exportAll": true, "type": "mayaAscii", "force": true}))?;
    if truthy(&get("limitBackups")) {
        let keep = get("maxBackups").as_i64().unwrap_or(10).max(1) as usize;
        let all = existing(&dir, &stem);
        if all.len() > keep {
            for (_, old) in &all[..all.len() - keep] {
                let _ = std::fs::remove_file(old);
            }
        }
    }
    Ok(p)
}

/// The newest autosave in the folder, of any scene.
pub fn latest(s: &mut Session) -> Option<PathBuf> {
    let dir = folder(s);
    std::fs::read_dir(&dir)
        .ok()?
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "ma"))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .max_by_key(|x| x.0)
        .map(|x| x.1)
}

pub fn register(s: &mut Session) {
    let mut c = Cmd::new(
        "autoSave",
        "Autosave settings, kept between sessions: -enable, -interval (seconds), -limitBackups and -maxBackups, -destination \
         (0 the project's autosave folder, 1 -folder, 2 the folder AZTEK_AUTOSAVE_FOLDER or MAYA_AUTOSAVE_FOLDER names) and \
         -prompt. -perform writes a numbered copy of the scene there now (scene.0001.ma, ...), the scene keeping its own name. \
         -q with a flag reads it; -q -latestFile gives the newest autosave, for recovery.",
        |s, a| {
            let queried: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            let asked = |n: &str| a.bool(n) || queried.iter().any(|x| x == n);
            if a.bool("query") {
                if asked("latestFile") {
                    return Ok(json!(latest(s).map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default()));
                }
                if asked("destinationFolder") {
                    return Ok(json!(folder(s).to_string_lossy().replace('\\', "/")));
                }
                for (flag, ..) in SETTINGS {
                    if asked(flag) {
                        let v = get(flag);
                        return Ok(match *flag {
                            "enable" | "limitBackups" | "prompt" => json!(truthy(&v)),
                            _ => v,
                        });
                    }
                }
                fail!("autoSave: query which setting?");
            }
            let mut set = Vec::new();
            for (flag, _, var, _) in SETTINGS {
                if let Some(v) = a.get(flag) {
                    let v = match *flag {
                        "enable" | "limitBackups" | "prompt" => json!(truthy(v)),
                        "interval" => json!(v.as_f64().unwrap_or(600.0).max(1.0)),
                        "maxBackups" => json!(v.as_i64().unwrap_or(10).max(1)),
                        "destination" => json!(v.as_i64().unwrap_or(0).clamp(0, 2)),
                        _ => v.clone(),
                    };
                    set.push((*var, v));
                }
            }
            if !set.is_empty() {
                set_option_vars(&set);
            }
            if a.bool("perform") {
                return Ok(json!(perform(s)?));
            }
            Ok(Value::Null)
        },
    )
    .undoable(false);
    for &(flag, short, _, _) in SETTINGS {
        let ty = match flag {
            "folder" => ArgType::String,
            "interval" => ArgType::Float,
            "maxBackups" | "destination" => ArgType::Int,
            _ => ArgType::Bool,
        };
        c = c.flag(FlagDef::new(flag, short, ty, Value::Null, "A setting; see the command's description."));
    }
    c = c
        .flag(FlagDef::new("perform", "p", ArgType::Bool, Value::Null, "Autosave now."))
        .flag(FlagDef::new("latestFile", "lf", ArgType::Bool, Value::Null, "With -q: the newest autosave."))
        .flag(FlagDef::new("destinationFolder", "df", ArgType::Bool, Value::Null, "With -q: the folder autosaves go to."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Read a setting."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What -q asks for."));
    s.register(c);
}
