//! A node's datablock storage: values by attribute, compounds by child, arrays by logical index.
//! Anything never written reads as its default.

use crate::attr::{coerce_leaf, leaf_default, AttrId, AttrKind};
use crate::plug::PlugPath;
use crate::registry::Registry;
use crate::types::{DgError, Result};
use crate::value::Value;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug)]
pub(crate) enum Slot {
    Leaf(Value),
    Compound(HashMap<AttrId, Slot>),
    Array(BTreeMap<u32, Slot>),
}

#[derive(Default, Debug)]
pub(crate) struct Store {
    root: HashMap<AttrId, Slot>,
}

fn empty(reg: &Registry, attr: AttrId, element: bool) -> Slot {
    let d = reg.attr(attr);
    if d.props.array && !element {
        Slot::Array(BTreeMap::new())
    } else if d.is_compound() {
        Slot::Compound(HashMap::new())
    } else {
        Slot::Leaf(leaf_default(d))
    }
}

/// Reads a slot (or its absence) as a value of `attr`; `element` says whether this is one element of
/// an array attribute rather than the whole array.
fn read_slot(reg: &Registry, attr: AttrId, element: bool, slot: Option<&Slot>) -> Value {
    let d = reg.attr(attr);
    if d.props.array && !element {
        return Value::Array(match slot {
            Some(Slot::Array(m)) => m.iter().map(|(i, s)| (*i, read_slot(reg, attr, true, Some(s)))).collect(),
            _ => Vec::new(),
        });
    }
    if let AttrKind::Compound { children, .. } = &d.kind {
        let m = match slot {
            Some(Slot::Compound(m)) => Some(m),
            _ => None,
        };
        return Value::Compound(children.iter().map(|c| read_slot(reg, *c, false, m.and_then(|m| m.get(c)))).collect());
    }
    match slot {
        Some(Slot::Leaf(v)) => v.clone(),
        _ => leaf_default(d),
    }
}

/// Converts a value for `attr`, recursing through compounds and arrays.
pub(crate) fn coerce(reg: &Registry, attr: AttrId, element: bool, v: Value) -> Result<Value> {
    let d = reg.attr(attr);
    let bad = || DgError::WrongValue(d.name.clone());
    if d.props.array && !element {
        let Value::Array(items) = v else { return Err(bad()) };
        return items
            .into_iter()
            .map(|(i, x)| Ok((i, coerce(reg, attr, true, x)?)))
            .collect::<Result<Vec<_>>>()
            .map(Value::Array);
    }
    if let AttrKind::Compound { children, .. } = &d.kind {
        let vals = match v {
            Value::Compound(c) => c,
            Value::Tuple(t) => t.into_iter().map(Value::Double).collect(),
            _ => return Err(bad()),
        };
        if vals.len() != children.len() {
            return Err(bad());
        }
        return children
            .iter()
            .zip(vals)
            .map(|(c, x)| coerce(reg, *c, false, x))
            .collect::<Result<Vec<_>>>()
            .map(Value::Compound);
    }
    coerce_leaf(d, v).ok_or_else(bad)
}

fn write_slot(reg: &Registry, attr: AttrId, element: bool, slot: &mut Slot, v: Value) {
    let d = reg.attr(attr);
    if d.props.array && !element {
        let Value::Array(items) = v else { return };
        *slot = Slot::Array(
            items
                .into_iter()
                .map(|(i, x)| {
                    let mut s = empty(reg, attr, true);
                    write_slot(reg, attr, true, &mut s, x);
                    (i, s)
                })
                .collect(),
        );
        return;
    }
    if let AttrKind::Compound { children, .. } = &d.kind {
        let Value::Compound(vals) = v else { return };
        if !matches!(slot, Slot::Compound(_)) {
            *slot = Slot::Compound(HashMap::new());
        }
        let Slot::Compound(m) = slot else { unreachable!() };
        for (c, x) in children.iter().zip(vals) {
            let e = m.entry(*c).or_insert_with(|| empty(reg, *c, false));
            write_slot(reg, *c, false, e, x);
        }
        return;
    }
    *slot = Slot::Leaf(v);
}

impl Store {
    /// The slot at `path`, made (with its parents) if missing. The path must name one plug: no array
    /// without an index before its last step.
    fn slot_mut(&mut self, reg: &Registry, path: &PlugPath) -> (&mut Slot, bool) {
        let first = path.0[0];
        let mut slot = self.root.entry(first.attr).or_insert_with(|| empty(reg, first.attr, false));
        let mut element = false;
        for (n, step) in path.0.iter().enumerate() {
            if n > 0 {
                if !matches!(slot, Slot::Compound(_)) {
                    *slot = Slot::Compound(HashMap::new());
                }
                let Slot::Compound(m) = slot else { unreachable!() };
                slot = m.entry(step.attr).or_insert_with(|| empty(reg, step.attr, false));
                element = false;
            }
            if let Some(i) = step.index {
                if !matches!(slot, Slot::Array(_)) {
                    *slot = Slot::Array(BTreeMap::new());
                }
                let Slot::Array(m) = slot else { unreachable!() };
                slot = m.entry(i).or_insert_with(|| empty(reg, step.attr, true));
                element = true;
            }
        }
        (slot, element)
    }

    fn slot(&self, path: &PlugPath) -> Option<&Slot> {
        let mut slot = self.root.get(&path.0[0].attr)?;
        for (n, step) in path.0.iter().enumerate() {
            if n > 0 {
                let Slot::Compound(m) = slot else { return None };
                slot = m.get(&step.attr)?;
            }
            if let Some(i) = step.index {
                let Slot::Array(m) = slot else { return None };
                slot = m.get(&i)?;
            }
        }
        Some(slot)
    }

    pub fn read(&self, reg: &Registry, path: &PlugPath) -> Value {
        let element = path.0.last().is_some_and(|s| s.index.is_some());
        read_slot(reg, path.leaf(), element, self.slot(path))
    }

    /// Writes an already converted value.
    pub fn write(&mut self, reg: &Registry, path: &PlugPath, v: Value) {
        let (slot, element) = self.slot_mut(reg, path);
        write_slot(reg, path.leaf(), element, slot, v);
    }

    /// Makes sure the plug exists, creating array elements on the way.
    pub fn ensure(&mut self, reg: &Registry, path: &PlugPath) {
        self.slot_mut(reg, path);
    }

    /// Logical indices held under an array plug.
    pub fn indices(&self, path: &PlugPath) -> Vec<u32> {
        match self.slot(path) {
            Some(Slot::Array(m)) => m.keys().copied().collect(),
            _ => Vec::new(),
        }
    }

    /// Removes one array element.
    pub fn remove_element(&mut self, path: &PlugPath) {
        let Some(parent) = path.array_of_element() else { return };
        let index = path.0.last().unwrap().index.unwrap();
        // Only reach an existing array; never create one to remove from it.
        if self.slot(&parent).is_none() {
            return;
        }
        let mut cur = self.root.get_mut(&parent.0[0].attr);
        for (n, step) in parent.0.iter().enumerate() {
            if n > 0 {
                cur = match cur {
                    Some(Slot::Compound(m)) => m.get_mut(&step.attr),
                    _ => None,
                };
            }
            if let Some(i) = step.index {
                cur = match cur {
                    Some(Slot::Array(m)) => m.get_mut(&i),
                    _ => None,
                };
            }
        }
        if let Some(Slot::Array(m)) = cur {
            m.remove(&index);
        }
    }

    pub fn remove_root(&mut self, attr: AttrId) {
        self.root.remove(&attr);
    }

    pub fn take_root(&mut self, attr: AttrId) -> Option<Slot> {
        self.root.remove(&attr)
    }

    pub fn put_root(&mut self, attr: AttrId, slot: Slot) {
        self.root.insert(attr, slot);
    }
}
