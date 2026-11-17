//! Time Slider bookmarks: the reference's timeSliderBookmark node, a named, coloured range of frames the time slider
//! shows. Kept as nodes so they save and load with the scene, as the reference keeps them.

use std::any::Any;
use std::sync::Arc;

use glam::DVec3;
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::node::{types, NodeBehavior, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue};

pub const TYPE: &str = "timeSliderBookmark";

struct Bookmark;

impl NodeBehavior for Bookmark {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn attr(name: &str, short: &str, ty: PlugType, default: PlugValue, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: default, flags: flags::INPUT | flags::STORABLE, doc: doc.into(), ..Default::default() }
}

pub(crate) fn register_types() {
    if types::find(TYPE).is_some() {
        return;
    }
    let mut color = attr("color", "cl", PlugType::Vec3, PlugValue::Vec3(DVec3::new(0.0, 0.5, 1.0)), "Its colour on the time slider.");
    color.flags |= flags::COLOR;
    for (i, c) in ["R", "G", "B"].into_iter().enumerate() {
        color.child_names[i] = format!("color{c}");
        color.child_short[i] = format!("cl{}", c.to_lowercase());
    }
    let _ = types::register_type(NodeType {
        name: TYPE.into(),
        doc: "A Time Slider bookmark: a named, coloured range of frames.".into(),
        icon: "bookmark".into(),
        own_attrs: vec![
            attr("name", "nm", PlugType::String, PlugValue::String(String::new()), "Its name."),
            color,
            attr("timeRangeStart", "trs", PlugType::Float, PlugValue::Float(1.0), "Its first frame."),
            attr("timeRangeStop", "tre", PlugType::Float, PlugValue::Float(1.0), "Its last frame."),
            attr("priority", "pri", PlugType::Int, PlugValue::Int(0), "Which draws on top where bookmarks overlap."),
        ],
        create: Some(Arc::new(|| Box::new(Bookmark))),
        ..Default::default()
    });
}

/// Every bookmark: name, node name, start, stop and colour, for the time slider.
pub fn all(g: &Graph) -> Value {
    let mut out = Vec::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if n.ty().name != TYPE {
            continue;
        }
        let t = n.type_arc().clone();
        let get = |a: &str| t.attr_index(a).map(|i| n.data().value(i).clone());
        let f = |a: &str| get(a).and_then(|v| v.scalar()).unwrap_or(0.0);
        let label = match get("name") {
            Some(PlugValue::String(s)) => s,
            _ => String::new(),
        };
        let c = match get("color") {
            Some(PlugValue::Vec3(c)) => c,
            _ => DVec3::new(0.0, 0.5, 1.0),
        };
        out.push(json!({"node": n.name(), "name": label, "start": f("timeRangeStart"), "stop": f("timeRangeStop"), "color": [c.x, c.y, c.z], "priority": f("priority")}));
    }
    Value::Array(out)
}

/// The node's attribute, for setting.
pub fn plug(g: &Graph, id: NodeId, attr: &str) -> Option<PlugRef> {
    let i = g.node(id)?.ty().attr_index(attr)?;
    Some(PlugRef::new(id, i as i32))
}
