//! MEL: the lexer, parser and interpreter scripts, the command line and the script editor run through. `call` turns a
//! command's evaluated arguments into a command call; the state that outlives a run lives in `Core::mel_state`.

mod builtins;
mod bundled;
mod builtins_sys;
mod call;
mod dupvars;
mod interp;
mod lexer;
mod parser;
pub mod parity;
pub(crate) mod pattern;
mod value;

pub use call::*;
pub use interp::{call_for_value, global_proc_names, has_global_proc, run, run_callback, MelState};
pub use value::{fmt_float, Value as MelValue};

/// Every procedure and built-in function the Script Editor's completion can name: the session's globals first, the
/// bundled scripts' next, then the built-in functions no procedure shadows. Signatures are declarations built from the
/// return type and parameters, the bundled ones as written.
pub fn completion_names(core: &crate::core::Core) -> (Vec<(String, String, &'static str)>, Vec<&'static str>) {
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    let mut procs = Vec::new();
    for (name, signature) in interp::global_proc_signatures(core) {
        seen.insert(name.clone());
        procs.push((name, signature, "session"));
    }
    for (name, signature) in bundled::procedure_signatures() {
        if !seen.insert(name.clone()) {
            continue;
        }
        procs.push((name.clone(), signature.clone(), "bundled"));
    }
    procs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut builtins: Vec<&'static str> = builtins::NAMES
        .iter()
        .chain(builtins_sys::NAMES.iter())
        .copied()
        .filter(|n| !seen.contains(*n))
        .collect();
    builtins.sort_unstable();
    builtins.dedup();
    (procs, builtins)
}
