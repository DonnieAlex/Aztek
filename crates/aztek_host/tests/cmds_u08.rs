//! Commands of the reference from polySoftEdge to select: query and edit of the poly commands' history nodes,
//! polySphericalProjection's settings, polyStraightenUVBorder and polyTransfer as live nodes, polyUnsmooth
//! -replaceOriginal, polyUVStackSimilarShells' answer, the new polyUniteSkinned, the NURBS rebuild, reverse, revolve and
//! project commands' nodes and flags, rotate's and scale's component and pivot options, scaleComponents -rotation,
//! scaleConstraint -scaleCompensate, scriptJob -optionVarChanged and -disregardIndex, scriptNode -ignoreReferenceEdits,
//! scriptTable's callbacks and filters, scriptedPanelType's queries, layout margins, polySplitCtx2, proximityWrap's
//! driver queries, removeMultiInstance, resolutionNode, saveViewportSettings and select -containerCentric.

use aztek_host::Core;
use serde_json::{json, Value};

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) if a.len() == 1 => num(&a[0]),
        Value::Bool(b) => *b as i64 as f64,
        Value::String(s) => s.parse().unwrap_or_else(|_| panic!("not a number: {s}")),
        x => x.as_f64().unwrap_or_else(|| panic!("not a number: {x}")),
    }
}

fn nums(v: &Value) -> Vec<f64> {
    fn walk(v: &Value, out: &mut Vec<f64>) {
        match v {
            Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            Value::Bool(b) => out.push(*b as i64 as f64),
            x => out.extend(x.as_f64()),
        }
    }
    let mut out = Vec::new();
    walk(v, &mut out);
    out
}

fn strs(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Value::String(s) if !s.is_empty() => vec![s.clone()],
        _ => Vec::new(),
    }
}

fn close(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() < 1e-3, "{what}: {a} vs {b}");
}

#[test]
fn poly_history_nodes_query_and_edit() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n s -r 2;");
    close(num(&ok(&mut c, "polySphere -q -r s;")), 2.0, "sphere radius");
    ok(&mut c, "polySphere -e -sx 10 -sy 10 polySphere1;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f s;")), 100.0, "edited sphere");
    ok(&mut c, "polyTorus -n t -sx 8 -sy 6;");
    assert_eq!(num(&ok(&mut c, "polyTorus -q -sx t;")), 8.0);
    ok(&mut c, "polyTorus -e -sx 4 polyTorus1;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f t;")), 24.0);
    ok(&mut c, "polyCube -n a; polySoftEdge -a 30 a;");
    close(num(&ok(&mut c, "polySoftEdge -q -a polySoftEdge1;")), 30.0, "soft angle");
    ok(&mut c, "polySoftEdge -e -a 60 polySoftEdge1;");
    close(num(&ok(&mut c, "getAttr polySoftEdge1.angle;")), 60.0, "edited angle");
    ok(&mut c, "polyCube -n b; polyTriangulate b;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f b;")), 12.0);
    ok(&mut c, "polyTriangulate -e -nds 1 polyTriangulate1;");
    assert_eq!(num(&ok(&mut c, "polyTriangulate -q -nds polyTriangulate1;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f b;")), 6.0, "has no effect passes the cube through");
    ok(&mut c, "polyCube -n w; polyWedgeFace -wa 90 -d 4 -ed 4 w.f[0];");
    assert_eq!(num(&ok(&mut c, "polyWedgeFace -q -d polyWedgeFace1;")), 4.0);
    ok(&mut c, "polyWedgeFace -e -d 2 polyWedgeFace1;");
    assert_eq!(num(&ok(&mut c, "getAttr polyWedgeFace1.divisions;")), 2.0);
    ok(&mut c, "polyCube -n e1; polySplitRing -wt 0.25 e1.e[1];");
    close(num(&ok(&mut c, "polySplitRing -q -wt polySplitRing1;")), 0.25, "split ring weight");
    ok(&mut c, "polySplitRing -e -wt 0.75 polySplitRing1;");
    close(num(&ok(&mut c, "getAttr polySplitRing1.weight;")), 0.75, "edited weight");
    ok(&mut c, "polyCube -n u1; polyCube -n u2; move 2 0 0 u2; polyUnite -n uu u1 u2;");
    ok(&mut c, "polyUnite -e -cch 1 polyUnite1;");
    assert_eq!(ok(&mut c, "polyUnite -q -cch polyUnite1;"), json!(true));
    ok(&mut c, "polyCube -n sv; polySplitVertex sv.vtx[0];");
    assert_eq!(num(&ok(&mut c, "polySplitVertex -q -nds polySplitVert1;")), 0.0);
}

#[test]
fn spherical_projection_settings() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n s;");
    let node = strs(&ok(&mut c, "polySphericalProjection -ps 90 45 -rx 10 s.f[*];"));
    assert_eq!(node, vec!["polySphProj1"]);
    close_all(&nums(&ok(&mut c, "polySphericalProjection -q -ps polySphProj1;")), &[90.0, 45.0], "ps");
    close(num(&ok(&mut c, "polySphericalProjection -q -psu polySphProj1;")), 90.0, "psu");
    close(num(&ok(&mut c, "polySphericalProjection -q -phs polySphProj1;")), 180.0, "the sweep is twice psu");
    close(num(&ok(&mut c, "polySphericalProjection -q -psv polySphProj1;")), 45.0, "psv");
    close(num(&ok(&mut c, "polySphericalProjection -q -rx polySphProj1;")), 10.0, "rx");
    let before = nums(&ok(&mut c, "polyEditUV -q s.map[5];"));
    ok(&mut c, "polySphericalProjection -e -ro 0 45 0 -ic 0.25 0.5 -is 2 1 polySphProj1;");
    close_all(&nums(&ok(&mut c, "polySphericalProjection -q -ro polySphProj1;")), &[0.0, 45.0, 0.0], "ro");
    close_all(&nums(&ok(&mut c, "polySphericalProjection -q -ic polySphProj1;")), &[0.25, 0.5], "ic");
    close_all(&nums(&ok(&mut c, "polySphericalProjection -q -is polySphProj1;")), &[2.0, 1.0], "is");
    let after = nums(&ok(&mut c, "polyEditUV -q s.map[5];"));
    assert!((before[0] - after[0]).abs() + (before[1] - after[1]).abs() > 1e-4, "the edit projects again: {before:?} {after:?}");
    ok(&mut c, "polySphericalProjection -e -r 3 -pc 0 1 0 -ws 1 polySphProj1;");
    close(num(&ok(&mut c, "polySphericalProjection -q -r polySphProj1;")), 3.0, "radius");
    close_all(&nums(&ok(&mut c, "polySphericalProjection -q -pc polySphProj1;")), &[0.0, 1.0, 0.0], "pc");
    assert_eq!(ok(&mut c, "polySphericalProjection -q -ws polySphProj1;"), json!(true));
    ok(&mut c, "polySphericalProjection -e -cch 1 -nds 0 polySphProj1;");
    assert_eq!(ok(&mut c, "polySphericalProjection -q -cch polySphProj1;"), json!(true));
    ok(&mut c, "polySphere -n s2; polySphericalProjection -n myProj -pi 1 s2.f[*];");
    assert_eq!(ok(&mut c, "objExists myProj;"), json!(true));
}

fn close_all(a: &[f64], b: &[f64], what: &str) {
    assert_eq!(a.len(), b.len(), "{what}: {a:?} vs {b:?}");
    for i in 0..a.len() {
        assert!((a[i] - b[i]).abs() < 1e-3, "{what}: {a:?} vs {b:?}");
    }
}

#[test]
fn straighten_border_is_a_live_node() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p -sx 4 -sy 1 -w 4 -h 1;");
    // The bottom row's UVs are maps 0 1 4 6 8: bend two, then straighten the row.
    ok(&mut c, "polyEditUV -r 1 -v 0.1 p.map[4]; polyEditUV -r 1 -v -0.05 p.map[6];");
    let node = strs(&ok(&mut c, "polyStraightenUVBorder -c 0 -uvs map1 p.map[0] p.map[1] p.map[4] p.map[6] p.map[8];"));
    assert_eq!(node.len(), 1);
    assert_eq!(ok(&mut c, &format!("nodeType {};", node[0])), json!("polyStraightenUVBorder"));
    let row = [0, 1, 4, 6, 8];
    let v: Vec<f64> = row.iter().map(|i| nums(&ok(&mut c, &format!("polyEditUV -q p.map[{i}];")))[1]).collect();
    for x in &v {
        close(*x, v[0], "straight");
    }
    close(num(&ok(&mut c, &format!("polyStraightenUVBorder -q -c {};", node[0]))), 0.0, "curvature");
    ok(&mut c, &format!("polyStraightenUVBorder -e -c 1 {};", node[0]));
    let mid = nums(&ok(&mut c, "polyEditUV -q p.map[4];"))[1];
    assert!((mid - v[0]).abs() > 0.1, "bowed by the edit: {mid} vs {}", v[0]);
    ok(&mut c, &format!("polyStraightenUVBorder -e -ws 1 -bo 1 {};", node[0]));
    assert_eq!(ok(&mut c, &format!("polyStraightenUVBorder -q -ws {};", node[0])), json!(true));
    close(nums(&ok(&mut c, "polyEditUV -q p.map[4];"))[1], 0.1, "blend 1 leaves the UVs where they were");
}

#[test]
fn transfer_follows_the_other_mesh() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; polyCube -n d; move -r 0 3 0 d.vtx[0];");
    let t = ok(&mut c, "polyTransfer -v 1 -uv 0 -ao d c;");
    assert_eq!(t, json!("polyTransfer1"));
    close(nums(&ok(&mut c, "pointPosition -l c.vtx[0];"))[1], 2.5, "copied");
    ok(&mut c, "move -r 0 1 0 d.vtx[0];");
    close(nums(&ok(&mut c, "pointPosition -l c.vtx[0];"))[1], 3.5, "follows the other mesh");
    assert_eq!(ok(&mut c, "polyTransfer -q -ao polyTransfer1;"), json!("d"));
    assert_eq!(ok(&mut c, "polyTransfer -q -v polyTransfer1;"), json!(true));
    ok(&mut c, "polyCube -n e;");
    ok(&mut c, "polyTransfer -e -ao e polyTransfer1;");
    assert_eq!(ok(&mut c, "polyTransfer -q -ao polyTransfer1;"), json!("e"));
    close(nums(&ok(&mut c, "pointPosition -l c.vtx[0];"))[1], -0.5, "the new mesh's");
    ok(&mut c, "polyTransfer -e -v 0 polyTransfer1;");
    assert_eq!(ok(&mut c, "polyTransfer -q -v polyTransfer1;"), json!(false));
}

#[test]
fn unsmooth_on_a_copy_and_stack_shells() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n e; polySmooth -dv 1 e;");
    let n = ok(&mut c, "polyUnsmooth -rpo 0 e;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f e;")), 24.0, "the original stays");
    assert_eq!(strs(&n), vec!["polyUnsmooth1"]);
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f e1;")), 6.0, "the copy is unsmoothed");
    assert_eq!(num(&ok(&mut c, "polyUnsmooth -q -dlv polyUnsmooth1;")), 1.0);
    ok(&mut c, "polyCube -n f; polySmooth -dv 1 f; polyUnsmooth f;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f f;")), 6.0, "in place by default");

    ok(&mut c, "polyPlane -n p1 -sx 1 -sy 1; polyPlane -n p2 -sx 1 -sy 1; move 3 0 0 p2; polyUnite -ch 0 -n pp p1 p2;");
    ok(&mut c, "polyEditUV -u 2 -v 0 pp.map[4:7];");
    let matched = strs(&ok(&mut c, "polyUVStackSimilarShells -om -to 0.01 pp;"));
    assert_eq!(matched.len(), 4, "the target shell's UVs: {matched:?}");
    let moved = nums(&ok(&mut c, "polyEditUV -q pp.map[4];"));
    close(moved[0], 2.0, "only matched, not moved");
    let stacked = strs(&ok(&mut c, "polyUVStackSimilarShells pp;"));
    assert_eq!(stacked.len(), 4, "the stacked shell's UVs: {stacked:?}");
    let a = nums(&ok(&mut c, "polyEvaluate -bc2 pp.map[0:3];"));
    let b = nums(&ok(&mut c, "polyEvaluate -bc2 pp.map[4:7];"));
    close_all(&a, &b, "stacked");
}

#[test]
fn unite_skinned_keeps_the_skin() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -n j1 -p 0 0 0; joint -n j2 -p 0 2 0; select -cl; joint -n k1 -p 4 0 0;");
    ok(&mut c, "polyCube -n m1; move 0 1 0 m1; skinCluster -tsb j1 j2 m1;");
    ok(&mut c, "polyCube -n m2; move 4 0 0 m2; skinCluster -tsb k1 m2;");
    let r = strs(&ok(&mut c, "polyUniteSkinned -cp m1 m2;"));
    assert!(!r.is_empty(), "{r:?}");
    let out = r[0].clone();
    assert_eq!(num(&ok(&mut c, &format!("polyEvaluate -v {out};"))), 16.0);
    let sk = strs(&ok(&mut c, &format!("ls -type skinCluster `listHistory {out}`;")));
    let sk: Vec<String> = sk.into_iter().filter(|x| x != "skinCluster1" && x != "skinCluster2").collect();
    assert_eq!(sk.len(), 1, "a new skin on the result");
    let inf = strs(&ok(&mut c, &format!("skinCluster -q -inf {};", sk[0])));
    for j in ["j1", "j2", "k1"] {
        assert!(inf.iter().any(|x| x == j), "{j} in {inf:?}");
    }
    // The result follows the joints: k1 moves the second cube's points.
    let before = nums(&ok(&mut c, &format!("pointPosition -w {out}.vtx[8];")));
    ok(&mut c, "move -r 0 5 0 k1;");
    let after = nums(&ok(&mut c, &format!("pointPosition -w {out}.vtx[8];")));
    close(after[1] - before[1], 5.0, "deformed by k1");
    let piv = nums(&ok(&mut c, &format!("xform -q -ws -rp {out};")));
    close(piv[0], 2.0, "centre pivot");
    assert_eq!(num(&ok(&mut c, "polyUniteSkinned -q -muv polyUnite1;")), 1.0);
}

#[test]
fn rebuild_reverse_and_project() {
    let mut c = Core::new();
    ok(&mut c, "curve -n c1 -d 3 -p 0 0 0 -p 1 1 0 -p 2 0 0 -p 3 1 0 -p 4 0 0 -p 5 1 0;");
    ok(&mut c, "rebuildCurve -s 6 -kr 0 c1;");
    assert_eq!(num(&ok(&mut c, "getAttr c1.spans;")), 6.0);
    close(num(&ok(&mut c, "getAttr c1.maxValue;")), 1.0, "range 0 to 1");
    assert_eq!(num(&ok(&mut c, "rebuildCurve -q -kr rebuildCurve1;")), 0.0);
    assert_eq!(ok(&mut c, "rebuildCurve -q -fr rebuildCurve1;"), json!(true));
    ok(&mut c, "rebuildCurve -e -kr 2 rebuildCurve1;");
    close(num(&ok(&mut c, "getAttr c1.maxValue;")), 6.0, "range 0 to spans");
    ok(&mut c, "rebuildCurve -e -fr 0 -scr 1 rebuildCurve1;");
    assert_eq!(ok(&mut c, "rebuildCurve -q -scr rebuildCurve1;"), json!(true));
    ok(&mut c, "rebuildCurve -e -nds 1 rebuildCurve1;");
    assert_eq!(num(&ok(&mut c, "getAttr c1.spans;")), 3.0, "has no effect: the input");
    ok(&mut c, "rebuildCurve -e -cch 1 -nds 0 rebuildCurve1;");
    assert_eq!(ok(&mut c, "rebuildCurve -q -cch rebuildCurve1;"), json!(true));
    ok(&mut c, "curve -n c2 -d 1 -p 0 0 0 -p 1 0 0 -p 2 0 0 -p 3 0 0 -p 4 0 0;");
    ok(&mut c, "rebuildCurve -rt 1 -d 3 -tol 0.01 -rn 1 c2;");
    assert_eq!(num(&ok(&mut c, "getAttr c2.spans;")), 1.0, "a straight line needs one span");
    // Match knots.
    ok(&mut c, "curve -n c3 -d 3 -p 0 0 0 -p 1 1 0 -p 2 0 0 -p 3 1 0; curve -n ref -d 3 -p 0 0 1 -p 1 0 1 -p 2 0 1 -p 3 0 1 -p 4 0 1 -p 5 0 1 -p 6 0 1;");
    ok(&mut c, "rebuildCurve -rt 2 c3 ref;");
    assert_eq!(num(&ok(&mut c, "getAttr c3.spans;")), num(&ok(&mut c, "getAttr ref.spans;")));

    ok(&mut c, "curve -n r1 -d 1 -p 0 0 0 -p 1 0 0; reverseCurve -cos 0 -rn 1 r1;");
    ok(&mut c, "reverseCurve -e -nds 1 reverseCurve1;");
    assert_eq!(num(&ok(&mut c, "reverseCurve -q -nds reverseCurve1;")), 1.0);
    close(nums(&ok(&mut c, "pointPosition r1.cv[0];"))[0], 0.0, "passes through");
    ok(&mut c, "reverseCurve -e -nds 0 reverseCurve1;");
    close(nums(&ok(&mut c, "pointPosition r1.cv[0];"))[0], 1.0, "reversed");
    ok(&mut c, "nurbsPlane -n sp; reverseSurface -d 0 sp; rebuildSurface -su 3 sp;");
    ok(&mut c, "reverseSurface -e -cch 1 reverseSurface1; rebuildSurface -e -nds 2 rebuildSurface1;");
    assert_eq!(ok(&mut c, "reverseSurface -q -cch reverseSurface1;"), json!(true));
    assert_eq!(num(&ok(&mut c, "rebuildSurface -q -nds rebuildSurface1;")), 2.0);

    ok(&mut c, "nurbsPlane -n np -ax 0 1 0 -w 10; curve -n pc -d 1 -p 0 3 0 -p 1 3 1;");
    let r = strs(&ok(&mut c, "projectCurve -dx 0 -dy -1 -dz 0 np pc;"));
    assert_eq!(r.len(), 2, "{r:?}");
    close(num(&ok(&mut c, "projectCurve -q -dy projectCurve1;")), -1.0, "dy");
    ok(&mut c, "projectCurve -e -dz 1 -cch 1 projectCurve1;");
    close_all(&nums(&ok(&mut c, "projectCurve -q -d projectCurve1;")), &[0.0, -1.0, 1.0], "direction");
}

#[test]
fn project_tangent_curvature() {
    let mut c = Core::new();
    ok(&mut c, "sphere -n sph -r 5; curve -n t1 -d 3 -p 5 0 0 -p 6 1 0 -p 7 2 0 -p 8 3 0;");
    ok(&mut c, "projectTangent -ts 2 t1 sph;");
    close(num(&ok(&mut c, "projectTangent -q -ts projectTangent1;")), 2.0, "ts");
    let flat = nums(&ok(&mut c, "pointPosition t1.cv[2];"));
    ok(&mut c, "projectTangent -e -c 1 projectTangent1;");
    assert_eq!(ok(&mut c, "projectTangent -q -c projectTangent1;"), json!(true));
    let bent = nums(&ok(&mut c, "pointPosition t1.cv[2];"));
    let d: f64 = flat.iter().zip(&bent).map(|(a, b)| (a - b).abs()).sum();
    assert!(d > 1e-4, "curvature moves the third CV: {flat:?} {bent:?}");
    ok(&mut c, "projectTangent -e -cs 1 -nds 1 projectTangent1;");
    close(num(&ok(&mut c, "projectTangent -q -cs projectTangent1;")), 1.0, "cs");
    close(nums(&ok(&mut c, "pointPosition t1.cv[1];"))[0], 6.0, "has no effect");
    ok(&mut c, "curve -n a1 -d 1 -p 0 0 0 -p 0 2 0; curve -n a2 -d 1 -p -1 0 0 -p 1 0 0; curve -n a3 -d 1 -p 0 0 -1 -p 0 0 1;");
    ok(&mut c, "projectTangent -td 2 a1 a2 a3;");
    let p = nums(&ok(&mut c, "pointPosition a1.cv[1];"));
    close(p[0], 0.0, "along the second curve");
    assert!(p[2].abs() > 1.0, "{p:?}");
}

#[test]
fn revolve_flags() {
    let mut c = Core::new();
    ok(&mut c, "curve -n prof -d 1 -p 1 0 0 -p 1 2 0;");
    let r = strs(&ok(&mut c, "revolve -s 8 prof;"));
    close(num(&ok(&mut c, "revolve -q -axx revolve1;")), 1.0, "X by default");
    let bb = nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", r[0])));
    close(bb[0], 1.0, "a disc about X stays at x 1");
    ok(&mut c, "revolve -e -ax 0 1 0 revolve1;");
    let bb = nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", r[0])));
    assert!(bb[0] < -0.9, "about Y now: {bb:?}");
    ok(&mut c, "revolve -e -px 1 revolve1;");
    close(num(&ok(&mut c, "revolve -q -pivotX revolve1;")), 1.0, "pivot x");
    ok(&mut c, "revolve -ax 0 1 0 -ssw 0 -esw 90 -br 1 -n pie prof;");
    assert_eq!(num(&ok(&mut c, "getAttr pie.formV;")), 1.0, "bridged: closed");
    assert_eq!(ok(&mut c, "revolve -q -br revolve2;"), json!(true));
    ok(&mut c, "revolve -cpa 2 -r 2 -n auto prof;");
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox auto;"));
    // The axis along the curve, radius 2 from it: the surface is about x -1, reaching from about -3 to 1.
    close((bb[0] + bb[3]) * 0.5, -1.0, "computed axis");
    assert!(bb[3] - bb[0] > 3.9, "{bb:?}");
    close(num(&ok(&mut c, "revolve -q -r revolve3;")), 2.0, "radius");
    close(num(&ok(&mut c, "revolve -q -ra revolve3;")), -1.0, "radius anchor");
    ok(&mut c, "revolve -ax 0 1 0 -ut 1 -tol 0.0001 -n fine prof;");
    let fine = num(&ok(&mut c, "getAttr fine.spansV;"));
    ok(&mut c, "revolve -e -tol 0.1 revolve4;");
    let coarse = num(&ok(&mut c, "getAttr fine.spansV;"));
    assert!(fine > coarse, "the tolerance sets the sections: {fine} {coarse}");
    ok(&mut c, "revolve -ax 0 1 0 -acn 1 -ulp 1 -aco 1 -rb 1 -n acn prof;");
    assert_eq!(ok(&mut c, "revolve -q -acn revolve5;"), json!(true));
    assert_eq!(ok(&mut c, "revolve -q -ulp revolve5;"), json!(true));
    assert_eq!(num(&ok(&mut c, "revolve -q -aco revolve5;")), 1.0);
    assert!(!strs(&ok(&mut c, "ls -type rebuildCurve;")).is_empty(), "-rb rebuilds the profile first");
    let only = strs(&ok(&mut c, "revolve -o 0 -ax 0 1 0 prof;"));
    assert_eq!(only.len(), 1, "only the node: {only:?}");
    ok(&mut c, "revolve -e -nds 2 -cch 1 revolve1;");
    assert_eq!(num(&ok(&mut c, "revolve -q -nds revolve1;")), 2.0);
}

#[test]
fn rotate_and_scale_options() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; move 2 0 0 a; polyCube -n b; move 4 0 0 b;");
    // -cp: about the middle of both (x 3).
    ok(&mut c, "rotate -r -cp 0 180 0 a b;");
    let ta = nums(&ok(&mut c, "xform -q -ws -rp a;"));
    close(ta[0], 4.0, "a turned round the middle");
    // Without -t the move goes into rotatePivotTranslate; with it, into translate.
    let rpt = nums(&ok(&mut c, "getAttr a.rotatePivotTranslate;"));
    close(rpt[0], 2.0, "rotate pivot translation");
    close(nums(&ok(&mut c, "getAttr a.translate;"))[0], 2.0, "translate kept");
    ok(&mut c, "polyCube -n t1; move 2 0 0 t1; rotate -r -p 0 0 0 -t 0 180 0 t1;");
    close(nums(&ok(&mut c, "getAttr t1.translate;"))[0], -2.0, "-t moves translate");
    // -oa: about axes turned 90 degrees about Z: a turn about X is one about world Y.
    ok(&mut c, "polyCube -n o; rotate -r -oa 0 0 90 90 0 0 o;");
    let m = nums(&ok(&mut c, "xform -q -ws -m o;"));
    close(m[0].abs(), 0.0, "x axis turned off x");
    // -ocp: each about its own middle.
    ok(&mut c, "polyCube -n s1; move 5 0 0 s1; scale -r -ocp 2 2 2 s1;");
    close(nums(&ok(&mut c, "xform -q -ws -rp s1;"))[0], 5.0, "scaled in place");
    // scale -cp about the middle of both.
    ok(&mut c, "polyCube -n q1; polyCube -n q2; move 2 0 0 q2; scale -r -cp 2 1 1 q1 q2;");
    close(nums(&ok(&mut c, "xform -q -ws -rp q2;"))[0], 3.0, "q2 moved out from the middle");
    // Components with reflection: the mirrored point moves too.
    ok(&mut c, "polyPlane -n p -sx 2 -sy 1 -w 2 -h 1;");
    let x0 = nums(&ok(&mut c, "pointPosition -w p.vtx[0];"))[0];
    let x2 = nums(&ok(&mut c, "pointPosition -w p.vtx[2];"))[0];
    ok(&mut c, "scale -r -rfl -rax -p 0 0 0 2 1 1 p.vtx[0];");
    close(nums(&ok(&mut c, "pointPosition -w p.vtx[0];"))[0], x0 * 2.0, "scaled");
    close(nums(&ok(&mut c, "pointPosition -w p.vtx[2];"))[0], x2 * 2.0, "its mirror image too");
    ok(&mut c, "polyCube -n uv; rotate -r -puv -p 0 0 0 0 90 0 uv.vtx[0];");
    let p = nums(&ok(&mut c, "pointPosition -w uv.vtx[0];"));
    close(p[0], 0.5, "turned");
    close(p[2], 0.5, "turned");
}

#[test]
fn scale_components_rotation() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n k; select -r k.vtx[0];");
    let before = nums(&ok(&mut c, "pointPosition -w k.vtx[0];"));
    ok(&mut c, "scaleComponents -p 0 0 0 2 2 2;");
    let after = nums(&ok(&mut c, "pointPosition -w k.vtx[0];"));
    close_all(&after, &before.iter().map(|x| x * 2.0).collect::<Vec<_>>(), "about the origin");
    ok(&mut c, "scaleComponents -p 0 0 0 -ro 0 0 45 1 1 1;");
    close_all(&nums(&ok(&mut c, "pointPosition -w k.vtx[0];")), &after, "a scale of one changes nothing");
}

#[test]
fn scale_constraint_compensates() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -n root; joint -n tip -p 0 2 0; spaceLocator -n tgt; scale 2 2 2 tgt;");
    ok(&mut c, "setAttr root.scale 3 3 3;");
    ok(&mut c, "scaleConstraint -sc 1 tgt tip;");
    let src = strs(&ok(&mut c, "listConnections -p 1 -s 1 -d 0 tip_scaleConstraint1.constraintScaleCompensate;"));
    assert_eq!(src, vec!["tip.segmentScaleCompensate"]);
    close(nums(&ok(&mut c, "getAttr tip.scale;"))[0], 2.0, "the parent's scale is not taken out");
    ok(&mut c, "scaleConstraint -e -sc 0 tgt tip;");
    close(nums(&ok(&mut c, "getAttr tip.scale;"))[0], 2.0 / 3.0, "taken out again");
}

#[test]
fn script_jobs_and_nodes() {
    let mut c = Core::new();
    ok(&mut c, "global int $gOvc = 0; global proc u08ovc() { global int $gOvc; $gOvc++; } optionVar -iv u08Var 1;");
    let id = num(&ok(&mut c, "scriptJob -ovc u08Var u08ovc;"));
    c.idle();
    ok(&mut c, "optionVar -iv u08Var 2;");
    c.idle();
    assert_eq!(num(&ok(&mut c, "$gOvc = $gOvc;")), 1.0, "fired once");
    ok(&mut c, "optionVar -rm u08Var;");
    c.idle();
    assert_eq!(ok(&mut c, &format!("scriptJob -ex {id};")), json!(false), "the job ends with the optionVar");
    ok(&mut c, "createNode plusMinusAverage -n pc; setAttr pc.input1D[0] 1; setAttr pc.input1D[3] 1; global int $gDri = 0; global proc u08dri() { global int $gDri; $gDri++; }");
    ok(&mut c, "scriptJob -dri -ac pc.input1D[0] u08dri;");
    c.idle();
    ok(&mut c, "setAttr pc.input1D[3] 5;");
    c.idle();
    assert!(num(&ok(&mut c, "$gDri = $gDri;")) >= 1.0, "another element counts");
    ok(&mut c, "scriptNode -ire -bs \"print 1\" -n sn;");
    assert_eq!(ok(&mut c, "getAttr sn.ignoreReferenceEdits;"), json!(true));
}

#[test]
fn script_table_filters_colours_and_removal() {
    let mut c = Core::new();
    ok(&mut c, "global proc string u08cell(int $r, int $c) { return (\"r\" + $r); }");
    ok(&mut c, "global proc int[] u08bg(int $r, int $c) { return {255, 0, $r}; }");
    ok(&mut c, "global int $gRemoved[]; global proc int u08may(int $rows[]) { return 1; } global proc u08gone(int $rows[]) { global int $gRemoved[]; $gRemoved = $rows; }");
    ok(&mut c, "window w; columnLayout col;");
    ok(&mut c, "scriptTable -rows 3 -columns 1 -gcc u08cell -cbc u08bg -rtc u08may -rrc u08gone -sb 1 -sm 2 -ed 0 -udc 1 t;");
    c.idle();
    assert_eq!(ok(&mut c, "scriptTable -q -sb t;"), json!(1));
    assert_eq!(ok(&mut c, "scriptTable -q -sm t;"), json!(2));
    assert_eq!(ok(&mut c, "scriptTable -q -ed t;"), json!(false));
    assert_eq!(ok(&mut c, "scriptTable -q -udc t;"), json!(true));
    aztek_host::scriptui::take_ops();
    ok(&mut c, "scriptTable -e -cf 1 \"r[12]\" t;");
    let ops = aztek_host::scriptui::take_ops();
    let hidden = ops.iter().find_map(|o| o["props"].get("hiddenRows").cloned()).expect("hidden rows sent");
    assert_eq!(hidden, json!([3]));
    ok(&mut c, "scriptTable -e -cf 1 \"\" t;");
    c.ui_event(&json!({"path": "w|col|t", "rowsRemove": [2]}));
    assert_eq!(ok(&mut c, "scriptTable -q -rows t;"), json!(2));
    assert_eq!(ok(&mut c, "scriptTable -ci 2 1 -q -cv t;"), json!("r3"));
    assert_eq!(nums(&ok(&mut c, "$gRemoved = $gRemoved;")), vec![2.0]);
    ok(&mut c, "scriptTable -e -ct t;");
    assert_eq!(ok(&mut c, "scriptTable -q -rows t;"), json!(0));
}

#[test]
fn panels_layouts_and_controls() {
    let mut c = Core::new();
    ok(&mut c, "scriptedPanelType -ccb u08Create -ocb u08Copy -hkc u08Keys u08Type;");
    assert_eq!(ok(&mut c, "scriptedPanelType -q -ocb u08Type;"), json!("u08Copy"));
    assert_eq!(ok(&mut c, "scriptedPanelType -q -hkc u08Type;"), json!("u08Keys"));
    ok(&mut c, "scriptedPanelType -e -hkc other u08Type;");
    assert_eq!(ok(&mut c, "scriptedPanelType -q -hkc u08Type;"), json!("other"));
    assert_eq!(ok(&mut c, "scriptedPanelType -q -ccb u08Type;"), json!("u08Create"), "an edit keeps the rest");
    ok(&mut c, "scriptedPanelType u08Plain;");
    assert_eq!(ok(&mut c, "scriptedPanelType -q -hkc u08Plain;"), json!("u08Plain"), "the type's name by default");
    ok(&mut c, "window w; rowLayout -nc 1 -ad1 1 -gsp 4 -mar 2 rl;");
    ok(&mut c, "rowLayout -e -mr4 1 2 3 4 rl; setParent ..;");
    ok(&mut c, "rowColumnLayout -nc 2 -gsp 3 rcl; rowColumnLayout -e -mar 5 rcl; setParent ..;");
    ok(&mut c, "scrollLayout -bv 1 -gsp 2 sl; scrollLayout -e -mr4 0 1 0 1 sl;");
    assert_eq!(ok(&mut c, "scrollLayout -q -bv sl;"), json!(true));
    ok(&mut c, "popupMenu -sh 1 pm;");
    assert_eq!(ok(&mut c, "popupMenu -q -sh pm;"), json!(true));
    ok(&mut c, "setParent w; columnLayout; rangeControl -wh 200 20 rc; rangeControl -e -wh 300 20 rc;");
    assert_eq!(ok(&mut c, "rangeControl -ex rc;"), json!(true));

    ok(&mut c, "polySplitCtx2 -ief 1 -aef 0.5 -em 3 -st 0.2 -svc 1 0 0 sc2;");
    assert_eq!(ok(&mut c, "polySplitCtx2 -ex sc2;"), json!(true));
    assert_eq!(ok(&mut c, "polySplitCtx2 -q -ief sc2;"), json!(true));
    close(num(&ok(&mut c, "polySplitCtx2 -q -aef sc2;")), 0.5, "edge flow");
    ok(&mut c, "polySplitCtx2 -e -cte 1 -sec 0 1 0 sc2;");
    assert_eq!(ok(&mut c, "polySplitCtx2 -q -cte sc2;"), json!(true));
    close_all(&nums(&ok(&mut c, "polySplitCtx2 -q -sec sc2;")), &[0.0, 1.0, 0.0], "edge colour");
    close_all(&nums(&ok(&mut c, "polySplitCtx2 -q -sfc sc2;")), &[0.86, 0.86, 0.86], "face colour default");
    ok(&mut c, "setToolTo sc2;");
}

#[test]
fn proximity_wrap_drivers() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n body; polyCube -n d1; polyCube -n d2; polyCube -n d3;");
    let n = strs(&ok(&mut c, "proximityWrap body;"))[0].clone();
    ok(&mut c, &format!("proximityWrap -e -ad d1 -ad d2 {n};"));
    assert_eq!(nums(&ok(&mut c, &format!("proximityWrap -q -di {n};"))), vec![0.0, 1.0]);
    assert_eq!(num(&ok(&mut c, &format!("proximityWrap -q -fdi {n};"))), 2.0);
    assert_eq!(ok(&mut c, &format!("proximityWrap -q -cba d3 {n};")), json!(true));
    assert_eq!(ok(&mut c, &format!("proximityWrap -q -cba d1 {n};")), json!(false), "already a driver");
    assert_eq!(ok(&mut c, &format!("proximityWrap -q -cba body {n};")), json!(false), "the deformed mesh");
    ok(&mut c, &format!("proximityWrap -e -rd d1 {n};"));
    assert_eq!(nums(&ok(&mut c, &format!("proximityWrap -q -di {n};"))), vec![1.0]);
    assert_eq!(num(&ok(&mut c, &format!("proximityWrap -q -fdi {n};"))), 0.0);
    ok(&mut c, &format!("proximityWrap -e -aud 0 -ad d3 {n};"));
    let info = ok(&mut c, &format!("proximityWrap -q -dui {n};"));
    assert!(info.to_string().contains("dShape3"), "{info}");
}

#[test]
fn misc_commands() {
    let mut c = Core::new();
    ok(&mut c, "createNode plusMinusAverage -n ch; setAttr ch.input1D[4] 1; polyCube -n pc; connectAttr pc.tx ch.input1D[5];");
    assert_eq!(ok(&mut c, "removeMultiInstance ch.input1D[4];"), json!(true));
    assert_eq!(ok(&mut c, "removeMultiInstance ch.input1D[5];"), json!(false), "connected, no -b");
    assert_eq!(ok(&mut c, "removeMultiInstance -b true ch.input1D[5];"), json!(true));
    assert_eq!(nums(&ok(&mut c, "getAttr -mi ch.input1D;")).len(), 0);
    ok(&mut c, "polyCube -n m; cluster -n c1 m; nonLinear -type bend -n b1 m;");
    ok(&mut c, "reorderDeformers -n unused c1 b1 m;");
    let r = ok(&mut c, "resolutionNode -n customRes;");
    assert_eq!(r, json!("customRes"));
    assert_eq!(ok(&mut c, "nodeType customRes;"), json!("resolution"));
    assert_eq!(ok(&mut c, "resolutionNode -s -n customRes;"), json!("customRes"), "shared: the one there");
    ok(&mut c, "saveViewportSettings;");
    assert_eq!(ok(&mut c, "optionVar -ex modelPanel4ViewportSettings;"), json!(true));
    ok(&mut c, "refresh -cv -fe jpg -fn \"/nonexistent/u08shot\";");
    ok(&mut c, "polyCube -n inside; container -n box -addNode inside; setAttr box.blackBox 1;");
    ok(&mut c, "select -cc inside;");
    assert_eq!(strs(&ok(&mut c, "ls -sl;")), vec!["box"]);
    ok(&mut c, "select inside;");
    assert_eq!(strs(&ok(&mut c, "ls -sl;")), vec!["inside"]);
}
