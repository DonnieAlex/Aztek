//! The dependency node function set: a node's identity, attributes and connections.

use crate::naming;
use aztek_dg::{AttrId, AttrSpec, DgError, Graph, NodeId, Plug, Result};

/// Creates a node outside any undo step, named `name` or after its type, made unique.
/// A DAG node goes under the world.
pub fn create(g: &mut Graph, type_name: &str, name: Option<&str>) -> Result<NodeId> {
    let mut d = g.create_detached(type_name, "")?;
    let dag = g.registry.is_dag(g.registry.class_by_name(type_name).unwrap());
    let desired = name.map(str::to_string).unwrap_or_else(|| format!("{type_name}#"));
    let parents = [None];
    let unique = naming::unique_name(g, &desired, None, dag.then_some(&parents[..]))?;
    d.set_name(&unique);
    g.attach(d)
}

pub fn name(g: &Graph, id: NodeId) -> Result<String> {
    g.name(id).map(str::to_string)
}

/// Renames a node; a clashing name is made unique. Returns the name it got.
pub fn set_name(g: &mut Graph, id: NodeId, name: &str) -> Result<String> {
    let unique = unique_for(g, id, name)?;
    g.rename(id, &unique)?;
    Ok(unique)
}

/// The name `id` would get if renamed to `name`.
pub fn unique_for(g: &Graph, id: NodeId, name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(DgError::Invalid("an empty name".into()));
    }
    let parents = g.is_dag(id).then(|| g.dag_parents(id));
    naming::unique_name(g, name, Some(id), parents.as_deref())
}

/// Whether no other node has this node's name.
pub fn has_unique_name(g: &Graph, id: NodeId) -> Result<bool> {
    Ok(g.nodes_named(g.name(id)?).len() == 1)
}

pub fn type_name(g: &Graph, id: NodeId) -> Result<String> {
    Ok(g.registry.class(g.class_of(id)?).name.clone())
}

pub fn type_id(g: &Graph, id: NodeId) -> Result<u32> {
    Ok(g.registry.class(g.class_of(id)?).type_id)
}

/// Every attribute, children included, in the order they were added: the type's, then dynamic ones.
pub fn attributes(g: &Graph, id: NodeId) -> Result<Vec<AttrId>> {
    let mut out = Vec::new();
    for a in g.attributes(id)? {
        g.registry.with_descendants(a, &mut out);
    }
    Ok(out)
}

pub fn attribute_count(g: &Graph, id: NodeId) -> Result<usize> {
    Ok(attributes(g, id)?.len())
}

pub fn attribute(g: &Graph, id: NodeId, index: usize) -> Result<AttrId> {
    attributes(g, id)?.get(index).copied().ok_or_else(|| DgError::Invalid(format!("no attribute at {index}")))
}

pub fn attribute_named(g: &Graph, id: NodeId, name: &str) -> Result<AttrId> {
    g.exists(id).then_some(()).ok_or(DgError::NoNode)?;
    g.find_attr(id, name).ok_or_else(|| DgError::NoAttribute(name.into()))
}

pub fn has_attribute(g: &Graph, id: NodeId, name: &str) -> bool {
    g.find_attr(id, name).is_some()
}

/// The plug for an attribute name or plug path (`a`, `a[2].b`).
pub fn find_plug(g: &Graph, id: NodeId, name: &str) -> Result<Plug> {
    g.plug(id, name)
}

pub fn find_plug_for(g: &Graph, id: NodeId, attr: AttrId) -> Result<Plug> {
    g.plug_for(id, attr)
}

/// Adds a dynamic attribute outside any undo step.
pub fn add_attribute(g: &mut Graph, id: NodeId, spec: AttrSpec) -> Result<AttrId> {
    g.add_attribute(id, spec)
}

/// Removes a dynamic attribute outside any undo step.
pub fn remove_attribute(g: &mut Graph, id: NodeId, attr: AttrId) -> Result<()> {
    g.remove_attribute(id, attr)
}

/// Whether the attribute was added to this node rather than coming with its type.
pub fn is_new_attribute(g: &Graph, id: NodeId, attr: AttrId) -> Result<bool> {
    Ok(g.attributes(id)?.contains(&g.registry.root_of(attr)) && !g.registry.attr_in_class(g.class_of(id)?, attr))
}

/// The plugs of this node that have connections, each once, in connection order.
pub fn connections(g: &Graph, id: NodeId) -> Result<Vec<Plug>> {
    g.exists(id).then_some(()).ok_or(DgError::NoNode)?;
    let mut out: Vec<Plug> = Vec::new();
    for c in g.connections(id) {
        for p in [c.src, c.dst] {
            if p.node == id && !out.contains(&p) {
                out.push(p);
            }
        }
    }
    Ok(out)
}

/// The attributes `attr` affects on this node's type.
pub fn affected_attributes(g: &Graph, id: NodeId, attr: AttrId) -> Result<Vec<AttrId>> {
    Ok(g.registry.affected_by(g.class_of(id)?, attr))
}

/// The attributes that affect `attr` on this node's type.
pub fn affected_by_attributes(g: &Graph, id: NodeId, attr: AttrId) -> Result<Vec<AttrId>> {
    let c = g.class_of(id)?;
    Ok(g.registry.class_attrs_all(c).into_iter().filter(|a| g.registry.affected_by(c, *a).contains(&attr)).collect())
}

pub fn is_locked(g: &Graph, id: NodeId) -> Result<bool> {
    g.is_node_locked(id)
}

pub fn set_locked(g: &mut Graph, id: NodeId, on: bool) -> Result<()> {
    g.set_node_locked(id, on)
}
