//! Commands of the reference from dockControl to imagePlane: doubleProfileBirailSurface, editDisplayLayerGlobals,
//! excludeObjectDisplayPreset, expressionEditorListen, freeze and freezeOptions, and the flags the others gained (query and
//! edit on the NURBS history commands, extendCurve's methods, extrude's pivots, globalStitch's node, ikSystem's solver
//! order, imagePlane's movie questions, dock and layout settings and the rest).

use aztek_host::Core;
use serde_json::{json, Value};

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

fn bbox(c: &mut Core, obj: &str) -> Vec<f64> {
    nums(&ok(c, &format!("exactWorldBoundingBox {obj};")))
}

#[test]
fn extend_curve_methods_and_query() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -n ln;");
    // The start by 3, linear: the curve now begins at -3, the original kept whole.
    let r = strs(&ok(&mut c, "extendCurve -em 0 -et 0 -s 1 -d 3 ln;"));
    assert!(r[1].starts_with("extendCurve"), "{r:?}");
    close(bbox(&mut c, "ln")[0], -3.0, "start extended");
    close(bbox(&mut c, "ln")[3], 4.0, "end kept");
    assert_eq!(ok(&mut c, "extendCurve -q -s ln;"), json!(1));
    close(num(&ok(&mut c, "extendCurve -q -d ln;")), 3.0, "distance");
    // Edit: both ends.
    ok(&mut c, "extendCurve -e -s 2 ln;");
    let b = bbox(&mut c, "ln");
    close(b[0], -3.0, "both: start");
    close(b[3], 7.0, "both: end");
    // To a point: the end reaches it.
    ok(&mut c, "extendCurve -e -s 0 -em 2 -px 6 -py 2 -pz 0 ln;");
    let end = nums(&ok(&mut c, "pointOnCurve -pr 1000 ln;"));
    close(end[0], 6.0, "reaches x");
    close(end[1], 2.0, "reaches y");
    close(num(&ok(&mut c, "extendCurve -q -py ln;")), 2.0, "pointY");
    // Not joined: only the extension.
    ok(&mut c, "extendCurve -e -em 0 -d 2 -jn 0 ln;");
    let b = bbox(&mut c, "ln");
    close(b[0], 4.0, "extension alone starts at the end");
    close(b[3], 6.0, "and runs 2");
    // noChanges passes the curve through; nodeState 2 keeps the last result.
    ok(&mut c, "extendCurve -e -nc 1 ln;");
    close(bbox(&mut c, "ln")[3], 4.0, "no changes");
    ok(&mut c, "extendCurve -e -nds 2 ln; extendCurve -e -nc 0 ln;");
    close(bbox(&mut c, "ln")[3], 4.0, "blocking keeps what it gave");
    assert_eq!(ok(&mut c, "extendCurve -q -nds ln;"), json!(2));
    // Circular: an arc carries on a circle's curvature, staying at the circle's radius.
    ok(&mut c, "circle -nr 0 0 1 -r 2 -sw 90 -n arc;");
    ok(&mut c, "extendCurve -et 1 -s 0 -d 1 arc;");
    let e = nums(&ok(&mut c, "pointOnCurve -pr 1000 arc;"));
    // Within the curve's own departure from a true circle (it is a cubic, not rational).
    assert!(((e[0] * e[0] + e[1] * e[1]).sqrt() - 2.0).abs() < 0.06, "circular extension keeps the radius: {e:?}");
    // Extrapolated from a cubic: it goes on bending, unlike a straight one.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 1 0 -p 3 0 0 -n cu; extendCurve -et 2 -s 0 -d 1 -rmk 1 cu;");
    let e = nums(&ok(&mut c, "pointOnCurve -pr 1000 cu;"));
    assert!(e[1] < -0.5, "the extrapolated end keeps falling: {e:?}");
}

#[test]
fn nurbs_history_query_edit() {
    let mut c = Core::new();
    // fitBspline: within its tolerance; a looser one needs fewer CVs.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 1 0.5 0 -p 2 0.8 0 -p 3 1 0 -p 4 0.8 0 -p 5 0.5 0 -p 6 0 0 -n pts;");
    let r = strs(&ok(&mut c, "fitBspline -tol 0.001 -n fit pts;"));
    assert!(r[1].starts_with("fitBspline"), "{r:?}");
    let tight = num(&ok(&mut c, "getAttr fit.spans;"));
    ok(&mut c, "fitBspline -e -tol 0.5 fit;");
    close(num(&ok(&mut c, "fitBspline -q -tol fit;")), 0.5, "tolerance edited");
    assert!(num(&ok(&mut c, "getAttr fit.spans;")) < tight, "a looser fit has fewer spans");
    ok(&mut c, "fitBspline -e -cch 1 fit;");
    assert_eq!(ok(&mut c, "fitBspline -q -cch fit;"), json!(true));

    // hardenPointCurve: -q and -e on its node.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 1 0 -p 3 0 0 -p 4 1 0 -n hc;");
    ok(&mut c, "hardenPointCurve -m 2 hc.cv[2];");
    assert_eq!(ok(&mut c, "hardenPointCurve -q -m hc;"), json!(2));
    ok(&mut c, "hardenPointCurve -e -nds 1 hc;");
    assert_eq!(ok(&mut c, "hardenPointCurve -q -nds hc;"), json!(1));

    // duplicateCurve: a range of the isoparm, absolute or relative.
    ok(&mut c, "nurbsPlane -ax 0 1 0 -w 10 -lr 1 -n pl;");
    let d = strs(&ok(&mut c, "duplicateCurve -min 0.25 -max 0.75 -r 1 -n iso pl.v[0.5];"));
    assert!(d[1].starts_with("curveFromSurfaceIso"), "{d:?}");
    let b = bbox(&mut c, "iso");
    close(b[3] - b[0], 5.0, "half the isoparm");
    assert_eq!(ok(&mut c, "duplicateCurve -q -r iso;"), json!(true));
    ok(&mut c, "duplicateCurve -e -max 1 iso;");
    close(num(&ok(&mut c, "duplicateCurve -q -max iso;")), 1.0, "max edited");
    let b = bbox(&mut c, "iso");
    close(b[3] - b[0], 7.5, "three quarters");

    // duplicateSurface: the patch flags, and edits on its node.
    ok(&mut c, "nurbsPlane -ax 0 1 0 -w 4 -lr 1 -u 4 -v 4 -d 1 -n grid;");
    let s = strs(&ok(&mut c, "duplicateSurface -ffu 1 -fcu 2 -ffv 0 -fcv 1 grid;"));
    let b = bbox(&mut c, &s[0]);
    close(b[3] - b[0], 2.0, "two patches across");
    assert_eq!(ok(&mut c, &format!("duplicateSurface -q -fcu {};", s[1])), json!(2));
    ok(&mut c, &format!("duplicateSurface -e -fcu 3 {};", s[1]));
    let b = bbox(&mut c, &s[0]);
    close(b[3] - b[0], 3.0, "three patches after the edit");

    // extendSurface: -q, -e and join off.
    ok(&mut c, "nurbsPlane -ax 0 1 0 -w 2 -lr 1 -n es;");
    ok(&mut c, "extendSurface -d 1 -es 1 es;");
    close(num(&ok(&mut c, "extendSurface -q -d es;")), 1.0, "distance");
    ok(&mut c, "extendSurface -e -d 2 es;");
    let b = bbox(&mut c, "es");
    close(b[3] - b[0], 4.0, "extended by 2 along U");
    ok(&mut c, "extendSurface -e -jn 0 es;");
    let b = bbox(&mut c, "es");
    close(b[3] - b[0], 2.0, "the extension alone");
    assert_eq!(ok(&mut c, "extendSurface -q -jn es;"), json!(false));
}

#[test]
fn extrude_settings() {
    let mut c = Core::new();
    ok(&mut c, "circle -nr 0 1 0 -r 1 -n prof;");
    let pb = bbox(&mut c, "prof");
    let width = pb[5] - pb[2];
    // Distance with the direction's components and a degree along the length.
    let r = strs(&ok(&mut c, "extrude -et 0 -dx 0 -dy 0 -dz 1 -l 5 -dl 3 prof;"));
    assert!(r[1].starts_with("extrude"), "{r:?}");
    let b = bbox(&mut c, &r[0]);
    close(b[5] - b[2], width + 5.0, "along Z by 5, the circle's width more");
    assert_eq!(ok(&mut c, &format!("extrude -q -dl {};", r[1])), json!(3));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.degreeV;", r[0]))), 3.0);
    ok(&mut c, &format!("extrude -e -l 2 {};", r[1]));
    let b = bbox(&mut c, &r[0]);
    close(b[5] - b[2], width + 2.0, "length edited");
    close(num(&ok(&mut c, &format!("extrude -q -dz {};", r[1]))), 1.0, "directionZ");
    // Tube along a path with a scale: the pivot decides where the profile shrinks to.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 0 4 0 -n path;");
    ok(&mut c, "circle -nr 0 1 0 -r 1 -c 2 0 0 -n offc;");
    let t = strs(&ok(&mut c, "extrude -et 2 -sc 0 -ucp 1 -p 2 0 0 offc path;"));
    let b = bbox(&mut c, &t[0]);
    close((b[0] + b[3]) * 0.5, 2.0, "about its own middle");
    ok(&mut c, &format!("extrude -e -ucp 0 {};", t[1]));
    let b = bbox(&mut c, &t[0]);
    assert!(b[0] < 1.0 + 1e-6, "about the path's end, it narrows toward the path: {b:?}");
    assert_eq!(ok(&mut c, &format!("extrude -q -ucp {};", t[1])), json!(0));
    // Only the node.
    let n = strs(&ok(&mut c, "extrude -o 0 -et 0 prof;"));
    assert_eq!(n.len(), 1);
    assert_eq!(ok(&mut c, &format!("nodeType {};", n[0])), json!("extrude"));
}

#[test]
fn fillets_and_birail() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 5 0 0 -n a; curve -d 1 -p 0 0 0 -p 0 5 0 -n b;");
    let f = strs(&ok(&mut c, "filletCurve -r 1 a b;"));
    assert_eq!(ok(&mut c, &format!("filletCurve -q -r {};", f[1])), json!(1.0));
    ok(&mut c, &format!("filletCurve -e -r 2 {};", f[1]));
    let p = nums(&ok(&mut c, &format!("pointOnCurve -pr 0 {};", f[0])));
    close(p[0].max(p[1]), 2.0, "radius 2 meets at 2");
    // Free form tangent against blend: a blend has more CVs.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 1 0 0 -n c1; curve -d 1 -p 3 1 0 -p 4 1 0 -n c2;");
    let t = strs(&ok(&mut c, "filletCurve -cir 0 -n tf c1 c2;"));
    let bl = strs(&ok(&mut c, "filletCurve -cir 0 -fb 1 -n bf c1 c2;"));
    assert!(num(&ok(&mut c, "getAttr bf.spans;")) > num(&ok(&mut c, "getAttr tf.spans;")), "{t:?} {bl:?}");
    assert_eq!(ok(&mut c, &format!("filletCurve -q -fb {};", bl[1])), json!(true));
    // In place: the first curve becomes the joined one, the second goes.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 5 0 0 -n ra; curve -d 1 -p 0 0 0 -p 0 5 0 -n rb;");
    ok(&mut c, "filletCurve -r 1 -rpo 1 ra rb;");
    assert_eq!(ok(&mut c, "objExists rb;"), json!(false));
    let b = bbox(&mut c, "ra");
    close(b[3], 5.0, "joined runs to the first's end");
    close(b[4], 5.0, "and the second's");

    // freeFormFillet's bias flag is -b, and its node answers -q and -e.
    ok(&mut c, "nurbsPlane -n p1 -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; nurbsPlane -n p2 -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; setAttr p2.tx 6;");
    let u1 = num(&ok(&mut c, "getAttr p1.maxValueU;"));
    let ff = strs(&ok(&mut c, &format!("freeFormFillet -b 0.5 -d 0.4 p1.u[{u1}] p2.u[0];")));
    close(num(&ok(&mut c, &format!("freeFormFillet -q -d {};", ff[1]))), 0.4, "depth");
    ok(&mut c, &format!("freeFormFillet -e -pt 0.01 {};", ff[1]));
    close(num(&ok(&mut c, &format!("freeFormFillet -q -pt {};", ff[1]))), 0.01, "position tolerance");

    // doubleProfileBirailSurface: the first profile turns into the second along the rails.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 0 0 10 -n r1; curve -d 1 -p 4 0 0 -p 4 0 10 -n r2;");
    ok(&mut c, "curve -d 2 -p 0 0 0 -p 2 2 0 -p 4 0 0 -n pr1; curve -d 2 -p 0 0 10 -p 2 -2 10 -p 4 0 10 -n pr2;");
    let s = strs(&ok(&mut c, "doubleProfileBirailSurface -bl 0.5 pr1 pr2 r1 r2;"));
    assert_eq!(s.len(), 2);
    assert_eq!(ok(&mut c, &format!("nodeType {};", s[1])), json!("doubleProfileBirailSurface"));
    let start = nums(&ok(&mut c, &format!("pointOnSurface -u 0.5 -v 0 {};", s[0])));
    let end = nums(&ok(&mut c, &format!("pointOnSurface -u 0.5 -v 1 {};", s[0])));
    assert!(start[1] > 0.5, "the first profile at the start: {start:?}");
    assert!(end[1] < -0.5, "the second at the end: {end:?}");
    close(num(&ok(&mut c, &format!("doubleProfileBirailSurface -q -bl {};", s[1]))), 0.5, "blend factor");
    let mid = nums(&ok(&mut c, &format!("pointOnSurface -u 0.5 -v 0.5 {};", s[0])))[1];
    ok(&mut c, &format!("doubleProfileBirailSurface -e -bl 0.9 {};", s[1]));
    let mid2 = nums(&ok(&mut c, &format!("pointOnSurface -u 0.5 -v 0.5 {};", s[0])))[1];
    assert!(mid2 > mid + 0.1, "more of the first profile: {mid} -> {mid2}");
    ok(&mut c, &format!("doubleProfileBirailSurface -e -tm 1 -tp1 1 {};", s[1]));
    assert_eq!(ok(&mut c, &format!("doubleProfileBirailSurface -q -tm {};", s[1])), json!(1));
    assert_eq!(ok(&mut c, &format!("doubleProfileBirailSurface -q -tp1 {};", s[1])), json!(true));
}

#[test]
fn global_stitch_node() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n a -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; nurbsPlane -n b -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; setAttr b.tx 2.2;");
    let r = strs(&ok(&mut c, "globalStitch -ms 0.1 a b;"));
    assert_eq!(r.len(), 3, "{r:?}");
    let node = r[2].clone();
    assert_eq!(ok(&mut c, &format!("nodeType {node};")), json!("globalStitch"));
    close(bbox(&mut c, "a")[3], 1.0, "out of reach: unchanged");
    ok(&mut c, &format!("globalStitch -e -ms 0.5 {node};"));
    close(bbox(&mut c, "a")[3], 1.1, "within reach: they meet halfway");
    close(bbox(&mut c, "b")[0], 1.1, "both");
    close(num(&ok(&mut c, &format!("globalStitch -q -ms {node};"))), 0.5, "max separation");
    // Locking the first: the second comes all the way.
    ok(&mut c, &format!("globalStitch -e -lk true -lk false {node};"));
    close(bbox(&mut c, "a")[3], 1.0, "locked stays");
    close(bbox(&mut c, "b")[0], 1.0, "the other comes to it");
    assert_eq!(ok(&mut c, &format!("globalStitch -q -lk {node};")), json!([true, false]));
    ok(&mut c, &format!("globalStitch -e -se 0 -sc 0 {node};"));
    close(bbox(&mut c, "b")[0], 1.2, "edges off: nothing stitched");
    assert_eq!(ok(&mut c, &format!("globalStitch -q -se {node};")), json!(0));
}

#[test]
fn flow_and_geom_bind() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 5 0 0 -p 10 0 0 -p 15 0 0 -n fpath; polyCube -n box; pathAnimation -c fpath -fm 1 -f 1 box;");
    let r = strs(&ok(&mut c, "flow -dv 3 6 2 box;"));
    assert_eq!(r.len(), 4, "{r:?}");
    assert_eq!(ok(&mut c, &format!("nodeType {};", r[0])), json!("flow"));
    assert_eq!(nums(&ok(&mut c, "flow -q -dv box;")), vec![3.0, 6.0, 2.0]);
    assert_eq!(ok(&mut c, "flow -q -oc box;"), json!(true));
    assert_eq!(ok(&mut c, "flow -q -lc box;"), json!(false));
    // Round the path: local control by default, the lattice as long as the path.
    ok(&mut c, "polyCube -n box2; pathAnimation -c fpath -fm 1 -f 1 box2;");
    let r2 = strs(&ok(&mut c, "flow -oc 0 box2;"));
    assert_eq!(ok(&mut c, "flow -q -lc box2;"), json!(true));
    let b = bbox(&mut c, &r2[2]);
    close(b[3] - b[0], 15.0, "lattice round the path");

    // geomBind: settings held on a geomBind node, read and changed through the skinCluster.
    ok(&mut c, "polyCylinder -r 1 -h 6 -sx 8 -sy 6 -n cyl; select -cl; joint -p 0 -3 0 -n j1; joint -p 0 0 0 -n j2; joint -p 0 3 0 -n j3;");
    let sk = strs(&ok(&mut c, "skinCluster j1 cyl;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("geomBind -bm 3 -fo 0.3 -mi 2 {sk};")), Value::Null);
    close(num(&ok(&mut c, &format!("geomBind -q -fo {sk};"))), 0.3, "falloff");
    assert_eq!(ok(&mut c, &format!("geomBind -q -mi {sk};")), json!(2));
    assert_eq!(ok(&mut c, &format!("geomBind -q -gvp {sk};")), json!([256, true]));
    ok(&mut c, &format!("geomBind -e -mi 1 -gvp 128 0 {sk};"));
    assert_eq!(ok(&mut c, &format!("geomBind -q -mi {sk};")), json!(1));
    assert_eq!(ok(&mut c, &format!("geomBind -q -gvp {sk};")), json!([128, false]));
    // One influence each: every weight is 0 or 1.
    let w = nums(&ok(&mut c, &format!("skinPercent -q -v {sk} cyl.vtx[10];")));
    assert!(w.iter().all(|x| *x < 1e-6 || (*x - 1.0).abs() < 1e-6), "{w:?}");
    assert!(c.mel_checked(&format!("geomBind -e -gvp 100 1 {sk};")).is_err(), "not a power of two");
}

#[test]
fn freeze_and_options() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n fc; createNode multiplyDivide -n md; setAttr md.input1X 2; setAttr md.input2X 3; connectAttr md.outputX fc.tx;");
    close(num(&ok(&mut c, "getAttr fc.tx;")), 6.0, "computing");
    let r = strs(&ok(&mut c, "freeze md;"));
    assert_eq!(r, vec!["md".to_string()]);
    assert_eq!(ok(&mut c, "getAttr md.frozen;"), json!(true));
    ok(&mut c, "setAttr md.input2X 5;");
    close(num(&ok(&mut c, "getAttr fc.tx;")), 6.0, "frozen: keeps what it had");
    let q = strs(&ok(&mut c, "freeze -q md fc;"));
    assert_eq!(q.first().map(String::as_str), Some("md"));
    ok(&mut c, "freeze -uf md;");
    close(num(&ok(&mut c, "getAttr fc.tx;")), 10.0, "unfrozen: computes again");
    // Downstream: what only frozen nodes feed is frozen indirectly.
    ok(&mut c, "createNode plusMinusAverage -n pma; connectAttr md.outputY pma.input1D[0];");
    let r = ok(&mut c, "freeze -dn md;");
    let list = strs(&r);
    assert!(list.contains(&"pma".to_string()), "{list:?}");
    assert_eq!(ok(&mut c, "getAttr pma.frozenAffected;"), json!(true));
    // -n only reports.
    ok(&mut c, "freeze -uf -all;");
    let r = strs(&ok(&mut c, "freeze -n md;"));
    assert_eq!(r, vec!["md".to_string()]);
    assert_eq!(ok(&mut c, "getAttr md.frozen;"), json!(false));
    // -invisible: under a hidden group.
    ok(&mut c, "polySphere -n s1; group -n hid s1; setAttr hid.visibility 0;");
    let r = strs(&ok(&mut c, "freeze -i hid;"));
    assert!(r.iter().any(|x| x.ends_with("s1")), "{r:?}");

    assert_eq!(ok(&mut c, "freezeOptions -dn \"force\";"), json!(true));
    assert_eq!(ok(&mut c, "freezeOptions -q -dn;"), json!("force"));
    ok(&mut c, "freezeOptions -inv 1 -rt 0;");
    assert_eq!(ok(&mut c, "freezeOptions -q -inv;"), json!(true));
    assert_eq!(ok(&mut c, "freezeOptions -q -rt;"), json!(false));
    assert!(c.mel_checked("freezeOptions -up \"sometimes\";").is_err());
}

#[test]
fn evaluation_manager_graph() {
    let mut c = Core::new();
    ok(&mut c, "createNode transform -n root; createNode transform -n mid; createNode transform -n leaf; connectAttr root.tx mid.tx; connectAttr mid.tx leaf.tx;");
    assert_eq!(strs(&ok(&mut c, "evaluationManager -downstreamFrom root;")), vec!["mid".to_string()]);
    assert_eq!(strs(&ok(&mut c, "evaluationManager -upstreamFrom leaf;")), vec!["mid".to_string()]);
    let walk = strs(&ok(&mut c, "evaluationManager -q -downstreamFrom root;"));
    assert_eq!(walk, vec!["0", "root", "1", "mid", "2", "leaf"]);
    ok(&mut c, "evaluationManager -ntp true transform;");
    assert_eq!(strs(&ok(&mut c, "evaluationManager -q -ntp;")), vec!["transform".to_string()]);
    assert_eq!(ok(&mut c, "evaluationManager -q -ntp transform joint;"), json!([true, false]));
    ok(&mut c, "evaluationManager -ntp false transform;");
    assert!(strs(&ok(&mut c, "evaluationManager -q -ntp;")).is_empty());
    assert_eq!(ok(&mut c, "evaluationManager -q -sfm;"), json!(true));
    ok(&mut c, "evaluationManager -sfm 0 -ia 2;");
    assert_eq!(ok(&mut c, "evaluationManager -q -sfm;"), json!(false));
    assert_eq!(ok(&mut c, "evaluationManager -q -ia;"), json!(2));
    assert_eq!(ok(&mut c, "evaluationManager -q -di;"), json!([]));
    assert_eq!(ok(&mut c, "evaluationManager -q -ft;"), json!(false));
    assert_eq!(ok(&mut c, "evaluationManager -q -mr;"), json!(true));
    assert_eq!(ok(&mut c, "evaluationManager -q -mt;"), json!(false));
    // Connections can't make cycles here, so no node is in one, with a mode on or off.
    assert!(strs(&ok(&mut c, "evaluationManager -cycleCluster mid;")).is_empty(), "mode off");
    ok(&mut c, "evaluationManager -mode \"parallel\";");
    assert!(strs(&ok(&mut c, "evaluationManager -cycleCluster mid;")).is_empty());
    assert_eq!(ok(&mut c, "evaluationManager -q -ft;"), json!(false), "a new mode clears the fallback");
}

#[test]
fn ik_system_and_handle() {
    let mut c = Core::new();
    ok(&mut c, "joint -p 0 0 0 -n a1; joint -p 0 4 1 -n a2; joint -p 0 8 0 -n a3; select -cl;");
    ok(&mut c, "joint -p 5 0 0 -n b1; joint -p 5 4 0 -n b2; select -cl;");
    ok(&mut c, "ikHandle -sj a1 -ee a3 -n rp; ikHandle -sj b1 -ee b2 -sol ikSCsolver -n sc;");
    let order = strs(&ok(&mut c, "ikSystem -q -ls;"));
    assert!(order.len() >= 2, "{order:?}");
    ok(&mut c, "ikSystem -e -ls 1 0;");
    let now = strs(&ok(&mut c, "ikSystem -q -ls;"));
    assert_eq!(now[0], order[1]);
    assert_eq!(now[1], order[0]);
    let types = strs(&ok(&mut c, "ikSystem -q -st;"));
    assert!(types.contains(&"ikRPsolver".to_string()) && types.contains(&"ikSplineSolver".to_string()));
    ok(&mut c, "ikSystem -e -ar 1;");
    assert_eq!(ok(&mut c, "ikSystem -q -ar;"), json!(true));
    // autoPriority by kind: single chain handles only.
    ok(&mut c, "ikHandle -e -p 7 rp; ikHandle -e -p 7 sc;");
    ok(&mut c, "ikSystem -e -aps 1;");
    assert_eq!(ok(&mut c, "ikHandle -q -p rp;"), json!(1));
    assert_eq!(ok(&mut c, "ikHandle -q -p sc;"), json!(1));
    // connectEffector off: the effector stops following the end joint.
    let eff = ok(&mut c, "ikHandle -q -ee rp;").as_str().unwrap().to_string();
    ok(&mut c, "ikHandle -e -ce 0 rp;");
    assert!(strs(&ok(&mut c, &format!("listConnections -s 1 -d 0 {eff}.translate;"))).is_empty());
    ok(&mut c, "ikHandle -e -ce 1 rp;");
    assert!(!strs(&ok(&mut c, &format!("listConnections -s 1 -d 0 {eff}.translate;"))).is_empty());
    // setupForRPsolver: the pole vector taken from the chain as it is, so the chain doesn't move.
    let before = nums(&ok(&mut c, "xform -q -ws -t a2;"));
    ok(&mut c, "setAttr rp.poleVectorX 1; setAttr rp.poleVectorY 0; setAttr rp.poleVectorZ 0;");
    ok(&mut c, "ikHandle -e -srp 1 rp;");
    let after = nums(&ok(&mut c, "xform -q -ws -t a2;"));
    for k in 0..3 {
        close(after[k], before[k], "the chain back where it was");
    }
    // Spline: the curve comes to the joints with -fj, and a root axis goes above the chain.
    ok(&mut c, "select -cl; joint -p 0 0 10 -n s1; joint -p 0 3 10 -n s2; joint -p 0 6 10 -n s3; select -cl;");
    ok(&mut c, "curve -d 3 -p 4 0 10 -p 4 2 10 -p 4 4 10 -p 4 6 10 -n scrv;");
    ok(&mut c, "ikHandle -sj s1 -ee s3 -sol ikSplineSolver -c scrv -fj 1 -ccv 0 -cra 1 -n sp;");
    let b = bbox(&mut c, "scrv");
    close((b[0] + b[3]) * 0.5, 0.0, "the curve on the chain");
    assert_eq!(ok(&mut c, "listRelatives -p s1;"), json!(["s1_rootAxis"]));
}

#[test]
fn image_plane_queries() {
    let mut c = Core::new();
    let dir = std::env::temp_dir().join(format!("aztek_u03_ip_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let png = dir.join("plate.png");
    image::RgbImage::new(64, 32).save(&png).unwrap();
    let ip = strs(&ok(&mut c, &format!("imagePlane -fn \"{}\" -n foo;", png.display())));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -iz {};", ip[1])), json!([64, 32]));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -qt {};", ip[1])), json!(false));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -tt {};", ip[1])), json!(false));
    ok(&mut c, "currentUnit -t film;");
    assert_eq!(ok(&mut c, &format!("imagePlane -q -nf {};", ip[1])), json!(24));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -ts {};", ip[1])), json!(2400));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -fd {};", ip[1])), json!(100));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -df {};", ip[1])), json!(false));
    ok(&mut c, "currentUnit -t \"29.97fps\";");
    assert_eq!(ok(&mut c, &format!("imagePlane -q -nf {};", ip[1])), json!(30));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -df {};", ip[1])), json!(true));
    ok(&mut c, "currentTime 12;");
    assert_eq!(ok(&mut c, &format!("imagePlane -q -tc {};", ip[1])), json!(12));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -nt {};", ip[1])), json!(false));
    assert_eq!(ok(&mut c, &format!("imagePlane -q -tf {};", ip[1])), json!(false));
    // The frame rate is the process's: what reads time is checked here, after it is put back.
    ok(&mut c, "currentUnit -t film;");
    // expression -attribute and -animated.
    ok(&mut c, "polyCube -n ec;");
    let e = ok(&mut c, "expression -o ec -a ty -s \"time * 2\" -an 2;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("expression -q -a {e};")), json!("ty"));
    assert_eq!(ok(&mut c, &format!("expression -q -an {e};")), json!(2));
    ok(&mut c, "currentTime 24;");
    close(num(&ok(&mut c, "getAttr ec.ty;")), 2.0, "drives the attribute");
    ok(&mut c, &format!("expression -e -an 0 {e};"));
    assert_eq!(ok(&mut c, &format!("expression -q -an {e};")), json!(0));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn display_layer_globals_and_members() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n m1; polyCube -n m2; createNode multiplyDivide -n nd;");
    let l = ok(&mut c, "createDisplayLayer -e -n L1;").as_str().unwrap().to_string();
    // Counts what went in; a non-DAG node is left out.
    assert_eq!(ok(&mut c, &format!("editDisplayLayerMembers {l} m1 m2 nd;")), json!(2));
    assert_eq!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))), vec!["m1".to_string(), "m2".to_string()]);
    assert_eq!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q -ufe {l};"))).len(), 2);
    // A group's transforms go in with it, unless -nr.
    ok(&mut c, "polyCube -n gc; group -n gg gc;");
    assert_eq!(ok(&mut c, &format!("editDisplayLayerMembers {l} gg;")), json!(1));
    assert_eq!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))).len(), 4);
    assert_eq!(ok(&mut c, &format!("editDisplayLayerMembers -clear {l};")), json!(4));
    assert!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))).is_empty());
    ok(&mut c, &format!("editDisplayLayerMembers -nr {l} m1;"));
    assert_eq!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))), vec!["m1".to_string()]);

    // The current layer takes new objects.
    assert_eq!(ok(&mut c, "editDisplayLayerGlobals -q -cdl;"), json!("defaultLayer"));
    ok(&mut c, &format!("editDisplayLayerGlobals -cdl {l};"));
    assert_eq!(ok(&mut c, "editDisplayLayerGlobals -q -cdl;"), json!(l.clone()));
    ok(&mut c, "polySphere -n fresh;");
    assert!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))).contains(&"fresh".to_string()));
    ok(&mut c, "editDisplayLayerGlobals -uc 0;");
    assert_eq!(ok(&mut c, "editDisplayLayerGlobals -q -uc;"), json!(false));
    ok(&mut c, "polySphere -n outside;");
    assert!(!strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l};"))).contains(&"outside".to_string()));
    // Base id: where new layers' numbers start.
    ok(&mut c, "editDisplayLayerGlobals -bi 10 -mt 2;");
    assert_eq!(ok(&mut c, "editDisplayLayerGlobals -q -bi;"), json!(10));
    assert_eq!(ok(&mut c, "editDisplayLayerGlobals -q -mt;"), json!(2));
    let l2 = ok(&mut c, "createDisplayLayer -e -n L2;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("getAttr {l2}.identification;")), json!(10));
}

#[test]
fn scene_flags() {
    let mut c = Core::new();
    // duplicate -fullPath.
    ok(&mut c, "polyCube -n dc; group -n dg dc;");
    assert_eq!(ok(&mut c, "duplicate -f dc;"), json!(["|dg|dc1"]));
    // findType -f, the node itself counting.
    ok(&mut c, "createNode transform -n silly; createNode transform -n putty; connectAttr putty.tx silly.tx;");
    assert_eq!(strs(&ok(&mut c, "findType -d -t \"transform\" silly;")), vec!["silly".to_string(), "putty".to_string()]);
    assert_eq!(strs(&ok(&mut c, "findType -d -f -t \"transform\" putty;")), vec!["putty".to_string(), "silly".to_string()]);
    // getAttr -caching, as setAttr -caching left it.
    assert_eq!(ok(&mut c, "getAttr -ca silly.tx;"), json!(true));
    ok(&mut c, "setAttr -ca off silly.ty;");
    assert_eq!(ok(&mut c, "getAttr -ca silly.ty;"), json!(false));
    // hide -all: every top level object.
    let cam = ok(&mut c, "getAttr persp.visibility;");
    ok(&mut c, "hide -all;");
    assert_eq!(ok(&mut c, "getAttr dg.visibility;"), json!(false));
    assert_eq!(ok(&mut c, "getAttr silly.visibility;"), json!(false));
    assert_eq!(ok(&mut c, "getAttr persp.visibility;"), cam, "the scene's cameras aside");
    assert_eq!(ok(&mut c, "hide -tv dg;"), json!(1));
    // getParticleAttr -object, the older way of naming them.
    ok(&mut c, "particle -p 0 0 0 -p 2 0 0 -n pt;");
    let v = nums(&ok(&mut c, "getParticleAttr -at position -o ptShape;"));
    close(v[0], 1.0, "mean x");
    // excludeObjectDisplayPreset.
    assert_eq!(ok(&mut c, "excludeObjectDisplayPreset -listPresets;"), json!(["All", "None"]));
}

#[test]
fn geometry_attr_info_chains() {
    let mut c = Core::new();
    ok(&mut c, "polyCylinder -r 1 -h 6 -sx 4 -sy 5 -sz 1 -name myGeo;");
    let och = strs(&ok(&mut c, "geometryAttrInfo -och myGeoShape.outMesh;"));
    assert!(och.first().is_some_and(|p| p.starts_with("polyCylinder1.")), "{och:?}");
    assert_eq!(och.last().map(String::as_str), Some("myGeoShape.outMesh"));
    let pch = strs(&ok(&mut c, "geometryAttrInfo -pch myGeoShape.outMesh;"));
    assert!(pch.len() > och.len(), "inputs too: {pch:?}");
    assert!(pch.iter().any(|p| p == "myGeoShape.inMesh"), "{pch:?}");
    // Tags: the hash moves when one is added, and the history names where.
    let h0 = ok(&mut c, "geometryAttrInfo -hsh myGeoShape.outMesh;");
    ok(&mut c, "componentTag -cr -ntn top myGeo.vtx[0:3];");
    let h1 = ok(&mut c, "geometryAttrInfo -hsh myGeoShape.outMesh;");
    assert_ne!(h0, h1);
    let hist = strs(&ok(&mut c, "geometryAttrInfo -cth myGeoShape.outMesh;"));
    assert_eq!(hist.first().map(String::as_str), Some("top"), "{hist:?}");
    let chh = ok(&mut c, "geometryAttrInfo -chh myGeoShape.outMesh;");
    assert!(chh.as_str().is_some_and(|s| !s.is_empty()));
}

#[test]
fn ui_flags() {
    let mut c = Core::new();
    let w = ok(&mut c, "window;").as_str().unwrap().to_string();
    let form = ok(&mut c, &format!("formLayout -parent {w};")).as_str().unwrap().to_string();
    ok(&mut c, &format!("formLayout -e -gsp 4 -mar 2 -mr4 1 2 3 4 {form};"));
    let dock = ok(&mut c, &format!("dockControl -area \"left\" -content {w} -fw 1 -sl \"vertical\" -epo 0;")).as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("dockControl -q -fw {dock};")), json!(true));
    assert_eq!(ok(&mut c, &format!("dockControl -q -fh {dock};")), json!(false));
    assert_eq!(ok(&mut c, &format!("dockControl -q -epo {dock};")), json!(false));
    assert_eq!(ok(&mut c, &format!("dockControl -q -ret {dock};")), json!(true));
    // -state round trip.
    let st = ok(&mut c, &format!("dockControl -q -st {dock};")).as_str().unwrap().to_string();
    assert!(st.chars().all(|ch| ch.is_ascii_hexdigit()), "{st}");
    ok(&mut c, &format!("dockControl -e -area \"right\" {dock};"));
    ok(&mut c, &format!("dockControl -e -st \"{st}\" {dock};"));
    assert_eq!(ok(&mut c, &format!("dockControl -q -a {dock};")), json!("left"));
    assert!(c.mel_checked(&format!("dockControl -e -st \"zz\" {dock};")).is_err());

    let fl = ok(&mut c, &format!("flowLayout -hr 0 -gsp 3 -parent {form};")).as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("flowLayout -q -hr {fl};")), json!(false));
    assert_eq!(ok(&mut c, &format!("flowLayout -q -v {fl};")), json!(true));
    let fr = ok(&mut c, &format!("frameLayout -bgs 1 -parent {form};")).as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("frameLayout -q -bgs {fr};")), json!(true));
    ok(&mut c, &format!("frameLayout -e -mar 5 -mr4 1 1 1 1 -gsp 2 {fr};"));
    let gl = ok(&mut c, &format!("gridLayout -parent {form};")).as_str().unwrap().to_string();
    ok(&mut c, &format!("gridLayout -e -mar 3 {gl};"));
    let sb = ok(&mut c, &format!("floatScrollBar -min -100 -max 100 -v 0 -s 1 -ls 10 -parent {form};")).as_str().unwrap().to_string();
    close(num(&ok(&mut c, &format!("floatScrollBar -q -ls {sb};"))), 10.0, "large step");
    ok(&mut c, &format!("floatScrollBar -e -ls 25 {sb};"));
    close(num(&ok(&mut c, &format!("floatScrollBar -q -ls {sb};"))), 25.0, "large step edited");

    // headsUpMessage's offsets, and editorTemplate's new flags in a template.
    ok(&mut c, "headsUpMessage -ho 20 -vo 10 \"Ouch!\";");
    ok(&mut c, "editorTemplate -beginScrollLayout; editorTemplate -bn; editorTemplate -addControl \"tx\" -ann \"Moves it\" -afo; editorTemplate -en; editorTemplate -fr 1; editorTemplate -acp; editorTemplate -aac;");
    let t = aztek_host::script_objects::take_template();
    let kinds: Vec<&str> = t.iter().filter_map(|x| x["kind"].as_str()).collect();
    assert_eq!(kinds, vec!["beginNoOptimize", "control", "endNoOptimize", "forceRebuild", "components"]);
    assert_eq!(t[1]["annotateFieldOnly"], json!(true));

    // fileDialog2's options pane: with no dialog to show, it runs the create and init procedures, then cancel.
    ok(&mut c, "global string $gOpts[]; clear $gOpts;");
    ok(&mut c, "global proc u03Create(string $p) { global string $gOpts[]; $gOpts[size($gOpts)] = \"create\"; checkBox -label \"Opt\" -parent $p u03Box; }");
    ok(&mut c, "global proc u03Init(string $p, string $f) { global string $gOpts[]; $gOpts[size($gOpts)] = (\"init \" + $f); }");
    ok(&mut c, "global proc u03Cancel() { global string $gOpts[]; $gOpts[size($gOpts)] = \"cancel\"; }");
    ok(&mut c, "fileDialog2 -ff \"Scenes (*.ma);;All (*.*)\" -ocr \"u03Create\" -oin \"u03Init\" -oca \"u03Cancel\" -oti \"Options\";");
    assert_eq!(ok(&mut c, "global string $gOpts[]; $tmp = $gOpts;"), json!(["create", "init Scenes (*.ma)", "cancel"]));
    assert_eq!(ok(&mut c, "control -exists u03Box;"), json!(false), "the pane goes with the dialog");
}

#[test]
fn expression_editor_listen() {
    let mut c = Core::new();
    ok(&mut c, "global string $gHeard[]; clear $gHeard;");
    ok(&mut c, "global proc EErebuildAttrList(string $n) { global string $gHeard[]; $gHeard[size($gHeard)] = (\"attrs \" + $n); }");
    ok(&mut c, "global proc EEupdateWindow(string $n, string $a) { global string $gHeard[]; $gHeard[size($gHeard)] = (\"name \" + $n); }");
    ok(&mut c, "global proc EEupdateExprList() { global string $gHeard[]; $gHeard[size($gHeard)] = \"exprs\"; }");
    ok(&mut c, "polyCube -n lc; expression -s \"lc.tx = 1;\" -n ex1;");
    ok(&mut c, "expressionEditorListen -la lc; expressionEditorListen -ln lc; expressionEditorListen -le ex1;");
    ok(&mut c, "addAttr -ln extra lc; rename lc lc2; expression -e -s \"lc2.tx = 2;\" ex1;");
    c.idle();
    let heard = strs(&ok(&mut c, "global string $gHeard[]; $tmp = $gHeard;"));
    assert!(heard.iter().any(|h| h.starts_with("attrs ")), "{heard:?}");
    assert!(heard.contains(&"name lc2".to_string()), "{heard:?}");
    assert!(heard.contains(&"exprs".to_string()), "{heard:?}");
    // Stopped: nothing more.
    ok(&mut c, "clear $gHeard; expressionEditorListen -sla lc2; expressionEditorListen -sln lc2; expressionEditorListen -sle ex1;");
    ok(&mut c, "addAttr -ln more lc2; rename lc2 lc3;");
    c.idle();
    assert!(strs(&ok(&mut c, "global string $gHeard[]; $tmp = $gHeard;")).is_empty());
}

/// The flags of the reference's these commands lacked: every one is now a flag of the command.
const FLAGS: &[(&str, &str)] = &[
    ("dockControl", "dockStation ds enablePopupOption epo fh fixedHeight fixedWidth fw ret sl splitLayout st state"),
    ("doubleProfileBirailSurface", "bl blendFactor caching cch ch constructionHistory e edit n name nds nodeState o object po polygon q query tangentContinuityProfile1 tangentContinuityProfile2 tm tp1 tp2 transformMode"),
    ("duplicate", "f fullPath"),
    ("duplicateCurve", "caching cch e edit max maxValue mergeItems mi min minValue nds nodeState q query"),
    ("duplicateSurface", "e edit faceCountU faceCountV fcu fcv ffu ffv firstFaceU firstFaceV q query"),
    ("editDisplayLayerGlobals", "baseId bi cdl currentDisplayLayer mergeType mt q query uc useCurrent"),
    ("editDisplayLayerMembers", "clear clr ufe ufeObjects"),
    ("editorTemplate", "acp addAdskAssetControls afo annotateFieldOnly bn en forceRebuild fr"),
    ("evaluationManager", "ccl cycleCluster di disableInfo downstreamFrom dst empty fallbackTriggered ft ia idleAction manipulationPrevalidation manipulationReady mp mr mt nodeTypeGloballySerialize nodeTypeParallel nodeTypeSerialize nodeTypeUntrusted ntg ntp nts ntu reduceGraphRebuild rgr safeMode sfm upstreamFrom ust"),
    ("excludeObjectDisplayPreset", ""),
    ("expression", "a an animated attribute"),
    ("expressionEditorListen", "la le lf listenFile listenForAttr listenForExpression listenForName ln sla sle sln stopListenForAttr stopListenForExpression stopListenForName"),
    ("extendCurve", "caching cch cos curveOnSurface e edit em extendMethod inputPoint ip jn join nc nds noChanges nodeState pointX pointY pointZ px py pz q query range removeMultipleKnots rmk rn"),
    ("extendSurface", "e edit q query"),
    ("extrude", "caching cch degreeAlongLength directionX directionY directionZ dl dx dy dz e edit mergeItems mi nds nodeState o object p pivot q query rb rebuild scs subCurveSubSurface"),
    ("fileDialog2", "oca optionsUITitle oti"),
    ("filletCurve", "e edit q query replaceOriginal rpo"),
    ("findType", "f"),
    ("fitBspline", "caching cch e edit nds nodeState q query"),
    ("floatScrollBar", "largeStep ls"),
    ("flow", "q query"),
    ("flowLayout", "generalSpacing gsp horizontal hr mar margins margins4 mr4"),
    ("formLayout", "generalSpacing gsp mar margins margins4 mr4"),
    ("frameLayout", "backgroundShade bgs generalSpacing gsp mar margins margins4 mr4"),
    ("freeFormFillet", "b e edit q query"),
    ("freeze", "all allNodes displayLayers dl dn downstream f fd forceDownstream frozen i invisible n noFreeze q query uf unfreeze up upstream"),
    ("freezeOptions", "displayLayers dl dn downstream ep explicitPropagation inv invisible q query referencedNodes rn rt runtimePropagation up upstream"),
    ("geomBind", "e edit q query"),
    ("geometryAttrInfo", "chh componentTagHash componentTagHistory componentTagHistoryHash cth gid groupId hsh och outputPlugChain pch plugChain"),
    ("geometryConstraint", "l layer"),
    ("getAttr", "ca caching"),
    ("getParticleAttr", "o object"),
    ("globalStitch", "e edit q query"),
    ("gridLayout", "generalSpacing gsp mar margins margins4 mr4"),
    ("hardenPointCurve", "caching cch e edit nds nodeState q query"),
    ("headsUpMessage", "ho"),
    ("hide", "all allObjects"),
    ("ikHandle", "ce connectEffector cra createRootAxis fj freezeJoints setupForRPsolver srp"),
    ("ikSystem", "allowRotation ap apm aps ar autoPriority autoPriorityMC autoPrioritySC list solverTypes st"),
    ("imagePlane", "df dropFrame fd frameDuration imageSize iz negTimesOK nf nt numFrames qt quickTime tc tf timeCode timeCodeTrack timeScale ts tt twentyFourHourMax"),
];

#[test]
fn every_flag_is_there() {
    let c = Core::new();
    let mut missing = Vec::new();
    for (cmd, flags) in FLAGS {
        let Some(def) = c.s.commands.find(cmd) else {
            missing.push(format!("{cmd}: no command"));
            continue;
        };
        for fl in flags.split_whitespace() {
            if !def.flags.iter().any(|f| f.name == fl || f.short_name == fl) {
                missing.push(format!("{cmd} -{fl}"));
            }
        }
    }
    assert!(missing.is_empty(), "{missing:?}");
}
