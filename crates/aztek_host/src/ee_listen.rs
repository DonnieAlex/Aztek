//! The reference's expressionEditorListen: the Expression Editor asks to hear when a file it handed an outside text editor
//! is saved, when a node's attributes or name change, and when an expression changes. Each is watched on idle and
//! passed to the editor's own procedures (EEgetFileExpression, EErebuildAttrList, EEupdateWindow, EEupdateExprList) when
//! they exist.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::SystemTime;

use aztek_ops::{ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::plug::NodeId;
use aztek_scene::Session;
use serde_json::Value;

#[derive(Default)]
struct Listening {
    /// File: its time of last change when last seen.
    files: BTreeMap<String, Option<SystemTime>>,
    /// Node: its attributes' names when last seen.
    attrs: BTreeMap<NodeId, Vec<String>>,
    /// Expression node: its source when last seen.
    expressions: BTreeMap<NodeId, String>,
    /// Node: its name when last seen.
    names: BTreeMap<NodeId, String>,
}

thread_local! {
    static LISTENING: RefCell<Listening> = RefCell::new(Listening::default());
}

fn modified(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

fn attr_names(s: &Session, id: NodeId) -> Vec<String> {
    s.graph.node(id).map(|n| n.ty().attrs.iter().map(|a| a.name.clone()).collect()).unwrap_or_default()
}

fn source(s: &Session, id: NodeId) -> String {
    aztek_scene::expression::current_source(&s.graph, id)
}

fn name_of(s: &Session, id: NodeId) -> String {
    s.graph.node(id).map(|n| n.name().to_string()).unwrap_or_default()
}

fn mel_text(t: &str) -> String {
    t.replace('\\', "\\\\").replace('"', "\\\"")
}

/// A call of the editor's procedure, when it is there.
fn call(proc_name: &str, args: &[&str]) -> Value {
    let quoted: Vec<String> = args.iter().map(|a| format!("\"{}\"", mel_text(a))).collect();
    Value::String(format!("if (`exists {proc_name}`) {proc_name} {};", quoted.join(" ")))
}

/// What has changed since last seen, as calls to make on idle.
pub(crate) fn due(s: &Session) -> Vec<Value> {
    LISTENING.with(|l| {
        let mut l = l.borrow_mut();
        let mut out = Vec::new();
        for (file, seen) in l.files.iter_mut() {
            let now = modified(file);
            if now.is_some() && now != *seen {
                *seen = now;
                out.push(call("EEgetFileExpression", &[file]));
            }
        }
        l.attrs.retain(|id, _| s.graph.node(*id).is_some());
        for (id, seen) in l.attrs.iter_mut() {
            let now = attr_names(s, *id);
            if now != *seen {
                *seen = now;
                out.push(call("EErebuildAttrList", &[&s.graph.path(*id)]));
            }
        }
        l.expressions.retain(|id, _| s.graph.node(*id).is_some());
        let mut expr_changed = false;
        for (id, seen) in l.expressions.iter_mut() {
            let now = source(s, *id);
            if now != *seen {
                *seen = now;
                expr_changed = true;
            }
        }
        if expr_changed {
            out.push(call("EEupdateExprList", &[]));
        }
        l.names.retain(|id, _| s.graph.node(*id).is_some());
        for (id, seen) in l.names.iter_mut() {
            let now = name_of(s, *id);
            if now != *seen {
                *seen = now;
                out.push(call("EEupdateWindow", &[&s.graph.path(*id), ""]));
            }
        }
        out
    })
}

fn listen(s: &mut Session, a: &Args) -> CmdResult {
    let text = |k: &str| a.get(k).and_then(Value::as_str).map(str::to_string);
    let node = |s: &Session, k: &str| -> CmdResult<Option<NodeId>> {
        let Some(n) = text(k) else { return Ok(None) };
        match s.graph.find(&n)? {
            Some(id) => Ok(Some(id)),
            None => aztek_ops::fail!("expressionEditorListen: no object matches name: {n}"),
        }
    };
    let file = text("listenFile");
    let add_attr = node(s, "listenForAttr")?;
    let add_expr = node(s, "listenForExpression")?;
    let add_name = node(s, "listenForName")?;
    let stop_attr = node(s, "stopListenForAttr")?;
    let stop_expr = node(s, "stopListenForExpression")?;
    let stop_name = node(s, "stopListenForName")?;
    let attrs = add_attr.map(|id| (id, attr_names(s, id)));
    let expr = add_expr.map(|id| (id, source(s, id)));
    let name = add_name.map(|id| (id, name_of(s, id)));
    LISTENING.with(|l| {
        let mut l = l.borrow_mut();
        if let Some(f) = file {
            let now = modified(&f);
            l.files.insert(f, now);
        }
        if let Some((id, v)) = attrs {
            l.attrs.insert(id, v);
        }
        if let Some((id, v)) = expr {
            l.expressions.insert(id, v);
        }
        if let Some((id, v)) = name {
            l.names.insert(id, v);
        }
        if let Some(id) = stop_attr {
            l.attrs.remove(&id);
        }
        if let Some(id) = stop_expr {
            l.expressions.remove(&id);
        }
        if let Some(id) = stop_name {
            l.names.remove(&id);
        }
    });
    Ok(Value::Null)
}

pub fn register(s: &mut Session) {
    let f = |n: &str, sh: &str, doc: &str| FlagDef::new(n, sh, ArgType::String, Value::Null, doc);
    s.register(
        CommandDef::<Session>::new(
            "expressionEditorListen",
            "For the Expression Editor: hear when a file is saved, when a node's attributes or name change, or when an expression \
             changes, and pass it to the editor's procedures.",
            listen,
        )
        .flag(f("listenFile", "lf", "Hear when this file changes: the editor reads it back."))
        .flag(f("listenForAttr", "la", "Hear when this node's attributes change."))
        .flag(f("listenForExpression", "le", "Hear when this expression changes."))
        .flag(f("listenForName", "ln", "Hear when this node is renamed."))
        .flag(f("stopListenForAttr", "sla", "Stop hearing about this node's attributes."))
        .flag(f("stopListenForExpression", "sle", "Stop hearing about this expression."))
        .flag(f("stopListenForName", "sln", "Stop hearing about this node's name.")),
    );
}
