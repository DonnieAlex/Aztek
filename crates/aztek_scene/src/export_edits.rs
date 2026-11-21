//! exportEdits: writes nodes, their connections and the reference edits on referenced nodes to an edits file
//! (editMA), which `file -import` brings into another scene: the nodes as a scene file has them, the edits as
//! `//edit kind "argument" ...` lines that importing applies to the nodes they name. The targets are the selection (-selected: the edits on
//! them, not the nodes), the edits on a reference (-onReferenceNode), or the selected nodes themselves
//! (-exportSelected); the include flags add the animation, constraints, deformers, set driven keys, shaders or the whole
//! network around them, and -includeNode and -excludeNode name nodes. -q lists the nodes that would be considered.

use std::collections::{BTreeSet, HashMap, HashSet};

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::NodeId;
use crate::ref_core::f;
use crate::session::Session;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, String as S};

const EDIT_COMMANDS: &[&str] = &["addAttr", "connectAttr", "deleteAttr", "disconnectAttr", "parent", "setAttr", "lock", "unlock"];

fn texts(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

/// What feeds each node, and what each feeds.
fn links(g: &Graph) -> (HashMap<NodeId, Vec<NodeId>>, HashMap<NodeId, Vec<NodeId>>) {
    let (mut up, mut down): (HashMap<NodeId, Vec<NodeId>>, HashMap<NodeId, Vec<NodeId>>) = Default::default();
    for (dst, src) in g.connections() {
        if dst.node != src.node {
            up.entry(dst.node).or_default().push(src.node);
            down.entry(src.node).or_default().push(dst.node);
        }
    }
    (up, down)
}

fn walk(start: &[NodeId], next: &HashMap<NodeId, Vec<NodeId>>, keep: impl Fn(NodeId) -> bool, through: impl Fn(NodeId) -> bool) -> Vec<NodeId> {
    let mut seen: HashSet<NodeId> = start.iter().copied().collect();
    let mut stack: Vec<NodeId> = start.to_vec();
    let mut out = Vec::new();
    while let Some(n) = stack.pop() {
        for &m in next.get(&n).into_iter().flatten() {
            if seen.insert(m) && keep(m) {
                out.push(m);
                if through(m) {
                    stack.push(m);
                }
            }
        }
    }
    out
}

fn type_of(g: &Graph, id: NodeId) -> String {
    g.node(id).map(|n| n.ty().name.clone()).unwrap_or_default()
}

fn is_a(g: &Graph, id: NodeId, t: &str) -> bool {
    g.node(id).is_some_and(|n| n.ty().is_a(t))
}

/// Nodes that come with animation: curves and the blending nodes between them and what they drive.
fn anim_helper(g: &Graph, id: NodeId) -> bool {
    let t = type_of(g, id);
    t.starts_with("animCurve") || t.starts_with("animBlendNode") || t == "pairBlend" || t == "character" || t == "clipLibrary"
}

fn considered(s: &mut Session, a: &Args) -> CmdResult<(Vec<NodeId>, Vec<NodeId>)> {
    let g = &s.graph;
    // The targets: whose edits go out, and the nodes the include flags start from.
    let mut targets: Vec<NodeId> = Vec::new();
    if a.bool("selected") || a.bool("exportSelected") {
        targets.extend(g.selection().iter().copied());
    }
    for rfn in texts(a.get("onReferenceNode")) {
        let Some(at) = crate::references::entry_named(s, &rfn) else { fail!("exportEdits: {rfn} is not a reference node") };
        let id = s.extras.references.entries[at].id;
        let g = &s.graph;
        targets.extend(g.nodes().into_iter().filter(|&n| g.node(n).is_some_and(|x| x.reference_id() == id)));
    }
    let g = &s.graph;
    let mut export: Vec<NodeId> = Vec::new();
    let add = |v: &mut Vec<NodeId>, id: NodeId| {
        if !v.contains(&id) {
            v.push(id);
        }
    };
    if a.bool("exportSelected") {
        for &t in &targets {
            add(&mut export, t);
        }
        // Default nodes never go out.
        export.retain(|&n| g.node(n).is_some_and(|x| !crate::defaults::is_scene_node(x.name()) && !crate::shading::is_default_node(x.name())));
        return Ok((targets, export));
    }
    let (up, down) = links(g);
    // The shapes of the targets count with them.
    let mut starts: Vec<NodeId> = targets.clone();
    for &t in &targets {
        starts.extend(g.children(t).iter().copied().filter(|&c| g.node(c).is_some_and(|n| n.ty().dag && !n.ty().is_a("transform"))));
    }
    if a.bool("includeAnimation") {
        for n in walk(&starts, &up, |m| anim_helper(g, m) && !type_of(g, m).starts_with("animCurveU"), |m| !type_of(g, m).starts_with("animCurve")) {
            add(&mut export, n);
        }
    }
    if a.bool("includeSetDrivenKeys") {
        for n in walk(&starts, &up, |m| type_of(g, m).starts_with("animCurveU") || type_of(g, m) == "unitConversion" || type_of(g, m) == "blendWeighted", |m| type_of(g, m) != "unitConversion" || true) {
            if type_of(g, n).starts_with("animCurveU") || type_of(g, n) == "blendWeighted" || type_of(g, n) == "unitConversion" {
                add(&mut export, n);
            }
        }
    }
    if a.bool("includeConstraints") {
        for n in walk(&starts, &up, |m| is_a(g, m, "constraint") || type_of(g, m).ends_with("Constraint"), |_| false) {
            add(&mut export, n);
        }
    }
    if a.bool("includeDeformers") {
        for &st in &starts {
            if is_a(g, st, "mesh") || is_a(g, st, "nurbsSurface") || is_a(g, st, "nurbsCurve") {
                for h in crate::history::history_of(g, st) {
                    let t = type_of(g, h);
                    if crate::deform::is_deformer(&t) || is_a(g, h, "geometryFilter") || matches!(t.as_str(), "tweak" | "groupParts" | "groupId" | "skinCluster" | "objectSet") {
                        add(&mut export, h);
                    }
                }
            }
        }
    }
    if a.bool("includeShaders") {
        let sgs = walk(&starts, &down, |m| type_of(g, m) == "shadingEngine", |_| false);
        for sg in &sgs {
            add(&mut export, *sg);
        }
        for n in walk(&sgs, &up, |m| g.node(m).is_some_and(|x| !x.ty().dag && !crate::shading::is_default_node(x.name()) && !crate::defaults::is_scene_node(x.name())), |_| true) {
            add(&mut export, n);
        }
    }
    if a.bool("includeNetwork") {
        let keep = |m: NodeId| g.node(m).is_some_and(|x| !crate::defaults::is_scene_node(x.name()) && !crate::shading::is_default_node(x.name()));
        for n in walk(&starts, &up, keep, |_| true).into_iter().chain(walk(&starts, &down, keep, |_| true)) {
            add(&mut export, n);
        }
    }
    for name in texts(a.get("includeNode")) {
        let Some(id) = g.find(&name)? else { fail!("exportEdits: no node named {name}") };
        add(&mut export, id);
    }
    // Unless -excludeHierarchy: the parents of what goes out and their history go with it.
    if !a.bool("excludeHierarchy") {
        let mut more = Vec::new();
        for &n in &export {
            let mut p = g.parent(n);
            while p != crate::plug::NO_NODE {
                if g.node(p).is_some_and(|x| x.reference_id() == 0) {
                    more.push(p);
                }
                p = g.parent(p);
            }
            if is_a(g, n, "mesh") {
                more.extend(crate::history::history_of(g, n));
            }
        }
        for m in more {
            add(&mut export, m);
        }
    }
    for name in texts(a.get("excludeNode")) {
        let Some(id) = g.find(&name)? else { fail!("exportEdits: no node named {name}") };
        export.retain(|&n| n != id);
    }
    // Referenced nodes are not written: their edits are.
    export.retain(|&n| g.node(n).is_some_and(|x| x.reference_id() == 0));
    Ok((targets, export))
}

/// The edits to write: those on the targets' references that concern the targets, as -editCommand and -isa allow.
fn edits(s: &mut Session, a: &Args, targets: &[NodeId]) -> CmdResult<Vec<crate::references::ExportEdit>> {
    let kinds = texts(a.get("editCommand"));
    for k in &kinds {
        if !EDIT_COMMANDS.contains(&k.as_str()) {
            fail!("exportEdits: {k} is not an edit command: {}", EDIT_COMMANDS.join(", "));
        }
    }
    let set_attrs = a.get("includeSetAttrs").map_or(true, |v| v.as_bool().unwrap_or_else(|| v.as_i64().is_some_and(|i| i != 0)));
    let by_rfn: BTreeSet<usize> = texts(a.get("onReferenceNode")).iter().filter_map(|r| crate::references::entry_named(s, r)).collect();
    let mut entries: BTreeSet<usize> = by_rfn.clone();
    let names: Vec<String> = targets.iter().map(|&t| s.graph.path(t)).collect();
    for n in &names {
        if let Some(at) = crate::references::entry_holding(s, n) {
            entries.insert(at);
        }
    }
    let full: Vec<(String, String)> = targets.iter().map(|&t| (s.graph.full_path(t), s.graph.path(t))).collect();
    let mut out = Vec::new();
    for at in entries {
        for e in crate::references::export_list(s, at) {
            if !kinds.is_empty() && !kinds.iter().any(|k| k == e.command) {
                continue;
            }
            if e.command == "setAttr" && !set_attrs {
                continue;
            }
            // On a reference node: all its edits. On the selection: those about the targets.
            let about = by_rfn.contains(&at)
                || e.sides.iter().any(|(n, _)| {
                    let leaf = n.rsplit('|').next().unwrap_or(n);
                    full.iter().any(|(f, p)| f == n || p == n || p == leaf || f.rsplit('|').next() == Some(leaf))
                });
            if about {
                out.push(e);
            }
        }
    }
    Ok(out)
}

fn quote(t: &str) -> String {
    format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\""))
}

/// The scene file text of the nodes going out, and only theirs: the header, their createNode blocks and the
/// connections that have one of them at an end. References, default nodes and the reference nodes' placeholders stay
/// out, so importing the file brings in no reference of its own.
fn nodes_text(full: &str, names: &HashSet<String>) -> String {
    let mut out = String::new();
    let mut statements: Vec<String> = Vec::new();
    for line in full.lines() {
        if line.starts_with('\t') || line.starts_with(' ') {
            if let Some(last) = statements.last_mut() {
                last.push('\n');
                last.push_str(line);
                continue;
            }
        }
        statements.push(line.to_string());
    }
    let unquote = |t: &str| t.trim_matches(|c| c == '"' || c == ';').to_string();
    let node_of = |plug: &str| {
        let p = unquote(plug);
        let n = p.split('.').next().unwrap_or("").to_string();
        n.rsplit('|').next().unwrap_or(&n).to_string()
    };
    for st in statements {
        let first = st.lines().next().unwrap_or("");
        let words: Vec<&str> = first.split_whitespace().collect();
        let keep = match words.first().copied() {
            Some(w) if w.starts_with("//") => !first.starts_with("// End"),
            Some("requires" | "currentUnit" | "fileInfo") => true,
            Some("createNode") => {
                let name = words.iter().position(|w| *w == "-n" || *w == "-name").and_then(|i| words.get(i + 1)).map(|w| unquote(w)).unwrap_or_default();
                names.contains(&name) && words.get(1) != Some(&"reference")
            }
            Some("connectAttr") => {
                let plain: Vec<&&str> = words.iter().skip(1).filter(|w| !w.starts_with('-')).collect();
                plain.len() >= 2 && {
                    let (a, b) = (node_of(plain[0]), node_of(plain[1]));
                    (names.contains(&a) || names.contains(&b)) && !plain[0].contains(".phl[") && !plain[1].contains(".phl[") && !plain[0].contains("placeHolderList") && !plain[1].contains("placeHolderList")
                }
            }
            _ => false,
        };
        if keep {
            out.push_str(&st);
            out.push('\n');
        }
    }
    out
}

fn export_edits(s: &mut Session, a: &Args) -> CmdResult {
    let mode = ["selected", "exportSelected"].iter().any(|k| a.bool(k)) || a.has("onReferenceNode");
    if !mode {
        fail!("exportEdits: say what to export: -selected, -exportSelected or -onReferenceNode");
    }
    let (targets, nodes) = considered(s, a)?;
    if a.bool("query") {
        let g = &s.graph;
        let mut out: Vec<String> = Vec::new();
        if !a.bool("exportSelected") {
            out.extend(targets.iter().map(|&t| g.path(t)));
        }
        for &n in &nodes {
            let p = g.path(n);
            if !out.contains(&p) {
                out.push(p);
            }
        }
        return Ok(json!(out));
    }
    let Some(path) = a.pos_str(0).map(str::to_string) else { fail!("exportEdits: give the file to write") };
    let ty = a.string("type", "editMA");
    if !matches!(ty.as_str(), "editMA" | "editMB") {
        fail!("exportEdits: -type is editMA or editMB");
    }
    let ext = if ty == "editMB" { "editMB" } else { "editMA" };
    let mut path = std::path::PathBuf::from(&path);
    if path.extension().is_none() {
        path.set_extension(ext);
    }
    if path.is_relative() {
        let base = crate::shading::project_dir(s).map(|p| std::path::PathBuf::from(p).join("scenes")).filter(|p| p.is_dir()).unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        path = base.join(path);
    }
    if path.exists() && !a.bool("force") {
        fail!("exportEdits: {} exists; use -force to write over it", path.display());
    }
    let edit_list = if a.bool("exportSelected") { Vec::new() } else { edits(s, a, &targets)? };
    let file = path.to_string_lossy().to_string();
    let (full, _) = crate::ma_file::write_text(s, &file, Some(&nodes));
    let names: HashSet<String> = nodes.iter().filter_map(|&n| s.graph.node(n).map(|x| x.name().to_string())).collect();
    let mut text = nodes_text(&full, &names);
    if !edit_list.is_empty() {
        // Connections from nodes the file makes: as connections, so the nodes are made with them.
        let made = |plug: &str| {
            let n = plug.split('.').next().unwrap_or("");
            names.contains(n.rsplit('|').next().unwrap_or(n))
        };
        let (conns, rest): (Vec<_>, Vec<_>) = edit_list.iter().partition(|e| e.kind == 5 && e.args.first().is_some_and(|a| made(a)));
        for e in conns {
            text.push_str(&format!("connectAttr {} {};\n", quote(&e.args[0]), quote(&e.args[1])));
        }
        if !rest.is_empty() {
            text.push_str("// Reference edits: each the edit's kind and arguments, as a reference node holds them.\n");
            for e in rest {
                text.push_str(&crate::references::edit_file_line(e.kind, &e.args));
                text.push('\n');
            }
        }
    }
    let leaf = path.file_name().and_then(|x| x.to_str()).unwrap_or("");
    text.push_str(&format!("// End of {leaf}\n"));
    std::fs::write(&path, text).map_err(|e| aztek_ops::CommandError::new(format!("exportEdits: {}: {e}", path.display())))?;
    Ok(json!(file))
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        crate::system_cmds::q_flags(Cmd::new(
            "exportEdits",
            "Write nodes, their connections and the reference edits on referenced nodes to an edits file (editMA) that file \
             -import brings into another scene. Targets: -sel the selection's edits, -orn a reference's, -exs the selected \
             nodes themselves; -ian, -ic, -idf, -sdk, -ish, -inw add what goes with them, -in and -en name nodes, -ec limits \
             the edit commands. Returns the file written; -q the nodes that would be considered.",
            export_edits,
        ))
        .positional("file")
        .undoable(false)
        .flag(f("editCommand", "ec", A, "Only edits of this command: addAttr, connectAttr, deleteAttr, disconnectAttr, parent, setAttr, lock or unlock (all by default).").multi(1))
        .flag(f("excludeHierarchy", "ehr", B, "Leave out the DAG parents and history that would otherwise go with the nodes."))
        .flag(f("excludeNode", "en", A, "A node to keep out.").multi(1))
        .flag(f("exportSelected", "exs", B, "Write the selected nodes, their connections to each other and to nodes not written; include and exclude flags are not used."))
        .flag(f("force", "f", B, "Write over a file there already."))
        .flag(f("includeAnimation", "ian", B, "Add the animation curves and the nodes between them and the targets."))
        .flag(f("includeConstraints", "ic", B, "Add the constraints driving the targets."))
        .flag(f("includeDeformers", "idf", B, "Add the deformers of the targets."))
        .flag(f("includeNetwork", "inw", B, "Add every node connected to the targets."))
        .flag(f("includeNode", "in", A, "A node to add.").multi(1))
        .flag(f("includeSetAttrs", "isa", A, "Whether setAttr edits go out (on)."))
        .flag(f("includeSetDrivenKeys", "sdk", B, "Add the set driven key curves driving the targets."))
        .flag(f("includeShaders", "ish", B, "Add the shading groups and shading networks of the targets."))
        .flag(f("onReferenceNode", "orn", A, "Only the edits this reference node holds.").multi(1))
        .flag(f("selected", "sel", B, "The selection's edits, without the nodes themselves."))
        .flag(f("type", "typ", S, "The file type: editMA (the default) or editMB.")),
    );
}
