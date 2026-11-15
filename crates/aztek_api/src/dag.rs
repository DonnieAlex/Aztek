//! The DAG node function set and DAG paths. `None` as a parent is the world.

use aztek_dg::{DagPath, DgError, Graph, NodeId, Result};

fn dag(g: &Graph, id: NodeId) -> Result<()> {
    if !g.exists(id) {
        return Err(DgError::NoNode);
    }
    if !g.is_dag(id) {
        return Err(DgError::Invalid(format!("{} is not a DAG node", g.name(id)?)));
    }
    Ok(())
}

pub fn parent_count(g: &Graph, id: NodeId) -> Result<usize> {
    dag(g, id)?;
    Ok(g.dag_parents(id).len())
}

pub fn parent(g: &Graph, id: NodeId, index: usize) -> Result<Option<NodeId>> {
    dag(g, id)?;
    g.dag_parents(id).get(index).copied().ok_or_else(|| DgError::Invalid(format!("no parent at {index}")))
}

pub fn child_count(g: &Graph, parent: Option<NodeId>) -> Result<usize> {
    if let Some(p) = parent {
        dag(g, p)?;
    }
    Ok(g.dag_children(parent).len())
}

pub fn child(g: &Graph, parent: Option<NodeId>, index: usize) -> Result<NodeId> {
    if let Some(p) = parent {
        dag(g, p)?;
    }
    g.dag_children(parent).get(index).copied().ok_or_else(|| DgError::Invalid(format!("no child at {index}")))
}

/// Makes `child` a child of `parent` outside any undo step, at `index` among its children (the end
/// for `None`). Without `keep_existing_parents` this becomes its only parent.
pub fn add_child(g: &mut Graph, parent: Option<NodeId>, child: NodeId, index: Option<usize>, keep_existing_parents: bool) -> Result<()> {
    g.set_parent_at(child, parent, keep_existing_parents, index)
}

/// Removes one parent of an instanced child, outside any undo step.
pub fn remove_child(g: &mut Graph, parent: Option<NodeId>, child: NodeId) -> Result<()> {
    g.remove_parent(child, parent)
}

pub fn remove_child_at(g: &mut Graph, parent: Option<NodeId>, index: usize) -> Result<()> {
    let c = child(g, parent, index)?;
    g.remove_parent(c, parent)
}

/// Whether `parent` is a direct parent of `id`.
pub fn has_parent(g: &Graph, id: NodeId, parent: Option<NodeId>) -> bool {
    g.dag_parents(id).contains(&parent)
}

/// Whether `child` is a direct child of `id`.
pub fn has_child(g: &Graph, id: Option<NodeId>, child: NodeId) -> bool {
    g.dag_children(id).contains(&child)
}

/// Whether `id` is under `other`, directly or not.
pub fn is_child_of(g: &Graph, id: NodeId, other: NodeId) -> bool {
    g.dag_descendants(other).contains(&id)
}

/// Whether `other` is under `id`, directly or not.
pub fn is_parent_of(g: &Graph, id: NodeId, other: NodeId) -> bool {
    g.dag_descendants(id).contains(&other)
}

/// Instanced through its own parents, or (with `indirect`) through any node above it.
pub fn is_instanced(g: &Graph, id: NodeId, indirect: bool) -> Result<bool> {
    dag(g, id)?;
    Ok(if indirect { g.instance_count(id) > 1 } else { g.dag_parents(id).len() > 1 })
}

/// Paths to the node (`total`) or its own parents.
pub fn instance_count(g: &Graph, id: NodeId, total: bool) -> Result<usize> {
    dag(g, id)?;
    Ok(if total { g.instance_count(id) } else { g.dag_parents(id).len() })
}

pub fn all_paths(g: &Graph, id: NodeId) -> Result<Vec<DagPath>> {
    dag(g, id)?;
    Ok(g.dag_paths(id))
}

/// The first path to the node.
pub fn path(g: &Graph, id: NodeId) -> Result<DagPath> {
    all_paths(g, id)?.into_iter().next().ok_or(DgError::NoNode)
}

pub fn full_path_name(g: &Graph, id: NodeId) -> Result<String> {
    path_full_name(g, &path(g, id)?)
}

pub fn partial_path_name(g: &Graph, id: NodeId) -> Result<String> {
    path_partial_name(g, &path(g, id)?)
}

// ---- DAG paths -------------------------------------------------------------------------------

/// `|top|middle|node`.
pub fn path_full_name(g: &Graph, p: &DagPath) -> Result<String> {
    let mut out = String::new();
    for n in &p.0 {
        out.push('|');
        out.push_str(g.name(*n)?);
    }
    Ok(out)
}

/// The shortest tail of the full path that names only this path: `node`, `middle|node`, and so on.
pub fn path_partial_name(g: &Graph, p: &DagPath) -> Result<String> {
    let names: Vec<&str> = p.0.iter().map(|n| g.name(*n)).collect::<Result<_>>()?;
    let leaf = *names.last().ok_or(DgError::NoNode)?;
    let candidates: Vec<Vec<String>> = g
        .nodes_named(leaf)
        .into_iter()
        .filter(|n| g.is_dag(*n))
        .flat_map(|n| g.dag_paths(n))
        .map(|q| q.0.iter().map(|n| g.name(*n).unwrap_or("").to_string()).collect())
        .collect();
    for k in 1..=names.len() {
        let tail = &names[names.len() - k..];
        let matching = candidates.iter().filter(|c| c.len() >= k && c[c.len() - k..].iter().zip(tail).all(|(a, b)| a == b)).count();
        if matching <= 1 {
            return Ok(tail.join("|"));
        }
    }
    path_full_name(g, p)
}

pub fn path_instance_number(g: &Graph, p: &DagPath) -> Result<u32> {
    g.instance_number(p).ok_or_else(|| DgError::BadPlug("not a path in the DAG".into()))
}

/// Whether every step of the path is still a parent of the next.
pub fn path_is_valid(g: &Graph, p: &DagPath) -> bool {
    !p.0.is_empty()
        && p.0.iter().all(|n| g.exists(*n) && g.is_dag(*n))
        && g.dag_parents(p.0[0]).contains(&None)
        && p.0.windows(2).all(|w| g.dag_parents(w[1]).contains(&Some(w[0])))
}

/// The path through `child`, if `child` is a child of the path's node.
pub fn path_push(g: &Graph, p: &DagPath, child: NodeId) -> Result<DagPath> {
    if !g.dag_parents(child).contains(&Some(p.node())) {
        return Err(DgError::Invalid(format!("{} is not a child of {}", g.name(child)?, g.name(p.node())?)));
    }
    let mut q = p.clone();
    q.0.push(child);
    Ok(q)
}

/// The path with its last `n` nodes removed.
pub fn path_pop(p: &DagPath, n: usize) -> Result<DagPath> {
    if n >= p.0.len() {
        return Err(DgError::Invalid("a path cannot be popped past its top node".into()));
    }
    Ok(DagPath(p.0[..p.0.len() - n].to_vec()))
}
