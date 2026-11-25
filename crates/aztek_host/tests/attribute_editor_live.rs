//! The Attribute Editor's data: the live values its tabs are built from, and that changing the scene changes them.

use aztek_host::{query, Core};
use serde_json::Value;

fn mel(c: &mut Core, src: &str) -> Value {
    let v = c.mel(src);
    let errors: Vec<String> = c.take_log().as_array().unwrap().iter().filter(|l| l["kind"] == 2).map(|l| l["text"].as_str().unwrap().to_string()).collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    v
}

fn attr<'a>(tab: &'a Value, name: &str) -> &'a Value {
    tab["attrs"].as_array().unwrap().iter().find(|a| a["attr"] == name).unwrap_or_else(|| panic!("no {name} in {tab}"))
}

fn tab<'a>(ae: &'a Value, name: &str) -> &'a Value {
    ae["tabs"].as_array().unwrap().iter().find(|t| t["name"] == name).unwrap_or_else(|| panic!("no {name} tab in {ae}"))
}

#[test]
fn values_are_read_live_and_undo_comes_back() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n pCube1;");
    mel(&mut c, "setAttr pCube1.translateX 4.5;");
    let ae = query::attribute_editor(&c, "pCube1");
    assert_eq!(attr(tab(&ae, "pCube1"), "translate")["value"], serde_json::json!([4.5, 0.0, 0.0]));

    mel(&mut c, "setAttr pCube1.translateX 7.25;");
    let ae = query::attribute_editor(&c, "pCube1");
    assert_eq!(attr(tab(&ae, "pCube1"), "translate")["value"], serde_json::json!([7.25, 0.0, 0.0]));

    mel(&mut c, "undo;");
    let ae = query::attribute_editor(&c, "pCube1");
    assert_eq!(attr(tab(&ae, "pCube1"), "translate")["value"], serde_json::json!([4.5, 0.0, 0.0]));
}

#[test]
fn the_uuid_and_offset_parent_matrix_are_read_from_the_node() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n pCube1;");
    let uuid = mel(&mut c, "ls -uuid pCube1;");
    let ae = query::attribute_editor(&c, "pCube1");
    let t = tab(&ae, "pCube1");
    assert_eq!(t["uuid"], uuid[0]);
    let matrix = attr(t, "offsetParentMatrix")["value"].as_array().unwrap();
    assert_eq!(matrix.len(), 16);
    assert_eq!(matrix[0], 1.0);
}

#[test]
fn the_type_label_and_tabs_follow_a_rename() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n pCube1;");
    mel(&mut c, "rename pCube1 myCube;");
    let ae = query::attribute_editor(&c, "myCube");
    assert_eq!(ae["focus"], "myCube");
    let tabs: Vec<&str> = ae["tabs"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tabs.contains(&"myCube"));
    assert!(!tabs.contains(&"pCube1"));
}

#[test]
fn a_poly_primitives_create_uvs_reads_its_names_and_writes_its_index() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n pCube1;");
    mel(&mut c, "select polyCube1;");
    let ae = query::attribute_editor(&c, "polyCube1");
    let t = tab(&ae, "polyCube1");
    let uv = attr(t, "createUVs");
    assert_eq!(uv["value"], 1);
    let names: Vec<&str> = uv["enum"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(names.first(), Some(&"None"));
    assert!(names.contains(&"Normalize Collectively"));

    mel(&mut c, "setAttr polyCube1.createUVs 2;");
    let ae = query::attribute_editor(&c, "polyCube1");
    assert_eq!(attr(tab(&ae, "polyCube1"), "createUVs")["value"], 2);
}

#[test]
fn changing_a_lights_type_keeps_the_new_shape_selected() {
    let mut c = Core::new();
    let shape = mel(&mut c, "pointLight -n lamp;").as_str().unwrap().to_string();
    mel(&mut c, &format!("select {shape};"));
    mel(&mut c, &format!("aztekChangeLightType {shape} directionalLight;"));
    assert_eq!(mel(&mut c, &format!("nodeType {shape};")), "directionalLight");
    assert_eq!(mel(&mut c, "ls -sl;"), serde_json::json!([shape]));
    let ae = query::attribute_editor(&c, &shape);
    assert_eq!(tab(&ae, &shape)["type"], "directionalLight");
}
