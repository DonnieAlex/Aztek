//! Nodes and the behaviour a node type supplies.

use crate::datablock::DataBlock;
use crate::graph::Graph;
use crate::plug::Plug;
use crate::types::Status;
use crate::value::Value;

/// A node in a graph. Ids are not reused while the graph lives: a deleted node's id stays dead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId {
    pub(crate) index: u32,
    pub(crate) generation: u32,
}

/// What a node type does, beyond holding attributes. Every method has the reference's default.
/// Nodes keep per-node state in their datablock; anything else needs interior mutability.
pub trait NodeImpl: Send + Sync {
    /// Computes `plug`, reading inputs and writing outputs through `data`, then marks what it wrote
    /// clean. Returning [`Status::UnknownParameter`] hands the plug to the type this one derives from.
    fn compute(&self, _plug: &Plug, _data: &mut DataBlock) -> Status {
        Status::UnknownParameter
    }

    /// Adds plugs made dirty when `plug` is made dirty, beyond the declared affects.
    fn set_dependents_dirty(&self, _graph: &Graph, _plug: &Plug, _affected: &mut Vec<Plug>) {}

    /// The value of an internal attribute, or `None` for the datablock's.
    fn get_internal_value(&self, _plug: &Plug) -> Option<Value> {
        None
    }

    /// Called when an internal attribute is set; the datablock is set as well.
    fn set_internal_value(&self, _plug: &Plug, _value: &Value) {}

    /// Called after a connection to or from this node is made; `as_source` says which end `plug` is.
    fn connection_made(&self, _plug: &Plug, _other: &Plug, _as_source: bool) {}

    /// Called after a connection to or from this node is broken.
    fn connection_broken(&self, _plug: &Plug, _other: &Plug, _as_source: bool) {}
}
