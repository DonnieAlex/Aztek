//! Issues of 18/11/2026.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

/// #141: the tangent types the reference's API and keyTangent know, auto mix, ease and custom among them.
#[test]
fn tangent_types_of_the_reference() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n k; setKeyframe -t 1 -v 0 k.tx; setKeyframe -t 10 -v 5 k.tx; setKeyframe -t 20 -v 1 k.tx;");
    for t in ["autoease", "automix", "autocustom", "slow", "fast"] {
        mel(&mut c, &format!("keyTangent -e -t 10 -itt {t} -ott {t} k.tx;"));
        assert_eq!(mel(&mut c, "keyTangent -q -t 10 -ott k.tx;"), json!([t]));
    }
    for (name, v) in [("kTangentAuto", 11), ("kTangentAutoMix", 12), ("kTangentAutoEase", 13), ("kTangentAutoCustom", 14)] {
        mel(&mut c, "python(\"import maya.OpenMayaAnim as oma; import maya.api.OpenMayaAnim as oma2\");");
        assert_eq!(mel(&mut c, &format!("python(\"oma.MFnAnimCurve.{name}\");")), json!(v), "API 1.0 {name}");
        assert_eq!(mel(&mut c, &format!("python(\"oma2.MFnAnimCurve.{name}\");")), json!(v), "API 2.0 {name}");
    }
    // The API reads and sets them by name.
    mel(&mut c, "keyTangent -e -t 10 -ott autoease k.tx;");
    mel(&mut c, "python(\"import maya.api.OpenMaya as om2; sl = om2.MSelectionList(); sl.add('k_translateX'); crv = oma2.MFnAnimCurve(sl.getDependNode(0))\");");
    assert_eq!(mel(&mut c, "python(\"crv.outTangentType(1)\");"), json!(13));
}

fn points(c: &mut Core, shape: &str) -> Vec<[f64; 3]> {
    let v = mel(c, &format!("xform -q -ws -t \"{shape}.vtx[*]\";"));
    v.as_array().unwrap().chunks(3).map(|p| [p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap()]).collect()
}

/// #143: the reference's attributes, a texture connected to .texture moving each point by its own colour, the handle
/// giving the direction, and strength and offset as the reference applies them.
#[test]
fn texture_deformer_samples_its_texture() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -n p -w 4 -h 4 -sx 8 -sy 8;");
    let made = mel(&mut c, "textureDeformer p;");
    assert_eq!(made, json!(["textureDeformer1", "textureDeformerHandle1"]));
    assert_eq!(mel(&mut c, "getAttr textureDeformer1.pointSpace;"), json!(2));
    assert_eq!(mel(&mut c, "getAttr textureDeformer1.direction;"), json!(1));
    assert_eq!(mel(&mut c, "listConnections -s 1 -d 0 textureDeformer1.handleMatrix;"), json!(["textureDeformerHandle1"]));
    // Nothing connected and a black texture: no move.
    let flat = points(&mut c, "p");
    assert!(flat.iter().all(|q| q[1].abs() < 1e-9));
    // A checker: white squares go up by strength, black stay; offset -strength/2 centres them on the plane.
    mel(&mut c, "shadingNode -asTexture checker -n chk; connectAttr chk.outColor textureDeformer1.texture;");
    mel(&mut c, "setAttr textureDeformer1.strength 0.16;");
    let ys: Vec<f64> = points(&mut c, "p").iter().map(|q| q[1]).collect();
    assert!(ys.iter().any(|y| (y - 0.16).abs() < 1e-3), "{ys:?}");
    assert!(ys.iter().any(|y| y.abs() < 1e-3), "{ys:?}");
    assert!(ys.iter().all(|y| *y > -1e-6 && *y < 0.16 + 1e-6), "{ys:?}");
    mel(&mut c, "setAttr textureDeformer1.offset -0.08;");
    let ys: Vec<f64> = points(&mut c, "p").iter().map(|q| q[1]).collect();
    assert!(ys.iter().all(|y| (y.abs() - 0.08).abs() < 1e-3), "{ys:?}");
    // The texture's own settings reach the points.
    mel(&mut c, "setAttr chk.color1 -type double3 0 0 0;");
    let ys: Vec<f64> = points(&mut c, "p").iter().map(|q| q[1]).collect();
    assert!(ys.iter().all(|y| (y + 0.08).abs() < 1e-3), "{ys:?}");
    // Strength is uncapped, and the handle's up axis is the direction.
    mel(&mut c, "setAttr chk.color1 -type double3 1 1 1; setAttr textureDeformer1.strength 16; setAttr textureDeformer1.offset 0; rotate 0 0 90 textureDeformerHandle1;");
    let pts = points(&mut c, "p");
    assert!(pts.iter().any(|q| q[0] < -10.0), "{pts:?}");
    assert!(pts.iter().all(|q| q[1].abs() < 1e-6));
    // Normal direction on a plane is up again.
    mel(&mut c, "setAttr textureDeformer1.direction 0;");
    assert!(points(&mut c, "p").iter().any(|q| (q[1] - 16.0).abs() < 1e-3));
    // The texture row of the Attribute Editor takes a texture through its map button.
    let ae = aztek_host::query::attribute_editor(&c, "textureDeformer1")["tabs"][0].clone();
    let tex = ae["attrs"].as_array().unwrap().iter().find(|a| a["attr"] == "texture").cloned().unwrap_or_else(|| panic!("{ae}"));
    assert_eq!(tex["color"], json!(true));
    assert_eq!(tex["mappable"], json!(true));
    for a in ["strength", "offset"] {
        let row = ae["attrs"].as_array().unwrap().iter().find(|x| x["attr"] == a).unwrap().clone();
        assert_eq!(row["hardMin"], Value::Null, "{a}");
        assert_eq!(row["hardMax"], Value::Null, "{a}");
    }
}

/// #143: a texture deformer, its handle and its texture come back from a saved file.
#[test]
fn texture_deformer_saves() {
    let mut c = Core::new();
    let d = std::env::temp_dir().join("aztek_issue143");
    let _ = std::fs::create_dir_all(&d);
    let f = d.join("td.ma");
    mel(&mut c, "polyPlane -n p -w 4 -h 4 -sx 8 -sy 8; textureDeformer p; shadingNode -asTexture checker -n chk; connectAttr chk.outColor textureDeformer1.texture; setAttr textureDeformer1.strength 0.5;");
    let before = points(&mut c, "p");
    mel(&mut c, &format!("file -rename \"{}\"; file -save -type \"mayaAscii\";", f.display()));
    mel(&mut c, &format!("file -f -new; file -o \"{}\";", f.display()));
    assert_eq!(mel(&mut c, "listConnections -s 1 -d 0 textureDeformer1.handleMatrix;"), json!(["textureDeformerHandle1"]));
    let after = points(&mut c, "p");
    for (a, b) in before.iter().zip(&after) {
        assert!((a[1] - b[1]).abs() < 1e-6, "{before:?} {after:?}");
    }
}
