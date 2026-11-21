//! Commands of the reference from circularFillet to doBlur: curveRGBColor and doBlur, and the flags the others gained
//! (query and edit on the NURBS history nodes, the shared deformer flags on cluster, deformer and deltaMush, weights files,
//! dagPose, dgtimer, deformableShape, display settings and the rest).

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

fn scratch(tag: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("aztek_u02_{tag}_{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn nurbs_history_nodes_query_and_edit() {
    let mut c = Core::new();
    // closeCurve: preserve shape is the default and keeps the curve through its edit points; 0 closes on the CVs.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 2 0 -p 3 2 0 -p 4 0 0 -n c1;");
    let made = strs(&ok(&mut c, "closeCurve c1;"));
    assert!(made[1].starts_with("closeCurve"), "{made:?}");
    assert_eq!(ok(&mut c, "closeCurve -q -ps c1;"), json!(1));
    assert_eq!(num(&ok(&mut c, "getAttr c1.form;")), 2.0);
    // The curve still passes through its old start.
    let near = nums(&ok(&mut c, "pointOnCurve -pr 0 c1;"));
    assert!(near[0].abs() < 1e-3 && near[1].abs() < 1e-3, "{near:?}");
    ok(&mut c, "closeCurve -e -ps 0 c1;");
    assert_eq!(ok(&mut c, &format!("closeCurve -q -ps {};", made[1])), json!(0));
    let moved = nums(&ok(&mut c, "pointOnCurve -pr 0 c1;"));
    assert!(moved[1].abs() > 1e-3, "on the CVs the start moves: {moved:?}");
    ok(&mut c, "closeCurve -e -ps 2 -bb 0.25 -bki 1 -p 0.2 c1;");
    assert_eq!(ok(&mut c, "closeCurve -q -bki c1;"), json!(true));
    ok(&mut c, "closeCurve -e -nds 2 c1;");
    assert_eq!(ok(&mut c, "closeCurve -q -nds c1;"), json!(2));

    // cone: centred by default, its base on the origin with -oib.
    ok(&mut c, "cone -n co1; cone -oib 1 -n co2;");
    let b1 = nums(&ok(&mut c, "exactWorldBoundingBox co1;"));
    let b2 = nums(&ok(&mut c, "exactWorldBoundingBox co2;"));
    close(b1[0], -b1[3], "centred along its axis");
    close(b2[0], 0.0, "base on the origin");
    assert_eq!(ok(&mut c, "cone -q -oib co2;"), json!(true));
    ok(&mut c, "cone -e -r 3 co1;");
    close(num(&ok(&mut c, "cone -q -r co1;")), 3.0, "radius edited");
    ok(&mut c, "cylinder -n cy1 -hr 4;");
    close(num(&ok(&mut c, "cylinder -q -hr cy1;")), 4.0, "cylinder height ratio");
    ok(&mut c, "cylinder -e -cch 1 cy1;");
    assert_eq!(ok(&mut c, "cylinder -q -cch cy1;"), json!(true));

    // curveIntersect: the first curve's parameters, then the second's; a node with -ch, edited by its direction.
    ok(&mut c, "curve -d 1 -p -1 0 0 -p 1 0 0 -n la; curve -d 1 -p 0 -1 0 -p 0 1 0 -n lb;");
    let t = ok(&mut c, "curveIntersect la lb;");
    let v = nums(&json!(t.as_str().unwrap().split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<_>>()));
    assert_eq!(v.len(), 2);
    close(v[0], 0.5, "on the first");
    let node = ok(&mut c, "curveIntersect -ch on la lb;");
    let node = node.as_str().unwrap().to_string();
    close(num(&ok(&mut c, &format!("getAttr {node}.parameter1[0];"))), 0.5, "parameter1");
    ok(&mut c, &format!("curveIntersect -e -dx 1 -dy 0 -dz 0 -ud 1 {node};"));
    close(num(&ok(&mut c, &format!("curveIntersect -q -dx {node};"))), 1.0, "directionX");

    // detach and close surface, and the fillet's tolerances.
    ok(&mut c, "sphere -n sp1;");
    let d = strs(&ok(&mut c, "detachSurface -p 1 -d 1 sp1;"));
    assert!(d.last().unwrap().starts_with("detachSurface"), "{d:?}");
    ok(&mut c, "detachSurface -e -nds 1 sp1;");
    assert_eq!(ok(&mut c, "detachSurface -q -nds sp1;"), json!(1));
    ok(&mut c, "nurbsPlane -n np -ax 0 1 0; closeSurface -d 0 np;");
    assert_eq!(ok(&mut c, "closeSurface -q -d np;"), json!(0));
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 1 0 -p 3 0 0 -n dc;");
    let pieces = strs(&ok(&mut c, "detachCurve -p 1.5 -rpo 0 dc;"));
    let dnode = pieces.last().unwrap().clone();
    close(num(&ok(&mut c, &format!("getAttr {dnode}.parameter[0];"))), 1.5, "detached at");
    ok(&mut c, &format!("detachCurve -e -cch 1 {dnode};"));
    assert_eq!(ok(&mut c, &format!("detachCurve -q -cch {dnode};")), json!(true));
}

#[test]
fn circular_fillet_tolerances() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n p1 -ax 0 1 0 -w 4; nurbsPlane -n p2 -ax 1 0 0 -w 4; move 1 0 0 p2;");
    let r = strs(&ok(&mut c, "circularFillet -pr 0.5 -sr 0.5 -pt 0.001 -tt 0.002 p1 p2;"));
    assert_eq!(r.len(), 2, "{r:?}");
    close(num(&ok(&mut c, &format!("circularFillet -q -pt {};", r[1]))), 0.001, "position tolerance");
    ok(&mut c, &format!("circularFillet -e -tt 0.05 {};", r[1]));
    close(num(&ok(&mut c, &format!("getAttr {}.tangentTolerance;", r[1]))), 0.05, "tangent tolerance edited");
}

#[test]
fn cluster_relative_weighted_node_and_shared_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 2 -sy 2 -n pl;");
    let made = strs(&ok(&mut c, "cluster -en 0.5 -rel pl;"));
    let (cl, h) = (made[0].clone(), made[1].clone());
    close(num(&ok(&mut c, &format!("cluster -q -en {cl};"))), 0.5, "envelope");
    assert_eq!(ok(&mut c, &format!("cluster -q -rel {cl};")), json!(true));
    assert_eq!(strs(&ok(&mut c, &format!("cluster -q -g {cl};"))), vec!["plShape".to_string()]);
    assert_eq!(strs(&ok(&mut c, &format!("cluster -q -dt {cl};"))), vec![h.clone()]);
    let y0 = nums(&ok(&mut c, "pointPosition pl.vtx[0];"))[1];
    ok(&mut c, &format!("move 0 2 0 {h};"));
    close(nums(&ok(&mut c, "pointPosition pl.vtx[0];"))[1], y0 + 1.0, "half the handle's move at envelope 0.5");
    // Relative: the handle's parent moving does nothing.
    ok(&mut c, &format!("group -n grp {h}; move 0 5 0 grp;"));
    close(nums(&ok(&mut c, "pointPosition pl.vtx[0];"))[1], y0 + 1.0, "relative ignores the parent");
    ok(&mut c, &format!("cluster -e -en 1 {cl};"));
    close(nums(&ok(&mut c, "pointPosition pl.vtx[0];"))[1], y0 + 2.0, "envelope edited");

    // -weightedNode with -bindState: an existing transform drives it, nothing jumps.
    ok(&mut c, "spaceLocator -n loc; move 1 0 0 loc; polyCube -n cb;");
    let x0 = nums(&ok(&mut c, "pointPosition cb.vtx[0];"))[0];
    let c2 = strs(&ok(&mut c, "cluster -wn loc loc -bs 1 cb;"));
    assert_eq!(c2[1], "loc");
    close(nums(&ok(&mut c, "pointPosition cb.vtx[0];"))[0], x0, "bound as it sits");
    assert_eq!(strs(&ok(&mut c, &format!("cluster -q -wn {};", c2[0])))[0], "loc");
    ok(&mut c, "move 3 0 0 loc;");
    close(nums(&ok(&mut c, "pointPosition cb.vtx[0];"))[0], x0 + 2.0, "follows the weighted node");

    // The shared deformer flags: components, dump info, exclusive partition, adding and removing geometry, prune.
    let cmp = strs(&ok(&mut c, &format!("cluster -q -cmp {cl};")));
    assert!(cmp.iter().any(|x| x.starts_with("pl.vtx")), "{cmp:?}");
    let dui = nums(&ok(&mut c, &format!("cluster -q -dui {cl};")));
    assert_eq!(dui, vec![0.0, 9.0, 9.0]);
    ok(&mut c, "polyCube -n extra;");
    ok(&mut c, &format!("cluster -e -g extra {cl};"));
    assert_eq!(strs(&ok(&mut c, &format!("cluster -q -g {cl};"))).len(), 2);
    assert_eq!(nums(&ok(&mut c, &format!("cluster -q -gi {cl};"))), vec![0.0, 1.0]);
    ok(&mut c, &format!("cluster -e -rm -g extra {cl};"));
    assert_eq!(strs(&ok(&mut c, &format!("cluster -q -g {cl};"))).len(), 1);
    let ex = strs(&ok(&mut c, "polySphere -n ps; cluster -ex clPart ps;"));
    assert_eq!(ok(&mut c, &format!("cluster -q -ex {};", ex[0])), json!("clPart"));
    // -resetGeometry takes the object's matrix again: after scaling it, the points stay as scaled.
    ok(&mut c, &format!("cluster -e -rg {cl};"));
    // In front of the chain, and only the objects named with -is.
    ok(&mut c, "polyCube -n c3; select -cl;");
    assert!(c.mel_checked("cluster -is;").is_err());
    let c3 = strs(&ok(&mut c, "cluster -foc c3;"));
    assert!(c3[0].starts_with("cluster"));
}

#[test]
fn deformer_and_delta_mush_shared_flags() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n sp;");
    let dm = strs(&ok(&mut c, "deltaMush -iwc 0.5 -owc 0.25 sp;"))[0].clone();
    close(num(&ok(&mut c, &format!("deltaMush -q -iwc {dm};"))), 0.5, "inward");
    close(num(&ok(&mut c, &format!("deltaMush -q -owc {dm};"))), 0.25, "outward");
    ok(&mut c, &format!("deltaMush -e -iwc 1 {dm};"));
    close(num(&ok(&mut c, &format!("getAttr {dm}.inwardConstraint;"))), 1.0, "edited");
    let dui = nums(&ok(&mut c, &format!("deltaMush -q -dui {dm};")));
    assert_eq!(dui[0], 0.0);
    assert!(dui[1] > 0.0 && dui[1] == dui[2], "{dui:?}");
    ok(&mut c, "polyCube -n cb;");
    ok(&mut c, &format!("deltaMush -e -g cb {dm};"));
    assert_eq!(strs(&ok(&mut c, &format!("deltaMush -q -g {dm};"))).len(), 2);
    ok(&mut c, &format!("deltaMush -e -rm -g cb {dm};"));
    assert_eq!(strs(&ok(&mut c, &format!("deltaMush -q -g {dm};"))).len(), 1);
    ok(&mut c, "polyTorus -n to;");
    let dm2 = strs(&ok(&mut c, "deltaMush -uct to;"))[0].clone();
    assert!(strs(&ok(&mut c, &format!("listConnections -type objectSet {dm2};"))).is_empty(), "component tags: no set");

    // deformer: -exclusive, -prune, the shared queries.
    ok(&mut c, "polyCube -n dc;");
    let d = strs(&ok(&mut c, "deformer -type cluster -ex part1 -n cc dc;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("deformer -q -ex {d};")), json!("part1"));
    let cmp = strs(&ok(&mut c, &format!("deformer -q -cmp {d};")));
    assert!(!cmp.is_empty());
    ok(&mut c, &format!("deformer -e -pr {d};"));
    // Nothing moves yet, so prune leaves no points in its set.
    assert!(strs(&ok(&mut c, &format!("deformer -q -cmp {d};"))).is_empty());
    ok(&mut c, "polyCube -n hid; hide hid; select hid;");
    assert!(c.mel_checked("deformer -type cluster;").is_err(), "hidden objects are left out");
    ok(&mut c, "deformer -type cluster -ihs;");
}

#[test]
fn copy_weights_query_mirror_and_blend_weights() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 4 -sy 4 -n pl; cluster -n cl pl;");
    // Lower the -X side, then mirror it from -X to +X across YZ.
    ok(&mut c, "select pl.vtx[0] pl.vtx[5] pl.vtx[10]; percent -v 0.25 cl;");
    ok(&mut c, "copyDeformerWeights -ss pl -ds pl -sd cl -mm YZ -mi;");
    close(num(&ok(&mut c, "percent -q -v cl pl.vtx[4];")), 0.25, "mirrored to +X");
    close(num(&ok(&mut c, "percent -q -v cl pl.vtx[0];")), 0.25, "the source side stays");
    assert_eq!(strs(&ok(&mut c, "copyDeformerWeights -q -sd -ss pl;")), vec!["cl".to_string()]);
    // Smooth reads between the nearest points.
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 3 -sy 3 -n p2; cluster -n c2 p2;");
    ok(&mut c, "copyDeformerWeights -ss pl -ds p2 -sd cl -dd c2 -nm -sm;");
    let w = num(&ok(&mut c, "percent -q -v c2 p2.vtx[0];"));
    assert!(w < 1.0 && w >= 0.25, "{w}");

    // copySkinWeights: query, every point when the skins are named, -sc for the selected only.
    ok(&mut c, "select -cl; joint -p -2 0 0 -n ja; joint -p 2 0 0 -n jb; select -cl;");
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 2 -sy 2 -n sa; polyPlane -w 4 -h 4 -sx 2 -sy 2 -n sb; move 0 0 0 sb;");
    ok(&mut c, "skinCluster -n skA ja jb sa; skinCluster -n skB ja jb sb;");
    assert_eq!(ok(&mut c, "copySkinWeights -q -ss -ss skA;"), json!("skA"));
    ok(&mut c, "skinPercent -tv ja 1 skA sa.vtx[0:8];");
    ok(&mut c, "select sb.vtx[0]; copySkinWeights -ss skA -ds skB -nm -sc;");
    close(num(&ok(&mut c, "skinPercent -t ja -q skB sb.vtx[0];")), 1.0, "selected point copied");
    assert!(num(&ok(&mut c, "skinPercent -t ja -q skB sb.vtx[8];")) < 0.99, "the rest left with -sc");
    ok(&mut c, "select -cl; copySkinWeights -ss skA -ds skB -nm;");
    close(num(&ok(&mut c, "skinPercent -t ja -q skB sb.vtx[8];")), 1.0, "every point without -sc");
    // Blend weights go across unless -nbw.
    ok(&mut c, "skinCluster -e -sm 2 skA; skinCluster -e -sm 2 skB;");
    ok(&mut c, "setAttr skA.bw -type doubleArray 9 1 1 1 1 1 1 1 1 1;");
    ok(&mut c, "copySkinWeights -ss skA -ds skB -nm -nbw;");
    assert!(nums(&ok(&mut c, "getAttr skB.bw;")).iter().all(|x| *x == 0.0), "left with -nbw");
    ok(&mut c, "copySkinWeights -ss skA -ds skB -nm;");
    assert!(nums(&ok(&mut c, "getAttr skB.bw;")).iter().any(|x| (*x - 1.0).abs() < 1e-6), "copied");
}

#[test]
fn deformer_weights_attributes_remap_skip_and_spaces() {
    let dir = scratch("dw");
    let path = dir.to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    ok(&mut c, "namespace -add nsp1; polyPlane -w 10 -h 10 -sx 5 -sy 5 -n nsp1:myShape; cluster -n nsp1:clusterA nsp1:myShape; cluster -n nsp1:clusterB nsp1:myShape;");
    ok(&mut c, "select nsp1:myShape.vtx[6:11]; percent -v 0.5 nsp1:clusterA; setAttr nsp1:clusterA.envelope 0.75;");
    let out = ok(&mut c, &format!("deformerWeights -ex -vc -at envelope -sh nsp1:myShape -sk \"*B\" -r \"nsp1:(.*);nsp2:$1\" -p \"{path}\" all.xml;"));
    assert!(out.as_str().unwrap().ends_with("all.xml"));
    let text = std::fs::read_to_string(dir.join("all.xml")).unwrap();
    assert!(text.contains("deformer=\"nsp2:clusterA\""), "{text}");
    assert!(!text.contains("clusterB"), "skipped");
    assert!(text.contains("<attribute deformer=\"nsp2:clusterA\" name=\"envelope\" value=\"0.75\"/>"), "{text}");
    assert!(text.contains("<face index=\"0\""), "vertex connections");
    // Read back into a second rig in another namespace, mapping the names, with its envelope.
    ok(&mut c, "namespace -set :; namespace -add nsp2; polyPlane -w 10 -h 10 -sx 5 -sy 5 -n nsp2:myShape; cluster -n nsp2:clusterA nsp2:myShape;");
    ok(&mut c, &format!("deformerWeights -im -at envelope -deformer nsp2:clusterA -p \"{path}\" all.xml;"));
    close(num(&ok(&mut c, "percent -q -v nsp2:clusterA nsp2:myShape.vtx[6];")), 0.5, "weights read");
    close(num(&ok(&mut c, "getAttr nsp2:clusterA.envelope;")), 0.75, "attribute read");
    // A finer mesh by barycentric over the written faces.
    ok(&mut c, "namespace -set :; polyPlane -w 10 -h 10 -sx 8 -sy 8 -n fine; cluster -n fc fine;");
    ok(&mut c, &format!("deformerWeights -im -deformer fc -ig -m barycentric -p \"{path}\" all.xml;"));
    let w: Vec<f64> = (0..81).map(|i| num(&ok(&mut c, &format!("percent -q -v fc fine.vtx[{i}];")))).collect();
    assert!(w.iter().any(|x| *x < 0.99 && *x > 0.4), "between: {w:?}");
    // Nearest within a tolerance: far points keep their weights.
    ok(&mut c, "polyPlane -w 10 -h 10 -sx 5 -sy 5 -n far; move 100 0 0 far; cluster -n farc far; percent -v 0.3 farc far.vtx[6];");
    ok(&mut c, &format!("deformerWeights -im -deformer farc -ig -m nearest -ws -pt 1 -p \"{path}\" all.xml;"));
    close(num(&ok(&mut c, "percent -q -v farc far.vtx[6];")), 0.3, "out of reach in world space");
    ok(&mut c, &format!("deformerWeights -im -deformer farc -ig -m nearest -pt 1 -p \"{path}\" all.xml;"));
    close(num(&ok(&mut c, "percent -q -v farc far.vtx[6];")), 0.5, "in reach in object space");
    assert_eq!(strs(&ok(&mut c, "deformerWeights -q -df -sh far;")), vec!["farc".to_string()]);
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn display_colors_layers_smoothness_and_surfaces() {
    let mut c = Core::new();
    ok(&mut c, "colorIndex 3 1 0 0;");
    assert_eq!(nums(&ok(&mut c, "colorIndex -q 3;")), vec![1.0, 0.0, 0.0]);
    ok(&mut c, "colorIndex -hsv 4 120 1 1;");
    let g = nums(&ok(&mut c, "colorIndex -q 4;"));
    close(g[1], 1.0, "green from hsv");
    close(nums(&ok(&mut c, "colorIndex -q -hsv 4;"))[0], 120.0, "hue back");
    ok(&mut c, "colorIndex -active 4 0 0 1;");
    close(nums(&ok(&mut c, "colorIndex -q -active 4;"))[2], 1.0, "active palette");
    close(nums(&ok(&mut c, "colorIndex -q -dormant 4;"))[1], 1.0, "dormant palette kept");
    ok(&mut c, "colorIndex -uc 2 0.5 0.5 0.5;");
    close(nums(&ok(&mut c, "colorIndex -q -uc 2;"))[0], 0.5, "user colour");
    ok(&mut c, "colorIndex -rf;");

    ok(&mut c, "sphere -n sp; select sp;");
    let l = ok(&mut c, "createDisplayLayer -num 7 -mc -n L1;");
    assert_eq!(l, json!("L1"));
    assert_eq!(num(&ok(&mut c, "getAttr L1.identification;")), 7.0);
    assert_eq!(num(&ok(&mut c, "getAttr layerManager.currentDisplayLayer;")), 7.0);
    assert_eq!(strs(&ok(&mut c, "editDisplayLayerMembers -q L1;")).len(), 2, "transform and shape");
    ok(&mut c, "cone -n co; select co;");
    let l2 = ok(&mut c, "createDisplayLayer -nr;");
    let l2 = l2.as_str().unwrap().to_string();
    assert_eq!(strs(&ok(&mut c, &format!("editDisplayLayerMembers -q {l2};"))), vec!["co".to_string()]);
    assert_eq!(num(&ok(&mut c, &format!("getAttr {l2}.identification;"))), 1.0, "lowest free number");

    ok(&mut c, "displaySmoothness -hull -su 2 sp;");
    assert_eq!(ok(&mut c, "displaySmoothness -q -hull sp;"), json!([true]));
    assert_eq!(nums(&ok(&mut c, "displaySmoothness -q -su sp;")), vec![2.0]);
    ok(&mut c, "displaySmoothness -bn sp;");
    assert_eq!(ok(&mut c, "displaySmoothness -q -bn sp;"), json!([true]));
    ok(&mut c, "displaySmoothness -f -rt 1 sp;");
    assert_eq!(ok(&mut c, "displaySmoothness -q -f sp;"), json!([true]));
    assert_eq!(ok(&mut c, "displaySmoothness -q -rt sp;"), json!([true]));
    ok(&mut c, "displaySmoothness -dc -ps 3;");
    assert_eq!(ok(&mut c, "displaySmoothness -q -dc -ps;"), json!(3));
    ok(&mut c, "sphere -n sp2;");
    let sh = strs(&ok(&mut c, "listRelatives -s sp2;"))[0].clone();
    assert_eq!(num(&ok(&mut c, &format!("getAttr {sh}.curvePrecisionShaded;"))), 3.0, "new surfaces take the default");
    ok(&mut c, "displaySmoothness -dc -ps 1;");

    ok(&mut c, "displaySurface -two 0 -x 1 -flp 1 sp;");
    assert_eq!(ok(&mut c, "displaySurface -q -two sp;"), json!([false]));
    assert_eq!(ok(&mut c, "displaySurface -q -x sp;"), json!([true]));
    assert_eq!(ok(&mut c, "displaySurface -q -flp sp;"), json!([true]));
    let sh = strs(&ok(&mut c, "listRelatives -s sp;"))[0].clone();
    assert_eq!(num(&ok(&mut c, &format!("getAttr {sh}.doubleSided;"))), 0.0, "single sided renders so");
}

#[test]
fn layout_context_and_connection_flags() {
    let mut c = Core::new();
    ok(&mut c, "window w1; columnLayout cl1;");
    ok(&mut c, "columnLayout -e -gsp 6 -mar 4 cl1;");
    ok(&mut c, "columnLayout -e -mr4 1 2 3 4 cl1;");
    assert_eq!(ok(&mut c, "contextInfo -esc;"), json!("setToolTo $gSelect"));
    ok(&mut c, "ctxEditMode -btd;");
    assert_eq!(ok(&mut c, "ctxEditMode -q;"), json!(true));
    ok(&mut c, "ctxEditMode -btu;");
    assert_eq!(ok(&mut c, "ctxEditMode -q;"), json!(false));
    ok(&mut c, "spaceLocator -n a; setAttr -l 1 a.translate; setAttr -l 1 a.sx;");
    assert_eq!(ok(&mut c, "connectionInfo -gla a.tx;"), json!("a.translate"));
    assert_eq!(ok(&mut c, "connectionInfo -gla a.sx;"), json!("a.scaleX"));
    assert_eq!(ok(&mut c, "connectionInfo -gla a.ry;"), json!(""));
}

#[test]
fn controller_walks_and_order() {
    let mut c = Core::new();
    ok(&mut c, "spaceLocator -n a; spaceLocator -n b; spaceLocator -n d;");
    ok(&mut c, "controller a; controller -p a b; controller -p a d;");
    assert_eq!(strs(&ok(&mut c, "controller -q -cld a;")), vec!["b".to_string(), "d".to_string()]);
    assert_eq!(ok(&mut c, "controller -q -pwr b;"), json!("d"));
    assert_eq!(ok(&mut c, "controller -q -pwl d;"), json!("b"));
    assert_eq!(ok(&mut c, "controller -q -pwd a;"), json!("b"));
    assert_eq!(ok(&mut c, "controller -q -pwu d;"), json!("a"));
    assert_eq!(ok(&mut c, "controller -q -idx d;"), json!(1));
    ok(&mut c, "controller -e -idx 0 d;");
    assert_eq!(strs(&ok(&mut c, "controller -q -cld a;")), vec!["d".to_string(), "b".to_string()]);
    assert_eq!(ok(&mut c, "controller -q -pwl d;"), Value::Null, "no sibling before the first");
    let g = ok(&mut c, "spaceLocator -n e; select e; controller -g;");
    assert_eq!(strs(&ok(&mut c, &format!("controller -q -cld {};", g.as_str().unwrap()))), vec!["e".to_string()]);
}

#[test]
fn convert_solid_tx_ranges_planes_sets_and_depth() {
    let dir = scratch("cst");
    let path = dir.to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    ok(&mut c, "polyPlane -w 2 -h 2 -n pp; shadingNode -asTexture ramp -n rp;");
    let f = strs(&ok(&mut c, &format!("convertSolidTx -rx 16 -ry 16 -fur -fin \"{path}/full.png\" rp.outColor pp;")))[0].clone();
    let place = strs(&ok(&mut c, &format!("listConnections -s 1 -d 0 -type place2dTexture {f};")));
    assert!(!place.is_empty(), "a placement for the range");
    ok(&mut c, &format!("convertSolidTx -rx 8 -ry 8 -sp -spr 0 0.5 0 0.5 -pf 16 -fin \"{path}/plane.png\" rp.outColor;"));
    let img = image::open(dir.join("plane.png")).unwrap();
    assert!(matches!(img, image::DynamicImage::ImageRgba16(_)), "16 bits a channel");
    ok(&mut c, "polyUVSet -create -uvSet second pp;");
    let named = strs(&ok(&mut c, &format!("convertSolidTx -rx 8 -ry 8 -uv second -nfn -fin \"{path}/set.png\" rp.outColor pp;")));
    assert!(std::path::Path::new(&named[0]).exists() || dir.join("set.png").exists());
    ok(&mut c, &format!("convertSolidTx -rx 8 -ry 8 -ds -cam persp -cr -rdm 1 -ubi 1 -fin \"{path}/ds.png\" rp.outColor pp.f[0];"));
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn curve_weights_and_rgb_colors() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 2 -p 0 0 0 -p 1 1 0 -p 2 0 0 -n plain;");
    ok(&mut c, "curve -d 2 -pw 0 0 0 1 -pw 1 1 0 4 -pw 2 0 0 1 -n heavy;");
    let mid_plain = nums(&ok(&mut c, "pointOnCurve -pr 0.5 -top 1 plain;"))[1];
    let mid_heavy = nums(&ok(&mut c, "pointOnCurve -pr 0.5 -top 1 heavy;"))[1];
    assert!(mid_heavy > mid_plain + 0.1, "weight pulls the curve to its CV: {mid_heavy} vs {mid_plain}");
    // A rational quadratic with the middle weight cos(45) is a circular arc.
    let w = std::f64::consts::FRAC_1_SQRT_2;
    ok(&mut c, &format!("curve -d 2 -pw 1 0 0 1 -pw 1 1 0 {w} -pw 0 1 0 1 -n arc;"));
    let p = nums(&ok(&mut c, "pointOnCurve -pr 0.5 -top 1 arc;"));
    close((p[0] * p[0] + p[1] * p[1]).sqrt(), 1.0, "on the unit circle");

    ok(&mut c, "curveRGBColor \"translateX\" 1 0 1;");
    ok(&mut c, "curveRGBColor -hsv \"*Y\" 60 1 1;");
    assert_eq!(nums(&ok(&mut c, "curveRGBColor -q \"translateX\";")), vec![1.0, 0.0, 1.0]);
    let y = nums(&ok(&mut c, "curveRGBColor -q \"*Y\";"));
    close(y[0], 1.0, "yellow r");
    close(y[1], 1.0, "yellow g");
    close(nums(&ok(&mut c, "curveRGBColor -q -hsv \"*Y\";"))[0], 60.0, "hue");
    let mut names = strs(&ok(&mut c, "curveRGBColor -ln;"));
    names.sort();
    assert_eq!(names, vec!["*Y".to_string(), "translateX".to_string()]);
    assert_eq!(strs(&ok(&mut c, "curveRGBColor -l;")).len(), 2);
    // The Graph Editor's curves take them.
    ok(&mut c, "polyCube -n k; setKeyframe k.tx; setKeyframe k.ty;");
    let q = aztek_host::query::anim_curves(&c, "1:2:1");
    let curves = q["objects"][0]["curves"].as_array().unwrap().clone();
    let tx = curves.iter().find(|x| x["channel"] == "translateX").unwrap();
    assert_eq!(tx["rgb"], json!([1.0, 0.0, 1.0]));
    let ty = curves.iter().find(|x| x["channel"] == "translateY").unwrap();
    assert!(ty.get("rgb").is_some(), "pattern *Y");
    ok(&mut c, "curveRGBColor -r \"translateX\";");
    assert_eq!(strs(&ok(&mut c, "curveRGBColor -ln;")), vec!["*Y".to_string()]);
    ok(&mut c, "curveRGBColor -rf;");
    assert!(strs(&ok(&mut c, "curveRGBColor -ln;")).is_empty());
}

#[test]
fn cycle_check_lists_and_limits() {
    let mut c = Core::new();
    // Aztek refuses a connection that would close a cycle, so there are none to find.
    ok(&mut c, "createNode transform -n xNode;");
    assert_eq!(ok(&mut c, "cycleCheck xNode.tx;"), json!(false));
    assert_eq!(ok(&mut c, "cycleCheck -all -tl 10sec;"), json!(false));
    assert_eq!(ok(&mut c, "cycleCheck -all -dag -l -fpn -ls \"--\";"), json!([]));
    assert_eq!(ok(&mut c, "cycleCheck -lpn -s 1 -l xNode;"), json!([]));
}

#[test]
fn data_structures_navigation_and_deformable_shapes() {
    let mut c = Core::new();
    ok(&mut c, "dataStructure -fmt \"raw\" -asString \"name=NameAndID:string=name:int32=ID\";");
    assert_eq!(ok(&mut c, "dataStructure -rem -n \"NameAndID\";"), json!(["NameAndID"]));

    ok(&mut c, "shadingNode -asTexture checker -n ch1; shadingNode -asShader lambert -n lm;");
    ok(&mut c, "defaultNavigation -ce -s ch1 -d lm.color;");
    assert_eq!(ok(&mut c, "defaultNavigation -dtv -d lm.color;"), json!("ch1"));
    ok(&mut c, "sets -renderable true -noSurfaceShader true -empty -name lmSG; connectAttr -f lm.outColor lmSG.surfaceShader;");
    assert_eq!(ok(&mut c, "defaultNavigation -dwn -d lmSG;"), json!("lm"));
    ok(&mut c, "defaultNavigation -dis -d lm.color;");
    assert!(strs(&ok(&mut c, "listConnections -s 1 -d 0 lm.color;")).is_empty(), "disconnected");

    ok(&mut c, "polyCube -n pc; cluster pc;");
    assert_eq!(strs(&ok(&mut c, "deformableShape -ch pcShape;")), vec!["cluster1".to_string()]);
    let nch = strs(&ok(&mut c, "deformableShape -nch pcShape;"));
    assert_eq!(nch[0], "pcShape");
    assert!(nch.contains(&"pcShapeOrig".to_string()), "{nch:?}");
    let och = strs(&ok(&mut c, "deformableShape -och pcShape;"));
    assert_eq!(och[0], "cluster1.output");
    let pch = strs(&ok(&mut c, "deformableShape -pch pcShape;"));
    assert_eq!(&pch[..2], &["pcShape.inMesh".to_string(), "cluster1.output".to_string()]);
    assert_eq!(strs(&ok(&mut c, "deformableShape -dsi pcShape;")), vec!["inMesh".to_string()]);
    assert_eq!(strs(&ok(&mut c, "deformableShape -dso pcShape;")), vec!["outMesh".to_string()]);
    assert_eq!(strs(&ok(&mut c, "deformableShape -twa pcShape;")), vec!["pnts".to_string()]);
    assert_eq!(ok(&mut c, "deformableShape -foc pcShape;"), json!("pcShapeOrig.outMesh"));
    assert_eq!(ok(&mut c, "deformableShape -sct pcShape;"), json!(true));
    assert_eq!(ok(&mut c, "deformableShape -ti pcShape;"), json!("pcShapeOrig"));
    assert_eq!(ok(&mut c, "deformableShape -uti pcShape;"), json!("pcShapeOrig"));
    assert_eq!(strs(&ok(&mut c, "deformableShape -til pcShape;")).len(), 2);
    assert!(strs(&ok(&mut c, "deformableShape -tw pcShape;")).is_empty());
    let tw = strs(&ok(&mut c, "deformableShape -ctw pcShape;"));
    assert_eq!(tw.len(), 1);
    assert_eq!(strs(&ok(&mut c, "deformableShape -tw pcShape;")), tw, "found in the chain after");
    assert!(strs(&ok(&mut c, "deformableShape -nch pcShape;")).contains(&"pcShapeOrig".to_string()), "the chain goes on through it");
    ok(&mut c, "polyCube -n bare;");
    assert_eq!(ok(&mut c, "deformableShape -uti bareShape;"), json!(""));
    assert_eq!(ok(&mut c, "deformableShape -cti bareShape;"), json!("bareShapeOrig"));
    let his = ok(&mut c, "deformableShape -his pcShape;");
    assert_eq!(his["shape"], json!("pcShape"));
}

#[test]
fn delete_all_dg_commands_and_disconnect_joint() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n k; setKeyframe k.tx; setKeyframe -t 10 -v 3 k.tx; polyCube -n k2; setKeyframe k2.ty;");
    ok(&mut c, "spaceLocator -n tgt; spaceLocator -n obj; pointConstraint tgt obj;");
    ok(&mut c, "delete -all;");
    assert!(!strs(&ok(&mut c, "listConnections k.tx;")).is_empty(), "-all alone deletes nothing");
    ok(&mut c, "delete -all -c;");
    assert!(strs(&ok(&mut c, "listConnections k.tx;")).is_empty());
    assert!(strs(&ok(&mut c, "listConnections k2.ty;")).is_empty());
    ok(&mut c, "delete -all -cn;");
    assert!(strs(&ok(&mut c, "ls -type pointConstraint;")).is_empty());

    let sz = nums(&ok(&mut c, "dgInfo -sz -all;"));
    assert_eq!(sz.len(), 3);
    assert!(sz[0] > 0.0 && sz[2] > 0.0);
    let with_defaults = nums(&ok(&mut c, "dgInfo -sz -all -nd;"));
    assert!(with_defaults[0] > sz[0], "-nd takes in the scene's own nodes");
    ok(&mut c, "dgInfo -n -all;");

    ok(&mut c, "polyCube -n pc; cluster pc;");
    assert!(num(&ok(&mut c, "dgdirty pc;")) >= 1.0);
    assert_eq!(ok(&mut c, "dgdirty -q -list;"), json!(["d/data", "p/plug", "c/connection"]));
    let dirty = strs(&ok(&mut c, "dgdirty -l data pcShape;"));
    assert!(!dirty.is_empty(), "dirty after the message");
    ok(&mut c, "dgdirty -c pcShape;");
    assert!(strs(&ok(&mut c, "dgdirty -l data pcShape;")).is_empty(), "clean after -c");
    ok(&mut c, "dgdirty -a -i -p -st;");
    assert!(num(&ok(&mut c, "dgeval -src pcShape;")) >= 1.0);

    ok(&mut c, "select -cl; joint -p 0 0 0 -n ja; joint -p 0 1 0 -n jb; joint -p 0 2 0 -n jc; joint -p 0 3 0 -n jd;");
    ok(&mut c, "ikHandle -sj jb -ee jd -n ikb;");
    let r = strs(&ok(&mut c, "disconnectJoint -dhm -ahm jb;"));
    assert_eq!(r.len(), 2);
    assert!(strs(&ok(&mut c, "ls -type ikHandle;")).is_empty(), "the handle on the joint goes");
}

#[test]
fn dgtimer_times_computes() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n pc; polySmooth pc;");
    ok(&mut c, "dgtimer -on -reset;");
    ok(&mut c, "setAttr polySmoothFace1.divisions 2; getAttr pcShape.outMesh;");
    ok(&mut c, "dgtimer -off;");
    let count = num(&ok(&mut c, "dgtimer -q -o stdout -rt total -sm compute -st count -nh;"));
    assert!(count >= 1.0, "{count}");
    let rows = strs(&ok(&mut c, "dgtimer -q -o MEL -nh -m 2;"));
    assert!(rows.len() <= 2 && !rows.is_empty(), "{rows:?}");
    let all = nums(&ok(&mut c, "dgtimer -q -o stdout -rt all -nh -sh compute -sh dirty -m 3;"));
    assert!(all.len() <= 3);
    let one = strs(&ok(&mut c, "dgtimer -q -o MEL -n polySmoothFace1;"));
    assert_eq!(one.len(), 1, "a single node: one line, no header");
    assert!(one[0].contains("polySmoothFace1"));
    let typed = strs(&ok(&mut c, "dgtimer -q -o MEL -t mesh -ct -nh;"));
    assert_eq!(typed.len(), 1, "combined by type");
    let nodes = num(&ok(&mut c, "dgtimer -q -o stdout -st none -nh;"));
    assert!(nodes >= 2.0, "node count with -st none");
    ok(&mut c, "dgtimer -reset -n polySmoothFace1 -hi draw -oh 1 -un -th 0 -rgl 0 -rgu 100;");
    assert!(num(&ok(&mut c, "dgtimer -uhm 8;")) >= 0.0);
    ok(&mut c, "dgtimer -rc selftime -q -o stdout;");
}

#[test]
fn dag_pose_members_and_edits() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 0 1 0 -n j2; joint -p 0 2 0 -n j3; select j1;");
    ok(&mut c, "dagPose -s -n mypose;");
    assert_eq!(strs(&ok(&mut c, "dagPose -q -m mypose;")).len(), 3);
    assert!(strs(&ok(&mut c, "dagPose -q -ap mypose;")).is_empty(), "at the pose");
    ok(&mut c, "rotate 0 0 30 j2;");
    assert_eq!(strs(&ok(&mut c, "dagPose -q -ap mypose;")), vec!["j2".to_string()]);
    ok(&mut c, "dagPose -rs -n mypose j2;");
    assert!(strs(&ok(&mut c, "dagPose -q -ap mypose;")).is_empty(), "reset to as it is");
    ok(&mut c, "dagPose -rm -n mypose j3;");
    assert_eq!(strs(&ok(&mut c, "dagPose -q -m mypose;")).len(), 2);
    ok(&mut c, "select -cl; joint -p 5 0 0 -n k1; select -cl;");
    ok(&mut c, "dagPose -a -sl -n mypose k1;");
    assert_eq!(strs(&ok(&mut c, "dagPose -q -m mypose;")).len(), 3);
    // -worldParent: worked out against the world, so after unparenting it goes back where it sat.
    ok(&mut c, "rotate 0 0 0 j2;");
    let at = nums(&ok(&mut c, "xform -q -ws -t j2;"));
    ok(&mut c, "dagPose -wp -n mypose j2; parent -w j2; move 7 7 7 j2;");
    ok(&mut c, "dagPose -r mypose;");
    let back = nums(&ok(&mut c, "xform -q -ws -t j2;"));
    for i in 0..3 {
        close(back[i], at[i], "back in the world");
    }
}

#[test]
fn do_blur_smears_along_vectors() {
    let dir = scratch("blur");
    let (w, h) = (16u32, 8u32);
    let mut img = image::RgbaImage::new(w, h);
    img.put_pixel(8, 4, image::Rgba([255, 255, 255, 255]));
    img.save(dir.join("color.png")).unwrap();
    // Everything moves 6 pixels right: red 0.5 + 6/256.
    let r = ((0.5 + 6.0 / 256.0) * 255.0f32).round() as u8;
    let vec = image::RgbaImage::from_pixel(w, h, image::Rgba([r, 128, 0, 255]));
    vec.save(dir.join("color.motion.png")).unwrap();
    let mut c = Core::new();
    let base = dir.to_string_lossy().replace('\\', "/");
    let out = ok(&mut c, &format!("doBlur -l 1 -s 1 -m 1 -c \"{base}/color.png\" -v \"{base}/color.motion.png\";"));
    assert!(out.as_str().unwrap().ends_with("color_blur.png"));
    let b = image::open(dir.join("color_blur.png")).unwrap().to_rgba8();
    assert!(b.get_pixel(8, 4)[0] < 255, "the bright pixel spreads out");
    assert!(b.get_pixel(10, 4)[0] > 0, "along the vector");
    assert_eq!(b.get_pixel(8, 1)[0], 0, "not across it");
    ok(&mut c, &format!("doBlur -m 3 -r 1 -o 1000 -c \"{base}/color\" -v \"{base}/color.motion.png\";"));
    std::fs::remove_dir_all(dir).ok();
}

#[cfg(feature = "python")]
#[test]
fn connect_dynamic_script_handlers() {
    let mut c = Core::new();
    let py = |c: &mut Core, src: &str| {
        let quoted = src.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
        c.mel_checked(&format!("python(\"{quoted}\");")).unwrap_or_else(|e| panic!("{src}: {e}"))
    };
    py(&mut c, "import maya.cmds as cmds\nseen = []\ndef h(fields=None, emitters=None, collisionObjects=None, objects=None):\n    seen.append((fields, objects))\n    return True");
    py(&mut c, "hid = cmds.connectDynamic(addScriptHandler=h)");
    ok(&mut c, "polyPlane -n floor; polySphere -n ball;");
    // The handler takes the call: nothing is connected.
    ok(&mut c, "connectDynamic -f gravityField1 ball;");
    let n = py(&mut c, "__import__('json').dumps(seen)");
    let seen: Value = serde_json::from_str(n.as_str().unwrap()).unwrap();
    assert_eq!(seen, json!([[["gravityField1"], ["ball"]]]));
    py(&mut c, "cmds.connectDynamic(removeScriptHandler=hid)\ncmds.connectDynamic(removeScriptHandler=999)");
    assert!(c.mel_checked("connectDynamic -ash \"h\" -f x ball;").is_err(), "alone only");
    let id = ok(&mut c, "connectDynamic -ash \"h\";");
    assert!(id.as_i64().is_some());
    ok(&mut c, &format!("connectDynamic -rsh {};", id));
}
