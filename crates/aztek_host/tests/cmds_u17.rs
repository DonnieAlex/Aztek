//! Unit u17: preloadRefEd, profiler, profilerTool, resourceManager, savePrefObjects, savePrefs, sceneLint,
//! sceneUIReplacement, scmh, scriptEditorInfo and selLoadSettings.

use aztek_host::{take_ui_requests_for_tests as take_ui_requests, Core};
use serde_json::{json, Value};

/// Preferences and the user folder in a folder of the tests' own, set before any core is made.
fn home() -> String {
    static HOME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HOME.get_or_init(|| {
        let d = std::env::temp_dir().join(format!("aztek_u17_home_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        std::env::set_var("AZTEK_PREFS", d.join("prefs"));
        std::env::set_var("AZTEK_APP_DIR", d.join("app"));
        d.to_string_lossy().replace('\\', "/")
    })
    .clone()
}

fn core() -> Core {
    home();
    Core::new()
}

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

fn dir(tag: &str) -> String {
    let d = std::env::temp_dir().join(format!("aztek_u17_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.to_string_lossy().replace('\\', "/")
}

fn num(v: &Value) -> f64 {
    v.as_f64().or_else(|| v.as_bool().map(|b| b as i64 as f64)).or_else(|| v.as_array().and_then(|a| a.first()).and_then(Value::as_f64)).unwrap_or(f64::NAN)
}

fn list(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn truthy(v: &Value) -> bool {
    v.as_bool().unwrap_or(num(v) != 0.0)
}

fn log_lines(c: &mut Core) -> Vec<String> {
    c.take_log().as_array().map(|a| a.iter().filter_map(|l| l["text"].as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn exists(c: &mut Core, n: &str) -> bool {
    truthy(&ok(c, &format!("objExists \"{n}\";")))
}

#[test]
fn profiler_records_queries_and_saves() {
    let mut c = core();
    assert!(!truthy(&ok(&mut c, "profiler -q -sampling;")));
    assert_eq!(num(&ok(&mut c, "profiler -q -bufferSize;")), 20.0);
    ok(&mut c, "profiler -bufferSize 5;");
    assert_eq!(num(&ok(&mut c, "profiler -q -b;")), 5.0);
    // Categories.
    let names = list(&ok(&mut c, "profiler -q -allCategories;"));
    assert!(names.contains(&"Maya Qt".to_string()) && names.contains(&"Command".to_string()));
    let idx = num(&ok(&mut c, "profiler -addCategory \"Test Category\" -categoryInfo \"Events related to my special test\";")) as i64;
    assert_eq!(ok(&mut c, &format!("profiler -q -categoryIndexToName {idx};")), json!("Test Category"));
    assert_eq!(num(&ok(&mut c, "profiler -q -categoryNameToIndex \"Test Category\";")) as i64, idx);
    assert_eq!(ok(&mut c, "profiler -q -categoryName \"Test Category\" -categoryInfo;"), json!("Events related to my special test"));
    assert_eq!(ok(&mut c, &format!("profiler -q -categoryIndex {idx} -categoryInfo;")), json!("Events related to my special test"));
    let with_info = list(&ok(&mut c, "profiler -q -allCategories -categoryInfo;"));
    let at = with_info.iter().position(|x| x == "Test Category").unwrap();
    assert_eq!(with_info[at + 1], "Events related to my special test");
    assert!(truthy(&ok(&mut c, "profiler -q -categoryRecording -categoryName \"Test Category\";")));
    // Recording: each command is an event in the Command category.
    ok(&mut c, "profiler -sampling true;");
    assert!(truthy(&ok(&mut c, "profiler -q -sampling;")));
    bad(&mut c, "profiler -bufferSize 10;");
    bad(&mut c, "profiler -reset;");
    ok(&mut c, "polyCube -n box; polySphere -n ball;");
    ok(&mut c, "profiler -sampling false;");
    let n = num(&ok(&mut c, "profiler -q -eventCount;")) as i64;
    assert!(n >= 2, "events: {n}");
    let mut seen = Vec::new();
    for i in 0..n {
        seen.push(ok(&mut c, &format!("profiler -q -eventName -eventIndex {i};")).as_str().unwrap().to_string());
    }
    assert!(seen.contains(&"polyCube".to_string()) && seen.contains(&"polySphere".to_string()), "{seen:?}");
    let cube = seen.iter().position(|x| x == "polyCube").unwrap();
    let cmd_cat = num(&ok(&mut c, "profiler -q -cni \"Command\";"));
    assert_eq!(num(&ok(&mut c, &format!("profiler -q -eventCategory -eventIndex {cube};"))), cmd_cat);
    assert!(num(&ok(&mut c, &format!("profiler -q -eventDuration -eventIndex {cube};"))) >= 0.0);
    assert!(num(&ok(&mut c, &format!("profiler -q -eventStartTime -eventIndex {cube};"))) > 0.0);
    assert_eq!(ok(&mut c, &format!("profiler -q -eventDescription -eventIndex {cube};")), json!("Command"));
    assert!(!truthy(&ok(&mut c, &format!("profiler -q -signalEvent -eventIndex {cube};"))));
    for flag in ["eventColor", "eventThreadId", "eventCPUId"] {
        assert!(num(&ok(&mut c, &format!("profiler -q -{flag} -eventIndex {cube};"))) >= 0.0);
    }
    bad(&mut c, "profiler -q -eventName -eventIndex 100000;");
    bad(&mut c, "profiler -q -eventName;");
    // Saved and read back.
    let d = dir("prof");
    ok(&mut c, &format!("profiler -output \"{d}/events.txt\";"));
    assert_eq!(ok(&mut c, "profiler -q -output;"), json!(format!("{d}/events.txt")));
    ok(&mut c, "profiler -reset;");
    assert_eq!(num(&ok(&mut c, "profiler -q -eventCount;")), 0.0);
    ok(&mut c, &format!("profiler -load \"{d}/events.txt\";"));
    assert_eq!(num(&ok(&mut c, "profiler -q -eventCount;")) as i64, n);
    assert_eq!(ok(&mut c, &format!("profiler -q -eventName -eventIndex {cube};")), json!("polyCube"));
    // A category turned off records nothing.
    ok(&mut c, "profiler -categoryRecording false -categoryName \"Command\";");
    assert!(!truthy(&ok(&mut c, "profiler -q -categoryRecording -categoryName \"Command\";")));
    ok(&mut c, "profiler -sampling true; polyCube; profiler -sampling false;");
    assert_eq!(num(&ok(&mut c, "profiler -q -eventCount;")), 0.0);
    ok(&mut c, "profiler -categoryRecording true -categoryName \"Command\";");
    // MEL instrumentation: a procedure timed in a category, colour and description of its own; a signal event.
    ok(&mut c, "global proc u17Work() { polyCube; } global proc u17Ping() { }");
    ok(&mut c, &format!("profiler -instrumentMel true -procedureName \"u17Work\" -categoryIndex {idx} -colorIndex 7 -procedureDescription \"Timing for work\";"));
    ok(&mut c, &format!("profiler -instrumentMel true -procedureName \"u17Ping\" -categoryIndex {idx} -colorIndex 3 -signalMelEvent true;"));
    ok(&mut c, "profiler -sampling true; u17Work; u17Ping; profiler -sampling false;");
    let n = num(&ok(&mut c, "profiler -q -eventCount;")) as i64;
    let names: Vec<String> = (0..n).map(|i| ok(&mut c, &format!("profiler -q -en -ei {i};")).as_str().unwrap().to_string()).collect();
    let w = names.iter().position(|x| x == "u17Work").expect("procedure event");
    assert_eq!(num(&ok(&mut c, &format!("profiler -q -eventColor -eventIndex {w};"))), 7.0);
    assert_eq!(num(&ok(&mut c, &format!("profiler -q -eventCategory -eventIndex {w};"))) as i64, idx);
    assert_eq!(ok(&mut c, &format!("profiler -q -eventDescription -eventIndex {w};")), json!("Timing for work"));
    let p = names.iter().position(|x| x == "u17Ping").expect("signal event");
    assert!(truthy(&ok(&mut c, &format!("profiler -q -signalEvent -eventIndex {p};"))));
    assert_eq!(num(&ok(&mut c, &format!("profiler -q -eventDuration -eventIndex {p};"))), 0.0);
    // Off again, then all off.
    ok(&mut c, "profiler -instrumentMel false -procedureName \"u17Ping\";");
    ok(&mut c, "profiler -clearAllMelInstrumentation;");
    ok(&mut c, "profiler -sampling true; u17Work; profiler -sampling false;");
    let n = num(&ok(&mut c, "profiler -q -eventCount;")) as i64;
    assert!(!(0..n).any(|i| ok(&mut c, &format!("profiler -q -en -ei {i};")) == json!("u17Work")));
    // Removing a category.
    assert_eq!(num(&ok(&mut c, "profiler -removeCategory \"Test Category\";")) as i64, idx);
    bad(&mut c, "profiler -q -categoryNameToIndex \"Test Category\";");

    // profilerTool on what was recorded.
    ok(&mut c, "profiler -sampling true; currentTime 1; polyCube -n a1; currentTime 2; polyCube -n a2; polyPlane -n a3; profiler -sampling false;");
    let segs = num(&ok(&mut c, "profilerTool -q -segmentCount;")) as i64;
    assert!(segs >= 2, "segments: {segs}");
    bad(&mut c, "profilerTool -e -isolateSegment 999;");
    ok(&mut c, "profilerTool -e -isolateSegment 0;");
    ok(&mut c, "profilerTool -e -unisolateSegment;");
    let found = ok(&mut c, "profilerTool -q -searchEvent polyCube;");
    assert!(found.as_array().unwrap().len() >= 2);
    ok(&mut c, "profilerTool -e -matchWholeWord true;");
    assert!(ok(&mut c, "profilerTool -q -searchEvent poly;").as_array().unwrap().is_empty());
    ok(&mut c, "profilerTool -e -matchWholeWord false;");
    let first = ok(&mut c, "profilerTool -q -searchEvent polyCube;").as_array().unwrap()[0].as_i64().unwrap();
    // Searching selects; -findNext moves on from the selection.
    let all_cubes = ok(&mut c, "profilerTool -q -searchEvent polyCube;");
    let last = all_cubes.as_array().unwrap().last().unwrap().as_i64().unwrap();
    let prev = ok(&mut c, "profilerTool -q -searchEvent polyCube -findPrevious;");
    assert!(prev.as_array().unwrap()[0].as_i64().unwrap() < last);
    let next = ok(&mut c, "profilerTool -q -searchEvent polyCube -findNext;");
    assert!(next.as_array().unwrap()[0].as_i64().unwrap() > first || next.as_array().unwrap()[0].as_i64().unwrap() == last);
    let types: Value = serde_json::from_str(ok(&mut c, "profilerTool -q -eventTypes false;").as_str().unwrap()).unwrap();
    let cube_type = types.as_array().unwrap().iter().find(|t| t["type"] == "polyCube").expect("polyCube type");
    assert!(cube_type["count"].as_i64().unwrap() >= 2);
    assert_eq!(cube_type["category"], json!("Command"));
    for e in ["frameAll", "frameSelected", "cpuView", "categoryView", "threadView", "showHotspot", "showCriticalPath", "collapseSelectedEvents", "expandSelectedEvents", "collapseSelectedEventsRepetition", "expandSelectedEventsRepetition"] {
        ok(&mut c, &format!("profilerTool -e -{e};"));
    }
    ok(&mut c, "profilerTool -e -showSelectedEvents true;");
    ok(&mut c, "profilerTool -e -showAllEvent false;");
    ok(&mut c, "profilerTool -e -showAllEvent true;");
    ok(&mut c, "profilerTool -e -showSelectedEventsRepetition false;");
    // The view exists once made in a layout.
    assert!(!truthy(&ok(&mut c, "profilerTool -q -exists;")));
    ok(&mut c, "window u17ProfWin; paneLayout;");
    ok(&mut c, "profilerTool -make;");
    assert!(truthy(&ok(&mut c, "profilerTool -q -exists;")));
    ok(&mut c, "profilerTool -destroy;");
    assert!(!truthy(&ok(&mut c, "profilerTool -q -ex;")));
}

#[test]
fn script_editor_info_suppresses_and_writes_history() {
    let mut c = core();
    assert_eq!(ok(&mut c, "scriptEditorInfo -q -suppressWarnings;"), json!(false));
    assert_eq!(ok(&mut c, "scriptEditorInfo -e -suppressWarnings true;"), json!("CommandWindow"));
    log_lines(&mut c);
    ok(&mut c, "warning \"hidden one\";");
    assert!(!log_lines(&mut c).iter().any(|l| l.contains("Warning: hidden one")));
    ok(&mut c, "scriptEditorInfo -e -sw false; warning \"shown one\";");
    assert!(log_lines(&mut c).iter().any(|l| l.contains("Warning: shown one")));
    ok(&mut c, "scriptEditorInfo -e -suppressResults true;");
    log_lines(&mut c);
    c.mel("polyCube -n resultBox;");
    assert!(!log_lines(&mut c).iter().any(|l| l.contains("// Result:")));
    ok(&mut c, "scriptEditorInfo -e -suppressResults false;");
    ok(&mut c, "scriptEditorInfo -e -suppressInfo true;");
    assert!(truthy(&ok(&mut c, "scriptEditorInfo -q -suppressInfo;")));
    ok(&mut c, "scriptEditorInfo -e -suppressInfo false -suppressErrors true -suppressStackWindow true;");
    assert!(truthy(&ok(&mut c, "scriptEditorInfo -q -se;")) && truthy(&ok(&mut c, "scriptEditorInfo -q -ssw;")));
    log_lines(&mut c);
    let _ = c.mel_checked("error \"quiet failure\";");
    assert!(!log_lines(&mut c).iter().any(|l| l.contains("Error: quiet failure")));
    ok(&mut c, "scriptEditorInfo -e -se false -ssw false;");
    // History to a file.
    let d = dir("hist");
    let file = format!("{d}/tempHistoryLog.txt");
    ok(&mut c, &format!("scriptEditorInfo -historyFilename \"{file}\" -writeHistory true;"));
    assert_eq!(ok(&mut c, "scriptEditorInfo -q -historyFilename;"), json!(file));
    assert!(truthy(&ok(&mut c, "scriptEditorInfo -q -writeHistory;")));
    c.mel("print \"into the file\\n\";");
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("into the file"), "{text}");
    ok(&mut c, "scriptEditorInfo -e -clearHistoryFile;");
    assert!(!std::fs::read_to_string(&file).unwrap().contains("into the file"));
    ok(&mut c, "scriptEditorInfo -writeHistory false;");
    c.mel("print \"not written\\n\";");
    assert!(!std::fs::read_to_string(&file).unwrap().contains("not written"));
    // A relative name is in the user folder.
    ok(&mut c, "scriptEditorInfo -historyFilename \"rel.txt\";");
    assert!(ok(&mut c, "scriptEditorInfo -q -hfn;").as_str().unwrap().ends_with("/rel.txt"));
    // The panes.
    take_ui_requests();
    ok(&mut c, "scriptEditorInfo -input \"// select -all; delete;\";");
    ok(&mut c, "scriptEditorInfo -e -clearHistory;");
    let reqs = list(&take_ui_requests());
    assert!(reqs.contains(&"scriptEditorInput|// select -all; delete;".to_string()), "{reqs:?}");
    assert!(reqs.contains(&"scriptEditorClear".to_string()));
}

#[test]
fn sel_load_settings_and_preload_editor() {
    let d = dir("refs");
    let mut c = core();
    ok(&mut c, "polyCube -n leaf;");
    ok(&mut c, &format!("file -rename \"{d}/grand.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"gc\" \"{d}/grand.ma\";"));
    ok(&mut c, "polySphere -n mid;");
    ok(&mut c, &format!("file -rename \"{d}/child.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"kid\" \"{d}/child.ma\";"));
    ok(&mut c, &format!("file -r -ns \"other\" \"{d}/grand.ma\";"));
    ok(&mut c, &format!("file -rename \"{d}/top.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -o -buildLoadSettings \"{d}/top.ma\";"));
    // top, kid, kid:gc, other.
    assert_eq!(num(&ok(&mut c, "selLoadSettings -q -numSettings;")), 4.0);
    assert_eq!(ok(&mut c, "selLoadSettings -q -fileName \"0\";"), json!(format!("{d}/top.ma")));
    assert_eq!(ok(&mut c, "selLoadSettings -q -fileName -shortName \"1\";"), json!("child.ma"));
    assert_eq!(ok(&mut c, "selLoadSettings -q -fileName -shortName \"2\";"), json!("grand.ma"));
    assert_eq!(ok(&mut c, "selLoadSettings -q -fileName -unresolvedName \"3\";"), json!(format!("{d}/grand.ma")));
    assert_eq!(ok(&mut c, "selLoadSettings -q -referenceNode \"1\";"), json!("kidRN"));
    assert_eq!(ok(&mut c, "selLoadSettings -q -rfn \"2\";"), json!("kid:gcRN"));
    assert_eq!(ok(&mut c, "selLoadSettings -q -rfn \"1\" \"3\";"), json!(["kidRN", "otherRN"]));
    assert_eq!(ok(&mut c, "selLoadSettings -q -deferReference \"2\";"), json!(false));
    assert_eq!(ok(&mut c, "selLoadSettings -q -proxyManager \"1\";"), json!(""));
    assert_eq!(ok(&mut c, "selLoadSettings -q -proxySetFiles -shortName \"1\";"), json!(["child.ma"]));
    assert_eq!(ok(&mut c, "selLoadSettings -q -proxySetTags \"1\";"), json!([""]));
    assert_eq!(ok(&mut c, "selLoadSettings -q -proxyTag \"1\";"), json!(""));
    ok(&mut c, "selLoadSettings -e -activeProxy \"hi\" \"1\";");
    assert_eq!(ok(&mut c, "selLoadSettings -q -activeProxy \"1\";"), json!("hi"));
    bad(&mut c, "selLoadSettings -q -fileName \"9\";");
    // Unloading a reference unloads what is under it; a child can't load under an unloaded parent.
    ok(&mut c, "selLoadSettings -e -deferReference 1 \"1\";");
    assert_eq!(ok(&mut c, "selLoadSettings -q -dr \"2\";"), json!(true));
    ok(&mut c, "selLoadSettings -e -deferReference 0 \"2\";");
    assert_eq!(ok(&mut c, "selLoadSettings -q -dr \"2\";"), json!(true));
    ok(&mut c, "selLoadSettings -e -deferReference 1 \"3\";");
    ok(&mut c, "selLoadSettings -e -deferReference 0 \"1\";");
    assert_eq!(ok(&mut c, "selLoadSettings -q -dr \"2\";"), json!(false));
    ok(&mut c, "selLoadSettings -e -deferReference 1 \"2\";");
    // Opened with them: kid loads, kid:gc and other wait.
    ok(&mut c, &format!("file -f -o -loadSettings \"implicitLoadSettings\" \"{d}/top.ma\";"));
    assert!(exists(&mut c, "kid:mid"));
    assert!(!exists(&mut c, "kid:gc:leaf"));
    assert!(!exists(&mut c, "other:leaf"));

    // The Preload Reference Editor.
    ok(&mut c, "window u17Pre; paneLayout;");
    let ed = ok(&mut c, "preloadRefEd u17Ed;").as_str().unwrap().to_string();
    assert!(ed.ends_with("u17Ed"));
    assert!(truthy(&ok(&mut c, "preloadRefEd -exists u17Ed;")));
    assert_eq!(ok(&mut c, "preloadRefEd -q -control u17Ed;"), json!(ed));
    assert_eq!(ok(&mut c, "preloadRefEd -q -selectFileNode u17Ed;"), json!("0"));
    ok(&mut c, "itemFilter -byType transform u17Filter;");
    ok(&mut c, "preloadRefEd -e -filter u17Filter -docTag \"pre\" -selectCommand \"print 1\" u17Ed;");
    assert_eq!(ok(&mut c, "preloadRefEd -q -filter u17Ed;"), json!("u17Filter"));
    assert_eq!(ok(&mut c, "preloadRefEd -q -docTag u17Ed;"), json!("pre"));
    assert_eq!(ok(&mut c, "preloadRefEd -q -selectCommand u17Ed;"), json!("print 1"));
    let state = ok(&mut c, "preloadRefEd -q -stateString u17Ed;").as_str().unwrap().to_string();
    assert!(state.starts_with("preloadRefEd -e") && state.contains("-filter \"u17Filter\"") && state.ends_with("$editorName;"), "{state}");
    ok(&mut c, "preloadRefEd -e -mainListConnection activeList -lockMainConnection u17Ed;");
    assert_eq!(ok(&mut c, "preloadRefEd -q -mainListConnection u17Ed;"), json!("activeList"));
    assert!(truthy(&ok(&mut c, "preloadRefEd -q -lockMainConnection u17Ed;")));
    ok(&mut c, "preloadRefEd -e -unlockMainConnection u17Ed;");
    assert!(!truthy(&ok(&mut c, "preloadRefEd -q -lockMainConnection u17Ed;")));
    ok(&mut c, "preloadRefEd -e -updateMainConnection -forceMainConnection modelList -highlightConnection highlightList -selectionConnection modelList u17Ed;");
    // What the shell draws.
    let rows = ok(&mut c, "aztekPreloadRefEdRows;");
    assert_eq!(rows.as_array().unwrap().len(), 4);
    assert_eq!(rows[2]["parent"], json!(1));
    assert_eq!(rows[2]["unloaded"], json!(true));
}

#[test]
fn save_prefs_writes_each_kind() {
    let mut c = core();
    let prefs = ok(&mut c, "internalVar -upd;").as_str().unwrap().to_string();
    ok(&mut c, "optionVar -iv \"u17Int\" 7 -sv \"u17Str\" \"hello\" -fv \"u17Float\" 1.5;");
    ok(&mut c, "optionVar -sv \"u17List\" \"a\" -sva \"u17List\" \"b\";");
    ok(&mut c, "savePrefs -general;");
    let text = std::fs::read_to_string(format!("{prefs}userPrefs.mel")).unwrap();
    assert!(text.contains("optionVar -iv \"u17Int\" 7;"), "{text}");
    assert!(text.contains("optionVar -sv \"u17Str\" \"hello\";"));
    assert!(text.contains("optionVar -fv \"u17Float\" 1.5;"));
    assert!(text.contains("optionVar -sv \"u17List\" \"a\" -sva \"u17List\" \"b\";"));
    // The optionVars come back in the next session.
    let ovs = std::fs::read_to_string(std::path::Path::new(&std::env::var("AZTEK_PREFS").unwrap()).join("optionVars.json")).unwrap();
    assert!(ovs.contains("u17Int"));
    // A file of its own for a category.
    ok(&mut c, "optionVar -category \"u17Cat\" -prefFile \"u17Prefs.mel\" -iv \"u17Own\" 3;");
    ok(&mut c, "savePrefs -file \"u17Prefs.mel\";");
    let own = std::fs::read_to_string(format!("{prefs}u17Prefs.mel")).unwrap();
    assert!(own.contains("optionVar -iv \"u17Own\" 3;"), "{own}");
    bad(&mut c, "savePrefs -file \"nobody.mel\";");
    // Colours, window places, hotkeys, menu sets, plug-ins.
    ok(&mut c, "displayRGBColor -create \"u17Colour\" 0.25 0.5 0.75;");
    ok(&mut c, "windowPref -width 300 -height 200 -topEdge 10 -leftEdge 20 u17Window;");
    ok(&mut c, "savePrefs -colors -uiLayout;");
    let colours = std::fs::read_to_string(format!("{prefs}userRGBColors2.mel")).unwrap();
    assert!(colours.contains("displayRGBColor -create \"u17Colour\" 0.25 0.5 0.75 1;"), "{colours}");
    let windows = std::fs::read_to_string(format!("{prefs}windowPrefs.mel")).unwrap();
    assert!(windows.contains("windowPref -topEdge 10 -leftEdge 20 -width 300 -height 200 \"u17Window\";"), "{windows}");
    // What it wrote runs as MEL.
    ok(&mut c, "displayRGBColor -resetToFactory; windowPref -remove u17Window;");
    ok(&mut c, &format!("source \"{prefs}userRGBColors2.mel\"; source \"{prefs}windowPrefs.mel\";"));
    assert_eq!(ok(&mut c, "displayRGBColor -q \"u17Colour\";"), json!([0.25, 0.5, 0.75]));
    assert!(truthy(&ok(&mut c, "windowPref -exists u17Window;")));
    let _ = std::fs::remove_file(format!("{prefs}menuSetPrefs.mel"));
    let _ = std::fs::remove_file(format!("{prefs}pluginPrefs.mel"));
    ok(&mut c, "savePrefs;");
    for f in ["userPrefs.mel", "userRGBColors2.mel", "windowPrefs.mel", "menuSetPrefs.mel", "pluginPrefs.mel"] {
        assert!(std::path::Path::new(&format!("{prefs}{f}")).is_file(), "{f}");
    }
    let hk = std::fs::read_dir(format!("{prefs}hotkeys")).unwrap().count();
    assert!(hk >= 1);
    ok(&mut c, "savePrefs -hotkeys;");
    ok(&mut c, "savePrefs -menuSets -plugins;");
}

#[test]
fn pref_objects_resources_and_lint() {
    let mut c = core();
    let prefs = ok(&mut c, "internalVar -upd;").as_str().unwrap().to_string();
    let file = format!("{prefs}userPrefObjects.ma");
    let _ = std::fs::remove_file(&file);
    assert_eq!(ok(&mut c, "savePrefObjects;"), json!(true));
    assert!(std::fs::read_to_string(&file).unwrap().starts_with("//Maya ASCII"));
    // The nodes loadPrefObjects reads are the ones saved again.
    std::fs::write(&file, "//Maya ASCII 2024 scene\nrequires maya \"2024\";\ncreateNode transform -n \"u17PrefThing\";\n// End\n").unwrap();
    assert_eq!(ok(&mut c, "loadPrefObjects;"), json!(true));
    ok(&mut c, "setAttr u17PrefThing.translateX 4;");
    assert_eq!(ok(&mut c, "savePrefObjects;"), json!(true));
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("u17PrefThing"), "{text}");
    ok(&mut c, "file -f -new;");
    assert_eq!(ok(&mut c, "loadPrefObjects;"), json!(true));
    assert_eq!(num(&ok(&mut c, "getAttr u17PrefThing.translateX;")), 4.0);

    // resourceManager: the application's images.
    let all = list(&ok(&mut c, "resourceManager;"));
    assert!(all.contains(&"aztek-icon.png".to_string()), "{all:?}");
    let a = list(&ok(&mut c, "resourceManager -nameFilter \"aztek-*.png\";"));
    assert!(!a.is_empty() && a.iter().all(|n| n.starts_with("aztek-") && n.ends_with(".png")));
    assert!(list(&ok(&mut c, "resourceManager -nf \"zz?nothing*\";")).is_empty());
    let d = dir("res");
    ok(&mut c, &format!("resourceManager -saveAs \"aztek-icon.png\" \"{d}/copy.png\";"));
    assert!(std::fs::metadata(format!("{d}/copy.png")).unwrap().len() > 100);
    bad(&mut c, &format!("resourceManager -saveAs \"no_such.png\" \"{d}/x.png\";"));

    // sceneLint.
    ok(&mut c, "file -f -new;");
    let types = list(&ok(&mut c, "sceneLint -q;"));
    assert!(types.contains(&"EMPTY_TRANSFORMS".to_string()) && types.contains(&"UNUSED_SHADERS".to_string()));
    let verbose = list(&ok(&mut c, "sceneLint -q -verbose;"));
    assert_eq!(verbose.len(), types.len() * 2);
    let two = list(&ok(&mut c, "sceneLint -issueType \"EMPTY_TRANSFORMS\" -issueType \"UNKNOWN_NODES\" -q;"));
    assert_eq!(two.len(), 4);
    assert_eq!(two[0], "EMPTY_TRANSFORMS");
    assert_eq!(two[2], "UNKNOWN_NODES");
    bad(&mut c, "sceneLint -issueType \"NOPE\";");
    let clean: Value = serde_json::from_str(ok(&mut c, "sceneLint;").as_str().unwrap()).unwrap();
    assert!(clean["sceneLint"].as_object().unwrap().is_empty(), "{clean}");
    ok(&mut c, "createNode transform -n u17Empty; polyCube -n u17Box; createNode animCurveTL -n u17Curve; shadingNode -asShader lambert -n u17Lonely;");
    let r: Value = serde_json::from_str(ok(&mut c, "sceneLint;").as_str().unwrap()).unwrap();
    let issues = &r["sceneLint"];
    let empties = list(&issues["EMPTY_TRANSFORMS"]["mitigation"][0]["objects"]);
    assert_eq!(empties, vec!["|u17Empty".to_string()]);
    assert!(issues["EMPTY_TRANSFORMS"]["description"].is_string());
    assert!(issues["EMPTY_TRANSFORMS"]["mitigation"][0]["code"].as_str().unwrap().contains("OBJECTS"));
    assert_eq!(list(&issues["UNUSED_ANIM_CURVES"]["mitigation"][0]["objects"]), vec!["u17Curve".to_string()]);
    assert_eq!(list(&issues["UNUSED_SHADERS"]["mitigation"][0]["objects"]), vec!["u17Lonely".to_string()]);
    let only: Value = serde_json::from_str(ok(&mut c, "sceneLint -i \"UNUSED_ANIM_CURVES\";").as_str().unwrap()).unwrap();
    assert_eq!(only["sceneLint"].as_object().unwrap().keys().cloned().collect::<Vec<_>>(), vec!["UNUSED_ANIM_CURVES".to_string()]);
}

#[test]
fn scene_ui_replacement_hands_out_panels() {
    let mut c = core();
    bad(&mut c, "sceneUIReplacement -getNextPanel \"modelPanel\" \"Top View\";");
    ok(&mut c, "sceneUIReplacement -update \"MainPane\";");
    assert_eq!(ok(&mut c, "sceneUIReplacement -getNextPanel \"modelPanel\" \"Top View\";"), json!("modelPanel1"));
    assert_eq!(ok(&mut c, "sceneUIReplacement -getNextPanel \"modelPanel\" \"Front View\";"), json!("modelPanel3"));
    // No second front view: the next one there is.
    assert_eq!(ok(&mut c, "sceneUIReplacement -getNextPanel \"modelPanel\" \"Front View\";"), json!("modelPanel4"));
    assert_eq!(ok(&mut c, "sceneUIReplacement -gp \"modelPanel\" \"Front View\";"), json!("modelPanel2"));
    assert_eq!(ok(&mut c, "sceneUIReplacement -gp \"modelPanel\" \"Front View\";"), json!(""));
    assert_eq!(ok(&mut c, "sceneUIReplacement -getNextScriptedPanel \"graphEditor\" \"Graph Editor\";"), json!("graphEditor1"));
    assert_eq!(ok(&mut c, "sceneUIReplacement -gsp \"graphEditor\" \"Graph Editor\";"), json!(""));
    ok(&mut c, "itemFilter -byType joint u17Joints; itemFilter -byType mesh u17Meshes;");
    assert_eq!(ok(&mut c, "sceneUIReplacement -getNextFilter \"itemFilter\" \"u17Meshes\";"), json!("u17Meshes"));
    // Again, from the top.
    ok(&mut c, "sceneUIReplacement -update \"MainPane\";");
    assert_eq!(ok(&mut c, "sceneUIReplacement -gp \"modelPanel\" \"Top View\";"), json!("modelPanel1"));
    // A scene's own panel and filter none were handed out for go.
    ok(&mut c, "scriptedPanelType u17PanelType; scriptedPanel -type \"u17PanelType\" -label \"Extra Graph\" u17Graph;");
    assert!(list(&ok(&mut c, "getPanel -all;")).contains(&"u17Graph".to_string()));
    ok(&mut c, "sceneUIReplacement -getNextFilter \"itemFilter\" \"u17Joints\";");
    ok(&mut c, "sceneUIReplacement -deleteRemaining;");
    let panels = list(&ok(&mut c, "getPanel -all;"));
    assert!(!panels.contains(&"u17Graph".to_string()), "{panels:?}");
    assert!(panels.contains(&"modelPanel4".to_string()) && panels.contains(&"graphEditor1".to_string()));
    assert!(truthy(&ok(&mut c, "itemFilter -exists u17Joints;")));
    assert!(!truthy(&ok(&mut c, "itemFilter -exists u17Meshes;")));
    ok(&mut c, "sceneUIReplacement -clear;");
    bad(&mut c, "sceneUIReplacement -deleteRemaining;");
}

#[test]
fn scmh_sets_the_active_handle() {
    let mut c = core();
    ok(&mut c, "polySphere -r 1 -n pSphere1; select -r pSphere1;");
    ok(&mut c, "manipMoveContext -e -ah 0 Move; setToolTo Move;");
    ok(&mut c, "scmh -r 10;");
    assert_eq!(num(&ok(&mut c, "getAttr pSphere1.tx;")), 10.0);
    ok(&mut c, "scmh -r 2;");
    assert_eq!(num(&ok(&mut c, "getAttr pSphere1.tx;")), 12.0);
    ok(&mut c, "scmh 3;");
    assert_eq!(num(&ok(&mut c, "getAttr pSphere1.tx;")), 3.0);
    // The centre handle takes all three; -i leaves an entry as it is.
    ok(&mut c, "setAttr pSphere1.ty 9; manipMoveContext -e -ah 3 Move;");
    ok(&mut c, "scmh -a -i 2 1 5 2;");
    assert_eq!(ok(&mut c, "getAttr pSphere1.t;"), json!([1.0, 9.0, 2.0]));
    // Rotate and scale.
    ok(&mut c, "manipRotateContext -e -ah 1 Rotate; setToolTo Rotate;");
    ok(&mut c, "scmh -r 30; scmh -r 15;");
    assert!((num(&ok(&mut c, "getAttr pSphere1.ry;")) - 45.0).abs() < 1e-6);
    ok(&mut c, "manipScaleContext -e -ah 3 Scale; setToolTo Scale;");
    ok(&mut c, "scmh 2;");
    assert_eq!(ok(&mut c, "getAttr pSphere1.s;"), json!([2.0, 2.0, 2.0]));
    ok(&mut c, "manipScaleContext -e -ah 2 Scale; scmh -r 3;");
    assert_eq!(num(&ok(&mut c, "getAttr pSphere1.sz;")), 6.0);
    // No handle: an error, unless -quiet.
    ok(&mut c, "setToolTo selectSuperContext;");
    bad(&mut c, "scmh 1;");
    ok(&mut c, "scmh -q 1;");
    bad(&mut c, "scmh;");
}


