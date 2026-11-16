//! The geometry schemas Aztek writes and reads (the reference library's AbcGeom): transforms (Xform_v3, as one matrix
//! operation), polygon meshes (PolyMesh_v1, faces wound clockwise as the format has them) and curves (Curve_v2).

use crate::abc::{meta_value, Kind, Object, Pod, Property};

pub const XFORM: &str = "AbcGeom_Xform_v3";
pub const POLYMESH: &str = "AbcGeom_PolyMesh_v1";
pub const CURVES: &str = "AbcGeom_Curve_v2";
const GEOM_BASE: &str = "AbcGeom_GeomBase_v1";

fn bytes_f32(v: impl IntoIterator<Item = f32>) -> Vec<u8> {
    v.into_iter().flat_map(f32::to_le_bytes).collect()
}
fn bytes_f64(v: impl IntoIterator<Item = f64>) -> Vec<u8> {
    v.into_iter().flat_map(f64::to_le_bytes).collect()
}
fn bytes_i32(v: impl IntoIterator<Item = i32>) -> Vec<u8> {
    v.into_iter().flat_map(i32::to_le_bytes).collect()
}
pub fn f32s(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect()
}
pub fn f64s(b: &[u8]) -> Vec<f64> {
    b.chunks_exact(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect()
}
pub fn i32s(b: &[u8]) -> Vec<i32> {
    b.chunks_exact(4).map(|c| i32::from_le_bytes(c.try_into().unwrap())).collect()
}

fn bounds(points: &[[f32; 3]]) -> Vec<u8> {
    if points.is_empty() {
        return bytes_f64([0.0; 6]);
    }
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in points {
        for a in 0..3 {
            lo[a] = lo[a].min(p[a] as f64);
            hi[a] = hi[a].max(p[a] as f64);
        }
    }
    bytes_f64([lo[0], lo[1], lo[2], hi[0], hi[1], hi[2]])
}

fn object_meta(schema: &str, base: Option<&str>, title: &str) -> String {
    match base {
        Some(b) => format!("schema={schema};schemaBaseType={b};schemaObjTitle={schema}:{title}"),
        None => format!("schema={schema};schemaObjTitle={schema}:{title}"),
    }
}

/// A transform: its local matrix at each sample, as 16 numbers column by column (the format's rows).
pub fn xform(name: &str, ts: u32, matrices: &[[f64; 16]], children: Vec<Object>) -> Object {
    let identity = {
        let mut m = [0.0; 16];
        for k in 0..4 {
            m[k * 5] = 1.0;
        }
        m
    };
    let mut props = vec![
        Property::scalar(".ops", "", Pod::U8, 1, 0, vec![vec![0x30]]),
        Property::scalar(".vals", "", Pod::F64, 16, ts, matrices.iter().map(|m| bytes_f64(m.iter().copied())).collect()),
    ];
    let animated: Vec<u32> = (0..16u32).filter(|&c| matrices.windows(2).any(|w| w[0][c as usize] != w[1][c as usize])).collect();
    if !animated.is_empty() {
        props.push(Property::array(".animChans", "", Pod::U32, 1, 0, vec![animated.iter().flat_map(|x| x.to_le_bytes()).collect()]));
    }
    if matrices.iter().any(|m| *m != identity) {
        props.push(Property::scalar("isNotConstantIdentity", "", Pod::Bool, 1, 0, vec![vec![1]]));
    }
    Object {
        name: name.into(),
        meta: object_meta(XFORM, None, ".xform"),
        props: vec![Property::compound(".xform", &format!("schema={XFORM}"), props)],
        children,
    }
}

/// One sample of a mesh.
pub struct MeshSample {
    pub points: Vec<[f32; 3]>,
}

/// A polygon mesh: points per sample, then the topology (counts and indices, counter-clockwise as Aztek has them; they
/// are written clockwise), and optionally a UV per face-vertex in the same order.
pub fn polymesh(name: &str, ts: u32, samples: &[MeshSample], counts: &[i32], indices: &[i32], uvs: Option<&[[f32; 2]]>) -> Object {
    let mut cw = Vec::with_capacity(indices.len());
    let mut uv_cw: Vec<[f32; 2]> = Vec::new();
    let mut at = 0usize;
    for &c in counts {
        let c = c.max(0) as usize;
        // Kept first, the rest reversed: 0 1 2 3 becomes 0 3 2 1.
        let face = &indices[at..at + c];
        cw.extend(face.iter().rev());
        if let Some(u) = uvs {
            uv_cw.extend(u[at..at + c].iter().rev());
        }
        at += c;
    }
    let mut props = vec![
        Property::scalar(".selfBnds", "interpretation=box", Pod::F64, 6, ts, samples.iter().map(|s| bounds(&s.points)).collect()),
        Property::array("P", "geoScope=vtx;interpretation=point", Pod::F32, 3, ts, samples.iter().map(|s| bytes_f32(s.points.iter().flatten().copied())).collect()),
        Property::array(".faceIndices", "", Pod::I32, 1, 0, vec![bytes_i32(cw)]),
        Property::array(".faceCounts", "", Pod::I32, 1, 0, vec![bytes_i32(counts.iter().copied())]),
    ];
    if uvs.is_some() {
        // Indexed, as the reference writes UVs: each distinct UV once, and an index per face-vertex.
        let mut vals: Vec<[f32; 2]> = Vec::new();
        let mut seen: std::collections::HashMap<[u32; 2], u32> = std::collections::HashMap::new();
        let idx: Vec<u32> = uv_cw
            .iter()
            .map(|u| {
                *seen.entry([u[0].to_bits(), u[1].to_bits()]).or_insert_with(|| {
                    vals.push(*u);
                    (vals.len() - 1) as u32
                })
            })
            .collect();
        let meta = "arrayExtent=1;geoScope=fvr;interpretation=vector;isGeomParam=true;podExtent=2;podName=float32_t";
        props.push(Property::compound(
            "uv",
            meta,
            vec![
                Property::array(".vals", "arrayExtent=1;geoScope=fvr;interpretation=vector;podExtent=2;podName=float32_t", Pod::F32, 2, 0, vec![bytes_f32(vals.iter().flatten().copied())]),
                Property::array(".indices", "", Pod::U32, 1, 0, vec![idx.iter().flat_map(|x| x.to_le_bytes()).collect()]),
            ],
        ));
    }
    Object {
        name: name.into(),
        meta: object_meta(POLYMESH, Some(GEOM_BASE), ".geom"),
        props: vec![Property::compound(".geom", &format!("schema={POLYMESH};schemaBaseType={GEOM_BASE}"), props)],
        children: Vec::new(),
    }
}

/// Curves: per sample, every curve's points one after another and a width per point; points per curve; linear or
/// cubic B-spline.
pub fn curves(name: &str, ts: u32, points: &[Vec<[f32; 3]>], widths: &[Vec<f32>], counts: &[i32], cubic: bool) -> Object {
    let mut props = vec![
        Property::scalar(".selfBnds", "interpretation=box", Pod::F64, 6, ts, points.iter().map(|p| bounds(p)).collect()),
        Property::array("P", "geoScope=vtx;interpretation=point", Pod::F32, 3, ts, points.iter().map(|p| bytes_f32(p.iter().flatten().copied())).collect()),
        Property::array("nVertices", "", Pod::I32, 1, 0, vec![bytes_i32(counts.iter().copied())]),
        // Type (cubic 0, linear 1), periodicity (non-periodic 0), basis (none 0, B-spline 2), basis step.
        Property::scalar("curveBasisAndType", "", Pod::U8, 4, 0, vec![if cubic { vec![0, 0, 2, 1] } else { vec![1, 0, 0, 1] }]),
    ];
    if !widths.is_empty() {
        props.push(Property::array("width", "arrayExtent=1;geoScope=vtx;isGeomParam=true;podExtent=1;podName=float32_t", Pod::F32, 1, ts, widths.iter().map(|w| bytes_f32(w.iter().copied())).collect()));
    }
    Object {
        name: name.into(),
        meta: object_meta(CURVES, Some(GEOM_BASE), ".geom"),
        props: vec![Property::compound(".geom", &format!("schema={CURVES};schemaBaseType={GEOM_BASE}"), props)],
        children: Vec::new(),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Reading

/// What an object is.
pub fn schema(o: &Object) -> Option<String> {
    o.meta_value("schema").or_else(|| match o.props.first().map(|p| &p.kind) {
        Some(Kind::Compound(_)) => meta_value(&o.props[0].meta, "schema"),
        _ => None,
    })
}

fn geom(o: &Object) -> Option<&Property> {
    o.props.first().filter(|p| matches!(p.kind, Kind::Compound(_)))
}

/// A transform's local matrix per sample (column by column), from its operations: matrix, translate, rotate (degrees,
/// about an axis or X, Y, Z) and scale, applied in order as the format composes them.
pub fn xform_matrices(o: &Object) -> Vec<[f64; 16]> {
    let Some(x) = geom(o) else { return vec![identity()] };
    let ops: Vec<u8> = x.child(".ops").and_then(|p| p.samples().first().cloned()).unwrap_or_default();
    let Some(vals) = x.child(".vals") else { return vec![identity()] };
    let samples: Vec<Vec<f64>> = vals.samples().iter().map(|b| f64s(b)).collect();
    if samples.is_empty() {
        return vec![identity()];
    }
    samples.iter().map(|v| compose(&ops, v)).collect()
}

pub fn identity() -> [f64; 16] {
    let mut m = [0.0; 16];
    for k in 0..4 {
        m[k * 5] = 1.0;
    }
    m
}

fn mul(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    // Column-major a * b.
    let mut r = [0.0; 16];
    for c in 0..4 {
        for row in 0..4 {
            r[c * 4 + row] = (0..4).map(|k| a[k * 4 + row] * b[c * 4 + k]).sum();
        }
    }
    r
}

fn rotation(axis: [f64; 3], deg: f64) -> [f64; 16] {
    let l = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt().max(1e-300);
    let (x, y, z) = (axis[0] / l, axis[1] / l, axis[2] / l);
    let (s, c) = deg.to_radians().sin_cos();
    let t = 1.0 - c;
    [
        t * x * x + c, t * x * y + s * z, t * x * z - s * y, 0.0,
        t * x * y - s * z, t * y * y + c, t * y * z + s * x, 0.0,
        t * x * z + s * y, t * y * z - s * x, t * z * z + c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

/// The operations, each applied to the right as the format reads them (the first operation outermost).
fn compose(ops: &[u8], v: &[f64]) -> [f64; 16] {
    let mut m = identity();
    let mut k = 0usize;
    let take = |n: usize, k: &mut usize| -> Vec<f64> {
        let s: Vec<f64> = (0..n).map(|i| v.get(*k + i).copied().unwrap_or(0.0)).collect();
        *k += n;
        s
    };
    for &op in ops {
        let o = match op >> 4 {
            0 => {
                let s = take(3, &mut k);
                let mut x = identity();
                x[0] = s[0];
                x[5] = s[1];
                x[10] = s[2];
                x
            }
            1 => {
                let t = take(3, &mut k);
                let mut x = identity();
                x[12] = t[0];
                x[13] = t[1];
                x[14] = t[2];
                x
            }
            2 => {
                let a = take(4, &mut k);
                rotation([a[0], a[1], a[2]], a[3])
            }
            3 => {
                let a = take(16, &mut k);
                let mut x = [0.0; 16];
                x.copy_from_slice(&a);
                x
            }
            4 => rotation([1.0, 0.0, 0.0], take(1, &mut k)[0]),
            5 => rotation([0.0, 1.0, 0.0], take(1, &mut k)[0]),
            6 => rotation([0.0, 0.0, 1.0], take(1, &mut k)[0]),
            _ => identity(),
        };
        // The format's row-vector order: the matrix is ops[n-1] * ... * ops[0] in its terms, so in column terms each
        // new operation goes on the right.
        m = mul(&m, &o);
    }
    m
}

/// A mesh read back: counts and indices counter-clockwise, points per sample, UVs per face-vertex in that order.
pub struct MeshRead {
    pub counts: Vec<i32>,
    pub indices: Vec<i32>,
    pub points: Vec<Vec<[f32; 3]>>,
    pub uvs: Option<Vec<[f32; 2]>>,
}

fn vec3s(b: &[u8]) -> Vec<[f32; 3]> {
    f32s(b).chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect()
}

pub fn read_mesh(o: &Object) -> Option<MeshRead> {
    let g = geom(o)?;
    let counts = i32s(g.child(".faceCounts")?.samples().first()?);
    let cw = i32s(g.child(".faceIndices")?.samples().first()?);
    let points: Vec<Vec<[f32; 3]>> = g.child("P")?.samples().iter().map(|b| vec3s(b)).collect();
    let uv_raw: Option<Vec<[f32; 2]>> = g.child("uv").and_then(|u| match &u.kind {
        Kind::Array { .. } => u.samples().first().map(|b| f32s(b).chunks_exact(2).map(|c| [c[0], c[1]]).collect()),
        Kind::Compound(_) => {
            let vals: Vec<[f32; 2]> = f32s(u.child(".vals")?.samples().first()?).chunks_exact(2).map(|c| [c[0], c[1]]).collect();
            let idx = u.child(".indices")?.samples().first()?.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap()) as usize).collect::<Vec<_>>();
            Some(idx.iter().map(|&i| vals.get(i).copied().unwrap_or_default()).collect())
        }
        _ => None,
    });
    let (mut indices, mut uvs) = (Vec::with_capacity(cw.len()), uv_raw.as_ref().map(|_| Vec::new()));
    let mut at = 0usize;
    for &c in &counts {
        let c = c.max(0) as usize;
        if at + c > cw.len() {
            break;
        }
        indices.extend(cw[at..at + c].iter().rev());
        if let (Some(u), Some(src)) = (uvs.as_mut(), uv_raw.as_ref()) {
            if src.len() >= at + c {
                u.extend(src[at..at + c].iter().rev());
            }
        }
        at += c;
    }
    Some(MeshRead { counts, indices, points, uvs })
}

pub struct CurvesRead {
    pub counts: Vec<i32>,
    pub points: Vec<Vec<[f32; 3]>>,
    pub widths: Vec<Vec<f32>>,
    pub cubic: bool,
}

pub fn read_curves(o: &Object) -> Option<CurvesRead> {
    let g = geom(o)?;
    let counts = i32s(g.child("nVertices")?.samples().first()?);
    let points: Vec<Vec<[f32; 3]>> = g.child("P")?.samples().iter().map(|b| vec3s(b)).collect();
    let widths: Vec<Vec<f32>> = g.child("width").map(|w| match &w.kind {
        Kind::Compound(_) => w.child(".vals").map(|v| v.samples().iter().map(|b| f32s(b)).collect()).unwrap_or_default(),
        _ => w.samples().iter().map(|b| f32s(b)).collect(),
    }).unwrap_or_default();
    let cubic = g.child("curveBasisAndType").and_then(|p| p.samples().first().cloned()).is_some_and(|b| b.first() == Some(&0));
    Some(CurvesRead { counts, points, widths, cubic })
}

/// The time sampling a schema's points use.
pub fn sampling_of(o: &Object) -> u32 {
    let Some(g) = geom(o) else { return 0 };
    for name in ["P", ".vals"] {
        if let Some(Kind::Scalar { ts, .. } | Kind::Array { ts, .. }) = g.child(name).map(|p| &p.kind) {
            return *ts;
        }
    }
    0
}
