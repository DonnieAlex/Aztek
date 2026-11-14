//! Character sets and clips in scene files, in the reference's layout (tests/data/ma_characters).

use aztek_scene::{ma_file, mb_file, Session};
use serde_json::{json, Value};

fn data(name: &str) -> String {
    format!("{}/tests/data/ma_characters/{name}", env!("CARGO_MANIFEST_DIR"))
}

fn temp(name: &str) -> String {
    let dir = std::env::temp_dir().join(format!("aztek_ma_characters_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name).to_str().unwrap().to_string()
}

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn at(s: &mut Session, frame: f64, plug: &str) -> f64 {
    run(s, "currentTime", json!({"args": [frame]}));
    run(s, "getAttr", json!({"args": [plug]})).as_f64().unwrap_or_else(|| panic!("{plug}"))
}

fn close(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() < 1e-6, "{what}: {a} against {b}");
}

/// What the reference's file holds: two joints in a character, two clips one after the other.
fn check_reference_scene(s: &mut Session) {
    let members = run(s, "character", json!({"args": ["character1"], "query": true}));
    assert_eq!(members.as_array().map(Vec::len), Some(12), "{members}");
    assert_eq!(run(s, "clip", json!({"args": ["character1"], "query": true, "queried": ["allClips"]})), json!(["clip1", "clip2"]));
    let mut sources = run(s, "clip", json!({"args": ["character1"], "query": true, "queried": ["allSourceClips"]})).as_array().cloned().unwrap();
    sources.sort_by_key(|v| v.to_string());
    assert_eq!(sources, vec![json!("clip1Source"), json!("clip2Source")]);
    let lines = run(s, "clipSchedule", json!({"args": ["character1Scheduler1"], "query": true}));
    assert_eq!(lines, json!(["clip1,0,1,1,20,1,0,1,0", "clip2,1,31,31,50,1,0,1,0"]));
    // The ends of each clip, as their curves key them.
    close(at(s, 20.0, "joint1.rotateY"), -45.0, "joint1.ry at 20");
    close(at(s, 20.0, "joint2.translateZ"), 3.0, "joint2.tz at 20");
    close(at(s, 50.0, "joint1.rotateY"), 49.307430731008935, "joint1.ry at 50");
    close(at(s, 50.0, "joint2.translateZ"), -3.0, "joint2.tz at 50");
    close(at(s, 1.0, "joint2.translateX"), 2.0, "joint2.tx at 1");
}

#[test]
fn the_reference_file_reads() {
    let mut s = Session::new();
    let w = ma_file::open_scene(&mut s, &data("TwoJointsLogicalParentAnim.ma")).unwrap();
    assert!(!w.iter().any(|x| x.contains("character") || x.contains("clip") || x.contains("animClip")), "{w:?}");
    check_reference_scene(&mut s);
}

#[test]
fn the_reference_file_saved_again_reads_the_same() {
    for ext in ["ma", "mb"] {
        let mut s = Session::new();
        ma_file::open_scene(&mut s, &data("TwoJointsLogicalParentAnim.ma")).unwrap();
        let out = temp(&format!("again.{ext}"));
        let w = ma_file::save_scene(&mut s, &out, ma_file::Format::of_path(&out)).unwrap();
        assert!(!w.iter().any(|x| x.contains("character") || x.contains("clip") || x.contains("left out")), "{ext}: {w:?}");
        if ext == "ma" {
            let text = std::fs::read_to_string(&out).unwrap();
            for line in ["createNode character -n \"character1\";", "-type \"characterMapping\" 12 ", "createNode clipLibrary -n \"character1Clips1\";", "createNode animClip -n \"clip2Source\";", "connectAttr \"character1.lv[1]\"", "\"character1Scheduler1.ce\" \"character1.cer\""] {
                assert!(text.contains(line), "{line}");
            }
        } else {
            assert!(mb_file::is_binary(&std::fs::read(&out).unwrap()));
        }
        let mut t = Session::new();
        let w = ma_file::open_scene(&mut t, &out).unwrap();
        assert!(!w.iter().any(|x| x.contains("character") || x.contains("clip")), "{ext}: {w:?}");
        check_reference_scene(&mut t);
    }
}

#[test]
fn a_character_made_here_saves_both_ways() {
    for ext in ["ma", "mb"] {
        let mut s = Session::new();
        run(&mut s, "polyCube", json!({"name": "box"}));
        run(&mut s, "polyCube", json!({"name": "lid"}));
        run(&mut s, "character", json!({"args": ["box.tx", "box.ry", "lid.sy"], "name": "hero"}));
        run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 1, "value": 0}));
        run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 11, "value": 10}));
        run(&mut s, "setKeyframe", json!({"args": ["box.ry"], "time": 1, "value": 0}));
        run(&mut s, "setKeyframe", json!({"args": ["box.ry"], "time": 11, "value": 90}));
        run(&mut s, "setKeyframe", json!({"args": ["lid.sy"], "time": 1, "value": 1}));
        run(&mut s, "setKeyframe", json!({"args": ["lid.sy"], "time": 11, "value": 3}));
        run(&mut s, "clip", json!({"args": ["hero"], "name": "walk"}));
        run(&mut s, "clipSchedule", json!({"args": ["heroScheduler"], "instance": "walkSource", "start": 21, "scale": 2.0}));
        let before = run(&mut s, "clipSchedule", json!({"args": ["heroScheduler"], "query": true}));
        let frames = [1.0, 6.0, 11.0, 15.0, 21.0, 31.0, 41.0];
        let plugs = ["box.tx", "box.ry", "lid.sy"];
        let want: Vec<f64> = frames.iter().flat_map(|&f| plugs.iter().map(move |p| (f, *p))).map(|(f, p)| at(&mut s, f, p)).collect();
        let out = temp(&format!("hero.{ext}"));
        let w = ma_file::save_scene(&mut s, &out, ma_file::Format::of_path(&out)).unwrap();
        assert!(w.is_empty(), "{ext}: {w:?}");
        let mut t = Session::new();
        let w = ma_file::open_scene(&mut t, &out).unwrap();
        assert!(w.is_empty(), "{ext}: {w:?}");
        assert_eq!(run(&mut t, "character", json!({"args": ["hero"], "query": true})), json!(["box.translateX", "box.rotateY", "lid.scaleY"]));
        assert_eq!(run(&mut t, "clipSchedule", json!({"args": ["heroScheduler"], "query": true})), before, "{ext}");
        let got: Vec<f64> = frames.iter().flat_map(|&f| plugs.iter().map(move |p| (f, *p))).map(|(f, p)| at(&mut t, f, p)).collect();
        for ((a, b), (f, p)) in got.iter().zip(&want).zip(frames.iter().flat_map(|&f| plugs.iter().map(move |p| (f, *p)))) {
            close(*a, *b, &format!("{ext} {p} at {f}"));
        }
    }
}

#[test]
fn a_character_with_keys_and_no_clips() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"name": "box"}));
    run(&mut s, "character", json!({"args": ["box.tx", "box.ty"], "name": "hero"}));
    run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 1, "value": 2}));
    run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 9, "value": 6}));
    let out = temp("keys.ma");
    ma_file::save_scene(&mut s, &out, ma_file::Format::Ma).unwrap();
    let text = std::fs::read_to_string(&out).unwrap();
    // The keys go to the character's value, which drives the member, as the reference keys a character member.
    assert!(text.contains("connectAttr \"box_translateX.o\" \"hero.lv[1]\";"), "{text}");
    assert!(text.contains("connectAttr \"hero.lv[1]\" \"box.translateX\";"));
    let mut t = Session::new();
    ma_file::open_scene(&mut t, &out).unwrap();
    assert_eq!(run(&mut t, "character", json!({"args": ["hero"], "query": true})), json!(["box.translateX", "box.translateY"]));
    close(at(&mut t, 5.0, "box.tx"), at(&mut s, 5.0, "box.tx"), "box.tx at 5");
}
