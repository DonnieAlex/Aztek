//! addExtension: an attribute added to every node of a type, those there now and those made later, as the reference's
//! extension attributes are. Each node holds it as an added attribute flagged EXTENSION; the definition lives on the
//! graph (`Graph::extensions`) so it outlasts new scenes, and scene files keep the values but not the definition.
//! What the attribute carries beyond Aztek's attribute model (readable, cachedInternally, internalSet, proxies and the
//! like) is kept beside it and answered by addExtension -q and attributeQuery.

use std::collections::BTreeMap;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::{json, Map, Value};

use crate::plug::{flags, AttrDef, PlugRef, PlugType, PlugValue};
use crate::ref_core::f;
use crate::session::Session;

type Cmd = CommandDef<Session>;
use ArgType::{Any as A, Bool as B, Float as F, Int as I, String as S};

/// What an extension attribute carries beyond its definition, by (node type, long name).
#[derive(Default)]
pub struct ExtensionMeta {
    pub attrs: BTreeMap<(String, String), Map<String, Value>>,
    /// Compounds waiting for their children: (node type, name) to (multi, number of children).
    pub compounds: BTreeMap<(String, String), (bool, i64)>,
}

/// The switches kept as given, with their defaults when not given.
const KEPT: &[(&str, bool)] = &[
    ("readable", true),
    ("internalSet", false),
    ("fromPlugin", false),
    ("usedAsProxy", false),
    ("indexMatters", true),
    ("writable", true),
];

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|t| matches!(t, "on" | "true" | "yes" | "1")))
}

/// The extension's metadata for an attribute of a node type, if it is one.
pub fn meta<'a>(s: &'a Session, ty: &crate::node::NodeType, attr: &str) -> Option<&'a Map<String, Value>> {
    s.extras.extensions.attrs.iter().find(|((t, n), _)| ty.is_a(t) && n == attr).map(|(_, m)| m)
}

/// The extension (node type and definition) named so on a type, by long or short name.
fn find_def<'a>(s: &'a Session, ty: &str, name: &str) -> Option<&'a (String, AttrDef)> {
    let leaf = |n: &str| n.rsplit('.').next().unwrap_or("").to_string();
    s.graph.extensions.iter().find(|(t, d)| t == ty && (leaf(&d.name) == name || (!d.short_name.is_empty() && leaf(&d.short_name) == name)))
}

/// Every node of the type or one derived from it.
fn nodes_of(s: &Session, ty: &str) -> Vec<crate::plug::NodeId> {
    let g = &s.graph;
    g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().is_a(ty))).collect()
}

fn add_extension(s: &mut Session, a: &Args) -> CmdResult {
    let ty = a.string("nodeType", "");
    if ty.is_empty() {
        fail!("addExtension: -nodeType is required");
    }
    if crate::node::types::find(&ty).is_none() {
        fail!("addExtension: unknown node type '{ty}'");
    }
    if a.bool("query") {
        return query(s, a, &ty);
    }
    if a.bool("edit") {
        return edit(s, a, &ty);
    }
    let long = a.string("longName", "");
    let short = a.string("shortName", "");
    let name = if long.is_empty() { short.clone() } else { long.clone() };
    if name.is_empty() {
        fail!("addExtension: give -longName or -shortName");
    }
    if find_def(s, &ty, &name).is_some() || s.extras.extensions.compounds.contains_key(&(ty.clone(), name.clone())) {
        fail!("addExtension: {ty} already has an extension attribute '{name}'");
    }
    if let Some(t) = crate::node::types::find(&ty) {
        if t.attr_index(&name).is_some() || (!short.is_empty() && t.attr_index(&short).is_some()) {
            fail!("addExtension: {ty} already has an attribute '{name}'");
        }
    }
    // addAttr's own reading of the type, limits and switches; -dt may be given several times: the first is held.
    let mut flags_in = a.flags.clone();
    if let Some(Value::Array(v)) = a.get("dataType") {
        flags_in.insert("dataType".into(), v.first().cloned().unwrap_or(Value::Null));
    }
    // A spectrum is a colour of three floats.
    if flags_in.get("dataType").and_then(Value::as_str).is_some_and(|d| d.starts_with("spectrum")) {
        flags_in.remove("dataType");
        flags_in.insert("attributeType".into(), json!("float3"));
    }
    for k in ["hasMinValue", "hasMaxValue", "hasSoftMinValue", "hasSoftMaxValue"] {
        flags_in.remove(k);
    }
    let args = Args { positional: Vec::new(), flags: flags_in };
    let mut m = Map::new();
    for (k, d) in KEPT {
        m.insert(k.to_string(), json!(a.get(k).map(on).unwrap_or(*d)));
    }
    let writable = m["writable"].as_bool().unwrap_or(true);
    m.insert("cachedInternally".into(), json!(a.get("cachedInternally").map(on).unwrap_or(writable)));
    m.insert("disconnectBehaviour".into(), json!(a.get("disconnectBehaviour").and_then(Value::as_i64).unwrap_or(2)));
    m.insert("binaryTag".into(), json!(a.string("binaryTag", "")));
    m.insert("proxy".into(), json!(a.string("proxy", "")));
    if a.has("proxy") {
        m.insert("usedAsProxy".into(), json!(true));
    }
    if let Some(Value::Array(v)) = a.get("dataType") {
        m.insert("dataType".into(), Value::Array(v.clone()));
    } else if let Some(t) = a.get("dataType").filter(|v| !v.is_null()) {
        m.insert("dataType".into(), json!([t]));
    }
    m.insert("multi".into(), json!(a.bool("multi")));
    m.insert("worldSpace".into(), json!(a.get("worldSpace").map(on).unwrap_or(false)));
    let at = a.string("attributeType", "");
    if at == "compound" {
        if a.has("parent") {
            fail!("addExtension: compounds nest one level; '{name}' can't have a parent");
        }
        let nc = a.get("numberOfChildren").and_then(Value::as_i64).unwrap_or(0);
        s.extras.extensions.compounds.insert((ty.clone(), name.clone()), (a.bool("multi"), nc));
        m.insert("attributeType".into(), json!("compound"));
        m.insert("numberOfChildren".into(), json!(nc));
        m.insert("shortName".into(), json!(short));
        m.insert("longName".into(), json!(name));
        m.insert("niceName".into(), json!(a.string("niceName", "")));
        s.extras.extensions.attrs.insert((ty, name), m);
        return Ok(Value::Null);
    }
    let group = a.string("parent", "");
    // A child of a double3 or float3: it names one of the three.
    if !group.is_empty() {
        if let Some(i) = s.graph.extensions.iter().position(|(t, d)| *t == ty && d.name == group && d.ty == PlugType::Vec3) {
            let mut def = s.graph.extensions[i].1.clone();
            let auto = |d: &AttrDef, c: usize| d.child_names[c].is_empty() || d.child_names[c] == format!("{group}{}", ["X", "Y", "Z"][c]);
            let Some(c) = (0..3).find(|&c| auto(&def, c)) else { fail!("addExtension: {group} already has its three children") };
            def.child_names[c] = name.clone();
            def.child_short[c] = short.clone();
            s.graph.extensions[i].1 = def.clone();
            for id in nodes_of(s, &ty) {
                let Some(k) = s.graph.node(id).and_then(|n| n.ty().attr_index(&group)) else { continue };
                let mut nd = s.graph.node(id).unwrap().ty().attrs[k].clone();
                nd.child_names = def.child_names.clone();
                nd.child_short = def.child_short.clone();
                s.graph.without_undo(|g| g.edit_attr(&PlugRef::new(id, k as i32), nd))?;
            }
            m.insert("parent".into(), json!(group));
            s.extras.extensions.attrs.insert((ty, name), m);
            return Ok(Value::Null);
        }
    }
    let mut def = crate::commands::attr_def_from_args(&args, &name)?;
    def.short_name = short.clone();
    if at == "double3" || at == "float3" {
        // Children come with -parent; until then they are named after it.
        def.child_names = Default::default();
    }
    if a.get("hidden").is_some_and(on) {
        def.flags |= flags::HIDDEN;
    } else {
        def.flags &= !flags::HIDDEN;
    }
    if a.get("keyable").is_some_and(on) {
        def.flags |= flags::KEYABLE;
    } else {
        def.flags &= !flags::KEYABLE;
    }
    // A spectrum is a colour.
    let dts: Vec<String> = m.get("dataType").and_then(Value::as_array).map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    if dts.iter().any(|d| d.starts_with("spectrum")) {
        def.ty = PlugType::Vec3;
        def.default_value = PlugValue::Vec3(glam::DVec3::ZERO);
        def.flags |= flags::COLOR;
    }
    if !group.is_empty() {
        let Some(&(multi, _)) = s.extras.extensions.compounds.get(&(ty.clone(), group.clone())) else { fail!("addExtension: no compound extension '{group}' on {ty}") };
        def.name = format!("{group}.{name}");
        if !short.is_empty() {
            def.short_name = format!("{group}.{short}");
        }
        if multi {
            def.flags |= flags::ARRAY;
        }
        m.insert("parent".into(), json!(group));
    }
    def.flags |= flags::EXTENSION | flags::USER | flags::INPUT | flags::STORABLE;
    if a.get("storable").is_some_and(|v| !on(v)) {
        def.flags &= !flags::STORABLE;
    }
    s.graph.extensions.push((ty.clone(), def.clone()));
    s.extras.extensions.attrs.insert((ty.clone(), name.clone()), m);
    for id in nodes_of(s, &ty) {
        if s.graph.node(id).is_some_and(|n| n.ty().attr_index(&def.name).is_none()) {
            let d = def.clone();
            s.graph.without_undo(|g| g.add_attr(id, d))?;
        }
    }
    Ok(Value::Null)
}

/// The definition and metadata the query and edit name: -nodeType with -longName or -shortName.
fn named(s: &Session, a: &Args, ty: &str) -> CmdResult<(String, Option<AttrDef>, Map<String, Value>)> {
    let name = if a.has("longName") { a.string("longName", "") } else { a.string("shortName", "") };
    if name.is_empty() {
        fail!("addExtension: name the attribute with -longName or -shortName");
    }
    let def = find_def(s, ty, &name).map(|(_, d)| d.clone());
    let key_name = def.as_ref().map(|d| d.name.rsplit('.').next().unwrap_or("").to_string()).unwrap_or(name.clone());
    let m = s.extras.extensions.attrs.get(&(ty.to_string(), key_name.clone())).cloned();
    match m {
        Some(m) => Ok((key_name, def, m)),
        None => fail!("addExtension: {ty} has no extension attribute '{name}'"),
    }
}

fn query(s: &mut Session, a: &Args, ty: &str) -> CmdResult {
    let asked: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let what = asked.iter().find(|k| !matches!(k.as_str(), "nodeType" | "longName" | "shortName" | "query" | "edit")).cloned().or_else(|| asked.first().cloned()).unwrap_or_default();
    if what == "nodeType" {
        return Ok(json!(ty));
    }
    if what == "exists" {
        return Ok(json!(named(s, a, ty).is_ok()));
    }
    let (name, def, m) = named(s, a, ty)?;
    let leaf = |n: &str| n.rsplit('.').next().unwrap_or("").to_string();
    let Some(d) = def else {
        // A compound still waiting for its children.
        return Ok(match what.as_str() {
            "attributeType" => json!("compound"),
            "longName" => json!(name),
            _ => m.get(&what).cloned().unwrap_or(Value::Null),
        });
    };
    let k = aztek_ops::units::factor(d.unit);
    let user = |x: Option<f64>| x.map(|v| v / k);
    Ok(match what.as_str() {
        "longName" => json!(leaf(&d.name)),
        "shortName" => json!(leaf(&d.short_name)),
        "niceName" => json!(d.nice_name),
        "attributeType" => json!(if m.contains_key("dataType") { "typed" } else { crate::ref_attrs::ref_type(&d, false) }),
        "dataType" => m.get("dataType").cloned().unwrap_or(json!([])),
        "category" => json!(crate::commands::categories(&d)),
        "defaultValue" => match &d.default_value {
            PlugValue::Vec3(v) => json!([v.x / k, v.y / k, v.z / k]),
            o => json!(o.scalar().map(|x| x / k).unwrap_or(0.0)),
        },
        "minValue" => json!(user(d.min_value).unwrap_or(0.0)),
        "maxValue" => json!(user(d.max_value).unwrap_or(0.0)),
        "softMinValue" => json!(user(d.soft_min.or(d.min_value)).unwrap_or(0.0)),
        "softMaxValue" => json!(user(d.soft_max.or(d.max_value)).unwrap_or(0.0)),
        "hasMinValue" => json!(d.min_value.is_some()),
        "hasMaxValue" => json!(d.max_value.is_some()),
        "hasSoftMinValue" => json!(d.soft_min.is_some()),
        "hasSoftMaxValue" => json!(d.soft_max.is_some()),
        "enumName" => json!(d.enum_names.join(":")),
        "hidden" => json!(d.has(flags::HIDDEN)),
        "keyable" => json!(d.has(flags::KEYABLE)),
        "storable" => json!(d.has(flags::STORABLE)),
        "usedAsColor" => json!(d.has(flags::COLOR)),
        "usedAsFilename" => json!(d.has(flags::FILE)),
        "enforcingUniqueName" => json!(!d.has(flags::NAME_NOT_UNIQUE)),
        "multi" => json!(d.is_array() && m.get("parent").is_none()),
        "parent" => json!(m.get("parent").cloned().unwrap_or(json!(""))),
        "numberOfChildren" => json!(if d.ty == PlugType::Vec3 { 3 } else { 0 }),
        "" => fail!("addExtension: nothing asked"),
        other => m.get(other).cloned().unwrap_or(Value::Null),
    })
}

/// -e: the flags marked edit in the reference: default, limits, nice name, enum names and categories.
fn edit(s: &mut Session, a: &Args, ty: &str) -> CmdResult {
    let (_, def, _) = named(s, a, ty)?;
    let Some(old) = def else { fail!("addExtension: a compound has nothing to edit") };
    let mut d = old.clone();
    let k = aztek_ops::units::factor(d.unit);
    if let Some(v) = a.get("defaultValue").and_then(Value::as_f64) {
        d.default_value = match d.default_value {
            PlugValue::Vec3(_) => PlugValue::Vec3(glam::DVec3::splat(v * k)),
            PlugValue::Int(_) => PlugValue::Int(v as i64),
            PlugValue::Bool(_) => PlugValue::Bool(v != 0.0),
            _ => PlugValue::Float(v * k),
        };
    }
    let num = |n: &str| a.get(n).and_then(Value::as_f64).map(|x| x * k);
    if let Some(v) = num("minValue") {
        d.min_value = Some(v);
    }
    if let Some(v) = num("maxValue") {
        d.max_value = Some(v);
    }
    if let Some(v) = num("softMinValue") {
        d.soft_min = Some(v);
    }
    if let Some(v) = num("softMaxValue") {
        d.soft_max = Some(v);
    }
    if a.get("hasMinValue").is_some_and(|v| !on(v)) {
        d.min_value = None;
    }
    if a.get("hasMaxValue").is_some_and(|v| !on(v)) {
        d.max_value = None;
    }
    if let (Some(lo), Some(hi)) = (d.min_value, d.max_value) {
        if lo > hi {
            fail!("addExtension: the minimum is more than the maximum");
        }
    }
    if a.has("niceName") {
        d.nice_name = a.string("niceName", "");
    }
    if a.has("enumName") {
        if d.ty != PlugType::Enum {
            fail!("addExtension: -enumName is for enum attributes");
        }
        d.enum_names = a.string("enumName", "").split(':').filter(|x| !x.is_empty()).map(|x| x.split('=').next().unwrap_or("").to_string()).collect();
    }
    // Categories are added, never taken away.
    let more = crate::commands::categories_of(a);
    if !more.is_empty() {
        let mut have = crate::commands::categories(&d);
        for c in more {
            if !have.contains(&c) {
                have.push(c);
            }
        }
        d.category = have.join(";");
    }
    if let Some(e) = s.graph.extensions.iter_mut().find(|(t, x)| t == ty && x.name == old.name) {
        e.1 = d.clone();
    }
    for id in nodes_of(s, ty) {
        let Some(i) = s.graph.node(id).and_then(|n| n.ty().attr_index(&old.name)) else { continue };
        let mut nd = s.graph.node(id).unwrap().ty().attrs[i].clone();
        if !nd.has(flags::EXTENSION) {
            continue;
        }
        nd.default_value = d.default_value.clone();
        nd.min_value = d.min_value;
        nd.max_value = d.max_value;
        nd.soft_min = d.soft_min;
        nd.soft_max = d.soft_max;
        nd.nice_name = d.nice_name.clone();
        nd.enum_names = d.enum_names.clone();
        nd.category = d.category.clone();
        s.graph.without_undo(|g| g.edit_attr(&PlugRef::new(id, i as i32), nd))?;
    }
    Ok(Value::Null)
}

/// Whether a node holds other than the default for an attribute: a value set, or array elements.
fn holds_data(s: &Session, id: crate::plug::NodeId, def: &AttrDef) -> bool {
    let Some(n) = s.graph.node(id) else { return false };
    let Some(i) = n.ty().attr_index(&def.name) else { return false };
    if def.is_array() {
        return !n.elements(i).is_empty();
    }
    *n.value(i) != def.default_value
}

/// deleteExtension: removes an extension attribute from a node type and its nodes. Nodes holding values for it keep it
/// unless -forceDelete is on; returns how many nodes lost data.
fn delete_extension(s: &mut Session, a: &Args) -> CmdResult {
    let ty = a.string("nodeType", "");
    let name = a.string("attribute", "");
    if ty.is_empty() || name.is_empty() {
        fail!("deleteExtension: -nodeType and -attribute are required");
    }
    // A compound: the compound and its children. A child of a compound on its own: refused.
    let compound = s.extras.extensions.compounds.keys().find(|(t, n)| *t == ty && (*n == name || s.extras.extensions.attrs.get(&(t.clone(), n.clone())).and_then(|m| m.get("shortName")).and_then(Value::as_str) == Some(name.as_str()))).map(|(_, n)| n.clone());
    let defs: Vec<AttrDef> = match &compound {
        Some(group) => s.graph.extensions.iter().filter(|(t, d)| *t == ty && d.name.starts_with(&format!("{group}."))).map(|(_, d)| d.clone()).collect(),
        None => {
            let Some((_, d)) = find_def(s, &ty, &name) else { fail!("deleteExtension: {ty} has no extension attribute '{name}'") };
            if d.name.contains('.') {
                fail!("deleteExtension: '{name}' is a child of a compound: delete the compound");
            }
            vec![d.clone()]
        }
    };
    let long = compound.clone().unwrap_or_else(|| defs[0].name.clone());
    if compound.is_none() && s.extras.extensions.attrs.get(&(ty.clone(), long.clone())).and_then(|m| m.get("parent")).is_some() {
        fail!("deleteExtension: '{name}' is a child of a compound: delete the compound");
    }
    let nodes = nodes_of(s, &ty);
    let holding: Vec<crate::plug::NodeId> = nodes.iter().copied().filter(|&id| defs.iter().any(|d| holds_data(s, id, d))).collect();
    let force = match a.get("forceDelete") {
        Some(v) => on(v),
        None => {
            if !holding.is_empty() {
                log::warn!("deleteExtension: {} nodes hold values for {ty}.{long}; kept. Use -forceDelete on to delete them", holding.len());
            }
            false
        }
    };
    if !force && !holding.is_empty() {
        return Ok(json!(0));
    }
    for &id in &nodes {
        for d in &defs {
            if let Some(i) = s.graph.node(id).and_then(|n| n.ty().attr_index(&d.name)) {
                let p = PlugRef::new(id, i as i32);
                for (dst, src) in s.graph.connections() {
                    if (dst.node == id && dst.attr == i as i32) || (src.node == id && src.attr == i as i32) {
                        s.graph.disconnect(&dst);
                    }
                }
                s.graph.without_undo(|g| g.delete_attr(&p))?;
            }
        }
    }
    s.graph.extensions.retain(|(t, d)| !(*t == ty && defs.iter().any(|x| x.name == d.name)));
    s.extras.extensions.attrs.retain(|(t, n), m| !(*t == ty && (*n == long || m.get("parent").and_then(Value::as_str) == Some(long.as_str()))));
    if compound.is_some() {
        s.extras.extensions.compounds.remove(&(ty.clone(), long.clone()));
    }
    Ok(json!(holding.len()))
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        Cmd::new(
            "deleteExtension",
            "Delete an extension attribute from a node type and its nodes, by long or short name; a compound goes with its \
             children. Nodes holding values for it keep it unless -forceDelete is on. Returns how many nodes lost data.",
            delete_extension,
        )
        .undoable(false)
        .flag(f("attribute", "at", S, "The extension attribute, by long or short name."))
        .flag(f("forceDelete", "fd", A, "On: delete it with every node's values. Off: keep it while any node holds a value for it."))
        .flag(f("nodeType", "nt", S, "The node type it was added to.")),
    );
    let mut cmd = crate::system_cmds::q_flags(Cmd::new(
        "addExtension",
        "Add an attribute to every node of a type, those there now and those made later: addExtension -nt mesh -ln mass -at double. \
         -q with -nt and -ln asks about one; -e changes its default, limits, nice name, enum names and categories.",
        add_extension,
    ))
    .undoable(false);
    for (name, short, ty, doc) in [
        ("nodeType", "nt", S, "The node type that gets the attribute."),
        ("longName", "ln", S, "The attribute's long name."),
        ("shortName", "sn", S, "Its short name."),
        ("niceName", "nn", S, "The name editors show."),
        ("attributeType", "at", S, "Its type, as addAttr takes it: double, long, bool, enum, double3, float3, compound and the rest."),
        ("parent", "p", S, "The compound it is a child of."),
        ("numberOfChildren", "nc", I, "How many children a compound has."),
        ("defaultValue", "dv", F, "The default of a number."),
        ("minValue", "min", F, "The hard minimum."),
        ("maxValue", "max", F, "The hard maximum."),
        ("softMinValue", "smn", F, "The slider's minimum."),
        ("softMaxValue", "smx", F, "The slider's maximum."),
        ("hasMinValue", "hnv", A, "Whether it has a hard minimum; off takes it away."),
        ("hasMaxValue", "hxv", A, "Whether it has a hard maximum; off takes it away."),
        ("hasSoftMinValue", "hsn", A, "Whether it has a soft minimum."),
        ("hasSoftMaxValue", "hsx", A, "Whether it has a soft maximum."),
        ("enumName", "en", S, "An enum's names, a:b:c, each =value optional."),
        ("multi", "m", B, "An array attribute."),
        ("keyable", "k", A, "Keyable by default."),
        ("hidden", "h", A, "Hidden from editors."),
        ("readable", "r", A, "Connections can come out of it (on)."),
        ("writable", "w", A, "Connections can go into it (on)."),
        ("storable", "s", A, "Saved with the scene (on)."),
        ("cachedInternally", "ci", A, "Its value is cached on the node: on for writable attributes unless said."),
        ("internalSet", "is", A, "Its node is told when it is set."),
        ("indexMatters", "im", A, "A multi's connections need an index (on)."),
        ("disconnectBehaviour", "dcb", I, "What a disconnection does: 0 deletes the element, 1 resets it, 2 nothing (the default)."),
        ("enforcingUniqueName", "eun", A, "Its name must be unique in the attribute tree (on)."),
        ("fromPlugin", "fp", A, "A plug-in made it."),
        ("binaryTag", "bt", S, "Obsolete; kept and answered, has no effect."),
        ("proxy", "pxy", S, "Another node's attribute this one stands in for; marks it as a proxy."),
        ("usedAsProxy", "uap", B, "It stands in for another attribute."),
        ("usedAsColor", "uac", B, "Holds a colour: three float or double children."),
        ("usedAsFilename", "uaf", B, "A string holding a file name."),
        ("worldSpace", "ws", A, "An array attribute that depends on the node's world transform."),
        ("exists", "ex", B, "With -q: whether the extension attribute exists."),
    ] {
        cmd = cmd.flag(f(name, short, ty, doc));
    }
    cmd = cmd.flag(f("dataType", "dt", A, "A data type, as setAttr takes them; give it again for each it accepts.").multi(1));
    cmd = cmd.flag(f("category", "ct", A, "A category for it, such as the plug-in that made it; give it again for more. Edit adds, never removes.").multi(1));
    c.register(cmd);
}
