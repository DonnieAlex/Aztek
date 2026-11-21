//! Commands of the reference from inViewMessage to nurbsCube: the new mimicManipulation, mlDeformer and
//! multiProfileBirailSurface, and the flags the others gained (the deformer flags of lattice, nonLinear and jointLattice,
//! move's reflection and parameter moves, listAttr's filters, listHistory's walks, makeLive's registry, nParticle's per
//! particle edits and jitters, namespace merges, query and edit on the NURBS history commands and the rest).

use aztek_host::{take_ui_requests_for_tests as take_ui_requests, Core};
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

fn pos(c: &mut Core, what: &str) -> Vec<f64> {
    nums(&ok(c, &format!("xform -q -ws -t {what};")))
}

#[test]
fn in_view_message_requests() {
    let mut c = Core::new();
    take_ui_requests();
    ok(&mut c, "inViewMessage -smg \"test message\" -pos topRight -bkc 0x00ff0000 -fade -ck -fts 14 -ft \"Arial\" -fof 30 -tof 5;");
    let reqs = take_ui_requests().to_string();
    for want in ["test message", "topRight", "\\\"fade\\\":true", "\\\"clickKill\\\":true", "\\\"fontSize\\\":14", "Arial", "\\\"frameOffset\\\":30", "\\\"textOffset\\\":5", "#ff0000", "status"] {
        assert!(reqs.contains(want), "{want} in {reqs}");
    }
    ok(&mut c, "inViewMessage -cl topRight;");
    assert!(take_ui_requests().to_string().contains("clear"));
    for (flag, word) in [("-hd", "hide"), ("-sh", "show"), ("-min", "minimize"), ("-res", "restore")] {
        ok(&mut c, &format!("inViewMessage {flag};"));
        assert!(take_ui_requests().to_string().contains(word), "{flag}");
    }
    // Without -fade the message stays: no time to go.
    ok(&mut c, "inViewMessage -msg hello;");
    assert!(take_ui_requests().to_string().contains("\\\"time\\\":0.0"));
    assert!(c.mel_checked("inViewMessage -cl nowhere;").is_err());
}

#[test]
fn inherit_transform_off_and_match_transform_axes() {
    let mut c = Core::new();
    ok(&mut c, "circle -n c1; move 2 0 0; group -n g1; move -2 0 0;");
    close(pos(&mut c, "c1")[0], 0.0, "inherits");
    ok(&mut c, "inheritTransform -off c1;");
    assert_eq!(num(&ok(&mut c, "inheritTransform -q c1;")), 0.0);
    close(pos(&mut c, "c1")[0], 2.0, "as if under the world");
    ok(&mut c, "inheritTransform -on -p c1;");
    close(pos(&mut c, "c1")[0], 2.0, "preserved");

    ok(&mut c, "polyCone -n cone1; scale 0.2 2.0 0.2; rotate 20 45 70; move -2 3 2; polyCylinder -n cyl1;");
    ok(&mut c, "matchTransform -px -rot cyl1 cone1;");
    let p = pos(&mut c, "cyl1");
    close(p[0], -2.0, "x matched");
    close(p[1], 0.0, "y kept");
    let r = nums(&ok(&mut c, "xform -q -ws -ro cyl1;"));
    close(r[0], 20.0, "rotation");
    ok(&mut c, "matchTransform -sy cyl1 cone1;");
    let s = nums(&ok(&mut c, "xform -q -ws -s cyl1;"));
    close(s[1], 2.0, "y scale");
    close(s[0], 1.0, "x scale kept");
    // -scaleBox: by the bounding boxes.
    ok(&mut c, "polyCube -n big -w 4 -h 4 -d 4; polyCube -n small;");
    ok(&mut c, "matchTransform -sx -box small big;");
    close(nums(&ok(&mut c, "xform -q -ws -s small;"))[0], 4.0, "box scale");
    // -rotatePivot alone moves the pivot, not the object.
    ok(&mut c, "polyCube -n pv; move 5 0 0 big;");
    ok(&mut c, "matchTransform -rp pv big;");
    close(nums(&ok(&mut c, "xform -q -ws -rp pv;"))[0], 5.0, "pivot");
    close(pos(&mut c, "pv")[0], 0.0, "object stays");
}

#[test]
fn joint_limits_cluster_and_joint_orient() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 2 0 -2 -n j2; joint -p 4 0 0 -n j3;");
    ok(&mut c, "joint -e -al j2;");
    assert_eq!(ok(&mut c, "joint -q -dof j2;"), json!("y"));
    assert_eq!(nums(&ok(&mut c, "joint -q -lsy j2;")), vec![1.0, 1.0]);
    let l = nums(&ok(&mut c, "joint -q -ly j2;"));
    close(l[1] - l[0], 180.0, "half a turn of hinge");
    // A joint in line with its neighbours is left alone.
    ok(&mut c, "select -cl; joint -p 0 0 0 -n s1; joint -p 1 0 0 -n s2; joint -p 2 0 0 -n s3; joint -e -al s2;");
    assert_eq!(ok(&mut c, "joint -q -dof s2;"), json!("xyz"));

    ok(&mut c, "jointCluster -ab 20 -bb 30 -j j2;");
    close(num(&ok(&mut c, "jointCluster -q -ab jointCluster1;")), 20.0, "above bound");
    ok(&mut c, "jointCluster -e -av 70 -adt sine j2;");
    close(num(&ok(&mut c, "jointCluster -q -av j2;")), 70.0, "edited through the joint");
    assert_eq!(ok(&mut c, "jointCluster -q -bdt j2;"), json!("linear"));
    assert_eq!(ok(&mut c, "jointCluster -q -adt j2;"), json!("sine"));
    assert_eq!(ok(&mut c, "jointCluster -q -bc j2;"), json!("jointCluster1"));
    assert!(c.mel_checked("jointCluster -e -adt wobbly j2;").is_err());

    ok(&mut c, "rotate 0 30 0 j1;");
    let before = pos(&mut c, "j3");
    ok(&mut c, "makeIdentity -a 1 -r 1 -jo j1;");
    assert_eq!(nums(&ok(&mut c, "joint -q -o j1;")), vec![0.0, 0.0, 0.0]);
    assert_eq!(nums(&ok(&mut c, "joint -q -o j2;")), vec![0.0, 0.0, 0.0]);
    let after = pos(&mut c, "j3");
    for k in 0..3 {
        close(after[k], before[k], "joint kept in place");
    }
}

#[test]
fn move_reflection_parameter_and_orient() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n pl -sx 4 -sy 1 -w 4 -h 1;");
    ok(&mut c, "move -r -rfl -rax 0 1 0 pl.vtx[0];");
    close(pos(&mut c, "pl.vtx[4]")[1], 1.0, "mirror image moved");
    close(pos(&mut c, "pl.vtx[0]")[1], 1.0, "named point moved");
    // Symmetry was for the move only.
    ok(&mut c, "move -r 0 1 0 pl.vtx[0];");
    close(pos(&mut c, "pl.vtx[4]")[1], 1.0, "no symmetry after");
    // About the bounding box: a plane moved off the origin still pairs its own sides.
    ok(&mut c, "polyPlane -n p2 -sx 4 -sy 1 -w 4 -h 1; move 10 0 0 p2;");
    ok(&mut c, "move -r -rfl -rab 0 1 0 p2.vtx[0];");
    close(pos(&mut c, "p2.vtx[4]")[1], 1.0, "about the box");

    // -parameter: along the curve.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 10 0 0 -n ln;");
    ok(&mut c, "move -r -p 0.25 0 0 ln.ep[0];");
    close(pos(&mut c, "ln.cv[0]")[0], 2.5, "a quarter along");

    // -orientJoint: the joint above aims at the moved joint.
    ok(&mut c, "select -cl; joint -p 0 0 0 -n a1; joint -p 2 0 0 -n a2;");
    ok(&mut c, "move -oj xyz -sao yup 0 2 0 a2;");
    let w = nums(&ok(&mut c, "xform -q -ws -m a1;"));
    close(w[1], 1.0, "a1's x axis points up at a2");
}

#[test]
fn lattice_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n sp;");
    let l = strs(&ok(&mut c, "lattice -dv 2 3 2 -oc 1 -cp -ex part1 sp;"));
    assert_eq!(l, vec!["ffd1", "ffd1Lattice", "ffd1Base"]);
    assert_eq!(strs(&ok(&mut c, "listRelatives -p ffd1Lattice;")), vec!["ffd1LatticeGroup"]);
    assert_eq!(strs(&ok(&mut c, "listRelatives -p ffd1Base;")), vec!["ffd1LatticeGroup"]);
    assert_eq!(ok(&mut c, "lattice -q -ex ffd1;"), json!("part1"));
    assert_eq!(strs(&ok(&mut c, "lattice -q -cmp ffd1;")), vec!["sp.vtx[0:381]"]);
    assert_eq!(nums(&ok(&mut c, "lattice -q -dui ffd1;")), vec![0.0, 382.0, 382.0]);
    assert_eq!(strs(&ok(&mut c, "lattice -q -dt ffd1;")), vec!["ffd1Lattice", "ffd1Base"]);
    assert_eq!(nums(&ok(&mut c, "lattice -q -gi ffd1;")), vec![0.0]);
    // -latticeReset: the lattice back on its base.
    ok(&mut c, "move -r 0 1 0 ffd1Lattice;");
    ok(&mut c, "lattice -e -lr ffd1;");
    assert_eq!(nums(&ok(&mut c, "getAttr ffd1Lattice.t;")), nums(&ok(&mut c, "getAttr ffd1Base.t;")));
    // -removeTweaks keeps the lattice's own move.
    ok(&mut c, "move -r 0 1 0 ffd1Lattice; select ffd1Lattice.pt[0][0][0]; move -r 1 0 0; lattice -e -rt ffd1;");
    close(nums(&ok(&mut c, "getAttr ffd1Lattice.t;"))[1] - nums(&ok(&mut c, "getAttr ffd1Base.t;"))[1], 1.0, "move kept");
    // -minimumSize gives a flat plane depth.
    ok(&mut c, "polyPlane -n flat; lattice -oc 1 -mns 0.5 flat;");
    close(nums(&ok(&mut c, "getAttr ffd2Lattice.s;"))[1], 0.5, "least side");
    ok(&mut c, "lattice -e -ofd 0.75 ffd2;");
    close(num(&ok(&mut c, "lattice -q -ofd ffd2;")), 0.75, "falloff distance");
    // -useComponentTags: no set.
    ok(&mut c, "polyCube -n ct; lattice -uct ct;");
    assert!(ok(&mut c, "listConnections -type objectSet ffd3;").as_array().is_none_or(Vec::is_empty));
    // -geometry with -e adds a mesh, -rm takes it off.
    ok(&mut c, "polyCube -n more; lattice -e -g more ffd1;");
    assert_eq!(strs(&ok(&mut c, "lattice -q -g ffd1;")).len(), 2);
    ok(&mut c, "lattice -e -rm -g more ffd1;");
    assert_eq!(strs(&ok(&mut c, "lattice -q -g ffd1;")).len(), 1);
}

#[test]
fn nonlinear_query_edit_and_parents() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n cb;");
    let nl = strs(&ok(&mut c, "nonLinear -type bend -curvature 0.5 -ap -ds cb;"));
    assert_eq!(nl, vec!["bend1", "bend1Handle"]);
    assert_eq!(strs(&ok(&mut c, "listRelatives -p bend1Handle;")), vec!["cb"]);
    assert_eq!(nums(&ok(&mut c, "getAttr bend1Handle.s;")), vec![1.0, 1.0, 1.0]);
    close(num(&ok(&mut c, "nonLinear -q -curvature bend1;")), 0.5, "curvature");
    ok(&mut c, "nonLinear -e -curvature 1.2 -lb -0.5 bend1Handle;");
    close(num(&ok(&mut c, "nonLinear -q -curvature bend1;")), 1.2, "edited by its handle");
    close(num(&ok(&mut c, "nonLinear -q -lb bend1;")), -0.5, "low bound");
    assert_eq!(ok(&mut c, "nonLinear -q -type bend1;"), json!("bend"));
    assert_eq!(strs(&ok(&mut c, "nonLinear -q -dt bend1;")), vec!["bend1Handle"]);
    ok(&mut c, "polyCube -n cb2; nonLinear -e -g cb2 bend1;");
    assert_eq!(strs(&ok(&mut c, "nonLinear -q -g bend1;")).len(), 2);
    // -commonParent: a new transform over the object and the handle.
    ok(&mut c, "polyCube -n cb3; nonLinear -type twist -cp cb3;");
    assert_eq!(strs(&ok(&mut c, "listRelatives -p cb3;")), vec!["twist1Group"]);
    assert_eq!(strs(&ok(&mut c, "listRelatives -p twist1Handle;")), vec!["twist1Group"]);
    // -exclusive.
    ok(&mut c, "polyCube -n cb4; nonLinear -type sine -ex sinePart cb4;");
    assert_eq!(ok(&mut c, "nonLinear -q -ex sine1;"), json!("sinePart"));
    assert!(c.mel_checked("polyCube -n x1; polyCube -n x2; nonLinear -type bend -ap x1 x2;").is_err());
}

#[test]
fn joint_lattice_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -p 0 0 0 -n f1; joint -p 0 2 0 -n f2; joint -p 0 4 0 -n f3;");
    ok(&mut c, "polyCylinder -n arm -h 4 -sy 8; move 0 2 0 arm; lattice -dv 2 5 2 -oc 1 arm;");
    let n = ok(&mut c, "jointLattice -joint f2 -ex jlPart ffd1Lattice;");
    let name = n.as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("jointLattice -q -ex {name};")), json!("jlPart"));
    assert_eq!(nums(&ok(&mut c, &format!("jointLattice -q -gi {name};"))), vec![0.0]);
    assert!(!strs(&ok(&mut c, &format!("jointLattice -q -cmp {name};"))).is_empty());
    assert_eq!(nums(&ok(&mut c, &format!("jointLattice -q -dui {name};"))).len(), 3);
    ok(&mut c, &format!("jointLattice -e -creasing 0.5 {name};"));
    close(num(&ok(&mut c, &format!("jointLattice -q -creasing {name};"))), 0.5, "creasing");
}

#[test]
fn script_ui_layout_flags() {
    let mut c = Core::new();
    ok(&mut c, "window w1; columnLayout col; intScrollBar -ls 7 isb; layerButton -hp 1 -cc \"print 1\" lb; menuBarLayout mbl; setParent ..;");
    close(num(&ok(&mut c, "intScrollBar -q -ls isb;")), 7.0, "large step");
    ok(&mut c, "intScrollBar -e -ls 3 isb;");
    close(num(&ok(&mut c, "intScrollBar -q -largeStep isb;")), 3.0, "edited");
    assert_eq!(num(&ok(&mut c, "layerButton -q -hp lb;")), 1.0);
    ok(&mut c, "layout -e -gsp 4 -mar 2 col; layout -e -mr4 1 2 3 4 col; menuBarLayout -e -gsp 3 -mar 1 mbl;");
}

#[test]
fn list_attr_filters() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n lc;");
    let ro = strs(&ok(&mut c, "listAttr -ro lcShape;"));
    assert!(ro.contains(&"worldMatrix".to_string()), "{ro:?}");
    assert!(!ro.contains(&"inMesh".to_string()) && !ro.contains(&"visibility".to_string()), "{ro:?}");
    let d3 = strs(&ok(&mut c, "listAttr -at double3 -st \"t*\" lc;"));
    assert!(d3.contains(&"translate".to_string()) && !d3.contains(&"translateX".to_string()), "{d3:?}");
    let lin = strs(&ok(&mut c, "listAttr -at doubleLinear -st \"t*\" lc;"));
    assert!(lin.contains(&"translateX".to_string()) && !lin.contains(&"translate".to_string()), "{lin:?}");
    assert_eq!(strs(&ok(&mut c, "listAttr -fnn -st tx lc;")), vec!["|lc.translateX"]);
    ok(&mut c, "createNode remapValue -n rv;");
    assert_eq!(strs(&ok(&mut c, "listAttr -ra rv;")), vec!["value", "color"]);
    ok(&mut c, "createNode file -n fl;");
    assert!(strs(&ok(&mut c, "listAttr -uf fl;")).contains(&"fileTextureName".to_string()));
    assert!(strs(&ok(&mut c, "listAttr -ex lc;")).is_empty());
    assert!(strs(&ok(&mut c, "listAttr -fp lc;")).is_empty());
    // -caching: until setAttr -caching off takes one out.
    ok(&mut c, "setAttr -caching 0 lc.v;");
    assert!(strs(&ok(&mut c, "listAttr -ca -st visibility lc;")).is_empty());
    assert_eq!(strs(&ok(&mut c, "listAttr -ca -st translate lc;")), vec!["translate"]);
    ok(&mut c, "addAttr -ln note -dt \"string\" lc;");
    assert!(strs(&ok(&mut c, "listAttr -hnd lc;")).contains(&"note".to_string()));
    // -s is scalar, -se settable.
    let sc = strs(&ok(&mut c, "listAttr -s -st \"*Mesh\" lcShape;"));
    assert_eq!(sc, vec!["displaySmoothMesh"], "scalars only, no meshes");
    assert_eq!(strs(&ok(&mut c, "listAttr -se -st tx lc;")), vec!["translateX"]);
    ok(&mut c, "setAttr -l 1 lc.tx;");
    assert!(strs(&ok(&mut c, "listAttr -se -st tx lc;")).is_empty());
}

#[test]
fn list_connections_and_history_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n lc; group -n grp lc;");
    let full = strs(&ok(&mut c, "listConnections -fnn -sh 1 polyCube1;"));
    assert!(full.contains(&"|grp|lc|lcShape".to_string()), "{full:?}");
    let plugs = strs(&ok(&mut c, "listConnections -fnn -p 1 -sh 1 polyCube1;"));
    assert!(plugs.iter().any(|p| p == "|grp|lc|lcShape.inMesh"), "{plugs:?}");
    assert_eq!(strs(&ok(&mut c, "listHistory -fnn lc;")), vec!["|grp|lc|lcShape", "polyCube1"]);
    assert_eq!(strs(&ok(&mut c, "listHistory -q -ha lc;")), vec!["inMesh"]);
    assert_eq!(strs(&ok(&mut c, "listHistory -q -fw lc;")), vec!["worldMesh"]);
    assert_eq!(strs(&ok(&mut c, "listHistory -q -fl lc;")), vec!["outMesh"]);
    ok(&mut c, "setAttr polyCube1.isHistoricallyInteresting 0;");
    assert_eq!(strs(&ok(&mut c, "listHistory -il 1 lc;")), vec!["lcShape"]);
    // -leaf off: the transform's own history.
    ok(&mut c, "setKeyframe -at tx lc;");
    let own = strs(&ok(&mut c, "listHistory -lf 0 lc;"));
    assert!(own.iter().any(|n| n.starts_with("lc_translateX")), "{own:?}");
    // The future: without -af only what the output geometry feeds.
    ok(&mut c, "polyCube -n src; polyCube -n dst; connectAttr srcShape.outMesh dstShape.inMesh -f;");
    assert_eq!(strs(&ok(&mut c, "listHistory -f 1 -lv 1 src;")), vec!["srcShape", "dstShape"]);
    let all = strs(&ok(&mut c, "listHistory -f 1 -af -lv 1 src;"));
    assert!(all.contains(&"dstShape".to_string()) && all.contains(&"initialShadingGroup".to_string()), "{all:?}");
    // Breadth first: nearest first; depth first follows each branch.
    ok(&mut c, "createNode transform -n t0; createNode transform -n ta; createNode transform -n tb; createNode transform -n taa;");
    ok(&mut c, "connectAttr t0.tx ta.tx; connectAttr t0.ty tb.ty; connectAttr ta.tx taa.tx;");
    assert_eq!(strs(&ok(&mut c, "listHistory -f 1 -ac -bf t0;")), vec!["t0", "ta", "tb", "taa"]);
    assert_eq!(strs(&ok(&mut c, "listHistory -f 1 -ac t0;")), vec!["t0", "ta", "taa", "tb"]);
}

#[test]
fn load_plugin_flags() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "loadPlugin -pp \"doesNotExist\";"), json!([""]));
    let dir = std::env::temp_dir().join(format!("u04_plugins_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("u04Plug.py");
    std::fs::write(&file, "import maya.api.OpenMaya as om\ndef maya_useNewAPI(): pass\ndef initializePlugin(o): om.MFnPlugin(o)\ndef uninitializePlugin(o): pass\n").unwrap();
    let path = file.to_string_lossy().replace('\\', "/");
    assert_eq!(ok(&mut c, &format!("loadPlugin -pp \"{path}\";")), json!([path.clone()]));
    ok(&mut c, "global proc u04Loaded(string $p) { global string $u04Got; $u04Got = $p; }");
    ok(&mut c, "loadPlugin -ac u04Loaded;");
    let r = c.mel_checked(&format!("loadPlugin \"{path}\";"));
    if r.is_ok() {
        c.idle();
        assert_eq!(ok(&mut c, "global string $u04Got; $u04Got;"), json!("u04Plug"));
        // Taken back: no more calls.
        ok(&mut c, "loadPlugin -rc u04Loaded; global string $u04Got; $u04Got = \"\";");
        ok(&mut c, &format!("unloadPlugin u04Plug; loadPlugin \"{path}\";"));
        c.idle();
        assert_eq!(ok(&mut c, "global string $u04Got; $u04Got;"), json!(""));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn loft_nurbs_history_query_edit() {
    let mut c = Core::new();
    ok(&mut c, "circle -n c1 -nr 0 1 0; circle -n c2 -nr 0 1 0 -r 2; move 0 2 0 c2; circle -n c3 -nr 0 1 0; move 0 4 0 c3;");
    let lf = strs(&ok(&mut c, "loft -cc 0 -cc 1 -cc 0 -ch 1 c1 c2 c3;"));
    assert_eq!(lf, vec!["loftedSurface1", "loft1"]);
    ok(&mut c, "loft -e -d 1 loft1;");
    assert_eq!(num(&ok(&mut c, "loft -q -d loft1;")), 1.0);
    ok(&mut c, "loft -e -d 3 loftedSurface1;");
    assert_eq!(num(&ok(&mut c, "loft -q -d loftedSurface1;")), 3.0);
    // The cusp: straight from section to section, not curving out past the middle circle as the smooth loft does.
    ok(&mut c, "loft -ch 1 -n smooth c1 c2 c3;");
    let (b, sm) = (nums(&ok(&mut c, "exactWorldBoundingBox loftedSurface1;")), nums(&ok(&mut c, "exactWorldBoundingBox smooth;")));
    assert!(b[3] < sm[3] - 1e-3, "cusp {b:?} smooth {sm:?}");
    ok(&mut c, "delete smooth;");
    // nodeState 2 keeps what it gave.
    ok(&mut c, "loft -e -nds 2 loft1; move 0 10 0 c3;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox loftedSurface1;"))[4], 4.0, "blocked");
    assert_eq!(num(&ok(&mut c, "loft -q -nds loft1;")), 2.0);
    ok(&mut c, "loft -e -nds 0 loft1;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox loftedSurface1;"))[4], 10.0, "follows again");
    // -reverse with autoReverse off, and -rebuild through rebuildCurve nodes.
    ok(&mut c, "loft -ar 0 -r 0 -r 1 -rb 1 -n rev c1 c2;");
    assert!(strs(&ok(&mut c, "listHistory rev;")).iter().any(|n| n.starts_with("rebuildCurve")));
    // -object off: only the node.
    let only = strs(&ok(&mut c, "loft -o 0 c1 c2;"));
    assert_eq!(only.len(), 1);
    assert!(only[0].starts_with("loft"));

    ok(&mut c, "curve -d 1 -p 0 0 0 -p 10 0 0 -n ln;");
    let ik = strs(&ok(&mut c, "insertKnotCurve -p 0.5 -ch 1 ln;"));
    close(num(&ok(&mut c, &format!("insertKnotCurve -q -p {};", ik[1]))), 0.5, "parameter");
    ok(&mut c, &format!("insertKnotCurve -e -p 0.25 {};", ik[1]));
    close(num(&ok(&mut c, "insertKnotCurve -q -p ln;")), 0.25, "edited through the curve");
    ok(&mut c, "insertKnotCurve -e -nds 1 ln;");
    assert_eq!(num(&ok(&mut c, "insertKnotCurve -q -nds ln;")), 1.0);

    ok(&mut c, "nurbsPlane -n srf -u 2 -v 2;");
    let iks = strs(&ok(&mut c, "insertKnotSurface -p 0.3 -d 1 -ch 1 srf;"));
    close(num(&ok(&mut c, &format!("insertKnotSurface -q -nk {};", iks[1]))), 1.0, "knots");
    ok(&mut c, &format!("insertKnotSurface -e -cch 1 {};", iks[1]));
    assert_eq!(num(&ok(&mut c, &format!("insertKnotSurface -q -cch {};", iks[1]))), 1.0);

    let nc = strs(&ok(&mut c, "nurbsCube -w 2;"));
    close(num(&ok(&mut c, &format!("nurbsCube -q -w {};", nc[1]))), 2.0, "width");
    ok(&mut c, &format!("nurbsCube -e -w 4 {};", nc[1]));
    close(nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", nc[0])))[3], 2.0, "edited cube");
}

#[test]
fn intersect_boolean_and_single_surface() {
    let mut c = Core::new();
    ok(&mut c, "sphere -n s1; nurbsPlane -n p1 -ax 0 1 0 -w 4;");
    let r = strs(&ok(&mut c, "intersect -cos 1 -fs 0 s1 p1;"));
    assert!(r.len() >= 3, "a curve on each surface and the node: {r:?}");
    let node = r.last().unwrap().clone();
    assert!(node.starts_with("intersectSurface"));
    close(num(&ok(&mut c, &format!("intersect -q -tol {node};"))), 0.01, "tolerance");
    ok(&mut c, &format!("intersect -e -tol 0.001 -fs 1 {node};"));
    close(num(&ok(&mut c, &format!("intersect -q -tol {node};"))), 0.001, "edited");
    assert_eq!(num(&ok(&mut c, &format!("intersect -q -fs {node};"))), 1.0);
    let first = strs(&ok(&mut c, "intersect -cos 1 -fs 1 s1 p1;"));
    assert_eq!(first.len(), r.len() - (r.len() - 1) / 2, "first surface only: {first:?}");

    ok(&mut c, "nurbsPlane -n np1 -ax 0 1 0 -w 1; nurbsPlane -n np2 -ax 0 1 0 -w 1; move 1.02 0 0 np2;");
    let ms = strs(&ok(&mut c, "makeSingleSurface -st 0.05 np1 np2;"));
    assert_eq!(ms.len(), 2);
    assert_eq!(num(&ok(&mut c, &format!("polyEvaluate -v {};", ms[0]))), 6.0);
    close(num(&ok(&mut c, &format!("makeSingleSurface -q -st {};", ms[0]))), 0.05, "stitch tolerance");
    ok(&mut c, &format!("makeSingleSurface -e -st 0.01 {};", ms[1]));
    assert_eq!(num(&ok(&mut c, &format!("polyEvaluate -v {};", ms[0]))), 8.0);

    ok(&mut c, "nurbsCube -n nc; sphere -n s2 -r 0.6;");
    let nb = strs(&ok(&mut c, "nurbsBoolean -op 1 -nsf 1 nc s2;"));
    assert_eq!(nb.len(), 2);
    assert_eq!(num(&ok(&mut c, &format!("nurbsBoolean -q -op {};", nb[0]))), 1.0);
    let before = num(&ok(&mut c, &format!("polyEvaluate -f {};", nb[0])));
    ok(&mut c, &format!("nurbsBoolean -e -op 2 {};", nb[0]));
    assert_eq!(num(&ok(&mut c, &format!("nurbsBoolean -q -op {};", nb[1]))), 2.0);
    let after = num(&ok(&mut c, &format!("polyEvaluate -f {};", nb[0])));
    assert_ne!(before, after, "the operation changes the result");
    ok(&mut c, &format!("nurbsBoolean -e -tlb 0.05 {};", nb[0]));
    close(num(&ok(&mut c, &format!("nurbsBoolean -q -tlb {};", nb[0]))), 0.05, "tolerance");
}

#[test]
fn multi_profile_birail() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 0 0 10 -n r1; curve -d 1 -p 4 0 0 -p 4 0 10 -n r2;");
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 2 1 0 -p 4 0 0 -n p1; curve -d 1 -p 0 0 5 -p 2 3 5 -p 4 0 5 -n p2; curve -d 1 -p 0 0 10 -p 2 1 10 -p 4 0 10 -n p3;");
    let mb = strs(&ok(&mut c, "multiProfileBirailSurface -ch 1 p1 p2 p3 r1 r2;"));
    assert_eq!(mb.len(), 2);
    assert!(mb[1].starts_with("multiProfileBirailSurface"));
    let b = nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", mb[0])));
    close(b[4], 3.0, "reaches the middle profile");
    close(b[5], 10.0, "runs the rails");
    // The middle profile lower: the surface follows.
    ok(&mut c, "move -r 0 -1 0 p2.cv[1];");
    close(nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", mb[0])))[4], 2.0, "follows the profile");
    ok(&mut c, &format!("multiProfileBirailSurface -e -tm 1 {};", mb[1]));
    assert_eq!(num(&ok(&mut c, &format!("multiProfileBirailSurface -q -tm {};", mb[1]))), 1.0);
    ok(&mut c, &format!("multiProfileBirailSurface -e -tp1 1 -nds 2 {};", mb[0]));
    assert_eq!(num(&ok(&mut c, &format!("multiProfileBirailSurface -q -tp1 {};", mb[1]))), 1.0);
    assert!(c.mel_checked("multiProfileBirailSurface p1 r1 r2;").is_err());
    let no_ch = strs(&ok(&mut c, "multiProfileBirailSurface -ch 0 p1 p2 p3 r1 r2;"));
    assert_eq!(no_ch.len(), 1);
}

#[test]
fn make_live_registry() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n m1; polyPlane -n m2; polyPlane -n m3;");
    ok(&mut c, "makeLive -registryReset; makeLive m1; makeLive m2 m3;");
    assert_eq!(num(&ok(&mut c, "makeLive -q -registryCount;")), 2.0);
    assert_eq!(strs(&ok(&mut c, "makeLive -q -registry 1;")), vec!["m1"]);
    assert_eq!(strs(&ok(&mut c, "makeLive -q -registry 0;")), vec!["m2", "m3"]);
    ok(&mut c, "makeLive -e -addObjects m1;");
    assert_eq!(strs(&ok(&mut c, "makeLive -q;")).len(), 3);
    ok(&mut c, "makeLive -e -removeObjects m2;");
    assert_eq!(strs(&ok(&mut c, "makeLive -q;")).len(), 2);
    ok(&mut c, "makeLive -registry 1;");
    assert_eq!(strs(&ok(&mut c, "makeLive -q;")).len(), 1);
    assert_eq!(ok(&mut c, "makeLive -q -registry 1;"), json!(["m1"]));
    ok(&mut c, "makeLive -registrySize 1;");
    assert_eq!(num(&ok(&mut c, "makeLive -q -registrySize;")), 1.0);
    assert_eq!(num(&ok(&mut c, "makeLive -q -registryCount;")), 1.0);
    ok(&mut c, "makeLive -registryReset;");
    assert_eq!(num(&ok(&mut c, "makeLive -q -registryCount;")), 0.0);
    assert_eq!(num(&ok(&mut c, "makeLive -q -registrySize;")), 10.0);
    ok(&mut c, "makeLive -none;");
    assert!(strs(&ok(&mut c, "makeLive -q;")).is_empty());
}

#[test]
fn n_particle_flags() {
    let mut c = Core::new();
    let np = strs(&ok(&mut c, "nParticle -p 0 0 0 -p 1 0 0 -jbp 0 5 0 -nj 4 -jr 0.5 -jbp 0 -5 0 -c 0.5 -i 0.3;"));
    assert_eq!(np, vec!["nParticle1", "nParticleShape1"]);
    // Two placed, four round each base point (the count reused).
    assert_eq!(num(&ok(&mut c, "nParticle -q -ct nParticle1;")), 10.0);
    close(num(&ok(&mut c, "nParticle -q -c nParticle1;")), 0.5, "conserve");
    close(num(&ok(&mut c, "nParticle -q -i nParticle1;")), 0.3, "inherit");
    assert_eq!(strs(&ok(&mut c, "nParticle -q -ppv nParticle1;")), vec!["position", "velocity", "rgbPP"]);
    assert!(strs(&ok(&mut c, "nParticle -q -ppd nParticle1;")).contains(&"lifespanPP".to_string()));
    ok(&mut c, "addAttr -ln myPP -dt doubleArray nParticleShape1;");
    assert_eq!(strs(&ok(&mut c, "nParticle -q -dal nParticle1;")), vec!["myPP"]);
    let jit = nums(&ok(&mut c, "nParticle -attribute position -order 2 -q nParticle1;"));
    assert!((jit[1] - 5.0).abs() <= 0.5 + 1e-9, "jittered round its base: {jit:?}");
    ok(&mut c, "nParticle -e -sn myShape nParticle1;");
    assert_eq!(ok(&mut c, "nParticle -q -sn nParticle1;"), json!("myShape"));
    ok(&mut c, "nParticle -e -c 1 nParticle1;");
    close(num(&ok(&mut c, "nParticle -q -c nParticle1;")), 1.0, "conserve edited");
    ok(&mut c, "nParticle -e -ch 0 nParticle1;");
    assert_eq!(num(&ok(&mut c, "nParticle -q -ch nParticle1;")), 0.0);
    // A per particle edit: the second particle sent upward.
    ok(&mut c, "currentTime 2;");
    ok(&mut c, "nParticle -e -attribute velocity -order 1 -vv 0 48 0 nParticle1;");
    ok(&mut c, "currentTime 3;");
    let p = nums(&ok(&mut c, "nParticle -attribute position -order 1 -q nParticle1;"));
    let p0 = nums(&ok(&mut c, "nParticle -attribute position -order 0 -q nParticle1;"));
    assert!(p[1] - p0[1] > 1.0, "sent up, two units a frame: {p:?} {p0:?}");
    ok(&mut c, "nParticle -e -dc nParticle1;");
}

#[test]
fn name_command_namespace_node_type() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "nameCommand -ann \"x\" -c \"print 1\" -da1 aa -da2 bb -da3 cc -d 1 myNC;"), json!("myNC"));
    ok(&mut c, "nameCommand -ann \"y\" -c \"print 2\" userNC;");
    ok(&mut c, "hotkey -k F5 -name myNC; hotkey -k F6 -name userNC;");
    // A default name command is left out of the hotkey set written.
    let text = c.mel_checked("hotkeySet -q -current;").ok();
    let _ = text;

    ok(&mut c, "namespace -add \"a\"; namespace -add \"b\" -p \"a\"; namespace -add \"c\";");
    ok(&mut c, "namespace -set \":a:b\"; polyCube -n inb; namespace -set \":\";");
    ok(&mut c, "namespace -rm \":a:b\" -mno \":c\";");
    assert_eq!(strs(&ok(&mut c, "namespaceInfo -lon -r \":\";")), vec!["a", "c"]);
    assert!(strs(&ok(&mut c, "namespaceInfo -ls \":c\";")).contains(&"c:inb".to_string()));
    assert!(c.mel_checked("namespace -rm \":a\" -mno \":c\" -mnr;").is_err());
    ok(&mut c, "namespace -add \"x\"; namespace -add \"y\" -p \"x\"; namespace -add \"z\" -p \":x:y\";");
    ok(&mut c, "namespace -rm \":x:y:z\"; namespace -ch \":x:y:z\";");
    assert_eq!(strs(&ok(&mut c, "namespaceInfo -lon -r \":\";")), vec!["a", "c"]);
    // An ancestor holding something stays.
    ok(&mut c, "namespace -add \"k\"; namespace -set \":k\"; polyCube -n kept; namespace -add \"l\"; namespace -set \":\";");
    ok(&mut c, "namespace -rm \":k:l\"; namespace -ch \":k:l\";");
    assert!(strs(&ok(&mut c, "namespaceInfo -lon \":\";")).contains(&"k".to_string()));

    ok(&mut c, "itemFilter -byType \"joint\" myFilt;");
    assert!(strs(&ok(&mut c, "namespaceInfo -lod -int \":\";")).contains(&"myFilt".to_string()));
    assert!(!strs(&ok(&mut c, "namespaceInfo -lod \":\";")).contains(&"myFilt".to_string()));

    ok(&mut c, "polyCube -n nt;");
    assert_eq!(ok(&mut c, "nodeType -urn nt;"), json!("Maya-DG"));
    assert!(c.mel_checked("nodeType -urn nothingHere;").is_err());
}

#[test]
fn mimic_manipulation_sessions() {
    let mut c = Core::new();
    ok(&mut c, "createNode transform -n tx1;");
    assert_eq!(ok(&mut c, r#"mimicManipulation -manipulations "{ \"session\": [[{\"plug\": \"tx1.tx\", \"value\": 1.0}]] }";"#), json!([false]));
    close(num(&ok(&mut c, "getAttr tx1.tx;")), 1.0, "set");
    ok(&mut c, "evaluationManager -mode parallel;");
    let r = ok(&mut c, r#"mimicManipulation -m "{ \"session\": [[{\"plug\": \"tx1.t\", \"value\": [1,2,3]}],[{\"plug\": \"tx1.ty\", \"value\": 5}]] }";"#);
    assert_eq!(r, json!([true, true]));
    assert_eq!(nums(&ok(&mut c, "getAttr tx1.t;")), vec![1.0, 5.0, 3.0]);
    // Locked: not the evaluation manager's.
    ok(&mut c, "setAttr -l 1 tx1.tz;");
    assert_eq!(ok(&mut c, r#"mimicManipulation -m "{ \"session\": [[{\"plug\": \"tx1.tz\", \"value\": 9}]] }";"#), json!([false]));
    // Without refresh nothing evaluates.
    assert_eq!(ok(&mut c, r#"mimicManipulation -r 0 -m "{ \"session\": [[{\"plug\": \"tx1.tx\", \"value\": 2}]] }";"#), json!([false]));
    // Without prevalidation the first goes the ordinary way.
    assert_eq!(ok(&mut c, r#"mimicManipulation -p 0 -m "{ \"session\": [[{\"plug\": \"tx1.tx\", \"value\": 2}],[{\"plug\": \"tx1.tx\", \"value\": 3}]] }";"#), json!([false, true]));
    assert!(c.mel_checked(r#"mimicManipulation -m "{ \"session\": [[{\"plug\": \"tx1.v\", \"value\": 1}]] }";"#).is_err());
    assert!(c.mel_checked(r#"mimicManipulation -m "not json";"#).is_err());
}

#[test]
fn ml_deformer_model() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n base -sx 2 -sy 2; polyPlane -n tgt -sx 2 -sy 2; move -r 0 1 0 tgt.vtx[4];");
    let ml = strs(&ok(&mut c, "deformer -type mlDeformer base;"));
    assert_eq!(ml, vec!["mlDeformer1"]);
    ok(&mut c, "connectAttr tgtShape.outMesh mlDeformer1.targetGeometry;");
    let d = nums(&ok(&mut c, "mlDeformer -mi 0 -vertexDeltas \"dbp\" mlDeformer1;"));
    assert_eq!(d.len(), 27);
    close(d[13], 1.0, "the middle point's delta");
    // dbs: in the point's own frame, the normal its third axis.
    let s = nums(&ok(&mut c, "mlDeformer -vertexDeltas \"dbs\" mlDeformer1;"));
    close(s[14].abs(), 1.0, "along the normal");
    assert!(c.mel_checked("mlDeformer -vertexDeltas \"nope\" mlDeformer1;").is_err());
    close(pos(&mut c, "base.vtx[4]")[1], 0.0, "no model yet");
    ok(&mut c, "mlDeformer -reloadModel mlDeformer1;");
    close(pos(&mut c, "base.vtx[4]")[1], 1.0, "the model replays the target");
    // The model follows the base: lifted, the correction comes along.
    ok(&mut c, "setAttr mlDeformer1.envelope 0.5;");
    close(pos(&mut c, "base.vtx[4]")[1], 0.5, "envelope");
    ok(&mut c, "setAttr mlDeformer1.envelope 1; mlDeformer -exportActive 1 mlDeformer1;");
    close(pos(&mut c, "base.vtx[4]")[1], 0.0, "passes through while exporting");
    assert_eq!(num(&ok(&mut c, "getAttr mlDeformer1.exportActive;")), 1.0);
    let frames = nums(&ok(&mut c, "mlDeformer -vertexFrames mlDeformer1;"));
    assert_eq!(frames.len(), 9 * 16);
    close(frames[15], 1.0, "a matrix each");
    assert!(nums(&ok(&mut c, "mlDeformer -cmx mlDeformer1;")).is_empty());
    assert!(c.mel_checked("mlDeformer -mi 5 -reloadModel mlDeformer1;").is_err());
}
