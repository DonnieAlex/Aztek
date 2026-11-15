//! The dependency graph: nodes, connections, push-dirty and pull evaluation.
//!
//! Changing a plug marks everything downstream dirty: the attributes it affects on its node, the
//! destinations of its connections, and what the node adds through `set_dependents_dirty`. Marking
//! stops at plugs that are already dirty. Reading a plug pulls: a dirty destination takes its source's
//! value, a dirty output is computed by its node, anything else is read from the datablock.

use crate::attr::{AttrDef, AttrId, AttrKind, AttrSpec, Owner};
use crate::datablock::DataBlock;
use crate::events::Event;
use crate::node::{NodeId, NodeImpl};
use crate::plug::{Plug, PlugPath, Step};
use crate::registry::{ClassId, Registry};
use crate::store::{coerce, Store};
use crate::types::{DataType, DgError, DisconnectBehavior, Result, Status};
use crate::value::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};

struct Mark {
    path: PlugPath,
    time: u64,
    dirty: bool,
}

struct Node {
    class: ClassId,
    name: String,
    /// Behaviour of the type and the types it derives from, the type itself first.
    impls: Vec<Arc<dyn NodeImpl>>,
    dynamic: Vec<AttrId>,
    store: Store,
    marks: Vec<Mark>,
    locks: Vec<PlugPath>,
    locked: bool,
    flags: HashMap<PlugPath, PlugFlags>,
    aliases: Vec<(String, PlugPath)>,
}

/// Per-plug overrides of the attribute's keyable, channel box and caching flags.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct PlugFlags {
    keyable: Option<bool>,
    channel_box: Option<bool>,
    caching: Option<bool>,
}

struct Entry {
    generation: u32,
    node: Option<Node>,
    /// Set while the node is out of the graph but held (by an undo step): alive, not valid.
    held: Weak<()>,
}

/// A node taken out of the graph, or made and not yet put in. Putting it back restores it under the
/// same id; dropping it ends the node.
pub struct Detached {
    id: NodeId,
    node: Node,
    /// DAG parents and the position under each.
    parents: Vec<(Option<NodeId>, usize)>,
    _alive: Arc<()>,
}

impl Detached {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.node.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.node.name = name.into();
    }

    pub fn class(&self) -> ClassId {
        self.node.class
    }
}

impl DetachedAttr {
    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn attr(&self) -> AttrId {
        self.attr
    }
}

/// A dynamic attribute taken off a node with its values, for undo.
pub struct DetachedAttr {
    node: NodeId,
    attr: AttrId,
    position: usize,
    slot: Option<crate::store::Slot>,
    marks: Vec<Mark>,
    locks: Vec<PlugPath>,
}

/// A DAG node's parents in order, with its position under each.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DagLinks(pub Vec<(Option<NodeId>, usize)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub src: Plug,
    pub dst: Plug,
}

enum Shape {
    Scalar,
    Tuple(usize),
    Compound(Vec<AttrId>),
    Matrix,
    Data(DataType),
    Message,
    Generic,
}

pub struct Graph {
    pub registry: Registry,
    entries: Vec<Entry>,
    incoming: HashMap<NodeId, Vec<Connection>>,
    outgoing: HashMap<NodeId, Vec<Connection>>,
    clock: u64,
    stack: Vec<Plug>,
    warnings: Vec<String>,
    /// DAG parents of each DAG node, in order; `None` is the world.
    pub(crate) parents: HashMap<NodeId, Vec<Option<NodeId>>>,
    /// DAG children of each parent, in order; `None` is the world.
    pub(crate) children: HashMap<Option<NodeId>, Vec<NodeId>>,
    names: HashMap<String, Vec<NodeId>>,
    events: Option<Vec<Event>>,
    listener: Option<Listener>,
}

/// Hears every [`Event`] as it happens, with the graph in the state the change left it in.
pub type Listener = Box<dyn FnMut(&Graph, &Event)>;

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self::with_registry(Registry::new())
    }

    pub fn with_registry(registry: Registry) -> Self {
        Graph {
            registry,
            entries: Vec::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
            clock: 0,
            stack: Vec::new(),
            warnings: Vec::new(),
            parents: HashMap::new(),
            children: HashMap::new(),
            names: HashMap::new(),
            events: None,
            listener: None,
        }
    }

    // ---- nodes -------------------------------------------------------------------------------

    fn node(&self, id: NodeId) -> Result<&Node> {
        self.entries
            .get(id.index as usize)
            .filter(|e| e.generation == id.generation)
            .and_then(|e| e.node.as_ref())
            .ok_or(DgError::NoNode)
    }

    fn node_mut(&mut self, id: NodeId) -> Result<&mut Node> {
        self.entries
            .get_mut(id.index as usize)
            .filter(|e| e.generation == id.generation)
            .and_then(|e| e.node.as_mut())
            .ok_or(DgError::NoNode)
    }

    pub fn nodes(&self) -> Vec<NodeId> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.node.is_some())
            .map(|(i, e)| NodeId { index: i as u32, generation: e.generation })
            .collect()
    }

    /// Whether the node is in the graph.
    pub fn exists(&self, id: NodeId) -> bool {
        self.node(id).is_ok()
    }

    /// Whether the node is in the graph or held out of it (by an undo step).
    pub fn is_alive(&self, id: NodeId) -> bool {
        self.entries
            .get(id.index as usize)
            .filter(|e| e.generation == id.generation)
            .is_some_and(|e| e.node.is_some() || e.held.upgrade().is_some())
    }

    /// Nodes with this name, oldest first.
    pub fn nodes_named(&self, name: &str) -> Vec<NodeId> {
        self.names.get(name).cloned().unwrap_or_default()
    }

    fn name_index_add(&mut self, id: NodeId, name: &str) {
        let l = self.names.entry(name.to_string()).or_default();
        if !l.contains(&id) {
            l.push(id);
            l.sort();
        }
    }

    fn name_index_remove(&mut self, id: NodeId, name: &str) {
        if let Some(l) = self.names.get_mut(name) {
            l.retain(|n| *n != id);
            if l.is_empty() {
                self.names.remove(name);
            }
        }
    }

    /// Creates a node of a registered, non-abstract type. Its computed outputs start dirty.
    pub fn create_node(&mut self, class: &str, name: &str) -> Result<NodeId> {
        let d = self.create_detached(class, name)?;
        self.attach(d)
    }

    /// Makes a node without putting it in the graph: it has an id and is alive, not valid.
    pub fn create_detached(&mut self, class: &str, name: &str) -> Result<Detached> {
        let c = self.registry.class_by_name(class).ok_or_else(|| DgError::NoClass(class.into()))?;
        if self.registry.class(c).is_abstract {
            return Err(DgError::Abstract(class.into()));
        }
        let impls = self
            .registry
            .chain(c)
            .iter()
            .filter_map(|k| self.registry.class(*k).factory.as_ref().map(|f| f()))
            .collect();
        let mut marks = Vec::new();
        for a in self.registry.class_attrs_all(c) {
            if self.registry.is_affected(c, a) {
                self.clock += 1;
                marks.push(Mark { path: self.path_for_attr(a, None), time: self.clock, dirty: true });
            }
        }
        let alive = Arc::new(());
        let id = NodeId { index: self.entries.len() as u32, generation: 0 };
        self.entries.push(Entry { generation: 0, node: None, held: Arc::downgrade(&alive) });
        let parents = if self.registry.is_dag(c) { vec![(None, usize::MAX)] } else { Vec::new() };
        let node = Node {
            class: c,
            name: name.into(),
            impls,
            dynamic: Vec::new(),
            store: Store::default(),
            marks,
            locks: Vec::new(),
            locked: false,
            flags: HashMap::new(),
            aliases: Vec::new(),
        };
        Ok(Detached { id, node, parents, _alive: alive })
    }

    /// Puts a detached node (back) in the graph, under its DAG parents at its old positions.
    pub fn attach(&mut self, d: Detached) -> Result<NodeId> {
        let id = d.id;
        let e = self
            .entries
            .get_mut(id.index as usize)
            .filter(|e| e.generation == id.generation && e.node.is_none())
            .ok_or(DgError::NoNode)?;
        let name = d.node.name.clone();
        let class = d.node.class;
        let dag = self.registry.is_dag(class);
        e.node = Some(d.node);
        e.held = Weak::new();
        self.name_index_add(id, &name);
        if dag {
            self.parents.insert(id, d.parents.iter().map(|p| p.0).collect());
            for (p, pos) in d.parents {
                let l = self.children.entry(p).or_default();
                l.insert(pos.min(l.len()), id);
            }
        }
        self.emit(Event::NodeAdded { node: id, class });
        Ok(id)
    }

    /// Takes a node out of the graph, to be put back by [`Graph::attach`]. It must have no
    /// connections and no DAG children.
    pub fn detach(&mut self, id: NodeId) -> Result<Detached> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        if !self.connections(id).is_empty() {
            return Err(DgError::Invalid(format!("{} still has connections", n.name)));
        }
        if !self.dag_children(Some(id)).is_empty() {
            return Err(DgError::Invalid(format!("{} still has children", n.name)));
        }
        let parents: Vec<(Option<NodeId>, usize)> = self
            .dag_parents(id)
            .into_iter()
            .map(|p| (p, self.children.get(&p).and_then(|l| l.iter().position(|c| *c == id)).unwrap_or(usize::MAX)))
            .collect();
        for (p, _) in &parents {
            if let Some(l) = self.children.get_mut(p) {
                l.retain(|c| *c != id);
            }
        }
        self.parents.remove(&id);
        self.children.remove(&Some(id));
        self.incoming.remove(&id);
        self.outgoing.remove(&id);
        let alive = Arc::new(());
        let e = &mut self.entries[id.index as usize];
        let node = e.node.take().unwrap();
        e.held = Arc::downgrade(&alive);
        let name = node.name.clone();
        let class = node.class;
        self.name_index_remove(id, &name);
        self.emit(Event::NodeRemoved { node: id, class });
        Ok(Detached { id, node, parents, _alive: alive })
    }

    /// Deletes a node, breaking its connections first. A DAG node takes its children with it, except
    /// instances that have another parent, which only lose this one.
    pub fn delete_node(&mut self, id: NodeId) -> Result<()> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        for c in self.children.get(&Some(id)).cloned().unwrap_or_default() {
            if self.parents.get(&c).is_some_and(|p| p.len() > 1) {
                self.remove_parent(c, Some(id))?;
            } else {
                self.delete_node(c)?;
            }
        }
        let mut conns: Vec<Connection> = self.outgoing.get(&id).cloned().unwrap_or_default();
        conns.extend(self.incoming.get(&id).cloned().unwrap_or_default());
        for c in conns {
            if self.source(&c.dst).as_ref() == Some(&c.src) {
                self.disconnect(&c.src, &c.dst)?;
            }
        }
        let d = self.detach(id)?;
        let e = &mut self.entries[id.index as usize];
        e.generation += 1;
        e.held = Weak::new();
        drop(d);
        Ok(())
    }

    pub fn name(&self, id: NodeId) -> Result<&str> {
        Ok(&self.node(id)?.name)
    }

    /// Renames a node as given; unique naming is the API layer's.
    pub fn rename(&mut self, id: NodeId, name: &str) -> Result<()> {
        let n = self.node_mut(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        let old = std::mem::replace(&mut n.name, name.into());
        self.name_index_remove(id, &old);
        self.name_index_add(id, name);
        self.emit(Event::NameChanged { node: id, previous: old });
        Ok(())
    }

    pub fn class_of(&self, id: NodeId) -> Result<ClassId> {
        Ok(self.node(id)?.class)
    }

    pub fn set_node_locked(&mut self, id: NodeId, on: bool) -> Result<()> {
        self.node_mut(id)?.locked = on;
        Ok(())
    }

    pub fn is_node_locked(&self, id: NodeId) -> Result<bool> {
        Ok(self.node(id)?.locked)
    }

    /// Adds a warning for whoever reports them.
    pub fn warn(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    /// Warnings from evaluation (cycles, failed computes), oldest first.
    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }

    // ---- attributes and plugs ----------------------------------------------------------------

    fn attr_on(&self, n: &Node, id: NodeId, a: AttrId) -> bool {
        let root = self.registry.root_of(a);
        match self.registry.attr(root).owner {
            Owner::Class(c) => self.registry.chain(n.class).contains(&c),
            Owner::Dynamic(owner) => owner == id && n.dynamic.contains(&root),
        }
    }

    /// Top-level attributes of a node: its type's, then its dynamic ones.
    pub fn attributes(&self, id: NodeId) -> Result<Vec<AttrId>> {
        let n = self.node(id)?;
        let mut out = self.registry.class_attrs(n.class);
        out.extend(&n.dynamic);
        Ok(out)
    }

    pub fn attr(&self, a: AttrId) -> &AttrDef {
        self.registry.attr(a)
    }

    /// Finds an attribute of a node by long or short name, children included.
    pub fn find_attr(&self, id: NodeId, name: &str) -> Option<AttrId> {
        let n = self.node(id).ok()?;
        self.registry.find_in_class(n.class, name).or_else(|| {
            let mut all = Vec::new();
            for a in &n.dynamic {
                self.registry.with_descendants(*a, &mut all);
            }
            all.into_iter().find(|a| {
                let d = self.registry.attr(*a);
                d.name == name || d.short == name
            })
        })
    }

    /// Adds a dynamic attribute to one node.
    pub fn add_attribute(&mut self, id: NodeId, spec: AttrSpec) -> Result<AttrId> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        let mut names = self.registry.names_in_chain(n.class);
        for a in &n.dynamic {
            let mut all = Vec::new();
            self.registry.with_descendants(*a, &mut all);
            for x in all {
                names.push(self.registry.attr(x).name.clone());
                names.push(self.registry.attr(x).short.clone());
            }
        }
        self.registry.check_names(&spec, &names)?;
        let a = self.registry.add_spec(spec, None, Owner::Dynamic(id), false);
        self.node_mut(id)?.dynamic.push(a);
        self.emit(Event::AttributeAdded { node: id, attr: a });
        Ok(a)
    }

    /// Makes a dynamic attribute for a node without adding it: it has an id, to be added with
    /// [`Graph::put_attribute`].
    pub fn prepare_attribute(&mut self, id: NodeId, spec: AttrSpec) -> Result<DetachedAttr> {
        let names = self.node_attr_names(id)?;
        self.registry.check_names(&spec, &names)?;
        let attr = self.registry.add_spec(spec, None, Owner::Dynamic(id), false);
        Ok(DetachedAttr { node: id, attr, position: usize::MAX, slot: None, marks: Vec::new(), locks: Vec::new() })
    }

    fn node_attr_names(&self, id: NodeId) -> Result<Vec<String>> {
        let n = self.node(id)?;
        let mut names = self.registry.names_in_chain(n.class);
        for a in &n.dynamic {
            let mut all = Vec::new();
            self.registry.with_descendants(*a, &mut all);
            for x in all {
                names.push(self.registry.attr(x).name.clone());
                names.push(self.registry.attr(x).short.clone());
            }
        }
        Ok(names)
    }

    /// Renames a dynamic attribute (or a child of one).
    pub fn rename_attribute(&mut self, id: NodeId, attr: AttrId, short: &str, long: &str) -> Result<()> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        if !n.dynamic.contains(&self.registry.root_of(attr)) {
            return Err(DgError::Invalid(format!("{} is not a dynamic attribute", self.registry.attr(attr).name)));
        }
        let d = self.registry.attr(attr);
        let mine = [d.name.clone(), d.short.clone()];
        let taken: Vec<String> = self.node_attr_names(id)?.into_iter().filter(|x| !mine.contains(x)).collect();
        for x in [short, long] {
            if x.is_empty() || taken.iter().any(|t| t == x) {
                return Err(DgError::NameClash(x.into()));
            }
        }
        self.registry.rename_attr(attr, short, long);
        self.emit(Event::AttributeRenamed { node: id, attr });
        Ok(())
    }

    /// Removes a dynamic attribute, breaking its connections.
    pub fn remove_attribute(&mut self, id: NodeId, attr: AttrId) -> Result<()> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        if !n.dynamic.contains(&attr) {
            return Err(DgError::Invalid(format!("{} is not a dynamic attribute", self.registry.attr(attr).name)));
        }
        let mut conns: Vec<Connection> = Vec::new();
        for c in self.incoming.get(&id).into_iter().flatten() {
            if c.dst.path.0[0].attr == attr {
                conns.push(c.clone());
            }
        }
        for c in self.outgoing.get(&id).into_iter().flatten() {
            if c.src.path.0[0].attr == attr {
                conns.push(c.clone());
            }
        }
        for c in conns {
            if self.source(&c.dst).as_ref() == Some(&c.src) {
                self.disconnect(&c.src, &c.dst)?;
            }
        }
        let n = self.node_mut(id)?;
        n.store.remove_root(attr);
        n.marks.retain(|m| m.path.0[0].attr != attr);
        n.locks.retain(|l| l.0[0].attr != attr);
        n.aliases.retain(|l| l.1 .0[0].attr != attr);
        n.dynamic.retain(|a| *a != attr);
        self.emit(Event::AttributeRemoved { node: id, attr });
        Ok(())
    }

    /// The path to `attr`. Array attributes on the way take their index from `context` when it goes
    /// through the same array, and otherwise stand for every element.
    pub(crate) fn path_for_attr(&self, attr: AttrId, context: Option<&PlugPath>) -> PlugPath {
        PlugPath(
            self.registry
                .lineage(attr)
                .into_iter()
                .map(|a| {
                    let index = if self.registry.attr(a).props.array {
                        context.and_then(|c| c.0.iter().find(|s| s.attr == a)).and_then(|s| s.index)
                    } else {
                        None
                    };
                    Step { attr: a, index }
                })
                .collect(),
        )
    }

    /// Checks a plug against its node. An array attribute without an index is only allowed as the last
    /// step, unless `wildcard` allows it anywhere.
    pub(crate) fn validate(&self, p: &Plug, wildcard: bool) -> Result<()> {
        let n = self.node(p.node)?;
        let steps = &p.path.0;
        if steps.is_empty() {
            return Err(DgError::BadPlug("empty plug".into()));
        }
        let mut prev: Option<AttrId> = None;
        for (k, s) in steps.iter().enumerate() {
            let d = self
                .registry
                .attr_checked(s.attr)
                .ok_or_else(|| DgError::NoAttribute(s.attr.to_string()))?;
            let ok = match prev {
                None => d.parent.is_none() && self.attr_on(n, p.node, s.attr),
                Some(pa) => d.parent == Some(pa),
            };
            if !ok {
                return Err(DgError::BadPlug(format!("{} is not on this plug's path", d.name)));
            }
            if s.index.is_some() && !d.props.array {
                return Err(DgError::BadPlug(format!("{} is not an array", d.name)));
            }
            if s.index.is_none() && d.props.array && k + 1 < steps.len() && !wildcard {
                return Err(DgError::BadPlug(format!("{} needs an index", d.name)));
            }
            prev = Some(s.attr);
        }
        Ok(())
    }

    /// The plug for an attribute outside any array element.
    pub fn plug_for(&self, id: NodeId, attr: AttrId) -> Result<Plug> {
        let p = Plug { node: id, path: self.path_for_attr(attr, None) };
        self.validate(&p, false)?;
        Ok(p)
    }

    /// Parses `name`, `name[3]`, `parent[3].child` or a child's own name.
    pub fn plug(&self, id: NodeId, text: &str) -> Result<Plug> {
        let mut path: Vec<Step> = Vec::new();
        for (k, seg) in text.split('.').enumerate() {
            let (name, index) = match seg.find('[') {
                Some(b) => {
                    let i = seg[b + 1..]
                        .strip_suffix(']')
                        .and_then(|s| s.parse::<u32>().ok())
                        .ok_or_else(|| DgError::BadPlug(text.into()))?;
                    (&seg[..b], Some(i))
                }
                None => (seg, None),
            };
            if k == 0 {
                if let Some((_, aliased)) = self.node(id)?.aliases.iter().find(|a| a.0 == name) {
                    if index.is_some() {
                        return Err(DgError::BadPlug(text.into()));
                    }
                    path = aliased.0.clone();
                    continue;
                }
            }
            let attr = if k == 0 {
                self.find_attr(id, name)
            } else {
                let mut all = Vec::new();
                self.registry.with_descendants(path.last().unwrap().attr, &mut all);
                all.into_iter().skip(1).find(|a| {
                    let d = self.registry.attr(*a);
                    d.name == name || d.short == name
                })
            }
            .ok_or_else(|| DgError::NoAttribute(name.into()))?;
            let lineage = self.registry.lineage(attr);
            let from = if k == 0 {
                0
            } else {
                lineage.iter().position(|a| *a == path.last().unwrap().attr).map_or(0, |p| p + 1)
            };
            path.extend(lineage[from..].iter().map(|a| Step { attr: *a, index: None }));
            if let Some(i) = index {
                path.last_mut().unwrap().index = Some(i);
            }
        }
        let p = Plug { node: id, path: PlugPath(path) };
        self.validate(&p, false)?;
        Ok(p)
    }

    /// `node.attr`, `node.array[3].child`: steps with an index, then the plug's own attribute.
    pub fn plug_name(&self, p: &Plug) -> String {
        let node = self.node(p.node).map(|n| n.name.as_str()).unwrap_or("?");
        let n = p.path.0.len();
        let parts: Vec<String> = p
            .path
            .0
            .iter()
            .enumerate()
            .filter(|(k, s)| s.index.is_some() || *k + 1 == n)
            .map(|(_, s)| {
                let name = &self.registry.attr(s.attr).name;
                match s.index {
                    Some(i) => format!("{name}[{i}]"),
                    None => name.clone(),
                }
            })
            .collect();
        format!("{node}.{}", parts.join("."))
    }

    // ---- values ------------------------------------------------------------------------------

    /// The value of a plug, evaluating what it depends on.
    pub fn get(&mut self, p: &Plug) -> Result<Value> {
        self.validate(p, false)?;
        self.eval(p)
    }

    /// Sets a plug that has no incoming connection, then marks its dependents dirty.
    pub fn set(&mut self, p: &Plug, v: Value) -> Result<()> {
        self.validate(p, false)?;
        if self.lock_blocks(p) {
            return Err(DgError::Locked(self.plug_name(p)));
        }
        if self.covering_destination(p).is_some() || self.destination_below(p) {
            return Err(DgError::ConnectedDestination(self.plug_name(p)));
        }
        let element = p.is_element();
        let v = coerce(&self.registry, p.leaf(), element, v)?;
        let new_element = self.new_element_on(p);
        if self.registry.attr(p.leaf()).props.internal {
            for imp in self.node(p.node)?.impls.clone() {
                imp.set_internal_value(p, &v);
            }
        }
        self.store_write(p, v)?;
        self.mark_clean(p);
        if let Some(e) = new_element {
            self.emit(Event::ElementAdded(e));
        }
        self.emit(Event::ValueSet(p.clone()));
        self.propagate(p);
        Ok(())
    }

    /// Sets a plug back to its default.
    pub fn reset(&mut self, p: &Plug) -> Result<()> {
        self.validate(p, false)?;
        let v = self.default_value(p);
        self.set(p, v)
    }

    fn default_value(&self, p: &Plug) -> Value {
        Store::default().read(&self.registry, &p.path)
    }

    /// Writes a value into the datablock without touching dirty state.
    pub(crate) fn write_value(&mut self, p: &Plug, v: Value) -> Result<()> {
        let v = coerce(&self.registry, p.leaf(), p.is_element(), v)?;
        self.store_write(p, v)
    }

    fn store_write(&mut self, p: &Plug, v: Value) -> Result<()> {
        let reg = &self.registry;
        let n = self
            .entries
            .get_mut(p.node.index as usize)
            .filter(|e| e.generation == p.node.generation)
            .and_then(|e| e.node.as_mut())
            .ok_or(DgError::NoNode)?;
        n.store.write(reg, &p.path, v);
        Ok(())
    }

    fn stored(&self, p: &Plug) -> Value {
        let Ok(n) = self.node(p.node) else { return Value::None };
        if self.registry.attr(p.leaf()).props.internal {
            for imp in &n.impls {
                if let Some(v) = imp.get_internal_value(p) {
                    return v;
                }
            }
        }
        n.store.read(&self.registry, &p.path)
    }

    pub(crate) fn eval(&mut self, p: &Plug) -> Result<Value> {
        if self.stack.contains(p) {
            let msg = format!("cycle on {}", self.plug_name(p));
            self.warnings.push(msg);
            return Ok(self.stored(p));
        }
        self.stack.push(p.clone());
        let r = self.eval_inner(p);
        self.stack.pop();
        r
    }

    fn eval_inner(&mut self, p: &Plug) -> Result<Value> {
        if let Some(dst) = self.covering_destination(p) {
            if self.is_dirty(&dst) {
                self.pull(&dst)?;
            }
            return Ok(self.stored(p));
        }
        let d = self.registry.attr(p.leaf());
        let array_plug = d.props.array && !p.is_element();
        let children = d.children().to_vec();
        let all_cached = self.all_cached(p.leaf());
        let dirty = self.is_dirty(p);
        if dirty && self.is_output(p) {
            self.run_compute(p)?;
        }
        let v = if array_plug {
            let mut items = Vec::new();
            for i in self.elements(p)? {
                items.push((i, self.eval(&p.element(i))?));
            }
            Value::Array(items)
        } else if !children.is_empty() {
            let mut vals = Vec::new();
            for c in children {
                vals.push(self.eval(&p.child(c))?);
            }
            Value::Compound(vals)
        } else {
            self.stored(p)
        };
        if dirty && all_cached {
            self.mark_clean(p);
        }
        Ok(v)
    }

    /// Whether `a` and everything under it is kept in the datablock.
    fn all_cached(&self, a: AttrId) -> bool {
        let mut all = Vec::new();
        self.registry.with_descendants(a, &mut all);
        all.iter().all(|x| self.registry.attr(*x).props.cached)
    }

    /// Whether the node computes this plug: something on its type affects it, a part of it, or a
    /// compound it is part of.
    fn is_output(&self, p: &Plug) -> bool {
        let Ok(n) = self.node(p.node) else { return false };
        let mut attrs = self.registry.lineage(p.leaf());
        self.registry.with_descendants(p.leaf(), &mut attrs);
        attrs.iter().any(|a| self.registry.is_affected(n.class, *a))
    }

    fn run_compute(&mut self, p: &Plug) -> Result<()> {
        let impls = self.node(p.node)?.impls.clone();
        let mut failure = None;
        {
            let mut db = DataBlock { graph: self, plug: p.clone() };
            for imp in &impls {
                match imp.compute(p, &mut db) {
                    Status::UnknownParameter => continue,
                    Status::Success => break,
                    Status::Failure(m) => {
                        failure = Some(m);
                        break;
                    }
                }
            }
        }
        if let Some(m) = failure {
            let msg = format!("{}: {m}", self.plug_name(p));
            self.warnings.push(msg);
        }
        Ok(())
    }

    /// A destination takes its source's value.
    fn pull(&mut self, dst: &Plug) -> Result<()> {
        let Some(src) = self.source(dst) else { return Ok(()) };
        let v = self.eval(&src)?;
        let v = coerce(&self.registry, dst.leaf(), dst.is_element(), v)
            .map_err(|_| DgError::Incompatible(self.plug_name(&src), self.plug_name(dst)))?;
        self.store_write(dst, v)?;
        if self.all_cached(dst.leaf()) {
            self.mark_clean(dst);
        }
        Ok(())
    }

    // ---- dirty state -------------------------------------------------------------------------

    fn add_mark(&mut self, node: NodeId, path: PlugPath, dirty: bool) {
        self.clock += 1;
        let time = self.clock;
        let Ok(n) = self.node_mut(node) else { return };
        n.marks.retain(|m| !path.covers(&m.path));
        if dirty || n.marks.iter().any(|m| m.dirty && m.path.related(&path)) {
            n.marks.push(Mark { path, time, dirty });
        }
    }

    /// Dirty when a dirty mark on the plug, a part of it or what it is part of is newer than the
    /// newest clean mark on the plug or what it is part of.
    pub fn is_dirty(&self, p: &Plug) -> bool {
        let Ok(n) = self.node(p.node) else { return false };
        let (mut d, mut c) = (0, 0);
        for m in &n.marks {
            if m.dirty {
                if m.path.related(&p.path) {
                    d = d.max(m.time);
                }
            } else if m.path.covers(&p.path) {
                c = c.max(m.time);
            }
        }
        d > c
    }

    pub(crate) fn mark_clean(&mut self, p: &Plug) {
        self.add_mark(p.node, p.path.clone(), false);
    }

    /// Marks a plug and its dependents dirty, as the reference's dgdirty does.
    pub fn mark_dirty(&mut self, p: &Plug) -> Result<()> {
        self.validate(p, true)?;
        self.add_mark(p.node, p.path.clone(), true);
        self.propagate(p);
        Ok(())
    }

    fn propagate(&mut self, start: &Plug) {
        let mut stack = vec![start.clone()];
        let mut seen: HashSet<Plug> = HashSet::new();
        seen.insert(start.clone());
        while let Some(p) = stack.pop() {
            for t in self.dependents(&p) {
                if !seen.insert(t.clone()) {
                    continue;
                }
                if self.is_dirty(&t) && self.all_cached(t.leaf()) {
                    continue;
                }
                self.add_mark(t.node, t.path.clone(), true);
                self.emit(Event::Dirty(t.clone()));
                stack.push(t);
            }
        }
    }

    /// What becomes dirty when `p` does.
    fn dependents(&self, p: &Plug) -> Vec<Plug> {
        let Ok(n) = self.node(p.node) else { return Vec::new() };
        let mut attrs = self.registry.lineage(p.leaf());
        let mut below = Vec::new();
        self.registry.with_descendants(p.leaf(), &mut below);
        attrs.extend(below.into_iter().skip(1));
        let mut out = Vec::new();
        for a in &attrs {
            for b in self.registry.affected_by(n.class, *a) {
                out.push(Plug { node: p.node, path: self.path_for_attr(b, Some(&p.path)) });
            }
        }
        // World-space attributes of a DAG node and everything under it follow what affects world space.
        if self.parents.contains_key(&p.node) {
            let moves = attrs.iter().any(|a| self.registry.attr(*a).props.affects_world_space);
            let world = attrs.iter().any(|a| self.registry.attr(*a).props.world_space);
            if moves || world {
                let mut nodes = if moves { vec![p.node] } else { Vec::new() };
                nodes.extend(self.dag_descendants(p.node));
                for m in nodes {
                    out.extend(self.world_space_plugs(m).into_iter().filter(|w| !(m == p.node && w.path.related(&p.path))));
                }
            }
        }
        for c in self.outgoing.get(&p.node).into_iter().flatten() {
            if c.src.path.related(&p.path) {
                out.push(c.dst.clone());
            }
        }
        for imp in &n.impls {
            imp.set_dependents_dirty(self, p, &mut out);
        }
        let mut seen = HashSet::new();
        out.retain(|x| seen.insert(x.clone()));
        out
    }

    // ---- connections -------------------------------------------------------------------------

    /// The connection destination that is this plug or a plug it is part of.
    fn covering_destination(&self, p: &Plug) -> Option<Plug> {
        self.incoming.get(&p.node)?.iter().find(|c| c.dst.path.covers(&p.path)).map(|c| c.dst.clone())
    }

    fn destination_below(&self, p: &Plug) -> bool {
        self.incoming.get(&p.node).into_iter().flatten().any(|c| p.path.covers(&c.dst.path))
    }

    pub fn source(&self, dst: &Plug) -> Option<Plug> {
        self.incoming.get(&dst.node)?.iter().find(|c| c.dst == *dst).map(|c| c.src.clone())
    }

    pub fn destinations(&self, src: &Plug) -> Vec<Plug> {
        self.outgoing.get(&src.node).into_iter().flatten().filter(|c| c.src == *src).map(|c| c.dst.clone()).collect()
    }

    /// Every connection into and out of a node.
    pub fn connections(&self, id: NodeId) -> Vec<Connection> {
        let mut out: Vec<Connection> = self.incoming.get(&id).cloned().unwrap_or_default();
        for c in self.outgoing.get(&id).into_iter().flatten() {
            if !out.contains(c) {
                out.push(c.clone());
            }
        }
        out
    }

    fn shape(&self, a: AttrId) -> Shape {
        match &self.registry.attr(a).kind {
            AttrKind::Numeric { ty, .. } if ty.arity() > 1 => Shape::Tuple(ty.arity()),
            AttrKind::Numeric { .. } | AttrKind::Unit { .. } | AttrKind::Enum { .. } => Shape::Scalar,
            AttrKind::Typed { ty: DataType::Matrix, .. } | AttrKind::Matrix { .. } => Shape::Matrix,
            AttrKind::Typed { ty, .. } => Shape::Data(*ty),
            AttrKind::Message => Shape::Message,
            AttrKind::Compound { children, .. } => Shape::Compound(children.clone()),
            AttrKind::Generic { .. } => Shape::Generic,
        }
    }

    fn all_scalar(&self, c: &[AttrId]) -> bool {
        c.iter().all(|x| matches!(self.shape(*x), Shape::Scalar))
    }

    /// Whether `src` can feed `dst`, by kind. Unit conversion is the command layer's.
    pub fn compatible(&self, src: AttrId, dst: AttrId) -> bool {
        let (s, d) = (self.shape(src), self.shape(dst));
        match (&s, &d) {
            (Shape::Message, Shape::Message) => true,
            (Shape::Message, _) | (_, Shape::Message) => false,
            (Shape::Generic, _) => true,
            (_, Shape::Generic) => {
                let AttrKind::Generic { numeric, data } = &self.registry.attr(dst).kind else { return false };
                let arity = |n: usize| numeric.iter().any(|t| t.arity() == n);
                match &s {
                    Shape::Scalar => arity(1),
                    Shape::Tuple(n) => arity(*n),
                    Shape::Compound(c) => self.all_scalar(c) && arity(c.len()),
                    Shape::Matrix => data.iter().any(|t| t.accepts(DataType::Matrix)),
                    Shape::Data(t) => data.iter().any(|x| x.accepts(*t)),
                    _ => false,
                }
            }
            (Shape::Scalar, Shape::Scalar) => true,
            (Shape::Tuple(a), Shape::Tuple(b)) => a == b,
            (Shape::Tuple(n), Shape::Compound(c)) | (Shape::Compound(c), Shape::Tuple(n)) => {
                c.len() == *n && self.all_scalar(c)
            }
            (Shape::Compound(a), Shape::Compound(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| self.compatible(*x, *y))
            }
            (Shape::Matrix, Shape::Matrix) => true,
            (Shape::Data(a), Shape::Data(b)) => b.accepts(*a),
            _ => false,
        }
    }

    /// Connects `src` to `dst`. With `force`, connections already into `dst` (or a plug it is part
    /// of, or a part of it) are broken first; without, they are an error.
    pub fn connect(&mut self, src: &Plug, dst: &Plug, force: bool) -> Result<()> {
        self.validate(src, false)?;
        self.validate(dst, false)?;
        let (sn, dn) = (self.plug_name(src), self.plug_name(dst));
        let (sd, dd) = (self.registry.attr(src.leaf()), self.registry.attr(dst.leaf()));
        if !sd.props.connectable {
            return Err(DgError::NotConnectable(sn));
        }
        if !dd.props.connectable {
            return Err(DgError::NotConnectable(dn));
        }
        if !sd.props.readable {
            return Err(DgError::NotReadable(sn));
        }
        if !dd.props.writable {
            return Err(DgError::NotWritable(dn));
        }
        if src == dst {
            return Err(DgError::Invalid(format!("{sn} cannot connect to itself")));
        }
        if self.lock_blocks(dst) {
            return Err(DgError::Locked(dn));
        }
        let s_array = sd.props.array && !src.is_element();
        let d_array = dd.props.array && !dst.is_element();
        if s_array != d_array || !self.compatible(src.leaf(), dst.leaf()) {
            return Err(DgError::Incompatible(sn, dn));
        }
        let existing: Vec<Connection> = self
            .incoming
            .get(&dst.node)
            .into_iter()
            .flatten()
            .filter(|c| c.dst.path.related(&dst.path))
            .cloned()
            .collect();
        if existing.iter().any(|c| c.src == *src && c.dst == *dst) || (!existing.is_empty() && !force) {
            return Err(DgError::AlreadyConnected(dn));
        }
        for c in existing {
            self.disconnect(&c.src, &c.dst)?;
        }
        let new_elements: Vec<Plug> = [src, dst].into_iter().filter_map(|p| self.new_element_on(p)).collect();
        {
            let reg = &self.registry;
            for p in [src, dst] {
                if let Some(n) = self.entries.get_mut(p.node.index as usize).and_then(|e| e.node.as_mut()) {
                    n.store.ensure(reg, &p.path);
                }
            }
        }
        let c = Connection { src: src.clone(), dst: dst.clone() };
        self.incoming.entry(dst.node).or_default().push(c.clone());
        self.outgoing.entry(src.node).or_default().push(c);
        for imp in self.node(src.node)?.impls.clone() {
            imp.connection_made(src, dst, true);
        }
        for imp in self.node(dst.node)?.impls.clone() {
            imp.connection_made(dst, src, false);
        }
        for e in new_elements {
            self.emit(Event::ElementAdded(e));
        }
        self.emit(Event::Connected { src: src.clone(), dst: dst.clone() });
        self.add_mark(dst.node, dst.path.clone(), true);
        self.propagate(dst);
        Ok(())
    }

    /// Breaks a connection. The destination then keeps its value, resets to its default or, for an
    /// array element, is deleted, as its attribute's disconnect behaviour says.
    pub fn disconnect(&mut self, src: &Plug, dst: &Plug) -> Result<()> {
        let found = self.incoming.get(&dst.node).is_some_and(|l| l.iter().any(|c| c.src == *src && c.dst == *dst));
        if !found {
            return Err(DgError::NotConnected(self.plug_name(src), self.plug_name(dst)));
        }
        let behavior = self.registry.attr(dst.leaf()).props.disconnect_behavior;
        if behavior == DisconnectBehavior::Nothing && self.is_dirty(dst) {
            if let Err(e) = self.pull(dst) {
                self.warnings.push(e.to_string());
            }
        }
        if let Some(l) = self.incoming.get_mut(&dst.node) {
            l.retain(|c| !(c.src == *src && c.dst == *dst));
        }
        if let Some(l) = self.outgoing.get_mut(&src.node) {
            l.retain(|c| !(c.src == *src && c.dst == *dst));
        }
        let mut removed = false;
        match behavior {
            DisconnectBehavior::Reset => {
                let v = self.default_value(dst);
                self.store_write(dst, v)?;
            }
            DisconnectBehavior::Delete if dst.is_element() => {
                let n = self.node_mut(dst.node)?;
                n.store.remove_element(&dst.path);
                n.marks.retain(|m| !dst.path.covers(&m.path));
                n.locks.retain(|l| !dst.path.covers(l));
                removed = true;
            }
            _ => {}
        }
        if let Ok(n) = self.node(src.node) {
            for imp in n.impls.clone() {
                imp.connection_broken(src, dst, true);
            }
        }
        if let Ok(n) = self.node(dst.node) {
            for imp in n.impls.clone() {
                imp.connection_broken(dst, src, false);
            }
        }
        self.emit(Event::Disconnected { src: src.clone(), dst: dst.clone() });
        if removed {
            self.emit(Event::ElementRemoved(dst.clone()));
            let array = Plug { node: dst.node, path: dst.path.array_of_element().unwrap() };
            self.propagate(&array);
        } else {
            self.mark_clean(dst);
            self.propagate(dst);
        }
        Ok(())
    }

    // ---- arrays ------------------------------------------------------------------------------

    /// The logical index of the element of array `array` that `other` is in or under.
    fn index_under(array: &PlugPath, other: &PlugPath) -> Option<u32> {
        let n = array.0.len();
        if other.0.len() < n || array.0[..n - 1] != other.0[..n - 1] || array.0[n - 1].attr != other.0[n - 1].attr {
            return None;
        }
        other.0[n - 1].index
    }

    /// The logical indices an array plug has: those in the datablock and those connected.
    pub fn elements(&self, p: &Plug) -> Result<Vec<u32>> {
        let d = self.registry.attr(p.leaf());
        if !d.props.array || p.is_element() {
            return Err(DgError::BadPlug(format!("{} is not an array plug", self.plug_name(p))));
        }
        let mut idx = self.node(p.node)?.store.indices(&p.path);
        if d.props.world_space && p.path.0.len() == 1 && self.parents.contains_key(&p.node) {
            idx.extend(0..self.instance_count(p.node) as u32);
        }
        for c in self.incoming.get(&p.node).into_iter().flatten() {
            idx.extend(Self::index_under(&p.path, &c.dst.path));
        }
        for c in self.outgoing.get(&p.node).into_iter().flatten() {
            idx.extend(Self::index_under(&p.path, &c.src.path));
        }
        idx.sort_unstable();
        idx.dedup();
        Ok(idx)
    }

    /// Drops stored elements at `keep` and above (world-space arrays after instances go).
    pub(crate) fn trim_elements(&mut self, p: &Plug, keep: u32) {
        let Ok(n) = self.node_mut(p.node) else { return };
        for i in n.store.indices(&p.path) {
            if i >= keep {
                let e = p.path.with_index(i);
                n.store.remove_element(&e);
                n.marks.retain(|m| !e.covers(&m.path));
            }
        }
    }

    /// Removes an array element. Connected elements need `break_connections`.
    pub fn remove_element(&mut self, p: &Plug, break_connections: bool) -> Result<()> {
        self.validate(p, false)?;
        if !p.is_element() {
            return Err(DgError::BadPlug(format!("{} is not an array element", self.plug_name(p))));
        }
        let mut conns: Vec<Connection> = Vec::new();
        for c in self.incoming.get(&p.node).into_iter().flatten() {
            if p.path.covers(&c.dst.path) {
                conns.push(c.clone());
            }
        }
        for c in self.outgoing.get(&p.node).into_iter().flatten() {
            if p.path.covers(&c.src.path) && !conns.contains(c) {
                conns.push(c.clone());
            }
        }
        if !conns.is_empty() && !break_connections {
            return Err(DgError::Invalid(format!("{} is connected", self.plug_name(p))));
        }
        for c in conns {
            if self.source(&c.dst).as_ref() == Some(&c.src) {
                self.disconnect(&c.src, &c.dst)?;
            }
        }
        let n = self.node_mut(p.node)?;
        n.store.remove_element(&p.path);
        n.marks.retain(|m| !p.path.covers(&m.path));
        n.locks.retain(|l| !p.path.covers(l));
        n.aliases.retain(|l| !p.path.covers(&l.1));
        self.emit(Event::ElementRemoved(p.clone()));
        let array = Plug { node: p.node, path: p.path.array_of_element().unwrap() };
        self.propagate(&array);
        Ok(())
    }

    /// The lowest logical index of an array with no incoming connection.
    pub fn next_available(&self, p: &Plug) -> Result<u32> {
        let used = self
            .incoming
            .get(&p.node)
            .into_iter()
            .flatten()
            .filter_map(|c| Self::index_under(&p.path, &c.dst.path))
            .collect::<HashSet<u32>>();
        let d = self.registry.attr(p.leaf());
        if !d.props.array || p.is_element() {
            return Err(DgError::BadPlug(format!("{} is not an array plug", self.plug_name(p))));
        }
        Ok((0..).find(|i| !used.contains(i)).unwrap())
    }

    // ---- locks -------------------------------------------------------------------------------

    /// Locks or unlocks a plug. A locked plug, a plug under it or a plug it is part of cannot be set
    /// or made a destination.
    pub fn set_locked(&mut self, p: &Plug, on: bool) -> Result<()> {
        self.validate(p, false)?;
        let n = self.node_mut(p.node)?;
        let was = n.locks.contains(&p.path);
        n.locks.retain(|l| *l != p.path);
        if on {
            n.locks.push(p.path.clone());
        }
        if was != on {
            self.emit(Event::Locked { plug: p.clone(), on });
        }
        Ok(())
    }

    /// The locks on this plug and the plugs under it.
    pub fn locks_under(&self, p: &Plug) -> Vec<PlugPath> {
        self.node(p.node).map(|n| n.locks.iter().filter(|l| p.path.covers(l)).cloned().collect()).unwrap_or_default()
    }

    /// Whether this exact plug carries a lock of its own.
    pub fn has_own_lock(&self, p: &Plug) -> bool {
        self.node(p.node).is_ok_and(|n| n.locks.contains(&p.path))
    }

    /// Whether the plug, or a plug it is part of, is locked.
    pub fn is_locked(&self, p: &Plug) -> bool {
        self.node(p.node).is_ok_and(|n| n.locks.iter().any(|l| l.covers(&p.path)))
    }

    /// Whether a lock on the plug, on a part of it or on what it is part of stops it changing.
    fn lock_blocks(&self, p: &Plug) -> bool {
        self.node(p.node).is_ok_and(|n| n.locks.iter().any(|l| l.related(&p.path)))
    }
}

// ---- what undo and the API layer need ---------------------------------------------------------

impl Graph {
    /// Takes a dynamic attribute off its node with its values; it must have no connections.
    pub fn take_attribute(&mut self, id: NodeId, attr: AttrId) -> Result<DetachedAttr> {
        let n = self.node(id)?;
        if n.locked {
            return Err(DgError::Locked(n.name.clone()));
        }
        let position = n
            .dynamic
            .iter()
            .position(|a| *a == attr)
            .ok_or_else(|| DgError::Invalid(format!("{} is not a dynamic attribute", self.registry.attr(attr).name)))?;
        let connected = self.connections(id).iter().any(|c| {
            (c.src.node == id && c.src.path.0[0].attr == attr) || (c.dst.node == id && c.dst.path.0[0].attr == attr)
        });
        if connected {
            return Err(DgError::Invalid(format!("{} is connected", self.registry.attr(attr).name)));
        }
        let n = self.node_mut(id)?;
        n.dynamic.remove(position);
        let slot = n.store.take_root(attr);
        let (marks, keep): (Vec<Mark>, Vec<Mark>) = std::mem::take(&mut n.marks).into_iter().partition(|m| m.path.0[0].attr == attr);
        n.marks = keep;
        let (locks, keep): (Vec<PlugPath>, Vec<PlugPath>) = std::mem::take(&mut n.locks).into_iter().partition(|l| l.0[0].attr == attr);
        n.locks = keep;
        n.flags.retain(|k, _| k.0[0].attr != attr);
        n.aliases.retain(|l| l.1 .0[0].attr != attr);
        self.emit(Event::AttributeRemoved { node: id, attr });
        Ok(DetachedAttr { node: id, attr, position, slot, marks, locks })
    }

    /// Puts a taken dynamic attribute back, values included.
    pub fn put_attribute(&mut self, d: DetachedAttr) -> Result<AttrId> {
        let n = self.node_mut(d.node)?;
        n.dynamic.insert(d.position.min(n.dynamic.len()), d.attr);
        if let Some(slot) = d.slot {
            n.store.put_root(d.attr, slot);
        }
        n.marks.extend(d.marks);
        n.locks.extend(d.locks);
        self.emit(Event::AttributeAdded { node: d.node, attr: d.attr });
        Ok(d.attr)
    }

    /// Writes a value whatever locks and connections say, then dirties dependents; for undo.
    pub fn force_value(&mut self, p: &Plug, v: Value) -> Result<()> {
        self.validate(p, false)?;
        let new_element = self.new_element_on(p);
        self.write_value(p, v)?;
        self.mark_clean(p);
        if let Some(e) = new_element {
            self.emit(Event::ElementAdded(e));
        }
        self.emit(Event::ValueSet(p.clone()));
        self.propagate(p);
        Ok(())
    }

    /// The value in the datablock, without evaluating.
    pub fn stored_value(&self, p: &Plug) -> Result<Value> {
        self.validate(p, false)?;
        Ok(self.stored(p))
    }

    /// Whether an array element exists in the datablock or through a connection.
    pub fn has_element(&self, p: &Plug) -> bool {
        let Some(array) = p.path.array_of_element() else { return false };
        let index = p.path.0.last().unwrap().index.unwrap();
        self.elements(&Plug { node: p.node, path: array }).is_ok_and(|e| e.contains(&index))
    }

    /// The parents of a DAG node and its position under each.
    pub fn dag_links(&self, id: NodeId) -> DagLinks {
        DagLinks(
            self.dag_parents(id)
                .into_iter()
                .map(|p| (p, self.children.get(&p).and_then(|l| l.iter().position(|c| *c == id)).unwrap_or(usize::MAX)))
                .collect(),
        )
    }

    /// Puts a DAG node back under exactly these parents at these positions; for undo.
    pub fn restore_dag_links(&mut self, id: NodeId, links: &DagLinks) -> Result<()> {
        if !self.is_dag(id) {
            return Err(DgError::NoNode);
        }
        let before = self.dag_parents(id);
        for p in &before {
            if !links.0.iter().any(|l| l.0 == *p) {
                self.emit(Event::ParentRemoved { child: id, parent: *p });
            }
        }
        for (p, _) in &links.0 {
            if !before.contains(p) {
                self.emit(Event::ParentAdded { child: id, parent: *p });
            }
        }
        for p in self.dag_parents(id) {
            if let Some(l) = self.children.get_mut(&p) {
                l.retain(|c| *c != id);
            }
        }
        self.parents.insert(id, links.0.iter().map(|l| l.0).collect());
        for (p, pos) in &links.0 {
            let l = self.children.entry(*p).or_default();
            l.insert((*pos).min(l.len()), id);
        }
        self.dag_changed(id);
        Ok(())
    }

    fn flag(&self, p: &Plug, pick: impl Fn(&PlugFlags) -> Option<bool>) -> Option<bool> {
        self.node(p.node).ok()?.flags.get(&p.path).and_then(pick)
    }

    fn set_flag(&mut self, p: &Plug, f: impl Fn(&mut PlugFlags)) -> Result<()> {
        self.validate(p, false)?;
        let n = self.node_mut(p.node)?;
        f(n.flags.entry(p.path.clone()).or_default());
        Ok(())
    }

    /// Keyable: the plug's own setting, else its attribute's.
    pub fn plug_keyable(&self, p: &Plug) -> bool {
        self.flag(p, |f| f.keyable).unwrap_or(self.registry.attr(p.leaf()).props.keyable)
    }

    pub fn set_plug_keyable(&mut self, p: &Plug, on: bool) -> Result<()> {
        let was = self.plug_keyable(p);
        self.set_flag(p, |f| f.keyable = Some(on))?;
        if was != on {
            self.emit(Event::Keyable { plug: p.clone(), on });
        }
        Ok(())
    }

    /// In the channel box: the plug's own setting, else its attribute's.
    pub fn plug_channel_box(&self, p: &Plug) -> bool {
        self.flag(p, |f| f.channel_box).unwrap_or(self.registry.attr(p.leaf()).props.channel_box)
    }

    pub fn set_plug_channel_box(&mut self, p: &Plug, on: bool) -> Result<()> {
        self.set_flag(p, |f| f.channel_box = Some(on))
    }

    /// Caching: the plug's own setting, else its attribute's.
    pub fn plug_caching(&self, p: &Plug) -> bool {
        self.flag(p, |f| f.caching).unwrap_or(self.registry.attr(p.leaf()).props.cached)
    }

    pub fn set_plug_caching(&mut self, p: &Plug, on: bool) -> Result<()> {
        self.set_flag(p, |f| f.caching = Some(on))
    }
}

// ---- events and aliases -----------------------------------------------------------------------

impl Graph {
    /// Starts or stops recording [`Event`]s; stopping drops what was not taken.
    pub fn set_events(&mut self, on: bool) {
        self.events = on.then(Vec::new);
    }

    pub fn take_events(&mut self) -> Vec<Event> {
        self.events.as_mut().map(std::mem::take).unwrap_or_default()
    }

    /// Sets the listener that hears each event as it happens.
    pub fn set_listener(&mut self, l: Option<Listener>) {
        self.listener = l;
    }

    pub(crate) fn emit(&mut self, e: Event) {
        if let Some(mut l) = self.listener.take() {
            l(self, &e);
            if self.listener.is_none() {
                self.listener = Some(l);
            }
        }
        if let Some(l) = &mut self.events {
            l.push(e);
        }
    }

    /// The outermost array element on the plug's path that does not exist yet.
    pub fn new_element_on(&self, p: &Plug) -> Option<Plug> {
        (1..=p.path.0.len()).find_map(|k| {
            let mut path = p.path.clone();
            path.0.truncate(k);
            let q = Plug { node: p.node, path };
            (q.is_element() && !self.has_element(&q)).then_some(q)
        })
    }

    /// Gives a plug an alias (`add`), or removes the alias `alias` (not `add`). An alias may not be an
    /// attribute name or another plug's alias; a plug has one alias at most. Returns whether anything
    /// changed.
    pub fn set_alias(&mut self, id: NodeId, alias: &str, p: &Plug, add: bool) -> Result<bool> {
        if !add {
            let n = self.node_mut(id)?;
            let before = n.aliases.len();
            n.aliases.retain(|a| a.0 != alias);
            return Ok(n.aliases.len() != before);
        }
        self.validate(p, false)?;
        if p.node != id {
            return Err(DgError::BadPlug("the plug is on another node".into()));
        }
        if alias.is_empty() || self.node_attr_names(id)?.iter().any(|n| n == alias) {
            return Err(DgError::NameClash(alias.into()));
        }
        let n = self.node_mut(id)?;
        if n.aliases.iter().any(|a| a.0 == alias && a.1 != p.path) {
            return Err(DgError::NameClash(alias.into()));
        }
        n.aliases.retain(|a| a.1 != p.path);
        n.aliases.push((alias.into(), p.path.clone()));
        Ok(true)
    }

    /// A node's aliases, in the order they were made.
    pub fn aliases(&self, id: NodeId) -> Result<Vec<(String, Plug)>> {
        Ok(self.node(id)?.aliases.iter().map(|(a, path)| (a.clone(), Plug { node: id, path: path.clone() })).collect())
    }

    pub fn plug_alias(&self, p: &Plug) -> Option<String> {
        self.node(p.node).ok()?.aliases.iter().find(|a| a.1 == p.path).map(|a| a.0.clone())
    }
}
