//! Commands of the reference from polyExtrudeFacet to polySmooth: query and edit of each command's history node, UV
//! nodes made again when their settings change, the flags the others gained (polyGeoSampler's sampling, polyHole's and
//! polyPinUV's history, polyMultiLayoutUV, polyOutput's parts, polyQueryBlindData's values, polySelect's query,
//! polySewEdge's tolerance, polySlideEdge's -ed, polyOptions -dcl, polyProjectCurve's node, polyHelix's old placement,
//! numderOfSides), and the new polySmartExtrude and polySelectConstraintMonitor.

use aztek_host::Core;
use serde_json::Value;

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) if a.len() == 1 => num(&a[0]),
        Value::Bool(b) => *b as i64 as f64,
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

fn close_all(a: &[f64], b: &[f64], what: &str) {
    assert_eq!(a.len(), b.len(), "{what}: {a:?} vs {b:?}");
    for i in 0..a.len() {
        assert!((a[i] - b[i]).abs() < 1e-3, "{what}: {a:?} vs {b:?}");
    }
}

fn uv(c: &mut Core, map: &str) -> Vec<f64> {
    nums(&ok(c, &format!("polyEditUV -q {map};")))
}

#[test]
fn modelling_nodes_query_and_edit() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyExtrudeFacet -ltz 1 a.f[1];");
    close(num(&ok(&mut c, "polyExtrudeFacet -q -ltz polyExtrudeFace1;")), 1.0, "extrude ltz");
    close(num(&ok(&mut c, "polyExtrudeFacet -q -magnX polyExtrudeFace1;")), 0.0, "magnet x");
    ok(&mut c, "polyExtrudeFacet -e -ltz 3 polyExtrudeFace1;");
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox a;"));
    close(bb[4], 3.5, "extruded up by the edit");
    ok(&mut c, "polyCube -n b; polyExtrudeVertex -l 0.2 b.vtx[0];");
    close(num(&ok(&mut c, "polyExtrudeVertex -q -l polyExtrudeVertex1;")), 0.2, "extrude vertex length");
    ok(&mut c, "polyExtrudeVertex -e -w 0.3 polyExtrudeVertex1;");
    close(num(&ok(&mut c, "getAttr polyExtrudeVertex1.width;")), 0.3, "edited width");
    ok(&mut c, "polyCube -n v; polyMoveVertex -tx 1 v.vtx[0];");
    ok(&mut c, "polyMoveVertex -e -tx 2 polyMoveVertex1;");
    close(num(&ok(&mut c, "polyMoveVertex -q -tx polyMoveVertex1;")), 2.0, "move vertex tx");
    close(nums(&ok(&mut c, "pointPosition -w v.vtx[0];"))[0], 1.5, "moved by the edit");
    ok(&mut c, "polyCube -n sm; polySmooth -dv 1 sm;");
    assert_eq!(num(&ok(&mut c, "polySmooth -q -dv polySmoothFace1;")), 1.0);
    ok(&mut c, "polySmooth -e -dv 2 polySmoothFace1;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f sm;")), 96.0, "two levels");
    ok(&mut c, "polyPlane -n pl -sx 2 -sy 2;");
    assert_eq!(num(&ok(&mut c, "polyPlane -q -sx pl;")), 2.0);
    ok(&mut c, "polyPlane -e -sx 4 pl;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f pl;")), 8.0);
    ok(&mut c, "polyPipe -n pi; polyPlatonicSolid -n ps; polyPrimitive -n pr;");
    assert!(num(&ok(&mut c, "polyPipe -q -h pi;")) > 0.0);
    assert_eq!(num(&ok(&mut c, "polyPlatonicSolid -q -st ps;")), 0.0);
    ok(&mut c, "polyCube -n pk; polyPoke pk.f[0];");
    assert_eq!(num(&ok(&mut c, "polyPoke -q -nds polyPoke1;")), 0.0);
    ok(&mut c, "polySphere -n rd; polyReduce -p 50 rd;");
    close(num(&ok(&mut c, "polyReduce -q -p polyReduce1;")), 50.0, "reduce percentage");
    ok(&mut c, "polyCube -n n1; polyNormal -nm 0 n1.f[0];");
    assert_eq!(num(&ok(&mut c, "polyNormal -q -nm polyNormal1;")), 0.0);
    ok(&mut c, "polyCube -n mv; polyMergeVertex -d 0.1 mv.vtx[*];");
    close(num(&ok(&mut c, "polyMergeVertex -q -d polyMergeVert1;")), 0.1, "merge distance");
    ok(&mut c, "polyCube -n mf; polyMoveFacet -tz 1 mf.f[0];");
    ok(&mut c, "polyMoveFacet -e -tz 2 polyMoveFace1;");
    close(num(&ok(&mut c, "polyMoveFacet -q -tz polyMoveFace1;")), 2.0, "move face tz");
    ok(&mut c, "polyCube -n me; polyMoveEdge -ty 1 me.e[0];");
    close(num(&ok(&mut c, "polyMoveEdge -q -ty polyMoveEdge1;")), 1.0, "move edge ty");
    ok(&mut c, "polyCube -n mi; polyMirrorFace -d 1 mi;");
    assert_eq!(num(&ok(&mut c, "polyMirrorFace -q -d polyMirror1;")), 1.0);
    ok(&mut c, "polyCube -n w1; polyCube -n w2; polyUnite -n w3 w1 w2; polySeparate w3;");
    assert_eq!(num(&ok(&mut c, "polySeparate -q -nds polySeparate1;")), 0.0);
    ok(&mut c, "polyCube -n rm; polyRemesh rm;");
    assert!(num(&ok(&mut c, "polyRemesh -q -nds polyRemesh1;")) == 0.0);
    ok(&mut c, "polyCube -n tq; polyTriangulate tq; polyQuad tq;");
    assert!(num(&ok(&mut c, "polyQuad -q -a polyQuad1;")) > 0.0);
    // HasNoEffect: the edit passes the mesh through.
    ok(&mut c, "polyCube -n ns; polyExtrudeFacet -ltz 2 ns.f[1];");
    ok(&mut c, "polyExtrudeFacet -e -nds 1 polyExtrudeFace2;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f ns;")), 6.0, "no effect");
}

#[test]
fn primitives_sides_and_helix() {
    let mut c = Core::new();
    ok(&mut c, "polyPrism -n pz -nsi 5;");
    assert_eq!(num(&ok(&mut c, "polyPrism -q -ns polyPrism1;")), 5.0, "nsi sets the sides");
    assert_eq!(num(&ok(&mut c, "polyPrism -q -nsi pz;")), 5.0);
    ok(&mut c, "polyPrism -e -nsi 6 polyPrism1;");
    assert_eq!(num(&ok(&mut c, "getAttr polyPrism1.numberOfSides;")), 6.0);
    ok(&mut c, "polyPyramid -n py;");
    assert_eq!(num(&ok(&mut c, "polyPyramid -q -numderOfSides py;")), 4.0);
    ok(&mut c, "polyPyramid -e -ns 3 py;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f py;")), 4.0, "a triangular pyramid");
    ok(&mut c, "polyHelix -n h1;");
    assert_eq!(num(&ok(&mut c, "polyHelix -q -oib polyHelix1;")), 0.0);
    let centred = nums(&ok(&mut c, "exactWorldBoundingBox h1;"));
    ok(&mut c, "polyHelix -e -useOldInitBehaviour 1 polyHelix1;");
    let old = nums(&ok(&mut c, "exactWorldBoundingBox h1;"));
    close(old[1] - centred[1], 1.0, "moved up half the height");
    ok(&mut c, "polyHelix -n h2 -oib 1;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox h2;"))[1], old[1], "made in the old place");
}

#[test]
fn uv_nodes_made_again_from_their_settings() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p -sx 1 -sy 1;");
    let before = uv(&mut c, "p.map[0]");
    let node = strs(&ok(&mut c, "polyMoveUV -tu 0.5 p.map[0];"));
    assert_eq!(node, vec!["polyMoveUV1"]);
    close(num(&ok(&mut c, "polyMoveUV -q -tu polyMoveUV1;")), 0.5, "translateU");
    close_all(&nums(&ok(&mut c, "polyMoveUV -q -t polyMoveUV1;")), &[0.5, 0.0], "translate pair");
    close_all(&nums(&ok(&mut c, "polyMoveUV -q -pvt polyMoveUV1;")), &[0.5, 0.5], "the reference's pivot");
    close_all(&uv(&mut c, "p.map[0]"), &[before[0] + 0.5, before[1]], "moved");
    ok(&mut c, "polyMoveUV -e -t 1 0.25 polyMoveUV1;");
    close_all(&uv(&mut c, "p.map[0]"), &[before[0] + 1.0, before[1] + 0.25], "made again by the edit");
    // Scaling is about 0.5 0.5 unless a pivot is given.
    ok(&mut c, "polyPlane -n q -sx 1 -sy 1;");
    let q0 = uv(&mut c, "q.map[0]");
    ok(&mut c, "polyMoveUV -su 2 q.map[0];");
    close(uv(&mut c, "q.map[0]")[0], 0.5 + (q0[0] - 0.5) * 2.0, "scaled about 0.5");
    // polyMoveFacetUV makes the same node.
    ok(&mut c, "polyPlane -n mf -sx 1 -sy 1; polyMoveFacetUV -tv 0.1 mf.f[0];");
    close(num(&ok(&mut c, "polyMoveFacetUV -q -tv polyMoveUV3;")), 0.1, "facet UV");

    // Flip: about the UVs' own middle by default, then the other way by an edit.
    ok(&mut c, "polyPlane -n fl -sx 1 -sy 1; polyFlipUV fl.f[0];");
    assert_eq!(num(&ok(&mut c, "polyFlipUV -q -ft polyFlipUV1;")), 0.0);
    assert_eq!(num(&ok(&mut c, "polyFlipUV -q -l polyFlipUV1;")), 1.0, "local by default");
    let flipped = uv(&mut c, "fl.map[0]");
    ok(&mut c, "polyFlipUV -e -ft 1 polyFlipUV1;");
    let vflip = uv(&mut c, "fl.map[0]");
    assert!((vflip[0] - flipped[0]).abs() > 0.5 || (vflip[1] - flipped[1]).abs() > 0.5, "{flipped:?} {vflip:?}");

    // Merge: a node even when nothing merges, which an edit can then make merge.
    ok(&mut c, "polyCube -n mc; polyMapCut mc.e[0];");
    assert_eq!(num(&ok(&mut c, "polyMapCut -q -mvr polyMapCut1;")), 0.0);
    let seams = num(&ok(&mut c, "polyEvaluate -uv mc;"));
    assert_eq!(strs(&ok(&mut c, "polyMergeUV -d 0 mc.map[*];")), vec!["polyMergeUV1"]);
    assert_eq!(num(&ok(&mut c, "polyEvaluate -uv mc;")), seams);
    ok(&mut c, "polyMergeUV -e -d 2 polyMergeUV1;");
    assert!(num(&ok(&mut c, "polyEvaluate -uv mc;")) < seams, "merged by the edit");
    close(num(&ok(&mut c, "polyMergeUV -q -d polyMergeUV1;")), 2.0, "distance");

    // Normalize, layout, sew and optimize keep their settings.
    ok(&mut c, "polyCube -n nz; polyNormalizeUV -nt 0 -pa 0 nz.f[*];");
    assert_eq!(num(&ok(&mut c, "polyNormalizeUV -q -nt polyNormalizeUV1;")), 0.0);
    assert_eq!(num(&ok(&mut c, "polyNormalizeUV -q -pa polyNormalizeUV1;")), 0.0);
    ok(&mut c, "polyNormalizeUV -e -nt 1 -pa 1 polyNormalizeUV1;");
    let all = nums(&ok(&mut c, "polyEditUV -q nz.map[*];"));
    let (us, vs): (Vec<f64>, Vec<f64>) = (all.iter().step_by(2).copied().collect(), all.iter().skip(1).step_by(2).copied().collect());
    let fold = |v: &[f64], f: fn(f64, f64) -> f64, i: f64| v.iter().copied().fold(i, f);
    close(fold(&us, f64::min, 9.0).min(fold(&vs, f64::min, 9.0)), 0.0, "fitted from 0");
    close(fold(&us, f64::max, -9.0).max(fold(&vs, f64::max, -9.0)), 1.0, "to 1");
    ok(&mut c, "polyCube -n lo; polyLayoutUV -ps 1 lo.f[*];");
    close(num(&ok(&mut c, "polyLayoutUV -q -ps polyLayoutUV1;")), 1.0, "layout spacing");
    ok(&mut c, "polyLayoutUV -e -sc 0 polyLayoutUV1;");
    assert_eq!(num(&ok(&mut c, "polyLayoutUV -q -sc polyLayoutUV1;")), 0.0);
    ok(&mut c, "polyCube -n sw; polyMapCut sw.e[1]; polyMapSewMove -lps 1 -nf 3 sw.e[1];");
    assert_eq!(num(&ok(&mut c, "polyMapSewMove -q -lps polyMapSewMove1;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyMapSewMove -q -nf polyMapSewMove1;")), 3.0);
    ok(&mut c, "polyMapSew sw.e[1];");
    assert_eq!(num(&ok(&mut c, "polyMapSew -q -nds polyMapSew1;")), 0.0);
    ok(&mut c, "polyCube -n op; polyOptUvs -i 50 op.f[*];");
    assert_eq!(num(&ok(&mut c, "polyOptUvs -q -i polyOptUvs1;")), 50.0);
    ok(&mut c, "polyCube -n md; polyMapDel md.f[0];");
    assert_eq!(num(&ok(&mut c, "polyMapDel -q -nds polyMapDel1;")), 0.0);
    // A UV node with no effect gives back its input's UVs.
    ok(&mut c, "polyMoveUV -e -nds 1 polyMoveUV1;");
    close_all(&uv(&mut c, "p.map[0]"), &before, "passed through");
}

#[test]
fn planar_projection_settings() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n pp -sx 1 -sy 1;");
    ok(&mut c, "polyPlanarProjection -md y -n prj pp.f[0];");
    let pc = nums(&ok(&mut c, "polyPlanarProjection -q -pc prj;"));
    assert_eq!(pc.len(), 3);
    let full = nums(&ok(&mut c, "polyEditUV -q pp.map[*];"));
    ok(&mut c, "polyPlanarProjection -e -is 0.5 0.5 prj;");
    close_all(&nums(&ok(&mut c, "polyPlanarProjection -q -is prj;")), &[0.5, 0.5], "image scale");
    let half = nums(&ok(&mut c, "polyEditUV -q pp.map[*];"));
    let span = |v: &[f64]| v.iter().step_by(2).copied().fold(f64::MIN, f64::max) - v.iter().step_by(2).copied().fold(f64::MAX, f64::min);
    close(span(&half), span(&full) * 0.5, "half as wide in U");
    ok(&mut c, "polyPlanarProjection -e -ro 0 0 45 prj;");
    close(nums(&ok(&mut c, "polyPlanarProjection -q -ro prj;"))[2], 45.0, "rotate z");
    ok(&mut c, "polyPlanarProjection -e -ps 4 2 prj;");
    close_all(&nums(&ok(&mut c, "polyPlanarProjection -q -ps prj;")), &[4.0, 2.0], "projection scale");
    assert_eq!(num(&ok(&mut c, "polyPlanarProjection -q -nds prj;")), 0.0);
}

#[test]
fn hole_and_pin_history() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n h;");
    assert_eq!(num(&ok(&mut c, "polyHole -q -ch h.f[0];")), 1.0, "a cube has history");
    assert_eq!(strs(&ok(&mut c, "polyHole -ah 1 h.f[0];")), vec!["polyHole1"]);
    ok(&mut c, "polyCube -n nh; delete -ch nh;");
    assert_eq!(num(&ok(&mut c, "polyHole -q -ch nh.f[0];")), 0.0);
    ok(&mut c, "polyHole -ah 1 nh.f[0];");
    assert_eq!(strs(&ok(&mut c, "ls -type polyHole;")).len(), 1, "no node without history");
    assert_eq!(num(&ok(&mut c, "polyHole -q -ah nh.f[0];")), 1.0, "still a hole");
    ok(&mut c, "polyHole -ch 1 -ah 1 nh.f[1];");
    assert_eq!(strs(&ok(&mut c, "ls -type polyHole;")).len(), 2, "forced history");
    ok(&mut c, "polyHole -e -ah 0 nh.f[0];");
    assert_eq!(num(&ok(&mut c, "polyHole -q -ah nh.f[0];")), 0.0, "cleared by an edit");

    ok(&mut c, "polyPlane -n pn -sx 1 -sy 1;");
    ok(&mut c, "polyPinUV -v 0.5 pn.map[0];");
    close_all(&nums(&ok(&mut c, "polyPinUV -q -v pn.map[0:1];")), &[0.5, 0.0], "pin values");
    assert_eq!(strs(&ok(&mut c, "polyPinUV -q -unp pn.map[0:1];")), vec!["pn.map[1]"]);
    ok(&mut c, "polyPinUV -v 0.25 -v 1 pn.map[2:3];");
    close_all(&nums(&ok(&mut c, "polyPinUV -q -v pn.map[2:3];")), &[0.25, 1.0], "one value each");
    ok(&mut c, "polyPinUV -e -op 2 pn;");
    assert_eq!(strs(&ok(&mut c, "polyPinUV -q -unp pn.map[0:3];")).len(), 4, "all unpinned");
}

#[test]
fn select_query_output_and_blind_data() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; select -cl;");
    let ring = nums(&ok(&mut c, "polySelect -q -edgeRing 1 c;"));
    assert!(ring.len() >= 4, "{ring:?}");
    assert!(strs(&ok(&mut c, "ls -sl;")).is_empty(), "the selection stays");
    assert!(!nums(&ok(&mut c, "polySelect -q -shortestEdgePath 0 6 c;")).is_empty());

    let summary = strs(&ok(&mut c, "polyOutput c;"));
    assert_eq!(summary.len(), 1, "a summary by default: {summary:?}");
    assert!(summary[0].contains("8 vertices"));
    let v = strs(&ok(&mut c, "polyOutput -v c;"));
    assert_eq!(v.iter().filter(|l| l.starts_with("vtx ")).count(), 8);
    let e = strs(&ok(&mut c, "polyOutput -e c;"));
    assert_eq!(e.iter().filter(|l| l.starts_with("edge ")).count(), 12);
    let ef = strs(&ok(&mut c, "polyOutput -ef c;"));
    assert!(ef.iter().filter(|l| l.starts_with("edgeFace ")).all(|l| l.split(':').nth(1).unwrap().split_whitespace().count() == 2));
    assert_eq!(strs(&ok(&mut c, "polyOutput -ve c;")).iter().filter(|l| l.starts_with("vertEdge ")).count(), 8);
    assert_eq!(strs(&ok(&mut c, "polyOutput -uvd c;")).iter().filter(|l| l.starts_with("uvDesc ")).count(), 6);
    assert_eq!(strs(&ok(&mut c, "polyOutput -nd c;")).iter().filter(|l| l.starts_with("normDesc ")).count(), 6);
    assert!(strs(&ok(&mut c, "polyOutput -no -a c;")).is_empty());
    let plug = strs(&ok(&mut c, "polyOutput polyCube1.outMesh;"));
    assert!(plug[0].contains("6 faces"));
    ok(&mut c, "polyColorPerVertex -rgb 1 0 0 c.vtx[0];");
    assert!(strs(&ok(&mut c, "polyOutput -cd c;")).iter().any(|l| l.starts_with("colorDesc ")));

    ok(&mut c, "polyBlindData -id 9001 -at \"face\" -ldn \"heat\" -dbd 2.0 c.f[1];");
    ok(&mut c, "polyBlindData -id 9001 -at \"face\" -ldn \"heat\" -dbd 5.0 c.f[2];");
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" -dbd 5 c.f[0:5];")), vec![5.0]);
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" -min 1 -max 3 c.f[0:5];")), vec![2.0]);
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" -sd \"x\" c.f[0:5];")), Vec::<f64>::new());
}

#[test]
fn sew_slide_and_display() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -sx 1 -sy 1 -n plg1; polyPlane -sx 1 -sy 1 -n plg2; move -ws 1.05 0 0 plg2;");
    ok(&mut c, "polyUnite -n u plg1 plg2;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -v u;")), 8.0);
    ok(&mut c, "polySewEdge -t 0.1 -tx 1 u.e[0:7];");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -v u;")), 6.0, "sewn within the tolerance");
    close(num(&ok(&mut c, "polySewEdge -q -t polySewEdge1;")), 0.1, "tolerance");
    assert_eq!(num(&ok(&mut c, "polySewEdge -q -tx polySewEdge1;")), 1.0, "texture");

    ok(&mut c, "polyPlane -n s -sx 2 -sy 1 -w 2 -h 1;");
    let x0 = nums(&ok(&mut c, "pointPosition -w s.vtx[1];"))[0];
    ok(&mut c, "select -r s.e[1]; polySlideEdge -ed 0.5 -d 1;");
    let x1 = nums(&ok(&mut c, "pointPosition -w s.vtx[1];"))[0];
    close((x1 - x0).abs(), 0.5, "half of a side edge");
    ok(&mut c, "polyOptions -dcl 1;");
    assert_eq!(num(&ok(&mut c, "polyOptions -q -dcl;")), 1.0);
}

#[test]
fn smart_extrude() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n plg -w 10 -h 10;");
    let r = strs(&ok(&mut c, "polySmartExtrude -ltz 2 plg.f[44:45];"));
    assert_eq!(r, vec!["polySmartExtrude1"]);
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f plg;")), 100.0 + 6.0, "two faces as one region: six sides");
    close(num(&ok(&mut c, "polySmartExtrude -q -ltz polySmartExtrude1;")), 2.0, "ltz");
    close_all(&nums(&ok(&mut c, "polySmartExtrude -q -ls polySmartExtrude1;")), &[1.0, 1.0, 1.0], "local scale");
    ok(&mut c, "polySmartExtrude -e -ltz 4 polySmartExtrude1;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox plg;"))[4], 4.0, "raised by the edit");
    assert_eq!(num(&ok(&mut c, "polySmartExtrude -q -lc polySmartExtrude1;")), 0.0);
}

#[test]
fn constraint_monitor() {
    let mut c = Core::new();
    ok(&mut c, "global int $refreshed = 0; global string $win = \"\"; global proc refreshMonitor(string $w) { global int $refreshed; global string $win; $refreshed++; $win = $w; }");
    ok(&mut c, "polySelectConstraintMonitor -create;");
    ok(&mut c, "polySelectConstraintMonitor -changeCommand \"refreshMonitor\" \"panel1\";");
    ok(&mut c, "polySelectConstraint -m 3 -t 8;");
    c.idle();
    assert_eq!(num(&ok(&mut c, "global int $refreshed; $refreshed;")), 1.0);
    assert_eq!(ok(&mut c, "global string $win; $win;"), Value::String("panel1".into()));
    ok(&mut c, "polySelectConstraintMonitor -delete; polySelectConstraint -m 0;");
    c.idle();
    assert_eq!(num(&ok(&mut c, "global int $refreshed; $refreshed;")), 1.0, "no callback once deleted");
}

#[test]
fn geo_sampler() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n g; shadingNode -asShader lambert -n red; setAttr red.color -type double3 1 0 0;");
    ok(&mut c, "sets -renderable true -noSurfaceShader true -empty -name redSG; connectAttr red.outColor redSG.surfaceShader; sets -e -forceElement redSG g;");
    ok(&mut c, "directionalLight -n sun; rotate -90 0 0 sun;");
    ok(&mut c, "polyGeoSampler -cdo g;");
    // The top faces the light: lit red. The bottom faces away, but the mesh is double-sided.
    let top = nums(&ok(&mut c, "polyColorPerVertex -q -rgb g.vtx[2];"));
    assert!(top[0] > 0.5 && top[1] < 1e-3 && top[2] < 1e-3, "{top:?}");
    ok(&mut c, "polyGeoSampler -e -sf 0.5 polyGeoSampler1;");
    let half = nums(&ok(&mut c, "polyColorPerVertex -q -rgb g.vtx[2];"));
    close(half[0], top[0] * 0.5, "scaled by the edit");
    ok(&mut c, "polyCube -n w; polyGeoSampler -lo -bf w;");
    let lit = nums(&ok(&mut c, "polyColorPerVertex -q -rgb w.vtxFace[2][1];"));
    assert!(lit[1] > 0.1, "light only, not the grey material: {lit:?}");
    ok(&mut c, "polyCube -n d; polyGeoSampler -dg -sf 0.1 -lo d;");
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox d;"));
    assert!(bb[4] > 0.5, "pushed out along the normals: {bb:?}");
    ok(&mut c, "polyCube -n m; polyColorPerVertex -rgb 0.5 0.5 0.5 m; polyGeoSampler -lo -cbl \"multiply\" -abl \"none\" m;");
    let mul = nums(&ok(&mut c, "polyColorPerVertex -q -rgb m.vtx[2];"));
    assert!(mul[0] <= 0.5 + 1e-6, "multiplied: {mul:?}");
}

#[test]
fn multi_layout() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n b;");
    assert_eq!(ok(&mut c, "polyMultiLayoutUV -sc 1 -rbf 2 -l 2 a b;"), Value::Null);
    let bounds = |c: &mut Core, m: &str| {
        let v = nums(&ok(c, &format!("polyEditUV -q {m}.map[*];")));
        let us: Vec<f64> = v.iter().step_by(2).copied().collect();
        let vs: Vec<f64> = v.iter().skip(1).step_by(2).copied().collect();
        [us.iter().copied().fold(f64::MAX, f64::min), vs.iter().copied().fold(f64::MAX, f64::min), us.iter().copied().fold(f64::MIN, f64::max), vs.iter().copied().fold(f64::MIN, f64::max)]
    };
    let (ba, bb) = (bounds(&mut c, "a"), bounds(&mut c, "b"));
    for x in ba.iter().chain(bb.iter()) {
        assert!(*x > -1e-6 && *x < 1.0 + 1e-6, "in 0 to 1: {ba:?} {bb:?}");
    }
    let apart = ba[2] <= bb[0] + 1e-6 || bb[2] <= ba[0] + 1e-6 || ba[3] <= bb[1] + 1e-6 || bb[3] <= ba[1] + 1e-6;
    assert!(apart, "the meshes' UVs don't overlap: {ba:?} {bb:?}");
    ok(&mut c, "polyMultiLayoutUV -sc 1 -l 1 -su 0.5 -ou 2 a b;");
    let (ba, bb) = (bounds(&mut c, "a"), bounds(&mut c, "b"));
    assert!(ba[0] >= 2.0 - 1e-6 && bb[2] <= 2.5 + 1e-6, "sized and moved: {ba:?} {bb:?}");
}

#[test]
fn project_curve_node() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n pl -w 10 -h 10 -sx 10 -sy 10 -ax 0 0 1;");
    ok(&mut c, "circle -n cr -nr 0 0 1 -r 2;");
    let r = strs(&ok(&mut c, "polyProjectCurve -direction 0 0 1 pl cr;"));
    assert_eq!(r.len(), 2, "{r:?}");
    assert!(r[1].starts_with("polyProjectCurve"));
    close_all(&nums(&ok(&mut c, &format!("polyProjectCurve -q -d {};", r[1]))), &[0.0, 0.0, 1.0], "direction");
    close(num(&ok(&mut c, &format!("polyProjectCurve -q -tol {};", r[1]))), 0.01, "tolerance");
    // Moving the plane moves the curve on it: the node is live.
    ok(&mut c, "move -r 0 0 3 pl;");
    let cv = nums(&ok(&mut c, &format!("pointPosition -w {}.cv[0];", r[0])));
    close(cv[2], 3.0, "followed the plane");
    ok(&mut c, &format!("polyProjectCurve -e -tol 0.5 {};", r[1]));
    close(num(&ok(&mut c, &format!("getAttr {}.tolerance;", r[1]))), 0.5, "edited tolerance");
}

/// Every flag of the reference's these commands lacked is now there, by its long or short name.
#[test]
fn every_flag_registered() {
    let c = Core::new();
    let rows: &[(&str, &str)] = &[
        ("polyExtrudeFacet", "e edit q query"),
        ("polyExtrudeVertex", "e edit q query"),
        ("polyFlipUV", "e edit q query"),
        ("polyGeoSampler", "abl ac alphaBlend amn amx averageColor bf cbl cdo clampAlphaMax clampAlphaMin clampRGBMax clampRGBMin colorBlend colorDisplayOption dg displaceGeometry e edit flatShading fs ids ignoreDoubleSided lightingOnly lo reuseShadows rs shareUV su ul useLightShadows"),
        ("polyHelix", "e edit oib q query useOldInitBehaviour"),
        ("polyHole", "createHistory"),
        ("polyLayoutUV", "e edit q query"),
        ("polyMapCut", "caching cch ch constructionHistory e edit moveratio mvr n name nds nodeState q query"),
        ("polyMapDel", "e edit q query"),
        ("polyMapSew", "caching cch ch constructionHistory e edit n name nds nodeState q query"),
        ("polyMapSewMove", "caching cch ch constructionHistory e edit limitPieceSize n name nds nodeState numberFaces q query uvSetName uvs worldSpace ws"),
        ("polyMergeEdge", "e edit q query"),
        ("polyMergeFacet", "e edit q query"),
        ("polyMergeUV", "e edit q query"),
        ("polyMergeVertex", "e edit q query"),
        ("polyMirrorFace", "e edit q query"),
        ("polyMoveEdge", "e edit q query"),
        ("polyMoveFacet", "e edit q query"),
        ("polyMoveFacetUV", "e edit q query"),
        ("polyMoveUV", "e edit q query"),
        ("polyMoveVertex", "e edit q query"),
        ("polyMultiLayoutUV", "flipReversed fr gridU gridV gu gv l layout layoutMethod lm offsetU offsetV ou ov percentageSpace prescale psc rbf rotateForBestFit sc scale sizeU sizeV su sv uvSetName uvs"),
        ("polyNormal", "e edit q query"),
        ("polyNormalizeUV", "caching cch centerOnTile ch cm constructionHistory cot createNewMap e edit ibd insertBeforeDeformers n name nds nodeState q query uvSetName uvs worldSpace ws"),
        ("polyOptUvs", "e edit q query uvSetName uvs worldSpace ws"),
        ("polyOptions", "dcl displayColorAsGreyScale"),
        ("polyOutput", "cd colorDesc e edgeFace ef fo force nd no noOutput normDesc uvDesc uvd ve vertEdge"),
        ("polyPinUV", "createHistory e edit q query"),
        ("polyPipe", "e edit q query"),
        ("polyPlanarProjection", "caching cch e edit ic imageCenter imageScale n name nds nodeState pc perInstance ph phs pi projectionCenter projectionHeight projectionHorizontalSweep projectionScale ps q query ro rotate worldSpace ws"),
        ("polyPlane", "e edit q query"),
        ("polyPlatonicSolid", "e edit q query"),
        ("polyPoke", "e edit q query"),
        ("polyPrimitive", "e edit q query"),
        ("polyPrism", "e edit nsi numderOfSides q query"),
        ("polyProjectCurve", "e edit q query"),
        ("polyPyramid", "e edit nsi numderOfSides q query"),
        ("polyQuad", "e edit q query"),
        ("polyQueryBlindData", "bd binaryData bnd booleanData dbd doubleData ind intData max maxValue min minValue sd stringData"),
        ("polyReduce", "e edit q query"),
        ("polyRemesh", "e edit q query"),
        ("polySelect", "q query"),
        ("polySelectConstraintMonitor", "c cc changeCommand create d delete"),
        ("polySeparate", "e edit q query"),
        ("polySewEdge", "e edit q query t"),
        ("polySlideEdge", "ed edgeDirection"),
        ("polySmartExtrude", "caching cch ch constructionHistory e edit lc ld ldx ldy ldz localCenter localDirection localDirectionX localDirectionY localDirectionZ localRotate localRotateX localRotateY localRotateZ localScale localScaleX localScaleY localScaleZ localTranslate localTranslateX localTranslateY localTranslateZ lr lrx lry lrz ls lsx lsy lsz lt ltx lty ltz n name nds nodeState pivot pivotX pivotY pivotZ pvt pvx pvy pvz q query ro rotate rotateX rotateY rotateZ rx ry rz s scale scaleX scaleY scaleZ sx sy sz t translate translateX translateY translateZ tx ty tz worldSpace ws"),
        ("polySmooth", "e edit q query"),
    ];
    let mut missing = Vec::new();
    for (cmd, flags) in rows {
        let Some(def) = c.s.commands.find(cmd) else {
            missing.push(format!("{cmd}: no command"));
            continue;
        };
        for fl in flags.split_whitespace() {
            if !def.flags.iter().any(|d| d.name == fl || d.short_name == fl) {
                missing.push(format!("{cmd} -{fl}"));
            }
        }
    }
    assert!(missing.is_empty(), "{missing:?}");
}
