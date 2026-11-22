//! Unit u16: makebot, melInfo, melOptions, memory, movIn, movOut, movieInfo, nurbsToPolygonsPref, openCLInfo,
//! openGLExtension, openMayaPref, performanceOptions and pluginInfo's flags.

use aztek_host::Core;
use serde_json::{json, Value};

/// Preferences and the user folder in a folder of the tests' own, set before any core is made.
fn home() -> String {
    static HOME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HOME.get_or_init(|| {
        let d = std::env::temp_dir().join(format!("aztek_u16_home_{}", std::process::id()));
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

fn log_lines(c: &mut Core) -> Vec<String> {
    c.take_log().as_array().map(|a| a.iter().filter_map(|l| l["text"].as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn bad(c: &mut Core, src: &str) -> String {
    c.mel_checked(src).expect_err(src)
}

fn dir(tag: &str) -> String {
    let d = std::env::temp_dir().join(format!("aztek_u16_{tag}_{}", std::process::id()));
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

#[test]
fn memory_reports() {
    let mut c = core();
    let phys = num(&ok(&mut c, "memory -physicalMemory -megaByte;"));
    assert!(phys > 16.0, "physical memory in MB: {phys}");
    let as_float = num(&ok(&mut c, "memory -phy -mb -af;"));
    assert!((as_float - phys).abs() < 1.0 && as_float >= phys);
    let gb = num(&ok(&mut c, "memory -phy -gb -asFloat;"));
    assert!((gb * 1024.0 - as_float).abs() < 1.0);
    let kb = num(&ok(&mut c, "memory -phy -kiloByte;"));
    assert!((kb / 1024.0 - as_float).abs() < 2.0);
    // Bytes past an int are cut to the largest one.
    if as_float > 2048.0 {
        assert_eq!(ok(&mut c, "memory -physicalMemory;"), json!(2147483647));
    }
    for flag in ["fr", "he", "pv", "av", "pf", "pr", "sw", "swf", "swl", "swm", "swp", "swr", "swv"] {
        let v = num(&ok(&mut c, &format!("memory -{flag} -kb;")));
        assert!(v >= 0.0, "-{flag}: {v}");
    }
    assert!(num(&ok(&mut c, "memory -processVirtualMemory -mb;")) > 0.0);
    assert!(num(&ok(&mut c, "memory -heapMemory -mb -af;")) > 0.0);
    let s = ok(&mut c, "memory -summary;");
    assert!(s.as_str().unwrap().contains("physicalMemory"));
    ok(&mut c, "memory -debug;");
    bad(&mut c, "memory -mb;");
}

#[test]
fn mel_info_and_options() {
    let mut c = core();
    ok(&mut c, "global proc u16First() {} global proc u16Second() {}");
    let procs = list(&ok(&mut c, "melInfo;"));
    assert!(procs.contains(&"u16First".to_string()) && procs.contains(&"u16Second".to_string()));
    assert_eq!(ok(&mut c, "whatIs u16First;"), json!("Mel procedure entered interactively."));

    let d = dir("mel");
    let file = format!("{d}/dups.mel");
    std::fs::write(&file, "global proc u16Dups() {\n    int $a = 1;\n    int $a = 2;\n    { int $b; }\n    int $b;\n}\n").unwrap();
    let was = ok(&mut c, "melOptions -q -duplicateVariableWarnings;");
    assert!(!truthy(&was));
    log_lines(&mut c);
    ok(&mut c, &format!("source \"{file}\";"));
    assert!(!log_lines(&mut c).iter().any(|l| l.contains("declared again")), "off: no warnings");
    ok(&mut c, "melOptions -dvw true;");
    assert!(truthy(&ok(&mut c, "melOptions -q -dvw;")));
    assert!(truthy(&ok(&mut c, "optionVar -q melDuplicateVariableWarnings;")));
    ok(&mut c, &format!("source \"{file}\";"));
    let lines = log_lines(&mut c);
    let warned: Vec<&String> = lines.iter().filter(|l| l.contains("declared again")).collect();
    assert_eq!(warned.len(), 1, "only $a is declared twice in one scope: {lines:?}");
    assert!(warned[0].contains("$a"));
    // Warnings come as the file is sourced, not as it runs.
    ok(&mut c, "u16Dups;");
    assert!(!log_lines(&mut c).iter().any(|l| l.contains("declared again")));
    ok(&mut c, "melOptions -duplicateVariableWarnings false;");
    assert!(!truthy(&ok(&mut c, "melOptions -q -dvw;")));
}

#[test]
fn mov_out_and_in() {
    let mut c = core();
    let d = dir("mov");
    ok(&mut c, "polyCube -n sph; currentTime 0; move 0 0 0 sph; setKeyframe sph.t; currentTime 24; move 8 9 10 sph; setKeyframe sph.t;");
    ok(&mut c, &format!("movOut -f \"{d}/motion\" -t \"0:24\" sph.t;"));
    let text = std::fs::read_to_string(format!("{d}/motion.mov")).unwrap();
    let rows: Vec<&str> = text.lines().collect();
    assert_eq!(rows.len(), 25);
    assert_eq!(rows[0], "0.000000 0.000000 0.000000");
    assert_eq!(rows[24], "8.000000 9.000000 10.000000");
    // The same with the channels named one by one, -precision and -comment.
    ok(&mut c, &format!("movOut -f \"{d}/each.mov\" -t \"0:2\" -pre 2 -c true sph.tx sph.ty sph.tz;"));
    let each = std::fs::read_to_string(format!("{d}/each.mov")).unwrap();
    assert!(each.lines().any(|l| l.starts_with('#') && l.contains("sph.tx")));
    assert_eq!(each.lines().filter(|l| !l.starts_with('#')).count(), 3);
    assert!(each.lines().last().unwrap().split(' ').all(|v| v.split('.').nth(1).map(str::len) == Some(2)));
    // Angles go out in degrees.
    ok(&mut c, "setAttr sph.ry 90;");
    ok(&mut c, &format!("movOut -f \"{d}/rot.mov\" -t \"5:5\" sph.ry;"));
    assert_eq!(std::fs::read_to_string(format!("{d}/rot.mov")).unwrap().trim(), "90.000000");

    // In again, onto another object, from frame 45.
    ok(&mut c, "polyCube -n other; currentTime 45;");
    ok(&mut c, &format!("movIn -f \"{d}/motion.mov\" other.t;"));
    ok(&mut c, "currentTime 69;");
    assert!((num(&ok(&mut c, "getAttr other.tz;")) - 10.0).abs() < 1e-6);
    assert!((num(&ok(&mut c, "getAttr -t 45 other.ty;")) - 0.0).abs() < 1e-6);
    let keys = ok(&mut c, "keyframe -q -keyframeCount other.tx;");
    assert_eq!(num(&keys) as i64, 25);
    // -startTime, and a file of the comment kind.
    ok(&mut c, &format!("movIn -f \"{d}/each.mov\" -st 100 other.tx other.ty other.tz;"));
    assert!((num(&ok(&mut c, "getAttr -t 100 other.tx;")) - 0.0).abs() < 1e-6);
    // Columns that don't match the attributes fail.
    bad(&mut c, &format!("movIn -f \"{d}/motion.mov\" other.tx;"));
    ok(&mut c, "addAttr -ln note -dt \"string\" sph; setAttr -type \"string\" sph.note \"hi\";");
    bad(&mut c, &format!("movOut -f \"{d}/x.mov\" sph.note;"));
}

/// A small QuickTime movie: a 64 x 48 video track of 10 samples and a timecode track (30 fps, drop frame).
fn quicktime_movie(start_frame: u32, flags: u32) -> Vec<u8> {
    fn bx(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut v = ((body.len() + 8) as u32).to_be_bytes().to_vec();
        v.extend_from_slice(kind);
        v.extend_from_slice(body);
        v
    }
    let mut tkhd = vec![0u8; 84];
    tkhd[76..80].copy_from_slice(&(64u32 << 16).to_be_bytes());
    tkhd[80..84].copy_from_slice(&(48u32 << 16).to_be_bytes());
    let hdlr = |h: &[u8; 4]| {
        let mut b = vec![0u8; 24];
        b[8..12].copy_from_slice(h);
        bx(b"hdlr", &b)
    };
    let mut stsz = vec![0u8; 12];
    stsz[8..12].copy_from_slice(&10u32.to_be_bytes());
    let video = bx(b"trak", &[bx(b"tkhd", &tkhd), bx(b"mdia", &[hdlr(b"vide"), bx(b"minf", &bx(b"stbl", &bx(b"stsz", &stsz)))].concat())].concat());
    // The timecode sample entry: size, 'tmcd', 6 reserved, data ref, then reserved, flags, scale, duration, fps.
    let mut entry = vec![0u8; 8 + 8 + 20];
    entry[0..4].copy_from_slice(&36u32.to_be_bytes());
    entry[4..8].copy_from_slice(b"tmcd");
    let d = 16;
    entry[d + 4..d + 8].copy_from_slice(&flags.to_be_bytes());
    entry[d + 8..d + 12].copy_from_slice(&30000u32.to_be_bytes());
    entry[d + 12..d + 16].copy_from_slice(&1001u32.to_be_bytes());
    entry[d + 16] = 30;
    let mut stsd = vec![0u8; 8];
    stsd[4..8].copy_from_slice(&1u32.to_be_bytes());
    stsd.extend_from_slice(&entry);
    let ftyp = bx(b"ftyp", b"qt  \0\0\0\0qt  ");
    let mdat = bx(b"mdat", &start_frame.to_be_bytes());
    let mdat_data_at = ftyp.len() + 8;
    let mut stco = vec![0u8; 8];
    stco[4..8].copy_from_slice(&1u32.to_be_bytes());
    stco.extend_from_slice(&(mdat_data_at as u32).to_be_bytes());
    let tc = bx(b"trak", &bx(b"mdia", &[hdlr(b"tmcd"), bx(b"minf", &bx(b"stbl", &[bx(b"stsd", &stsd), bx(b"stco", &stco)].concat()))].concat()));
    let moov = bx(b"moov", &[video, tc].concat());
    [ftyp, mdat, moov].concat()
}

#[test]
fn movie_info_reads_movies() {
    let mut c = core();
    let d = dir("movie");
    let qt = format!("{d}/clip.mov");
    // One hour, two minutes and three seconds at 30 fps, drop frame.
    std::fs::write(&qt, quicktime_movie(111_578, 0x1)).unwrap();
    assert_eq!(ok(&mut c, &format!("movieInfo -frameCount \"{qt}\";")), json!(10));
    assert_eq!(ok(&mut c, &format!("movieInfo -w \"{qt}\";")), json!(64));
    assert_eq!(ok(&mut c, &format!("movieInfo -h \"{qt}\";")), json!(48));
    assert_eq!(ok(&mut c, &format!("movieInfo -qt \"{qt}\";")), json!(true));
    assert_eq!(ok(&mut c, &format!("movieInfo -tt \"{qt}\";")), json!(true));
    assert_eq!(ok(&mut c, &format!("movieInfo -timeScale \"{qt}\";")), json!(30000));
    assert_eq!(ok(&mut c, &format!("movieInfo -fd \"{qt}\";")), json!(1001));
    assert_eq!(ok(&mut c, &format!("movieInfo -nf \"{qt}\";")), json!(30));
    assert_eq!(ok(&mut c, &format!("movieInfo -df \"{qt}\";")), json!(true));
    assert_eq!(ok(&mut c, &format!("movieInfo -tf \"{qt}\";")), json!(false));
    assert_eq!(ok(&mut c, &format!("movieInfo -nt \"{qt}\";")), json!(false));
    assert_eq!(ok(&mut c, &format!("movieInfo -cn \"{qt}\";")), json!(false));
    assert_eq!(ok(&mut c, &format!("movieInfo -tc \"{qt}\";")), json!([1, 2, 3, 0]));
    // A counter's timecode is the frame number.
    std::fs::write(&qt, quicktime_movie(42, 0x8)).unwrap();
    assert_eq!(ok(&mut c, &format!("movieInfo -counter \"{qt}\";")), json!(true));
    assert_eq!(ok(&mut c, &format!("movieInfo -timeCode \"{qt}\";")), json!(42));

    // An AVI: its main header.
    let mut avi = b"RIFF\0\0\0\0AVI LIST\0\0\0\0hdrlavih\x38\0\0\0".to_vec();
    let mut avih = vec![0u8; 56];
    avih[16..20].copy_from_slice(&24u32.to_le_bytes());
    avih[32..36].copy_from_slice(&1024u32.to_le_bytes());
    avih[36..40].copy_from_slice(&576u32.to_le_bytes());
    avi.extend_from_slice(&avih);
    let file = format!("{d}/myMovie.avi");
    std::fs::write(&file, avi).unwrap();
    assert_eq!(ok(&mut c, &format!("movieInfo -frameCount \"{file}\";")), json!(24));
    assert_eq!(ok(&mut c, &format!("movieInfo -width \"{file}\";")), json!(1024));
    assert_eq!(ok(&mut c, &format!("movieInfo -quickTime \"{file}\";")), json!(false));
    assert_eq!(ok(&mut c, &format!("movieInfo -timeCodeTrack \"{file}\";")), json!(false));
    bad(&mut c, &format!("movieInfo -timeCode \"{file}\";"));
    // By a texture node's file.
    ok(&mut c, &format!("createNode file -n movieTex; setAttr -type \"string\" movieTex.fileTextureName \"{qt}\";"));
    assert_eq!(ok(&mut c, "movieInfo -mt movieTex -frameCount;"), json!(10));
    bad(&mut c, &format!("movieInfo -frameCount \"{d}/none.mov\";"));
}

#[test]
fn makebot_writes_tiles() {
    let mut c = core();
    let d = dir("bot");
    let img = image::RgbaImage::from_fn(40, 20, |x, y| image::Rgba([x as u8, y as u8, 7, 255]));
    let png = format!("{d}/in.png");
    img.save(&png).unwrap();
    let out = format!("{d}/out.bot");
    log_lines(&mut c);
    ok(&mut c, &format!("makebot -i \"{png}\" -o \"{out}\" -v;"));
    assert!(log_lines(&mut c).iter().any(|l| l.contains("makebot: wrote")));
    let b = std::fs::read(&out).unwrap();
    assert_eq!(&b[0..8], b"AZTKBOT1");
    let w = u32::from_le_bytes(b[8..12].try_into().unwrap());
    let h = u32::from_le_bytes(b[12..16].try_into().unwrap());
    assert_eq!((w, h), (40, 20));
    // Two tiles of 32 x 32 RGBA; the second starts at x 32, its padding repeating the last column.
    assert_eq!(b.len(), 24 + 2 * 32 * 32 * 4);
    let second = 24 + 32 * 32 * 4;
    assert_eq!(&b[second..second + 4], &[32, 0, 7, 255]);
    assert_eq!(&b[second + 31 * 4..second + 32 * 4], &[39, 0, 7, 255]);
    // -c: up to date, so not made again.
    let before = std::fs::metadata(&out).unwrap().modified().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    ok(&mut c, &format!("makebot -c -i \"{png}\" -o \"{out}\";"));
    assert_eq!(std::fs::metadata(&out).unwrap().modified().unwrap(), before);
    // -r: only when the image is big enough.
    let small = format!("{d}/small.bot");
    ok(&mut c, &format!("makebot -r 64 -i \"{png}\" -o \"{small}\";"));
    assert!(!std::path::Path::new(&small).exists());
    ok(&mut c, &format!("makebot -r 32 -i \"{png}\" -o \"{small}\";"));
    assert!(std::path::Path::new(&small).exists());
    // -nov leaves one that is there.
    std::fs::write(&small, b"keep").unwrap();
    ok(&mut c, &format!("makebot -r 32 -nov -i \"{png}\" -o \"{small}\";"));
    assert_eq!(std::fs::read(&small).unwrap(), b"keep");
    // A relative output goes in the project's source images folder.
    ok(&mut c, &format!("workspace -o \"{d}\";"));
    let _ = ok(&mut c, &format!("makebot -i \"{png}\" -o \"rel.bot\";"));
    let rule = ok(&mut c, "workspace -fileRuleEntry sourceImages;");
    let folder = rule.as_str().filter(|r| !r.is_empty()).unwrap_or("sourceimages").to_string();
    assert!(std::path::Path::new(&format!("{d}/{folder}/rel.bot")).exists(), "{d}/{folder}/rel.bot");
    bad(&mut c, "makebot -o \"x.bot\";");
    bad(&mut c, &format!("makebot -i \"{d}/none.png\";"));
}

#[test]
fn nurbs_to_polygons_pref() {
    let mut c = core();
    let d = home();
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -f;"), json!(1));
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -polyCount;"), json!(200));
    assert_eq!(num(&ok(&mut c, "nurbsToPolygonsPref -q -mergeTolerance;")), 0.1);
    ok(&mut c, "nurbsToPolygonsPref -f 0 -pc 500 -pt 1 -uch true -cht 0.05;");
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -format;"), json!(0));
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -pc;"), json!(500));
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -polyType;"), json!(1));
    assert_eq!(ok(&mut c, "nurbsToPolygonsPref -q -useChordHeight;"), json!(true));
    assert_eq!(num(&ok(&mut c, "nurbsToPolygonsPref -q -chordHeight;")), 0.05);
    bad(&mut c, "nurbsToPolygonsPref -f 7;");
    // Kept between sessions.
    let found = walk_find(std::path::Path::new(&d), "nurbsToPolygonsPrefs.json");
    assert!(found, "the settings are saved under {d}");
    // The option box's scripts read and write it.
    ok(&mut c, "nurbsToPolygonsPref -f 1 -pc 200 -pt 0 -uch false -cht 1;");
}

fn walk_find(d: &std::path::Path, name: &str) -> bool {
    std::fs::read_dir(d).into_iter().flatten().flatten().any(|e| {
        let p = e.path();
        if p.is_dir() {
            walk_find(&p, name)
        } else {
            p.file_name().is_some_and(|f| f == name)
        }
    })
}

#[test]
fn opencl_opengl_and_api_prefs() {
    let mut c = core();
    assert_eq!(ok(&mut c, "openCLInfo -q -valid;"), json!(true));
    ok(&mut c, "openCLInfo -mvb 3000;");
    assert_eq!(ok(&mut c, "openCLInfo -q -mvb;"), json!(3000));
    ok(&mut c, "openCLInfo -lmv false;");
    assert_eq!(ok(&mut c, "openCLInfo -q -limitMinimumVerts;"), json!(false));
    assert_eq!(ok(&mut c, "openCLInfo -q -dbl;"), json!(true));
    // No view made: nothing known.
    assert_eq!(ok(&mut c, "openGLExtension -renderer;"), json!(""));
    aztek_host::sys_info::set_gpu_info("Test Device", 0x10DE, "Vulkan 1.3.0", &["VK_KHR_swapchain".to_string(), "VK_EXT_foo".to_string()]);
    assert_eq!(ok(&mut c, "openGLExtension -renderer;"), json!("Test Device"));
    assert_eq!(ok(&mut c, "openGLExtension -vendor;"), json!("NVIDIA Corporation"));
    assert_eq!(ok(&mut c, "openGLExtension -version;"), json!("Vulkan 1.3.0"));
    assert_eq!(ok(&mut c, "openGLExtension -extension \"VK_EXT_foo\";"), json!("VK_EXT_foo"));
    assert_eq!(ok(&mut c, "openGLExtension -extension \"GL_ARB_multitexture\";"), json!(""));
    assert_eq!(ok(&mut c, "openGLExtension -extension \"\";"), json!("VK_KHR_swapchain VK_EXT_foo"));

    assert_eq!(ok(&mut c, "openMayaPref -q -lazyLoad;"), json!(false));
    assert_eq!(ok(&mut c, "openMayaPref -q -oldPluginWarning;"), json!(true));
    assert_eq!(ok(&mut c, "openMayaPref -lz true;"), json!(1));
    assert_eq!(ok(&mut c, "openMayaPref -q -lz;"), json!(true));
    ok(&mut c, "openMayaPref -e -lz false -ow false;");
    assert_eq!(ok(&mut c, "openMayaPref -q -ow;"), json!(false));
    ok(&mut c, "openMayaPref -ow true;");
    // The error log: a failed API call, written in the current folder.
    let d = dir("errlog");
    let was = std::env::current_dir().unwrap();
    std::env::set_current_dir(&d).unwrap();
    ok(&mut c, "openMayaPref -errlog true;");
    assert_eq!(ok(&mut c, "openMayaPref -q -errlog;"), json!(true));
    let _ = c.mel_checked("python(\"import maya.api.OpenMaya as om\\ntry:\\n    om.MSelectionList().add('noSuchNode_u16')\\nexcept Exception:\\n    pass\");");
    ok(&mut c, "openMayaPref -errlog false;");
    aztek_host::sys_info::log_api_error("(kFailure): after", "");
    std::env::set_current_dir(was).unwrap();
    let log = std::fs::read_to_string(format!("{d}/OpenMayaErrorLog")).unwrap_or_default();
    assert!(log.contains("kInvalidParameter") || log.contains("kFailure") || log.contains("noSuchNode"), "{log}");
    assert!(!log.contains("after"), "nothing is written with the log off");
}

#[test]
fn performance_options_pass_through() {
    let mut c = core();
    assert_eq!(ok(&mut c, "performanceOptions -q -passThroughCluster;"), json!("off"));
    ok(&mut c, "polyCube -n box; select box.vtx[0:3]; cluster -n cl;");
    ok(&mut c, "move -r 0 5 0 clHandle;");
    let moved = num(&ok(&mut c, "pointPosition box.vtx[0];").as_array().map(|a| a[1].clone()).unwrap_or(json!(0)));
    assert!(moved > 4.0, "the cluster moves the points: {moved}");
    ok(&mut c, "performanceOptions -pc on;");
    assert_eq!(ok(&mut c, "performanceOptions -q -pc;"), json!("on"));
    let through = num(&ok(&mut c, "pointPosition box.vtx[0];").as_array().map(|a| a[1].clone()).unwrap_or(json!(0)));
    assert!((through + 0.5).abs() < 1e-6, "passed through, the cube is as it was: {through}");
    // Interactive: only while playback runs.
    ok(&mut c, "performanceOptions -passThroughCluster interactive;");
    assert_eq!(ok(&mut c, "performanceOptions -q -passThroughCluster;"), json!("interactive"));
    let back = num(&ok(&mut c, "pointPosition box.vtx[0];").as_array().map(|a| a[1].clone()).unwrap_or(json!(0)));
    assert!(back > 4.0);
    ok(&mut c, "performanceOptions -pc off;");
    // The others: on, off, interactive or true and false.
    ok(&mut c, "performanceOptions -ds true -ps interactive -pbs on -pw off;");
    assert_eq!(ok(&mut c, "performanceOptions -q -ds;"), json!("on"));
    assert_eq!(ok(&mut c, "performanceOptions -q -passThroughSculpt;"), json!("interactive"));
    assert_eq!(ok(&mut c, "performanceOptions -q -pbs;"), json!("on"));
    for f in ["dtb", "dt", "pbf", "pdm", "pf", "pl", "pmb", "pp", "roe", "ucr", "ulr"] {
        ok(&mut c, &format!("performanceOptions -{f} interactive;"));
        assert_eq!(ok(&mut c, &format!("performanceOptions -q -{f};")), json!("interactive"), "-{f}");
        ok(&mut c, &format!("performanceOptions -{f} off;"));
    }
    ok(&mut c, "performanceOptions -ds off -ps off -pbs off;");
    bad(&mut c, "performanceOptions -pc sometimes;");
    ok(&mut c, "performanceOptions -cr 2.5 -lr 0.5 -sht true;");
    assert_eq!(num(&ok(&mut c, "performanceOptions -q -clusterResolution;")), 2.5);
    assert_eq!(num(&ok(&mut c, "performanceOptions -q -lr;")), 0.5);
    assert_eq!(ok(&mut c, "performanceOptions -q -skipHierarchyTraversal;"), json!(true));
    bad(&mut c, "performanceOptions -cr 11;");
    bad(&mut c, "performanceOptions -lr 2;");
    ok(&mut c, "performanceOptions -cr 0 -lr 0 -sht false;");
}

const PLUGIN: &str = r#"
import maya.api.OpenMaya as om
import maya.api.OpenMayaUI as omui

def maya_useNewAPI():
    pass

class U16Cmd(om.MPxCommand):
    def doIt(self, args):
        om.MPxCommand.setResult(16)

class U16Loc(omui.MPxLocatorNode):
    id = om.MTypeId(0x7f416)

def initializePlugin(obj):
    p = om.MFnPlugin(obj, "U16 Vendor", "2.5")
    p.registerCommand("u16Cmd", U16Cmd)
    p.registerNode("u16Loc", U16Loc.id, U16Loc, lambda: None, om.MPxNode.kLocatorNode)

def uninitializePlugin(obj):
    p = om.MFnPlugin(obj)
    p.deregisterCommand("u16Cmd")
    p.deregisterNode(U16Loc.id)
"#;

#[test]
fn plugin_info_flags() {
    let mut c = core();
    let prefs = home();
    let d = dir("plug");
    let file = format!("{d}/u16Plugin.py");
    std::fs::write(&file, PLUGIN).unwrap();
    ok(&mut c, "global int $u16Changed = 0; global proc u16Bump() { global int $u16Changed; $u16Changed++; }");
    ok(&mut c, "pluginInfo -changedCommand \"u16Bump\";");
    ok(&mut c, "python(\"import maya.api.OpenMaya\");");
    ok(&mut c, &format!("loadPlugin \"{file}\";"));
    c.idle();
    assert!(num(&ok(&mut c, "global int $u16Changed; $u16Changed;")) >= 1.0, "-changedCommand ran on load");
    assert!(list(&ok(&mut c, "pluginInfo -q -listPlugins;")).contains(&"u16Plugin".to_string()));
    let paths = list(&ok(&mut c, "pluginInfo -q -listPluginsPath;"));
    assert!(paths.iter().any(|p| p.ends_with("u16Plugin.py")), "{paths:?}");
    assert_eq!(ok(&mut c, "pluginInfo -q -vendor u16Plugin;"), json!("U16 Vendor"));
    assert_eq!(ok(&mut c, "pluginInfo -q -version u16Plugin;"), json!("2.5"));
    assert_eq!(ok(&mut c, "pluginInfo -q -command u16Plugin;"), json!(["u16Cmd"]));
    assert_eq!(ok(&mut c, "pluginInfo -q -dependNode u16Plugin;"), json!(["u16Loc"]));
    assert_eq!(ok(&mut c, "pluginInfo -q -dependNodeId u16Plugin;"), json!([0x7f416]));
    assert!(list(&ok(&mut c, "pluginInfo -q -dependNodeByType kLocatorNode;")).contains(&"u16Loc".to_string()));
    assert!(!list(&ok(&mut c, "pluginInfo -q -dependNodeByType kDeformerNode;")).contains(&"u16Loc".to_string()));
    assert_eq!(ok(&mut c, "pluginInfo -q -pluginForNodeType u16Loc;"), json!("u16Plugin"));
    for f in ["constraintCommand", "controlCommand", "modelEditorCommand", "tool", "data", "device", "dragAndDropBehavior", "evaluator", "iksolver", "renderer", "animCurveInterp", "serviceDescriptions", "translator"] {
        assert_eq!(ok(&mut c, &format!("pluginInfo -q -{f} u16Plugin;")), json!([]), "-{f}");
    }
    assert_eq!(ok(&mut c, "pluginInfo -q -allEvaluators;"), json!([]));
    assert_eq!(ok(&mut c, "pluginInfo -q -cacheFormat;"), json!([]));
    assert_eq!(ok(&mut c, "pluginInfo -q -userNamed u16Plugin;"), json!(false));
    assert_eq!(ok(&mut c, "pluginInfo -q -settings u16Plugin;"), json!([1, 0, 1]));

    // In use once the scene has its nodes; not safe to unload then.
    assert_eq!(ok(&mut c, "pluginInfo -q -unloadOk u16Plugin;"), json!(true));
    assert!(!list(&ok(&mut c, "pluginInfo -q -pluginsInUse;")).contains(&"u16Plugin".to_string()));
    ok(&mut c, "createNode u16Loc -n locShape1;");
    assert_eq!(ok(&mut c, "pluginInfo -q -unloadOk u16Plugin;"), json!(false));
    assert!(list(&ok(&mut c, "pluginInfo -q -pluginsInUse;")).contains(&"u16Plugin".to_string()));
    // -activeFile: only what is selected.
    ok(&mut c, "select -cl;");
    assert!(!list(&ok(&mut c, "pluginInfo -q -activeFile -pluginsInUse;")).contains(&"u16Plugin".to_string()));
    assert_eq!(ok(&mut c, "pluginInfo -q -activeFile -dependNode u16Plugin;"), json!([]));
    ok(&mut c, "select locShape1;");
    assert!(list(&ok(&mut c, "pluginInfo -q -af -pu;")).contains(&"u16Plugin".to_string()));
    // Without translators, it is among those -referenceTranslators false gives.
    assert!(list(&ok(&mut c, "pluginInfo -q -pu -rtr false;")).contains(&"u16Plugin".to_string()));
    assert!(!list(&ok(&mut c, "pluginInfo -q -pu -rtr true;")).contains(&"u16Plugin".to_string()));

    // -writeRequires: saved files require it.
    ok(&mut c, "delete `ls -type u16Loc`;");
    assert_eq!(ok(&mut c, "pluginInfo -q -writeRequires u16Plugin;"), json!(false));
    ok(&mut c, "pluginInfo -e -writeRequires true u16Plugin;");
    assert_eq!(ok(&mut c, "pluginInfo -q -wr u16Plugin;"), json!(true));
    assert!(list(&ok(&mut c, "pluginInfo -q -pluginsInUse;")).contains(&"u16Plugin".to_string()));
    let ma = format!("{d}/req.ma");
    ok(&mut c, &format!("file -rename \"{ma}\"; file -save -type \"mayaAscii\";"));
    assert!(std::fs::read_to_string(&ma).unwrap().contains("requires \"u16Plugin\""));
    ok(&mut c, "pluginInfo -e -writeRequires false u16Plugin;");

    // Autoload, and the plug-in preferences file.
    ok(&mut c, "pluginInfo -e -autoload true u16Plugin;");
    assert_eq!(ok(&mut c, "pluginInfo -q -autoload u16Plugin;"), json!(true));
    ok(&mut c, "pluginInfo -savePluginPrefs;");
    let pref = walk_text(std::path::Path::new(&prefs), "pluginPrefs.mel").expect("pluginPrefs.mel written");
    assert!(pref.contains("u16Plugin.py"));
    ok(&mut c, "pluginInfo -e -autoload false u16Plugin;");
    assert_eq!(ok(&mut c, "pluginInfo -q -autoload u16Plugin;"), json!(false));
    ok(&mut c, "pluginInfo -loadPluginPrefs;");
    assert_eq!(ok(&mut c, "pluginInfo -q -autoload u16Plugin;"), json!(true), "read back from pluginPrefs.mel");
    ok(&mut c, "pluginInfo -e -autoload false u16Plugin;");

    // -remove: only once unloaded.
    bad(&mut c, "pluginInfo -e -remove u16Plugin;");
    ok(&mut c, "unloadPlugin u16Plugin;");
    assert_eq!(ok(&mut c, "pluginInfo -q -loaded u16Plugin;"), json!(false));
    assert_eq!(ok(&mut c, &format!("pluginInfo -q -registered \"{file}\";")), json!(true));
    ok(&mut c, &format!("pluginInfo -e -remove \"{file}\";"));
    assert_eq!(ok(&mut c, &format!("pluginInfo -q -registered \"{file}\";")), json!(false));
    ok(&mut c, &format!("loadPlugin \"{file}\";"));
    assert_eq!(ok(&mut c, "pluginInfo -q -registered u16Plugin;"), json!(true));
    ok(&mut c, "unloadPlugin u16Plugin;");

    // The ones that come with Aztek: their translators and node types.
    let tr = list(&ok(&mut c, "pluginInfo -q -translator objExport;"));
    assert!(tr.contains(&"OBJ".to_string()) && tr.contains(&"OBJexport".to_string()), "{tr:?}");
    assert_eq!(ok(&mut c, "pluginInfo -q -pluginForNodeType HIKCharacterNode;"), json!("mayaHIK"));
    assert_eq!(ok(&mut c, "pluginInfo -q -pnt transform;"), json!(""));
    assert_eq!(ok(&mut c, "pluginInfo -q -unloadOk mayaHIK;"), json!(true));
    let s = ok(&mut c, "pluginInfo -q -settings fbx;");
    assert_eq!(s.as_array().map(|a| a.len()), Some(3));
}

fn walk_text(d: &std::path::Path, name: &str) -> Option<String> {
    for e in std::fs::read_dir(d).ok()?.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(t) = walk_text(&p, name) {
                return Some(t);
            }
        } else if p.file_name().is_some_and(|f| f == name) {
            return std::fs::read_to_string(p).ok();
        }
    }
    None
}
