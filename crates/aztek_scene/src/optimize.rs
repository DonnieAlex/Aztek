//! File > Optimize Scene Size (`cleanUpScene`, the runtime command OptimizeScene) and the Hypershade's Delete Unused
//! Nodes (`MLdeleteUnused`): what each removes follows the reference's Optimize Scene Size Options page. Referenced,
//! locked and default nodes are never removed.

use std::collections::{HashMap, HashSet};

use aztek_ops::{ArgType, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::plug::NodeId;
use crate::session::Session;

/// The categories, by flag: (flag, short, what it removes).
pub const CATEGORIES: &[(&str, &str, &str)] = &[
    ("emptySets", "es", "Empty sets, other than the default ones."),
    ("emptyTransforms", "et", "Transforms with no children and no connections."),
    ("displayLayers", "dl", "Empty, unlocked display layers other than the default one."),
    ("animationCurves", "ac", "Animation curves that drive nothing."),
    ("expressions", "ex", "Expressions with no output connected."),
    ("groupIds", "gi", "groupId nodes with no connections."),
    ("renderingNodes", "rn", "Empty shading groups, materials no group uses, and textures and utilities that feed nothing."),
    ("locators", "lc", "Locators with no connections, alone under a transform with none."),
    ("constraints", "cn", "Constraints that drive nothing."),
    ("pairBlends", "pb", "pairBlends with no output or nothing into input2."),
    ("unitConversions", "uc", "unitConversion nodes with one connection or none."),
    ("unknownNodes", "un", "unknown, unknownDag and unknownTransform nodes."),
];

struct Links {
    /// Per node: the nodes it feeds and the nodes feeding it (other nodes only), and how many connections it has.
    out: HashMap<NodeId, Vec<NodeId>>,
    inn: HashMap<NodeId, Vec<NodeId>>,
    count: HashMap<NodeId, usize>,
    /// Plugs with something connected into them: (node, attribute name).
    driven: HashSet<(NodeId, String)>,
}

fn links(g: &Graph) -> Links {
    let mut l = Links { out: HashMap::new(), inn: HashMap::new(), count: HashMap::new(), driven: HashSet::new() };
    for (dst, src) in g.connections() {
        *l.count.entry(dst.node).or_default() += 1;
        *l.count.entry(src.node).or_default() += 1;
        if dst.node != src.node {
            l.out.entry(src.node).or_default().push(dst.node);
            l.inn.entry(dst.node).or_default().push(src.node);
        }
        l.driven.insert((dst.node, g.attr(&dst).name.clone()));
    }
    l
}

fn ty(g: &Graph, id: NodeId) -> String {
    g.node(id).map(|n| n.ty().name.clone()).unwrap_or_default()
}

fn removable(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| n.reference_id() == 0 && !n.is_locked()) && !crate::defaults::undeletable(g, id)
}

/// A node the scene keeps lists of (defaultTextureList1 and the like): feeding one doesn't count as being used.
fn is_list(g: &Graph, id: NodeId) -> bool {
    g.node(id).is_some_and(|n| {
        let name = n.name();
        crate::defaults::is_scene_node(name) && (name.ends_with("List1") || name.starts_with("lightLinker") || name == "defaultRenderingList1")
    })
}

/// What one category finds, in the scene as it is.
fn find(g: &Graph, l: &Links, cat: &str) -> Vec<NodeId> {
    let nodes = g.nodes();
    let none = |id: NodeId| l.count.get(&id).copied().unwrap_or(0) == 0;
    let outs = |id: NodeId| l.out.get(&id).map_or(0, |v| v.iter().filter(|&&d| !is_list(g, d)).count());
    let mut found: Vec<NodeId> = match cat {
        "emptySets" => nodes.iter().copied().filter(|&id| ty(g, id) == "objectSet" && crate::object_sets::members(g, id).is_empty()).collect(),
        "emptyTransforms" => nodes.iter().copied().filter(|&id| ty(g, id) == "transform" && g.children(id).is_empty() && none(id)).collect(),
        "displayLayers" => nodes
            .iter()
            .copied()
            .filter(|&id| ty(g, id) == "displayLayer" && crate::display_layers::members(g, id).is_empty())
            .collect(),
        "animationCurves" => nodes.iter().copied().filter(|&id| ty(g, id).starts_with("animCurve") && outs(id) == 0).collect(),
        "expressions" => nodes.iter().copied().filter(|&id| ty(g, id) == "expression" && outs(id) == 0).collect(),
        "groupIds" => nodes.iter().copied().filter(|&id| ty(g, id) == "groupId" && none(id)).collect(),
        "locators" => nodes
            .iter()
            .copied()
            .filter(|&id| {
                let t = g.parent(id);
                ty(g, id) == "locator" && none(id) && t != crate::plug::NO_NODE && none(t) && g.children(t).len() == 1
            })
            .flat_map(|id| [id, g.parent(id)])
            .collect(),
        "constraints" => nodes
            .iter()
            .copied()
            .filter(|&id| ty(g, id).ends_with("Constraint") && ty(g, id) != "dynamicConstraint" && outs(id) == 0)
            .collect(),
        "pairBlends" => nodes
            .iter()
            .copied()
            .filter(|&id| {
                ty(g, id) == "pairBlend" && (outs(id) == 0 || !["inTranslate2", "inRotate2"].iter().any(|a| l.driven.iter().any(|(n, at)| *n == id && at.starts_with(a))))
            })
            .collect(),
        "unitConversions" => nodes.iter().copied().filter(|&id| ty(g, id) == "unitConversion" && l.count.get(&id).copied().unwrap_or(0) <= 1).collect(),
        "unknownNodes" => nodes.iter().copied().filter(|&id| matches!(ty(g, id).as_str(), "unknown" | "unknownDag" | "unknownTransform")).collect(),
        "renderingNodes" => rendering(g, l),
        _ => Vec::new(),
    };
    found.retain(|&id| removable(g, id));
    found.sort_unstable();
    found.dedup();
    found
}

/// The rendering nodes in the reference's three stages: shading groups with no members or no material; materials no
/// group uses; then textures and utilities that feed nothing left, until none is found.
fn rendering(g: &Graph, l: &Links) -> Vec<NodeId> {
    let mut gone: HashSet<NodeId> = HashSet::new();
    for sg in crate::shading::all_groups(g) {
        if !removable(g, sg) {
            continue;
        }
        let empty = crate::object_sets::members(g, sg).is_empty();
        if empty || crate::shading::material_of_group(g, sg).is_none() {
            gone.insert(sg);
        }
    }
    let used: HashSet<NodeId> = crate::shading::all_groups(g).into_iter().filter(|sg| !gone.contains(sg)).filter_map(|sg| crate::shading::material_of_group(g, sg)).collect();
    for id in g.nodes() {
        if g.node(id).is_some_and(|n| crate::shading::is_material_type(n.ty())) && !used.contains(&id) && removable(g, id) {
            gone.insert(id);
        }
    }
    // Textures and utilities: a node whose every output goes to something already going, or to a list.
    let shading_kind = |id: NodeId| {
        crate::shading::classification(&ty(g, id)).is_some_and(|c| c.starts_with("texture") || c.starts_with("utility"))
    };
    loop {
        let more: Vec<NodeId> = g
            .nodes()
            .into_iter()
            .filter(|&id| !gone.contains(&id) && shading_kind(id) && removable(g, id) && ty(g, id) != "unitConversion")
            .filter(|&id| l.out.get(&id).is_none_or(|v| v.iter().all(|d| gone.contains(d) || is_list(g, *d))))
            .collect();
        if more.is_empty() {
            break;
        }
        gone.extend(more);
    }
    gone.into_iter().collect()
}

/// Runs the categories asked for, each until it finds nothing more (an empty transform's parent may become empty).
/// Returns the names of what went.
pub fn optimize(s: &mut Session, cats: &[&str]) -> CmdResult<Vec<String>> {
    let mut gone = Vec::new();
    s.graph.open_chunk("Optimize Scene Size");
    let r = (|| -> CmdResult<()> {
        for _ in 0..64 {
            let mut round = Vec::new();
            for cat in cats {
                let l = links(&s.graph);
                for id in find(&s.graph, &l, cat) {
                    if !round.contains(&id) {
                        round.push(id);
                    }
                }
            }
            // Children before parents, so a path is still good when it's deleted.
            let mut paths: Vec<String> = round.iter().filter(|&&id| s.graph.node(id).is_some()).map(|&id| s.graph.path(id)).collect();
            paths.sort_by_key(|p| std::cmp::Reverse(p.matches('|').count()));
            paths.dedup();
            if paths.is_empty() {
                break;
            }
            let mut any = false;
            for p in paths {
                let Ok(Some(id)) = s.graph.find(&p) else { continue };
                if s.graph.node(id).is_none() {
                    continue;
                }
                if s.run("delete", json!({"args": [p.clone()]})).is_ok() {
                    gone.push(p);
                    any = true;
                }
            }
            if !any {
                break;
            }
        }
        Ok(())
    })();
    s.graph.close_chunk();
    r?;
    Ok(gone)
}

pub fn register(c: &mut Commands<Session>) {
    let mut cmd = CommandDef::<Session>::new(
        "cleanUpScene",
        "File > Optimize Scene Size (the runtime command OptimizeScene): removes what the flags name, or every category \
         when none is named: empty sets, empty transforms, empty display layers, unused animation curves, expressions \
         and groupIds, unused rendering nodes, lone locators, idle constraints, pairBlends and unitConversions, and \
         unknown nodes. Referenced, locked and default nodes stay. One undo step. Returns what went.",
        |s, a| {
            let named: Vec<&str> = CATEGORIES.iter().map(|c| c.0).filter(|f| a.bool(f)).collect();
            let cats: Vec<&str> = if named.is_empty() { CATEGORIES.iter().map(|c| c.0).collect() } else { named };
            Ok(json!(optimize(s, &cats)?))
        },
    )
    .positional("[mode]");
    for (flag, short, doc) in CATEGORIES {
        cmd = cmd.flag(FlagDef::new(flag, short, ArgType::Bool, Value::Null, doc));
    }
    c.register(cmd);
    c.register(
        CommandDef::<Session>::new(
            "MLdeleteUnused",
            "The Hypershade's Edit > Delete Unused Nodes: shading groups with no members or material, materials no group \
             uses, and textures and utilities that feed nothing. One undo step. Returns what went.",
            |s, _a| Ok(json!(optimize(s, &["renderingNodes"])?)),
        ),
    );
}
