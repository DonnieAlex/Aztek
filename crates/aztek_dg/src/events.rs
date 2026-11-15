//! What changed in the graph, recorded in order for the API layer's callbacks. Recording is off until
//! something turns it on.

use crate::attr::AttrId;
use crate::node::NodeId;
use crate::plug::Plug;
use crate::registry::ClassId;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    NodeAdded { node: NodeId, class: ClassId },
    NodeRemoved { node: NodeId, class: ClassId },
    NameChanged { node: NodeId, previous: String },
    Connected { src: Plug, dst: Plug },
    Disconnected { src: Plug, dst: Plug },
    ValueSet(Plug),
    ElementAdded(Plug),
    ElementRemoved(Plug),
    AttributeAdded { node: NodeId, attr: AttrId },
    AttributeRemoved { node: NodeId, attr: AttrId },
    AttributeRenamed { node: NodeId, attr: AttrId },
    Locked { plug: Plug, on: bool },
    Keyable { plug: Plug, on: bool },
    Dirty(Plug),
    ParentAdded { child: NodeId, parent: Option<NodeId> },
    ParentRemoved { child: NodeId, parent: Option<NodeId> },
}
