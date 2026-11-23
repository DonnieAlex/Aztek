//! What building an mGear 4 guide (the Kaneda character's, EPIC components) asked of Aztek. Its own file: the Python
//! here starts in a process of its own.

use aztek_host::Core;

/// What building an mGear 4 guide (an EPIC spine) asked for: a deformer made on a hidden object just selected, and
/// OpenMaya 1.0's getParamAtPoint into an MScriptUtil pointer.
#[test]
fn a_hidden_selected_curve_takes_a_deformer_and_om1_finds_a_param() {
    let mut c = Core::new();
    c.mel("polyCube -n box; setAttr box.v 0; select box;");
    let made = c.mel_checked("deformer -type cluster;");
    assert!(made.is_ok(), "{made:?}");
    // The cluster command still leaves a hidden selection out.
    assert!(c.mel_checked("select box; cluster;").is_err());
    c.mel("curve -d 1 -p 0 0 0 -p 10 0 0 -n crv;");
    let r = c.mel_checked(
        "python(\"import maya.OpenMaya as om\\n\
sl = om.MSelectionList(); sl.add('crv'); dag = om.MDagPath(); sl.getDagPath(0, dag)\\n\
fn = om.MFnNurbsCurve(dag); u = om.MScriptUtil(); p = u.asDoublePtr()\\n\
fn.getParamAtPoint(om.MPoint(4, 0, 0), p, 0.001, om.MSpace.kObject)\\n\
assert abs(u.getDouble(p) - 0.4) < 1e-6, u.getDouble(p)\")",
    );
    assert!(r.is_ok(), "{r:?}");
}
