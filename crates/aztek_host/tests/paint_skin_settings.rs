//! The Paint Skin Weights tool's settings: the colour ramp, a value past 1, and the hammer working on vertices only.

use aztek_host::Core;
use serde_json::json;

fn rig(c: &mut Core) {
    c.mel("polyCylinder -r 1 -h 6 -sy 12 -sx 16; move 0 3 0; select -cl; joint -p 0 0 0; joint -p 0 3 0; joint -p 0 6 0; skinCluster joint1 pCylinder1;");
}

#[test]
fn the_colour_ramp_and_a_value_past_one() {
    let mut c = Core::new();
    rig(&mut c);
    assert_eq!(c.mel("artAttrSkinPaintCtx -q -ucr artAttrSkinContext;"), json!(false));
    c.mel("artAttrSkinPaintCtx -e -ucr true artAttrSkinContext;");
    assert_eq!(c.mel("artAttrSkinPaintCtx -q -ucr artAttrSkinContext;"), json!(true));
    c.mel("artAttrSkinPaintCtx -e -maxvalue 2 -value 1.5 artAttrSkinContext;");
    assert_eq!(c.mel("artAttrSkinPaintCtx -q -value artAttrSkinContext;"), json!(1.5));
    assert_eq!(c.mel("artAttrSkinPaintCtx -q -mxv artAttrSkinContext;"), json!(2.0));
    // Within the range only.
    c.mel("artAttrSkinPaintCtx -e -value 3 artAttrSkinContext;");
    assert_eq!(c.mel("artAttrSkinPaintCtx -q -value artAttrSkinContext;"), json!(2.0));
}

#[test]
fn the_hammer_leaves_a_whole_object_alone() {
    let mut c = Core::new();
    rig(&mut c);
    let before = c.mel("skinPercent -q -v skinCluster1 pCylinder1.vtx[100];");
    c.mel("select -r pCylinder1; weightHammerVerts;");
    assert_eq!(c.mel("skinPercent -q -v skinCluster1 pCylinder1.vtx[100];"), before);
    // The vertex picked is.
    c.mel("select -r pCylinder1.vtx[100]; weightHammerVerts;");
    assert_ne!(c.mel("skinPercent -q -v skinCluster1 pCylinder1.vtx[100];"), before);
}
