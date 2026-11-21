//! Commands of the reference from textScrollList to xform: textScrollList's item enabling, empty label and tags,
//! textureDeformer's and wire's deformer flags, toolButton's tool images and state, toolCollection -clear, toolDropped,
//! torus, trim and untrim query and edit, track's up distances, transferAttributes query and edit, translator's file rules
//! and finding, undoInfo's redo queue, unfold -applyToShell, unknownNode's class tag, unloadPlugin's callbacks,
//! uvSnapshot's UV set, viewFit's clip planes and children, viewPlace's directions, viewSet's queries, window's menu bar,
//! workspaceControl, workspaceControlState, workspaceLayoutManager, wrinkle's shape flags and xform's euler, parent space,
//! reflection and UV flags.

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

/// The last value the shell was sent for a control's prop.
fn shell_prop(ops: &[Value], path_end: &str, key: &str) -> Option<Value> {
    ops.iter().rev().find_map(|o| {
        let p = o["path"].as_str().unwrap_or("");
        if p.ends_with(path_end) {
            o["props"].get(key).cloned()
        } else {
            None
        }
    })
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("aztek_u10_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn text_scroll_list_items_enabled_and_tags() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout; textScrollList -a \"one\" -a \"two\" -a \"three\" -utg \"t1\" -utg \"t2\" -utg \"t3\" -el \"Nothing\" l;");
    assert_eq!(strs(&ok(&mut c, "textScrollList -q -ait l;")), vec!["t1", "t2", "t3"]);
    assert_eq!(ok(&mut c, "textScrollList -q -el l;"), json!("Nothing"));
    let _ = aztek_host::scriptui::take_ops();
    // Turned off by text, by index and by tag: the shell is told which rows.
    ok(&mut c, "textScrollList -e -ei \"two\" 0 l;");
    let ops = aztek_host::scriptui::take_ops();
    assert_eq!(nums(&shell_prop(&ops, "|l", "disabled").unwrap()), vec![1.0]);
    ok(&mut c, "textScrollList -e -eii 3 0 -eut \"t1\" 0 l;");
    let ops = aztek_host::scriptui::take_ops();
    assert_eq!(nums(&shell_prop(&ops, "|l", "disabled").unwrap()), vec![0.0, 1.0, 2.0]);
    ok(&mut c, "textScrollList -e -eii 2 1 l;");
    let ops = aztek_host::scriptui::take_ops();
    assert_eq!(nums(&shell_prop(&ops, "|l", "disabled").unwrap()), vec![0.0, 2.0]);
    // Rows that go take their state with them; the rest move up.
    ok(&mut c, "textScrollList -e -rii 1 l;");
    let ops = aztek_host::scriptui::take_ops();
    assert_eq!(nums(&shell_prop(&ops, "|l", "disabled").unwrap()), vec![1.0]);
    assert_eq!(strs(&ok(&mut c, "textScrollList -q -ait l;")), vec!["t2", "t3"]);
    ok(&mut c, "textScrollList -e -ea l;");
    let ops = aztek_host::scriptui::take_ops();
    assert_eq!(nums(&shell_prop(&ops, "|l", "disabled").unwrap()), Vec::<f64>::new());
    assert!(bad(&mut c, "textScrollList -e -eii 9 0 l;").contains("no item"));
    assert!(bad(&mut c, "textScrollList -e -ei \"nope\" 0 l;").contains("no item"));
    ok(&mut c, "textScrollList -e -el \"Empty\" l;");
    assert_eq!(ok(&mut c, "textScrollList -q -el l;"), json!("Empty"));
}

#[test]
fn tool_button_images_and_state() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout; toolCollection tc;");
    ok(&mut c, "toolButton -t moveSuperContext -t scaleSuperContext -ti1 moveSuperContext \"move_M.png\" -ti1 scaleSuperContext \"scale_M.png\" b;");
    assert_eq!(strs(&ok(&mut c, "toolButton -q -ti1 b;")), vec!["moveSuperContext", "move_M.png", "scaleSuperContext", "scale_M.png"]);
    assert_eq!(ok(&mut c, "toolButton -q -t b;"), json!("moveSuperContext"));
    assert_eq!(ok(&mut c, "toolButton -q -tc b;"), json!(2));
    // A tool already on the button becomes its current one.
    ok(&mut c, "toolButton -e -t scaleSuperContext b;");
    assert_eq!(ok(&mut c, "toolButton -q -t b;"), json!("scaleSuperContext"));
    assert_eq!(ok(&mut c, "toolButton -q -tc b;"), json!(2));
    ok(&mut c, "toolButton -e -ti2 scaleSuperContext \"s2.png\" -ti3 moveSuperContext \"m3.png\" b;");
    assert_eq!(strs(&ok(&mut c, "toolButton -q -ti3 b;")), vec!["moveSuperContext", "m3.png"]);
    // Replaced for the same tool.
    ok(&mut c, "toolButton -e -ti1 moveSuperContext \"other.png\" b;");
    assert_eq!(strs(&ok(&mut c, "toolButton -q -ti1 b;")), vec!["moveSuperContext", "other.png", "scaleSuperContext", "scale_M.png"]);
    assert_eq!(ok(&mut c, "toolButton -q -piv b;"), json!(true));
    ok(&mut c, "toolButton -e -piv 0 b;");
    assert_eq!(ok(&mut c, "toolButton -q -piv b;"), json!(false));
    assert_eq!(ok(&mut c, "toolButton -q -ssm b;"), json!(false));

    // The tools keep the button's state unless it manages its own.
    ok(&mut c, "toolButton -t moveSuperContext auto; toolButton -t moveSuperContext -ssm 1 own;");
    ok(&mut c, "setToolTo moveSuperContext;");
    c.idle();
    assert_eq!(ok(&mut c, "toolButton -q -sl auto;"), json!(true));
    assert_eq!(ok(&mut c, "toolButton -q -sl own;"), json!(false));
    assert_eq!(ok(&mut c, "toolButton -q -ssm own;"), json!(true));
    // A tool of several on a button made current elsewhere: the button follows it.
    ok(&mut c, "setToolTo scaleSuperContext;");
    c.idle();
    assert_eq!(ok(&mut c, "toolButton -q -sl b;"), json!(true));
    assert_eq!(ok(&mut c, "toolButton -q -t b;"), json!("scaleSuperContext"));
    ok(&mut c, "setToolTo moveSuperContext;");
    c.idle();
    assert_eq!(ok(&mut c, "toolButton -q -t b;"), json!("moveSuperContext"));
}

#[test]
fn tool_collection_clear_and_tool_dropped() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout col; toolCollection tc; toolButton -t moveSuperContext b1; toolButton -t scaleSuperContext b2;");
    ok(&mut c, "toolCollection -e -sl b2 tc;");
    assert_eq!(ok(&mut c, "toolCollection -q -sl tc;"), json!("b2"));
    ok(&mut c, "toolCollection -e -cl tc;");
    assert_eq!(ok(&mut c, "toolCollection -q -sl tc;"), json!("NONE"));
    assert_eq!(ok(&mut c, "toolButton -q -sl b2;"), json!(false));

    // A tool dropped: its button, in the current layout and collection.
    let before = strs(&ok(&mut c, "columnLayout -q -ca col;")).len();
    assert_eq!(ok(&mut c, "toolDropped selectSuperContext;"), Value::Null);
    let kids = strs(&ok(&mut c, "columnLayout -q -ca col;"));
    assert_eq!(kids.len(), before + 1);
    let made = kids.last().unwrap().clone();
    assert_eq!(ok(&mut c, &format!("toolButton -q -t {made};")), json!("selectSuperContext"));
    assert_eq!(ok(&mut c, "toolCollection -q -nci tc;"), json!(3));
    bad(&mut c, "toolDropped;");
}

#[test]
fn torus_query_and_edit() {
    let mut c = Core::new();
    let r = strs(&ok(&mut c, "torus -n tor -r 2 -hr 0.25;"));
    assert_eq!(r.len(), 2);
    close(num(&ok(&mut c, "torus -q -r tor;")), 2.0, "radius");
    close(num(&ok(&mut c, "torus -q -hr tor;")), 0.25, "height ratio");
    let before = nums(&ok(&mut c, "exactWorldBoundingBox tor;"));
    ok(&mut c, "torus -e -r 4 tor;");
    close(num(&ok(&mut c, "torus -q -r tor;")), 4.0, "radius edited");
    let after = nums(&ok(&mut c, "exactWorldBoundingBox tor;"));
    assert!(after[4] - after[1] > before[4] - before[1] + 3.0, "{before:?} {after:?}");
    assert_eq!(num(&ok(&mut c, "torus -q -nds tor;")), 0.0);
    ok(&mut c, "torus -e -nds 1 -cch 1 tor;");
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.nodeState;", r[1]))), 1.0);
    assert_eq!(ok(&mut c, "torus -q -cch tor;"), json!(true));
    // Made with them.
    let r2 = strs(&ok(&mut c, "torus -nds 2 -n t2;"));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.nodeState;", r2[1]))), 2.0);
}

#[test]
fn trim_and_untrim_query_and_edit() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n p -ax 0 1 0 -w 10 -lr 1 -d 1;");
    let on = ok(&mut c, "curveOnSurface -d 1 -uv 0.25 0.25 -uv 0.75 0.25 -uv 0.75 0.75 -uv 0.25 0.75 -uv 0.25 0.25 p;");
    let name = on.as_str().unwrap().to_string();
    let r = strs(&ok(&mut c, &format!("trim -lu 0.5 -lv 0.5 -tol 0.01 p \"{name}\";")));
    let node = r[1].clone();
    let shape = c.s.graph.find("pShape").unwrap().unwrap();
    let keep = |c: &mut Core| aztek_scene::surfaces::world_surface(&mut c.s.graph, shape).unwrap().trim[0].keep_odd;
    assert!(keep(&mut c));
    close(num(&ok(&mut c, &format!("trim -q -lu {node};"))), 0.5, "locator u");
    close(num(&ok(&mut c, &format!("trim -q -tol {node};"))), 0.01, "tolerance");
    assert_eq!(num(&ok(&mut c, &format!("trim -q -sl {node};"))), 0.0);
    // Throwing the region away instead: the inside goes.
    ok(&mut c, &format!("trim -e -sl 1 {node};"));
    assert!(!keep(&mut c));
    ok(&mut c, &format!("trim -e -sl 0 -lu 0.1 -lv 0.1 {node};"));
    assert!(!keep(&mut c), "the locator outside the loop keeps the outside");
    ok(&mut c, &format!("trim -e -lu 0.5 -lv 0.5 {node};"));
    assert!(keep(&mut c));
    // -shrink cuts the surface down to the loop.
    assert_eq!(ok(&mut c, &format!("trim -q -sh {node};")), json!(false));
    ok(&mut c, &format!("trim -e -sh 1 {node};"));
    let s = aztek_scene::surfaces::world_surface(&mut c.s.graph, shape).unwrap();
    let (lo, hi) = s.domain_u();
    assert!(lo > 0.2 && lo < 0.26 && hi > 0.74 && hi < 0.8, "{lo} {hi}");
    assert_eq!(ok(&mut c, &format!("trim -q -nds {node};")), json!(0));

    // untrim: all the trims, or with -untrimAll off only the last.
    ok(&mut c, &format!("trim -e -sh 0 {node};"));
    let on2 = ok(&mut c, "curveOnSurface -d 1 -uv 0.05 0.05 -uv 0.15 0.05 -uv 0.15 0.15 -uv 0.05 0.15 -uv 0.05 0.05 p;");
    ok(&mut c, &format!("trim -lu 0.5 -lv 0.5 p \"{}\";", on2.as_str().unwrap()));
    let count = |c: &mut Core| aztek_scene::surfaces::world_surface(&mut c.s.graph, shape).unwrap().trim.len();
    assert_eq!(count(&mut c), 2);
    let u = strs(&ok(&mut c, "untrim p;"));
    assert_eq!(count(&mut c), 0);
    assert_eq!(ok(&mut c, &format!("untrim -q -all {};", u[1])), json!(true));
    ok(&mut c, &format!("untrim -e -all 0 {};", u[1]));
    assert_eq!(count(&mut c), 1);
    ok(&mut c, &format!("untrim -e -nds 1 {};", u[1]));
    assert_eq!(count(&mut c), 2);
}

#[test]
fn track_up_distances() {
    let mut c = Core::new();
    ok(&mut c, "camera; setAttr camera1.t 0 0 0;");
    ok(&mut c, "track -u 2 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.ty;")), 2.0, "up 01");
    ok(&mut c, "track -up 3 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.ty;")), 5.0, "up 02");
    ok(&mut c, "track -upDistance01 -1 -upDistance02 -1 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.ty;")), 3.0, "both");
    ok(&mut c, "track -d 3 -r 1 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.ty;")), 0.0, "down");
    close(num(&ok(&mut c, "getAttr camera1.tx;")), 1.0, "right");
}

#[test]
fn transfer_attributes_query_and_edit() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n src; polySphere -n dst; move 0 0.5 0 src;");
    let r = strs(&ok(&mut c, "transferAttributes -pos 1 -spa 0 src dst;"));
    let node = r[0].clone();
    close(point(&mut c, "dst.vtx[0]")[1], point(&mut c, "src.vtx[0]")[1], "moved to the source");
    assert_eq!(ok(&mut c, &format!("transferAttributes -q -pos {node};")), json!(1));
    // Found from the mesh it feeds as well.
    assert_eq!(ok(&mut c, "transferAttributes -q -sm dst;"), json!(3));
    ok(&mut c, &format!("transferAttributes -e -pos 0 {node};"));
    assert_eq!(ok(&mut c, &format!("transferAttributes -q -pos {node};")), json!(0));
    close(point(&mut c, "dst.vtx[0]")[1], -1.0, "back where it was");
    let n = num(&ok(&mut c, "polyEvaluate -v dst;"));
    assert_eq!(nums(&ok(&mut c, &format!("transferAttributes -q -dui {node};"))), vec![0.0, n, n]);
    assert_eq!(strs(&ok(&mut c, &format!("transferAttributes -q -g {node};"))), vec!["dstShape"]);
    assert_eq!(nums(&ok(&mut c, &format!("transferAttributes -q -gi {node};"))), vec![0.0]);
    assert_eq!(strs(&ok(&mut c, &format!("transferAttributes -q -cmp {node};"))).len(), 1);
    ok(&mut c, "select -r dst.vtx[3:4];");
    assert_eq!(strs(&ok(&mut c, &format!("transferAttributes -q -cms {node};"))), vec!["dst.vtx[3:4]"]);
    assert!(bad(&mut c, "transferAttributes -q -pos src;").contains("no transferAttributes"));
}

#[test]
fn translator_rules_and_finding() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "translator -q -dfr \"FBX\";"), json!("FBX"));
    assert_eq!(ok(&mut c, "translator -q -ot mayaAscii;"), json!("mayaAscii"));
    assert_eq!(ok(&mut c, "translator -q -os mayaAscii;"), json!(""));
    assert_eq!(ok(&mut c, "translator -ft \"scene.ma\" 0;"), json!("mayaAscii"));
    assert_eq!(ok(&mut c, "translator -ft \"scene.mb\" 2;"), json!("mayaBinary"));
    assert_eq!(ok(&mut c, "translator -ft \"thing.obj\" 1;"), json!("OBJexport"));
    assert_eq!(ok(&mut c, "translator -ft \"thing.obj\" 0;"), json!("OBJ"));
    assert_eq!(ok(&mut c, "translator -ft \"thing.obj\" 2;"), json!(""));
    assert_eq!(ok(&mut c, "translator -ft \"thing.xyz\" 0;"), json!(""));
    assert_eq!(ok(&mut c, "translator -q -cmp mayaBinary;"), json!("uncompressed"));
    ok(&mut c, "translator -cmp \"compressed\" mayaBinary;");
    assert_eq!(ok(&mut c, "translator -q -cmp mayaBinary;"), json!("compressed"));
    bad(&mut c, "translator -cmp \"zipped\" mayaBinary;");
}

#[test]
fn undo_info_prints_queues() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polySphere -n b;");
    ok(&mut c, "undo;");
    assert_eq!(ok(&mut c, "undoInfo -q -prq;"), Value::Null);
    assert_eq!(ok(&mut c, "undoInfo -q -pq;"), Value::Null);
    c.idle();
    let lines: Vec<String> = c.log.iter().map(|l| l.text.clone()).filter(|t| t.starts_with("# ")).collect();
    assert!(lines.iter().any(|l| l.starts_with("# 0:") && l.contains("polySphere")), "{lines:?}");
    assert!(lines.iter().any(|l| l.contains("polyCube")), "{lines:?}");
}

#[test]
fn unfold_apply_to_shell() {
    // The second face's UVs pulled out of shape; the first face is unfolded.
    let run = |flag: &str| -> (f64, f64) {
        let mut c = Core::new();
        ok(&mut c, "polyPlane -w 2 -h 1 -sx 2 -sy 1 -n p; polyEditUV -r 1 -u 0.3 -v 0.2 p.map[5];");
        ok(&mut c, &format!("unfold {flag} p.f[0];"));
        (num(&ok(&mut c, "polyEvaluate -ufa p.f[0];")), num(&ok(&mut c, "polyEvaluate -ufa p.f[1];")))
    };
    // Only the face named: the other keeps its stretched area. The whole shell: both come out even.
    let (_, other) = run("");
    close(other, 0.7, "the other face kept");
    let (a, b) = run("-as 1");
    close(a, b, "the shell unfolded as one");
    close(b, 0.6, "even areas");
}

#[test]
fn unknown_node_class_tag() {
    let mut c = Core::new();
    let dir = temp_dir("unknown");
    let file = dir.join("u.ma");
    std::fs::write(&file, "//Maya ASCII 2027 scene\nrequires maya \"2027\";\nrequires \"absentPlug\" \"1.0\";\ncreateNode absentType -n \"u1\";\n").unwrap();
    ok(&mut c, &format!("file -f -o \"{}\";", file.to_string_lossy().replace('\\', "/")));
    assert_eq!(ok(&mut c, "unknownNode -q -rcn u1;"), json!("absentType"));
    // A text file has no class tag: 0.
    assert_eq!(ok(&mut c, "unknownNode -q -rct u1;"), json!(0));
}

#[test]
fn unload_plugin_callbacks() {
    let mut c = Core::new();
    ok(&mut c, "global string $gGone = \"\"; global proc onGone(string $p) { global string $gGone; $gGone = $gGone + $p + \";\"; }");
    ok(&mut c, "unloadPlugin -ac \"onGone\";");
    ok(&mut c, "loadPlugin quatNodes;");
    assert_eq!(strs(&ok(&mut c, "unloadPlugin quatNodes;")), vec!["quatNodes"]);
    assert_eq!(ok(&mut c, "global string $gGone; $gGone;"), json!("quatNodes;"));
    ok(&mut c, "unloadPlugin -rc \"onGone\";");
    ok(&mut c, "loadPlugin quatNodes; unloadPlugin quatNodes;");
    assert_eq!(ok(&mut c, "global string $gGone; $gGone;"), json!("quatNodes;"));
    assert!(bad(&mut c, "unloadPlugin noSuchPlug;").contains("not loaded"));
}

#[test]
fn uv_snapshot_uv_set() {
    let mut c = Core::new();
    let dir = temp_dir("snap");
    ok(&mut c, "polyPlane -w 1 -h 1 -sx 2 -sy 2 -n p; polyUVSet -cr -uvs \"second\" p;");
    let a = dir.join("a.png").to_string_lossy().replace('\\', "/");
    let b = dir.join("b.png").to_string_lossy().replace('\\', "/");
    ok(&mut c, &format!("uvSnapshot -n \"{a}\" -o -xr 64 -yr 64 p;"));
    ok(&mut c, &format!("uvSnapshot -n \"{b}\" -o -xr 64 -yr 64 -uvs \"second\" p;"));
    assert!(std::fs::read(&a).unwrap() != std::fs::read(&b).unwrap(), "the second set draws differently");
    ok(&mut c, &format!("uvSnapshot -n \"{b}\" -o -xr 64 -yr 64 -uvs \"map1\" p;"));
    assert_eq!(std::fs::read(&a).unwrap(), std::fs::read(&b).unwrap());
    assert!(bad(&mut c, &format!("uvSnapshot -n \"{b}\" -o -uvs \"nope\" p;")).contains("no UV set"));
}

#[test]
fn view_fit_clip_planes_and_children() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box -h 4; move 0 10 0 box;");
    ok(&mut c, "setAttr topShape.nearClipPlane 0.1; setAttr topShape.farClipPlane 100000;");
    ok(&mut c, "viewFit -all -fcp 1 top;");
    let eye = num(&ok(&mut c, "getAttr top.ty;"));
    let (near, far) = (num(&ok(&mut c, "getAttr topShape.nearClipPlane;")), num(&ok(&mut c, "getAttr topShape.farClipPlane;")));
    assert!(near <= eye - 12.0 + 1e-6 && near > eye - 12.5, "near {near} eye {eye}");
    assert!(far >= eye - 8.0 - 1e-6 && far < eye - 7.5, "far {far} eye {eye}");
    // Not given: the planes stay.
    ok(&mut c, "setAttr topShape.farClipPlane 5000; viewFit -all top;");
    close(num(&ok(&mut c, "getAttr topShape.farClipPlane;")), 5000.0, "far kept");

    // -noChildren: the parent alone, as a cube of its own size would fit.
    ok(&mut c, "polyCube -n lone; polyCube -n par; polyCube -n kid; move 20 0 0 kid; parent kid par;");
    ok(&mut c, "select -r lone; viewFit persp;");
    let alone = nums(&ok(&mut c, "getAttr persp.t;"));
    ok(&mut c, "select -r par; viewFit -noc persp;");
    let no_kids = nums(&ok(&mut c, "getAttr persp.t;"));
    ok(&mut c, "select -r par; viewFit persp;");
    let with_kids = nums(&ok(&mut c, "getAttr persp.t;"));
    for i in 0..3 {
        close(no_kids[i], alone[i], "fits as the parent alone");
    }
    assert!((with_kids[0] - alone[0]).abs() > 1.0, "{with_kids:?} {alone:?}");
}

#[test]
fn view_place_directions() {
    let mut c = Core::new();
    ok(&mut c, "camera;");
    ok(&mut c, "viewPlace -eye 0 0 10 -vd 0 0 -1 cameraShape1;");
    let r = nums(&ok(&mut c, "getAttr camera1.r;"));
    assert!(r.iter().all(|x| x.abs() < 1e-3), "{r:?}");
    close(num(&ok(&mut c, "getAttr camera1.tz;")), 10.0, "eye");
    ok(&mut c, "viewPlace -vd 1 0 0 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.ry;")).abs(), 90.0, "turned to look along x");
    // Up along x: the camera rolls a quarter turn.
    ok(&mut c, "viewPlace -eyePoint 0 0 10 -la 0 0 0 -upDirection 1 0 0 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.rz;")).abs(), 90.0, "rolled");
    ok(&mut c, "viewPlace -an 1 -eye 0 0 5 cameraShape1;");
    close(num(&ok(&mut c, "getAttr camera1.tz;")), 5.0, "animated place lands");
    assert!(bad(&mut c, "viewPlace -la 0 0 0 -vd 0 0 -1 cameraShape1;").contains("both"));
}

#[test]
fn view_set_back_and_queries() {
    let mut c = Core::new();
    ok(&mut c, "camera;");
    ok(&mut c, "viewSet -b cameraShape1;");
    // Looking along +z: the camera's own z axis points down -z.
    close(nums(&ok(&mut c, "xform -q -ws -m camera1;"))[10], -1.0, "back looks along +z");
    assert_eq!(ok(&mut c, "viewSet -q -pv cameraShape1;"), json!(true));
    assert_eq!(ok(&mut c, "viewSet -q -nv cameraShape1;"), json!(false));
    let back = nums(&ok(&mut c, "getAttr camera1.t;"));
    ok(&mut c, "viewSet -t cameraShape1;");
    let top = nums(&ok(&mut c, "getAttr camera1.t;"));
    ok(&mut c, "viewSet -pv cameraShape1;");
    let now = nums(&ok(&mut c, "getAttr camera1.t;"));
    for i in 0..3 {
        close(now[i], back[i], "previous view");
    }
    assert_eq!(ok(&mut c, "viewSet -q -nv cameraShape1;"), json!(true));
    ok(&mut c, "viewSet -nv cameraShape1;");
    let again = nums(&ok(&mut c, "getAttr camera1.t;"));
    for i in 0..3 {
        close(again[i], top[i], "next view");
    }
    assert_eq!(ok(&mut c, "viewSet -q -fit cameraShape1;"), json!(false));
    ok(&mut c, "viewSet -p -fit cameraShape1;");
    assert_eq!(ok(&mut c, "viewSet -q -fit cameraShape1;"), json!(true));
    ok(&mut c, "viewSet -krs 1 -f cameraShape1;");
    assert_eq!(ok(&mut c, "viewSet -q -krs cameraShape1;"), json!(true));
}

#[test]
fn window_menu_bar_flags() {
    let mut c = Core::new();
    ok(&mut c, "window -mb 1 w; menu -l \"A\" ma; menu -l \"B\" mbb; menu -l \"C\" mc; columnLayout; button bt; text tx;");
    assert_eq!(ok(&mut c, "window -q -nm w;"), json!(3));
    ok(&mut c, "window -e -mi \"mc\" 0 w;");
    assert_eq!(strs(&ok(&mut c, "window -q -ma w;")), vec!["mc", "ma", "mbb"]);
    ok(&mut c, "window -e -mi \"mc\" 2 w;");
    assert_eq!(strs(&ok(&mut c, "window -q -ma w;")), vec!["ma", "mbb", "mc"]);
    assert!(bad(&mut c, "window -e -mi \"zz\" 0 w;").contains("no menu"));
    ok(&mut c, "window -e -mcw bt \"topRight\" w;");
    ok(&mut c, "window -e -mcw tx \"bottomLeft\" -mbr w;");
    assert_eq!(ok(&mut c, "window -q -mcw w;"), json!(["", "bt", "tx", ""]));
    assert!(bad(&mut c, "window -e -mcw bt \"middle\" w;").contains("corner"));
    assert_eq!(ok(&mut c, "window -q -mm w;"), json!(false));
    ok(&mut c, "window -mm 1 w2;");
    assert_eq!(ok(&mut c, "window -q -mainMenuBar w2;"), json!(true));
    ok(&mut c, "window -ds 1 w3;");
    assert_eq!(ok(&mut c, "window -q -dockStation w3;"), json!(true));
    // A menu bar layout's menus move the same way.
    ok(&mut c, "window w4; menuBarLayout mbl; menu -l \"X\" mx; menu -l \"Y\" my;");
    ok(&mut c, "menuBarLayout -e -mi \"my\" 0 mbl;");
    assert_eq!(strs(&ok(&mut c, "menuBarLayout -q -ma mbl;")), vec!["my", "mx"]);
}

#[test]
fn workspace_control_flags() {
    let mut c = Core::new();
    ok(&mut c, "global string $gInit = \"\"; global int $gDir = -1;");
    ok(&mut c, "global proc wcInit(string $n) { global string $gInit; $gInit = $n; }");
    ok(&mut c, "global proc wcDir(int $h) { global int $gDir; $gDir = $h; }");
    ok(&mut c, "workspaceControl -l \"W\" -ic \"wcInit\" -ldc \"wcDir\" wc;");
    assert_eq!(ok(&mut c, "global string $gInit; $gInit;"), json!("wc"));
    assert_eq!(ok(&mut c, "workspaceControl -q -ic wc;"), json!("wcInit"));
    assert_eq!(ok(&mut c, "workspaceControl -q -ldc wc;"), json!("wcDir"));
    ok(&mut c, "workspaceControl -e -hr 1 wc;");
    assert_eq!(ok(&mut c, "global int $gDir; $gDir;"), json!(1));
    ok(&mut c, "workspaceControl -e -hr 0 wc;");
    assert_eq!(ok(&mut c, "global int $gDir; $gDir;"), json!(0));
    assert_eq!(ok(&mut c, "workspaceControl -q -alm wc;"), json!(false));
    ok(&mut c, "workspaceControl -e -alm 1 wc;");
    assert_eq!(ok(&mut c, "workspaceControl -q -actLikeMayaUIElement wc;"), json!(true));
    ok(&mut c, "workspaceControl -e -rsw 300 -rsh 200 wc;");
    assert_eq!(ok(&mut c, "workspaceControl -q -w wc;"), json!(300));
    assert_eq!(ok(&mut c, "workspaceControl -q -h wc;"), json!(200));
    assert_eq!(ok(&mut c, "workspaceControl -q -tp wc;"), json!("north"));
    ok(&mut c, "workspaceControl -e -tp \"west\" 1 wc;");
    assert_eq!(ok(&mut c, "workspaceControl -q -tp wc;"), json!("west"));
    bad(&mut c, "workspaceControl -e -tp \"south\" 0 wc;");
    // Closed: kept hidden (retained by default), its size kept as its state.
    assert_eq!(ok(&mut c, "workspaceControl -q -vis wc;"), json!(true));
    ok(&mut c, "workspaceControl -e -cl wc;");
    assert_eq!(ok(&mut c, "workspaceControl -q -vis wc;"), json!(false));
    assert_eq!(ok(&mut c, "workspaceControl -ex wc;"), json!(true));
    assert_eq!(ok(&mut c, "workspaceControlState -ex wc;"), json!(true));
    assert_eq!(ok(&mut c, "workspaceControlState -q -w wc;"), json!(300));
    // Not retained: closing deletes it.
    ok(&mut c, "workspaceControl -rt 0 gone;");
    ok(&mut c, "workspaceControl -e -close gone;");
    assert_eq!(ok(&mut c, "workspaceControl -ex gone;"), json!(false));
}

#[test]
fn workspace_control_state_kept() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "workspaceControlState -ex s1;"), json!(false));
    ok(&mut c, "workspaceControlState -wh 400 300 -tlc 10 20 s1;");
    assert_eq!(ok(&mut c, "workspaceControlState -ex s1;"), json!(true));
    assert_eq!(ok(&mut c, "workspaceControlState -q -w s1;"), json!(400));
    assert_eq!(ok(&mut c, "workspaceControlState -q -h s1;"), json!(300));
    assert_eq!(ok(&mut c, "workspaceControlState -q -wh s1;"), json!([400, 300]));
    assert_eq!(ok(&mut c, "workspaceControlState -q -tlc s1;"), json!([10, 20]));
    assert_eq!(ok(&mut c, "workspaceControlState -q -le s1;"), json!(20));
    ok(&mut c, "workspaceControlState -e -max 1 -dc 5 6 -dwh 50 60 -w 410 s1;");
    assert_eq!(ok(&mut c, "workspaceControlState -q -max s1;"), json!(true));
    assert_eq!(ok(&mut c, "workspaceControlState -q -dc s1;"), json!([5, 6]));
    assert_eq!(ok(&mut c, "workspaceControlState -q -dwh s1;"), json!([50, 60]));
    assert_eq!(ok(&mut c, "workspaceControlState -q -width s1;"), json!(410));
    // A workspace control of that name is made at the size kept.
    ok(&mut c, "workspaceControl -l \"S\" s1;");
    assert_eq!(ok(&mut c, "workspaceControl -q -w s1;"), json!(410));
    assert_eq!(ok(&mut c, "workspaceControl -q -h s1;"), json!(300));
    ok(&mut c, "workspaceControlState -r s1;");
    assert_eq!(ok(&mut c, "workspaceControlState -ex s1;"), json!(false));
    bad(&mut c, "workspaceControlState -q -w s1;");
}

#[test]
fn workspace_layout_manager_layouts() {
    let prefs = temp_dir("layouts");
    std::env::set_var("AZTEK_PREFS", &prefs);
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "workspaceLayoutManager -lul;"), json!([]));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -lml;"), json!([]));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -t \"General\";"), json!("FACTORY"));
    bad(&mut c, "workspaceLayoutManager -t \"Nope\";");
    ok(&mut c, "global string $gCur = \"\"; global int $gMod = 0;");
    ok(&mut c, "global proc onCur(string $n) { global string $gCur; $gCur = $n; }");
    ok(&mut c, "global proc onMod() { global int $gMod; $gMod++; }");
    ok(&mut c, "workspaceLayoutManager -scc \"onCur\"; workspaceLayoutManager -smc \"onMod\";");
    ok(&mut c, "workspaceLayoutManager -sa \"Mine\";");
    assert!(prefs.join("workspaces").join("Mine.json").is_file());
    assert_eq!(ok(&mut c, "workspaceLayoutManager -lul;"), json!(["Mine"]));
    assert!(strs(&ok(&mut c, "workspaceLayoutManager -ll;")).contains(&"Mine".to_string()));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -t \"Mine\";"), json!("USER"));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -q -cu;"), json!("Mine"));
    // Changing a workspace control modifies the current layout; saving clears it.
    assert_eq!(ok(&mut c, "workspaceLayoutManager -m \"Mine\";"), json!(false));
    ok(&mut c, "workspaceControl -l \"P\" pane1; workspaceControl -e -fl 0 pane1;");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -m \"Mine\";"), json!(true));
    assert_eq!(ok(&mut c, "global int $gMod; $gMod;"), json!(1));
    ok(&mut c, "workspaceLayoutManager -s;");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -m \"Mine\";"), json!(false));
    assert_eq!(ok(&mut c, "global int $gMod; $gMod;"), json!(2));
    ok(&mut c, "workspaceLayoutManager -sc \"General\";");
    assert_eq!(ok(&mut c, "global string $gCur; $gCur;"), json!("General"));
    // Saving over a factory layout overrides it; deleting the override puts it back.
    ok(&mut c, "workspaceLayoutManager -sa \"General\";");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -t \"General\";"), json!("FACTORY_OVERRIDE"));
    ok(&mut c, "workspaceLayoutManager -d \"General\";");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -t \"General\";"), json!("FACTORY"));
    ok(&mut c, "workspaceLayoutManager -d \"Mine\";");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -lul;"), json!([]));
    bad(&mut c, "workspaceLayoutManager -d \"Mine\";");
    // Imported from a file.
    let f = prefs.join("Brought.json");
    std::fs::write(&f, "{\"name\": \"Brought\"}").unwrap();
    ok(&mut c, &format!("workspaceLayoutManager -i \"{}\";", f.to_string_lossy().replace('\\', "/")));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -lul;"), json!(["Brought"]));
    // The workspace control a control is in.
    ok(&mut c, "workspaceControl -l \"Q\" -ui \"\" wq; columnLayout -p wq inner; button -p inner deep;");
    assert_eq!(ok(&mut c, "workspaceLayoutManager -pwc \"deep\";"), json!("wq"));
    assert_eq!(ok(&mut c, "workspaceLayoutManager -pwc \"nothing\";"), json!(""));
    ok(&mut c, "workspaceLayoutManager -cmw \"Outliner\" 1;");
    ok(&mut c, "workspaceLayoutManager -rmw;");
}

#[test]
fn texture_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -n s; polyCube -n hidden; hide hidden;");
    let r = strs(&ok(&mut c, "select -r s hidden; textureDeformer -envelope 0.8 -strength 1.0 -offset 1.0 -direction \"Normal\" -pointSpace \"World\" -ex \"texPart\";"));
    let node = r[0].clone();
    assert_eq!(ok(&mut c, &format!("getAttr {node}.direction;")), json!(0));
    assert_eq!(ok(&mut c, &format!("getAttr {node}.pointSpace;")), json!(0));
    // Hidden objects left out without -ihs.
    assert_eq!(strs(&ok(&mut c, &format!("textureDeformer -q -g {node};"))), vec!["sShape"]);
    assert_eq!(ok(&mut c, &format!("textureDeformer -q -ex {node};")), json!("texPart"));
    let n = num(&ok(&mut c, "polyEvaluate -v s;"));
    assert_eq!(nums(&ok(&mut c, &format!("textureDeformer -q -dui {node};"))), vec![0.0, n, n]);
    assert_eq!(strs(&ok(&mut c, &format!("textureDeformer -q -dt {node};"))).len(), 1);
    assert_eq!(nums(&ok(&mut c, &format!("textureDeformer -q -gi {node};"))), vec![0.0]);
    assert_eq!(strs(&ok(&mut c, &format!("textureDeformer -q -cmp {node};"))).len(), 1);
    // Geometry added and taken off.
    ok(&mut c, "polyCube -n more;");
    ok(&mut c, &format!("textureDeformer -e -g more {node};"));
    assert_eq!(strs(&ok(&mut c, &format!("textureDeformer -q -g {node};"))).len(), 2);
    ok(&mut c, &format!("textureDeformer -e -rm -g more {node};"));
    assert_eq!(strs(&ok(&mut c, &format!("textureDeformer -q -g {node};"))), vec!["sShape"]);
    // -ihs takes the hidden one; -uct leaves no set.
    let r2 = strs(&ok(&mut c, "textureDeformer -ihs -uct -d \"Vector\" -vsp \"Tangent\" hidden;"));
    assert_eq!(ok(&mut c, &format!("getAttr {}.direction;", r2[0])), json!(2));
    assert_eq!(ok(&mut c, &format!("getAttr {}.vectorSpace;", r2[0])), json!(2));
    assert_eq!(ok(&mut c, &format!("listConnections -type objectSet {};", r2[0])), json!([]));
    bad(&mut c, "textureDeformer -d \"Sideways\" s;");
    bad(&mut c, "select -cl; textureDeformer -is;");
}

#[test]
fn wire_holder_count_and_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -w 10 -h 10 -sx 10 -sy 10 -n pa; polyPlane -w 10 -h 10 -sx 10 -sy 10 -n pb;");
    ok(&mut c, "curve -d 1 -p -5 0 0 -p 5 0 0 -n w1; curve -d 1 -p -5 0 3 -p 5 0 3 -n w2; curve -d 1 -p -5 0 2 -p 5 0 2 -n h;");
    let a = strs(&ok(&mut c, "wire -w w1 -dds 0 5 -n wa pa;"))[0].clone();
    let b = strs(&ok(&mut c, "wire -w w1 -w w2 -dds 0 5 -dds 1 5 -n wb -ex \"wirePart\" pb;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("wire -q -wc {b};")), json!(2));
    ok(&mut c, &format!("wire -e -wc 1 {b};"));
    assert_eq!(ok(&mut c, &format!("wire -q -wc {b};")), json!(1));
    assert_eq!(strs(&ok(&mut c, &format!("wire -q -w {b};"))), vec!["w1"]);
    ok(&mut c, &format!("wire -e -ho 0 h {b};"));
    assert_eq!(strs(&ok(&mut c, &format!("wire -q -ho {b};"))), vec!["h"]);
    assert!(bad(&mut c, &format!("wire -e -ho 3 h {b};")).contains("no wire"));
    assert_eq!(ok(&mut c, &format!("wire -q -ex {b};")), json!("wirePart"));
    let n = num(&ok(&mut c, "polyEvaluate -v pa;"));
    assert_eq!(nums(&ok(&mut c, &format!("wire -q -dui {a};"))), vec![0.0, n, n]);
    assert_eq!(nums(&ok(&mut c, &format!("wire -q -gi {a};"))), vec![0.0]);
    assert_eq!(strs(&ok(&mut c, &format!("wire -q -g {a};"))), vec!["paShape"]);
    let tools = strs(&ok(&mut c, &format!("wire -q -dt {b};")));
    assert!(tools.contains(&"w1".to_string()) && tools.contains(&"h".to_string()), "{tools:?}");

    // The holder bounds the effect: halfway between the wire and the holder, half of it.
    let find = |c: &mut Core, mesh: &str, x: f64, z: f64| -> String {
        for i in 0..121 {
            let p = point(c, &format!("{mesh}.vtx[{i}]"));
            if (p[0] - x).abs() < 1e-6 && (p[2] - z).abs() < 1e-6 {
                return format!("{mesh}.vtx[{i}]");
            }
        }
        panic!("no point at {x} {z}");
    };
    let (va, vb) = (find(&mut c, "pa", 0.0, 1.0), find(&mut c, "pb", 0.0, 1.0));
    let (ha, hb) = (find(&mut c, "pa", 0.0, 2.0), find(&mut c, "pb", 0.0, 2.0));
    ok(&mut c, "move -r 0 1 0 w1;");
    let (ya, yb) = (point(&mut c, &va)[1], point(&mut c, &vb)[1]);
    assert!(ya > 0.1, "{ya}");
    close(yb, ya * 0.5, "half with the holder");
    assert!(point(&mut c, &ha)[1] > 0.05);
    close(point(&mut c, &hb)[1], 0.0, "none at the holder");

    // Geometry added by edit; -uct leaves no set.
    ok(&mut c, "polyPlane -n pc;");
    ok(&mut c, &format!("wire -e -g pc {a};"));
    assert_eq!(strs(&ok(&mut c, &format!("wire -q -g {a};"))).len(), 2);
    let d = strs(&ok(&mut c, "polyPlane -n pd; wire -uct -w w2 pd;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("listConnections -type objectSet {d};")), json!([]));
}

#[test]
fn wrinkle_shape_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -w 10 -h 10 -sx 20 -sy 20 -n wp;");
    let r = strs(&ok(&mut c, "wrinkle -ct 2 0 1 -ax 1 0 0 wp;"));
    let (node, handle) = (r[0].clone(), r[1].clone());
    let t = nums(&ok(&mut c, &format!("getAttr {handle}.t;")));
    close(t[0], 2.0, "centre x");
    close(t[2], 1.0, "centre z");
    // The handle's up turns to the axis.
    let up = nums(&ok(&mut c, &format!("xform -q -ws -m {handle};")));
    close(up[4], 1.0, "up along x");
    ok(&mut c, &format!("delete {handle};"));
    ok(&mut c, "delete -ch wp;");

    // Branching changes the creases.
    let shape = |branches: &str| -> Vec<f64> {
        let mut c = Core::new();
        ok(&mut c, "polyPlane -w 10 -h 10 -sx 20 -sy 20 -n wp;");
        ok(&mut c, &format!("wrinkle -st \"radial\" -wc 3 -dds 5 {branches} wp;"));
        (0..441).map(|i| point(&mut c, &format!("wp.vtx[{i}]"))[1]).collect()
    };
    let plain = shape("-bd 0");
    let branched = shape("-brc 3 -bd 2");
    assert!(plain.iter().zip(&branched).any(|(a, b)| (a - b).abs() > 1e-3));
    let r = strs(&ok(&mut c, "wrinkle -brc 3 -bd 2 wp;"));
    assert_eq!(ok(&mut c, &format!("getAttr {}.branchCount;", r[0])), json!(3));
    assert_eq!(ok(&mut c, &format!("getAttr {}.branchDepth;", r[0])), json!(2));
    let _ = node;

    // Creases along a curve given: points on it sink, those far off stay.
    ok(&mut c, "polyPlane -w 10 -h 10 -sx 20 -sy 20 -n wc;");
    ok(&mut c, "curve -d 1 -p -5 0 0 -p 5 0 0 -n cr;");
    ok(&mut c, "wrinkle -cr cr -dds 1 -wi 0.5 wc;");
    let mut sunk = false;
    let mut far_moved = false;
    for i in 0..441 {
        let p = point(&mut c, &format!("wc.vtx[{i}]"));
        if p[2].abs() < 1e-6 && p[1] < -0.1 {
            sunk = true;
        }
        if p[2].abs() > 2.0 && p[1].abs() > 1e-6 {
            far_moved = true;
        }
    }
    assert!(sunk);
    assert!(!far_moved);
    bad(&mut c, "wrinkle -cr nothingHere wc;");

    // A UV patch: points outside it stay.
    ok(&mut c, "polyPlane -w 10 -h 10 -sx 20 -sy 20 -n wu;");
    ok(&mut c, "wrinkle -uv 0.2 0.2 0 0.5 0.5 -dds 2 wu;");
    let mut inside = false;
    for i in 0..441 {
        let p = point(&mut c, &format!("wu.vtx[{i}]"));
        if (p[0].abs() > 2.6 || p[2].abs() > 2.6) && p[1].abs() > 1e-6 {
            panic!("moved outside the patch: {p:?}");
        }
        if p[1].abs() > 1e-4 {
            inside = true;
        }
    }
    assert!(inside);
    bad(&mut c, "wrinkle -uv 1 2 3 wu;");
}

#[test]
fn xform_euler_parent_space_and_reflection() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n b;");
    ok(&mut c, "xform -ro 30 0 0 a; xform -r -eu -ro 0 30 0 a;");
    assert_eq!(nums(&ok(&mut c, "getAttr a.r;")).iter().map(|x| (x * 1000.0).round() / 1000.0).collect::<Vec<_>>(), vec![30.0, 30.0, 0.0]);
    // Without -euler the object turns by the angles: about its own axes, so the result isn't the sum.
    ok(&mut c, "xform -ro 30 0 0 b; xform -r -ro 0 30 0 b;");
    let r = nums(&ok(&mut c, "getAttr b.r;"));
    assert!((r[0] - 30.0).abs() + (r[1] - 30.0).abs() + r[2].abs() > 1.0, "{r:?}");
    // The same turn as a matrix: a then turned about its own y.
    ok(&mut c, "polyCube -n m; xform -ro 30 0 0 m; rotate -r -os 0 30 0 m;");
    let (mb, mm) = (nums(&ok(&mut c, "xform -q -ws -m b;")), nums(&ok(&mut c, "xform -q -ws -m m;")));
    for i in 0..16 {
        close(mb[i], mm[i], "turned in object space");
    }

    // -parentSpace: pivots and bounds through the object's own matrix.
    ok(&mut c, "polyCube -n p; xform -rp 1 0 0 p; setAttr p.t 5 0 0;");
    close(nums(&ok(&mut c, "xform -q -ps -rp p;"))[0], 6.0, "pivot in parent space");
    close(nums(&ok(&mut c, "xform -q -rp p;"))[0], 1.0, "pivot in object space");
    ok(&mut c, "xform -ps -rp 7 0 0 p;");
    close(nums(&ok(&mut c, "xform -q -rp p;"))[0], 2.0, "set in parent space");
    let bb = nums(&ok(&mut c, "xform -q -ps -bb p;"));
    close(bb[0], 4.5, "box min");
    close(bb[3], 5.5, "box max");
    let bb = nums(&ok(&mut c, "xform -q -bb p;"));
    close(bb[0], -0.5, "object box min");

    // -reflection: the mirrored point moves too.
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 2 -sy 2 -n rp;");
    let find = |c: &mut Core, x: f64, z: f64| -> String {
        for i in 0..9 {
            let p = point(c, &format!("rp.vtx[{i}]"));
            if (p[0] - x).abs() < 1e-6 && (p[2] - z).abs() < 1e-6 {
                return format!("rp.vtx[{i}]");
            }
        }
        panic!("no point at {x} {z}");
    };
    let (left, right) = (find(&mut c, -2.0, 2.0), find(&mut c, 2.0, 2.0));
    ok(&mut c, &format!("xform -r -t 0 1 0 -rfl -rax {left};"));
    close(point(&mut c, &left)[1], 1.0, "moved");
    close(point(&mut c, &right)[1], 1.0, "its mirror moved");
    // Without it, only the one named.
    ok(&mut c, &format!("xform -r -t 0 1 0 {left};"));
    close(point(&mut c, &left)[1], 2.0, "moved again");
    close(point(&mut c, &right)[1], 1.0, "mirror left");
    // About the bounding box of an object off the origin, and with a tolerance.
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 2 -sy 2 -n offp; move 10 0 0 offp;");
    ok(&mut c, "xform -r -t 0 1 0 -rfl -rab -rax -rft 0.01 offp.vtx[0];");
    let moved: usize = (0..9).filter(|i| point(&mut c, &format!("offp.vtx[{i}]"))[1] > 0.5).count();
    assert_eq!(moved, 2);

    // -preserveUV: the UVs of the point moved follow.
    ok(&mut c, "polyPlane -w 4 -h 4 -sx 2 -sy 2 -n uvp;");
    let map = strs(&ok(&mut c, "polyListComponentConversion -fv -tuv uvp.vtx[4];"))[0].clone();
    let before = nums(&ok(&mut c, &format!("polyEditUV -q {map};")));
    ok(&mut c, "xform -r -t 0.5 0 0 uvp.vtx[4];");
    close(nums(&ok(&mut c, &format!("polyEditUV -q {map};")))[0], before[0], "UV stays without it");
    ok(&mut c, "xform -r -t 0.5 0 0 -puv uvp.vtx[4];");
    let after = nums(&ok(&mut c, &format!("polyEditUV -q {map};")))[0];
    assert!(after > before[0] + 0.1 && after < before[0] + 0.2, "UV follows the move: {after}");
}

#[test]
fn every_flag_of_the_unit_is_there() {
    let c = Core::new();
    for (cmd, flags) in [
        ("textScrollList", "ait allItemsUniqueTags ea ei eii el emptyLabel enableAll enableIndexedItem enableItem enableUniqueTagItem eut"),
        ("textureDeformer", "afterReference ar cmp cms components deformerTools dt dui dumpInfo ex exclusive g geometry geometryIndices gi ignoreSelected ihs includeHiddenSelections par parallel remove rm selectedComponents sp split uct useComponentTags"),
        ("toolButton", "piv ssm stateSelfManaged ti1 ti2 ti3 toolImage1 toolImage2 toolImage3"),
        ("toolCollection", "cl clear"),
        ("toolDropped", ""),
        ("torus", "caching cch e edit nds nodeState q query"),
        ("track", "upDistance01 upDistance02"),
        ("transferAttributes", "dui dumpInfo e edit q query"),
        ("translator", "cmp defaultFileRule dfr fileCompression findTranslator ft objectType optionsScript os ot"),
        ("trim", "e edit q query"),
        ("undoInfo", "printRedoQueue prq"),
        ("unfold", "applyToShell"),
        ("unknownNode", "rct realClassTag"),
        ("unloadPlugin", "ac addCallback rc removeCallback"),
        ("untrim", "all caching cch e edit nds nodeState q query untrimAll"),
        ("uvSnapshot", "uvSetName uvs"),
        ("viewFit", "fcp fitClipPlanes noChildren noc"),
        ("viewPlace", "an animate eyePoint upDirection vd viewDirection"),
        ("viewSet", "b q query"),
        ("window", "ds mainMenuBar mbr mcw menuBarCornerWidget menuBarResize menuIndex mi mm nm numberOfMenus"),
        ("wire", "afterReference ar cmp cms components deformerTools dt dui dumpInfo ex exclusive g geometry geometryIndices gi ho holder ignoreSelected ihs includeHiddenSelections par parallel remove rm selectedComponents sp split uct useComponentTags wc wireCount"),
        ("workspaceControl", "alm cl close ic initCallback layoutDirectionCallback ldc resizeHeight resizeWidth rsh rsw rt tabPosition tp"),
        ("workspaceControlState", "dc h height max maximized w width"),
        ("workspaceLayoutManager", "cmw collapseMainWindowControls d delete i listModuleLayouts listUserLayouts lml lul m modified parentWorkspaceControl pwc restoreMainWindowControls rmw s sa save saveAs scc setCurrentCallback setModifiedCallback smc t type"),
        ("wrinkle", "ax axis bd branchCount branchDepth brc center cr crease ct uv uvSpace"),
        ("xform", "euler parentSpace preserveUV ps puv rao rax ray raz reflectionAboutBBox reflectionAboutOrigin reflectionAboutX reflectionAboutY reflectionAboutZ reflectionTolerance rft"),
    ] {
        let def = c.s.commands.find(cmd).unwrap_or_else(|| panic!("no command {cmd}"));
        for flag in flags.split_whitespace() {
            assert!(def.flags.iter().any(|f| f.name == flag || f.short_name == flag), "{cmd} -{flag}");
        }
    }
}
