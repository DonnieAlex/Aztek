//! Prints Aztek's parity with the reference (the welcome screen's bar): cargo run -p aztek_host --example parity
fn main() {
    let mut c = aztek_host::Core::new();
    let v = c.exec("aztekParity", serde_json::json!({"query": true})).unwrap();
    let missing = v["missing"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(" ")).unwrap_or_default();
    println!("{}% ({} of {} commands, {} of {} flags)", v["percent"], v["commandsHave"], v["commands"], v["flagsHave"], v["flags"]);
    if std::env::args().any(|a| a == "--missing") {
        println!("missing: {missing}");
    }
}
