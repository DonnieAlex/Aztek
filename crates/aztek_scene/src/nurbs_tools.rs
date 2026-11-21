//! arcLengthDimension (the arc length of a curve or surface up to a point, as a live dimension shape), arubaNurbsToPoly
//! (a surface tessellated to polygons through an arubaTessellate node), and -q, -e, -caching and -nodeState for the NURBS
//! commands that make history nodes.

use std::any::Any;
use std::sync::Arc;

use aztek_geo::nurbs::{NurbsCurve, NurbsSurface};
use aztek_ops::{fail, objects, ArgType, Args, CmdResult, Commands};
use serde_json::{json, Value};

use crate::graph::{Graph, SelectMode};
use crate::history_qe::{apply_common, f, node_flags, node_state, qe, qe_flags, Cmd};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::rig::plug;
use crate::session::Session;
use crate::transform::{DagNode, Shape, Transform};

// ---------------------------------------------------------------------------------------------------------------------
// paramLocator

/// paramLocator: a locator held at a parameter of a curve (curve.u[0.5], curve.ep[1], curve.un[0.25] normalized) or of a
/// surface (surface.uv[0.5][0.5], surface.uvn[..][..] normalized). The locator is parented under the object, its
/// translate fed by a pointOnCurveInfo or pointOnSurfaceInfo on the shape's local geometry, so it rides the shape
/// as the reference's locator in the shape's underworld does. -p: the parameters are normalized to 0 to 1.
fn param_locator(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let Some(item) = names.first().cloned() else { fail!("paramLocator: name a point on a curve or surface") };
    let g = &mut s.graph;
    let Some((head, rest)) = item.split_once('[') else { fail!("paramLocator: name a point: curve.u[0.5], curve.ep[1] or surface.uv[0.5][0.5]") };
    let Some((obj, kind)) = head.rsplit_once('.') else { fail!("paramLocator: name a point: curve.u[0.5], curve.ep[1] or surface.uv[0.5][0.5]") };
    let nums: Vec<f64> = rest.split(['[', ']']).filter(|x| !x.trim().is_empty()).filter_map(|x| x.trim().parse().ok()).collect();
    let normalized = a.bool("position") || kind.ends_with('n');
    let id = crate::rig::find(g, obj, "paramLocator")?;
    let tr = crate::dag::transform_of(g, id);
    g.open_chunk("paramLocator");
    let r = (|| -> CmdResult {
        let (lt, _) = crate::dag::create_with_shape(g, "locator", "locator1", tr)?;
        if let Some(shape) = crate::curves::as_curve(g, id) {
            let Some(c) = crate::curves::local_curve(g, shape) else { fail!("paramLocator: {obj} has no curve") };
            let (mut u, mut top) = (nums.first().copied().unwrap_or(0.0), normalized);
            if kind == "ep" {
                let i = u as usize;
                let Some(&p) = c.distinct_params().get(i) else { fail!("paramLocator: {obj} has no edit point {i}") };
                u = p;
                top = false;
            } else if !matches!(kind, "u" | "un") {
                fail!("paramLocator: a curve's point is .u[], .un[] or .ep[]");
            }
            let t = types::find("pointOnCurveInfo").expect("pointOnCurveInfo");
            let info = g.create_node(&t, "pointOnCurveInfo1", NO_NODE, true)?;
            g.connect(&plug(shape, crate::curves::CurveShape::LOCAL), &plug(info, 0), false)?;
            g.set_value(plug(info, 1), PlugValue::Float(u))?;
            g.set_value(plug(info, 2), PlugValue::Bool(top))?;
            g.connect(&plug(info, 3), &plug(lt, Transform::TRANSLATE), false)?;
        } else if let Some(shape) = crate::surfaces::as_surface(g, id) {
            if !matches!(kind, "uv" | "uvn") || nums.len() < 2 {
                fail!("paramLocator: a surface's point is .uv[u][v]");
            }
            let t = types::find("pointOnSurfaceInfo").expect("pointOnSurfaceInfo");
            let info = g.create_node(&t, "pointOnSurfaceInfo1", NO_NODE, true)?;
            g.connect(&plug(shape, crate::surfaces::SurfaceShape::LOCAL), &plug(info, 0), false)?;
            g.set_value(plug(info, 1), PlugValue::Float(nums[0]))?;
            g.set_value(plug(info, 2), PlugValue::Float(nums[1]))?;
            g.set_value(plug(info, 3), PlugValue::Bool(normalized))?;
            let pos = g.node(info).unwrap().ty().attr_index("position").unwrap();
            g.connect(&plug(info, pos), &plug(lt, Transform::TRANSLATE), false)?;
        } else {
            fail!("paramLocator: {obj} is not a NURBS curve or surface");
        }
        g.evaluate();
        g.select(&[lt], SelectMode::Replace);
        Ok(json!(g.path(lt)))
    })();
    g.close_chunk();
    r
}

// ---------------------------------------------------------------------------------------------------------------------
// arcLengthDimension

const AL_U: usize = Shape::COUNT;
const AL_V: usize = Shape::COUNT + 1;
const AL_LENGTH: usize = Shape::COUNT + 2;
const AL_LENGTH_V: usize = Shape::COUNT + 3;
const AL_GEOMETRY: usize = Shape::COUNT + 4;

/// A curve's length along it from its start to u.
pub(crate) fn length_to(c: &NurbsCurve, u: f64) -> f64 {
    let (lo, hi) = c.domain();
    let u = u.clamp(lo, hi);
    if u <= lo {
        return 0.0;
    }
    let steps = (c.spans().max(1) * 64).min(4096);
    let mut total = 0.0;
    let mut prev = c.eval(lo);
    for i in 1..=steps {
        let p = c.eval(lo + (u - lo) * i as f64 / steps as f64);
        total += (p - prev).length();
        prev = p;
    }
    total
}

struct ArcLengthDim;

impl NodeBehavior for ArcLengthDim {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
        let (u, v) = (d.float(AL_U), d.float(AL_V));
        let geo = d.value(AL_GEOMETRY).clone();
        if let Some(c) = crate::curves::curve_value(&geo) {
            d.set_output(AL_LENGTH, PlugValue::Float(length_to(c, u)));
            d.set_output(AL_LENGTH_V, PlugValue::Float(0.0));
        } else if let Some(s) = crate::surfaces::surface_value(&geo) {
            // Along U at v, and along V at u.
            d.set_output(AL_LENGTH, PlugValue::Float(length_to(&s.iso_v(v), u)));
            d.set_output(AL_LENGTH_V, PlugValue::Float(length_to(&s.iso_u(u), v)));
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn attr(name: &str, short: &str, ty: PlugType, def: PlugValue, fl: u32, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: def, flags: fl, doc: doc.into(), ..Default::default() }
}

/// curve.u[0.5], surface.uv[0.5][0.5], surface.u[0.5] or curve.ep[2]: the shape and the parameters.
fn point_arg(g: &mut Graph, text: &str) -> Option<(NodeId, f64, f64)> {
    let (head, rest) = text.split_once('[')?;
    let (obj, kind) = head.rsplit_once('.')?;
    let id = g.find(obj).ok()??;
    let inner = rest.trim_end_matches(']');
    let (a, b) = inner.split_once("][").unwrap_or((inner, "0"));
    let (x, y): (f64, f64) = (a.parse().ok()?, b.parse().ok()?);
    if let Some(c) = crate::curves::as_curve(g, id) {
        let u = if kind == "ep" { crate::curves::local_curve(g, c)?.distinct_params().get(x as usize).copied()? } else { x };
        return Some((c, u, 0.0));
    }
    let s = crate::surfaces::as_surface(g, id)?;
    if kind == "v" {
        // A V isoparm: its point at the start of U.
        let lo = crate::surfaces::local_surface(g, s)?.domain_u().0;
        return Some((s, lo, x));
    }
    Some((s, x, y))
}

fn arc_length_dimension(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    let Some(text) = names.first() else { fail!("arcLengthDimension: name a point: curve.u[0.5] or surface.uv[0.5][0.5]") };
    let Some((shape, u, v)) = point_arg(g, text) else { fail!("arcLengthDimension: {text} is not a point on a curve or surface") };
    g.evaluate();
    let is_curve = g.node(shape).is_some_and(|n| n.ty().is_a(crate::curves::SHAPE_TYPE));
    let at = if is_curve { crate::curves::world_curve(g, shape).map(|c| c.eval(u)) } else { crate::surfaces::world_surface(g, shape).map(|x| x.eval(u, v)) };
    let Some(at) = at else { fail!("arcLengthDimension: {text} has no geometry") };
    g.open_chunk("arcLengthDimension");
    let r = (|| -> CmdResult<NodeId> {
        let (tr, sh) = crate::dag::create_with_shape(g, "arcLengthDimension", "arcLengthDimension1", NO_NODE)?;
        g.set_value(plug(tr, Transform::TRANSLATE), PlugValue::Vec3(at))?;
        g.set_value(plug(sh, AL_U), PlugValue::Float(u))?;
        g.set_value(plug(sh, AL_V), PlugValue::Float(v))?;
        let from = if is_curve { crate::curves::CurveShape::WORLD_SPACE } else { crate::surfaces::SurfaceShape::WORLD_SPACE };
        g.connect(&plug(shape, from), &plug(sh, AL_GEOMETRY), false)?;
        Ok(sh)
    })();
    g.close_chunk();
    let sh = r?;
    g.evaluate();
    Ok(json!(g.path(sh)))
}

/// The arc length a dimension shape shows, with where. The viewport calls this.
pub fn dimension_label(n: &crate::node::Node) -> Option<(String, glam::DVec3)> {
    if n.ty().name != "arcLengthDimension" {
        return None;
    }
    let d = n.data();
    let at = d.matrix(DagNode::WORLD_MATRIX).w_axis.truncate();
    let surface = crate::surfaces::surface_value(d.value(AL_GEOMETRY)).is_some();
    let text = if surface { format!("{:.3}, {:.3}", d.float(AL_LENGTH), d.float(AL_LENGTH_V)) } else { format!("{:.3}", d.float(AL_LENGTH)) };
    Some((text, at))
}

// ---------------------------------------------------------------------------------------------------------------------
// arubaNurbsToPoly

const T_IN: usize = 0;
const T_OUT: usize = 1;

fn aruba_options() -> crate::surfaces::TessOptions {
    crate::surfaces::TessOptions { format: 1, quads: false, count: 200, tolerance: 0.01, u_type: 3, u_number: 3, v_type: 3, v_number: 3 }
}

struct ArubaNode;

impl NodeBehavior for ArubaNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let state = node_state(d);
        if state == 2 || state == 5 {
            return;
        }
        let out = crate::surfaces::surface_value(d.value(T_IN)).map(|s: &NurbsSurface| crate::surfaces::tessellate(s, aruba_options()));
        d.set_output(T_OUT, PlugValue::Mesh(out.map(Arc::new)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn aruba_to_poly(s: &mut Session, a: &Args) -> CmdResult {
    if let Some(r) = qe(s, a, "arubaNurbsToPoly", &["arubaTessellate"], &[]) {
        return r;
    }
    let names = objects(s, a);
    let Some(name) = names.first() else { fail!("arubaNurbsToPoly: name or select a NURBS surface") };
    let g = &mut s.graph;
    let surf = crate::surfaces::surface_arg(g, name, "arubaNurbsToPoly")?;
    let world_space = a.bool("worldSpace") && !a.bool("localSpace");
    let ch = a.get("constructionHistory").and_then(Value::as_bool).unwrap_or(g.construction_history);
    let object = a.get("object").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64() != Some(0)));
    g.evaluate();
    g.open_chunk("arubaNurbsToPoly");
    let r = (|| -> CmdResult<(Option<NodeId>, NodeId)> {
        let t = types::find("arubaTessellate").expect("arubaTessellate");
        let node = g.create_node(&t, "arubaTessellate1", NO_NODE, true)?;
        let from = if world_space { crate::surfaces::SurfaceShape::WORLD_SPACE } else { crate::surfaces::SurfaceShape::LOCAL };
        g.connect(&plug(surf, from), &plug(node, T_IN), false)?;
        apply_common(g, node, a)?;
        if !object {
            return Ok((None, node));
        }
        let (tr, sh) = crate::mesh_nodes::create_mesh_node(g, &a.string("name", "polySurface1"))?;
        g.connect(&plug(node, T_OUT), &plug(sh, crate::mesh_nodes::MeshNode::IN_MESH), true)?;
        if !world_space {
            // In local space the polygons sit where the surface does.
            let src = crate::dag::transform_of(g, surf);
            let w = crate::rig::world(g, src);
            crate::transform::Transform::set_local_matrix(g, tr, &w).map_err(|e| aztek_ops::CommandError::new(e.to_string()))?;
        }
        g.select(&[tr], SelectMode::Replace);
        Ok((Some(sh), node))
    })();
    g.close_chunk();
    let (sh, node) = r?;
    g.evaluate();
    let Some(sh) = sh else { return Ok(json!([g.path(node)])) };
    let tr = crate::dag::transform_of(g, sh);
    if !ch {
        crate::history::delete_history(g, sh)?;
        if g.node(node).is_some() {
            g.delete_node(node)?;
        }
        return Ok(json!([g.path(tr)]));
    }
    Ok(json!([g.path(tr), g.path(node)]))
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let shape = types::find("shape").expect("shape");
        let store = flags::INPUT | flags::STORABLE;
        for t in [
            NodeType {
                name: "arcLengthDimension".into(),
                base: Some(shape),
                dag: true,
                icon: "ruler".into(),
                doc: "The length along a curve or surface from its start to a point, shown where the point is.".into(),
                own_attrs: vec![
                    attr("uParamValue", "upv", PlugType::Float, PlugValue::Float(0.0), store | flags::KEYABLE, "The point's U parameter."),
                    attr("vParamValue", "vpv", PlugType::Float, PlugValue::Float(0.0), store | flags::KEYABLE, "The point's V parameter, on a surface."),
                    attr("arcLength", "al", PlugType::Float, PlugValue::Float(0.0), flags::OUTPUT, "The length along U up to the point."),
                    attr("arcLengthInV", "alv", PlugType::Float, PlugValue::Float(0.0), flags::OUTPUT, "On a surface, the length along V up to the point."),
                    crate::curves::curve_attr("nurbsGeometry", "ng", flags::INPUT | flags::HIDDEN, "The curve or surface measured, world space."),
                ],
                create: Some(Arc::new(|| Box::new(ArcLengthDim))),
                ..Default::default()
            },
            NodeType {
                name: "arubaTessellate".into(),
                icon: "chart-line".into(),
                doc: "A NURBS surface as triangles.".into(),
                own_attrs: vec![
                    crate::surfaces::surface_attr("inputSurface", "is", flags::INPUT | flags::HIDDEN, "The surface."),
                    crate::attr_defs::mesh_attr("outputPolygon", "op", flags::OUTPUT | flags::HIDDEN, "The polygons."),
                ],
                create: Some(Arc::new(|| Box::new(ArubaNode))),
                ..Default::default()
            },
        ] {
            let name = t.name.clone();
            types::register_type(t).expect(&name);
        }
    });
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    crate::nurbs_align::register(c);
    c.register(
        Cmd::new(
            "arcLengthDimension",
            "Measure the arc length of a curve or surface up to a point: arcLengthDimension curveShape1.u[0.5], or surface.uv[0.5][0.5] for the \
             lengths along U and V. Makes a live arcLengthDimension shape there; returns it.",
            arc_length_dimension,
        )
        .positional("a point on a curve or surface"),
    );
    c.register(node_flags(qe_flags(
        Cmd::new(
            "arubaNurbsToPoly",
            "Tessellate a NURBS surface to polygons through an arubaTessellate node. -ls (the default) in the surface's space, -ws in world \
             space. Returns the polygon object, and the node with history.",
            aruba_to_poly,
        )
        .positional("a NURBS surface (the selection when none)")
        .flag(f("localSpace", "ls", ArgType::Bool, "Tessellate in local space (the default)."))
        .flag(f("worldSpace", "ws", ArgType::Bool, "Tessellate in world space."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Keep the node, so the tessellation follows the surface (on by default)."))
        .flag(f("object", "o", ArgType::Bool, "Make the polygons (on); off, only the node."))
        .flag(f("name", "n", ArgType::String, "Name of the polygon object.")),
    )));
    c.register(
        Cmd::new(
            "paramLocator",
            "A locator held at a parameter of a NURBS curve (curve.u[0.5], curve.ep[1], curve.un[0.25]) or surface (surface.uv[0.5][0.5]), \
             following the shape as it changes. -p: the parameters are normalized, 0 to 1. Returns the locator.",
            param_locator,
        )
        .positional("a point on a curve or surface (the selection when none)")
        .flag(f("position", "p", ArgType::Bool, "The parameters are in normalized space, 0 to 1 over the range.")),
    );
    // The NURBS commands whose settings live on a history node: -q and -e reach the node.
    crate::history_qe::extend(c, "attachCurve", &["attachCurve"], &[], Vec::new());
    crate::history_qe::extend(c, "attachSurface", &["attachSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "boundary", &["boundary"], &[], Vec::new());
    crate::history_qe::extend(c, "bevel", &["bevel"], &[], Vec::new());
    crate::history_qe::extend(c, "bevelPlus", &["bevelPlus"], &[], Vec::new());
    crate::history_qe::extend(c, "blend2", &["ffBlendSrf"], &[], Vec::new());
    crate::history_qe::extend(c, "closeCurve", &["closeCurve"], &[], Vec::new());
    crate::history_qe::extend(c, "closeSurface", &["closeSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "detachCurve", &["detachCurve"], &[], vec![f("curveOnSurface", "cos", ArgType::Bool, "A curve on a surface gives pieces on the surface where it can; other curves give 3D pieces.")]);
    crate::history_qe::extend(c, "detachSurface", &["detachSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "cone", &["makeNurbCone"], &[], Vec::new());
    crate::history_qe::extend(c, "cylinder", &["makeNurbCylinder"], &[], Vec::new());
    crate::history_qe::extend(c, "circularFillet", &["rbfSrf"], &[], Vec::new());
    crate::history_qe::extend(c, "duplicateCurve", &["curveFromSurfaceIso"], &[("relative", "relativeValue")], Vec::new());
    crate::history_qe::extend(c, "duplicateSurface", &["subSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "extendCurve", &["extendCurve"], &[], Vec::new());
    crate::history_qe::extend(c, "extendSurface", &["extendSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "extrude", &["extrude"], &[], Vec::new());
    crate::history_qe::extend(c, "filletCurve", &["filletCurve"], &[], Vec::new());
    crate::history_qe::extend(c, "fitBspline", &["fitBspline"], &[], Vec::new());
    crate::history_qe::extend(c, "freeFormFillet", &["ffFilletSrf"], &[], Vec::new());
    crate::history_qe::extend(c, "hardenPointCurve", &["hardenPointCurve"], &[], Vec::new());
    crate::history_qe::extend(
        c,
        "insertKnotCurve",
        &["insertKnotCurve"],
        &[],
        vec![f("curveOnSurface", "cos", ArgType::Bool, "Make a curve on a surface where the input is one; curves here are 3D, so the result is too.")],
    );
    crate::history_qe::extend(c, "insertKnotSurface", &["insertKnotSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "intersect", &["intersectSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "loft", &["loft"], &[], Vec::new());
    crate::history_qe::extend(c, "nurbsCube", &["makeNurbCube"], &[], Vec::new());
    crate::history_qe::extend(c, "nurbsPlane", &["makeNurbPlane"], &[], Vec::new());
    crate::history_qe::extend(c, "offsetSurface", &["offsetSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "offsetCurve", &["offsetCurve"], &[], Vec::new());
    crate::history_qe::extend(c, "offsetCurveOnSurface", &["offsetCurveOnSurface"], &[], Vec::new());
    crate::history_qe::extend(c, "pointCurveConstraint", &["pointCurveConstraint"], &[], Vec::new());
}
