//! Commands of the reference from selectPriority to text: selectPriority's further types, selectionConnection's
//! character, cache and editor lists and templates, setAttr -capacityHint, setInfinity's hierarchy, shape and control
//! point options, setParticleAttr's random and relative values, sets' component kinds, colours and -afterFilters, the
//! shelf and tab layouts' and buttons' flags, showHidden, simplify's tolerances, the NURBS commands' history nodes
//! (birail, smoothCurve, smoothTangentSurface, sphere, squareSurface, the stitches), skinCluster's deformer flags and
//! volume binding, snapMode's settings, softMod's flags, softSelect's curves, spaceLocator -q and -e,
//! substituteGeometry's options, symbolCheckBox's images, symmetricModelling's seam curve and text -dropRectCallback.

use aztek_host::Core;
use serde_json::{json, Value};

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    match c.mel_checked(src) {
        Ok(v) => panic!("{src}: should fail, gave {v}"),
        Err(e) => e,
    }
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
            Value::String(s) => out.extend(s.parse::<f64>().ok()),
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

fn point(c: &mut Core, p: &str) -> Vec<f64> {
    nums(&ok(c, &format!("pointPosition -w {p};")))
}

#[test]
fn select_priority_types() {
    let mut c = Core::new();
    ok(&mut c, "selectPriority -fo 7 -hs 6 -ha 4 -nl 9 -ps 3 -nps 2 -sk 8 -sr 6 -xyz 5 -msh 4 -mtp 3 -mtt 2 -sme 1 -smf 1 -smp 1 -smu 1 -fl 2;");
    assert_eq!(num(&ok(&mut c, "selectPriority -q -follicle;")), 7.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -hs;")), 6.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -nonlinear;")), 9.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -surfaceKnot;")), 8.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -locatorXYZ;")), 5.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -fluid;")), 2.0);
    // Untouched: the default.
    assert_eq!(num(&ok(&mut c, "selectPriority -q -motionTrailPoint;")), 3.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -particleShape;")), 3.0);
    // A user-defined type by name.
    ok(&mut c, "selectPriority -byName \"myType\" true;");
    assert_eq!(num(&ok(&mut c, "selectPriority -q -queryByName \"myType\";")), 1.0);
    assert_eq!(num(&ok(&mut c, "selectPriority -q -qbn \"other\";")), 0.0);
}

#[test]
fn selection_connection_lists_and_templates() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; polySphere -n ball; select -cl;");
    ok(&mut c, "character -n hero box.tx box.ty;");
    ok(&mut c, "selectionConnection -acl heroes;");
    ok(&mut c, "select box;");
    assert_eq!(strs(&ok(&mut c, "selectionConnection -q -obj heroes;")), vec!["hero"]);
    ok(&mut c, "select ball;");
    assert!(strs(&ok(&mut c, "selectionConnection -q -obj heroes;")).is_empty());
    assert_eq!(ok(&mut c, "selectionConnection -q -id heroes;"), json!("activeCharacterList"));
    ok(&mut c, "selectionConnection -atc caches;");
    assert_eq!(ok(&mut c, "selectionConnection -q -activeCacheList caches;"), json!(true));
    // An editor's main list connection.
    ok(&mut c, "selectionConnection -obj ball held;");
    ok(&mut c, "window w; columnLayout; outlinerEditor -mlc held ed1;");
    ok(&mut c, "selectionConnection -ed ed1 follows;");
    assert_eq!(ok(&mut c, "selectionConnection -q -ed follows;"), json!("ed1"));
    assert_eq!(strs(&ok(&mut c, "selectionConnection -q -obj follows;")), vec!["ball"]);
    // A connection list, by its short name.
    ok(&mut c, "selectionConnection -lst lists;");
    assert_eq!(ok(&mut c, "selectionConnection -q -connectionList lists;"), json!(true));
    // Templates: defaults for later connections.
    ok(&mut c, "uiTemplate t1; selectionConnection -dt t1 -wl;");
    ok(&mut c, "selectionConnection -ut t1 fromTemplate;");
    assert_eq!(ok(&mut c, "selectionConnection -q -wl fromTemplate;"), json!(true));
    ok(&mut c, "selectionConnection plain;");
    assert_eq!(ok(&mut c, "selectionConnection -q -wl plain;"), json!(false));
}

#[test]
fn set_attr_capacity_hint() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b;");
    ok(&mut c, "setAttr -ch 24 b.tx 2;");
    close(num(&ok(&mut c, "getAttr b.tx;")), 2.0, "set with a hint");
}

#[test]
fn set_infinity_hierarchy_and_shape() {
    let mut c = Core::new();
    ok(&mut c, "group -em -n top; polyCube -n kid; parent kid top;");
    ok(&mut c, "setKeyframe -t 1 -v 0 top.tx; setKeyframe -t 10 -v 5 top.tx; setKeyframe -t 1 -v 0 kid.ty; setKeyframe -t 10 -v 3 kid.ty;");
    ok(&mut c, "setInfinity -poi cycle -hi below top;");
    assert_eq!(strs(&ok(&mut c, "setInfinity -q -poi kid.ty;")), vec!["cycle"]);
    ok(&mut c, "setInfinity -pri linear -hi none top;");
    assert_eq!(strs(&ok(&mut c, "setInfinity -q -pri kid.ty;")), vec!["constant"]);
    // A shape's channels come in unless -s 0.
    ok(&mut c, "pointLight -n lit; setKeyframe -t 1 -v 1 litShape.intensity; setKeyframe -t 5 -v 0 litShape.intensity;");
    ok(&mut c, "setInfinity -poi oscillate -s 0 lit;");
    assert_eq!(strs(&ok(&mut c, "setInfinity -q -poi litShape.intensity;")), vec!["constant"]);
    ok(&mut c, "setInfinity -poi oscillate lit;");
    assert_eq!(strs(&ok(&mut c, "setInfinity -q -poi litShape.intensity;")), vec!["oscillate"]);
    // -cp takes in control points as well; the channels are as before.
    ok(&mut c, "setInfinity -poi linear -cp 1 -hi both kid;");
    assert_eq!(strs(&ok(&mut c, "setInfinity -q -poi top.tx;")), vec!["linear"]);
}

#[test]
fn set_particle_attr_random_and_relative() {
    let mut c = Core::new();
    // What is set shows from the next frame on; X and Z have nothing acting on them.
    ok(&mut c, "nParticle -p 0 0 0 -p 1 0 0 -p 2 0 0 -n pt; currentTime 2;");
    ok(&mut c, "setParticleAttr -at velocity -vv 1 2 3 pt;");
    ok(&mut c, "currentTime 3;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 pt;"));
    close(v[0], 1.0, "set");
    close(v[5], 3.0, "set on each");
    ok(&mut c, "setParticleAttr -at velocity -vv 1 1 1 -r 1 pt;");
    ok(&mut c, "currentTime 4;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 pt;"));
    close(v[0], 2.0, "relative adds");
    close(v[2], 4.0, "relative adds");
    // -object takes every particle of it whatever is named.
    ok(&mut c, "setParticleAttr -at velocity -vv 0 0 0 -o pt pt.pt[0];");
    ok(&mut c, "currentTime 5;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 pt;"));
    assert!(v.chunks(3).all(|p| p[0].abs() < 1e-9 && p[2].abs() < 1e-9), "{v:?}");
    // Random amounts stay within their range and differ between particles.
    ok(&mut c, "setParticleAttr -at velocity -vv 0 0 0 -rv 1 0 0 pt;");
    ok(&mut c, "currentTime 6;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 pt;"));
    assert!(v.chunks(3).all(|p| p[0].abs() <= 1.0 + 1e-9 && p[2].abs() < 1e-9), "{v:?}");
    assert!((v[0] - v[3]).abs() > 1e-9 || (v[3] - v[6]).abs() > 1e-9, "{v:?}");
    ok(&mut c, "setParticleAttr -at velocity -vv 0 0 0 -rf 0.5 pt;");
    ok(&mut c, "currentTime 7;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 pt;"));
    assert!(v.chunks(3).all(|p| p[0].abs() <= 0.5 + 1e-9 && p[2].abs() <= 0.5 + 1e-9), "{v:?}");
    // A float attribute given a vector takes its length.
    ok(&mut c, "setParticleAttr -at opacityPP -vv 3 4 0 pt;");
}

#[test]
fn sets_component_kinds_and_colours() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b;");
    let set = ok(&mut c, "sets -v -n vset b.vtx[0:1] b.e[0] b.f[1];");
    assert_eq!(set, json!("vset"));
    assert_eq!(ok(&mut c, "sets -q -v vset;"), json!(true));
    assert_eq!(ok(&mut c, "sets -q -eg vset;"), json!(false));
    assert_eq!(strs(&ok(&mut c, "sets -q vset;")), vec!["b.vtx[0:1]"]);
    ok(&mut c, "sets -add vset b.f[2] b.vtx[5];");
    assert_eq!(strs(&ok(&mut c, "sets -q vset;")), vec!["b.vtx[0:1]", "b.vtx[5]"]);
    ok(&mut c, "sets -eg -n eset b.e[2] b.vtx[3];");
    assert_eq!(strs(&ok(&mut c, "sets -q eset;")), vec!["b.e[2]"]);
    ok(&mut c, "sets -ep -n epset;");
    assert_eq!(ok(&mut c, "sets -q -editPoints epset;"), json!(true));
    // Channel set colours.
    ok(&mut c, "sets -coi 3 -n cs b;");
    assert_eq!(num(&ok(&mut c, "sets -q -coi cs;")), 3.0);
    ok(&mut c, "sets -e -coi -1 -csc 1 0.5 0 cs;");
    assert_eq!(num(&ok(&mut c, "sets -q -channelSetColorIndex cs;")), -1.0);
    assert_eq!(nums(&ok(&mut c, "sets -q -csc cs;")), vec![1.0, 0.5, 0.0]);
}

#[test]
fn sets_after_filters_leave_the_deformer() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b; cluster -n cl b.vtx[0];");
    ok(&mut c, "setAttr clHandle.tx 1;");
    let before = point(&mut c, "b.vtx[1]");
    // Into the set after the deformer's filters: the point is a member but doesn't move.
    ok(&mut c, "sets -af -add clSet b.vtx[1];");
    let after = point(&mut c, "b.vtx[1]");
    close(after[0], before[0], "-af keeps the deformer as it was");
    assert!(ok(&mut c, "sets -im clSet b.vtx[1];") == json!(true));
    // Without it the point joins the deformer.
    ok(&mut c, "sets -add clSet b.vtx[2];");
    let p2 = point(&mut c, "b.vtx[2]");
    close(p2[0], 1.5, "a member the deformer moves");
}

#[test]
fn show_hidden_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; group -n g1 a; hide g1 a;");
    ok(&mut c, "showHidden a;");
    assert_eq!(ok(&mut c, "getAttr a.v;"), json!(true));
    assert_eq!(ok(&mut c, "getAttr g1.v;"), json!(false));
    ok(&mut c, "hide a; showHidden -a a;");
    assert_eq!(ok(&mut c, "getAttr g1.v;"), json!(true));
    assert_eq!(ok(&mut c, "getAttr a.v;"), json!(true));
    // -below takes the object named too.
    ok(&mut c, "hide g1 a; showHidden -b g1;");
    assert_eq!(ok(&mut c, "getAttr g1.v;"), json!(true));
    assert_eq!(ok(&mut c, "getAttr a.v;"), json!(true));
    ok(&mut c, "polySphere -n s; hide s a;");
    ok(&mut c, "showHidden -all;");
    assert_eq!(ok(&mut c, "getAttr s.v;"), json!(true));
    // What the last hide hid.
    ok(&mut c, "hide s; showHidden -lh;");
    assert_eq!(ok(&mut c, "getAttr s.v;"), json!(true));
}

#[test]
fn simplify_tolerances() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b;");
    for f in 0..=10 {
        ok(&mut c, &format!("setKeyframe -t {f} -v {} -itt linear -ott linear b.tx;", f as f64 * 0.5));
    }
    assert_eq!(num(&ok(&mut c, "simplify -vt 0.01 -tt 0.05 b.tx;")), 1.0);
    assert_eq!(num(&ok(&mut c, "keyframe -q -kc b.tx;")), 2.0);
    // A driven curve goes by -ft.
    ok(&mut c, "polyCube -n d;");
    for f in 0..=4 {
        ok(&mut c, &format!("setDrivenKeyframe -cd b.ty -dv {} -v {} d.tx;", f as f64 * 0.01, f as f64 * 0.3));
    }
    let n0 = num(&ok(&mut c, "keyframe -q -kc d.tx;"));
    ok(&mut c, "simplify -vt 0.000001 -ft 0.025 d.tx;");
    let n1 = num(&ok(&mut c, "keyframe -q -kc d.tx;"));
    assert!(n1 < n0, "{n0} {n1}");
}

#[test]
fn nurbs_history_query_and_edit() {
    let mut c = Core::new();
    // sphere
    ok(&mut c, "sphere -n s -r 2;");
    close(num(&ok(&mut c, "sphere -q -r s;")), 2.0, "sphere radius");
    ok(&mut c, "sphere -e -r 3 makeNurbSphere1;");
    close(num(&ok(&mut c, "getAttr makeNurbSphere1.radius;")), 3.0, "edited");
    ok(&mut c, "sphere -e -nds 2 makeNurbSphere1;");
    assert_eq!(num(&ok(&mut c, "sphere -q -nds s;")), 2.0);
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox s;"));
    ok(&mut c, "sphere -e -r 5 makeNurbSphere1;");
    let bb2 = nums(&ok(&mut c, "exactWorldBoundingBox s;"));
    close(bb2[3], bb[3], "blocking keeps what it gave");
    ok(&mut c, "sphere -cch 1 -n s2;");
    assert_eq!(ok(&mut c, "sphere -q -cch s2;"), json!(true));
    // smoothCurve
    ok(&mut c, "curve -n cv -d 3 -p 0 0 0 -p 1 2 0 -p 2 -2 0 -p 3 2 0 -p 4 0 0;");
    ok(&mut c, "smoothCurve -s 0.3 cv;");
    close(num(&ok(&mut c, "smoothCurve -q -s cv;")), 0.3, "smoothness");
    ok(&mut c, "smoothCurve -e -s 0.8 cv;");
    close(num(&ok(&mut c, "smoothCurve -q -s cv;")), 0.8, "edited");
    // smoothTangentSurface
    ok(&mut c, "nurbsPlane -n p -ax 0 1 0 -w 4 -u 4 -v 1 -d 1;");
    ok(&mut c, "smoothTangentSurface -p 2 -d 1 p;");
    close(num(&ok(&mut c, "smoothTangentSurface -q -p p;")), 2.0, "parameter");
    ok(&mut c, "smoothTangentSurface -e -d 0 p;");
    assert_eq!(num(&ok(&mut c, "smoothTangentSurface -q -d p;")), 0.0);
    // singleProfileBirailSurface
    ok(&mut c, "curve -n r0 -d 1 -p 0 0 0 -p 0 0 4; curve -n r1 -d 1 -p 2 0 0 -p 2 0 4; curve -n pr -d 1 -p 0 0 0 -p 1 1 0 -p 2 0 0;");
    let r = ok(&mut c, "singleProfileBirailSurface pr r0 r1;");
    let node = strs(&r)[1].clone();
    ok(&mut c, &format!("singleProfileBirailSurface -e -nds 1 {node};"));
    assert_eq!(num(&ok(&mut c, "singleProfileBirailSurface -q -nodeState birailSurface1;")), 1.0);
}

#[test]
fn square_surface_settings() {
    let mut c = Core::new();
    ok(&mut c, "curve -n a -d 1 -p 0 0 0 -p 2 0 0; curve -n b -d 1 -p 2 0 0 -p 2 0 2; curve -n e -d 1 -p 2 0 2 -p 0 0 2; curve -n d -d 1 -p 0 0 2 -p 0 0 0;");
    let r = ok(&mut c, "squareSurface -ct1 1 -ct2 3 -ct3 2 -ct4 1 -cfc 7 -rc2 1 a b e d;");
    let node = strs(&r)[1].clone();
    assert_eq!(num(&ok(&mut c, &format!("squareSurface -q -ct2 {node};"))), 3.0);
    assert_eq!(num(&ok(&mut c, "squareSurface -q -cfc squareSurface1;")), 7.0);
    assert_eq!(ok(&mut c, "squareSurface -q -rc2 squareSurface1;"), json!(true));
    // Rebuilt cubic, the surface comes out of a higher degree.
    let shape = strs(&ok(&mut c, "listRelatives -s squareSurface1;"))[0].clone();
    let degree = |c: &mut Core| num(&ok(c, &format!("getAttr {shape}.degreeU;"))) + num(&ok(c, &format!("getAttr {shape}.degreeV;")));
    let spans_rebuilt = degree(&mut c);
    ok(&mut c, "squareSurface -e -rc2 0 squareSurface1;");
    let spans_plain = degree(&mut c);
    assert!(spans_rebuilt > spans_plain, "{spans_rebuilt} {spans_plain}");
    ok(&mut c, "squareSurface -e -ct4 2 squareSurface1;");
    assert_eq!(num(&ok(&mut c, "squareSurface -q -ct4 squareSurface1;")), 2.0);
    // Three curves do too.
    ok(&mut c, "curve -n t0 -d 1 -p 0 0 5 -p 2 0 5; curve -n t1 -d 1 -p 2 0 5 -p 1 0 7; curve -n t2 -d 1 -p 1 0 7 -p 0 0 5;");
    assert_eq!(strs(&ok(&mut c, "squareSurface -ct1 1 -ct2 1 -ct3 1 t0 t1 t2;")).len(), 2);
    bad(&mut c, "squareSurface -ct1 7 a b e d;");
}

#[test]
fn stitch_surface_weights_and_continuity() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n a -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; nurbsPlane -n b -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; setAttr b.tx 3;");
    // All the weight on the second edge: the first surface goes all the way to it.
    let r = ok(&mut c, "stitchSurface -wt0 0 -wt1 1 a b;");
    let nodes: Vec<String> = strs(&r).into_iter().filter(|x| x.starts_with("stitchSrf")).collect();
    assert_eq!(nodes.len(), 2);
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox a;"));
    close(bb[3], 2.0, "first moves to the second");
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox b;"));
    close(bb[0], 2.0, "second stays");
    // Query and edit reach the node, and both surfaces' nodes.
    close(num(&ok(&mut c, "stitchSurface -q -wt0 a;")), 0.0, "weight0");
    ok(&mut c, "stitchSurface -e -wt0 0.5 -wt1 0.5 a;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox a;"))[3], 1.5, "halfway");
    close(nums(&ok(&mut c, "exactWorldBoundingBox b;"))[0], 1.5, "halfway");
    // -bias 0 gives the input back.
    ok(&mut c, "stitchSurface -e -b 0 a;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox a;"))[3], 1.0, "bias 0 on the first");
    ok(&mut c, "stitchSurface -e -b 1 -pc 0 a;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox a;"))[3], 1.0, "no positional continuity");
    assert_eq!(ok(&mut c, "stitchSurface -q -pc a;"), json!(false));
    // Point constraints hold a CV.
    ok(&mut c, "stitchSurface -e -pc 1 -ci 1 -cj 0 a;");
    let held = point(&mut c, "a.cv[1][0]");
    close(held[0], 1.0, "held by -ci -cj");
    let free = point(&mut c, "a.cv[1][1]");
    close(free[0], 1.5, "the other edge CV moves");
    ok(&mut c, "stitchSurface -e -tpp 0 a;");
    close(point(&mut c, "a.cv[1][0]")[0], 1.5, "position constraint off");
    // Tangential continuity lines the rows either side up.
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n a -ax 0 1 0 -w 2 -lr 1 -u 2 -v 1 -d 1; nurbsPlane -n b -ax 0 1 0 -w 2 -lr 1 -u 2 -v 1 -d 1; setAttr b.tx 2; setAttr b.ty 0; move -r 0 1 0 b.cv[1][0:1];");
    ok(&mut c, "stitchSurface -tc 1 a b;");
    let (inner, edge, other) = (point(&mut c, "a.cv[1][0]"), point(&mut c, "a.cv[2][0]"), point(&mut c, "b.cv[1][0]"));
    let d1 = [edge[0] - inner[0], edge[1] - inner[1]];
    let d2 = [other[0] - edge[0], other[1] - edge[1]];
    let cross = d1[0] * d2[1] - d1[1] * d2[0];
    assert!(cross.abs() < 1e-6, "rows line up: {inner:?} {edge:?} {other:?}");
    assert_eq!(ok(&mut c, "stitchSurface -q -tc a;"), json!(true));
}

#[test]
fn stitch_surface_points_node() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n a -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; nurbsPlane -n b -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; setAttr b.tx 3;");
    let r = ok(&mut c, "stitchSurfacePoints -ewt 0 a.cv[1][0] b.cv[0][0];");
    let list = strs(&r);
    assert_eq!(list.len(), 4, "{r}");
    let (x, y) = (point(&mut c, "a.cv[1][0]"), point(&mut c, "b.cv[0][0]"));
    close(x[0], 1.0, "the first has the weight");
    close(y[0], 1.0, "the second goes to it");
    close(num(&ok(&mut c, "stitchSurfacePoints -q -b a;")), 1.0, "bias");
    ok(&mut c, "stitchSurfacePoints -e -b 0.5 b;");
    close(point(&mut c, "b.cv[0][0]")[0], 1.5, "half way back");
    ok(&mut c, "stitchSurfacePoints -e -nds 1 b;");
    close(point(&mut c, "b.cv[0][0]")[0], 2.0, "no effect");
    bad(&mut c, "stitchSurfacePoints a.cv[0][0] a.cv[1][0];");
    // Equal weights meet at the mean, as one undo.
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n a -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; nurbsPlane -n b -ax 0 1 0 -w 2 -lr 1 -u 1 -v 1 -d 1; setAttr b.tx 3;");
    ok(&mut c, "stitchSurfacePoints -ch 0 a.cv[1][0] b.cv[0][0];");
    close(point(&mut c, "a.cv[1][0]")[0], 1.5, "mean");
}

#[test]
fn skin_cluster_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCylinder -n m -h 4 -sy 4; select -cl; joint -n j1 -p 0 -2 0; joint -n j2 -p 0 0 0; joint -n j3 -p 0 2 0; select -cl;");
    ok(&mut c, "cluster -n cl m;");
    let r = ok(&mut c, "skinCluster -n sk -frontOfChain -exclusive skinPart j1 m;");
    assert_eq!(strs(&r), vec!["sk"]);
    // In front of the cluster.
    let hist = strs(&ok(&mut c, "listHistory m;"));
    let (ps, pc) = (hist.iter().position(|x| x == "sk").unwrap(), hist.iter().position(|x| x == "cl").unwrap());
    assert!(ps > pc, "the skin goes above the cluster: {hist:?}");
    assert_eq!(ok(&mut c, "skinCluster -q -ex sk;"), json!("skinPart"));
    let info = nums(&ok(&mut c, "skinCluster -q -dui sk;"));
    assert_eq!(info.len(), 3);
    assert_eq!(info[1], info[2]);
    assert_eq!(nums(&ok(&mut c, "skinCluster -q -gi sk;")), vec![0.0]);
    assert!(!strs(&ok(&mut c, "skinCluster -q -cmp sk;")).is_empty());
    ok(&mut c, "select m.vtx[0];");
    assert_eq!(strs(&ok(&mut c, "skinCluster -q -cms sk;")), vec!["m.vtx[0]"]);
    // The points an influence moves, selected.
    ok(&mut c, "skinCluster -e -siv j3 sk;");
    let sel = strs(&ok(&mut c, "ls -sl -fl;"));
    assert!(!sel.is_empty() && sel.iter().all(|x| x.starts_with("m.vtx[")), "{sel:?}");
    let top = sel.len();
    ok(&mut c, "skinCluster -e -siv j1 -ats sk;");
    assert!(strs(&ok(&mut c, "ls -sl -fl;")).len() > top);
    let both = strs(&ok(&mut c, "ls -sl -fl;")).len();
    ok(&mut c, "skinCluster -e -siv j1 -rfs sk;");
    assert!(strs(&ok(&mut c, "ls -sl -fl;")).len() < both);
    // nurbsSamples kept on the node.
    ok(&mut c, "skinCluster -e -ns 25 sk;");
    assert_eq!(num(&ok(&mut c, "skinCluster -q -ns sk;")), 25.0);
    // A second skin only with -multi.
    let e = bad(&mut c, "skinCluster j1 m;");
    assert!(e.contains("multi"), "{e}");
    ok(&mut c, "skinCluster -mul 1 -n sk2 -tsb j2 m;");
    assert!(strs(&ok(&mut c, "listHistory m;")).contains(&"sk2".to_string()));
    // -remove -geometry: unbound.
    ok(&mut c, "skinCluster -e -rm -g m sk2;");
    assert!(!strs(&ok(&mut c, "listHistory m;")).contains(&"sk2".to_string()));
    // The older closest distance switch.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n m; select -cl; joint -n j1; select -cl;");
    ok(&mut c, "skinCluster -ih 1 -n sk j1 m;");
    assert_eq!(ok(&mut c, "skinCluster -q -ih sk;"), json!(true));
    assert_eq!(num(&ok(&mut c, "skinCluster -q -bm sk;")), 0.0);
    // -ignoreSelected needs the objects named.
    ok(&mut c, "polyCube -n m2; select -cl; joint -n k1; select k1 m2;");
    bad(&mut c, "skinCluster -is;");
}

#[test]
fn skin_cluster_volume_bind_and_transforms() {
    let mut c = Core::new();
    ok(&mut c, "polyCylinder -n m -h 8 -sy 8 -r 0.5; select -cl; joint -n j1 -p 0 -4 0; joint -n j2 -p 0 0 0; joint -n j3 -p 0 4 0; select -cl;");
    ok(&mut c, "skinCluster -n sk -vb 0.5 -vt 2 j1 m;");
    // Every point is weighted, and weights add to one.
    for v in [0, 20, 40, 60] {
        let w = nums(&ok(&mut c, &format!("skinPercent -q -v sk m.vtx[{v}];")));
        close(w.iter().sum::<f64>(), 1.0, "normalized");
    }
    bad(&mut c, "polyCube -n q; select -cl; joint -n z1; select -cl; skinCluster -vb 0.5 -vt 4 z1 q;");
    // -toSkeletonAndTransforms: a group in the hierarchy is an influence too.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n m; select -cl; joint -n j1; select -cl; group -em -n ctl; parent ctl j1; select -cl;");
    ok(&mut c, "skinCluster -n sk -tst j1 m;");
    let infs = strs(&ok(&mut c, "skinCluster -q -inf sk;"));
    assert!(infs.iter().any(|x| x.ends_with("ctl")), "{infs:?}");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n m; select -cl; joint -n j1; select -cl; group -em -n ctl; parent ctl j1; select -cl;");
    ok(&mut c, "skinCluster -n sk j1 m;");
    let infs = strs(&ok(&mut c, "skinCluster -q -inf sk;"));
    assert!(!infs.iter().any(|x| x.ends_with("ctl")), "{infs:?}");
}

#[test]
fn snap_mode_settings() {
    let mut c = Core::new();
    ok(&mut c, "snapMode -dsi 2 -em 3 -emt 5 -t 12 -uvt 8 -ut 1 -lp 1 -lfc 1 -ps 1 -pc 1;");
    close(num(&ok(&mut c, "snapMode -q -distanceIncrement;")), 2.0, "distance");
    assert_eq!(ok(&mut c, "snapMode -q -em;"), json!(3));
    close(num(&ok(&mut c, "snapMode -q -emt;")), 5.0, "magnet tolerance");
    assert_eq!(ok(&mut c, "snapMode -q -tolerance;"), json!(12));
    assert_eq!(ok(&mut c, "snapMode -q -uvt;"), json!(8));
    for f in ["useTolerance", "livePoint", "liveFaceCenter", "pixelSnap", "pixelCenter"] {
        assert_eq!(ok(&mut c, &format!("snapMode -q -{f};")), json!(true), "{f}");
    }
    ok(&mut c, "snapMode -ps 0;");
    assert_eq!(ok(&mut c, "snapMode -q -ps;"), json!(false));
    // In inches: given in the current unit, kept in centimetres.
    ok(&mut c, "currentUnit -l inch; snapMode -dsi 1;");
    close(num(&ok(&mut c, "snapMode -q -dsi;")), 1.0, "in the current unit");
    ok(&mut c, "currentUnit -l cm;");
    close(num(&ok(&mut c, "snapMode -q -dsi;")), 2.54, "in centimetres");
}

#[test]
fn soft_mod_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p -w 10 -h 10 -sx 10 -sy 10;");
    let r = ok(&mut c, "softMod -fr 3 -fbx 0 -fm 1 -cp 0 -cv 1 -ci 1 -cp 1 -cv 0 -ci 1 p.vtx[60];");
    let (node, handle) = (strs(&r)[0].clone(), strs(&r)[1].clone());
    assert_eq!(ok(&mut c, &format!("softMod -q -fbx {node};")), json!(false));
    assert_eq!(ok(&mut c, &format!("softMod -q -fm {node};")), json!(true));
    // The ramp given: linear.
    close(num(&ok(&mut c, &format!("getAttr {node}.falloffCurve[0].falloffCurve_Interp;"))), 1.0, "interp");
    ok(&mut c, &format!("setAttr {handle}.ty 1;"));
    // Masking: only the point it was made on moves.
    close(point(&mut c, "p.vtx[60]")[1], 1.0, "the selected point");
    close(point(&mut c, "p.vtx[61]")[1], 0.0, "masked");
    ok(&mut c, &format!("softMod -e -fm 0 {node};"));
    assert!(point(&mut c, "p.vtx[61]")[1] > 0.1, "no mask");
    // The shared deformer queries.
    assert_eq!(strs(&ok(&mut c, &format!("softMod -q -g {node};"))), vec!["pShape"]);
    assert_eq!(nums(&ok(&mut c, &format!("softMod -q -gi {node};"))), vec![0.0]);
    let info = nums(&ok(&mut c, &format!("softMod -q -dui {node};")));
    assert_eq!(info.len(), 3);
    assert!(strs(&ok(&mut c, &format!("softMod -q -dt {node};"))).iter().any(|x| x.ends_with(&handle)));
    let wn = strs(&ok(&mut c, &format!("softMod -q -wn {node};")));
    assert_eq!(wn.len(), 2);
    // -weightedNode: another transform drives it.
    ok(&mut c, "spaceLocator -n drv;");
    ok(&mut c, &format!("softMod -e -wn drv drv {node};"));
    let before = point(&mut c, "p.vtx[60]")[1];
    ok(&mut c, "setAttr drv.ty 2;");
    close(point(&mut c, "p.vtx[60]")[1] - before, 2.0, "the weighted node drives it");
    assert_eq!(strs(&ok(&mut c, &format!("softMod -q -wn {node};"))), vec!["drv", "drv"]);
    // -remove -geometry and adding geometry.
    ok(&mut c, "polyCube -n q;");
    ok(&mut c, &format!("softMod -e -g q {node};"));
    assert_eq!(strs(&ok(&mut c, &format!("softMod -q -g {node};"))).len(), 2);
    ok(&mut c, &format!("softMod -e -rm -g q {node};"));
    assert_eq!(strs(&ok(&mut c, &format!("softMod -q -g {node};"))).len(), 1);
    ok(&mut c, &format!("softMod -e -rg {node};"));
    // Creation flags of every deformer: -exclusive, -afterReference, -useComponentTags.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b;");
    ok(&mut c, "softMod -n sm -ex part1 b;");
    assert_eq!(ok(&mut c, "softMod -q -ex sm;"), json!("part1"));
    ok(&mut c, "polyCube -n b2; softMod -n sm2 -uct b2;");
    assert!(strs(&ok(&mut c, "listConnections -type objectSet sm2;")).is_empty());
    ok(&mut c, "polyCube -n b3; cluster -n c3 b3; softMod -n sm3 -ar b3;");
    let hist = strs(&ok(&mut c, "listHistory b3;"));
    assert!(hist.contains(&"sm3".to_string()));
    bad(&mut c, "softMod -cp 0 -cv 1 b3;");
}

#[test]
fn soft_select_curves() {
    let mut c = Core::new();
    ok(&mut c, "softSelect -ssr;");
    ok(&mut c, "softSelect -sse 1 -ssd 2.0 -ssc \"0,1,2,1,0,2\" -ssf 2;");
    assert_eq!(ok(&mut c, "softSelect -q -ssc;"), json!("1,0,2,0,1,2"));
    assert_eq!(ok(&mut c, "softSelect -q -ssf;"), json!(2));
    ok(&mut c, "softSelect -ssc \"1,0,1,0.5,0.5,1,0,1,1\";");
    let keys = ok(&mut c, "softSelectCurve;");
    assert_eq!(keys["keys"].as_array().unwrap().len(), 3);
    bad(&mut c, "softSelect -ssc \"1,0\";");
    ok(&mut c, "softSelect -scc \"1,0,0,1,1,0,0,1,0,1\" -efc 0 -sud 0.25 -cu 1;");
    assert_eq!(ok(&mut c, "softSelect -q -scc;"), json!("1,0,0,1,1,0,0,1,0,1"));
    assert_eq!(ok(&mut c, "softSelect -q -efc;"), json!(0));
    close(num(&ok(&mut c, "softSelect -q -sud;")), 0.25, "uv distance");
    assert_eq!(ok(&mut c, "softSelect -q -cu;"), json!(1));
    bad(&mut c, "softSelect -cu 4;");
    // The false colours follow the ramp.
    let ramp = aztek_scene::soft_select::FalseColors::current();
    assert_eq!(ramp.at(1.0), [255, 0, 0, 255]);
    assert_eq!(ramp.at(0.0), [0, 0, 255, 255]);
    ok(&mut c, "softSelect -ssr;");
    close(num(&ok(&mut c, "softSelect -q -ssd;")), 5.0, "reset radius");
    assert_eq!(ok(&mut c, "softSelect -q -efc;"), json!(1));
    assert_eq!(ok(&mut c, "softSelect -q -ssc;"), json!("1,0,2,0,1,2"));
    let ramp = aztek_scene::soft_select::FalseColors::current();
    assert_eq!(ramp.at(1.0), [255, 255, 0, 255]);
    ok(&mut c, "softSelect -sse 0;");
}

#[test]
fn space_locator_query_and_edit() {
    let mut c = Core::new();
    let r = ok(&mut c, "spaceLocator -p 1 2 3;");
    let loc = strs(&r)[0].clone();
    assert_eq!(nums(&ok(&mut c, &format!("spaceLocator -q -p {loc};"))), vec![1.0, 2.0, 3.0]);
    ok(&mut c, &format!("spaceLocator -e -p 4 5 6 {loc};"));
    let shape = strs(&ok(&mut c, &format!("listRelatives -s {loc};")))[0].clone();
    assert_eq!(nums(&ok(&mut c, &format!("getAttr {shape}.localPosition;"))), vec![4.0, 5.0, 6.0]);
    ok(&mut c, &format!("spaceLocator -e -a -p 1 1 1 {loc};"));
    assert_eq!(nums(&ok(&mut c, &format!("getAttr {loc}.rotatePivot;"))), vec![1.0, 1.0, 1.0]);
    ok(&mut c, &format!("spaceLocator -e -n hello {loc};"));
    assert_eq!(nums(&ok(&mut c, "spaceLocator -q -p hello;")), vec![1.0, 1.0, 1.0]);
    // In inches.
    ok(&mut c, "currentUnit -l inch;");
    close(num(&ok(&mut c, "spaceLocator -q -p hello;").as_array().unwrap()[0].clone()), 1.0 / 2.54, "inches");
    ok(&mut c, "currentUnit -l cm;");
}

#[test]
fn substitute_geometry_options() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n old -h 2 -sy 2; select -cl; joint -n j1 -p 0 -1 0; joint -n j2 -p 0 1 0; select -cl; skinCluster -tsb j1 j2 old;");
    ok(&mut c, "polyCube -n fresh -h 2 -sy 4; move -r 0 0 0.0 fresh;");
    ok(&mut c, "polyCube -n far -h 2; move -r 10 0 0 far.vtx[0];");
    let r = ok(&mut c, "substituteGeometry -ogl -ngl -wdt 0.5 -dnd old fresh;");
    assert_eq!(strs(&r).len(), 1);
    // The old one is kept, in a layer of its own.
    assert!(ok(&mut c, "objExists old;") == json!(true));
    let layers = strs(&ok(&mut c, "ls -type displayLayer;"));
    assert!(layers.contains(&"oldGeometryLayer".to_string()) && layers.contains(&"newGeometryLayer".to_string()), "{layers:?}");
    assert_eq!(strs(&ok(&mut c, "editDisplayLayerMembers -q newGeometryLayer;")), vec!["fresh"]);
    // A point far from the old geometry takes its neighbours' weights.
    ok(&mut c, "select -cl; joint -n k1 -p 0 -1 0; joint -n k2 -p 0 1 0; select -cl; skinCluster -tsb k1 k2 far; polyCube -n far2 -h 2;");
    ok(&mut c, "move -r 0 0 30 far2.vtx[0];");
    ok(&mut c, "substituteGeometry -wdt 1 far far2;");
    let sk = strs(&ok(&mut c, "findRelatedSkinCluster far2;"))[0].clone();
    let w = nums(&ok(&mut c, &format!("skinPercent -q -v {sk} far2.vtx[0];")));
    close(w.iter().sum::<f64>(), 1.0, "filled from the neighbours");
}

#[test]
fn symmetric_modelling_seam_curve() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n b; select b;");
    ok(&mut c, "symmetricModelling -e -s 1 -ax x -a world -t 0.01;");
    assert_eq!(ok(&mut c, "symmetricModelling -q -a;"), json!("world"));
    close(num(&ok(&mut c, "symmetricModelling -q -t;")), 0.01, "tolerance");
    assert_eq!(ok(&mut c, "symmetricModelling -q -sf;"), json!("1,0,2,0,1,2"));
    ok(&mut c, "symmetricModelling -e -sf \"1,0,1,0.5,0.5,1,0,1,1\";");
    assert_eq!(ok(&mut c, "symmetricModelling -q -seamFalloffCurve;"), json!("1,0,1,0.5,0.5,1,0,1,1"));
    bad(&mut c, "symmetricModelling -e -sf \"1,0\";");
    ok(&mut c, "symmetricModelling -e -r;");
    assert_eq!(ok(&mut c, "symmetricModelling -q -sf;"), json!("1,0,2,0,1,2"));
    ok(&mut c, "symmetricModelling -e -s 0;");
}

#[test]
fn shelf_and_tab_layouts() {
    let mut c = Core::new();
    ok(&mut c, "window w; tabLayout -bs \"notop\" -stb 0 -i \"tab.png\" -iv 0 tabs;");
    ok(&mut c, "columnLayout one; setParent ..; columnLayout two; setParent ..;");
    assert_eq!(ok(&mut c, "tabLayout -q -bs tabs;"), json!("notop"));
    assert_eq!(ok(&mut c, "tabLayout -q -stb tabs;"), json!(false));
    assert_eq!(ok(&mut c, "tabLayout -q -iv tabs;"), json!(false));
    ok(&mut c, "tabLayout -e -tt one \"First\" -tti 2 \"Second\" -ti one \"a.png\" -tii 2 \"b.png\" tabs;");
    assert_eq!(strs(&ok(&mut c, "tabLayout -q -tt tabs;")), vec!["First", "Second"]);
    assert_eq!(strs(&ok(&mut c, "tabLayout -q -tii tabs;")), vec!["a.png", "b.png"]);
    ok(&mut c, "tabLayout -e -mar 4 -gsp 2 -mr4 1 2 3 4 tabs;");
    bad(&mut c, "tabLayout -e -bs \"round\" tabs;");
    ok(&mut c, "tabLayout -e -st two tabs;");
    ok(&mut c, "tabLayout -e -ct 1 tabs;");
    assert_eq!(strs(&ok(&mut c, "tabLayout -q -ca tabs;")), vec!["two"]);
    assert_eq!(ok(&mut c, "columnLayout -ex one;"), json!(false));
    bad(&mut c, "tabLayout -e -ct 5 tabs;");
    // Defaults.
    ok(&mut c, "setParent w; tabLayout plain;");
    assert_eq!(ok(&mut c, "tabLayout -q -bs plain;"), json!("full"));
    assert_eq!(ok(&mut c, "tabLayout -q -scrollableTabs plain;"), json!(true));
    // shelfTabLayout takes the same.
    ok(&mut c, "setParent w; shelfTabLayout -bs top -pmc \"print 1\" shelves;");
    assert_eq!(ok(&mut c, "shelfTabLayout -q -bs shelves;"), json!("top"));
    ok(&mut c, "shelfLayout s1; setParent ..; shelfTabLayout -e -tti 1 \"Shelf one\" shelves;");
    assert_eq!(strs(&ok(&mut c, "shelfTabLayout -q -tt shelves;")), vec!["Shelf one"]);
    // shelfLayout.
    ok(&mut c, "setParent w; shelfLayout -cw 40 -hr 0 sh;");
    assert_eq!(num(&ok(&mut c, "shelfLayout -q -cw sh;")), 40.0);
    assert_eq!(ok(&mut c, "shelfLayout -q -hr sh;"), json!(false));
    ok(&mut c, "shelfLayout -e -gsp 3 -mar 2 -mr4 1 1 1 1 -hr 1 sh;");
    assert_eq!(ok(&mut c, "shelfLayout -q -horizontal sh;"), json!(true));
    ok(&mut c, "setParent w; shelfLayout sh2;");
    assert_eq!(ok(&mut c, "shelfLayout -q -hr sh2;"), json!(true));
}

#[test]
fn shelf_button_and_symbol_check_box() {
    let mut c = Core::new();
    ok(&mut c, "global int $hit = 0; global string $dropped = \"\"; global string $renamed = \"\";");
    ok(&mut c, "global proc onDrop(string $n) { global string $dropped; $dropped = $n; }");
    ok(&mut c, "global proc onLabel(string $l) { global string $renamed; $renamed = $l; }");
    ok(&mut c, "window w; shelfLayout; shelfButton -l \"B\" -fx 1 -fy 0 -ecr 0 -ndp -mi \"One\" \"$hit = 1\" -mi \"Two\" \"$hit = 2\" -mio \"Three\" \"$hit = 3\" \"$hit = 30\" -hnd \"onDrop\" -lec \"onLabel\" sb;");
    assert_eq!(ok(&mut c, "shelfButton -q -fx sb;"), json!(true));
    assert_eq!(ok(&mut c, "shelfButton -q -fy sb;"), json!(false));
    assert_eq!(ok(&mut c, "shelfButton -q -ecr sb;"), json!(false));
    assert_eq!(ok(&mut c, "shelfButton -q -ndp sb;"), json!(true));
    // The popup's entries fire their commands by number.
    let ev = |path: &str, fire: &str, args: Value| json!({"path": path, "fire": [fire], "args": args});
    let full = strs(&ok(&mut c, "shelfButton -q -fpn sb;"))[0].clone();
    c.ui_event(&ev(&full, "menuItemCommand2", json!([])));
    assert_eq!(num(&ok(&mut c, "global int $hit; $hit = $hit;")), 2.0);
    c.ui_event(&ev(&full, "menuItemOptionCommand3", json!([])));
    assert_eq!(num(&ok(&mut c, "global int $hit; $hit = $hit;")), 30.0);
    // A node dropped on it, and its label renamed.
    c.ui_event(&ev(&full, "handleNodeDropCallback", json!(["pCube1"])));
    assert_eq!(ok(&mut c, "global string $dropped; $dropped = $dropped;"), json!("pCube1"));
    c.ui_event(&json!({"path": full, "set": {"label": "New"}, "fire": ["labelEditingCallback"], "args": ["New"]}));
    assert_eq!(ok(&mut c, "global string $renamed; $renamed = $renamed;"), json!("New"));
    assert_eq!(ok(&mut c, "shelfButton -q -l sb;"), json!("New"));
    // Python entries.
    ok(&mut c, "shelfButton -e -mi \"Py\" \"print('x')\" -mip 0 sb;");
    // symbolCheckBox.
    ok(&mut c, "setParent w; columnLayout; symbolCheckBox -oni \"on.png\" -ofi \"off.png\" -dni \"don.png\" -dfi \"doff.png\" -im -ver \"2027\" scb;");
    assert_eq!(ok(&mut c, "symbolCheckBox -q -dni scb;"), json!("don.png"));
    assert_eq!(ok(&mut c, "symbolCheckBox -q -disableOffImage scb;"), json!("doff.png"));
    assert_eq!(ok(&mut c, "symbolCheckBox -q -im scb;"), json!(true));
    assert_eq!(ok(&mut c, "symbolCheckBox -q -ver scb;"), json!("2027"));
}

#[test]
fn text_drop_rect_callback() {
    let mut c = Core::new();
    ok(&mut c, "global proc int[] rectFor(string $c, int $x, int $y) { return {$x - 1, $y - 1, 10, 4}; }");
    ok(&mut c, "window w; columnLayout; text -l \"Drop here\" t1;");
    ok(&mut c, "text -e -drc \"rectFor\" t1;");
    let full = strs(&ok(&mut c, "text -q -fpn t1;"))[0].clone();
    let r = aztek_host::attr_ui::drop_rect_for_tests(&mut c, &json!({"path": full, "x": 5, "y": 7}).to_string());
    assert_eq!(r, json!([4, 6, 10, 4]));
    // No callback: nothing to draw.
    ok(&mut c, "text t2;");
    let full2 = strs(&ok(&mut c, "text -q -fpn t2;"))[0].clone();
    assert_eq!(aztek_host::attr_ui::drop_rect_for_tests(&mut c, &json!({"path": full2, "x": 5, "y": 7}).to_string()), json!([]));
}
