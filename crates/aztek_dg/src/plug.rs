//! Plugs: an attribute on a node, down through compound children and array elements.

use crate::attr::AttrId;
use crate::node::NodeId;

/// One level of a plug: an attribute, and for an array attribute the logical index of an element.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Step {
    pub attr: AttrId,
    pub index: Option<u32>,
}

/// The steps from a top-level attribute down to the plug's attribute. An array attribute without an
/// index is the whole array when it is the last step; anywhere else it stands for every element.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlugPath(pub Vec<Step>);

impl PlugPath {
    pub fn leaf(&self) -> AttrId {
        self.0.last().expect("empty plug path").attr
    }

    /// Whether this path is `other` or one of its parents (a whole array covers its elements).
    pub fn covers(&self, other: &PlugPath) -> bool {
        self.0.len() <= other.0.len()
            && self.0.iter().zip(&other.0).all(|(a, b)| a.attr == b.attr && (a.index.is_none() || a.index == b.index))
    }

    /// Whether one path covers the other.
    pub fn related(&self, other: &PlugPath) -> bool {
        self.covers(other) || other.covers(self)
    }

    /// The path of the whole array this element belongs to.
    pub fn array_of_element(&self) -> Option<PlugPath> {
        let last = self.0.last()?;
        last.index?;
        let mut p = self.clone();
        p.0.last_mut().unwrap().index = None;
        Some(p)
    }

    pub fn with_child(&self, attr: AttrId) -> PlugPath {
        let mut p = self.clone();
        p.0.push(Step { attr, index: None });
        p
    }

    pub fn with_index(&self, index: u32) -> PlugPath {
        let mut p = self.clone();
        if let Some(l) = p.0.last_mut() {
            l.index = Some(index);
        }
        p
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Plug {
    pub node: NodeId,
    pub path: PlugPath,
}

impl Plug {
    pub fn leaf(&self) -> AttrId {
        self.path.leaf()
    }

    pub fn element(&self, index: u32) -> Plug {
        Plug { node: self.node, path: self.path.with_index(index) }
    }

    pub fn child(&self, attr: AttrId) -> Plug {
        Plug { node: self.node, path: self.path.with_child(attr) }
    }

    /// Whether this is an element of an array.
    pub fn is_element(&self) -> bool {
        self.path.0.last().is_some_and(|s| s.index.is_some())
    }
}
