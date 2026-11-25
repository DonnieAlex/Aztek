//! Skin Tools: layered skin weights, as the reference keeps them since it took in ngSkinTools 2. A skinCluster with
//! layers has a data node (ngst2SkinLayerData) holding a stack of layers; each layer has weights for some influences,
//! a mask and dual quaternion weights, and the stack, blended from the bottom up, is what the skinCluster deforms with.
//!
//! How the stack blends: going up, a layer's weights at a point cover what is below in proportion to its opacity times
//! its mask there (no mask: fully). Where a layer's weights at a point add up to less than one, the rest comes from the
//! layers below, in their proportions. A layer with children is a group: its children blend over what is below it, and
//! the result covers what is below by the group's own opacity and mask. Dual quaternion weights blend the same way and
//! go to the skinCluster's blendWeights, with skinningMethod 2.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use aztek_geo::Mesh;
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec3};
use serde_json::{json, Map, Value};

use crate::graph::Graph;
use crate::node::{types, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::rig::{plug, JsonData};
use crate::session::Session;
use crate::skin;

pub const DATA_TYPE: &str = "ngst2SkinLayerData";
pub const PLUGIN: &str = "ngSkinTools2";
const D_SKIN: usize = 0;
const D_LAYERS: usize = 1;
const D_JSON: usize = 2;

/// What a layer's values are for: an influence (by the skinCluster's matrix element), its mask, or its dual
/// quaternion weights.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Influence(u32),
    Mask,
    Dq,
}

impl Target {
    fn name(self) -> Value {
        match self {
            Target::Influence(i) => json!(i),
            Target::Mask => json!("mask"),
            Target::Dq => json!("dq"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub opacity: f64,
    pub parent: Option<u32>,
    /// Per influence, a value per point.
    pub weights: BTreeMap<u32, Vec<f64>>,
    pub mask: Option<Vec<f64>>,
    pub dq: Option<Vec<f64>>,
    pub paint_target: Option<Target>,
    pub locked: Vec<u32>,
    /// What Mirror mirrors on this layer by default: weights, mask, dual quaternion weights.
    pub mirror: [bool; 3],
}

impl Layer {
    fn new(id: u32, name: &str, parent: Option<u32>) -> Layer {
        Layer { id, name: name.into(), enabled: true, opacity: 1.0, parent, weights: BTreeMap::new(), mask: None, dq: None, paint_target: None, locked: Vec::new(), mirror: [true, true, true] }
    }

    fn values(&self, t: Target) -> Option<&Vec<f64>> {
        match t {
            Target::Influence(i) => self.weights.get(&i),
            Target::Mask => self.mask.as_ref(),
            Target::Dq => self.dq.as_ref(),
        }
    }

    fn values_mut(&mut self, t: Target, n: usize) -> &mut Vec<f64> {
        let v = match t {
            Target::Influence(i) => self.weights.entry(i).or_insert_with(|| vec![0.0; n]),
            Target::Mask => self.mask.get_or_insert_with(|| vec![1.0; n]),
            Target::Dq => self.dq.get_or_insert_with(|| vec![0.0; n]),
        };
        v.resize(n, 0.0);
        v
    }

    /// The layer's own weights at a point: (influence, weight), none at zero.
    fn at(&self, p: usize) -> Vec<(u32, f64)> {
        self.weights.iter().filter_map(|(&i, v)| v.get(p).copied().filter(|&x| x > 1e-12).map(|x| (i, x))).collect()
    }

    fn set_at(&mut self, p: usize, w: &[(u32, f64)], n: usize) {
        for v in self.weights.values_mut() {
            if p < v.len() {
                v[p] = 0.0;
            }
        }
        for &(i, x) in w {
            let v = self.weights.entry(i).or_insert_with(|| vec![0.0; n]);
            v.resize(n, 0.0);
            v[p] = x;
        }
    }

    fn to_json(&self) -> Value {
        let w: Map<String, Value> = self.weights.iter().map(|(i, v)| (i.to_string(), json!(v))).collect();
        json!({"id": self.id, "name": self.name, "enabled": self.enabled, "opacity": self.opacity, "parent": self.parent,
               "weights": w, "mask": self.mask, "dq": self.dq, "paintTarget": self.paint_target.map(Target::name),
               "locked": self.locked, "mirror": self.mirror})
    }

    fn from_json(v: &Value) -> Option<Layer> {
        let floats = |x: &Value| x.as_array().map(|a| a.iter().map(|f| f.as_f64().unwrap_or(0.0)).collect::<Vec<f64>>());
        let mut l = Layer::new(v.get("id")?.as_u64()? as u32, v.get("name").and_then(Value::as_str).unwrap_or("layer"), v.get("parent").and_then(Value::as_u64).map(|x| x as u32));
        l.enabled = v.get("enabled").and_then(Value::as_bool).unwrap_or(true);
        l.opacity = v.get("opacity").and_then(Value::as_f64).unwrap_or(1.0);
        if let Some(m) = v.get("weights").and_then(Value::as_object) {
            for (k, x) in m {
                if let (Ok(i), Some(f)) = (k.parse::<u32>(), floats(x)) {
                    l.weights.insert(i, f);
                }
            }
        }
        l.mask = v.get("mask").and_then(floats);
        l.dq = v.get("dq").and_then(floats);
        l.paint_target = v.get("paintTarget").and_then(target_from_json);
        l.locked = v.get("locked").and_then(Value::as_array).map(|a| a.iter().filter_map(|x| x.as_u64().map(|x| x as u32)).collect()).unwrap_or_default();
        if let Some(m) = v.get("mirror").and_then(Value::as_array) {
            for (k, x) in m.iter().take(3).enumerate() {
                l.mirror[k] = x.as_bool().unwrap_or(true);
            }
        }
        Some(l)
    }
}

fn target_from_json(v: &Value) -> Option<Target> {
    match v {
        Value::Number(n) => n.as_u64().map(|i| Target::Influence(i as u32)),
        Value::String(s) if s == "mask" => Some(Target::Mask),
        Value::String(s) if s == "dq" => Some(Target::Dq),
        _ => None,
    }
}

/// Which way Mirror copies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    PositiveToNegative,
    NegativeToPositive,
    Flip,
}

#[derive(Clone, Debug)]
pub struct MirrorConfig {
    pub axis: usize,
    pub direction: Direction,
    pub seam: f64,
    /// Influence pairs set by hand, over the ones found by name and place.
    pub pairs: Vec<(u32, u32)>,
}

impl Default for MirrorConfig {
    fn default() -> Self {
        MirrorConfig { axis: 0, direction: Direction::PositiveToNegative, seam: 0.0, pairs: Vec::new() }
    }
}

/// The layers of one skinCluster. Siblings keep their order in `layers`, the first at the bottom.
#[derive(Clone, Debug, Default)]
pub struct Layers {
    pub points: usize,
    pub layers: Vec<Layer>,
    pub current: Option<u32>,
    pub next_id: u32,
    pub mirror: MirrorConfig,
}

impl crate::plug::Data for Layers {
    fn kind(&self) -> &str {
        "skinLayers"
    }
    fn to_json(&self) -> Value {
        let dir = match self.mirror.direction {
            Direction::PositiveToNegative => "positiveToNegative",
            Direction::NegativeToPositive => "negativeToPositive",
            Direction::Flip => "flip",
        };
        json!({"kind": "skinLayers", "points": self.points, "current": self.current, "nextId": self.next_id,
               "layers": self.layers.iter().map(Layer::to_json).collect::<Vec<_>>(),
               "mirror": {"axis": self.mirror.axis, "direction": dir, "seam": self.mirror.seam, "pairs": self.mirror.pairs}})
    }
    fn bytes(&self) -> usize {
        self.layers.iter().map(|l| (l.weights.len() + 2) * self.points * 8).sum::<usize>() + 64
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Layers {
    pub fn from_json(v: &Value) -> Layers {
        let m = v.get("mirror").cloned().unwrap_or(Value::Null);
        Layers {
            points: v.get("points").and_then(Value::as_u64).unwrap_or(0) as usize,
            current: v.get("current").and_then(Value::as_u64).map(|x| x as u32),
            next_id: v.get("nextId").and_then(Value::as_u64).unwrap_or(0) as u32,
            layers: v.get("layers").and_then(Value::as_array).map(|a| a.iter().filter_map(Layer::from_json).collect()).unwrap_or_default(),
            mirror: MirrorConfig {
                axis: m.get("axis").and_then(Value::as_u64).unwrap_or(0).min(2) as usize,
                direction: match m.get("direction").and_then(Value::as_str).unwrap_or("") {
                    "negativeToPositive" => Direction::NegativeToPositive,
                    "flip" => Direction::Flip,
                    _ => Direction::PositiveToNegative,
                },
                seam: m.get("seam").and_then(Value::as_f64).unwrap_or(0.0),
                pairs: m.get("pairs").and_then(Value::as_array).map(|a| a.iter().filter_map(|p| Some((p.get(0)?.as_u64()? as u32, p.get(1)?.as_u64()? as u32))).collect()).unwrap_or_default(),
            },
        }
    }

    pub fn layer(&self, id: u32) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == id)
    }

    pub fn layer_mut(&mut self, id: u32) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    /// The children of a parent (None: the top level), bottom first.
    pub fn children(&self, parent: Option<u32>) -> Vec<u32> {
        self.layers.iter().filter(|l| l.parent == parent).map(|l| l.id).collect()
    }

    pub fn add(&mut self, name: &str, parent: Option<u32>) -> u32 {
        let id = self.next_id.max(self.layers.iter().map(|l| l.id + 1).max().unwrap_or(0));
        self.next_id = id + 1;
        self.layers.push(Layer::new(id, name, parent));
        self.current = Some(id);
        id
    }

    /// Takes a layer out, and its children with it.
    pub fn remove(&mut self, id: u32) {
        let mut gone = vec![id];
        let mut i = 0;
        while i < gone.len() {
            let p = gone[i];
            gone.extend(self.layers.iter().filter(|l| l.parent == Some(p)).map(|l| l.id));
            i += 1;
        }
        self.layers.retain(|l| !gone.contains(&l.id));
        if self.current.is_some_and(|c| gone.contains(&c)) {
            self.current = self.layers.last().map(|l| l.id);
        }
    }

    /// Moves a layer to a place among its siblings, 0 the bottom.
    pub fn set_index(&mut self, id: u32, index: usize) {
        let Some(pos) = self.layers.iter().position(|l| l.id == id) else { return };
        let l = self.layers.remove(pos);
        let sibs: Vec<usize> = self.layers.iter().enumerate().filter(|(_, x)| x.parent == l.parent).map(|(k, _)| k).collect();
        let at = if index >= sibs.len() { sibs.last().map(|k| k + 1).unwrap_or(self.layers.len()) } else { sibs[index] };
        self.layers.insert(at, l);
    }

    pub fn index_of(&self, id: u32) -> usize {
        let Some(l) = self.layer(id) else { return 0 };
        self.children(l.parent).iter().position(|&x| x == id).unwrap_or(0)
    }

    fn is_inside(&self, id: u32, group: u32) -> bool {
        let mut at = Some(id);
        while let Some(x) = at {
            if x == group {
                return true;
            }
            at = self.layer(x).and_then(|l| l.parent);
        }
        false
    }

    pub fn set_parent(&mut self, id: u32, parent: Option<u32>) -> Result<(), String> {
        if let Some(p) = parent {
            if self.layer(p).is_none() {
                return Err(format!("no layer {p}"));
            }
            if self.is_inside(p, id) {
                return Err("a layer can't go inside itself".into());
            }
        }
        let Some(pos) = self.layers.iter().position(|l| l.id == id) else { return Err(format!("no layer {id}")) };
        let mut l = self.layers.remove(pos);
        l.parent = parent;
        self.layers.push(l);
        Ok(())
    }

    /// The skinCluster's weights from the stack. `fallback` is what a point no layer covers keeps.
    pub fn compose(&self, fallback: &[Vec<(u32, f64)>]) -> (Vec<Vec<(u32, f64)>>, Option<Vec<f64>>) {
        let n = self.points;
        let below = vec![Vec::new(); n];
        let mut w = self.stack(None, below);
        for (p, pw) in w.iter_mut().enumerate() {
            let s: f64 = pw.iter().map(|x| x.1).sum();
            if s <= 1e-9 {
                *pw = fallback.get(p).cloned().unwrap_or_default();
                continue;
            }
            pw.retain(|x| x.1 > 1e-9);
            for x in pw.iter_mut() {
                x.1 /= s;
            }
            pw.sort_by_key(|x| x.0);
        }
        let dq = if self.layers.iter().any(|l| l.dq.is_some()) { Some(self.stack_dq(None, vec![0.0; n])) } else { None };
        (w, dq)
    }

    fn alpha(&self, l: &Layer, p: usize) -> f64 {
        (l.opacity * l.mask.as_ref().and_then(|m| m.get(p).copied()).unwrap_or(1.0)).clamp(0.0, 1.0)
    }

    fn stack(&self, parent: Option<u32>, mut cur: Vec<Vec<(u32, f64)>>) -> Vec<Vec<(u32, f64)>> {
        for id in self.children(parent) {
            let Some(l) = self.layer(id) else { continue };
            if !l.enabled {
                continue;
            }
            let group = self.layers.iter().any(|x| x.parent == Some(id));
            let own: Vec<Vec<(u32, f64)>> = if group {
                self.stack(Some(id), cur.clone())
            } else {
                (0..self.points)
                    .map(|p| {
                        let mut mine = l.at(p);
                        let s: f64 = mine.iter().map(|x| x.1).sum();
                        if s >= 1.0 - 1e-9 {
                            for x in mine.iter_mut() {
                                x.1 /= s;
                            }
                        } else {
                            // The rest from below, in its proportions.
                            let under = &cur[p];
                            let us: f64 = under.iter().map(|x| x.1).sum();
                            if us > 1e-12 {
                                let k = (1.0 - s) / us;
                                for &(i, x) in under {
                                    add(&mut mine, i, x * k);
                                }
                            }
                        }
                        mine
                    })
                    .collect()
            };
            for p in 0..self.points {
                let a = self.alpha(l, p);
                if a <= 0.0 {
                    continue;
                }
                let mut next: Vec<(u32, f64)> = cur[p].iter().map(|&(i, x)| (i, x * (1.0 - a))).collect();
                for &(i, x) in &own[p] {
                    add(&mut next, i, x * a);
                }
                next.retain(|x| x.1 > 1e-12);
                cur[p] = next;
            }
        }
        cur
    }

    fn stack_dq(&self, parent: Option<u32>, mut cur: Vec<f64>) -> Vec<f64> {
        for id in self.children(parent) {
            let Some(l) = self.layer(id) else { continue };
            if !l.enabled {
                continue;
            }
            let group = self.layers.iter().any(|x| x.parent == Some(id));
            let own = if group { self.stack_dq(Some(id), cur.clone()) } else if let Some(d) = &l.dq { d.clone() } else { continue };
            for p in 0..self.points {
                let a = self.alpha(l, p);
                cur[p] = cur[p] * (1.0 - a) + own.get(p).copied().unwrap_or(0.0) * a;
            }
        }
        cur
    }
}

fn add(w: &mut Vec<(u32, f64)>, i: u32, x: f64) {
    match w.iter_mut().find(|y| y.0 == i) {
        Some(y) => y.1 += x,
        None => w.push((i, x)),
    }
}

fn get(w: &[(u32, f64)], i: u32) -> f64 {
    w.iter().find(|x| x.0 == i).map(|x| x.1).unwrap_or(0.0)
}

// ---------------------------------------------------------------------------------------------------------------------
// The data node

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        crate::plug::data::register_kind("skinLayers", Arc::new(|j| Ok(Arc::new(Layers::from_json(j)) as Arc<dyn crate::plug::Data>)));
        let _ = types::register_type(NodeType {
            name: DATA_TYPE.into(),
            icon: "layers".into(),
            doc: "Skin Tools' layers for a skinCluster: each layer's weights, mask and dual quaternion weights, which blend into \
                  the skinCluster's weights."
                .into(),
            own_attrs: vec![
                AttrDef { name: "skinCluster".into(), short_name: "sc".into(), ty: PlugType::Int, default_value: PlugValue::Int(0), flags: flags::INPUT | flags::MESSAGE, doc: "The skinCluster the layers are for.".into(), ..Default::default() },
                AttrDef { name: "layerData".into(), short_name: "ld".into(), ty: PlugType::Data, default_value: PlugValue::Data(None), flags: flags::INPUT | flags::HIDDEN, doc: "The layers.".into(), ..Default::default() },
                AttrDef { name: "layerJson".into(), short_name: "lj".into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: flags::INPUT | flags::STORABLE | flags::HIDDEN, doc: "The layers as a file holds them; read into layerData.".into(), ..Default::default() },
            ],
            create: Some(Arc::new(|| Box::new(crate::node::Plain))),
            ..Default::default()
        });
    });
}

/// The layer data node of a skinCluster.
pub fn data_node(g: &Graph, sk: NodeId) -> Option<NodeId> {
    let msg = g.node(sk)?.ty().attr_index("message")?;
    g.destinations(&plug(sk, msg)).into_iter().find(|d| d.attr == D_SKIN as i32 && g.node(d.node).is_some_and(|n| n.ty().name == DATA_TYPE)).map(|d| d.node)
}

/// The skinCluster a layer data node is for.
pub fn skin_of_data(g: &Graph, d: NodeId) -> Option<NodeId> {
    g.source(&plug(d, D_SKIN)).map(|p| p.node)
}

pub fn layers_of(g: &Graph, sk: NodeId) -> Option<Layers> {
    let d = data_node(g, sk)?;
    let n = g.node(d)?;
    // As a file left them, until the first edit.
    if let PlugValue::String(j) = n.data().value(D_JSON) {
        if !j.is_empty() && !matches!(n.data().value(D_LAYERS), PlugValue::Data(Some(_))) {
            if let Ok(v) = serde_json::from_str::<Value>(j) {
                return Some(Layers::from_json(&v));
            }
        }
    }
    match n.data().value(D_LAYERS) {
        PlugValue::Data(Some(x)) => {
            if let Some(l) = x.as_any().downcast_ref::<Layers>() {
                return Some(l.clone());
            }
            x.as_any().downcast_ref::<JsonData>().map(|j| Layers::from_json(&j.0))
        }
        _ => Some(Layers::default()),
    }
}

/// Stores the layers and puts what they blend to on the skinCluster.
pub fn commit(g: &mut Graph, sk: NodeId, l: Layers) -> CmdResult<()> {
    let Some(d) = data_node(g, sk) else { fail!("{}: no skin layers", g.path(sk)) };
    let fallback = skin::bind_of(g, sk).map(|b| b.weights.clone()).unwrap_or_default();
    let (w, dq) = l.compose(&fallback);
    g.set_value(plug(d, D_LAYERS), PlugValue::Data(Some(Arc::new(l))))?;
    if !matches!(g.node(d).map(|n| n.data().value(D_JSON)), Some(PlugValue::String(j)) if j.is_empty()) {
        g.set_value(plug(d, D_JSON), PlugValue::String(String::new()))?;
    }
    skin::write_weights(g, sk, w)?;
    if let Some(dq) = dq {
        skin::write_blend_weights(g, sk, dq)?;
        if let Some(i) = g.node(sk).and_then(|n| n.ty().attr_index("skinningMethod")) {
            g.set_value(plug(sk, i), PlugValue::Int(2))?;
        }
    }
    Ok(())
}

/// Starts layers on a skinCluster: a data node, and a base layer holding the weights it has.
pub fn init(g: &mut Graph, sk: NodeId) -> CmdResult<NodeId> {
    if let Some(d) = data_node(g, sk) {
        return Ok(d);
    }
    register_types();
    let Some(b) = skin::bind_of(g, sk) else { fail!("{}: no bind data", g.path(sk)) };
    let name = format!("ngSkinToolsData_{}", g.node(sk).map(|n| n.name.clone()).unwrap_or_default());
    let t = types::find(DATA_TYPE).unwrap();
    let d = g.create_node(&t, &name, NO_NODE, true)?;
    let msg = g.node(sk).and_then(|n| n.ty().attr_index("message")).unwrap_or(0);
    g.connect(&plug(sk, msg), &plug(d, D_SKIN), true)?;
    let mut l = Layers { points: b.weights.len(), ..Default::default() };
    let id = l.add("base weights", None);
    let n = l.points;
    let base = l.layer_mut(id).unwrap();
    for (p, pw) in b.weights.iter().enumerate() {
        base.set_at(p, pw, n);
    }
    if let Some(bw) = skin::blend_weights(g, sk).filter(|v| v.iter().any(|x| *x > 0.0)) {
        let mut bw = bw;
        bw.resize(n, 0.0);
        base.dq = Some(bw);
    }
    g.set_value(plug(d, D_LAYERS), PlugValue::Data(Some(Arc::new(l))))?;
    Ok(d)
}

/// The skinCluster a name stands for, with layers or not.
fn the_skin(s: &Session, a: &Args, cmd: &str) -> CmdResult<NodeId> {
    let names = aztek_ops::objects(s, a);
    let Some(n) = names.first() else { fail!("{cmd}: name a skinned mesh or a skinCluster") };
    match skin::skin_named(&s.graph, n) {
        Some(sk) => Ok(sk),
        None => fail!("{cmd}: {n} has no skinCluster"),
    }
}

fn with_layers(s: &Session, a: &Args, cmd: &str) -> CmdResult<(NodeId, Layers)> {
    let sk = the_skin(s, a, cmd)?;
    match layers_of(&s.graph, sk) {
        Some(l) => Ok((sk, l)),
        None => fail!("{cmd}: {} has no skin layers; initialize them first", s.graph.path(sk)),
    }
}

fn the_layer(l: &Layers, a: &Args, cmd: &str) -> CmdResult<u32> {
    let id = match a.get("layer") {
        Some(v) => v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())).map(|x| x as u32),
        None => l.current,
    };
    match id {
        Some(i) if l.layer(i).is_some() => Ok(i),
        Some(i) => fail!("{cmd}: no layer {i}"),
        None => fail!("{cmd}: no layer given and none is current"),
    }
}

/// A target from a flag: mask, dq, an influence's matrix element, or an influence by name.
fn target_named(g: &Graph, sk: NodeId, v: &Value, cmd: &str) -> CmdResult<Target> {
    match v {
        Value::String(s) if s.eq_ignore_ascii_case("mask") => Ok(Target::Mask),
        Value::String(s) if s.eq_ignore_ascii_case("dq") || s.eq_ignore_ascii_case("dual_quaternion") => Ok(Target::Dq),
        Value::Number(n) => Ok(Target::Influence(n.as_u64().unwrap_or(0) as u32)),
        Value::String(s) => {
            if let Ok(i) = s.parse::<u32>() {
                return Ok(Target::Influence(i));
            }
            let infs = skin::influences(g, sk);
            let found = g.find(s).ok().flatten().and_then(|j| infs.iter().find(|x| x.1 == j));
            match found {
                Some(x) => Ok(Target::Influence(x.0 as u32)),
                None => fail!("{cmd}: {s} is not an influence of {}", g.path(sk)),
            }
        }
        _ => fail!("{cmd}: give a target: an influence, mask or dq"),
    }
}

fn layer_info(l: &Layers, x: &Layer) -> Value {
    json!({"id": x.id, "name": x.name, "enabled": x.enabled, "opacity": x.opacity, "parent": x.parent.map(|p| p as i64).unwrap_or(-1),
           "index": l.index_of(x.id), "children": l.children(Some(x.id)), "paintTarget": x.paint_target.map(Target::name),
           "hasMask": x.mask.is_some(), "hasDq": x.dq.is_some(), "locked": x.locked, "mirror": x.mirror,
           "influences": x.weights.iter().filter(|(_, v)| v.iter().any(|w| *w > 1e-9)).map(|(i, _)| *i).collect::<Vec<_>>()})
}

// ---------------------------------------------------------------------------------------------------------------------
// Editing a layer's values: flood, brush, smooth, sharpen

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Replace,
    Add,
    Scale,
    Smooth,
    Sharpen,
}

pub fn mode_named(s: &str) -> Option<Mode> {
    Some(match s.to_ascii_lowercase().as_str() {
        "replace" | "absolute" => Mode::Replace,
        "add" | "additive" => Mode::Add,
        "scale" => Mode::Scale,
        "smooth" => Mode::Smooth,
        "sharpen" => Mode::Sharpen,
        _ => return None,
    })
}

pub fn mode_name(m: Mode) -> &'static str {
    match m {
        Mode::Replace => "replace",
        Mode::Add => "add",
        Mode::Scale => "scale",
        Mode::Smooth => "smooth",
        Mode::Sharpen => "sharpen",
    }
}

/// The layer brush's settings, kept for the session as the reference's tool keeps them.
#[derive(Clone, Debug)]
pub struct Brush {
    pub mode: Mode,
    pub intensity: f64,
    pub iterations: usize,
    pub distribute: bool,
    pub mirror: bool,
}

impl Default for Brush {
    fn default() -> Self {
        Brush { mode: Mode::Replace, intensity: 1.0, iterations: 1, distribute: true, mirror: false }
    }
}

static BRUSH: Mutex<Option<Brush>> = Mutex::new(None);

pub fn brush() -> Brush {
    BRUSH.lock().unwrap().clone().unwrap_or_default()
}

fn set_brush(b: Brush) {
    *BRUSH.lock().unwrap() = Some(b);
}

/// Changes one point's value for a target, `f` the strength (brush falloff, or 1 for a flood). For an influence, the
/// layer's other unlocked influences give way when the point would hold more than one, and take up what is freed when
/// `distribute` is on and they hold any.
fn edit_point(l: &mut Layer, t: Target, p: usize, n: usize, mode: Mode, value: f64, f: f64, distribute: bool) {
    match t {
        Target::Mask | Target::Dq => {
            let v = l.values_mut(t, n);
            let now = v[p];
            v[p] = match mode {
                Mode::Replace => now + (value - now) * f,
                Mode::Add => now + value * f,
                Mode::Scale => now * (1.0 + (value - 1.0) * f),
                Mode::Sharpen => now + (now - 0.5) * value * f,
                Mode::Smooth => now,
            }
            .clamp(0.0, 1.0);
        }
        Target::Influence(i) => {
            if l.locked.contains(&i) {
                return;
            }
            let mut pw = l.at(p);
            let now = get(&pw, i);
            let locked: f64 = pw.iter().filter(|x| x.0 != i && l.locked.contains(&x.0)).map(|x| x.1).sum();
            let room = (1.0 - locked).max(0.0);
            let next = match mode {
                Mode::Replace => now + (value - now) * f,
                Mode::Add => now + value * f,
                Mode::Scale => now * (1.0 + (value - 1.0) * f),
                _ => now,
            }
            .clamp(0.0, room);
            let others: f64 = pw.iter().filter(|x| x.0 != i && !l.locked.contains(&x.0)).map(|x| x.1).sum();
            let free = room - next;
            let scale = if others > 1e-12 && (others > free || distribute) { free / others } else { 1.0 };
            for x in pw.iter_mut() {
                if x.0 != i && !l.locked.contains(&x.0) {
                    x.1 *= scale;
                }
            }
            match pw.iter_mut().find(|x| x.0 == i) {
                Some(x) => x.1 = next,
                None => pw.push((i, next)),
            }
            l.set_at(p, &pw, n);
        }
    }
}

/// Smooths the target over the points, each blended toward its neighbours' average by `f(p)`.
fn smooth_points(l: &mut Layer, t: Target, m: &Mesh, pts: &[(usize, f64)], n: usize) {
    match t {
        Target::Mask | Target::Dq => {
            let old = l.values_mut(t, n).clone();
            let v = l.values_mut(t, n);
            for &(p, f) in pts {
                let around = skin::neighbours(m, p as u32);
                if around.is_empty() {
                    continue;
                }
                let avg = around.iter().map(|&q| old.get(q as usize).copied().unwrap_or(0.0)).sum::<f64>() / around.len() as f64;
                v[p] = old[p] + (avg - old[p]) * f;
            }
        }
        Target::Influence(_) => {
            let old: Vec<Vec<(u32, f64)>> = (0..n).map(|p| l.at(p)).collect();
            for &(p, f) in pts {
                let around = skin::neighbours(m, p as u32);
                if around.is_empty() {
                    continue;
                }
                let own = &old[p];
                let mut avg: Vec<(u32, f64)> = Vec::new();
                for &q in &around {
                    for &(i, x) in old.get(q as usize).map(|v| v.as_slice()).unwrap_or(&[]) {
                        add(&mut avg, i, x / around.len() as f64);
                    }
                }
                let mut keys: Vec<u32> = own.iter().map(|x| x.0).chain(avg.iter().map(|x| x.0)).collect();
                keys.sort_unstable();
                keys.dedup();
                let next: Vec<(u32, f64)> = keys
                    .into_iter()
                    .map(|i| {
                        let a = get(own, i);
                        (i, if l.locked.contains(&i) { a } else { a + (get(&avg, i) - a) * f })
                    })
                    .filter(|x| x.1 > 1e-12)
                    .collect();
                l.set_at(p, &next, n);
            }
        }
    }
}

/// Sharpens a point's influences: the larger ones gain on the smaller, the total staying as it was.
fn sharpen_point(l: &mut Layer, p: usize, n: usize, f: f64) {
    let pw = l.at(p);
    let total: f64 = pw.iter().filter(|x| !l.locked.contains(&x.0)).map(|x| x.1).sum();
    if total <= 1e-12 {
        return;
    }
    let k = 1.0 + 2.0 * f.max(0.0);
    let raised: Vec<(u32, f64)> = pw.iter().map(|&(i, x)| (i, if l.locked.contains(&i) { x } else { (x / total).powf(k) })).collect();
    let rs: f64 = raised.iter().filter(|x| !l.locked.contains(&x.0)).map(|x| x.1).sum();
    let next: Vec<(u32, f64)> = raised.into_iter().map(|(i, x)| (i, if l.locked.contains(&i) { x } else { x / rs * total })).filter(|x| x.1 > 1e-12).collect();
    l.set_at(p, &next, n);
}

/// Applies a mode to the points, each with its strength.
pub fn apply(l: &mut Layer, t: Target, m: &Mesh, pts: &[(usize, f64)], n: usize, b: &Brush, invert: bool) {
    match b.mode {
        Mode::Smooth => {
            for _ in 0..b.iterations.max(1) {
                let scaled: Vec<(usize, f64)> = pts.iter().map(|&(p, f)| (p, f * b.intensity)).collect();
                smooth_points(l, t, m, &scaled, n);
            }
        }
        Mode::Sharpen if matches!(t, Target::Influence(_)) => {
            for _ in 0..b.iterations.max(1) {
                for &(p, f) in pts {
                    sharpen_point(l, p, n, f * b.intensity);
                }
            }
        }
        mode => {
            let value = match (mode, invert) {
                (Mode::Replace, true) => 1.0 - b.intensity,
                (Mode::Add, true) => -b.intensity,
                (Mode::Scale, true) => 1.0 / b.intensity.max(1e-6),
                _ => b.intensity,
            };
            for &(p, f) in pts {
                edit_point(l, t, p, n, mode, value, f, b.distribute);
            }
        }
    }
}

/// The skin's mesh in bind pose, in world space: what tools measure on.
fn bind_points(g: &mut Graph, sk: NodeId) -> Option<(Mesh, Vec<DVec3>)> {
    let (m, geom) = skin::bind_mesh(g, sk)?;
    let pts = (0..m.point_capacity()).map(|p| if m.point_valid(p) { geom.transform_point3(m.position(p)) } else { DVec3::splat(f64::NAN) }).collect();
    Some((m, pts))
}

/// The points a tool works on: the skin's mesh's selected points, or all.
fn tool_points(g: &mut Graph, sk: NodeId, selection_only: bool, n: usize) -> Vec<usize> {
    if selection_only {
        if let Some(mesh) = skin::skin_geometry(g, sk) {
            let sel = skin::selected_points(g, mesh);
            if !sel.is_empty() {
                return sel.into_iter().map(|p| p as usize).filter(|&p| p < n).collect();
            }
        }
    }
    (0..n).collect()
}

/// One dab of the layer brush at a world point on the posed mesh, as skin::stamp is for plain weights.
#[allow(clippy::too_many_arguments)]
pub fn stamp_layer(l: &mut Layers, layer: u32, t: Target, m: &Mesh, world: &DMat4, center: DVec3, normal: Option<DVec3>, ctx: &skin::PaintCtx, b: &Brush, invert: bool, smooth: bool) -> usize {
    let n = l.points;
    let mut hit = Vec::new();
    for p in 0..m.point_capacity() {
        if !m.point_valid(p) || p as usize >= n {
            continue;
        }
        let wp = world.transform_point3(m.position(p));
        let d = (wp - center).length() / ctx.radius.max(1e-9);
        if d >= 1.0 {
            continue;
        }
        if let Some(nn) = normal {
            let mut pn = DVec3::ZERO;
            for f in m.point_faces(p) {
                pn += m.face_area_vector(f);
            }
            if world.transform_vector3(pn).normalize_or_zero().dot(nn) < 0.0 {
                continue;
            }
        }
        hit.push((p as usize, skin::falloff(ctx.stamp, d) * ctx.opacity));
    }
    let b = if smooth { Brush { mode: Mode::Smooth, ..b.clone() } } else { b.clone() };
    if let Some(x) = l.layer_mut(layer) {
        apply(x, t, m, &hit, n, &b, invert);
    }
    hit.len()
}

/// A layer's values for a target, a value per point: an unset mask shows as full, unset dual quaternion weights as 0.
pub fn target_values(l: &Layers, layer: u32, t: Target) -> Vec<f64> {
    let fill = if t == Target::Mask { 1.0 } else { 0.0 };
    let mut v = l.layer(layer).and_then(|x| x.values(t).cloned()).unwrap_or_else(|| vec![fill; l.points]);
    v.resize(l.points, fill);
    v
}

/// The target a layer paints: its own, else the first influence.
pub fn paint_target(g: &Graph, sk: NodeId, l: &Layers, layer: u32) -> Option<Target> {
    l.layer(layer).and_then(|x| x.paint_target).or_else(|| skin::influences(g, sk).first().map(|x| Target::Influence(x.0 as u32)))
}

// ---------------------------------------------------------------------------------------------------------------------
// Tools: closest joint, unify, mirror, transfer

fn bind_position(b: &skin::SkinBind, inf: u32) -> DVec3 {
    b.bind_pre.get(inf as usize).map(|m| m.inverse().w_axis.truncate()).unwrap_or(DVec3::ZERO)
}

/// Each point to the one influence bound nearest it.
pub fn assign_closest(g: &mut Graph, sk: NodeId, l: &mut Layers, layer: u32, only: &[u32], pts: &[usize]) -> CmdResult<()> {
    let Some(b) = skin::bind_of(g, sk) else { fail!("no bind data") };
    let Some((_, at)) = bind_points(g, sk) else { fail!("no geometry") };
    let infs: Vec<u32> = if only.is_empty() { skin::influences(g, sk).iter().map(|x| x.0 as u32).collect() } else { only.to_vec() };
    if infs.is_empty() {
        fail!("no influences");
    }
    let places: Vec<(u32, DVec3)> = infs.iter().map(|&i| (i, bind_position(&b, i))).collect();
    let n = l.points;
    let Some(x) = l.layer_mut(layer) else { fail!("no layer {layer}") };
    for &p in pts {
        let Some(wp) = at.get(p).copied().filter(|v| v.is_finite()) else { continue };
        let best = places.iter().min_by(|a, c| (a.1 - wp).length_squared().total_cmp(&(c.1 - wp).length_squared())).map(|v| v.0).unwrap();
        x.set_at(p, &[(best, 1.0)], n);
    }
    Ok(())
}

/// The points all take the average of their weights.
pub fn unify(l: &mut Layer, pts: &[usize], n: usize) {
    if pts.is_empty() {
        return;
    }
    let mut avg: Vec<(u32, f64)> = Vec::new();
    for &p in pts {
        for (i, x) in l.at(p) {
            add(&mut avg, i, x / pts.len() as f64);
        }
    }
    for &p in pts {
        l.set_at(p, &avg, n);
    }
}

/// For each point, the point nearest its reflection across the axis.
fn mirror_points(at: &[DVec3], axis: usize) -> Vec<usize> {
    let mut cell = 0.0f64;
    let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
    for p in at.iter().filter(|p| p.is_finite()) {
        lo = lo.min(*p);
        hi = hi.max(*p);
    }
    if lo.x > hi.x {
        return (0..at.len()).collect();
    }
    let size = (hi - lo).max_element().max(1e-6);
    cell = cell.max(size / (at.len() as f64).cbrt().max(1.0));
    let key = |p: DVec3| ((p.x / cell).floor() as i64, (p.y / cell).floor() as i64, (p.z / cell).floor() as i64);
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for (i, p) in at.iter().enumerate().filter(|(_, p)| p.is_finite()) {
        grid.entry(key(*p)).or_default().push(i);
    }
    at.iter()
        .enumerate()
        .map(|(i, p)| {
            if !p.is_finite() {
                return i;
            }
            let mut q = *p;
            q[axis] = -q[axis];
            let (cx, cy, cz) = key(q);
            let mut best = (f64::MAX, i);
            let mut r = 1;
            loop {
                for x in cx - r..=cx + r {
                    for y in cy - r..=cy + r {
                        for z in cz - r..=cz + r {
                            for &j in grid.get(&(x, y, z)).map(|v| v.as_slice()).unwrap_or(&[]) {
                                let d = (at[j] - q).length_squared();
                                if d < best.0 {
                                    best = (d, j);
                                }
                            }
                        }
                    }
                }
                if best.0 < f64::MAX || r > 64 {
                    break;
                }
                r *= 2;
            }
            best.1
        })
        .collect()
}

/// A name with its side swapped: L/R as prefix, suffix or a part between underscores (arm_L0_0_jnt), left/right.
pub fn other_side(name: &str) -> Option<String> {
    if let Some(m) = skin::mirrored_name(name) {
        return Some(m);
    }
    let parts: Vec<&str> = name.split('_').collect();
    for (k, part) in parts.iter().enumerate() {
        let swapped = match part.chars().next() {
            Some('L') if part[1..].chars().all(|c| c.is_ascii_digit()) => format!("R{}", &part[1..]),
            Some('R') if part[1..].chars().all(|c| c.is_ascii_digit()) => format!("L{}", &part[1..]),
            Some('l') if part[1..].chars().all(|c| c.is_ascii_digit()) => format!("r{}", &part[1..]),
            Some('r') if part[1..].chars().all(|c| c.is_ascii_digit()) => format!("l{}", &part[1..]),
            _ => continue,
        };
        let mut v: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
        v[k] = swapped;
        return Some(v.join("_"));
    }
    None
}

/// Each influence's counterpart across the mirror axis: set by hand, else by name, else the influence bound nearest
/// its mirrored place (within a small distance), else itself.
pub fn influence_pairs(g: &Graph, sk: NodeId, cfg: &MirrorConfig) -> HashMap<u32, u32> {
    let infs = skin::influences(g, sk);
    let b = skin::bind_of(g, sk);
    let mut out = HashMap::new();
    for &(e, j) in &infs {
        let e = e as u32;
        if let Some(&(_, o)) = cfg.pairs.iter().find(|p| p.0 == e) {
            out.insert(e, o);
            continue;
        }
        if let Some(&(o, _)) = cfg.pairs.iter().find(|p| p.1 == e) {
            out.insert(e, o);
            continue;
        }
        let name = g.node(j).map(|n| n.name.clone()).unwrap_or_default();
        if let Some(other) = other_side(&name) {
            if let Some(x) = infs.iter().find(|x| g.node(x.1).is_some_and(|n| n.name == other)) {
                out.insert(e, x.0 as u32);
                continue;
            }
        }
        if let Some(b) = &b {
            let mut q = bind_position(b, e);
            q[cfg.axis] = -q[cfg.axis];
            let near = infs.iter().map(|x| (x.0 as u32, (bind_position(b, x.0 as u32) - q).length())).min_by(|a, c| a.1.total_cmp(&c.1));
            if let Some((o, d)) = near {
                if d < 1e-2 {
                    out.insert(e, o);
                    continue;
                }
            }
        }
        out.insert(e, e);
    }
    out
}

/// Mirrors a layer's weights, mask and dual quaternion weights (as `what` says) across the axis.
pub fn mirror(g: &mut Graph, sk: NodeId, l: &mut Layers, layer: u32, what: [bool; 3]) -> CmdResult<()> {
    let Some((_, at)) = bind_points(g, sk) else { fail!("no geometry") };
    let cfg = l.mirror.clone();
    let pairs = influence_pairs(g, sk, &cfg);
    let across = mirror_points(&at, cfg.axis);
    let n = l.points;
    let Some(x) = l.layer_mut(layer) else { fail!("no layer {layer}") };
    let old: Vec<Vec<(u32, f64)>> = (0..n).map(|p| x.at(p)).collect();
    let old_mask = x.mask.clone();
    let old_dq = x.dq.clone();
    let seam = cfg.seam.max(0.0);
    for p in 0..n.min(at.len()) {
        let c = at[p][cfg.axis];
        if !c.is_finite() {
            continue;
        }
        let takes = match cfg.direction {
            Direction::PositiveToNegative => c < 0.0 || (seam > 0.0 && c.abs() < seam),
            Direction::NegativeToPositive => c > 0.0 || (seam > 0.0 && c.abs() < seam),
            Direction::Flip => true,
        };
        if !takes {
            continue;
        }
        let q = across[p];
        // Within the seam the point blends from the average of the two sides at the axis to the mirrored value.
        let t = if seam > 0.0 && cfg.direction != Direction::Flip { (c.abs() / seam).min(1.0) } else { 1.0 };
        let t = if seam > 0.0 && cfg.direction != Direction::Flip && c.abs() < seam { 0.5 + 0.5 * t } else { 1.0 };
        if what[0] {
            let mirrored: Vec<(u32, f64)> = old[q].iter().map(|&(i, w)| (pairs.get(&i).copied().unwrap_or(i), w)).collect();
            let mut next: Vec<(u32, f64)> = old[p].iter().map(|&(i, w)| (i, w * (1.0 - t))).collect();
            for (i, w) in mirrored {
                add(&mut next, i, w * t);
            }
            next.retain(|y| y.1 > 1e-12);
            x.set_at(p, &next, n);
        }
        if what[1] {
            if let Some(m) = &old_mask {
                let v = m.get(p).copied().unwrap_or(1.0) * (1.0 - t) + m.get(q).copied().unwrap_or(1.0) * t;
                x.values_mut(Target::Mask, n)[p] = v;
            }
        }
        if what[2] {
            if let Some(d) = &old_dq {
                let v = d.get(p).copied().unwrap_or(0.0) * (1.0 - t) + d.get(q).copied().unwrap_or(0.0) * t;
                x.values_mut(Target::Dq, n)[p] = v;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
// Files and transfer

/// The layers as a file holds them: influences by name and bind place, the points' bind places, and the layers with
/// influences by their position in that list.
pub fn export_value(g: &mut Graph, sk: NodeId, l: &Layers) -> CmdResult<Value> {
    let Some(b) = skin::bind_of(g, sk) else { fail!("no bind data") };
    let Some((_, at)) = bind_points(g, sk) else { fail!("no geometry") };
    let infs = skin::influences(g, sk);
    let influences: Vec<Value> = infs.iter().map(|&(e, j)| json!({"index": e, "path": g.path(j), "name": g.node(j).map(|n| n.name.clone()).unwrap_or_default(), "pivot": bind_position(&b, e as u32).to_array()})).collect();
    let mut v = l.to_json_value();
    v["influences"] = json!(influences);
    v["mesh"] = json!({"vertices": at.iter().map(|p| if p.is_finite() { json!(p.to_array()) } else { Value::Null }).collect::<Vec<_>>()});
    v["format"] = json!("aztek-skin-layers-1");
    Ok(v)
}

impl Layers {
    fn to_json_value(&self) -> Value {
        crate::plug::Data::to_json(self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Transfer {
    VertexId,
    ClosestPoint,
}

/// Layers from a file's value or another skin's export onto a skin: points by id or by nearest bind place, influences
/// by name, else by nearest pivot.
pub fn import_value(g: &mut Graph, sk: NodeId, v: &Value, mode: Transfer) -> CmdResult<Layers> {
    let src = Layers::from_json(v);
    let Some((_, at)) = bind_points(g, sk) else { fail!("no geometry") };
    let Some(b) = skin::bind_of(g, sk) else { fail!("no bind data") };
    let n = at.len();
    let infs = skin::influences(g, sk);
    // Source influence element -> ours.
    let mut map: HashMap<u32, u32> = HashMap::new();
    for i in v.get("influences").and_then(Value::as_array).cloned().unwrap_or_default() {
        let Some(e) = i.get("index").and_then(Value::as_u64) else { continue };
        let name = i.get("name").and_then(Value::as_str).unwrap_or("");
        let path = i.get("path").and_then(Value::as_str).unwrap_or("");
        let found = infs.iter().find(|x| g.path(x.1) == path).or_else(|| infs.iter().find(|x| g.node(x.1).is_some_and(|n| n.name == name || n.name.rsplit(':').next() == name.rsplit(':').next())));
        let found = found.map(|x| x.0 as u32).or_else(|| {
            let pv = i.get("pivot").and_then(Value::as_array).map(|a| DVec3::new(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0), a[2].as_f64().unwrap_or(0.0)))?;
            infs.iter().map(|x| (x.0 as u32, (bind_position(&b, x.0 as u32) - pv).length())).min_by(|a, c| a.1.total_cmp(&c.1)).map(|x| x.0)
        });
        if let Some(f) = found {
            map.insert(e as u32, f);
        }
    }
    if map.is_empty() {
        fail!("none of the file's influences are on {}", g.path(sk));
    }
    // Our point -> the source's.
    let src_at: Vec<DVec3> = v.pointer("/mesh/vertices").and_then(Value::as_array).map(|a| a.iter().map(|p| p.as_array().map(|c| DVec3::new(c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0), c[2].as_f64().unwrap_or(0.0))).unwrap_or(DVec3::splat(f64::NAN))).collect()).unwrap_or_default();
    let from: Vec<usize> = match mode {
        Transfer::VertexId => (0..n).map(|p| p.min(src.points.saturating_sub(1))).collect(),
        Transfer::ClosestPoint => {
            if src_at.is_empty() {
                fail!("the file has no point places to match by; transfer by vertex id");
            }
            nearest_each(&at, &src_at)
        }
    };
    let mut out = Layers { points: n, current: src.current, next_id: src.next_id, mirror: src.mirror.clone(), layers: Vec::new() };
    for sl in &src.layers {
        let mut l = Layer { weights: BTreeMap::new(), mask: None, dq: None, ..sl.clone() };
        l.paint_target = match sl.paint_target {
            Some(Target::Influence(i)) => map.get(&i).map(|&x| Target::Influence(x)),
            t => t,
        };
        l.locked = sl.locked.iter().filter_map(|i| map.get(i).copied()).collect();
        for (i, w) in &sl.weights {
            let Some(&to) = map.get(i) else { continue };
            let v = l.weights.entry(to).or_insert_with(|| vec![0.0; n]);
            for p in 0..n {
                v[p] += w.get(from[p]).copied().unwrap_or(0.0);
            }
        }
        l.mask = sl.mask.as_ref().map(|m| from.iter().map(|&q| m.get(q).copied().unwrap_or(1.0)).collect());
        l.dq = sl.dq.as_ref().map(|m| from.iter().map(|&q| m.get(q).copied().unwrap_or(0.0)).collect());
        out.layers.push(l);
    }
    Ok(out)
}

fn nearest_each(at: &[DVec3], src: &[DVec3]) -> Vec<usize> {
    let ok: Vec<usize> = (0..src.len()).filter(|&i| src[i].is_finite()).collect();
    let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
    for &i in &ok {
        lo = lo.min(src[i]);
        hi = hi.max(src[i]);
    }
    let cell = ((hi - lo).max_element().max(1e-6)) / (ok.len() as f64).cbrt().max(1.0);
    let key = |p: DVec3| ((p.x / cell).floor() as i64, (p.y / cell).floor() as i64, (p.z / cell).floor() as i64);
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for &i in &ok {
        grid.entry(key(src[i])).or_default().push(i);
    }
    at.iter()
        .map(|&q| {
            if !q.is_finite() {
                return 0;
            }
            let (cx, cy, cz) = key(q);
            let mut best = (f64::MAX, 0usize);
            let mut r = 1;
            loop {
                for x in cx - r..=cx + r {
                    for y in cy - r..=cy + r {
                        for z in cz - r..=cz + r {
                            for &j in grid.get(&(x, y, z)).map(|v| v.as_slice()).unwrap_or(&[]) {
                                let d = (src[j] - q).length_squared();
                                if d < best.0 {
                                    best = (d, j);
                                }
                            }
                        }
                    }
                }
                if best.0 < f64::MAX || r > 1 << 12 {
                    break;
                }
                r *= 2;
            }
            best.1
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------------------------------
// Commands

fn fl(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

fn skin_layers_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let cmd = "skinLayers";
    let sk = the_skin(s, a, cmd)?;
    let g = &mut s.graph;
    if a.bool("initialize") {
        let d = init(g, sk)?;
        let l = layers_of(g, sk).unwrap_or_default();
        commit(g, sk, l)?;
        return Ok(json!(g.path(d)));
    }
    if a.bool("query") && a.has("exists") {
        return Ok(json!(data_node(g, sk).is_some()));
    }
    if a.bool("delete") {
        if let Some(d) = data_node(g, sk) {
            g.delete_node(d)?;
        }
        return Ok(Value::Null);
    }
    if a.bool("query") && a.has("influences") {
        let infs = skin::influences(g, sk);
        return Ok(json!(infs.iter().map(|&(e, j)| json!({"index": e, "path": g.path(j), "name": g.node(j).map(|n| n.name.clone()).unwrap_or_default()})).collect::<Vec<_>>()));
    }
    let Some(mut l) = layers_of(g, sk) else { fail!("{cmd}: {} has no skin layers; initialize them first", g.path(sk)) };
    if a.bool("query") {
        if a.has("layers") {
            return Ok(json!(l.layers.iter().map(|x| layer_info(&l, x)).collect::<Vec<_>>()));
        }
        if a.has("current") {
            return Ok(json!(l.current.map(|c| c as i64).unwrap_or(-1)));
        }
        if a.has("dataNode") {
            return Ok(json!(data_node(g, sk).map(|d| g.path(d))));
        }
        if a.has("mirrorAxis") || a.has("mirrorDirection") || a.has("seamWidth") {
            let dir = match l.mirror.direction {
                Direction::PositiveToNegative => "positiveToNegative",
                Direction::NegativeToPositive => "negativeToPositive",
                Direction::Flip => "flip",
            };
            let axis = ["x", "y", "z"][l.mirror.axis];
            return Ok(json!({"axis": axis, "direction": dir, "seamWidth": l.mirror.seam}));
        }
        if a.has("influencePairs") {
            let pairs = influence_pairs(g, sk, &l.mirror);
            let mut v: Vec<(u32, u32)> = pairs.into_iter().collect();
            v.sort();
            return Ok(json!(v.iter().map(|p| json!([p.0, p.1])).collect::<Vec<_>>()));
        }
        let id = the_layer(&l, a, cmd)?;
        let x = l.layer(id).unwrap();
        if a.has("weights") {
            let t = target_named(g, sk, a.get("target").unwrap_or(&Value::Null), cmd)?;
            return Ok(json!(x.values(t).cloned().unwrap_or_default()));
        }
        return Ok(layer_info(&l, x));
    }
    // Mirror settings for the skin.
    if let Some(ax) = a.get("mirrorAxis").and_then(Value::as_str) {
        l.mirror.axis = match ax.to_ascii_lowercase().as_str() {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            o => fail!("{cmd}: -mirrorAxis {o}: give x, y or z"),
        };
    }
    if let Some(d) = a.get("mirrorDirection").and_then(Value::as_str) {
        l.mirror.direction = match d {
            "positiveToNegative" | "positive_to_negative" => Direction::PositiveToNegative,
            "negativeToPositive" | "negative_to_positive" => Direction::NegativeToPositive,
            "flip" => Direction::Flip,
            o => fail!("{cmd}: -mirrorDirection {o}: give positiveToNegative, negativeToPositive or flip"),
        };
    }
    if a.has("seamWidth") {
        l.mirror.seam = a.float("seamWidth", 0.0).max(0.0);
    }
    if let Some(p) = a.get("influencePairs").and_then(Value::as_array) {
        l.mirror.pairs = p.iter().filter_map(|x| Some((x.get(0)?.as_u64()? as u32, x.get(1)?.as_u64()? as u32))).collect();
    }
    let result;
    if let Some(name) = a.get("add").and_then(Value::as_str) {
        let parent = a.get("parent").and_then(Value::as_i64).filter(|&p| p >= 0).map(|p| p as u32);
        if let Some(p) = parent {
            if l.layer(p).is_none() {
                fail!("{cmd}: no layer {p}");
            }
        }
        let id = l.add(name, parent);
        if let Some(i) = a.get("index").and_then(Value::as_u64) {
            l.set_index(id, i as usize);
        }
        result = json!(id);
    } else if let Some(id) = a.get("remove").and_then(Value::as_u64) {
        if l.layer(id as u32).is_none() {
            fail!("{cmd}: no layer {id}");
        }
        l.remove(id as u32);
        result = Value::Null;
    } else if let Some(id) = a.get("duplicate").and_then(Value::as_u64) {
        let Some(src) = l.layer(id as u32).cloned() else { fail!("{cmd}: no layer {id}") };
        let nid = l.add(&format!("{} copy", src.name), src.parent);
        let pos = l.index_of(id as u32) + 1;
        let x = l.layer_mut(nid).unwrap();
        *x = Layer { id: nid, name: x.name.clone(), ..src };
        l.set_index(nid, pos);
        result = json!(nid);
    } else if let Some(id) = a.get("mergeDown").and_then(Value::as_u64) {
        // The layer and the one under it become one, blended as the stack blends them.
        let id = id as u32;
        let Some(top) = l.layer(id).cloned() else { fail!("{cmd}: no layer {id}") };
        let sibs = l.children(top.parent);
        let k = sibs.iter().position(|&x| x == id).unwrap_or(0);
        if k == 0 {
            fail!("{cmd}: layer {id} has nothing under it to merge into");
        }
        let under = sibs[k - 1];
        let part = Layers { points: l.points, layers: vec![Layer { parent: None, ..l.layer(under).cloned().unwrap() }, Layer { parent: None, ..top.clone() }], ..Default::default() };
        let (w, dq) = part.compose(&vec![Vec::new(); l.points]);
        let n = l.points;
        let u = l.layer_mut(under).unwrap();
        u.weights.clear();
        for (p, pw) in w.iter().enumerate() {
            u.set_at(p, pw, n);
        }
        if dq.is_some() {
            u.dq = dq;
        }
        u.mask = None;
        u.opacity = 1.0;
        l.remove(id);
        l.current = Some(under);
        result = json!(under);
    } else {
        result = Value::Null;
    }
    if a.has("current") && !a.bool("query") {
        let c = a.int("current", -1);
        if c < 0 || l.layer(c as u32).is_none() {
            fail!("{cmd}: no layer {c}");
        }
        l.current = Some(c as u32);
    }
    // Edits of one layer.
    if a.bool("edit") {
        let id = the_layer(&l, a, cmd)?;
        if let Some(p) = a.get("parent").and_then(Value::as_i64) {
            l.set_parent(id, if p < 0 { None } else { Some(p as u32) }).map_err(|e| aztek_ops::CommandError::new(format!("{cmd}: {e}")))?;
        }
        if let Some(i) = a.get("index").and_then(Value::as_u64) {
            l.set_index(id, i as usize);
        }
        let t = match a.get("target") {
            Some(v) => Some(target_named(g, sk, v, cmd)?),
            None => None,
        };
        let n = l.points;
        let x = l.layer_mut(id).unwrap();
        if let Some(v) = a.get("name").and_then(Value::as_str) {
            x.name = v.to_string();
        }
        if let Some(v) = a.get("enabled") {
            x.enabled = v.as_bool().unwrap_or_else(|| v.as_f64().unwrap_or(1.0) != 0.0);
        }
        if a.has("opacity") {
            x.opacity = a.float("opacity", 1.0).clamp(0.0, 1.0);
        }
        if a.has("paintTarget") {
            x.paint_target = Some(target_named(g, sk, a.get("paintTarget").unwrap(), cmd)?);
        }
        if let Some(v) = a.get("lockInfluences").and_then(Value::as_array) {
            let mut locks = Vec::new();
            for i in v {
                if let Target::Influence(e) = target_named(g, sk, i, cmd)? {
                    locks.push(e);
                }
            }
            x.locked = locks;
        }
        if let Some(m) = a.get("mirrorEffects").and_then(Value::as_array) {
            for (k, v) in m.iter().take(3).enumerate() {
                x.mirror[k] = v.as_bool().unwrap_or(true);
            }
        }
        if let Some(w) = a.get("weights") {
            let Some(t) = t else { fail!("{cmd}: -weights needs -target") };
            let vals: Vec<f64> = match w {
                Value::Array(v) => v.iter().map(|x| x.as_f64().unwrap_or(0.0)).collect(),
                _ => fail!("{cmd}: -weights takes a list of values, one per point"),
            };
            match t {
                Target::Mask | Target::Dq if vals.is_empty() => {
                    if t == Target::Mask {
                        x.mask = None;
                    } else {
                        x.dq = None;
                    }
                }
                _ => {
                    if vals.len() != n {
                        fail!("{cmd}: {} values for {n} points", vals.len());
                    }
                    *x.values_mut(t, n) = vals;
                }
            }
        }
    }
    commit(g, sk, l)?;
    Ok(result)
}

fn skin_layers_tool_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let cmd = "skinLayersTool";
    // The brush's settings need no skin.
    let settings = ["mode", "intensity", "iterations", "distribute", "brushMirror"];
    if !a.bool("flood") && !a.has("stamp") && settings.iter().any(|f| a.has(f)) && a.positional.is_empty() || (a.bool("query") && settings.iter().any(|f| a.has(f))) {
        let mut b = brush();
        if a.bool("query") {
            return Ok(json!({"mode": mode_name(b.mode), "intensity": b.intensity, "iterations": b.iterations, "distribute": b.distribute, "mirror": b.mirror}));
        }
        if let Some(m) = a.get("mode").and_then(Value::as_str) {
            b.mode = mode_named(m).ok_or_else(|| aztek_ops::CommandError::new(format!("{cmd}: -mode {m}: give replace, add, scale, smooth or sharpen")))?;
        }
        if a.has("intensity") {
            b.intensity = a.float("intensity", 1.0);
        }
        if a.has("iterations") {
            b.iterations = a.int("iterations", 1).clamp(1, 1000) as usize;
        }
        if a.has("distribute") {
            b.distribute = a.bool("distribute");
        }
        if a.has("brushMirror") {
            b.mirror = a.bool("brushMirror");
        }
        set_brush(b);
        return Ok(Value::Null);
    }
    if let Some(src) = a.get("transferFrom").and_then(Value::as_str) {
        let dst = the_skin(s, a, cmd)?;
        let Some(ss) = skin::skin_named(&s.graph, src) else { fail!("{cmd}: {src} has no skinCluster") };
        let Some(sl) = layers_of(&s.graph, ss) else { fail!("{cmd}: {src} has no skin layers") };
        let v = export_value(&mut s.graph, ss, &sl)?;
        let mode = transfer_mode(a, cmd)?;
        let out = import_value(&mut s.graph, dst, &v, mode)?;
        init(&mut s.graph, dst)?;
        commit(&mut s.graph, dst, out)?;
        return Ok(Value::Null);
    }
    if let Some(file) = a.get("importFile").and_then(Value::as_str).map(str::to_string) {
        let sk = the_skin(s, a, cmd)?;
        let text = std::fs::read_to_string(&file).map_err(|e| aztek_ops::CommandError::new(format!("{cmd}: {file}: {e}")))?;
        let v: Value = serde_json::from_str(&text).map_err(|e| aztek_ops::CommandError::new(format!("{cmd}: {file}: {e}")))?;
        let mode = transfer_mode(a, cmd)?;
        let out = import_value(&mut s.graph, sk, &v, mode)?;
        init(&mut s.graph, sk)?;
        commit(&mut s.graph, sk, out)?;
        return Ok(Value::Null);
    }
    let (sk, mut l) = with_layers(s, a, cmd)?;
    let g = &mut s.graph;
    if let Some(file) = a.get("exportFile").and_then(Value::as_str) {
        let v = export_value(g, sk, &l)?;
        std::fs::write(file, serde_json::to_string(&v).unwrap_or_default()).map_err(|e| aztek_ops::CommandError::new(format!("{cmd}: {file}: {e}")))?;
        return Ok(json!(file));
    }
    let id = the_layer(&l, a, cmd)?;
    let n = l.points;
    let pts = tool_points(g, sk, a.bool("selectedComponents"), n);
    if a.bool("flood") {
        let t = match a.get("target") {
            Some(v) => target_named(g, sk, v, cmd)?,
            None => match paint_target(g, sk, &l, id) {
                Some(t) => t,
                None => fail!("{cmd}: no target to flood"),
            },
        };
        let mut b = brush();
        if let Some(m) = a.get("mode").and_then(Value::as_str) {
            b.mode = mode_named(m).ok_or_else(|| aztek_ops::CommandError::new(format!("{cmd}: -mode {m}: give replace, add, scale, smooth or sharpen")))?;
        }
        if a.has("intensity") {
            b.intensity = a.float("intensity", 1.0);
        }
        if a.has("iterations") {
            b.iterations = a.int("iterations", 1).clamp(1, 1000) as usize;
        }
        if a.has("distribute") {
            b.distribute = a.bool("distribute");
        }
        let Some((m, _)) = bind_points(g, sk) else { fail!("{cmd}: no geometry") };
        let strength: Vec<(usize, f64)> = pts.iter().map(|&p| (p, 1.0)).collect();
        apply(l.layer_mut(id).unwrap(), t, &m, &strength, n, &b, false);
        if a.bool("brushMirror") || (b.mirror && !a.has("brushMirror")) {
            let what = [matches!(t, Target::Influence(_)), t == Target::Mask, t == Target::Dq];
            mirror(g, sk, &mut l, id, what)?;
        }
    } else if a.bool("assignFromClosestJoint") {
        let mut only = Vec::new();
        for v in a.get("influences").and_then(Value::as_array).cloned().unwrap_or_default() {
            if let Target::Influence(e) = target_named(g, sk, &v, cmd)? {
                only.push(e);
            }
        }
        assign_closest(g, sk, &mut l, id, &only, &pts)?;
    } else if a.bool("unify") {
        unify(l.layer_mut(id).unwrap(), &pts, n);
    } else if a.bool("mirror") {
        let x = l.layer(id).unwrap();
        let mut what = x.mirror;
        if a.has("mirrorWeights") || a.has("mirrorMask") || a.has("mirrorDq") {
            what = [a.bool("mirrorWeights"), a.bool("mirrorMask"), a.bool("mirrorDq")];
        }
        mirror(g, sk, &mut l, id, what)?;
    } else if let Some(at) = a.vec3("stamp") {
        // One dab of the layer brush on the posed mesh, for scripts and tests.
        let Some(mesh) = skin::skin_geometry(g, sk) else { fail!("{cmd}: no geometry") };
        g.evaluate();
        let Some(m) = crate::components::mesh_of(g, mesh) else { fail!("{cmd}: no geometry") };
        let xf = crate::dag::transform_of(g, mesh);
        let w = crate::rig::world(g, xf);
        let Some(t) = paint_target(g, sk, &l, id) else { fail!("{cmd}: no target to paint") };
        let ctx = s.paint_ctx.clone();
        let b = brush();
        let hit = stamp_layer(&mut l, id, t, &m, &w, DVec3::from_array(at), None, &ctx, &b, a.bool("invert"), a.bool("smooth"));
        commit(&mut s.graph, sk, l)?;
        return Ok(json!(hit));
    } else {
        fail!("{cmd}: give -flood, -assignFromClosestJoint, -unify, -mirror, -stamp, -exportFile, -importFile or -transferFrom");
    }
    commit(g, sk, l)?;
    Ok(Value::Null)
}

fn transfer_mode(a: &Args, cmd: &str) -> CmdResult<Transfer> {
    Ok(match a.string("vertexTransferMode", "closestPoint").as_str() {
        "closestPoint" | "closest_point" => Transfer::ClosestPoint,
        "vertexId" | "vertex_id" => Transfer::VertexId,
        o => fail!("{cmd}: -vertexTransferMode {o}: give closestPoint or vertexId"),
    })
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        CommandDef::<Session>::new(
            "skinLayers",
            "Skin Tools' layers on a skinned mesh or skinCluster. -init starts them (a base layer with the weights the skin has), \
             -delete takes them away leaving the weights as they blend; -add name a layer (-parent, -index), -remove, -duplicate, \
             -mergeDown id; -e -layer id with -name, -enabled, -opacity, -parent (-1 the top), -index, -paintTarget, \
             -lockInfluences, -mirrorEffects, or -weights with -target (an influence, mask or dq). -q with -layers, -current, \
             -influences, -exists, -dataNode, -weights -target, or -layer id for one layer. -mirrorAxis, -mirrorDirection, \
             -seamWidth and -influencePairs set how Mirror works on this skin.",
            skin_layers_cmd,
        )
        .positional("a skinned mesh or a skinCluster")
        .flag(fl("initialize", "init", ArgType::Bool, "Start layers on the skin."))
        .flag(fl("delete", "d", ArgType::Bool, "Take the layers away; the weights stay."))
        .flag(fl("exists", "ex", ArgType::Bool, "With -q: whether the skin has layers."))
        .flag(fl("layers", "ls", ArgType::Bool, "With -q: every layer, bottom first among siblings."))
        .flag(fl("influences", "inf", ArgType::Bool, "With -q: the skin's influences, by matrix element."))
        .flag(fl("dataNode", "dn", ArgType::Bool, "With -q: the node holding the layers."))
        .flag(fl("add", "a", ArgType::String, "Add a layer of this name, empty, and make it current."))
        .flag(fl("remove", "rm", ArgType::Int, "Remove a layer and its children."))
        .flag(fl("duplicate", "dup", ArgType::Int, "Copy a layer, above it."))
        .flag(fl("mergeDown", "md", ArgType::Int, "Merge a layer into the one under it."))
        .flag(fl("layer", "l", ArgType::Any, "The layer edited or queried; the current one if not given."))
        .flag(fl("current", "cur", ArgType::Any, "The current layer, the one tools and the brush work on."))
        .flag(fl("name", "n", ArgType::String, "A layer's name."))
        .flag(fl("enabled", "en", ArgType::Any, "Whether a layer takes part."))
        .flag(fl("opacity", "op", ArgType::Float, "How much a layer covers what is below, 0 to 1."))
        .flag(fl("parent", "p", ArgType::Int, "The layer group a layer is in, -1 none."))
        .flag(fl("index", "i", ArgType::Int, "A layer's place among its siblings, 0 the bottom."))
        .flag(fl("paintTarget", "pt", ArgType::Any, "What the brush paints on the layer: an influence, mask or dq."))
        .flag(fl("lockInfluences", "li", ArgType::Any, "The influences whose weights on the layer no tool changes.").multi(1))
        .flag(fl("mirrorEffects", "me", ArgType::Any, "What Mirror mirrors on the layer: three booleans, weights, mask, dq.").multi(1))
        .flag(fl("target", "t", ArgType::Any, "With -weights: an influence, mask or dq."))
        .flag(fl("weights", "w", ArgType::Any, "A value per point for -target; with -q, gives them. An empty list removes a mask or dq.").multi(1))
        .flag(fl("mirrorAxis", "ma", ArgType::Any, "x, y or z."))
        .flag(fl("mirrorDirection", "mdi", ArgType::Any, "positiveToNegative, negativeToPositive or flip."))
        .flag(fl("seamWidth", "sw", ArgType::Any, "Around the axis, how wide a band blends the two sides."))
        .flag(fl("influencePairs", "ip", ArgType::Any, "Influence pairs across the axis, by matrix element; with -q, every influence's pair.").multi(1))
        .flag(fl("query", "q", ArgType::Bool, "Query."))
        .flag(fl("edit", "e", ArgType::Bool, "Edit.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "skinLayersTool",
            "Skin Tools' tools on a layer (-layer, else the current one): -flood with -target, -mode replace, add, scale, smooth \
             or sharpen, -intensity, -iterations; -assignFromClosestJoint (-influences); -unify; -mirror (-mirrorWeights, \
             -mirrorMask, -mirrorDq); -stamp x y z, one dab of the brush. -sc limits a tool to the selected points. \
             -exportFile and -importFile write and read the layers (-vertexTransferMode closestPoint or vertexId); \
             -transferFrom another skinned mesh copies its layers. -mode, -intensity, -iterations, -distribute and \
             -brushMirror with no mesh set the brush; -q gives them.",
            skin_layers_tool_cmd,
        )
        .positional("a skinned mesh or a skinCluster")
        .flag(fl("layer", "l", ArgType::Any, "The layer; the current one if not given."))
        .flag(fl("target", "t", ArgType::Any, "An influence, mask or dq; the layer's paint target if not given."))
        .flag(fl("flood", "fl", ArgType::Bool, "Apply the mode to every point (or the selected ones)."))
        .flag(fl("mode", "m", ArgType::Any, "replace, add, scale, smooth or sharpen."))
        .flag(fl("intensity", "int", ArgType::Float, "The value set, added or scaled by, or how strongly smooth and sharpen work."))
        .flag(fl("iterations", "it", ArgType::Int, "How many times smooth and sharpen run."))
        .flag(fl("distribute", "dis", ArgType::Bool, "Weight taken from an influence goes to the layer's others."))
        .flag(fl("brushMirror", "bm", ArgType::Bool, "Mirror what a flood or a stroke changed."))
        .flag(fl("selectedComponents", "sc", ArgType::Bool, "Only the selected points."))
        .flag(fl("assignFromClosestJoint", "acj", ArgType::Bool, "Each point fully to the influence bound nearest it."))
        .flag(fl("influences", "inf", ArgType::Any, "The influences -assignFromClosestJoint picks from.").multi(1))
        .flag(fl("unify", "u", ArgType::Bool, "The points take the average of their weights."))
        .flag(fl("mirror", "mi", ArgType::Bool, "Mirror the layer across the skin's mirror axis."))
        .flag(fl("mirrorWeights", "mw", ArgType::Bool, "With -mirror: the weights."))
        .flag(fl("mirrorMask", "mm", ArgType::Bool, "With -mirror: the mask."))
        .flag(fl("mirrorDq", "mdq", ArgType::Bool, "With -mirror: the dual quaternion weights."))
        .flag(fl("stamp", "st", ArgType::Vec3, "One dab of the brush at this world point."))
        .flag(fl("invert", "inv", ArgType::Bool, "With -stamp: the inverse, as Ctrl does."))
        .flag(fl("smooth", "sm", ArgType::Bool, "With -stamp: smooth, as Shift does."))
        .flag(fl("exportFile", "ef", ArgType::String, "Write the layers to a file."))
        .flag(fl("importFile", "if", ArgType::String, "Read layers from a file onto the skin, replacing its own."))
        .flag(fl("transferFrom", "tf", ArgType::String, "Copy another skinned mesh's layers onto this one."))
        .flag(fl("vertexTransferMode", "vtm", ArgType::String, "How points match: closestPoint (by bind place) or vertexId."))
        .flag(fl("query", "q", ArgType::Bool, "Query the brush.")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layers(points: usize) -> Layers {
        Layers { points, ..Default::default() }
    }

    #[test]
    fn a_layer_covers_by_opacity_and_mask_and_leaves_the_rest() {
        let mut l = layers(3);
        let base = l.add("base", None);
        for p in 0..3 {
            l.layer_mut(base).unwrap().set_at(p, &[(0, 1.0)], 3);
        }
        let top = l.add("top", None);
        {
            let t = l.layer_mut(top).unwrap();
            t.set_at(0, &[(1, 1.0)], 3);
            t.set_at(1, &[(1, 0.5)], 3);
            t.mask = Some(vec![1.0, 1.0, 0.0]);
        }
        let (w, _) = l.compose(&[]);
        assert_eq!(w[0], vec![(1, 1.0)]);
        // Half from the top layer, the rest from below.
        assert!((get(&w[1], 0) - 0.5).abs() < 1e-9 && (get(&w[1], 1) - 0.5).abs() < 1e-9);
        assert_eq!(w[2], vec![(0, 1.0)]);
        l.layer_mut(top).unwrap().opacity = 0.25;
        let (w, _) = l.compose(&[]);
        assert!((get(&w[0], 1) - 0.25).abs() < 1e-9);
        l.layer_mut(top).unwrap().enabled = false;
        assert_eq!(l.compose(&[]).0[0], vec![(0, 1.0)]);
    }

    #[test]
    fn a_group_masks_its_children() {
        let mut l = layers(2);
        let base = l.add("base", None);
        l.layer_mut(base).unwrap().set_at(0, &[(0, 1.0)], 2);
        l.layer_mut(base).unwrap().set_at(1, &[(0, 1.0)], 2);
        let g = l.add("group", None);
        let c = l.add("child", Some(g));
        l.layer_mut(c).unwrap().set_at(0, &[(2, 1.0)], 2);
        l.layer_mut(c).unwrap().set_at(1, &[(2, 1.0)], 2);
        l.layer_mut(g).unwrap().mask = Some(vec![0.0, 1.0]);
        let (w, _) = l.compose(&[]);
        assert_eq!(w[0], vec![(0, 1.0)]);
        assert_eq!(w[1], vec![(2, 1.0)]);
        assert!(l.set_parent(g, Some(c)).is_err(), "a group can't go inside its child");
    }

    #[test]
    fn replace_takes_from_the_others_and_locks_hold() {
        let mut x = Layer::new(0, "l", None);
        x.set_at(0, &[(0, 0.5), (1, 0.5)], 1);
        edit_point(&mut x, Target::Influence(2), 0, 1, Mode::Replace, 0.5, 1.0, true);
        let w = x.at(0);
        assert!((get(&w, 2) - 0.5).abs() < 1e-9 && (get(&w, 0) - 0.25).abs() < 1e-9);
        x.locked = vec![0];
        edit_point(&mut x, Target::Influence(2), 0, 1, Mode::Replace, 1.0, 1.0, true);
        let w = x.at(0);
        assert!((get(&w, 0) - 0.25).abs() < 1e-9, "locked stays");
        assert!((get(&w, 2) - 0.75).abs() < 1e-9 && get(&w, 1) < 1e-9);
    }

    #[test]
    fn side_names_swap() {
        assert_eq!(other_side("arm_L0_0_jnt").as_deref(), Some("arm_R0_0_jnt"));
        assert_eq!(other_side("L_hand").as_deref(), Some("R_hand"));
        assert_eq!(other_side("spine_C0_jnt"), None);
    }

    #[test]
    fn mirror_points_find_the_other_side() {
        let at = vec![DVec3::new(1.0, 0.0, 0.0), DVec3::new(-1.0, 0.0, 0.0), DVec3::new(0.0, 1.0, 0.0)];
        assert_eq!(mirror_points(&at, 0), vec![1, 0, 2]);
    }
}
