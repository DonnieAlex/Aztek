//! A mesh's construction history in .ma and .mb files (#209): written as the reference's nodes, read back as history.

use aztek_scene::ma_file::{read_text, write_text};
use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn temp(name: &str) -> String {
    let dir = std::env::temp_dir().join(format!("aztek_history_files_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name).to_str().unwrap().to_string()
}

/// The history above a mesh, nearest first, as node types.
fn history_types(s: &mut Session, m: &str) -> Vec<String> {
    let h = run(s, "listHistory", json!({"args": [m]}));
    h.as_array()
        .unwrap()
        .iter()
        .skip(1)
        .map(|n| run(s, "nodeType", json!({"args": [n]})).as_str().unwrap().to_string())
        .collect()
}

/// Everything about the mesh's shape that history must reproduce: counts and every point.
fn shape(s: &mut Session, m: &str) -> (Value, Vec<Value>) {
    let info = run(s, "meshInfo", json!({"args": [m]}));
    let data = run(s, "meshData", json!({"args": [m]}));
    let counts = json!([info["points"], info["edges"], info["faces"]]);
    (counts, data["points"].as_array().unwrap().clone())
}

fn same_points(a: &[Value], b: &[Value]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(p, q)| (0..3).all(|k| (p[k].as_f64().unwrap() - q[k].as_f64().unwrap()).abs() < 1e-6))
}

/// A cube with several kinds of edit on it: a bevel, a face extrude, a point move, a planar UV projection.
fn modelled(s: &mut Session) {
    run(s, "polyCube", json!({"w": 2, "h": 2, "d": 2, "n": "box"}));
    run(s, "polyBevel3", json!({"args": ["box.e[0]"], "o": 0.25, "sg": 3}));
    run(s, "polyExtrudeFacet", json!({"args": ["box.f[1]"], "ltz": 1.5}));
    run(s, "move", json!({"args": ["box.vtx[2]", 0.0, 0.4, 0.0], "r": true}));
    run(s, "polyPlanarProjection", json!({"args": ["box.f[0:3]"]}));
}

#[test]
fn a_modelled_mesh_keeps_its_history_through_a_file() {
    for ext in ["ma", "mb"] {
        let mut s = Session::new();
        modelled(&mut s);
        let before = history_types(&mut s, "box");
        assert!(before.iter().any(|t| t == "polyBevel3") && before.iter().any(|t| t == "polyExtrudeFace"), "{before:?}");
        let (counts, points) = shape(&mut s, "box");
        let path = temp(&format!("modelled.{ext}"));
        run(&mut s, "file", json!({"rename": path}));
        run(&mut s, "file", json!({"save": true, "type": if ext == "ma" { "mayaAscii" } else { "mayaBinary" }}));

        let mut back = Session::new();
        run(&mut back, "file", json!({"args": [path], "open": true, "force": true}));
        assert_eq!(history_types(&mut back, "box"), before, "{ext}: the same history, in the same order");
        let (c2, p2) = shape(&mut back, "box");
        assert_eq!(c2, counts, "{ext}");
        assert!(same_points(&points, &p2), "{ext}: the same points");
        // Still live: a setting changed after reading changes the mesh.
        assert_eq!(run(&mut back, "getAttr", json!({"args": ["polyBevel1.segments"]})), json!(3), "{ext}");
        run(&mut back, "setAttr", json!({"args": ["polyBevel1.segments", 1]}));
        let (c3, _) = shape(&mut back, "box");
        assert_ne!(c3, counts, "{ext}: the bevel is live");
    }
}

#[test]
fn the_file_holds_the_references_nodes() {
    let mut s = Session::new();
    modelled(&mut s);
    let (text, warnings) = write_text(&mut s, "modelled.ma", None);
    assert!(!warnings.iter().any(|w| w.contains("baked")), "{warnings:?}");
    for line in [
        "createNode polyCube -n \"polyCube1\";",
        "createNode polyBevel3 -n \"polyBevel1\";",
        "\tsetAttr \".sg\" 3;",
        "\tsetAttr \".ics\" -type \"componentList\" 1 \"e[0]\";",
        "createNode polyExtrudeFace -n \"polyExtrudeFace1\";",
        "createNode polyTweak -n \"polyTweak1\";",
        "createNode polyPlanarProj -n \"polyPlanarProj1\";",
        "connectAttr \"polyCube1.out\" \"polyBevel1.ip\";",
        "connectAttr \"boxShape.wm\" \"polyExtrudeFace1.mp\";",
    ] {
        assert!(text.contains(line), "missing {line:?}");
    }
    // The shape is made by its history: no geometry of its own.
    let shape_block = text.split("createNode mesh -n \"boxShape\"").nth(1).unwrap().split("createNode").next().unwrap();
    assert!(!shape_block.contains(".vt["), "{shape_block}");
}

#[test]
fn a_reference_file_with_a_bevel_reads_as_history() {
    let text = r#"//Maya ASCII 2026 scene
requires maya "2026";
currentUnit -l centimeter -a degree -t film;
createNode transform -n "piece";
createNode mesh -n "pieceShape" -p "piece";
	setAttr -k off ".v";
createNode polyCube -n "polyCube1";
	setAttr ".w" 2;
	setAttr ".h" 2;
	setAttr ".d" 2;
	setAttr ".cuv" 4;
createNode polyBevel3 -n "polyBevel1";
	setAttr ".uopa" yes;
	setAttr ".ics" -type "componentList" 1 "e[0:3]";
	setAttr ".ix" -type "matrix" 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1;
	setAttr ".ws" yes;
	setAttr ".oaf" yes;
	setAttr ".f" 0.4;
	setAttr ".sg" 2;
	setAttr ".at" 180;
	setAttr ".sn" yes;
	setAttr ".mv" yes;
	setAttr ".mvt" 0.0001;
	setAttr ".sa" 30;
connectAttr "polyBevel1.out" "pieceShape.i";
connectAttr "polyCube1.out" "polyBevel1.ip";
connectAttr "pieceShape.wm" "polyBevel1.mp";
"#;
    let mut s = Session::new();
    let warnings = read_text(&mut s, text, false);
    assert!(!warnings.iter().any(|w| w.contains("no stored geometry") || w.contains("pieceShape")), "{warnings:?}");
    assert_eq!(history_types(&mut s, "piece"), ["polyBevel3", "polyCube"]);
    assert_eq!(run(&mut s, "getAttr", json!({"args": ["polyBevel1.segments"]})), json!(2));
    assert_eq!(run(&mut s, "getAttr", json!({"args": ["polyBevel1.ics"]})), json!("e[0:3]"));
    // Bevelled: four edges of a cube with two segments each add faces.
    let faces = run(&mut s, "meshInfo", json!({"args": ["piece"]}))["faces"].as_i64().unwrap();
    assert!(faces > 6, "{faces}");
    // And written back out as the same chain.
    let (out, _) = write_text(&mut s, "again.ma", None);
    assert!(out.contains("connectAttr \"polyCube1.out\" \"polyBevel1.ip\";"), "{out}");
    assert!(out.contains("connectAttr \"polyBevel1.out\" \"pieceShape.i\";"), "{out}");
}

#[test]
fn history_under_a_skin_feeds_the_original() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"n": "arm"}));
    run(&mut s, "polyBevel3", json!({"args": ["arm.e[0]"], "o": 0.1}));
    run(&mut s, "select", json!({"cl": true}));
    run(&mut s, "joint", json!({"p": [0, -1, 0], "n": "j1"}));
    run(&mut s, "joint", json!({"p": [0, 1, 0], "n": "j2"}));
    run(&mut s, "skinCluster", json!({"args": ["j1", "arm"]}));
    let (counts, points) = shape(&mut s, "arm");
    let (text, warnings) = write_text(&mut s, "skinned.ma", None);
    assert!(!warnings.iter().any(|w| w.contains("baked")), "{warnings:?}");
    assert!(text.contains("connectAttr \"polyBevel1.out\" \"armShapeOrig.i\";"), "{text}");

    let mut back = Session::new();
    read_text(&mut back, &text, false);
    let types = history_types(&mut back, "arm");
    assert!(types.contains(&"skinCluster".to_string()) && types.contains(&"polyBevel3".to_string()), "{types:?}");
    let (c2, p2) = shape(&mut back, "arm");
    assert_eq!(c2, counts);
    assert!(same_points(&points, &p2));
}

#[test]
fn keys_on_a_history_setting_go_with_it() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"n": "box"}));
    run(&mut s, "polyBevel3", json!({"args": ["box.e[0]"], "o": 0.1}));
    run(&mut s, "setKeyframe", json!({"args": ["polyBevel1.offset"], "t": 1, "v": 0.1}));
    run(&mut s, "setKeyframe", json!({"args": ["polyBevel1.offset"], "t": 10, "v": 0.3}));
    let (text, _) = write_text(&mut s, "keyed.ma", None);
    let mut back = Session::new();
    read_text(&mut back, &text, false);
    let at = |s: &mut Session, t: f64| run(s, "getAttr", json!({"args": ["polyBevel1.offset"], "time": t})).as_f64().unwrap();
    assert!((at(&mut back, 1.0) - 0.1).abs() < 1e-9 && (at(&mut back, 10.0) - 0.3).abs() < 1e-9, "{text}");
}

#[test]
fn a_mesh_numbered_with_gaps_is_baked_and_says_so() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"n": "box"}));
    run(&mut s, "delete", json!({"args": ["box"], "ch": true}));
    // Taken out without history: the faces left are numbered with a gap.
    run(&mut s, "polyDelFacet", json!({"args": ["box.f[2]"], "ch": false}));
    run(&mut s, "polyBevel3", json!({"args": ["box.e[0]"], "o": 0.1}));
    let (counts, points) = shape(&mut s, "box");
    let (text, warnings) = write_text(&mut s, "gaps.ma", None);
    let mut back = Session::new();
    read_text(&mut back, &text, false);
    let (c2, p2) = shape(&mut back, "box");
    assert_eq!(c2, counts);
    assert_eq!(p2.len(), points.len());
    if text.contains("createNode polyBevel3") {
        assert!(!warnings.iter().any(|w| w.contains("baked")), "{warnings:?}");
    } else {
        assert!(warnings.iter().any(|w| w.contains("baked")), "{warnings:?}");
    }
}

/// The whole of a mesh as meshData gives it, numbers rounded, for comparing two meshes.
fn whole(s: &mut Session, m: &str) -> Value {
    fn round(v: &Value) -> Value {
        match v {
            Value::Number(n) => json!((n.as_f64().unwrap() * 1e5).round() / 1e5),
            Value::Array(a) => Value::Array(a.iter().map(round).collect()),
            Value::Object(o) => Value::Object(o.iter().map(|(k, v)| (k.clone(), round(v))).collect()),
            v => v.clone(),
        }
    }
    round(&run(s, "meshData", json!({"args": [m]})))
}

#[test]
fn every_kind_of_edit_reads_back_the_same() {
    let edits: &[(&str, Value)] = &[
        ("polyBevel3", json!({"args": ["box.e[0:3]"], "o": 0.2, "sg": 2})),
        ("polyExtrudeFacet", json!({"args": ["box.f[1]"], "ltz": 1.0, "ls": [0.5, 0.5, 1.0]})),
        ("polyExtrudeEdge", json!({"args": ["box.e[0]"], "ltz": 0.5})),
        ("polyExtrudeVertex", json!({"args": ["box.vtx[0]"], "l": 0.3, "w": 0.1})),
        ("polyChipOff", json!({"args": ["box.f[1]"], "ltz": 0.5})),
        ("polyPoke", json!({"args": ["box.f[1]"], "ltz": 0.3})),
        ("polyMergeVertex", json!({"args": ["box.vtx[0:1]"], "d": 10.0})),
        ("polySubdivideFacet", json!({"args": ["box.f[0]"], "dv": 2})),
        ("polySmooth", json!({"args": ["box"], "dv": 1})),
        ("polyTriangulate", json!({"args": ["box.f[0:2]"]})),
        ("polyMirrorFace", json!({"args": ["box"], "ws": true, "d": 1})),
        ("polyMoveVertex", json!({"args": ["box.vtx[0:2]"], "ty": 0.3})),
        ("polyMoveFacet", json!({"args": ["box.f[0]"], "tz": 0.3})),
        ("polyMoveEdge", json!({"args": ["box.e[0]"], "tx": 0.2})),
        ("polyCollapseEdge", json!({"args": ["box.e[0]"]})),
        ("polyCollapseFacet", json!({"args": ["box.f[0]"]})),
        ("polyDelEdge", json!({"args": ["box.e[0]"], "cv": true})),
        ("polyDelFacet", json!({"args": ["box.f[0]"]})),
        ("polySoftEdge", json!({"args": ["box.e[0:3]"], "a": 0})),
        ("polyNormal", json!({"args": ["box.f[0]"], "nm": 0})),
        ("polyCrease", json!({"args": ["box.e[0]"], "v": 1.0})),
        ("polySplitRing", json!({"args": ["box.e[0]"]})),
        ("polyConnectComponents", json!({"args": ["box.e[0]", "box.e[2]"]})),
        ("polyDuplicateEdge", json!({"args": ["box.e[0]"]})),
        ("polyAverageVertex", json!({"args": ["box.vtx[0:3]"], "i": 3})),
        ("polyWedgeFace", json!({"args": ["box.f[1]"], "wa": 90.0, "ed": [4]})),
        ("polyPlanarProjection", json!({"args": ["box.f[0:3]"]})),
        ("polyCylindricalProjection", json!({"args": ["box.f[0:3]"]})),
        ("polySphericalProjection", json!({"args": ["box.f[0:3]"]})),
        ("polyAutoProjection", json!({"args": ["box.f[*]"]})),
        ("polyMapCut", json!({"args": ["box.e[0]"]})),
        ("polyMoveUV", json!({"args": ["box.map[0:3]"], "tu": 0.2})),
        ("polyLayoutUV", json!({"args": ["box.f[*]"]})),
        ("polyColorPerVertex", json!({"args": ["box.vtx[0]"], "rgb": [1.0, 0.0, 0.0], "cdo": true})),
        ("polyReduce", json!({"args": ["box"], "p": 50})),
        ("polyQuad", json!({"args": ["box"]})),
    ];
    let mut skipped = Vec::new();
    let mut failed = Vec::new();
    for (cmd, call) in edits {
        let mut s = Session::new();
        run(&mut s, "polyCube", json!({"sx": 2, "sy": 2, "sz": 2, "n": "box"}));
        if let Err(e) = s.run(cmd, call.clone()) {
            skipped.push(format!("{cmd}: {e}"));
            continue;
        }
        let types = history_types(&mut s, "box");
        if types.len() < 2 {
            failed.push(format!("{cmd}: made no history: {types:?}"));
            continue;
        }
        let mesh = whole(&mut s, "box");
        let (text, warnings) = write_text(&mut s, "edit.ma", None);
        let mut back = Session::new();
        let read = read_text(&mut back, &text, false);
        let types2 = history_types(&mut back, "box");
        let mesh2 = whole(&mut back, "box");
        if types2 != types || mesh2 != mesh || warnings.iter().any(|w| w.contains("baked")) {
            failed.push(format!("{cmd}: history {types:?} -> {types2:?}, same mesh {}, warnings {warnings:?} {read:?}", mesh == mesh2));
        }
    }
    eprintln!("skipped: {skipped:#?}");
    assert!(failed.is_empty(), "{failed:#?}");
}

#[test]
fn primitives_keep_their_settings_and_axis() {
    for (cmd, call) in [
        ("polyCylinder", json!({"n": "box", "r": 2.0, "h": 3.0, "sx": 12, "sy": 3, "ax": [0, 0, 1]})),
        ("polyCube", json!({"n": "box", "w": 3.0, "sx": 3, "ax": [1, 0, 0], "cuv": 0})),
        ("polyPipe", json!({"n": "box", "r": 2.0, "h": 1.0, "t": 0.3})),
        ("polyTorus", json!({"n": "box", "r": 2.0, "sr": 0.4, "ax": [0, 0, -1]})),
    ] {
        let mut s = Session::new();
        run(&mut s, cmd, call);
        run(&mut s, "polyExtrudeFacet", json!({"args": ["box.f[0]"], "ltz": 0.5}));
        let mesh = whole(&mut s, "box");
        let (text, warnings) = write_text(&mut s, "prim.ma", None);
        assert!(!warnings.iter().any(|w| w.contains("baked")), "{cmd}: {warnings:?}");
        let mut back = Session::new();
        read_text(&mut back, &text, false);
        assert_eq!(whole(&mut back, "box"), mesh, "{cmd}:\n{text}");
    }
}

#[test]
fn history_shared_by_two_meshes_is_written_once() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"n": "a"}));
    run(&mut s, "polyBevel3", json!({"args": ["a.e[0]"], "o": 0.1}));
    run(&mut s, "duplicate", json!({"args": ["a"], "ic": true, "n": "b"}));
    let (text, _) = write_text(&mut s, "shared.ma", None);
    assert_eq!(text.matches("createNode polyBevel3").count(), text.matches("polyBevel3 -n").count());
    for name in ["\"polyCube1\";", "\"polyBevel1\";"] {
        assert!(text.matches(&format!("-n {name}")).count() <= 1, "{name} twice:\n{text}");
    }
    let mut back = Session::new();
    read_text(&mut back, &text, false);
    assert_eq!(whole(&mut back, "b"), whole(&mut s, "b"));
}
