//! Modifiers: the one way commands change the graph. Operations are recorded, applied by `do_it`,
//! reverted by `undo_it`, and applied again by the next `do_it`. Each operation runs as a sequence of
//! primitive changes, and every primitive hands back its own inverse, so undo and redo replay exactly
//! what happened, node ids and attribute ids included.

use crate::naming;
use aztek_dg::{AttrId, AttrSpec, DagLinks, Detached, DetachedAttr, DgError, Graph, NodeId, Plug, Result, Value};

enum Prim {
    Attach(Detached),
    Detach(NodeId),
    Rename(NodeId, String),
    NodeLock(NodeId, bool),
    PlugLock(Plug, bool),
    Connect(Plug, Plug),
    Disconnect(Plug, Plug),
    /// The last flag forces the value past locks and connections (undo only).
    SetValue(Plug, Value, bool),
    RemoveElement(Plug),
    PutAttr(DetachedAttr),
    TakeAttr(NodeId, AttrId),
    RenameAttr(NodeId, AttrId, String, String),
    Links(NodeId, DagLinks),
    Seq(Vec<Prim>),
}

/// Applies a primitive and returns what undoes it.
fn apply(g: &mut Graph, p: Prim) -> Result<Prim> {
    Ok(match p {
        Prim::Attach(d) => Prim::Detach(g.attach(d)?),
        Prim::Detach(id) => Prim::Attach(g.detach(id)?),
        Prim::Rename(id, name) => {
            let old = g.name(id)?.to_string();
            g.rename(id, &name)?;
            Prim::Rename(id, old)
        }
        Prim::NodeLock(id, on) => {
            let old = g.is_node_locked(id)?;
            g.set_node_locked(id, on)?;
            Prim::NodeLock(id, old)
        }
        Prim::PlugLock(p, on) => {
            let old = g.has_own_lock(&p);
            g.set_locked(&p, on)?;
            Prim::PlugLock(p, old)
        }
        Prim::Connect(s, d) => {
            let new_element = g.new_element_on(&d);
            let old = g.stored_value(&d)?;
            g.connect(&s, &d, false)?;
            let restore = match new_element {
                Some(e) => Prim::RemoveElement(e),
                None => Prim::SetValue(d.clone(), old, true),
            };
            Prim::Seq(vec![Prim::Disconnect(s, d), restore])
        }
        Prim::Disconnect(s, d) => {
            g.disconnect(&s, &d)?;
            Prim::Connect(s, d)
        }
        Prim::SetValue(p, v, forced) => {
            let new_element = g.new_element_on(&p);
            let old = g.stored_value(&p)?;
            if forced {
                g.force_value(&p, v)?;
            } else {
                g.set(&p, v)?;
            }
            match new_element {
                Some(e) => Prim::RemoveElement(e),
                None => Prim::SetValue(p, old, true),
            }
        }
        Prim::RemoveElement(p) => {
            let old = g.stored_value(&p)?;
            g.remove_element(&p, false)?;
            Prim::SetValue(p, old, true)
        }
        Prim::PutAttr(d) => {
            let node = d.node();
            Prim::TakeAttr(node, g.put_attribute(d)?)
        }
        Prim::TakeAttr(n, a) => Prim::PutAttr(g.take_attribute(n, a)?),
        Prim::RenameAttr(n, a, short, long) => {
            let d = g.attr(a);
            let (old_short, old_long) = (d.short.clone(), d.name.clone());
            g.rename_attribute(n, a, &short, &long)?;
            Prim::RenameAttr(n, a, old_short, old_long)
        }
        Prim::Links(id, links) => {
            let old = g.dag_links(id);
            g.restore_dag_links(id, &links)?;
            Prim::Links(id, old)
        }
        Prim::Seq(items) => {
            let mut inv = Vec::new();
            for it in items {
                match apply(g, it) {
                    Ok(i) => inv.push(i),
                    Err(e) => {
                        for i in inv.into_iter().rev() {
                            let _ = apply(g, i);
                        }
                        return Err(e);
                    }
                }
            }
            inv.reverse();
            Prim::Seq(inv)
        }
    })
}

enum Op {
    Create { node: Option<Detached>, desired: String, parent: Option<NodeId> },
    Delete { node: NodeId },
    Rename { node: NodeId, name: String },
    NodeLock { node: NodeId, on: bool },
    PlugLock { plug: Plug, on: bool },
    Connect { src: Plug, dst: Plug },
    Disconnect { src: Plug, dst: Plug },
    AddAttr { attr: Option<DetachedAttr> },
    RemoveAttr { node: NodeId, attr: AttrId },
    RenameAttr { node: NodeId, attr: AttrId, short: String, long: String },
    SetValue { plug: Plug, value: Value },
    RemoveElement { plug: Plug, break_connections: bool },
    Reparent { node: NodeId, parent: Option<NodeId>, add_instance: bool, index: Option<usize> },
}

/// Records operations on the graph and applies them as one undoable unit.
#[derive(Default)]
pub struct DgModifier {
    pending: Vec<Op>,
    /// Inverses of what was applied, oldest first.
    done: Vec<Prim>,
    /// Inverses of what undo applied, newest undo first.
    undone: Vec<Prim>,
}

fn step(g: &mut Graph, log: &mut Vec<Prim>, p: Prim) -> Result<()> {
    log.push(apply(g, p)?);
    Ok(())
}

fn connections_on(g: &Graph, node: NodeId, f: impl Fn(&Plug) -> bool) -> Vec<(Plug, Plug)> {
    g.connections(node)
        .into_iter()
        .filter(|c| (c.src.node == node && f(&c.src)) || (c.dst.node == node && f(&c.dst)))
        .map(|c| (c.src, c.dst))
        .collect()
}

fn delete(g: &mut Graph, log: &mut Vec<Prim>, id: NodeId) -> Result<()> {
    for c in g.dag_children(Some(id)) {
        let parents = g.dag_parents(c);
        if parents.len() > 1 {
            let links = DagLinks(g.dag_links(c).0.into_iter().filter(|l| l.0 != Some(id)).collect());
            step(g, log, Prim::Links(c, links))?;
        } else {
            delete(g, log, c)?;
        }
    }
    for (s, d) in connections_on(g, id, |_| true) {
        step(g, log, Prim::Disconnect(s, d))?;
    }
    step(g, log, Prim::Detach(id))
}

/// The nodes a delete of `id` takes with it.
fn deleted_with(g: &Graph, id: NodeId, out: &mut Vec<NodeId>) {
    out.push(id);
    for c in g.dag_children(Some(id)) {
        if g.dag_parents(c).len() == 1 {
            deleted_with(g, c, out);
        }
    }
}

fn run(g: &mut Graph, log: &mut Vec<Prim>, op: Op) -> Result<()> {
    match op {
        Op::Create { node, desired, parent } => {
            let mut d = node.ok_or_else(|| DgError::Invalid("node already created".into()))?;
            let dag = g.registry.is_dag(d.class());
            let parents = [parent];
            let name = naming::unique_name(g, &desired, None, dag.then_some(&parents[..]))?;
            d.set_name(&name);
            let id = d.id();
            step(g, log, Prim::Attach(d))?;
            if dag && parent.is_some() {
                step(g, log, Prim::Links(id, DagLinks(vec![(parent, usize::MAX)])))?;
            }
            Ok(())
        }
        Op::Delete { node } => {
            let mut all = Vec::new();
            deleted_with(g, node, &mut all);
            for n in all {
                if g.is_node_locked(n)? {
                    return Err(DgError::Locked(g.name(n)?.into()));
                }
            }
            delete(g, log, node)
        }
        Op::Rename { node, name } => {
            let unique = crate::node::unique_for(g, node, &name)?;
            step(g, log, Prim::Rename(node, unique))
        }
        Op::NodeLock { node, on } => step(g, log, Prim::NodeLock(node, on)),
        Op::PlugLock { plug, on } => step(g, log, Prim::PlugLock(plug, on)),
        Op::Connect { src, dst } => step(g, log, Prim::Connect(src, dst)),
        Op::Disconnect { src, dst } => step(g, log, Prim::Disconnect(src, dst)),
        Op::AddAttr { attr } => step(g, log, Prim::PutAttr(attr.ok_or_else(|| DgError::Invalid("attribute already added".into()))?)),
        Op::RemoveAttr { node, attr } => {
            for (s, d) in connections_on(g, node, |p| p.path.0[0].attr == attr) {
                step(g, log, Prim::Disconnect(s, d))?;
            }
            step(g, log, Prim::TakeAttr(node, attr))
        }
        Op::RenameAttr { node, attr, short, long } => step(g, log, Prim::RenameAttr(node, attr, short, long)),
        Op::SetValue { plug, value } => step(g, log, Prim::SetValue(plug, value, false)),
        Op::RemoveElement { plug, break_connections } => {
            if !plug.is_element() {
                return Err(DgError::BadPlug(format!("{} is not an array element", g.plug_name(&plug))));
            }
            let conns = connections_on(g, plug.node, |p| plug.path.covers(&p.path));
            if !conns.is_empty() && !break_connections {
                return Err(DgError::Invalid(format!("{} is connected", g.plug_name(&plug))));
            }
            for (s, d) in conns {
                step(g, log, Prim::Disconnect(s, d))?;
            }
            if g.has_element(&plug) {
                step(g, log, Prim::RemoveElement(plug))?;
            }
            Ok(())
        }
        Op::Reparent { node, parent, add_instance, index } => {
            if !g.is_dag(node) {
                return Err(DgError::Invalid(format!("{} is not a DAG node", g.name(node)?)));
            }
            if let Some(p) = parent {
                if !g.is_dag(p) {
                    return Err(DgError::Invalid(format!("{} is not a DAG node", g.name(p)?)));
                }
                if p == node || g.dag_descendants(node).contains(&p) {
                    return Err(DgError::Invalid(format!("{} cannot go under itself", g.name(node)?)));
                }
            }
            let pos = index.unwrap_or(usize::MAX);
            let links = if add_instance {
                let mut l = g.dag_links(node);
                if l.0.iter().any(|x| x.0 == parent) {
                    return Err(DgError::Invalid(format!("{} is already under that parent", g.name(node)?)));
                }
                l.0.push((parent, pos));
                l
            } else {
                DagLinks(vec![(parent, pos)])
            };
            step(g, log, Prim::Links(node, links))
        }
    }
}

impl DgModifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node creation. The node's id is valid now; the node joins the graph at `do_it`, named
    /// `name` or after its type, made unique then.
    pub fn create_node(&mut self, g: &mut Graph, type_name: &str, name: Option<&str>) -> Result<NodeId> {
        self.create(g, type_name, name, None)
    }

    fn create(&mut self, g: &mut Graph, type_name: &str, name: Option<&str>, parent: Option<NodeId>) -> Result<NodeId> {
        let desired = name.map(str::to_string).unwrap_or_else(|| format!("{type_name}#"));
        if !naming::is_valid_name(&desired) {
            return Err(DgError::Invalid(format!("{desired:?} is not a valid node name")));
        }
        let d = g.create_detached(type_name, &desired)?;
        let id = d.id();
        self.pending.push(Op::Create { node: Some(d), desired, parent });
        Ok(id)
    }

    /// Adds a node deletion: its connections go, DAG children go with it, instanced children only
    /// lose it as a parent.
    pub fn delete_node(&mut self, node: NodeId) {
        self.pending.push(Op::Delete { node });
    }

    pub fn rename_node(&mut self, node: NodeId, name: &str) {
        self.pending.push(Op::Rename { node, name: name.into() });
    }

    pub fn set_node_lock_state(&mut self, node: NodeId, on: bool) {
        self.pending.push(Op::NodeLock { node, on });
    }

    pub fn set_plug_lock_state(&mut self, plug: &Plug, on: bool) {
        self.pending.push(Op::PlugLock { plug: plug.clone(), on });
    }

    pub fn connect(&mut self, src: &Plug, dst: &Plug) {
        self.pending.push(Op::Connect { src: src.clone(), dst: dst.clone() });
    }

    pub fn disconnect(&mut self, src: &Plug, dst: &Plug) {
        self.pending.push(Op::Disconnect { src: src.clone(), dst: dst.clone() });
    }

    /// Adds a dynamic attribute addition. The attribute's id is valid now.
    pub fn add_attribute(&mut self, g: &mut Graph, node: NodeId, spec: AttrSpec) -> Result<AttrId> {
        let d = g.prepare_attribute(node, spec)?;
        let id = d.attr();
        self.pending.push(Op::AddAttr { attr: Some(d) });
        Ok(id)
    }

    /// Adds a dynamic attribute removal; its connections are broken first.
    pub fn remove_attribute(&mut self, node: NodeId, attr: AttrId) {
        self.pending.push(Op::RemoveAttr { node, attr });
    }

    pub fn rename_attribute(&mut self, node: NodeId, attr: AttrId, short: &str, long: &str) {
        self.pending.push(Op::RenameAttr { node, attr, short: short.into(), long: long.into() });
    }

    pub fn new_plug_value(&mut self, plug: &Plug, value: Value) {
        self.pending.push(Op::SetValue { plug: plug.clone(), value });
    }

    /// Adds the removal of an array element; a connected element needs `break_connections`.
    pub fn remove_multi_instance(&mut self, plug: &Plug, break_connections: bool) {
        self.pending.push(Op::RemoveElement { plug: plug.clone(), break_connections });
    }

    pub(crate) fn reparent(&mut self, node: NodeId, parent: Option<NodeId>, add_instance: bool, index: Option<usize>) {
        self.pending.push(Op::Reparent { node, parent, add_instance, index });
    }

    /// Applies what undo reverted, then every operation added since the last `do_it`. On an error the
    /// failing operation and those after it are dropped; what was applied stays, for `undo_it`.
    pub fn do_it(&mut self, g: &mut Graph) -> Result<()> {
        while let Some(p) = self.undone.pop() {
            self.done.push(apply(g, p)?);
        }
        let ops: Vec<Op> = std::mem::take(&mut self.pending);
        for op in ops {
            run(g, &mut self.done, op)?;
        }
        Ok(())
    }

    /// Reverts everything applied, newest first.
    pub fn undo_it(&mut self, g: &mut Graph) -> Result<()> {
        while let Some(p) = self.done.pop() {
            self.undone.push(apply(g, p)?);
        }
        Ok(())
    }

    /// Whether anything waits for `do_it`.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty() || !self.undone.is_empty()
    }
}

/// A modifier that knows the DAG: nodes made under a parent, and reparenting.
#[derive(Default)]
pub struct DagModifier {
    inner: DgModifier,
}

impl std::ops::Deref for DagModifier {
    type Target = DgModifier;
    fn deref(&self) -> &DgModifier {
        &self.inner
    }
}

impl std::ops::DerefMut for DagModifier {
    fn deref_mut(&mut self) -> &mut DgModifier {
        &mut self.inner
    }
}

impl DagModifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the creation of a DAG node under `parent` (the world for `None`).
    pub fn create_dag_node(&mut self, g: &mut Graph, type_name: &str, name: Option<&str>, parent: Option<NodeId>) -> Result<NodeId> {
        let c = g.registry.class_by_name(type_name).ok_or_else(|| DgError::NoClass(type_name.into()))?;
        if !g.registry.is_dag(c) {
            return Err(DgError::Invalid(format!("{type_name} is not a DAG node type")));
        }
        self.inner.create(g, type_name, name, parent)
    }

    /// Adds moving `node` under `parent` (the world for `None`) as its only parent.
    pub fn reparent_node(&mut self, node: NodeId, parent: Option<NodeId>) {
        self.inner.reparent(node, parent, false, None);
    }

    /// Adds `node` under `parent` at `index` among its children, keeping its other parents when
    /// `add_instance` (an instance).
    pub fn add_child(&mut self, parent: Option<NodeId>, node: NodeId, index: Option<usize>, add_instance: bool) {
        self.inner.reparent(node, parent, add_instance, index);
    }

    pub fn into_dg(self) -> DgModifier {
        self.inner
    }
}

/// A modifier as a step on the undo queue.
pub struct UndoStep(pub DgModifier);

impl aztek_undo::UndoAction<Graph> for UndoStep {
    fn undo(&mut self, g: &mut Graph) {
        if let Err(e) = self.0.undo_it(g) {
            g.warn(format!("undo: {e}"));
        }
    }

    fn redo(&mut self, g: &mut Graph) {
        if let Err(e) = self.0.do_it(g) {
            g.warn(format!("redo: {e}"));
        }
    }
}
