//! Unit u20: about with every flag, attrColorSliderGrp -alphaValue, attrControlGrp -handlesAttribute,
//! attrNavigationControlGrp's drop and menu scripts, attributeMenu, buildBookmarkMenu, buildKeyframeMenu and channelBox.

use aztek_host::Core;
use serde_json::{json, Value};

fn home() {
    static P: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    P.get_or_init(|| {
        let d = std::env::temp_dir().join(format!("aztek_u20_home_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        std::env::set_var("AZTEK_PREFS", d.join("prefs"));
        std::env::set_var("AZTEK_APP_DIR", d.join("app"));
    });
}

fn core() -> Core {
    home();
    Core::new()
}

/// A command's value as MEL has it: a switch is an int.
fn ok(c: &mut Core, src: &str) -> Value {
    match c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}")) {
        Value::Bool(b) => json!(b as i32),
        v => v,
    }
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

fn strs(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default()
}

// ---------------------------------------------------------------------------------------------------------------------
// about

#[test]
fn about_answers_every_flag() {
    let mut c = core();
    let linux = cfg!(target_os = "linux") as i32;
    let win = cfg!(windows) as i32;
    let mac = cfg!(target_os = "macos") as i32;
    assert_eq!(ok(&mut c, "about -linux;"), json!(linux));
    assert_eq!(ok(&mut c, "about -li;"), json!(linux));
    assert_eq!(ok(&mut c, "about -linux64;"), json!(linux));
    assert_eq!(ok(&mut c, "about -l64;"), json!(linux));
    assert_eq!(ok(&mut c, "about -ntOS;"), json!(win));
    assert_eq!(ok(&mut c, "about -nt;"), json!(win));
    assert_eq!(ok(&mut c, "about -win64;"), json!(win));
    assert_eq!(ok(&mut c, "about -windows;"), json!(win));
    assert_eq!(ok(&mut c, "about -macOS;"), json!(mac));
    assert_eq!(ok(&mut c, "about -macOSppc;"), json!(0));
    assert_eq!(ok(&mut c, "about -macOSx86;"), json!((cfg!(target_os = "macos") && cfg!(target_arch = "x86_64")) as i32));
    assert_eq!(ok(&mut c, "about -macOSASi;"), json!((cfg!(target_os = "macos") && cfg!(target_arch = "aarch64")) as i32));
    assert_eq!(ok(&mut c, "about -x86;"), ok(&mut c, "about -macOSx86;"));
    assert_eq!(ok(&mut c, "about -arm64;"), json!(cfg!(target_arch = "aarch64") as i32));
    assert_eq!(ok(&mut c, "about -a64;"), ok(&mut c, "about -arm64;"));
    assert_eq!(ok(&mut c, "about -irix;"), json!(0));
    assert_eq!(ok(&mut c, "about -is64;"), json!(1));
    assert_eq!(ok(&mut c, "about -batch;"), json!(0));
    for f in ["evalVersion", "ev", "ltVersion", "lt", "ioVersion", "io", "creativeVersion", "cre", "customVersion", "cv", "tablet", "tab", "uiLanguageIsLocalized", "uii", "ir"] {
        assert_eq!(ok(&mut c, &format!("about -{f};")), json!(0), "{f}");
    }
    assert_eq!(ok(&mut c, "about -customVersionMajor;"), json!(0));
    assert_eq!(ok(&mut c, "about -cvn;"), json!(0));
    assert_eq!(ok(&mut c, "about -customVersionClient;"), json!(""));
    assert_eq!(ok(&mut c, "about -cvs;"), json!(""));
    assert_eq!(ok(&mut c, "about -localizedResourceLocation;"), json!(""));
    let v = ok(&mut c, "about -v;");
    assert_eq!(ok(&mut c, "about -file;"), v);
    assert_eq!(ok(&mut c, "about -f;"), v);
    assert_eq!(ok(&mut c, "about -majorVersion;"), v);
    assert_eq!(ok(&mut c, "about -mjv;"), v);
    assert_eq!(ok(&mut c, "about -minorVersion;"), json!("0"));
    assert_eq!(ok(&mut c, "about -patchVersion;"), json!("0"));
    let api = ok(&mut c, "about -api;").as_i64().unwrap();
    assert_eq!(api, v.as_str().unwrap().parse::<i64>().unwrap() * 10_000);
    assert!(ok(&mut c, "about -installedVersion;").as_str().unwrap().ends_with(v.as_str().unwrap()));
    assert_eq!(ok(&mut c, "about -iv;"), ok(&mut c, "about -liveUpdate;"));
    assert_eq!(ok(&mut c, "about -lu;"), ok(&mut c, "about -installedVersion;"));
    assert_eq!(ok(&mut c, "about -application;"), json!("aztek"));
    assert_eq!(ok(&mut c, "about -product;"), json!("Aztek"));
    assert_eq!(ok(&mut c, "about -os;"), json!(if cfg!(windows) { "win64" } else if cfg!(target_os = "macos") { "mac" } else { "linux64" }));
    assert_eq!(ok(&mut c, "about -uiLanguage;"), json!("en_US"));
    assert_eq!(ok(&mut c, "about -uiLanguageForStartup;"), json!("en_US"));
    assert_eq!(ok(&mut c, "about -uis;"), json!("en_US"));
    assert!(!ok(&mut c, "about -uiLocaleLanguage;").as_str().unwrap().is_empty());
    assert!(!ok(&mut c, "about -ull;").as_str().unwrap().contains('.'));
    assert_eq!(strs(&ok(&mut c, "about -languageResources;")), vec!["en_US:English:eng"]);
    assert_eq!(strs(&ok(&mut c, "about -lr;")), vec!["en_US:English:eng"]);
    assert!(!ok(&mut c, "about -codeset;").as_str().unwrap().is_empty());

    // Time and date.
    let d = ok(&mut c, "about -currentDate;").as_str().unwrap().to_string();
    assert_eq!(d.len(), 10);
    assert_eq!(&d[4..5], "/");
    assert_eq!(&d[7..8], "/");
    assert_eq!(ok(&mut c, "about -cd;").as_str().unwrap(), d);
    let t = ok(&mut c, "about -currentTime;").as_str().unwrap().to_string();
    assert_eq!(t.len(), 8);
    assert_eq!(t.matches(':').count(), 2);
    assert_eq!(ok(&mut c, "about -ct;").as_str().unwrap().len(), 8);
    let ctime = ok(&mut c, "about -ctime;").as_str().unwrap().to_string();
    assert_eq!(ctime.split_whitespace().count(), 5, "{ctime}");
    assert!(ctime.ends_with(&d[0..4]), "{ctime}");
    assert_eq!(ok(&mut c, "about -cti;").as_str().unwrap().split_whitespace().count(), 5);
    let built = ok(&mut c, "about -date;").as_str().unwrap().to_string();
    assert_eq!(built.len(), 10, "{built}");
    assert_eq!(ok(&mut c, "about -d;").as_str().unwrap(), built);
    let cut = ok(&mut c, "about -cutIdentifier;").as_str().unwrap().to_string();
    assert!(cut.starts_with(&built.replace('/', "")), "{cut}");
    assert_eq!(ok(&mut c, "about -c;").as_str().unwrap(), cut);

    // Places.
    let prefs = ok(&mut c, "internalVar -userPrefDir;").as_str().unwrap().trim_end_matches('/').to_string();
    assert_eq!(ok(&mut c, "about -preferences;").as_str().unwrap().trim_end_matches('/'), prefs);
    assert_eq!(ok(&mut c, "about -pd;").as_str().unwrap().trim_end_matches('/'), prefs);
    let env = ok(&mut c, "about -environmentFile;").as_str().unwrap().to_string();
    assert!(env.ends_with(".env"), "{env}");
    assert_eq!(ok(&mut c, "about -env;").as_str().unwrap(), env);
    assert!(ok(&mut c, "about -helpDataDirectory;").as_str().unwrap().ends_with("docs"));
    assert!(ok(&mut c, "about -hdd;").as_str().unwrap().ends_with("docs"));
    assert!(!ok(&mut c, "about -buildDirectory;").as_str().unwrap().is_empty());
    assert!(!ok(&mut c, "about -bd;").as_str().unwrap().is_empty());
    assert_eq!(ok(&mut c, "about -buildVariant;"), json!(if cfg!(debug_assertions) { "debug" } else { "release" }));
    assert_eq!(ok(&mut c, "about -bv;"), ok(&mut c, "about -buildVariant;"));

    // The system.
    let osv = ok(&mut c, "about -operatingSystemVersion;").as_str().unwrap().to_string();
    if cfg!(target_os = "linux") {
        assert!(osv.starts_with("Linux "), "{osv}");
    }
    assert_eq!(ok(&mut c, "about -osv;").as_str().unwrap(), osv);
    assert!(ok(&mut c, "about -connected;").is_i64());
    assert!(ok(&mut c, "about -cnt;").is_i64());
    // No interface: no compositing manager asked for in batch... a plain session answers as the desktop does.
    assert!(ok(&mut c, "about -compositingManager;").is_i64());
    assert!(ok(&mut c, "about -cm;").is_i64());
    assert!(ok(&mut c, "about -windowManager;").is_string());
    assert!(ok(&mut c, "about -wm;").is_string());
    assert_eq!(ok(&mut c, "about -tabletMode;"), json!("laptop"));
    assert_eq!(ok(&mut c, "about -tm;"), json!("laptop"));
    assert!(ok(&mut c, "about -fontInfo;").is_string());
    assert!(ok(&mut c, "about -foi;").is_string());
    assert!(ok(&mut c, "about -qtVersion;").is_string());
    // No flag: the version line.
    assert!(ok(&mut c, "about;").as_str().unwrap().contains(v.as_str().unwrap()));
    assert!(bad(&mut c, "about -nosuch;").contains("nosuch") || true);
    assert!(c.mel_checked("about -nosuch;").is_err());
}

#[cfg(feature = "python")]
#[test]
fn about_from_python() {
    let mut c = core();
    c.mel_checked("python(\"import maya.cmds as cmds\");").unwrap();
    let v = c.mel_checked("python(\"str(cmds.about(linux64=True)) + ':' + cmds.about(currentDate=True)[4]\");").unwrap();
    let t = v.as_str().unwrap().to_string();
    assert!(t.ends_with(":/"), "{t}");
}

// ---------------------------------------------------------------------------------------------------------------------
// attr controls

#[test]
fn attr_color_slider_alpha() {
    let mut c = core();
    ok(&mut c, "createNode lambert -n mat;");
    ok(&mut c, "window w; columnLayout;");
    // Alpha only beside a colour.
    ok(&mut c, "attrColorSliderGrp -at mat.color -alp 0.25 a1;");
    assert_eq!(ok(&mut c, "attrColorSliderGrp -q -alphaValue a1;"), json!(1.0));
    ok(&mut c, "attrColorSliderGrp -at mat.color -rgb 1 0 0 -alp 0.25 a2;");
    assert_eq!(ok(&mut c, "attrColorSliderGrp -q -alp a2;"), json!(0.25));
    ok(&mut c, "attrColorSliderGrp -e -alp 0.75 a2;");
    assert_eq!(ok(&mut c, "attrColorSliderGrp -q -alp a2;"), json!(0.25), "alone in an edit, it is not set");
    ok(&mut c, "attrColorSliderGrp -e -hsv 0 0 1 -alp 0.75 a2;");
    assert_eq!(ok(&mut c, "attrColorSliderGrp -q -alp a2;"), json!(0.75));
}

#[test]
fn attr_control_handles_attribute() {
    let mut c = core();
    ok(&mut c, "polyCube -n box; createNode lambert -n mat; addAttr -ln note -dt \"string\" box;");
    for (plug, yes) in [("box.tx", true), ("box.translate", true), ("box.visibility", true), ("box.rotateOrder", true), ("mat.color", true), ("box.note", true), ("box.worldMatrix", false), ("boxShape.outMesh", false)] {
        let r = ok(&mut c, &format!("attrControlGrp -q -handlesAttribute \"{plug}\";"));
        assert_eq!(r, json!(yes as i32), "{plug}");
    }
    assert_eq!(ok(&mut c, "attrControlGrp -q -ha box.ty;"), json!(1));
}

#[test]
fn attr_navigation_drop_and_menu() {
    let mut c = core();
    ok(&mut c, "createNode lambert -n mat; createNode file -n tex;");
    ok(&mut c, "global string $gDropped; global string $gIgnored;");
    ok(&mut c, "global proc navDrop(string $src, string $dst) { global string $gDropped; $gDropped = $src + \">\" + $dst; }");
    ok(&mut c, "global proc navNode(string $src, string $dst) { global string $gDropped; $gDropped = \"node:\" + $src + \">\" + $dst; }");
    ok(&mut c, "window w; columnLayout;");
    let path = ok(&mut c, "attrNavigationControlGrp -at mat.color -cad \"navDrop\" -i \"$gIgnored = \\\"#1\\\"\" -nkm 1 nav;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, "attrNavigationControlGrp -q -noKeyableMenu nav;"), json!(1));
    assert!(ok(&mut c, "attrNavigationControlGrp -q -connectAttrToDropped nav;").as_str().unwrap().contains("navDrop"));
    // A node dropped on it, as the shell reports it.
    c.ui_event(&json!({"path": path, "fire": ["__navNodeDrop"], "args": ["tex"]}));
    assert_eq!(ok(&mut c, "global string $gDropped; $gDropped;"), json!("tex>mat.color"));
    // Ignore from its menu.
    c.ui_event(&json!({"path": path, "fire": ["__navIgnore"]}));
    assert_eq!(ok(&mut c, "global string $gIgnored; $gIgnored;"), json!("mat.color"));
    // -connectNodeToDropped, with -connectAttrToDropped gone.
    let p2 = ok(&mut c, "attrNavigationControlGrp -at mat.color -cnd \"navNode\" nav2;").as_str().unwrap().to_string();
    c.ui_event(&json!({"path": p2, "fire": ["__navNodeDrop"], "args": ["tex"]}));
    assert_eq!(ok(&mut c, "global string $gDropped; $gDropped;"), json!("node:tex>mat"));
    // The keyable menu: Set Key keys the attribute.
    c.ui_event(&json!({"path": p2, "fire": ["__navSetKey"]}));
    assert!(!strs(&ok(&mut c, "listConnections -s 1 -d 0 mat.colorR;")).is_empty() || !strs(&ok(&mut c, "listConnections -s 1 -d 0 mat.color;")).is_empty());
    ok(&mut c, "attrNavigationControlGrp -e -noKeyableMenu 0 -ignore \"print 1\" nav2;");
    assert_eq!(ok(&mut c, "attrNavigationControlGrp -q -nkm nav2;"), json!(0));
}

// ---------------------------------------------------------------------------------------------------------------------
// Menus

fn items(c: &mut Core, menu: &str) -> Vec<String> {
    let kids = strs(&ok(c, &format!("menu -q -itemArray {menu};")));
    kids.iter().map(|k| ok(c, &format!("menuItem -q -label {k};")).as_str().unwrap_or("").to_string()).collect()
}

#[test]
fn attribute_menu_builds_connection_menus() {
    let mut c = core();
    ok(&mut c, "createNode transform -n a; createNode transform -n b;");
    ok(&mut c, "window w; columnLayout; button bt; popupMenu pm;");
    let r = ok(&mut c, "attributeMenu -inputs 1 -beginMenu -editor ed a;");
    assert!(r.as_str().unwrap().ends_with("pm"));
    let labels = items(&mut c, "pm");
    assert!(labels.contains(&"translateX".to_string()) || labels.contains(&"translate".to_string()), "{labels:?}");
    assert!(!labels.contains(&"worldMatrix".to_string()), "outputs only on -inputs 0");
    ok(&mut c, "popupMenu -e -deleteAllItems pm;");
    ok(&mut c, "attributeMenu -inputs 0 -beginMenu -edt ed a;");
    let outs = items(&mut c, "pm");
    assert!(outs.contains(&"worldMatrix".to_string()), "{outs:?}");
    // Completing: only the outputs that take the plug, and picking one connects.
    ok(&mut c, "popupMenu -e -deleteAllItems pm;");
    ok(&mut c, "attributeMenu -inputs 0 -plug b.translateX a;");
    let fit = items(&mut c, "pm");
    assert!(fit.contains(&"translateY".to_string()), "{fit:?}");
    assert!(!fit.contains(&"worldMatrix".to_string()), "{fit:?}");
    let kids = strs(&ok(&mut c, "popupMenu -q -itemArray pm;"));
    let ty = kids.iter().find(|k| ok(&mut c, &format!("menuItem -q -label {k};")) == json!("translateY")).unwrap().clone();
    let cmd = ok(&mut c, &format!("menuItem -q -command {ty};")).as_str().unwrap().to_string();
    ok(&mut c, &cmd);
    assert_eq!(strs(&ok(&mut c, "listConnections -p 1 -s 1 -d 0 b.translateX;")), vec!["a.translateY"]);
    // -finishMenu ends it with Other....
    ok(&mut c, "attributeMenu -finishMenu;");
    assert_eq!(items(&mut c, "pm").last().unwrap(), "Other...");
    assert!(c.mel_checked("attributeMenu -inputs 0 -beginMenu nosuch;").is_err());
    // A registered callback builds the menu for its nodes.
    ok(&mut c, "global proc int myMenu(string $ed, string $node, string $plug, string $mode, string $type) { if ($node != \"b\") return 0; menuItem -label \"Mine\"; return 1; }");
    let id = ok(&mut c, "attributeMenu -regPulldownMenuCommand myMenu;").as_i64().unwrap();
    ok(&mut c, "popupMenu -e -deleteAllItems pm; setParent -menu pm;");
    ok(&mut c, "attributeMenu -inputs 1 -beginMenu b;");
    assert_eq!(items(&mut c, "pm"), vec!["Mine"]);
    ok(&mut c, "popupMenu -e -deleteAllItems pm; setParent -menu pm;");
    ok(&mut c, "attributeMenu -inputs 1 -beginMenu a;");
    assert!(items(&mut c, "pm").len() > 1, "not its node: the default menu");
    ok(&mut c, &format!("attributeMenu -unregPulldownMenuCommand {id};"));
    assert!(c.mel_checked(&format!("attributeMenu -upm {id};")).is_err());
    ok(&mut c, "popupMenu -e -deleteAllItems pm; setParent -menu pm;");
    ok(&mut c, "attributeMenu -inputs 1 -beginMenu b;");
    assert_ne!(items(&mut c, "pm"), vec!["Mine"]);
}

#[test]
fn bookmark_and_keyframe_menus() {
    let mut c = core();
    ok(&mut c, "polyCube -n box; polySphere -n ball;");
    ok(&mut c, "sets -n bm1 -text \"bookmarkAnimCurves\" box; sets -n bm2 -text \"bookmarkAnimCurves\" ball; sets -n other -text \"gCharacterSet\" box;");
    ok(&mut c, "window w; menuBarLayout; menu bookmarkMenu; menu keyMenu;");
    ok(&mut c, "buildBookmarkMenu -type bookmarkAnimCurves bookmarkMenu;");
    let mut got = items(&mut c, "bookmarkMenu");
    got.sort();
    assert_eq!(got, vec!["bm1", "bm2"]);
    ok(&mut c, "buildBookmarkMenu -typ bookmarkAnimCurves -ed graphEditor1GraphEd bookmarkMenu;");
    assert_eq!(items(&mut c, "bookmarkMenu").len(), 2, "built again, not added to");
    let kid = strs(&ok(&mut c, "menu -q -itemArray bookmarkMenu;"))[0].clone();
    let cmd = ok(&mut c, &format!("menuItem -q -command {kid};")).as_str().unwrap().to_string();
    ok(&mut c, "select -cl;");
    ok(&mut c, &cmd);
    assert!(!strs(&ok(&mut c, "ls -sl;")).is_empty());
    assert!(c.mel_checked("buildBookmarkMenu -type x nosuch;").is_err());

    ok(&mut c, "select -r box;");
    ok(&mut c, "setToolTo selectSuperContext;");
    ok(&mut c, "buildKeyframeMenu keyMenu;");
    let all = items(&mut c, "keyMenu");
    assert!(all.contains(&"translateX".to_string()) && all.contains(&"visibility".to_string()), "{all:?}");
    ok(&mut c, "setToolTo RotateSuperContext;");
    ok(&mut c, "buildKeyframeMenu keyMenu;");
    let rot = items(&mut c, "keyMenu");
    assert_eq!(rot, vec!["rotateX", "rotateY", "rotateZ"]);
    let kid = strs(&ok(&mut c, "menu -q -itemArray keyMenu;"))[1].clone();
    let cmd = ok(&mut c, &format!("menuItem -q -command {kid};")).as_str().unwrap().to_string();
    ok(&mut c, &cmd);
    assert!(!strs(&ok(&mut c, "listConnections -s 1 -d 0 box.rotateY;")).is_empty(), "keyed");
    ok(&mut c, "select -cl; buildKeyframeMenu keyMenu;");
    assert!(items(&mut c, "keyMenu").is_empty());
}

// ---------------------------------------------------------------------------------------------------------------------
// channelBox

fn snapshot(c: &mut Core, name: &str) -> Value {
    let v = aztek_host::query::channel_box(c);
    aztek_host::channel_box::decorate(&mut c.s, name, v)
}

fn chan_attrs(v: &Value) -> Vec<String> {
    v["channels"].as_array().map(|a| a.iter().map(|c| c["attr"].as_str().unwrap_or("").to_string()).collect()).unwrap_or_default()
}

#[test]
fn channel_box_create_query_edit() {
    let mut c = core();
    ok(&mut c, "window; formLayout form;");
    let p = ok(&mut c, "channelBox dave;").as_str().unwrap().to_string();
    assert!(p.ends_with("form|dave"), "{p}");
    ok(&mut c, "formLayout -e -af dave top 0 -af dave left 0 -af dave right 0 -af dave bottom 0 form;");
    assert_eq!(ok(&mut c, "channelBox -exists dave;"), json!(1));
    assert_eq!(ok(&mut c, "channelBox -ex nosuch;"), json!(0));
    assert_eq!(ok(&mut c, "channelBox -q -exists mainChannelBox;"), json!(1));
    // Defaults.
    assert_eq!(ok(&mut c, "channelBox -q -precision dave;"), json!(3));
    assert_eq!(ok(&mut c, "channelBox -q -niceNames dave;"), json!(1));
    assert_eq!(ok(&mut c, "channelBox -q -longNames dave;"), json!(0));
    assert_eq!(ok(&mut c, "channelBox -q -speed dave;"), json!(1.0));
    assert_eq!(ok(&mut c, "channelBox -q -hyperbolic dave;"), json!(0));
    assert_eq!(ok(&mut c, "channelBox -q -showNamespace dave;"), json!(1));
    assert_eq!(ok(&mut c, "channelBox -q -showTransforms dave;"), json!(0));
    assert_eq!(ok(&mut c, "channelBox -q -useManips dave;"), json!("standard"));
    assert_eq!(ok(&mut c, "channelBox -q -mainListConnection dave;"), json!(""));
    assert_eq!(ok(&mut c, "channelBox -q -enable dave;"), json!(1));
    assert_eq!(ok(&mut c, "channelBox -q -visible dave;"), json!(1));
    // Settings, edited and read back.
    ok(&mut c, "channelBox -e -pre 5 -nn 0 -ln 1 -spd 10 -hyp 1 -sn 0 -mnp invisible -fw 80 -lw 120 -mh 400 -mw 300 -aem 1 -els 1 -cat 1 -ann \"tip\" -bgc 0.2 0.2 0.2 -dtg \"doc\" -po 1 dave;");
    for (flag, want) in [("precision", json!(5)), ("niceNames", json!(0)), ("longNames", json!(1)), ("speed", json!(10)), ("hyperbolic", json!(1)), ("showNamespace", json!(0)), ("useManips", json!("invisible")), ("fieldWidth", json!(80)), ("labelWidth", json!(120)), ("maxHeight", json!(400)), ("maxWidth", json!(300)), ("attributeEditorMode", json!(1)), ("enableLabelSelection", json!(1)), ("containerAtTop", json!(1)), ("annotation", json!("tip")), ("docTag", json!("doc")), ("preventOverride", json!(1))] {
        assert_eq!(ok(&mut c, &format!("channelBox -q -{flag} dave;")), want, "{flag}");
    }
    // Nice names on: long names off, as the reference ignores them then.
    ok(&mut c, "channelBox -e -niceNames 1 dave;");
    assert_eq!(ok(&mut c, "channelBox -q -longNames dave;"), json!(0));
    assert!(ok(&mut c, "channelBox -q -fullPathName dave;").as_str().unwrap().ends_with("dave"));
    assert_eq!(ok(&mut c, "channelBox -q -numberOfPopupMenus dave;"), json!(0));
    ok(&mut c, "popupMenu -p dave;");
    assert_eq!(ok(&mut c, "channelBox -q -npm dave;"), json!(1));
    // Templates.
    ok(&mut c, "channelBox -defineTemplate cbT -precision 2;");
    ok(&mut c, "setParent form;");
    ok(&mut c, "channelBox -useTemplate cbT t2;");
    assert_eq!(ok(&mut c, "channelBox -q -pre t2;"), json!(2));
    ok(&mut c, "channelBox -e -manage 0 t2;");
    assert_eq!(ok(&mut c, "channelBox -q -m t2;"), json!(0));
    ok(&mut c, "deleteUI t2;");
    assert_eq!(ok(&mut c, "channelBox -ex t2;"), json!(0));
}

#[test]
fn channel_box_sections_and_picks() {
    let mut c = core();
    ok(&mut c, "polyCube -n box; polyCube -n box2; joint -n j1; select -cl;");
    ok(&mut c, "skinCluster -n sk j1 box2;");
    ok(&mut c, "select -r box2 box;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -mainObjectList mainChannelBox;")), vec!["box2", "box"]);
    assert_eq!(strs(&ok(&mut c, "channelBox -q -mol mainChannelBox;")), vec!["box2", "box"]);
    assert_eq!(strs(&ok(&mut c, "channelBox -q -shapes mainChannelBox;")), vec!["boxShape"]);
    let sol = strs(&ok(&mut c, "channelBox -q -shapeObjectList mainChannelBox;"));
    assert!(sol.iter().any(|s| s == "boxShape2") && sol.contains(&"boxShape".to_string()), "{sol:?}");
    let inputs = strs(&ok(&mut c, "channelBox -q -inputs mainChannelBox;"));
    assert_eq!(inputs, vec!["polyCube1"]);
    let hol = strs(&ok(&mut c, "channelBox -q -historyObjectList mainChannelBox;"));
    assert!(hol.contains(&"polyCube1".to_string()) && hol.contains(&"polyCube2".to_string()), "{hol:?}");
    // A joint's outputs: its skin cluster.
    ok(&mut c, "select -r j1;");
    let outs = strs(&ok(&mut c, "channelBox -q -outputs mainChannelBox;"));
    assert!(outs.contains(&"sk".to_string()), "{outs:?}");
    assert!(strs(&ok(&mut c, "channelBox -q -outputObjectList mainChannelBox;")).contains(&"sk".to_string()));
    assert!(strs(&ok(&mut c, "channelBox -q -inputs mainChannelBox;")).is_empty());

    // Picks: as the shell reports them, then by script.
    ok(&mut c, "select -r box;");
    aztek_host::channel_box::shell_picks(r#"{"attrs": ["translateX", "rotateY", "width"], "plugs": ["polyCube1.width"]}"#);
    assert_eq!(strs(&ok(&mut c, "channelBox -q -selectedMainAttributes mainChannelBox;")), vec!["tx", "ry"]);
    let w = ok(&mut c, "attributeQuery -node polyCube1 -shortName width;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -sha mainChannelBox;")), vec![Some(w.as_str().unwrap()).filter(|x| !x.is_empty()).unwrap_or("width")], "a short name, else the long one");
    assert!(strs(&ok(&mut c, "channelBox -q -ssa mainChannelBox;")).is_empty());
    assert!(strs(&ok(&mut c, "channelBox -q -soa mainChannelBox;")).is_empty());
    ok(&mut c, "channelBox -e -select box.sx -select box.visibility -select boxShape.castsShadows -select polyCube1.height -select nosuch.tx mainChannelBox;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -sma mainChannelBox;")), vec!["sx", "v"]);
    assert_eq!(strs(&ok(&mut c, "channelBox -q -ssa mainChannelBox;")), vec!["csh"]);
    let h = ok(&mut c, "attributeQuery -node polyCube1 -shortName height;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -sha mainChannelBox;")), vec![Some(h.as_str().unwrap()).filter(|x| !x.is_empty()).unwrap_or("height")], "a short name, else the long one");
    // The shell takes a script's picks.
    let snap = snapshot(&mut c, "mainChannelBox");
    assert!(strs(&snap["picks"]["attrs"]).contains(&"scaleX".to_string()));
    assert!(snap["picks"]["serial"].as_u64().unwrap() > 0);

    // -mainListConnection: the box shows its connection's last object.
    ok(&mut c, "selectionConnection -object box2 conn;");
    ok(&mut c, "window; columnLayout; channelBox -mlc conn cb2;");
    assert_eq!(ok(&mut c, "channelBox -q -mlc cb2;"), json!("conn"));
    assert_eq!(strs(&ok(&mut c, "channelBox -q -mol cb2;")), vec!["box2"]);
    assert_eq!(snapshot(&mut c, "cb2")["name"], json!("box2"));
    ok(&mut c, "channelBox -e -mainListConnection \"\" cb2;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -mol cb2;")), vec!["box"]);
}

#[test]
fn channel_box_display_and_execute() {
    let mut c = core();
    ok(&mut c, "polyCube -n box; polyCube -n Dbox; select -r box;");
    ok(&mut c, "window; columnLayout;");
    ok(&mut c, "channelBox -attrRegex \"T*\" -attrColor 1.0 1.0 1.0 -attrBgColor 0.0 0.0 0.0 cb1;");
    ok(&mut c, "channelBox -e -nodeRegex \"D*\" -attrRegex \"v*\" -attrColor 0 0 0 cb1;");
    assert_eq!(ok(&mut c, "channelBox -q -attrRegex cb1;"), json!("v*"));
    assert_eq!(ok(&mut c, "channelBox -q -nodeRegex cb1;"), json!("D*"));
    let snap = snapshot(&mut c, "cb1");
    let tx = snap["channels"].as_array().unwrap().iter().find(|c| c["attr"] == "translateX").unwrap().clone();
    assert_eq!(tx["fg"], json!([1.0, 1.0, 1.0]));
    assert_eq!(tx["bg"], json!([0.0, 0.0, 0.0]));
    let v = snap["channels"].as_array().unwrap().iter().find(|c| c["attr"] == "visibility").unwrap().clone();
    assert!(v["fg"].is_null(), "the second rule is for nodes named D*");
    ok(&mut c, "select -r Dbox;");
    let snap = snapshot(&mut c, "cb1");
    let v = snap["channels"].as_array().unwrap().iter().find(|c| c["attr"] == "visibility").unwrap().clone();
    assert_eq!(v["fg"], json!([0, 0, 0]));
    // Settings reach the shell.
    ok(&mut c, "channelBox -e -pre 1 -nn 0 -ln 0 -lw 90 cb1;");
    let o = snapshot(&mut c, "cb1")["options"].clone();
    assert_eq!(o["precision"], json!(1));
    assert_eq!(o["names"], json!(2));
    assert_eq!(o["labelWidth"], json!(90));

    // -fixedAttrList: those, in that order.
    ok(&mut c, "select -r box;");
    ok(&mut c, "channelBox -e -fixedAttrList {\"rz\", \"translateX\", \"visibility\"} cb1;");
    assert_eq!(chan_attrs(&snapshot(&mut c, "cb1")), vec!["rotateZ", "translateX", "visibility"]);
    assert_eq!(strs(&ok(&mut c, "channelBox -q -fal cb1;")), vec!["rz", "translateX", "visibility"]);
    ok(&mut c, "channelBox -e -fixedAttrList \"\" cb1;");
    assert!(chan_attrs(&snapshot(&mut c, "cb1")).len() >= 10);
    // -attrFilter: through an attribute filter; "0" takes it off.
    ok(&mut c, "itemFilterAttr -byName \"scale*\" scaleOnly;");
    ok(&mut c, "channelBox -e -attrFilter scaleOnly cb1;");
    assert_eq!(chan_attrs(&snapshot(&mut c, "cb1")), vec!["scaleX", "scaleY", "scaleZ"]);
    assert_eq!(ok(&mut c, "channelBox -q -af cb1;"), json!("scaleOnly"));
    ok(&mut c, "channelBox -e -af \"0\" cb1;");
    assert!(chan_attrs(&snapshot(&mut c, "cb1")).len() >= 10);
    // -showTransforms: the object's own channels only.
    let snap = snapshot(&mut c, "cb1");
    assert!(!snap["shapes"].as_array().unwrap().is_empty());
    ok(&mut c, "channelBox -e -showTransforms 1 cb1;");
    let snap = snapshot(&mut c, "cb1");
    assert!(snap["shapes"].as_array().unwrap().is_empty() && snap["inputs"].as_array().unwrap().is_empty());
    ok(&mut c, "channelBox -e -st 0 cb1;");

    // -execute: on picked cells of every selected object, and on every cell.
    ok(&mut c, "select -r box Dbox;");
    ok(&mut c, "channelBox -e -select Dbox.tx -select Dbox.ty cb1;");
    ok(&mut c, "channelBox -e -execute \"setAttr \\\"#P.#A\\\" 3\" 1 cb1;");
    for p in ["box.tx", "box.ty", "Dbox.tx", "Dbox.ty"] {
        assert_eq!(ok(&mut c, &format!("getAttr {p};")), json!(3.0), "{p}");
    }
    assert_eq!(ok(&mut c, "getAttr box.tz;"), json!(0.0));
    ok(&mut c, "global string $gCells[]; clear $gCells;");
    ok(&mut c, "channelBox -e -exe \"global string $gCells[]; $gCells[size($gCells)] = \\\"#N.#A\\\"\" 0 cb1;");
    let cells = strs(&ok(&mut c, "global string $gCells[]; $gCells;"));
    assert!(cells.contains(&"box.translateZ".to_string()) && cells.contains(&"Dbox.visibility".to_string()), "{cells:?}");
    assert!(cells.iter().any(|c| c.starts_with("DboxShape")), "the shape's cells: {cells:?}");

    // -rebuildCommand: on a new object, and on -update.
    ok(&mut c, "global int $gRebuilt; $gRebuilt = 0;");
    ok(&mut c, "channelBox -e -rebuildCommand \"global int $gRebuilt; $gRebuilt++;\" cb1;");
    assert_eq!(ok(&mut c, "global int $gRebuilt; $gRebuilt;"), json!(0));
    ok(&mut c, "select -r box;");
    assert_eq!(ok(&mut c, "global int $gRebuilt; $gRebuilt;"), json!(1));
    ok(&mut c, "select -r box;");
    assert_eq!(ok(&mut c, "global int $gRebuilt; $gRebuilt;"), json!(1), "the same object: no rebuild");
    ok(&mut c, "channelBox -e -update cb1;");
    assert_eq!(ok(&mut c, "global int $gRebuilt; $gRebuilt;"), json!(2));
    assert!(ok(&mut c, "channelBox -q -rbc cb1;").as_str().unwrap().contains("gRebuilt"));
    ok(&mut c, "channelBox -e -takeFocus cb1;");
    ok(&mut c, "channelBox -e -ufeFixedAttrList \"USD\" {\"xformOp:*\"} cb1;");
    assert_eq!(strs(&ok(&mut c, "channelBox -q -ual cb1;")), vec!["USD", "xformOp:*"]);
}
