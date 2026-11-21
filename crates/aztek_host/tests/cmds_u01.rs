//! Commands of the reference from addAttr to circle: align, alignCurve and alignSurface, arcLengthDimension,
//! arubaNurbsToPoly, autoPlace, blindDataType, and the flags the others gained (query and edit on the NURBS commands'
//! history nodes, blendShape's target tools, boneLattice's deformer flags, constraints on animation layers).

use aztek_host::Core;
use serde_json::{json, Value};

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) if a.len() == 1 => num(&a[0]),
        x => x.as_f64().unwrap_or_else(|| panic!("not a number: {x}")),
    }
}

fn nums(v: &Value) -> Vec<f64> {
    fn walk(v: &Value, out: &mut Vec<f64>) {
        match v {
            Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            x => out.extend(x.as_f64()),
        }
    }
    let mut out = Vec::new();
    walk(v, &mut out);
    out
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
fn add_attr_categories_world_space_and_unique_names() {
    let mut c = Core::new();
    ok(&mut c, "createNode transform -n n1;");
    ok(&mut c, "addAttr -ln foo -at double -ct \"myPlugin\" -ct \"v2\" n1;");
    assert_eq!(ok(&mut c, "addAttr -q -ct n1.foo;"), json!(["myPlugin", "v2"]));
    ok(&mut c, "addAttr -e -ct \"extra\" n1.foo;");
    assert_eq!(ok(&mut c, "addAttr -q -ct n1.foo;"), json!(["myPlugin", "v2", "extra"]));
    assert_eq!(ok(&mut c, "addAttr -q -eun n1.foo;"), json!(true));
    assert_eq!(ok(&mut c, "addAttr -q -ws n1.foo;"), json!(false));
    // World space only on multis.
    assert!(c.mel_checked("addAttr -ln bar -at double -ws true n1;").is_err());
    ok(&mut c, "addAttr -ln bars -at double -m -ws true n1;");
    assert_eq!(ok(&mut c, "addAttr -q -ws n1.bars;"), json!(true));
    // A compound's child may not share a name with another attribute unless -eun is off.
    ok(&mut c, "addAttr -ln grp -at compound -nc 1 n1;");
    assert!(c.mel_checked("addAttr -ln foo -at double -p grp n1;").is_err());
    ok(&mut c, "addAttr -ln grp2 -at compound -nc 1 n1;");
    ok(&mut c, "addAttr -ln foo -at double -p grp2 -eun false n1;");
    assert_eq!(ok(&mut c, "addAttr -q -eun n1.grp2.foo;"), json!(false));
}

#[test]
fn constraint_goes_into_an_animation_layer() {
    let mut c = Core::new();
    ok(&mut c, "spaceLocator -n target; move 0 0 10 target; spaceLocator -n obj;");
    ok(&mut c, "animLayer layer1;");
    let made = ok(&mut c, "aimConstraint -l layer1 target obj;");
    let con = made.as_array().unwrap()[0].as_str().unwrap().to_string();
    // The constraint feeds a layer blend, not the rotate directly.
    let dest = ok(&mut c, &format!("listConnections -s 0 -d 1 {con}.constraintRotateY;"));
    let names: Vec<String> = dest.as_array().unwrap().iter().filter_map(|v| v.as_str().map(str::to_string)).collect();
    assert!(names.iter().any(|n| n.contains("layer1")), "{names:?}");
    // At full weight the object aims at the target: +Z means -90 about Y for an X aim vector.
    close(num(&ok(&mut c, "getAttr obj.ry;")), -90.0, "aimed through the layer");
    ok(&mut c, "animLayer -e -w 0 layer1;");
    close(num(&ok(&mut c, "getAttr obj.ry;")), 0.0, "layer off");
}

#[test]
fn align_modes_and_axes() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n a -w 2 -h 2 -d 2; polyCube -n b -w 2 -h 2 -d 2; polyCube -n c3 -w 2 -h 2 -d 2;");
    ok(&mut c, "move 0 0 0 a; move 3 0 0 b; move 10 0 0 c3;");
    ok(&mut c, "align -x dist a b c3;");
    close(num(&ok(&mut c, "getAttr b.tx;")), 5.0, "dist");
    ok(&mut c, "align -y stack a b c3;");
    close(num(&ok(&mut c, "getAttr b.ty;")), 2.0, "stacked");
    close(num(&ok(&mut c, "getAttr c3.ty;")), 4.0, "stacked twice");
    ok(&mut c, "align -y min -atl a b c3;");
    close(num(&ok(&mut c, "getAttr a.ty;")), 4.0, "to the lead");
    // In another object's axes: a turned frame moves along its own X.
    ok(&mut c, "spaceLocator -n frame; rotate 0 90 0 frame; move 0 0 0 a b; select -cl;");
    ok(&mut c, "move -a 0 0 0 a; move -a 0 0 6 b;");
    ok(&mut c, "align -x max -cs frame a b;");
    close(num(&ok(&mut c, "getAttr a.tz;")), num(&ok(&mut c, "getAttr b.tz;")), "aligned along the frame's X");
}

#[test]
fn align_curve_positional_tangent_and_curvature() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 1 0 -p 3 0 0 -n c1;");
    ok(&mut c, "curve -d 3 -p 5 0 0 -p 6 -1 0 -p 7 -1 0 -p 8 0 0 -n c2;");
    let r = ok(&mut c, "alignCurve c1 c2;");
    let r: Vec<String> = r.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert!(r[0].ends_with("c1") && r[1].starts_with("alignCurve"), "{r:?}");
    // Move the first (the default): its end is at the second's start.
    let end = nums(&ok(&mut c, "pointOnCurve -pr 1 c1;"));
    close3(&end, [5.0, 0.0, 0.0], "ends meet");
    // Tangent: the first's end tangent follows the second's start tangent.
    let t1 = nums(&ok(&mut c, "pointOnCurve -pr 1 -nt c1;"));
    let t2 = nums(&ok(&mut c, "pointOnCurve -pr 0 -nt c2;"));
    close3(&t1, [t2[0], t2[1], t2[2]], "tangents line up");
    // The node answers and takes edits; the curve follows.
    assert_eq!(ok(&mut c, "alignCurve -q -pct alignCurve1;"), json!(1));
    ok(&mut c, "alignCurve -e -pct 2 alignCurve1;");
    let start2 = nums(&ok(&mut c, "pointOnCurve -pr 0 c2;"));
    close3(&start2, [3.0, 0.0, 0.0], "move the second instead");
    // nodeState 1 passes the curves through.
    ok(&mut c, "alignCurve -e -nds 1 alignCurve1;");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 0 c2;")), [5.0, 0.0, 0.0], "has no effect");
    assert_eq!(ok(&mut c, "alignCurve -q -nds c1;"), json!(1));

    // Curvature continuity, the second changed, without history.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 1 0 -p 2 2 0 -p 3 2 0 -n c3;");
    ok(&mut c, "curve -d 3 -p 3 2 0 -p 4 2 0 -p 5 1 0 -p 6 0 0 -n c4;");
    ok(&mut c, "alignCurve -ch 0 -cc 1 -tct 2 c3 c4;");
    let k = |c: &mut Core, curve: &str, u: f64| {
        let h = 1e-3;
        let p = |c: &mut Core, x: f64| nums(&ok(c, &format!("pointOnCurve -pr {x} {curve};")));
        let (a, b, d) = (p(c, u), p(c, u + h), p(c, u + 2.0 * h));
        let (a, b, d) = ([a[0], a[1]], [b[0], b[1]], [d[0], d[1]]);
        let d1 = [(b[0] - a[0]) / h, (b[1] - a[1]) / h];
        let d2 = [(d[0] - 2.0 * b[0] + a[0]) / (h * h), (d[1] - 2.0 * b[1] + a[1]) / (h * h)];
        (d1[0] * d2[1] - d1[1] * d2[0]) / (d1[0] * d1[0] + d1[1] * d1[1]).powf(1.5)
    };
    let k3 = {
        let h = 1e-3;
        let p = |c: &mut Core, x: f64| nums(&ok(c, &format!("pointOnCurve -pr {x} c3;")));
        let (a, b, d) = (p(&mut c, 1.0), p(&mut c, 1.0 - h), p(&mut c, 1.0 - 2.0 * h));
        let d1 = [(a[0] - b[0]) / h, (a[1] - b[1]) / h];
        let d2 = [(a[0] - 2.0 * b[0] + d[0]) / (h * h), (a[1] - 2.0 * b[1] + d[1]) / (h * h)];
        (d1[0] * d2[1] - d1[1] * d2[0]) / (d1[0] * d1[0] + d1[1] * d1[1]).powf(1.5)
    };
    let k4 = k(&mut c, "c4", 0.0);
    assert!((k3 - k4).abs() < 0.05 * k3.abs().max(0.1), "curvature matches: {k3} vs {k4}");
    assert_eq!(ok(&mut c, "ls -type alignCurve;").as_array().unwrap().len(), 1, "no node without history");

    // Attach joins them into the first, keeping the multiple knot.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 0 0 -p 2 0 0 -p 3 0 0 -n c5;");
    ok(&mut c, "curve -d 3 -p 3 1 0 -p 4 1 0 -p 5 1 0 -p 6 1 0 -n c6;");
    ok(&mut c, "alignCurve -at 1 -tc 0 c5 c6;");
    // Seven CVs: the two curves' four each, sharing the joint, which keeps its triple knot.
    close(num(&ok(&mut c, "getAttr c5.spans;")), 4.0, "joined");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 1 c5;")), [3.0, 1.0, 0.0], "the joint is a point of the curve");
    close3(&nums(&ok(&mut c, "pointOnCurve -pr 2 c5;")), [6.0, 1.0, 0.0], "the second's end");
}

#[test]
fn align_surface_along_u_and_v() {
    let mut c = Core::new();
    ok(&mut c, "nurbsPlane -ax 0 1 0 -w 2 -u 2 -v 2 -n p1; nurbsPlane -ax 0 1 0 -w 2 -u 2 -v 2 -n p2; move 5 1 0 p2;");
    let r = ok(&mut c, "alignSurface -tc 0 p1 p2;");
    assert!(r.as_array().unwrap()[1].as_str().unwrap().starts_with("alignSurface"));
    // p1 moved so its U end meets p2's U start (U runs 0 to 2 over two patches).
    let a = nums(&ok(&mut c, "pointOnSurface -u 2 -v 0.5 p1;"));
    let b = nums(&ok(&mut c, "pointOnSurface -u 0 -v 0.5 p2;"));
    close3(&a, [b[0], b[1], b[2]], "edges meet");
    assert_eq!(ok(&mut c, "alignSurface -q -du alignSurface1;"), json!(true));
    ok(&mut c, "alignSurface -e -du 0 -pct 4 alignSurface1;");
    let a = nums(&ok(&mut c, "pointOnSurface -u 0.5 -v 2 p1;"));
    let b = nums(&ok(&mut c, "pointOnSurface -u 0.5 -v 0 p2;"));
    close3(&a, [b[0], b[1], b[2]], "along V, the first's edge changed");
}

#[test]
fn arc_length_dimension_follows_the_curve() {
    let mut c = Core::new();
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 4 0 0 -n crv;");
    let dim = ok(&mut c, "arcLengthDimension crvShape.u[0.5];");
    let dim = dim.as_str().unwrap().to_string();
    close(num(&ok(&mut c, &format!("getAttr {dim}.arcLength;"))), 2.0, "half way");
    ok(&mut c, "scale 2 1 1 crv;");
    close(num(&ok(&mut c, &format!("getAttr {dim}.arcLength;"))), 4.0, "live");
    ok(&mut c, &format!("setAttr {dim}.uParamValue 1;"));
    close(num(&ok(&mut c, &format!("getAttr {dim}.arcLength;"))), 8.0, "to the end");
    ok(&mut c, "sphere -n ball;");
    let d2 = ok(&mut c, "arcLengthDimension ball.uv[0.5][0.5];").as_str().unwrap().to_string();
    assert!(num(&ok(&mut c, &format!("getAttr {d2}.arcLengthInV;"))) > 0.0);
}

#[test]
fn aruba_tessellates_with_history() {
    let mut c = Core::new();
    ok(&mut c, "sphere -n ball; move 3 0 0 ball;");
    let r = ok(&mut c, "arubaNurbsToPoly ball;");
    let r: Vec<String> = r.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert!(r[1].starts_with("arubaTessellate"), "{r:?}");
    let faces = num(&ok(&mut c, &format!("polyEvaluate -f {};", r[0])));
    assert!(faces > 10.0);
    // Local space: the polygons sit where the surface does.
    let bb = nums(&ok(&mut c, &format!("exactWorldBoundingBox {};", r[0])));
    close((bb[0] + bb[3]) * 0.5, 3.0, "where the surface is");
    // World space, without history.
    let w = ok(&mut c, "arubaNurbsToPoly -ws -ch 0 ball;");
    assert_eq!(w.as_array().unwrap().len(), 1);
    ok(&mut c, &format!("arubaNurbsToPoly -e -nds 2 {};", r[1]));
    assert_eq!(ok(&mut c, &format!("arubaNurbsToPoly -q -nds {};", r[1])), json!(2));
}

#[test]
fn auto_place_on_the_ground_and_the_live_surface() {
    let mut c = Core::new();
    let p = nums(&ok(&mut c, "autoPlace;"));
    // No live surface: the ground plane, along the persp camera's axis.
    close(p[1], 0.0, "on the ground");
    ok(&mut c, "polyPlane -w 100 -h 100 -n floor; move 0 2 0 floor; makeLive floor;");
    let p = nums(&ok(&mut c, "autoPlace -um;"));
    close(p[1], 2.0, "on the live surface");
}

#[test]
fn blind_data_types() {
    let mut c = Core::new();
    let n = ok(&mut c, "blindDataType -id 9005 -dt \"float\" -longDataName \"roughness\" -shortDataName \"ruf\" -dt \"float\" -longDataName \"smoothness\" -shortDataName \"smo\" -dt \"int\" -longDataName \"count\" -shortDataName \"cnt\";");
    let n = n.as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("getAttr {n}.typeId;")), json!(9005));
    assert_eq!(ok(&mut c, "blindDataType -q -id 9005 -tn;"), json!(["float", "float", "int"]));
    assert_eq!(ok(&mut c, "blindDataType -q -id 9005 -tn -ldn \"smoothness\";"), json!(["float"]));
    assert_eq!(ok(&mut c, "blindDataType -q -id 9005 -sn;"), json!(["ruf", "smo", "cnt"]));
    assert_eq!(ok(&mut c, "blindDataType -q -id 9001;"), json!(""));
    assert!(c.mel_checked("blindDataType -id 9005 -dt \"int\" -ldn \"x\" -sdn \"x\";").is_err(), "a type can't be made twice");
    ok(&mut c, &format!("setAttr {n}.roughness 0.5;"));
}

#[test]
fn nurbs_commands_query_and_edit_their_nodes() {
    let mut c = Core::new();
    // circle: every component flag, -fc, -ut, and -q/-e on the node.
    ok(&mut c, "circle -cx 1 -cy 2 -cz 3 -r 2 -n ring;");
    close3(&nums(&ok(&mut c, "circle -q -c ring;")), [1.0, 2.0, 3.0], "centre");
    close(num(&ok(&mut c, "circle -q -r makeNurbCircle1;")), 2.0, "radius");
    ok(&mut c, "circle -e -r 3 ring;");
    let radius = |c: &mut Core| {
        let p = nums(&ok(c, "pointOnCurve -pr 0 ring;"));
        ((p[0] - 1.0).powi(2) + (p[1] - 2.0).powi(2) + (p[2] - 3.0).powi(2)).sqrt()
    };
    close(radius(&mut c), 3.0, "edited radius");
    ok(&mut c, "circle -fc 0 -fp 5 0 0 -r 1 -n started;");
    // Through the first point, its centre a radius back toward the centre given.
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox started;"));
    close((bb[0] + bb[3]) * 0.5, 4.0, "centre moved to start at the first point");
    close((bb[1] + bb[4]) * 0.5, 0.0, "centre in Y");
    ok(&mut c, "circle -ut 1 -tol 0.0001 -n fine;");
    assert!(num(&ok(&mut c, "getAttr fine.spans;")) > 8.0, "more sections for the tolerance");
    ok(&mut c, "circle -e -nds 2 ring; circle -e -r 10 ring;");
    close(radius(&mut c), 3.0, "blocking keeps the curve");
    assert_eq!(ok(&mut c, "circle -q -nds ring;"), json!(2));

    // attachCurve: -q and -e on its node, -kmk.
    ok(&mut c, "curve -d 3 -p 0 0 0 -p 1 0 0 -p 2 0 0 -p 3 0 0 -n a1; curve -d 3 -p 3 0 0 -p 4 1 0 -p 5 1 0 -p 6 0 0 -n a2;");
    ok(&mut c, "attachCurve -cch 1 a1 a2;");
    assert_eq!(ok(&mut c, "attachCurve -q -kmk attachCurve1;"), json!(true));
    assert_eq!(ok(&mut c, "attachCurve -q -cch attachCurve1;"), json!(true));
    ok(&mut c, "attachCurve -e -rv2 1 attachCurve1;");
    assert_eq!(ok(&mut c, "attachCurve -q -rv2 a1;"), json!(true));

    // attachSurface: along V, swapped, twisted.
    ok(&mut c, "nurbsPlane -ax 0 1 0 -n s1; nurbsPlane -ax 0 1 0 -n s2; move 0 0 1 s2;");
    ok(&mut c, "attachSurface -du 0 s1 s2;");
    assert_eq!(ok(&mut c, "attachSurface -q -du attachSurface1;"), json!(false));
    let bb = nums(&ok(&mut c, "exactWorldBoundingBox s1;"));
    close(bb[5] - bb[2], 2.0, "joined along V");
    ok(&mut c, "attachSurface -e -tw 1 -sw1 0 attachSurface1;");
    assert_eq!(ok(&mut c, "attachSurface -q -tw attachSurface1;"), json!(true));

    // bevel: one side, separate pieces.
    ok(&mut c, "circle -n prof;");
    let r = ok(&mut c, "bevel -ns 2 -js 0 prof;");
    assert_eq!(r.as_array().unwrap().len(), 3, "the start bevel, the wall, and the node: {r}");
    let r = ok(&mut c, "bevel -w 0.2 prof;");
    let node = r.as_array().unwrap()[1].as_str().unwrap().to_string();
    close(num(&ok(&mut c, &format!("bevel -q -w {node};"))), 0.2, "width");
    assert_eq!(ok(&mut c, &format!("bevel -q -ns {node};")), json!(4));
    ok(&mut c, &format!("bevel -e -js 0 {node};"));
    assert_eq!(ok(&mut c, &format!("bevel -q -js {node};")), json!(false));

    // bevelPlus: inside, normals out.
    let r = ok(&mut c, "bevelPlus -po 0 -bin 1 -cap 1 prof;");
    let node = r.as_array().unwrap().last().unwrap().as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("bevelPlus -q -bin {node};")), json!(true));
    assert_eq!(ok(&mut c, &format!("bevelPlus -q -no {node};")), json!(true));

    // boundary and blend2 take -q and -e on their nodes.
    ok(&mut c, "curve -d 1 -p 0 0 0 -p 1 0 0 -n b1; curve -d 1 -p 1 0 0 -p 1 0 1 -n b2; curve -d 1 -p 1 0 1 -p 0 0 1 -n b3; curve -d 1 -p 0 0 1 -p 0 0 0 -n b4;");
    ok(&mut c, "boundary -nds 0 b1 b2 b3 b4;");
    assert_eq!(ok(&mut c, "boundary -q -nds boundary1;"), json!(0));
    ok(&mut c, "boundary -e -cch 1 boundary1;");
    assert_eq!(ok(&mut c, "boundary -q -cch boundary1;"), json!(true));

    // arclen: -ch makes a curveInfo node that follows the curve.
    let info = ok(&mut c, "arclen -ch 1 b1;").as_str().unwrap().to_string();
    close(num(&ok(&mut c, &format!("getAttr {info}.arcLength;"))), 1.0, "length");
    ok(&mut c, "scale 3 1 1 b1;");
    close(num(&ok(&mut c, &format!("getAttr {info}.arcLength;"))), 3.0, "live length");
    ok(&mut c, &format!("arclen -e -nds 2 {info};"));
    assert_eq!(ok(&mut c, &format!("arclen -q -nds {info};")), json!(2));
}

#[test]
fn angle_between_parts_and_history() {
    let mut c = Core::new();
    let v = nums(&ok(&mut c, "angleBetween -v1x 1 -v2y 1;"));
    close(v[3], 90.0, "axis and angle from the parts");
    let n = ok(&mut c, "angleBetween -v1 1 0 0 -v2 1 0 0 -ch on;").as_str().unwrap().to_string();
    ok(&mut c, &format!("setAttr {n}.vector2X 0; setAttr {n}.vector2Z 1;"));
    close(num(&ok(&mut c, &format!("getAttr {n}.angle;"))), 90.0, "the node computes");
}

#[test]
fn ambient_light_soft_shadow_and_attribute_info_leaf() {
    let mut c = Core::new();
    let l = ok(&mut c, "ambientLight -ss 1 -as 0.3;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("ambientLight -q -ss {l};")), json!(true));
    ok(&mut c, &format!("ambientLight -e -ss 0 {l};"));
    assert_eq!(ok(&mut c, &format!("getAttr {l}.castSoftShadows;")), json!(false));
    ok(&mut c, "createNode transform -n t1; addAttr -ln grp -at compound -nc 2 t1; addAttr -ln aa -at double -p grp t1; addAttr -ln bb -at double -p grp t1;");
    let leaves = ok(&mut c, "attributeInfo -l on t1;");
    let list: Vec<String> = leaves.as_array().unwrap().iter().filter_map(|v| v.as_str().map(str::to_string)).collect();
    assert!(list.iter().any(|x| x.ends_with("aa")), "{list:?}");
    assert!(!list.iter().any(|x| x == "translate"), "{list:?}");
}

#[test]
fn bind_skin_locks_and_unlocks() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 0 2 0 -n j2; select -cl; polyCube -n box;");
    ok(&mut c, "bindSkin -ts j1 box;");
    assert_eq!(ok(&mut c, "getAttr -l box.tx;"), json!(true));
    ok(&mut c, "bindSkin -ul box;");
    assert_eq!(ok(&mut c, "getAttr -l box.tx;"), json!(false));
    ok(&mut c, "bindSkin -d j1 box;");
}

#[test]
fn buffer_curve_from_a_referenced_curve() {
    let dir = std::env::temp_dir().join(format!("aztek_u01_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("anim.ma").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    ok(&mut c, "polyCube -n box; setKeyframe -t 1 -v 0 box.tx; setKeyframe -t 10 -v 5 box.tx;");
    ok(&mut c, &format!("file -rename \"{file}\"; file -save -type \"mayaAscii\";"));
    let mut c = Core::new();
    ok(&mut c, &format!("file -r -ns \"ref\" \"{file}\";"));
    ok(&mut c, "setKeyframe -t 10 -v 9 ref:box.tx;");
    // The buffer becomes the curve as the file has it.
    assert_eq!(ok(&mut c, "bufferCurve -urc ref:box.tx;"), json!(1));
    assert_eq!(ok(&mut c, "bufferCurve -q -urc ref:box.tx;"), json!(true));
    ok(&mut c, "bufferCurve -sw ref:box.tx;");
    close(num(&ok(&mut c, "keyframe -t 10 -q -vc ref:box.tx;")), 5.0, "swapped to the referenced curve");
    // Left out, -ov is off: an existing buffer stays.
    ok(&mut c, "polyCube -n b2; setKeyframe -t 1 -v 1 b2.ty; bufferCurve b2.ty; setKeyframe -t 1 -v 2 b2.ty; bufferCurve b2.ty; bufferCurve -sw b2.ty;");
    close(num(&ok(&mut c, "getAttr b2.ty;")), 1.0, "the first buffer was kept");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn blend_shape_target_tools() {
    let mut c = Core::new();
    ok(&mut c, "polyPlane -sx 2 -sy 1 -w 2 -h 1 -n base; polyPlane -sx 2 -sy 1 -w 2 -h 1 -n t1; polyPlane -sx 2 -sy 1 -w 2 -h 1 -n t2;");
    // t1 lifts the right edge (+X).
    ok(&mut c, "move -r 0 1 0 t1.vtx[2] t1.vtx[5];");
    ok(&mut c, "move -r 0 0.5 0 t2.vtx[1] t2.vtx[4];");
    let bs = ok(&mut c, "blendShape -n bs t1 t2 base;");
    assert_eq!(bs, json!(["bs"]));
    // Components and dump info.
    let comps = ok(&mut c, "blendShape -q -cmp bs;");
    assert!(comps.as_array().unwrap()[0].as_str().unwrap().contains("vtx["), "{comps}");
    assert_eq!(ok(&mut c, "blendShape -q -dui bs;"), json!([0, 6, 6]));
    ok(&mut c, "select -r base.vtx[0];");
    assert_eq!(ok(&mut c, "blendShape -q -cms bs;").as_array().unwrap().len(), 1);
    // Copy target 0 onto target 2.
    ok(&mut c, "blendShape -e -cd 0 0 2 bs;");
    ok(&mut c, "setAttr bs.w[2] 1;");
    close(nums(&ok(&mut c, "pointPosition -l base.vtx[2];"))[1], 1.0, "copied delta");
    ok(&mut c, "setAttr bs.w[2] 0;");
    // Flip target 0 across X: the left edge lifts.
    ok(&mut c, "blendShape -e -sa X -ft 0 0 bs; setAttr bs.w[0] 1;");
    assert_eq!(ok(&mut c, "blendShape -q -sa bs;"), json!("X"));
    let left = nums(&ok(&mut c, "pointPosition -l base.vtx[0];"));
    close(left[1], 1.0, "flipped onto the left");
    let right = nums(&ok(&mut c, "pointPosition -l base.vtx[2];"));
    close(right[1], 0.0, "and off the right");
    // Mirror the negative side onto the positive: both edges lift.
    ok(&mut c, "blendShape -e -md 0 -mt 0 0 bs;");
    close(nums(&ok(&mut c, "pointPosition -l base.vtx[2];"))[1], 1.0, "mirrored");
    ok(&mut c, "setAttr bs.w[0] 0;");
    // Merge targets 0 and 1 into 3.
    ok(&mut c, "blendShape -e -mgs 0 -mgs 1 -mgt 3 bs; setAttr bs.w[3] 1;");
    close(nums(&ok(&mut c, "pointPosition -l base.vtx[1];"))[1], 0.5, "merged");
    ok(&mut c, "setAttr bs.w[3] 0;");
    // Export and import.
    let dir = std::env::temp_dir().join(format!("aztek_u01_shp_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bs.shp").to_string_lossy().replace('\\', "/");
    ok(&mut c, &format!("blendShape -e -ep \"{path}\" -et 0 1 bs;"));
    ok(&mut c, "polyPlane -sx 2 -sy 1 -w 2 -h 1 -n other; polyPlane -sx 2 -sy 1 -w 2 -h 1 -n seed;");
    ok(&mut c, "blendShape -n bs2 seed other;");
    ok(&mut c, &format!("blendShape -e -ip \"{path}\" bs2;"));
    ok(&mut c, "setAttr bs2.w[1] 1;");
    close(nums(&ok(&mut c, "pointPosition -l other.vtx[4];"))[1], 0.5, "imported");
    let _ = std::fs::remove_dir_all(&dir);
    // Settings that are only read back.
    assert_eq!(ok(&mut c, "blendShape -q -ng bs;"), json!([]));
    assert_eq!(ok(&mut c, "blendShape -q -edt bs;"), json!(""));
    ok(&mut c, "blendShape -e -ss 1 -se \"base.e[1]\" bs;");
    assert_eq!(ok(&mut c, "blendShape -q -ss bs;"), json!(1));
}

#[test]
fn blend_shape_creation_flags() {
    let mut c = Core::new();
    ok(&mut c, "polyCube -n base; polyCube -n tgt; move -r 0 1 0 tgt.vtx[0];");
    ok(&mut c, "blendShape -n bs -uct -ex \"myPart\" tgt base;");
    assert!(c.mel_checked("sets -q bsSet;").is_err(), "no deformer set with -uct");
    ok(&mut c, "polyCube -n base2; polyCube -n tgt2;");
    ok(&mut c, "blendShape -n bs2 -ex \"myPart\" tgt2 base2;");
    assert_eq!(ok(&mut c, "blendShape -q -ex bs2;"), json!("myPart"));
    // Tangent space: a delta follows the surface as the base turns under it.
    ok(&mut c, "polyPlane -sx 1 -sy 1 -n p; polyPlane -sx 1 -sy 1 -n pt; move -r 0 1 0 pt.vtx[0];");
    ok(&mut c, "blendShape -n bst -ts pt p; setAttr bst.w[0] 1;");
    assert_eq!(ok(&mut c, "blendShape -q -ts bst;"), json!(true));
    close(nums(&ok(&mut c, "pointPosition -l p.vtx[0];"))[1], 1.0, "along the normal");
    // A post transform turns the deltas with it.
    ok(&mut c, "polyPlane -sx 1 -sy 1 -n q; polyPlane -sx 1 -sy 1 -n qt; move -r 0 1 0 qt.vtx[0]; select -cl; joint -n jt;");
    ok(&mut c, "blendShape -n bsq qt q; setAttr bsq.w[0] 1; blendShape -e -tr jt bsq;");
    assert!(ok(&mut c, "blendShape -q -tr bsq;").as_str().unwrap().ends_with("jt"));
    ok(&mut c, "setAttr jt.rz 90;");
    // The lift (0 1 0) turned 90 degrees about Z is (-1 0 0).
    let p = nums(&ok(&mut c, "pointPosition -l q.vtx[0];"));
    close3(&p, [-1.5, 0.0, 0.5], "turned with the joint");
}

#[test]
fn bone_lattice_deformer_flags() {
    let mut c = Core::new();
    ok(&mut c, "select -cl; joint -p 0 0 0 -n j1; joint -p 0 4 0 -n j2; joint -p 0 8 0 -n j3; select -cl;");
    ok(&mut c, "polyCylinder -h 8 -sy 8 -n arm; move 0 4 0 arm;");
    ok(&mut c, "lattice -dv 2 5 2 -oc 1 -n armLat arm;");
    let n = ok(&mut c, "boneLattice -j j2 -t j1 armLatLattice;").as_str().unwrap().to_string();
    assert_eq!(ok(&mut c, &format!("boneLattice -q -gi {n};")), json!([0]));
    let comps = ok(&mut c, &format!("boneLattice -q -cmp {n};"));
    assert!(comps.as_array().unwrap()[0].as_str().unwrap().contains(".pt[0:1][0:4][0:1]"), "{comps}");
    assert_eq!(ok(&mut c, &format!("boneLattice -q -dui {n};")), json!([0, 20, 20]));
    let tools = ok(&mut c, &format!("boneLattice -q -dt {n};"));
    assert_eq!(tools.as_array().unwrap().len(), 2);
    // Nothing bends: pruning takes every point out.
    ok(&mut c, &format!("boneLattice -e -pr {n};"));
    assert_eq!(ok(&mut c, &format!("boneLattice -q -dui {n};")), json!([0, 0, 20]));
    ok(&mut c, &format!("boneLattice -e -ex \"flexPart\" {n};"));
    assert_eq!(ok(&mut c, &format!("boneLattice -q -ex {n};")), json!("flexPart"));
    ok(&mut c, &format!("boneLattice -e -rm -g armLatLattice {n};"));
    assert_eq!(ok(&mut c, &format!("boneLattice -q -gi {n};")), json!([]));
    // A second one in front of the first.
    let m = ok(&mut c, "boneLattice -foc -j j2 -t j1 armLatLattice;").as_str().unwrap().to_string();
    assert!(!m.is_empty());
}
