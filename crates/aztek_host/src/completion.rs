//! The Script Editor's completion index: every command with its flags and help, every MEL procedure, MEL's built-in
//! functions, and the Python API's modules and classes. Built once for the editor and filtered there; this module only
//! reads the registries.

use serde_json::{json, Value};

use crate::core::Core;

fn arg_type(t: aztek_ops::ArgType) -> &'static str {
    match t {
        aztek_ops::ArgType::Bool => "bool",
        aztek_ops::ArgType::Int => "int",
        aztek_ops::ArgType::Float => "float",
        aztek_ops::ArgType::String => "string",
        aztek_ops::ArgType::Vec3 => "vector",
        aztek_ops::ArgType::List => "list",
        aztek_ops::ArgType::Any => "any",
    }
}

/// The command registry as completion needs it: names, positional help, and each flag's long and short names, type and
/// help. A flag reported as `queried` by the shell is left out, as the command reference leaves it out.
pub fn commands(core: &Core) -> Vec<Value> {
    let mut v: Vec<Value> = core
        .s
        .commands
        .all()
        .map(|c| {
            json!({
                "name": c.name,
                "doc": c.doc,
                "positional": c.positional_doc,
                "scripted": c.scripted,
                "flags": c
                    .flags
                    .iter()
                    .filter(|f| f.name != "queried")
                    .map(|f| json!({
                        "name": f.name,
                        "short": f.short_name,
                        "type": arg_type(f.ty),
                        "doc": f.doc,
                        "arity": f.arity,
                        "choices": f.choices,
                    }))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    v.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    v
}

/// Every MEL procedure the editor can name: the bundled scripts' globals, then the session's own, then the built-in
/// functions. A session procedure of the same name wins over the bundled declaration, as sourcing it does.
pub fn procedures(core: &Core) -> (Vec<Value>, Vec<Value>) {
    let (procs, builtins) = crate::mel::completion_names(core);
    let procs: Vec<Value> = procs
        .into_iter()
        .map(|(name, signature, source)| {
            json!({
                "name": name,
                "signature": signature,
                "doc": if source == "session" { "MEL procedure, defined in this session." } else { "MEL procedure, from Aztek's bundled scripts." },
                "source": source,
            })
        })
        .collect();
    let builtins: Vec<Value> = builtins
        .into_iter()
        .map(|n| json!({"name": n, "signature": n, "doc": "MEL built-in function."}))
        .collect();
    (procs, builtins)
}

/// The procedures defined in the session now, for the editor to add to its index between edits.
pub fn live_procedures(core: &Core) -> Value {
    let v: Vec<Value> = crate::mel::global_proc_signatures(core)
        .into_iter()
        .map(|(name, signature)| {
            json!({"name": name, "signature": signature, "doc": "MEL procedure, defined in this session."})
        })
        .collect();
    Value::Array(v)
}

/// The whole host half of the completion index, as the editor's query returns it.
pub fn core_index(core: &Core) -> Value {
    let (procs, builtins) = procedures(core);
    json!({
        "commands": commands(core),
        "procedures": procs,
        "builtins": builtins,
    })
}
