//! The Sculpt modifier's layers: each holds moves of spline points, in the root's frame (tangent, bitangent, normal)
//! so they follow the surface, keyed by spline id. Layers add up, each scaled by its weight, skipped when hidden.
//! The moves are kept on the node as plug-in data and written in scene files as text.

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Arc;

use glam::DVec3;
use serde_json::{json, Value};

use super::{at, data_value, spline_data, Spline};
use crate::node::NodeData;
use crate::plug::{flags, AttrDef, PlugType, PlugValue};

pub const KIND: &str = "xjenSculptData";

/// One layer's moves: per spline id, a move per point (root first).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layer {
    pub name: String,
    pub moves: BTreeMap<u64, Vec<[f32; 3]>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SculptData {
    pub layers: Vec<Layer>,
}

impl SculptData {
    /// As a scene file holds it: the layer count, then per layer its name and entry count, then per entry the spline
    /// id, its point count and the moves.
    pub fn text(&self) -> String {
        let mut out = vec![self.layers.len().to_string()];
        for l in &self.layers {
            out.push(word(&l.name));
            out.push(l.moves.len().to_string());
            for (id, m) in &l.moves {
                out.push(id.to_string());
                out.push(m.len().to_string());
                for v in m {
                    out.extend(v.iter().map(|x| fmt(*x)));
                }
            }
        }
        out.join(" ")
    }

    pub fn parse(text: &str) -> Result<Self, String> {
        let mut it = text.split_whitespace();
        let mut next = || it.next().ok_or_else(|| "sculpt data ends early".to_string());
        let n: usize = next()?.parse().map_err(|_| "layer count")?;
        let mut layers = Vec::with_capacity(n.min(1024));
        for _ in 0..n {
            let name = next()?.trim_matches('"').to_string();
            let m: usize = next()?.parse().map_err(|_| "entry count")?;
            let mut moves = BTreeMap::new();
            for _ in 0..m {
                let id: u64 = next()?.parse().map_err(|_| "spline id")?;
                let k: usize = next()?.parse().map_err(|_| "point count")?;
                let mut v = Vec::with_capacity(k.min(4096));
                for _ in 0..k {
                    let mut p = [0f32; 3];
                    for x in &mut p {
                        *x = next()?.parse().map_err(|_| "move")?;
                    }
                    v.push(p);
                }
                moves.insert(id, v);
            }
            layers.push(Layer { name, moves });
        }
        Ok(SculptData { layers })
    }
}

fn fmt(x: f32) -> String {
    if x == 0.0 { "0".into() } else { format!("{x}") }
}

/// A layer name as one word: spaces become underscores.
fn word(s: &str) -> String {
    let w: String = s.chars().map(|c| if c.is_whitespace() { '_' } else { c }).collect();
    if w.is_empty() { "layer".into() } else { w }
}

impl crate::plug::Data for SculptData {
    fn kind(&self) -> &str {
        KIND
    }
    fn to_json(&self) -> Value {
        json!({"kind": KIND, "pluginData": true, "text": self.text()})
    }
    fn bytes(&self) -> usize {
        self.layers.iter().map(|l| 64 + l.moves.values().map(|m| 16 + m.len() * 12).sum::<usize>()).sum()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register_kind() {
    crate::plug::data::register_kind(
        KIND,
        Arc::new(|j: &Value| {
            let text = j.get("text").and_then(Value::as_str).unwrap_or("0");
            SculptData::parse(text).map(|d| Arc::new(d) as crate::plug::DataPtr).map_err(crate::error::GraphError::invalid)
        }),
    );
}

pub fn sculpt_data(v: &PlugValue) -> Option<&SculptData> {
    match v {
        PlugValue::Data(Some(d)) => d.as_any().downcast_ref::<SculptData>(),
        _ => None,
    }
}

fn kid(name: &str, short: &str, ty: PlugType, d: PlugValue, fl: u32, doc: &str) -> AttrDef {
    AttrDef {
        name: format!("layer.{name}"),
        short_name: format!("lyr.{short}"),
        ty,
        default_value: d,
        flags: fl | flags::ARRAY,
        doc: doc.into(),
        ..Default::default()
    }
}

pub fn attrs() -> Vec<AttrDef> {
    let store = flags::INPUT | flags::STORABLE;
    vec![
        AttrDef {
            name: "sculptData".into(),
            short_name: "scd".into(),
            ty: PlugType::Data,
            default_value: PlugValue::Data(None),
            flags: store | flags::HIDDEN,
            category: format!("{}{KIND}", crate::scripted::PLUGIN_DATA),
            doc: "The layers' moves.".into(),
            ..Default::default()
        },
        kid("layerName", "lnm", PlugType::String, PlugValue::String(String::new()), store, "The layer's name."),
        kid("layerWeight", "lw", PlugType::Float, PlugValue::Float(1.0), store | flags::KEYABLE, "How much of the layer: 0 none, 1 all; keyable."),
        kid("layerVisibility", "lv", PlugType::Bool, PlugValue::Bool(true), store, "Hidden layers do nothing."),
        kid("layerGroup", "lg", PlugType::String, PlugValue::String(String::new()), store, "The group the layer is shown under."),
    ]
}

/// Applies a move, in the root's frame, to a point.
pub fn apply(sp: &mut Spline, j: usize, m: [f32; 3], k: f64) {
    let (t, b, n) = (sp.tangent, sp.bitangent(), sp.normal);
    if let Some(p) = sp.points.get_mut(j) {
        *p += (t * m[0] as f64 + b * m[1] as f64 + n * m[2] as f64) * k;
    }
}

/// A world move as one in the root's frame.
pub fn to_frame(sp: &Spline, v: DVec3) -> [f32; 3] {
    [v.dot(sp.tangent) as f32, v.dot(sp.bitangent()) as f32, v.dot(sp.normal) as f32]
}

pub(super) fn compute(d: &mut NodeData) {
    let Some((mut s, mag)) = super::modifier_input(d) else { return };
    let data = sculpt_data(d.value(at(d, "sculptData"))).cloned().unwrap_or_default();
    let (wa, va) = (at(d, "layer.layerWeight"), at(d, "layer.layerVisibility"));
    let mut index: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    for (k, sp) in s.splines.iter().enumerate() {
        index.insert(sp.id, k);
    }
    for (li, layer) in data.layers.iter().enumerate() {
        let w = d.element_or(wa, li as i32).as_float();
        if !d.element_or(va, li as i32).as_bool() || w == 0.0 {
            continue;
        }
        for (id, moves) in &layer.moves {
            let Some(&k) = index.get(id) else { continue };
            for (j, m) in moves.iter().enumerate() {
                super::sculpt::apply(&mut s.splines[k], j, *m, w * mag);
            }
        }
    }
    let _ = spline_data;
    d.set_output(at(d, "outSplineData"), data_value(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_round_trips() {
        let mut l = Layer { name: "base layer".into(), ..Default::default() };
        l.moves.insert(4294967297, vec![[0.0, 0.0, 0.0], [0.5, -0.25, 1.0]]);
        let d = SculptData { layers: vec![l, Layer { name: "two".into(), ..Default::default() }] };
        let back = SculptData::parse(&d.text()).unwrap();
        assert_eq!(back.layers[0].name, "base_layer");
        assert_eq!(back.layers[0].moves, d.layers[0].moves);
        assert_eq!(back.layers.len(), 2);
    }
}
