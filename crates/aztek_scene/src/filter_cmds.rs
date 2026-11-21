//! The reference's data filters and selection filters: `filter` makes the nodes that clean recorded device data
//! (filterEuler, filterResample, filterSimplify, filterClosestSample) before it becomes keys; `filterInstances` drops
//! selected items that are instances of one already selected; `filterStudioImport` holds the studio import options.
//!
//! A filter node takes its data on inputTime, inputData and inputUsage (the record size) and gives the result on
//! outputTime and outputData, as its type's settings say. `run_filter` does the same for a command that has the data
//! in hand.

use std::any::Any;
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::DVec3;
use serde_json::{json, Value};

use crate::node::{types, vec3_attr, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::rig::{data_attr, json_of, JsonData};
use crate::session::Session;

type Cmd = CommandDef<Session>;

pub const FILTER_TYPES: &[&str] = &["filterEuler", "filterResample", "filterSimplify", "filterClosestSample"];

// The base filter's attributes.
const IN_TIME: usize = 0;
const IN_DATA: usize = 1;
const IN_USAGE: usize = 2;
const OUT_TIME: usize = 3;
const OUT_DATA: usize = 4;
const OUT_USAGE: usize = 5;

fn floats(v: &Value) -> Vec<f64> {
    v.as_array().map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
}

/// One filter's settings, read from its node or given to `run_filter`.
#[derive(Clone, Debug)]
pub enum Settings {
    Euler { smooth: i64, nearest_to: i64, initial: Option<DVec3>, weight: DVec3 },
    Resample { kernel: i64, step: f64 },
    Simplify { allow_duplicates: bool, max_step: f64, min_step: f64, time_tolerance: f64, tolerance: f64 },
    ClosestSample { frequency: f64 },
}

/// Filters records of `size` values stamped with `times` (seconds). Returns the new times and data.
pub fn run_filter(settings: &Settings, times: &[f64], data: &[f64], size: usize) -> (Vec<f64>, Vec<f64>) {
    let size = size.max(1);
    let n = times.len().min(data.len() / size);
    let rec = |i: usize| &data[i * size..i * size + size];
    match settings {
        Settings::Euler { smooth, nearest_to, initial, weight } => {
            let mut out: Vec<f64> = data[..n * size].to_vec();
            if size < 3 || *smooth == 0 {
                return (times[..n].to_vec(), out);
            }
            let wrap = |a: f64| (a + PI).rem_euclid(TAU) - PI;
            let mut prev = initial.unwrap_or_else(|| if n > 0 { DVec3::new(out[0], out[1], out[2]) } else { DVec3::ZERO });
            for i in 0..n {
                let e = DVec3::new(out[i * size], out[i * size + 1], out[i * size + 2]);
                let target = match nearest_to {
                    0 => DVec3::ZERO,
                    1 => initial.unwrap_or_else(|| DVec3::new(data[0], data[1], data[2])),
                    _ => prev,
                };
                let unwrap = |e: DVec3| DVec3::new(e.x + TAU * ((target.x - e.x) / TAU).round(), e.y + TAU * ((target.y - e.y) / TAU).round(), e.z + TAU * ((target.z - e.z) / TAU).round());
                let r = match smooth {
                    1 => DVec3::new(wrap(e.x), wrap(e.y), wrap(e.z)),
                    2 => unwrap(e),
                    _ => {
                        // The same rotation reached the other way round the middle axis.
                        let a = unwrap(e);
                        let b = unwrap(DVec3::new(e.x + PI, PI - e.y, e.z + PI));
                        let dist = |x: DVec3| ((x - target) * *weight).length_squared();
                        if dist(b) + 1e-12 < dist(a) {
                            b
                        } else {
                            a
                        }
                    }
                };
                out[i * size] = r.x;
                out[i * size + 1] = r.y;
                out[i * size + 2] = r.z;
                prev = r;
            }
            (times[..n].to_vec(), out)
        }
        Settings::Resample { kernel, step } => {
            if *kernel == 100 || n < 2 {
                return (times[..n].to_vec(), data[..n * size].to_vec());
            }
            let step = step.max(1e-9);
            let (t0, t1) = (times[0], times[n - 1]);
            let count = ((t1 - t0) / step).floor() as usize + 1;
            let mut ot = Vec::with_capacity(count);
            let mut od = Vec::with_capacity(count * size);
            for k in 0..count {
                let t = t0 + k as f64 * step;
                ot.push(t);
                let j = times[..n].partition_point(|x| *x < t).min(n - 1);
                let closest = if j > 0 && (t - times[j - 1]).abs() <= (times[j] - t).abs() { j - 1 } else { j };
                for c in 0..size {
                    let v = match kernel {
                        200 => rec(closest)[c],
                        300 => {
                            let (a, b) = (j.saturating_sub(1), j);
                            if a == b || (times[b] - times[a]).abs() < 1e-12 {
                                rec(b)[c]
                            } else {
                                let s = ((t - times[a]) / (times[b] - times[a])).clamp(0.0, 1.0);
                                rec(a)[c] + (rec(b)[c] - rec(a)[c]) * s
                            }
                        }
                        _ => {
                            // Box, tent and the two gaussians weigh the samples within a step of t (x in steps).
                            let w = |x: f64| -> f64 {
                                match kernel {
                                    400 => (x.abs() <= 0.5) as i32 as f64,
                                    500 => (1.0 - x.abs()).max(0.0),
                                    600 => 2f64.powf(-2.0 * x * x),
                                    _ => 2f64.powf(-4.0 * x * x),
                                }
                            };
                            let reach = if *kernel == 400 { 0.5 } else { 2.0 };
                            let (mut sum, mut ws) = (0.0, 0.0);
                            for i in 0..n {
                                let x = (times[i] - t) / step;
                                if x.abs() <= reach {
                                    let wi = w(x);
                                    sum += rec(i)[c] * wi;
                                    ws += wi;
                                }
                            }
                            if ws > 0.0 {
                                sum / ws
                            } else {
                                rec(closest)[c]
                            }
                        }
                    };
                    od.push(v);
                }
            }
            (ot, od)
        }
        Settings::ClosestSample { frequency } => {
            let step = 1.0 / frequency.max(1e-9);
            run_filter(&Settings::Resample { kernel: 200, step }, &times[..n], &data[..n * size], size)
        }
        Settings::Simplify { allow_duplicates, max_step, min_step, time_tolerance, tolerance } => {
            if n < 3 {
                return (times[..n].to_vec(), data[..n * size].to_vec());
            }
            // Each value's range sets how close to a line counts as on it.
            let mut range = vec![0.0; size];
            for (c, r) in range.iter_mut().enumerate() {
                let (lo, hi) = (0..n).map(|i| rec(i)[c]).fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(v), b.max(v)));
                *r = (hi - lo).max(1e-12) * tolerance;
            }
            let mut keep = vec![0usize];
            for i in 1..n {
                let last = *keep.last().unwrap();
                if times[i] - times[last] <= *time_tolerance || times[i] - times[last] < *min_step {
                    continue;
                }
                if !allow_duplicates && rec(i) == rec(last) {
                    continue;
                }
                if i + 1 == n || times[i + 1] - times[last] > *max_step {
                    keep.push(i);
                    continue;
                }
                // Dropped when the next record still lies on the line from the last kept one through this one.
                let next = i + 1;
                let s = (times[i] - times[last]) / (times[next] - times[last]).max(1e-12);
                let on_line = (0..size).all(|c| (rec(last)[c] + (rec(next)[c] - rec(last)[c]) * s - rec(i)[c]).abs() <= range[c]);
                if !on_line {
                    keep.push(i);
                }
            }
            let ot = keep.iter().map(|&i| times[i]).collect();
            let od = keep.iter().flat_map(|&i| rec(i).to_vec()).collect();
            (ot, od)
        }
    }
}

fn settings_of(ty: &str, d: &NodeData) -> Settings {
    let t = types::find(ty);
    let at = |n: &str| t.as_ref().and_then(|t| t.attr_index(n)).unwrap_or(0);
    match ty {
        "filterEuler" => Settings::Euler {
            nearest_to: d.int(at("nearestTo")),
            initial: d.bool(at("enableInitialRotation")).then(|| d.vec3(at("initialRotation"))),
            smooth: d.int(at("smooth")),
            weight: d.vec3(at("weight")),
        },
        "filterResample" => {
            let s = d.float(at("timeStep"));
            Settings::Resample { kernel: d.int(at("kernel")), step: if s > 0.0 { s / crate::anim::fps() } else { 1.0 / crate::anim::fps() } }
        }
        "filterSimplify" => Settings::Simplify {
            allow_duplicates: d.bool(at("allowDuplicates")),
            max_step: d.float(at("maxTimeStep")),
            min_step: d.float(at("minTimeStep")),
            time_tolerance: d.float(at("timeTolerance")),
            tolerance: d.float(at("tolerance")),
        },
        _ => Settings::ClosestSample { frequency: d.float(at("frequency")) },
    }
}

struct FilterNode(&'static str);

impl NodeBehavior for FilterNode {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        let times = floats(&json_of(d, IN_TIME));
        let data = floats(&json_of(d, IN_DATA));
        let usage = d.int(IN_USAGE);
        let size = if usage > 0 { usage as usize } else if self.0 == "filterEuler" { 3 } else { 1 };
        let (t, v) = run_filter(&settings_of(self.0, d), &times, &data, size);
        d.set_output(OUT_TIME, PlugValue::Data(Some(Arc::new(JsonData(json!(t))))));
        d.set_output(OUT_DATA, PlugValue::Data(Some(Arc::new(JsonData(json!(v))))));
        let ou = d.int(OUT_USAGE);
        d.set_output(OUT_USAGE, PlugValue::Int(if ou == -1 { usage } else { ou }));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn base_attrs() -> Vec<AttrDef> {
    let int = |name: &str, short: &str, fl: u32, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(-1), flags: fl, doc: doc.into(), ..Default::default() };
    let out_data = |name: &str, short: &str, doc: &str| {
        let mut a = data_attr(name, short, doc);
        a.flags = flags::OUTPUT | flags::HIDDEN;
        a
    };
    vec![
        data_attr("inputTime", "it", "Timestamps of the data coming in, seconds."),
        data_attr("inputData", "id", "The data coming in."),
        int("inputUsage", "iu", flags::INPUT | flags::HIDDEN, "The kind of data coming in: how many values make a record."),
        out_data("outputTime", "ot", "Timestamps of the filtered data."),
        out_data("outputData", "od", "The filtered data."),
        int("outputUsage", "ou", flags::OUTPUT | flags::HIDDEN, "How many values make a record going out; -1 is the input's."),
        AttrDef { name: "deviceEditorActive".into(), short_name: "dea".into(), ty: PlugType::Bool, default_value: PlugValue::Bool(false), flags: flags::INPUT | flags::STORABLE, doc: "Whether the filter is active in the device editor.".into(), ..Default::default() },
    ]
}

fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let io = flags::INPUT | flags::STORABLE;
        let b = |name: &str, short: &str, d: bool, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Bool, default_value: PlugValue::Bool(d), flags: io, doc: doc.into(), ..Default::default() };
        let fl = |name: &str, short: &str, d: f64, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Float, default_value: PlugValue::Float(d), flags: io, doc: doc.into(), ..Default::default() };
        let en = |name: &str, short: &str, d: i64, names: &[&str], fl: u32, doc: &str| AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Enum, default_value: PlugValue::Int(d), flags: fl, enum_names: names.iter().map(|x| x.to_string()).collect(), doc: doc.into(), ..Default::default() };
        types::register_type(NodeType { name: "filter".into(), doc: "The base of the device data filters.".into(), own_attrs: base_attrs(), create: Some(Arc::new(|| Box::new(crate::node::Plain))), ..Default::default() }).expect("filter");
        let base = types::find("filter").unwrap();
        let mut initial = vec3_attr("initialRotation", "ir", DVec3::ZERO, io, "The rotation the first comparisons are made against, with enableInitialRotation.");
        initial.unit = aztek_ops::units::Unit::Angle;
        let weight = vec3_attr("weight", "w", DVec3::ONE, io, "How much each axis counts when judging closeness.");
        let mut euler_attrs = base_attrs();
        euler_attrs.clear();
        euler_attrs.extend([
            en("nearestTo", "nt", 2, &["zero", "initialValue", "previousValue"], io, "What the cut or solution chosen is nearest to."),
            initial,
            en("smooth", "s", 3, &["none", "bound", "nearestCut", "nearestSolution"], io, "No smoothing, bound to +-180, the nearest cut, or the nearest Euler solution."),
            en("outputUsageOpt", "use", 0, &["default"], io | flags::HIDDEN, "For the reference's own use."),
            b("enableInitialRotation", "eir", false, "Compare against initialRotation rather than the first sample."),
            weight,
        ]);
        let made: [(&'static str, Vec<AttrDef>, &str); 4] = [
            ("filterEuler", euler_attrs, "Euler angle demangler for recorded rotations."),
            (
                "filterResample",
                vec![
                    en("kernel", "kr", 600, &[], io, "100 passes through, 200 closest sample, 300 linear, 400 box, 500 tent, 600 gaussian2, 700 gaussian4."),
                    fl("timeStep", "ts", 0.0, "The output period in frames; 0 is one frame."),
                ],
                "Resamples data at a fixed rate.",
            ),
            (
                "filterSimplify",
                vec![
                    b("allowDuplicates", "ad", true, "Runs of the same value count as data."),
                    fl("maxTimeStep", "mxt", 3600.0, "The longest gap between records out, seconds."),
                    fl("minTimeStep", "mnt", 0.0, "The shortest gap between records out, seconds."),
                    fl("timeTolerance", "tt", 0.0, "Records this close to the last one out are skipped, seconds."),
                    fl("tolerance", "tol", 0.001, "The fraction of the data's range within which records count as on a line."),
                ],
                "Joins records that lie nearly on a line.",
            ),
            ("filterClosestSample", vec![fl("frequency", "f", 30.0, "Records out per second.")], "Resamples data at a fixed rate from the closest sample."),
        ];
        for (name, own, doc) in made {
            let mut own = own;
            if name == "filterResample" {
                own[0].enum_names = Vec::new();
                own[0].ty = PlugType::Int;
            }
            types::register_type(NodeType { name: name.into(), base: Some(base.clone()), doc: doc.into(), own_attrs: own, create: Some(Arc::new(move || Box::new(FilterNode(name)))), ..Default::default() }).expect(name);
        }
    });
}

/// The filter node's own attributes, as flags of the command (the reference's example sets -irx on one).
fn attr_flags() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for t in FILTER_TYPES {
        let Some(ty) = types::find(t) else { continue };
        for a in &ty.own_attrs {
            out.push((a.name.clone(), a.short_name.clone()));
            if a.ty == PlugType::Vec3 {
                for i in 0..3 {
                    out.push((a.child_names[i].clone(), a.child_short[i].clone()));
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|(l, _)| seen.insert(l.clone()));
    out
}

fn filter_cmd(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let g = &mut s.graph;
    let attrs = attr_flags();
    if a.bool("query") || a.bool("edit") {
        let Some(name) = names.first() else { fail!("filter: name the filter") };
        let Some(id) = g.find(name)? else { fail!("filter: no object named {name}") };
        let Some(ty) = g.node(id).map(|n| n.ty().name.clone()).filter(|t| FILTER_TYPES.contains(&t.as_str())) else { fail!("filter: {name} is not a filter") };
        let queried: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let asked = |f: &str| queried.iter().any(|x| x == f) || (a.bool("query") && a.get(f) == Some(&json!(true)));
        if a.bool("query") {
            if asked("type") {
                return Ok(json!(ty));
            }
            if asked("name") {
                return Ok(json!(g.path(id)));
            }
            for (long, _) in &attrs {
                if asked(long) {
                    let p = g.plug(&format!("{}.{long}", g.path(id)))?;
                    let unit = g.attr(&p).unit;
                    let v = g.value(&p)?;
                    let u = |x: f64| aztek_ops::units::to_user(unit, x);
                    return Ok(match v {
                        PlugValue::Vec3(x) if p.component < 0 => json!([u(x.x), u(x.y), u(x.z)]),
                        PlugValue::Vec3(x) => json!(u(x[p.component as usize])),
                        PlugValue::Bool(b) => json!(b),
                        PlugValue::Int(i) => json!(i),
                        PlugValue::Float(x) => json!(u(x)),
                        _ => Value::Null,
                    });
                }
            }
            return Ok(json!(g.path(id)));
        }
        if let Some(n) = a.get("name").and_then(Value::as_str) {
            g.rename(id, n)?;
        }
        if let Some(t) = a.get("type").and_then(Value::as_str) {
            if t != ty {
                fail!("filter: a filter's type can't be changed ({} is a {ty})", g.path(id));
            }
        }
        set_attrs(g, id, a, &attrs)?;
        return Ok(json!(g.path(id)));
    }
    let ty = a.string("type", "filterEuler");
    if !FILTER_TYPES.contains(&ty.as_str()) {
        fail!("filter: unknown filter type {ty}: one of {}", FILTER_TYPES.join(", "));
    }
    let t = types::find(&ty).unwrap();
    let wanted = a.get("name").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| format!("{ty}1"));
    g.open_chunk("filter");
    let r = (|| -> CmdResult<NodeId> {
        let id = g.create_node(&t, &wanted, NO_NODE, true)?;
        set_attrs(g, id, a, &attrs)?;
        Ok(id)
    })();
    g.close_chunk();
    let id = r?;
    Ok(json!(g.path(id)))
}

fn set_attrs(g: &mut crate::graph::Graph, id: NodeId, a: &Args, attrs: &[(String, String)]) -> CmdResult<()> {
    for (long, _) in attrs {
        let Some(v) = a.get(long) else { continue };
        let Ok(p) = g.plug(&format!("{}.{long}", g.path(id))) else { continue };
        let k = aztek_ops::units::factor(g.attr(&p).unit);
        let num = |x: Option<&Value>| x.and_then(|y| y.as_f64().or_else(|| y.as_str().and_then(|s| s.parse().ok()))).unwrap_or(0.0);
        let pv = match (g.attr(&p).ty, v) {
            (PlugType::Vec3, Value::Array(x)) if p.component < 0 => PlugValue::Vec3(DVec3::new(num(x.first()), num(x.get(1)), num(x.get(2))) * k),
            (PlugType::Vec3, v) => PlugValue::Float(num(Some(v)) * k),
            (PlugType::Bool, v) => PlugValue::Bool(v.as_bool().unwrap_or_else(|| num(Some(v)) != 0.0)),
            (PlugType::Int | PlugType::Enum, v) => PlugValue::Int(num(Some(v)) as i64),
            (_, v) => PlugValue::Float(num(Some(v)) * k),
        };
        g.set_value(p, pv)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
// filterInstances

/// What makes two selected items the same: the shape an instance shares (its root), with the component part.
fn instance_key(g: &crate::graph::Graph, item: &str, shapes: bool) -> String {
    let (node, rest) = match item.find('.') {
        Some(i) => (&item[..i], &item[i..]),
        None => (item, ""),
    };
    let Ok(Some(id)) = g.find(node) else { return item.to_string() };
    let is_transform = g.node(id).is_some_and(|n| n.ty().is_a("transform"));
    // Components on a transform are its shape's; a transform alone counts by its shape only with -shapes.
    let shape = if !rest.is_empty() || !is_transform || shapes { crate::dag::shape_of_type(g, id, "mesh").or(if is_transform { None } else { Some(id) }) } else { None };
    match shape {
        Some(sh) if g.node(sh).is_some_and(|n| n.ty().is_a("mesh")) => format!("#{}{rest}", crate::instancing::root(g, sh)),
        _ => format!("#{id}{rest}"),
    }
}

fn filter_instances(s: &mut Session, a: &Args) -> CmdResult {
    let items: Vec<String> = match s.run("ls", json!({"selection": true, "flatten": false}))? {
        Value::Array(v) => v.into_iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    let shapes = a.bool("shapes");
    let mut seen: Vec<String> = Vec::new();
    let mut kept: Vec<String> = Vec::new();
    for it in &items {
        let k = instance_key(&s.graph, it, shapes);
        if !seen.contains(&k) {
            seen.push(k);
            kept.push(it.clone());
        }
    }
    if a.bool("query") {
        return Ok(json!(kept.len() < items.len()));
    }
    if kept.len() < items.len() {
        s.run("select", json!({"args": kept.clone(), "replace": true}))?;
    }
    Ok(json!(kept))
}

// ---------------------------------------------------------------------------------------------------------------------

pub fn register(c: &mut Commands<Session>) {
    register_types();
    let f = |name: &str, short: &str, ty: ArgType, doc: &str| FlagDef::new(name, short, ty, Value::Null, doc);
    let mut cmd = Cmd::new(
        "filter",
        "Make a filter node for recorded device data (-t filterEuler, filterResample, filterSimplify or filterClosestSample; -n its name), \
         or with -e and -q change and read it. The node's attributes are flags too. Returns its name.",
        filter_cmd,
    )
    .positional("the filter, with -e or -q")
    .flag(f("name", "n", ArgType::String, "The filter's name."))
    .flag(f("type", "t", ArgType::String, "The kind of filter (filterEuler)."))
    .flag(f("query", "q", ArgType::Bool, "Read."))
    .flag(f("edit", "e", ArgType::Bool, "Change."))
    .flag(f("queried", "", ArgType::List, "What -q asks for."));
    for (long, short) in attr_flags() {
        cmd = cmd.flag(f(&long, &short, ArgType::Any, "The filter node's attribute."));
    }
    c.register(cmd);

    c.register(
        Cmd::new(
            "filterInstances",
            "Drop selected items that are instances of one selected before them (the same shape or the same components). \
             -s counts transforms whose shapes are instances. Returns the selection left; -q whether any were instances.",
            filter_instances,
        )
        .flag(f("shapes", "s", ArgType::Bool, "Look at the shapes below selected transforms to tell whether they are instances."))
        .flag(f("query", "q", ArgType::Bool, "Whether any selected item is an instance of another.")),
    );

    c.register(
        Cmd::new(
            "filterStudioImport",
            "The studio import options, set without the options window: -ic cameras, -il lights, -sp shells as meshes, -td where its binary files go.",
            |s, a| {
                let map: &mut BTreeMap<String, Value> = &mut s.extras.studio_import;
                for k in ["includeCameras", "includeLights", "convertShellToPoly", "transferDirectoryName"] {
                    if let Some(v) = a.get(k) {
                        map.insert(k.to_string(), v.clone());
                    }
                }
                // An empty transfer directory is the temporary folder.
                if map.get("transferDirectoryName").and_then(Value::as_str).is_some_and(str::is_empty) {
                    map.insert("transferDirectoryName".into(), json!(std::env::temp_dir().to_string_lossy()));
                }
                Ok(Value::Null)
            },
        )
        .undoable(false)
        .flag(f("includeCameras", "ic", ArgType::Bool, "Translate the cameras."))
        .flag(f("includeLights", "il", ArgType::Bool, "Translate the lights."))
        .flag(f("convertShellToPoly", "sp", ArgType::Bool, "Shells become triangle meshes rather than groups of surfaces."))
        .flag(f("transferDirectoryName", "td", ArgType::String, "Where embedded binary files go; empty is the temporary folder.")),
    );
}
