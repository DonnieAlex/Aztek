//! System commands that need the host: launch, license, linearPrecision, listNodesWithIncorrectNames, loadPrefObjects,
//! and the parts of file and lsThroughFilter that run scripts (file -preSaveScript, -postSaveScript, -command,
//! -exportSnapshotCallback; item filters' -byScript and -secondScript), with delete taking item filters away.

use std::cell::Cell;

use aztek_ops::{execute, fail, ArgType, Args, CallSource, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Value};

use crate::core::{Core, LineKind};

thread_local! {
    static LINEAR_PRECISION: Cell<i64> = const { Cell::new(3) };
    /// Inside file -command's callback, which may itself use file.
    static IN_FILE_CALLBACK: Cell<bool> = const { Cell::new(false) };
}

/// The digits linear fields show after the point, as linearPrecision set it.
pub fn linear_precision() -> i64 {
    LINEAR_PRECISION.with(Cell::get)
}

fn launch(_s: &mut Session, a: &Args) -> CmdResult {
    let pick = ["webPage", "movie", "pdfFile", "directory"].into_iter().find(|f| a.has(f));
    let Some(flag) = pick else { fail!("launch: give -webPage, -movie, -pdfFile or -directory") };
    let target = a.string(flag, "");
    let lower = target.to_ascii_lowercase();
    match flag {
        "movie" if ![".mpg", ".mpeg", ".mp4", ".wmv", ".mov", ".qt"].iter().any(|e| lower.ends_with(e)) => fail!("launch: a movie's name ends in .mpg, .mpeg, .mp4, .wmv, .mov or .qt"),
        "pdfFile" if !lower.ends_with(".pdf") => fail!("launch: a PDF's name ends in .pdf"),
        "movie" | "pdfFile" if !std::path::Path::new(&target).is_file() => fail!("launch: {target} not found"),
        "directory" if !std::path::Path::new(&target).is_dir() => fail!("launch: {target} is not a folder"),
        _ => {}
    }
    // The shell opens it in the system's application for it.
    crate::core::push_ui_request(format!("launch|{target}"));
    Ok(Value::Null)
}

fn license(_s: &mut Session, a: &Args) -> CmdResult {
    let version = format!("Aztek {} ({} compatible)", env!("CARGO_PKG_VERSION"), aztek_core::REFERENCE_VERSION);
    if a.bool("showProductInfoDialog") {
        crate::core::push_ui_request("aboutBox".to_string());
        return Ok(json!(version));
    }
    // The rest are obsolete in the reference; they answer as a local install with nothing borrowed would.
    let obsolete = ["borrow", "info", "isBorrowed", "isExported", "isTrial", "licenseMethod", "productChoice", "return", "showBorrowInfo", "status", "usage"];
    if let Some(f) = obsolete.iter().find(|f| a.bool(f)) {
        crate::native_plugin::defer(format!("// Warning: license: -{f} is obsolete //"), LineKind::Warning);
        return Ok(json!(match *f {
            "isBorrowed" | "isExported" | "isTrial" => "0".to_string(),
            "licenseMethod" => "standalone".to_string(),
            "status" => "licensed".to_string(),
            "productChoice" => "Aztek".to_string(),
            "info" => version,
            _ => String::new(),
        }));
    }
    Ok(json!(version))
}

fn linear_precision_cmd(_s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        return Ok(json!(linear_precision()));
    }
    let Some(n) = a.positional.first().and_then(|v| v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)).or_else(|| v.as_str().and_then(|t| t.parse().ok()))) else { fail!("linearPrecision: give the digits after the point, 0 to 6") };
    LINEAR_PRECISION.with(|c| c.set(n.clamp(0, 6)));
    Ok(Value::Null)
}

/// A name the reference accepts: letters, digits and underscores, not starting with a digit, in each namespace part.
pub fn valid_node_name(name: &str) -> bool {
    !name.is_empty() && name.split(':').all(|p| !p.is_empty() && !p.starts_with(|c: char| c.is_ascii_digit()) && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
}

fn list_incorrect(s: &mut Session, _a: &Args) -> CmdResult {
    let g = &s.graph;
    let bad: Vec<String> = g.nodes().into_iter().filter_map(|id| g.node(id).map(|n| n.name().to_string())).filter(|n| !valid_node_name(n)).collect();
    for b in &bad {
        crate::native_plugin::defer(b.clone(), LineKind::Info);
    }
    Ok(Value::Null)
}

fn load_pref_objects(s: &mut Session, _a: &Args) -> CmdResult {
    crate::sys_prefs::load_pref_objects(s)
}

pub fn register(s: &mut Session) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    s.register(
        CommandDef::new("launch", "Open a web page, movie, PDF or folder in the system's application for it.", launch)
            .undoable(false)
            .flag(f("directory", "dir", ArgType::String, "A folder."))
            .flag(f("movie", "mov", ArgType::String, "A movie: .mpg, .mpeg, .mp4, .wmv, .mov or .qt."))
            .flag(f("pdfFile", "pdf", ArgType::String, "A PDF document."))
            .flag(f("webPage", "web", ArgType::String, "A web page.")),
    );
    let mut lic = CommandDef::new("license", "The application's version; -showProductInfoDialog shows the About box. The other flags are obsolete.", license).undoable(false);
    for (n, sh) in [("borrow", "b"), ("info", "i"), ("isBorrowed", "ib"), ("isExported", "ie"), ("isTrial", "it"), ("licenseMethod", "lm"), ("productChoice", "pc"), ("return", "r"), ("showBorrowInfo", "sbi"), ("showProductInfoDialog", "spi"), ("status", "s"), ("usage", "u")] {
        lic = lic.flag(f(n, sh, ArgType::Bool, if n == "showProductInfoDialog" { "Show the About box." } else { "Obsolete." }));
    }
    s.register(lic);
    s.register(
        CommandDef::new("linearPrecision", "How many digits after the point linear fields show, 0 to 6 (more is taken as 6). Query: how many.", linear_precision_cmd)
            .positional("digits")
            .flag(f("query", "q", ArgType::Bool, "Query mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );
    s.register(CommandDef::new("listNodesWithIncorrectNames", "Print the nodes whose names aren't valid to the Script Editor.", list_incorrect).undoable(false));
    s.register(CommandDef::new("loadPrefObjects", "Read the preference nodes in userPrefObjects.ma in the preferences folder, if it is there. Returns whether it was read.", load_pref_objects));
}

// ---------------------------------------------------------------------------------------------------------------------
// The parts that run scripts

fn on(call: &Value, k: &str) -> bool {
    call.get(k).is_some_and(|v| v.as_bool().unwrap_or(v.as_i64().is_some_and(|i| i != 0)))
}

fn texts(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a.iter().flat_map(|x| texts(Some(x))).collect(),
        _ => Vec::new(),
    }
}

/// Looks a flag up by its long or short name in a raw call.
fn flag<'a>(call: &'a Value, long: &str, short: &str) -> Option<&'a Value> {
    call.get(long).or_else(|| call.get(short)).filter(|v| !v.is_null())
}

fn run_mel(core: &mut Core, src: &str) -> Result<(), String> {
    crate::mel::run_callback(core, src)
}

fn file_intercept(core: &mut Core, call: &Value) -> Option<Result<Value, String>> {
    if on(call, "query") || on(call, "q") {
        return None;
    }
    let pre = flag(call, "preSaveScript", "prs").and_then(Value::as_str).map(str::to_string);
    let post = flag(call, "postSaveScript", "pos").and_then(Value::as_str).map(str::to_string);
    let snap = texts(flag(call, "exportSnapshotCallback", "esc"));
    let cb = aztek_scene::file_more::file_callback();
    let has_cb = !cb.is_empty() && !cb[0].is_empty() && !IN_FILE_CALLBACK.with(Cell::get);
    if pre.is_none() && post.is_none() && snap.is_empty() && !has_cb {
        return None;
    }
    if has_cb {
        IN_FILE_CALLBACK.with(|c| c.set(true));
        let r = run_mel(core, &cb[0]);
        IN_FILE_CALLBACK.with(|c| c.set(false));
        if let Err(e) = r {
            core.push(format!("// Error: {e} //"), LineKind::Error);
        }
    }
    let saving = on(call, "save") || on(call, "s");
    if let (Some(p), true) = (&pre, saving) {
        if let Err(e) = run_mel(core, p) {
            return Some(Err(format!("file: the pre-save script failed: {e}")));
        }
    }
    let r = if snap.len() >= 2 { snapshot_export(core, call, &snap[0], &snap[1]) } else { execute(&mut core.s, "file", call, CallSource::Script).map_err(|e| e.to_string()) };
    if let (Some(p), true, true) = (&post, saving, r.is_ok()) {
        if let Err(e) = run_mel(core, p) {
            core.push(format!("// Error: {e} //"), LineKind::Error);
        }
    }
    Some(r)
}

/// file -exportSnapshotCallback: copies of what is to be exported, made in a namespace of their own; the script runs
/// with them selected; they are written as if they had no namespace, then go.
fn snapshot_export(core: &mut Core, call: &Value, script: &str, ns: &str) -> Result<Value, String> {
    let all = on(call, "exportAll") || on(call, "ea");
    let strict = on(call, "exportSelectedStrict") || on(call, "ess");
    if !all && !strict && !(on(call, "exportSelected") || on(call, "es")) {
        return Err("file: -exportSnapshotCallback goes with -exportAll, -exportSelected or -exportSelectedStrict".into());
    }
    let ns = ns.trim_start_matches(':').to_string();
    if ns.is_empty() {
        return Err("file: -exportSnapshotCallback needs a namespace".into());
    }
    let s = &mut core.s;
    let targets: Vec<String> = if all {
        let g = &s.graph;
        g.roots().iter().filter(|&&r| g.node(r).is_some_and(|n| !aztek_scene::cameras::is_startup(g, r) && n.reference_id() == 0)).map(|&r| g.path(r)).collect()
    } else {
        s.graph.selection().iter().map(|&i| s.graph.path(i)).collect()
    };
    if targets.is_empty() {
        return Err("file: nothing to export".into());
    }
    let had_ns = aztek_scene::ref_scene::namespace_exists(&s.graph, &ns);
    if !had_ns {
        s.run("namespace", json!({"addNamespace": ns.clone()})).map_err(|e| e.to_string())?;
    }
    let was = std::mem::replace(&mut s.graph.current_namespace, ns.clone());
    let dup = s.run("duplicate", json!({"args": targets}));
    s.graph.current_namespace = was;
    let dup = texts(Some(&dup.map_err(|e| e.to_string())?));
    // The copies take the names of what they copy, in their namespace.
    let mut named = Vec::new();
    for (d, t) in dup.iter().zip(targets.iter()) {
        let leaf = t.rsplit('|').next().unwrap_or(t).rsplit(':').next().unwrap_or(t);
        let to = format!("{ns}:{leaf}");
        let r = match s.graph.find(d) {
            Ok(Some(id)) => s.graph.rename(id, &to).ok().map(|_| s.graph.path(id)),
            _ => None,
        };
        named.push(r.unwrap_or_else(|| d.clone()));
    }
    let dup = named;
    let saved: Vec<_> = s.graph.selection().to_vec();
    s.run("select", json!({"args": dup.clone(), "replace": true})).map_err(|e| e.to_string())?;
    let cb = run_mel(core, script);
    let s = &mut core.s;
    let mut export = call.clone();
    if let Some(o) = export.as_object_mut() {
        for k in ["exportSnapshotCallback", "esc", "exportAll", "ea"] {
            o.remove(k);
        }
        if !strict {
            o.insert("exportSelected".into(), json!(true));
        }
        o.insert("relativeNamespace".into(), json!(ns.clone()));
    }
    let r = match cb {
        Ok(()) => execute(s, "file", &export, CallSource::Script).map_err(|e| e.to_string()),
        Err(e) => Err(format!("file: the snapshot callback failed: {e}")),
    };
    // The copies go, and the namespace if it was made for them.
    let still: Vec<String> = dup.into_iter().filter(|d| matches!(s.graph.find(d), Ok(Some(_)))).collect();
    if !still.is_empty() {
        let _ = s.run("delete", json!({"args": still}));
    }
    if !had_ns {
        let _ = s.run("namespace", json!({"removeNamespace": ns, "deleteNamespaceContent": true}));
    }
    s.graph.select(&saved, aztek_scene::graph::SelectMode::Replace);
    r
}

/// lsThroughFilter with filters that run scripts: each script's answer for each item, worked out first.
fn filter_intercept(core: &mut Core, call: &Value) -> Option<Result<Value, String>> {
    let name = call.get("args").and_then(|a| a.get(0).or(Some(a))).and_then(Value::as_str)?.to_string();
    let scripted = crate::script_objects::scripted_filters(&name);
    if scripted.is_empty() {
        return None;
    }
    let attr = crate::script_objects::is_attr_filter(&name);
    let s = &mut core.s;
    let mut items = texts(flag(call, "item", "it"));
    let selected = on(call, "selection") || on(call, "sl");
    if items.is_empty() && (selected || attr) {
        items = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
    }
    if items.is_empty() {
        items = s.graph.nodes().into_iter().map(|i| s.graph.path(i)).collect();
    }
    // What a script sees: node names, or for an attribute filter each node.attribute.
    let mut subjects: Vec<String> = Vec::new();
    if attr {
        for n in &items {
            let list = s.run("listAttr", json!({"args": [n]})).ok().map(|v| texts(Some(&v))).unwrap_or_default();
            subjects.extend(list.into_iter().map(|a| format!("{n}.{a}")));
        }
    } else {
        subjects = items.clone();
    }
    let key = |x: &str| if attr { x.to_string() } else { x.rsplit('|').next().unwrap_or(x).to_string() };
    let mut results = Vec::new();
    let quote = |t: &str| format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\""));
    for (filter, by, second, module) in scripted {
        if !second.is_empty() {
            let passed: Vec<String> = if module.is_empty() {
                let list = subjects.iter().map(|x| quote(x)).collect::<Vec<_>>().join(", ");
                crate::mel::call_for_value(core, &format!("{second}({{{list}}});")).map(|v| texts(Some(&v))).unwrap_or_default()
            } else {
                let arg = serde_json::to_string(&subjects).unwrap_or_default();
                crate::core::python_from_mel(core, &format!("__import__('{module}').{second}({arg})")).map(|v| texts(Some(&v))).unwrap_or_default()
            };
            for x in &subjects {
                results.push(((filter.clone(), key(x)), passed.contains(x)));
            }
            continue;
        }
        for x in &subjects {
            let v = if module.is_empty() {
                let args = if attr {
                    let (n, a) = x.rsplit_once('.').unwrap_or((x, ""));
                    format!("{}, {}", quote(n), quote(a))
                } else {
                    quote(x)
                };
                crate::mel::call_for_value(core, &format!("{by}({args});"))
            } else {
                let args = if attr {
                    let (n, a) = x.rsplit_once('.').unwrap_or((x, ""));
                    format!("{}, {}", serde_json::to_string(n).unwrap_or_default(), serde_json::to_string(a).unwrap_or_default())
                } else {
                    serde_json::to_string(x).unwrap_or_default()
                };
                crate::core::python_from_mel(core, &format!("__import__('{module}').{by}({args})"))
            };
            let pass = v.is_ok_and(|v| v.as_bool().unwrap_or(v.as_i64().is_some_and(|i| i != 0) || v.as_f64().is_some_and(|f| f != 0.0)));
            results.push(((filter.clone(), key(x)), pass));
        }
    }
    crate::script_objects::set_script_pass(results);
    let r = execute(&mut core.s, "lsThroughFilter", call, CallSource::Script).map_err(|e| e.to_string());
    crate::script_objects::set_script_pass(Vec::new());
    Some(r)
}

/// delete given item filters: they go, and the rest of the call runs as usual.
fn delete_intercept(core: &mut Core, call: &Value) -> Option<Result<Value, String>> {
    let args = texts(call.get("args"));
    if !args.iter().any(|a| crate::script_objects::is_filter(a)) {
        return None;
    }
    let rest: Vec<String> = args.into_iter().filter(|a| !crate::script_objects::delete_filter(a)).collect();
    if rest.is_empty() {
        return Some(Ok(Value::Null));
    }
    let mut c = call.clone();
    c["args"] = json!(rest);
    Some(execute(&mut core.s, "delete", &c, CallSource::Script).map_err(|e| e.to_string()))
}

pub(crate) fn intercept(core: &mut Core, name: &str, call: &Value) -> Option<Result<Value, String>> {
    match name {
        "file" => file_intercept(core, call),
        "lsThroughFilter" => filter_intercept(core, call),
        "delete" => delete_intercept(core, call),
        _ => None,
    }
}
