//! Plug queries and values. Plugs are values; every function takes the graph they live in.

use aztek_dg::{AttrId, AttrKind, DgError, Graph, Plug, PlugPath, Result, Value};

pub fn attribute(p: &Plug) -> AttrId {
    p.leaf()
}

/// `node.attribute`, with array indices and the parents they sit under: `node.inputs[3].weight`.
pub fn name(g: &Graph, p: &Plug) -> String {
    g.plug_name(p)
}

/// Options for [`partial_name`]; all off gives the shortest name.
#[derive(Clone, Copy, Debug, Default)]
pub struct PartialName {
    pub include_node_name: bool,
    pub include_non_mandatory_indices: bool,
    pub include_instanced_indices: bool,
    pub use_alias: bool,
    pub use_full_attribute_path: bool,
    pub use_long_names: bool,
}

/// The plug's name, shortest by default: short attribute names, no node name, no parents that carry
/// no index, and no instance index on world-space attributes.
pub fn partial_name(g: &Graph, p: &Plug, o: PartialName) -> String {
    if o.use_alias {
        if let Some(a) = g.plug_alias(p) {
            return if o.include_node_name { format!("{}.{a}", g.name(p.node).unwrap_or("?")) } else { a };
        }
    }
    let n = p.path.0.len();
    let mut parts = Vec::new();
    for (k, s) in p.path.0.iter().enumerate() {
        let d = g.attr(s.attr);
        if !(o.use_full_attribute_path || s.index.is_some() || k + 1 == n) {
            continue;
        }
        let mut part = if o.use_long_names { d.name.clone() } else { d.short.clone() };
        let instanced = d.props.world_space && k == 0;
        if let Some(i) = s.index {
            if !instanced || o.include_instanced_indices {
                part.push_str(&format!("[{i}]"));
            }
        }
        parts.push(part);
    }
    let attr = parts.join(".");
    if o.include_node_name {
        format!("{}.{attr}", g.name(p.node).unwrap_or("?"))
    } else {
        attr
    }
}

/// `node.attribute` and the attribute's long name, as an information string.
pub fn info(g: &Graph, p: &Plug) -> String {
    name(g, p)
}

pub fn is_array(g: &Graph, p: &Plug) -> bool {
    g.attr(p.leaf()).props.array && !p.is_element()
}

pub fn is_element(p: &Plug) -> bool {
    p.is_element()
}

pub fn is_compound(g: &Graph, p: &Plug) -> bool {
    g.attr(p.leaf()).is_compound()
}

/// Whether the plug's attribute is a child of a compound.
pub fn is_child(g: &Graph, p: &Plug) -> bool {
    g.attr(p.leaf()).parent.is_some()
}

pub fn is_dynamic(g: &Graph, p: &Plug) -> bool {
    matches!(g.attr(g.registry.root_of(p.leaf())).owner, aztek_dg::attr::Owner::Dynamic(_))
}

pub fn logical_index(p: &Plug) -> Result<u32> {
    p.path.0.last().and_then(|s| s.index).ok_or_else(|| DgError::BadPlug("not an array element".into()))
}

/// Elements in the datablock or connected.
pub fn existing_indices(g: &Graph, p: &Plug) -> Result<Vec<u32>> {
    g.elements(p)
}

pub fn num_elements(g: &Graph, p: &Plug) -> Result<usize> {
    Ok(g.elements(p)?.len())
}

pub fn element_by_logical_index(g: &Graph, p: &Plug, index: u32) -> Result<Plug> {
    if !is_array(g, p) {
        return Err(DgError::BadPlug(format!("{} is not an array plug", name(g, p))));
    }
    Ok(p.element(index))
}

/// The element at a position among the existing elements.
pub fn element_by_physical_index(g: &Graph, p: &Plug, index: usize) -> Result<Plug> {
    let e = g.elements(p)?;
    e.get(index).map(|i| p.element(*i)).ok_or_else(|| DgError::BadPlug(format!("no element at {index}")))
}

pub fn num_children(g: &Graph, p: &Plug) -> usize {
    if is_array(g, p) {
        return 0;
    }
    g.attr(p.leaf()).children().len()
}

pub fn child(g: &Graph, p: &Plug, index: usize) -> Result<Plug> {
    if is_array(g, p) {
        return Err(DgError::BadPlug(format!("{} needs an index first", name(g, p))));
    }
    g.attr(p.leaf()).children().get(index).map(|c| p.child(*c)).ok_or_else(|| DgError::BadPlug(format!("no child at {index}")))
}

pub fn child_by_attr(g: &Graph, p: &Plug, attr: AttrId) -> Result<Plug> {
    let i = g.attr(p.leaf()).children().iter().position(|c| *c == attr).ok_or_else(|| DgError::BadPlug("not a child".into()))?;
    child(g, p, i)
}

/// The compound plug this one is a child of.
pub fn parent(g: &Graph, p: &Plug) -> Result<Plug> {
    if !is_child(g, p) {
        return Err(DgError::BadPlug(format!("{} is not a child", name(g, p))));
    }
    let mut path = p.path.clone();
    path.0.pop();
    Ok(Plug { node: p.node, path })
}

/// The array plug this element belongs to.
pub fn array(p: &Plug) -> Result<Plug> {
    p.path
        .array_of_element()
        .map(|path| Plug { node: p.node, path })
        .ok_or_else(|| DgError::BadPlug("not an array element".into()))
}

// ---- connections -------------------------------------------------------------------------

pub fn source(g: &Graph, p: &Plug) -> Option<Plug> {
    g.source(p)
}

pub fn destinations(g: &Graph, p: &Plug) -> Vec<Plug> {
    g.destinations(p)
}

pub fn is_source(g: &Graph, p: &Plug) -> bool {
    !g.destinations(p).is_empty()
}

pub fn is_destination(g: &Graph, p: &Plug) -> bool {
    g.source(p).is_some()
}

pub fn is_connected(g: &Graph, p: &Plug) -> bool {
    is_source(g, p) || is_destination(g, p)
}

/// The plugs connected to this one: its source (`as_dst`) and its destinations (`as_src`).
pub fn connected_to(g: &Graph, p: &Plug, as_dst: bool, as_src: bool) -> Vec<Plug> {
    let mut out = Vec::new();
    if as_dst {
        out.extend(g.source(p));
    }
    if as_src {
        out.extend(g.destinations(p));
    }
    out
}

fn under(p: &PlugPath, q: &PlugPath) -> bool {
    p.covers(q) && p != q
}

/// Elements of this array plug with a connection to or from them or under them.
pub fn num_connected_elements(g: &Graph, p: &Plug) -> Result<usize> {
    let mut idx: Vec<u32> = g
        .elements(p)?
        .into_iter()
        .filter(|i| {
            let e = p.element(*i);
            g.connections(p.node).iter().any(|c| (c.dst.node == p.node && e.path.covers(&c.dst.path)) || (c.src.node == p.node && e.path.covers(&c.src.path)))
        })
        .collect();
    idx.dedup();
    Ok(idx.len())
}

/// Children of this compound plug with a connection to or from them or under them.
pub fn num_connected_children(g: &Graph, p: &Plug) -> usize {
    (0..num_children(g, p))
        .filter_map(|i| child(g, p, i).ok())
        .filter(|c| g.connections(p.node).iter().any(|x| (x.dst.node == p.node && c.path.covers(&x.dst.path)) || (x.src.node == p.node && c.path.covers(&x.src.path))))
        .count()
}

// ---- flags -------------------------------------------------------------------------------

pub fn is_keyable(g: &Graph, p: &Plug) -> bool {
    g.plug_keyable(p)
}

/// Overrides the attribute's keyable setting for this plug.
pub fn set_keyable(g: &mut Graph, p: &Plug, on: bool) -> Result<()> {
    g.set_plug_keyable(p, on)
}

pub fn is_channel_box(g: &Graph, p: &Plug) -> bool {
    g.plug_channel_box(p)
}

pub fn set_channel_box(g: &mut Graph, p: &Plug, on: bool) -> Result<()> {
    g.set_plug_channel_box(p, on)
}

pub fn is_caching(g: &Graph, p: &Plug) -> bool {
    g.plug_caching(p)
}

pub fn set_caching(g: &mut Graph, p: &Plug, on: bool) -> Result<()> {
    g.set_plug_caching(p, on)
}

pub fn is_locked(g: &Graph, p: &Plug) -> bool {
    g.is_locked(p)
}

/// Locks or unlocks this plug, outside any undo step.
pub fn set_locked(g: &mut Graph, p: &Plug, on: bool) -> Result<()> {
    g.set_locked(p, on)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreeToChange {
    FreeToChange,
    NotFreeToChange,
    ChildrenNotFreeToChange,
}

/// Whether the plug's value may be set: not locked and not a destination, nor (when checked) a plug
/// it is part of; with `check_children`, a locked or connected part reports its children.
pub fn is_free_to_change(g: &Graph, p: &Plug, check_parents: bool, check_children: bool) -> FreeToChange {
    let conns = g.connections(p.node);
    let dst_at = |path: &PlugPath| conns.iter().any(|c| c.dst.node == p.node && &c.dst.path == path);
    if g.has_own_lock(p) || dst_at(&p.path) {
        return FreeToChange::NotFreeToChange;
    }
    if check_parents {
        let mut path = p.path.clone();
        loop {
            match path.0.last_mut() {
                Some(s) if s.index.is_some() => s.index = None,
                Some(_) => {
                    path.0.pop();
                }
                None => break,
            }
            if path.0.is_empty() {
                break;
            }
            let q = Plug { node: p.node, path: path.clone() };
            if g.has_own_lock(&q) || dst_at(&path) {
                return FreeToChange::NotFreeToChange;
            }
        }
    }
    if check_children {
        let locked_below = g.locks_under(p).into_iter().any(|l| under(&p.path, &l));
        let dst_below = conns.iter().any(|c| c.dst.node == p.node && under(&p.path, &c.dst.path));
        if locked_below || dst_below {
            return FreeToChange::ChildrenNotFreeToChange;
        }
    }
    FreeToChange::FreeToChange
}

// ---- values ------------------------------------------------------------------------------

/// The plug's value, evaluating what it depends on.
pub fn value(g: &mut Graph, p: &Plug) -> Result<Value> {
    g.get(p)
}

pub fn as_double(g: &mut Graph, p: &Plug) -> Result<f64> {
    value(g, p)?.as_f64().ok_or_else(|| DgError::WrongValue(name(g, p)))
}

pub fn as_int(g: &mut Graph, p: &Plug) -> Result<i64> {
    value(g, p)?.as_i64().ok_or_else(|| DgError::WrongValue(name(g, p)))
}

pub fn as_bool(g: &mut Graph, p: &Plug) -> Result<bool> {
    value(g, p)?.as_bool().ok_or_else(|| DgError::WrongValue(name(g, p)))
}

pub fn as_string(g: &mut Graph, p: &Plug) -> Result<String> {
    let v = value(g, p)?;
    v.as_str().map(str::to_string).ok_or_else(|| DgError::WrongValue(name(g, p)))
}

/// Sets the plug's value outside any undo step.
pub fn set_value(g: &mut Graph, p: &Plug, v: Value) -> Result<()> {
    g.set(p, v)
}

/// Whether the plug holds its attribute's default.
pub fn is_default_value(g: &mut Graph, p: &Plug) -> Result<bool> {
    let v = value(g, p)?;
    let d = default_value(g, p);
    Ok(v == d)
}

/// The attribute's default for this plug (an empty array for an array plug).
pub fn default_value(g: &Graph, p: &Plug) -> Value {
    let d = g.attr(p.leaf());
    if d.props.array && !p.is_element() {
        return Value::Array(Vec::new());
    }
    match &d.kind {
        AttrKind::Compound { children, .. } => {
            Value::Compound(children.iter().map(|c| default_value(g, &p.child(*c))).collect())
        }
        _ => aztek_dg::attr::leaf_default(d),
    }
}
