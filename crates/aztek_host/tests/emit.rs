//! `emit`: particles added to particles that exist, with per particle values paired to the new particles by the
//! flags' order, as the reference's documentation describes it and shows in its examples.

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
    match v {
        Value::Array(a) => a.iter().flat_map(|x| nums(x)).collect(),
        x => vec![x.as_f64().unwrap_or_else(|| panic!("not a number: {x}"))],
    }
}

fn close(a: &[f64], b: &[f64], what: &str) {
    assert_eq!(a.len(), b.len(), "{what}: {a:?} against {b:?}");
    for (x, y) in a.iter().zip(b) {
        assert!((x - y).abs() < 1e-6, "{what}: {a:?} against {b:?}");
    }
}

#[test]
fn emit_adds_one_particle_per_position_and_returns_ids() {
    let mut c = Core::new();
    // The documentation's first example: an empty particle object, then one particle at 1 1 1.
    ok(&mut c, "particle;");
    let ids = ok(&mut c, "emit -object particle1 -position 1 1 1;");
    assert_eq!(ids, json!([0]));
    assert_eq!(nums(&ok(&mut c, "particle -q -ct particle1;")), vec![1.0]);
    close(&nums(&ok(&mut c, "getParticleAttr -at position -array 1 particle1;")), &[1.0, 1.0, 1.0], "where it is");
    // More, in the order of the -pos flags; the ids go on from those there.
    let ids = ok(&mut c, "emit -o particle1 -pos 2 0 0 -pos 3 0 0;");
    assert_eq!(ids, json!([1, 2]));
    close(&nums(&ok(&mut c, "getParticleAttr -at position -array 1 particle1;")), &[1.0, 1.0, 1.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0], "in order");
    // The shape's name does as well as the transform's.
    assert_eq!(ok(&mut c, "emit -o particleShape1 -pos 0 5 0;"), json!([3]));
}

#[test]
fn emit_values_follow_the_flags_order() {
    let mut c = Core::new();
    ok(&mut c, "particle -n p;");
    // The second example: two velocities, then rgbPP given a vector and a number, the number on each axis.
    ok(&mut c, "emit -o p -pos 1 1 1 -pos 2 2 2 -at velocity -vv 1 2 3 -vv 2 3 4 -at rgbPP -vv .5 1 0 -fv .1;");
    close(&nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 p;")), &[1.0, 2.0, 3.0, 2.0, 3.0, 4.0], "velocities");
    close(&nums(&ok(&mut c, "getParticleAttr -at rgbPP -array 1 p;")), &[0.5, 1.0, 0.0, 0.1, 0.1, 0.1], "colours");
}

#[test]
fn emit_repeats_the_last_value_and_picks_up_a_repeated_attribute() {
    let mut c = Core::new();
    ok(&mut c, "particle -n p;");
    // The third example's shape: velocity given twice picks up where it left off, the rest take the last value;
    // a vector for a number attribute gives its length.
    ok(&mut c, "emit -o p -pos 1 1 1 -pos 2 2 2 -pos 3 3 3 -pos 4 4 4 -pos 5 5 5 -at velocity -vv 1 2 3 -vv 2 3 4 -at opacityPP -fv .5 -vv 3 4 0 -at velocity -vv 3 4 5;");
    let v = nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 p;"));
    close(&v, &[1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0], "velocities");
    let o = nums(&ok(&mut c, "getParticleAttr -at opacityPP -array 1 p;"));
    close(&o, &[0.5, 5.0, 5.0, 5.0, 5.0], "opacities");
}

#[test]
fn emit_needs_object_and_known_attributes() {
    let mut c = Core::new();
    ok(&mut c, "particle -n p; select -cl;");
    // The selection is not used.
    ok(&mut c, "select p;");
    bad(&mut c, "emit -pos 1 1 1;");
    bad(&mut c, "emit -o p -pos 1 1 1 -at notAnAttribute -fv 1;");
    bad(&mut c, "emit -o p -pos 1 1 1 -vv 1 0 0;");
    // Nothing was added by the calls that failed.
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![0.0]);
}

#[test]
fn emitted_particles_join_the_solve_and_go_on_rewind() {
    let mut c = Core::new();
    ok(&mut c, "particle -p 0 0 0 -n p; playbackOptions -min 1 -max 10; currentTime 1;");
    ok(&mut c, "currentTime 2; currentTime 3;");
    // Mid solve: added where the solve is, kept with those there.
    assert_eq!(ok(&mut c, "emit -o p -pos 0 10 0 -at velocity -vv 1 0 0;"), json!([1]));
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![2.0]);
    ok(&mut c, "currentTime 4;");
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![2.0], "still there on the next frame");
    let pos = nums(&ok(&mut c, "getParticleAttr -at position -array 1 p;"));
    assert!(pos[3] > 0.0, "it moved along its velocity: {pos:?}");
    // Back at the start only the particles of the initial state are there.
    ok(&mut c, "currentTime 1;");
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![1.0]);
}

#[test]
fn emit_at_the_start_frame_stays_until_the_solve_moves_on() {
    let mut c = Core::new();
    ok(&mut c, "particle -n p; playbackOptions -min 1 -max 10; currentTime 1;");
    assert_eq!(ok(&mut c, "emit -o p -pos 0 0 0 -pos 1 0 0;"), json!([0, 1]));
    // Laid out again at the start (another evaluation) they are still there, with the same ids.
    ok(&mut c, "currentTime 1;");
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![2.0]);
    assert_eq!(nums(&ok(&mut c, "getParticleAttr -at particleId -array 1 p;")), vec![0.0, 1.0]);
    ok(&mut c, "currentTime 2;");
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![2.0]);
    ok(&mut c, "currentTime 1;");
    assert_eq!(nums(&ok(&mut c, "particle -q -ct p;")), vec![0.0], "rewound: not part of the initial state");
}

#[test]
fn emit_from_a_script_call_with_one_attribute() {
    let mut c = Core::new();
    ok(&mut c, "particle -n p;");
    // As Python gives it: the flags' values without their order.
    let ids = c.exec("emit", json!({"object": "p", "position": [[1, 0, 0], [2, 0, 0]], "attribute": "velocity", "vectorValue": [[0, 1, 0], [0, 2, 0]]})).unwrap();
    assert_eq!(ids, json!([0, 1]));
    close(&nums(&ok(&mut c, "getParticleAttr -at velocity -array 1 p;")), &[0.0, 1.0, 0.0, 0.0, 2.0, 0.0], "velocities");
}
