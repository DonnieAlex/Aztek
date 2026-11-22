//! file's remaining flags, ls's, geomToBBox, illustratorCurves, itemFilter's pythonModule, itemFilterAttr,
//! itemFilterType, launch, license, linearPrecision, listNodesWithIncorrectNames, loadModule, loadPrefObjects.

use aztek_host::Core;
use serde_json::{json, Value};

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

fn dir(tag: &str) -> String {
    let d = std::env::temp_dir().join(format!("aztek_u15_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.to_string_lossy().replace('\\', "/")
}

fn num(v: &Value) -> f64 {
    v.as_f64().or_else(|| v.as_array().and_then(|a| a.first()).and_then(Value::as_f64)).unwrap_or(f64::NAN)
}

fn list(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn exists(c: &mut Core, n: &str) -> bool {
    ok(c, &format!("objExists \"{n}\";")) == json!(true) || ok(c, &format!("objExists \"{n}\";")) == json!(1)
}

#[test]
fn file_settings_and_queries() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "file -q -defaultExtensions;"), json!(true));
    assert_eq!(ok(&mut c, "file -q -renameToSave;"), json!(false));
    assert_eq!(ok(&mut c, "file -q -saveDiskCache;"), json!("always"));
    assert_eq!(ok(&mut c, "file -q -saveTextures;"), json!("unlessRef"));
    ok(&mut c, "file -saveDiskCache \"never\" -saveTextures \"always\";");
    assert_eq!(ok(&mut c, "file -q -sdc;"), json!("never"));
    assert_eq!(ok(&mut c, "file -q -stx;"), json!("always"));
    bad(&mut c, "file -saveTextures \"sometimes\";");
    ok(&mut c, "file -uiConfiguration false -uiLoadConfiguration false;");
    assert_eq!(ok(&mut c, "file -q -uc;"), json!(false));
    assert_eq!(ok(&mut c, "file -q -ulc;"), json!(false));
    ok(&mut c, "file -command \"print 1\" \"mel\";");
    assert_eq!(ok(&mut c, "file -q -command;"), json!(["print 1", "mel"]));
    ok(&mut c, "file -command \"\" \"\";");
    assert_eq!(ok(&mut c, "file -q -anyModified;"), json!(false));
    ok(&mut c, "polyCube;");
    assert_eq!(ok(&mut c, "file -q -amf;"), json!(true));
    // -newFile, short -new.
    ok(&mut c, "file -f -newFile;");
    assert!(!exists(&mut c, "pCube1"));
    // The error status: a failed read sets it, -resetError clears it.
    assert_eq!(ok(&mut c, "file -q -errorStatus;"), json!(false));
    bad(&mut c, "file -f -o \"/no/such/file.ma\";");
    assert_eq!(ok(&mut c, "file -q -err;"), json!(true));
    ok(&mut c, "file -resetError;");
    assert_eq!(ok(&mut c, "file -q -err;"), json!(false));
    ok(&mut c, "file -options \"v=0;p=17\" -type \"mayaAscii\";");
    assert_eq!(ok(&mut c, "file -q -lastFileOption;"), json!("v=0;p=17"));
    ok(&mut c, "file -moveSelected;");
}

#[test]
fn saves_compressed_through_a_temporary_file() {
    let d = dir("cmp");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    ok(&mut c, &format!("file -rename \"{d}/a.ma\";"));
    let saved = ok(&mut c, "file -save -compress -type \"mayaAscii\";");
    assert_eq!(saved, json!(format!("{d}/a.ma.gz")));
    let bytes = std::fs::read(format!("{d}/a.ma.gz")).unwrap();
    assert_eq!(&bytes[..2], &[0x1f, 0x8b]);
    assert_eq!(ok(&mut c, "file -q -lastTempFile;"), json!(format!("{d}/.a.ma.gz.tmp")));
    assert!(!std::path::Path::new(&format!("{d}/.a.ma.gz.tmp")).exists());
    ok(&mut c, &format!("file -rename \"{d}/b.ma\";"));
    ok(&mut c, "file -save -compress -preserveName;");
    assert_eq!(&std::fs::read(format!("{d}/b.ma")).unwrap()[..2], &[0x1f, 0x8b]);
    ok(&mut c, &format!("file -f -o \"{d}/b.ma\";"));
    assert!(exists(&mut c, "box"));
    // -renameToSave: a save needs a new name first.
    ok(&mut c, "file -renameToSave true;");
    bad(&mut c, "file -save;");
    ok(&mut c, &format!("file -save \"{d}/c.ma\";"));
    ok(&mut c, "file -renameToSave false;");
    // -defaultExtensions off: a name without one stays without.
    ok(&mut c, "file -defaultExtensions false;");
    ok(&mut c, "select -r box;");
    let p = ok(&mut c, &format!("file -es -type \"mayaAscii\" \"{d}/noext\";"));
    assert_eq!(p, json!(format!("{d}/noext")));
    ok(&mut c, "file -defaultExtensions true;");
}

#[test]
fn strict_preview_and_animation_exports() {
    let d = dir("exp");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; group -n grp box; select -r box;");
    let strict = list(&ok(&mut c, &format!("file -ess -preview -type \"mayaAscii\" \"{d}/s.ma\";")));
    assert_eq!(strict, vec!["box".to_string()]);
    let loose = list(&ok(&mut c, &format!("file -es -preview -type \"mayaAscii\" \"{d}/l.ma\";")));
    assert!(loose.len() > strict.len(), "{loose:?}");
    assert!(!std::path::Path::new(&format!("{d}/l.ma")).exists());
    ok(&mut c, &format!("file -ess -type \"mayaAscii\" \"{d}/s.ma\";"));
    let text = std::fs::read_to_string(format!("{d}/s.ma")).unwrap();
    assert!(text.contains("\"box\"") && !text.contains("boxShape"), "{text}");
    // Animation: the curves and connections by name.
    ok(&mut c, "setKeyframe -t 1 -v 0 -at tx box; setKeyframe -t 10 -v 5 -at tx box;");
    ok(&mut c, &format!("file -exportSelectedAnim -type \"mayaAscii\" \"{d}/anim.ma\";"));
    let text = std::fs::read_to_string(format!("{d}/anim.ma")).unwrap();
    assert!(text.contains("animCurveTL") && text.contains("connectAttr") && !text.contains("createNode mesh"), "{text}");
    ok(&mut c, "cutKey -at tx box; setAttr box.tx 0;");
    ok(&mut c, &format!("file -i \"{d}/anim.ma\";"));
    ok(&mut c, "currentTime 10;");
    assert_eq!(num(&ok(&mut c, "getAttr box.tx;")), 5.0);
    ok(&mut c, &format!("file -exportAnim -type \"mayaAscii\" \"{d}/all.ma\";"));
    assert!(std::fs::read_to_string(format!("{d}/all.ma")).unwrap().contains("animCurveTL"));
    // -relativeNamespace: written without the namespace.
    ok(&mut c, "namespace -add ns; namespace -set ns; polySphere -n ball; namespace -set \":\"; select -r \"ns:ball\";");
    ok(&mut c, &format!("file -es -relativeNamespace \"ns\" -type \"mayaAscii\" \"{d}/rel.ma\";"));
    let text = std::fs::read_to_string(format!("{d}/rel.ma")).unwrap();
    assert!(text.contains("-n \"ball\"") && !text.contains("ns:ball"), "{text}");
    assert!(exists(&mut c, "ns:ball"));
}

#[test]
fn export_as_reference_swaps_the_selection() {
    let d = dir("er");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; select -r box;");
    let rfn = ok(&mut c, &format!("file -exportAsReference -type \"mayaAscii\" -namespace \"part\" \"{d}/part.ma\";"));
    assert_eq!(rfn, json!("partRN"));
    assert!(!exists(&mut c, "box"));
    assert!(exists(&mut c, "part:box"));
    assert_eq!(ok(&mut c, "referenceQuery -isNodeReferenced \"part:box\";"), json!(true));
}

#[test]
fn import_options() {
    let d = dir("imp");
    let mut c = Core::new();
    ok(&mut c, "currentUnit -t pal; playbackOptions -min 5 -max 50; polyCube -n box;");
    ok(&mut c, &format!("file -rename \"{d}/src.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new; currentUnit -t film; playbackOptions -min 1 -max 20;");
    ok(&mut c, &format!("file -i \"{d}/src.ma\";"));
    assert_eq!(ok(&mut c, "currentUnit -q -t;"), json!("film"));
    assert_eq!(num(&ok(&mut c, "playbackOptions -q -max;")), 20.0);
    ok(&mut c, "file -f -new; currentUnit -t film; playbackOptions -min 1 -max 20;");
    ok(&mut c, &format!("file -i -importFrameRate true -importTimeRange \"override\" \"{d}/src.ma\";"));
    assert_eq!(ok(&mut c, "currentUnit -q -t;"), json!("pal"));
    assert_eq!(num(&ok(&mut c, "playbackOptions -q -min;")), 5.0);
    ok(&mut c, "file -f -new; playbackOptions -min 10 -max 30;");
    ok(&mut c, &format!("file -i -importTimeRange \"combine\" \"{d}/src.ma\";"));
    assert_eq!(num(&ok(&mut c, "playbackOptions -q -min;")), 5.0);
    assert_eq!(num(&ok(&mut c, "playbackOptions -q -max;")), 50.0);
    bad(&mut c, &format!("file -i -importTimeRange \"merge\" \"{d}/src.ma\";"));
    // -replaceName on the new nodes' names.
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -i -replaceName \"box\" \"crate\" \"{d}/src.ma\";"));
    assert!(exists(&mut c, "crate"));
    assert!(!exists(&mut c, "box"));
    // -defaultNamespace: a clash is an error and nothing stays.
    ok(&mut c, "file -f -new; polyCube -n box;");
    bad(&mut c, &format!("file -i -defaultNamespace \"{d}/src.ma\";"));
    assert!(!exists(&mut c, "box1"));
    // -removeDuplicateNetworks: the same material the scene has is not added again.
    ok(&mut c, "file -f -new; polyCube -n m1; string $sh = `shadingNode -asShader lambert -n red`; setAttr red.color 1 0 0; sets -renderable true -noSurfaceShader true -empty -name redSG; connectAttr -f red.outColor redSG.surfaceShader; sets -fe redSG m1;");
    ok(&mut c, &format!("file -rename \"{d}/mat.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, &format!("file -i -removeDuplicateNetworks \"{d}/mat.ma\";"));
    assert_eq!(list(&ok(&mut c, "ls -type lambert;")).iter().filter(|n| n.starts_with("red")).count(), 1);
    assert!(list(&ok(&mut c, "sets -q redSG;")).len() >= 2);
    // -mapPlaceHolderNamespace: a placeholder namespace in the file becomes the given one.
    std::fs::write(format!("{d}/ph.ma"), "//Maya ASCII 2024 scene\nrequires maya \"2024\";\ncurrentUnit -l centimeter -a degree -t film;\ncreateNode transform -n \"<chr>:loc\";\n// End of ph.ma\n").unwrap();
    ok(&mut c, "namespace -add hero;");
    ok(&mut c, &format!("file -i -mapPlaceHolderNamespace \"<chr>\" \"hero\" \"{d}/ph.ma\";"));
    assert!(exists(&mut c, "hero:loc"));
}

#[test]
fn opening_with_reference_load_choices() {
    let d = dir("open");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    ok(&mut c, &format!("file -rename \"{d}/child.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"kid\" \"{d}/child.ma\";"));
    ok(&mut c, &format!("file -rename \"{d}/parent.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, &format!("file -f -o -loadNoReferences \"{d}/parent.ma\";"));
    assert!(!exists(&mut c, "kid:box"));
    ok(&mut c, &format!("file -f -o -loadAllReferences \"{d}/parent.ma\";"));
    assert!(exists(&mut c, "kid:box"));
    ok(&mut c, &format!("file -f -o -loadNoReferences -reserveNamespaces \"{d}/parent.ma\";"));
    assert_eq!(ok(&mut c, "namespace -ex kid;"), json!(true));
    // -buildLoadSettings reads which references there are; -loadSettings implicitLoadSettings opens with them.
    ok(&mut c, &format!("file -o -buildLoadSettings \"{d}/parent.ma\";"));
    ok(&mut c, &format!("file -f -o -loadSettings \"implicitLoadSettings\" \"{d}/parent.ma\";"));
    assert!(exists(&mut c, "kid:box"));
    bad(&mut c, &format!("file -f -o -loadSettings \"mine\" \"{d}/parent.ma\";"));
    // -executeScriptNodes off: the file's open scripts don't run.
    ok(&mut c, "file -f -new; scriptNode -st 1 -bs \"polySphere -n fromScript;\" -n sn;");
    ok(&mut c, &format!("file -rename \"{d}/script.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, &format!("file -f -o -executeScriptNodes false \"{d}/script.ma\";"));
    assert!(!exists(&mut c, "fromScript"));
    ok(&mut c, &format!("file -f -o \"{d}/script.ma\";"));
    assert!(exists(&mut c, "fromScript"));
}

#[test]
fn reference_queries_select_lock_and_save_back() {
    let d = dir("refs");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box;");
    ok(&mut c, "file -lockFile true;");
    ok(&mut c, &format!("file -rename \"{d}/locked.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -lockFile false;");
    ok(&mut c, &format!("file -rename \"{d}/open.ma\"; file -save -type \"mayaAscii\";"));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"a\" \"{d}/open.ma\"; file -r -ns \"b\" \"{d}/open.ma\"; file -r -ns \"l\" \"{d}/locked.ma\";"));
    assert_eq!(ok(&mut c, &format!("file -q -copyNumberList \"{d}/open.ma\";")), json!(["0", "1"]));
    assert_eq!(ok(&mut c, &format!("file -q -location \"{d}/open.ma\";")), json!(format!("{d}/open.ma")));
    assert_eq!(ok(&mut c, &format!("file -q -writable \"{d}/open.ma\";")), json!(false));
    assert_eq!(ok(&mut c, &format!("file -q -parentNamespace \"{d}/open.ma\";")), json!([":"]));
    assert_eq!(ok(&mut c, "file -q -renamingPrefixList;"), json!(["a", "b", "l"]));
    assert_eq!(ok(&mut c, "file -q -namespace -absoluteName aRN;"), json!(":a"));
    // The locked file comes in locked.
    assert!(c.mel_checked("setAttr l:box.tx 3;").is_err());
    ok(&mut c, "setAttr a:box.tx 3;");
    // -selectAll, and -add to keep what was selected.
    ok(&mut c, &format!("file -selectAll \"{d}/open.ma\";"));
    let sel = list(&ok(&mut c, "ls -sl;"));
    assert!(sel.contains(&"a:box".to_string()) && !sel.contains(&"l:box".to_string()), "{sel:?}");
    ok(&mut c, &format!("file -selectAll -add \"{d}/locked.ma\";"));
    assert!(list(&ok(&mut c, "ls -sl;")).contains(&"l:box".to_string()));
    // -saveReference: the edit is part of the file now.
    ok(&mut c, "file -saveReference -referenceNode aRN;");
    let text = std::fs::read_to_string(format!("{d}/open.ma")).unwrap();
    assert!(text.contains("\"box\"") && !text.contains("a:box"), "{text}");
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -r -ns \"x\" \"{d}/open.ma\";"));
    assert_eq!(num(&ok(&mut c, "getAttr x:box.tx;")), 3.0);
    // -removeReference -mergeNamespaceWithRoot: what is left in its namespace goes to the root.
    ok(&mut c, "namespace -set x; polySphere -n extra; namespace -set \":\";");
    ok(&mut c, "file -removeReference -mergeNamespaceWithRoot -referenceNode xRN;");
    assert!(exists(&mut c, "extra"));
    assert_eq!(ok(&mut c, "namespace -ex x;"), json!(false));
    // The scene's own file can be written.
    ok(&mut c, &format!("file -rename \"{d}/main.ma\";"));
    assert_eq!(ok(&mut c, "file -q -writable;"), json!(true));
}

#[test]
fn scripts_around_a_save_and_snapshot_exports() {
    let d = dir("scripts");
    let mut c = Core::new();
    ok(&mut c, "global int $gPre = 0; global int $gPost = 0; polyCube -n box;");
    ok(&mut c, &format!("file -rename \"{d}/s.ma\";"));
    ok(&mut c, "file -save -type \"mayaAscii\" -preSaveScript \"$gPre = 1;\" -postSaveScript \"$gPost = 2;\";");
    assert_eq!(ok(&mut c, "global int $gPre; $gPre;"), json!(1));
    assert_eq!(ok(&mut c, "global int $gPost; $gPost;"), json!(2));
    // -command runs before each file operation.
    ok(&mut c, "global int $gCount = 0; file -command \"global int $gCount; $gCount++;\" \"mel\";");
    ok(&mut c, "file -q -sn;");
    ok(&mut c, "file -save;");
    assert_eq!(ok(&mut c, "global int $gCount; $gCount;"), json!(1));
    ok(&mut c, "file -command \"\" \"\";");
    // -exportSnapshotCallback: copies, changed by the script, are written; the scene is as it was.
    ok(&mut c, "select -r box;");
    ok(&mut c, &format!("file -es -type \"mayaAscii\" -exportSnapshotCallback \"move -a 7 0 0;\" \"snap\" \"{d}/snap.ma\";"));
    assert_eq!(num(&ok(&mut c, "getAttr box.tx;")), 0.0);
    assert_eq!(ok(&mut c, "namespace -ex snap;"), json!(false));
    ok(&mut c, "file -f -new;");
    ok(&mut c, &format!("file -i \"{d}/snap.ma\";"));
    assert_eq!(num(&ok(&mut c, "getAttr box.tx;")), 7.0);
}

#[test]
fn wave_time_reference() {
    let d = dir("wav");
    let mut bext = vec![0u8; 602];
    bext[338..342].copy_from_slice(&48000u32.to_le_bytes());
    let mut body: Vec<u8> = b"WAVE".to_vec();
    body.extend(b"bext");
    body.extend((bext.len() as u32).to_le_bytes());
    body.extend(&bext);
    let mut f = b"RIFF".to_vec();
    f.extend((body.len() as u32).to_le_bytes());
    f.extend(body);
    std::fs::write(format!("{d}/b.wav"), f).unwrap();
    std::fs::write(format!("{d}/plain.txt"), "x").unwrap();
    let mut c = Core::new();
    assert_eq!(ok(&mut c, &format!("file -q -fileMetaData \"{d}/b.wav\";")), json!(48000));
    assert_eq!(ok(&mut c, &format!("file -q -fileMetaData \"{d}/plain.txt\";")), json!(0));
}

#[test]
fn ls_more_kinds_and_shapes_of_answer() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; polySphere -n ball; namespace -add ns; namespace -set ns; polyCube -n inner; namespace -set \":\";");
    let ex = list(&ok(&mut c, "ls -excludeType camera;"));
    assert!(ex.contains(&"box".to_string()) && !ex.contains(&"perspShape".to_string()), "{ex:?}");
    let ex = list(&ok(&mut c, "ls -excludeType camera -excludeType mesh -dag;"));
    assert!(!ex.iter().any(|n| n.ends_with("Shape")), "{ex:?}");
    bad(&mut c, "ls -excludeType mesh -type transform;");
    assert_eq!(list(&ok(&mut c, "ls -r true \"inner\";")), vec!["ns:inner".to_string()]);
    assert_eq!(ok(&mut c, "ls -showNamespace box \"ns:inner\";"), json!(["box", ":", "ns:inner", "ns"]));
    assert_eq!(ok(&mut c, "ls -sns -an \"ns:inner\";"), json!(["ns:inner", ":ns"]));
    bad(&mut c, "ls -sns -st box;");
    assert!(list(&ok(&mut c, "ls -partitions;")).contains(&"renderPartition".to_string()));
    assert_eq!(list(&ok(&mut c, "ls -renderGlobals;")), vec!["defaultRenderGlobals".to_string()]);
    assert_eq!(ok(&mut c, "ls -renderSetups;"), ok(&mut c, "ls -rg;"));
    assert_eq!(list(&ok(&mut c, "ls -renderQualities;")), vec!["defaultRenderQuality".to_string()]);
    assert_eq!(list(&ok(&mut c, "ls -renderResolutions;")), vec!["defaultResolution".to_string()]);
    ok(&mut c, "container -n box_c;");
    assert!(list(&ok(&mut c, "ls -containers;")).contains(&"box_c".to_string()));
    assert!(list(&ok(&mut c, "ls -containerType container;")).contains(&"box_c".to_string()));
    ok(&mut c, "setAttr box.template 1;");
    assert_eq!(list(&ok(&mut c, "ls -templated;")), vec!["box".to_string()]);
    assert!(!list(&ok(&mut c, "ls -untemplated;")).contains(&"box".to_string()));
    ok(&mut c, "setAttr ball.ghosting 1;");
    assert_eq!(list(&ok(&mut c, "ls -ghost;")), vec!["ball".to_string()]);
    ok(&mut c, "plane -s 4 -n cp;");
    assert_eq!(list(&ok(&mut c, "ls -planes;")), vec!["cpShape".to_string()]);
    ok(&mut c, "makeLive ball;");
    assert!(list(&ok(&mut c, "ls -live;")).iter().any(|n| n.starts_with("ball")));
    ok(&mut c, "makeLive -n;");
    // -modified: what changed since the last save.
    let d = dir("lsmod");
    ok(&mut c, &format!("file -rename \"{d}/m.ma\"; file -save -type \"mayaAscii\";"));
    assert!(list(&ok(&mut c, "ls -modified;")).is_empty());
    ok(&mut c, "setAttr ball.tx 2;");
    assert_eq!(list(&ok(&mut c, "ls -modified -type transform;")), vec!["ball".to_string()]);
    // References, dependency nodes, persistent nodes and pre-selection.
    ok(&mut c, &format!("file -r -ns \"r\" \"{d}/m.ma\";"));
    assert_eq!(list(&ok(&mut c, "ls -references;")), vec!["rRN".to_string()]);
    assert_eq!(ok(&mut c, "size(`ls -dep`);"), ok(&mut c, "size(`ls`);"));
    ok(&mut c, "itemFilter -byType joint jf;");
    assert!(list(&ok(&mut c, "ls -persistentNodes;")).contains(&"jf".to_string()));
    assert_eq!(ok(&mut c, "ls -preSelectHilite;"), json!([]));
    // Components grouped by object.
    ok(&mut c, "select -r box.vtx[0] ball.vtx[1] box.vtx[2];");
    let ocs = list(&ok(&mut c, "ls -sl -ocs -fl;"));
    assert_eq!(ocs.first().map(|x| x.starts_with("box")), Some(true));
    assert!(ocs[1].starts_with("box"), "{ocs:?}");
    ok(&mut c, "select -cl;");
    assert_eq!(list(&ok(&mut c, "ls -allPaths -dag box;")).first().cloned(), Some("box".to_string()));
    ok(&mut c, "ls -sl -ufe;");
}

#[test]
fn geom_to_bbox_boxes() {
    let mut c = Core::new();
    ok(&mut c, "polySphere -r 1 -n a; polySphere -r 2 -n b; move 10 0 0 b; parent b a; select -r a;");
    ok(&mut c, "geomToBBox -keepOriginal -name \"bx\" -nameSuffix \"_bb\" -shaderColor 1 0 0;");
    let made = list(&ok(&mut c, "ls -sl;"));
    assert_eq!(made.len(), 2, "{made:?}");
    assert!(exists(&mut c, "a"));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.sx;", made[1]))), 4.0);
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.tx;", made[1]))), 10.0);
    let sg = list(&ok(&mut c, &format!("listConnections -type shadingEngine {}Shape;", made[0].trim_start_matches('|'))));
    assert!(!sg.is_empty());
    // One box for the hierarchy; the originals go.
    ok(&mut c, "select -r a; geomToBBox -single -name whole;");
    assert!(!exists(&mut c, "a"));
    let one = list(&ok(&mut c, "ls -sl;"));
    assert_eq!(one.len(), 1);
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.sx;", one[0]))), 13.0);
    bad(&mut c, "polyCube -n q; select -r q; geomToBBox -single -combineMesh;");
    // Combined, and baked.
    ok(&mut c, "polyCube -n p1; polyCube -n p2; move 5 0 0 p2; select -r p1 p2; geomToBBox -cm -ko;");
    assert_eq!(list(&ok(&mut c, "ls -sl;")).len(), 1);
    ok(&mut c, "polyCube -n mover; setKeyframe -t 1 -v 0 -at tx mover; setKeyframe -t 9 -v 8 -at tx mover; select -r mover;");
    ok(&mut c, "geomToBBox -ko -bakeAnimation -startTime 3 -endTime 7 -sampleBy 0.5 -n baked;");
    let b = list(&ok(&mut c, "ls -sl;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("keyframe -q -keyframeCount {b}.tx;")), json!(9));
}

#[test]
fn illustrator_files_to_curves() {
    let d = dir("ai");
    std::fs::write(format!("{d}/a.ai"), "%!PS-Adobe-3.0\n%%Creator: test\n0 0 m\n10 0 l\n10 10 20 10 20 0 c\nS\n5 5 10 10 re\nf\n").unwrap();
    let mut c = Core::new();
    let r = list(&ok(&mut c, &format!("illustratorCurves -ifn \"{d}/a.ai\";")));
    assert_eq!(r.len(), 2);
    let kids = list(&ok(&mut c, &format!("listRelatives -c {};", r[0])));
    assert_eq!(kids.len(), 2);
    assert_eq!(ok(&mut c, &format!("nodeType {};", r[1])), json!("illustratorCurves"));
    assert_eq!(ok(&mut c, &format!("illustratorCurves -q -tolerance {};", r[1])), json!(0.001));
    ok(&mut c, &format!("illustratorCurves -e -tolerance 0.5 {};", r[1]));
    assert_eq!(ok(&mut c, &format!("illustratorCurves -q -tl {};", r[1])), json!(0.5));
    assert_eq!(list(&ok(&mut c, &format!("listRelatives -c {};", r[0]))).len(), 2);
    let only = list(&ok(&mut c, &format!("illustratorCurves -object false -ifn \"{d}/a.ai\";")));
    assert_eq!(only.len(), 1);
    let no_hist = list(&ok(&mut c, &format!("illustratorCurves -ch false -ifn \"{d}/a.ai\";")));
    assert_eq!(no_hist.len(), 1);
    bad(&mut c, "illustratorCurves -ifn \"/no/such.ai\";");
}

#[test]
fn item_filters_scripts_attributes_and_types() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n boxA; polyCube -n boxB; spaceLocator -n loc;");
    ok(&mut c, "global proc int onlyA(string $n) { return `gmatch $n \"*boxA\"`; }");
    ok(&mut c, "itemFilter -byScript onlyA sf;");
    assert_eq!(list(&ok(&mut c, "lsThroughFilter sf;")), vec!["boxA".to_string()]);
    ok(&mut c, "global proc string[] keepLoc(string $n[]) { string $o[]; for ($x in $n) if (`gmatch $x \"*loc\"`) $o[size($o)] = $x; return $o; }");
    ok(&mut c, "itemFilter -secondScript keepLoc sf2;");
    assert_eq!(list(&ok(&mut c, "lsThroughFilter sf2;")), vec!["loc".to_string()]);
    ok(&mut c, "itemFilter -byScript onlyA -pythonModule \"mymod\" pf;");
    assert_eq!(ok(&mut c, "itemFilter -q -pythonModule pf;"), json!("mymod"));
    ok(&mut c, "itemFilter -classification builtIn -byType joint bif;");
    assert_eq!(ok(&mut c, "itemFilter -listBuiltInFilters;"), json!(["bif"]));
    assert!(!list(&ok(&mut c, "itemFilter -listUserFilters;")).contains(&"bif".to_string()));
    // Attribute filters.
    let srt = ok(&mut c, "itemFilterAttr -scaleRotateTranslate true;");
    let srt = srt.as_str().unwrap().to_string();
    ok(&mut c, "itemFilterAttr -keyable true keyf;");
    ok(&mut c, &format!("itemFilterAttr -intersect {srt} keyf both;"));
    let plugs = list(&ok(&mut c, "lsThroughFilter -item boxA both;"));
    assert!(plugs.contains(&"boxA.translateX".to_string()) && !plugs.contains(&"boxA.visibility".to_string()), "{plugs:?}");
    ok(&mut c, "setKeyframe -at ty boxA;");
    ok(&mut c, "itemFilterAttr -hasCurve true curvef;");
    assert_eq!(list(&ok(&mut c, "lsThroughFilter -item boxA curvef;")), vec!["boxA.translateY".to_string()]);
    ok(&mut c, "itemFilterAttr -byName \"rotate?\" rf; itemFilterAttr -e -negate true rf;");
    assert!(!list(&ok(&mut c, "lsThroughFilter -item boxA rf;")).contains(&"boxA.rotateX".to_string()));
    assert_eq!(ok(&mut c, "itemFilterAttr -q -byName rf;"), json!("rotate?"));
    ok(&mut c, "itemFilterAttr -byNameString \"tx\" -byNameString \"visibility\" nsf;");
    assert_eq!(list(&ok(&mut c, "lsThroughFilter -item boxA nsf;")), vec!["boxA.visibility".to_string()]);
    bad(&mut c, "itemFilterAttr -byName x -byScript y;");
    assert!(list(&ok(&mut c, "itemFilterAttr -listUserFilters;")).contains(&"keyf".to_string()));
    // Types and notes.
    assert_eq!(ok(&mut c, "itemFilterType -q -type sf;"), json!("itemFilter"));
    assert_eq!(ok(&mut c, "itemFilterType -q -type keyf;"), json!("attributeFilter"));
    assert_eq!(ok(&mut c, "itemFilterType -q -type nothing;"), json!("unknownFilter"));
    ok(&mut c, "itemFilterType -e -text \"a note\" keyf;");
    assert_eq!(ok(&mut c, "itemFilterType -q -text keyf;"), json!("a note"));
    // delete takes filters away.
    ok(&mut c, "delete sf keyf;");
    assert_eq!(ok(&mut c, "itemFilterType -q -type sf;"), json!("unknownFilter"));
    assert_eq!(ok(&mut c, "itemFilter -ex sf;"), json!(false));
}

#[test]
fn launch_license_precision_and_names() {
    let d = dir("misc");
    let mut c = Core::new();
    ok(&mut c, "launch -web \"https://example.com\";");
    bad(&mut c, "launch -movie \"clip.avi\";");
    bad(&mut c, "launch -pdf \"doc.txt\";");
    bad(&mut c, "launch -pdf \"/no/such.pdf\";");
    ok(&mut c, &format!("launch -dir \"{d}\";"));
    bad(&mut c, "launch;");
    let v = ok(&mut c, "license;");
    assert!(v.as_str().unwrap().starts_with("Aztek"));
    ok(&mut c, "license -showProductInfoDialog;");
    assert_eq!(ok(&mut c, "license -isTrial;"), json!("0"));
    assert_eq!(ok(&mut c, "license -status;"), json!("licensed"));
    ok(&mut c, "linearPrecision 4;");
    assert_eq!(ok(&mut c, "linearPrecision -q;"), json!(4));
    ok(&mut c, "linearPrecision 10;");
    assert_eq!(ok(&mut c, "linearPrecision -q;"), json!(6));
    assert!(aztek_host::sys_more::valid_node_name("ns:pCube_1"));
    assert!(!aztek_host::sys_more::valid_node_name("1abc"));
    assert!(!aztek_host::sys_more::valid_node_name("a b"));
    ok(&mut c, "listNodesWithIncorrectNames;");
}

#[test]
fn modules_and_pref_objects() {
    let d = dir("mods");
    let mods = format!("{d}/modules");
    std::fs::create_dir_all(format!("{mods}/late/scripts")).unwrap();
    let was = std::env::var_os("AZTEK_MODULE_PATH");
    let mut paths: Vec<std::path::PathBuf> = was.as_ref().map(|v| std::env::split_paths(v).collect()).unwrap_or_default();
    paths.push(mods.clone().into());
    let mut c = Core::new();
    // The user folder in force, as start-up puts it, before the module file is added.
    ok(&mut c, "moduleInfo -listModules;");
    std::env::set_var("AZTEK_MODULE_PATH", std::env::join_paths(paths).unwrap());
    // Added after start-up: only loadModule reads it.
    std::fs::write(format!("{mods}/late.mod"), "+ lateModule 1.0 ./late\n").unwrap();
    let found = list(&ok(&mut c, "loadModule -scan;"));
    assert!(found.contains(&format!("{mods}/late.mod")), "{found:?}");
    assert!(!list(&ok(&mut c, "moduleInfo -listModules;")).contains(&"lateModule".to_string()));
    assert_eq!(ok(&mut c, &format!("loadModule -load \"{mods}/late.mod\";")), json!(["lateModule"]));
    assert!(list(&ok(&mut c, "moduleInfo -listModules;")).contains(&"lateModule".to_string()));
    assert!(!list(&ok(&mut c, "loadModule -scan;")).contains(&format!("{mods}/late.mod")));
    ok(&mut c, "loadModule -allModules;");
    bad(&mut c, "loadModule -load \"/no/such.mod\";");
    // loadPrefObjects: the preferences folder's userPrefObjects.ma, when there is one.
    let prefs = ok(&mut c, "internalVar -upd;").as_str().unwrap().to_string();
    let file = format!("{prefs}userPrefObjects.ma");
    let had = std::path::Path::new(&file).exists();
    if !had {
        std::fs::create_dir_all(&prefs).unwrap();
        std::fs::write(&file, "//Maya ASCII 2024 scene\nrequires maya \"2024\";\ncreateNode transform -n \"prefThing\";\n// End\n").unwrap();
        assert_eq!(ok(&mut c, "loadPrefObjects;"), json!(true));
        assert!(exists(&mut c, "prefThing"));
        let _ = std::fs::remove_file(&file);
        assert_eq!(ok(&mut c, "loadPrefObjects;"), json!(false));
    }
}
