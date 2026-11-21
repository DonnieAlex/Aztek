//! The rest of what a skin does in the reference: heat map binding, deformerWeights export and import, Hammer Skin Weights,
//! transferring weight between influences, geometry influences, Substitute Geometry, Remove Unused Influences.

use std::collections::HashMap;

use aztek_geo::bvh::MeshBvh;
use aztek_geo::Mesh;
use aztek_ops::{execute, fail, objects, ArgType, Args, CallSource, CmdResult, Commands};
use glam::{DMat4, DVec3};
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::deform::{self, f, floats_of, set_floats, Cmd, WEIGHTS};
use crate::graph::Graph;
use crate::plug::NodeId;
use crate::session::Session;
use crate::skin::{bind_of, commit_paint, influences, normalize_point, selected_points, skin_geometry, skin_named, SkinBind};

// ---------------------------------------------------------------------------------------------------------------------
// Heat map bind

/// Solves A x = b for the mesh's Laplacian plus a diagonal, by conjugate gradients with the diagonal as preconditioner.
fn solve(adj: &[Vec<(u32, f64)>], diag: &[f64], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let apply = |x: &[f64]| -> Vec<f64> { (0..n).map(|i| diag[i] * x[i] - adj[i].iter().map(|&(j, w)| w * x[j as usize]).sum::<f64>()).collect() };
    let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>();
    let mut x = vec![0.0; n];
    let mut r = b.to_vec();
    let mut z: Vec<f64> = r.iter().zip(diag).map(|(a, d)| a / d.max(1e-12)).collect();
    let mut p = z.clone();
    let mut rz = dot(&r, &z);
    let goal = dot(b, b).sqrt() * 1e-7;
    for _ in 0..(n * 4).clamp(64, 4000) {
        if dot(&r, &r).sqrt() <= goal {
            break;
        }
        let ap = apply(&p);
        let denom = dot(&p, &ap);
        if denom.abs() < 1e-30 {
            break;
        }
        let alpha = rz / denom;
        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        z = r.iter().zip(diag).map(|(a, d)| a / d.max(1e-12)).collect();
        let next = dot(&r, &z);
        let beta = next / rz.max(1e-30);
        rz = next;
        for i in 0..n {
            p[i] = z[i] + beta * p[i];
        }
    }
    x
}

fn nearest_on_bone(p: DVec3, a: DVec3, b: DVec3) -> DVec3 {
    let ab = b - a;
    let t = if ab.length_squared() < 1e-12 { 0.0 } else { ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) };
    a + ab * t
}

/// Heat map weights: each influence heats the points that see it nearest, and the heat spreads over the mesh's edges to
/// equilibrium, (L + H) w = H p. Because it spreads along the surface, a point takes weight from the bone inside its own
/// part of the mesh, even with another part's bone nearer across a gap. `falloff` from 1 (as solved) down to 0 tightens the
/// result; points go to their strongest `max_inf` and sum to 1.
pub(crate) fn heat_weights(m: &Mesh, geom: &DMat4, bones: &[(DVec3, DVec3)], max_inf: usize, falloff: f64) -> Vec<Vec<(u32, f64)>> {
    let cap = m.point_capacity() as usize;
    let mut out = vec![Vec::new(); cap];
    if bones.is_empty() {
        return out;
    }
    let world_mesh = {
        let mut w = m.clone();
        for p in 0..w.point_capacity() {
            if w.point_valid(p) {
                w.set_position(p, geom.transform_point3(m.position(p)));
            }
        }
        w
    };
    let bvh = MeshBvh::new(&world_mesh);
    let pts: Vec<DVec3> = world_mesh.positions().to_vec();
    let mut adj: Vec<Vec<(u32, f64)>> = vec![Vec::new(); cap];
    let mut laplace = vec![0.0; cap];
    let (mut total_len, mut edges) = (0.0, 0.0);
    for p in 0..cap as u32 {
        if !m.point_valid(p) {
            continue;
        }
        m.for_each_point_edge(p, |e| {
            let q = m.edge_other_point(e, p);
            let len = (pts[p as usize] - pts[q as usize]).length().max(1e-9);
            let w = 1.0 / len;
            adj[p as usize].push((q, w));
            laplace[p as usize] += w;
            total_len += len;
            edges += 1.0;
        });
    }
    let mean_len = if edges > 0.0 { total_len / edges } else { 1.0 };
    let mean_diag = laplace.iter().sum::<f64>() / cap.max(1) as f64;
    // Each point's heat, from its nearest bones it can see.
    let heat: Vec<(f64, Vec<usize>)> = (0..cap as u32)
        .into_par_iter()
        .map(|p| {
            if !m.point_valid(p) {
                return (0.0, Vec::new());
            }
            let at = pts[p as usize];
            let mut by: Vec<(usize, f64, DVec3)> = bones
                .iter()
                .enumerate()
                .map(|(i, (a, b))| {
                    let q = nearest_on_bone(at, *a, *b);
                    (i, (q - at).length(), q)
                })
                .collect();
            by.sort_by(|a, b| a.1.total_cmp(&b.1));
            let seen = by.iter().find(|(_, d, q)| {
                if *d < 1e-9 {
                    return true;
                }
                // A touch off the exact line, which on a regular mesh can run along its edges and through its points.
                let aim = *q + DVec3::new(0.317, 0.571, 0.757) * (*d * 1e-3);
                let dir = (aim - at).normalize_or_zero();
                let start = at + dir * (*d * 1e-3).min(mean_len * 1e-2);
                !bvh.occluded(start, dir, (aim - start).length())
            });
            match seen {
                Some(&(_, d, _)) => {
                    let near: Vec<usize> = by.iter().filter(|x| (x.1 - d).abs() <= d * 1e-4 + 1e-9).map(|x| x.0).collect();
                    (mean_diag * (mean_len / d.max(mean_len * 0.05)).powi(2), near)
                }
                // Hidden from every bone: it takes its weight from around it alone.
                None => (0.0, Vec::new()),
            }
        })
        .collect();
    let diag: Vec<f64> = (0..cap).map(|i| (laplace[i] + heat[i].0).max(1e-9)).collect();
    let solved: Vec<Vec<f64>> = (0..bones.len())
        .into_par_iter()
        .map(|j| {
            let b: Vec<f64> = heat.iter().map(|(h, near)| if near.contains(&j) { h / near.len() as f64 } else { 0.0 }).collect();
            solve(&adj, &diag, &b)
        })
        .collect();
    let power = 1.0 + 2.0 * (1.0 - falloff.clamp(0.0, 1.0));
    for p in 0..cap {
        if !m.point_valid(p as u32) {
            continue;
        }
        let mut w: Vec<(u32, f64)> = solved.iter().enumerate().map(|(j, s)| (j as u32, s[p].max(0.0).powf(power))).filter(|x| x.1 > 1e-6).collect();
        w.sort_by(|a, b| b.1.total_cmp(&a.1));
        w.truncate(max_inf.max(1));
        let sum: f64 = w.iter().map(|x| x.1).sum();
        if sum <= 1e-9 {
            // Nothing reached it, a shell no bone can see: the nearest bone has it.
            let at = pts[p];
            let j = (0..bones.len()).min_by(|&a, &b| (nearest_on_bone(at, bones[a].0, bones[a].1) - at).length_squared().total_cmp(&(nearest_on_bone(at, bones[b].0, bones[b].1) - at).length_squared())).unwrap_or(0);
            out[p] = vec![(j as u32, 1.0)];
            continue;
        }
        for x in w.iter_mut() {
            x.1 /= sum;
        }
        w.sort_by_key(|x| x.0);
        out[p] = w;
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// deformerWeights

#[derive(Default, Debug, Clone)]
struct WeightLayer {
    deformer: String,
    source: String,
    shape: String,
    default: f64,
    points: Vec<(usize, f64)>,
}

#[derive(Default, Debug)]
struct WeightDoc {
    /// Point positions in the shape's own space, and the shape's world matrix when written.
    shape_points: Vec<(usize, DVec3)>,
    world: Option<DMat4>,
    /// The faces, as point indices, when written with -vertexConnections.
    faces: Vec<Vec<usize>>,
    layers: Vec<WeightLayer>,
    /// Deformer attributes written with -attribute: (deformer, attribute, value).
    attrs: Vec<(String, String, Vec<f64>)>,
}

fn numbers(text: &str) -> Vec<f64> {
    text.split_whitespace().filter_map(|x| x.parse().ok()).collect()
}

fn matrix_of(v: &[f64]) -> Option<DMat4> {
    (v.len() == 16).then(|| DMat4::from_cols_array(&v.try_into().unwrap()))
}

/// One tag's attributes.
fn xml_attrs(tag: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut rest = tag;
    while let Some(eq) = rest.find('=') {
        let key = rest[..eq].trim().rsplit(char::is_whitespace).next().unwrap_or("").to_string();
        let after = &rest[eq + 1..];
        let Some(open) = after.find(['"', '\'']) else { break };
        let quote = after.as_bytes()[open] as char;
        let Some(close) = after[open + 1..].find(quote) else { break };
        out.insert(key, after[open + 1..open + 1 + close].to_string());
        rest = &after[open + close + 2..];
    }
    out
}

fn parse_xml(text: &str) -> CmdResult<WeightDoc> {
    if !text.contains("<deformerWeight") {
        fail!("deformerWeights: not a deformer weights file");
    }
    let mut doc = WeightDoc::default();
    let mut in_shape = false;
    for raw in text.split('<').skip(1) {
        let tag = raw.split('>').next().unwrap_or("");
        let name = tag.split_whitespace().next().unwrap_or("");
        let attrs = xml_attrs(tag);
        match name {
            "headerInfo" => doc.world = attrs.get("worldMatrix").and_then(|m| matrix_of(&numbers(m))),
            "face" => doc.faces.push(numbers(attrs.get("value").map_or("", String::as_str)).into_iter().map(|x| x as usize).collect()),
            "attribute" => doc.attrs.push((attrs.get("deformer").cloned().unwrap_or_default(), attrs.get("name").cloned().unwrap_or_default(), numbers(attrs.get("value").map_or("", String::as_str)))),
            "shape" => in_shape = true,
            "/shape" => in_shape = false,
            "weights" => {
                in_shape = false;
                doc.layers.push(WeightLayer {
                    deformer: attrs.get("deformer").cloned().unwrap_or_default(),
                    source: attrs.get("source").cloned().unwrap_or_default(),
                    shape: attrs.get("shape").cloned().unwrap_or_default(),
                    default: attrs.get("defaultValue").and_then(|x| x.parse().ok()).unwrap_or(0.0),
                    points: Vec::new(),
                });
            }
            "point" => {
                let index = attrs.get("index").and_then(|x| x.parse().ok()).unwrap_or(0usize);
                let value = attrs.get("value").cloned().unwrap_or_default();
                if in_shape {
                    let v: Vec<f64> = value.split_whitespace().filter_map(|x| x.parse().ok()).collect();
                    if v.len() >= 3 {
                        doc.shape_points.push((index, DVec3::new(v[0], v[1], v[2])));
                    }
                } else if let Some(l) = doc.layers.last_mut() {
                    l.points.push((index, value.trim().parse().unwrap_or(0.0)));
                }
            }
            _ => {}
        }
    }
    Ok(doc)
}

fn parse_json(text: &str) -> CmdResult<WeightDoc> {
    let v: Value = serde_json::from_str(text).map_err(|e| aztek_ops::CommandError::new(format!("deformerWeights: {e}")))?;
    let Some(root) = v.get("deformerWeight") else { fail!("deformerWeights: not a deformer weights file") };
    let mut doc = WeightDoc::default();
    let nums = |v: Option<&Value>| -> Vec<f64> { v.and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default() };
    doc.world = matrix_of(&nums(root.get("headerInfo").and_then(|h| h.get("worldMatrix"))));
    for at in root.get("attributes").and_then(Value::as_array).into_iter().flatten() {
        let s = |k: &str| at.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let value = match at.get("value") {
            Some(Value::Array(_)) => nums(at.get("value")),
            Some(v) => v.as_f64().into_iter().collect(),
            None => Vec::new(),
        };
        doc.attrs.push((s("deformer"), s("name"), value));
    }
    for sh in root.get("shapes").and_then(Value::as_array).into_iter().flatten() {
        for fc in sh.get("faces").and_then(Value::as_array).into_iter().flatten() {
            doc.faces.push(nums(Some(fc)).into_iter().map(|x| x as usize).collect());
        }
        for p in sh.get("points").and_then(Value::as_array).into_iter().flatten() {
            let c = |k: usize| p.get("value").and_then(|x| x.get(k)).and_then(Value::as_f64).unwrap_or(0.0);
            doc.shape_points.push((p.get("index").and_then(Value::as_u64).unwrap_or(0) as usize, DVec3::new(c(0), c(1), c(2))));
        }
    }
    for w in root.get("weights").and_then(Value::as_array).into_iter().flatten() {
        let s = |k: &str| w.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        doc.layers.push(WeightLayer {
            deformer: s("deformer"),
            source: s("source"),
            shape: s("shape"),
            default: w.get("defaultValue").and_then(Value::as_f64).unwrap_or(0.0),
            points: w.get("points").and_then(Value::as_array).into_iter().flatten().map(|p| (p.get("index").and_then(Value::as_u64).unwrap_or(0) as usize, p.get("value").and_then(Value::as_f64).unwrap_or(0.0))).collect(),
        });
    }
    Ok(doc)
}

/// The layers a deformer has: each influence of a skin, the weights of anything else.
fn layers_of(g: &Graph, n: NodeId, tolerance: f64, default: f64) -> CmdResult<(NodeId, Vec<WeightLayer>)> {
    let name = g.node(n).map(|x| x.name().to_string()).unwrap_or_default();
    if let Some(b) = g.node(n).filter(|x| x.ty().name == crate::skin::SKIN_TYPE).and_then(|_| bind_of(g, n)) {
        let Some(mesh) = skin_geometry(g, n) else { fail!("deformerWeights: {name} has no geometry") };
        let shape = g.node(mesh).map(|x| x.name().to_string()).unwrap_or_default();
        let mut out = Vec::new();
        for (i, j) in influences(g, n) {
            let points: Vec<(usize, f64)> = b.weights.iter().enumerate().filter_map(|(p, w)| w.iter().find(|x| x.0 == i as u32).map(|x| (p, x.1))).filter(|x| x.1 > tolerance).collect();
            out.push(WeightLayer { deformer: name.clone(), source: g.node(j).map(|x| x.name().to_string()).unwrap_or_default(), shape: shape.clone(), default, points });
        }
        return Ok((mesh, out));
    }
    let Some(mesh) = deform::mesh_fed_by(g, n) else { fail!("deformerWeights: {name} has no geometry") };
    let shape = g.node(mesh).map(|x| x.name().to_string()).unwrap_or_default();
    let points: Vec<(usize, f64)> = match g.node(n).and_then(|x| floats_of(x.data(), WEIGHTS)) {
        Some(w) => w.0.iter().copied().enumerate().filter(|x| x.1 > tolerance).collect(),
        None => g.node(n).and_then(|x| x.data().mesh(deform::IN)).map(|m| (0..m.point_capacity() as usize).filter(|&p| m.point_valid(p as u32)).map(|p| (p, 1.0)).collect()).unwrap_or_default(),
    };
    Ok((mesh, vec![WeightLayer { deformer: name.clone(), source: name, shape, default, points }]))
}

/// What else goes in a file beside the weights: faces with -vertexConnections, attributes with -attribute.
#[derive(Default)]
struct Extras {
    faces: Vec<Vec<u32>>,
    attrs: Vec<(String, String, Vec<f64>)>,
}

fn write_xml(file: &str, shape: &str, wm: &DMat4, pts: &[(usize, DVec3)], layers: &[WeightLayer], precision: usize, extra: &Extras) -> String {
    let matrix: Vec<String> = wm.to_cols_array().iter().map(|x| format!("{x}")).collect();
    let mut s = String::from("<?xml version=\"1.0\"?>\n<deformerWeight>\n");
    s += &format!("  <headerInfo fileName=\"{file}\" worldMatrix=\"{}\"/>\n", matrix.join(" "));
    let max = pts.last().map_or(0, |x| x.0);
    s += &format!("  <shape name=\"{shape}\" group=\"0\" stride=\"3\" size=\"{}\" max=\"{max}\">\n", pts.len());
    for (i, p) in pts {
        s += &format!("    <point index=\"{i}\" value=\" {:.*} {:.*} {:.*}\"/>\n", precision, p.x, precision, p.y, precision, p.z);
    }
    for (k, face) in extra.faces.iter().enumerate() {
        let ids: Vec<String> = face.iter().map(u32::to_string).collect();
        s += &format!("    <face index=\"{k}\" value=\"{}\"/>\n", ids.join(" "));
    }
    s += "  </shape>\n";
    for (d, name, v) in &extra.attrs {
        let vals: Vec<String> = v.iter().map(|x| format!("{x}")).collect();
        s += &format!("  <attribute deformer=\"{d}\" name=\"{name}\" value=\"{}\"/>\n", vals.join(" "));
    }
    for l in layers {
        let max = l.points.last().map_or(0, |x| x.0);
        s += &format!("  <weights deformer=\"{}\" source=\"{}\" shape=\"{}\" layer=\"0\" defaultValue=\"{:.*}\" size=\"{}\" max=\"{max}\">\n", l.deformer, l.source, l.shape, precision, l.default, l.points.len());
        for (i, v) in &l.points {
            s += &format!("    <point index=\"{i}\" value=\"{:.*}\"/>\n", precision, v);
        }
        s += "  </weights>\n";
    }
    s += "</deformerWeight>\n";
    s
}

fn write_json(file: &str, shape: &str, wm: &DMat4, pts: &[(usize, DVec3)], layers: &[WeightLayer], extra: &Extras) -> String {
    let weights: Vec<Value> = layers
        .iter()
        .map(|l| {
            json!({
                "deformer": l.deformer, "source": l.source, "shape": l.shape, "layer": 0, "defaultValue": l.default,
                "size": l.points.len(), "max": l.points.last().map_or(0, |x| x.0),
                "points": l.points.iter().map(|(i, v)| json!({"index": i, "value": v})).collect::<Vec<_>>(),
            })
        })
        .collect();
    let doc = json!({"deformerWeight": {
        "headerInfo": {"fileName": file, "worldMatrix": wm.to_cols_array().to_vec()},
        "shapes": [{"name": shape, "group": 0, "stride": 3, "size": pts.len(), "max": pts.last().map_or(0, |x| x.0),
            "points": pts.iter().map(|(i, p)| json!({"index": i, "value": [p.x, p.y, p.z]})).collect::<Vec<_>>(),
            "faces": extra.faces}],
        "weights": weights,
        "attributes": extra.attrs.iter().map(|(d, n, v)| json!({"deformer": d, "name": n, "value": v})).collect::<Vec<_>>(),
    }});
    serde_json::to_string_pretty(&doc).unwrap_or_default()
}

/// Barycentric shares of the nearest point on a triangle.
fn barycentric(p: DVec3, a: DVec3, b: DVec3, c: DVec3) -> (DVec3, [f64; 3]) {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d1, d2) = (ab.dot(ap), ac.dot(ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return (a, [1.0, 0.0, 0.0]);
    }
    let bp = p - b;
    let (d3, d4) = (ab.dot(bp), ac.dot(bp));
    if d3 >= 0.0 && d4 <= d3 {
        return (b, [0.0, 1.0, 0.0]);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (a + ab * v, [1.0 - v, v, 0.0]);
    }
    let cp = p - c;
    let (d5, d6) = (ab.dot(cp), ac.dot(cp));
    if d6 >= 0.0 && d5 <= d6 {
        return (c, [0.0, 0.0, 1.0]);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (a + ac * w, [1.0 - w, 0.0, w]);
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (b + (c - b) * w, [0.0, 1.0 - w, w]);
    }
    let denom = 1.0 / (va + vb + vc);
    let (v, w) = (vb * denom, vc * denom);
    (a + ab * v + ac * w, [1.0 - v - w, v, w])
}

/// Bilinear shares of the nearest point on a quad a b c d, found by refining (s, t) from its middle.
fn bilinear(p: DVec3, q: [DVec3; 4]) -> (DVec3, [f64; 4]) {
    let at = |s: f64, t: f64| q[0] * (1.0 - s) * (1.0 - t) + q[1] * s * (1.0 - t) + q[2] * s * t + q[3] * (1.0 - s) * t;
    let (mut s, mut t, mut step) = (0.5, 0.5, 0.25);
    for _ in 0..40 {
        let mut best = (s, t, at(s, t).distance(p));
        for (ds, dt) in [(step, 0.0), (-step, 0.0), (0.0, step), (0.0, -step)] {
            let (s2, t2) = ((s + ds).clamp(0.0, 1.0), (t + dt).clamp(0.0, 1.0));
            let d = at(s2, t2).distance(p);
            if d < best.2 {
                best = (s2, t2, d);
            }
        }
        if (best.0, best.1) == (s, t) {
            step *= 0.5;
        }
        (s, t) = (best.0, best.1);
    }
    (at(s, t), [(1.0 - s) * (1.0 - t), s * (1.0 - t), s * t, (1.0 - s) * t])
}

/// How each destination point reads the file's: (stored index, share). The positions compared are the shapes' own,
/// or world space with `world`. Nearest looks no further than `radius`.
fn mapping(m: &Mesh, wm: &DMat4, doc: &WeightDoc, method: &str, world: bool, radius: f64) -> CmdResult<Vec<Vec<(usize, f64)>>> {
    let cap = m.point_capacity() as usize;
    let nearest_count = match method {
        "index" | "over" => 0,
        "nearest" => 1,
        "barycentric" => 3,
        "bilinear" => 4,
        o => fail!("deformerWeights: method {o} is not index, nearest, barycentric, bilinear or over"),
    };
    if nearest_count == 0 {
        return Ok((0..cap).map(|p| vec![(p, 1.0)]).collect());
    }
    if doc.shape_points.is_empty() {
        fail!("deformerWeights: the file has no point positions for -method {method}");
    }
    let file_world = doc.world.unwrap_or(DMat4::IDENTITY);
    let stored: HashMap<usize, DVec3> = doc.shape_points.iter().map(|(i, q)| (*i, if world { file_world.transform_point3(*q) } else { *q })).collect();
    // Faces written with -vertexConnections give barycentric and bilinear their triangles and quads; without them the
    // nearest points share by distance.
    let faces: Vec<Vec<(usize, DVec3)>> = if nearest_count >= 3 {
        doc.faces.iter().filter_map(|f| f.iter().map(|i| stored.get(i).map(|q| (*i, *q))).collect::<Option<Vec<_>>>()).filter(|f| f.len() >= 3).collect()
    } else {
        Vec::new()
    };
    Ok((0..cap as u32)
        .into_par_iter()
        .map(|p| {
            if !m.point_valid(p) {
                return Vec::new();
            }
            let at = if world { wm.transform_point3(m.position(p)) } else { m.position(p) };
            if !faces.is_empty() {
                let mut best: (f64, Vec<(usize, f64)>) = (f64::MAX, Vec::new());
                for f in &faces {
                    if nearest_count == 4 && f.len() == 4 {
                        let (q, w) = bilinear(at, [f[0].1, f[1].1, f[2].1, f[3].1]);
                        let d = q.distance(at);
                        if d < best.0 {
                            best = (d, (0..4).map(|k| (f[k].0, w[k])).collect());
                        }
                        continue;
                    }
                    // Any other face in a fan of triangles.
                    for k in 1..f.len() - 1 {
                        let (q, w) = barycentric(at, f[0].1, f[k].1, f[k + 1].1);
                        let d = q.distance(at);
                        if d < best.0 {
                            best = (d, vec![(f[0].0, w[0]), (f[k].0, w[1]), (f[k + 1].0, w[2])]);
                        }
                    }
                }
                return best.1;
            }
            let mut by: Vec<(usize, f64)> = stored.iter().map(|(i, q)| (*i, (*q - at).length())).collect();
            by.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
            by.truncate(nearest_count);
            if by[0].1 > radius {
                return Vec::new();
            }
            if by[0].1 < 1e-9 {
                return vec![(by[0].0, 1.0)];
            }
            let total: f64 = by.iter().map(|x| 1.0 / x.1).sum();
            by.iter().map(|x| (x.0, 1.0 / x.1 / total)).collect()
        })
        .collect())
}

/// -remap's renames, "pattern;format" each, the format taking $1 to $9.
fn remaps(a: &Args) -> CmdResult<Vec<(regex_lite::Regex, String)>> {
    let list: Vec<String> = match a.get("remap") {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    let mut out = Vec::new();
    for r in list {
        let Some((pat, fmt)) = r.split_once(';') else { fail!("deformerWeights: -remap takes \"pattern;format\"") };
        let re = regex_lite::Regex::new(&format!("^(?:{pat})$")).map_err(|e| aztek_ops::CommandError::new(format!("deformerWeights: -remap {pat}: {e}")))?;
        // The reference's $1 is the regex crate's ${1}, so a letter after it is not read as part of the name.
        let fmt = regex_lite::Regex::new(r"\$(\d)").unwrap().replace_all(fmt, "$${$1}").to_string();
        out.push((re, fmt));
    }
    Ok(out)
}

fn renamed(maps: &[(regex_lite::Regex, String)], name: &str) -> String {
    for (re, fmt) in maps {
        if re.is_match(name) {
            return re.replace(name, fmt.as_str()).to_string();
        }
    }
    name.to_string()
}

/// -skip's patterns: a regular expression, or a glob where it doesn't read as one (*B).
fn skips(a: &Args) -> Vec<String> {
    match a.get("skip") {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

fn skipped(pats: &[String], name: &str) -> bool {
    let short = name.rsplit(['|', ':']).next().unwrap_or(name);
    pats.iter().any(|p| match regex_lite::Regex::new(&format!("^(?:{p})$")) {
        Ok(re) if !p.starts_with('*') => re.is_match(name) || re.is_match(short),
        _ => crate::ref_core::glob(p, name) || crate::ref_core::glob(p, short),
    })
}

/// A deformer's numeric or matrix attribute, as numbers.
fn attr_numbers(g: &mut Graph, n: NodeId, name: &str) -> Option<Vec<f64>> {
    let p = g.plug(&format!("{}.{name}", g.path(n))).ok()?;
    Some(match g.value(&p).ok()? {
        crate::plug::PlugValue::Float(x) => vec![x],
        crate::plug::PlugValue::Int(x) => vec![x as f64],
        crate::plug::PlugValue::Bool(x) => vec![x as i64 as f64],
        crate::plug::PlugValue::Vec3(v) => v.to_array().to_vec(),
        crate::plug::PlugValue::Matrix(m) => m.to_cols_array().to_vec(),
        _ => return None,
    })
}

fn set_attr_numbers(s: &mut Session, n: NodeId, name: &str, v: &[f64]) -> CmdResult<()> {
    let plug_name = format!("{}.{name}", s.graph.path(n));
    let Ok(p) = s.graph.plug(&plug_name) else { return Ok(()) };
    if s.graph.attr(&p).ty == crate::plug::PlugType::Matrix {
        s.graph.set_value(p, crate::plug::PlugValue::Matrix(matrix_of(v).unwrap_or(DMat4::IDENTITY)))?;
        return Ok(());
    }
    let mut args = vec![json!(plug_name)];
    args.extend(v.iter().map(|x| json!(x)));
    s.run("setAttr", json!({ "args": args }))?;
    Ok(())
}

fn deformers_named(s: &Session, a: &Args) -> CmdResult<Vec<NodeId>> {
    let g = &s.graph;
    let mut list: Vec<String> = match a.get("deformer") {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    if list.is_empty() {
        if let Some(sh) = a.get("shape").and_then(Value::as_str) {
            let id = crate::rig::find(g, sh, "deformerWeights")?;
            if let Some(m) = crate::components::as_mesh(g, id) {
                list = crate::history::history_of(g, m).into_iter().filter(|&h| g.node(h).is_some_and(|n| crate::rig::is_rig_chain_node(n.ty().name.as_str()))).map(|h| g.path(h)).collect();
            }
        }
    }
    if list.is_empty() {
        fail!("deformerWeights: give -deformer or -shape");
    }
    list.iter().map(|n| crate::rig::find(g, n, "deformerWeights")).collect()
}

fn names_of(a: &Args, k: &str) -> Vec<String> {
    match a.get(k) {
        Some(Value::String(x)) => vec![x.clone()],
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}

fn deformer_weights(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        // What a call would work on: the deformers (on -shape), their shapes, the attributes asked for.
        let g = &s.graph;
        if crate::skin::asked(a, "shape") {
            let list = deformers_named(s, a)?;
            let shapes: Vec<String> = list.iter().filter_map(|&n| if g.node(n).is_some_and(|x| x.ty().name == crate::skin::SKIN_TYPE) { skin_geometry(g, n) } else { deform::mesh_fed_by(g, n) }).map(|m| g.path(m)).collect();
            return Ok(json!(shapes));
        }
        if crate::skin::asked(a, "deformer") {
            let list = deformers_named(s, a)?;
            return Ok(json!(list.iter().map(|&n| g.path(n)).collect::<Vec<_>>()));
        }
        if crate::skin::asked(a, "path") {
            return Ok(json!(a.string("path", &std::env::current_dir().map(|d| d.to_string_lossy().to_string()).unwrap_or_default())));
        }
        if crate::skin::asked(a, "format") {
            return Ok(json!(a.string("format", "XML")));
        }
        if crate::skin::asked(a, "method") {
            return Ok(json!(a.string("method", "index")));
        }
        if crate::skin::asked(a, "weightPrecision") {
            return Ok(json!(a.int("weightPrecision", 3)));
        }
        if crate::skin::asked(a, "weightTolerance") {
            return Ok(json!(a.float("weightTolerance", 0.001)));
        }
        if crate::skin::asked(a, "attribute") {
            return Ok(json!(names_of(a, "attribute")));
        }
        fail!("deformerWeights: query -deformer, -shape, -path, -format, -method or -attribute");
    }
    let Some(file) = a.named().into_iter().next() else { fail!("deformerWeights: name the file") };
    let path = match a.get("path").and_then(Value::as_str) {
        Some(dir) => std::path::Path::new(dir).join(&file),
        None => std::path::PathBuf::from(&file),
    };
    let json_format = a.get("format").and_then(Value::as_str).map_or_else(|| path.extension().is_some_and(|e| e == "json"), |x| x.eq_ignore_ascii_case("json"));
    let skip = skips(a);
    let maps = remaps(a)?;
    let attributes = names_of(a, "attribute");
    let targets: Vec<NodeId> = deformers_named(s, a)?.into_iter().filter(|&n| !skipped(&skip, &s.graph.path(n))).collect();
    if targets.is_empty() {
        fail!("deformerWeights: every deformer named is skipped");
    }
    let default_value = a.float("defaultValue", 0.0);
    let g = &mut s.graph;
    g.evaluate();
    if a.bool("export") {
        let tolerance = a.float("weightTolerance", 0.0);
        let precision = a.int("weightPrecision", 3).clamp(0, 15) as usize;
        let mut layers = Vec::new();
        let mut shape = None;
        let mut extra = Extras::default();
        for &n in &targets {
            let (mesh, l) = layers_of(g, n, tolerance, default_value)?;
            shape.get_or_insert(mesh);
            layers.extend(l.into_iter().filter(|x| !skipped(&skip, &x.source)));
            let dname = g.node(n).map(|x| x.name().to_string()).unwrap_or_default();
            for at in &attributes {
                if let Some(v) = attr_numbers(g, n, at) {
                    extra.attrs.push((renamed(&maps, &dname), at.clone(), v));
                }
            }
        }
        for l in layers.iter_mut() {
            l.deformer = renamed(&maps, &l.deformer);
            l.source = renamed(&maps, &l.source);
            l.shape = renamed(&maps, &l.shape);
        }
        let mesh = shape.unwrap();
        let Some(m) = crate::modifiers::cage(g, mesh) else { fail!("deformerWeights: no geometry") };
        let xf = crate::dag::transform_of(g, mesh);
        let wm = crate::rig::world(g, xf);
        // Points as they were before deforming, by way of the original where there is one, in the shape's own space.
        let rest = crate::deform_sets::orig_of(g, mesh).and_then(|o| g.value(&crate::deform::plug(o, crate::mesh_nodes::MeshNode::OUT_MESH)).ok()).and_then(|v| v.as_mesh().cloned());
        let from = rest.as_deref().unwrap_or(&m);
        let pts: Vec<(usize, DVec3)> = (0..from.point_capacity()).filter(|&p| from.point_valid(p)).map(|p| (p as usize, from.position(p))).collect();
        if a.bool("vertexConnections") {
            extra.faces = (0..from.face_capacity()).filter(|&f| from.face_valid(f)).map(|f| from.face_points(f)).collect();
        }
        let shape_name = renamed(&maps, &g.node(mesh).map(|x| x.name().to_string()).unwrap_or_default());
        let name = path.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let text = if json_format { write_json(&name, &shape_name, &wm, &pts, &layers, &extra) } else { write_xml(&name, &shape_name, &wm, &pts, &layers, precision, &extra) };
        if let Err(e) = std::fs::write(&path, text) {
            fail!("deformerWeights: can't write {}: {e}", path.display());
        }
        return Ok(json!(path.to_string_lossy()));
    }
    if !a.bool("import") {
        fail!("deformerWeights: give -ex or -im");
    }
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => fail!("deformerWeights: can't read {}: {e}", path.display()),
    };
    let mut doc = if json_format { parse_json(&text)? } else { parse_xml(&text)? };
    // The file's names as the scene has them.
    for l in doc.layers.iter_mut() {
        l.deformer = renamed(&maps, &l.deformer);
        l.source = renamed(&maps, &l.source);
        l.shape = renamed(&maps, &l.shape);
    }
    for at in doc.attrs.iter_mut() {
        at.0 = renamed(&maps, &at.0);
    }
    doc.layers.retain(|l| !skipped(&skip, &l.source) && !skipped(&skip, &l.shape));
    let method = a.string("method", "index");
    let ignore_name = a.bool("ignoreName");
    let world_space = a.bool("worldSpace");
    let radius = a.float("positionTolerance", f64::MAX);
    s.graph.open_chunk("Deformer Weights");
    let r = (|| -> CmdResult<()> {
        for &n in &targets {
            let g = &mut s.graph;
            let name = g.node(n).map(|x| x.name().to_string()).unwrap_or_default();
            let of_this: Vec<&WeightLayer> = doc.layers.iter().filter(|l| ignore_name || l.deformer == name).collect();
            if of_this.is_empty() {
                fail!("deformerWeights: the file has nothing for {name}");
            }
            let is_skin = g.node(n).is_some_and(|x| x.ty().name == crate::skin::SKIN_TYPE);
            let mesh = if is_skin { skin_geometry(g, n) } else { deform::mesh_fed_by(g, n) };
            let Some(mesh) = mesh else { fail!("deformerWeights: {name} has no geometry") };
            let Some(m) = crate::modifiers::cage(g, mesh) else { fail!("deformerWeights: no geometry") };
            let xf = crate::dag::transform_of(g, mesh);
            let wm = crate::rig::world(g, xf);
            let map = mapping(&m, &wm, &doc, &method, world_space, radius)?;
            let over = method == "over";
            let value_at = |l: &WeightLayer, p: usize, lookup: &HashMap<usize, f64>| -> Option<f64> {
                let def = if a.has("defaultValue") { default_value } else { l.default };
                let mut sum = 0.0;
                let mut any = false;
                for &(i, share) in &map[p] {
                    match lookup.get(&i) {
                        Some(v) => {
                            sum += v * share;
                            any = true;
                        }
                        None => sum += def * share,
                    }
                }
                // A point nothing in the file was found near keeps what it has.
                if map[p].is_empty() {
                    return None;
                }
                (any || !over).then_some(sum)
            };
            if is_skin {
                let mut b: SkinBind = (*bind_of(g, n).unwrap_or_default()).clone();
                let list = influences(g, n);
                let known: Vec<(u32, String)> = list.iter().map(|&(i, j)| (i as u32, g.node(j).map(|x| x.name().to_string()).unwrap_or_default())).collect();
                for (k, l) in of_this.iter().enumerate() {
                    let inf = if ignore_name { known.get(k).map(|x| x.0) } else { known.iter().find(|x| x.1 == l.source).map(|x| x.0) };
                    let Some(inf) = inf else { continue };
                    let lookup: HashMap<usize, f64> = l.points.iter().copied().collect();
                    for p in 0..b.weights.len() {
                        let Some(v) = value_at(l, p, &lookup) else { continue };
                        b.weights[p].retain(|x| x.0 != inf);
                        if v > 1e-12 {
                            b.weights[p].push((inf, v));
                            b.weights[p].sort_by_key(|x| x.0);
                        }
                    }
                }
                commit_paint(g, n, b)?;
            } else {
                let l = of_this[0];
                let lookup: HashMap<usize, f64> = l.points.iter().copied().collect();
                let size = m.point_capacity() as usize;
                let mut w = g.node(n).and_then(|x| floats_of(x.data(), WEIGHTS)).map(|x| x.0.clone()).unwrap_or_else(|| vec![1.0; size]);
                w.resize(size, 0.0);
                for (p, slot) in w.iter_mut().enumerate() {
                    if let Some(v) = value_at(l, p, &lookup) {
                        *slot = v;
                    }
                }
                set_floats(g, n, WEIGHTS, w)?;
            }
            let attrs: Vec<(String, Vec<f64>)> = doc.attrs.iter().filter(|x| ignore_name || x.0 == name).map(|x| (x.1.clone(), x.2.clone())).collect();
            for (at, v) in attrs {
                set_attr_numbers(s, n, &at, &v)?;
            }
        }
        Ok(())
    })();
    s.graph.close_chunk();
    r?;
    Ok(json!(path.to_string_lossy()))
}

// ---------------------------------------------------------------------------------------------------------------------
// Editing weights

/// Hammer Skin Weights: each point takes the average of its neighbours outside what is being hammered; a point with none
/// outside takes the average of all of them.
fn hammer(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let g = &mut s.graph;
    // Only the vertices named or selected: a whole object is not hammered, as the reference's works on components.
    let comps: Vec<String> = names.iter().filter(|n| n.contains('.')).cloned().collect();
    let mut targets = crate::skin::points_named(g, &comps);
    if targets.iter().all(|t| t.1.is_empty()) {
        targets.clear();
        let mode = crate::components::mode(g);
        if mode != crate::components::ComponentMode::Object {
            for n in &names {
                let id = crate::rig::find(g, n.split('.').next().unwrap_or(n), "weightHammerVerts")?;
                if let Some(m) = crate::components::as_mesh(g, id) {
                    let ids = crate::components::selected(g, m, mode).to_vec();
                    if ids.is_empty() {
                        continue;
                    }
                    if let Some(mm) = crate::components::mesh_of(g, m) {
                        let pts = crate::components::convert(&mm, mode, &ids, crate::components::ComponentMode::Vertex, false).to_vec();
                        targets.push((m, pts));
                    }
                }
            }
        }
    }
    if targets.iter().all(|t| t.1.is_empty()) {
        log::warn!("weightHammerVerts: select the vertices to hammer");
        return Ok(Value::Null);
    }
    for (mesh, ids) in targets {
        let Some(sk) = crate::skin::skin_of(g, mesh) else { fail!("weightHammerVerts: {} has no skin", g.path(mesh)) };
        let Some(m) = crate::modifiers::cage(g, mesh) else { continue };
        let mut b = (*bind_of(g, sk).unwrap_or_default()).clone();
        let old = b.weights.clone();
        let locks = SkinBind { locks: b.locks.clone(), ..Default::default() };
        for &p in &ids {
            let mut around = Vec::new();
            m.for_each_point_edge(p, |e| around.push(m.edge_other_point(e, p)));
            let outside: Vec<u32> = around.iter().copied().filter(|q| !ids.contains(q)).collect();
            let from = if outside.is_empty() { around } else { outside };
            if from.is_empty() {
                continue;
            }
            let mut acc: HashMap<u32, f64> = HashMap::new();
            for q in &from {
                for &(i, w) in old.get(*q as usize).map(Vec::as_slice).unwrap_or(&[]) {
                    *acc.entry(i).or_default() += w / from.len() as f64;
                }
            }
            let Some(own) = old.get(p as usize) else { continue };
            let held: Vec<(u32, f64)> = own.iter().copied().filter(|x| locks.locks.get(x.0 as usize).copied().unwrap_or(false)).collect();
            let mut next: Vec<(u32, f64)> = acc.into_iter().filter(|x| !locks.locks.get(x.0 as usize).copied().unwrap_or(false)).collect();
            next.extend(held);
            next.sort_by_key(|x| x.0);
            normalize_point(&mut next, &locks);
            b.weights[p as usize] = next;
        }
        commit_paint(g, sk, b)?;
    }
    Ok(Value::Null)
}

/// Moves a fraction of one influence's weight to another on some points.
fn transfer(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let g = &mut s.graph;
    let Some(skin_name) = names.first() else { fail!("transferBetweenInfluences: name the skinCluster") };
    let Some(sk) = skin_named(g, skin_name) else { fail!("transferBetweenInfluences: no skinCluster {skin_name}") };
    let (Some(src), Some(dst)) = (a.get("source").and_then(Value::as_str), a.get("destination").and_then(Value::as_str)) else { fail!("transferBetweenInfluences: give -src and -dst") };
    let list = influences(g, sk);
    let inf = |g: &Graph, n: &str| -> CmdResult<u32> {
        let j = crate::rig::find(g, n, "transferBetweenInfluences")?;
        match list.iter().find(|x| x.1 == j) {
            Some(e) => Ok(e.0 as u32),
            None => fail!("transferBetweenInfluences: {n} is not an influence of {skin_name}"),
        }
    };
    let (from, to) = (inf(g, src)?, inf(g, dst)?);
    let mut b = (*bind_of(g, sk).unwrap_or_default()).clone();
    if b.locks.get(from as usize).copied().unwrap_or(false) || b.locks.get(to as usize).copied().unwrap_or(false) {
        fail!("transferBetweenInfluences: unlock both influences first");
    }
    let ids: Vec<u32> = if names.len() > 1 {
        crate::skin::points_named(g, &names[1..]).into_iter().flat_map(|x| x.1).collect()
    } else {
        match skin_geometry(g, sk) {
            Some(m) => selected_points(g, m),
            None => Vec::new(),
        }
    };
    let amount = a.float("value", 1.0).clamp(0.0, 1.0);
    for p in ids {
        let Some(w) = b.weights.get_mut(p as usize) else { continue };
        let Some(i) = w.iter().position(|x| x.0 == from) else { continue };
        let moved = w[i].1 * amount;
        w[i].1 -= moved;
        match w.iter_mut().find(|x| x.0 == to) {
            Some(x) => x.1 += moved,
            None => w.push((to, moved)),
        }
        w.retain(|x| x.1 > 1e-9);
        w.sort_by_key(|x| x.0);
    }
    commit_paint(g, sk, b)?;
    Ok(Value::Null)
}

/// Weights on a skin for a mesh influence: by distance from its surface, each point keeping what it had in proportion.
pub(crate) fn geometry_influence_weights(b: &mut SkinBind, m: &Mesh, inf: u32, influence_mesh: &Mesh, influence_world: &DMat4, dropoff: f64) {
    let mut w_mesh = influence_mesh.clone();
    for p in 0..w_mesh.point_capacity() {
        if w_mesh.point_valid(p) {
            w_mesh.set_position(p, influence_world.transform_point3(influence_mesh.position(p)));
        }
    }
    let bvh = MeshBvh::new(&w_mesh);
    let locks = SkinBind { locks: b.locks.clone(), ..Default::default() };
    let geom = b.geom;
    for p in 0..m.point_capacity().min(b.weights.len() as u32) {
        if !m.point_valid(p) {
            continue;
        }
        let at = geom.transform_point3(m.position(p));
        let Some(c) = bvh.closest_point(at) else { continue };
        let others: f64 = b.weights[p as usize].iter().map(|x| x.1).sum();
        let w = (1.0 / (1.0 + c.distance)).powf(dropoff) * others.max(1e-3);
        let pw = &mut b.weights[p as usize];
        pw.retain(|x| x.0 != inf);
        pw.push((inf, w));
        pw.sort_by_key(|x| x.0);
        normalize_point(pw, &locks);
    }
}

/// Substitute Geometry: a new mesh takes over from a skinned one, with the same influences and the weights copied across.
fn substitute(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let (old, new) = match (a.get("oldGeometry").and_then(Value::as_str), a.get("newGeometry").and_then(Value::as_str)) {
        (Some(o), Some(n)) => (o.to_string(), n.to_string()),
        _ => {
            let [old, new] = &names[..] else { fail!("substituteGeometry: select the skinned mesh, then its replacement") };
            (old.clone(), new.clone())
        }
    };
    let g = &mut s.graph;
    let Some(old_skin) = skin_named(g, &old) else { fail!("substituteGeometry: {old} has no skin") };
    // Where the old geometry's points are, for -reWeightDistTolerance.
    g.evaluate();
    let old_points: Vec<glam::DVec3> = match g.find(&old).ok().flatten().and_then(|id| crate::components::as_mesh(g, id)) {
        Some(m) => {
            let w = crate::rig::world(g, crate::dag::transform_of(g, m));
            crate::modifiers::cage(g, m).map(|c| (0..c.point_capacity()).filter(|&p| c.point_valid(p)).map(|p| w.transform_point3(c.position(p))).collect()).unwrap_or_default()
        }
        None => Vec::new(),
    };
    let joints: Vec<String> = influences(g, old_skin).into_iter().map(|x| g.path(x.1)).collect();
    let max_inf = g.node(old_skin).map(|n| n.data().int(crate::skin::SK_MAX_INF)).unwrap_or(5);
    let mut args: Vec<Value> = joints.iter().map(|j| json!(j)).collect();
    args.push(json!(new));
    let made = execute(s, "skinCluster", &json!({"args": args, "toSelectedBones": true, "maximumInfluences": max_inf}), CallSource::Script)?;
    let new_skin = made.as_array().and_then(|v| v.first()).and_then(Value::as_str).unwrap_or("").to_string();
    let old_skin_path = s.graph.path(old_skin);
    execute(s, "copySkinWeights", &json!({"sourceSkin": old_skin_path, "destinationSkin": new_skin, "noMirror": true, "surfaceAssociation": "closestPoint", "influenceAssociation": ["oneToOne"]}), CallSource::Script)?;
    if let Some(tol) = a.get("reWeightDistTolerance").and_then(Value::as_f64) {
        if let Ok(Some(sk)) = s.graph.find(&new_skin) {
            crate::skin::reweight_far(&mut s.graph, sk, &old_points, tol.max(0.0))?;
        }
    }
    // -disableNonSkinDeformers: the other deformers on the geometry are left with no effect.
    if a.bool("disableNonSkinDeformers") {
        let g = &mut s.graph;
        let mut shapes = Vec::new();
        for n in [&old, &new] {
            if let Some(m) = g.find(n).ok().flatten().and_then(|id| crate::components::as_mesh(g, id)) {
                shapes.push(m);
            }
        }
        for m in shapes {
            for h in crate::history::history_of(g, m) {
                let Some(node) = g.node(h) else { continue };
                if crate::deform::is_deformer(node.ty().name.as_str()) && !node.ty().is_a("skinCluster") {
                    if let Some(i) = node.ty().attr_index("nodeState") {
                        g.set_value(crate::plug::PlugRef::new(h, i as i32), crate::plug::PlugValue::Int(1))?;
                    }
                }
            }
        }
    }
    // -newGeometryToLayer and -oldGeometryToLayer: each goes into a new display layer. The old geometry kept in one
    // isn't deleted.
    if a.bool("newGeometryToLayer") {
        execute(s, "createDisplayLayer", &json!({"args": [new], "name": "newGeometryLayer", "noRecurse": true}), CallSource::Script)?;
    }
    let keep_old = a.bool("retainOldGeometry") || a.bool("oldGeometryToLayer");
    if a.bool("oldGeometryToLayer") {
        execute(s, "createDisplayLayer", &json!({"args": [old], "name": "oldGeometryLayer", "noRecurse": true}), CallSource::Script)?;
    }
    if !keep_old {
        execute(s, "delete", &json!({"args": [old]}), CallSource::Script)?;
    }
    Ok(json!([new_skin]))
}

fn remove_unused(s: &mut Session, a: &Args) -> CmdResult {
    let names = objects(s, a);
    let mut removed = Vec::new();
    for n in names {
        let Some(sk) = skin_named(&s.graph, &n) else { fail!("removeUnusedInfluences: {n} has no skin") };
        let g = &s.graph;
        let b = bind_of(g, sk).unwrap_or_default();
        for (i, j) in influences(g, sk) {
            if !b.weights.iter().any(|w| w.iter().any(|x| x.0 == i as u32 && x.1 > 0.0)) {
                removed.push(g.path(j));
            }
        }
        let path = s.graph.path(sk);
        execute(s, "skinCluster", &json!({"args": [path], "edit": true, "removeUnusedInfluence": true}), CallSource::Script)?;
    }
    Ok(json!(removed))
}

pub(crate) fn register(c: &mut Commands<Session>) {
    c.register(
        aztek_ops::CommandDef::<Session>::new(
            "artAttrSkinWeightCopy",
            "Copy Vertex Weights: holds the weights of the selected vertex, or their mean when several are selected.",
            |s, _| {
                let names = s.run("filterExpand", serde_json::json!({"selectionMask": 31, "expand": true}))?;
                let Some(list) = names.as_array().filter(|l| !l.is_empty()) else { aztek_ops::fail!("artAttrSkinWeightCopy: select vertices") };
                let first = list[0].as_str().unwrap_or("").split('.').next().unwrap_or("").to_string();
                let skin = s.run("findRelatedSkinCluster", serde_json::json!({"args": [first]}))?;
                let Some(skin) = skin.as_str().map(str::to_string) else { aztek_ops::fail!("artAttrSkinWeightCopy: {first} has no skin") };
                let infs = s.run("skinCluster", serde_json::json!({"args": [skin], "query": true, "queried": ["influence"]}))?;
                let mut held = Vec::new();
                for inf in infs.as_array().into_iter().flatten() {
                    let inf = inf.as_str().unwrap_or("").to_string();
                    let mut sum = 0.0;
                    for v in list {
                        let w = s.run("skinPercent", serde_json::json!({"args": [skin, v], "query": true, "queried": ["transform"], "transform": inf}))?;
                        sum += w.as_f64().unwrap_or(0.0);
                    }
                    held.push((inf, sum / list.len() as f64));
                }
                s.extras.more.copied_weights = held;
                Ok(serde_json::Value::Null)
            },
        )
        .undoable(false),
    );
    c.register(aztek_ops::CommandDef::<Session>::new(
        "artAttrSkinWeightPaste",
        "Paste Vertex Weights: gives the selected vertices the weights last copied.",
        |s, _| {
            let held = s.extras.more.copied_weights.clone();
            if held.is_empty() {
                aztek_ops::fail!("artAttrSkinWeightPaste: copy some weights first");
            }
            let names = s.run("filterExpand", serde_json::json!({"selectionMask": 31, "expand": true}))?;
            let Some(list) = names.as_array().filter(|l| !l.is_empty()).cloned() else { aztek_ops::fail!("artAttrSkinWeightPaste: select vertices") };
            let first = list[0].as_str().unwrap_or("").split('.').next().unwrap_or("").to_string();
            let skin = s.run("findRelatedSkinCluster", serde_json::json!({"args": [first]}))?;
            let Some(skin) = skin.as_str().map(str::to_string) else { aztek_ops::fail!("artAttrSkinWeightPaste: {first} has no skin") };
            let tv: Vec<serde_json::Value> = held.iter().map(|(j, w)| serde_json::json!([j, w])).collect();
            for v in list {
                s.run("skinPercent", serde_json::json!({"args": [skin, v], "transformValue": tv}))?;
            }
            Ok(serde_json::Value::Null)
        },
    ));

    c.register(
        Cmd::new(
            "deformerWeights",
            "Export or import a deformer's weights in the reference's layout: deformerWeights name.xml -ex -deformer skinCluster1 -path dir; -im with -method \
             index, nearest, barycentric, bilinear or over. A .json file, or -fm JSON, goes as JSON. Returns the file's path.",
            deformer_weights,
        )
        .undoable(true)
        .positional("the file")
        .flag(f("export", "ex", ArgType::Bool, "Write the file."))
        .flag(f("import", "im", ArgType::Bool, "Read the file."))
        .flag(f("deformer", "df", ArgType::Any, "The deformers. -shape takes all those on a mesh."))
        .flag(f("shape", "sh", ArgType::String, "The deformed shape."))
        .flag(f("path", "p", ArgType::String, "The file's folder."))
        .flag(f("method", "m", ArgType::String, "index (the default), nearest, barycentric, bilinear or over: over leaves points the file doesn't list."))
        .flag(f("defaultValue", "dv", ArgType::Float, "The weight of a point the file doesn't list."))
        .flag(f("format", "fm", ArgType::String, "XML or JSON."))
        .flag(f("weightPrecision", "wp", ArgType::Int, "Decimal places in XML (3)."))
        .flag(f("weightTolerance", "wt", ArgType::Float, "Leave out weights at or below this."))
        .flag(f("ignoreName", "ig", ArgType::Bool, "Match deformers and sources by order, not name."))
        .flag(f("attribute", "at", ArgType::List, "A deformer attribute (numbers or a matrix) written and read with the weights: envelope, say. Repeat it.").multi(1))
        .flag(f("positionTolerance", "pt", ArgType::Float, "With -method nearest: how far to look for a point (no limit)."))
        .flag(f("remap", "r", ArgType::List, "\"pattern;format\": names matching the pattern become the format, $1 to $9 its groups; scene names to the file's on export, the file's to the scene's on import. Repeat it.").multi(1))
        .flag(f("skip", "sk", ArgType::List, "Deformers, shapes and influences whose names match are left out. Repeat it.").multi(1))
        .flag(f("vertexConnections", "vc", ArgType::Bool, "With -ex: write the faces too, which -method barycentric and bilinear use."))
        .flag(f("worldSpace", "ws", ArgType::Bool, "With -method nearest: compare world positions rather than the shapes' own."))
        ,
    );
    if let Some(d) = c.find("deformerWeights").cloned() {
        c.register(crate::skin::modes(d));
    }
    c.register(
        Cmd::new("weightHammerVerts", "Hammer Skin Weights: the points named or selected take the average weights of the points around them.", hammer)
            .positional("points (the selection when none)"),
    );
    c.register(
        Cmd::new("transferBetweenInfluences", "Moves weight from one influence to another on the points named or selected: transferBetweenInfluences skinCluster1 -src joint1 -dst joint2 -v 0.5.", transfer)
            .positional("the skinCluster, then points")
            .flag(f("source", "src", ArgType::String, "The influence giving weight."))
            .flag(f("destination", "dst", ArgType::String, "The influence taking it."))
            .flag(f("value", "v", ArgType::Float, "How much of it moves (1).")),
    );
    c.register(
        Cmd::new("substituteGeometry", "Substitute Geometry: the second mesh takes over from the skinned first, with its influences and weights. The old mesh goes unless -rog. Returns the new skinCluster.", substitute)
            .positional("the skinned mesh, then its replacement (the selection when none)")
            .flag(f("oldGeometry", "og", ArgType::String, "The skinned mesh."))
            .flag(f("newGeometry", "ng", ArgType::String, "Its replacement."))
            .flag(f("retainOldGeometry", "rog", ArgType::Bool, "Keep the old mesh."))
            .flag(f("disableNonSkinDeformers", "dnd", ArgType::Bool, "Leave the deformers other than skins on the geometry with no effect afterwards (off)."))
            .flag(f("newGeometryToLayer", "ngl", ArgType::Bool, "Put the new geometry in a new display layer."))
            .flag(f("oldGeometryToLayer", "ogl", ArgType::Bool, "Keep the old geometry, in a new display layer."))
            .flag(f("reWeightDistTolerance", "wdt", ArgType::Float, "New points farther than this from the old geometry take their weights from their neighbours, not from points they were matched to by mistake.")),
    );
    c.register(
        Cmd::new("removeUnusedInfluences", "Takes influences holding no weight off a skin. Returns those taken off.", remove_unused).positional("skinned meshes or skinClusters (the selection when none)"),
    );
}

// ---------------------------------------------------------------------------------------------------------------------
// UV space

/// A point's UV: from its first corner.
pub(crate) fn uv_of_point(m: &Mesh, p: u32) -> Option<[f32; 2]> {
    let uv = m.corner_attrs().get::<[f32; 2]>(crate::uv_ops::UV);
    let face = m.point_faces(p).into_iter().next()?;
    let c = m.face_corners(face).into_iter().find(|&c| m.corner_point(c) == p)?;
    uv.get(c as usize).copied()
}

/// A mesh's triangles in UV space, to find what lies at a UV.
pub(crate) struct UvLookup {
    tris: Vec<([u32; 3], [[f32; 2]; 3])>,
}

impl UvLookup {
    pub(crate) fn new(m: &Mesh) -> UvLookup {
        let uv = m.corner_attrs().get::<[f32; 2]>(crate::uv_ops::UV);
        let mut tris = Vec::new();
        for face in (0..m.face_capacity()).filter(|&x| m.face_valid(x)) {
            let corners = m.face_corners(face);
            let at = |k: usize| -> (u32, [f32; 2]) { (m.corner_point(corners[k]), uv.get(corners[k] as usize).copied().unwrap_or([0.0, 0.0])) };
            for k in 1..corners.len().saturating_sub(1) {
                let (a, b, c) = (at(0), at(k), at(k + 1));
                tris.push(([a.0, b.0, c.0], [a.1, b.1, c.1]));
            }
        }
        UvLookup { tris }
    }

    /// The source points at a UV and their shares: the triangle holding it, or failing that the nearest corner.
    pub(crate) fn sample(&self, uv: [f32; 2]) -> Option<Vec<(u32, f64)>> {
        let (x, y) = (uv[0] as f64, uv[1] as f64);
        let mut nearest: Option<(f64, u32)> = None;
        for (pts, t) in &self.tris {
            let (ax, ay, bx, by, cx, cy) = (t[0][0] as f64, t[0][1] as f64, t[1][0] as f64, t[1][1] as f64, t[2][0] as f64, t[2][1] as f64);
            let det = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
            if det.abs() > 1e-14 {
                let u = ((by - cy) * (x - cx) + (cx - bx) * (y - cy)) / det;
                let v = ((cy - ay) * (x - cx) + (ax - cx) * (y - cy)) / det;
                let w = 1.0 - u - v;
                if u >= -1e-9 && v >= -1e-9 && w >= -1e-9 {
                    return Some(vec![(pts[0], u.max(0.0)), (pts[1], v.max(0.0)), (pts[2], w.max(0.0))]);
                }
            }
            for k in 0..3 {
                let d = (t[k][0] as f64 - x).powi(2) + (t[k][1] as f64 - y).powi(2);
                if nearest.is_none_or(|n| d < n.0) {
                    nearest = Some((d, pts[k]));
                }
            }
        }
        nearest.map(|n| vec![(n.1, 1.0)])
    }
}
