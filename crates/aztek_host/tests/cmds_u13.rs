//! Commands of the reference from aaf2fcp to dbfootprint: aaf2fcp, addExtension, affectedNet, appHome, attributeQuery's
//! further flags, backgroundEvaluationManager, callbacks, choice, commandLogging, crashInfo, dagObjectCompare, dbcount and
//! dbfootprint.

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

fn nums(v: &Value) -> Vec<f64> {
    match v {
        Value::Array(a) => a.iter().map(num).collect(),
        x => vec![num(x)],
    }
}

/// The lines printed since the last call.
fn printed(c: &mut Core) -> Vec<String> {
    let lines = c.take_log();
    lines.as_array().unwrap().iter().filter(|l| l["kind"] != json!(0)).map(|l| l["text"].as_str().unwrap().to_string()).filter(|t| !t.starts_with("// Result")).collect()
}

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("aztek_u13_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn choice_switches_a_channel_over_time() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n drv; connectAttr drv.translate a.translate; setAttr drv.ty 2;");
    // A new choice node takes what drove the channel as its first input and keys its selector at each time.
    assert_eq!(strs(&ok(&mut c, "choice -at ty -t 1 -t 30 -t 50 a;")), vec!["choice1", "choice1.selector"]);
    assert_eq!(strs(&ok(&mut c, "listConnections -s 1 -d 0 -p 1 choice1.input;")), vec!["drv.translateY"]);
    assert_eq!(strs(&ok(&mut c, "listConnections -s 1 -d 0 -p 1 a.ty;")), vec!["choice1.output"]);
    // The other axes stay with their driver.
    assert_eq!(strs(&ok(&mut c, "listConnections -s 1 -d 0 -p 1 a.tx;")), vec!["drv.translateX"]);
    // A second choice with no source starts a new input.
    ok(&mut c, "choice -at ty -t 31 a;");
    assert_eq!(nums(&ok(&mut c, "choice -q -index -at ty -t 1 -t 30 -t 40 -t 50 a;")), vec![0.0, 0.0, 1.0, 0.0]);
    // Keys on the channel go to the input picked now, as the reference's example keys between its choices.
    ok(&mut c, "currentTime 31; setKeyframe -at ty -t 31 -v 7 a; setKeyframe -at ty -t 49 -v 9 a;");
    assert_eq!(strs(&ok(&mut c, "listConnections -s 1 -d 0 choice1.input[1];")), vec!["a_translateY"]);
    ok(&mut c, "currentTime 10;");
    assert_eq!(num(&ok(&mut c, "getAttr a.ty;")), 2.0);
    ok(&mut c, "currentTime 40;");
    assert!((num(&ok(&mut c, "getAttr a.ty;")) - 8.0).abs() < 1.5);
    ok(&mut c, "currentTime 60;");
    assert_eq!(num(&ok(&mut c, "getAttr a.ty;")), 2.0);
    assert_eq!(strs(&ok(&mut c, "choice -q -n -at ty a;")), vec!["choice1"]);
    assert_eq!(strs(&ok(&mut c, "choice -q -sl -at ty a;")), vec!["choice1_selector.output"]);

    // -sourceAttribute, -name and -index; -selector drives the selector in place of keys.
    ok(&mut c, "polyCube -n b; setAttr b.tx 3;");
    assert_eq!(strs(&ok(&mut c, "choice -n pick -at tx -sa drv.ty -t 5 b;")), vec!["pick", "pick.selector"]);
    assert_eq!(nums(&ok(&mut c, "choice -q -index -at tx -t 5 b;")), vec![1.0]);
    ok(&mut c, "currentTime 5;");
    assert_eq!(num(&ok(&mut c, "getAttr b.tx;")), 2.0);
    ok(&mut c, "polyCube -n sel; choice -at tx -sa drv.tz -index 4 -sl sel.tx b;");
    assert_eq!(strs(&ok(&mut c, "choice -q -sl -at tx b;")), vec!["sel.translateX"]);
    ok(&mut c, "setAttr drv.tz 6; setAttr sel.tx 4;");
    assert_eq!(num(&ok(&mut c, "getAttr b.tx;")), 6.0);
    ok(&mut c, "setAttr sel.tx 0;");
    assert_eq!(num(&ok(&mut c, "getAttr b.tx;")), 3.0);
    // Without -at: every keyable attribute (shapes too, control points left out).
    ok(&mut c, "polyCube -n d;");
    let made = strs(&ok(&mut c, "choice -s 0 d;"));
    assert!(made.len() >= 20, "{made:?}");
    assert!(strs(&ok(&mut c, "listConnections -s 1 -d 0 d.visibility;"))[0].starts_with("choice"));
}

#[test]
fn affected_net_networks() {
    let mut c = Core::new();
    ok(&mut c, "affectedNet -t transform;");
    // A node an attribute; what affects it connects to its input.
    assert_eq!(strs(&ok(&mut c, "getAttr translateAffect.attributeName;")), vec!["transform.translate"]);
    let into: Vec<String> = strs(&ok(&mut c, "listConnections -s 1 -d 0 worldMatrixAffect.input;"));
    assert!(into.contains(&"translateAffect".to_string()), "{into:?}");
    assert!(!strs(&ok(&mut c, "listConnections -s 0 -d 1 translateAffect.output;")).is_empty());
    // An output affects nothing.
    assert!(strs(&ok(&mut c, "listConnections -s 0 -d 1 worldMatrixAffect.output;")).is_empty());
    assert_eq!(ok(&mut c, "nodeType translateAffect;"), json!("TdnAffect"));
    // From one node, and from several types at once.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; affectedNet polyCube1;");
    assert!(!strs(&ok(&mut c, "listConnections -s 0 -d 1 widthAffect.output;")).is_empty());
    assert_eq!(strs(&ok(&mut c, "getAttr widthAffect.attributeName;")), vec!["polyCube.width"]);
    let mut c = Core::new();
    let before = strs(&ok(&mut c, "ls -type TdnAffect;")).len();
    ok(&mut c, "affectedNet -t polyCube -t polySphere;");
    let names = strs(&ok(&mut c, "ls -type TdnAffect;"));
    assert!(names.len() > before && names.contains(&"radiusAffect".to_string()) && names.contains(&"widthAffect".to_string()));
    ok(&mut c, "undo;");
    assert!(strs(&ok(&mut c, "ls -type TdnAffect;")).len() < names.len());
    bad(&mut c, "affectedNet -t noSuchType;");
}

#[test]
fn add_extension_attributes() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n before;");
    ok(&mut c, "addExtension -nodeType mesh -longName mass -sn ms -defaultValue 1.0 -minValue 0.001 -maxValue 10000;");
    // Every mesh has it: those there before and those made after.
    assert_eq!(num(&ok(&mut c, "getAttr beforeShape.mass;")), 1.0);
    ok(&mut c, "polySphere -n after;");
    assert_eq!(num(&ok(&mut c, "getAttr afterShape.ms;")), 1.0);
    ok(&mut c, "setAttr afterShape.mass 0;");
    assert!((num(&ok(&mut c, "getAttr afterShape.mass;")) - 0.001).abs() < 1e-9);
    // Transforms don't.
    assert!(!truth(&ok(&mut c, "attributeQuery -n after -ex mass;")));
    // Query and edit.
    assert!(truth(&ok(&mut c, "addExtension -nt mesh -ln mass -q -ex;")));
    assert!(!truth(&ok(&mut c, "addExtension -nt mesh -ln nothing -q -ex;")));
    assert_eq!(num(&ok(&mut c, "addExtension -nt mesh -ln mass -q -max;")), 10000.0);
    assert!(truth(&ok(&mut c, "addExtension -nt mesh -ln mass -q -hnv;")));
    assert_eq!(ok(&mut c, "addExtension -nt mesh -ln mass -q -sn;"), json!("ms"));
    ok(&mut c, "addExtension -e -nt mesh -ln mass -dv 2 -nn \"Body Mass\" -ct physics;");
    assert_eq!(num(&ok(&mut c, "addExtension -nt mesh -ln mass -q -dv;")), 2.0);
    ok(&mut c, "polyTorus -n third;");
    assert_eq!(num(&ok(&mut c, "getAttr thirdShape.mass;")), 2.0);
    assert_eq!(ok(&mut c, "attributeQuery -n thirdShape -nn mass;"), json!("Body Mass"));
    assert_eq!(strs(&ok(&mut c, "attributeQuery -n beforeShape -ct mass;")), vec!["physics"]);
    // Categories are added, never taken away.
    ok(&mut c, "addExtension -e -nt mesh -ln mass -ct sim;");
    assert_eq!(strs(&ok(&mut c, "addExtension -nt mesh -ln mass -q -ct;")), vec!["physics", "sim"]);
    // listAttr: an extension, not a user-defined attribute.
    assert!(strs(&ok(&mut c, "listAttr -ex beforeShape;")).contains(&"mass".to_string()));
    assert!(!strs(&ok(&mut c, "listAttr -ud beforeShape;")).contains(&"mass".to_string()));
    bad(&mut c, "addExtension -nt mesh -ln mass;");
    bad(&mut c, "addExtension -nt noSuchType -ln x;");
    bad(&mut c, "addExtension -ln x;");

    // Saved values, not definitions.
    let dir = scratch("ext");
    let file = dir.join("ext.ma");
    ok(&mut c, &format!("setAttr beforeShape.mass 5; file -rename \"{}\"; file -save -type mayaAscii;", file.display()));
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(!text.contains("-ln \"mass\""), "the definition is left out");
    assert!(text.contains(".mass\" 5") || text.contains(".ms\" 5"), "the value is kept");
    ok(&mut c, &format!("file -f -new; file -o \"{}\";", file.display()));
    assert_eq!(num(&ok(&mut c, "getAttr beforeShape.mass;")), 5.0);

    // A multi double3, a compound with children, a colour of three floats, a spectrum data type, on every dag node.
    let mut c = Core::new();
    ok(&mut c, "addExtension -nt mesh -ln points3 -at double3 -m;");
    ok(&mut c, "polyCube -n m;");
    assert!(truth(&ok(&mut c, "attributeQuery -n mShape -m points3;")));
    ok(&mut c, "addExtension -nt choice -ln comp -at compound -nc 2; addExtension -nt choice -ln kidA -at long -p comp; addExtension -nt choice -ln kidB -dt \"string\" -p comp;");
    ok(&mut c, "createNode choice -n ch;");
    assert_eq!(strs(&ok(&mut c, "attributeQuery -n ch -lc comp;")), vec!["kidA", "kidB"]);
    assert_eq!(num(&ok(&mut c, "attributeQuery -n ch -nc comp;")), 2.0);
    assert_eq!(num(&ok(&mut c, "addExtension -nt choice -ln comp -q -nc;")), 2.0);
    ok(&mut c, "setAttr ch.kidA 3;");
    assert_eq!(num(&ok(&mut c, "getAttr ch.kidA;")), 3.0);
    ok(&mut c, "addExtension -nt lambert -ln tint -at float3 -uac; addExtension -nt lambert -ln tintR -at \"float\" -p tint; addExtension -nt lambert -ln tintG -at \"float\" -p tint; addExtension -nt lambert -ln tintB -at \"float\" -p tint;");
    ok(&mut c, "shadingNode -asShader lambert -n lam;");
    assert!(truth(&ok(&mut c, "attributeQuery -n lam -uac tint;")));
    assert_eq!(strs(&ok(&mut c, "attributeQuery -n lam -lc tint;")), vec!["tintR", "tintG", "tintB"]);
    ok(&mut c, "addExtension -nt lambert -ln spec -dt spectrumRGB;");
    assert!(truth(&ok(&mut c, "attributeQuery -n lam -uac spec;")));
    ok(&mut c, "addExtension -nt dagNode -ln pos -at double3; addExtension -nt dagNode -ln posX -at double -p pos; addExtension -nt dagNode -ln posY -at double -p pos; addExtension -nt dagNode -ln posZ -at double -p pos;");
    assert!(truth(&ok(&mut c, "attributeQuery -n m -ex posY;")));
    // What the attribute model doesn't hold is kept and answered.
    ok(&mut c, "addExtension -nt mesh -ln tag -at long -r 0 -ci 0 -is 1 -dcb 1 -fp 1;");
    assert!(!truth(&ok(&mut c, "attributeQuery -n mShape -rd tag;")));
    assert!(!truth(&ok(&mut c, "attributeQuery -n mShape -ci tag;")));
    assert!(truth(&ok(&mut c, "attributeQuery -n mShape -is tag;")));
    assert!(truth(&ok(&mut c, "attributeQuery -n mShape -i tag;")));
    assert_eq!(num(&ok(&mut c, "addExtension -nt mesh -ln tag -q -dcb;")), 1.0);
    assert!(truth(&ok(&mut c, "addExtension -nt mesh -ln tag -q -fp;")));
    ok(&mut c, "addExtension -nt mesh -ln ro -at long -w 0;");
    assert!(!truth(&ok(&mut c, "attributeQuery -n mShape -w ro;")));
}

#[test]
fn attribute_query_more_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a;");
    ok(&mut c, "addAttr -ln lim -at double -min 0 -max 5 a; addAttr -ln soft -at double -smn 1 -smx 3 a; addAttr -ln mode -at enum -en \"x:y:z\" a;");
    ok(&mut c, "addAttr -ln link -at message a; addAttr -ln grp -at compound -nc 2 a; addAttr -ln g1 -at long -p grp a; addAttr -ln g2 -at long -p grp a;");
    ok(&mut c, "addAttr -ln tagged -at long -ct alpha -ct beta a;");
    let q = |c: &mut Core, flags: &str, attr: &str| ok(c, &format!("attributeQuery -n a {flags} {attr};"));
    assert!(truth(&q(&mut c, "-re", "lim")) && !truth(&q(&mut c, "-re", "soft")));
    assert!(truth(&q(&mut c, "-se", "soft")) && !truth(&q(&mut c, "-se", "lim")));
    assert_eq!(nums(&q(&mut c, "-r", "lim")), vec![0.0, 5.0]);
    assert_eq!(nums(&q(&mut c, "-s", "soft")), vec![1.0, 3.0]);
    assert!(truth(&q(&mut c, "-e", "mode")) && !truth(&q(&mut c, "-e", "lim")));
    assert_eq!(strs(&q(&mut c, "-lz", "mode")), vec!["x:y:z"]);
    assert!(truth(&q(&mut c, "-msg", "link")) && !truth(&q(&mut c, "-msg", "lim")));
    assert_eq!(num(&q(&mut c, "-nc", "translate")), 3.0);
    assert_eq!(num(&q(&mut c, "-nc", "grp")), 2.0);
    assert_eq!(num(&q(&mut c, "-nc", "lim")), 0.0);
    assert_eq!(strs(&q(&mut c, "-ct", "tagged")), vec!["alpha", "beta"]);
    assert!(strs(&q(&mut c, "-ct", "lim")).is_empty());
    assert!(truth(&q(&mut c, "-aws", "translate")) && truth(&q(&mut c, "-aws", "rotateOrder")) && !truth(&q(&mut c, "-aws", "visibility")));
    assert!(truth(&q(&mut c, "-ws", "worldMatrix")) && !truth(&q(&mut c, "-ws", "matrix")));
    assert!(truth(&q(&mut c, "-ci", "translate")) && !truth(&q(&mut c, "-ci", "worldMatrix")));
    assert!(truth(&q(&mut c, "-rd", "translate")) && truth(&q(&mut c, "-st", "translate")) && !truth(&q(&mut c, "-st", "worldMatrix")));
    for flag in ["-aa", "-i", "-is", "-ig", "-idt", "-rs", "-umb"] {
        assert!(!truth(&q(&mut c, flag, "translate")), "{flag}");
    }
    assert!(truth(&q(&mut c, "-eun", "translate")));
    // Several switches: all of them at once.
    assert!(truth(&q(&mut c, "-k -st", "translateX")));
    assert!(!truth(&q(&mut c, "-k -msg", "translateX")));
    // By type.
    assert!(truth(&ok(&mut c, "attributeQuery -typ transform -aws rotate;")));
}

#[test]
fn background_evaluation_manager() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "backgroundEvaluationManager -q -m;"), json!("parallel"));
    assert!(truth(&ok(&mut c, "backgroundEvaluationManager -mode \"serial\";")));
    assert_eq!(ok(&mut c, "backgroundEvaluationManager -q -m;"), json!("serial"));
    bad(&mut c, "backgroundEvaluationManager -mode bogus;");
    assert!(!truth(&ok(&mut c, "backgroundEvaluationManager -q -p;")));
    ok(&mut c, "backgroundEvaluationManager -pause;");
    assert!(truth(&ok(&mut c, "backgroundEvaluationManager -q -p;")));
    // Paused twice, by two callers: one resume leaves the other's pause.
    ok(&mut c, "backgroundEvaluationManager -pause;");
    ok(&mut c, "backgroundEvaluationManager -resume;");
    assert!(truth(&ok(&mut c, "backgroundEvaluationManager -q -p;")));
    ok(&mut c, "backgroundEvaluationManager -resume;");
    assert!(!truth(&ok(&mut c, "backgroundEvaluationManager -q -p;")));
    ok(&mut c, "backgroundEvaluationManager -interrupt 1;");
    assert!(truth(&ok(&mut c, "backgroundEvaluationManager -q -i;")));
}

#[test]
fn callbacks_at_hooks() {
    let mut c = Core::new();
    ok(&mut c, "global proc string u13Classify() { return \"texture/2d\"; }");
    ok(&mut c, "global proc string u13Pair(string $a, string $b) { return ($a + \"+\" + $b); }");
    ok(&mut c, "global proc u13Quiet(string $a, string $b) { }");
    ok(&mut c, "callbacks -addCallback u13Pair -hook u13Hook -owner u13Plugin;");
    ok(&mut c, "callbacks -addCallback u13Quiet -hook u13Hook -owner u13Other;");
    ok(&mut c, "callbacks -addCallback u13Classify -hook u13Class -owner u13Plugin;");
    assert_eq!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Hook;")), vec!["u13Pair", "u13Quiet"]);
    assert_eq!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Hook -owner u13Plugin;")), vec!["u13Pair"]);
    // Each callback given the arguments; "" for one that returns nothing.
    assert_eq!(strs(&ok(&mut c, "callbacks -executeCallbacks -hook u13Hook \"one\" \"two\";")), vec!["one+two", ""]);
    assert_eq!(strs(&ok(&mut c, "string $r[] = `callbacks -executeCallbacks -hook u13Class`; $r[0];")), vec!["texture/2d"]);
    assert!(strs(&ok(&mut c, "callbacks -dumpCallbacks;")).iter().any(|l| l == "u13Hook u13Plugin u13Pair"));
    let hooks = strs(&ok(&mut c, "callbacks -describeHooks;"));
    assert!(hooks.iter().any(|h| h.starts_with("AETemplateCustomContent(string $nodeName)")));
    assert!(hooks.iter().any(|h| h.starts_with("provideAETemplateForNodeType")));
    ok(&mut c, "callbacks -removeCallback u13Pair -hook u13Hook -owner u13Plugin;");
    assert_eq!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Hook;")), vec!["u13Quiet"]);
    bad(&mut c, "callbacks -addCallback u13Pair -hook u13Hook;");
    ok(&mut c, "callbacks -addCallback u13Pair -hook u13Hook -owner u13Plugin;");
    // An owner's callbacks on one hook, then all of them.
    ok(&mut c, "callbacks -clearCallbacks -owner u13Plugin -hook u13Hook;");
    assert_eq!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Class;")), vec!["u13Classify"]);
    ok(&mut c, "callbacks -clearCallbacks -owner u13Plugin;");
    assert!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Class;")).is_empty());
    assert_eq!(strs(&ok(&mut c, "callbacks -listCallbacks -hook u13Hook;")), vec!["u13Quiet"]);
    ok(&mut c, "callbacks -clearCallbacks -owner u13Other;");
    assert!(strs(&ok(&mut c, "callbacks -executeCallbacks -hook u13Hook;")).is_empty());
}

#[test]
fn command_logging_and_crash_info() {
    let mut c = Core::new();
    ok(&mut c, "commandLogging -historySize 20;");
    assert_eq!(num(&ok(&mut c, "commandLogging -q -historySize;")), 20.0);
    assert!(truth(&ok(&mut c, "commandLogging -q -recordCommands;")));
    let dir = scratch("log");
    let log = dir.join("log.txt");
    ok(&mut c, &format!("commandLogging -logFile \"{}\";", log.display()));
    assert_eq!(ok(&mut c, "commandLogging -q -logFile;"), json!(log.display().to_string()));
    ok(&mut c, "commandLogging -logCommands 1;");
    assert!(truth(&ok(&mut c, "commandLogging -q -logCommands;")));
    ok(&mut c, "polyCube -n loggedCube;");
    ok(&mut c, "commandLogging -logCommands 0;");
    ok(&mut c, "polySphere -n notLogged;");
    let text = std::fs::read_to_string(&log).unwrap();
    assert!(text.contains("loggedCube") && !text.contains("notLogged"), "{text}");
    ok(&mut c, "commandLogging -resetLogFile;");
    assert!(ok(&mut c, "commandLogging -q -logFile;").as_str().unwrap().ends_with("aztekCommandLog.txt"));
    bad(&mut c, "commandLogging -historySize -1;");

    // No crash recorded: nothing to name.
    let prefs = scratch("prefs");
    std::env::set_var("AZTEK_PREFS", &prefs);
    for flag in ["-cf", "-cl", "-sbc"] {
        assert_eq!(ok(&mut c, &format!("crashInfo -q {flag};")), json!(""), "{flag}");
    }
    // A panic leaves a log and a record that the next session's crashInfo reads.
    ok(&mut c, "polyCube -n beforeTheCrash;");
    c.s.scene_path = Some("/scenes/test.ma".into());
    ok(&mut c, "polyCube -n last;");
    aztek_host::sys_host::install_crash_handler();
    let _ = std::panic::catch_unwind(|| panic!("u13 crash"));
    let _ = std::panic::take_hook();
    let log = ok(&mut c, "crashInfo -q -cl;").as_str().unwrap().to_string();
    assert!(log.contains("aztek.") && log.ends_with(".log"), "{log}");
    let text = std::fs::read_to_string(&log).unwrap();
    assert!(text.contains("u13 crash") && text.contains("beforeTheCrash"), "{text}");
    assert_eq!(ok(&mut c, "crashInfo -q -sbc;"), json!("/scenes/test.ma"));
    assert!(ok(&mut c, "crashInfo -q -cf;").is_string());
    bad(&mut c, "crashInfo -cf;");
    let _ = std::fs::remove_file(log);
    std::env::remove_var("AZTEK_PREFS");
}

#[test]
fn app_home_visibility() {
    let mut c = Core::new();
    ok(&mut c, "appHome -e -v 0;");
    assert!(!truth(&ok(&mut c, "appHome -q -v;")));
    ok(&mut c, "appHome -e -v 1 -t GettingStarted;");
    assert!(truth(&ok(&mut c, "appHome -q -v;")));
    ok(&mut c, "appHome -tv;");
    assert!(!truth(&ok(&mut c, "appHome -q -v;")));
    ok(&mut c, "appHome -updateRecentFiles;");
    bad(&mut c, "appHome -e -t Nowhere;");
    assert!(truth(&ok(&mut c, "appHome -q -iv;")));
    ok(&mut c, "appHome -e -iv 0;");
    assert!(!truth(&ok(&mut c, "appHome -q -iv;")));
    ok(&mut c, "appHome -e -iv 1;");
    ok(&mut c, "appHome -i \"opened from a test\";");
    assert_eq!(aztek_host::sys_host::app_home_instrument(), "opened from a test");
}

#[test]
fn dag_object_compare_reports() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; duplicate -n b a; group -n g b;");
    printed(&mut c);
    ok(&mut c, "dagObjectCompare -t a b;");
    assert!(printed(&mut c).iter().any(|l| l.contains("no differences")));
    ok(&mut c, "setAttr b.tx 2; setAttr b.ry 30;");
    ok(&mut c, "dagObjectCompare -a a b;");
    let lines: Vec<String> = printed(&mut c).into_iter().filter(|l| l.contains("differs")).collect();
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[0].contains("translate"));
    // -bail first stops at the first.
    ok(&mut c, "dagObjectCompare -a -b first a b;");
    assert_eq!(printed(&mut c).iter().filter(|l| l.contains("differs")).count(), 1);
    // Relatives: b is under g.
    ok(&mut c, "dagObjectCompare -r -s a b;");
    let short = printed(&mut c);
    assert!(short.iter().any(|l| l.starts_with("relative: parent 'g'")), "{short:?}");
    // Type.
    ok(&mut c, "spaceLocator -n loc;");
    ok(&mut c, "dagObjectCompare -t -s a locShape;");
    assert!(printed(&mut c).iter().any(|l| l.starts_with("type:")));
    // Connections.
    ok(&mut c, "connectAttr a.tx a.ty;");
    ok(&mut c, "dagObjectCompare -c -s a b;");
    assert!(printed(&mut c).iter().any(|l| l.contains("no connection")));
    // The baseline in a namespace.
    ok(&mut c, "namespace -add base; polyCube -n \"base:box\"; polyCube -n box; setAttr box.tz 4;");
    printed(&mut c);
    ok(&mut c, "dagObjectCompare -a -n base box;");
    assert!(printed(&mut c).iter().any(|l| l.contains("translate") && l.contains("base:box")));
    bad(&mut c, "dagObjectCompare a;");
    bad(&mut c, "dagObjectCompare -b sometimes a b;");
}

#[test]
fn dbcount_counters() {
    let mut c = Core::new();
    bad(&mut c, "dbcount;");
    assert_eq!(strs(&ok(&mut c, "dbcount -e \"on\";")), vec!["compute", "dirty", "createNode", "connect"]);
    ok(&mut c, "dbcount -r;");
    ok(&mut c, "polyCube; polyCube; createNode multiplyDivide;");
    printed(&mut c);
    ok(&mut c, "dbcount -k createNode;");
    let lines = printed(&mut c);
    let total: f64 = lines.iter().find(|l| l.starts_with("createNode = ")).and_then(|l| l.split_whitespace().nth(2)).unwrap().parse().unwrap();
    assert!(total >= 5.0, "{lines:?}");
    assert!(lines.iter().any(|l| l.trim().starts_with("multiplyDivide = ")), "{lines:?}");
    // Totals only, as a spreadsheet, to a file.
    ok(&mut c, "dbcount -q -k createNode;");
    assert!(printed(&mut c).iter().all(|l| !l.contains('{') && !l.trim().starts_with("multiplyDivide")));
    let dir = scratch("dbcount");
    let file = dir.join("xls.txt");
    ok(&mut c, &format!("dbcount -s -f \"{}\";", file.display()));
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.starts_with("Counter\tCount\n") && text.contains("createNode/multiplyDivide\t"), "{text}");
    // Reset keeps only the top counters, at zero.
    assert_eq!(strs(&ok(&mut c, "dbcount -r -k createNode;")), vec!["createNode"]);
    ok(&mut c, "dbcount -k createNode -s;");
    let lines = printed(&mut c);
    assert!(!lines.iter().any(|l| l.contains("createNode/")), "{lines:?}");
    ok(&mut c, "dbcount -l;");
    assert!(printed(&mut c).iter().any(|l| l == "dirty (on)"));
    ok(&mut c, "dbcount -e off -k dirty;");
    ok(&mut c, "dbcount -l -k dirty;");
    assert_eq!(printed(&mut c), vec!["dirty (off)"]);
    ok(&mut c, "dbcount -e off;");
    bad(&mut c, "dbcount;");
}

#[test]
fn dbfootprint_memory() {
    let mut c = Core::new();
    assert_eq!(strs(&ok(&mut c, "dbfootprint -q;")), vec!["nodes"]);
    assert!(ok(&mut c, "dbfootprint -q -t nodes;").as_str().unwrap().contains("bytes"));
    bad(&mut c, "dbfootprint -q -t nothing;");
    ok(&mut c, "polySphere -n ball -sx 40 -sy 40; polyCube -n box;");
    let one: Value = serde_json::from_str(ok(&mut c, "dbfootprint -t nodes ballShape;").as_str().unwrap()).unwrap();
    let objs = one["objects"].as_object().unwrap();
    assert_eq!(objs.len(), 1);
    let ball = objs.values().next().unwrap()["bytes"].as_u64().unwrap();
    ok(&mut c, "select -r boxShape;");
    let sel: Value = serde_json::from_str(ok(&mut c, "dbfootprint;").as_str().unwrap()).unwrap();
    let boxb = sel["objects"].as_object().unwrap().values().next().unwrap()["bytes"].as_u64().unwrap();
    assert!(ball > boxb, "a dense mesh holds more: {ball} {boxb}");
    let all: Value = serde_json::from_str(ok(&mut c, "dbfootprint -all;").as_str().unwrap()).unwrap();
    assert!(all["objects"].as_object().unwrap().len() > 5 && all["total"].as_u64().unwrap() > ball);
    let dir = scratch("footprint");
    let file = dir.join("fp.json");
    assert_eq!(ok(&mut c, &format!("dbfootprint -of \"{}\" ball;", file.display())), Value::Null);
    let text: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert!(text["objects"].get("ball").is_some());
}

/// A small AAF file: a structured storage holding one property set with a mob named so.
fn write_aaf(path: &std::path::Path, mob: &str) {
    let mut name: Vec<u8> = mob.encode_utf16().chain([0]).flat_map(|u| u.to_le_bytes()).collect();
    let mut props = vec![0x4c, 0x00, 1, 0];
    props.extend(0x4402u16.to_le_bytes());
    props.extend(0x0082u16.to_le_bytes());
    props.extend((name.len() as u16).to_le_bytes());
    props.append(&mut name);
    let mut f = vec![0u8; 512 * 5];
    f[..8].copy_from_slice(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]);
    let put32 = |f: &mut Vec<u8>, at: usize, v: u32| f[at..at + 4].copy_from_slice(&v.to_le_bytes());
    f[0x1A] = 0x3E;
    f[0x1C] = 0xFE;
    f[0x1D] = 0xFF;
    f[0x1E] = 9;
    f[0x20] = 6;
    put32(&mut f, 0x2C, 1);
    put32(&mut f, 0x30, 1);
    put32(&mut f, 0x38, 4096);
    put32(&mut f, 0x3C, 3);
    put32(&mut f, 0x40, 1);
    put32(&mut f, 0x44, 0xFFFF_FFFE);
    for i in 0..109 {
        put32(&mut f, 0x4C + i * 4, if i == 0 { 0 } else { 0xFFFF_FFFF });
    }
    // Sector 0, the FAT: itself, then the directory, the mini stream and the mini FAT each one sector long.
    let fat = 512;
    for i in 0..128 {
        put32(&mut f, fat + i * 4, match i {
            0 => 0xFFFF_FFFD,
            1..=3 => 0xFFFF_FFFE,
            _ => 0xFFFF_FFFF,
        });
    }
    // Sector 1, the directory: the root (its stream the mini stream, sector 2) and the property stream.
    let dir = 1024;
    let entry = |f: &mut Vec<u8>, at: usize, name: &str, kind: u8, start: u32, size: u64| {
        let units: Vec<u8> = name.encode_utf16().chain([0]).flat_map(|u| u.to_le_bytes()).collect();
        f[at..at + units.len()].copy_from_slice(&units);
        f[at + 64..at + 66].copy_from_slice(&(units.len() as u16).to_le_bytes());
        f[at + 66] = kind;
        for k in [68, 72, 76] {
            f[at + k..at + k + 4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        }
        f[at + 116..at + 120].copy_from_slice(&start.to_le_bytes());
        f[at + 120..at + 128].copy_from_slice(&size.to_le_bytes());
    };
    entry(&mut f, dir, "Root Entry", 5, 2, 64);
    entry(&mut f, dir + 128, "properties", 2, 0, props.len() as u64);
    f[dir + 76..dir + 80].copy_from_slice(&1u32.to_le_bytes());
    // Sector 2, the mini stream; sector 3, the mini FAT.
    f[1536..1536 + props.len()].copy_from_slice(&props);
    for i in 0..128 {
        put32(&mut f, 2048 + i * 4, if i == 0 { 0xFFFF_FFFE } else { 0xFFFF_FFFF });
    }
    std::fs::write(path, f).unwrap();
}

#[test]
fn aaf_to_final_cut_xml() {
    let mut c = Core::new();
    let dir = scratch("aaf");
    let src = dir.join("edit.aaf");
    write_aaf(&src, "Main Sequence");
    let h = num(&ok(&mut c, &format!("aaf2fcp -src \"{}\" -dst \"{}\";", src.display(), dir.display()))) as i64;
    assert!(h > 0);
    let out = ok(&mut c, &format!("aaf2fcp -getFileName {h};")).as_str().unwrap().to_string();
    assert!(out.ends_with("edit.xml"));
    ok(&mut c, &format!("aaf2fcp -waitCompletion {h};"));
    assert_eq!(num(&ok(&mut c, &format!("aaf2fcp -progress {h};"))), 100.0);
    let xml = std::fs::read_to_string(&out).unwrap();
    assert!(xml.contains("<xmeml version=\"4\">") && xml.contains("<name>edit</name>") && xml.contains("<name>Main Sequence</name>"), "{xml}");
    ok(&mut c, &format!("aaf2fcp -terminate {h} -df false;"));
    assert!(std::path::Path::new(&out).exists());
    bad(&mut c, &format!("aaf2fcp -progress {h};"));
    // Deleting the file as it ends.
    let h = num(&ok(&mut c, &format!("aaf2fcp -src \"{}\" -dst \"{}\";", src.display(), dir.display()))) as i64;
    ok(&mut c, &format!("aaf2fcp -wc {h}; aaf2fcp -t {h} -df true;"));
    assert!(!std::path::Path::new(&out).exists());
    // Not an AAF file.
    let not = dir.join("not.aaf");
    std::fs::write(&not, b"plain text, not structured storage").unwrap();
    bad(&mut c, &format!("aaf2fcp -src \"{}\" -dst \"{}\";", not.display(), dir.display()));
    bad(&mut c, "aaf2fcp -src \"/no/such/file.aaf\" -dst \"/tmp\";");
}

#[test]
fn every_flag_of_the_unit_is_there() {
    let c = Core::new();
    let want: &[(&str, &str)] = &[
        ("aaf2fcp", "deleteFile df dst dstPath getFileName gfn pr progress src srcFile t terminate waitCompletion wc"),
        ("addExtension", "at attributeType binaryTag bt cachedInternally category ci ct dataType dcb defaultValue disconnectBehaviour dt dv e edit en enforcingUniqueName enumName eun ex exists fp fromPlugin h hasMaxValue hasMinValue hasSoftMaxValue hasSoftMinValue hidden hnv hsn hsx hxv im indexMatters internalSet k keyable ln longName m max maxValue min minValue multi nc niceName nn nodeType nt numberOfChildren p parent proxy pxy q query r readable s shortName smn smx sn softMaxValue softMinValue storable uac uaf uap usedAsColor usedAsFilename usedAsProxy w worldSpace writable ws"),
        ("affectedNet", "t type"),
        ("appHome", "e edit i iconVisible instrument iv q query setTab t toggleVisibility tv updateRecentFiles urf v visible"),
        ("attributeQuery", "aa affectsAppearance affectsWorldspace aws cachedInternally categories ci ct e enum i idt ig indeterminant internal internalGet internalSet localizedListEnum lz message msg nc numberOfChildren rangeExists rd re renderSource rs se softRangeExists st umb usesMultiBuilder worldspace ws"),
        ("backgroundEvaluationManager", "i interrupt m mode p pause q query r resume"),
        ("callbacks", "ac addCallback cac cc clearAllCallbacks clearCallbacks dc describeHooks dh dumpCallbacks ec executeCallbacks h hook lc listCallbacks o owner rc removeCallback"),
        ("choice", "at attribute controlPoints cp index n name q query s sa selector shape sl sourceAttribute t time"),
        ("commandLogging", "historySize hs lc lf logCommands logFile q query rc recordCommands resetLogFile rl"),
        ("crashInfo", "cf cl crashFile crashLog q query savedBeforeCrash sbc"),
        ("dagObjectCompare", "a attribute b bail c connection n namespace r relative s short t type"),
        ("dbcount", "e enabled f file k keyword l list maxdepth md q quick r reset s spreadsheet"),
        ("dbfootprint", "all allObjects of outputFile q query t type"),
    ];
    let mut missing = Vec::new();
    for (cmd, flags) in want {
        let Some(def) = c.s.commands.find(cmd) else {
            missing.push(cmd.to_string());
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
