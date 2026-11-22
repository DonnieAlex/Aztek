//! melOptions -duplicateVariableWarnings: variables declared twice in one scope, found as a script is sourced.

use std::collections::HashMap;

use super::parser::{parse_program, Stmt};

/// The names declared more than once in one scope, in the order the second declarations come.
pub fn duplicates(src: &str) -> Vec<String> {
    let Ok(prog) = parse_program(src) else { return Vec::new() };
    let mut out = Vec::new();
    let mut top = HashMap::new();
    for t in &prog {
        walk(&t.stmt, &mut top, &mut out);
    }
    out
}

/// A scope's declarations: each name and whether it was declared global.
type Scope = HashMap<String, bool>;

fn declare(scope: &mut Scope, name: &str, global: bool, out: &mut Vec<String>) {
    match scope.get(name) {
        // The same global declared again is the usual way to reach it, not a mistake.
        Some(&was) if !(was && global) => out.push(name.to_string()),
        Some(_) => {}
        None => {
            scope.insert(name.to_string(), global);
        }
    }
}

fn block(stmts: &[Stmt], out: &mut Vec<String>) {
    let mut scope = Scope::new();
    for s in stmts {
        walk(s, &mut scope, out);
    }
}

fn nested(s: &Stmt, out: &mut Vec<String>) {
    match s {
        Stmt::Block(b) => block(b, out),
        other => block(std::slice::from_ref(other), out),
    }
}

fn walk(s: &Stmt, scope: &mut Scope, out: &mut Vec<String>) {
    match s {
        Stmt::Decl(global, ds) => {
            for d in ds {
                declare(scope, &d.name, *global, out);
            }
        }
        Stmt::Block(b) => block(b, out),
        Stmt::If(_, a, b) => {
            nested(a, out);
            if let Some(b) = b {
                nested(b, out);
            }
        }
        Stmt::While(_, b) | Stmt::DoWhile(b, _) => nested(b, out),
        Stmt::For(init, _, _, body) => {
            let mut sc = Scope::new();
            for i in init {
                walk(i, &mut sc, out);
            }
            match &**body {
                Stmt::Block(b) => {
                    for x in b {
                        walk(x, &mut sc, out);
                    }
                }
                other => walk(other, &mut sc, out),
            }
        }
        Stmt::ForIn(ty, var, _, body) => {
            let mut sc = Scope::new();
            if ty.is_some() {
                declare(&mut sc, var, false, out);
            }
            match &**body {
                Stmt::Block(b) => {
                    for x in b {
                        walk(x, &mut sc, out);
                    }
                }
                other => walk(other, &mut sc, out),
            }
        }
        Stmt::Switch(_, cases) => {
            let mut sc = Scope::new();
            for (_, stmts) in cases {
                for x in stmts {
                    walk(x, &mut sc, out);
                }
            }
        }
        Stmt::Proc(p) => {
            let mut sc = Scope::new();
            for (_, n) in &p.params {
                declare(&mut sc, n, false, out);
            }
            for x in &p.body {
                walk(x, &mut sc, out);
            }
        }
        _ => {}
    }
}
