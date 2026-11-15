//! The DAG: DAG nodes under the world, with instancing through more than one parent.
//!
//! A DAG node has one path from the world per instance. Paths are numbered depth first in parent
//! order: the instance number of a path is its position in [`Graph::dag_paths`]. World-space array
//! attributes have one element per instance.

use crate::graph::Graph;
use crate::node::NodeId;
use crate::plug::{Plug, PlugPath, Step};
use crate::types::{DgError, Result};

/// The nodes from a child of the world down to one node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DagPath(pub Vec<NodeId>);

impl DagPath {
    pub fn node(&self) -> NodeId {
        *self.0.last().expect("empty DAG path")
    }

    /// The path to the parent, or `None` for a child of the world.
    pub fn parent(&self) -> Option<DagPath> {
        (self.0.len() > 1).then(|| DagPath(self.0[..self.0.len() - 1].to_vec()))
    }
}

impl Graph {
    pub fn is_dag(&self, id: NodeId) -> bool {
        self.parents.contains_key(&id)
    }

    fn dag_check(&self, id: NodeId) -> Result<()> {
        if !self.exists(id) {
            return Err(DgError::NoNode);
        }
        if !self.is_dag(id) {
            return Err(DgError::Invalid(format!("{} is not a DAG node", self.name(id)?)));
        }
        Ok(())
    }

    /// A DAG node's parents in order; `None` is the world.
    pub fn dag_parents(&self, id: NodeId) -> Vec<Option<NodeId>> {
        self.parents.get(&id).cloned().unwrap_or_default()
    }

    /// The children of a DAG node, or of the world for `None`, in order.
    pub fn dag_children(&self, parent: Option<NodeId>) -> Vec<NodeId> {
        self.children.get(&parent).cloned().unwrap_or_default()
    }

    /// Every DAG node under `id`, each once.
    pub fn dag_descendants(&self, id: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut stack = self.dag_children(Some(id));
        while let Some(n) = stack.pop() {
            if !out.contains(&n) {
                out.push(n);
                stack.extend(self.dag_children(Some(n)));
            }
        }
        out
    }

    /// Every path from the world to `id`, by instance number.
    pub fn dag_paths(&self, id: NodeId) -> Vec<DagPath> {
        let mut out = Vec::new();
        for p in self.dag_parents(id) {
            match p {
                None => out.push(DagPath(vec![id])),
                Some(parent) => {
                    for mut path in self.dag_paths(parent) {
                        path.0.push(id);
                        out.push(path);
                    }
                }
            }
        }
        out
    }

    pub fn instance_count(&self, id: NodeId) -> usize {
        self.dag_paths(id).len()
    }

    pub fn instance_number(&self, path: &DagPath) -> Option<u32> {
        self.dag_paths(path.node()).iter().position(|p| p == path).map(|i| i as u32)
    }

    /// Parents `child` under `parent` (`None`: the world). With `add_instance` the child keeps its
    /// other parents and gains an instance; without, this becomes its only parent.
    pub fn set_parent(&mut self, child: NodeId, parent: Option<NodeId>, add_instance: bool) -> Result<()> {
        self.set_parent_at(child, parent, add_instance, None)
    }

    /// As [`Graph::set_parent`], placing the child at `index` among the parent's children (the end
    /// when `None` or past the end).
    pub fn set_parent_at(
        &mut self,
        child: NodeId,
        parent: Option<NodeId>,
        add_instance: bool,
        index: Option<usize>,
    ) -> Result<()> {
        self.dag_check(child)?;
        if self.is_node_locked(child)? {
            return Err(DgError::Locked(self.name(child)?.into()));
        }
        if let Some(p) = parent {
            self.dag_check(p)?;
            if p == child || self.dag_descendants(child).contains(&p) {
                return Err(DgError::Invalid(format!("{} cannot go under itself", self.name(child)?)));
            }
        }
        let current = self.dag_parents(child);
        if add_instance {
            if current.contains(&parent) {
                return Err(DgError::Invalid(format!("{} is already under that parent", self.name(child)?)));
            }
            self.parents.get_mut(&child).unwrap().push(parent);
        } else {
            for p in current.clone() {
                if let Some(l) = self.children.get_mut(&p) {
                    l.retain(|c| *c != child);
                }
            }
            self.parents.insert(child, vec![parent]);
        }
        let l = self.children.entry(parent).or_default();
        l.insert(index.unwrap_or(usize::MAX).min(l.len()), child);
        if !add_instance {
            for p in current.iter().filter(|p| **p != parent) {
                self.emit(crate::events::Event::ParentRemoved { child, parent: *p });
            }
        }
        if add_instance || !current.contains(&parent) {
            self.emit(crate::events::Event::ParentAdded { child, parent });
        }
        self.hierarchy_changed(child);
        Ok(())
    }

    /// Removes one parent of an instanced node.
    pub fn remove_parent(&mut self, child: NodeId, parent: Option<NodeId>) -> Result<()> {
        self.dag_check(child)?;
        let current = self.dag_parents(child);
        if !current.contains(&parent) {
            return Err(DgError::Invalid(format!("{} is not under that parent", self.name(child)?)));
        }
        if current.len() == 1 {
            return Err(DgError::Invalid(format!("{} has no other parent", self.name(child)?)));
        }
        self.parents.get_mut(&child).unwrap().retain(|p| *p != parent);
        if let Some(l) = self.children.get_mut(&parent) {
            l.retain(|c| *c != child);
        }
        self.emit(crate::events::Event::ParentRemoved { child, parent });
        self.hierarchy_changed(child);
        Ok(())
    }

    /// The world-space attributes of a DAG node, as whole plugs.
    pub(crate) fn world_space_plugs(&self, id: NodeId) -> Vec<Plug> {
        self.attributes(id)
            .unwrap_or_default()
            .into_iter()
            .filter(|a| self.registry.attr(*a).props.world_space)
            .map(|a| Plug { node: id, path: PlugPath(vec![Step { attr: a, index: None }]) })
            .collect()
    }

    /// Instance numbers and parents changed: world-space values under `id` are stale.
    pub(crate) fn dag_changed(&mut self, id: NodeId) {
        self.hierarchy_changed(id)
    }

    fn hierarchy_changed(&mut self, id: NodeId) {
        let mut nodes = vec![id];
        nodes.extend(self.dag_descendants(id));
        for n in nodes {
            let count = self.instance_count(n) as u32;
            for p in self.world_space_plugs(n) {
                if self.registry.attr(p.leaf()).props.array {
                    self.trim_elements(&p, count);
                }
                let _ = self.mark_dirty(&p);
            }
        }
    }
}
