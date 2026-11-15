//! Issue 48: hiding components with H in a component mode, as the reference does.

use aztek_host::Core;
use serde_json::json;

fn hidden(c: &Core) -> Vec<u32> {
    let g = &c.s.graph;
    let shape = g.find("aShape").unwrap().unwrap();
    aztek_scene::hidden_components::hidden(g, shape).to_vec()
}

#[test]
fn h_hides_selected_faces_and_shows_them_again() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; selectMode -component; selectType -polymeshFace 1; select -r a.f[1:2];").unwrap();
    // H: every selected face shows, so they hide.
    assert_eq!(c.mel_checked("toggleVisibility;").unwrap(), json!(false));
    assert_eq!(hidden(&c), vec![1, 2]);
    assert_eq!(c.mel_checked("getAttr a.visibility;").unwrap(), json!(true), "the object stays");
    // The views leave them out, with the edges and points only they used.
    let items = aztek_host::viewport::gather(&mut c.s.graph);
    let m = &items.iter().find(|i| c.s.graph.path(i.shape).ends_with("aShape")).unwrap().mesh;
    assert!(!m.face_valid(1) && !m.face_valid(2) && m.face_valid(0));
    // Component tools skip them: select all faces takes the four that show.
    c.mel_checked("select -r a.f[0]; selectAllComponents;").unwrap();
    let all = c.mel_checked("ls -sl -fl;").unwrap();
    assert_eq!(all.as_array().unwrap().len(), 4, "{all}");
    // Double-click on what shows picks the whole shell, hidden faces too: H then shows them.
    let shape = c.mel_checked("selectConnected aShape -kind face -id 0;").unwrap();
    assert_eq!(shape.as_array().unwrap().len(), 6, "{shape}");
    // H with hidden faces among the selection shows every hidden face.
    assert_eq!(c.mel_checked("toggleVisibility;").unwrap(), json!(true));
    assert!(hidden(&c).is_empty());
    c.mel_checked("undo;").unwrap();
    assert_eq!(hidden(&c), vec![1, 2]);
    c.mel_checked("redo;").unwrap();
    assert!(hidden(&c).is_empty());
}

#[test]
fn edges_and_vertices_hide_the_faces_they_touch() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; selectMode -component; selectType -polymeshVertex 1;").unwrap();
    c.mel_checked("hide a.vtx[0];").unwrap();
    assert_eq!(hidden(&c).len(), 3, "a cube corner touches three faces");
    // Shift+H shows the named components; with none selected, every hidden one on the selected objects.
    let first = hidden(&c)[0];
    c.mel_checked(&format!("showHidden a.f[{first}];")).unwrap();
    assert_eq!(hidden(&c).len(), 2);
    c.mel_checked("select -r a; select -cl;").unwrap();
    c.mel_checked("showHidden a.f[*];").unwrap();
    assert!(hidden(&c).is_empty());
    // In object mode hide still hides objects.
    c.mel_checked("selectMode -object; hide a;").unwrap();
    assert_eq!(c.mel_checked("getAttr a.visibility;").unwrap(), json!(false));
}
