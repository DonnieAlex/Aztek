//! Strands for the renderer: NURBS curves with Render Curve on, and whatever registers itself as a source of strands
//! (hair descriptions, through their plug-in). Each becomes a set of curves Harold renders as ribbons or tubes.

use std::sync::Mutex;

use glam::{DMat4, Vec2, Vec3};

use crate::graph::Graph;
use crate::plug::NodeId;

/// Strands a shape gives the renderer, in its object space.
#[derive(Clone, Debug, Default)]
pub struct Strands {
    pub shape: NodeId,
    pub points: Vec<Vec3>,
    /// Radius at each point.
    pub radius: Vec<f32>,
    /// Points per strand, in order.
    pub counts: Vec<u32>,
    /// Per strand, where its root is on the surface it grows from; may be empty.
    pub root_uv: Vec<Vec2>,
    pub thick: bool,
    pub world: DMat4,
    /// A material node to shade with; None takes the shape's shading group, as a mesh does.
    pub shader: Option<NodeId>,
    /// The colour the views draw it in.
    pub color: [f32; 3],
}

/// A source of strands: given the graph, the strands to render at the current time.
pub type Source = fn(&mut Graph) -> Vec<Strands>;

fn sources() -> &'static Mutex<Vec<(&'static str, Source)>> {
    static S: Mutex<Vec<(&'static str, Source)>> = Mutex::new(Vec::new());
    &S
}

/// Registers a source of strands by name; a second registration under the same name replaces the first.
pub fn register(name: &'static str, f: Source) {
    let mut s = sources().lock().unwrap();
    s.retain(|(n, _)| *n != name);
    s.push((name, f));
}

pub fn unregister(name: &str) {
    sources().lock().unwrap().retain(|(n, _)| *n != name);
    view_sources().lock().unwrap().retain(|(n, _)| *n != name);
}

/// A source of strands for the views: the graph as last evaluated, not evaluated again.
pub type ViewSource = fn(&Graph) -> Vec<Strands>;

fn view_sources() -> &'static Mutex<Vec<(&'static str, ViewSource)>> {
    static S: Mutex<Vec<(&'static str, ViewSource)>> = Mutex::new(Vec::new());
    &S
}

pub fn register_view(name: &'static str, f: ViewSource) {
    let mut s = view_sources().lock().unwrap();
    s.retain(|(n, _)| *n != name);
    s.push((name, f));
}

/// Strands the views draw: the registered view sources'. Curves marked to render are drawn as curves, as the
/// reference's views draw them.
pub fn view_gather(g: &Graph) -> Vec<Strands> {
    let list: Vec<ViewSource> = view_sources().lock().unwrap().iter().map(|(_, f)| *f).collect();
    list.into_iter().flat_map(|f| f(g)).collect()
}

/// Every strand to render now: NURBS curves marked to render, then the registered sources'.
pub fn gather(g: &mut Graph) -> Vec<Strands> {
    let mut out = render_curves(g);
    let list: Vec<Source> = sources().lock().unwrap().iter().map(|(_, f)| *f).collect();
    for f in list {
        out.extend(f(g));
    }
    out
}

/// NURBS curves with Render Curve on: sampled by Sample Rate a span, Curve Width across (a diameter, as
/// the reference's renderer has it), shaded by what Curve Shader connects.
fn render_curves(g: &mut Graph) -> Vec<Strands> {
    g.evaluate();
    let ids: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().is_a(crate::curves::SHAPE_TYPE))).collect();
    let mut out = Vec::new();
    for id in ids {
        if !super::network::get_b(g, id, "aiRenderCurve", false) || !super::visible(g, id) {
            continue;
        }
        let Some(curve) = crate::curves::local_curve(g, id) else { continue };
        let rate = super::network::get_f(g, id, "aiSampleRate", 5.0).clamp(1.0, 64.0) as usize;
        let width = super::network::get_f(g, id, "aiCurveWidth", 0.01).max(0.0) as f32;
        let pts: Vec<Vec3> = curve.tessellate(rate).into_iter().map(|p| p.as_vec3()).collect();
        if pts.len() < 2 {
            continue;
        }
        let world = g.node(id).map(|n| n.data().matrix(crate::transform::DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY);
        let shader = super::network::attr(g, id, "aiCurveShader").and_then(|p| g.source(&p)).map(|s| s.node);
        let n = pts.len();
        out.push(Strands { shape: id, radius: vec![width * 0.5; n], counts: vec![n as u32], points: pts, root_uv: Vec::new(), thick: super::network::get_f(g, id, "aiMode", 0.0) as i64 == 1, world, shader, color: [0.6, 0.6, 0.6] });
    }
    out
}
