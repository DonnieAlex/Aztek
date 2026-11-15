//! Issue 66: clusters on curve and surface CVs, through the original shape and the CV bridge, as the reference does.

use aztek_host::Core;

fn at(c: &mut Core, what: &str) -> [f64; 3] {
    let v = c.mel_checked(&format!("pointPosition {what};")).unwrap();
    let a: Vec<f64> = v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    [a[0], a[1], a[2]]
}

fn near(a: [f64; 3], b: [f64; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-6)
}

#[test]
fn clusters_take_curve_and_surface_cvs() {
    let mut c = Core::new();
    c.mel_checked("curve -n lin -d 1 -p 0 0 0 -p 1 0 0 -p 2 0 0;").unwrap();
    c.mel_checked("cluster -n c1 lin.cv[1]; move -r 0 3 0 c1Handle;").unwrap();
    assert!(near(at(&mut c, "lin.cv[1]"), [1.0, 3.0, 0.0]));
    assert!(near(at(&mut c, "lin.cv[0]"), [0.0, 0.0, 0.0]), "only the picked CV follows");
    c.mel_checked("circle -n ring; cluster -n c2 ring.cv[0:2]; move -r 0 0 5 c2Handle;").unwrap();
    assert!((at(&mut c, "ring.cv[2]")[2] - 5.0).abs() < 1e-6);
    assert!(at(&mut c, "ring.cv[4]")[2].abs() < 1e-6);
    c.mel_checked("nurbsPlane -n srf -u 2 -v 2; cluster -n c3 srf.cv[0][0:1]; move -r 0 7 0 c3Handle;").unwrap();
    let before = at(&mut c, "srf.cv[1][0]");
    let moved = at(&mut c, "srf.cv[0][1]");
    // A second cluster on the same curve joins the chain.
    c.mel_checked("cluster -n c4 lin.cv[2]; move -r 0 0 2 c4Handle;").unwrap();
    assert!(near(at(&mut c, "lin.cv[2]"), [2.0, 0.0, 2.0]));
    assert!(near(at(&mut c, "lin.cv[1]"), [1.0, 3.0, 0.0]));
    let shapes = c.mel_checked("listRelatives -s lin;").unwrap();
    assert_eq!(shapes, serde_json::json!(["linShape", "linShapeOrig"]));
    // Saved and opened again, the chain and the moves come back.
    let path = std::env::temp_dir().join(format!("aztek_nurbs_cluster_{}.ma", std::process::id())).display().to_string().replace('\\', "/");
    c.mel_checked(&format!("file -rename \"{path}\"; file -save -type \"mayaAscii\";")).unwrap();
    let mut d = Core::new();
    d.mel_checked(&format!("file -f -open \"{path}\";")).unwrap();
    assert!(near(at(&mut d, "lin.cv[1]"), [1.0, 3.0, 0.0]));
    assert!(near(at(&mut d, "lin.cv[2]"), [2.0, 0.0, 2.0]));
    assert!(near(at(&mut d, "srf.cv[0][1]"), moved));
    assert!(near(at(&mut d, "srf.cv[1][0]"), before));
    d.mel_checked("move -r 0 1 0 c1Handle;").unwrap();
    assert!(near(at(&mut d, "lin.cv[1]"), [1.0, 4.0, 0.0]), "still live");
}

#[test]
fn undoing_a_curve_cluster_restores_the_curve() {
    let mut c = Core::new();
    c.mel_checked("curve -n lin -d 1 -p 0 0 0 -p 1 0 0 -p 2 0 0;").unwrap();
    c.mel_checked("cluster -n c1 lin.cv[1];").unwrap();
    c.mel_checked("undo;").unwrap();
    assert_eq!(c.mel_checked("listRelatives -s lin;").unwrap(), serde_json::json!(["linShape"]));
    assert!(near(at(&mut c, "lin.cv[1]"), [1.0, 0.0, 0.0]));
    c.mel_checked("move -r 0 2 0 lin.cv[1];").unwrap();
    assert!(near(at(&mut c, "lin.cv[1]"), [1.0, 2.0, 0.0]), "the curve edits as before");
}

#[test]
fn clusters_on_a_spline_ik_curve_drive_the_chain() {
    let mut c = Core::new();
    c.mel_checked("select -cl; joint -n j1 -p 0 0 0; joint -n j2 -p 0 3 0; joint -n j3 -p 0 6 0;").unwrap();
    c.mel_checked("curve -n crv -d 1 -p 0 0 0 -p 0 3 0 -p 0 6 0;").unwrap();
    c.mel_checked("ikHandle -n ik -sol ikSplineSolver -ccv false -sj j1 -ee j3 -c crv;").unwrap();
    c.mel_checked("cluster -n top crv.cv[2]; move -r 4 0 0 topHandle;").unwrap();
    let end = c.mel_checked("xform -q -ws -t j3;").unwrap();
    let x = end[0].as_f64().unwrap();
    assert!(x > 1.0, "the chain follows the cluster: {end}");
}
