//! vectorize: the reference's vector renderer. The visible meshes and NURBS surfaces are seen through a camera, their
//! faces filled flat or with gradients by the fill style and drawn back to front, with outlines, crease edges or every
//! edge on top by the edge style. Written as SVG (animated, optionally compressed), EPS, Illustrator, SWF (one shape
//! per face, a frame per frame rendered), or rasterised into PNG, JPEG, TIFF, Targa, BMP or SGI images.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec2, DVec3};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::harold::network::get_f;
use crate::mesh_nodes::MeshNode;
use crate::plug::NodeId;
use crate::session::Session;
use crate::surfaces::SurfaceShape;
use crate::transform::DagNode;

type Cmd = CommandDef<Session>;

const VECTOR_FORMATS: &[&str] = &["swf", "svg", "eps", "ai"];
const RASTER_FORMATS: &[&str] = &["png", "jpg", "tga", "tif", "bmp", "sgi"];
/// Named by the reference's documentation; not written here.
const OTHER_FORMATS: &[&str] = &["iff", "als", "cin", "gif", "yuv", "rla", "si", "pntg", "ps", "pict", "qtif", "qt"];
const FILL_STYLES: &[&str] = &["SingleColor", "TwoColor", "FourColor", "FullColor", "AverageColor", "AreaGradient", "MeshGradient", "None"];
const EDGE_STYLES: &[&str] = &["Outline", "EntireMesh", "None"];
const FILENAME_FORMATS: &[&str] = &["name", "name.ext", "name.#.ext", "name.ext.#", "name.#", "name#.ext", "name_#.ext"];

#[derive(Clone)]
struct Opts {
    format: String,
    width: u32,
    height: u32,
    par: f64,
    fill: String,
    edge_style: String,
    edge_color: [u8; 3],
    edge_weight: f64,
    edge_detail: bool,
    min_edge_angle: f64,
    hidden_edges: bool,
    back_faces: bool,
    highlights: bool,
    highlight_level: u32,
    detail: u32,
    optimization: String,
    frame_rate: f64,
    flash_version: u8,
    combine: bool,
    svg_animation: String,
    svg_compression: bool,
}

/// One face as seen: its points on the image (y down), depth, colours at its points and a fill colour.
struct Face {
    pts: Vec<DVec2>,
    depth: f64,
    fill: Option<[f64; 3]>,
    /// Per point colours, for the gradient fills.
    point_colors: Vec<[f64; 3]>,
    /// Edges to draw with it: pairs of indices into pts.
    edges: Vec<(usize, usize)>,
}

struct Frame {
    faces: Vec<Face>,
    /// Edges drawn over everything (hidden edges).
    over: Vec<(DVec2, DVec2)>,
}

struct View {
    world_to_cam: DMat4,
    ortho: bool,
    sx: f64,
    sy: f64,
    w: f64,
    h: f64,
    eye: DVec3,
}

impl View {
    fn new(g: &mut Graph, cam: NodeId, o: &Opts) -> View {
        let world = g.node(cam).map(|n| n.data().matrix(DagNode::WORLD_MATRIX)).unwrap_or_default();
        let (_, r, t) = world.to_scale_rotation_translation();
        let m = DMat4::from_rotation_translation(r, t);
        let f = get_f(g, cam, "focalLength", 35.0);
        let hap = get_f(g, cam, "horizontalFilmAperture", 1.41732) * 25.4;
        let vap = get_f(g, cam, "verticalFilmAperture", 0.94488) * 25.4;
        let fit = get_f(g, cam, "filmFit", 1.0) as i64;
        let ortho = get_f(g, cam, "orthographic", 0.0) != 0.0;
        let ortho_width = get_f(g, cam, "orthographicWidth", 30.0);
        let image_aspect = o.width as f64 * o.par / o.height.max(1) as f64;
        let film_aspect = hap / vap.max(1e-9);
        // Fill fits the film inside the image the long way, Overscan the other; 1 horizontal, 2 vertical.
        let horizontal = match fit {
            0 => image_aspect >= film_aspect,
            2 => false,
            3 => image_aspect < film_aspect,
            _ => true,
        };
        let (sx, sy) = if ortho {
            let k = 2.0 / ortho_width.max(1e-9);
            (k, k * image_aspect)
        } else if horizontal {
            let k = 2.0 * f / hap;
            (k, k * image_aspect)
        } else {
            let k = 2.0 * f / vap;
            (k / image_aspect, k)
        };
        View { world_to_cam: m.inverse(), ortho, sx, sy, w: o.width as f64, h: o.height as f64, eye: t }
    }

    /// Image position (pixels, y down) and depth; None behind the camera.
    fn project(&self, p: DVec3) -> Option<(DVec2, f64)> {
        let c = self.world_to_cam.transform_point3(p);
        let d = -c.z;
        let (x, y) = if self.ortho {
            (c.x * self.sx, c.y * self.sy)
        } else {
            if d < 1e-3 {
                return None;
            }
            (c.x / d * self.sx, c.y / d * self.sy)
        };
        Some((DVec2::new((x + 1.0) * 0.5 * self.w, (1.0 - y) * 0.5 * self.h), d))
    }

    fn view_dir(&self, p: DVec3) -> DVec3 {
        if self.ortho {
            let inv = self.world_to_cam.inverse();
            -inv.transform_vector3(DVec3::NEG_Z).normalize_or_zero()
        } else {
            (self.eye - p).normalize_or_zero()
        }
    }
}

fn shade(base: [f64; 3], n: DVec3, to_eye: DVec3, o: &Opts, specular: bool) -> [f64; 3] {
    // A light at the eye: half lit at grazing, fully facing the camera fully lit.
    let ndl = n.dot(to_eye).abs();
    let mut k = 0.25 + 0.75 * ndl;
    match o.fill.as_str() {
        "TwoColor" => k = if ndl > 0.5 { 1.0 } else { 0.5 },
        "FourColor" => k = 0.25 + 0.75 * ((ndl * 4.0).floor().min(3.0) / 3.0),
        _ => {}
    }
    let mut c = base.map(|x| x * k);
    if specular {
        let s = ndl.powf(30.0);
        let levels = o.highlight_level.clamp(1, 8) as f64;
        let s = (s * levels).floor() / levels;
        c = c.map(|x| x + s * (1.0 - x));
    }
    // Lower detail levels share colours between neighbours: fewer distinct fills.
    if o.detail < 50 {
        let steps = (o.detail as f64 * 5.0 + 5.0).max(2.0);
        c = c.map(|x| (x * steps).round() / steps);
    }
    c.map(|x| x.clamp(0.0, 1.0))
}

fn shape_geometry(g: &Graph, id: NodeId) -> Option<(std::sync::Arc<aztek_geo::Mesh>, DMat4)> {
    let n = g.node(id)?;
    let world = n.data().matrix(DagNode::WORLD_MATRIX);
    if n.ty().is_a("nurbsSurface") {
        return Some((n.data().mesh(SurfaceShape::OUT_MESH).cloned()?, world));
    }
    Some((n.data().mesh(MeshNode::OUT_MESH).cloned()?, world))
}

fn render_frame(s: &mut Session, view: &View, o: &Opts) -> Frame {
    let shapes = crate::harold::render_shapes(&mut s.graph);
    let mut faces = Vec::new();
    let mut over = Vec::new();
    let min_area = match o.optimization.as_str() {
        "Good" => 0.5,
        "Aggressive" => 2.0,
        _ => 1e-9,
    };
    let crease_cos = o.min_edge_angle.to_radians().cos();
    for shape in shapes {
        let g = &mut s.graph;
        let Some((mesh, world)) = shape_geometry(g, shape) else { continue };
        let groups = crate::shading::membership_of(g, shape);
        let face_group = crate::shading::face_groups(&mesh, &groups);
        let colors: Vec<[f64; 3]> = groups.iter().map(|(sg, _)| crate::shading::draw_material(g, *sg, None, None).map_or([0.5; 3], |d| d.color.map(|x| x as f64))).collect();
        let nmat = world.inverse().transpose();
        // Edges and the faces on each side.
        let mut edge_faces: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
        let mut normals: HashMap<u32, DVec3> = HashMap::new();
        for f in 0..mesh.face_capacity() {
            if !mesh.face_valid(f) {
                continue;
            }
            let pts = mesh.face_points(f);
            for i in 0..pts.len() {
                let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                edge_faces.entry((a.min(b), a.max(b))).or_default().push(f);
            }
            normals.insert(f, nmat.transform_vector3(mesh.face_normal(f)).normalize_or_zero());
        }
        let world_pts: Vec<DVec3> = (0..mesh.point_capacity()).map(|p| if mesh.point_valid(p) { world.transform_point3(mesh.position(p)) } else { DVec3::ZERO }).collect();
        let front = |f: u32| -> bool {
            let pts = mesh.face_points(f);
            let c = pts.iter().map(|&p| world_pts[p as usize]).sum::<DVec3>() / pts.len().max(1) as f64;
            normals[&f].dot(view.view_dir(c)) > 0.0
        };
        let fronts: HashMap<u32, bool> = normals.keys().map(|&f| (f, front(f))).collect();
        // The point normals, for the mesh gradient.
        let mut point_n: HashMap<u32, DVec3> = HashMap::new();
        for (&f, &n) in &normals {
            for p in mesh.face_points(f) {
                *point_n.entry(p).or_insert(DVec3::ZERO) += n;
            }
        }
        let mut mine: Vec<Face> = Vec::new();
        for f in 0..mesh.face_capacity() {
            if !mesh.face_valid(f) {
                continue;
            }
            let pts = mesh.face_points(f);
            let n = normals[&f];
            let is_front = fronts[&f];
            if !is_front && !o.back_faces {
                // Its edges may still be wanted as hidden edges.
                if o.hidden_edges {
                    for i in 0..pts.len() {
                        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                        if let (Some((pa, _)), Some((pb, _))) = (view.project(world_pts[a as usize]), view.project(world_pts[b as usize])) {
                            over.push((pa, pb));
                        }
                    }
                }
                continue;
            }
            let projected: Option<Vec<(DVec2, f64)>> = pts.iter().map(|&p| view.project(world_pts[p as usize])).collect();
            let Some(projected) = projected else { continue };
            let p2: Vec<DVec2> = projected.iter().map(|x| x.0).collect();
            // Shoelace area: the render optimisation leaves out what is too small to see.
            let area = (0..p2.len()).map(|i| p2[i].perp_dot(p2[(i + 1) % p2.len()])).sum::<f64>().abs() * 0.5;
            if area < min_area {
                continue;
            }
            let depth = projected.iter().map(|x| x.1).sum::<f64>() / projected.len() as f64;
            let base = face_group.get(f as usize).and_then(|&k| colors.get(k)).copied().unwrap_or([0.5; 3]);
            let centre = pts.iter().map(|&p| world_pts[p as usize]).sum::<DVec3>() / pts.len() as f64;
            let spec_ok = o.highlights && matches!(o.fill.as_str(), "SingleColor" | "AverageColor" | "AreaGradient");
            let fill = (o.fill != "None").then(|| shade(base, n, view.view_dir(centre), o, spec_ok));
            let point_colors: Vec<[f64; 3]> = pts.iter().map(|&p| shade(base, point_n.get(&p).copied().unwrap_or(n).normalize_or_zero(), view.view_dir(world_pts[p as usize]), o, spec_ok)).collect();
            let mut edges = Vec::new();
            for i in 0..pts.len() {
                let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                let others: Vec<u32> = edge_faces.get(&(a.min(b), a.max(b))).map(|v| v.iter().copied().filter(|&x| x != f).collect()).unwrap_or_default();
                let silhouette = others.is_empty() || others.iter().any(|x| !fronts[x]);
                let crease = o.edge_detail && others.iter().any(|x| normals[x].dot(n) < crease_cos);
                let take = match o.edge_style.as_str() {
                    "EntireMesh" => true,
                    "Outline" => silhouette || crease,
                    _ => crease,
                };
                // Each inner edge once: by the face drawn later, which is the nearer.
                if take {
                    edges.push((i, (i + 1) % pts.len()));
                }
            }
            mine.push(Face { pts: p2, depth, fill, point_colors, edges });
        }
        if o.fill == "AverageColor" && !mine.is_empty() {
            let mut sum = [0.0; 3];
            let mut k = 0.0;
            for fc in &mine {
                if let Some(c) = fc.fill {
                    for i in 0..3 {
                        sum[i] += c[i];
                    }
                    k += 1.0;
                }
            }
            if k > 0.0 {
                let avg = sum.map(|x| x / k);
                for fc in &mut mine {
                    fc.fill = Some(avg);
                }
            }
        }
        faces.extend(mine);
    }
    // Back to front.
    faces.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));
    Frame { faces, over }
}

// ---------------------------------------------------------------------------------------------------------------------
// Writers

fn hex(c: [f64; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", (c[0] * 255.0).round() as u8, (c[1] * 255.0).round() as u8, (c[2] * 255.0).round() as u8)
}

fn edge_hex(c: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

fn gradient_ends(f: &Face) -> (DVec2, DVec2, [f64; 3], [f64; 3]) {
    let lum = |c: &[f64; 3]| c[0] * 0.3 + c[1] * 0.59 + c[2] * 0.11;
    let (mut lo, mut hi) = (0, 0);
    for i in 0..f.point_colors.len() {
        if lum(&f.point_colors[i]) < lum(&f.point_colors[lo]) {
            lo = i;
        }
        if lum(&f.point_colors[i]) > lum(&f.point_colors[hi]) {
            hi = i;
        }
    }
    (f.pts[lo], f.pts[hi], f.point_colors[lo], f.point_colors[hi])
}

fn svg_frame(out: &mut String, fr: &Frame, o: &Opts, grad: &mut usize) {
    let sw = if o.edge_weight <= 0.0 { 1.0 } else { o.edge_weight };
    let ec = edge_hex(o.edge_color);
    for f in &fr.faces {
        let pts: String = f.pts.iter().map(|p| format!("{:.2},{:.2}", p.x, p.y)).collect::<Vec<_>>().join(" ");
        if let Some(c) = f.fill {
            let gradient = matches!(o.fill.as_str(), "AreaGradient" | "MeshGradient");
            if gradient {
                let (a, b, ca, cb) = gradient_ends(f);
                *grad += 1;
                let _ = writeln!(out, "<linearGradient id=\"g{}\" gradientUnits=\"userSpaceOnUse\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"><stop offset=\"0\" stop-color=\"{}\"/><stop offset=\"1\" stop-color=\"{}\"/></linearGradient>", grad, a.x, a.y, b.x, b.y, hex(ca), hex(cb));
                let _ = writeln!(out, "<polygon points=\"{pts}\" fill=\"url(#g{})\" stroke=\"url(#g{})\" stroke-width=\"0.5\"/>", grad, grad);
            } else {
                // A hair of stroke in the fill's colour closes the seams between neighbours.
                let _ = writeln!(out, "<polygon points=\"{pts}\" fill=\"{0}\" stroke=\"{0}\" stroke-width=\"0.5\"/>", hex(c));
            }
        }
        for &(i, j) in &f.edges {
            let _ = writeln!(out, "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{ec}\" stroke-width=\"{sw}\" stroke-linecap=\"round\"/>", f.pts[i].x, f.pts[i].y, f.pts[j].x, f.pts[j].y);
        }
    }
    for (a, b) in &fr.over {
        let _ = writeln!(out, "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{ec}\" stroke-width=\"{sw}\" stroke-dasharray=\"3,2\"/>", a.x, a.y, b.x, b.y);
    }
}

fn write_svg(frames: &[Frame], o: &Opts) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(out, "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">", o.width, o.height, o.width, o.height);
    let mut grad = 0;
    if frames.len() == 1 {
        svg_frame(&mut out, &frames[0], o, &mut grad);
    } else {
        let dt = 1.0 / o.frame_rate.max(1e-3);
        let script = o.svg_animation.eq_ignore_ascii_case("HTMLScript");
        for (i, fr) in frames.iter().enumerate() {
            let shown = if i == 0 { "inline" } else { "none" };
            let _ = writeln!(out, "<g id=\"frame{i}\" display=\"{shown}\">");
            if !script {
                // Shown for its frame's time, the last one kept.
                let fill = if i + 1 == frames.len() { "freeze" } else { "remove" };
                if i > 0 {
                    let _ = writeln!(out, "<set attributeName=\"display\" to=\"inline\" begin=\"{:.4}s\" dur=\"{:.4}s\" fill=\"{fill}\"/>", i as f64 * dt, dt);
                } else {
                    let _ = writeln!(out, "<set attributeName=\"display\" to=\"none\" begin=\"{:.4}s\" fill=\"freeze\"/>", dt);
                }
            }
            svg_frame(&mut out, fr, o, &mut grad);
            let _ = writeln!(out, "</g>");
        }
        if script {
            let _ = writeln!(
                out,
                "<script type=\"text/ecmascript\"><![CDATA[\nvar frame = 0, frames = {};\nsetInterval(function () {{\n  document.getElementById('frame' + frame).setAttribute('display', 'none');\n  frame = (frame + 1) % frames;\n  document.getElementById('frame' + frame).setAttribute('display', 'inline');\n}}, {});\n]]></script>",
                frames.len(),
                (dt * 1000.0).round() as i64
            );
        }
    }
    let _ = writeln!(out, "</svg>");
    out
}

/// PostScript (EPS) or Illustrator: y up, in points, one page.
fn write_ps(fr: &Frame, o: &Opts, illustrator: bool) -> String {
    let h = o.height as f64;
    let mut out = String::new();
    if illustrator {
        let _ = writeln!(out, "%!PS-Adobe-3.0 EPSF-3.0\n%%Creator: Aztek vectorize\n%%AI5_FileFormat 3\n%%BoundingBox: 0 0 {} {}\n%%EndComments\n%%EndProlog\n%%BeginSetup\n%%EndSetup", o.width, o.height);
    } else {
        let _ = writeln!(out, "%!PS-Adobe-3.0 EPSF-3.0\n%%Creator: Aztek vectorize\n%%BoundingBox: 0 0 {} {}\n%%Pages: 1\n%%EndComments\n%%Page: 1 1\n1 setlinejoin 1 setlinecap", o.width, o.height);
    }
    let pt = |p: DVec2| (p.x, h - p.y);
    let lw = o.edge_weight.max(0.0);
    let ec = o.edge_color.map(|c| c as f64 / 255.0);
    for f in &fr.faces {
        // Gradients are not available here: a gradient fill is drawn in its mean colour.
        let fill = f.fill.map(|c| if matches!(o.fill.as_str(), "AreaGradient" | "MeshGradient") { let (_, _, a, b) = gradient_ends(f); [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5] } else { c });
        if let Some(c) = fill {
            let (x, y) = pt(f.pts[0]);
            if illustrator {
                let _ = writeln!(out, "{:.4} {:.4} {:.4} Xa", c[0], c[1], c[2]);
                let _ = writeln!(out, "{x:.2} {y:.2} m");
                for p in &f.pts[1..] {
                    let (x, y) = pt(*p);
                    let _ = writeln!(out, "{x:.2} {y:.2} L");
                }
                let _ = writeln!(out, "f");
            } else {
                let _ = write!(out, "{:.4} {:.4} {:.4} setrgbcolor newpath {x:.2} {y:.2} moveto", c[0], c[1], c[2]);
                for p in &f.pts[1..] {
                    let (x, y) = pt(*p);
                    let _ = write!(out, " {x:.2} {y:.2} lineto");
                }
                let _ = writeln!(out, " closepath fill");
            }
        }
        for &(i, j) in &f.edges {
            let ((x0, y0), (x1, y1)) = (pt(f.pts[i]), pt(f.pts[j]));
            if illustrator {
                let _ = writeln!(out, "{:.4} {:.4} {:.4} XA {lw:.2} w\n{x0:.2} {y0:.2} m\n{x1:.2} {y1:.2} L\nS", ec[0], ec[1], ec[2]);
            } else {
                let _ = writeln!(out, "{:.4} {:.4} {:.4} setrgbcolor {lw:.2} setlinewidth newpath {x0:.2} {y0:.2} moveto {x1:.2} {y1:.2} lineto stroke", ec[0], ec[1], ec[2]);
            }
        }
    }
    for (a, b) in &fr.over {
        let ((x0, y0), (x1, y1)) = (pt(*a), pt(*b));
        if illustrator {
            let _ = writeln!(out, "{:.4} {:.4} {:.4} XA {lw:.2} w\n{x0:.2} {y0:.2} m\n{x1:.2} {y1:.2} L\nS", ec[0], ec[1], ec[2]);
        } else {
            let _ = writeln!(out, "{:.4} {:.4} {:.4} setrgbcolor {lw:.2} setlinewidth [3 2] 0 setdash newpath {x0:.2} {y0:.2} moveto {x1:.2} {y1:.2} lineto stroke [] 0 setdash", ec[0], ec[1], ec[2]);
        }
    }
    if illustrator {
        let _ = writeln!(out, "%%PageTrailer\n%%Trailer\n%%EOF");
    } else {
        let _ = writeln!(out, "showpage\n%%EOF");
    }
    out
}

// SWF -----------------------------------------------------------------------------------------------------------------

#[derive(Default)]
struct Bits {
    out: Vec<u8>,
    cur: u32,
    n: u32,
}

impl Bits {
    fn put(&mut self, v: u32, bits: u32) {
        for i in (0..bits).rev() {
            self.cur = (self.cur << 1) | ((v >> i) & 1);
            self.n += 1;
            if self.n == 8 {
                self.out.push(self.cur as u8);
                self.cur = 0;
                self.n = 0;
            }
        }
    }
    fn put_signed(&mut self, v: i32, bits: u32) {
        self.put((v as u32) & if bits >= 32 { u32::MAX } else { (1u32 << bits) - 1 }, bits);
    }
    fn flush(&mut self) {
        if self.n > 0 {
            let pad = 8 - self.n;
            self.put(0, pad);
        }
    }
    fn bytes(mut self) -> Vec<u8> {
        self.flush();
        self.out
    }
}

fn signed_bits(v: i32) -> u32 {
    let a = if v < 0 { !v } else { v } as u32;
    33 - a.leading_zeros().min(32)
}

fn rect(b: &mut Bits, x0: i32, x1: i32, y0: i32, y1: i32) {
    let n = [x0, x1, y0, y1].iter().map(|&v| signed_bits(v)).max().unwrap_or(1).max(1);
    b.put(n, 5);
    for v in [x0, x1, y0, y1] {
        b.put_signed(v, n);
    }
}

fn tag(out: &mut Vec<u8>, code: u16, body: &[u8]) {
    if body.len() < 63 {
        out.extend(((code << 6) | body.len() as u16).to_le_bytes());
    } else {
        out.extend(((code << 6) | 0x3f).to_le_bytes());
        out.extend((body.len() as u32).to_le_bytes());
    }
    out.extend(body);
}

/// A DefineShape2 of one closed polygon (filled or not) and lines.
fn swf_shape(id: u16, poly: &[(i32, i32)], fill: Option<[u8; 3]>, lines: &[((i32, i32), (i32, i32))], line: Option<(u16, [u8; 3])>) -> Vec<u8> {
    let all: Vec<(i32, i32)> = poly.iter().copied().chain(lines.iter().flat_map(|(a, b)| [*a, *b])).collect();
    let pad = line.map_or(0, |l| l.0 as i32);
    let (x0, x1) = (all.iter().map(|p| p.0).min().unwrap_or(0) - pad, all.iter().map(|p| p.0).max().unwrap_or(0) + pad);
    let (y0, y1) = (all.iter().map(|p| p.1).min().unwrap_or(0) - pad, all.iter().map(|p| p.1).max().unwrap_or(0) + pad);
    let mut body = Vec::new();
    body.extend(id.to_le_bytes());
    let mut b = Bits::default();
    rect(&mut b, x0, x1, y0, y1);
    body.extend(b.bytes());
    // Fill styles.
    match fill {
        Some(c) => {
            body.push(1);
            body.push(0x00);
            body.extend(c);
        }
        None => body.push(0),
    }
    match line {
        Some((w, c)) => {
            body.push(1);
            body.extend(w.to_le_bytes());
            body.extend(c);
        }
        None => body.push(0),
    }
    let mut b = Bits::default();
    b.put(1, 4);
    b.put(1, 4);
    let edge = |b: &mut Bits, dx: i32, dy: i32| {
        let n = signed_bits(dx).max(signed_bits(dy)).max(2);
        b.put(1, 1);
        b.put(1, 1);
        b.put(n - 2, 4);
        b.put(1, 1);
        b.put_signed(dx, n);
        b.put_signed(dy, n);
    };
    let move_to = |b: &mut Bits, p: (i32, i32), fill0: Option<u32>, line_style: Option<u32>| {
        b.put(0, 1);
        b.put(0, 1); // new styles
        b.put(line_style.is_some() as u32, 1);
        b.put(0, 1); // fill style 1
        b.put(fill0.is_some() as u32, 1);
        b.put(1, 1); // move to
        let n = signed_bits(p.0).max(signed_bits(p.1)).max(1);
        b.put(n, 5);
        b.put_signed(p.0, n);
        b.put_signed(p.1, n);
        if let Some(f) = fill0 {
            b.put(f, 1);
        }
        if let Some(l) = line_style {
            b.put(l, 1);
        }
    };
    if fill.is_some() && poly.len() >= 3 {
        move_to(&mut b, poly[0], Some(1), Some(0));
        let mut at = poly[0];
        for &p in poly[1..].iter().chain(std::iter::once(&poly[0])) {
            edge(&mut b, p.0 - at.0, p.1 - at.1);
            at = p;
        }
    }
    if line.is_some() {
        let mut first = true;
        for (a, c) in lines {
            move_to(&mut b, *a, if first { Some(0) } else { None }, if first { Some(1) } else { None });
            first = false;
            if a != c {
                edge(&mut b, c.0 - a.0, c.1 - a.1);
            }
        }
    }
    b.put(0, 6);
    body.extend(b.bytes());
    body
}

fn write_swf(frames: &[Frame], o: &Opts) -> Vec<u8> {
    let tw = |v: f64| (v * 20.0).round() as i32;
    let mut tags = Vec::new();
    tag(&mut tags, 9, &[255, 255, 255]);
    let mut next_id: u16 = 1;
    let mut placed: u16 = 0;
    let edge_line = ((o.edge_weight.max(0.05) * 20.0).round() as u16, o.edge_color);
    for fr in frames {
        for d in 1..=placed {
            tag(&mut tags, 28, &d.to_le_bytes());
        }
        let mut depth: u16 = 0;
        let place = |tags: &mut Vec<u8>, id: u16, depth: &mut u16| {
            *depth += 1;
            let mut body = vec![0x02u8];
            body.extend(depth.to_le_bytes());
            body.extend(id.to_le_bytes());
            tag(tags, 26, &body);
        };
        for f in &fr.faces {
            let poly: Vec<(i32, i32)> = f.pts.iter().map(|p| (tw(p.x), tw(p.y))).collect();
            let lines: Vec<((i32, i32), (i32, i32))> = f.edges.iter().map(|&(i, j)| (poly[i], poly[j])).collect();
            let fill = f.fill.map(|c| c.map(|x| (x * 255.0).round() as u8));
            if o.combine || lines.is_empty() {
                let id = next_id;
                next_id = next_id.wrapping_add(1);
                tag(&mut tags, 22, &swf_shape(id, &poly, fill, &lines, (!lines.is_empty()).then_some(edge_line)));
                place(&mut tags, id, &mut depth);
            } else {
                let id = next_id;
                next_id = next_id.wrapping_add(1);
                tag(&mut tags, 22, &swf_shape(id, &poly, fill, &[], None));
                place(&mut tags, id, &mut depth);
                let id = next_id;
                next_id = next_id.wrapping_add(1);
                tag(&mut tags, 22, &swf_shape(id, &[], None, &lines, Some(edge_line)));
                place(&mut tags, id, &mut depth);
            }
        }
        if !fr.over.is_empty() {
            let lines: Vec<((i32, i32), (i32, i32))> = fr.over.iter().map(|(a, b)| ((tw(a.x), tw(a.y)), (tw(b.x), tw(b.y)))).collect();
            let id = next_id;
            next_id = next_id.wrapping_add(1);
            tag(&mut tags, 22, &swf_shape(id, &[], None, &lines, Some(edge_line)));
            place(&mut tags, id, &mut depth);
        }
        placed = depth;
        tag(&mut tags, 1, &[]);
    }
    tag(&mut tags, 0, &[]);
    let mut head = Bits::default();
    rect(&mut head, 0, tw(o.width as f64), 0, tw(o.height as f64));
    let mut rest = head.bytes();
    let rate = (o.frame_rate.clamp(1.0, 120.0) * 256.0).round() as u16;
    rest.extend(rate.to_le_bytes());
    rest.extend((frames.len() as u16).to_le_bytes());
    rest.extend(tags);
    let mut out = b"FWS".to_vec();
    out.push(o.flash_version);
    out.extend(((8 + rest.len()) as u32).to_le_bytes());
    out.extend(rest);
    out
}

// Raster --------------------------------------------------------------------------------------------------------------

fn raster(fr: &Frame, o: &Opts, background: [f64; 3]) -> Vec<[f64; 3]> {
    let (w, h) = (o.width as usize, o.height as usize);
    let mut px = vec![background; w * h];
    let gradient = matches!(o.fill.as_str(), "AreaGradient" | "MeshGradient");
    let line = |px: &mut Vec<[f64; 3]>, a: DVec2, b: DVec2, width: f64, c: [f64; 3]| {
        let r = (width.max(1.0)) * 0.5;
        let (x0, x1) = ((a.x.min(b.x) - r).floor().max(0.0) as usize, ((a.x.max(b.x) + r).ceil() as usize).min(w));
        let (y0, y1) = ((a.y.min(b.y) - r).floor().max(0.0) as usize, ((a.y.max(b.y) + r).ceil() as usize).min(h));
        let d = b - a;
        let len2 = d.length_squared().max(1e-12);
        for y in y0..y1 {
            for x in x0..x1 {
                let p = DVec2::new(x as f64 + 0.5, y as f64 + 0.5);
                let t = ((p - a).dot(d) / len2).clamp(0.0, 1.0);
                if (a + d * t - p).length() <= r {
                    px[y * w + x] = c;
                }
            }
        }
    };
    let ec = o.edge_color.map(|c| c as f64 / 255.0);
    for f in &fr.faces {
        if let Some(c) = f.fill {
            let n = f.pts.len();
            let (x0, x1) = (f.pts.iter().map(|p| p.x).fold(f64::MAX, f64::min).floor().max(0.0) as usize, (f.pts.iter().map(|p| p.x).fold(f64::MIN, f64::max).ceil().max(0.0) as usize).min(w));
            let (y0, y1) = (f.pts.iter().map(|p| p.y).fold(f64::MAX, f64::min).floor().max(0.0) as usize, (f.pts.iter().map(|p| p.y).fold(f64::MIN, f64::max).ceil().max(0.0) as usize).min(h));
            let ends = gradient.then(|| gradient_ends(f));
            for y in y0..y1 {
                for x in x0..x1 {
                    let p = DVec2::new(x as f64 + 0.5, y as f64 + 0.5);
                    // Even-odd crossing test.
                    let mut inside = false;
                    for i in 0..n {
                        let (a, b) = (f.pts[i], f.pts[(i + 1) % n]);
                        if (a.y > p.y) != (b.y > p.y) && p.x < a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x) {
                            inside = !inside;
                        }
                    }
                    if !inside {
                        continue;
                    }
                    px[y * w + x] = match ends {
                        Some((a, b, ca, cb)) => {
                            let d = b - a;
                            let t = if d.length_squared() > 1e-12 { ((p - a).dot(d) / d.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
                            [ca[0] + (cb[0] - ca[0]) * t, ca[1] + (cb[1] - ca[1]) * t, ca[2] + (cb[2] - ca[2]) * t]
                        }
                        None => c,
                    };
                }
            }
        }
        for &(i, j) in &f.edges {
            line(&mut px, f.pts[i], f.pts[j], o.edge_weight, ec);
        }
    }
    for (a, b) in &fr.over {
        line(&mut px, *a, *b, o.edge_weight, ec);
    }
    px
}

fn write_raster(path: &Path, px: &[[f64; 3]], o: &Opts) -> Result<(), String> {
    let rgb: Vec<u8> = px.iter().flat_map(|c| c.map(|x| (x.clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
    let (w, h) = (o.width, o.height);
    let fmt = match o.format.as_str() {
        "png" => image::ImageFormat::Png,
        "jpg" => image::ImageFormat::Jpeg,
        "tga" => image::ImageFormat::Tga,
        "tif" => image::ImageFormat::Tiff,
        "bmp" => image::ImageFormat::Bmp,
        "sgi" => {
            // SGI image: a 512 byte header, then each channel's rows bottom to top, uncompressed.
            let mut out = vec![0u8; 512];
            out[0..2].copy_from_slice(&474u16.to_be_bytes());
            out[2] = 0;
            out[3] = 1;
            out[4..6].copy_from_slice(&3u16.to_be_bytes());
            out[6..8].copy_from_slice(&(w as u16).to_be_bytes());
            out[8..10].copy_from_slice(&(h as u16).to_be_bytes());
            out[10..12].copy_from_slice(&3u16.to_be_bytes());
            out[16..20].copy_from_slice(&255u32.to_be_bytes());
            for c in 0..3 {
                for y in (0..h as usize).rev() {
                    for x in 0..w as usize {
                        out.push(rgb[(y * w as usize + x) * 3 + c]);
                    }
                }
            }
            return std::fs::write(path, out).map_err(|e| e.to_string());
        }
        other => return Err(format!("the {other} format isn't written here")),
    };
    image::save_buffer_with_format(path, &rgb, w, h, image::ColorType::Rgb8, fmt).map_err(|e| e.to_string())
}

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
        }
    }
    !c
}

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 255];
    out.extend(miniz_oxide::deflate::compress_to_vec(data, 9));
    out.extend(crc32(data).to_le_bytes());
    out.extend((data.len() as u32).to_le_bytes());
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// The command

fn frame_text(f: f64) -> String {
    if (f - f.round()).abs() < 1e-9 {
        format!("{}", f.round() as i64)
    } else {
        format!("{f}")
    }
}

/// The file name for a frame, by -filenameFormat.
fn file_name(name: &str, ext: &str, fmt: &str, frame: Option<f64>) -> String {
    let n = frame.map(frame_text).unwrap_or_default();
    match (fmt, frame) {
        ("name", _) => name.to_string(),
        ("name.ext", _) | (_, None) => format!("{name}.{ext}"),
        ("name.#.ext", _) => format!("{name}.{n}.{ext}"),
        ("name.ext.#", _) => format!("{name}.{ext}.{n}"),
        ("name.#", _) => format!("{name}.{n}"),
        ("name#.ext", _) => format!("{name}{n}.{ext}"),
        ("name_#.ext", _) => format!("{name}_{n}.{ext}"),
        _ => format!("{name}.{ext}"),
    }
}

fn pick(a: &Args, flag: &str, list: &[&str], default: &str) -> CmdResult<String> {
    let Some(v) = a.get(flag).and_then(Value::as_str) else { return Ok(default.to_string()) };
    match list.iter().find(|x| x.eq_ignore_ascii_case(v)) {
        Some(x) => Ok(x.to_string()),
        None => fail!("vectorize: -{flag} takes one of {}", list.join(", ")),
    }
}

fn vectorize(s: &mut Session, a: &Args) -> CmdResult {
    let format = a.get("imageFormat").and_then(Value::as_str).unwrap_or("swf").to_ascii_lowercase();
    let format = match format.as_str() {
        "tiff" => "tif".to_string(),
        "jpeg" => "jpg".to_string(),
        f => f.to_string(),
    };
    if OTHER_FORMATS.contains(&format.as_str()) {
        fail!("vectorize: the {format} image format isn't written here: use swf, svg, eps, ai, png, jpg, tga, tif, bmp or sgi");
    }
    if !VECTOR_FORMATS.contains(&format.as_str()) && !RASTER_FORMATS.contains(&format.as_str()) {
        fail!("vectorize: unknown image format {format}");
    }
    let g = &mut s.graph;
    let res = g.find("defaultResolution").ok().flatten();
    let res_f = |g: &mut Graph, k: &str, d: f64| res.map_or(d, |r| get_f(g, r, k, d));
    let width = a.get("width").and_then(Value::as_i64).map_or_else(|| res_f(g, "width", 640.0) as i64, |v| v).max(1) as u32;
    let height = a.get("height").and_then(Value::as_i64).map_or_else(|| res_f(g, "height", 480.0) as i64, |v| v).max(1) as u32;
    let par = a.get("pixelAspectRatio").and_then(Value::as_f64).unwrap_or_else(|| res_f(g, "pixelAspect", 1.0)).max(1e-6);
    let fill = pick(a, "fillStyle", FILL_STYLES, "SingleColor")?;
    let edge_style = pick(a, "edgeStyle", EDGE_STYLES, "Outline")?;
    let edge_color = match a.get("edgeColor").and_then(Value::as_array) {
        Some(v) if v.len() == 3 => [0, 1, 2].map(|i| v[i].as_f64().unwrap_or(0.0).clamp(0.0, 255.0) as u8),
        Some(_) => fail!("vectorize: -edgeColor takes red, green and blue, 0 to 255"),
        None => [0, 0, 0],
    };
    let flash_version = a.get("flashVersion").and_then(Value::as_i64).unwrap_or(5);
    if !(3..=5).contains(&flash_version) {
        fail!("vectorize: -flashVersion is 3, 4 or 5");
    }
    let svg_animation = pick(a, "svgAnimation", &["Native", "HTMLScript"], "Native")?;
    let optimization = pick(a, "renderOptimization", &["Safe", "Good", "Aggressive"], "Safe")?;
    let fmt_name = pick(a, "filenameFormat", FILENAME_FORMATS, "")?;
    let o = Opts {
        format: format.clone(),
        width,
        height,
        par,
        fill,
        edge_style,
        edge_color,
        edge_weight: a.float("edgeWeight", 1.0).max(0.0),
        edge_detail: a.bool("edgeDetail"),
        min_edge_angle: a.float("minEdgeAngle", 30.0).clamp(0.0, 90.0),
        hidden_edges: a.bool("hiddenEdges"),
        back_faces: a.bool("showBackFaces"),
        highlights: a.bool("highlights"),
        highlight_level: a.int("highlightLevel", 4).clamp(1, 8) as u32,
        detail: a.int("detailLevel", 20).clamp(0, 50) as u32,
        optimization,
        frame_rate: a.get("frameRate").and_then(Value::as_f64).unwrap_or_else(crate::anim::fps),
        flash_version: flash_version as u8,
        combine: a.bool("combineFillsEdges"),
        svg_animation,
        svg_compression: a.bool("svgCompression"),
    };
    // The camera: named, else the one Harold renders through.
    let cam = match a.get("camera").and_then(Value::as_str) {
        Some(n) => {
            let Some(id) = g.find(n)? else { fail!("vectorize: No object matches name: {n}") };
            let Some(c) = crate::cameras::shape_of(g, id) else { fail!("vectorize: {n} is not a camera") };
            c
        }
        None => match crate::harold::find_camera(g, None) {
            Some(c) => c,
            None => fail!("vectorize: no camera to render through"),
        },
    };
    let background = {
        let b = get_f(g, cam, "backgroundColorR", 0.0);
        [b, get_f(g, cam, "backgroundColorG", 0.0), get_f(g, cam, "backgroundColorB", 0.0)]
    };
    // Frames: -currentFrame, or -startFrame to -endFrame by -byFrame; the current frame when none is given.
    let now = crate::anim::frame(g);
    let frames: Vec<f64> = if a.bool("currentFrame") || (!a.has("startFrame") && !a.has("endFrame")) {
        vec![now]
    } else {
        let start = a.get("startFrame").and_then(Value::as_f64).unwrap_or(now);
        let end = a.get("endFrame").and_then(Value::as_f64).unwrap_or(start);
        let by = a.float("byFrame", 1.0).abs().max(1e-3);
        let mut v = Vec::new();
        let mut t = start;
        while t <= end + 1e-9 && v.len() < 100_000 {
            v.push(t);
            t += by;
        }
        v
    };
    // Layers: one named, every renderable one, or the scene as it is.
    let layer_names: Vec<Option<String>> = if let Some(l) = a.get("layer").and_then(Value::as_str) {
        let ok = g.find(l).ok().flatten().is_some_and(|id| g.node(id).is_some_and(|n| n.ty().name == "renderLayer"));
        if !ok {
            fail!("vectorize: {l} is not a render layer");
        }
        vec![Some(l.to_string())]
    } else if a.bool("renderLayers") {
        let layers: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().name == "renderLayer")).collect();
        let mut v: Vec<Option<String>> = Vec::new();
        for id in layers {
            if get_f(g, id, "renderable", 1.0) != 0.0 {
                v.push(Some(g.path(id)));
            }
        }
        if v.is_empty() {
            v.push(None);
        }
        v
    } else {
        vec![None]
    };
    let scene = s.run("file", json!({"query": true, "sceneName": true})).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
    let default_name = Path::new(&scene).file_stem().and_then(|x| x.to_str()).filter(|x| !x.is_empty()).unwrap_or("untitled").to_string();
    let base = a.get("outputFileName").and_then(Value::as_str).filter(|x| !x.is_empty()).map(str::to_string).unwrap_or(default_name);
    let ext = match a.get("customExtension").and_then(Value::as_str).filter(|x| !x.is_empty()) {
        Some(e) => e.trim_start_matches('.').to_string(),
        None if format == "svg" && o.svg_compression => "svgz".to_string(),
        None => format.clone(),
    };
    let animated_file = matches!(format.as_str(), "swf" | "svg");
    let fmt_name = if !fmt_name.is_empty() {
        fmt_name
    } else if frames.len() > 1 && !animated_file {
        "name.#.ext".to_string()
    } else {
        "name.ext".to_string()
    };
    let mut written = Vec::new();
    let manager = s.graph.find("renderLayerManager").ok().flatten();
    for layer in &layer_names {
        // The layer named becomes the current one; its name goes on the file's.
        if let (Some(l), Some(m)) = (layer, manager) {
            let lid = s.graph.find(l).ok().flatten().map_or(0.0, |id| get_f(&mut s.graph, id, "identification", 0.0));
            let _ = s.run("setAttr", json!({"args": [format!("{}.currentRenderLayer", s.graph.path(m)), lid]}));
        }
        let stem = match layer {
            Some(l) if layer_names.len() > 1 || a.has("layer") => format!("{base}_{}", l.rsplit('|').next().unwrap_or(l)),
            _ => base.clone(),
        };
        let mut rendered = Vec::new();
        for &t in &frames {
            if (crate::anim::frame(&s.graph) - t).abs() > 1e-9 {
                s.run("currentTime", json!({"args": [t]}))?;
            }
            s.graph.evaluate();
            let view = View::new(&mut s.graph, cam, &o);
            let fr = render_frame(s, &view, &o);
            if animated_file {
                rendered.push(fr);
                continue;
            }
            let file = PathBuf::from(file_name(&stem, &ext, &fmt_name, (frames.len() > 1).then_some(t)));
            if let Some(d) = file.parent().filter(|d| !d.as_os_str().is_empty()) {
                let _ = std::fs::create_dir_all(d);
            }
            let r = match format.as_str() {
                "eps" => std::fs::write(&file, write_ps(&fr, &o, false)).map_err(|e| e.to_string()),
                "ai" => std::fs::write(&file, write_ps(&fr, &o, true)).map_err(|e| e.to_string()),
                _ => write_raster(&file, &raster(&fr, &o, background), &o),
            };
            if let Err(e) = r {
                fail!("vectorize: {}: {e}", file.display());
            }
            written.push(file.display().to_string());
        }
        if animated_file {
            let file = PathBuf::from(file_name(&stem, &ext, &fmt_name, None));
            if let Some(d) = file.parent().filter(|d| !d.as_os_str().is_empty()) {
                let _ = std::fs::create_dir_all(d);
            }
            let bytes = if format == "swf" {
                write_swf(&rendered, &o)
            } else {
                let text = write_svg(&rendered, &o);
                if o.svg_compression {
                    gzip(text.as_bytes())
                } else {
                    text.into_bytes()
                }
            };
            if let Err(e) = std::fs::write(&file, bytes) {
                fail!("vectorize: {}: {e}", file.display());
            }
            written.push(file.display().to_string());
        }
    }
    if (crate::anim::frame(&s.graph) - now).abs() > 1e-9 {
        s.run("currentTime", json!({"args": [now]}))?;
    }
    // -browserView opens a Flash file in the web browser; -renderView shows the image in the Render View.
    if let Some(last) = written.last().cloned() {
        let abs = std::fs::canonicalize(&last).map(|p| p.display().to_string()).unwrap_or(last);
        if a.bool("browserView") && format == "swf" && s.commands.find("launch").is_some() {
            let _ = s.run("launch", json!({"webPage": format!("file://{abs}")}));
        }
        if a.bool("renderView") && RASTER_FORMATS.contains(&format.as_str()) && s.commands.find("renderWindowEditor").is_some() {
            let _ = s.run("renderWindowEditor", json!({"edit": true, "loadImage": abs, "args": ["renderView"]}));
        }
    }
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    use ArgType::{Bool as B, Float as F, Int as I, String as S};
    c.register(
        Cmd::new(
            "vectorize",
            "Renders the scene with the vector renderer: the visible meshes and surfaces through a camera, faces filled by -fillStyle and edges drawn by -edgeStyle, \
             written as SWF, SVG, EPS or Illustrator, or rasterised into an image. Not undoable.",
            vectorize,
        )
        .undoable(false)
        .flag(f("browserView", "bv", B, "Opens the SWF file in the web browser when done."))
        .flag(f("byFrame", "bf", F, "The frame step (1)."))
        .flag(f("camera", "c", S, "The camera to render through (the renderable camera)."))
        .flag(f("combineFillsEdges", "cfe", B, "SWF: fills and edges in one shape each face."))
        .flag(f("currentFrame", "cf", B, "Renders only the current frame."))
        .flag(f("curveTolerance", "ct", F, "Curve fitting, 0 (none) to 15 (aggressive). Edges are kept straight here."))
        .flag(f("customExtension", "ce", S, "The file name's extension instead of the format's."))
        .flag(f("detailLevel", "dl", I, "0 to 50: lower shares colours between more faces (20)."))
        .flag(f("edgeColor", "ec", ArgType::Vec3, "Edge colour, red green blue 0 to 255 (black)."))
        .flag(f("edgeDetail", "ed", B, "Draws edges where neighbouring faces meet at more than -minEdgeAngle."))
        .flag(f("edgeStyle", "es", S, "Outline, EntireMesh or None (Outline).").choices(EDGE_STYLES))
        .flag(f("edgeWeight", "ew", F, "Edge width in points, 72 to the inch; 0 a hairline (1)."))
        .flag(f("endFrame", "ef", F, "The last frame (the start frame)."))
        .flag(f("filenameFormat", "ff", S, "name, name.ext, name.#.ext, name.ext.#, name.#, name#.ext or name_#.ext.").choices(FILENAME_FORMATS))
        .flag(f("fillStyle", "fs", S, "SingleColor, TwoColor, FourColor, FullColor, AverageColor, AreaGradient, MeshGradient or None (SingleColor).").choices(FILL_STYLES))
        .flag(f("flashVersion", "fv", I, "SWF: the Flash version, 3, 4 or 5 (5)."))
        .flag(f("frameRate", "fr", I, "SVG and SWF: frames a second (the scene's)."))
        .flag(f("height", "h", I, "Image height in pixels (defaultResolution's)."))
        .flag(f("hiddenEdges", "he", B, "Also draws the edges the camera can't see, over everything."))
        .flag(f("highlightLevel", "hl", I, "Rings a highlight is drawn in, 1 to 8 (4); SingleColor, AverageColor and AreaGradient."))
        .flag(f("highlights", "hi", B, "Draws highlights; SingleColor, AverageColor and AreaGradient."))
        .flag(f("imageFormat", "if", S, "swf, svg, eps, ai, png, jpg, tga, tif, bmp or sgi (swf)."))
        .flag(f("layer", "l", S, "Renders this render layer, which becomes the current one; its name goes on the file's."))
        .flag(f("minEdgeAngle", "mea", F, "With -edgeDetail: the angle in degrees, 0 to 90, between faces that draws their edge (30)."))
        .flag(f("outlinesAtIntersections", "oai", B, "Edges where polygons pass through each other (on)."))
        .flag(f("outputFileName", "of", S, "The output file's name, without extension (the scene's)."))
        .flag(f("pixelAspectRatio", "par", F, "The pixel aspect ratio (defaultResolution's)."))
        .flag(f("reflectionDepth", "rd", I, "Reflection levels, 1 to 4; no effect for ai, eps or svg."))
        .flag(f("reflections", "rf", B, "Renders reflections; no effect for ai, eps or svg."))
        .flag(f("renderLayers", "rl", B, "Renders each renderable render layer into its own file."))
        .flag(f("renderOptimization", "ro", S, "Safe drops degenerate faces, Good also sub-pixel ones, Aggressive also faces about a pixel big (Safe).").choices(&["Safe", "Good", "Aggressive"]))
        .flag(f("renderView", "rv", B, "Shows the image in the Render View."))
        .flag(f("secondaryCurveFitting", "scf", B, "Secondary curve fitting."))
        .flag(f("shadows", "sh", B, "Renders shadows; no effect for ai, eps or svg."))
        .flag(f("showBackFaces", "sb", B, "Renders faces turned away from the camera."))
        .flag(f("startFrame", "sf", F, "The first frame (the current frame)."))
        .flag(f("svgAnimation", "sa", S, "SVG: Native (SMIL) or HTMLScript (a script) animation.").choices(&["Native", "HTMLScript"]))
        .flag(f("svgCompression", "sc", B, "SVG: gzip compressed (.svgz)."))
        .flag(f("width", "w", I, "Image width in pixels (defaultResolution's).")),
    );
}
