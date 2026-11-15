//! Callbacks on graph changes, in the shape of the reference's message classes: node added and
//! removed (by type), connections, attribute changes per node with the reference's message flags,
//! attributes added and removed, dirty plugs, name changes, DAG parent changes, named events.

use aztek_dg::{Event, Graph, NodeId, Plug, PlugPath, Step};

/// The reference's attribute message flags.
pub mod attr_msg {
    pub const CONNECTION_MADE: u32 = 0x01;
    pub const CONNECTION_BROKEN: u32 = 0x02;
    pub const ATTRIBUTE_EVAL: u32 = 0x04;
    pub const ATTRIBUTE_SET: u32 = 0x08;
    pub const ATTRIBUTE_LOCKED: u32 = 0x10;
    pub const ATTRIBUTE_UNLOCKED: u32 = 0x20;
    pub const ATTRIBUTE_ADDED: u32 = 0x40;
    pub const ATTRIBUTE_REMOVED: u32 = 0x80;
    pub const ATTRIBUTE_RENAMED: u32 = 0x100;
    pub const ATTRIBUTE_KEYABLE: u32 = 0x200;
    pub const ATTRIBUTE_UNKEYABLE: u32 = 0x400;
    pub const INCOMING_DIRECTION: u32 = 0x800;
    pub const ATTRIBUTE_ARRAY_ADDED: u32 = 0x1000;
    pub const ATTRIBUTE_ARRAY_REMOVED: u32 = 0x2000;
    pub const OTHER_PLUG_SET: u32 = 0x4000;
}

pub type CallbackId = u64;

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    Connection { src: Plug, dst: Plug, made: bool },
    AttributeChanged { msg: u32, plug: Plug, other: Option<Plug> },
    AttributeAddedOrRemoved { msg: u32, plug: Plug },
    NodeDirtyPlug { node: NodeId, plug: Plug },
    NameChanged { node: NodeId, previous: String },
    ParentAdded { child: NodeId, parent: Option<NodeId> },
    ParentRemoved { child: NodeId, parent: Option<NodeId> },
    Event(String),
}

enum Kind {
    NodeAdded(Option<String>),
    NodeRemoved(Option<String>),
    Connection,
    AttributeChanged(NodeId),
    AttributeAddedOrRemoved(NodeId),
    NodeDirtyPlug(NodeId),
    NameChanged(Option<NodeId>),
    ParentAdded,
    ParentRemoved,
    Event(String),
}

type Callback = Box<dyn FnMut(&Graph, &Message)>;

struct Entry {
    id: CallbackId,
    kind: Kind,
    f: Callback,
}

#[derive(Default)]
pub struct Messages {
    next: CallbackId,
    list: Vec<Entry>,
}

fn whole(node: NodeId, attr: aztek_dg::AttrId) -> Plug {
    Plug { node, path: PlugPath(vec![Step { attr, index: None }]) }
}

impl Messages {
    pub fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, kind: Kind, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.next += 1;
        self.list.push(Entry { id: self.next, kind, f: Box::new(f) });
        self.next
    }

    /// Nodes added, of `type_name` or a type derived from it (all nodes for `None` or `dependNode`).
    pub fn add_node_added_callback(&mut self, type_name: Option<&str>, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::NodeAdded(type_name.map(str::to_string)), f)
    }

    pub fn add_node_removed_callback(&mut self, type_name: Option<&str>, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::NodeRemoved(type_name.map(str::to_string)), f)
    }

    pub fn add_connection_callback(&mut self, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::Connection, f)
    }

    /// Attribute changes on one node, with [`attr_msg`] flags.
    pub fn add_attribute_changed_callback(&mut self, node: NodeId, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::AttributeChanged(node), f)
    }

    pub fn add_attribute_added_or_removed_callback(&mut self, node: NodeId, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::AttributeAddedOrRemoved(node), f)
    }

    pub fn add_node_dirty_plug_callback(&mut self, node: NodeId, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::NodeDirtyPlug(node), f)
    }

    /// Name changes of one node, or of every node for `None`.
    pub fn add_name_changed_callback(&mut self, node: Option<NodeId>, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::NameChanged(node), f)
    }

    pub fn add_parent_added_callback(&mut self, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::ParentAdded, f)
    }

    pub fn add_parent_removed_callback(&mut self, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::ParentRemoved, f)
    }

    /// A named event, such as `SelectionChanged`.
    pub fn add_event_callback(&mut self, event: &str, f: impl FnMut(&Graph, &Message) + 'static) -> CallbackId {
        self.add(Kind::Event(event.into()), f)
    }

    pub fn remove_callback(&mut self, id: CallbackId) -> bool {
        let before = self.list.len();
        self.list.retain(|e| e.id != id);
        self.list.len() != before
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    /// The messages an event makes.
    pub fn messages_for(e: &Event) -> Vec<Message> {
        use attr_msg::*;
        let changed = |msg, plug: &Plug, other: Option<&Plug>| Message::AttributeChanged { msg, plug: plug.clone(), other: other.cloned() };
        match e {
            Event::NodeAdded { node, .. } => vec![Message::NodeAdded(*node)],
            Event::NodeRemoved { node, .. } => vec![Message::NodeRemoved(*node)],
            Event::NameChanged { node, previous } => vec![Message::NameChanged { node: *node, previous: previous.clone() }],
            Event::Connected { src, dst } => vec![
                Message::Connection { src: src.clone(), dst: dst.clone(), made: true },
                changed(CONNECTION_MADE, src, Some(dst)),
                changed(CONNECTION_MADE | INCOMING_DIRECTION, dst, Some(src)),
            ],
            Event::Disconnected { src, dst } => vec![
                Message::Connection { src: src.clone(), dst: dst.clone(), made: false },
                changed(CONNECTION_BROKEN, src, Some(dst)),
                changed(CONNECTION_BROKEN | INCOMING_DIRECTION, dst, Some(src)),
            ],
            Event::ValueSet(p) => vec![changed(ATTRIBUTE_SET, p, None)],
            Event::ElementAdded(p) => vec![changed(ATTRIBUTE_ARRAY_ADDED, p, None)],
            Event::ElementRemoved(p) => vec![changed(ATTRIBUTE_ARRAY_REMOVED, p, None)],
            Event::AttributeAdded { node, attr } => vec![Message::AttributeAddedOrRemoved { msg: ATTRIBUTE_ADDED, plug: whole(*node, *attr) }],
            Event::AttributeRemoved { node, attr } => vec![Message::AttributeAddedOrRemoved { msg: ATTRIBUTE_REMOVED, plug: whole(*node, *attr) }],
            Event::AttributeRenamed { node, attr } => vec![changed(ATTRIBUTE_RENAMED, &whole(*node, *attr), None)],
            Event::Locked { plug, on } => vec![changed(if *on { ATTRIBUTE_LOCKED } else { ATTRIBUTE_UNLOCKED }, plug, None)],
            Event::Keyable { plug, on } => vec![changed(if *on { ATTRIBUTE_KEYABLE } else { ATTRIBUTE_UNKEYABLE }, plug, None)],
            Event::Dirty(p) => vec![Message::NodeDirtyPlug { node: p.node, plug: p.clone() }],
            Event::ParentAdded { child, parent } => vec![Message::ParentAdded { child: *child, parent: *parent }],
            Event::ParentRemoved { child, parent } => vec![Message::ParentRemoved { child: *child, parent: *parent }],
        }
    }

    fn wants(g: &Graph, kind: &Kind, e: &Event, m: &Message) -> bool {
        let of_type = |filter: &Option<String>, class| match filter.as_deref() {
            None | Some("dependNode") => true,
            Some(t) => g.registry.is_a(class, t),
        };
        match (kind, m) {
            (Kind::NodeAdded(f), Message::NodeAdded(_)) => matches!(e, Event::NodeAdded { class, .. } if of_type(f, *class)),
            (Kind::NodeRemoved(f), Message::NodeRemoved(_)) => matches!(e, Event::NodeRemoved { class, .. } if of_type(f, *class)),
            (Kind::Connection, Message::Connection { .. }) => true,
            (Kind::AttributeChanged(n), Message::AttributeChanged { plug, .. }) => plug.node == *n,
            (Kind::AttributeAddedOrRemoved(n), Message::AttributeAddedOrRemoved { plug, .. }) => plug.node == *n,
            (Kind::NodeDirtyPlug(n), Message::NodeDirtyPlug { node, .. }) => node == n,
            (Kind::NameChanged(n), Message::NameChanged { node, .. }) => n.is_none_or(|n| n == *node),
            (Kind::ParentAdded, Message::ParentAdded { .. }) => true,
            (Kind::ParentRemoved, Message::ParentRemoved { .. }) => true,
            (Kind::Event(name), Message::Event(e)) => name == e,
            _ => false,
        }
    }

    /// Calls every callback that wants this event.
    pub fn dispatch(&mut self, g: &Graph, e: &Event) {
        for m in Self::messages_for(e) {
            for entry in &mut self.list {
                if Self::wants(g, &entry.kind, e, &m) {
                    (entry.f)(g, &m);
                }
            }
        }
    }

    /// Calls the callbacks of a named event.
    pub fn post_event(&mut self, g: &Graph, name: &str) {
        let m = Message::Event(name.into());
        for entry in &mut self.list {
            if matches!(&entry.kind, Kind::Event(n) if n == name) {
                (entry.f)(g, &m);
            }
        }
    }
}
