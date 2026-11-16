//! Strands in the renderer: NURBS curves with Render Curve on, shaded with aiStandardHair or a surface, as ribbons and
//! tubes; and the hair fibre's melanin darkening it.

use std::sync::Arc;

use aztek_scene::harold::{self, Request, Translator};
use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn set(s: &mut Session, plug: &str, v: Value) {
    let mut args = vec![json!(plug)];
    match v {
        Value::Array(a) => args.extend(a),
        other => args.push(other),
    }
    run(s, "setAttr", json!({ "args": args }));
}

fn scene() -> Session {
    let mut s = Session::new();
    aztek_scene::cameras::make_defaults(&mut s.graph);
    set(&mut s, "defaultResolution.width", json!(48));
    set(&mut s, "defaultResolution.height", json!(48));
    set(&mut s, "defaultArnoldRenderOptions.AASamples", json!(3));
    s
}

/// Seen from the front: a vertical row of curves across the frame, lit from the camera's side.
fn curves(s: &mut Session, width: f64) -> Vec<String> {
    let mut names = Vec::new();
    for k in 0..9 {
        let x = -4.0 + k as f64;
        let r = run(s, "curve", json!({"degree": 3, "point": [[x, -4.0, 0.0], [x + 0.3, -1.0, 0.0], [x - 0.3, 1.0, 0.0], [x, 4.0, 0.0]]}));
        let name = r.as_str().map(str::to_string).unwrap_or_else(|| r[0].as_str().unwrap().to_string());
        let shape = run(s, "listRelatives", json!({"args": [name.clone()], "shapes": true}))[0].as_str().unwrap().to_string();
        set(s, &format!("{shape}.aiRenderCurve"), json!(true));
        set(s, &format!("{shape}.aiCurveWidth"), json!(width));
        names.push(shape);
    }
    run(s, "directionalLight", json!({"rotation": [0.0, 0.0, 0.0], "intensity": 2.0}));
    names
}

/// The image, and the curve sets the scene held: (mode, radii) of each.
fn render(s: &mut Session) -> (aztek_render::film::Film, Vec<(aztek_render::geom::CurveMode, Vec<f32>)>) {
    let mut tr = Translator::default();
    let (scene, _) = harold::translate(s, &mut tr, &Request { camera: Some("front".into()), ..Default::default() }).unwrap();
    let keep = scene.geo.meshes.iter().filter_map(|m| m.curves.as_ref().map(|c| (c.mode, c.radius.clone()))).collect();
    let (film, _) = aztek_render::render::render(Arc::new(scene), 2);
    (film, keep)
}

fn mean(film: &aztek_render::film::Film) -> f32 {
    let px = film.pixels(0);
    px.iter().map(|p| p.x + p.y + p.z).sum::<f32>() / px.len() as f32
}

#[test]
fn render_curve_shows_and_off_shows_nothing() {
    let mut s = scene();
    let shapes = curves(&mut s, 0.4);
    let (lit, sc) = render(&mut s);
    assert_eq!(sc.len(), 9);
    assert!(mean(&lit) > 0.02, "{}", mean(&lit));
    for sh in &shapes {
        set(&mut s, &format!("{sh}.aiRenderCurve"), json!(false));
    }
    let (dark, sc) = render(&mut s);
    assert_eq!(sc.len(), 0);
    assert!(mean(&dark) < 1e-4, "{}", mean(&dark));
}

#[test]
fn hair_shader_on_curves_darkens_with_melanin() {
    let mut s = scene();
    let shapes = curves(&mut s, 0.4);
    let hair = run(&mut s, "shadingNode", json!({"args": ["aiStandardHair"], "asShader": true}));
    let hair = hair.as_str().unwrap().to_string();
    for sh in &shapes {
        run(&mut s, "connectAttr", json!({"args": [format!("{hair}.outColor"), format!("{sh}.aiCurveShader")]}));
    }
    set(&mut s, &format!("{hair}.melanin"), json!(0.05));
    let (blond, _) = render(&mut s);
    set(&mut s, &format!("{hair}.melanin"), json!(1.0));
    let (black, _) = render(&mut s);
    assert!(mean(&blond) > 0.0, "{}", mean(&blond));
    assert!(mean(&blond) > mean(&black) * 1.5, "{} {}", mean(&blond), mean(&black));
}

#[test]
fn thick_curves_cover_their_width() {
    let mut s = scene();
    let shapes = curves(&mut s, 0.6);
    for sh in &shapes {
        set(&mut s, &format!("{sh}.aiMode"), json!(1));
    }
    let (_, sc) = render(&mut s);
    let (mode, radius) = &sc[0];
    assert_eq!(*mode, aztek_render::geom::CurveMode::Thick);
    // Radius is half the width, at every point.
    assert!(radius.iter().all(|&r| (r - 0.3).abs() < 1e-6));
}
