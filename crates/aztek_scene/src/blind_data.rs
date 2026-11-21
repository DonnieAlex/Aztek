//! blindDataType: a blind data type as a blindDataTemplate node, its typeId and one attribute for each piece of data it
//! holds, each added as addAttr adds one so scene files keep it. polyBlindData then stores data of the type on components.

use std::sync::Arc;

use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::history_qe::{f, Cmd};
use crate::node::{types, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::rig::plug;
use crate::session::Session;

const TYPE_ID: usize = 0;
/// Each attribute's long name, short name and data type, "ln:sn:type" separated by spaces, in the order made.
const LAYOUT: usize = 1;

const KINDS: &[&str] = &["int", "float", "double", "string", "boolean", "binary"];

fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let store = flags::INPUT | flags::STORABLE;
        types::register_type(NodeType {
            name: "blindDataTemplate".into(),
            icon: "database".into(),
            doc: "A blind data type: its id and the data it holds, one attribute each.".into(),
            own_attrs: vec![
                AttrDef { name: "typeId".into(), short_name: "tid".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: store, doc: "The type's id.".into(), ..Default::default() },
                AttrDef { name: "bdUserInfo".into(), short_name: "bdui".into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: store | flags::HIDDEN, doc: "Each attribute's long name, short name and data type.".into(), ..Default::default() },
            ],
            create: Some(Arc::new(|| Box::new(crate::node::Plain))),
            ..Default::default()
        })
        .expect("blindDataTemplate");
    });
}

/// The template node of a type id.
pub fn template_of(g: &Graph, id: i64) -> Option<NodeId> {
    g.nodes().into_iter().find(|&n| g.node(n).is_some_and(|x| x.ty().name == "blindDataTemplate" && x.data().int(TYPE_ID) == id))
}

/// (long name, short name, data type) of each attribute of a template.
pub fn layout_of(g: &Graph, n: NodeId) -> Vec<(String, String, String)> {
    let Some(node) = g.node(n) else { return Vec::new() };
    node.data()
        .string(LAYOUT)
        .split_whitespace()
        .filter_map(|e| {
            let mut it = e.split(':');
            Some((it.next()?.to_string(), it.next()?.to_string(), it.next()?.to_string()))
        })
        .collect()
}

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Some(Value::String(s)) => vec![s.clone()],
        _ => Vec::new(),
    }
}

fn blind_data_type(s: &mut Session, a: &Args) -> CmdResult {
    register_types();
    let Some(id) = a.get("typeId").and_then(Value::as_i64) else { fail!("blindDataType: give -id") };
    let (kinds, longs, shorts) = (strings(a.get("dataType")), strings(a.get("longDataName")), strings(a.get("shortDataName")));
    if a.bool("query") {
        let g = &s.graph;
        let Some(n) = template_of(g, id) else {
            return Ok(if a.bool("typeNames") || a.bool("longNames") || a.bool("shortNames") { json!([]) } else { json!("") });
        };
        let wanted = |l: &str, sh: &str| (longs.is_empty() || longs.iter().any(|x| x == l)) && (shorts.is_empty() || shorts.iter().any(|x| x == sh));
        if !(a.bool("typeNames") || a.bool("longNames") || a.bool("shortNames")) {
            return Ok(json!(g.path(n)));
        }
        let mut out = Vec::new();
        for (l, sh, kind) in layout_of(g, n) {
            if !wanted(&l, &sh) {
                continue;
            }
            if a.bool("longNames") {
                out.push(json!(l));
            }
            if a.bool("shortNames") {
                out.push(json!(sh));
            }
            if a.bool("typeNames") {
                out.push(json!(kind));
            }
        }
        return Ok(Value::Array(out));
    }
    if kinds.is_empty() {
        fail!("blindDataType: give -dt, -ldn and -sdn for each attribute");
    }
    if kinds.len() != longs.len() || kinds.len() != shorts.len() {
        fail!("blindDataType: each -dt needs its -ldn and -sdn");
    }
    if let Some(k) = kinds.iter().find(|k| !KINDS.contains(&k.as_str())) {
        fail!("blindDataType: {k} is not int, float, double, string, boolean or binary");
    }
    if template_of(&s.graph, id).is_some() {
        fail!("blindDataType: the type id {id} is already in use");
    }
    let g = &mut s.graph;
    g.open_chunk("blindDataType");
    let r = (|| -> CmdResult<NodeId> {
        let t = types::find("blindDataTemplate").expect("blindDataTemplate");
        let n = g.create_node(&t, "blindDataTemplate1", NO_NODE, true)?;
        g.set_value(plug(n, TYPE_ID), PlugValue::Int(id))?;
        let layout: Vec<String> = kinds.iter().zip(&longs).zip(&shorts).map(|((k, l), sh)| format!("{l}:{sh}:{k}")).collect();
        g.set_value(plug(n, LAYOUT), PlugValue::String(layout.join(" ")))?;
        Ok(n)
    })();
    g.close_chunk();
    let n = r?;
    let path = s.graph.path(n);
    for ((k, l), sh) in kinds.iter().zip(&longs).zip(&shorts) {
        let mut call = json!({"args": [path], "longName": l, "shortName": sh});
        match k.as_str() {
            "int" => call["attributeType"] = json!("long"),
            "float" => call["attributeType"] = json!("float"),
            "double" => call["attributeType"] = json!("double"),
            "boolean" => call["attributeType"] = json!("bool"),
            _ => call["dataType"] = json!("string"),
        }
        s.run("addAttr", call)?;
    }
    Ok(json!(path))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        Cmd::new(
            "blindDataType",
            "Create a blind data type as a blindDataTemplate node: -id, then -dt -ldn -sdn for each attribute (largest types first). A type can't be \
             changed once made. -q -id returns its node, with -tn, -ln, -sn its attributes' types and names (one attribute with -ldn or -sdn).",
            blind_data_type,
        )
        .flag(f("typeId", "id", ArgType::Int, "The type's id."))
        .flag(f("dataType", "dt", ArgType::Any, "An attribute's type: int, float, double, string, boolean or binary. Repeat it.").multi(1))
        .flag(f("longDataName", "ldn", ArgType::Any, "An attribute's long name. Repeat it.").multi(1))
        .flag(f("shortDataName", "sdn", ArgType::Any, "An attribute's short name. Repeat it.").multi(1))
        .flag(f("query", "q", ArgType::Bool, "Ask about the type rather than make it."))
        .flag(f("typeNames", "tn", ArgType::Bool, "With -q: the attributes' types."))
        .flag(f("longNames", "ln", ArgType::Bool, "With -q: the attributes' long names."))
        .flag(f("shortNames", "sn", ArgType::Bool, "With -q: the attributes' short names.")),
    );
}
