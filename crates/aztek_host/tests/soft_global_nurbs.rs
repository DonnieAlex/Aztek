//! Soft select Global reaches NURBS (PLAN 3.11): a mesh's selection pulls a curve's CVs nearby, and a curve's CVs pull
//! a mesh.

use aztek_host::Core;
use serde_json::Value;

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn y(c: &mut Core, p: &str) -> f64 {
    mel(c, &format!("pointPosition -w {p};"))[1].as_f64().unwrap()
}

#[test]
fn global_reaches_curves_and_back() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -w 1 -h 1 -sx 1 -sy 1 -n p;");
    mel(&mut c, "curve -d 1 -p 1 0 0 -p 2 0 0 -p 10 0 0 -n cv;");
    mel(&mut c, "softSelect -e -softSelectEnabled true -softSelectDistance 3 -softSelectFalloff 2;");
    mel(&mut c, "move -r 0 1 0 p.vtx[1];");
    assert!((y(&mut c, "p.vtx[1]") - 1.0).abs() < 1e-9);
    let near = y(&mut c, "cv.cv[0]");
    assert!(near > 0.05 && near < 1.0, "a CV in reach moves part of the way: {near}");
    assert!(y(&mut c, "cv.cv[2]").abs() < 1e-9, "a CV out of reach stays");
    mel(&mut c, "undo;");
    assert!(y(&mut c, "cv.cv[0]").abs() < 1e-9);
    // From the curve to the mesh.
    mel(&mut c, "move -r 0 1 0 cv.cv[0];");
    let m = y(&mut c, "p.vtx[1]");
    assert!(m > 0.05 && m < 1.0, "the mesh takes its share: {m}");
    // Volume mode keeps to the object.
    mel(&mut c, "undo; softSelect -e -softSelectFalloff 0; move -r 0 1 0 cv.cv[0];");
    assert!(y(&mut c, "p.vtx[1]").abs() < 1e-9);
}
