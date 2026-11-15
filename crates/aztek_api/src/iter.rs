//! Walking the scene: every node, the DAG, and the graph from a node or plug.

use aztek_dg::{DagPath, Graph, NodeId, Plug, PlugPath, Step};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Traversal {
    DepthFirst,
    BreadthFirst,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Downstream,
    Upstream,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    NodeLevel,
    PlugLevel,
}

/// Every node, oldest first, of `type_name` or a type derived from it when given.
pub fn dependency_nodes(g: &Graph, type_name: Option<&str>) -> Vec<NodeId> {
    g.nodes()
        .into_iter()
        .filter(|n| type_name.is_none_or(|t| g.class_of(*n).is_ok_and(|c| g.registry.is_a(c, t))))
        .collect()
}

/// Every path under `root` (the world for `None`), the root included, one per instance, in DAG
/// order; with a type, only paths whose node is of that type.
pub fn dag(g: &Graph, root: Option<&DagPath>, order: Traversal, type_name: Option<&str>) -> Vec<DagPath> {
    let start: Vec<DagPath> = match root {
        Some(r) => vec![r.clone()],
        None => g.dag_children(None).into_iter().map(|n| DagPath(vec![n])).collect(),
    };
    let kids = |p: &DagPath| -> Vec<DagPath> {
        g.dag_children(Some(p.node()))
            .into_iter()
            .map(|c| {
                let mut q = p.clone();
                q.0.push(c);
                q
            })
            .collect()
    };
    let mut out = Vec::new();
    match order {
        Traversal::DepthFirst => {
            let mut stack: Vec<DagPath> = start.into_iter().rev().collect();
            while let Some(p) = stack.pop() {
                stack.extend(kids(&p).into_iter().rev());
                out.push(p);
            }
        }
        Traversal::BreadthFirst => {
            let mut queue: VecDeque<DagPath> = start.into();
            while let Some(p) = queue.pop_front() {
                queue.extend(kids(&p));
                out.push(p);
            }
        }
    }
    out.retain(|p| type_name.is_none_or(|t| g.class_of(p.node()).is_ok_and(|c| g.registry.is_a(c, t))));
    out
}

/// The plugs a plug feeds (downstream) or is fed by (upstream), one step: connections, and within
/// its node the attributes it affects or that affect it.
fn plug_neighbours(g: &Graph, p: &Plug, dir: Direction) -> Vec<Plug> {
    let mut out = Vec::new();
    let Ok(class) = g.class_of(p.node) else { return out };
    let whole = |a| Plug { node: p.node, path: PlugPath(g.registry.lineage(a).into_iter().map(|attr| Step { attr, index: None }).collect()) };
    match dir {
        Direction::Downstream => {
            out.extend(g.destinations(p));
            for a in g.registry.affected_by(class, p.leaf()) {
                out.push(whole(a));
            }
        }
        Direction::Upstream => {
            out.extend(g.source(p));
            for a in g.registry.class_attrs_all(class) {
                if g.registry.affected_by(class, a).contains(&p.leaf()) {
                    out.push(whole(a));
                }
            }
        }
    }
    out
}

/// The plugs reached from `start`, `start` first, each once.
pub fn dependency_graph_plugs(g: &Graph, start: &Plug, dir: Direction, order: Traversal) -> Vec<Plug> {
    walk(start.clone(), |p| plug_neighbours(g, p, dir), order)
}

/// The nodes reached from `start` through connections, `start` first, each once.
pub fn dependency_graph_nodes(g: &Graph, start: NodeId, dir: Direction, order: Traversal) -> Vec<NodeId> {
    walk(
        start,
        |n| {
            let mut out: Vec<NodeId> = Vec::new();
            for c in g.connections(*n) {
                let next = match dir {
                    Direction::Downstream if c.src.node == *n => c.dst.node,
                    Direction::Upstream if c.dst.node == *n => c.src.node,
                    _ => continue,
                };
                if !out.contains(&next) {
                    out.push(next);
                }
            }
            out
        },
        order,
    )
}

fn walk<T: Clone + Eq + std::hash::Hash>(start: T, next: impl Fn(&T) -> Vec<T>, order: Traversal) -> Vec<T> {
    let mut seen: HashSet<T> = HashSet::new();
    let mut out = Vec::new();
    let mut pending: VecDeque<T> = VecDeque::from([start]);
    while let Some(x) = match order {
        Traversal::DepthFirst => pending.pop_back(),
        Traversal::BreadthFirst => pending.pop_front(),
    } {
        if !seen.insert(x.clone()) {
            continue;
        }
        let mut n = next(&x);
        if order == Traversal::DepthFirst {
            n.reverse();
        }
        pending.extend(n);
        out.push(x);
    }
    out
}
