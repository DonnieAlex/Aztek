//! The parity ledger (parity/): the statuses Aztek can check and the summary the welcome screen reads are up to date.

use aztek_host::mel::parity::{self, Table};
use aztek_host::Core;
use std::path::PathBuf;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../parity")
}

/// A command added (or a flag, node type or attribute) ticks its row once the ledger is rewritten; until then this
/// fails.
#[test]
fn the_ledger_is_up_to_date() {
    let c = Core::new();
    let changed = parity::check(&c.s, &dir()).unwrap();
    let names: Vec<String> = changed.iter().map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned()).collect();
    assert!(names.is_empty(), "stale: {names:?}; run cargo run -p aztek_host --example parity -- --write");
}

/// Every area file has the area's key columns, a status from the four, and no key twice.
#[test]
fn the_ledger_is_well_formed() {
    for a in parity::areas(&dir()).unwrap() {
        let path = dir().join(format!("{}.tsv", a.area));
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let t = Table::parse(&text);
        let keys: Vec<usize> = a.key.iter().map(|k| t.col(k).unwrap_or_else(|| panic!("{}: no {k} column", a.area))).collect();
        let si = t.col("status").unwrap();
        let mut seen = std::collections::HashSet::new();
        for r in &t.rows {
            assert!(parity::STATUSES.contains(&r[si].as_str()), "{}: {:?}", a.area, r);
            let k: Vec<&str> = keys.iter().map(|&i| r[i].as_str()).collect();
            assert!(seen.insert(k.join("\t")), "{}: {:?} twice", a.area, k);
        }
    }
}

/// A row ticked by hand keeps its status; a checked one follows what Aztek has.
#[test]
fn the_checker_leaves_rows_ticked_by_hand() {
    let tmp = std::env::temp_dir().join(format!("aztek_parity_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    std::fs::write(tmp.join("areas.tsv"), "area\tkey\tcheck\tweight\tabout\ncommands\tcommand\tcommands\t1\t\nmenus\tpath\thand\t1\t\n").unwrap();
    std::fs::write(
        tmp.join("commands.tsv"),
        "command\tstatus\tevidence\npolyCube\tmissing\t\nnoSuchCommandAnywhere\tdone\tauto\nanotherMissingOne\tpartial\tits query only\n",
    )
    .unwrap();
    std::fs::write(tmp.join("menus.tsv"), "path\tstatus\tevidence\nFile|Open\tdone\tmainwindow.cpp\nFile|Nope\tn/a\tout of scope\n").unwrap();
    let c = Core::new();
    let changed = parity::check(&c.s, &tmp).unwrap();
    let text = |name: &str| changed.iter().find(|(p, _)| p.ends_with(name)).map(|(_, t)| t.clone()).unwrap();
    let commands = Table::parse(&text("commands.tsv"));
    assert_eq!(commands.rows[0], ["polyCube", "done", "auto"]);
    assert_eq!(commands.rows[1], ["noSuchCommandAnywhere", "missing", ""]);
    assert_eq!(commands.rows[2], ["anotherMissingOne", "partial", "its query only"]);
    // Commands: one done, one partial of three, 50%; menus: one done of the one that applies, 100%; the mean 75%.
    let summary = Table::parse(&text("summary.tsv"));
    let pct = |area: &str| summary.rows.iter().find(|r| r[0] == area).unwrap()[6].clone();
    assert_eq!(pct("commands"), "50.0");
    assert_eq!(pct("menus"), "100.0");
    assert_eq!(pct("all"), "75.0");
    std::fs::remove_dir_all(&tmp).ok();
}
