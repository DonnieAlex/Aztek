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

fn locked(c: &mut Core, attr: &str) -> bool {
    c.mel(&format!("getAttr -lock ball.{attr}")) == json!(true)
}

/// #207: a skin locks the bound transform, and deleting the history takes the skin away: the object moves again.
#[test]
fn deleting_a_skins_history_frees_the_bound_transform() {
    let mut c = Core::new();
    mel(&mut c, "polySphere -n ball; select -cl; joint -n j;");
    mel(&mut c, "skinCluster -n sk j ball;");
    assert!(locked(&mut c, "tx") && locked(&mut c, "ry") && locked(&mut c, "sz"), "a bind locks the transform");
    mel(&mut c, "DeleteHistory ball;");
    assert!(!locked(&mut c, "tx") && !locked(&mut c, "ry") && !locked(&mut c, "sz"), "the history took the skin with it");
    mel(&mut c, "setAttr ball.tx 4;");
    assert_eq!(mel(&mut c, "getAttr ball.tx;"), json!(4.0));
    assert_eq!(mel(&mut c, "objExists sk;"), json!(false), "the skinCluster went with the history");
}

/// A history that holds no skin leaves the locks as they were: unlocking is the skin's doing, not Delete History's.
#[test]
fn deleting_plain_history_keeps_a_lock_the_user_made() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n ball;");
    mel(&mut c, "setAttr ball.tx -lock true;");
    mel(&mut c, "DeleteHistory ball;");
    assert!(locked(&mut c, "tx"), "a lock of the user's is not the skin's to take");
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
