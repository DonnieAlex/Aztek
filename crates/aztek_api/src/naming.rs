//! Node names: validity and making a name unique by its numeric suffix.
//!
//! A non-DAG node's name clashes with any other node of that name. A DAG node's name clashes with a
//! non-DAG node of that name and with DAG nodes that share a parent with it.

use aztek_dg::{DgError, Graph, NodeId, Result};

/// Letters, digits and underscores, not starting with a digit; `#` stands for a number to fill in.
pub fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' || c == '#' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '#')
}

/// Whether `name` clashes for node `me`, whose DAG parents are `parents` (`None` for a non-DAG node).
pub fn clashes(g: &Graph, name: &str, me: Option<NodeId>, parents: Option<&[Option<NodeId>]>) -> bool {
    g.nodes_named(name).into_iter().filter(|n| Some(*n) != me).any(|other| match parents {
        Some(mine) if g.is_dag(other) => g.dag_parents(other).iter().any(|p| mine.contains(p)),
        _ => true,
    })
}

/// The name a node gets when it asks for `desired`: a run of `#` becomes the first number that is
/// free; otherwise a clashing name has its numeric suffix raised (or `1` added) until it is free.
pub fn unique_name(g: &Graph, desired: &str, me: Option<NodeId>, parents: Option<&[Option<NodeId>]>) -> Result<String> {
    if !is_valid_name(desired) {
        return Err(DgError::Invalid(format!("{desired:?} is not a valid node name")));
    }
    if let Some(at) = desired.find('#') {
        let run = desired[at..].chars().take_while(|c| *c == '#').count();
        let (head, tail) = (&desired[..at], &desired[at + run..]);
        if tail.contains('#') {
            return Err(DgError::Invalid(format!("{desired:?} has more than one run of #")));
        }
        return Ok((1u64..)
            .map(|n| format!("{head}{n}{tail}"))
            .find(|n| !clashes(g, n, me, parents))
            .unwrap());
    }
    if !clashes(g, desired, me, parents) {
        return Ok(desired.to_string());
    }
    let digits = desired.chars().rev().take_while(|c| c.is_ascii_digit()).count();
    let (base, suffix) = desired.split_at(desired.len() - digits);
    let start = suffix.parse::<u64>().map_or(1, |n| n + 1);
    Ok((start..).map(|n| format!("{base}{n}")).find(|n| !clashes(g, n, me, parents)).unwrap())
}
