//! The reference's gpuCache plug-in: the gpuCache command writes objects' geometry over a frame range to an Alembic
//! file, and the gpuCache shape plays such a file back in the views, at the scene's time, without bringing the
//! geometry into the scene. Animators swap a heavy rig for one to play back fast (mGear's Cache Manager does).
//!
//! The shape reads its file's meshes under `cacheGeomPath`, each moved by the transforms above it in the file, and
//! draws them as one mesh under its own transform. Files are read through the same Alembic reader as AbcImport.

use std::path::Path;
use std::sync::{Arc, Mutex};

use aztek_abc::geom;
use aztek_geo::Mesh;
use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::DMat4;
use serde_json::{json, Value};

use crate::alembic::{archive, find, sample_at};
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, PlugType, PlugValue};
use crate::session::Session;

pub const NODE_TYPE: &str = "gpuCache";
pub const PLUGIN: &str = "gpuCache";

fn at(d: &NodeData, n: &str) -> usize {
    d.ty().attr_index(n).unwrap()
}

/// The mesh drawn for a file, path and time, kept while they stay the same.
#[derive(Default)]
struct GpuCache {
    held: Mutex<Option<(String, String, f64, Arc<Mesh>)>>,
}

/// The file's meshes under `object`, at time `t` (seconds), in the file's world space, as one mesh.
pub fn read_at(file: &str, object: &str, t: f64) -> Option<Mesh> {
    let a = archive(file).ok()?;
    let top = if object.is_empty() || object == "|" || object == "/" { &a.top } else { find(&a.top, &object.replace('|', "/"))? };
    let mut parts: Vec<Mesh> = Vec::new();
    // The transforms above the start, so a path into the file keeps its place.
    let mut above = DMat4::IDENTITY;
    if !std::ptr::eq(top, &a.top) {
        let mut chain = Vec::new();
        let mut path = String::new();
        for piece in object.replace('|', "/").split('/').filter(|p| !p.is_empty()) {
            path.push('/');
            path.push_str(piece);
            if let Some(o) = find(&a.top, &path) {
                chain.push(o);
            }
        }
        chain.pop();
        for o in chain {
            above *= local(&a, o, t);
        }
    }
    walk(&a, top, above, t, &mut parts);
    let refs: Vec<&Mesh> = parts.iter().collect();
    Some(aztek_geo::ops::basic::combine(&refs))
}

fn local(a: &aztek_abc::abc::Archive, o: &aztek_abc::abc::Object, t: f64) -> DMat4 {
    if geom::schema(o).as_deref() != Some(geom::XFORM) {
        return DMat4::IDENTITY;
    }
    let ms = geom::xform_matrices(o);
    if ms.is_empty() {
        return DMat4::IDENTITY;
    }
    let k = sample_at(a, geom::sampling_of(o), ms.len(), t);
    DMat4::from_cols_array(&ms[k])
}

fn walk(a: &aztek_abc::abc::Archive, o: &aztek_abc::abc::Object, parent: DMat4, t: f64, out: &mut Vec<Mesh>) {
    let world = parent * local(a, o, t);
    if geom::schema(o).as_deref() == Some(geom::POLYMESH) {
        if let Some(r) = geom::read_mesh(o) {
            let k = sample_at(a, geom::sampling_of(o), r.points.len(), t);
            let mut m = crate::alembic::mesh_at(&r, k);
            for p in m.positions_mut() {
                *p = world.transform_point3(*p);
            }
            out.push(m);
        }
    }
    for c in &o.children {
        walk(a, c, world, t, out);
    }
}

impl NodeBehavior for GpuCache {
    fn compute(&mut self, ctx: &EvalContext, d: &mut NodeData) {
        let file = d.value(at(d, "cacheFileName")).as_str().to_string();
        let object = d.value(at(d, "cacheGeomPath")).as_str().to_string();
        let t = ctx.time * d.value(at(d, "animTimeScale")).as_float() + d.value(at(d, "animOffset")).as_float();
        let out = at(d, "outMesh");
        if file.is_empty() {
            d.set_output(out, PlugValue::Mesh(None));
            return;
        }
        let mut held = self.held.lock().unwrap();
        if let Some((f, o, ht, m)) = held.as_ref() {
            if *f == file && *o == object && *ht == t {
                d.set_output(out, PlugValue::Mesh(Some(m.clone())));
                return;
            }
        }
        let mesh = read_at(&file, &object, t).map(Arc::new);
        if let Some(m) = &mesh {
            *held = Some((file, object, t, m.clone()));
        }
        d.set_output(out, PlugValue::Mesh(mesh));
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
        let Some(shape) = types::find("shape") else { return };
        let _ = types::register_type(NodeType {
            name: NODE_TYPE.into(),
            base: Some(shape),
            dag: true,
            icon: "box-multiple".into(),
            time_dependent: true,
            doc: "An Alembic file drawn in the views at the scene's time, without its geometry in the scene: a heavy rig's \
                  playback stand-in. gpuCache writes the files."
                .into(),
            own_attrs: vec![
                a("cacheFileName", "cfn", PlugType::String, PlugValue::String(String::new()), store, "The Alembic file."),
                a("cacheGeomPath", "cmp", PlugType::String, PlugValue::String("|".into()), store, "Which of the file's objects: a path in the file, | for all of them."),
                a("animTimeScale", "ats", PlugType::Float, PlugValue::Float(1.0), store, "How fast the file plays."),
                a("animOffset", "aof", PlugType::Float, PlugValue::Float(0.0), store, "Seconds added to the scene's time."),
                a("outMesh", "om", PlugType::Mesh, PlugValue::Mesh(None), flags::OUTPUT | flags::HIDDEN, "What is drawn, in the shape's space."),
            ],
            create: Some(Arc::new(|| Box::new(GpuCache::default()))),
            ..Default::default()
        });
    });
}

/// The gpuCache command: writes each object's geometry over the frame range to an Alembic file and returns the files.
fn gpu_cache_cmd(s: &mut Session, a: &aztek_ops::Args) -> CmdResult {
    let mut objects: Vec<String> = a.positional.iter().filter_map(Value::as_str).map(str::to_string).collect();
    if a.bool("allDagObjects") || objects.is_empty() {
        objects = if a.bool("allDagObjects") { s.graph.roots().iter().copied().filter(|&r| s.graph.node(r).is_some_and(|n| n.ty().dag && !crate::defaults::is_default_node(n.name()))).map(|r| s.graph.path(r)).collect() } else { s.graph.selection().iter().map(|&n| s.graph.path(n)).collect() };
    }
    if objects.is_empty() {
        fail!("gpuCache: name the objects to cache, select them, or give -allDagObjects");
    }
    let frame = crate::anim::frame(&s.graph);
    let start = a.get("startTime").and_then(Value::as_f64).unwrap_or(frame);
    let end = a.get("endTime").and_then(Value::as_f64).unwrap_or(start);
    let step = a.get("simulationRate").and_then(Value::as_f64).filter(|v| *v > 0.0).unwrap_or(1.0);
    let dir = a.get("directory").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| {
        let ws = s.run("workspace", json!({"query": true, "queried": ["rootDirectory"]})).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_else(|| std::env::temp_dir().to_string_lossy().into_owned());
        Path::new(&ws).join("cache").join("alembic").to_string_lossy().into_owned()
    });
    std::fs::create_dir_all(&dir).map_err(|e| aztek_ops::CommandError::new(format!("gpuCache: {dir}: {e}")))?;
    let prefix = a.get("prefix").and_then(Value::as_str).unwrap_or("").to_string();
    let named = a.get("fileName").and_then(Value::as_str).map(str::to_string);
    // One file for everything unless each object is asked for its own; a file name given with several objects in
    // files of their own is their prefix.
    let separate = a.get("saveMultipleFiles").and_then(Value::as_bool).unwrap_or(true) && objects.len() > 1;
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    if separate {
        for o in &objects {
            let short = o.rsplit('|').next().unwrap_or(o).replace(':', "_");
            groups.push((format!("{prefix}{}{short}", named.clone().unwrap_or_default()), vec![o.clone()]));
        }
    } else {
        let base = named.clone().unwrap_or_else(|| objects[0].rsplit('|').next().unwrap_or(&objects[0]).replace(':', "_"));
        groups.push((format!("{prefix}{base}"), objects.clone()));
    }
    let mut written = Vec::new();
    for (name, roots) in groups {
        let file = Path::new(&dir).join(format!("{name}.abc")).to_string_lossy().replace('\\', "/");
        let job = crate::alembic::Job { file: file.clone(), roots: roots.clone(), start, end, step, uv: true, world_space: true };
        crate::alembic::export(s, &job)?;
        written.push(json!(file));
    }
    Ok(Value::Array(written))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    let f = |l: &str, s: &str, t: ArgType, d: &str| FlagDef::new(l, s, t, Value::Null, d);
    c.register(
        CommandDef::<Session>::new(
            "gpuCache",
            "Write objects' geometry over a frame range to Alembic files for gpuCache shapes to play back: -startTime, -endTime, -directory, -fileName, -prefix, -saveMultipleFiles (a file each, the default), -allDagObjects. Returns the files written.",
            gpu_cache_cmd,
        )
        .flag(f("startTime", "st", ArgType::Float, "First frame."))
        .flag(f("endTime", "et", ArgType::Float, "Last frame."))
        .flag(f("simulationRate", "smr", ArgType::Float, "Frames between samples (1)."))
        .flag(f("sampleMultiplier", "spm", ArgType::Int, "Accepted."))
        .flag(f("directory", "dir", ArgType::String, "Folder the files go in (the project's cache/alembic)."))
        .flag(f("fileName", "f", ArgType::String, "The file's name, without .abc."))
        .flag(f("prefix", "p", ArgType::String, "Put before each file's name."))
        .flag(f("saveMultipleFiles", "smf", ArgType::Bool, "A file for each object (true)."))
        .flag(f("allDagObjects", "all", ArgType::Bool, "Every object in the scene."))
        .flag(f("optimize", "o", ArgType::Bool, "Accepted: the file holds the meshes as they are."))
        .flag(f("optimizationThreshold", "ot", ArgType::Int, "Accepted."))
        .flag(f("writeMaterials", "wm", ArgType::Bool, "Accepted."))
        .flag(f("writeUVs", "wuv", ArgType::Bool, "Accepted: UVs are written."))
        .flag(f("useBaseTessellation", "ubt", ArgType::Bool, "Accepted."))
        .flag(f("showStats", "ss", ArgType::Bool, "Accepted."))
        .flag(f("showGlobalStats", "sgs", ArgType::Bool, "Accepted."))
        .flag(f("dataFormat", "df", ArgType::String, "ogawa, the only one written."))
        .flag(f("mergeHierarchy", "mh", ArgType::Bool, "Accepted."))
        .flag(f("writeToFile", "wf", ArgType::Bool, "Accepted."))
        .positional("objects")
        .undoable(false),
    );
}
