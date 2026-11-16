//! Alembic in the scene, as the reference's AbcExport and AbcImport plug-ins: AbcExport writes transforms, meshes
//! (points per frame, UVs) and XJEN descriptions (as curves with widths) under the roots of each job; AbcImport makes
//! them again, a static file as plain nodes, an animated one through an AlembicNode that streams the file into the
//! shapes and transforms as time changes.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aztek_abc::abc::{self, Archive, Object, TimeSampling};
use aztek_abc::geom;
use aztek_geo::Mesh;
use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::mesh_nodes::MeshNode;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugRef, PlugType, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::DagNode;

pub const NODE_TYPE: &str = "AlembicNode";

// ---------------------------------------------------------------------------------------------------------------------
// Job strings

/// One AbcExport job, from its -j string.
#[derive(Clone, Debug, PartialEq)]
pub struct Job {
    pub file: String,
    pub roots: Vec<String>,
    pub start: f64,
    pub end: f64,
    pub step: f64,
    pub uv: bool,
    pub world_space: bool,
}

/// Words of a job string, quotes kept together.
fn words(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote = false;
    for ch in s.chars() {
        match ch {
            '"' => quote = !quote,
            c if c.is_whitespace() && !quote => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn parse_job(s: &str, frame: f64) -> Result<Job, String> {
    let w = words(s);
    let mut j = Job { file: String::new(), roots: Vec::new(), start: frame, end: frame, step: 1.0, uv: false, world_space: false };
    let mut i = 0;
    let num = |w: &[String], i: usize, f: &str| -> Result<f64, String> { w.get(i).and_then(|x| x.parse().ok()).ok_or(format!("{f} needs a number")) };
    while i < w.len() {
        match w[i].as_str() {
            "-file" => {
                j.file = w.get(i + 1).cloned().ok_or("-file needs a path")?;
                i += 1;
            }
            "-root" | "-rt" => {
                j.roots.push(w.get(i + 1).cloned().ok_or("-root needs a node")?);
                i += 1;
            }
            "-frameRange" | "-fr" => {
                j.start = num(&w, i + 1, "-frameRange")?;
                j.end = num(&w, i + 2, "-frameRange")?;
                i += 2;
            }
            "-step" | "-s" => {
                j.step = num(&w, i + 1, "-step")?.max(1e-6);
                i += 1;
            }
            "-uvWrite" | "-uv" => j.uv = true,
            "-worldSpace" | "-ws" => j.world_space = true,
            // Accepted and not acted on: Aztek writes Ogawa, and these change nothing it writes.
            "-dataFormat" | "-df" | "-attr" | "-a" | "-attrPrefix" | "-atp" | "-userAttr" | "-u" | "-userAttrPrefix" | "-uatp" | "-frameRelativeSample" | "-frs" | "-preRollStartFrame" | "-melPerFrameCallback" | "-pythonPerFrameCallback" | "-melPostJobCallback" | "-pythonPostJobCallback" => i += 1,
            "-stripNamespaces" | "-sn" | "-noNormals" | "-nn" | "-renderableOnly" | "-ro" | "-selection" | "-sl" | "-writeVisibility" | "-wv" | "-eulerFilter" | "-ef" | "-writeColorSets" | "-writeFaceSets" | "-writeUVSets" | "-wholeFrameGeo" | "-wfg" | "-autoSubd" | "-writeCreases" | "-verbose" | "-v" | "-dontSkipUnwrittenFrames" | "-writeCurvesGroup" => {}
            other => return Err(format!("unknown job flag {other}")),
        }
        i += 1;
    }
    if j.file.is_empty() {
        return Err("a job needs -file".into());
    }
    Ok(j)
}

// ---------------------------------------------------------------------------------------------------------------------
// Export

fn cols(m: &DMat4) -> [f64; 16] {
    m.to_cols_array()
}

/// A mesh's topology: counts, indices into its compacted points, UVs per face-vertex (when it has them).
fn topology(m: &Mesh) -> (Vec<i32>, Vec<i32>, Vec<u32>, Option<Vec<[f32; 2]>>) {
    let mut remap = vec![u32::MAX; m.point_capacity() as usize];
    let mut kept = Vec::new();
    for p in 0..m.point_capacity() {
        if m.point_valid(p) {
            remap[p as usize] = kept.len() as u32;
            kept.push(p);
        }
    }
    let uv = m.corner_attrs().get::<[f32; 2]>(aztek_geo::names::uv(0).as_str());
    let has_uv = !uv.is_empty();
    let (mut counts, mut idx, mut uvs) = (Vec::new(), Vec::new(), Vec::new());
    for f in 0..m.face_capacity() {
        if !m.face_valid(f) {
            continue;
        }
        let c = m.face_corners(f);
        counts.push(c.len() as i32);
        for k in c {
            idx.push(remap[m.corner_point(k) as usize] as i32);
            if has_uv {
                uvs.push(uv.get(k as usize).copied().unwrap_or_default());
            }
        }
    }
    (counts, idx, kept, has_uv.then_some(uvs))
}

/// Per node to write: what it is and its samples.
enum Track {
    Xform { mats: Vec<[f64; 16]> },
    Mesh { topo: (Vec<i32>, Vec<i32>, Vec<u32>, Option<Vec<[f32; 2]>>), points: Vec<Vec<[f32; 3]>> },
    Hair { counts: Vec<i32>, points: Vec<Vec<[f32; 3]>>, widths: Vec<Vec<f32>> },
}

fn out_mesh(g: &Graph, id: NodeId) -> Option<Arc<Mesh>> {
    g.node(id)?.data().value(MeshNode::OUT_MESH).as_mesh().cloned()
}

/// Nodes under a root to write, parents before children.
fn gather(g: &Graph, id: NodeId, out: &mut Vec<NodeId>) {
    let Some(n) = g.node(id) else { return };
    let t = n.ty();
    let keep = crate::dag::is_transform(g, id) || (t.is_a("mesh") && !n.bool(DagNode::INTERMEDIATE_OBJECT)) || t.is_a(crate::xjen::DESCRIPTION);
    if !keep {
        return;
    }
    out.push(id);
    for &c in g.children(id) {
        gather(g, c, out);
    }
}

pub fn export(s: &mut Session, job: &Job) -> CmdResult<usize> {
    let mut roots = Vec::new();
    for r in &job.roots {
        let id = s.graph.find(r)?.ok_or_else(|| aztek_ops::CommandError::new(format!("AbcExport: no {r}")))?;
        roots.push(id);
    }
    if roots.is_empty() {
        // Every top-level transform but the cameras the scene starts with.
        let mut all: Vec<NodeId> = s.graph.nodes().into_iter().filter(|&id| crate::dag::is_transform(&s.graph, id) && s.graph.parent(id) == NO_NODE && !crate::cameras::is_default_camera(&s.graph, id)).collect();
        all.sort_unstable();
        roots = all;
    }
    let mut nodes = Vec::new();
    for &r in &roots {
        gather(&s.graph, r, &mut nodes);
    }
    let frames: Vec<f64> = {
        let n = (((job.end - job.start) / job.step) + 1e-6).floor().max(0.0) as usize + 1;
        (0..n).map(|k| job.start + job.step * k as f64).collect()
    };
    let was = crate::anim::frame(&s.graph);
    let mut tracks: HashMap<NodeId, Track> = HashMap::new();
    for (fi, &f) in frames.iter().enumerate() {
        crate::anim::set_frame(&mut s.graph, f);
        s.graph.evaluate();
        let g = &s.graph;
        for &id in &nodes {
            let n = g.node(id).unwrap();
            if crate::dag::is_transform(g, id) {
                let m = if job.world_space && roots.contains(&id) { n.data().matrix(DagNode::WORLD_MATRIX) } else { n.data().matrix(crate::transform::Transform::MATRIX) };
                match tracks.entry(id).or_insert(Track::Xform { mats: Vec::new() }) {
                    Track::Xform { mats, .. } => mats.push(cols(&m)),
                    _ => {}
                }
            } else if n.ty().is_a("mesh") {
                let Some(m) = out_mesh(g, id) else { continue };
                let e = tracks.entry(id).or_insert_with(|| Track::Mesh { topo: topology(&m), points: Vec::new() });
                if let Track::Mesh { topo, points, .. } = e {
                    if fi == 0 || topo.2.len() as u32 == m.num_points() as u32 {
                        points.push(topo.2.iter().map(|&p| m.position(p).as_vec3().to_array()).collect());
                    }
                }
            } else if let Some(d) = crate::xjen::render_splines(g, id) {
                let e = tracks.entry(id).or_insert_with(|| Track::Hair { counts: d.splines.iter().map(|x| x.points.len() as i32).collect(), points: Vec::new(), widths: Vec::new() });
                if let Track::Hair { points, widths, .. } = e {
                    points.push(d.splines.iter().flat_map(|x| x.points.iter().map(|p| p.as_vec3().to_array())).collect());
                    widths.push(d.splines.iter().flat_map(|x| x.width.iter().copied()).collect());
                }
            }
        }
    }
    crate::anim::set_frame(&mut s.graph, was);
    s.graph.evaluate();
    // Objects, built from the leaves up.
    let fps = crate::anim::fps();
    let mut a = Archive::default();
    a.meta = format!("_ai_AlembicVersion=Alembic {}.{}.{};_ai_Application=Aztek;_ai_DateWritten=;_ai_UserDescription=", abc::LIBRARY_VERSION / 10000, (abc::LIBRARY_VERSION / 100) % 100, abc::LIBRARY_VERSION % 100);
    a.samplings.push(TimeSampling::Uniform { start: job.start / fps, step: job.step / fps });
    fn build(g: &Graph, id: NodeId, tracks: &HashMap<NodeId, Track>, uv: bool) -> Option<Object> {
        let name = g.node(id)?.name().rsplit(':').next().unwrap_or("").to_string();
        match tracks.get(&id)? {
            Track::Xform { mats, .. } => {
                let kids: Vec<Object> = g.children(id).iter().filter_map(|&c| build(g, c, tracks, uv)).collect();
                Some(geom::xform(&name, 1, mats, kids))
            }
            Track::Mesh { topo, points, .. } => {
                let samples: Vec<geom::MeshSample> = points.iter().map(|p| geom::MeshSample { points: p.clone() }).collect();
                Some(geom::polymesh(&name, 1, &samples, &topo.0, &topo.1, if uv { topo.3.as_deref() } else { None }))
            }
            Track::Hair { counts, points, widths, .. } => Some(geom::curves(&name, 1, points, widths, counts, false)),
        }
    }
    a.top.children = roots.iter().filter_map(|&r| build(&s.graph, r, &tracks, job.uv)).collect();
    let bytes = abc::write(&a);
    std::fs::write(&job.file, bytes).map_err(|e| aztek_ops::CommandError::new(format!("AbcExport: {}: {e}", job.file)))?;
    Ok(a.top.children.len())
}

// ---------------------------------------------------------------------------------------------------------------------
// The AlembicNode: a file streamed into shapes and transforms.

type Cached = (std::time::SystemTime, Arc<Archive>);

fn cache() -> &'static Mutex<HashMap<String, Cached>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, Cached>>> = std::sync::OnceLock::new();
    C.get_or_init(Default::default)
}

/// A file's archive, read once while it stays the same.
pub fn archive(path: &str) -> Result<Arc<Archive>, String> {
    let stamp = std::fs::metadata(path).and_then(|m| m.modified()).map_err(|e| format!("{path}: {e}"))?;
    if let Some((t, a)) = cache().lock().unwrap().get(path) {
        if *t == stamp {
            return Ok(a.clone());
        }
    }
    let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let a = Arc::new(abc::read(bytes).map_err(|e| format!("{path}: {e}"))?);
    cache().lock().unwrap().insert(path.to_string(), (stamp, a.clone()));
    Ok(a)
}

/// The frames an archive's samples span, at the scene's frame rate; None when nothing in it moves.
pub fn frame_range(path: &str) -> Option<(f64, f64)> {
    let a = archive(path).ok()?;
    let mut range: Option<(f64, f64)> = None;
    fn walk(a: &Archive, o: &Object, range: &mut Option<(f64, f64)>) {
        let count = match geom::schema(o).as_deref() {
            Some(s) if s.contains("Xform") => geom::xform_matrices(o).len(),
            Some(s) if s.contains("PolyMesh") => geom::read_mesh(o).map_or(0, |m| m.points.len()),
            _ => 0,
        };
        if count > 1 {
            if let Some(ts) = a.samplings.get(geom::sampling_of(o) as usize) {
                let (t0, t1) = (ts.time(0), ts.time(count - 1));
                *range = Some(range.map_or((t0, t1), |(lo, hi)| (lo.min(t0), hi.max(t1))));
            }
        }
        for c in &o.children {
            walk(a, c, range);
        }
    }
    walk(&a, &a.top, &mut range);
    let fps = crate::anim::fps();
    range.map(|(lo, hi)| ((lo * fps).round(), (hi * fps).round()))
}

/// An object by its path in the archive ("/a/b").
pub fn find<'a>(top: &'a Object, path: &str) -> Option<&'a Object> {
    let mut o = top;
    for part in path.split('/').filter(|p| !p.is_empty()) {
        o = o.children.iter().find(|c| c.name == part)?;
    }
    Some(o)
}

/// The sample of a time: the nearest at or before it, clamped to the samples there are.
pub fn sample_at(a: &Archive, ts: u32, count: usize, seconds: f64) -> usize {
    if count <= 1 {
        return 0;
    }
    match a.samplings.get(ts as usize) {
        Some(TimeSampling::Uniform { start, step }) if *step > 0.0 => (((seconds - start) / step) + 1e-6).floor().clamp(0.0, (count - 1) as f64) as usize,
        Some(TimeSampling::Acyclic(t)) => t.iter().rposition(|&x| x <= seconds + 1e-9).unwrap_or(0).min(count - 1),
        _ => 0,
    }
}

fn mesh_from(r: &geom::MeshRead, k: usize) -> Mesh {
    let pts: Vec<DVec3> = r.points.get(k).or(r.points.first()).map(|p| p.iter().map(|x| DVec3::new(x[0] as f64, x[1] as f64, x[2] as f64)).collect()).unwrap_or_default();
    let counts: Vec<u32> = r.counts.iter().map(|&c| c.max(0) as u32).collect();
    let idx: Vec<u32> = r.indices.iter().map(|&i| i.max(0) as u32).collect();
    let mut m = Mesh::from_polygons(&pts, &counts, &idx);
    if let Some(u) = &r.uvs {
        let dst = m.corner_attrs_mut().get_or_add::<[f32; 2]>(&aztek_geo::names::uv(0));
        for (d, s) in dst.iter_mut().zip(u) {
            *d = *s;
        }
    }
    m
}

struct AlembicNode;

fn at(d: &NodeData, n: &str) -> usize {
    d.ty().attr_index(n).unwrap()
}

impl NodeBehavior for AlembicNode {
    fn compute(&mut self, ctx: &EvalContext, d: &mut NodeData) {
        let path = d.value(at(d, "abc_File")).as_str().to_string();
        let objects: Vec<(String, String)> = serde_json::from_str::<Vec<(String, String)>>(d.value(at(d, "abc_Objects")).as_str()).unwrap_or_default();
        let shift = d.value(at(d, "offset")).as_float();
        let speed = d.value(at(d, "speed")).as_float();
        let t = (ctx.time - shift) * if speed == 0.0 { 1.0 } else { speed };
        let Ok(a) = archive(&path) else { return };
        let (mut meshes, mut chans) = (Vec::new(), Vec::new());
        let (mut mi, mut ci) = (0i32, 0i32);
        for (p, kind) in &objects {
            let Some(o) = find(&a.top, p) else {
                if kind == "mesh" {
                    mi += 1;
                } else {
                    ci += 9;
                }
                continue;
            };
            if kind == "mesh" {
                if let Some(r) = geom::read_mesh(o) {
                    let k = sample_at(&a, geom::sampling_of(o), r.points.len(), t);
                    meshes.push((mi, PlugValue::Mesh(Some(Arc::new(mesh_from(&r, k))))));
                }
                mi += 1;
            } else {
                let ms = geom::xform_matrices(o);
                let k = sample_at(&a, geom::sampling_of(o), ms.len(), t);
                let m = DMat4::from_cols_array(&ms[k]);
                let (tr, rot, sc, _) = crate::transform::decompose_full(&m);
                let r = crate::transform::matrix_to_euler(&rot, crate::transform::RotateOrder::Xyz);
                for (j, v) in [tr.x, tr.y, tr.z, r.x, r.y, r.z, sc.x, sc.y, sc.z].into_iter().enumerate() {
                    chans.push((ci + j as i32, PlugValue::Float(v)));
                }
                ci += 9;
            }
        }
        let (om, ot) = (at(d, "outPolyMesh"), at(d, "transOp"));
        d.set_output_elements(om, meshes);
        d.set_output_elements(ot, chans);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let store = flags::INPUT | flags::STORABLE;
        let a = |name: &str, short: &str, ty: PlugType, d: PlugValue, fl: u32, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty, default_value: d, flags: fl, doc: doc.into(), ..Default::default() };
        let _ = types::register_type(NodeType {
            name: NODE_TYPE.into(),
            icon: "file-import".into(),
            time_dependent: true,
            doc: "An Alembic file streamed into the scene: each frame, the meshes' points and the transforms' channels at that time.".into(),
            own_attrs: vec![
                a("abc_File", "fn", PlugType::String, PlugValue::String(String::new()), store, "The file."),
                a("abc_Objects", "abco", PlugType::String, PlugValue::String("[]".into()), store | flags::HIDDEN, "What each output is: the objects' paths in the file, in output order."),
                a("time", "tm", PlugType::Float, PlugValue::Float(0.0), flags::INPUT | flags::HIDDEN, "Kept for files that connect time1 here; the scene's time is used."),
                a("offset", "of", PlugType::Float, PlugValue::Float(0.0), store, "Seconds the file is shifted by."),
                a("speed", "sp", PlugType::Float, PlugValue::Float(1.0), store, "How fast the file plays."),
                a("outPolyMesh", "opm", PlugType::Mesh, PlugValue::Mesh(None), flags::OUTPUT | flags::HIDDEN | flags::ARRAY, "Meshes, in object order."),
                a("transOp", "to", PlugType::Float, PlugValue::Float(0.0), flags::OUTPUT | flags::ARRAY, "Transforms' channels, nine each: translate, rotate, scale."),
            ],
            create: Some(Arc::new(|| Box::new(AlembicNode))),
            ..Default::default()
        });
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// Import

fn animated(o: &Object) -> bool {
    match geom::schema(o).as_deref() {
        Some(geom::XFORM) => geom::xform_matrices(o).windows(2).any(|w| w[0] != w[1]),
        Some(geom::POLYMESH) => geom::read_mesh(o).is_some_and(|m| m.points.windows(2).any(|w| w[0] != w[1])),
        _ => false,
    }
}

struct Importer {
    made: Vec<String>,
    streamed: Vec<(String, String, String)>,
}

impl Importer {
    fn object(&mut self, s: &mut Session, o: &Object, path: &str, parent: NodeId) -> CmdResult<()> {
        let me = format!("{path}/{}", o.name);
        match geom::schema(o).as_deref() {
            Some(geom::XFORM) => {
                let tt = types::find("transform").unwrap();
                let id = s.graph.create_node(&tt, &o.name, parent, true)?;
                let name = s.graph.path(id);
                let ms = geom::xform_matrices(o);
                s.run("xform", json!({"args": [name.clone()], "matrix": ms[0].to_vec()}))?;
                if parent == NO_NODE {
                    self.made.push(name.clone());
                }
                if animated(o) {
                    self.streamed.push((me.clone(), "xform".into(), name.clone()));
                }
                for c in &o.children {
                    self.object(s, c, &me, id)?;
                }
            }
            Some(geom::POLYMESH) => {
                let Some(r) = geom::read_mesh(o) else { return Ok(()) };
                // Under an imported transform the shape goes straight there; at the top it gets a transform of its own.
                let shape = if parent != NO_NODE {
                    let st = types::find("mesh").unwrap();
                    s.graph.create_node(&st, &o.name, parent, true)?
                } else {
                    let (tr, sh) = crate::dag::create_with_shape(&mut s.graph, "mesh", &o.name, NO_NODE)?;
                    self.made.push(s.graph.path(tr));
                    sh
                };
                s.graph.set_value(PlugRef::new(shape, MeshNode::IN_MESH as i32), PlugValue::Mesh(Some(Arc::new(mesh_from(&r, 0)))))?;
                let _ = s.run("sets", json!({"edit": true, "forceElement": "initialShadingGroup", "args": [s.graph.path(shape)]}));
                if animated(o) {
                    self.streamed.push((me, "mesh".into(), s.graph.path(shape)));
                }
            }
            Some(geom::CURVES) => {
                let Some(c) = geom::read_curves(o) else { return Ok(()) };
                let mut at = 0usize;
                let pts = c.points.first().cloned().unwrap_or_default();
                for (k, &n) in c.counts.iter().enumerate() {
                    let n = n.max(0) as usize;
                    let p: Vec<Vec<f64>> = pts[at..(at + n).min(pts.len())].iter().map(|x| vec![x[0] as f64, x[1] as f64, x[2] as f64]).collect();
                    at += n;
                    if p.len() < 2 {
                        continue;
                    }
                    let deg = if c.cubic && p.len() >= 4 { 3 } else { 1 };
                    let r = s.run("curve", json!({"degree": deg, "point": p, "name": format!("{}{}", o.name, k + 1)}))?;
                    let name = r.as_str().map(str::to_string).or_else(|| r[0].as_str().map(str::to_string)).unwrap_or_default();
                    if parent != NO_NODE {
                        s.run("parent", json!({"args": [name.clone(), s.graph.path(parent)], "relative": true}))?;
                    } else {
                        self.made.push(name);
                    }
                }
            }
            _ => {
                for c in &o.children {
                    self.object(s, c, path, parent)?;
                }
            }
        }
        Ok(())
    }
}

pub fn import(s: &mut Session, path: &str) -> CmdResult<Vec<String>> {
    let a = archive(path).map_err(|e| aztek_ops::CommandError::new(format!("AbcImport: {e}")))?;
    let mut im = Importer { made: Vec::new(), streamed: Vec::new() };
    for c in &a.top.children {
        im.object(s, c, "", NO_NODE)?;
    }
    if !im.streamed.is_empty() {
        register_types();
        let stem = std::path::Path::new(path).file_stem().and_then(|x| x.to_str()).unwrap_or("abc").replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "_");
        let node = s.run("createNode", json!({"args": [NODE_TYPE], "name": format!("{stem}_AlembicNode"), "skipSelect": true}))?;
        let node = node.as_str().unwrap_or_default().to_string();
        s.run("setAttr", json!({"args": [format!("{node}.abc_File"), path], "type": "string"}))?;
        let objects: Vec<(String, String)> = im.streamed.iter().map(|(p, k, _)| (p.clone(), k.clone())).collect();
        s.run("setAttr", json!({"args": [format!("{node}.abc_Objects"), serde_json::to_string(&objects).unwrap_or_default()], "type": "string"}))?;
        let (mut mi, mut ci) = (0, 0);
        for (_, kind, target) in &im.streamed {
            if kind == "mesh" {
                s.run("connectAttr", json!({"args": [format!("{node}.outPolyMesh[{mi}]"), format!("{target}.inMesh")], "force": true}))?;
                mi += 1;
            } else {
                for (j, ch) in ["translateX", "translateY", "translateZ", "rotateX", "rotateY", "rotateZ", "scaleX", "scaleY", "scaleZ"].iter().enumerate() {
                    s.run("connectAttr", json!({"args": [format!("{node}.transOp[{}]", ci + j), format!("{target}.{ch}")], "force": true}))?;
                }
                ci += 9;
            }
        }
        im.made.push(node);
    }
    Ok(im.made)
}

/// Both commands.
pub fn register(c: &mut Commands<Session>) {
    register_export(c);
    register_import(c);
}

pub fn register_export(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "AbcExport",
            "Writes Alembic (Ogawa) files, one per -j job string: -file path, -root node (repeatable; every top-level object \
             when none), -frameRange start end, -step, -uvWrite, -worldSpace. Transforms, meshes (points every frame, \
             topology from the first) and XJEN descriptions (curves with widths) are written; other job flags are \
             accepted.",
            |s, a| {
                let jobs: Vec<String> = match a.get("jobArg") {
                    Some(Value::Array(v)) => v.iter().filter_map(Value::as_str).map(str::to_string).collect(),
                    Some(Value::String(j)) => vec![j.clone()],
                    _ => fail!("AbcExport: give -j \"-file path ...\""),
                };
                let frame = crate::anim::frame(&s.graph);
                let mut n = 0;
                for j in jobs {
                    let job = parse_job(&j, frame).map_err(|e| aztek_ops::CommandError::new(format!("AbcExport: {e}")))?;
                    n += export(s, &job)?;
                }
                Ok(json!(n))
            },
        )
        .flag(FlagDef::new("jobArg", "j", ArgType::Any, Value::Null, "A job string; give several for several files."))
        .undoable(false),
    );
}

pub fn register_import(c: &mut Commands<Session>) {
    register_types();
    c.register(
        CommandDef::<Session>::new(
            "AbcImport",
            "Reads an Alembic (Ogawa) file: transforms, meshes (with UVs) and curves. An animated file gets an AlembicNode \
             that streams it as time changes. Returns the top nodes made.",
            |s, a| {
                let names = a.named();
                let Some(path) = names.first() else { fail!("AbcImport: give a file") };
                Ok(json!(import(s, path)?))
            },
        )
        .positional("file")
        .flag(FlagDef::new("mode", "m", ArgType::String, Value::Null, "import (the default), open or replace: Aztek imports in each case."))
        .flag(FlagDef::new("connect", "ct", ArgType::String, Value::Null, "Accepted; not acted on yet.")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_strings_parse() {
        let j = parse_job("-frameRange 1 24 -step 0.5 -uvWrite -worldSpace -root |pCube1 -file \"/tmp/a b.abc\"", 1.0).unwrap();
        assert_eq!((j.start, j.end, j.step, j.uv, j.world_space), (1.0, 24.0, 0.5, true, true));
        assert_eq!(j.roots, vec!["|pCube1"]);
        assert_eq!(j.file, "/tmp/a b.abc");
        assert!(parse_job("-frameRange 1 2", 1.0).is_err());
    }
}
