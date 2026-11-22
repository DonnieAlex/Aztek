//! Commands of the reference from dbmessage to fcheck: dbmessage, dbpeek, dbtrace, defineDataServer, deformerEvaluator,
//! deleteExtension, dgValidateCurve, dgfilter, dgmodified, disableIncorrectNameWarning, displayPref's -gf and -mhr,
//! displayString, evaluator, event, exportEdits and fcheck.

use aztek_host::{take_ui_requests_for_tests as take_ui_requests, Core};
use serde_json::{json, Value};

/// The lines commands print go to one process-wide buffer, and the first core to read it takes them: the tests that
/// read what was printed take turns.
static TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn turn() -> std::sync::MutexGuard<'static, ()> {
    TURN.lock().unwrap_or_else(|e| e.into_inner())
}

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
        Value::String(s) => s.parse().unwrap(),
        x => x.as_f64().unwrap_or_else(|| panic!("a number: {x}")),
    }
}

fn truth(v: &Value) -> bool {
    num(v) != 0.0
}

fn strs(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().map(|x| x.as_str().map(str::to_string).unwrap_or_else(|| x.to_string())).collect(),
        Value::String(s) => vec![s.clone()],
        Value::Null => Vec::new(),
        x => vec![x.to_string()],
    }
}

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(a) if a.len() == 1 => text(&a[0]),
        x => x.to_string(),
    }
}

/// The lines printed since the last call.
fn printed(c: &mut Core) -> Vec<String> {
    let lines = c.take_log();
    lines.as_array().unwrap().iter().filter(|l| l["kind"] != json!(0)).map(|l| l["text"].as_str().unwrap().to_string()).filter(|t| !t.starts_with("// Result")).collect()
}

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("aztek_u14_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn path(p: &std::path::Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

#[test]
fn dgfilter_defines_combines_and_tests_filters() {
    let _turn = turn();
    let mut c = Core::new();
    // The reference's own example.
    assert_eq!(text(&ok(&mut c, "dgfilter -atr translateX -n TX;")), "TX");
    assert_eq!(text(&ok(&mut c, "dgfilter -plug node.ty -n TY;")), "TY");
    assert_eq!(text(&ok(&mut c, "dgfilter -n OR -logicalOr TX TY;")), "OR");
    ok(&mut c, "polyCube; polyCube;");
    assert!(truth(&ok(&mut c, "dgfilter -n TX pCube1.tx;")));
    assert!(truth(&ok(&mut c, "dgfilter -n TX pCube1.translateX;")), "long and short names both match");
    assert!(truth(&ok(&mut c, "dgfilter -n OR pCube1.tx;")));
    assert!(!truth(&ok(&mut c, "dgfilter -n OR pCube1.tz;")));
    assert!(!truth(&ok(&mut c, "dgfilter -n TY pCube2.ty;")), "wrong node");
    assert_eq!(strs(&ok(&mut c, "dgfilter -list;")), vec!["OR", "TX", "TY"]);
    assert_eq!(text(&ok(&mut c, "dgfilter -list -n OR;")), "logicalOr( Plug(node.ty), Attribute(tx) )");
    // Node, node type, and, not; several objects pass only if all do.
    ok(&mut c, "dgfilter -nd \"pCube*\" -n CUBES; dgfilter -nt mesh -n MESH; dgfilter -n NOTMESH -logicalNot MESH; dgfilter -n BOTH -logicalAnd CUBES NOTMESH;");
    assert!(truth(&ok(&mut c, "dgfilter -n CUBES pCube1 pCube2;")));
    assert!(!truth(&ok(&mut c, "dgfilter -n CUBES pCube1 polyCube1;")));
    assert!(truth(&ok(&mut c, "dgfilter -n MESH pCubeShape1;")));
    assert!(truth(&ok(&mut c, "dgfilter -n BOTH pCube1;")));
    assert!(!truth(&ok(&mut c, "dgfilter -n BOTH pCubeShape1;")));
    assert_eq!(text(&ok(&mut c, "dgfilter -list -n NOTMESH;")), "logicalNot( NodeType(mesh) )");
    // A filter with nothing in it passes everything, as an invalid one does.
    assert!(truth(&ok(&mut c, "dgfilter -n nosuch pCube1.tz;")));
    bad(&mut c, "dgfilter -list -n missing;");
}

#[test]
fn dbmessage_monitors_messages_into_a_file() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("msg");
    let file = path(&d.join("msgs.txt"));
    assert_eq!(strs(&ok(&mut c, "dbmessage -t dgNodeAdded -m on;")), vec!["dgNodeAdded"]);
    ok(&mut c, &format!("dbmessage -f \"{file}\";"));
    ok(&mut c, "polySphere -n ball;");
    // More types: the list grows; off stops them.
    let now = strs(&ok(&mut c, "dbmessage -t Connection -m on;"));
    assert!(now.contains(&"dgConnectionMade".to_string()) && now.contains(&"dgNodeAdded".to_string()), "{now:?}");
    ok(&mut c, "createNode transform -n tgt; connectAttr ball.tx tgt.tx;");
    assert!(strs(&ok(&mut c, "dbmessage -m off;")).is_empty());
    ok(&mut c, "polyCube -n after;");
    let out = std::fs::read_to_string(&file).unwrap();
    assert!(out.contains("dgNodeAdded: ball (transform)"), "{out}");
    assert!(out.contains("dgNodeAdded: ballShape (mesh)"), "{out}");
    assert!(out.contains("dgConnectionMade: ball.translateX -> tgt.translateX"), "{out}");
    assert!(!out.contains("after"), "monitoring stopped: {out}");
    bad(&mut c, "dbmessage -t nothingLikeThis -m on;");
    // -l lists every type with its state.
    printed(&mut c);
    ok(&mut c, "dbmessage -l;");
}

#[test]
fn dbtrace_turns_traces_on_and_sends_them_out() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("trace");
    let file = path(&d.join("trace.txt"));
    ok(&mut c, "polyCube -n box;");
    assert!(strs(&ok(&mut c, "dbtrace -q;")).is_empty(), "nothing on at first");
    ok(&mut c, &format!("dbtrace -k compute -k setAttr -o \"{file}\";"));
    // -o alone changes only where they go; the default action turns them on.
    assert!(strs(&ok(&mut c, "dbtrace -q;")).is_empty());
    ok(&mut c, "dbtrace -k compute -k setAttr;");
    assert_eq!(strs(&ok(&mut c, "dbtrace -q;")), vec!["compute", "setAttr"]);
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -k compute -o;")), vec![file.clone()]);
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -k dirty -o;")), Vec::<String>::new(), "dirty is off");
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -v -k dirty -o;")), vec!["stdout"], "-v shows those off");
    ok(&mut c, "setAttr box.tx 3; getAttr box.worldMatrix;");
    ok(&mut c, "dbtrace -k compute -t \"A Title\"; dbtrace -m;");
    // A filter: only transforms' values.
    ok(&mut c, "dgfilter -nt transform -n XF; dbtrace -k setAttr -f XF;");
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -k setAttr -f;")), vec!["XF"]);
    ok(&mut c, "setAttr boxShape.castsShadows 0; setAttr box.ty 2;");
    ok(&mut c, "dbtrace -k setAttr -tm on;");
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -k setAttr -tm;")), vec!["true"]);
    ok(&mut c, "setAttr box.tz 5;");
    ok(&mut c, "dbtrace -off;");
    assert!(strs(&ok(&mut c, "dbtrace -q;")).is_empty());
    assert_eq!(strs(&ok(&mut c, "dbtrace -q -off;")).len(), 5);
    ok(&mut c, "setAttr box.tx 9;");
    let out = std::fs::read_to_string(&file).unwrap();
    assert!(out.contains("setAttr: box.translateX (transform)"), "{out}");
    assert!(out.contains("compute: box (transform)"), "{out}");
    assert!(out.contains("A Title") && out.contains("mark 1"), "{out}");
    assert!(out.contains("setAttr: box.translateY"), "{out}");
    assert!(!out.contains("castsShadows"), "filtered out: {out}");
    assert!(out.lines().any(|l| l.starts_with('[') && l.contains("box.translateZ")), "timed: {out}");
    assert_eq!(out.matches("setAttr: box.translateX").count(), 1, "tx 3 traced, tx 9 after -off not: {out}");
    assert!(text(&ok(&mut c, "dbtrace -q -i -k dirty;")).contains("dirty"));
    bad(&mut c, "dbtrace -k noSuchKeyword;");
}

#[test]
fn dbpeek_operations_and_their_help() {
    let _turn = turn();
    let mut c = Core::new();
    let ops = strs(&ok(&mut c, "dbpeek -op -query;"));
    for o in ["nodes", "plugIterator", "metadata", "references"] {
        assert!(ops.contains(&o.to_string()), "{ops:?}");
    }
    assert!(text(&ok(&mut c, "dbpeek -query -op \"nodes\";")).contains("attributes"));
    assert_eq!(strs(&ok(&mut c, "dbpeek -q -op nodes -a;")).len(), 2);
    assert!(text(&ok(&mut c, "dbpeek -op plugIterator -query;")).contains("no arguments"));
    ok(&mut c, "polyCube -n box; polyCube -n box2; select -cl;");
    let all: Value = serde_json::from_str(&text(&ok(&mut c, "dbpeek -op nodes;"))).unwrap();
    assert_eq!(all["nodes"]["polyCube"], json!(2));
    assert!(all["nodes"]["time"].is_number(), "everything with no selection: {all}");
    // The visible argument leaves the scene's own nodes out; attributes counts them by kind.
    let vis: Value = serde_json::from_str(&text(&ok(&mut c, "dbpeek -op nodes -all -a visible -a attributes;"))).unwrap();
    assert!(vis["nodes"].get("time").is_none(), "{vis}");
    ok(&mut c, "addAttr -ln extra -at double box;");
    let one: Value = serde_json::from_str(&text(&ok(&mut c, "dbpeek -op nodes -a attributes box;"))).unwrap();
    assert_eq!(one["nodes"]["transform"]["count"], json!(1));
    assert_eq!(one["nodes"]["transform"]["dynamicAttributes"], json!(1));
    // -eg: only what animation reaches.
    ok(&mut c, "setKeyframe -at tx -t 1 -v 0 box; setKeyframe -at tx -t 5 -v 3 box;");
    let eg: Value = serde_json::from_str(&text(&ok(&mut c, "dbpeek -op nodes -all -eg;"))).unwrap();
    assert_eq!(eg["nodes"]["animCurveTL"], json!(1), "{eg}");
    assert!(eg["nodes"].get("polyCube").is_none(), "{eg}");
    let pi = text(&ok(&mut c, "dbpeek -op plugIterator -count 1000;"));
    assert!(pi.contains("Loops") && pi.contains("Average"), "{pi}");
    let d = scratch("peek");
    let f = path(&d.join("out.txt"));
    assert_eq!(num(&ok(&mut c, &format!("dbpeek -op plugIterator -count 100 -of \"{f}\";"))), 0.0);
    assert!(std::fs::read_to_string(&f).unwrap().contains("Total time"));
    // Metadata, as a summary.
    ok(&mut c, "dataStructure -fmt raw -asString \"name=luck:int32=value\"; select -cl; addMetadata -structure luck -streamName ls -channelName face boxShape; editMetadata -streamName ls -memberName value -value 7 -index 2 boxShape;");
    let md = text(&ok(&mut c, "dbpeek -op metadata -a summary boxShape;"));
    assert_eq!(md, "Node boxShape : face( ls[1] )");
    bad(&mut c, "dbpeek -op nothing;");
}

#[test]
fn dgmodified_lists_nodes_changed_since_saving() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("mod");
    assert!(strs(&ok(&mut c, "dgmodified;")).is_empty(), "a new scene has nothing changed");
    ok(&mut c, "polyCube -n box;");
    let m = strs(&ok(&mut c, "dgmodified;"));
    assert!(m.contains(&"box".to_string()) && m.contains(&"polyCube1".to_string()), "{m:?}");
    ok(&mut c, &format!("file -rename \"{}\"; file -type mayaAscii -save;", path(&d.join("m.ma"))));
    assert!(strs(&ok(&mut c, "dgmodified;")).is_empty());
    ok(&mut c, "setAttr box.tx 2;");
    assert_eq!(strs(&ok(&mut c, "dgmodified;")), vec!["box"]);
    ok(&mut c, "file -f -new;");
    assert!(strs(&ok(&mut c, "dgmodified;")).is_empty());
}

#[test]
fn dgvalidatecurve_tags_static_curves() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -at tx -t 1 -v 0 box; setKeyframe -at tx -t 10 -v 5 box; setKeyframe -at ty -t 1 -v 2 box; setKeyframe -at ty -t 10 -v 2 box;");
    // The flat curve gets its static tag; again, nothing changes.
    assert_eq!(num(&ok(&mut c, "dgValidateCurve -allCurves;")), 1.0);
    assert_eq!(num(&ok(&mut c, "dgValidateCurve -a;")), 0.0);
    // Edited into motion: its tag goes.
    ok(&mut c, "keyframe -e -t 10 -vc 7 box_translateY;");
    printed(&mut c);
    assert_eq!(num(&ok(&mut c, "dgValidateCurve -v box;")), 1.0);
    let lines = printed(&mut c);
    assert!(lines.iter().any(|l| l.contains("box_translateY: not static")), "{lines:?}");
    // Named curves only.
    ok(&mut c, "keyframe -e -t 10 -vc 2 box_translateY;");
    assert_eq!(num(&ok(&mut c, "dgValidateCurve box_translateX;")), 0.0);
    assert_eq!(num(&ok(&mut c, "dgValidateCurve box_translateY;")), 1.0);
}

#[test]
fn displaystring_table() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "displayString -value \"Hello world\" kExampleHelloWorld;");
    assert_eq!(text(&ok(&mut c, "displayString -query -value kExampleHelloWorld;")), "Hello world");
    ok(&mut c, "displayString -value \"^1s...\" kExampleEllipsesFormat;");
    assert_eq!(text(&ok(&mut c, "displayString -query -value kExampleEllipsesFormat kExampleHelloWorld;")), "Hello world...");
    ok(&mut c, "displayString -value \"These ^1s are ^2s me ^3s\" kExampleAnotherFormat; displayString -value \"pretzels\" kExamplePretzels;");
    assert_eq!(text(&ok(&mut c, "displayString -query -value kExampleAnotherFormat kExamplePretzels \"making\" \"thirsty\";")), "These pretzels are making me thirsty");
    ok(&mut c, "displayString -v a m_testStrings.one; displayString -v b m_testStrings.two;");
    assert_eq!(strs(&ok(&mut c, "displayString -query -keys \"m_testStrings.\";")), vec!["m_testStrings.one", "m_testStrings.two"]);
    ok(&mut c, "displayString kExampleMissingValue;");
    assert_eq!(text(&ok(&mut c, "displayString -query -value kExampleMissingValue;")), "kExampleMissingValue");
    // An identifier that exists changes only with -replace.
    bad(&mut c, "displayString -v other kExampleHelloWorld;");
    ok(&mut c, "displayString -r -v other kExampleHelloWorld;");
    assert_eq!(text(&ok(&mut c, "displayString -q -v kExampleHelloWorld;")), "other");
    assert!(truth(&ok(&mut c, "displayString -exists kExampleHelloWorld;")));
    ok(&mut c, "displayString -delete kExampleHelloWorld;");
    assert!(!truth(&ok(&mut c, "displayString -exists kExampleHelloWorld;")));
    bad(&mut c, "displayString -delete kExampleHelloWorld;");
}

#[test]
fn evaluator_turns_evaluators_and_types_on_and_off() {
    let _turn = turn();
    let mut c = Core::new();
    let all = strs(&ok(&mut c, "evaluator -q;"));
    for e in ["dynamics", "deformer", "pruneRoots", "frozen", "invisibility"] {
        assert!(all.contains(&e.to_string()), "{all:?}");
    }
    assert!(!truth(&ok(&mut c, "evaluator -q -name \"deformer\";")));
    assert!(strs(&ok(&mut c, "evaluator -enable off -q;")).contains(&"deformer".to_string()));
    // Turning one on answers its state before.
    assert!(!truth(&ok(&mut c, "evaluator -enable on -name \"deformer\";")));
    assert!(truth(&ok(&mut c, "evaluator -enable on -name \"deformer\";")));
    assert!(strs(&ok(&mut c, "evaluator -enable on -q;")).contains(&"deformer".to_string()));
    assert_eq!(strs(&ok(&mut c, "evaluator -enable on -name \"deformer\" -nodeType \"transform\";")), vec!["transform"]);
    let kids = strs(&ok(&mut c, "evaluator -enable on -name \"deformer\" -nodeType \"transform\" -nodeTypeChildren;"));
    assert_eq!(kids[0], "transform");
    assert!(kids.contains(&"joint".to_string()), "{kids:?}");
    // Priorities: unique.
    assert_eq!(num(&ok(&mut c, "evaluator -q -name \"pruneRoots\" -priority;")), 1000.0);
    assert!(truth(&ok(&mut c, "evaluator -name \"pruneRoots\" -priority 2500;")));
    assert_eq!(num(&ok(&mut c, "evaluator -q -name \"pruneRoots\" -priority;")), 2500.0);
    bad(&mut c, "evaluator -name \"dynamics\" -priority 2500;");
    // Configuration and values.
    assert!(strs(&ok(&mut c, "evaluator -q -name \"deformer\" -configuration;")).iter().any(|x| x.starts_with("cluster=")));
    assert!(truth(&ok(&mut c, "evaluator -name \"deformer\" -configuration \"cluster=subgraph\";")));
    assert!(!truth(&ok(&mut c, "evaluator -name \"deformer\" -configuration \"nonsense=1\";")));
    assert_eq!(text(&ok(&mut c, "evaluator -q -name \"deformer\" -valueName \"cluster\";")), "subgraph");
    assert!(!text(&ok(&mut c, "evaluator -q -name \"deformer\" -info;")).is_empty());
    // Clusters: the frozen evaluator claims the frozen nodes.
    ok(&mut c, "polyCube -n a; polyCube -n b; setAttr a.frozen 1; setAttr b.frozen 1;");
    let cl = strs(&ok(&mut c, "evaluator -name \"frozen\" -q -cl;"));
    assert_eq!(cl, vec!["2", "a", "b"]);
    // The transforms turned on for the deformer evaluator are claimed by it.
    let dcl = strs(&ok(&mut c, "evaluator -name \"deformer\" -q -cl;"));
    assert!(dcl.contains(&"a".to_string()), "{dcl:?}");
    // An unknown name lists the evaluators.
    assert_eq!(strs(&ok(&mut c, "evaluator -q -name \"nothing\";")), all);
}

#[test]
fn deformerevaluator_partitions_deformation_chains() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "polySphere -sx 40 -sy 40 -n ball; select ball; nonLinear -type bend; polyCube -n box; select box; cluster; select -cl;");
    let dfs = strs(&ok(&mut c, "deformerEvaluator -q -deformers;"));
    assert!(dfs.contains(&"skinCluster".to_string()) && dfs.contains(&"cluster".to_string()), "{dfs:?}");
    // Off until the deformer evaluator is on; small meshes stay off.
    assert!(strs(&ok(&mut c, "deformerEvaluator -q -ls;")).is_empty());
    let chains = strs(&ok(&mut c, "deformerEvaluator -q -dch ball box;"));
    assert_eq!(chains, vec!["ballShape: bend1 -> ballShape (evaluator off)", "boxShape: cluster1 -> boxShape (too few vertices)"]);
    ok(&mut c, "evaluator -en on -n deformer;");
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ls;")), vec!["bend1", "ballShape"]);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ls -m;")), vec!["ballShape"]);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ns ball box;")), vec!["1", "3"]);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ns -txt -nm ball;")), vec!["ballShape: active"]);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -mbr bend1;")), vec!["bend1", "ballShape"]);
    assert!(text(&ok(&mut c, "deformerEvaluator -q -msg box;")).contains("under the minimum of 500"));
    let info: Value = serde_json::from_str(&text(&ok(&mut c, "deformerEvaluator -query -dumpInfo -opt;"))).unwrap();
    assert_eq!(info["clusters"].as_array().unwrap().len(), 2);
    assert_eq!(strs(&ok(&mut c, "evaluator -name deformer -q -cl;")), vec!["2", "bend1", "ballShape"]);
    // Settings.
    ok(&mut c, "deformerEvaluator -mnv 4;");
    assert_eq!(num(&ok(&mut c, "deformerEvaluator -q -mnv;")), 4.0);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ns -txt box;")), vec!["active"]);
    ok(&mut c, "deformerEvaluator -lmv off; deformerEvaluator -mnv 500;");
    assert!(!truth(&ok(&mut c, "deformerEvaluator -q -lmv;")));
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ns -txt box;")), vec!["active"], "the limit is not enforced");
    ok(&mut c, "deformerEvaluator -lmv on;");
    ok(&mut c, "deformerEvaluator -gbp \"on\" ball;");
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -gbp ball;")), vec!["on"]);
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ns -txt ball;")), vec!["blocked"]);
    bad(&mut c, "deformerEvaluator -gbp \"sideways\" ball;");
    ok(&mut c, "deformerEvaluator -ru \"always\" \"never\";");
    assert_eq!(strs(&ok(&mut c, "deformerEvaluator -q -ru;")), vec!["always", "never"]);
    ok(&mut c, "deformerEvaluator -clv off;");
    assert!(!truth(&ok(&mut c, "deformerEvaluator -q -clv;")));
    let h1 = text(&ok(&mut c, "deformerEvaluator -q -olh;"));
    ok(&mut c, "deformerEvaluator -gbp off ball;");
    assert_ne!(h1, text(&ok(&mut c, "deformerEvaluator -q -olh;")));
    assert!(text(&ok(&mut c, "deformerEvaluator -q -dol;")).starts_with("[{'name': "));
    // Without -q the answers go to the Script Editor.
    printed(&mut c);
    ok(&mut c, "select ball; deformerEvaluator;");
    let lines = printed(&mut c);
    assert!(lines.iter().any(|l| l.starts_with("ballShape (mesh): ")), "{lines:?}");
}

#[test]
fn deleteextension_keeps_values_unless_forced() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "addExtension -nt transform -ln martians -sn mr -at double; createNode transform -n jupiter; createNode transform -n mars; setAttr mars.mr 35;");
    // Off: a node holds a value, so it stays everywhere.
    assert_eq!(num(&ok(&mut c, "deleteExtension -nt transform -forceDelete off -at mr;")), 0.0);
    assert!(truth(&ok(&mut c, "attributeQuery -n jupiter -ex mr;")));
    assert_eq!(num(&ok(&mut c, "getAttr mars.mr;")), 35.0);
    // On: gone from every node and from the type.
    assert_eq!(num(&ok(&mut c, "deleteExtension -nt transform -forceDelete on -at martians;")), 1.0);
    assert!(!truth(&ok(&mut c, "attributeQuery -n jupiter -ex mr;")));
    assert!(!truth(&ok(&mut c, "attributeQuery -n mars -ex mr;")));
    ok(&mut c, "createNode transform -n venus;");
    assert!(!truth(&ok(&mut c, "attributeQuery -n venus -ex mr;")), "new nodes no longer get it");
    // Nothing holding values: deleted with -fd off too.
    ok(&mut c, "addExtension -nt transform -ln moons -at long;");
    assert_eq!(num(&ok(&mut c, "deleteExtension -nt transform -fd off -at moons;")), 0.0);
    assert!(!truth(&ok(&mut c, "attributeQuery -n venus -ex moons;")));
    bad(&mut c, "deleteExtension -nt transform -at moons;");
    // A compound goes with its children; a child alone can't.
    ok(&mut c, "addExtension -nt transform -ln grp -at compound -nc 2; addExtension -nt transform -ln ga -at double -p grp; addExtension -nt transform -ln gb -at double -p grp;");
    bad(&mut c, "deleteExtension -nt transform -at ga;");
    ok(&mut c, "deleteExtension -nt transform -fd on -at grp;");
    assert!(!truth(&ok(&mut c, "attributeQuery -n venus -ex ga;")));
}

#[test]
fn displaypref_ghost_frames_and_texture_limit() {
    let _turn = turn();
    let mut c = Core::new();
    assert!(num(&ok(&mut c, "displayPref -q -mhr;")) >= 1024.0);
    assert!(num(&ok(&mut c, "displayPref -q -maxHardwareTextureResolution;")) >= 1024.0);
    ok(&mut c, "displayPref -gf 2 4 1;");
    assert_eq!(strs(&ok(&mut c, "displayPref -q -gf;")), vec!["2", "4", "1"]);
    // The ghosting options are what it sets.
    assert_eq!(num(&ok(&mut c, "ghosting -q -preFrames;")), 2.0);
    bad(&mut c, "displayPref -gf 2 4;");
    // The other settings still work.
    ok(&mut c, "displayPref -displayAffected 0;");
    assert!(!truth(&ok(&mut c, "displayPref -q -da;")));
}

#[test]
fn disableincorrectnamewarning_silences_bad_names_in_files() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("names");
    let f = d.join("bad.ma");
    std::fs::write(&f, "//Maya ASCII 2027 scene\nrequires maya \"2027\";\ncreateNode transform -n \"9lives\";\n").unwrap();
    // What reading the file warns about.
    let lines = aztek_scene::ma_file::open_scene(&mut c.s, &path(&f)).unwrap();
    assert!(lines.iter().any(|l| l.contains("Incorrect node name \"9lives\"")), "{lines:?}");
    ok(&mut c, "disableIncorrectNameWarning;");
    let lines = aztek_scene::ma_file::open_scene(&mut c.s, &path(&f)).unwrap();
    assert!(!lines.iter().any(|l| l.contains("Incorrect node name")), "{lines:?}");
}

#[test]
fn event_acts_on_particle_collisions() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "playbackOptions -min 1 -max 40; polyPlane -w 200 -h 200 -n floor; collider floor; particle -p 0 10 0 -p 5 10 0 -p -5 10 0 -n pt; particle -n other;");
    ok(&mut c, "global string $hits[]; global proc hitProc(string $obj, int $id, string $objHit) { global string $hits[]; $hits[size($hits)] = ($obj + \" \" + $id + \" \" + $objHit); }");
    assert_eq!(text(&ok(&mut c, "event -em 2 -t other pt;")), "event0");
    assert_eq!(text(&ok(&mut c, "event -n killer -die 1 -count 1 pt;")), "killer");
    assert_eq!(text(&ok(&mut c, "event -n caller -proc hitProc pt;")), "caller");
    bad(&mut c, "event -n killer pt;");
    assert_eq!(strs(&ok(&mut c, "event -ls pt;")), vec!["event0", "killer", "caller"]);
    assert_eq!(strs(&ok(&mut c, "select -cl; event -ls;")), vec!["ptShape", "event0", "ptShape", "killer", "ptShape", "caller"]);
    assert_eq!(num(&ok(&mut c, "event -count -q -name killer pt;")), 1.0);
    assert!(truth(&ok(&mut c, "event -die -q -name killer pt;")));
    assert_eq!(num(&ok(&mut c, "event -em -q -name event0 pt;")), 2.0);
    assert_eq!(text(&ok(&mut c, "event -t -q -name event0 pt;")), "other");
    ok(&mut c, "event -e -name event0 -emit 3 pt;");
    assert_eq!(num(&ok(&mut c, "event -em -q -name event0 pt;")), 3.0);
    // The events are the particle node's arrays.
    assert_eq!(text(&ok(&mut c, "getAttr ptShape.eventName[1];")), "killer");
    let mut hit_at = None;
    for f in 1..=30 {
        ok(&mut c, &format!("currentTime {f};"));
        let n = num(&ok(&mut c, "particle -q -ct pt;"));
        if n == 0.0 && hit_at.is_none() {
            hit_at = Some(f);
        }
    }
    assert!(hit_at.is_some(), "the particles die where they hit the floor");
    // Each emitted three into the other object at its collision.
    assert_eq!(num(&ok(&mut c, "particle -q -ct other;")), 9.0);
    c.idle();
    let hits = strs(&ok(&mut c, "global string $hits[]; $hits;"));
    assert_eq!(hits.len(), 3, "{hits:?}");
    assert!(hits.iter().all(|h| h.starts_with("ptShape ") && h.ends_with(" floor")), "{hits:?}");
    // Rename and delete.
    ok(&mut c, "event -name caller -e -rename called pt;");
    assert_eq!(strs(&ok(&mut c, "event -ls pt;")), vec!["event0", "killer", "called"]);
    ok(&mut c, "event -d -name killer pt;");
    assert_eq!(strs(&ok(&mut c, "event -ls pt;")), vec!["event0", "called"]);
    bad(&mut c, "event -d -name killer pt;");
    bad(&mut c, "event -em 1 floor;");
}

#[test]
fn event_split_and_count() {
    let _turn = turn();
    let mut c = Core::new();
    ok(&mut c, "playbackOptions -min 1 -max 40; polyPlane -w 200 -h 200 -n floor; collider floor; particle -p 0 10 0 -p 5 10 0 -n pt;");
    // Splitting: each colliding particle leaves its object and becomes two in the target.
    ok(&mut c, "particle -n other; event -split 2 -t other -count 1 pt;");
    for f in 1..=30 {
        ok(&mut c, &format!("currentTime {f};"));
    }
    assert_eq!(num(&ok(&mut c, "particle -q -ct pt;")), 0.0);
    assert_eq!(num(&ok(&mut c, "particle -q -ct other;")), 4.0, "two particles split into four");
    // -random: from 1 to the number given.
    ok(&mut c, "currentTime 1; event -e -n event0 -split 3 -random on pt;");
    assert!(truth(&ok(&mut c, "event -r -q -n event0 pt;")));
    for f in 1..=30 {
        ok(&mut c, &format!("currentTime {f};"));
    }
    let n = num(&ok(&mut c, "particle -q -ct other;"));
    assert!((2.0..=6.0).contains(&n), "{n}");
}

#[test]
fn exportedits_round_trips_reference_edits() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("edits");
    let refbox = path(&d.join("refbox.ma"));
    ok(&mut c, &format!("polyCube -n box; file -rename \"{refbox}\"; file -type mayaAscii -save; file -f -new;"));
    ok(&mut c, &format!("file -r -ns \"rb\" \"{refbox}\";"));
    ok(&mut c, "setAttr rb:box.sx 2; setAttr rb:box.tz 3; polySphere -n ball; connectAttr ball.ty rb:box.ty; setKeyframe -at rz -t 1 -v 0 rb:box; setKeyframe -at rz -t 10 -v 30 rb:box;");
    // What would go: the targets and, with -ian, the curve.
    ok(&mut c, "select rb:box;");
    assert_eq!(strs(&ok(&mut c, "exportEdits -q -sel -ian;")), vec!["rb:box", "box_rotateZ"]);
    assert_eq!(strs(&ok(&mut c, "exportEdits -q -sel;")), vec!["rb:box"]);
    bad(&mut c, "exportEdits -q -ian;");
    let f1 = path(&d.join("ed1"));
    let written = text(&ok(&mut c, &format!("exportEdits -sel -ian -type \"editMA\" \"{f1}\";")));
    assert!(written.ends_with("ed1.editMA"), "{written}");
    let body = std::fs::read_to_string(&written).unwrap();
    assert!(body.contains("createNode animCurveTA -n \"box_rotateZ\""), "{body}");
    assert!(!body.contains("file -r"), "no reference of its own: {body}");
    assert!(!body.contains("createNode reference"), "{body}");
    assert!(body.contains("//edit 2 \"|rb:box\" \"scale\""), "{body}");
    // Only setAttr edits, from the reference node; -isa off leaves setAttr out.
    let f2 = path(&d.join("ed2.editMA"));
    ok(&mut c, &format!("exportEdits -orn rbRN -ec setAttr \"{f2}\";"));
    let b2 = std::fs::read_to_string(&f2).unwrap();
    assert!(b2.contains("\"scale\"") && !b2.contains("//edit 5"), "{b2}");
    bad(&mut c, &format!("exportEdits -orn rbRN -ec setAttr \"{f2}\";"));
    ok(&mut c, &format!("exportEdits -f -orn rbRN -isa off \"{f2}\";"));
    let b3 = std::fs::read_to_string(&f2).unwrap();
    assert!(!b3.contains("//edit 2"), "{b3}");
    bad(&mut c, &format!("exportEdits -sel -ec nonsense \"{}\";", path(&d.join("x"))));
    // The selected nodes themselves.
    ok(&mut c, "select ball;");
    assert_eq!(strs(&ok(&mut c, "exportEdits -q -exs;")), vec!["ball"]);
    // Into a fresh scene with the same reference: the edits apply.
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"rb\" \"{refbox}\";"));
    assert_eq!(num(&ok(&mut c, "getAttr rb:box.sx;")), 1.0);
    ok(&mut c, &format!("file -i \"{written}\";"));
    assert_eq!(num(&ok(&mut c, "getAttr rb:box.sx;")), 2.0);
    assert_eq!(strs(&ok(&mut c, "listConnections -s 1 -d 0 rb:box.rz;")), vec!["box_rotateZ"]);
    ok(&mut c, "currentTime 10;");
    assert!((num(&ok(&mut c, "getAttr rb:box.rz;")) - 30.0).abs() < 1e-6);
}

#[test]
fn definedataserver_connects_and_undefines() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("server");
    // No server there: no device.
    let e = bad(&mut c, &format!("defineDataServer -s \"{}\" -d melvin;", path(&d.join("nothing"))));
    assert!(e.contains("can't reach"), "{e}");
    assert!(aztek_scene::input_devices::device(&c.s, "melvin").is_none());
    #[cfg(unix)]
    {
        let sock = d.join("ultratrak_server");
        let listener = std::os::unix::net::UnixListener::bind(&sock).unwrap();
        let t = std::thread::spawn(move || {
            use std::io::Write;
            let (mut s, _) = listener.accept().unwrap();
            let _ = s.write_all(b"head.x head.y\n");
            std::thread::sleep(std::time::Duration::from_millis(300));
        });
        ok(&mut c, &format!("defineDataServer -s \"{}\" -d \"melvin\";", path(&sock)));
        let dev = aztek_scene::input_devices::device(&c.s, "melvin").expect("the device");
        assert_eq!(dev.kind, "dataServer");
        assert_eq!(dev.axes, vec!["head.x", "head.y"]);
        // Names are unique.
        bad(&mut c, &format!("defineDataServer -s \"{}\" -d \"melvin\";", path(&sock)));
        // Undo takes it away, redo gives it back.
        ok(&mut c, "undo;");
        assert!(aztek_scene::input_devices::device(&c.s, "melvin").is_none());
        ok(&mut c, "redo;");
        assert!(aztek_scene::input_devices::device(&c.s, "melvin").is_some());
        ok(&mut c, "defineDataServer -undefine -d \"melvin\";");
        assert!(aztek_scene::input_devices::device(&c.s, "melvin").is_none());
        t.join().unwrap();
    }
    // UDP: a port on this machine.
    let udp = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let port = udp.local_addr().unwrap().port();
    ok(&mut c, &format!("defineDataServer -s \"localhost:{port}\" -d labFlock;"));
    assert!(aztek_scene::input_devices::device(&c.s, "labFlock").is_some());
    ok(&mut c, "defineDataServer -undefine -d \"labFlock\";");
    bad(&mut c, "defineDataServer -undefine -d \"labFlock\";");
    bad(&mut c, "defineDataServer -s \"nohost.invalid:nosuchservice\" -d x;");
}

#[test]
fn fcheck_opens_images_and_sequences() {
    let _turn = turn();
    let mut c = Core::new();
    let d = scratch("fcheck");
    for n in ["myImage.png", "myTest1.png", "myTest2.png", "mySequence.1.png", "mySequence.2.png", "mySequence.10.png"] {
        std::fs::write(d.join(n), b"x").unwrap();
    }
    take_ui_requests();
    ok(&mut c, &format!("fcheck \"{}\";", path(&d.join("myImage.png"))));
    let r = take_ui_requests().to_string();
    assert!(r.contains("fcheck|") && r.contains("myImage.png"), "{r}");
    // A wildcard: a window for each.
    ok(&mut c, &format!("fcheck \"{}\";", path(&d.join("myTest*"))));
    let r: Vec<String> = serde_json::from_value(take_ui_requests()).unwrap();
    assert_eq!(r.iter().filter(|x| x.starts_with("fcheck|")).count(), 2, "{r:?}");
    // A trailing dot: one window playing the sequence in frame order.
    ok(&mut c, &format!("fcheck \"{}\";", path(&d.join("mySequence."))));
    let r: Vec<String> = serde_json::from_value(take_ui_requests()).unwrap();
    let seq: Vec<&String> = r.iter().filter(|x| x.starts_with("fcheck|")).collect();
    assert_eq!(seq.len(), 1);
    let o: Value = serde_json::from_str(&seq[0]["fcheck|".len()..]).unwrap();
    assert_eq!(o["sequence"], json!(true));
    let files: Vec<String> = serde_json::from_value(o["files"].clone()).unwrap();
    assert!(files[0].ends_with("mySequence.1.png") && files[2].ends_with("mySequence.10.png"), "{files:?}");
    bad(&mut c, &format!("fcheck \"{}\";", path(&d.join("missing.png"))));
}
