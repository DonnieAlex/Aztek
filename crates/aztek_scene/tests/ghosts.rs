//! Ghosts for the views: a ghosted object's meshes at its ghost frames, coloured and faded.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[test]
fn ghosts_are_the_meshes_at_their_frames() {
    let mut s = Session::new();
    run(&mut s, "polyCube", json!({"name": "box"}));
    run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 1.0, "value": 0.0}));
    run(&mut s, "setKeyframe", json!({"args": ["box.tx"], "time": 21.0, "value": 20.0}));
    run(&mut s, "keyTangent", json!({"args": ["box_translateX"], "edit": true, "inTangentType": "linear", "outTangentType": "linear"}));
    run(&mut s, "currentTime", json!({"args": [10.0]}));
    assert!(aztek_scene::anim_extra::ghosts(&mut s.graph).is_empty());
    run(&mut s, "ghosting", json!({"preFrames": 2, "postFrames": 1, "ghostsStep": 2}));
    run(&mut s, "ghosting", json!({"args": ["box"], "action": "ghost"}));
    let gh = aztek_scene::anim_extra::ghosts(&mut s.graph);
    let frames: Vec<f64> = gh.iter().map(|x| x.frame).collect();
    assert_eq!(frames, vec![6.0, 8.0, 12.0]);
    // Where the box is at each: tx is frame - 1 on this linear curve.
    for x in &gh {
        assert!((x.world.w_axis.x - (x.frame - 1.0)).abs() < 1e-6, "{} {}", x.frame, x.world.w_axis.x);
    }
    // The nearest before is brighter than the farthest; after is in the post colour.
    assert!(gh[1].opacity > gh[0].opacity && (gh[1].opacity - 0.5).abs() < 1e-6 && (gh[0].opacity - 0.1).abs() < 1e-6);
    assert_eq!(gh[0].color, [0.5, 0.5, 1.0]);
    assert_eq!(gh[2].color, [1.0, 0.5, 0.5]);
    // The scene is back at frame 10.
    assert_eq!(run(&mut s, "getAttr", json!({"args": ["box.tx"]})), json!(9.0));
    run(&mut s, "ghosting", json!({"enable": false}));
    assert!(aztek_scene::anim_extra::ghosts(&mut s.graph).is_empty());
    run(&mut s, "ghosting", json!({"enable": true}));
    run(&mut s, "ghosting", json!({"action": "unghostAll"}));
    assert!(aztek_scene::anim_extra::ghosts(&mut s.graph).is_empty());
}
