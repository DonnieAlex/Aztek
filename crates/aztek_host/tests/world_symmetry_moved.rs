//! World symmetry follows a mesh moved after symmetry was set (PLAN 3.11).

use aztek_host::Core;
use serde_json::Value;

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn x_of(c: &mut Core, v: &str) -> f64 {
    mel(c, &format!("pointPosition -w {v};"))[0].as_f64().unwrap()
}

#[test]
fn world_pairs_follow_the_mesh() {
    let mut c = Core::new();
    // A plane across the world YZ plane: 4 by 1 faces from x = -2 to 2.
    mel(&mut c, "polyPlane -w 4 -h 1 -sx 4 -sy 1 -n p;");
    mel(&mut c, "symmetricModelling -e -symmetry 1 -about world -axis x;");
    // Moved by one unit after symmetry was set: x = -1 .. 3. World x = 1 (vtx[2]) now mirrors x = -1 (vtx[0]).
    mel(&mut c, "move -r 1 0 0 p;");
    assert!((x_of(&mut c, "p.vtx[2]") - 1.0).abs() < 1e-9 && (x_of(&mut c, "p.vtx[0]") + 1.0).abs() < 1e-9);
    mel(&mut c, "move -r 0 1 0 p.vtx[2];");
    let y = |c: &mut Core, v: &str| mel(c, &format!("pointPosition -w {v};"))[1].as_f64().unwrap();
    assert!((y(&mut c, "p.vtx[0]") - 1.0).abs() < 1e-9, "the mirror across world x = 0 moved");
    assert!(y(&mut c, "p.vtx[1]").abs() < 1e-9 && y(&mut c, "p.vtx[3]").abs() < 1e-9);
}
