//! Issues of 20/11/2026: the Channel Box's outputs, the locks a skin leaves behind, and a locked attribute in the
//! Attribute Editor (#206, #207, #212).

use aztek_host::{query, Core};
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

/// The Attribute Editor's row for an attribute, over every tab.
fn ae_attr<'a>(ae: &'a Value, name: &str) -> &'a Value {
    for t in ae["tabs"].as_array().expect("tabs") {
        for a in t["attrs"].as_array().expect("attrs") {
            if a["attr"] == name {
                return a;
            }
        }
    }
    panic!("{name} is not in the attribute editor: {ae}");
}

fn locked(c: &mut Core, node: &str, attr: &str) -> bool {
    c.mel(&format!("getAttr -lock {node}.{attr}")) == json!(true)
}

/// #207: a skin locks the bound transform, and deleting the history takes the skin away: the object moves again.
#[test]
fn deleting_a_skins_history_frees_the_bound_transform() {
    let mut c = Core::new();
    mel(&mut c, "polySphere -n ball; select -cl; joint -n j;");
    mel(&mut c, "skinCluster -n sk j ball;");
    assert!(locked(&mut c, "ball", "tx") && locked(&mut c, "ball", "ry") && locked(&mut c, "ball", "sz"), "a bind locks the transform");
    mel(&mut c, "DeleteHistory ball;");
    assert!(!locked(&mut c, "ball", "tx") && !locked(&mut c, "ball", "ry") && !locked(&mut c, "ball", "sz"), "the history took the skin with it");
    mel(&mut c, "setAttr ball.tx 4;");
    assert_eq!(mel(&mut c, "getAttr ball.tx;"), json!(4.0));
    assert_eq!(mel(&mut c, "objExists sk;"), json!(false), "the skinCluster went with the history");
}

/// A deformer that does not lock the transform leaves the locks as they were: unlocking is a binding deformer's doing,
/// not Delete History's. A lock the user made on a skinned object survives too, unless it is one the bind set.
#[test]
fn deleting_history_keeps_a_lock_no_binding_deformer_made() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n ball;");
    mel(&mut c, "nonLinear -type bend -n bend1 ball;");
    mel(&mut c, "setAttr ball.tx -lock true;");
    mel(&mut c, "DeleteHistory ball;");
    assert!(locked(&mut c, "ball", "tx"), "a lock of the user's is not the history's to take");

    // A skinned object's own lock, off the channels the bind set, is the user's as well.
    mel(&mut c, "polySphere -n skinned; select -cl; joint -n j2; skinCluster -n sk2 j2 skinned;");
    mel(&mut c, "setAttr skinned.visibility -lock true;");
    mel(&mut c, "DeleteHistory skinned;");
    assert!(!locked(&mut c, "skinned", "tx"), "the bind's lock went with the skin");
    assert_eq!(c.mel("getAttr -lock skinned.visibility"), json!(true), "the user's own lock stayed");
}

/// #206: the Channel Box's outputs are every node the selection drives, one level deep and all of them: a joint lists
/// each skin bound to it.
#[test]
fn a_joint_lists_every_skin_as_an_output() {
    let mut c = Core::new();
    mel(&mut c, "joint -n j;");
    for i in 1..=5 {
        mel(&mut c, &format!("polySphere -n ball{i}; select -cl; skinCluster -n sk{i} j ball{i};"));
    }
    mel(&mut c, "select -r j;");
    let outs = mel(&mut c, "channelBox -q -outputs mainChannelBox;");
    let outs: Vec<&str> = outs.as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    for i in 1..=5 {
        assert!(outs.contains(&format!("sk{i}").as_str()), "sk{i} missing from {outs:?}");
    }
    assert_eq!(mel(&mut c, "channelBox -q -inputs mainChannelBox;"), json!([]));
}

/// #212: the Attribute Editor is told which attributes are locked, a vector's parts one by one, so its fields can show
/// the lock and take no typing.
#[test]
fn the_attribute_editor_knows_the_locks() {
    let mut c = Core::new();
    mel(&mut c, "polySphere -n ball;");
    let ae = query::attribute_editor(&c, "ball");
    let translate = ae_attr(&ae, "translate");
    assert_eq!(translate["childLocked"], json!([false, false, false]));
    assert_eq!(translate["locked"], json!(false));

    mel(&mut c, "setAttr ball.translateX -lock true;");
    let ae = query::attribute_editor(&c, "ball");
    let translate = ae_attr(&ae, "translate");
    assert_eq!(translate["childLocked"], json!([true, false, false]), "only the part that was locked");
    assert_eq!(translate["locked"], json!(false), "the whole is not itself locked");

    // A lock on the whole attribute reaches every part of it.
    mel(&mut c, "setAttr ball.rotate -lock true;");
    let ae = query::attribute_editor(&c, "ball");
    let rotate = ae_attr(&ae, "rotate");
    assert_eq!(rotate["locked"], json!(true));
    assert_eq!(rotate["childLocked"], json!([true, true, true]));

    // And a plain attribute carries the flag its row reads.
    mel(&mut c, "setAttr ball.visibility -lock true;");
    let ae = query::attribute_editor(&c, "ball");
    assert_eq!(ae_attr(&ae, "visibility")["locked"], json!(true));
}
