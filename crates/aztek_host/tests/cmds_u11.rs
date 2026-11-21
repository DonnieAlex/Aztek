//! Commands of the reference from animLayer to marker: animLayer's affected layers by attribute, autoKeyframe's plug
//! list and character option, bakeClip's blend, bakeDeformer, bakePartialHistory -all and -preCache, bakeResults' keyset
//! and smart bake flags, character's set flags, sub-characters and offset node, characterMap -unmapNode, copyKey's layer
//! and Euler flags, cutKey -selectKey, filter, filterCurve's kernel, sync, precision and quaternion flags,
//! filterInstances, filterStudioImport, findKeyframe's keyset flags, hotkey's context, hotkeySet -rename, keyTangent's
//! plug-in types, keyframe -clipTime, keyframeOutliner -animCurve, keyframeStats, keyingGroup and marker.

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

fn key_count(c: &mut Core, plug: &str) -> usize {
    num(&ok(c, &format!("keyframe -q -kc {plug};"))) as usize
}

// ---------------------------------------------------------------------------------------------------------------------

#[test]
fn anim_layer_affected_layers_by_attribute() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; animLayer layerA; animLayer layerB;");
    ok(&mut c, "animLayer -e -at box.tx layerA; animLayer -e -at box.ry layerB;");
    ok(&mut c, "select -r box;");
    let all = strs(&ok(&mut c, "animLayer -q -afl;"));
    assert!(all.contains(&"layerA".to_string()) && all.contains(&"layerB".to_string()), "{all:?}");
    let only = strs(&ok(&mut c, "animLayer -q -afl -afa translateX;"));
    assert!(only.contains(&"layerA".to_string()) && !only.contains(&"layerB".to_string()), "{only:?}");
    let ry = strs(&ok(&mut c, "animLayer -q -afl -afa box.ry;"));
    assert!(ry.contains(&"layerB".to_string()) && !ry.contains(&"layerA".to_string()), "{ry:?}");
}

#[test]
fn auto_keyframe_list_and_character_option() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 box.tx box.ty box.tz;");
    ok(&mut c, "autoKeyframe -state on;");
    assert_eq!(ok(&mut c, "autoKeyframe -q -state;"), json!(true));
    ok(&mut c, "autoKeyframe -e -addAttr box.ty;");
    assert_eq!(strs(&ok(&mut c, "autoKeyframe -q -listAttr;")), vec!["box.translateY"]);
    // The next set keys the listed plug too, and the list goes.
    ok(&mut c, "currentTime 5; setAttr box.tx 3;");
    assert_eq!(key_count(&mut c, "box.tx"), 2);
    assert_eq!(key_count(&mut c, "box.ty"), 2);
    assert_eq!(key_count(&mut c, "box.tz"), 1);
    assert!(strs(&ok(&mut c, "autoKeyframe -q -listAttr;")).is_empty());
    // No flags: the plugs listed are keyed now.
    ok(&mut c, "currentTime 8; autoKeyframe -e -addAttr box.tz;");
    assert_eq!(num(&ok(&mut c, "autoKeyframe;")), 1.0);
    assert_eq!(key_count(&mut c, "box.tz"), 2);
    // Changing the state clears the list unless -noReset.
    ok(&mut c, "autoKeyframe -e -addAttr box.tz; autoKeyframe -state off -noReset;");
    assert_eq!(strs(&ok(&mut c, "autoKeyframe -q -lsa;")).len(), 1);
    ok(&mut c, "autoKeyframe -state on;");
    assert!(strs(&ok(&mut c, "autoKeyframe -q -lsa;")).is_empty());
    // -characterOption all: keying one member keys them all.
    ok(&mut c, "character -n ch box.tx box.ty box.tz box.rx;");
    ok(&mut c, "setKeyframe -t 1 box.rx;");
    ok(&mut c, "autoKeyframe -characterOption all;");
    assert_eq!(ok(&mut c, "autoKeyframe -q -co;"), json!("all"));
    ok(&mut c, "currentTime 12; setAttr box.tx 4;");
    assert!(key_count(&mut c, "box.rx") >= 2);
    assert!(nums(&ok(&mut c, "keyframe -q -tc box.tz;")).contains(&12.0));
    ok(&mut c, "autoKeyframe -co standard -state off;");
}

#[test]
fn bake_clip_blend_indices() {
    let mut c = Core::new();
    ok(&mut c, "cone -n bakeCone; character -n coneCharacter bakeCone.tx;");
    ok(&mut c, "setKeyframe -t 0 -v 0 bakeCone.tx; setKeyframe -t 10 -v 10 bakeCone.tx; clip -n up -s 0 -end 10 coneCharacter;");
    ok(&mut c, "setKeyframe -t 15 -v 10 bakeCone.tx; setKeyframe -t 25 -v 0 bakeCone.tx; clip -n down -s 15 -end 25 coneCharacter;");
    let r = ok(&mut c, "bakeClip -b 0 1 -name \"bakedUpAndDown\" coneCharacter;");
    assert!(r.to_string().contains("bakedUpAndDown"), "{r}");
    assert!(bad(&mut c, "bakeClip -b 0 7 coneCharacter;").contains("no such clip"));
}

#[test]
fn bake_deformer_matches_a_skin() {
    let mut c = Core::new();
    // Two like chains, each with a box; the first bound and posed.
    ok(&mut c, "select -cl; joint -n a1 -p 0 0 0; joint -n a2 -p 0 2 0; joint -n a3 -p 0 4 0;");
    ok(&mut c, "select -cl; joint -n b1 -p 5 0 0; joint -n b2 -p 5 2 0; joint -n b3 -p 5 4 0;");
    ok(&mut c, "polyCube -n boxA -h 4 -sy 4; move 0 2 0 boxA; makeIdentity -a 1 boxA;");
    ok(&mut c, "polyCube -n boxB -h 4 -sy 4; move 5 2 0 boxB; makeIdentity -a 1 boxB;");
    ok(&mut c, "skinCluster -tsb a1 a2 a3 boxA;");
    let r = ok(&mut c, "bakeDeformer -ss a1 -sm boxA -ds b1 -dm boxB -mi 3;");
    assert!(!strs(&r).is_empty(), "{r}");
    assert_eq!(strs(&ok(&mut c, "skinCluster -q -inf boxB;")).len(), 3);
    // Posed alike, the two boxes match.
    ok(&mut c, "setAttr a1.rz 20; setAttr a2.rz -35; setAttr a2.rx 15; setAttr a3.ry 30;");
    ok(&mut c, "setAttr b1.rz 20; setAttr b2.rz -35; setAttr b2.rx 15; setAttr b3.ry 30;");
    for v in 0..10 {
        let pa = nums(&ok(&mut c, &format!("pointPosition -w boxA.vtx[{v}];")));
        let pb = nums(&ok(&mut c, &format!("pointPosition -w boxB.vtx[{v}];")));
        let d = ((pa[0] + 5.0 - pb[0]).powi(2) + (pa[1] - pb[1]).powi(2) + (pa[2] - pb[2]).powi(2)).sqrt();
        assert!(d < 0.05, "vertex {v}: {pa:?} vs {pb:?}");
    }
    // -bakeRangeOfMotion keys the poses: the rest pose and six for each influence.
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -n r1 -p 0 0 0; joint -n r2 -p 0 2 0;");
    assert_eq!(num(&ok(&mut c, "bakeDeformer -ss r1 -brm;")), 13.0);
    assert_eq!(key_count(&mut c, "r2.rz"), 13);
    // The same mesh and skeleton: its deformers give way to a skin with the weights found.
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -n s1 -p 0 0 0; joint -n s2 -p 0 2 0; joint -n s3 -p 0 4 0;");
    ok(&mut c, "polyCube -n boxS -h 4 -sy 4; move 0 2 0 boxS; makeIdentity -a 1 boxS;");
    ok(&mut c, "skinCluster -tsb s1 s2 s3 boxS;");
    ok(&mut c, "setAttr s2.rz 30;");
    let before = nums(&ok(&mut c, "pointPosition -w boxS.vtx[6];"));
    ok(&mut c, "bakeDeformer -ss s1 -sm boxS -mi 2 -sw 0 -pw 0.01 -cs -ds s1;");
    ok(&mut c, "setAttr s2.rz 30;");
    let after = nums(&ok(&mut c, "pointPosition -w boxS.vtx[6];"));
    for k in 0..3 {
        close(before[k], after[k], "same mesh rebaked");
    }
    assert_eq!(num(&ok(&mut c, "getAttr s2.useObjectColor;")), 1.0);
    assert!(bad(&mut c, "bakeDeformer -sm boxS;").contains("-ss"));
}

#[test]
fn bake_partial_history_all_and_pre_cache() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polySmooth a; polyCube -n b; polyBevel b.e[0];");
    let would = strs(&ok(&mut c, "bakePartialHistory -q -ppt a;"));
    assert!(would.iter().any(|n| n.starts_with("polySmooth")), "{would:?}");
    // No cache on either: -preCache has nothing to bake.
    assert!(strs(&ok(&mut c, "bakePartialHistory -q -pc a;")).is_empty());
    ok(&mut c, "select -cl;");
    let baked = strs(&ok(&mut c, "bakePartialHistory -all;"));
    assert_eq!(baked.len(), 2, "{baked:?}");
    assert_eq!(ok(&mut c, "objExists polySmoothFace1;"), json!(false));
    assert!(strs(&ok(&mut c, "listHistory b;")).iter().all(|n| !n.starts_with("polyBevel")));
    // A scene with no shapes: nothing, and no error.
    let mut c = Core::new();
    assert!(strs(&ok(&mut c, "bakePartialHistory -all;")).is_empty());
}

#[test]
fn bake_results_keyset_and_smart() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n src; polyCube -n dst; setKeyframe -t 1 -v 0 src.tx; setKeyframe -t 10 -v 9 src.tx; setKeyframe -t 20 -v 0 src.tx;");
    ok(&mut c, "connectAttr src.tx dst.tx;");
    // -iub off: the range's end is left out.
    ok(&mut c, "bakeResults -t \"1:10\" -iub false dst.tx;");
    assert_eq!(nums(&ok(&mut c, "keyframe -q -tc dst.tx;")).last().copied(), Some(9.0));
    // Smart: keys only where the source had them.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n src; polyCube -n dst; setKeyframe -t 1 -v 0 src.tx; setKeyframe -t 10 -v 9 src.tx; setKeyframe -t 20 -v 0 src.tx;");
    ok(&mut c, "connectAttr src.tx dst.tx;");
    ok(&mut c, "bakeResults -t \"1:20\" -sr true dst.tx;");
    assert_eq!(nums(&ok(&mut c, "keyframe -q -tc dst.tx;")), vec![1.0, 10.0, 20.0]);
    // With a tolerance more keys go where the curve strays from the straight lines.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n src; polyCube -n dst; setKeyframe -t 1 -v 0 src.tx; setKeyframe -t 10 -v 9 src.tx; setKeyframe -t 20 -v 0 src.tx;");
    ok(&mut c, "connectAttr src.tx dst.tx;");
    ok(&mut c, "bakeResults -t \"1:20\" -sr true 0.01 dst.tx;");
    assert!(key_count(&mut c, "dst.tx") > 3);
    // -index: the keyset is the keys picked; -osr steps the scene more often but keys every -sb.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -v 0 box.tx; setKeyframe -t 5 -v 4 box.tx; setKeyframe -t 9 -v 0 box.tx;");
    ok(&mut c, "bakeResults -index \"0:1\" -osr 4 box.tx;");
    assert_eq!(nums(&ok(&mut c, "keyframe -q -tc box.tx;")), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    // -an keys: the active keys' channel over their span.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -v 0 box.ty; setKeyframe -t 4 -v 3 box.ty; selectKey -add -k box.ty;");
    ok(&mut c, "bakeResults -an keys;");
    assert_eq!(key_count(&mut c, "box.ty"), 4);
    ok(&mut c, "polyCube -n other; setKeyframe -t 1 -v 0 other.tx; setKeyframe -t 3 -v 3 other.tx;");
    ok(&mut c, "bakeSimulation -t \"1:3\" -f \"1:3\" -smart 0 -an objects -index 0 other.tx;");
    assert_eq!(key_count(&mut c, "other.tx"), 3);
}

#[test]
fn character_set_flags_sub_characters_and_offset() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n b; polyCube -n d;");
    ok(&mut c, "character -n c1 a.tx a.ty; character -n c2 b.tx; character -n c3 a.tz -t \"note\";");
    assert_eq!(ok(&mut c, "character -q -t c3;"), json!("note"));
    ok(&mut c, "character -e -t \"other\" c3;");
    assert_eq!(ok(&mut c, "character -q -text c3;"), json!("other"));
    assert_eq!(ok(&mut c, "character -am c1 a.ty b.tx;"), json!(true));
    assert_eq!(ok(&mut c, "character -am c1 b.tx;"), json!(false));
    assert!(bad(&mut c, "character -add c2 a.tx;").contains("already in"));
    ok(&mut c, "character -in c2 -nw a.tx d.tx;");
    assert!(strs(&ok(&mut c, "character -q c2;")).contains(&"d.translateX".to_string()));
    assert!(!strs(&ok(&mut c, "character -q c2;")).contains(&"a.translateX".to_string()));
    // Sub-characters, and flattening them.
    ok(&mut c, "character -n parent c1 c2;");
    assert_eq!(strs(&ok(&mut c, "character -q parent;")), vec!["c1", "c2"]);
    assert_eq!(ok(&mut c, "character -im parent c1;"), json!(true));
    ok(&mut c, "character -fl parent;");
    let flat = strs(&ok(&mut c, "character -q parent;"));
    assert_eq!(flat.len(), 4, "{flat:?}");
    assert!(strs(&ok(&mut c, "character -q c1;")).is_empty());
    // Set queries.
    ok(&mut c, "character -n s1 b.ry b.rz; character -n s2 b.sx;");
    assert_eq!(strs(&ok(&mut c, "character -q -un s1 s2;")).len(), 3);
    assert!(strs(&ok(&mut c, "character -q -int s1 s2;")).is_empty());
    assert_eq!(ok(&mut c, "character -q -ii s1 s2;"), json!(false));
    assert_eq!(strs(&ok(&mut c, "character -q -sub s1 s2;")).len(), 2);
    // -split: the items leave for a new character.
    let made = ok(&mut c, "character -sp s1 -n s3 b.rz;");
    assert_eq!(made, json!("s3"));
    assert_eq!(strs(&ok(&mut c, "character -q s1;")), vec!["b.rotateY"]);
    // Member index, character plug, user alias.
    assert_eq!(ok(&mut c, "character -mi 0 -q s1;"), json!("b.rotateY"));
    assert_eq!(num(&ok(&mut c, "character -q -mi s1 b.ry;")), 0.0);
    assert_eq!(ok(&mut c, "character -q -cp b.ry;"), json!("s1.angularValues[1]"));
    assert_eq!(ok(&mut c, "character -q -ua b.ry;"), json!(""));
    // A root with its offset node: the offset turns and moves the root's motion.
    ok(&mut c, "polyCube -n hips; setAttr hips.tx 2;");
    ok(&mut c, "character -n walker -rt hips hips.tx hips.ty hips.tz hips.rx hips.ry hips.rz;");
    let ofs = ok(&mut c, "character -q -ofs walker;");
    let ofs = ofs.as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("nodeType {ofs};")), json!("characterOffset"));
    ok(&mut c, &format!("setAttr {ofs}.offsetRootRotateY 90; setAttr {ofs}.offsetRootTranslateZ 1;"));
    let t = nums(&ok(&mut c, "getAttr hips.t;"));
    close(t[0], 0.0, "turned x");
    close(t[2], -1.0, "turned and moved z");
    close(nums(&ok(&mut c, "getAttr hips.ry;"))[0], 90.0, "turned ry");
    ok(&mut c, &format!("setAttr {ofs}.enable 0;"));
    close(nums(&ok(&mut c, "getAttr hips.tx;"))[0], 2.0, "offset off");
    // Offset objects.
    ok(&mut c, "select -r hips; character -e -aoo walker;");
    assert_eq!(strs(&ok(&mut c, "character -q -aoo walker;")), vec!["hips"]);
    ok(&mut c, "select -r hips; character -e -roo walker;");
    assert!(strs(&ok(&mut c, "character -q -aoo walker;")).is_empty());
}

#[test]
fn character_map_unmap_node() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n s1; polyCube -n s2; character -n ch1 s1.tx s1.ty; character -n ch2 s2.tx s2.ty;");
    let m = ok(&mut c, "characterMap ch1 ch2;");
    let map = strs(&m)[0].clone();
    assert_eq!(strs(&ok(&mut c, &format!("characterMap -q {map};"))).len(), 2);
    ok(&mut c, &format!("characterMap -umn s1 s2 {map};"));
    assert!(strs(&ok(&mut c, &format!("characterMap -q {map};"))).is_empty());
    ok(&mut c, &format!("characterMap -mn s1 s2 {map};"));
    assert_eq!(strs(&ok(&mut c, &format!("characterMap -q {map};"))).len(), 2);
}

#[test]
fn copy_key_layer_and_euler() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -v 1 box.tx; setKeyframe -t 2 -v 2 box.tx;");
    ok(&mut c, "animLayer -at box.tx lay; setKeyframe -al lay -t 5 -v 7 box.tx;");
    assert_eq!(num(&ok(&mut c, "copyKey -al lay box.tx;")), 1.0);
    assert_eq!(num(&ok(&mut c, "copyKey -al BaseAnimation box.tx;")), 2.0);
    ok(&mut c, "polyCube -n none;");
    assert_eq!(num(&ok(&mut c, "copyKey -al lay none.tx;")), 0.0);
    // Quaternion rotation curves stay so when pasted, unless -fea.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n r; setKeyframe -t 1 r.rx r.ry r.rz; setKeyframe -t 10 -v 90 r.rx r.ry r.rz; rotationInterpolation -c quaternionSlerp r;");
    ok(&mut c, "polyCube -n q; copyKey r; pasteKey -o replaceCompletely q;");
    assert_eq!(strs(&ok(&mut c, "rotationInterpolation -q q.rx;")), vec!["quaternionSlerp"]);
    ok(&mut c, "polyCube -n e; copyKey -fea true r; pasteKey -o replaceCompletely e;");
    assert_ne!(strs(&ok(&mut c, "rotationInterpolation -q e.rx;")), vec!["quaternionSlerp"]);
}

#[test]
fn cut_key_select_key() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 box.tx; setKeyframe -t 5 box.tx; setKeyframe -t 9 box.tx; setKeyframe -t 1 box.ty;");
    ok(&mut c, "selectKey -cl;");
    assert_eq!(num(&ok(&mut c, "cutKey -t 5 -sl box;")), 1.0);
    assert_eq!(nums(&ok(&mut c, "keyframe -q -sl -tc;")), vec![1.0, 9.0]);
    assert_eq!(strs(&ok(&mut c, "keyframe -q -sl -n;")).len(), 1);
}

#[test]
fn filter_nodes() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "filter -t filterEuler -n houston;"), json!("houston"));
    ok(&mut c, "filter -edit -irx 10.0 houston;");
    close(num(&ok(&mut c, "filter -query -irx houston;")), 10.0, "irx");
    assert_eq!(ok(&mut c, "filter -q -t houston;"), json!("filterEuler"));
    assert_eq!(ok(&mut c, "nodeType -inherited houston;"), json!(["filter", "filterEuler"]));
    // The Euler filter takes the jump out of recorded rotations.
    ok(&mut c, "setAttr houston.inputUsage 3;");
    let data = json!([0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0 - std::f64::consts::TAU + 0.2, 0.0, 0.0]);
    let fl = c.s.graph.find("houston").unwrap().unwrap();
    let g = &mut c.s.graph;
    let ty = g.node(fl).unwrap().ty().clone();
    let put = |g: &mut aztek_scene::graph::Graph, at: &str, v: Value| {
        let i = ty.attr_index(at).unwrap() as i32;
        g.set_value(aztek_scene::plug::PlugRef::new(fl, i), aztek_scene::plug::PlugValue::Data(Some(std::sync::Arc::new(aztek_scene::rig::JsonData(v))))).unwrap();
    };
    put(g, "inputTime", json!([0.0, 0.1, 0.2]));
    put(g, "inputData", data);
    let out = g.value(&aztek_scene::plug::PlugRef::new(fl, ty.attr_index("outputData").unwrap() as i32)).unwrap();
    let v = match out {
        aztek_scene::plug::PlugValue::Data(Some(d)) => d.to_json(),
        _ => panic!("no output"),
    };
    let xs: Vec<f64> = v["value"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    close(xs[6], 3.2, "unwrapped");
    // The other kinds, and their settings as flags.
    ok(&mut c, "filter -t filterResample -n rs -kr 300 -ts 2;");
    close(num(&ok(&mut c, "getAttr rs.kernel;")), 300.0, "kernel");
    ok(&mut c, "filter -t filterSimplify -n simp; filter -t filterClosestSample -n cs;");
    close(num(&ok(&mut c, "filter -q -tol simp;")), 0.001, "tolerance");
    close(num(&ok(&mut c, "getAttr cs.frequency;")), 30.0, "frequency");
    assert!(bad(&mut c, "filter -t filterNothing;").contains("unknown filter type"));
    let fs = aztek_scene::filter_cmds::run_filter(&aztek_scene::filter_cmds::Settings::Simplify { allow_duplicates: true, max_step: 3600.0, min_step: 0.0, time_tolerance: 0.0, tolerance: 0.001 }, &[0.0, 1.0, 2.0, 3.0], &[0.0, 1.0, 2.0, 5.0], 1);
    assert_eq!(fs.0, vec![0.0, 2.0, 3.0]);
}

#[test]
fn filter_curve_kernels_sync_and_quaternions() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -v 0 box.tx; setKeyframe -t 2 -v 10 box.tx; setKeyframe -t 3 -v 0 box.tx; setKeyframe -t 4 -v 10 box.tx; setKeyframe -t 5 -v 0 box.tx;");
    // A box kernel averages over each step: the zigzag flattens.
    ok(&mut c, "filterCurve -f resample -per 0.0833333 -ker box box_translateX;");
    let v = nums(&ok(&mut c, "keyframe -q -vc box_translateX;"));
    assert!(v.iter().all(|x| *x > 0.5 && *x < 9.5) || v.len() <= 2, "{v:?}");
    // -s/-e limit the part filtered.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; for ($i = 1; $i <= 20; $i++) setKeyframe -t $i -v ($i % 2 * 5) box.tx;");
    ok(&mut c, "filterCurve -f gaussian -w 2 -sc 5 -s 10 -e 20 box_translateX;");
    let before: Vec<f64> = nums(&ok(&mut c, "keyframe -t \"1:9\" -q -vc box_translateX;"));
    assert_eq!(before[0], 5.0);
    let after: Vec<f64> = nums(&ok(&mut c, "keyframe -t \"12:18\" -q -vc box_translateX;"));
    assert!(after.iter().all(|x| *x > 0.5 && *x < 4.5), "{after:?}");
    // Key reducer: absolute precision, then key sync puts X, Y and Z keys back in step.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; for ($i = 1; $i <= 10; $i++) { setKeyframe -itt linear -ott linear -t $i -v ($i) box.tx; setKeyframe -itt linear -ott linear -t $i -v ($i == 5 ? 20 : $i * 2) box.ty; }");
    ok(&mut c, "filterCurve -f keyReducer -pm 0 -pre 0.001 -ks box_translateX box_translateY;");
    let tx = nums(&ok(&mut c, "keyframe -q -tc box_translateX;"));
    let ty = nums(&ok(&mut c, "keyframe -q -tc box_translateY;"));
    assert_eq!(tx, ty);
    assert!(tx.len() < 10);
    // Quaternions: a rotate's three curves smoothed together.
    let mut c = Core::new();
    ok(&mut c, "polyCube -n r; for ($i = 1; $i <= 10; $i++) { setKeyframe -t $i -v ($i * 10 + ($i % 2) * 4) r.rx; setKeyframe -t $i -v 5 r.ry; setKeyframe -t $i -v 0 r.rz; }");
    ok(&mut c, "filterCurve -f gaussian -w 1 -uq r_rotateX r_rotateY r_rotateZ;");
    let rx = nums(&ok(&mut c, "keyframe -q -vc r_rotateX;"));
    let jumps: Vec<f64> = rx.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(jumps.iter().all(|d| *d > 0.0), "{rx:?}");
}

#[test]
fn filter_instances_and_studio_import() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; instance -n b a; polyCube -n other;");
    ok(&mut c, "select -r |a|aShape |b|aShape other;");
    assert_eq!(ok(&mut c, "filterInstances -q;"), json!(true));
    let kept = strs(&ok(&mut c, "filterInstances;"));
    assert_eq!(kept.len(), 2, "{kept:?}");
    assert_eq!(ok(&mut c, "filterInstances -q;"), json!(false));
    // Transforms count by their shapes only with -s.
    ok(&mut c, "select -r a b;");
    assert_eq!(ok(&mut c, "filterInstances -q;"), json!(false));
    assert_eq!(strs(&ok(&mut c, "filterInstances -s 1;")).len(), 1);
    ok(&mut c, "filterStudioImport -includeCameras false -il true -sp true -td \"\";");
    let si = &c.s.extras.studio_import;
    assert_eq!(si["includeCameras"], json!(false));
    assert!(!si["transferDirectoryName"].as_str().unwrap().is_empty());
}

#[test]
fn find_keyframe_keyset_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 box.tx; setKeyframe -t 10 box.tx; setKeyframe -t 20 box.ty; currentTime 5;");
    assert_eq!(num(&ok(&mut c, "findKeyframe -which next box;")), 10.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -time 10 -which next box;")), 20.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -which next -at tx box;")), 10.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -which last -at tx box;")), 10.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -which first -t \"5:30\" box;")), 10.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -which last -t \"1:20\" -iub false box;")), 10.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -which last -index 0 box;")), 20.0);
    assert_eq!(strs(&ok(&mut c, "findKeyframe -curve -at translateX box;")), vec!["box_translateX"]);
    ok(&mut c, "select -r box;");
    assert_eq!(num(&ok(&mut c, "findKeyframe -timeSlider -which next;")), 10.0);
    // Active keys: with keysOrObjects and nothing named they are the keyset.
    ok(&mut c, "selectKey -k -t 20 box.ty;");
    assert_eq!(num(&ok(&mut c, "findKeyframe -which first;")), 20.0);
    assert_eq!(num(&ok(&mut c, "findKeyframe -an objects -which first box;")), 1.0);
    ok(&mut c, "selectKey -cl; select -cl;");
    assert_eq!(num(&ok(&mut c, "findKeyframe -an keys -which next;")), 5.0);
    // Not animated: the current time.
    ok(&mut c, "polyCube -n still;");
    assert_eq!(num(&ok(&mut c, "findKeyframe -which next still;")), 5.0);
}

#[test]
fn hotkeys_context_and_set_rename() {
    let mut c = Core::new();
    ok(&mut c, "nameCommand -ann \"x\" -c \"print 1\" myNC;");
    ok(&mut c, "hotkey -k \"b\" -n \"myNC\" -cc \"paintEffects\";");
    assert_eq!(ok(&mut c, "hotkey -cc \"paintEffects\" -q -n b;"), json!("myNC"));
    assert_eq!(ok(&mut c, "hotkey -q -n b;"), json!(""));
    assert_eq!(ok(&mut c, "hotkey -cc \"paintEffects\" -q b;"), json!(true));
    ok(&mut c, "hotkey -k \"b\" -n \"\" -cc \"paintEffects\";");
    assert_eq!(ok(&mut c, "hotkey -cc \"paintEffects\" -q b;"), json!(false));
    ok(&mut c, "hotkey -k \"F5\" -alt -mod -n \"myNC\";");
    assert_eq!(ok(&mut c, "hotkey -q -alt -n F5;"), json!("myNC"));

    ok(&mut c, "hotkeySet -current MyNewKeySet;");
    assert_eq!(ok(&mut c, "hotkeySet -edit -rename \"Renamed Set\" MyNewKeySet;"), json!("Renamed_Set"));
    assert_eq!(ok(&mut c, "hotkeySet -q -current;"), json!("Renamed_Set"));
    assert_eq!(ok(&mut c, "hotkeySet -exists MyNewKeySet;"), json!(false));
    assert!(bad(&mut c, "hotkeySet -e -re \"x\" Aztek_Default;").contains("default"));
}

#[test]
fn key_tangent_plugin_types_and_keyframe_clip_time() {
    let mut c = Core::new();
    assert_eq!(ok(&mut c, "keyTangent -q -ptt;"), json!([]));
    ok(&mut c, "polyCube -n box; setKeyframe -t 10 -v 1 box.tx; setKeyframe -t 20 -v 2 box.tx;");
    // Moved to 12, then scaled by 2 about 10 and offset by 1: 15.
    ok(&mut c, "keyframe -e -t 20 -a -tc 12 -ct 1 10 2 box.tx;");
    assert_eq!(nums(&ok(&mut c, "keyframe -q -tc box.tx;")), vec![10.0, 15.0]);
    ok(&mut c, "keyframe -e -t 10 -ct 0 0 3 box.tx;");
    assert_eq!(nums(&ok(&mut c, "keyframe -q -tc box.tx;")), vec![15.0, 30.0]);
}

#[test]
fn keyframe_outliner_and_stats_controls() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 2 -v 9 box.tx; setKeyframe -t 4 -v 1 box.tx;");
    ok(&mut c, "window w; formLayout f; keyframeOutliner -animCurve box_translateX ko;");
    assert_eq!(ok(&mut c, "keyframeOutliner -q -ac ko;"), json!("box_translateX"));
    ok(&mut c, "keyframeOutliner -e -ac box_translateX -dsp wide ko;");
    assert_eq!(ok(&mut c, "keyframeOutliner -q -dsp ko;"), json!("wide"));
    ok(&mut c, "keyframeStats -tan \"Frame\" -van \"Value\" -cw 1 60 -adj 2 -auw 4 -bgc 0.2 0.2 0.2 ks;");
    assert_eq!(ok(&mut c, "keyframeStats -q -tan ks;"), json!("Frame"));
    assert_eq!(ok(&mut c, "keyframeStats -q -van ks;"), json!("Value"));
    assert_eq!(num(&ok(&mut c, "keyframeStats -q -pre ks;")), 3.0);
    ok(&mut c, "keyframeStats -e -pre 5 -ae graphEditor1GraphEd ks;");
    assert_eq!(num(&ok(&mut c, "keyframeStats -q -pre ks;")), 5.0);
    assert_eq!(num(&ok(&mut c, "keyframeStats -q -auw ks;")), 4.0);
    assert_eq!(ok(&mut c, "keyframeStats -ex ks;"), json!(true));
    assert!(ok(&mut c, "keyframeStats -q -fpn ks;").as_str().unwrap().ends_with("ks"));
    // What the fields show: the selected key.
    ok(&mut c, "selectKey -k -t 2 box.tx;");
    assert_eq!(ok(&mut c, "keyStats;"), json!(["2", "9"]));
}

#[test]
fn keying_groups() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n objA; polyCube -n objB; polyCube -n objC; polyCube -n ctl;");
    ok(&mut c, "select -r objA; keyingGroup -name myKG -et -cat \"arms\";");
    assert_eq!(strs(&ok(&mut c, "keyingGroup -q myKG;")), vec!["objA"]);
    ok(&mut c, "keyingGroup -e -add myKG objB;");
    assert_eq!(num(&ok(&mut c, "keyingGroup -q -s myKG;")), 2.0);
    ok(&mut c, "keyingGroup -n parentKG myKG;");
    assert_eq!(ok(&mut c, "keyingGroup -im parentKG myKG;"), json!(true));
    ok(&mut c, "select -r ctl; keyingGroup -e -act myKG;");
    assert_eq!(strs(&ok(&mut c, "keyingGroup -q -act myKG;")), vec!["ctl"]);
    assert_eq!(ok(&mut c, "keyingGroup -q -cat myKG;"), json!("arms"));
    // Keying the activator keys the group, translate left out.
    ok(&mut c, "setKeyframe -t 1 ctl;");
    assert_eq!(key_count(&mut c, "objA.rx"), 1);
    assert_eq!(key_count(&mut c, "objB.sy"), 1);
    assert_eq!(key_count(&mut c, "objA.tx"), 0);
    // The filter: none, then another category, then this one.
    ok(&mut c, "keyingGroup -fil \"NoKeyingGroups\";");
    assert_eq!(ok(&mut c, "keyingGroup -q -fil;"), json!("NoKeyingGroups"));
    ok(&mut c, "setKeyframe -t 5 ctl;");
    assert_eq!(key_count(&mut c, "objA.rx"), 1);
    ok(&mut c, "keyingGroup -fil \"legs\"; setKeyframe -t 6 ctl;");
    assert_eq!(key_count(&mut c, "objA.rx"), 1);
    ok(&mut c, "keyingGroup -fil \"arms\"; setKeyframe -t 7 ctl;");
    assert_eq!(key_count(&mut c, "objA.rx"), 2);
    ok(&mut c, "keyingGroup -fil \"AllKeyingGroups\";");
    ok(&mut c, "select -r ctl; keyingGroup -e -rac myKG;");
    assert!(strs(&ok(&mut c, "keyingGroup -q -act myKG;")).is_empty());
    // Set flags as sets has them.
    ok(&mut c, "keyingGroup -n kg2 objB objC;");
    assert_eq!(strs(&ok(&mut c, "keyingGroup -int myKG kg2;")), vec!["objB"]);
    assert_eq!(ok(&mut c, "keyingGroup -ii myKG kg2;"), json!(true));
    assert_eq!(ok(&mut c, "keyingGroup -am myKG objC objA;"), json!(true));
    let cp = ok(&mut c, "keyingGroup -cp kg2 -n kg3;");
    assert_eq!(ok(&mut c, &format!("nodeType {};", cp.as_str().unwrap())), json!("keyingGroup"));
    ok(&mut c, "keyingGroup -sp kg3 -n kg4 objC;");
    assert_eq!(strs(&ok(&mut c, "keyingGroup -q kg3;")), vec!["objB"]);
    ok(&mut c, "keyingGroup -e -cl kg3;");
    assert!(strs(&ok(&mut c, "keyingGroup -q kg3;")).is_empty());
    ok(&mut c, "keyingGroup -e -mr 1 -coi 3 kg4;");
    assert_eq!(ok(&mut c, "keyingGroup -q -mr kg4;"), json!(true));
    assert_eq!(num(&ok(&mut c, "keyingGroup -q -coi kg4;")), 3.0);
}

#[test]
fn markers_on_a_motion_path() {
    let mut c = Core::new();
    ok(&mut c, "string $path = `curve -d 3 -p -10 0 0 -p -6 0 10 -p -3 0 -10 -p 10 0 0 -k 0 -k 0 -k 0 -k 1 -k 1 -k 1`;");
    ok(&mut c, "string $object[] = `sphere -ch off -axis 0 1 0`;");
    ok(&mut c, "pathAnimation -f 1 -stu 0 -etu 30 -c curve1 nurbsSphere1;");
    ok(&mut c, "currentTime -edit 20;");
    let pm = strs(&ok(&mut c, "marker -u 0.75 curve1;"));
    assert_eq!(pm.len(), 1);
    let pm = pm[0].clone();
    assert_eq!(ok(&mut c, &format!("nodeType {pm};")), json!("positionMarker"));
    close(num(&ok(&mut c, &format!("marker -q -t {pm};"))), 20.0, "time");
    close(num(&ok(&mut c, &format!("marker -q -u {pm};"))), 0.75, "u");
    assert_eq!(ok(&mut c, &format!("marker -q -pm {pm};")), json!(true));
    // The key is there: the object is at u 0.75 at frame 20.
    close(num(&ok(&mut c, "getAttr motionPath1.uValue;")), 0.75, "path u");
    let om = strs(&ok(&mut c, "marker -om -t 15 -ft 30 curve1;"))[0].clone();
    assert_eq!(ok(&mut c, &format!("marker -q -om {om};")), json!(true));
    close(num(&ok(&mut c, &format!("marker -q -ft {om};"))), 30.0, "front twist");
    close(num(&ok(&mut c, "getAttr -t 15 motionPath1.frontTwist;")), 30.0, "front twist key");
    let both = strs(&ok(&mut c, "marker -pm -om -t 10 -u 0.35 curve1;"));
    assert_eq!(both.len(), 2);
    // One position marker a time: a second at 20 moves the first.
    let again = strs(&ok(&mut c, "marker -u 0.5 -t 20 curve1;"));
    assert_eq!(again[0], pm);
    close(num(&ok(&mut c, &format!("marker -q -u {pm};"))), 0.5, "moved u");
    // Detached and back.
    ok(&mut c, &format!("marker -d {pm};"));
    ok(&mut c, &format!("marker -a {pm};"));
    close(num(&ok(&mut c, &format!("marker -q -t {pm};"))), 20.0, "time kept");
    assert!(bad(&mut c, "polyCube -n plain; marker plain;").contains("not a curve"));
}
