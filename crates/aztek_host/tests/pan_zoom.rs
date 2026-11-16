//! panZoom: the 2D pan and zoom of a camera's film, on its attributes.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn pan_zoom_sets_the_film() {
    let mut c = Core::new();
    mel(&mut c, "panZoom -r 0.5 -u 0.25 -z 2 persp;");
    assert_eq!(mel(&mut c, "getAttr perspShape.panZoomEnabled;"), json!(true));
    assert_eq!(mel(&mut c, "getAttr perspShape.horizontalPan;"), json!(0.5));
    assert_eq!(mel(&mut c, "getAttr perspShape.verticalPan;"), json!(0.25));
    assert_eq!(mel(&mut c, "getAttr perspShape.zoom;"), json!(2.0));
    mel(&mut c, "panZoom -l 0.25 -z 0.5 persp;");
    assert_eq!(mel(&mut c, "getAttr perspShape.horizontalPan;"), json!(0.25));
    assert_eq!(mel(&mut c, "getAttr perspShape.zoom;"), json!(1.0));
    mel(&mut c, "panZoom -abs -r 0.1 -z 3 persp;");
    assert_eq!(mel(&mut c, "getAttr perspShape.horizontalPan;"), json!(0.1));
    assert_eq!(mel(&mut c, "getAttr perspShape.zoom;"), json!(3.0));
    mel(&mut c, "undo;");
    assert_eq!(mel(&mut c, "getAttr perspShape.zoom;"), json!(1.0));
}
