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
