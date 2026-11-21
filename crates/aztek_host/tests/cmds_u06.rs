//! Commands of the reference from polyBevel to polyExtrudeEdge: query and edit of each poly command's history node, the
//! new polyCanBridgeEdge, polyCacheMonitor, polyCircularizeEdge, polyCircularizeFace and polyCollapseTweaks, and the
//! flags the others gained (polyBevel's settings, blind data editing and false colour rules, polyColorMod's ramps,
//! polyCrease on vertices and as history, polyCut's parts, the cylindrical projection's settings, polyEditUV's turn,
//! scale and UV set, polyEvaluate's areas).

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

#[test]
fn history_nodes_query_and_edit() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c1;");
    // Query reads the node, by its name or the object; edit changes only what it names.
    assert_eq!(num(&ok(&mut c, "polyCube -q -w polyCube1;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyCube -q -sx c1;")), 1.0);
    ok(&mut c, "polyCube -e -w 3 polyCube1;");
    assert_eq!(num(&ok(&mut c, "getAttr polyCube1.width;")), 3.0);
    assert_eq!(num(&ok(&mut c, "getAttr polyCube1.height;")), 1.0, "an edit leaves the other settings");
    close_all(&nums(&ok(&mut c, "exactWorldBoundingBox c1;")), &[-1.5, -0.5, -0.5, 1.5, 0.5, 0.5], "edited cube");
    ok(&mut c, "polyCylinder -n cy -sx 12;");
    assert_eq!(num(&ok(&mut c, "polyCylinder -q -sx cy;")), 12.0);
    ok(&mut c, "polyCylinder -e -sx 8 cy; polyCylinder -n other -sx 8;");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f cy;")), num(&ok(&mut c, "polyEvaluate -f other;")), "rebuilt with 8 sides");
    ok(&mut c, "polyCone -n co;");
    ok(&mut c, "polyCone -e -sa 5 polyCone1;");
    assert_eq!(num(&ok(&mut c, "polyCone -q -sa co;")), 5.0);
    // The modelling edits.
    ok(&mut c, "polyCube -n c2; polyExtrudeEdge -ltz 0.5 c2.e[0];");
    close(num(&ok(&mut c, "polyExtrudeEdge -q -ltz polyExtrudeEdge1;")), 0.5, "extrude edge");
    ok(&mut c, "polyExtrudeEdge -e -ltz 2 polyExtrudeEdge1;");
    close(num(&ok(&mut c, "getAttr polyExtrudeEdge1.localTranslateZ;")), 2.0, "edited extrude");
    ok(&mut c, "polyCube -n c3; polyChipOff -kft 0 c3.f[2];");
    assert_eq!(num(&ok(&mut c, "polyChipOff -q -kft polyChipOff1;")), 0.0);
    ok(&mut c, "polyCube -n c4; polyDelVertex c4.vtx[0];");
    assert_eq!(num(&ok(&mut c, "polyDelVertex -q -nds polyDelVertex1;")), 0.0);
    ok(&mut c, "polyDelVertex -e -nds 1 polyDelVertex1;");
    assert_eq!(num(&ok(&mut c, "getAttr polyDelVertex1.nodeState;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyDelVertex -q -nodeState polyDelVertex1;")), 1.0);
    ok(&mut c, "polyCube -n c5; polyDelEdge -cv 1 c5.e[4];");
    assert_eq!(num(&ok(&mut c, "polyDelEdge -q -cv polyDelEdge1;")), 1.0);
    ok(&mut c, "polyCube -n c6; polyDelFacet c6.f[0]; polyCloseBorder c6;");
    assert_eq!(strs(&ok(&mut c, "polyDelFacet -q -nds c6;")).len(), 0, "a number comes back");
    ok(&mut c, "polyCloseBorder -e -nds 0 polyCloseBorder1;");
    ok(&mut c, "polyCube -n c7; polyCollapseEdge c7.e[0]; polyCollapseFacet c7.f[1];");
    assert_eq!(num(&ok(&mut c, "polyCollapseEdge -q -nds polyCollapseEdge1;")), 0.0);
    assert_eq!(num(&ok(&mut c, "polyCollapseFacet -q -nds c7;")), 0.0);
    ok(&mut c, "polyCube -n c8; polyDuplicateEdge -of 0.25 c8.e[4];");
    close(num(&ok(&mut c, "polyDuplicateEdge -q -of polyDuplicateEdge1;")), 0.25, "duplicate edge");
    ok(&mut c, "polyCube -n c9; polyEditEdgeFlow -adjustEdgeFlow 0.5 c9.e[0];");
    close(num(&ok(&mut c, "polyEditEdgeFlow -q -adjustEdgeFlow polyEditEdgeFlow1;")), 0.5, "edge flow");
    ok(&mut c, "polyCube -n b1; polyCube -n b2; move 0.5 0 0 b2;");
    let made = strs(&ok(&mut c, "polyCBoolOp -op 1 b1 b2;"));
    assert_eq!(num(&ok(&mut c, &format!("polyCBoolOp -q -op {};", made[1]))), 1.0);
    ok(&mut c, &format!("polyCBoolOp -e -op 2 {};", made[1]));
    assert_eq!(num(&ok(&mut c, &format!("polyBoolOp -q -op {};", made[1]))), 2.0);
    ok(&mut c, "polyCube -n k1; polyColorPerVertex -rgb 1 0 0 -cdo k1; polyColorMod -huev 120 k1;");
    close(num(&ok(&mut c, "polyColorMod -q -huev polyColorMod1;")), 120.0, "hue");
    ok(&mut c, "polyCube -n cl; polyClean cl;");
    assert_eq!(num(&ok(&mut c, "polyClean -q -ce polyClean1;")), 1.0);
    ok(&mut c, "polyCreateFacet -p 0 0 0 -p 1 0 0 -p 0 1 0;");
    assert_eq!(num(&ok(&mut c, "polyCreateFacet -q -nds polyCreateFace1;")), 0.0);
}

#[test]
fn bevel_settings() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c;");
    let made = strs(&ok(&mut c, "polyBevel -o 0.1 -sg 2 -af 0 -r 0.3 -mv 1 -mvt 0.001 -sn 1 -ua 1 -n myBevel c.e[0];"));
    assert_eq!(made, vec!["myBevel"]);
    close(num(&ok(&mut c, "polyBevel -q -o myBevel;")), 0.1, "offset");
    assert_eq!(num(&ok(&mut c, "polyBevel -q -sg myBevel;")), 2.0);
    assert_eq!(num(&ok(&mut c, "polyBevel -q -af myBevel;")), 0.0);
    close(num(&ok(&mut c, "polyBevel -q -r myBevel;")), 0.3, "roundness");
    assert_eq!(num(&ok(&mut c, "polyBevel -q -mv myBevel;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyBevel -q -sn myBevel;")), 1.0, "subdivideNgons is the node's fillNgons");
    assert_eq!(num(&ok(&mut c, "polyBevel -q -ua myBevel;")), 1.0);
    let before = num(&ok(&mut c, "polyEvaluate -f c;"));
    ok(&mut c, "polyBevel -e -sg 4 myBevel;");
    assert_eq!(num(&ok(&mut c, "polyBevel -q -sg c;")), 4.0);
    assert!(num(&ok(&mut c, "polyEvaluate -f c;")) > before, "more segments, more faces");
    close(num(&ok(&mut c, "polyBevel -q -o myBevel;")), 0.1, "the edit leaves the offset");
    // Caching and the node's state.
    ok(&mut c, "polyBevel -e -cch 1 -nds 1 myBevel;");
    assert_eq!(num(&ok(&mut c, "polyBevel -q -cch myBevel;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyBevel -q -nds myBevel;")), 1.0);
    // polyBevel3 reads and edits the same node.
    ok(&mut c, "polyBevel3 -e -o 0.2 myBevel;");
    close(num(&ok(&mut c, "polyBevel3 -q -o myBevel;")), 0.2, "bevel3 offset");
    // No history: no node.
    ok(&mut c, "polyCube -n d; polyBevel -ch 0 -o 0.1 d.e[0];");
    assert!(strs(&ok(&mut c, "listHistory d;")).iter().all(|n| !n.starts_with("polyBevel")));
}

#[test]
fn cut_parts_and_direction() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c;");
    ok(&mut c, "polyCut -pcx 0.1 -ry 90 -ps 2 3 -eox 1 c;");
    close_all(&nums(&ok(&mut c, "polyCut -q -pc polyCut1;")), &[0.1, 0.0, 0.0], "centre");
    close(num(&ok(&mut c, "polyCut -q -pcx polyCut1;")), 0.1, "centre x");
    close_all(&nums(&ok(&mut c, "polyCut -q -ro polyCut1;")), &[0.0, 90.0, 0.0], "rotate in degrees");
    close(num(&ok(&mut c, "polyCut -q -ry polyCut1;")), 90.0, "rotate y");
    close_all(&nums(&ok(&mut c, "polyCut -q -ps polyCut1;")), &[2.0, 3.0], "size");
    close_all(&nums(&ok(&mut c, "polyCut -q -eo polyCut1;")), &[1.0, 0.5, 0.5], "extract offset");
    ok(&mut c, "polyCut -e -ps 4 5 -pcy 0.2 polyCut1;");
    close_all(&nums(&ok(&mut c, "polyCut -q -ps polyCut1;")), &[4.0, 5.0], "edited size");
    close(num(&ok(&mut c, "getAttr polyCut1.cutPlaneCenterY;")), 0.2, "edited centre");
    // A direction cuts across that axis through the middle of the faces, deleting one side.
    ok(&mut c, "polyCube -n d; move 3 0 0 d; polyCut -cd \"x\" -df 1 d;");
    close_all(&nums(&ok(&mut c, "polyCut -q -pc polyCut2;")), &[3.0, 0.0, 0.0], "middle of the box");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f d;")), 5.0);
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox d;"));
    assert!((bb[3] - bb[0] - 0.5).abs() < 1e-3, "half the cube is left: {bb:?}");
}

#[test]
fn circularize_edge_and_face() {
    let mut c = Core::new();
    ok(&mut c, "polyCylinder -n cy -sx 8 -sy 1;");
    let made = strs(&ok(&mut c, "polyCircularizeEdge -ro 0.5 -n ce cy.e[0:7];"));
    assert_eq!(made, vec!["ce"]);
    assert_eq!(ok(&mut c, "nodeType ce;"), Value::String("polyCircularize".into()));
    close(num(&ok(&mut c, "polyCircularizeEdge -q -ro ce;")), 0.5, "radial offset");
    assert_eq!(num(&ok(&mut c, "polyCircularizeEdge -q -ed ce;")), 1.0);
    ok(&mut c, "polyCircularizeEdge -e -ro 0 ce;");
    close(num(&ok(&mut c, "getAttr ce.radialOffset;")), 0.0, "edited");
    // Faces: the border of the region; -cc adds a circle along it.
    ok(&mut c, "polyPlane -n p -sx 3 -sy 3;");
    let made = strs(&ok(&mut c, "polyCircularizeFace -cc p.f[4];"));
    assert_eq!(made.len(), 1);
    assert_eq!(num(&ok(&mut c, &format!("polyCircularizeFace -q -sa {};", made[0]))), 30.0);
    assert_eq!(strs(&ok(&mut c, "ls -type nurbsCurve;")).len(), 1, "one circle");
    // The middle face's four corners now sit on a circle about its centre.
    let r: Vec<f64> = (0..4).map(|i| {
        let ids = [5, 6, 9, 10];
        let p = nums(&ok(&mut c, &format!("pointPosition -w p.vtx[{}];", ids[i])));
        (p[0] * p[0] + p[2] * p[2]).sqrt()
    }).collect();
    for x in &r {
        close(*x, r[0], "on one circle");
    }
}

#[test]
fn can_bridge_edges() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p -sx 1 -sy 3; delete p.f[1];");
    // Two border edges across the gap bridge; an edge and its neighbour on the same run do not.
    assert_eq!(ok(&mut c, "polyCanBridgeEdge p.e[1] p.e[5];"), Value::Bool(true));
    assert_eq!(ok(&mut c, "polyCanBridgeEdge p.e[1] p.e[0];"), Value::Bool(false));
    ok(&mut c, "polyCube -n k;");
    assert_eq!(ok(&mut c, "polyCanBridgeEdge k.e[0] k.e[1];"), Value::Bool(false), "inside edges");
    // The selection is read and left as it was.
    ok(&mut c, "select -r p.e[1] p.e[5];");
    assert_eq!(ok(&mut c, "polyCanBridgeEdge;"), Value::Bool(true));
    assert_eq!(strs(&ok(&mut c, "ls -sl -fl;")).len(), 2);
}

#[test]
fn cache_monitor_counts() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p;");
    ok(&mut c, "polyCacheMonitor -nodeName \"polyPlane1\";");
    ok(&mut c, "polyCacheMonitor -nm polyPlane1;");
    assert_eq!(num(&ok(&mut c, "getAttr polyPlane1.cacheInput;")), 2.0);
    assert_eq!(num(&ok(&mut c, "getAttr polyPlane1.caching;")), 1.0, "caches while counted");
    ok(&mut c, "polyCacheMonitor -nm polyPlane1 -chv false;");
    ok(&mut c, "polyCacheMonitor -nm polyPlane1 -cacheValue false;");
    ok(&mut c, "polyCacheMonitor -nm polyPlane1 -cacheValue false;");
    assert_eq!(num(&ok(&mut c, "getAttr polyPlane1.cacheInput;")), 0.0, "never below zero");
    assert_eq!(num(&ok(&mut c, "getAttr polyPlane1.caching;")), 0.0, "released");
    assert!(c.mel_checked("polyCacheMonitor -nm nothingHere;").is_err());
}

#[test]
fn collapse_tweaks() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n pPlane1;");
    assert_eq!(num(&ok(&mut c, "polyCollapseTweaks -q -hvt pPlane1;")), 0.0);
    ok(&mut c, "move -r 10 20 30 pPlane1.vtx[8];");
    assert_eq!(num(&ok(&mut c, "polyCollapseTweaks -q -hasVertexTweaks pPlane1;")), 1.0);
    let before = nums(&ok(&mut c, "pointPosition -l pPlane1.vtx[8];"));
    ok(&mut c, "select -r pPlane1; polyCollapseTweaks;");
    assert_eq!(num(&ok(&mut c, "polyCollapseTweaks -q -hvt pPlane1;")), 0.0);
    close_all(&nums(&ok(&mut c, "pointPosition -l pPlane1.vtx[8];")), &before, "the point stays");
    // A new move is a tweak again.
    ok(&mut c, "move -r 0 1 0 pPlane1.vtx[0];");
    assert_eq!(num(&ok(&mut c, "polyCollapseTweaks -q -hvt pPlane1;")), 1.0);
    ok(&mut c, "undo;");
    assert_eq!(num(&ok(&mut c, "polyCollapseTweaks -q -hvt pPlane1;")), 0.0);
}

#[test]
fn crease_edges_vertices_and_history() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; delete -ch c;");
    assert_eq!(ok(&mut c, "polyCrease -v 2 -v 3 c.e[0:1];"), Value::Bool(true));
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v c.e[0:1];")), vec![2.0, 3.0], "one value per edge");
    assert!(strs(&ok(&mut c, "listHistory c;")).iter().all(|n| !n.starts_with("polyCrease")), "no history made");
    ok(&mut c, "polyCrease -vv 4 c.vtx[2];");
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -vv c.vtx[2];")), vec![4.0]);
    ok(&mut c, "polyCrease -rv 1 c.e[0] c.vtx[2];");
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v c.e[0];")), vec![3.0]);
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -vv c.vtx[2];")), vec![5.0]);
    ok(&mut c, "polyCrease -op 1 c.e[1];");
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v c.e[0:1];")), vec![3.0, 0.0]);
    // -ch makes a polyCreaseEdge node; a mesh with history always gets one.
    ok(&mut c, "polyCrease -ch 1 -v 5 c.e[3];");
    assert!(strs(&ok(&mut c, "listHistory c;")).contains(&"polyCreaseEdge1".to_string()));
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v c.e[3];")), vec![5.0]);
    let h = strs(&ok(&mut c, "polyCube -n h;"));
    ok(&mut c, "polyCrease -e -v 1.5 h.e[0];");
    assert!(strs(&ok(&mut c, "listHistory h;")).iter().any(|n| n.starts_with("polyCreaseEdge")));
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v h.e[0];")), vec![1.5]);
    // The node keeps its values through a change upstream.
    ok(&mut c, &format!("setAttr {}.width 2;", h[1]));
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v h.e[0];")), vec![1.5]);
    ok(&mut c, "polyCrease -op 2 c;");
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -v c.e[0:3];")), vec![0.0; 4]);
    assert_eq!(nums(&ok(&mut c, "polyCrease -q -vv c.vtx[2];")), vec![0.0]);
}

#[test]
fn blind_data_edit_delete_rescan() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c;");
    ok(&mut c, "polyBlindData -id 9001 -at \"face\" -ldn \"heat\" -dbd 2 c.f[1] c.f[2];");
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" c.f[1];")), vec![2.0]);
    // Edit changes the node already holding this type and name.
    let before = strs(&ok(&mut c, "listHistory c;")).len();
    let edited = strs(&ok(&mut c, "polyBlindData -e -id 9001 -at \"face\" -ldn \"heat\" -dbd 7 c.f[3];"));
    assert_eq!(edited.len(), 1);
    assert_eq!(strs(&ok(&mut c, "listHistory c;")).len(), before, "no new node");
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" c.f[3];")), vec![7.0]);
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" c.f[1];")), vec![7.0], "the node's value");
    // Delete takes the data off the components named.
    ok(&mut c, "polyBlindData -delete -id 9001 -at \"face\" -ldn \"heat\" c.f[1];");
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9001 -at \"face\" -ldn \"heat\" c.f[1];")), Vec::<f64>::new());
    // 64-bit whole numbers by -lid, and a rescan that finds nothing to refresh.
    ok(&mut c, "polyBlindData -id 9002 -at \"vertex\" -ldn \"big\" -lid 5 c.vtx[0];");
    assert_eq!(nums(&ok(&mut c, "polyQueryBlindData -id 9002 -at \"vertex\" -ldn \"big\" c.vtx[0];")), vec![5.0]);
    assert_eq!(strs(&ok(&mut c, "polyBlindData -res;")), Vec::<String>::new());
}

#[test]
fn color_blind_data_rules() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c;");
    ok(&mut c, "polyBlindData -id 1000 -at \"face\" -ldn \"fluffy\" -dbd 2 c.f[1] c.f[2];");
    ok(&mut c, "polyBlindData -id 1000 -at \"face\" -ldn \"fluffy\" -dbd 4 c.f[3];");
    // Query mode colours nothing and returns what matches.
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 0 -n \"fluffy\" c;")), vec!["c.f[1]", "c.f[2]", "c.f[3]"]);
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 1 -n \"fluffy\" -dt \"double\" -v \"2\" c;")), vec!["c.f[1]", "c.f[2]"]);
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 2 -n \"fluffy\" -mnv 3 -mxv 5 c;")), vec!["c.f[3]"]);
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 2 -n \"fluffy\" -umn 0 -mxv 3 c;")), vec!["c.f[1]", "c.f[2]"]);
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 6 -n \"fluffy\" -mnv 1 -mxv 3 c;")), vec!["c.f[1]", "c.f[2]"], "grayscale out of range is left out");
    assert_eq!(strs(&ok(&mut c, "polyColorBlindData -q -id 1000 -num 1 -m 5 -n \"fluffy\" -v \"4\" c;")), vec!["c.f[3]"]);
    // Two rules: 2 red, 4 green; no data black.
    let made = strs(&ok(&mut c, "polyColorBlindData -id 1000 -num 1 -m 1 -n \"fluffy\" -v \"2\" -cr 1 -cg 0 -cb 0 -id 1000 -num 1 -m 1 -n \"fluffy\" -v \"4\" -cr 0 -cg 1 -cb 0 -ncr 0 -ncg 0 -ncb 0 c;"));
    assert_eq!(made.len(), 1);
    assert_eq!(strs(&ok(&mut c, "polyColorSet -q -ccs c;")), vec!["blindData"]);
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtxFace[2][1];")), &[1.0, 0.0, 0.0], "face 1 red");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtxFace[6][3];")), &[0.0, 1.0, 0.0], "face 3 green");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtxFace[0][0];")), &[0.0, 0.0, 0.0], "face 0 has none");
    // Rules colouring a face differently: the clash colour.
    ok(&mut c, "polyColorBlindData -id 1000 -num 1 -m 0 -n \"fluffy\" -cr 1 -cg 1 -cb 1 -id 1000 -num 1 -m 1 -n \"fluffy\" -v \"4\" -cr 0 -cg 0 -cb 1 -ccr 1 -ccg 0 -ccb 1 c;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtxFace[6][3];")), &[1.0, 0.0, 1.0], "clash");
    // False colour off takes them away.
    ok(&mut c, "polyColorBlindData -efc 0;");
    assert!(!strs(&ok(&mut c, "polyColorSet -q -acs c;")).contains(&"blindData".to_string()));
}

#[test]
fn color_mod_ramps_and_color_del() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; polyColorPerVertex -rgb 1 0.5 0 -cdo c;");
    ok(&mut c, "polyColorMod -rfv 0.5 c;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtx[0];")), &[0.5, 0.5, 0.0], "red scaled");
    close(num(&ok(&mut c, "polyColorMod -q -rfv polyColorMod1;")), 0.5, "query");
    ok(&mut c, "polyColorMod -e -rfv 0.25 -gfv 2 polyColorMod1;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtx[0];")), &[0.25, 1.0, 0.0], "edited");
    ok(&mut c, "polyColorMod -e -nfv 0.5 polyColorMod1;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb c.vtx[0];")), &[0.125, 0.5, 0.0], "intensity");
    // -h and -s are the hue and saturation.
    ok(&mut c, "polyCube -n k; polyColorPerVertex -rgb 1 0 0 -cdo k; polyColorMod -h 120 k;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb k.vtx[0];")), &[0.0, 1.0, 0.0], "hue turned");
    ok(&mut c, "polyColorMod -e -s 0 polyColorMod2;");
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb k.vtx[0];")), &[1.0, 1.0, 1.0], "no saturation");
    // polyColorDel takes the colour off the faces named; a whole mesh loses the set.
    ok(&mut c, "polyCube -n d; polyColorPerVertex -rgb 0 0 1 -cdo d;");
    let set = strs(&ok(&mut c, "polyColorSet -q -ccs d;"))[0].clone();
    ok(&mut c, &format!("polyColorDel -cls {set} d.f[0];"));
    close_all(&nums(&ok(&mut c, "polyColorPerVertex -q -rgb d.vtxFace[4][1];")), &[0.0, 0.0, 1.0], "another face keeps it");
    assert_eq!(num(&ok(&mut c, "polyColorDel -q -nds polyColorDel1;")), 0.0);
    ok(&mut c, "polyColorDel d;");
    assert!(strs(&ok(&mut c, "polyColorSet -q -acs d;")).is_empty());
}

#[test]
fn cylindrical_projection_settings() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n sp;");
    let made = strs(&ok(&mut c, "polyCylindricalProjection -ra 90 -ic 0.25 0.5 -ph 5 -r 4 -n cp sp.f[*];"));
    assert_eq!(made, vec!["cp"]);
    close(num(&ok(&mut c, "polyCylindricalProjection -q -ra cp;")), 90.0, "rotation angle");
    close_all(&nums(&ok(&mut c, "polyCylindricalProjection -q -ic cp;")), &[0.25, 0.5], "image centre");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -ph sp;")), 5.0, "height");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -r sp;")), 4.0, "radius");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -phs cp;")), 360.0, "sweep in degrees");
    close_all(&nums(&ok(&mut c, "polyCylindricalProjection -q -ps cp;")), &[360.0, 5.0], "scale: sweep and height");
    close_all(&nums(&ok(&mut c, "polyCylindricalProjection -q -is cp;")), &[1.0, 1.0], "image scale");
    ok(&mut c, "polyCylindricalProjection -e -phs 180 -isu 2 cp;");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -phs cp;")), 180.0, "edited sweep");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -isu cp;")), 2.0, "edited U scale");
    close(num(&ok(&mut c, "polyCylindricalProjection -q -ra cp;")), 90.0, "the edit leaves the rest");
    assert_eq!(num(&ok(&mut c, "polyCylindricalProjection -q -nds cp;")), 0.0);
    // A turn of the UVs: a projection with -ra 90 is the plain one turned about the image centre.
    ok(&mut c, "polyCylinder -n a; polyCylinder -n b;");
    ok(&mut c, "polyCylindricalProjection -ra 0.001 a.f[*]; polyCylindricalProjection -ra 90 b.f[*];");
    let ua = nums(&ok(&mut c, "polyEditUV -q a.map[3];"));
    let ub = nums(&ok(&mut c, "polyEditUV -q b.map[3];"));
    let (du, dv) = (ua[0] - 0.5, ua[1] - 0.5);
    close_all(&ub, &[0.5 - dv, 0.5 + du], "turned a quarter");
}

#[test]
fn edit_uv_turn_scale_and_set() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n p -sx 1 -sy 1;");
    // Known places first: absolute values.
    for (i, u, v) in [(0, 0.0, 0.0), (1, 1.0, 0.0), (2, 0.0, 1.0), (3, 1.0, 1.0)] {
        ok(&mut c, &format!("polyEditUV -r 0 -u {u} -v {v} p.map[{i}];"));
    }
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[3];")), &[1.0, 1.0], "start");
    ok(&mut c, "polyEditUV -uValue 0.5 -vValue 0.25 p.map[3];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[3];")), &[1.5, 1.25], "moved");
    close_all(&nums(&ok(&mut c, "polyEditUV -q -u p.map[3];")), &[1.5], "only U");
    // The turn is the angle by the ratio; -rot 0 and -s 0 leave them out.
    ok(&mut c, "polyEditUV -pu 0.5 -pv 0.5 -a 90 -rr 0.5 p.map[0];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[0];")), &[0.5, 0.5 - 0.5f64.sqrt()], "turned 45");
    ok(&mut c, "polyEditUV -pu 0.5 -pv 0.5 -a 90 -rot 0 p.map[1];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[1];")), &[1.0, 0.0], "not turned");
    ok(&mut c, "polyEditUV -pu 0 -pv 0 -su 2 -sv 2 -s 0 p.map[1];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[1];")), &[1.0, 0.0], "not scaled");
    ok(&mut c, "polyEditUV -pu 0 -pv 0 -su 2 -sv 2 -s 1 p.map[1];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[1];")), &[2.0, 0.0], "scaled");
    // Another UV set, the current one left as it was.
    ok(&mut c, "polyUVSet -create -uvSet \"second\" p;");
    ok(&mut c, "polyEditUV -uvs \"second\" -r 0 -u 0.1 -v 0.2 p.map[2];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q p.map[2];")), &[0.0, 1.0], "map1 untouched");
    close_all(&nums(&ok(&mut c, "polyEditUV -uvs second -q p.map[2];")), &[0.1, 0.2], "second set");
    assert_eq!(strs(&ok(&mut c, "polyUVSet -q -cuv p;")), vec!["map1"]);
    assert!(c.mel_checked("polyEditUV -uvs nothing -u 1 p.map[0];").is_err());
    // polyEditUVShell: the turn by the ratio, the scale switch and the set.
    ok(&mut c, "polyPlane -n s -sx 1 -sy 1;");
    for (i, u, v) in [(0, 0.0, 0.0), (1, 1.0, 0.0), (2, 0.0, 1.0), (3, 1.0, 1.0)] {
        ok(&mut c, &format!("polyEditUV -r 0 -u {u} -v {v} s.map[{i}];"));
    }
    ok(&mut c, "polyEditUVShell -a 180 -rr 0.5 s.map[0];");
    let turned = nums(&ok(&mut c, "polyEditUV -q s.map[0];"));
    close_all(&turned, &[1.0, 0.0], "a quarter about the shell's middle");
    ok(&mut c, "polyEditUVShell -su 3 -sv 3 -s 0 s.map[0];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q s.map[0];")), &turned, "scale off");
    ok(&mut c, "polyUVSet -create -uvSet \"other\" s; polyEditUVShell -uvs other -u 1 s.map[0];");
    close_all(&nums(&ok(&mut c, "polyEditUV -q s.map[0];")), &turned, "current set untouched");
    close_all(&nums(&ok(&mut c, "polyEditUVShell -uvs other -q -u s.map[0];")), &[1.0], "the other set moved");
}

#[test]
fn evaluate_areas() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; scale 2 1 1 c;");
    assert_eq!(nums(&ok(&mut c, "polyEvaluate -fa c.f[0:1];")), vec![1.0, 1.0]);
    assert_eq!(nums(&ok(&mut c, "polyEvaluate -faceArea c.f[2];")), vec![1.0]);
    assert_eq!(nums(&ok(&mut c, "polyEvaluate -wfa c.f[0:1];")), vec![2.0, 2.0]);
    assert_eq!(nums(&ok(&mut c, "polyEvaluate -worldFaceArea c.f[4];")), vec![1.0], "the side face keeps its size");
    // The cube's default UVs: each face 1/16 of the square, the whole 6/16.
    close_all(&nums(&ok(&mut c, "polyEvaluate -ufa c.f[0:1];")), &[0.0625, 0.0625], "uv face area");
    close(num(&ok(&mut c, "polyEvaluate -uva c;")), 0.375, "uv area");
    close(num(&ok(&mut c, "polyEvaluate -uvArea c.f[0];")), 0.0625, "uv area of faces");
    // An edge on the seam: a pair for each side.
    assert_eq!(strs(&ok(&mut c, "polyEvaluate -uep c.e[1];")).len(), 2, "inside the shell: one pair");
    let seam = strs(&ok(&mut c, "polyEvaluate -uvEdgePairs c.e[0];"));
    assert_eq!(seam.len(), 4, "on the seam: a pair each side: {seam:?}");
    assert!(seam.iter().all(|x| x.starts_with("c.map[")));
    // UV shell ids of faces, by the reference's short name.
    ok(&mut c, "polyPlane -n p -sx 2 -sy 1; polyMapCut p.e[1];");
    let ids = nums(&ok(&mut c, "polyEvaluate -usi p.f[0:1];"));
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1], "the cut parts the faces' shells");
}

#[test]
fn edit_mode_leaves_defaults_alone() {
    // An edit sets only the flags it names, whatever defaults the command has.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n c; polyBevel3 -o 0.1 -sg 3 -f 0.3 c.e[0];");
    ok(&mut c, "polyBevel3 -e -o 0.2 polyBevel1;");
    assert_eq!(num(&ok(&mut c, "polyBevel3 -q -sg polyBevel1;")), 3.0);
    close(num(&ok(&mut c, "polyBevel3 -q -f polyBevel1;")), 0.3, "fraction kept");
    // Query asks only for what it names, not for defaults that are on.
    assert_eq!(num(&ok(&mut c, "polyBevel3 -q -af polyBevel1;")), 1.0);
    close(num(&ok(&mut c, "polyBevel3 -q -o polyBevel1;")), 0.2, "offset");
}

/// Every flag this unit's commands lacked is now one of theirs, by long or short name.
const UNIT_FLAGS: &[(&str, &str)] = &[
    ("polyBevel", "af angleTolerance at autoFit caching cch ch com constructionHistory e edit ma mergeVertexTolerance mergeVertices miteringAngle mv mvt n name nds nodeState q query r roundness sa sn subdivideNgons ua useLegacyBevelAlgorithm uvAssignment"),
    ("polyBevel3", "e edit q query"),
    ("polyBlendColor", "e edit q query"),
    ("polyBlindData", "delete e edit lid res rescan"),
    ("polyBoolOp", "e edit q query"),
    ("polyBooleanCmd", "e edit q query"),
    ("polyBridgeEdge", "e edit q query"),
    ("polyCBoolOp", "e edit q query"),
    ("polyCacheMonitor", "cacheValue chv nm nodeName"),
    ("polyCanBridgeEdge", ""),
    ("polyChipOff", "e edit q query"),
    ("polyCircularize", "e edit q query"),
    ("polyCircularizeEdge", "al alignment caching cc cch ch constructionHistory createCurve e ed edit evenlyDistribute inc inputCurve n name nds nodeState nor normalOrientation q query radialOffset ro sa smoothingAngle worldSpace ws"),
    ("polyCircularizeFace", "al alignment caching cc cch ch constructionHistory createCurve e ed edit evenlyDistribute inc inputCurve n name nds nodeState nor normalOrientation q query radialOffset ro sa smoothingAngle worldSpace ws"),
    ("polyClean", "e edit q query"),
    ("polyCloseBorder", "e edit q query"),
    ("polyCollapseEdge", "e edit q query"),
    ("polyCollapseFacet", "e edit q query"),
    ("polyCollapseTweaks", "hasVertexTweaks hvt q query"),
    ("polyColorBlindData", "aboveMaxColorBlue aboveMaxColorGreen aboveMaxColorRed amb amg amr attrName belowMinColorBlue belowMinColorGreen belowMinColorRed bmb bmg bmr ccb ccg ccr clashColorBlue clashColorGreen clashColorRed dataType dt efc enableFalseColor maxColorBlue maxColorGreen maxColorRed minColorBlue minColorGreen minColorRed mnb mng mnr mxb mxg mxr num numIdTypes q queryMode umn umx useMax useMin v value"),
    ("polyColorDel", "cls e edit q query"),
    ("polyColorMod", "afv ai alphaScale_FloatValue alphaScale_Interp alphaScale_Position ap bfv bi blueScale_FloatValue blueScale_Interp blueScale_Position bp e edit gfv gi gp greenScale_FloatValue greenScale_Interp greenScale_Position h intensityScale_FloatValue intensityScale_Interp intensityScale_Position nfv ni np q query redScale_FloatValue redScale_Interp redScale_Position rfv ri rp s"),
    ("polyCone", "e edit q query"),
    ("polyConnectComponents", "e edit q query"),
    ("polyContourProjection", "e edit q query"),
    ("polyCopyUV", "e edit q query"),
    ("polyCrease", "createHistory e edit"),
    ("polyCreateFacet", "e edit q query"),
    ("polyCube", "e edit q query"),
    ("polyCut", "cd cutPlaneCenterX cutPlaneCenterY cutPlaneCenterZ cutPlaneRotateX cutPlaneRotateY cutPlaneRotateZ cutPlaneSize cuttingDirection e edit eox eoy eoz extractOffsetX extractOffsetY extractOffsetZ pcx pcy pcz ps q query rx ry rz"),
    ("polyCylinder", "e edit q query"),
    ("polyCylindricalProjection", "caching cch e edit ic imageCenter imageScale n name nds nodeState pc perInstance ph phs pi projectionCenter projectionHeight projectionHorizontalSweep projectionScale ps q query r radius worldSpace ws"),
    ("polyDelEdge", "e edit q query"),
    ("polyDelFacet", "e edit q query"),
    ("polyDelVertex", "caching cch e edit nds nodeState q query"),
    ("polyDuplicateEdge", "e edit q query"),
    ("polyEditEdgeFlow", "e edit q query"),
    ("polyEditUV", "rot rotateRatio rotation rr s scale uValue uvSetName uvs vValue"),
    ("polyEditUVShell", "s scale uvSetName uvs"),
    ("polyEvaluate", "fa faceArea uep ufa usi uvArea uvEdgePairs uvFaceArea uva wfa worldFaceArea"),
    ("polyExtrudeEdge", "e edit q query"),
];

#[test]
fn unit_flags_exist() {
    let c = Core::new();
    let mut missing = Vec::new();
    for (cmd, flags) in UNIT_FLAGS {
        let Some(def) = c.s.commands.find(cmd) else {
            missing.push(format!("{cmd} (no command)"));
            continue;
        };
        for fl in flags.split_whitespace() {
            if !def.flags.iter().any(|d| d.name == fl || d.short_name == fl) {
                missing.push(format!("{cmd} -{fl}"));
            }
        }
    }
    assert!(missing.is_empty(), "missing: {missing:?}");
}
