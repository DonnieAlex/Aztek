//! The queries and edits Windows > Relationship Editors runs, one kind at a time.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn relationship_edits() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a; polySphere -n b; spaceLocator -n loc;");

    mel(&mut c, "createDisplayLayer -e -n lay;");
    mel(&mut c, "editDisplayLayerMembers lay a;");
    assert_eq!(mel(&mut c, "editDisplayLayerMembers -q lay;"), json!(["a"]));
    assert_eq!(mel(&mut c, "editDisplayLayerMembers -q -fn lay;"), json!(["|a"]));
    mel(&mut c, "editDisplayLayerMembers defaultLayer a;");
    assert_eq!(mel(&mut c, "editDisplayLayerMembers -q lay;"), json!([]));

    mel(&mut c, "sets -em -n s1; sets -add s1 b;");
    assert_eq!(mel(&mut c, "sets -q s1;"), json!(["b"]));
    mel(&mut c, "sets -rm s1 b;");
    assert_eq!(mel(&mut c, "sets -q s1;"), json!([]));

    mel(&mut c, "shadingNode -asShader lambert -n red; sets -renderable true -noSurfaceShader true -empty -n redSG; connectAttr red.outColor redSG.surfaceShader;");
    mel(&mut c, "sets -e -fe redSG b;");
    assert_eq!(mel(&mut c, "sets -q redSG;"), json!(["bShape"]));
    assert_eq!(mel(&mut c, "sets -q initialShadingGroup;"), json!(["aShape"]));

    mel(&mut c, "select a; character -n ch;");
    mel(&mut c, "character -add ch b;");
    assert_eq!(mel(&mut c, "character -q -no ch;"), json!(["a", "b"]));
    mel(&mut c, "character -rm ch b;");
    assert_eq!(mel(&mut c, "character -q -no ch;"), json!(["a"]));

    mel(&mut c, "pointLight -n pl;");
    assert_eq!(mel(&mut c, "ls -lights;"), json!(["plShape"]));
    mel(&mut c, "lightlink -b -light pl -object a;");
    assert_eq!(mel(&mut c, "lightlink -q -light pl;"), json!(["b", "bShape"]));
    mel(&mut c, "lightlink -make -light pl -object a;");
    assert!(mel(&mut c, "lightlink -q -light pl;").as_array().unwrap().contains(&json!("a")));

    mel(&mut c, "select b; cluster -n cl;");
    assert_eq!(mel(&mut c, "listConnections clSet.usedBy;"), json!(["cl"]));
    assert_eq!(mel(&mut c, "listConnections s1.usedBy;"), json!([]));
    assert_eq!(mel(&mut c, "listRelatives -p -f aShape;"), json!(["|a"]));
}

/// UV > Split (SplitUV), as the runtime command runs it.
const SPLIT_UV: &str = "polyForceUV -u `polyListComponentConversion -tuv`;";

#[test]
fn split_uv() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 2 -sy 2 -n p;");
    let centre = mel(&mut c, "polyListComponentConversion -fv -tuv p.vtx[4];");
    assert_eq!(centre.as_array().unwrap().len(), 1);
    mel(&mut c, &format!("select {};", centre[0].as_str().unwrap()));
    mel(&mut c, SPLIT_UV);
    // Four at the centre; the cut runs to the border, so each border middle splits in two as well.
    assert_eq!(mel(&mut c, "polyListComponentConversion -fv -tuv p.vtx[4];").as_array().unwrap().len(), 4);
    assert_eq!(mel(&mut c, "polyEvaluate -uv p;"), json!(16));
    mel(&mut c, "undo;");
    assert_eq!(mel(&mut c, "polyEvaluate -uv p;"), json!(9));
    // From a vertex too.
    mel(&mut c, "select p.vtx[4];");
    mel(&mut c, SPLIT_UV);
    assert_eq!(mel(&mut c, "polyListComponentConversion -fv -tuv p.vtx[4];").as_array().unwrap().len(), 4);
}

/// Deform > Edit > Wrap > Add Influence and Remove Influence, Wire > Add Wire: the wrapped (or wired) object first,
/// the influence (or curve) last.
const ADD_WRAP: &str = "{ string $s[] = `ls -sl -o`; if (size($s) < 2) error \"Select the wrapped object, then the influence\"; string $w[] = `ls -type wrap (listHistory($s[0]))`; if (!size($w)) error (\"No wrap on \" + $s[0]); wrap -e -inf $s[size($s) - 1] $w[0]; }";
const REMOVE_WRAP: &str = "{ string $s[] = `ls -sl -o`; if (size($s) < 2) error \"Select the wrapped object, then the influence\"; string $w[] = `ls -type wrap (listHistory($s[0]))`; if (!size($w)) error (\"No wrap on \" + $s[0]); wrap -e -ri $s[size($s) - 1] $w[0]; }";

#[test]
fn wrap_influences() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 4 -sy 4 -n target; polyCube -n w1; polyCube -n w2; move 3 0 0 w2;");
    mel(&mut c, "select target w1; wrap;");
    let w = mel(&mut c, "ls -type wrap;");
    let name = w[0].as_str().unwrap().to_string();
    assert_eq!(mel(&mut c, &format!("wrap -q -inf {name};")).as_array().unwrap().len(), 1);
    mel(&mut c, "select target w2;");
    mel(&mut c, ADD_WRAP);
    assert_eq!(mel(&mut c, &format!("wrap -q -inf {name};")).as_array().unwrap().len(), 2);
    mel(&mut c, "select target w2;");
    mel(&mut c, REMOVE_WRAP);
    assert_eq!(mel(&mut c, &format!("wrap -q -inf {name};")).as_array().unwrap().len(), 1);
}

const ADD_WIRE: &str = "{ string $s[] = `ls -sl -o`; if (size($s) < 2) error \"Select the wired object, then the curve\"; string $w[] = `ls -type wire (listHistory($s[0]))`; if (!size($w)) error (\"No wire on \" + $s[0]); wire -e -w $s[size($s) - 1] $w[0]; }";

#[test]
fn wire_add() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 4 -sy 4 -n target; curve -d 1 -p -1 0 0 -p 1 0 0 -n c1; curve -d 1 -p 0 0 -1 -p 0 0 1 -n c2;");
    mel(&mut c, "wire -w c1 target;");
    let w = mel(&mut c, "ls -type wire;")[0].as_str().unwrap().to_string();
    mel(&mut c, "select target c2;");
    mel(&mut c, ADD_WIRE);
    assert_eq!(mel(&mut c, &format!("wire -q -w {w};")).as_array().unwrap().len(), 2);
}
