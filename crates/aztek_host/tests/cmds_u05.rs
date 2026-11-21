//! Commands of the reference from nurbsPlane to polyAxis: the new overrideModifier, paramLocator, partition,
//! pluginDisplayFilter and polyAxis, and the flags the others gained (the NURBS history commands' query and edit, the
//! square's four sides, nurbsToPoly's novice tessellation, offsets that cut loops and join breaks, curvature on curves,
//! the constraint cache, percent's dropoffs, playblast's grading and sound, the panel and layout flags and the rest).

use aztek_host::{take_ui_requests_for_tests as take_ui_requests, Core};
use serde_json::{json, Value};

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) if a.len() == 1 => num(&a[0]),
        Value::Bool(b) => *b as i64 as f64,
        x => x.as_f64().unwrap_or_else(|| panic!("not a number: {x}")),
    }
}

fn nums(v: &Value) -> Vec<f64> {
    fn walk(v: &Value, out: &mut Vec<f64>) {
        match v {
            Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            Value::Bool(b) => out.push(*b as i64 as f64),
            x => out.extend(x.as_f64()),
        }
    }
    let mut out = Vec::new();
    walk(v, &mut out);
    out
}

fn strs(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Value::String(s) if !s.is_empty() => vec![s.clone()],
        _ => Vec::new(),
    }
}

fn close(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() < 1e-3, "{what}: {a} vs {b}");
}

fn close3(a: &[f64], b: [f64; 3], what: &str) {
    for i in 0..3 {
        assert!((a[i] - b[i]).abs() < 1e-3, "{what}: {a:?} vs {b:?}");
    }
}

#[test]
fn nurbs_square_four_sides() {
    let mut c = Core::new();
    assert_eq!(strs(&ok(&mut c, "nurbsSquare -sl1 2 -sl2 4;")), vec!["nurbsSquare1", "makeNurbsSquare1"]);
    assert_eq!(strs(&ok(&mut c, "listRelatives -c nurbsSquare1;")), vec!["topnurbsSquare1", "leftnurbsSquare1", "bottomnurbsSquare1", "rightnurbsSquare1"]);
    // Normal Z by default: the square in XY, 2 wide and 4 high.
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0 topnurbsSquare1;")), [-1.0, 2.0, 0.0], "top start");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 1 topnurbsSquare1;")), [1.0, 2.0, 0.0], "top end");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0.5 rightnurbsSquare1;")), [1.0, 0.0, 0.0], "right middle");
    close(num(&ok(&mut c, "nurbsSquare -q -sl2 makeNurbsSquare1;")), 4.0, "query through the node");
    ok(&mut c, "nurbsSquare -e -sps 3 -d 1 nurbsSquare1;");
    assert_eq!(num(&ok(&mut c, "getAttr topnurbsSquareShape1.spans;")), 3.0);
    assert_eq!(num(&ok(&mut c, "nurbsSquare -q -d nurbsSquare1;")), 1.0);
    // Centre and normal by their parts; -ch off leaves no node; -o off only the node.
    ok(&mut c, "nurbsSquare -cx 5 -cy 1 -nrx 0 -nry 1 -nrz 0 -n flat;");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0.5 topflat;")), [5.0, 1.0, -0.5], "facing up: top toward -Z");
    assert_eq!(strs(&ok(&mut c, "nurbsSquare -ch 0 -n bare;")), vec!["bare"]);
    assert!(strs(&ok(&mut c, "listHistory topbare;")).iter().all(|n| !n.starts_with("makeNurbsSquare")));
    let only = strs(&ok(&mut c, "nurbsSquare -o 0;"));
    assert_eq!(only.len(), 1);
    assert!(only[0].starts_with("makeNurbsSquare"));
    // Blocking keeps what it gave.
    ok(&mut c, "nurbsSquare -e -nds 2 makeNurbsSquare1; setAttr makeNurbsSquare1.sideLength1 10;");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0 topnurbsSquare1;")), [-1.0, 2.0, 0.0], "blocked");
    assert_eq!(num(&ok(&mut c, "nurbsSquare -q -nds makeNurbsSquare1;")), 2.0);
}

#[test]
fn planar_from_sides_holes_and_keep_outside() {
    let mut c = Core::new();
    ok(&mut c, "nurbsSquare -sl1 2 -sl2 2 -nr 0 1 0;");
    let r = strs(&ok(&mut c, "planarSrf nurbsSquare1;"));
    assert_eq!(r, vec!["planarTrimmedSurface1", "planarTrim1"]);
    assert_eq!(num(&ok(&mut c, "getAttr planarTrimmedSurface1.degreeU;")), 1.0);
    assert_eq!(num(&ok(&mut c, "planarSrf -q -ko planarTrim1;")), 0.0);
    close(num(&ok(&mut c, "planarSrf -q -tol planarTrim1;")), 0.01, "tolerance");
    // Two circles: the inner one a hole; -ko keeps the rest instead.
    ok(&mut c, "circle -r 3 -nr 0 1 0 -n outer; circle -r 1 -nr 0 1 0 -n inner;");
    ok(&mut c, "planarSrf -n holed -d 1 outer inner;");
    assert_eq!(num(&ok(&mut c, "planarSrf -q -d holed;")), 1.0);
    let faces = num(&ok(&mut c, "polyEvaluate -f (`nurbsToPoly -ch 0 holed`);"));
    assert!(faces > 0.0);
    ok(&mut c, "planarSrf -e -ko 1 holed;");
    assert_eq!(num(&ok(&mut c, "planarSrf -q -ko holed;")), 1.0);
    // Curves not in one plane make nothing.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 1 0 0 -p 1 1 0 -p 0 1 1 -p 0 0 0 -n bent;");
    let s = strs(&ok(&mut c, "planarSrf -tol 0.001 bent;"));
    assert_eq!(nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", s[0]))).iter().map(|x| x.abs()).sum::<f64>(), 0.0, "nothing made");
    let loose = strs(&ok(&mut c, "planarSrf -tol 2 bent;"));
    assert!(nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", loose[0])))[3] > 0.5, "within a loose tolerance");
    let o = strs(&ok(&mut c, "planarSrf -o 0 outer;"));
    assert_eq!(o.len(), 1);
}

#[test]
fn nurbs_to_poly_novice_and_history_flags() {
    let mut c = Core::new();
    ok(&mut c, "sphere -n s1;");
    assert_eq!(strs(&ok(&mut c, "nurbsToPoly -eta 0 -nuf 2 -nvf 2 -cvt 3 s1;")), vec!["nurbsToPoly1", "nurbsTessellate1"]);
    let low = num(&ok(&mut c, "polyEvaluate -f nurbsToPoly1;"));
    ok(&mut c, "nurbsToPoly -e -cvt 0 nurbsTessellate1;");
    let high = num(&ok(&mut c, "polyEvaluate -f nurbsToPoly1;"));
    assert!(high > low, "more where it curves: {low} {high}");
    close(num(&ok(&mut c, "nurbsToPoly -q -nuf nurbsTessellate1;")), 2.0, "factor");
    assert_eq!(num(&ok(&mut c, "nurbsToPoly -q -eta nurbsToPoly1;")), 0.0);
    // The factor drives the count.
    ok(&mut c, "nurbsToPoly -e -nuf 4 -cvt 3 nurbsTessellate1;");
    assert!(num(&ok(&mut c, "polyEvaluate -f nurbsToPoly1;")) > low);
    // Smooth edges: a coarse count is divided until the borders are smooth.
    ok(&mut c, "cylinder -n cy -s 8;");
    ok(&mut c, "nurbsToPoly -f 2 -ut 3 -un 1 -vt 3 -vn 1 -n coarse cy;");
    ok(&mut c, "nurbsToPoly -f 2 -ut 3 -un 1 -vt 3 -vn 1 -ues 1 -esr 0.99 -n smooth cy;");
    assert!(num(&ok(&mut c, "polyEvaluate -f smooth;")) > num(&ok(&mut c, "polyEvaluate -f coarse;")));
    close(num(&ok(&mut c, "nurbsToPoly -q -esr smooth;")), 0.99, "ratio");
    let only = strs(&ok(&mut c, "nurbsToPoly -o 0 s1;"));
    assert_eq!(only.len(), 1);
    assert!(only[0].starts_with("nurbsTessellate"));
    ok(&mut c, "nurbsToPoly -e -cch 1 nurbsTessellate1;");
    assert_eq!(num(&ok(&mut c, "nurbsToPoly -q -cch nurbsTessellate1;")), 1.0);
}

#[test]
fn nurbs_plane_and_offset_surface_query_edit() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n np -w 3;");
    close(num(&ok(&mut c, "nurbsPlane -q -w np;")), 3.0, "width");
    ok(&mut c, "nurbsPlane -e -w 6 makeNurbPlane1;");
    close(num(&ok(&mut c, "nurbsPlane -q -w makeNurbPlane1;")), 6.0, "edited");
    ok(&mut c, "nurbsPlane -e -nds 1 np;");
    assert_eq!(num(&ok(&mut c, "nurbsPlane -q -nds np;")), 1.0);
    ok(&mut c, "nurbsPlane -e -nds 0 np;");
    ok(&mut c, "nurbsPlane -ax 0 1 0 -n flat;");
    let off = strs(&ok(&mut c, "offsetSurface -d 2 flat;"));
    close(num(&ok(&mut c, &format!("offsetSurface -q -d {};", off[1]))), 2.0, "distance");
    ok(&mut c, &format!("offsetSurface -e -d 3 {};", off[0]));
    close(num(&ok(&mut c, &format!("offsetSurface -q -d {};", off[1]))), 3.0, "edited through the surface");
    ok(&mut c, &format!("offsetSurface -e -cch 1 {};", off[1]));
    assert_eq!(num(&ok(&mut c, &format!("offsetSurface -q -cch {};", off[1]))), 1.0);
}

#[test]
fn offset_curve_breaks_loops_and_edits() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -p 4 4 0 -n el;");
    let r = strs(&ok(&mut c, "offsetCurve -d 1 -nr 0 0 1 el;"));
    assert_eq!(r.len(), 2);
    // A sharp corner by default: the outside corner at (5, -1).
    let b = nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", r[0])));
    close(b[0], 0.0, "starts at the start");
    close(b[1], -1.0, "below");
    close(b[3], 5.0, "right");
    assert_eq!(num(&ok(&mut c, &format!("offsetCurve -q -cb {};", r[1]))), 2.0);
    assert_eq!(nums(&ok(&mut c, &format!("getAttr {}.spans;", r[0]))), vec![2.0]);
    // An arc round the corner has more spans.
    ok(&mut c, &format!("offsetCurve -e -cb 1 {};", r[1]));
    assert!(num(&ok(&mut c, &format!("getAttr {}.spans;", r[0]))) > 2.0);
    // Off: a straight jump across, a span more than the corner.
    ok(&mut c, &format!("offsetCurve -e -cb 0 {};", r[1]));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {}.spans;", r[0]))), 3.0);
    // Inside a tight bend the offset crosses itself: -cl cuts the loop out.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -p 4 1 0 -p 0 1 0 -n zig;");
    let inner = strs(&ok(&mut c, "offsetCurve -d -0.8 -nr 0 0 1 -cb 0 -cl 0 zig;"));
    let cut = strs(&ok(&mut c, "offsetCurve -d -0.8 -nr 0 0 1 -cb 0 -cl 1 zig;"));
    let (a, b2) = (num(&ok(&mut c, &format!("arclen {};", inner[0]))), num(&ok(&mut c, &format!("arclen {};", cut[0]))));
    assert!(b2 < a, "the loop cut out: {a} {b2}");
    assert_eq!(num(&ok(&mut c, &format!("offsetCurve -q -cl {};", cut[1]))), 1.0);
    // Smooth curves keep the distance to the tolerance.
    ok(&mut c, "circle -nr 0 0 1 -r 2 -n ring;");
    let ro = strs(&ok(&mut c, "offsetCurve -d 1 -tol 0.001 -sd 6 ring;"));
    let p = nums(&ok(&mut c, &format!("pointOnCurve -pr 0.3 {};", ro[0])));
    let rad = (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!((rad - 3.0).abs() < 0.1 || (rad - 1.0).abs() < 0.1, "a ring a unit out or in: {rad}");
    close(num(&ok(&mut c, &format!("offsetCurve -q -tol {};", ro[1]))), 0.001, "tolerance");
    assert_eq!(num(&ok(&mut c, &format!("offsetCurve -q -sd {};", ro[1]))), 6.0);
    // -ugn off: the curve's own plane, not the normal given.
    ok(&mut c, "offsetCurve -ugn 0 -nr 1 0 0 -d 1 -n own el;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox own;"))[3], 5.0, "in the curve's plane");
    // -o off: only the node.
    let only = strs(&ok(&mut c, "offsetCurve -o 0 el;"));
    assert_eq!(only.len(), 1);
    assert!(only[0].starts_with("offsetCurve"));
}

#[test]
fn offset_curve_on_surface_settings() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -ch on -o on -po 0 -ax 0 1 0 -w 10 -lr 2; circle -ch on -o on -nr 0 1 0 -r 3;");
    let pc = strs(&ok(&mut c, "projectCurve -ch 0 -rn false -un false -tol 0.01 nurbsCircle1 nurbsPlane1;"));
    let cos = pc.iter().find(|x| x.contains("->")).cloned().unwrap_or_else(|| pc[0].clone());
    let r = strs(&ok(&mut c, &format!("offsetCurveOnSurface -d 0.05 -tol 0.02 -cp 4 {cos};")));
    assert_eq!(r.len(), 2);
    let node = r[1].clone();
    close(num(&ok(&mut c, &format!("offsetCurveOnSurface -q -tol {node};"))), 0.02, "tolerance");
    assert_eq!(num(&ok(&mut c, &format!("offsetCurveOnSurface -q -cp {node};"))), 4.0);
    ok(&mut c, &format!("offsetCurveOnSurface -e -cb 1 -cl 1 {node};"));
    assert_eq!(num(&ok(&mut c, &format!("offsetCurveOnSurface -q -cb {node};"))), 1.0);
    assert_eq!(num(&ok(&mut c, &format!("offsetCurveOnSurface -q -cl {node};"))), 1.0);
    assert_eq!(num(&ok(&mut c, &format!("offsetCurveOnSurface -q -st {node};"))), 1.0);
}

#[test]
fn point_on_curve_curvature_and_info_node() {
    let mut c = Core::new();
    ok(&mut c, "circle -r 2 -nr 0 0 1 -s 16 -n ring;");
    let r = num(&ok(&mut c, "pointOnCurve -pr 0.3 -cr ring;"));
    assert!((r - 2.0).abs() < 0.1, "radius of curvature {r}");
    let cc = nums(&ok(&mut c, "pointOnCurve -pr 0.3 -cc ring;"));
    assert!(cc[0].abs() < 0.15 && cc[1].abs() < 0.15, "centre of curvature {cc:?}");
    let nn = nums(&ok(&mut c, "pointOnCurve -pr 0.3 -nn ring;"));
    let p = nums(&ok(&mut c, "pointOnCurve -pr 0.3 ring;"));
    let inward = -(p[0] * nn[0] + p[1] * nn[1]);
    assert!(inward > 1.5, "the normal points in: {nn:?}");
    let no = nums(&ok(&mut c, "pointOnCurve -pr 0.3 -no ring;"));
    assert!(no[0] * nn[1] - no[1] * nn[0] < 1e-6, "the normal along the unit normal: {no:?} {nn:?}");
    // A straight line has no bend.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -n ln;");
    assert_eq!(num(&ok(&mut c, "pointOnCurve -pr 0.5 -cr ln;")), 0.0);
    // -top: a fraction of the parameter range.
    close3(&nums(&ok(&mut c, "pointOnCurve -top 1 -pr 0.25 ln;")), [1.0, 0.0, 0.0], "a quarter");
    // -ch on: a pointOnCurveInfo following the curve.
    let info = ok(&mut c, "pointOnCurve -ch on -pr 0.5 ln;");
    let info = info.as_str().unwrap().to_string();
    assert!(info.starts_with("pointOnCurveInfo"));
    close3(&nums(&ok(&mut c, &format!("getAttr {info}.position;"))), [2.0, 0.0, 0.0], "info position");
    ok(&mut c, "move 0 3 0 ln;");
    close3(&nums(&ok(&mut c, &format!("getAttr {info}.position;"))), [2.0, 3.0, 0.0], "follows");
    ok(&mut c, &format!("pointOnCurve -e -pr 1 {info};"));
    close(num(&ok(&mut c, &format!("pointOnCurve -q -pr {info};"))), 1.0, "edited");
    close3(&nums(&ok(&mut c, &format!("pointOnCurve -q -p {info};"))), [4.0, 3.0, 0.0], "query reads the node: the end");
    ok(&mut c, &format!("pointOnCurve -e -nds 2 {info}; move 0 9 0 ln;"));
    close3(&nums(&ok(&mut c, &format!("getAttr {info}.position;"))), [4.0, 3.0, 0.0], "blocked");
}

#[test]
fn point_on_surface_info_node_and_several_answers() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -n np -w 2 -ax 0 1 0;");
    let info = ok(&mut c, "pointOnSurface -ch on -u 0.2 -v 0.3 np;").as_str().unwrap().to_string();
    assert!(info.starts_with("pointOnSurfaceInfo"));
    close(num(&ok(&mut c, &format!("pointOnSurface -q -u {info};"))), 0.2, "u");
    ok(&mut c, &format!("pointOnSurface -e -v 0.9 {info};"));
    close(num(&ok(&mut c, &format!("pointOnSurface -q -v {info};"))), 0.9, "v");
    let both = nums(&ok(&mut c, "pointOnSurface -u 0.5 -v 0.5 -p -nn np;"));
    assert_eq!(both.len(), 6, "position then the unit normal");
    close(both[4].abs(), 1.0, "unit normal up");
    let n = nums(&ok(&mut c, &format!("getAttr {info}.normal;")));
    assert!(n[1].abs() > 1.0, "the unnormalized normal: {n:?}");
}

#[test]
fn point_curve_constraint_locator_and_settings() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 0 0 -p 3 1 0 -n pc;");
    let r = strs(&ok(&mut c, "pointCurveConstraint -ch 1 -pw 1 pc.u[0.5];"));
    assert_eq!(r.len(), 3, "the curve, the node and its locator: {r:?}");
    close3(&nums(&ok(&mut c, &format!("pointCurveConstraint -q -puv {};", r[1]))), [0.5, 0.0, 0.0], "where");
    let before = nums(&ok(&mut c, "pointOnCurve -pr 0.5 pc;"));
    ok(&mut c, &format!("move -r 0 2 0 {};", r[2]));
    let after = nums(&ok(&mut c, "pointOnCurve -pr 0.5 pc;"));
    close(after[1] - before[1], 2.0, "the curve follows the locator");
    ok(&mut c, &format!("pointCurveConstraint -e -pw 0 {};", r[1]));
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0.5 pc;")), [before[0], before[1], before[2]], "no pull");
    ok(&mut c, &format!("pointCurveConstraint -e -nds 1 {};", r[1]));
    assert_eq!(num(&ok(&mut c, &format!("pointCurveConstraint -q -nds {};", r[1]))), 1.0);
    // With -puv in place of the point named.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -n ln;");
    let q = strs(&ok(&mut c, "pointCurveConstraint -puv 0.25 0 0 -p 1 5 0 ln.u[0.9];"));
    close(nums(&ok(&mut c, "pointOnCurve -pr 0.25 ln;"))[1], 5.0, "pulled at u 0.25");
    assert!(q[1].starts_with("pointCurveConstraint"));
}

#[test]
fn object_type_tags() {
    let mut c = Core::new();
    let tag = num(&ok(&mut c, "objectType -tgt transform;"));
    assert_eq!(tag as u32, u32::from_be_bytes(*b"XFRM"));
    assert_eq!(ok(&mut c, &format!("objectType -tpt {};", tag as i64)), json!("transform"));
    ok(&mut c, "polyCube -n cb;");
    assert_eq!(num(&ok(&mut c, "objectType -tt cb;")) as u32, u32::from_be_bytes(*b"XFRM"));
    assert_eq!(num(&ok(&mut c, "objectType -tt cbShape;")) as u32, u32::from_be_bytes(*b"DMSH"));
    assert_eq!(num(&ok(&mut c, "objectType -tgt noSuchType;")), 0.0);
}

#[test]
fn option_menu_pane_layout_and_panel_flags() {
    let mut c = Core::new();
    ok(&mut c, "window w1; columnLayout; optionMenu -mvi 5 -pmo 1 -pmc \"global int $u05Shown; $u05Shown++;\" om; menuItem -l a; menuItem -l b;");
    assert_eq!(num(&ok(&mut c, "optionMenu -q -mvi om;")), 5.0);
    assert_eq!(num(&ok(&mut c, "optionMenu -q -pmo om;")), 1.0);
    ok(&mut c, "optionMenu -e -mvi 8 om;");
    assert_eq!(num(&ok(&mut c, "optionMenu -q -mvi om;")), 8.0);
    ok(&mut c, "setParent ..; paneLayout -cn \"vertical2\" -aft 3 pl;");
    assert_eq!(num(&ok(&mut c, "paneLayout -q -aft pl;")), 3.0);
    ok(&mut c, "paneLayout -e -gsp 4 -mar 2 -mr4 1 2 3 4 -aft 0 pl;");
    assert_eq!(num(&ok(&mut c, "paneLayout -q -aft pl;")), 0.0);
    ok(&mut c, "modelPanel -mrl 1 mp1;");
    assert_eq!(num(&ok(&mut c, "panel -q -mrl mp1;")), 1.0);
    take_ui_requests();
    ok(&mut c, "panel -e -tor -mrl 0 mp1;");
    assert!(take_ui_requests().to_string().contains("panelFocus|mp1"));
    assert_eq!(num(&ok(&mut c, "panel -q -mrl mp1;")), 0.0);
}

#[test]
fn option_menu_post_menu_command_once() {
    let mut c = Core::new();
    ok(&mut c, "global int $u05Count; $u05Count = 0; window w2; columnLayout; optionMenu -pmo 1 -pmc \"global int $u05Count; $u05Count++;\" om2;");
    let path = ok(&mut c, "optionMenu -q -fpn om2;").as_str().unwrap_or("w2|columnLayout1|om2").to_string();
    for _ in 0..3 {
        let cbs = aztek_host::scriptui::shell_event(&json!({"path": path, "fire": ["postMenuCommand"]}));
        for (cb, _) in cbs {
            if let Some(src) = cb.as_str() {
                ok(&mut c, src);
            }
        }
    }
    assert_eq!(num(&ok(&mut c, "global int $u05Count; $u05Count;")), 1.0, "once");
}

#[test]
fn panel_configuration_columns_and_replace() {
    let mut c = Core::new();
    ok(&mut c, "panelConfiguration -l \"Mine\" -di \"one.png\" -ap false \"Outliner\" \"outlinerPanel\" \"mkOut\" \"edOut\" -ap true \"Persp View\" \"modelPanel\" \"mkPersp\" \"edPersp\" cfgA;");
    assert_eq!(strs(&ok(&mut c, "panelConfiguration -q -ls cfgA;")), vec!["Outliner", "Persp View"]);
    assert_eq!(strs(&ok(&mut c, "panelConfiguration -q -ts cfgA;")), vec!["outlinerPanel", "modelPanel"]);
    assert_eq!(strs(&ok(&mut c, "panelConfiguration -q -cs cfgA;")), vec!["mkOut", "mkPersp"]);
    assert_eq!(strs(&ok(&mut c, "panelConfiguration -q -es cfgA;")), vec!["edOut", "edPersp"]);
    assert_eq!(nums(&ok(&mut c, "panelConfiguration -q -isFixedState cfgA;")), vec![0.0, 1.0]);
    ok(&mut c, "panelConfiguration -e -rp 1 true \"Graph\" \"scriptedPanel\" \"mkG\" \"edG\" cfgA;");
    assert_eq!(strs(&ok(&mut c, "panelConfiguration -q -ls cfgA;")), vec!["Graph", "Persp View"]);
    // The default image stays once set; -i is the user's own.
    ok(&mut c, "panelConfiguration -e -di \"two.png\" -i \"mine.png\" cfgA;");
    assert_eq!(ok(&mut c, "panelConfiguration -q -di cfgA;"), json!("one.png"));
    assert_eq!(ok(&mut c, "panelConfiguration -q -i cfgA;"), json!("mine.png"));
}

#[test]
fn constraint_cache_and_layer() {
    let mut c = Core::new();
    ok(&mut c, "spaceLocator -n a; spaceLocator -n b; spaceLocator -n d; rotate 0 90 0 b;");
    ok(&mut c, "setKeyframe -t 1 -at ry -v 0 a; setKeyframe -t 10 -at ry -v 180 a;");
    let oc = strs(&ok(&mut c, "orientConstraint a b d;"))[0].clone();
    ok(&mut c, &format!("orientConstraint -e -cc 1 10 {oc};"));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {oc}.interpType;"))), 4.0);
    assert!(!strs(&ok(&mut c, &format!("listConnections -s 1 -d 0 {oc}.interpCacheY;"))).is_empty());
    // The cache answers: a target moved after caching changes nothing.
    ok(&mut c, "currentTime 5;");
    let cached = nums(&ok(&mut c, "getAttr d.rotate;"));
    ok(&mut c, "rotate 0 30 0 b;");
    close3(&nums(&ok(&mut c, "getAttr d.rotate;")), [cached[0], cached[1], cached[2]], "from the cache");
    ok(&mut c, &format!("orientConstraint -e -dc {oc};"));
    assert!(strs(&ok(&mut c, &format!("listConnections -s 1 -d 0 {oc}.interpCacheY;"))).is_empty());
    assert!((nums(&ok(&mut c, "getAttr d.rotate;"))[1] - cached[1]).abs() > 1e-3, "live again");
    // Parent constraints too; -dr is taken.
    ok(&mut c, "spaceLocator -n e; spaceLocator -n f;");
    let pc = strs(&ok(&mut c, "parentConstraint -mo -dr a e f;"))[0].clone();
    ok(&mut c, &format!("parentConstraint -e -cc 1 3 {pc};"));
    assert_eq!(num(&ok(&mut c, &format!("getAttr {pc}.interpType;"))), 4.0);
    // -layer on the others.
    ok(&mut c, "animLayer L1; spaceLocator -n g;");
    ok(&mut c, "pointConstraint -l L1 a g;");
    ok(&mut c, "spaceLocator -n h; polyCube -n pv; ");
}

#[test]
fn percent_dropoffs() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n pc2; cluster -n cl pc2;");
    ok(&mut c, "percent -v 0 cl pc2.vtx[*];");
    ok(&mut c, "percent -v 1 -dp 0.5 0 0 -dds 1 cl pc2.vtx[*];");
    let w = nums(&ok(&mut c, "percent -q -v cl pc2.vtx[*];"));
    let near = (1.0 - 0.5f64.sqrt()).max(0.0);
    close(w[1], near, "+X points sqrt(0.5) away");
    close(w[0], 0.0, "-X points out of reach");
    ok(&mut c, "percent -v 0.5 -mp cl pc2.vtx[1];");
    close(nums(&ok(&mut c, "percent -q -v cl pc2.vtx[1];"))[0], near * 0.5, "multiplied");
    ok(&mut c, "percent -v 0.25 -ap cl pc2.vtx[0];");
    close(nums(&ok(&mut c, "percent -q -v cl pc2.vtx[0];"))[0], 0.25, "added");
    // Along an axis from a point, sine dropoff; and round a curve.
    ok(&mut c, "percent -v 1 -dp 0 -0.5 0 -dax 0 1 0 -dds 1 -dt sine cl pc2.vtx[*];");
    let w = nums(&ok(&mut c, "percent -q -v cl pc2.vtx[*];"));
    close(w[0], 1.0, "on the plane");
    close(w[2], 0.0, "a unit along the axis");
    ok(&mut c, "curve -d 1 -p -0.5 -0.5 0.5 -p 0.5 -0.5 0.5 -n cv;");
    ok(&mut c, "percent -v 0 cl pc2.vtx[*]; percent -v 1 -dc cv -dds 0.5 cl pc2.vtx[*];");
    let w = nums(&ok(&mut c, "percent -q -v cl pc2.vtx[*];"));
    close(w[0], 1.0, "on the curve");
    close(w[6], 0.0, "far from it");
    assert!(c.mel_checked("percent -v 1 -dp 0 0 0 -dt wobbly cl pc2.vtx[0];").is_err());
}

#[test]
fn pick_walk_recurse_and_play_sound() {
    let mut c = Core::new();
    ok(&mut c, "group -em -n top; group -em -n mid -p top; group -em -n low -p mid;");
    ok(&mut c, "select top;");
    assert_eq!(strs(&ok(&mut c, "pickWalk -d down -r;")), vec!["low"]);
    assert_eq!(strs(&ok(&mut c, "pickWalk -d up -r;")), vec!["top"]);
    assert_eq!(strs(&ok(&mut c, "pickWalk -d down;")), vec!["mid"]);
    ok(&mut c, "play -ps 0;");
    assert_eq!(ok(&mut c, "play -q -ps;"), json!(false));
    ok(&mut c, "play -playSound 1;");
    assert_eq!(ok(&mut c, "play -q -ps;"), json!(true));
    assert_eq!(ok(&mut c, "play -q -st;"), json!(false), "setting the sound doesn't start playback");
}

#[test]
fn partition_sets() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a; polyCube -n b; polyCube -n d;");
    ok(&mut c, "sets -n s1 a; sets -n s2 b; sets -n s3 d; sets -n s4 a;");
    assert_eq!(ok(&mut c, "partition -n p1 s1 s2;"), json!("p1"));
    assert_eq!(strs(&ok(&mut c, "partition -q p1;")), vec!["s1", "s2"]);
    ok(&mut c, "partition -add p1 s3;");
    assert_eq!(strs(&ok(&mut c, "partition -q p1;")).len(), 3);
    assert!(c.mel_checked("partition -add p1 s4;").is_err(), "s4 shares a with s1");
    ok(&mut c, "partition -rm p1 s1;");
    ok(&mut c, "partition -add p1 s4;");
    assert_eq!(strs(&ok(&mut c, "partition -q p1;")), vec!["s2", "s3", "s4"]);
    assert_eq!(ok(&mut c, "partition -q -re p1;"), json!(false));
    let r = ok(&mut c, "partition -render;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("partition -q -re {r};")), json!(true));
    assert!(c.mel_checked(&format!("partition -add {r} s2;")).is_err(), "not a render set");
    ok(&mut c, "shadingNode -asShader lambert -n m1; sets -renderable true -noSurfaceShader true -empty -n m1SG;");
    ok(&mut c, &format!("partition -add {r} m1SG;"));
    assert_eq!(strs(&ok(&mut c, &format!("partition -q {r};"))), vec!["m1SG"]);
    // A selected set goes in.
    ok(&mut c, "select -ne s3;");
    let p3 = ok(&mut c, "partition;").as_str().unwrap().to_string();
    assert_eq!(strs(&ok(&mut c, &format!("partition -q {p3};"))), vec!["s3"]);
    ok(&mut c, "undo;");
    assert!(c.mel_checked(&format!("partition -q {p3};")).is_err(), "undone");
}

#[test]
fn param_locator_rides_the_shape() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -n ln;");
    let l = ok(&mut c, "paramLocator ln.u[0.5];").as_str().unwrap().to_string();
    close3(&nums(&ok(&mut c, &format!("xform -q -ws -t {l};"))), [2.0, 0.0, 0.0], "at u 0.5");
    ok(&mut c, "move -r 0 0 3 ln.cv[1];");
    close3(&nums(&ok(&mut c, &format!("xform -q -ws -t {l};"))), [2.0, 0.0, 1.5], "follows the curve");
    ok(&mut c, "move 5 0 0 ln;");
    close3(&nums(&ok(&mut c, &format!("xform -q -ws -t {l};"))), [7.0, 0.0, 1.5], "and its transform");
    let e = ok(&mut c, "paramLocator ln.ep[1];").as_str().unwrap().to_string();
    close3(&nums(&ok(&mut c, &format!("xform -q -ws -t {e};"))), [9.0, 0.0, 3.0], "the second edit point");
    let n = ok(&mut c, "paramLocator ln.un[0.25];").as_str().unwrap().to_string();
    close(nums(&ok(&mut c, &format!("xform -q -ws -t {n};")))[0], 6.0, "normalized");
    ok(&mut c, "nurbsPlane -n np -ax 0 1 0 -w 2;");
    let s = ok(&mut c, "paramLocator -p np.uv[1][1];").as_str().unwrap().to_string();
    let p = nums(&ok(&mut c, &format!("xform -q -ws -t {s};")));
    assert!((p[0].abs() - 1.0).abs() < 1e-3 && (p[2].abs() - 1.0).abs() < 1e-3, "a corner: {p:?}");
    assert!(c.mel_checked("paramLocator np.u[0.5];").is_err());
}

#[test]
fn override_modifier_and_display_filters() {
    let mut c = Core::new();
    take_ui_requests();
    ok(&mut c, "overrideModifier -p Alt -p Shift;");
    assert_eq!(aztek_host::host_general::forced_modifiers(0), 5);
    ok(&mut c, "overrideModifier -r Shift;");
    assert_eq!(aztek_host::host_general::forced_modifiers(1), 4, "shift held up");
    assert!(take_ui_requests().to_string().contains("overrideModifier"));
    ok(&mut c, "overrideModifier -cl;");
    assert_eq!(aztek_host::host_general::forced_modifiers(2), 2);
    assert!(c.mel_checked("overrideModifier -p Meta;").is_err());

    ok(&mut c, "pluginDisplayFilter -register -label \"My Display Filter\" -classification \"drawdb/geometry/myShape\" myDisplayFilter;");
    assert_eq!(ok(&mut c, "pluginDisplayFilter -q -label myDisplayFilter;"), json!("My Display Filter"));
    assert_eq!(ok(&mut c, "pluginDisplayFilter -q -cls myDisplayFilter;"), json!("drawdb/geometry/myShape"));
    assert_eq!(strs(&ok(&mut c, "pluginDisplayFilter -q -listFilters;")), vec!["myDisplayFilter"]);
    // Counted: registered twice, it takes two deregisters.
    ok(&mut c, "pluginDisplayFilter plainFilter; pluginDisplayFilter plainFilter;");
    assert_eq!(ok(&mut c, "pluginDisplayFilter -q -l plainFilter;"), json!("plainFilter"));
    ok(&mut c, "pluginDisplayFilter -dr plainFilter;");
    assert_eq!(ok(&mut c, "pluginDisplayFilter -ex plainFilter;"), json!(true));
    ok(&mut c, "pluginDisplayFilter -dr plainFilter;");
    assert_eq!(ok(&mut c, "pluginDisplayFilter -ex plainFilter;"), json!(false));
    assert!(c.mel_checked("pluginDisplayFilter -dr plainFilter;").is_err());
    ok(&mut c, "pluginDisplayFilter -dr myDisplayFilter;");
}

#[test]
fn poly_append_average_and_auto_projection() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -n pl -sx 1 -sy 1;");
    let r = strs(&ok(&mut c, "polyAppendVertex -a 1 -a 3 -p 1 0 2 pl;"));
    assert!(r.iter().any(|n| n.starts_with("polyAppendVertex")), "{r:?}");
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f pl;")), 2.0);
    ok(&mut c, "polyAppendVertex -e -nds 2 polyAppendVertex1;");
    assert_eq!(num(&ok(&mut c, "polyAppendVertex -q -nds polyAppendVertex1;")), 2.0);
    ok(&mut c, "polyAppendVertex -e -cch 1 pl;");
    assert_eq!(num(&ok(&mut c, "polyAppendVertex -q -cch pl;")), 1.0);

    ok(&mut c, "polyPlane -n p2 -sx 1 -sy 1;");
    ok(&mut c, "polyAppend -a 1 -p 1 0 1 p2;");
    assert_eq!(num(&ok(&mut c, "polyAppend -q -nds polyAppend1;")), 0.0);
    ok(&mut c, "polyAppend -e -nds 2 polyAppend1;");
    assert_eq!(num(&ok(&mut c, "polyAppend -q -nds p2;")), 2.0);

    ok(&mut c, "polyCube -n cb;");
    ok(&mut c, "polyAverageVertex -i 2 cb.vtx[0:3];");
    assert_eq!(num(&ok(&mut c, "polyAverageVertex -q -i polyAverageVertex1;")), 2.0);
    ok(&mut c, "polyAverageVertex -e -i 5 cb;");
    assert_eq!(num(&ok(&mut c, "polyAverageVertex -q -i cb;")), 5.0);

    ok(&mut c, "polySphere -n sp;");
    let made = strs(&ok(&mut c, "polyAutoProjection -pb 1 -ro 0 45 0 -l 2 sp.f[*];"));
    assert!(made[0].starts_with("polyAutoProj"));
    close3(&nums(&ok(&mut c, "polyAutoProjection -q -ro polyAutoProj1;")), [0.0, 45.0, 0.0], "rotate in degrees");
    assert_eq!(num(&ok(&mut c, "polyAutoProjection -q -pb polyAutoProj1;")), 1.0);
    let before = nums(&ok(&mut c, "polyEditUV -q sp.map[5];"));
    ok(&mut c, "polyAutoProjection -e -p 12 -l 1 polyAutoProj1;");
    let after = nums(&ok(&mut c, "polyEditUV -q sp.map[5];"));
    assert!((before[0] - after[0]).abs() + (before[1] - after[1]).abs() > 1e-4, "projected again: {before:?} {after:?}");
    assert_eq!(num(&ok(&mut c, "polyAutoProjection -q -p polyAutoProj1;")), 12.0);
    // Every UV in the unit square with scale mode 1.
    let all = nums(&ok(&mut c, "polyEditUV -q sp.map[*];"));
    assert!(all.iter().all(|x| *x >= -1e-4 && *x <= 1.0 + 1e-4));
    // Translate, scale and the X/Y/Z parts.
    ok(&mut c, "polyCube -n c2; polyAutoProjection -sx 2 -tx 1 -l 0 -sc 0 c2.f[*];");
    close3(&nums(&ok(&mut c, "polyAutoProjection -q -s polyAutoProj2;")), [2.0, 1.0, 1.0], "scale");
    close3(&nums(&ok(&mut c, "polyAutoProjection -q -t polyAutoProj2;")), [1.0, 0.0, 0.0], "translate");
}

#[test]
fn poly_axis_frame() {
    let mut c = Core::new();
    let r = strs(&ok(&mut c, "polyAxis -sa 16 -arw 1 -sca 2.0 -lx 1.5;"));
    assert_eq!(r, vec!["pAxis1", "polyAxis1"]);
    close(num(&ok(&mut c, "polyAxis -q -sca pAxis1;")), 2.0, "scale");
    assert_eq!(num(&ok(&mut c, "polyAxis -q -sa polyAxis1;")), 16.0);
    let b = nums(&ok(&mut c, "exactWorldBoundingBox pAxis1;"));
    close(b[3], 3.0, "X arm 1.5 long at scale 2");
    close(b[4], 2.0, "Y arm");
    close(b[5], 2.0, "Z arm");
    let faces = num(&ok(&mut c, "polyEvaluate -f pAxis1;"));
    ok(&mut c, "polyAxis -e -arw 2 pAxis1;");
    assert_ne!(num(&ok(&mut c, "polyEvaluate -f pAxis1;")), faces, "cones in place of domes");
    ok(&mut c, "polyAxis -e -rhd 0 pAxis1;");
    close(nums(&ok(&mut c, "exactWorldBoundingBox pAxis1;"))[2], -2.0, "left-handed: Z the other way");
    // The wedge frame and a name.
    let w = strs(&ok(&mut c, "polyAxis -ftp 1 -sca 0.5 -n myFrame;"));
    assert_eq!(w[0], "myFrame");
    assert_eq!(num(&ok(&mut c, "polyAxis -q -ftp myFrame;")), 1.0);
    assert_eq!(num(&ok(&mut c, "polyEvaluate -f myFrame;")), 18.0);
    // A material per arm.
    ok(&mut c, "polyAxis -n myColoredFrame -mt \"polyAxis_materialTemplate\";");
    let sgs = strs(&ok(&mut c, "listConnections -type shadingEngine myColoredFrameShape;"));
    for axis in ["X", "Y", "Z"] {
        assert!(sgs.iter().any(|s| s == &format!("polyAxis_materialTemplate{axis}SG")), "{sgs:?}");
    }
    ok(&mut c, "polyAxis -e -nds 2 myColoredFrame;");
    assert_eq!(num(&ok(&mut c, "polyAxis -q -nds myColoredFrame;")), 2.0);
}

#[test]
fn playblast_flags_without_a_view() {
    let mut c = Core::new();
    // No view to capture: a warning, or with -toe an error.
    let dir = std::env::temp_dir().join(format!("u05_pb_{}", std::process::id()));
    let base = dir.join("blast").display().to_string().replace('\\', "/");
    ok(&mut c, &format!("playblast -fmt image -st 1 -et 2 -exp 1 -g 2.2 -f \"{base}\";"));
    assert!(c.mel_checked(&format!("playblast -toe -fmt image -st 1 -et 2 -f \"{base}\";")).is_err());
    // -cameraSetup: each camera's frames named for it.
    let r = ok(&mut c, &format!("playblast -fmt image -st 1 -et 1 -cs \"select persp\" \"Left\" -cs \"select top\" \"Right\" -f \"{base}\";"));
    assert!(r.as_str().unwrap().contains("blastLeft"), "{r}");
    assert!(c.mel_checked("playblast -cs onlyOne;").is_err());
    // -options opens the options, no playblast.
    assert_eq!(ok(&mut c, "playblast -options;"), Value::Null);
    assert_eq!(ok(&mut c, "playblast -co;"), Value::Null);
    // -replaceAudioOnly needs a movie and a sound.
    assert!(c.mel_checked("playblast -rao -rf \"/no/such/movie.mov\";").is_err());
    for flag in ["csd", "uts", "osv", "rst", "ret", "rf"] {
        assert!(ok(&mut c, &format!("help playblast;")).to_string().len() > 0, "{flag}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// Nothing of this unit is missing from the parity ledger's view of the commands.
#[test]
fn every_flag_of_the_unit_is_there() {
    let c = Core::new();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../parity");
    let changed = aztek_host::mel::parity::check(&c.s, &dir).expect("parity");
    let Some((_, text)) = changed.iter().find(|(p, _)| p.ends_with("command_flags.tsv")) else { return };
    let ours = [
        "nurbsPlane", "nurbsSquare", "nurbsToPoly", "objectType", "offsetCurve", "offsetCurveOnSurface", "offsetSurface", "optionMenu", "orientConstraint",
        "overrideModifier", "paneLayout", "panel", "panelConfiguration", "paramLocator", "parentConstraint", "partition", "percent", "pickWalk", "planarSrf",
        "play", "playblast", "pluginDisplayFilter", "pointConstraint", "pointCurveConstraint", "pointOnCurve", "pointOnPolyConstraint", "pointOnSurface",
        "poleVectorConstraint", "polyAppend", "polyAppendVertex", "polyAutoProjection", "polyAverageVertex", "polyAxis",
    ];
    let missing: Vec<&str> = text.lines().filter(|l| {
        let cols: Vec<&str> = l.split('\t').collect();
        cols.len() > 2 && ours.contains(&cols[0]) && cols[2] == "missing"
    }).collect();
    assert!(missing.is_empty(), "still missing:\n{}", missing.join("\n"));
}
