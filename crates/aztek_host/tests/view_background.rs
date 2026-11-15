//! Issue 63: the gradient background, switched by displayPref -displayGradient, coloured by displayRGBColor
//! backgroundTop and backgroundBottom, and kept across a new scene.

use aztek_host::Core;
use aztek_scene::system_cmds::{display_gradient, view_rgb};
use serde_json::json;

#[test]
fn the_gradient_is_a_display_preference() {
    let mut c = Core::new();
    assert!(!display_gradient(&c.s));
    assert_eq!(c.mel_checked("displayPref -q -displayGradient;").unwrap(), json!(false));
    c.mel_checked("displayPref -displayGradient 1;").unwrap();
    assert!(display_gradient(&c.s));
    c.mel_checked("displayRGBColor backgroundTop 0.2 0.3 0.4;").unwrap();
    assert_eq!(view_rgb("backgroundTop").unwrap()[..3], [0.2, 0.3, 0.4]);
    c.mel_checked("file -f -new;").unwrap();
    assert!(display_gradient(&c.s), "a preference, not part of the scene");
    c.mel_checked("displayPref -dgr 0;").unwrap();
    assert!(!display_gradient(&c.s));
    c.mel_checked("displayRGBColor -rf;").unwrap();
}

/// Issue 62: the grid's lines, subdivisions, axes and numbers as the grid command sets them.
#[test]
fn the_grid_command_sets_the_views_grid_style() {
    use aztek_host::viewport::GridStyle;
    let mut c = Core::new();
    c.mel_checked("grid -sp 4 -d 2 -da 0 -ddl 0 -dpl 1 -plp \"edge\";").unwrap();
    assert_eq!(c.mel_checked("grid -q -d;").unwrap(), json!(2));
    assert_eq!(c.mel_checked("grid -q -ddl;").unwrap(), json!(false));
    let st = GridStyle::from_json(&json!({"divisions": 2, "displayAxes": false, "displayDivisionLines": false, "displayPerspectiveLabels": true, "perspectiveLabelPosition": "edge"}));
    assert_eq!(st.divisions, 2);
    assert!(!st.axes && !st.division_lines && st.lines && st.persp_labels && st.persp_label_edge && !st.ortho_labels);
    c.mel_checked("displayRGBColor gridHighlight 1 0 0;").unwrap();
    assert_eq!(GridStyle::colors()[1], [255, 0, 0, 255]);
    c.mel_checked("displayRGBColor -rf; grid -r;").unwrap();
    assert_eq!(GridStyle::colors()[1], [70, 70, 70, 255]);
}
