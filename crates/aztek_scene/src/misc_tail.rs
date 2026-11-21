//! Commands of the reference from removeMultiInstance to select that lacked a flag or two, and the new resolutionNode:
//! removeMultiInstance -allChildren and -break, reorderDeformers -name, select -containerCentric.

use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use serde_json::{json, Value};

use crate::history_qe::{f, upsert, Cmd};
use crate::plug::PlugRef;
use crate::session::Session;

pub fn register(c: &mut Commands<Session>) {
    remove_multi_instance(c);
    if let Some(old) = c.find("reorderDeformers").cloned() {
        c.register(upsert(old, f("name", "n", ArgType::String, "Obsolete: not used.")));
    }
    select_cc(c);
    c.register(
        Cmd::new(
            "resolutionNode",
            "Make a render resolution node (width, height, aspect ratios), as createNode can't. Returns its name.",
            resolution_node,
        )
        .flag(f("name", "n", ArgType::String, "The new node's name; a namespace in it is made if missing."))
        .flag(f("parent", "p", ArgType::String, "The DAG parent; a resolution node is not a DAG node, so it is left out."))
        .flag(f("shared", "s", ArgType::Bool, "Shared across files: made only if no node of the name exists, which is returned."))
        .flag(f("skipSelect", "ss", ArgType::Bool, "Leave the selection as it is.")),
    );
}

fn truthy(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0))
}

fn remove_multi_instance(c: &mut Commands<Session>) {
    let cmd = Cmd::new(
        "removeMultiInstance",
        "Remove an element of a multi attribute: 'node.attr[3]'. Connected elements need -b on, which breaks their connections \
         first; -all removes every child of a compound multi's element ('node.group[3]'). Returns true when removed.",
        remove_multi,
    )
    .positional("the element plug")
    .flag(f("allChildren", "all", ArgType::Bool, "Remove every child of the multi parent's element."))
    .flag(f("break", "b", ArgType::Bool, "Break the element's connections first (off: a connected element stays and the command fails)."));
    c.register(cmd);
}

fn remove_multi(s: &mut Session, a: &Args) -> CmdResult {
    let breaks = a.get("break").is_some_and(truthy);
    let mut done = true;
    for path in a.named() {
        let (Some(dot), Some(open)) = (path.find('.'), path.rfind('[')) else { fail!("removeMultiInstance: '{path}' is not an element of a multi") };
        if open < dot || !path.ends_with(']') {
            fail!("removeMultiInstance: '{path}' is not an element of a multi");
        }
        let Ok(index) = path[open + 1..path.len() - 1].parse::<i32>() else { fail!("removeMultiInstance: bad element index in '{path}'") };
        let Some(id) = s.graph.find(&path[..dot])? else { fail!("removeMultiInstance: No object matches name: {}", &path[..dot]) };
        let group = path[dot + 1..open].to_string();
        // The plugs that make up the element: the attribute's, or each child's of a compound multi.
        let n = s.graph.node(id).unwrap();
        let children = n.ty().group_children(&group);
        let plugs: Vec<PlugRef> = if !children.is_empty() && (a.get("allChildren").is_some_and(truthy) || n.ty().attr_index(&group).is_none()) {
            children.into_iter().map(|c| PlugRef::element(id, c as i32, index)).collect()
        } else {
            vec![s.graph.plug(&path)?]
        };
        let connected = plugs.iter().any(|p| s.graph.source(p).is_some() || !s.graph.destinations(p).is_empty());
        if connected && !breaks {
            // Connected and -b not on: left as it is, and the answer is false.
            done = false;
            continue;
        }
        for p in plugs {
            for d in s.graph.destinations(&p) {
                s.graph.disconnect(&d);
            }
            s.graph.remove_element(&p)?;
        }
    }
    Ok(json!(done))
}

fn select_cc(c: &mut Commands<Session>) {
    let Some(old) = c.find("select").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = upsert(
        old,
        f("containerCentric", "cc", ArgType::Bool, "Pick as the views do: a member of a black-boxed container that isn't published picks the container."),
    );
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if !a.bool("containerCentric") {
            return run(s, a);
        }
        let mut b = a.clone();
        let swap = |s: &Session, v: &Value| -> Value {
            let Some(name) = v.as_str() else { return v.clone() };
            // Components stay what they are.
            if name.contains('.') {
                return v.clone();
            }
            match s.graph.find(name) {
                Ok(Some(id)) => {
                    let to = crate::rig_misc::container_centric(&s.graph, id);
                    if to == id {
                        v.clone()
                    } else {
                        json!(s.graph.path(to))
                    }
                }
                _ => v.clone(),
            }
        };
        b.positional = a
            .positional
            .iter()
            .map(|v| match v {
                Value::Array(l) => Value::Array(l.iter().map(|x| swap(s, x)).collect()),
                x => swap(s, x),
            })
            .collect();
        b.flags.remove("containerCentric");
        run(s, &b)
    });
    c.register(cmd);
}

fn resolution_node(s: &mut Session, a: &Args) -> CmdResult {
    let name = a.string("name", "");
    if a.bool("shared") && !name.is_empty() {
        if let Ok(Some(id)) = s.graph.find(&name) {
            return Ok(json!(s.graph.path(id)));
        }
    }
    let mut call = json!({"args": ["resolution"], "skipSelect": a.bool("skipSelect")});
    if !name.is_empty() {
        call["name"] = json!(name);
    }
    let made = s.run("createNode", call)?;
    Ok(match made {
        Value::Array(l) if !l.is_empty() => l[0].clone(),
        x => x,
    })
}
