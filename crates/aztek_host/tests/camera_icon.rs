//! The camera icon scales with its transform and its shape's locatorScale, as the reference's does.

use aztek_host::viewport::rig_items;
use aztek_host::Core;
use serde_json::json;

fn icon_width(c: &mut Core, cam: &str, shape: &str) -> f64 {
    // The world matrix as the view sees it, worked out.
    c.s.run("getAttr", json!({"args": [format!("{shape}.worldMatrix")]})).unwrap();
    let g = &c.s.graph;
    let xf = g.find(cam).unwrap().unwrap();
    let item = rig_items(g).into_iter().find(|r| r.node == xf).expect("the camera draws");
    let xs: Vec<f64> = item.segs.iter().flat_map(|(a, b)| [a.x, b.x]).collect();
    xs.iter().cloned().fold(f64::MIN, f64::max) - xs.iter().cloned().fold(f64::MAX, f64::min)
}

#[test]
fn the_icon_follows_scale_and_locator_scale() {
    let mut c = Core::new();
    c.s.run("camera", json!({"name": "c1"})).unwrap();
    let shape = c.s.run("listRelatives", json!({"args": ["c1"], "shapes": true})).unwrap()[0].as_str().unwrap().to_string();
    let base = icon_width(&mut c, "c1", &shape);
    c.s.run("scale", json!({"args": [3, 3, 3, "c1"]})).unwrap();
    assert!((icon_width(&mut c, "c1", &shape) - base * 3.0).abs() < 1e-9);
    c.s.run("setAttr", json!({"args": [format!("{shape}.locatorScale"), 2]})).unwrap();
    assert!((icon_width(&mut c, "c1", &shape) - base * 6.0).abs() < 1e-9);
}
