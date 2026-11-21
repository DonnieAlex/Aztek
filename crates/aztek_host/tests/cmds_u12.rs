//! Commands of the reference from pasteKey to timeControl: pasteKey's layer, keyset, float, matching and scale/fit
//! options, pause, scaleKey into a new range and on driven curves, setDrivenKeyframe's count, insert, hierarchy and
//! blend flags, setKeyframe's animated, clip, insertBlend, shape-preserving and query/edit flags, snapKey
//! -mergeDuplicate, snapshot's query, edit and anchored trails, soundControl, soundPopup and timeControl.

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

fn num(v: &Value) -> f64 {
    let n = nums(v);
    assert_eq!(n.len(), 1, "one number: {v}");
    n[0]
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
}

fn times(c: &mut Core, plug: &str) -> Vec<f64> {
    nums(&ok(c, &format!("keyframe -q -tc {plug};")))
}

fn values(c: &mut Core, plug: &str) -> Vec<f64> {
    nums(&ok(c, &format!("keyframe -q -vc {plug};")))
}

/// A cube with tx keys 0 at 1, 10 at 11, 0 at 21.
fn keyed(c: &mut Core, name: &str) {
    ok(c, &format!("polyCube -n {name};"));
    for (t, v) in [(1, 0.0), (11, 10.0), (21, 0.0)] {
        ok(c, &format!("setKeyframe -t {t} -v {v} -at tx {name};"));
    }
}

#[test]
fn paste_key_options_and_matching() {
    let mut c = Core::new();
    keyed(&mut c, "a");
    ok(&mut c, "polyCube -n b;");
    ok(&mut c, "copyKey -t \"1:11\" -at tx a;");
    // The count is of curves pasted.
    assert_eq!(ok(&mut c, "pasteKey -t 1 -at tx b;"), json!(1));
    assert_eq!(times(&mut c, "b.tx"), vec![1.0, 11.0]);
    // scaleReplace fits the clipboard's 10 frames into 30 to 50.
    ok(&mut c, "pasteKey -t \"30:50\" -o scaleReplace -at tx b;");
    assert_eq!(times(&mut c, "b.tx"), vec![1.0, 11.0, 30.0, 50.0]);
    // fitMerge cuts it to the range's length: only the first key goes in a 5 frame range.
    ok(&mut c, "pasteKey -t \"60:65\" -o fitMerge -at tx b;");
    assert_eq!(times(&mut c, "b.tx"), vec![1.0, 11.0, 30.0, 50.0, 60.0]);
    // replace over a range with -iub off keeps the key on the range's end.
    ok(&mut c, "pasteKey -t \"30:50\" -o replace -iub false -at tx b;");
    assert_eq!(times(&mut c, "b.tx"), vec![1.0, 11.0, 30.0, 40.0, 50.0, 60.0]);
    // -index: at the time of that key; -connect starts the pasted keys from the target's value there.
    ok(&mut c, "polyCube -n d; setKeyframe -t 5 -v 3 -at tx d; setKeyframe -t 7 -v 3 -at tx d;");
    ok(&mut c, "pasteKey -index 1 -o merge -connect true -at tx d;");
    assert_eq!(times(&mut c, "d.tx"), vec![5.0, 7.0, 17.0]);
    assert_eq!(values(&mut c, "d.tx"), vec![3.0, 3.0, 13.0]);
    // -selectPasted selects the keys put in.
    ok(&mut c, "polyCube -n e;");
    ok(&mut c, "pasteKey -t 100 -sp true -at tx e;");
    assert_eq!(ok(&mut c, "selectKey -q -kc;"), json!(2));
    // -an keys pastes onto the curves of the active keys.
    ok(&mut c, "pasteKey -an keys -t 200 -o merge;");
    assert_eq!(times(&mut c, "e.tx"), vec![100.0, 110.0, 200.0, 210.0]);
    // -matchByName: only where the object and channel are the copied ones.
    assert_eq!(ok(&mut c, "pasteKey -mn true -t 300 -at tx b;"), json!(0));
    assert_eq!(ok(&mut c, "pasteKey -mn true -t 300 -at tx a;"), json!(1));
    assert!(times(&mut c, "a.tx").contains(&300.0));
    bad(&mut c, "pasteKey -o sideways -at tx b;");
}

#[test]
fn paste_key_layers_and_driven_curves() {
    let mut c = Core::new();
    keyed(&mut c, "a");
    ok(&mut c, "polyCube -n b; setKeyframe -t 1 -at tx b; select -r b; animLayer -aso L1;");
    ok(&mut c, "copyKey -t \"1:11\" -at tx a;");
    // A layer the channel has no curve on takes nothing.
    ok(&mut c, "animLayer Empty;");
    assert_eq!(ok(&mut c, "pasteKey -al Empty -t 40 -at tx a;"), json!(0));
    assert_eq!(ok(&mut c, "pasteKey -al L1 -t 40 -o merge -at tx b;"), json!(1));
    let curves = ok(&mut c, "animLayer -q -anc L1;");
    let layer_curve = curves.as_array().unwrap().iter().filter_map(Value::as_str).find(|n| n.contains("translateX")).expect("curve on the layer").to_string();
    assert_eq!(times(&mut c, &layer_curve), vec![40.0, 50.0]);
    // Driven key curves paste by driver value: -f where, -fo offsets it.
    ok(&mut c, "polyCube -n drv; polyCube -n dn; polyCube -n dn2;");
    ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -v 0 dn.tx; setDrivenKeyframe -cd drv.ty -dv 2 -v 4 dn.tx;");
    ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -v 0 dn2.tx;");
    ok(&mut c, "copyKey -f \"0:2\" dn.tx;");
    ok(&mut c, "pasteKey -f 5 -fo 1 -o merge dn2.tx;");
    assert_eq!(times(&mut c, "dn2.tx"), vec![0.0, 6.0, 8.0]);
}

#[test]
fn pause_waits() {
    let mut c = Core::new();
    let t = std::time::Instant::now();
    ok(&mut c, "pause -sec 1;");
    assert!(t.elapsed().as_secs_f64() >= 0.99);
    ok(&mut c, "pause -seconds 0;");
}

#[test]
fn scale_key_ranges_pivots_and_snapping() {
    let mut c = Core::new();
    keyed(&mut c, "a");
    // -nst -net: 1 to 11 fills 1 to 21; the key after it stays put with -ssk on.
    assert_eq!(ok(&mut c, "scaleKey -t \"1:11\" -nst 1 -net 21 -at tx a;"), json!(1));
    assert_eq!(times(&mut c, "a.tx"), vec![1.0, 21.0]);
    keyed(&mut c, "b");
    // -ssk off: the keys after the range keep their distance from its end.
    ok(&mut c, "scaleKey -t \"1:11\" -nst 1 -net 21 -ssk false -at tx b;");
    assert_eq!(times(&mut c, "b.tx"), vec![1.0, 21.0, 31.0]);
    // -asp snaps what lands between frames.
    keyed(&mut c, "d");
    ok(&mut c, "scaleKey -ts 0.33 -tp 1 -asp true -at tx d;");
    assert_eq!(times(&mut c, "d.tx"), vec![1.0, 4.0, 8.0]);
    keyed(&mut c, "e");
    ok(&mut c, "scaleKey -ts 0.33 -tp 1 -asp false -at tx e;");
    near(times(&mut c, "e.tx")[1], 4.3);
    // The optionVar is the default.
    keyed(&mut c, "f");
    ok(&mut c, "optionVar -iv scaleKeyAutoSnap 1; scaleKey -ts 0.33 -tp 1 -at tx f;");
    assert_eq!(times(&mut c, "f.tx"), vec![1.0, 4.0, 8.0]);
    ok(&mut c, "optionVar -iv scaleKeyAutoSnap 0;");
    // Driven curves scale on the float flags, and not on the time ones.
    ok(&mut c, "polyCube -n drv; polyCube -n dn;");
    ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -v 0 dn.tx; setDrivenKeyframe -cd drv.ty -dv 2 -v 4 dn.tx;");
    ok(&mut c, "scaleKey -ts 5 dn.tx;");
    assert_eq!(times(&mut c, "dn.tx"), vec![0.0, 2.0]);
    ok(&mut c, "scaleKey -fs 3 -fp 0 dn.tx;");
    assert_eq!(times(&mut c, "dn.tx"), vec![0.0, 6.0]);
    ok(&mut c, "scaleKey -nsf 1 -nef 2 dn.tx;");
    assert_eq!(times(&mut c, "dn.tx"), vec![1.0, 2.0]);
}

#[test]
fn snap_key_merges_and_counts_clashes() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a;");
    for (t, v) in [(1.0, 0.0), (1.4, 1.0), (5.0, 2.0)] {
        ok(&mut c, &format!("setKeyframe -t {t} -v {v} -at tx a;"));
    }
    // 1.4 would land on the key at 1: left alone, and the curve counts as a clash.
    assert_eq!(ok(&mut c, "snapKey -at tx a;"), json!(1));
    assert_eq!(times(&mut c, "a.tx"), vec![1.0, 1.4, 5.0]);
    // -md merges them into one.
    assert_eq!(ok(&mut c, "snapKey -md true -at tx a;"), json!(0));
    assert_eq!(times(&mut c, "a.tx"), vec![1.0, 5.0]);
    // Values to multiples of -vm.
    ok(&mut c, "setKeyframe -t 9 -v 2.6 -at tx a; snapKey -vm 1 -at tx a;");
    assert_eq!(values(&mut c, "a.tx").last().copied(), Some(3.0));
}

#[test]
fn set_driven_keyframe_count_insert_and_targets() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n drv; polyCube -n drv2; polyCube -n dn;");
    ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -v 0 -at tx dn; setDrivenKeyframe -cd drv.ty -dv 10 -v 10 -at tx dn;");
    ok(&mut c, "setDrivenKeyframe -cd drv2.tz -dv 0 -v 0 -at tx dn;");
    assert_eq!(ok(&mut c, "setDrivenKeyframe -q -dr dn.tx;").as_array().unwrap().len(), 2);
    assert_eq!(ok(&mut c, "setDrivenKeyframe -q -cnt -dr dn.tx;"), json!(2));
    assert_eq!(ok(&mut c, "setDrivenKeyframe -q -cnt -cd dn.tx;"), json!(2));
    assert_eq!(ok(&mut c, "setDrivenKeyframe -q -cnt -dn drv.ty;"), json!(1));
    // -insert: a key at 5 on the curve as it runs, its value whatever the curve has there.
    let curve = "dn_translateX";
    assert_eq!(ok(&mut c, "setDrivenKeyframe -i -cd drv.ty -dv 5 -at tx dn;"), json!(1));
    assert_eq!(times(&mut c, curve), vec![0.0, 5.0, 10.0]);
    // -hi below takes in the children.
    ok(&mut c, "polyCube -n kid; parent kid dn;");
    ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -v 1 -hi below -at ry dn;");
    assert!(ok(&mut c, "setDrivenKeyframe -q -dn drv.ty;").as_array().unwrap().iter().any(|x| x.as_str().is_some_and(|s| s.contains("kid"))));
    // -insertBlend: a constrained channel gets a pairBlend for its driven keys; off, no key.
    ok(&mut c, "polyCube -n tgt; polyCube -n held; pointConstraint tgt held;");
    assert_eq!(ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -ib false -at ty held;"), json!(0));
    assert_eq!(ok(&mut c, "setDrivenKeyframe -cd drv.ty -dv 0 -ib true -at ty held;"), json!(1));
    assert_eq!(ok(&mut c, "nodeType `listConnections -s 1 -d 0 held.ty`;"), json!("pairBlend"));
    assert_eq!(ok(&mut c, "setDrivenKeyframe -q -dr held.ty;"), json!(["drv.translateY"]));
}

#[test]
fn set_keyframe_new_flags() {
    let mut c = Core::new();
    keyed(&mut c, "a");
    // -an keys only channels already keyed.
    assert_eq!(ok(&mut c, "setKeyframe -an true -t 5 a;"), json!(1));
    // -q -bd / -e -bd on the keys at the time.
    ok(&mut c, "currentTime 11;");
    assert_eq!(ok(&mut c, "setKeyframe -q -bd a.tx;"), json!([false]));
    ok(&mut c, "setKeyframe -e -bd true -t 11 -at tx a;");
    assert_eq!(ok(&mut c, "setKeyframe -q -bd a.tx;"), json!([true]));
    assert_eq!(times(&mut c, "a.tx"), vec![1.0, 5.0, 11.0, 21.0]);
    // -pcs: a key between keys keeps the curve's shape; -i -adt off leaves the neighbours' tangents.
    keyed(&mut c, "b");
    ok(&mut c, "keyTangent -e -itt spline -ott spline b.tx;");
    ok(&mut c, "currentTime 6;");
    let v6 = num(&ok(&mut c, "getAttr b.tx;"));
    ok(&mut c, "setKeyframe -pcs true -at tx b;");
    ok(&mut c, "currentTime 8;");
    let after = num(&ok(&mut c, "getAttr b.tx;"));
    ok(&mut c, "currentTime 6;");
    near(num(&ok(&mut c, "getAttr b.tx;")), v6);
    keyed(&mut c, "d");
    ok(&mut c, "keyTangent -e -itt spline -ott spline d.tx;");
    ok(&mut c, "currentTime 8;");
    near(num(&ok(&mut c, "getAttr d.tx;")), after);
    ok(&mut c, "setKeyframe -i -adt false -t 6 -at tx d;");
    assert_eq!(ok(&mut c, "keyTangent -q -ott -t 1 d.tx;"), json!(["spline"]));
    ok(&mut c, "setKeyframe -i -t 16 -at tx d;");
    assert_eq!(ok(&mut c, "keyTangent -q -itt -t 21 d.tx;"), json!(["fixed"]));
    // -insertBlend keys a constrained channel through a pairBlend; without it there is nothing to key.
    ok(&mut c, "polyCube -n tgt; polyCube -n held; pointConstraint tgt held;");
    assert_eq!(ok(&mut c, "setKeyframe -t 1 -at tx held;"), json!(0));
    assert_eq!(ok(&mut c, "setKeyframe -ib true -t 1 -v 3 -at tx held;"), json!(1));
    assert_eq!(ok(&mut c, "nodeType `listConnections -s 1 -d 0 held.tx`;"), json!("pairBlend"));
    assert_eq!(ok(&mut c, "keyframe -q -kc held.tx;"), json!(1));
    assert_eq!(ok(&mut c, "setKeyframe -t 5 -v 4 -at tx held;"), json!(1));
    assert_eq!(ok(&mut c, "keyframe -q -kc held.tx;"), json!(2));
    // -lw keeps the weights of the key keyed over on a weighted curve; without it they go back.
    ok(&mut c, "polyCube -n w; setKeyframe -t 1 -at tx w; setKeyframe -t 10 -v 5 -at tx w; keyTangent -e -wt 1 w.tx;");
    ok(&mut c, "keyTangent -e -t 10 -wl 1 -iw 4 w.tx;");
    ok(&mut c, "setKeyframe -lw true -t 10 -v 6 -at tx w;");
    near(num(&ok(&mut c, "keyTangent -q -t 10 -iw w.tx;")), 4.0);
    ok(&mut c, "setKeyframe -t 10 -v 6 -at tx w;");
    assert!((num(&ok(&mut c, "keyTangent -q -t 10 -iw w.tx;")) - 4.0).abs() > 1e-6);
}

#[test]
fn set_keyframe_into_a_clip() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; setKeyframe -t 1 -v 0 -at tx a; setKeyframe -t 10 -v 10 -at tx a;");
    ok(&mut c, "character -n hero a.tx;");
    ok(&mut c, "clip -name walk -startTime 1 -endTime 10 hero;");
    // A key in the clip, at frame 5: the clip's own curve takes it.
    assert_eq!(ok(&mut c, "setKeyframe -c walk -t 5 -v 100 a.tx;"), json!(1));
    ok(&mut c, "currentTime 5;");
    near(num(&ok(&mut c, "getAttr a.tx;")), 100.0);
    // A clip the channel isn't in: keyed as usual.
    ok(&mut c, "polyCube -n b;");
    assert_eq!(ok(&mut c, "setKeyframe -c walk -t 3 -v 2 b.tx;"), json!(1));
    assert_eq!(ok(&mut c, "keyframe -q -kc b.tx;"), json!(1));
}

#[test]
fn snapshot_query_edit_and_anchored_trails() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; setKeyframe -t 1 -v 0 -at tx a; setKeyframe -t 21 -v 20 -at tx a; keyTangent -e -itt linear -ott linear a.tx;");
    // The defaults: frames 1 to 10, every frame.
    assert_eq!(ok(&mut c, "snapshot a;"), json!(["snapshot1Group", "snapshot1"]));
    assert_eq!(ok(&mut c, "listRelatives snapshot1Group;").as_array().unwrap().len(), 10);
    assert_eq!(num(&ok(&mut c, "snapshot -q -st snapshot1;")), 1.0);
    assert_eq!(num(&ok(&mut c, "snapshot -q -et snapshot1;")), 10.0);
    assert_eq!(ok(&mut c, "snapshot -q -u snapshot1;"), json!("always"));
    // Edit: new times make the copies again.
    ok(&mut c, "snapshot -e -i 5 -et 21 -u demand snapshot1;");
    assert_eq!(ok(&mut c, "listRelatives snapshot1Group;").as_array().unwrap().len(), 5);
    assert_eq!(num(&ok(&mut c, "snapshot -q -i snapshot1Group;")), 5.0);
    assert_eq!(ok(&mut c, "snapshot -q -u snapshot1;"), json!("demand"));
    // An anchored trail: in the anchor's space, the handle under it.
    ok(&mut c, "polyCube -n cam; setKeyframe -t 1 -v 0 -at tx cam; setKeyframe -t 21 -v 20 -at tx cam; keyTangent -e -itt linear -ott linear cam.tx;");
    let r = ok(&mut c, "snapshot -mt 1 -at cam -st 1 -et 21 -i 10 a;");
    assert_eq!(r, json!(["motionTrail1Handle", "motionTrail1"]));
    let p = ok(&mut c, "motionTrail -q -pts motionTrail1;");
    let xs: Vec<f64> = p.as_array().unwrap().iter().map(|v| v[0].as_f64().unwrap()).collect();
    assert_eq!(xs, vec![0.0, 0.0, 0.0], "the object moves with the anchor");
    assert_eq!(ok(&mut c, "listRelatives -p motionTrail1Handle;"), json!(["cam"]));
    assert_eq!(ok(&mut c, "snapshot -q -at motionTrail1;"), json!("cam"));
    // -opm: the handle leaves the anchor and follows it through its offsetParentMatrix.
    ok(&mut c, "snapshot -e -opm 1 motionTrail1;");
    assert_eq!(ok(&mut c, "listRelatives -p motionTrail1Handle;"), json!([]));
    assert_eq!(ok(&mut c, "listConnections -s 1 -d 0 motionTrail1Handle.offsetParentMatrix;"), json!(["cam"]));
    ok(&mut c, "snapshot -e -opm 0 motionTrail1;");
    assert_eq!(ok(&mut c, "listRelatives -p motionTrail1Handle;"), json!(["cam"]));
    // Edit the trail's frames.
    ok(&mut c, "snapshot -e -i 5 motionTrail1;");
    assert_eq!(ok(&mut c, "motionTrail -q -pts motionTrail1;").as_array().unwrap().len(), 5);
}

#[test]
fn sound_control_and_popup() {
    let mut c = Core::new();
    ok(&mut c, "window w; columnLayout;");
    let path = ok(&mut c, "soundControl -w 300 -h 40 -min 1 -max 48 -wf bottom -pc \"$gPressed = 1\" -rc \"$gReleased = 1\" sc;");
    assert_eq!(path, json!("w|columnLayout1|sc"));
    assert_eq!(ok(&mut c, "soundControl -q -wf sc;"), json!("bottom"));
    assert_eq!(num(&ok(&mut c, "soundControl -q -min sc;")), 1.0);
    assert_eq!(num(&ok(&mut c, "soundControl -q -max sc;")), 48.0);
    assert_eq!(num(&ok(&mut c, "soundControl -q -rcs sc;")), 1.0);
    assert_eq!(ok(&mut c, "soundControl -q -roh sc;"), json!(false));
    assert_eq!(ok(&mut c, "soundControl -q -s sc;"), json!(""));
    ok(&mut c, "soundControl -e -s clip1 -ds true -roh true -rcs 2 sc;");
    assert_eq!(ok(&mut c, "soundControl -q -s sc;"), json!("clip1"));
    assert_eq!(ok(&mut c, "soundControl -q -ss sc;"), json!("clip1"));
    assert_eq!(ok(&mut c, "soundControl -q -roh sc;"), json!(true));
    assert_eq!(num(&ok(&mut c, "soundControl -q -rcs sc;")), 2.0);
    ok(&mut c, "soundControl -e -beginScrub sc;");
    ok(&mut c, "soundControl -e -endScrub sc;");
    ok(&mut c, "soundControl -e -resample sc;");
    bad(&mut c, "soundControl -e -wf sideways sc;");
    assert_eq!(ok(&mut c, "soundControl -ex sc;"), json!(true));
    assert_eq!(ok(&mut c, "soundControl -q -w sc;"), json!(300));
    assert_eq!(ok(&mut c, "soundControl -q -fpn sc;"), json!("w|columnLayout1|sc"));
    // The mouse scripts run when the shell says the mouse went down and up on it.
    ok(&mut c, "global int $gPressed = 0; global int $gReleased = 0;");
    c.ui_event(&json!({"path": "w|columnLayout1|sc", "fire": ["pressCommand"]}));
    c.ui_event(&json!({"path": "w|columnLayout1|sc", "fire": ["releaseCommand"]}));
    assert_eq!(ok(&mut c, "global int $gPressed; $x = $gPressed;"), json!(1));
    assert_eq!(ok(&mut c, "global int $gReleased; $x = $gReleased;"), json!(1));
    // soundPopup: a control with the common flags.
    let p = ok(&mut c, "soundPopup -ann \"sounds\" sp;");
    assert_eq!(p, json!("w|columnLayout1|sp"));
    assert_eq!(ok(&mut c, "soundPopup -ex sp;"), json!(true));
    assert_eq!(ok(&mut c, "objectTypeUI sp;"), json!("soundPopup"));
}

#[test]
fn time_control_flags() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "global string $gPlayBackSlider; $s = $gPlayBackSlider;"), json!("timeControl1"));
    assert_eq!(ok(&mut c, "timeControl -ex timeControl1;"), json!(true));
    // Defaults.
    assert_eq!(ok(&mut c, "timeControl -q -gt $gPlayBackSlider;"), json!(true));
    assert_eq!(ok(&mut c, "timeControl -q -wf $gPlayBackSlider;"), json!("top"));
    assert_eq!(ok(&mut c, "timeControl -q -alf $gPlayBackSlider;"), json!("allAffecting"));
    assert_eq!(ok(&mut c, "timeControl -q -tsp $gPlayBackSlider;"), json!(1));
    // Edits, read back.
    let r = ok(&mut c, "timeControl -e -gt false -wf both -tsp 10 -alf active -mlc conn1 -cfc 1 0 0 0.5 -fgc 0 1 0 -sn false -ts 70 -ann \"slider\" -bgc 0.1 0.2 0.3 $gPlayBackSlider;");
    assert_eq!(r, json!("timeControl1"));
    assert_eq!(ok(&mut c, "timeControl -q -gt $gPlayBackSlider;"), json!(false));
    assert_eq!(ok(&mut c, "timeControl -q -wf $gPlayBackSlider;"), json!("both"));
    assert_eq!(ok(&mut c, "timeControl -q -tsp $gPlayBackSlider;"), json!(10));
    assert_eq!(ok(&mut c, "timeControl -q -alf $gPlayBackSlider;"), json!("active"));
    assert_eq!(ok(&mut c, "timeControl -q -mlc $gPlayBackSlider;"), json!("conn1"));
    assert_eq!(ok(&mut c, "timeControl -q -sn $gPlayBackSlider;"), json!(false));
    assert_eq!(ok(&mut c, "timeControl -q -ts $gPlayBackSlider;"), json!(63));
    assert_eq!(ok(&mut c, "timeControl -q -ann $gPlayBackSlider;"), json!("slider"));
    assert_eq!(ok(&mut c, "timeControl -q -fpn $gPlayBackSlider;"), json!("timeControl1"));
    bad(&mut c, "timeControl -e -wf sideways $gPlayBackSlider;");
    bad(&mut c, "timeControl -e -alf nothing $gPlayBackSlider;");
    // What the shell reads to draw the main slider.
    let st = c.s.run("aztekTimeControlState", json!({})).unwrap();
    assert_eq!(st["currentFrameColor"], json!([1.0, 0.0, 0.0, 0.5]));
    assert_eq!(st["foregroundColor"], json!([0.0, 1.0, 0.0]));
    assert_eq!(st["globalTime"], json!(false));
    assert_eq!(st["snap"], json!(false));
    assert_eq!(st["tickSpan"], json!(10));
    // Scrubbing, redraws and the sound part.
    ok(&mut c, "timeControl -e -bs $gPlayBackSlider;");
    assert_eq!(c.s.run("aztekTimeControlState", json!({})).unwrap()["scrubbing"], json!(true));
    ok(&mut c, "timeControl -e -es -fr -fd -r -roh true -rcs 0.5 $gPlayBackSlider;");
    assert_eq!(c.s.run("aztekTimeControlState", json!({})).unwrap()["scrubbing"], json!(false));
    assert_eq!(ok(&mut c, "timeControl -q -roh $gPlayBackSlider;"), json!(true));
    // -acn: the curves whose keys show, from the main list connection or the selection.
    ok(&mut c, "polyCube -n a; setKeyframe -t 1 -at tx a; polyCube -n b; setKeyframe -t 1 -at ty b;");
    ok(&mut c, "timeControl -e -mlc \"\" $gPlayBackSlider; select -r a;");
    assert_eq!(ok(&mut c, "timeControl -q -acn $gPlayBackSlider;"), json!(["a_translateX"]));
    ok(&mut c, "selectionConnection -object b conn2; timeControl -e -mlc conn2 $gPlayBackSlider;");
    assert_eq!(ok(&mut c, "timeControl -q -acn $gPlayBackSlider;"), json!(["b_translateY"]));
    ok(&mut c, "timeControl -e -showKeys none $gPlayBackSlider;");
    assert_eq!(ok(&mut c, "timeControl -q -acn $gPlayBackSlider;"), json!([]));
    ok(&mut c, "timeControl -e -showKeys active $gPlayBackSlider;");
    // Press and release scripts, run when the shell reports them.
    ok(&mut c, "global int $gDown = 0; timeControl -e -pc \"$gDown = 7\" -rc \"$gDown = 9\" $gPlayBackSlider;");
    c.ui_event(&json!({"path": "timeControl1", "fire": ["pressCommand"]}));
    assert_eq!(ok(&mut c, "global int $gDown; $x = $gDown;"), json!(7));
    c.ui_event(&json!({"path": "timeControl1", "fire": ["releaseCommand"]}));
    assert_eq!(ok(&mut c, "global int $gDown; $x = $gDown;"), json!(9));
    // The range flags stay the scene's.
    ok(&mut c, "timeControl -e -ra 5 9 $gPlayBackSlider;");
    assert_eq!(ok(&mut c, "timeControl -q -rv $gPlayBackSlider;"), json!(true));
    assert_eq!(nums(&ok(&mut c, "timeControl -q -ra $gPlayBackSlider;")), vec![5.0, 9.0]);
    ok(&mut c, "timeControl -e -ra 0 0 $gPlayBackSlider;");
}

#[test]
fn every_flag_of_the_reference_is_there() {
    let c = Core::new();
    let want: &[(&str, &str)] = &[
        ("pasteKey", "al an animLayer animation f float floatOffset fo includeUpperBound index iub matchByName mn selectPasted sp ma matchByAttr"),
        ("pause", "sec seconds"),
        ("scaleKey", "asp autoSnap floatPivot floatScale fp fs nef net newEndFloat newEndTime newStartFloat newStartTime nsf nst scaleSpecifiedKeys ssk"),
        ("setDrivenKeyframe", "cnt controlPoints count cp hi hierarchy i ib insert insertBlend s shape"),
        ("setKeyframe", "adjustTangent adt an animated c clip e edit ib insertBlend lw pcs preserveCurveShape q query useCurrentLockedWeights"),
        ("snapKey", "md mergeDuplicate"),
        ("snapshot", "anchorTransform at e edit offsetParentMatrix opm q query"),
        (
            "soundControl",
            "ann annotation backgroundColor beginScrub bgc bs defineTemplate dgc displaySound docTag dpc dragCallback dropCallback ds dt dtg e ebg edit ekf en enable \
             enableBackground enableKeyboardFocus endScrub es ex exists fpn fullPathName h height highlightColor hlc io isObscured m manage max maxTime min minTime nbg \
             noBackground npm numberOfPopupMenus p parent pc pma po popupMenuArray pressCommand preventOverride q query r rc rcs releaseCommand repeatChunkSize \
             repeatOnHold resample roh s sbm sound statusBarMessage useTemplate ut vcc vis visible visibleChangeCommand w waveform wf width",
        ),
        (
            "soundPopup",
            "ann annotation backgroundColor bgc defineTemplate dgc docTag dpc dragCallback dropCallback dt dtg ebg ekf en enable enableBackground enableKeyboardFocus ex \
             exists fpn fullPathName h height highlightColor hlc io isObscured m manage nbg noBackground npm numberOfPopupMenus p parent pma po popupMenuArray \
             preventOverride sbm statusBarMessage useTemplate ut vcc vis visible visibleChangeCommand w width",
        ),
        (
            "timeControl",
            "acn alf animCurveNames animLayerFilterOptions ann annotation backgroundColor bgc cfc currentFrameColor defineTemplate dgc docTag dpc dragCallback \
             dropCallback dt dtg ebg ekf en enable enableBackground enableKeyboardFocus ex exists fd fgc forceRefresh foregroundColor fpn fullPathName globalTime gt h \
             height highlightColor hlc io isObscured m mainListConnection manage mlc nbg noBackground npm numberOfPopupMenus p parent pc pma po popupMenuArray \
             pressCommand preventOverride r rc rcs releaseCommand repeatChunkSize repeatOnHold resample roh sbm sn statusBarMessage tickSpan tsp useTemplate ut vcc \
             vis visible visibleChangeCommand w waveform wf width fr forceRedraw ss soundSource",
        ),
    ];
    for (cmd, flags) in want {
        let d = c.s.commands.find(cmd).unwrap_or_else(|| panic!("no {cmd}"));
        for fl in flags.split_whitespace() {
            assert!(d.flags.iter().any(|x| x.name == fl || x.short_name == fl), "{cmd} has no -{fl}");
        }
    }
    // Short names that once differed from the reference's.
    let tc = c.s.commands.find("timeControl").unwrap();
    let short = |n: &str| tc.flags.iter().find(|x| x.name == n).map(|x| x.short_name.clone());
    assert_eq!(short("snap").as_deref(), Some("sn"));
    assert_eq!(short("forceRedraw").as_deref(), Some("fd"));
    assert_eq!(short("forceRefresh").as_deref(), Some("fr"));
}
