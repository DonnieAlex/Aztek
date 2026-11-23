//! What every deformer command of the reference takes, worked for any deformer here: where the new node goes in the chain
//! (-afterReference, -automatic, -split), -exclusive's partition, -useComponentTags, -includeHiddenSelections, and the
//! queries -components, -selectedComponents, -dumpInfo and -geometryIndices.

use aztek_ops::{fail, objects, Args, CmdResult};
use serde_json::{json, Value};

use crate::components::ComponentMode;
use crate::deform::{self, Place};
use crate::graph::Graph;
use crate::object_sets::Member;
use crate::plug::{NodeId, PlugRef};
use crate::session::Session;

/// Deformers the reference counts as invertible: what -automatic puts a new deformer in front of.
const INVERTIBLE: &[&str] = &["skinCluster", "cluster", "jointCluster", "tweak", "polyTweak"];

/// Where a new deformer goes, the reference's way: -frontOfChain, -before, -after, -afterReference (after on referenced
/// geometry, else before), -automatic (in front when only invertible deformers are there, else before), -split (a branch
/// of its own from the original shape, which here is in front of the chain).
pub(crate) fn placement(g: &Graph, a: &Args, mesh: NodeId) -> Place {
    if a.bool("afterReference") {
        let referenced = g.node(crate::dag::transform_of(g, mesh)).is_some_and(|n| n.reference_id() != 0) || g.node(mesh).is_some_and(|n| n.reference_id() != 0);
        return if referenced { Place::After } else { Place::Before };
    }
    if a.bool("automatic") {
        let chain: Vec<NodeId> = crate::history::history_of(g, mesh).into_iter().filter(|&h| g.node(h).is_some_and(|n| crate::rig::is_rig_chain_node(n.ty().name.as_str()))).collect();
        let all_invertible = chain.iter().all(|&h| g.node(h).is_some_and(|n| INVERTIBLE.iter().any(|t| n.ty().is_a(t))));
        return if all_invertible { Place::Front } else { Place::Before };
    }
    if a.bool("split") {
        return Place::Front;
    }
    Place::from_args(a)
}

/// The objects a deformer command works on: those named, or the selection without hidden objects unless
/// -includeHiddenSelections.
pub(crate) fn targets(s: &Session, a: &Args) -> Vec<String> {
    let named = a.named();
    if !named.is_empty() {
        return named;
    }
    let g = &s.graph;
    objects(s, a)
        .into_iter()
        .filter(|n| a.bool("includeHiddenSelections") || g.find(n.split('.').next().unwrap_or(n)).ok().flatten().is_none_or(|id| crate::transform::effectively_visible(g, id)))
        .collect()
}

/// -useComponentTags: the deformer works by component tags rather than a set, so it has no deformer set.
pub(crate) fn use_component_tags(g: &mut Graph, n: NodeId) -> CmdResult<()> {
    if let Some(set) = crate::deform_sets::set_of(g, n) {
        g.delete_node(set)?;
    }
    Ok(())
}

fn message(g: &Graph, n: NodeId) -> Option<PlugRef> {
    g.node(n)?.ty().attr_index("message").map(|i| PlugRef::new(n, i as i32))
}

/// -exclusive: the deformer's set goes into a partition of that name, made when there is none.
pub(crate) fn exclusive(g: &mut Graph, n: NodeId, name: &str) -> CmdResult<()> {
    let Some(set) = crate::deform_sets::set_of(g, n) else { fail!("exclusive: the deformer has no set to partition") };
    let part = match g.find(name)? {
        Some(p) if g.node(p).is_some_and(|x| x.ty().name == "partition") => p,
        Some(_) => fail!("exclusive: {name} is not a partition"),
        None => {
            let t = crate::node::types::find("partition").expect("partition");
            g.create_node(&t, name, crate::plug::NO_NODE, true)?
        }
    };
    let Some(sets) = g.node(part).and_then(|x| x.ty().attr_index("sets")) else { return Ok(()) };
    let Some(msg) = message(g, set) else { return Ok(()) };
    let next = g.elements(&PlugRef::new(part, sets as i32)).iter().map(|e| e.element + 1).max().unwrap_or(0);
    g.connect(&msg, &PlugRef::element(part, sets as i32, next), false)?;
    Ok(())
}

/// The partition a deformer's set is in, or "".
pub(crate) fn exclusive_of(g: &Graph, n: NodeId) -> String {
    let Some(set) = crate::deform_sets::set_of(g, n) else { return String::new() };
    let Some(msg) = message(g, set) else { return String::new() };
    g.destinations(&msg).into_iter().find(|d| g.node(d.node).is_some_and(|x| x.ty().name == "partition")).map(|d| g.path(d.node)).unwrap_or_default()
}

/// The points a deformer acts on, each mesh's: its set's members, or every point when it has none.
pub(crate) fn members(g: &mut Graph, n: NodeId) -> Vec<Member> {
    if let Some(set) = crate::deform_sets::set_of(g, n) {
        return crate::object_sets::members(g, set);
    }
    let mut out = Vec::new();
    for x in deform::group(g, n) {
        let Some(mesh) = deform::mesh_fed_by(g, x) else { continue };
        let ids: Vec<u32> = crate::modifiers::cage(g, mesh).map(|m| (0..m.point_capacity()).filter(|&p| m.point_valid(p)).collect()).unwrap_or_default();
        out.push(Member { node: mesh, kind: ComponentMode::Vertex, ids });
    }
    out
}

/// -components: the deformer's points as component names.
pub(crate) fn components(g: &mut Graph, n: NodeId) -> Value {
    let list = members(g, n);
    json!(crate::object_sets::format(g, &list))
}

/// -selectedComponents: those of its points that are selected.
pub(crate) fn selected_components(g: &mut Graph, n: NodeId) -> Value {
    let picked = crate::object_sets::from_selection(g);
    let mut out = Vec::new();
    for m in members(g, n) {
        let mut ids: Vec<u32> = picked.iter().filter(|p| p.node == m.node && p.kind == ComponentMode::Vertex).flat_map(|p| p.ids.iter().copied()).filter(|i| m.ids.contains(i)).collect();
        ids.sort_unstable();
        ids.dedup();
        if !ids.is_empty() {
            out.push(Member { node: m.node, kind: ComponentMode::Vertex, ids });
        }
    }
    json!(crate::object_sets::format(g, &out))
}

/// -dumpInfo: for each geometry, its index, how many points the deformer acts on and how many it has.
pub(crate) fn dump_info(g: &mut Graph, n: NodeId) -> Value {
    let list = members(g, n);
    let mut out = Vec::new();
    for (i, x) in deform::group(g, n).into_iter().enumerate() {
        let Some(mesh) = deform::mesh_fed_by(g, x) else { continue };
        let full = crate::modifiers::cage(g, mesh).map(|m| (0..m.point_capacity()).filter(|&p| m.point_valid(p)).count()).unwrap_or(0);
        let affect: usize = list.iter().filter(|m| m.node == mesh).map(|m| m.ids.len()).sum();
        out.extend([json!(i), json!(affect), json!(full)]);
    }
    Value::Array(out)
}

/// The creation flags every deformer command shares, taking the ones a command already has as they are.
pub(crate) fn creation_flags(mut c: crate::history_qe::Cmd) -> crate::history_qe::Cmd {
    use aztek_ops::ArgType;
    use crate::history_qe::{f, upsert};
    for fl in [
        f("afterReference", "ar", ArgType::Bool, "Go before the other deformers, or after them when the geometry is referenced."),
        f("automatic", "at", ArgType::Bool, "Go in front of the chain when only invertible deformers (skin, clusters) are there, else before."),
        f("split", "sp", ArgType::Bool, "A branch of its own from the original shape rather than into the chain."),
        f("exclusive", "ex", ArgType::String, "The partition the deformer's set goes into; -q: the partition."),
        f("includeHiddenSelections", "ihs", ArgType::Bool, "Deform hidden objects in the selection too (off)."),
        f("useComponentTags", "uct", ArgType::Bool, "Work by component tags: no deformer set is made."),
        f("components", "cmp", ArgType::Bool, "Query: the points the deformer acts on."),
        f("selectedComponents", "cms", ArgType::Bool, "Query: those of its points that are selected."),
        f("deformerTools", "dt", ArgType::Bool, "Query: the deformer's tool objects."),
        f("dumpInfo", "dui", ArgType::Bool, "Query: per geometry, its index, how many points it acts on and how many there are."),
    ] {
        if c.flags.iter().any(|x| x.name == fl.name) {
            continue;
        }
        c = upsert(c, fl);
    }
    c
}

/// Answers the shared queries, or None for another flag.
pub(crate) fn query(g: &mut Graph, n: NodeId, a: &Args, tools: &[NodeId]) -> Option<Value> {
    let asked = |k: &str| crate::skin::asked(a, k);
    if asked("components") {
        return Some(components(g, n));
    }
    if asked("selectedComponents") {
        return Some(selected_components(g, n));
    }
    if asked("dumpInfo") {
        return Some(dump_info(g, n));
    }
    if asked("deformerTools") {
        return Some(json!(tools.iter().map(|&t| g.path(t)).collect::<Vec<_>>()));
    }
    if asked("exclusive") {
        return Some(json!(exclusive_of(g, n)));
    }
    None
}

/// What every deformer command answers about a deformer it names: the shared queries, -geometry and -geometryIndices.
/// `tools` are its tool objects for -deformerTools.
pub(crate) fn shared_query(g: &mut Graph, n: NodeId, a: &Args, tools: &[NodeId]) -> Option<Value> {
    if let Some(v) = query(g, n, a, tools) {
        return Some(v);
    }
    let asked = |k: &str| crate::skin::asked(a, k);
    if asked("geometryIndices") {
        return Some(json!((0..deform::geometry_names(g, n).len()).collect::<Vec<_>>()));
    }
    if asked("geometry") {
        return Some(json!(deform::geometry_names(g, n)));
    }
    None
}

/// The shared edits on a deformer named: -geometry (with -remove) and -prune. None when the call has neither.
pub(crate) fn shared_edit(s: &mut Session, a: &Args, name: &str, cmd: &str) -> Option<CmdResult> {
    if let Some(geo) = a.get("geometry").cloned() {
        let mut call = serde_json::Map::new();
        call.insert("edit".into(), json!(true));
        call.insert("geometry".into(), geo);
        if a.bool("remove") {
            call.insert("remove".into(), json!(true));
        }
        call.insert("args".into(), json!([name]));
        return Some(aztek_ops::execute(s, "deformer", &Value::Object(call), aztek_ops::CallSource::Script));
    }
    if a.bool("prune") {
        return Some(deform::prune(s, &Args { positional: vec![json!(name)], flags: Default::default() }, cmd));
    }
    None
}

/// The arguments a deformer command makes its deformer with, once the shared creation flags are worked: the objects named
/// or selected (hidden ones left out unless -includeHiddenSelections, the selection not at all with -ignoreSelected), and
/// where it goes as -before, -after or -frontOfChain.
pub(crate) fn creation_args(s: &mut Session, a: &Args, cmd: &str) -> CmdResult<Args> {
    let mut b = a.clone();
    let named = a.named();
    if named.is_empty() {
        if a.bool("ignoreSelected") {
            fail!("{cmd}: -ignoreSelected: name the objects to deform");
        }
        let all = objects(s, a);
        let kept = targets(s, a);
        // Hidden objects taken in, or some left out: the inner command is given the objects by name.
        if kept.len() != all.len() || a.bool("includeHiddenSelections") {
            // The generic deformer command takes a selection that is all hidden as it is: mGear makes its
            // slideCurve2 on a hidden curve it has just selected, and the reference makes it.
            let take = if kept.is_empty() && cmd == "deformer" { &all } else { &kept };
            if take.is_empty() {
                fail!("{cmd}: the objects selected are hidden (use -includeHiddenSelections)");
            }
            b.positional = take.iter().map(|k| json!(k)).collect();
        }
    }
    if a.bool("afterReference") || a.bool("automatic") || a.bool("split") || a.bool("parallel") {
        let first = b.named().into_iter().next().or_else(|| objects(s, a).into_iter().next());
        let mesh = first.and_then(|n| s.graph.find(n.split('.').next().unwrap_or(&n)).ok().flatten()).and_then(|id| crate::components::as_mesh(&s.graph, id));
        // -parallel puts the deformer beside the others, from the shape they start from; it goes in front of them here.
        let place = match mesh {
            Some(m) if !a.bool("parallel") => placement(&s.graph, a, m),
            _ => Place::Front,
        };
        for k in ["before", "after", "frontOfChain", "afterReference", "automatic", "split", "parallel"] {
            b.flags.remove(k);
        }
        let k = match place {
            Place::After => "after",
            Place::Before => "before",
            Place::Front => "frontOfChain",
        };
        b.flags.insert(k.into(), json!(true));
    }
    Ok(b)
}

/// After a deformer is made: -exclusive's partition and -useComponentTags.
pub(crate) fn after_create(g: &mut Graph, n: NodeId, a: &Args) -> CmdResult<()> {
    if let Some(ex) = a.get("exclusive").and_then(Value::as_str) {
        exclusive(g, n, ex)?;
    }
    if a.bool("useComponentTags") {
        use_component_tags(g, n)?;
    }
    Ok(())
}

/// The flags a deformer command shares beyond creation_flags: what it deforms, edited and queried.
pub(crate) fn geometry_flags(mut c: crate::history_qe::Cmd) -> crate::history_qe::Cmd {
    use aztek_ops::ArgType;
    use crate::history_qe::{f, upsert};
    for fl in [
        f("geometry", "g", ArgType::Any, "With -e: geometry to add (or take off with -remove). With -q: the shapes it deforms.").multi(1),
        f("geometryIndices", "gi", ArgType::Bool, "With -q -g: the index of each shape."),
        f("remove", "rm", ArgType::Bool, "With -e -g: take the geometry off the deformer."),
        f("ignoreSelected", "is", ArgType::Bool, "Deform only the objects named, not the selection."),
        f("parallel", "par", ArgType::Bool, "Work beside the other deformers, from the shape they start from."),
        f("prune", "pr", ArgType::Bool, "With -e: the points it doesn't move leave its set."),
    ] {
        if c.flags.iter().any(|x| x.name == fl.name) {
            continue;
        }
        c = upsert(c, fl);
    }
    c
}
