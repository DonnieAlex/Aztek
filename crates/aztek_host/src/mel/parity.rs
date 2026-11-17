//! How far Aztek is towards the reference, as one number: of the reference's commands and their flags, the share
//! Aztek takes. Each command counts once and each of its flags once more; a flag counts only when its command is
//! there. A command is there when it is registered, runs under another name (an alias), or is built into MEL.

use super::call;
use aztek_scene::Session;
use serde_json::{json, Value};
use std::collections::HashSet;

const REFERENCE: &str = include_str!("../../../../tools/reference_capture/reference_commands.txt");

/// `{percent, commands, commandsHave, flags, flagsHave, missing}`; `missing` is the absent commands, by name.
pub fn measure(s: &Session) -> Value {
    let (mut commands, mut commands_have, mut flags, mut flags_have) = (0usize, 0usize, 0usize, 0usize);
    let mut missing = Vec::new();
    for line in REFERENCE.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let mut words = line.split_whitespace();
        let Some(name) = words.next() else { continue };
        let theirs: Vec<&str> = words.collect();
        commands += 1;
        flags += theirs.len();
        // What it answers to: its own definition, or the one an alias runs, with the alias's renamed flags.
        let alias = call::alias(name);
        let def = s.commands.find(name).or_else(|| alias.and_then(|a| s.commands.find(a.target)));
        let builtin = super::builtins::NAMES.contains(&name) || super::builtins_sys::NAMES.contains(&name);
        if def.is_none() && alias.is_none() && !builtin {
            missing.push(name.to_string());
            continue;
        }
        commands_have += 1;
        let Some(def) = def else {
            // Built into MEL: its flags are read by hand, so all of them count.
            if builtin {
                flags_have += theirs.len();
            }
            continue;
        };
        let mut ours: HashSet<&str> = def.flags.iter().flat_map(|f| [f.name.as_str(), f.short_name.as_str()]).filter(|n| !n.is_empty()).collect();
        if let Some(a) = alias {
            ours.extend(a.flags.iter().map(|(r, _)| *r));
        }
        flags_have += theirs.iter().filter(|f| ours.contains(**f)).count();
    }
    let items = (commands + flags).max(1);
    let percent = (commands_have + flags_have) as f64 * 100.0 / items as f64;
    json!({
        "percent": (percent * 10.0).round() / 10.0,
        "commands": commands,
        "commandsHave": commands_have,
        "flags": flags,
        "flagsHave": flags_have,
        "missing": missing,
    })
}
