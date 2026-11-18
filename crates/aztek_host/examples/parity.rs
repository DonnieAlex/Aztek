//! The parity ledger (parity/): prints each area's counts and the welcome screen's figure, and with --write brings the
//! checked statuses and parity/summary.tsv up to date. --missing [area] lists what an area still lacks (commands unless
//! named).
//!
//!     cargo run -p aztek_host --example parity -- --write
fn main() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../parity");
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(i) = args.iter().position(|a| a == "--missing") {
        let area = args.get(i + 1).map(String::as_str).unwrap_or("commands");
        match aztek_host::mel::parity::missing(&dir, area) {
            Ok(v) => v.iter().for_each(|k| println!("{k}")),
            Err(e) => eprintln!("{e}"),
        }
        return;
    }
    let c = aztek_host::Core::new();
    let changed = match aztek_host::mel::parity::check(&c.s, &dir) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let write = args.iter().any(|a| a == "--write");
    for (path, text) in &changed {
        if write {
            std::fs::write(path, text).expect("write");
            println!("wrote {}", path.display());
        } else {
            println!("stale {}", path.display());
        }
    }
    let summary = changed.iter().find(|(p, _)| p.ends_with("summary.tsv")).map(|(_, t)| t.clone());
    let summary = summary.unwrap_or_else(|| std::fs::read_to_string(dir.join("summary.tsv")).unwrap_or_default());
    print!("{summary}");
    if !changed.is_empty() && !write {
        println!("run again with --write to update them");
    }
}
