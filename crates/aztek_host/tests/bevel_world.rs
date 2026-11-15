//! Issue 67: Bevel measures in world space by default, as the reference does, so a scaled object bevels as it shows.

use aztek_host::Core;

fn world_points(c: &mut Core, obj: &str) -> Vec<[f64; 3]> {
    let n = c.mel_checked(&format!("polyEvaluate -v {obj};")).unwrap().as_i64().unwrap();
    let mut v: Vec<[f64; 3]> = (0..n)
        .map(|i| {
            let p = c.mel_checked(&format!("pointPosition -w {obj}.vtx[{i}];")).unwrap();
            [p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap()].map(|x| (x * 1e6).round() / 1e6)
        })
        .collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

#[test]
fn a_scaled_cube_bevels_like_a_frozen_one() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; scale 12.86 0.336 1 a; polyBevel3 -oaf 1 -f 0.5 a.e[0:11];").unwrap();
    c.mel_checked("polyCube -n b; scale 12.86 0.336 1 b; makeIdentity -a 1 b; polyBevel3 -oaf 1 -f 0.5 b.e[0:11];").unwrap();
    assert_eq!(world_points(&mut c, "a"), world_points(&mut c, "b"));
    // The thin side keeps its faces: the bevel is a fraction of the shortest edge as it shows, 0.336.
    let ys: Vec<f64> = world_points(&mut c, "a").iter().map(|p| p[1]).collect();
    assert!(ys.iter().any(|y| (y.abs() - 0.168).abs() < 1e-6) && ys.iter().any(|y| y.abs() < 0.168 - 1e-3), "{ys:?}");
    // worldSpace off measures on the object's own points, as before.
    c.mel_checked("polyCube -n o; scale 12.86 0.336 1 o; polyBevel3 -oaf 1 -f 0.5 -ws 0 o.e[0:11];").unwrap();
    assert_ne!(world_points(&mut c, "o"), world_points(&mut c, "a"));
}

#[test]
fn the_bevel_menu_on_a_scaled_object_bevels_as_it_shows() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; scale 12.86 0.336 1 a; select -r a; polyBevel;").unwrap();
    c.mel_checked("polyCube -n b; scale 12.86 0.336 1 b; makeIdentity -a 1 b; select -r b; polyBevel;").unwrap();
    assert_eq!(world_points(&mut c, "a"), world_points(&mut c, "b"));
    // Undo takes it back in one step.
    c.mel_checked("undo;").unwrap();
    assert_eq!(c.mel_checked("polyEvaluate -v b;").unwrap(), serde_json::json!(8));
}
