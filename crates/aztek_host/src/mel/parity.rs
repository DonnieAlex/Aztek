//! The parity ledger: `parity/` at the top of the repository holds one tab-separated file per area of the reference
//! (commands, their flags, node types, attributes, the APIs, environment variables, menus, hotkeys and the rest), one
//! row per item, each with a `status` (done, partial, missing, n/a) and an `evidence` column. `parity/areas.tsv` lists
//! the areas. Some areas are checked here against what Aztek has; the rest are ticked by hand. `parity/summary.tsv`
//! holds the counts, and the welcome screen's bar reads it (`aztekParity -q`).
//!
//! `cargo run -p aztek_host --example parity -- --write` brings the checked statuses and the summary up to date; a
//! test fails while they are stale.

use super::call;
use aztek_scene::Session;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const SUMMARY: &str = include_str!("../../../../parity/summary.tsv");

pub const STATUSES: [&str; 4] = ["done", "partial", "missing", "n/a"];

/// The evidence the checker writes; a row with any other evidence is the hand's and the checker leaves it.
pub const AUTO: &str = "auto";

/// `{percent, areas: [{area, rows, done, partial, missing, na, percent}], commands, commandsHave, flags, flagsHave}`,
/// from the ledger's summary as built into the program.
pub fn measure() -> Value {
    let t = Table::parse(SUMMARY);
    let col = |r: &Vec<String>, name: &str| t.col(name).and_then(|i| r.get(i)).cloned().unwrap_or_default();
    let num = |r: &Vec<String>, name: &str| col(r, name).parse::<f64>().unwrap_or(0.0);
    let mut out = json!({"percent": 0.0, "areas": [], "commands": 0, "commandsHave": 0, "flags": 0, "flagsHave": 0});
    let mut areas = Vec::new();
    for r in &t.rows {
        let area = col(r, "area");
        if area == "all" {
            out["percent"] = json!(num(r, "percent"));
            continue;
        }
        let (rows, done) = (num(r, "rows") as u64, num(r, "done") as u64);
        match area.as_str() {
            "commands" => {
                out["commands"] = json!(rows);
                out["commandsHave"] = json!(done);
            }
            "command_flags" => {
                out["flags"] = json!(rows);
                out["flagsHave"] = json!(done);
            }
            _ => {}
        }
        areas.push(json!({
            "area": area,
            "rows": rows,
            "done": done,
            "partial": num(r, "partial") as u64,
            "missing": num(r, "missing") as u64,
            "na": num(r, "na") as u64,
            "percent": num(r, "percent"),
        }));
    }
    out["areas"] = json!(areas);
    out
}

/// A tab-separated file with a header row. Cells stay as written (tabs and newlines in them are escaped).
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn parse(text: &str) -> Table {
        let mut lines = text.lines().filter(|l| !l.is_empty());
        let header: Vec<String> = lines.next().unwrap_or("").split('\t').map(str::to_string).collect();
        let rows = lines
            .map(|l| {
                let mut r: Vec<String> = l.split('\t').map(str::to_string).collect();
                r.resize(header.len(), String::new());
                r
            })
            .collect();
        Table { header, rows }
    }

    pub fn col(&self, name: &str) -> Option<usize> {
        self.header.iter().position(|h| h == name)
    }

    pub fn text(&self) -> String {
        let mut s = self.header.join("\t");
        s.push('\n');
        for r in &self.rows {
            s.push_str(&r.join("\t"));
            s.push('\n');
        }
        s
    }
}

/// One line of `parity/areas.tsv`.
#[derive(Clone, Debug)]
pub struct Area {
    pub area: String,
    /// The columns that name a row, comma-separated in the file.
    pub key: Vec<String>,
    /// `commands`, `flags`, `node_types`, `node_attributes`, or `hand`.
    pub check: String,
    pub weight: f64,
}

pub fn areas(dir: &Path) -> Result<Vec<Area>, String> {
    let path = dir.join("areas.tsv");
    let t = Table::parse(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?);
    let get = |r: &Vec<String>, n: &str| t.col(n).and_then(|i| r.get(i)).cloned().unwrap_or_default();
    Ok(t.rows
        .iter()
        .map(|r| Area {
            area: get(r, "area"),
            key: get(r, "key").split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            check: get(r, "check"),
            weight: get(r, "weight").parse().unwrap_or(1.0),
        })
        .collect())
}

/// What Aztek answers to for a command name: whether it's there at all, and its flag names (long and short), or
/// None for all of them (MEL built-ins read their flags by hand).
fn command_flags(s: &Session, name: &str) -> Option<Option<HashSet<String>>> {
    let alias = call::alias(name);
    let def = s.commands.find(name).or_else(|| alias.and_then(|a| s.commands.find(a.target)));
    let builtin = super::builtins::NAMES.contains(&name) || super::builtins_sys::NAMES.contains(&name);
    match def {
        Some(def) => {
            let mut ours: HashSet<String> =
                def.flags.iter().flat_map(|f| [f.name.clone(), f.short_name.clone()]).filter(|n| !n.is_empty()).collect();
            if let Some(a) = alias {
                ours.extend(a.flags.iter().map(|(r, _)| r.to_string()));
            }
            Some(Some(ours))
        }
        None if alias.is_some() => Some(Some(HashSet::new())),
        None if builtin => Some(None),
        None => None,
    }
}

/// Whether Aztek has this row, for the areas it can tell; None for areas ticked by hand.
struct Checker<'a> {
    s: &'a Session,
    commands: HashMap<String, Option<Option<HashSet<String>>>>,
}

impl Checker<'_> {
    fn command(&mut self, name: &str) -> &Option<Option<HashSet<String>>> {
        if !self.commands.contains_key(name) {
            let v = command_flags(self.s, name);
            self.commands.insert(name.to_string(), v);
        }
        &self.commands[name]
    }

    fn has(&mut self, check: &str, t: &Table, r: &[String]) -> Option<bool> {
        let get = |n: &str| t.col(n).map(|i| r[i].as_str()).unwrap_or("");
        let first = r.first().map(String::as_str).unwrap_or("");
        Some(match check {
            "commands" => self.command(first).is_some(),
            "flags" => {
                let (flag, short) = (get("flag").to_string(), get("short").to_string());
                match self.command(get("command")) {
                    None => false,
                    Some(None) => true,
                    Some(Some(ours)) => ours.contains(&flag) || (!short.is_empty() && ours.contains(&short)),
                }
            }
            "node_types" => aztek_scene::node::types::find(get("node_type")).is_some(),
            "node_attributes" => {
                let attr = get("attribute");
                aztek_scene::node::types::find(get("node_type")).is_some_and(|ty| ty.attrs.iter().any(|a| a.name == attr))
            }
            _ => return None,
        })
    }
}

/// Counts for one area: rows, done, partial, missing, n/a.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Counts {
    pub rows: usize,
    pub done: usize,
    pub partial: usize,
    pub missing: usize,
    pub na: usize,
}

impl Counts {
    /// Done, and partial at half, over the rows that apply; None when none do.
    pub fn percent(&self) -> Option<f64> {
        let applies = self.rows - self.na;
        (applies > 0).then(|| (self.done as f64 + 0.5 * self.partial as f64) * 100.0 / applies as f64)
    }
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

/// The ledger brought up to date: each file whose text would change, with its new text (checked statuses first, then
/// `summary.tsv`). Errors name bad statuses or unreadable files.
pub fn check(s: &Session, dir: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let mut changed = Vec::new();
    let mut checker = Checker { s, commands: HashMap::new() };
    let mut summary = Table {
        header: ["area", "rows", "done", "partial", "missing", "na", "percent", "weight"].map(String::from).to_vec(),
        rows: Vec::new(),
    };
    let (mut sum, mut weights) = (0.0, 0.0);
    let mut errors = Vec::new();
    for a in areas(dir)? {
        let path = dir.join(format!("{}.tsv", a.area));
        let mut c = Counts::default();
        if let Ok(text) = std::fs::read_to_string(&path) {
            let mut t = Table::parse(&text);
            let (Some(si), Some(ei)) = (t.col("status"), t.col("evidence")) else {
                return Err(format!("{}: no status and evidence columns", path.display()));
            };
            let mut rows = std::mem::take(&mut t.rows);
            for (n, r) in rows.iter_mut().enumerate() {
                if r[ei].is_empty() || r[ei] == AUTO {
                    if let Some(has) = checker.has(&a.check, &t, r) {
                        r[si] = if has { "done" } else { "missing" }.to_string();
                        r[ei] = if has { AUTO.to_string() } else { String::new() };
                    }
                }
                if r[si].is_empty() {
                    r[si] = "missing".to_string();
                }
                match r[si].as_str() {
                    "done" => c.done += 1,
                    "partial" => c.partial += 1,
                    "missing" => c.missing += 1,
                    "n/a" => c.na += 1,
                    other => errors.push(format!("{}:{}: status '{other}' (done, partial, missing or n/a)", path.display(), n + 2)),
                }
                c.rows += 1;
            }
            t.rows = rows;
            let new = t.text();
            if new != text {
                changed.push((path, new));
            }
        }
        if let Some(p) = c.percent() {
            sum += p * a.weight;
            weights += a.weight;
        }
        summary.rows.push(vec![
            a.area.clone(),
            c.rows.to_string(),
            c.done.to_string(),
            c.partial.to_string(),
            c.missing.to_string(),
            c.na.to_string(),
            c.percent().map(|p| format!("{:.1}", round1(p))).unwrap_or_default(),
            format!("{}", a.weight),
        ]);
    }
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    let all = if weights > 0.0 { sum / weights } else { 0.0 };
    summary.rows.push(vec!["all".into(), String::new(), String::new(), String::new(), String::new(), String::new(), format!("{:.1}", round1(all)), String::new()]);
    let path = dir.join("summary.tsv");
    let new = summary.text();
    if std::fs::read_to_string(&path).ok().as_deref() != Some(new.as_str()) {
        changed.push((path, new));
    }
    Ok(changed)
}

/// The rows of an area still missing, by their key.
pub fn missing(dir: &Path, area: &str) -> Result<Vec<String>, String> {
    let a = areas(dir)?.into_iter().find(|a| a.area == area).ok_or_else(|| format!("no area '{area}' in areas.tsv"))?;
    let path = dir.join(format!("{area}.tsv"));
    let t = Table::parse(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?);
    let si = t.col("status").ok_or("no status column")?;
    let keys: Vec<usize> = a.key.iter().filter_map(|k| t.col(k)).collect();
    Ok(t.rows.iter().filter(|r| r[si] == "missing").map(|r| keys.iter().map(|&i| r[i].as_str()).collect::<Vec<_>>().join(" ")).collect())
}
