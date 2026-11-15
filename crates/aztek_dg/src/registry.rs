//! Node types and their attributes. A type inherits its parent's attributes and affects; extension
//! attributes added later reach the type and every type derived from it.

use crate::attr::{AttrDef, AttrId, AttrKind, AttrSpec, Owner, SpecKind};
use crate::node::NodeImpl;
use crate::types::{DgError, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassId(pub u32);

/// Makes the behaviour of one node.
pub type Factory = Arc<dyn Fn() -> Arc<dyn NodeImpl> + Send + Sync>;

pub struct NodeClass {
    pub id: ClassId,
    pub name: String,
    pub type_id: u32,
    pub parent: Option<ClassId>,
    pub is_abstract: bool,
    /// Nodes of this type (and types derived from it) live in the DAG hierarchy.
    pub dag: bool,
    /// Top-level attributes this type adds, in the order they were added.
    pub attrs: Vec<AttrId>,
    /// Input attribute to the attributes it affects, as declared on this type.
    pub affects: HashMap<AttrId, Vec<AttrId>>,
    pub factory: Option<Factory>,
}

#[derive(Default)]
pub struct Registry {
    attrs: Vec<AttrDef>,
    classes: Vec<NodeClass>,
    by_name: HashMap<String, ClassId>,
    by_type_id: HashMap<u32, ClassId>,
}

/// What a node type's initialize step works with.
pub struct ClassInit<'a> {
    reg: &'a mut Registry,
    class: ClassId,
}

impl ClassInit<'_> {
    pub fn add_attribute(&mut self, spec: AttrSpec) -> Result<AttrId> {
        let class = self.class;
        let names = self.reg.names_in_chain(class);
        self.reg.check_names(&spec, &names)?;
        let id = self.reg.add_spec(spec, None, Owner::Class(class), false);
        self.reg.classes[class.0 as usize].attrs.push(id);
        Ok(id)
    }

    /// Declares that changing `input` makes `affected` dirty.
    pub fn attribute_affects(&mut self, input: AttrId, affected: AttrId) -> Result<()> {
        self.reg.add_affects(self.class, input, affected)
    }

    pub fn set_abstract(&mut self) {
        self.reg.classes[self.class.0 as usize].is_abstract = true;
    }

    /// Makes this a DAG node type.
    pub fn set_dag(&mut self) {
        self.reg.classes[self.class.0 as usize].dag = true;
    }

    /// Finds an attribute this type has, inherited ones included.
    pub fn find(&self, name: &str) -> Option<AttrId> {
        self.reg.find_in_class(self.class, name)
    }
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a node type. `init` adds its attributes and declares what affects what.
    pub fn register_class(
        &mut self,
        name: &str,
        type_id: u32,
        parent: Option<&str>,
        factory: Option<Factory>,
        init: impl FnOnce(&mut ClassInit) -> Result<()>,
    ) -> Result<ClassId> {
        if self.by_name.contains_key(name) || self.by_type_id.contains_key(&type_id) {
            return Err(DgError::NameClash(name.into()));
        }
        let parent = match parent {
            Some(p) => Some(self.class_by_name(p).ok_or_else(|| DgError::NoClass(p.into()))?),
            None => None,
        };
        let id = ClassId(self.classes.len() as u32);
        self.classes.push(NodeClass {
            id,
            name: name.into(),
            type_id,
            parent,
            is_abstract: false,
            dag: false,
            attrs: Vec::new(),
            affects: HashMap::new(),
            factory,
        });
        let r = init(&mut ClassInit { reg: self, class: id });
        if let Err(e) = r {
            self.classes.pop();
            return Err(e);
        }
        self.by_name.insert(name.into(), id);
        self.by_type_id.insert(type_id, id);
        Ok(id)
    }

    /// Adds an extension attribute to a node type, reaching every type derived from it.
    pub fn add_extension(&mut self, class: &str, spec: AttrSpec) -> Result<AttrId> {
        let c = self.class_by_name(class).ok_or_else(|| DgError::NoClass(class.into()))?;
        let mut names = self.names_in_chain(c);
        for d in self.descendants(c) {
            names.extend(self.names_in_chain(d));
        }
        self.check_names(&spec, &names)?;
        let id = self.add_spec(spec, None, Owner::Class(c), true);
        self.classes[c.0 as usize].attrs.push(id);
        Ok(id)
    }

    /// Declares an affects relation on a registered type, for extension attributes.
    pub fn extension_affects(&mut self, class: &str, input: AttrId, affected: AttrId) -> Result<()> {
        let c = self.class_by_name(class).ok_or_else(|| DgError::NoClass(class.into()))?;
        self.add_affects(c, input, affected)
    }

    fn add_affects(&mut self, class: ClassId, input: AttrId, affected: AttrId) -> Result<()> {
        for a in [input, affected] {
            if !self.attr_in_class(class, a) {
                return Err(DgError::NoAttribute(format!("{a} on {}", self.classes[class.0 as usize].name)));
            }
        }
        let list = self.classes[class.0 as usize].affects.entry(input).or_default();
        if !list.contains(&affected) {
            list.push(affected);
        }
        Ok(())
    }

    pub(crate) fn add_spec(&mut self, spec: AttrSpec, parent: Option<AttrId>, owner: Owner, extension: bool) -> AttrId {
        let id = AttrId(self.attrs.len() as u32);
        let (kind, children) = match spec.kind {
            SpecKind::Leaf(k) => (k, Vec::new()),
            SpecKind::Compound { children, numeric } => (AttrKind::Compound { children: Vec::new(), numeric }, children),
        };
        self.attrs.push(AttrDef {
            id,
            name: spec.name,
            short: spec.short,
            nice: spec.nice,
            kind,
            props: spec.props,
            limits: spec.limits,
            parent,
            owner,
            extension,
        });
        let ids: Vec<AttrId> = children.into_iter().map(|c| self.add_spec(c, Some(id), owner, extension)).collect();
        if let AttrKind::Compound { children, .. } = &mut self.attrs[id.0 as usize].kind {
            *children = ids;
        }
        id
    }

    pub(crate) fn check_names(&self, spec: &AttrSpec, taken: &[String]) -> Result<()> {
        let mut mine = Vec::new();
        fn walk(s: &AttrSpec, out: &mut Vec<String>) {
            out.push(s.name.clone());
            if !s.short.is_empty() && s.short != s.name {
                out.push(s.short.clone());
            }
            if let SpecKind::Compound { children, .. } = &s.kind {
                children.iter().for_each(|c| walk(c, out));
            }
        }
        walk(spec, &mut mine);
        for (i, n) in mine.iter().enumerate() {
            if n.is_empty() {
                return Err(DgError::Invalid("an attribute needs a name".into()));
            }
            if taken.contains(n) || mine[..i].contains(n) {
                return Err(DgError::NameClash(n.clone()));
            }
        }
        Ok(())
    }

    /// Every long and short name a type's attributes use, inherited ones included.
    pub(crate) fn names_in_chain(&self, class: ClassId) -> Vec<String> {
        let mut out = Vec::new();
        for a in self.class_attrs_all(class) {
            let d = self.attr(a);
            out.push(d.name.clone());
            out.push(d.short.clone());
        }
        out
    }

    fn descendants(&self, class: ClassId) -> Vec<ClassId> {
        self.classes.iter().filter(|c| c.id != class && self.chain(c.id).contains(&class)).map(|c| c.id).collect()
    }

    pub fn attr_checked(&self, id: AttrId) -> Option<&AttrDef> {
        self.attrs.get(id.0 as usize)
    }

    pub(crate) fn rename_attr(&mut self, id: AttrId, short: &str, long: &str) {
        let d = &mut self.attrs[id.0 as usize];
        d.short = short.into();
        d.name = long.into();
    }

    pub fn attr(&self, id: AttrId) -> &AttrDef {
        &self.attrs[id.0 as usize]
    }

    pub fn class(&self, id: ClassId) -> &NodeClass {
        &self.classes[id.0 as usize]
    }

    pub fn class_by_name(&self, name: &str) -> Option<ClassId> {
        self.by_name.get(name).copied()
    }

    pub fn class_by_type_id(&self, id: u32) -> Option<ClassId> {
        self.by_type_id.get(&id).copied()
    }

    pub fn classes(&self) -> impl Iterator<Item = &NodeClass> {
        self.classes.iter().filter(|c| self.by_name.contains_key(&c.name))
    }

    /// The type and the types it derives from, the type itself first.
    pub fn chain(&self, class: ClassId) -> Vec<ClassId> {
        let mut out = vec![class];
        let mut c = self.classes[class.0 as usize].parent;
        while let Some(p) = c {
            out.push(p);
            c = self.classes[p.0 as usize].parent;
        }
        out
    }

    /// Whether nodes of the type live in the DAG.
    pub fn is_dag(&self, class: ClassId) -> bool {
        self.chain(class).iter().any(|c| self.classes[c.0 as usize].dag)
    }

    /// Whether `class` is `other` or derives from it.
    pub fn is_a(&self, class: ClassId, other: &str) -> bool {
        self.chain(class).iter().any(|c| self.classes[c.0 as usize].name == other)
    }

    /// Top-level attributes of a type, the base type's first, as the reference lists them.
    pub fn class_attrs(&self, class: ClassId) -> Vec<AttrId> {
        self.chain(class).iter().rev().flat_map(|c| self.classes[c.0 as usize].attrs.iter().copied()).collect()
    }

    /// Every attribute of a type, children included, parents before their children.
    pub fn class_attrs_all(&self, class: ClassId) -> Vec<AttrId> {
        let mut out = Vec::new();
        for a in self.class_attrs(class) {
            self.with_descendants(a, &mut out);
        }
        out
    }

    pub fn with_descendants(&self, a: AttrId, out: &mut Vec<AttrId>) {
        out.push(a);
        for c in self.attr(a).children().to_vec() {
            self.with_descendants(c, out);
        }
    }

    /// The top-level attribute `a` belongs to.
    pub fn root_of(&self, mut a: AttrId) -> AttrId {
        while let Some(p) = self.attr(a).parent {
            a = p;
        }
        a
    }

    /// `a` and its parents, the top-level one first.
    pub fn lineage(&self, a: AttrId) -> Vec<AttrId> {
        let mut out = vec![a];
        let mut c = self.attr(a).parent;
        while let Some(p) = c {
            out.push(p);
            c = self.attr(p).parent;
        }
        out.reverse();
        out
    }

    pub fn attr_in_class(&self, class: ClassId, a: AttrId) -> bool {
        match self.attr(self.root_of(a)).owner {
            Owner::Class(c) => self.chain(class).contains(&c),
            Owner::Dynamic(_) => false,
        }
    }

    /// Finds a type's attribute by long or short name, children included.
    pub fn find_in_class(&self, class: ClassId, name: &str) -> Option<AttrId> {
        self.class_attrs_all(class).into_iter().find(|a| {
            let d = self.attr(*a);
            d.name == name || d.short == name
        })
    }

    /// The attributes `input` affects on a type, inherited declarations included.
    pub fn affected_by(&self, class: ClassId, input: AttrId) -> Vec<AttrId> {
        let mut out = Vec::new();
        for c in self.chain(class) {
            if let Some(list) = self.classes[c.0 as usize].affects.get(&input) {
                for a in list {
                    if !out.contains(a) {
                        out.push(*a);
                    }
                }
            }
        }
        out
    }

    /// Whether anything on the type affects `a`.
    pub fn is_affected(&self, class: ClassId, a: AttrId) -> bool {
        self.chain(class).iter().any(|c| self.classes[c.0 as usize].affects.values().any(|l| l.contains(&a)))
    }
}
