//! Shading baked into images over a mesh's UVs (#108): `convertSolidTx` as the reference's (a network's colour at each
//! texel of the faces, written to a file and given a file node), the UV Editor's Image > UV Editor Baking (the material
//! behind the UVs, whatever makes its colour), and the reference's PSD networks: `psdTextureFile` writes a Photoshop
//! file with a layer set for each channel and the UV snapshot over them, `psdExport` reads one back out,
//! `createPsdNetwork` does what Create PSD Network does (the document from the shader's attributes as they stand,
//! psdFileTex nodes in their place), and `updatePsdNetworks` draws the UV snapshot layer again from the UVs as they are
//! now, which Photoshop then shows and the textures pick up.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use aztek_geo::normals::{point_normals, NormalWeight};
use aztek_geo::{Index, Mesh};
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, CommandError, Commands, FlagDef};
use aztek_render::shader::{Input, Network, Out, ShadeCtx};
use glam::{DVec2, Vec2, Vec3};
use serde_json::{json, Value};

use super::network::NetBuilder;
use crate::graph::Graph;
use crate::mesh_nodes::MeshNode;
use crate::plug::{flags, NodeId, PlugRef, PlugType, PlugValue};
use crate::psd::{Layer, LayerSet, Psd};
use crate::session::Session;

type Cmd = CommandDef<Session>;

/// How an image is baked: its size, the part of UV space it covers (u0, u1, v0, v1), what is behind the faces, and
/// whether the faces' edges are grown out into it so filtering doesn't bring the background in at the seams.
#[derive(Clone, Debug)]
pub struct Opts {
    pub width: u32,
    pub height: u32,
    pub range: [f64; 4],
    pub background: [u8; 3],
    pub fill_seams: bool,
    pub anti_alias: bool,
    pub alpha: bool,
    /// The UV set read, by name; the current one when None.
    pub uv_set: Option<String>,
    /// Sample a flat plane spanning this UV range rather than the surfaces.
    pub plane: Option<[f64; 4]>,
    /// Where the camera is, for -doubleSided: normals facing away from it are turned round.
    pub camera: Option<Vec3>,
}

impl Default for Opts {
    fn default() -> Self {
        Opts { width: 256, height: 256, range: [0.0, 1.0, 0.0, 1.0], background: [0, 0, 0], fill_seams: true, anti_alias: false, alpha: false, uv_set: None, plane: None, camera: None }
    }
}

fn encode(c: f32) -> u8 {
    let c = if c.is_finite() { c.clamp(0.0, 1.0) } else { 0.0 };
    let s = if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
    (s * 255.0 + 0.5) as u8
}

/// What gives a plug its colour: its connection or its value when it takes one (a material's color), or the node's own
/// output when it is one (a texture's outColor).
fn input_of(nb: &mut NetBuilder, p: &PlugRef) -> Option<Input> {
    let def = nb.g.attr(p).clone();
    let name = def.name.clone();
    if def.flags & flags::OUTPUT != 0 && def.flags & flags::INPUT == 0 {
        let n = nb.node(p.node)?;
        let out = if p.component >= 0 {
            [Out::R, Out::G, Out::B][p.component.clamp(0, 2) as usize]
        } else if name.ends_with("Alpha") {
            Out::A
        } else if def.ty == PlugType::Vec3 {
            Out::Rgb
        } else {
            Out::R
        };
        return Some(Input::Link(n, out));
    }
    Some(nb.input(p.node, &name))
}

/// A mesh's faces in world space, ready to rasterize: per corner its point, normal and UV.
struct Surface {
    tris: Vec<[(Vec3, Vec3, Vec3, Vec2); 3]>,
}

/// The UVs of a corner in a set, by its layer name.
fn uv_in(m: &Mesh, layer: Option<&str>, c: Index) -> DVec2 {
    match layer {
        Some(l) => m.corner_attrs().get::<[f32; 2]>(l).get(c as usize).map_or(DVec2::ZERO, |u| DVec2::new(u[0] as f64, u[1] as f64)),
        None => crate::uv_misc::uv_at(m, c),
    }
}

/// The UV extent of a shape's faces (all of them when None): u0 u1 v0 v1.
pub fn uv_bounds(g: &mut Graph, shape: NodeId, faces: Option<&[Index]>, set: Option<&str>) -> Option<[f64; 4]> {
    g.evaluate();
    let m: std::sync::Arc<Mesh> = g.node(shape)?.data().mesh(MeshNode::OUT_MESH).cloned()?;
    let layer = set.and_then(|s| crate::uv_sets::layer(&m, s));
    let all: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
    let mut b = [f64::MAX, f64::MIN, f64::MAX, f64::MIN];
    for &f in faces.unwrap_or(&all) {
        if !m.face_valid(f) {
            continue;
        }
        for c in m.face_corners(f) {
            let uv = uv_in(&m, layer.as_deref(), c);
            b = [b[0].min(uv.x), b[1].max(uv.x), b[2].min(uv.y), b[3].max(uv.y)];
        }
    }
    (b[1] > b[0] && b[3] > b[2]).then_some(b)
}

fn surface_in(g: &mut Graph, shape: NodeId, faces: Option<&[Index]>, set: Option<&str>) -> Option<Surface> {
    g.evaluate();
    let m: std::sync::Arc<Mesh> = g.node(shape)?.data().mesh(MeshNode::OUT_MESH).cloned()?;
    let layer = set.and_then(|s| crate::uv_sets::layer(&m, s));
    let xf = crate::dag::transform_of(g, shape);
    let world = crate::rig::world(g, xf).as_mat4();
    let normal_m = world.inverse().transpose();
    let pn = point_normals(&m, NormalWeight::AreaAngle);
    let all: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
    let list = faces.unwrap_or(&all);
    let mut tris = Vec::new();
    for &f in list {
        if !m.face_valid(f) {
            continue;
        }
        let cs = m.face_corners(f);
        let at = |c: Index| {
            let p = m.corner_point(c);
            let obj = m.position(p).as_vec3();
            let n = pn.get(p as usize).copied().unwrap_or(Vec3::Z);
            let uv = uv_in(&m, layer.as_deref(), c);
            (world.transform_point3(obj), normal_m.transform_vector3(n).normalize_or_zero(), obj, Vec2::new(uv.x as f32, uv.y as f32))
        };
        for k in 1..cs.len().saturating_sub(1) {
            tris.push([at(cs[0]), at(cs[k]), at(cs[k + 1])]);
        }
    }
    Some(Surface { tris })
}

/// The colour of `src` over the surfaces' UVs: RGBA, top row first.
pub fn bake(s: &mut Session, src: &PlugRef, shapes: &[(NodeId, Option<Vec<Index>>)], o: &Opts) -> Result<Vec<u8>, String> {
    let project = crate::shading::project_dir(s);
    let scene = s.scene_path.clone();
    let g = &mut s.graph;
    let tex = super::swatch::textures();
    let mut net = Network::default();
    let input = {
        let mut nb = NetBuilder { g, net: &mut net, built: HashMap::new(), warnings: Vec::new(), scene_path: scene, project, ignore_textures: false, textures: tex.clone() };
        let i = input_of(&mut nb, src).ok_or("nothing the renderer shades")?;
        for w in nb.warnings {
            log::warn!("{w}");
        }
        i
    };
    let (w, h) = (o.width.max(1) as usize, o.height.max(1) as usize);
    let mut rgba = vec![0u8; w * h * 4];
    let mut covered = vec![false; w * h];
    let [u0, u1, v0, v1] = o.range;
    let to_px = |uv: Vec2| Vec2::new(((uv.x as f64 - u0) / (u1 - u0) * w as f64) as f32, ((1.0 - (uv.y as f64 - v0) / (v1 - v0)) * h as f64) as f32);
    let samples: &[Vec2] = if o.anti_alias { &[Vec2::new(0.25, 0.25), Vec2::new(0.75, 0.25), Vec2::new(0.25, 0.75), Vec2::new(0.75, 0.75)] } else { &[Vec2::new(0.5, 0.5)] };
    // A sample plane: a flat square whose UVs span the range, in place of the surfaces.
    let surfaces: Vec<Surface> = match o.plane {
        Some([pu0, pu1, pv0, pv1]) => {
            let c = |u: f64, v: f64| (Vec3::new(u as f32, v as f32, 0.0), Vec3::Z, Vec3::new(u as f32, v as f32, 0.0), Vec2::new(u as f32, v as f32));
            vec![Surface { tris: vec![[c(pu0, pv0), c(pu1, pv0), c(pu1, pv1)], [c(pu0, pv0), c(pu1, pv1), c(pu0, pv1)]] }]
        }
        None => shapes.iter().filter_map(|(shape, faces)| surface_in(g, *shape, faces.as_deref(), o.uv_set.as_deref())).collect(),
    };
    for surf in &surfaces {
        for t in &surf.tris {
            let p = [to_px(t[0].3), to_px(t[1].3), to_px(t[2].3)];
            let area = (p[1] - p[0]).perp_dot(p[2] - p[0]);
            if area.abs() < 1e-9 {
                continue;
            }
            let lo = p[0].min(p[1]).min(p[2]).floor().max(Vec2::ZERO);
            let hi = p[0].max(p[1]).max(p[2]).ceil().min(Vec2::new(w as f32, h as f32));
            for y in lo.y as usize..hi.y as usize {
                for x in lo.x as usize..hi.x as usize {
                    let mut sum = Vec3::ZERO;
                    let mut hits = 0;
                    for s in samples {
                        let q = Vec2::new(x as f32, y as f32) + *s;
                        let b1 = (q - p[0]).perp_dot(p[2] - p[0]) / area;
                        let b2 = (p[1] - p[0]).perp_dot(q - p[0]) / area;
                        let b0 = 1.0 - b1 - b2;
                        if b0 < -1e-4 || b1 < -1e-4 || b2 < -1e-4 {
                            continue;
                        }
                        let mix3 = |k: usize| -> Vec3 {
                            let get = |i: usize| match k {
                                0 => t[i].0,
                                1 => t[i].1,
                                _ => t[i].2,
                            };
                            get(0) * b0 + get(1) * b1 + get(2) * b2
                        };
                        let uv = t[0].3 * b0 + t[1].3 * b1 + t[2].3 * b2;
                        let mut n = mix3(1).normalize_or_zero();
                        // Double sided: the side the camera sees.
                        if let Some(cam) = o.camera {
                            if n.dot(cam - mix3(0)) < 0.0 {
                                n = -n;
                            }
                        }
                        let ctx = ShadeCtx { p: mix3(0), p_obj: mix3(2), n, ng: n, uv, wo: n, footprint: 1.0 / w.max(h) as f32, ..Default::default() };
                        let c = net.eval(input, &ctx, &tex);
                        sum += if c.is_finite() { c } else { Vec3::ZERO };
                        hits += 1;
                    }
                    if hits == 0 {
                        continue;
                    }
                    let c = sum / hits as f32;
                    let at = (y * w + x) * 4;
                    rgba[at..at + 4].copy_from_slice(&[encode(c.x), encode(c.y), encode(c.z), 255]);
                    covered[y * w + x] = true;
                }
            }
        }
    }
    if o.fill_seams {
        // A few texels out from every face's edge, each the mean of its covered neighbours.
        for _ in 0..4 {
            let was = covered.clone();
            let src = rgba.clone();
            for y in 0..h {
                for x in 0..w {
                    if was[y * w + x] {
                        continue;
                    }
                    let mut sum = [0u32; 3];
                    let mut n = 0;
                    for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (-1, 1), (1, -1)] {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 || !was[ny as usize * w + nx as usize] {
                            continue;
                        }
                        let at = (ny as usize * w + nx as usize) * 4;
                        for k in 0..3 {
                            sum[k] += src[at + k] as u32;
                        }
                        n += 1;
                    }
                    if n > 0 {
                        let at = (y * w + x) * 4;
                        for k in 0..3 {
                            rgba[at + k] = (sum[k] / n) as u8;
                        }
                        rgba[at + 3] = if o.alpha { 0 } else { 255 };
                        covered[y * w + x] = true;
                    }
                }
            }
        }
    }
    for i in 0..w * h {
        if rgba[i * 4 + 3] == 0 && !covered[i] {
            rgba[i * 4..i * 4 + 3].copy_from_slice(&o.background);
            rgba[i * 4 + 3] = if o.alpha { 0 } else { 255 };
        }
    }
    Ok(rgba)
}

/// The UV edges drawn white on nothing, as a layer goes over a PSD's channels.
pub fn uv_lines(g: &mut Graph, shapes: &[NodeId], w: u32, h: u32) -> Vec<u8> {
    let (w, h) = (w.max(1) as usize, h.max(1) as usize);
    let mut rgba = vec![0u8; w * h * 4];
    g.evaluate();
    for &shape in shapes {
        let Some(m) = g.node(shape).and_then(|n| n.data().mesh(MeshNode::OUT_MESH).cloned()) else { continue };
        for f in (0..m.face_capacity()).filter(|&f| m.face_valid(f)) {
            let cs = m.face_corners(f);
            for k in 0..cs.len() {
                let (a, b) = (crate::uv_misc::uv_at(&m, cs[k]), crate::uv_misc::uv_at(&m, cs[(k + 1) % cs.len()]));
                let to = |x: DVec2| DVec2::new(x.x * (w - 1) as f64, (1.0 - x.y) * (h - 1) as f64);
                let (p, q) = (to(a), to(b));
                let steps = (p.distance(q).ceil() as usize).max(1);
                for i in 0..=steps {
                    let d = p.lerp(q, i as f64 / steps as f64);
                    let (x, y) = (d.x.round(), d.y.round());
                    if x >= 0.0 && y >= 0.0 && (x as usize) < w && (y as usize) < h {
                        let at = (y as usize * w + x as usize) * 4;
                        rgba[at..at + 4].copy_from_slice(&[255, 255, 255, 255]);
                    }
                }
            }
        }
    }
    rgba
}

/// Writes 16 bits a channel where the format takes them (png, tif), else 8.
fn save_depth(path: &Path, rgba: &[u8], w: u32, h: u32, sixteen: bool) -> Result<(), String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if sixteen && matches!(ext.as_str(), "png" | "tif" | "tiff") {
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let wide: Vec<u16> = rgba.iter().map(|&x| x as u16 * 257).collect();
        let Some(img) = image::ImageBuffer::<image::Rgba<u16>, Vec<u16>>::from_raw(w, h, wide) else { return Err("bad image size".into()) };
        return img.save(path).map_err(|e| format!("{}: {e}", path.display()));
    }
    save(path, rgba, w, h)
}

fn save(path: &Path, rgba: &[u8], w: u32, h: u32) -> Result<(), String> {
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext == "jpg" || ext == "jpeg" {
        let rgb: Vec<u8> = rgba.chunks(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
        return image::save_buffer(path, &rgb, w, h, image::ExtendedColorType::Rgb8).map_err(|e| format!("{}: {e}", path.display()));
    }
    if matches!(ext.as_str(), "png" | "tif" | "tiff" | "tga" | "bmp") {
        return image::save_buffer(path, rgba, w, h, image::ExtendedColorType::Rgba8).map_err(|e| format!("{}: {e}", path.display()));
    }
    Err(format!("{}: write .png, .tif, .tga, .bmp or .jpg here", path.display()))
}

/// An image file as RGBA at a size.
fn load(path: &Path, w: u32, h: u32) -> Result<Vec<u8>, String> {
    let img = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?.to_rgba8();
    let img = if img.width() != w || img.height() != h { image::imageops::resize(&img, w, h, image::imageops::FilterType::Triangle) } else { img };
    Ok(img.into_raw())
}

fn plug_of(g: &Graph, name: &str, cmd: &str) -> CmdResult<PlugRef> {
    g.plug(name).map_err(|_| CommandError::new(format!("{cmd}: no attribute {name}")))
}

/// The shapes and faces named, or the selection's.
fn shapes_named(s: &mut Session, names: &[String], cmd: &str) -> CmdResult<Vec<(NodeId, Option<Vec<Index>>)>> {
    let g = &mut s.graph;
    let mut by_shape: Vec<(NodeId, Option<Vec<Index>>)> = Vec::new();
    let list: Vec<String> = if names.is_empty() { g.selection().iter().map(|&id| g.path(id)).collect() } else { names.to_vec() };
    for n in list {
        let (obj, comp) = match n.split_once(".f[") {
            Some((o, c)) => (o.to_string(), Some(c.trim_end_matches(']').to_string())),
            None => (n.clone(), None),
        };
        let Some(id) = g.find(&obj)? else { fail!("{cmd}: no object {obj}") };
        let Some(shape) = crate::components::as_mesh(g, id) else { fail!("{cmd}: {obj} is not a mesh") };
        let faces = comp.map(|c| {
            let (a, b) = c.split_once(':').unwrap_or((&c, &c));
            let (a, b) = (a.parse::<Index>().unwrap_or(0), b.parse::<Index>().unwrap_or(0));
            (a..=b).collect::<Vec<_>>()
        });
        match by_shape.iter_mut().find(|(s, _)| *s == shape) {
            Some((_, Some(have))) => {
                if let Some(f) = faces {
                    have.extend(f);
                } else {
                    *have = Vec::new();
                }
            }
            Some((_, None)) => {}
            None => by_shape.push((shape, faces)),
        }
    }
    for e in &mut by_shape {
        if e.1.as_ref().is_some_and(Vec::is_empty) {
            e.1 = None;
        }
    }
    if by_shape.is_empty() {
        fail!("{cmd}: name or select the meshes to bake");
    }
    Ok(by_shape)
}

fn opts_from(a: &Args, w: u32, h: u32) -> Opts {
    let mut o = Opts { width: a.int("resolutionX", w as i64).clamp(1, 8192) as u32, height: a.int("resolutionY", h as i64).clamp(1, 8192) as u32, ..Default::default() };
    if let Some(r) = a.get("uvRange").and_then(Value::as_array) {
        let v: Vec<f64> = r.iter().filter_map(Value::as_f64).collect();
        if v.len() == 4 && v[1] > v[0] && v[3] > v[2] {
            o.range = [v[0], v[1], v[2], v[3]];
        }
    }
    if let Some(c) = a.get("backgroundColor").and_then(Value::as_array) {
        let v: Vec<u8> = c.iter().filter_map(Value::as_f64).map(|x| x.clamp(0.0, 255.0) as u8).collect();
        if v.len() == 3 {
            o.background = [v[0], v[1], v[2]];
        }
    }
    o.fill_seams = a.get("fillTextureSeams").and_then(Value::as_bool).unwrap_or(true);
    o.anti_alias = a.bool("antiAlias");
    o.alpha = a.bool("alpha");
    o
}

/// convertSolidTx attribute objects...: the attribute's colour over the objects' UVs into -fileImageName, with a file
/// node reading it (none with -noFileNode). Returns the file node, or the image's path with -noFileNode.
fn convert_solid_tx(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let Some(attr) = names.first().cloned() else { fail!("convertSolidTx: name the attribute to bake, then the objects") };
    let src = plug_of(&s.graph, &attr, "convertSolidTx")?;
    let plane = a.bool("samplePlane");
    // A sample plane needs no surface.
    let shapes = if plane && names.len() < 2 { Vec::new() } else { shapes_named(s, &names[1..], "convertSolidTx")? };
    let mut o = opts_from(a, 256, 256);
    let uv_set = a.get("uvSetName").and_then(Value::as_str).filter(|x| !x.is_empty()).map(str::to_string);
    o.uv_set = uv_set.clone();
    if plane {
        let r: Vec<f64> = a.get("samplePlaneRange").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
        let r = if r.len() == 4 && r[1] > r[0] && r[3] > r[2] { [r[0], r[1], r[2], r[3]] } else { [0.0, 1.0, 0.0, 1.0] };
        o.plane = Some(r);
        o.range = r;
    }
    // The UVs sampled: the given range, the surfaces' whole extent (-fullUvRange), or the extent of the faces named
    // (-componentRange).
    let mut fitted = a.get("uvRange").is_some();
    if !plane && (a.bool("fullUvRange") || (a.bool("componentRange") && shapes.iter().any(|x| x.1.is_some()))) {
        let mut b = [f64::MAX, f64::MIN, f64::MAX, f64::MIN];
        for (sh, faces) in &shapes {
            let faces = if a.bool("fullUvRange") { None } else { faces.as_deref() };
            if let Some(x) = uv_bounds(&mut s.graph, *sh, faces, uv_set.as_deref()) {
                b = [b[0].min(x[0]), b[1].max(x[1]), b[2].min(x[2]), b[3].max(x[3])];
            }
        }
        if b[1] > b[0] && b[3] > b[2] {
            o.range = b;
            fitted = true;
        }
    }
    if a.bool("doubleSided") {
        let cam = a.get("camera").and_then(Value::as_str).map(str::to_string).or_else(|| s.graph.nodes().into_iter().find(|&n| s.graph.node(n).is_some_and(|x| x.name() == "persp")).map(|n| s.graph.path(n)));
        if let Some(c) = cam.and_then(|c| s.graph.find(&c).ok().flatten()) {
            let xf = crate::dag::transform_of(&s.graph, c);
            let w = crate::rig::world(&mut s.graph, xf);
            o.camera = Some(w.w_axis.truncate().as_vec3());
        }
    }
    let node_name = s.graph.node(src.node).map(|n| n.name().to_string()).unwrap_or_default();
    let attr_name = s.graph.attr(&src).name.clone();
    let first = match shapes.first() {
        Some(sh) => s.graph.path(crate::dag::transform_of(&s.graph, sh.0)).trim_start_matches('|').replace('|', "_"),
        None => "samplePlane".to_string(),
    };
    // With a UV set, the name says it: surface-set.
    let first = match &uv_set {
        Some(u) => format!("{first}-{u}"),
        None => first,
    };
    let file = match a.get("fileImageName").and_then(Value::as_str).filter(|x| !x.is_empty()) {
        Some(f) => PathBuf::from(f),
        None => {
            let dir = crate::shading::project_dir(s).map(|p| PathBuf::from(p).join("sourceimages")).unwrap_or_else(std::env::temp_dir);
            let ext = a.get("fileFormat").and_then(Value::as_str).unwrap_or("png");
            dir.join(format!("{node_name}_{attr_name}_{first}.{ext}"))
        }
    };
    if file.exists() && !a.bool("force") && !a.bool("noFileNode") {
        fail!("convertSolidTx: {} exists; -force writes over it", file.display());
    }
    let rgba = bake(s, &src, &shapes, &o).map_err(|e| CommandError::new(format!("convertSolidTx: {e}")))?;
    let sixteen = a.get("pixelFormat").map(|v| v.as_str().map(str::to_string).unwrap_or_else(|| v.to_string())).is_some_and(|v| v == "16");
    save_depth(&file, &rgba, o.width, o.height, sixteen).map_err(|e| CommandError::new(format!("convertSolidTx: {e}")))?;
    let path = file.to_string_lossy().replace('\\', "/");
    if a.bool("noFileNode") {
        return Ok(json!(path));
    }
    let name = a.get("name").and_then(Value::as_str).filter(|x| !x.is_empty()).map(str::to_string).unwrap_or_else(|| format!("{node_name}_{attr_name}_{first}"));
    let made = s.run("shadingNode", json!({"args": ["file"], "asTexture": true, "name": name}))?;
    let f = made.as_str().unwrap_or_default().to_string();
    s.run("setAttr", json!({"args": [format!("{f}.fileTextureName"), path], "type": "string"}))?;
    if fitted {
        // A placement maps the image back onto the part of UV space it was sampled from.
        let place = s.run("shadingNode", json!({"args": ["place2dTexture"], "asUtility": true}))?;
        let place = place.as_str().unwrap_or_default().to_string();
        s.run("defaultNavigation", json!({"connectToExisting": true, "source": place, "destination": f}))?;
        let [u0, u1, v0, v1] = o.range;
        s.run("setAttr", json!({"args": [format!("{place}.coverage"), u1 - u0, v1 - v0]}))?;
        s.run("setAttr", json!({"args": [format!("{place}.translateFrame"), u0, v0]}))?;
    }
    Ok(json!([f]))
}

/// psdTextureFile -psf f.psd -xr -yr -chs name [alpha] ... -ifn image layerSet [index] ... -ssi snapshot -uvt: a PSD with
/// a layer set for each channel, each filled from the image given it (transparent otherwise), and the UV snapshot on
/// top (-uvt, the default) or under the sets.
fn psd_texture_file(_s: &mut Session, a: &Args) -> CmdResult {
    let Some(file) = a.get("psdFileName").and_then(Value::as_str).filter(|x| !x.is_empty()).map(PathBuf::from) else { fail!("psdTextureFile: -psdFileName names the file") };
    let (w, h) = (a.int("xResolution", 1024).clamp(1, 16384) as u32, a.int("yResolution", 1024).clamp(1, 16384) as u32);
    let groups = |v: Option<&Value>| -> Vec<Vec<String>> {
        match v {
            Some(Value::Array(l)) if l.iter().all(Value::is_array) => l.iter().map(|x| x.as_array().unwrap().iter().map(|y| y.as_str().map(str::to_string).unwrap_or_else(|| y.to_string())).collect()).collect(),
            Some(Value::Array(l)) => vec![l.iter().map(|y| y.as_str().map(str::to_string).unwrap_or_else(|| y.to_string())).collect()],
            Some(Value::String(x)) => vec![vec![x.clone()]],
            _ => Vec::new(),
        }
    };
    let channels = groups(a.get("channels"));
    if channels.is_empty() {
        fail!("psdTextureFile: -channels names at least one layer set");
    }
    let images = groups(a.get("imageFileName"));
    let mut sets = Vec::new();
    for ch in &channels {
        let name = ch[0].clone();
        let mut layers = Vec::new();
        for im in images.iter().filter(|im| im.get(1) == Some(&name)) {
            let rgba = load(Path::new(&im[0]), w, h).map_err(|e| CommandError::new(format!("psdTextureFile: {e}")))?;
            layers.push(Layer::pixels(&name, w, h, rgba));
        }
        if layers.is_empty() {
            layers.push(Layer::pixels(&name, w, h, vec![0; (w * h * 4) as usize]));
        }
        sets.push(LayerSet { name, layers });
    }
    let mut top = Vec::new();
    if let Some(snap) = a.get("snapShotImageName").and_then(Value::as_str).filter(|x| !x.is_empty()) {
        let mut rgba = load(Path::new(snap), w, h).map_err(|e| CommandError::new(format!("psdTextureFile: {e}")))?;
        // A snapshot on black: the black goes, the lines stay.
        for p in rgba.chunks_mut(4) {
            if p[3] == 255 {
                p[3] = p[0].max(p[1]).max(p[2]);
            }
        }
        top.push(Layer::pixels("UVSnapShot", w, h, rgba));
    }
    let doc = if a.get("uvSnapPostionTop").and_then(Value::as_bool).unwrap_or(true) {
        Psd::from_sets(w, h, sets, top)
    } else {
        let mut d = Psd::from_sets(w, h, sets, Vec::new());
        d.layers.splice(0..0, top);
        d.merged = d.composite(None).unwrap_or_default();
        d
    };
    crate::psd::write_file(&file, &doc).map_err(|e| CommandError::new(format!("psdTextureFile: {e}")))?;
    Ok(json!(file.to_string_lossy().replace('\\', "/")))
}

/// psdExport -psf f.psd -ofn out.png [-lsn set]: a layer set (or all of the image) out to an image file; without -ofn,
/// the document's layer sets, top first.
fn psd_export(s: &mut Session, a: &Args) -> CmdResult {
    let _ = s;
    let Some(file) = a.get("psdFileName").and_then(Value::as_str).filter(|x| !x.is_empty()).map(PathBuf::from) else { fail!("psdExport: -psdFileName names the file") };
    let doc = crate::psd::read_file(&file).map_err(|e| CommandError::new(format!("psdExport: {e}")))?;
    // With no image to write: the layer sets, top first.
    if a.get("outFileName").and_then(Value::as_str).is_none_or(str::is_empty) {
        return Ok(json!(doc.layer_sets()));
    }
    let set = a.get("layerSetName").and_then(Value::as_str).unwrap_or("");
    let rgba = if set.is_empty() { if doc.layers.is_empty() { Some(doc.merged.clone()) } else { doc.composite(None) } } else { doc.composite(Some(set)) };
    let Some(rgba) = rgba else { fail!("psdExport: {} has no layer set {set}", file.display()) };
    let Some(out) = a.get("outFileName").and_then(Value::as_str).filter(|x| !x.is_empty()) else { fail!("psdExport: -outFileName names the image") };
    save(Path::new(out), &rgba, doc.width, doc.height).map_err(|e| CommandError::new(format!("psdExport: {e}")))?;
    Ok(json!(out))
}

/// The shapes a material is on, from its shading group's members.
fn shapes_of_material(g: &mut Graph, mat: NodeId) -> Vec<(NodeId, Option<Vec<Index>>)> {
    let Some(sg) = crate::shading::group_of_material(g, mat, false).ok().flatten() else { return Vec::new() };
    let mut out: Vec<(NodeId, Option<Vec<Index>>)> = Vec::new();
    for m in crate::object_sets::members(g, sg) {
        let faces = (m.kind == crate::components::ComponentMode::Face).then(|| m.ids.clone());
        if crate::components::as_mesh(g, m.node).is_some() {
            out.push((m.node, faces));
        }
    }
    out
}

/// What a channel's psdFileTex feeds: a colour's whole, a number's alpha, transparency's own, a bump through bump2d.
fn wire_channel(s: &mut Session, tex: &str, shader: &str, attr: &str) -> CmdResult<()> {
    let dst = format!("{shader}.{attr}");
    let g = &mut s.graph;
    let p = plug_of(g, &dst, "createPsdNetwork")?;
    // What drove it is in the layer now.
    if let Some(src) = g.source(&p) {
        g.disconnect(&p);
        let _ = src;
    }
    for k in 0..3 {
        let c = PlugRef::component(p.node, p.attr, k);
        if g.source(&c).is_some() {
            g.disconnect(&c);
        }
    }
    let def = g.attr(&p).clone();
    if attr == "normalCamera" {
        let b = s.run("shadingNode", json!({"args": ["bump2d"], "asUtility": true}))?;
        let b = b.as_str().unwrap_or_default().to_string();
        s.run("connectAttr", json!({"args": [format!("{tex}.outAlpha"), format!("{b}.bumpValue")]}))?;
        s.run("connectAttr", json!({"args": [format!("{b}.outNormal"), dst], "force": true}))?;
        return Ok(());
    }
    let out = if attr == "transparency" {
        "outTransparency"
    } else if def.ty == PlugType::Vec3 {
        "outColor"
    } else {
        "outAlpha"
    };
    s.run("connectAttr", json!({"args": [format!("{tex}.{out}"), dst], "force": true}))?;
    Ok(())
}

/// createPsdNetwork shader -psf f.psd -xr -yr -attributes color ... [-uvSnapshot false]: as UV Editor > Image > Create
/// PSD Network: a PSD with a layer set for each attribute (named shader.attribute, filled with the attribute as it is
/// now), the UV snapshot over them, and a psdFileTex for each in the attribute's place. Returns the file and the nodes.
fn create_psd_network(s: &mut Session, a: &Args) -> CmdResult {
    let g = &mut s.graph;
    let shader = match a.named().first() {
        Some(n) => n.clone(),
        None => {
            // The material on the first selected mesh.
            let mut found = None;
            for id in g.selection().to_vec() {
                let Some(m) = crate::components::as_mesh(g, id) else { continue };
                if let Some(mat) = crate::shading::groups_of(g, m).into_iter().find_map(|sg| crate::shading::material_of_group(g, sg)) {
                    found = Some(g.path(mat));
                    break;
                }
            }
            match found {
                Some(x) => x,
                None => fail!("createPsdNetwork: name a material, or select a mesh that has one"),
            }
        }
    };
    let Some(mat) = g.find(&shader)? else { fail!("createPsdNetwork: no node {shader}") };
    let shader = g.node(mat).map(|n| n.name().to_string()).unwrap_or(shader);
    let attrs: Vec<String> = match a.get("attributes") {
        Some(Value::Array(l)) => l.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        Some(Value::String(x)) => x.split_whitespace().map(str::to_string).collect(),
        _ => vec!["color".to_string()],
    };
    if attrs.is_empty() {
        fail!("createPsdNetwork: -attributes names at least one");
    }
    let shapes = shapes_of_material(g, mat);
    if shapes.is_empty() {
        fail!("createPsdNetwork: {shader} is on no mesh");
    }
    let (w, h) = (a.int("xResolution", 1024).clamp(1, 16384) as u32, a.int("yResolution", 1024).clamp(1, 16384) as u32);
    let file = match a.get("psdFileName").and_then(Value::as_str).filter(|x| !x.is_empty()) {
        Some(f) => PathBuf::from(f),
        None => {
            let dir = crate::shading::project_dir(s).map(|p| PathBuf::from(p).join("sourceimages")).unwrap_or_else(std::env::temp_dir);
            dir.join(format!("{shader}.psd"))
        }
    };
    let mut sets = Vec::new();
    for attr in &attrs {
        let src = plug_of(&s.graph, &format!("{shader}.{attr}"), "createPsdNetwork")?;
        let o = Opts { width: w, height: h, fill_seams: true, ..Default::default() };
        let rgba = bake(s, &src, &shapes, &o).map_err(|e| CommandError::new(format!("createPsdNetwork: {e}")))?;
        let name = format!("{shader}.{attr}");
        sets.push(LayerSet { name: name.clone(), layers: vec![Layer::pixels(&name, w, h, rgba)] });
    }
    let mut top = Vec::new();
    if a.get("uvSnapshot").and_then(Value::as_bool).unwrap_or(true) {
        let ids: Vec<NodeId> = shapes.iter().map(|x| x.0).collect();
        top.push(Layer::pixels("UVSnapShot", w, h, uv_lines(&mut s.graph, &ids, w, h)));
    }
    let doc = Psd::from_sets(w, h, sets, top);
    crate::psd::write_file(&file, &doc).map_err(|e| CommandError::new(format!("createPsdNetwork: {e}")))?;
    let path = file.to_string_lossy().replace('\\', "/");
    let mut nodes = Vec::new();
    for attr in &attrs {
        let t = s.run("shadingNode", json!({"args": ["psdFileTex"], "asTexture": true}))?.as_str().unwrap_or_default().to_string();
        let place = s.run("shadingNode", json!({"args": ["place2dTexture"], "asUtility": true}))?.as_str().unwrap_or_default().to_string();
        s.run("defaultNavigation", json!({"connectToExisting": true, "source": place, "destination": t}))?;
        s.run("setAttr", json!({"args": [format!("{t}.fileTextureName"), path], "type": "string"}))?;
        s.run("setAttr", json!({"args": [format!("{t}.layerSetName"), format!("{shader}.{attr}")], "type": "string"}))?;
        for (k, n) in doc.layer_sets().iter().enumerate() {
            s.run("setAttr", json!({"args": [format!("{t}.layerSets[{k}]"), n], "type": "string"}))?;
        }
        wire_channel(s, &t, &shader, attr)?;
        nodes.push(t);
    }
    Ok(json!({"file": path, "nodes": nodes}))
}

/// updatePsdNetworks [psdFileTex...]: the UV snapshot layer of each PSD the nodes read (all of them when none are named)
/// drawn again from the UVs as they are, the other layers left as Photoshop left them.
fn update_psd_networks(s: &mut Session, a: &Args) -> CmdResult {
    let project = crate::shading::project_dir(s);
    let scene = s.scene_path.clone();
    let g = &mut s.graph;
    let named = a.named();
    let mut texes = Vec::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        if n.ty().name == "psdFileTex" && (named.is_empty() || named.iter().any(|x| x == n.name() || *x == g.path(id))) {
            texes.push(id);
        }
    }
    // Each document with the meshes its nodes' materials are on.
    let mut docs: Vec<(PathBuf, Vec<NodeId>)> = Vec::new();
    for t in texes {
        let written = g.plug(&format!("{}.fileTextureName", g.path(t))).ok().and_then(|p| g.value(&p).ok()).and_then(|v| if let PlugValue::String(x) = v { Some(x) } else { None }).unwrap_or_default();
        let Some(file) = crate::shading::resolve_texture_uncached(&written, scene.as_deref(), project.as_deref()) else { continue };
        // Downstream to the materials, then their meshes.
        let mut todo = vec![t];
        let mut seen = Vec::new();
        let mut shapes = Vec::new();
        while let Some(n) = todo.pop() {
            if seen.contains(&n) {
                continue;
            }
            seen.push(n);
            let ty = g.node(n).map(|x| x.type_arc().clone());
            if ty.as_ref().is_some_and(|t| crate::shading::is_material_type(t)) {
                shapes.extend(shapes_of_material(g, n).into_iter().map(|x| x.0));
                continue;
            }
            let count = g.node(n).map_or(0, |x| x.ty().attrs.len());
            for i in 0..count {
                for k in -1..3 {
                    let p = if k < 0 { PlugRef::new(n, i as i32) } else { PlugRef::component(n, i as i32, k) };
                    todo.extend(g.destinations(&p).into_iter().map(|d| d.node));
                }
            }
        }
        match docs.iter_mut().find(|d| d.0 == file) {
            Some(d) => d.1.extend(shapes),
            None => docs.push((file, shapes)),
        }
    }
    let mut out = Vec::new();
    for (file, mut shapes) in docs {
        shapes.sort_unstable();
        shapes.dedup();
        let mut doc = crate::psd::read_file(&file).map_err(|e| CommandError::new(format!("updatePsdNetworks: {e}")))?;
        let lines = uv_lines(g, &shapes, doc.width, doc.height);
        match doc.layers.iter_mut().find(|l| l.name == "UVSnapShot" && l.kind == crate::psd::Kind::Pixels) {
            Some(l) => {
                l.rect = [0, 0, doc.height as i32, doc.width as i32];
                l.rgba = lines;
            }
            None => doc.layers.push(Layer::pixels("UVSnapShot", doc.width, doc.height, lines)),
        }
        doc.merged = doc.composite(None).unwrap_or_default();
        crate::psd::write_file(&file, &doc).map_err(|e| CommandError::new(format!("updatePsdNetworks: {e}")))?;
        out.push(file.to_string_lossy().replace('\\', "/"));
    }
    Ok(json!(out))
}

/// The UV Editor's background with Image > UV Editor Baking on: the colour of the first selected mesh's material baked
/// over its UVs, kept until the material or the mesh changes. None with nothing to bake.
pub fn uv_editor_bake(s: &mut Session, size: u32) -> Option<String> {
    static KEPT: std::sync::Mutex<Option<(u64, String)>> = std::sync::Mutex::new(None);
    let g = &mut s.graph;
    let (shape, mat) = g.selection().to_vec().into_iter().find_map(|id| {
        let m = crate::components::as_mesh(g, id)?;
        let mat = crate::shading::groups_of(g, m).into_iter().find_map(|sg| crate::shading::material_of_group(g, sg))?;
        Some((m, mat))
    })?;
    let attr = ["color", "baseColor", "outColor"].into_iter().find_map(|a| g.node(mat)?.ty().attr_index(a))?;
    // What the picture depends on: the material's network, the mesh, the size.
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    super::swatch::signature(g, mat).hash(&mut h);
    g.path(shape).hash(&mut h);
    size.hash(&mut h);
    g.evaluate();
    if let Some(m) = g.node(shape).and_then(|n| n.data().mesh(MeshNode::OUT_MESH).cloned()) {
        for f in (0..m.face_capacity()).filter(|&f| m.face_valid(f)) {
            for c in m.face_corners(f) {
                let uv = crate::uv_misc::uv_at(&m, c);
                (uv.x.to_bits(), uv.y.to_bits()).hash(&mut h);
            }
        }
    }
    let key = h.finish();
    if let Some((k, p)) = KEPT.lock().unwrap().as_ref() {
        if *k == key && Path::new(p).is_file() {
            return Some(p.clone());
        }
    }
    let src = PlugRef::new(mat, attr as i32);
    let o = Opts { width: size, height: size, ..Default::default() };
    let rgba = bake(s, &src, &[(shape, None)], &o).ok()?;
    let dir = std::env::temp_dir().join("aztek-uvbake");
    let path = dir.join(format!("bake_{key:016x}.png"));
    save(&path, &rgba, size, size).ok()?;
    let p = path.to_string_lossy().replace('\\', "/");
    *KEPT.lock().unwrap() = Some((key, p.clone()));
    Some(p)
}

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        Cmd::new("convertSolidTx", "Bake an attribute's colour (a material's color, a texture's outColor, anything a network gives) over the objects' UVs into an image, as the reference's: -rx -ry its size (256), -fin the file (a name in the project's sourceimages without it), -bc r g b behind the faces, -fts grows the faces out at the seams (on), -aa four samples a texel, -al leaves what no face covers transparent, -uvr u0 u1 v0 v1 the part of UV space. Makes a file node reading it and returns it; -noFileNode only writes the image and returns its path.", convert_solid_tx)
            .positional("attribute, then objects or faces (the selection when none)")
            .flag(f("resolutionX", "rx", ArgType::Int, "Width."))
            .flag(f("resolutionY", "ry", ArgType::Int, "Height."))
            .flag(f("fileImageName", "fin", ArgType::String, "The image to write."))
            .flag(f("fileFormat", "fil", ArgType::String, "png, tif, tga, bmp or jpg, with no -fin."))
            .flag(f("backgroundColor", "bc", ArgType::List, "0 to 255 each.").multi(3))
            .flag(f("backgroundMode", "bm", ArgType::String, "Accepted: the colour is what -bc gives."))
            .flag(f("fillTextureSeams", "fts", ArgType::Bool, "Grow the faces out at their edges."))
            .flag(f("antiAlias", "aa", ArgType::Bool, "Four samples a texel."))
            .flag(f("alpha", "al", ArgType::Bool, "Uncovered texels transparent."))
            .flag(f("uvRange", "uvr", ArgType::List, "u0 u1 v0 v1.").multi(4))
            .flag(f("force", "f", ArgType::Bool, "Write over the file."))
            .flag(f("name", "n", ArgType::String, "The file node's name."))
            .flag(f("doubleSided", "ds", ArgType::Bool, "Normals facing away from the camera are turned round, so the side it sees is sampled."))
            .flag(f("camera", "cam", ArgType::String, "The camera for -doubleSided (persp)."))
            .flag(f("componentRange", "cr", ArgType::Bool, "With faces named: the image covers their UV extent."))
            .flag(f("fullUvRange", "fur", ArgType::Bool, "The image covers the surfaces' whole UV extent; a placement maps it back."))
            .flag(f("pixelFormat", "pf", ArgType::Any, "8 or 16 bits a channel (8); 16 where the format takes it (png, tif)."))
            .flag(f("reuseDepthMap", "rdm", ArgType::Bool, "Accepted: no shadows are baked, so there are no depth maps."))
            .flag(f("samplePlane", "sp", ArgType::Bool, "Sample a flat plane whose UVs span -samplePlaneRange rather than the surfaces."))
            .flag(f("samplePlaneRange", "spr", ArgType::List, "u0 u1 v0 v1 for -samplePlane (0 1 0 1).").multi(4))
            .flag(f("uvBBoxIntersect", "ubi", ArgType::Bool, "Obsolete: has no effect."))
            .flag(f("uvSetName", "uv", ArgType::String, "The UV set sampled; the image is named surface-set."))
            .flag(f("shadows", "sh", ArgType::Bool, "Accepted: no lighting is baked."))
            .flag(f("noFileNode", "nfn", ArgType::Bool, "Aztek: only the image, no file node.")),
    );
    c.register(
        Cmd::new("psdTextureFile", "Write a Photoshop file for painting, as the reference's: -psf the file, -xr -yr its size (1024), -chs a layer set for each channel name, -ifn image layerSet fills one from an image, -ssi a UV snapshot image as a layer over them (-uvt false: under them). Returns the file.", psd_texture_file)
            .flag(f("psdFileName", "psf", ArgType::String, "The PSD to write."))
            .flag(f("xResolution", "xr", ArgType::Int, "Width."))
            .flag(f("yResolution", "yr", ArgType::Int, "Height."))
            .flag(f("channels", "chs", ArgType::List, "A layer set: its name (then whether it has alpha, accepted). Repeat it.").multi(1))
            .flag(f("imageFileName", "ifn", ArgType::List, "image layerSet [index]: an image in a layer set. Repeat it.").multi(2))
            .flag(f("snapShotImageName", "ssi", ArgType::String, "The UV snapshot's image: its black goes transparent."))
            .flag(f("uvSnapPostionTop", "uvt", ArgType::Bool, "The snapshot over the sets (the default) or under them.")),
    );
    c.register(
        Cmd::new("psdExport", "Read a Photoshop file out to an image: -psf the PSD, -ofn the image, -lsn the layer set (all of it without). Without -ofn it lists the layer sets, top first.", psd_export)
            .flag(f("psdFileName", "psf", ArgType::String, "The PSD."))
            .flag(f("outFileName", "ofn", ArgType::String, "The image to write."))
            .flag(f("layerSetName", "lsn", ArgType::String, "The layer set."))
            .flag(f("format", "fmt", ArgType::String, "Accepted: the type is the name's extension."))
            .undoable(false),
    );
    c.register(
        Cmd::new("createPsdNetwork", "UV Editor > Image > Create PSD Network: a PSD for the material named (or the selected mesh's) with a layer set for each of -attributes (color without it), filled with the attribute as it is now over the meshes' UVs, the UV snapshot over them (-uvSnapshot false: none), and a psdFileTex reading each set in the attribute's place (a bump through bump2d). -psf the file (the material's name in sourceimages without it), -xr -yr its size (1024). Returns the file and the nodes.", create_psd_network)
            .positional("material")
            .flag(f("psdFileName", "psf", ArgType::String, "The PSD to write."))
            .flag(f("xResolution", "xr", ArgType::Int, "Width."))
            .flag(f("yResolution", "yr", ArgType::Int, "Height."))
            .flag(f("attributes", "at", ArgType::List, "The material's attributes, a layer set each.").multi(1))
            .flag(f("uvSnapshot", "uvs", ArgType::Bool, "The UV snapshot layer on top.")),
    );
    c.register(
        Cmd::new("updatePsdNetworks", "UV Editor > Image > Update PSD Networks: the UV snapshot layer of the PSDs the psdFileTex nodes named (all of them without) drawn again from the UVs as they are now, the painted layers kept. The textures show the file again. Returns the files written.", update_psd_networks)
            .positional("psdFileTex nodes"),
    );
}
