//! Selection lists: nodes, DAG paths with optional components, and plugs.

use crate::dag;
use aztek_dg::{DagPath, DgError, Graph, NodeId, Plug, Result};
use std::collections::BTreeSet;

/// Components of one kind on a DAG path: single (`vtx[3]`), double (`cv[1][2]`) or triple indexed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Component {
    pub kind: String,
    /// 1, 2 or 3 indices per element; unused ones are 0.
    pub dimensions: u8,
    pub elements: BTreeSet<[i32; 3]>,
}

impl Component {
    pub fn single(kind: &str, indices: impl IntoIterator<Item = i32>) -> Self {
        Component { kind: kind.into(), dimensions: 1, elements: indices.into_iter().map(|i| [i, 0, 0]).collect() }
    }

    pub fn double(kind: &str, indices: impl IntoIterator<Item = (i32, i32)>) -> Self {
        Component { kind: kind.into(), dimensions: 2, elements: indices.into_iter().map(|(u, v)| [u, v, 0]).collect() }
    }

    pub fn triple(kind: &str, indices: impl IntoIterator<Item = (i32, i32, i32)>) -> Self {
        Component { kind: kind.into(), dimensions: 3, elements: indices.into_iter().map(|(u, v, w)| [u, v, w]).collect() }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn same_kind(&self, o: &Component) -> bool {
        self.kind == o.kind && self.dimensions == o.dimensions
    }

    /// `kind[a:b]` per run of consecutive single indices; `kind[u][v]` (and `[w]`) per element otherwise.
    pub fn strings(&self) -> Vec<String> {
        if self.dimensions != 1 {
            return self
                .elements
                .iter()
                .map(|e| {
                    let idx: String = e[..self.dimensions as usize].iter().map(|i| format!("[{i}]")).collect();
                    format!("{}{idx}", self.kind)
                })
                .collect();
        }
        let mut out = Vec::new();
        let mut run: Option<(i32, i32)> = None;
        for e in &self.elements {
            let i = e[0];
            run = match run {
                Some((a, b)) if i == b + 1 => Some((a, i)),
                Some((a, b)) => {
                    out.push(range(&self.kind, a, b));
                    Some((i, i))
                }
                None => Some((i, i)),
            };
        }
        if let Some((a, b)) = run {
            out.push(range(&self.kind, a, b));
        }
        out
    }
}

fn range(kind: &str, a: i32, b: i32) -> String {
    if a == b {
        format!("{kind}[{a}]")
    } else {
        format!("{kind}[{a}:{b}]")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    /// A node that is not in the DAG.
    Node(NodeId),
    /// A DAG node by path, or components on it.
    Dag { path: DagPath, component: Option<Component> },
    Plug(Plug),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Add what is not there yet.
    MergeNormal,
    /// Remove what is there, add what is not.
    XorWithList,
    /// Remove what is there.
    RemoveFromList,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectionList {
    items: Vec<Item>,
}

/// `*` matches any run of characters, `?` any one character.
pub fn matches(pattern: &str, text: &str) -> bool {
    fn go(p: &[char], t: &[char]) -> bool {
        match p.first() {
            None => t.is_empty(),
            Some('*') => (0..=t.len()).any(|k| go(&p[1..], &t[k..])),
            Some('?') => !t.is_empty() && go(&p[1..], &t[1..]),
            Some(c) => t.first() == Some(c) && go(&p[1..], &t[1..]),
        }
    }
    let (p, t): (Vec<char>, Vec<char>) = (pattern.chars().collect(), text.chars().collect());
    go(&p, &t)
}

impl SelectionList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    fn position(&self, item: &Item) -> Option<usize> {
        self.items.iter().position(|x| match (x, item) {
            (Item::Dag { path: a, component: ca }, Item::Dag { path: b, component: cb }) => {
                a == b && match (ca, cb) {
                    (None, None) => true,
                    (Some(x), Some(y)) => x.same_kind(y),
                    _ => false,
                }
            }
            _ => x == item,
        })
    }

    /// Adds an item. With `merge`, an item already in the list is not added again, and components
    /// of one kind on one path join the existing entry.
    pub fn add(&mut self, item: Item, merge: bool) {
        if merge {
            if let Some(i) = self.position(&item) {
                if let (Item::Dag { component: Some(have), .. }, Item::Dag { component: Some(new), .. }) = (&mut self.items[i], &item) {
                    have.elements.extend(new.elements.iter().copied());
                }
                return;
            }
        }
        self.items.push(item);
    }

    /// A node: DAG nodes go in by their first path.
    pub fn add_node(&mut self, g: &Graph, id: NodeId, merge: bool) -> Result<()> {
        if !g.exists(id) {
            return Err(DgError::NoNode);
        }
        let item = if g.is_dag(id) { Item::Dag { path: dag::path(g, id)?, component: None } } else { Item::Node(id) };
        self.add(item, merge);
        Ok(())
    }

    pub fn add_path(&mut self, path: DagPath, component: Option<Component>, merge: bool) {
        self.add(Item::Dag { path, component }, merge);
    }

    pub fn add_plug(&mut self, plug: Plug, merge: bool) {
        self.add(Item::Plug(plug), merge);
    }

    /// Adds what a name pattern names: nodes (`name`, `na*e`), DAG paths (`|top|leaf`, `top|leaf`) or
    /// plugs (`node.attr`, `node*.attr[2]`). Nothing matching is an error.
    pub fn add_matching(&mut self, g: &Graph, pattern: &str) -> Result<()> {
        let (node_part, plug_part) = match pattern.split_once('.') {
            Some((n, p)) => (n, Some(p)),
            None => (pattern, None),
        };
        let mut found = false;
        for id in g.nodes() {
            let mut targets: Vec<Item> = Vec::new();
            if g.is_dag(id) {
                for path in g.dag_paths(id) {
                    let full = dag::path_full_name(g, &path)?;
                    let hit = if node_part.starts_with('|') {
                        matches(node_part, &full)
                    } else {
                        let depth = node_part.matches('|').count() + 1;
                        let tail: Vec<&str> = full.rsplit('|').take(depth).collect();
                        tail.len() == depth && matches(node_part, &tail.into_iter().rev().collect::<Vec<_>>().join("|"))
                    };
                    if hit {
                        targets.push(Item::Dag { path, component: None });
                    }
                }
            } else if matches(node_part, g.name(id)?) {
                targets.push(Item::Node(id));
            }
            for t in targets {
                match plug_part {
                    Some(p) => {
                        if let Ok(plug) = g.plug(id, p) {
                            self.add(Item::Plug(plug), true);
                            found = true;
                        }
                    }
                    None => {
                        self.add(t, true);
                        found = true;
                    }
                }
                if plug_part.is_some() {
                    break;
                }
            }
        }
        if found {
            Ok(())
        } else {
            Err(DgError::Invalid(format!("no object matches name: {pattern}")))
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        if index >= self.items.len() {
            return Err(DgError::Invalid(format!("no item at {index}")));
        }
        self.items.remove(index);
        Ok(())
    }

    pub fn replace(&mut self, index: usize, item: Item) -> Result<()> {
        *self.items.get_mut(index).ok_or_else(|| DgError::Invalid(format!("no item at {index}")))? = item;
        Ok(())
    }

    /// The node of an item: the node, the path's node, or the plug's node.
    pub fn depend_node(&self, index: usize) -> Result<NodeId> {
        match self.items.get(index) {
            Some(Item::Node(n)) => Ok(*n),
            Some(Item::Dag { path, .. }) => Ok(path.node()),
            Some(Item::Plug(p)) => Ok(p.node),
            None => Err(DgError::Invalid(format!("no item at {index}"))),
        }
    }

    pub fn dag_path(&self, index: usize) -> Result<(DagPath, Option<Component>)> {
        match self.items.get(index) {
            Some(Item::Dag { path, component }) => Ok((path.clone(), component.clone())),
            Some(_) => Err(DgError::Invalid(format!("item {index} is not a DAG path"))),
            None => Err(DgError::Invalid(format!("no item at {index}"))),
        }
    }

    pub fn plug(&self, index: usize) -> Result<Plug> {
        match self.items.get(index) {
            Some(Item::Plug(p)) => Ok(p.clone()),
            Some(_) => Err(DgError::Invalid(format!("item {index} is not a plug"))),
            None => Err(DgError::Invalid(format!("no item at {index}"))),
        }
    }

    pub fn has_node(&self, id: NodeId) -> bool {
        self.items.iter().any(|x| match x {
            Item::Node(n) => *n == id,
            Item::Dag { path, component: None } => path.node() == id,
            _ => false,
        })
    }

    /// Whether the path, or every given component on it, is in the list.
    pub fn has_path(&self, path: &DagPath, component: Option<&Component>) -> bool {
        self.items.iter().any(|x| match (x, component) {
            (Item::Dag { path: p, component: None }, None) => p == path,
            (Item::Dag { path: p, component: Some(have) }, Some(c)) => {
                p == path && have.same_kind(c) && c.elements.is_subset(&have.elements)
            }
            _ => false,
        })
    }

    /// Whether at least one of the given components on the path is in the list.
    pub fn has_path_partly(&self, path: &DagPath, component: &Component) -> bool {
        self.items.iter().any(|x| match x {
            Item::Dag { path: p, component: Some(have) } => {
                p == path && have.same_kind(component) && !have.elements.is_disjoint(&component.elements)
            }
            _ => false,
        })
    }

    pub fn has_plug(&self, plug: &Plug) -> bool {
        self.items.iter().any(|x| matches!(x, Item::Plug(p) if p == plug))
    }

    /// Removes the path (or the given components on it) where present and adds it where not.
    pub fn toggle(&mut self, path: &DagPath, component: Option<&Component>) {
        let item = Item::Dag { path: path.clone(), component: component.cloned() };
        match (self.position(&item), component) {
            (Some(i), None) => {
                self.items.remove(i);
            }
            (Some(i), Some(c)) => {
                if let Item::Dag { component: Some(have), .. } = &mut self.items[i] {
                    for e in &c.elements {
                        if !have.elements.remove(e) {
                            have.elements.insert(*e);
                        }
                    }
                    if have.is_empty() {
                        self.items.remove(i);
                    }
                }
            }
            (None, _) => self.items.push(item),
        }
    }

    fn subtract(&mut self, item: &Item) {
        match item {
            Item::Dag { path, component: Some(c) } => {
                if let Some(i) = self.position(item) {
                    if let Item::Dag { component: Some(have), .. } = &mut self.items[i] {
                        have.elements.retain(|e| !c.elements.contains(e));
                        if have.is_empty() {
                            self.items.remove(i);
                        }
                    }
                }
                let _ = path;
            }
            _ => {
                if let Some(i) = self.position(item) {
                    self.items.remove(i);
                }
            }
        }
    }

    pub fn merge(&mut self, other: &SelectionList, strategy: MergeStrategy) {
        for item in &other.items {
            match strategy {
                MergeStrategy::MergeNormal => self.add(item.clone(), true),
                MergeStrategy::RemoveFromList => self.subtract(item),
                MergeStrategy::XorWithList => match item {
                    Item::Dag { path, component } => self.toggle(path, component.as_ref()),
                    _ => match self.position(item) {
                        Some(i) => {
                            self.items.remove(i);
                        }
                        None => self.items.push(item.clone()),
                    },
                },
            }
        }
    }

    /// Keeps only what is also in `other`; components keep their common elements.
    pub fn intersect(&mut self, other: &SelectionList) {
        let mut kept = Vec::new();
        for item in std::mem::take(&mut self.items) {
            match (&item, other.position(&item).map(|i| &other.items[i])) {
                (Item::Dag { path, component: Some(c) }, Some(Item::Dag { component: Some(o), .. })) => {
                    let common: BTreeSet<[i32; 3]> = c.elements.intersection(&o.elements).copied().collect();
                    if !common.is_empty() {
                        kept.push(Item::Dag { path: path.clone(), component: Some(Component { elements: common, ..c.clone() }) });
                    }
                }
                (_, Some(_)) => kept.push(item),
                (_, None) => {}
            }
        }
        self.items = kept;
    }

    /// The strings that name the item: partial path names, `node.attr`, `path.vtx[0:3]`.
    pub fn item_strings(&self, g: &Graph, index: usize) -> Result<Vec<String>> {
        Ok(match self.items.get(index).ok_or_else(|| DgError::Invalid(format!("no item at {index}")))? {
            Item::Node(n) => vec![g.name(*n)?.to_string()],
            Item::Plug(p) => vec![g.plug_name(p)],
            Item::Dag { path, component: None } => vec![dag::path_partial_name(g, path)?],
            Item::Dag { path, component: Some(c) } => {
                let base = dag::path_partial_name(g, path)?;
                c.strings().into_iter().map(|s| format!("{base}.{s}")).collect()
            }
        })
    }

    pub fn strings(&self, g: &Graph) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for i in 0..self.items.len() {
            out.extend(self.item_strings(g, i)?);
        }
        Ok(out)
    }
}
