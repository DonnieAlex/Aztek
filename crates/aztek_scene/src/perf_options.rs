//! performanceOptions: the global settings that turn features off to save time. A deformer kind set to pass through
//! hands its input geometry on unchanged, always ("on") or while playback runs ("interactive"); evaluation asks
//! `bypass` before it computes a node. None of it changes what Harold renders from a saved file: the settings are the
//! session's, not the scene's.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use serde_json::{json, Value};

use crate::node::{NodeData, NodeType};
use crate::plug::PlugValue;
use crate::session::Session;

/// The on, off or interactive options: (long name, short name, what it covers).
pub const STATE_OPTIONS: &[(&str, &str, &str)] = &[
    ("disableStitch", "ds", "Stitch surfaces are not made."),
    ("disableTrimBoundaryDisplay", "dtb", "Surface trim boundaries are not drawn."),
    ("disableTrimDisplay", "dt", "Surface trims are not drawn."),
    ("passThroughBindSkinAndFlexors", "pbf", "Skin clusters and flexors pass their input through."),
    ("passThroughBlendShape", "pbs", "Blend shapes pass their input through."),
    ("passThroughCluster", "pc", "Clusters pass their input through."),
    ("passThroughDeltaMush", "pdm", "Delta mush deformers pass their input through."),
    ("passThroughFlexors", "pf", "Flexors pass their input through."),
    ("passThroughLattice", "pl", "Lattice deformers pass their input through."),
    ("passThroughMeshBoolean", "pmb", "Mesh booleans pass their first input through."),
    ("passThroughPaintEffects", "pp", "Paint effects strokes pass their input through."),
    ("passThroughSculpt", "ps", "Sculpt deformers pass their input through."),
    ("passThroughWire", "pw", "Wire deformers pass their input through."),
    ("regionOfEffect", "roe", "Moving components with a manipulator updates only what changed."),
    ("useClusterResolution", "ucr", "Clusters may be computed at -clusterResolution."),
    ("useLatticeResolution", "ulr", "Lattices may be computed at -latticeResolution."),
];

/// Node types each pass-through option covers.
const PASS_TYPES: &[(&str, &[&str])] = &[
    ("passThroughBindSkinAndFlexors", &["skinCluster", "jointCluster", "jointFfd", "jointLattice"]),
    ("passThroughFlexors", &["jointCluster", "jointFfd", "jointLattice"]),
    ("passThroughBlendShape", &["blendShape"]),
    ("passThroughCluster", &["cluster"]),
    ("passThroughDeltaMush", &["deltaMush"]),
    ("passThroughLattice", &["ffd"]),
    ("passThroughMeshBoolean", &["polyCBoolOp", "polyBoolOp"]),
    ("passThroughPaintEffects", &["stroke"]),
    ("passThroughSculpt", &["sculpt"]),
    ("passThroughWire", &["wire"]),
];

struct State {
    states: BTreeMap<&'static str, &'static str>,
    cluster_resolution: f64,
    lattice_resolution: f64,
    skip_hierarchy: bool,
}

static STATE: RwLock<Option<State>> = RwLock::new(None);
/// Some pass-through option is not off: evaluation looks further.
static ANY_PASS: AtomicBool = AtomicBool::new(false);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut g = STATE.write().unwrap();
    let s = g.get_or_insert_with(|| State { states: STATE_OPTIONS.iter().map(|(n, ..)| (*n, "off")).collect(), cluster_resolution: 0.0, lattice_resolution: 0.0, skip_hierarchy: false });
    f(s)
}

/// An option's state: "on", "off" or "interactive".
pub fn state(option: &str) -> &'static str {
    with(|s| s.states.get(option).copied().unwrap_or("off"))
}

fn active(st: &str) -> bool {
    match st {
        "on" => true,
        "interactive" => crate::anim_time::playing().is_some(),
        _ => false,
    }
}

/// Whether an option is in force now.
pub fn in_force(option: &str) -> bool {
    active(state(option))
}

/// Whether skipHierarchyTraversal is on.
pub fn skip_hierarchy_traversal() -> bool {
    with(|s| s.skip_hierarchy)
}

/// The resolution clusters and lattices are computed at, when their use option is in force; 0 is exact.
pub fn cluster_resolution() -> f64 {
    if in_force("useClusterResolution") {
        with(|s| s.cluster_resolution)
    } else {
        0.0
    }
}

pub fn lattice_resolution() -> f64 {
    if in_force("useLatticeResolution") {
        with(|s| s.lattice_resolution)
    } else {
        0.0
    }
}

/// The option that covers a node type, if any.
fn option_of(ty: &NodeType) -> Option<&'static str> {
    PASS_TYPES.iter().find(|(_, types)| types.iter().any(|t| ty.is_a(t))).map(|(o, _)| *o)
}

fn covered(ty: &NodeType) -> bool {
    PASS_TYPES.iter().any(|(o, types)| types.iter().any(|t| ty.is_a(t)) && in_force(o))
}

/// Before a node computes: a deformer the options pass through hands its input geometry to its output and is not
/// computed. True when it was handled so.
pub fn bypass(d: &mut NodeData) -> bool {
    if !ANY_PASS.load(Ordering::Relaxed) || !covered(&d.ty) {
        return false;
    }
    let ty = d.ty.clone();
    let mut done = false;
    for (inp, out) in [("inputPolymesh", "output"), ("inputCurve", "outputCurve")] {
        if let (Some(i), Some(o)) = (ty.attr_index(inp), ty.attr_index(out)) {
            let v = d.values[i].clone();
            d.values[o] = v;
            done = true;
        }
    }
    // A boolean: its first mesh.
    if !done {
        if let (Some(i), Some(o)) = (ty.attr_index("inputPoly"), ty.attr_index("output")) {
            let first = d.elements.get(i).and_then(|els| els.iter().min_by_key(|(k, _)| *k).map(|(_, v)| v.clone()));
            d.values[o] = first.unwrap_or(PlugValue::Mesh(None));
            done = true;
        }
    }
    done
}

fn parse_state(v: &Value) -> Option<&'static str> {
    match v {
        Value::Bool(true) => Some("on"),
        Value::Bool(false) => Some("off"),
        Value::Number(n) => Some(if n.as_f64().unwrap_or(0.0) != 0.0 { "on" } else { "off" }),
        Value::String(s) => match s.to_ascii_lowercase().as_str() {
            "on" | "true" | "yes" | "1" => Some("on"),
            "off" | "false" | "no" | "0" => Some("off"),
            "interactive" => Some("interactive"),
            _ => None,
        },
        _ => None,
    }
}

fn asked(a: &Args) -> Vec<String> {
    a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn performance_options(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let mut q = asked(a);
        if q.is_empty() {
            q = STATE_OPTIONS.iter().map(|(n, ..)| n.to_string()).chain(["clusterResolution", "latticeResolution", "skipHierarchyTraversal"].map(String::from)).filter(|n| a.get(n).is_some()).collect();
        }
        let Some(f) = q.first() else { fail!("performanceOptions: name the option to query") };
        return Ok(match f.as_str() {
            "clusterResolution" => json!(with(|s| s.cluster_resolution)),
            "latticeResolution" => json!(with(|s| s.lattice_resolution)),
            "skipHierarchyTraversal" => json!(with(|s| s.skip_hierarchy)),
            other => json!(state(other)),
        });
    }
    let mut changed = false;
    for (name, ..) in STATE_OPTIONS {
        let Some(v) = a.get(name).filter(|v| !v.is_null()) else { continue };
        let Some(st) = parse_state(v) else { fail!("performanceOptions: -{name} takes on, off or interactive") };
        with(|s| s.states.insert(name, st));
        changed = true;
    }
    if let Some(v) = a.get("clusterResolution").and_then(Value::as_f64) {
        if !(0.0..=10.0).contains(&v) {
            fail!("performanceOptions: -clusterResolution goes from 0.0 (exact) to 10.0");
        }
        with(|s| s.cluster_resolution = v);
    }
    if let Some(v) = a.get("latticeResolution").and_then(Value::as_f64) {
        if !(0.0..=1.0).contains(&v) {
            fail!("performanceOptions: -latticeResolution goes from 0.0 (exact) to 1.0");
        }
        with(|s| s.lattice_resolution = v);
    }
    if let Some(v) = a.get("skipHierarchyTraversal").filter(|v| !v.is_null()) {
        let on = v.as_bool().unwrap_or(v.as_f64().unwrap_or(0.0) != 0.0);
        with(|s| s.skip_hierarchy = on);
    }
    if changed {
        let any = with(|s| PASS_TYPES.iter().any(|(o, _)| s.states.get(o).is_some_and(|st| *st != "off")));
        ANY_PASS.store(any, Ordering::Relaxed);
        // What the options cover computes again.
        let g = &mut s.graph;
        let hit: Vec<_> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| option_of(n.ty()).is_some())).collect();
        for id in hit {
            g.touch(id);
        }
        g.evaluate();
    }
    Ok(Value::Null)
}

pub fn register(s: &mut Session) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    let mut c = CommandDef::new(
        "performanceOptions",
        "The global performance options: features turned off always (on), never (off) or during playback (interactive). None of them affect rendering. Query: an option's state, or a resolution.",
        performance_options,
    )
    .flag(f("query", "q", ArgType::Bool, "Query mode."))
    .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
    .flag(f("clusterResolution", "cr", ArgType::Float, "The global cluster resolution, 0.0 (exact) to 10.0 (rough)."))
    .flag(f("latticeResolution", "lr", ArgType::Float, "The global lattice resolution, 0.0 (exact) to 1.0 (rough)."))
    .flag(f("skipHierarchyTraversal", "sht", ArgType::Bool, "Invisible objects' hierarchies are not walked."));
    for (n, sh, doc) in STATE_OPTIONS {
        c = c.flag(f(n, sh, ArgType::Any, &format!("{doc} on, off or interactive.")));
    }
    s.register(c);
}
