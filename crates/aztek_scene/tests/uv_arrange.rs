//! The UV Toolkit's shell arranging (#108): orient, stack, unstack, gather, distribute, align, snap, randomize, linear align.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// A mesh's UV bounds: (umin, vmin, umax, vmax).
fn bounds(s: &mut Session, mesh: &str) -> [f64; 4] {
    let v: Vec<f64> = run(s, "polyEditUV", json!({"args": [format!("{mesh}.map[*]")], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let mut b = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
    for p in v.chunks(2) {
        b[0] = b[0].min(p[0]);
        b[1] = b[1].min(p[1]);
        b[2] = b[2].max(p[0]);
        b[3] = b[3].max(p[1]);
    }
    b
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-4
}

/// Two one-face planes, each one shell of 0 to 1; b's shrunk to half and moved off.
fn two(s: &mut Session) {
    run(s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1, "name": "a"}));
    run(s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1, "name": "b"}));
    run(s, "polyEditUV", json!({"args": ["b.map[*]"], "scaleU": 0.5, "scaleV": 0.5, "pivotU": 0.0, "pivotV": 0.0}));
    run(s, "polyEditUV", json!({"args": ["b.map[*]"], "u": 2.3, "v": -1.6}));
}

#[test]
fn gather_stack_distribute_align_snap() {
    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "texGatherShells", json!({"args": ["b.map[*]"]}));
    let b = bounds(&mut s, "b");
    assert!(b[0] >= 0.0 && b[2] <= 1.0 && b[1] >= 0.0 && b[3] <= 1.0, "{b:?}");

    run(&mut s, "texStackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    assert!(close((a[0] + a[2]) / 2.0, (b[0] + b[2]) / 2.0) && close((a[1] + a[3]) / 2.0, (b[1] + b[3]) / 2.0), "{a:?} {b:?}");

    run(&mut s, "texDistributeShells", json!({"args": ["a.map[*]", "b.map[*]"], "direction": "u", "spacing": 0.1}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    let (first, second) = if a[0] < b[0] { (a, b) } else { (b, a) };
    assert!(close(second[0], first[2] + 0.1), "{a:?} {b:?}");

    run(&mut s, "texAlignShells", json!({"args": ["a.map[*]", "b.map[*]"], "alignTo": "top"}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    assert!(close(a[3], b[3]), "{a:?} {b:?}");

    run(&mut s, "texSnapShells", json!({"args": ["b.map[*]"], "position": 8}));
    let b = bounds(&mut s, "b");
    assert!(close(b[2], 1.0) && close(b[1], 0.0), "{b:?}");
    run(&mut s, "undo", json!({}));
    let b2 = bounds(&mut s, "b");
    assert!(!close(b2[2], 1.0) || !close(b2[1], 0.0), "{b2:?}");
}

#[test]
fn orient_and_unstack() {
    let mut s = Session::new();
    two(&mut s);
    // Turned 30 degrees, a square shell's box grows; oriented, it is square to U and V again.
    run(&mut s, "polyEditUV", json!({"args": ["a.map[*]"], "angle": 30.0, "pivotU": 0.5, "pivotV": 0.5}));
    let t = bounds(&mut s, "a");
    assert!(t[2] - t[0] > 1.2);
    run(&mut s, "texOrientShells", json!({"args": ["a.map[*]"]}));
    let o = bounds(&mut s, "a");
    assert!(close(o[2] - o[0], 1.0) && close(o[3] - o[1], 1.0), "{o:?}");

    // Stacked, then unstacked: they no longer overlap.
    run(&mut s, "texStackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    run(&mut s, "texUnstackShells", json!({"args": ["a.map[*]", "b.map[*]"]}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    let overlap = a[0] < b[2] && b[0] < a[2] && a[1] < b[3] && b[1] < a[3];
    assert!(!overlap, "{a:?} {b:?}");
}

#[test]
fn orient_to_edge_randomize_linear_align() {
    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "polyEditUV", json!({"args": ["a.map[*]"], "angle": 20.0, "pivotU": 0.5, "pivotV": 0.5}));
    run(&mut s, "texOrientEdge", json!({"args": ["a.e[0]"]}));
    let o = bounds(&mut s, "a");
    assert!(close(o[2] - o[0], 1.0), "{o:?}");

    let before = bounds(&mut s, "b");
    run(&mut s, "texRandomizeShells", json!({"args": ["b.map[*]"], "translate": 0.5, "rotate": 30.0, "seed": 7}));
    let after = bounds(&mut s, "b");
    assert!(before != after);

    // Three UVs of a corner onto one line.
    run(&mut s, "texLinearAlignUVs", json!({"args": ["a.map[0:2]"]}));
    let p: Vec<f64> = run(&mut s, "polyEditUV", json!({"args": ["a.map[0:2]"], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let (x0, y0, x1, y1, x2, y2) = (p[0], p[1], p[2], p[3], p[4], p[5]);
    let cross = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
    assert!(cross.abs() < 1e-5, "{p:?}");
}

fn uv(s: &mut Session, what: &str) -> (f64, f64) {
    let v = run(s, "polyEditUV", json!({"args": [what], "query": true}));
    (v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

#[test]
fn straighten_match_grid_snap_together_select() {
    let mut s = Session::new();
    two(&mut s);
    // A corner of a pushed a little off its line comes back onto it.
    run(&mut s, "polyEditUV", json!({"args": ["a.map[1]"], "v": 0.05}));
    run(&mut s, "texStraightenUVs", json!({"args": ["a.map[0:1]"], "angle": 10.0}));
    let (p, q) = (uv(&mut s, "a.map[0]"), uv(&mut s, "a.map[1]"));
    assert!(close(p.1, q.1), "{p:?} {q:?}");

    // Two UVs close together meet at their middle; one off on its own stays.
    run(&mut s, "polyEditUV", json!({"args": ["a.map[0]"], "u": 0.004}));
    let before = uv(&mut s, "a.map[3]");
    run(&mut s, "texMatchUVs", json!({"args": ["a.map[0]", "a.map[1]", "a.map[3]"], "tolerance": 2.0}));
    let (p, q) = (uv(&mut s, "a.map[0]"), uv(&mut s, "a.map[1]"));
    assert!(close(p.0, q.0) && close(p.1, q.1));
    run(&mut s, "undo", json!({}));
    run(&mut s, "texMatchUVs", json!({"args": ["a.map[0]", "a.map[3]"], "tolerance": 0.001}));
    assert_eq!(uv(&mut s, "a.map[3]"), before);

    // Onto a grid of 4.
    run(&mut s, "polyEditUV", json!({"args": ["a.map[2]"], "u": 0.07, "v": -0.06}));
    run(&mut s, "texMatchGrid", json!({"args": ["a.map[2]"], "gridU": 4.0, "gridV": 4.0}));
    let p = uv(&mut s, "a.map[2]");
    assert!(close((p.0 * 4.0).fract(), 0.0) && close((p.1 * 4.0).fract().abs(), 0.0), "{p:?}");

    // b's shell moves so its UV 0 lands on a's UV 2.
    let target = uv(&mut s, "a.map[2]");
    run(&mut s, "texSnapTogether", json!({"args": ["a.map[2]", "b.map[0]"]}));
    let got = uv(&mut s, "b.map[0]");
    assert!(close(got.0, target.0) && close(got.1, target.1), "{got:?} {target:?}");

    // Flipped in U, a's face runs backwards.
    assert_eq!(run(&mut s, "texSelectByType", json!({"args": ["a"], "type": "backFacing"})), json!([]));
    run(&mut s, "polyEditUV", json!({"args": ["a.map[*]"], "scaleU": -1.0, "pivotU": 0.5, "pivotV": 0.5}));
    assert_eq!(run(&mut s, "texSelectByType", json!({"args": ["a"], "type": "backFacing"})), json!(["a.f[0]"]));
    assert_eq!(run(&mut s, "texSelectByType", json!({"args": ["a"], "type": "textureBorders"})).as_array().unwrap().len(), 4);
}

fn uvs(s: &mut Session, what: &str) -> Vec<f64> {
    run(s, "polyEditUV", json!({"args": [what], "query": true})).as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

#[test]
fn cycle_unitize_unmapped_normalize_direction() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 1, "name": "p"}));
    // Cycle: the face's UVs move on one corner.
    let before = uvs(&mut s, "p.map[*]");
    run(&mut s, "polyRotateUVsByVertex", json!({"args": ["p.f[0]"]}));
    assert_ne!(uvs(&mut s, "p.map[*]"), before);
    run(&mut s, "undo", json!({}));
    // Unitize: each face its own shell, filling 0 to 1.
    run(&mut s, "polyForceUV", json!({"args": ["p.f[*]"], "unitize": true}));
    let b = bounds(&mut s, "p");
    assert!(close(b[0], 0.0) && close(b[2], 1.0) && close(b[1], 0.0) && close(b[3], 1.0), "{b:?}");
    assert_eq!(uvs(&mut s, "p.map[*]").len(), 16);
    run(&mut s, "undo", json!({}));
    // Unmapped: a face with all its corners on one UV.
    assert_eq!(run(&mut s, "texSelectByType", json!({"args": ["p"], "type": "unmapped"})), json!([]));
    // Normalize along U alone leaves V be.
    run(&mut s, "polyEditUV", json!({"args": ["p.map[*]"], "scaleU": 0.3, "scaleV": 0.3, "pivotU": 0.0, "pivotV": 0.0}));
    run(&mut s, "polyNormalizeUV", json!({"args": ["p"], "normalizeType": 1, "normalizeDirection": 1}));
    let b = bounds(&mut s, "p");
    assert!(close(b[2] - b[0], 1.0) && close(b[3] - b[1], 0.3), "{b:?}");
}

#[test]
fn grid_shell_layout_along_distribute_snap_stack() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 3, "subdivisionsZ": 2, "name": "g"}));
    run(&mut s, "polyEditUV", json!({"args": ["g.map[*]"], "angle": 25.0, "pivotU": 0.5, "pivotV": 0.5}));
    run(&mut s, "texCreateShell", json!({"args": ["g.f[*]"], "grid": true}));
    let b = bounds(&mut s, "g");
    assert!(close(b[0], 0.0) && close(b[2], 1.0) && close(b[1], 0.0) && close(b[3], 1.0), "{b:?}");
    // Evenly spaced: every U is a third apart.
    let u: Vec<f64> = uvs(&mut s, "g.map[*]").chunks(2).map(|p| (p[0] * 3.0).round() / 3.0 - p[0]).collect();
    assert!(u.iter().all(|d| d.abs() < 1e-4), "{u:?}");

    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "texLayoutAlong", json!({"args": ["a.map[*]", "b.map[*]"], "direction": "u", "spacing": 0.0}));
    let (a, b) = (bounds(&mut s, "a"), bounds(&mut s, "b"));
    assert!(a[0] >= -1e-6 && b[2] <= 1.0 + 1e-6 && a[1] >= -1e-6 && b[1] >= -1e-6 && (close(a[2], b[0]) || close(b[2], a[0])), "{a:?} {b:?}");

    // Snap and Stack: b's corners 0 and 1 onto a's.
    let mut s = Session::new();
    two(&mut s);
    run(&mut s, "polyEditUV", json!({"args": ["b.map[*]"], "angle": 40.0, "pivotU": 2.5, "pivotV": -1.5}));
    run(&mut s, "texSnapStackShells", json!({"args": ["b.map[0]", "b.map[1]", "a.map[0]", "a.map[1]"]}));
    let (p, q) = (uv(&mut s, "b.map[0]"), uv(&mut s, "a.map[0]"));
    assert!(close(p.0, q.0) && close(p.1, q.1), "{p:?} {q:?}");
    let (p, q) = (uv(&mut s, "b.map[1]"), uv(&mut s, "a.map[1]"));
    assert!(close(p.0, q.0) && close(p.1, q.1), "{p:?} {q:?}");

    // Distribute UVs: three UVs along U evenly between the ends.
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 1, "name": "d"}));
    run(&mut s, "polyEditUV", json!({"args": ["d.map[1]"], "u": 0.2}));
    run(&mut s, "texDistributeUVs", json!({"args": ["d.map[0:2]"], "direction": "u"}));
    let us: Vec<f64> = uvs(&mut s, "d.map[0:2]").chunks(2).map(|p| p[0]).collect();
    let mut us2 = us.clone();
    us2.sort_by(f64::total_cmp);
    assert!(close(us2[1] - us2[0], us2[2] - us2[1]), "{us:?}");
}

#[test]
fn unfold_along_projections_symmetrize() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 2, "name": "p"}));
    run(&mut s, "polyEditUV", json!({"args": ["p.map[4]"], "u": 0.1, "v": 0.13}));
    let v_before: Vec<f64> = uvs(&mut s, "p.map[*]").chunks(2).map(|p| p[1]).collect();
    run(&mut s, "unfold", json!({"args": ["p"], "optimizeAxis": 1}));
    let v_after: Vec<f64> = uvs(&mut s, "p.map[*]").chunks(2).map(|p| p[1]).collect();
    for (a, b) in v_before.iter().zip(&v_after) {
        assert!(close(*a, *b), "{v_before:?} {v_after:?}");
    }
    // Along the faces' normals and through the camera: in 0 to 1.
    let cam = run(&mut s, "camera", json!({}));
    let cam = cam.as_array().and_then(|l| l.first()).and_then(|v| v.as_str()).unwrap_or("camera1").to_string();
    run(&mut s, "rotate", json!({"args": [-90.0, 0.0, 0.0, cam.clone()]}));
    for md in ["n", "c"] {
        run(&mut s, "polyProjection", json!({"args": ["p.f[*]"], "type": "planar", "mapDirection": md, "camera": cam}));
        let b = bounds(&mut s, "p");
        assert!(b[0] >= -1e-6 && b[2] <= 1.0 + 1e-6 && b[1] >= -1e-6 && b[3] <= 1.0 + 1e-6 && (close(b[2] - b[0], 1.0) || close(b[3] - b[1], 1.0)), "{md} {b:?}");
    }
    // Symmetrize: a plane of two faces, a UV on one side moved, mirrored across the middle edge.
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 1, "name": "q"}));
    let mut mid = None;
    for e in 0..7 {
        let pts = run(&mut s, "xform", json!({"args": [format!("q.e[{e}]")], "query": true, "translation": true, "worldSpace": true}));
        let l: Vec<f64> = pts.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
        if l.chunks(3).all(|p| p[0].abs() < 1e-6) {
            mid = Some(e);
        }
    }
    let mid = mid.expect("middle edge");
    // Map 0 moved up; its mirror is the map at the other end of its row.
    let all = uvs(&mut s, "q.map[*]");
    let start = (all[0], all[1]);
    let other = all.chunks(2).position(|p| close(p[0], 1.0 - start.0) && close(p[1], start.1)).expect("mirror map");
    run(&mut s, "polyEditUV", json!({"args": ["q.map[0]"], "v": 0.1}));
    let p0 = uv(&mut s, "q.map[0]");
    run(&mut s, "texSymmetrizeUVs", json!({"args": ["q.map[0]"], "edge": format!("q.e[{mid}]")}));
    let p2b = uv(&mut s, &format!("q.map[{other}]"));
    assert!(close(p2b.1, p0.1) && close(p2b.0, 1.0 - p0.0), "{p0:?} {p2b:?}");

    // Straighten Shell: the bottom row, out of line, runs straight again.
    run(&mut s, "polyEditUV", json!({"args": ["q.map[1]"], "v": 0.04}));
    run(&mut s, "texStraightenShell", json!({"args": ["q.map[1]"]}));
    let after = uvs(&mut s, "q.map[*]");
    let row: Vec<f64> = all.chunks(2).zip(after.chunks(2)).filter(|(b, _)| close(b[1], start.1)).map(|(_, a)| a[1]).collect();
    assert!(row.len() == 3 && row.iter().all(|v| close(*v, row[0])), "{row:?}");
}

#[test]
fn brushes() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 4, "subdivisionsZ": 4, "name": "p"}));
    let before = uvs(&mut s, "p.map[*]");
    // Grab at the middle: the UV there moves with the stroke, one far off does not.
    run(&mut s, "texBrushUVs", json!({"args": ["p"], "mode": "grab", "radius": 0.2, "samples": [0.5, 0.5, 0.1, 0.0, 0.6, 0.5, 0.05, 0.0]}));
    let after = uvs(&mut s, "p.map[*]");
    let mid = before.chunks(2).position(|p| close(p[0], 0.5) && close(p[1], 0.5)).unwrap();
    assert!(close(after[mid * 2] - 0.5, 0.15), "{}", after[mid * 2]);
    assert!(close(after[0], before[0]) && close(after[1], before[1]));
    // Pinned UVs hold.
    run(&mut s, "undo", json!({}));
    run(&mut s, "texBrushUVs", json!({"args": ["p"], "mode": "pin", "radius": 0.05, "samples": [0.5, 0.5, 0.0, 0.0]}));
    run(&mut s, "texBrushUVs", json!({"args": ["p"], "mode": "smear", "radius": 0.3, "samples": [0.5, 0.5, 0.1, 0.0]}));
    let after = uvs(&mut s, "p.map[*]");
    assert!(close(after[mid * 2], 0.5));
    // Smooth pulls a UV pushed out back toward its neighbours.
    run(&mut s, "polyEditUV", json!({"args": [format!("p.map[{}]", mid + 1)], "v": 0.1}));
    let pushed = uvs(&mut s, &format!("p.map[{}]", mid + 1))[1];
    run(&mut s, "texBrushUVs", json!({"args": ["p"], "mode": "smooth", "radius": 0.3, "samples": [0.75, 0.5, 0.0, 0.0, 0.75, 0.5, 0.0, 0.0]}));
    let smoothed = uvs(&mut s, &format!("p.map[{}]", mid + 1))[1];
    assert!((smoothed - 0.5).abs() < (pushed - 0.5).abs(), "{pushed} {smoothed}");
}

#[test]
fn lattice_moves_uvs() {
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 2, "subdivisionsZ": 2, "name": "p"}));
    let before = uvs(&mut s, "p.map[*]");
    // A 2x2 lattice over the tile, its top right point dragged right by 0.2: the top right UV goes with it, the
    // bottom left stays, the middle moves a quarter.
    run(&mut s, "texLatticeUVs", json!({"args": ["p.map[*]"], "columns": 2, "rows": 2, "box": [0.0, 0.0, 1.0, 1.0], "points": [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.2, 1.0]}));
    let after = uvs(&mut s, "p.map[*]");
    for (b, a) in before.chunks(2).zip(after.chunks(2)) {
        let want = b[0] + 0.2 * b[0] * b[1];
        assert!(close(a[0], want) && close(a[1], b[1]), "{b:?} {a:?} {before:?} {after:?}");
    }
}

#[test]
fn warp_image_between_sets() {
    let dir = std::env::temp_dir().join(format!("az_warp_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    // A 4x4 image: left half red, right half blue.
    let mut px = Vec::new();
    for _y in 0..4 {
        for x in 0..4 {
            px.push(if x < 2 { glam::Vec4::new(1.0, 0.0, 0.0, 1.0) } else { glam::Vec4::new(0.0, 0.0, 1.0, 1.0) });
        }
    }
    let src = dir.join("in.png");
    aztek_render::output::write_ldr(&src, 4, 4, &px, aztek_render::output::Format::Png).unwrap();
    let mut s = Session::new();
    run(&mut s, "polyPlane", json!({"subdivisionsX": 1, "subdivisionsZ": 1, "name": "p"}));
    run(&mut s, "polyUVSet", json!({"args": ["p"], "copy": true, "uvSet": "map1", "newUVSet": "flipped"}));
    run(&mut s, "polyUVSet", json!({"args": ["p"], "currentUVSet": true, "uvSet": "flipped"}));
    run(&mut s, "polyFlipUV", json!({"args": ["p.map[*]"], "flipType": 0}));
    let out = dir.join("out.png");
    run(&mut s, "polyWarpImage", json!({"args": ["p"], "inputName": src.to_string_lossy(), "outputName": out.to_string_lossy(), "inputUvSetName": "map1", "outputUvSetName": "flipped", "xResolution": 8, "yResolution": 8}));
    let cache = aztek_render::texture::TextureCache::new(16);
    let img = cache.image(&out.to_string_lossy(), aztek_render::texture::ColorSpace::Srgb).unwrap();
    // Flipped in U: blue on the left now.
    let left = img.sample(glam::Vec2::new(0.2, 0.5), 0.0);
    let right = img.sample(glam::Vec2::new(0.8, 0.5), 0.0);
    assert!(left.z > 0.8 && left.x < 0.2 && right.x > 0.8 && right.z < 0.2, "{left} {right}");
    let _ = std::fs::remove_dir_all(&dir);
}
