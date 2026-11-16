//! translator: the file types file -type knows.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[test]
fn lists_types_by_loaded_plug_in() {
    let mut s = Session::new();
    let l = run(&mut s, "translator", json!({"query": true, "queried": ["list"]}));
    assert_eq!(l, json!(["mayaAscii", "mayaBinary"]));
    aztek_scene::alembic::register(&mut s.commands);
    let l = run(&mut s, "translator", json!({"query": true, "queried": ["list"]}));
    assert_eq!(l, json!(["mayaAscii", "mayaBinary", "Alembic"]));
    assert_eq!(run(&mut s, "translator", json!({"args": ["Alembic"], "query": true, "queried": ["extension"]})), json!("abc"));
    assert_eq!(run(&mut s, "translator", json!({"args": ["Alembic"], "query": true, "queried": ["writeSupport"]})), json!(false));
    assert_eq!(run(&mut s, "translator", json!({"args": ["mayaAscii"], "query": true, "queried": ["filter"]})), json!("*.ma"));
    // file reads Alembic by type or extension.
    run(&mut s, "polyCube", json!({"name": "c"}));
    let path = std::env::temp_dir().join(format!("tr_{}.abc", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "AbcExport", json!({"jobArg": format!("-root c -file {path}")}));
    run(&mut s, "file", json!({"args": [path.clone()], "import": true, "type": "Alembic"}));
    assert_eq!(run(&mut s, "ls", json!({"args": ["c*"], "type": "transform"})).as_array().unwrap().len(), 2);
}
