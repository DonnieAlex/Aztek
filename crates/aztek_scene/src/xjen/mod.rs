//! XJEN's Interactive Groom: spline hair as nodes in the scene. `xgmSplineBase` grows splines on a bound mesh,
//! modifiers (`xgmModifierScale`, `xgmModifierSculpt`) change them in a stack, and the `xgmSplineDescription` shape
//! gives them width and hands them to the views and to Harold. Node type names are the reference's, so its scenes
//! load; attribute names follow its documented labels until a capture (docs/XJEN.md) gives the file names.
//!
//! Roots are bound by face, triangle and weights, drawn from the seed and face alone, so hair stays where it grew as
//! the mesh deforms or is skinned.

pub mod cmds;
pub mod sculpt;

use std::any::Any;
use std::sync::Arc;

use aztek_geo::Mesh;
use glam::{DMat4, DVec2, DVec3, Vec2, Vec3};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::harold::strands::{self, Strands};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue};
use crate::transform::{DagNode, Shape};

pub const BASE: &str = "xgmSplineBase";
pub const DESCRIPTION: &str = "xgmSplineDescription";
pub const SCALE: &str = "xgmModifierScale";
pub const SCULPT: &str = "xgmModifierSculpt";

const IN: u32 = flags::INPUT | flags::STORABLE | flags::KEYABLE;
const STORE: u32 = flags::INPUT | flags::STORABLE;

// ---------------------------------------------------------------------------------------------------------------------
// Spline data

/// One spline: where its root is on the bound mesh, its frame there, and its points and widths.
#[derive(Clone, Debug)]
pub struct Spline {
    /// Stable across evaluations: the face in the high half, the spline's number on it in the low.
    pub id: u64,
    pub face: u32,
    /// The triangle's face-vertices and the root's weights on them.
    pub corners: [u32; 3],
    pub w: [f64; 3],
    pub uv: [f32; 2],
    pub normal: DVec3,
    pub tangent: DVec3,
    /// Root first.
    pub points: Vec<DVec3>,
    /// Width at each point; empty until the description sets it.
    pub width: Vec<f32>,
}

impl Spline {
    pub fn root(&self) -> DVec3 {
        self.points.first().copied().unwrap_or(DVec3::ZERO)
    }
    pub fn bitangent(&self) -> DVec3 {
        self.normal.cross(self.tangent)
    }
    pub fn length(&self) -> f64 {
        self.points.windows(2).map(|w| w[0].distance(w[1])).sum()
    }
}

/// Splines passed along the stack, base to description.
#[derive(Clone, Debug, Default)]
pub struct SplineData {
    pub splines: Vec<Spline>,
}

impl crate::plug::Data for SplineData {
    fn kind(&self) -> &str {
        "xgmSplineData"
    }
    fn to_json(&self) -> Value {
        json!({"kind": "xgmSplineData", "count": self.splines.len()})
    }
    fn bytes(&self) -> usize {
        self.splines.iter().map(|s| 128 + s.points.len() * 28).sum()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn spline_data(v: &PlugValue) -> Option<&SplineData> {
    match v {
        PlugValue::Data(Some(d)) => d.as_any().downcast_ref::<SplineData>(),
        _ => None,
    }
}

fn data_value(s: SplineData) -> PlugValue {
    PlugValue::Data(Some(Arc::new(s)))
}

// ---------------------------------------------------------------------------------------------------------------------
// Generation

/// SplitMix64: small, fast and the same everywhere, so a seed grows the same hair on every machine.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed ^ 0x9E37_79B9_7F4A_7C15)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// In [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// A number in [0, 1) that depends only on an id: for thinning splines the same way every time.
pub fn hash01(id: u64) -> f64 {
    Rng::new(id.wrapping_mul(0xD6E8_FEB8_6659_FD93)).next_f64()
}

/// What `generate` grows.
#[derive(Clone, Debug)]
pub struct GrowParams {
    /// Splines per face.
    pub density: f64,
    pub seed: u64,
    pub length: f64,
    pub cv_count: usize,
    /// The faces to grow on; empty grows on all.
    pub faces: Vec<u32>,
}

fn uv_at(mesh: &Mesh, c: u32) -> Option<DVec2> {
    let uv = mesh.corner_attrs().get::<[f32; 2]>(aztek_geo::names::uv(0).as_str());
    uv.get(c as usize).map(|u| DVec2::new(u[0] as f64, u[1] as f64))
}

/// Splines on a mesh: on each face, density of them (the fraction as a chance), placed in its triangles by their UV
/// area (by count where there are no UVs, so placing never depends on how the mesh is deformed), straight along the
/// surface normal for length, with cv_count points.
pub fn generate(mesh: &Mesh, p: &GrowParams) -> SplineData {
    let normals = aztek_geo::normals::point_normals(mesh, aztek_geo::normals::NormalWeight::Angle);
    let cv = p.cv_count.max(2);
    let mut out = Vec::new();
    let only: std::collections::HashSet<u32> = p.faces.iter().copied().collect();
    for f in 0..mesh.face_capacity() {
        if !mesh.face_valid(f) || (!only.is_empty() && !only.contains(&(f as u32))) {
            continue;
        }
        let c = mesh.face_corners(f);
        if c.len() < 3 {
            continue;
        }
        let tris: Vec<[u32; 3]> = (1..c.len() - 1).map(|k| [c[0], c[k], c[k + 1]]).collect();
        let weights: Vec<f64> = tris
            .iter()
            .map(|t| match (uv_at(mesh, t[0]), uv_at(mesh, t[1]), uv_at(mesh, t[2])) {
                (Some(a), Some(b), Some(cc)) => ((b - a).perp_dot(cc - a) * 0.5).abs(),
                _ => 0.0,
            })
            .collect();
        let total: f64 = weights.iter().sum();
        let weights: Vec<f64> = if total > 1e-14 { weights } else { vec![1.0; tris.len()] };
        let total: f64 = weights.iter().sum();
        let mut rng = Rng::new(p.seed.wrapping_mul(0x1000_0000_01B3) ^ (f as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
        let want = p.density.max(0.0);
        let mut count = want.floor() as usize;
        if rng.next_f64() < want.fract() {
            count += 1;
        }
        for k in 0..count {
            let mut pick = rng.next_f64() * total;
            let mut ti = 0;
            while ti + 1 < tris.len() && pick >= weights[ti] {
                pick -= weights[ti];
                ti += 1;
            }
            let t = tris[ti];
            let (r1, r2) = (rng.next_f64().sqrt(), rng.next_f64());
            let w = [1.0 - r1, r1 * (1.0 - r2), r1 * r2];
            out.push(place(mesh, &normals, f as u32, (f as u64) << 32 | k as u64, t, w, p.length, cv));
        }
    }
    SplineData { splines: out }
}

/// A straight spline at a root.
fn place(mesh: &Mesh, normals: &[Vec3], face: u32, id: u64, t: [u32; 3], w: [f64; 3], length: f64, cv: usize) -> Spline {
    let (pos, n, tan) = frame(mesh, normals, t, w);
    let uv = t.iter().zip(w).fold(DVec2::ZERO, |a, (&c, wk)| a + uv_at(mesh, c).unwrap_or_default() * wk);
    let points = (0..cv).map(|j| pos + n * (length * j as f64 / (cv - 1) as f64)).collect();
    Spline { id, face, corners: t, w, uv: [uv.x as f32, uv.y as f32], normal: n, tangent: tan, points, width: Vec::new() }
}

/// Position, normal (point normals blended) and a tangent along the triangle's first edge, at a root.
pub fn frame(mesh: &Mesh, normals: &[Vec3], t: [u32; 3], w: [f64; 3]) -> (DVec3, DVec3, DVec3) {
    let pts = t.map(|c| mesh.corner_point(c));
    let p = pts.map(|k| mesh.position(k));
    let pos = p[0] * w[0] + p[1] * w[1] + p[2] * w[2];
    let face_n = (p[1] - p[0]).cross(p[2] - p[0]).normalize_or_zero();
    let mut n = DVec3::ZERO;
    for (k, &i) in pts.iter().enumerate() {
        if let Some(v) = normals.get(i as usize) {
            n += v.as_dvec3() * w[k];
        }
    }
    let n = if n.length_squared() > 1e-20 { n.normalize() } else { face_n };
    let e = p[1] - p[0];
    let mut tan = (e - n * e.dot(n)).normalize_or_zero();
    if tan == DVec3::ZERO {
        tan = n.any_orthonormal_vector();
    }
    (pos, n, tan)
}

// ---------------------------------------------------------------------------------------------------------------------
// Attributes

fn base(name: &str, short: &str, ty: PlugType, default: PlugValue, fl: u32) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: default, flags: fl, ..Default::default() }
}
fn doc(mut a: AttrDef, d: &str) -> AttrDef {
    a.doc = d.into();
    a
}
fn fl(name: &str, short: &str, d: f64, text: &str) -> AttrDef {
    doc(base(name, short, PlugType::Float, PlugValue::Float(d), IN), text)
}
fn ranged(mut a: AttrDef, lo: f64, hi: f64) -> AttrDef {
    a.min_value = Some(lo);
    a.soft_max = Some(hi);
    a
}
fn int(name: &str, short: &str, d: i64, text: &str) -> AttrDef {
    doc(base(name, short, PlugType::Int, PlugValue::Int(d), STORE), text)
}
fn boolean(name: &str, short: &str, d: bool, text: &str) -> AttrDef {
    doc(base(name, short, PlugType::Bool, PlugValue::Bool(d), STORE), text)
}
fn string(name: &str, short: &str, text: &str) -> AttrDef {
    doc(base(name, short, PlugType::String, PlugValue::String(String::new()), STORE), text)
}
fn splines_in(name: &str, short: &str) -> AttrDef {
    doc(base(name, short, PlugType::Data, PlugValue::Data(None), flags::INPUT | flags::HIDDEN), "Splines from the node before.")
}
fn splines_out(name: &str, short: &str) -> AttrDef {
    doc(base(name, short, PlugType::Data, PlugValue::Data(None), flags::OUTPUT | flags::HIDDEN), "Splines for the node after.")
}
fn color(name: &str, short: &str, d: [f64; 3], text: &str) -> AttrDef {
    let mut a = doc(base(name, short, PlugType::Vec3, PlugValue::Vec3(DVec3::from_array(d)), STORE | flags::COLOR), text);
    for (k, ch) in ["R", "G", "B"].iter().enumerate() {
        a.child_names[k] = format!("{name}{ch}");
        a.child_short[k] = format!("{short}{}", ch.to_lowercase());
    }
    a
}

/// What every modifier has: Mute, Mask, Magnitude and Magnitude Scale.
fn modifier_attrs() -> Vec<AttrDef> {
    vec![
        boolean("mute", "mu", false, "Off: the splines pass through unchanged."),
        string("mask", "msk", "A map scaling the effect per spline. Kept; not applied yet."),
        ranged(fl("magnitude", "mag", 1.0, "How much of the effect: 0 none, 1 all."), 0.0, 1.0),
        fl("magnitudeScale", "mags", 1.0, "A multiplier on Magnitude."),
        splines_in("inSplineData", "isd"),
        splines_out("outSplineData", "osd"),
    ]
}

pub(crate) fn at(d: &NodeData, name: &str) -> usize {
    d.ty().attr_index(name).unwrap_or_else(|| panic!("{} has no {name}", d.ty().name))
}
fn getf(d: &NodeData, name: &str) -> f64 {
    d.value(at(d, name)).as_float()
}
fn geti(d: &NodeData, name: &str) -> i64 {
    d.value(at(d, name)).as_int()
}
fn getb(d: &NodeData, name: &str) -> bool {
    d.value(at(d, name)).as_bool()
}

// ---------------------------------------------------------------------------------------------------------------------
// Computes

struct Compute(fn(&mut NodeData));

impl NodeBehavior for Compute {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        (self.0)(d)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Faces from a list such as "0:3 7": single numbers and inclusive ranges.
pub fn parse_faces(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    for w in text.split(|c: char| c.is_whitespace() || c == ',') {
        if let Some((a, b)) = w.split_once(':') {
            if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                out.extend(a.min(b)..=a.max(b));
            }
        } else if let Ok(x) = w.parse::<u32>() {
            out.push(x);
        }
    }
    out
}

fn base_compute(d: &mut NodeData) {
    let out = at(d, "outSplineData");
    let Some(mesh) = d.mesh(at(d, "boundMesh")).cloned() else {
        d.set_output(out, data_value(SplineData::default()));
        return;
    };
    let p = GrowParams {
        density: getf(d, "density") * getf(d, "densityMultiplier"),
        seed: geti(d, "seed").max(0) as u64,
        length: getf(d, "initialLength"),
        cv_count: geti(d, "cvCount").clamp(2, 1000) as usize,
        faces: parse_faces(d.value(at(d, "boundFaces")).as_str()),
    };
    d.set_output(out, data_value(generate(&mesh, &p)));
}

/// The splines in, or None when the modifier is muted or has none (then they pass through as they are).
fn modifier_input(d: &mut NodeData) -> Option<(SplineData, f64)> {
    let input = spline_data(d.value(at(d, "inSplineData"))).cloned();
    let out = at(d, "outSplineData");
    let pass = input.clone().unwrap_or_default();
    if getb(d, "mute") || input.is_none() {
        d.set_output(out, data_value(pass));
        return None;
    }
    let mag = getf(d, "magnitude") * getf(d, "magnitudeScale");
    Some((pass, mag))
}

/// Scale: each spline scaled about its root by 1 + (Scale - 1) x magnitude.
fn scale_compute(d: &mut NodeData) {
    let Some((mut s, mag)) = modifier_input(d) else { return };
    let k = 1.0 + (getf(d, "scale") - 1.0) * mag;
    for sp in &mut s.splines {
        let root = sp.root();
        for p in &mut sp.points {
            *p = root + (*p - root) * k;
        }
    }
    d.set_output(at(d, "outSplineData"), data_value(s));
}

/// The description's widths: Width Scale, times the Width Ramp along the spline (0 at the root, 1 at the tip; no
/// points, 1 all along), times the taper: full width up to Taper Start, falling linearly by Taper to the tip.
pub fn width_at(scale: f64, taper: f64, start: f64, ramp: Option<f64>, t: f64) -> f64 {
    let start = start.clamp(0.0, 1.0);
    let tap = if t <= start || start >= 1.0 { 1.0 } else { 1.0 - taper * (t - start) / (1.0 - start) };
    (scale * ramp.unwrap_or(1.0) * tap).max(0.0)
}

fn description_compute(d: &mut NodeData) {
    Shape::compute_matrices(d);
    let out = at(d, "outRenderData");
    let Some(mut s) = spline_data(d.value(at(d, "inSplineData"))).cloned() else {
        d.set_output(out, PlugValue::Data(None));
        return;
    };
    let (scale, taper, start) = (getf(d, "widthScale"), getf(d, "taper"), getf(d, "taperStart"));
    let ramp = crate::utility_nodes::ramp_points(d, "widthRamp.widthRamp_Position", "widthRamp.widthRamp_FloatValue", "widthRamp.widthRamp_Interp");
    for sp in &mut s.splines {
        let n = sp.points.len().max(2) - 1;
        sp.width = (0..sp.points.len())
            .map(|j| {
                let t = j as f64 / n as f64;
                let r = crate::utility_nodes::ramp_value(&ramp, t).map(|v| v.x);
                width_at(scale, taper, start, r, t) as f32
            })
            .collect();
    }
    d.set_output(out, data_value(s));
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        sculpt::register_kind();
        let shape = types::find("shape").expect("shape");
        types::register_type(NodeType {
            name: BASE.into(),
            icon: "grain".into(),
            doc: "Interactive Groom generation: splines grown on the bound mesh by density and seed, straight along the normal, before any modifier.".into(),
            own_attrs: vec![
                doc(base("boundMesh", "bm", PlugType::Mesh, PlugValue::Mesh(None), flags::INPUT | flags::HIDDEN), "The mesh the splines grow on, in world space."),
                string("boundFaces", "bf", "The faces they grow on, as \"0:3 7\"; empty, every face."),
                ranged(fl("density", "den", 10.0, "Splines per face."), 0.0, 100.0),
                ranged(fl("densityMultiplier", "dm", 1.0, "A multiplier on Density."), 0.0, 10.0),
                string("densityMask", "dmsk", "A map scaling density. Kept; not applied yet."),
                int("seed", "sd", 0, "Generator Seed: another number places the splines elsewhere."),
                ranged(fl("initialLength", "il", 1.0, "Length of a new spline, in scene units."), 0.0, 10.0),
                int("cvCount", "cvc", 5, "Points on each spline."),
                int("subdivisionLevel", "sl", 3, "Density grid subdivision, 1 to 5. Kept; not applied yet."),
                boolean("interpolate", "itp", false, "Interpolate new splines from others. Kept; not applied yet."),
                splines_out("outSplineData", "osd"),
            ],
            create: Some(Arc::new(|| Box::new(Compute(base_compute)))),
            ignores_added_attrs: true,
            ..Default::default()
        })
        .expect(BASE);
        let mut scale = modifier_attrs();
        scale.push(fl("scale", "sc", 1.0, "Length multiplier, about the root."));
        types::register_type(NodeType {
            name: SCALE.into(),
            icon: "arrows-maximize".into(),
            doc: "Interactive Groom modifier: scales splines about their roots.".into(),
            own_attrs: scale,
            create: Some(Arc::new(|| Box::new(Compute(scale_compute)))),
            ignores_added_attrs: true,
            ..Default::default()
        })
        .expect(SCALE);
        let mut sc = modifier_attrs();
        sc.extend(sculpt::attrs());
        types::register_type(NodeType {
            name: SCULPT.into(),
            icon: "brush".into(),
            doc: "Interactive Groom modifier: sculpt layers, each a set of moves on spline points made by the grooming tools, blended by weight.".into(),
            own_attrs: sc,
            create: Some(Arc::new(|| Box::new(Compute(sculpt::compute)))),
            ignores_added_attrs: true,
            ..Default::default()
        })
        .expect(SCULPT);
        let mut own = vec![
            splines_in("inSplineData", "isd"),
            doc(base("outRenderData", "ord", PlugType::Data, PlugValue::Data(None), flags::OUTPUT | flags::HIDDEN), "The splines with their widths, as views and renderers read them."),
            ranged(fl("widthScale", "ws", 0.01, "Width of the splines, in scene units."), 0.0, 0.1),
            ranged(fl("taper", "tp", 0.0, "How much the width falls to the tip."), 0.0, 1.0),
            ranged(fl("taperStart", "tps", 0.0, "Where along the spline the taper starts, 0 root to 1 tip."), 0.0, 1.0),
            boolean("tubeShade", "ts", true, "Shade the splines as tubes in the views."),
            boolean("highlightPlacedSplines", "hps", false, "Show splines just placed in another colour."),
            color("color", "col", [0.55, 0.42, 0.28], "The splines' colour in the views."),
            ranged(fl("colorIntensity", "coli", 1.0, "Brightness of that colour."), 0.0, 2.0),
            boolean("drawControlVertices", "dcv", false, "Draw each spline's points."),
            ranged(fl("cvSize", "cvs", 4.0, "Size of the points drawn, in pixels."), 1.0, 20.0),
            ranged(fl("cvOpacity", "cvo", 1.0, "Opacity of the points drawn."), 0.0, 1.0),
            ranged(fl("renderDensityMultiplier", "rdm", 1.0, "The fraction of the splines rendered. Below 1 thins them; above 1 is kept but renders them all."), 0.0, 1.0),
            crate::shading::inst_obj_groups(),
            crate::shading::object_groups(),
        ];
        own.extend(crate::utility_nodes::ramp_attrs("widthRamp", "wr"));
        for (n, s, dflt) in [("primaryVisibility", "vis", true), ("castsShadows", "csh", true), ("receiveShadows", "rcsh", true), ("aiOpaque", "ai_opaque", true)] {
            own.push(boolean(n, s, dflt, "Render visibility."));
        }
        own.push(doc(base("aiMode", "ai_mode", PlugType::Enum, PlugValue::Int(0), STORE), "Ribbon (faces the camera) or Thick (round)."));
        if let Some(a) = own.last_mut() {
            a.enum_names = vec!["ribbon".into(), "thick".into()];
        }
        types::register_type(NodeType {
            name: DESCRIPTION.into(),
            base: Some(shape),
            dag: true,
            icon: "grain".into(),
            doc: "An Interactive Groom description: the splines its stack makes, given width and taper, drawn and rendered.".into(),
            own_attrs: own,
            create: Some(Arc::new(|| Box::new(Compute(description_compute)))),
            ..Default::default()
        })
        .expect(DESCRIPTION);
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// Strands for the views and Harold

fn read<'a>(g: &'a Graph, id: NodeId, name: &str) -> Option<&'a PlugValue> {
    let n = g.node(id)?;
    let i = n.ty().attr_index(name)?;
    Some(n.data().value(i))
}

/// Descriptions in the scene.
pub fn descriptions(g: &Graph) -> Vec<NodeId> {
    let mut v: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().is_a(DESCRIPTION))).collect();
    v.sort_unstable();
    v
}

/// The splines a description draws or renders, as last evaluated.
pub fn render_splines(g: &Graph, id: NodeId) -> Option<&SplineData> {
    spline_data(read(g, id, "outRenderData")?)
}

fn to_strands(g: &Graph, id: NodeId, thin: f64) -> Option<Strands> {
    let s = render_splines(g, id)?;
    let mut st = Strands { shape: id, ..Default::default() };
    for sp in &s.splines {
        if thin < 1.0 && hash01(sp.id) >= thin {
            continue;
        }
        st.counts.push(sp.points.len() as u32);
        st.points.extend(sp.points.iter().map(|p| p.as_vec3()));
        let w = |j: usize| sp.width.get(j).copied().unwrap_or(0.01);
        st.radius.extend((0..sp.points.len()).map(|j| w(j) * 0.5));
        st.root_uv.push(Vec2::from(sp.uv));
    }
    st.world = g.node(id).map(|n| n.data().matrix(DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY);
    st.thick = read(g, id, "aiMode").map(|v| v.as_int()) == Some(1);
    let c = read(g, id, "color").map(|v| v.as_vec3()).unwrap_or(DVec3::splat(0.5));
    let k = read(g, id, "colorIntensity").map(|v| v.as_float()).unwrap_or(1.0);
    st.color = (c * k).as_vec3().to_array();
    Some(st)
}

fn view_source(g: &Graph) -> Vec<Strands> {
    descriptions(g).into_iter().filter(|&id| crate::harold::visible(g, id)).filter_map(|id| to_strands(g, id, 1.0)).collect()
}

fn render_source(g: &mut Graph) -> Vec<Strands> {
    g.evaluate();
    let g: &Graph = g;
    descriptions(g)
        .into_iter()
        .filter(|&id| crate::harold::visible(g, id))
        .filter_map(|id| {
            let thin = read(g, id, "renderDensityMultiplier").map(|v| v.as_float()).unwrap_or(1.0);
            to_strands(g, id, thin)
        })
        .collect()
}

/// Puts descriptions in the views and in Harold.
pub fn register_strands() {
    strands::register("xjen", render_source);
    strands::register_view("xjen", view_source);
}

pub fn unregister_strands() {
    strands::unregister("xjen");
}

/// The modifier chain feeding a description, base first: (node, type).
pub fn stack(g: &Graph, desc: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut cur = desc;
    for _ in 0..256 {
        let Some(p) = crate::harold::network::attr(g, cur, "inSplineData") else { break };
        let Some(src) = g.source(&p) else { break };
        out.push(src.node);
        if g.node(src.node).is_some_and(|n| n.ty().is_a(BASE)) {
            break;
        }
        cur = src.node;
    }
    out.reverse();
    out
}

/// The base at the bottom of a description's stack.
pub fn base_of(g: &Graph, desc: NodeId) -> Option<NodeId> {
    stack(g, desc).into_iter().find(|&n| g.node(n).is_some_and(|x| x.ty().is_a(BASE)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_taper_after_start() {
        assert_eq!(width_at(0.02, 0.5, 0.5, None, 0.25), 0.02);
        assert!((width_at(0.02, 0.5, 0.5, None, 1.0) - 0.01).abs() < 1e-12);
        assert!((width_at(0.02, 1.0, 0.0, Some(0.5), 0.5) - 0.005).abs() < 1e-12);
    }

    #[test]
    fn faces_parse() {
        assert_eq!(parse_faces("0:2 7,9"), vec![0, 1, 2, 7, 9]);
    }
}
