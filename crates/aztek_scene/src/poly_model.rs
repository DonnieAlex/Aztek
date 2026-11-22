//! Polygon commands of the reference from polyExtrudeFacet to polySmooth that lacked their query and edit modes or some
//! flags: query and edit of each command's history node, UV nodes that make their edit again when their settings
//! change, polySmartExtrude and polySelectConstraintMonitor.

use std::collections::BTreeSet;

use aztek_geo::{Index, Mesh};
use aztek_ops::{fail, ArgType, Args, CmdResult, Commands};
use glam::DVec2;
use serde_json::{json, Value};

use crate::history_qe::{f, Cmd};
use crate::node::NodeData;
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue};
use crate::session::Session;
use crate::uv_ops::{self, Target, UvTopo};

/// Each command and the node types its history node may be.
const HISTORY: &[(&str, &[&str])] = &[
    ("polyExtrudeVertex", &["polyExtrudeVertex"]),
    ("polyHelix", &["polyHelix"]),
    ("polyMapDel", &["polyMapDel"]),
    ("polyMergeEdge", &["polyMergeEdge"]),
    ("polyMergeFacet", &["polyMergeFace"]),
    ("polyMergeVertex", &["polyMergeVert"]),
    ("polyMirrorFace", &["polyMirror"]),
    ("polyMoveEdge", &["polyMoveEdge"]),
    ("polyMoveVertex", &["polyMoveVertex"]),
    ("polyNormal", &["polyNormal"]),
    ("polyPipe", &["polyPipe"]),
    ("polyPlane", &["polyPlane"]),
    ("polyPlatonicSolid", &["polyPlatonicSolid"]),
    ("polyPoke", &["polyPoke"]),
    ("polyPrimitive", &["polyPrimitive"]),
    ("polyQuad", &["polyQuad"]),
    ("polyReduce", &["polyReduce"]),
    ("polyRemesh", &["polyRemesh"]),
    ("polySeparate", &["polySeparate"]),
    ("polySmooth", &["polySmoothFace", "polySmoothProxy"]),
];

/// The magnet flags' attributes: the X, Y and Z of the node's magnet vector.
const MAGNET_MAP: &[(&str, &str)] = &[("magnX", "magnetX"), ("magnY", "magnetY"), ("magnZ", "magnetZ")];

pub fn register(c: &mut Commands<Session>) {
    for &(name, types) in HISTORY {
        crate::history_qe::extend(c, name, types, &[], Vec::new());
    }
    crate::history_qe::extend(c, "polyExtrudeFacet", &["polyExtrudeFace"], MAGNET_MAP, Vec::new());
    crate::history_qe::extend(c, "polyMoveFacet", &["polyMoveFace"], MAGNET_MAP, Vec::new());
    for (name, types) in [("polySewEdge", &["polySewEdge"][..]), ("polySmartExtrude", &["polySmartExtrude"][..])] {
        crate::history_qe::extend(c, name, types, &[], Vec::new());
    }
    for (name, types) in [("polyPrism", &["polyPrism"][..]), ("polyPyramid", &["polyPyramid"][..])] {
        crate::history_qe::extend(c, name, types, NSI_MAP, Vec::new());
    }
    register_live_uv(c);
    planar(c);
    hole(c);
    pin_uv(c);
    select_query(c);
    poly_output(c);
    geo_sampler(c);
    multi_layout(c);
    project_curve_cmd(c);
    for name in ["polyFlipUV", "polyLayoutUV", "polyMergeUV", "polyMoveUV", "polyMoveFacetUV", "polyNormalizeUV", "polyMapCut", "polyMapSew", "polyMapSewMove", "polyOptUvs", "polyMultiLayoutUV", "polyPinUV", "polyStraightenUVBorder"] {
        crate::poly_extra::uv_set_wrapped(c, name);
    }
}

/// The reference's misspelt second name of numberOfSides.
const NSI_MAP: &[(&str, &str)] = &[("numderOfSides", "numberOfSides")];

// ---------------------------------------------------------------------------------------------------------------------
// UV nodes that make their edit again from their settings

/// A setting a UV node keeps: its name and short name (the command's flags), 'b' 'i' 'f' or 's', its default, and
/// what it does.
type Setting = (&'static str, &'static str, char, f64, &'static str);

/// What a UV command does to a mesh, from its settings: the same for the command and for the node made again.
type LiveFn = fn(&Args, &mut Mesh, &Target, &UvTopo);

const FLIP: &[Setting] = &[
    ("flipType", "ft", 'i', 0.0, "0 flips in U (horizontally), 1 in V (vertically)."),
    ("local", "l", 'b', 1.0, "On flips about the middle of the UVs flipped; off about the middle of 0 to 1."),
    ("usePivot", "up", 'b', 0.0, "Flip about pivotU, pivotV."),
    ("pivotU", "pu", 'f', 0.0, "The pivot's U, with usePivot."),
    ("pivotV", "pv", 'f', 0.0, "The pivot's V, with usePivot."),
    ("cutUV", "cut", 'b', 1.0, "Cut the UVs round faces flipped apart from the rest of their shell."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];
const LAYOUT: &[Setting] = &[
    ("flipReversed", "fr", 'b', 1.0, "Mirror pieces whose UVs face backwards."),
    ("layout", "l", 'i', 2.0, "How pieces move: 0 not at all, 1 along U, 2 into a square."),
    ("layoutMethod", "lm", 'i', 0.0, "0 block stacking, 1 shape stacking."),
    ("percentageSpace", "ps", 'f', 0.2, "Space round each piece, percent of the square (5 at most)."),
    ("rotateForBestFit", "rbf", 'i', 1.0, "0 no turning, 1 by right angles, 2 freely."),
    ("scale", "sc", 'i', 1.0, "After: 0 no scaling, 1 uniform into the unit square, 2 each way into it."),
    ("separate", "se", 'i', 0.0, "Cuts: 0 none, 1 along folds, 2 all that avoid overlaps."),
    ("worldSpace", "ws", 'b', 0.0, "Pieces sized by world-space areas rather than the object's."),
];
const MERGE: &[Setting] = &[
    ("distance", "d", 'f', 0.0, "UVs of a vertex closer than this become one."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];
const MOVE: &[Setting] = &[
    ("translateU", "tu", 'f', 0.0, "U offset."),
    ("translateV", "tv", 'f', 0.0, "V offset."),
    ("scaleU", "su", 'f', 1.0, "U scale about the pivot."),
    ("scaleV", "sv", 'f', 1.0, "V scale about the pivot."),
    ("pivotU", "pvu", 'f', 0.5, "The pivot's U."),
    ("pivotV", "pvv", 'f', 0.5, "The pivot's V."),
    ("rotationAngle", "ra", 'f', 0.0, "Turn about the pivot, degrees counterclockwise."),
    ("random", "ran", 'f', 0.0, "Each UV moves on by up to this much more, at random."),
    ("axisLenX", "lx", 'f', 1.0, "The manipulator's U handle length."),
    ("axisLenY", "ly", 'f', 1.0, "The manipulator's V handle length."),
];
const NORMALIZE: &[Setting] = &[
    ("normalizeType", "nt", 'i', 1.0, "0 each shell on its own, 1 all together."),
    ("normalizeDirection", "nd", 'i', 0.0, "0 along U and V, 1 along U alone, 2 along V alone."),
    ("preserveAspectRatio", "pa", 'b', 1.0, "Scale U and V alike."),
    ("centerOnTile", "cot", 'b', 0.0, "Centre on the tile the UVs are mostly over, rather than on 0 to 1."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];
const STRAIGHTEN: &[Setting] = &[
    ("blendOriginal", "bo", 'f', 0.0, "0 fits the UVs to the line, 1 leaves them where they were."),
    ("curvature", "c", 'f', 0.0, "How far the line bows: at 1 its middle stands half the run's length off the straight line."),
    ("preserveLength", "pl", 'f', 1.0, "1 keeps the UVs' spacing along the line, 0 spaces them evenly."),
    ("gapTolerance", "gt", 'i', 0.0, "Gaps of up to this many UVs between the UVs picked are filled."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];
const MAP_CUT: &[Setting] = &[("moveratio", "mvr", 'f', 0.0, "How far the cut opens: a share of the way from each cut edge to the middle of its face.")];
const MAP_SEW: &[Setting] = &[];
const SEW_MOVE: &[Setting] = &[
    ("limitPieceSize", "lps", 'b', 0.0, "Refuse to move pieces of more than numberFaces faces."),
    ("numberFaces", "nf", 'i', 10.0, "With limitPieceSize: the most faces a piece moved may have."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];
const OPT: &[Setting] = &[
    ("iterations", "i", 'i', 100.0, "The most solver steps per piece."),
    ("stoppingThreshold", "ss", 'f', 0.1, "Stop once a step improves the distortion by less than this percent."),
    ("useScale", "us", 'b', 0.0, "Keep the UVs at scale times their 3D size."),
    ("scale", "s", 'f', 1.0, "With useScale: UV size over 3D size."),
    ("areaWeight", "aw", 'f', 0.0, "0 every face counts the same, 1 larger faces count more."),
    ("globalBlend", "gb", 'f', 0.0, "0 the local method, 1 the global one."),
    ("globalMethodBlend", "gmb", 'f', 0.5, "In the global method: toward 1 edge lengths count more, toward 0 areas."),
    ("optimizeAxis", "oa", 'i', 0.0, "0 move freely, 1 only in V, 2 only in U."),
    ("pinSelected", "ps", 'b', 0.0, "Hold the UVs named rather than the others."),
    ("pinUvBorder", "pub", 'b', 0.0, "Hold the pieces' borders."),
    ("applyToShell", "as", 'b', 0.0, "Work on the whole pieces of the UVs named."),
    ("worldSpace", "ws", 'b', 0.0, "World-space values rather than the object's; UVs are the same either way."),
];

/// Node type, its settings, what makes the edit.
const LIVE: &[(&str, &[Setting], LiveFn)] = &[
    ("polyFlipUV", FLIP, flip_uv),
    ("polyLayoutUV", LAYOUT, layout_uv),
    ("polyMergeUV", MERGE, merge_uv),
    ("polyMoveUV", MOVE, move_uv),
    ("polyNormalizeUV", NORMALIZE, normalize_uv),
    ("polyMapCut", MAP_CUT, map_cut),
    ("polyMapSew", MAP_SEW, map_sew),
    ("polyMapSewMove", SEW_MOVE, map_sew_move),
    ("polyOptUvs", OPT, opt_uvs),
    ("polyStraightenUVBorder", STRAIGHTEN, crate::uv_misc::straighten_uvs),
];

/// Commands, the node type each makes, and the undo label.
const LIVE_CMDS: &[(&str, &str, &str)] = &[
    ("polyFlipUV", "polyFlipUV", "Flip UVs"),
    ("polyLayoutUV", "polyLayoutUV", "Layout"),
    ("polyMergeUV", "polyMergeUV", "Merge UVs"),
    ("polyMoveUV", "polyMoveUV", "Move UVs"),
    ("polyMoveFacetUV", "polyMoveUV", "Move UVs"),
    ("polyNormalizeUV", "polyNormalizeUV", "Normalize UVs"),
    ("polyMapCut", "polyMapCut", "Cut UV Edges"),
    ("polyMapSew", "polyMapSew", "Sew UV Edges"),
    ("polyMapSewMove", "polyMapSewMove", "Move and Sew UVs"),
    ("polyOptUvs", "polyOptUvs", "Optimize UVs"),
    ("polyStraightenUVBorder", "polyStraightenUVBorder", "Straighten UV Border"),
];

/// Flags of two values and the settings they stand for.
const PAIRS: &[(&str, &str, &str)] = &[("translate", "translateU", "translateV"), ("scale", "scaleU", "scaleV"), ("pivot", "pivotU", "pivotV"), ("axisLen", "axisLenX", "axisLenY")];

fn live(node_type: &str) -> Option<&'static (&'static str, &'static [Setting], LiveFn)> {
    LIVE.iter().find(|l| l.0 == node_type)
}

/// The attributes a UV node of this type adds for its settings: the settings, the components and the state they were
/// last made in.
pub(crate) fn live_uv_attrs(node_type: &str) -> Vec<AttrDef> {
    let Some(&(_, settings, _)) = live(node_type) else { return Vec::new() };
    let k = flags::INPUT | flags::STORABLE;
    let mut out: Vec<AttrDef> = settings
        .iter()
        .map(|&(name, short, kind, def, doc)| match kind {
            'b' => AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Bool, default_value: PlugValue::Bool(def != 0.0), flags: k, doc: doc.into(), ..Default::default() },
            'i' => AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::Int, default_value: PlugValue::Int(def as i64), flags: k | flags::KEYABLE, doc: doc.into(), ..Default::default() },
            's' => AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: k, doc: doc.into(), ..Default::default() },
            _ => crate::attr_defs::float_attr_short(name, short, def, doc, None, None),
        })
        .collect();
    for (name, short, doc) in [
        ("inputComponents", "ics", "The components the edit works on: f[0:3] e[5] map[2], * for the whole mesh."),
        ("settingsState", "sst", "The settings the stored UVs were made with: while they match, the stored UVs stand."),
    ] {
        out.push(AttrDef { name: name.into(), short_name: short.into(), ty: PlugType::String, default_value: PlugValue::String(String::new()), flags: k | flags::HIDDEN, doc: doc.into(), ..Default::default() });
    }
    out
}

/// The settings a node holds, as the arguments its edit runs with.
fn settings_of(d: &NodeData, settings: &[Setting]) -> Args {
    let mut a = Args::default();
    for &(name, _, kind, _, _) in settings {
        let Some(i) = d.ty().attr_index(name) else { continue };
        let v = match kind {
            'b' => json!(d.bool(i)),
            'i' => json!(d.int(i)),
            's' => json!(d.string(i)),
            _ => json!(d.float(i)),
        };
        a.flags.insert(name.into(), v);
    }
    a
}

fn state_of(a: &Args) -> String {
    serde_json::to_string(&a.flags).unwrap_or_default()
}

fn encode_target(t: &Target) -> String {
    if t.whole {
        return "*".into();
    }
    use crate::components::ComponentMode as K;
    let mut parts = Vec::new();
    for (kind, ids) in [(K::Face, &t.faces), (K::Edge, &t.edges), (K::Uv, &t.maps)] {
        if !ids.is_empty() {
            parts.push(crate::history::format_components(kind, ids));
        }
    }
    parts.join(" ")
}

fn decode_target(s: &str) -> Target {
    let mut t = Target::default();
    if s.trim() == "*" || s.trim().is_empty() {
        t.whole = true;
        return t;
    }
    for tok in s.split_whitespace() {
        let ids = crate::history::parse_components(tok);
        if tok.starts_with("f[") {
            t.faces.extend(ids);
        } else if tok.starts_with("e[") {
            t.edges.extend(ids);
        } else if tok.starts_with("map[") {
            t.maps.extend(ids);
        }
    }
    t
}

/// A UV node's mesh made again from its settings, when they differ from those its stored UVs were made with.
pub(crate) fn live_uv(d: &NodeData, input: &Mesh) -> Option<Mesh> {
    let &(_, settings, run) = live(&d.ty().name)?;
    let state = d.string(d.ty().attr_index("settingsState")?);
    if state.is_empty() {
        return None;
    }
    let a = settings_of(d, settings);
    if state_of(&a) == state {
        return None;
    }
    let t = decode_target(d.string(d.ty().attr_index("inputComponents")?));
    let mut m = input.clone();
    let tp = uv_ops::topo(&m);
    run(&a, &mut m, &t, &tp);
    Some(m)
}

/// The settings a call gives, with the defaults for the rest. Two-value flags give both settings.
fn call_settings(a: &Args, settings: &[Setting]) -> Args {
    let mut out = Args::default();
    for &(name, _, kind, def, _) in settings {
        let mut v = a.get(name).cloned().filter(|v| !v.is_null());
        if v.is_none() {
            for &(pair, x, y) in PAIRS {
                if name == x || name == y {
                    if let Some(Value::Array(l)) = a.get(pair) {
                        v = l.get(usize::from(name == y)).cloned();
                    }
                }
            }
        }
        let v = match (kind, v) {
            ('b', Some(x)) => json!(x.as_bool().unwrap_or(x.as_f64().unwrap_or(0.0) != 0.0)),
            ('b', None) => json!(def != 0.0),
            ('i', Some(x)) => json!(x.as_i64().unwrap_or(x.as_f64().unwrap_or(def) as i64)),
            ('i', None) => json!(def as i64),
            ('s', Some(x)) => json!(x.as_str().unwrap_or("")),
            ('s', None) => json!(""),
            (_, Some(x)) => json!(x.as_f64().unwrap_or(def)),
            (_, None) => json!(def),
        };
        out.flags.insert(name.into(), v);
    }
    out
}

/// Makes the UV edit on each mesh the call names or has selected: a node holding the settings in each mesh's history.
/// Returns the nodes.
fn each_live(s: &mut Session, a: &Args, cmd: &str, label: &str, node_type: &str) -> CmdResult {
    let Some(&(_, settings, run)) = live(node_type) else { fail!("{cmd}: no edit for {node_type}") };
    let targets = uv_ops::targets(s, a, cmd)?;
    let set = call_settings(a, settings);
    let g = &mut s.graph;
    g.open_chunk(label);
    let mut made = Vec::new();
    let r = (|| -> CmdResult<()> {
        for (mesh, t) in targets {
            g.evaluate();
            let Some(cage) = crate::modifiers::cage(g, mesh) else { fail!("{cmd}: {} is not a mesh", g.path(mesh)) };
            let before = (*cage).clone();
            let mut after = before.clone();
            let tp = uv_ops::topo(&after);
            run(&set, &mut after, &t, &tp);
            let edit = uv_ops::UvEdit::diff(&before, &after);
            let Some(h) = uv_ops::record_edit(g, mesh, node_type, edit)? else { continue };
            for (k, v) in &set.flags {
                let Some(i) = g.node(h).and_then(|n| n.ty().attr_index(k)) else { continue };
                let pv = match v {
                    Value::Bool(b) => PlugValue::Bool(*b),
                    Value::String(x) => PlugValue::String(x.clone()),
                    x if g.node(h).unwrap().ty().attrs[i].ty == PlugType::Int => PlugValue::Int(x.as_i64().unwrap_or(0)),
                    x => PlugValue::Float(x.as_f64().unwrap_or(0.0)),
                };
                g.set_value(crate::plug::PlugRef::new(h, i as i32), pv)?;
            }
            let state = state_of(&settings_of(g.node(h).unwrap().data(), settings));
            for (k, v) in [("inputComponents", encode_target(&t)), ("settingsState", state)] {
                if let Some(i) = g.node(h).and_then(|n| n.ty().attr_index(k)) {
                    g.set_value(crate::plug::PlugRef::new(h, i as i32), PlugValue::String(v))?;
                }
            }
            crate::history_qe::apply_common(g, h, a)?;
            if let Some(n) = a.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                g.rename(h, n)?;
            }
            made.push(json!(g.path(h)));
        }
        Ok(())
    })();
    g.close_chunk();
    r?;
    s.graph.evaluate();
    Ok(Value::Array(made))
}

/// Flags that steer the command rather than name a setting.
const META: &[&str] = &["query", "edit", "queried", "name", "constructionHistory", "uvSetName", "insertBeforeDeformers", "createNewMap", "args"];

/// -q and -e on a UV node: two-value flags read and set both settings; an edit makes the node's UVs again.
fn live_qe(s: &mut Session, a: &Args, cmd: &str, node_type: &'static str) -> Option<CmdResult> {
    if !a.bool("query") && !a.bool("edit") {
        return None;
    }
    Some((|| -> CmdResult {
        let types: &'static [&'static str] = match node_type {
            "polyFlipUV" => &["polyFlipUV"],
            "polyLayoutUV" => &["polyLayoutUV"],
            "polyMergeUV" => &["polyMergeUV"],
            "polyMoveUV" => &["polyMoveUV"],
            "polyNormalizeUV" => &["polyNormalizeUV"],
            "polyMapCut" => &["polyMapCut"],
            "polyMapSew" => &["polyMapSew"],
            "polyMapSewMove" => &["polyMapSewMove"],
            "polyStraightenUVBorder" => &["polyStraightenUVBorder"],
            _ => &["polyOptUvs"],
        };
        let names = a.named();
        let Some(first) = names.first() else { fail!("{cmd}: name the {node_type} node or a mesh it feeds") };
        let Some(node) = crate::history_qe::find_hist(&s.graph, first, types) else { fail!("{cmd}: {first} has no {node_type} node") };
        let path = s.graph.path(node);
        let given: Vec<String> = s.commands.current_given().map(|g| g.to_vec()).unwrap_or_default();
        // Only polyMoveUV's flags of two values stand for two settings.
        let pairs: &[(&str, &str, &str)] = if node_type == "polyMoveUV" { PAIRS } else { &[] };
        if a.bool("query") {
            let asked: Vec<String> = a.get("queried").and_then(Value::as_array).map(|q| q.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            let flag = asked.into_iter().find(|k| !META.contains(&k.as_str())).or_else(|| given.iter().find(|k| !META.contains(&k.as_str())).cloned());
            let Some(flag) = flag else { fail!("{cmd}: name a flag to query") };
            if let Some(&(_, x, y)) = pairs.iter().find(|p| p.0 == flag) {
                let mut out = Vec::new();
                for attr in [x, y] {
                    let v = s.run("getAttr", json!({"args": [format!("{path}.{attr}")]}))?;
                    out.push(match v {
                        Value::Array(l) if l.len() == 1 => l[0].clone(),
                        x => x,
                    });
                }
                return Ok(Value::Array(out));
            }
            let mut b = a.clone();
            b.positional = vec![json!(path)];
            b.flags.insert("queried".into(), json!([flag]));
            return crate::history_qe::qe(s, &b, cmd, types, &[]).unwrap_or_else(|| Ok(Value::Null));
        }
        for flag in given.iter().filter(|k| !META.contains(&k.as_str()) && k.as_str() != "edit") {
            let Some(v) = a.get(flag).cloned() else { continue };
            let sets: Vec<(String, Value)> = match pairs.iter().find(|p| p.0 == flag.as_str()) {
                Some(&(_, x, y)) => {
                    let l = v.as_array().cloned().unwrap_or_default();
                    [x, y].iter().zip(l).map(|(k, v)| (k.to_string(), v)).collect()
                }
                None => vec![(flag.clone(), v)],
            };
            for (attr, v) in sets {
                let plug = format!("{path}.{attr}");
                if s.graph.plug(&plug).is_err() {
                    fail!("{cmd}: -{flag} can't be edited");
                }
                let v = match v {
                    Value::Bool(b) => json!(b as i64),
                    x => x,
                };
                s.run("setAttr", json!({"args": [plug, v]}))?;
            }
        }
        s.graph.evaluate();
        Ok(json!([path]))
    })())
}

fn register_live_uv(c: &mut Commands<Session>) {
    for &(name, node_type, label) in LIVE_CMDS {
        let Some(old) = c.find(name).cloned() else {
            log::warn!("{name}: nothing to extend");
            continue;
        };
        let settings = live(node_type).map(|l| l.1).unwrap_or(&[]);
        let mut cmd = crate::history_qe::node_flags(crate::history_qe::qe_flags(old));
        // Flags kept under their short names only by older scripts give way to the reference's.
        cmd.flags.retain(|fl| !(fl.name == fl.short_name && settings.iter().any(|s| s.1 == fl.name)));
        for &(long, short, kind, _, doc) in settings {
            let ty = match kind {
                'b' => ArgType::Bool,
                'i' => ArgType::Int,
                's' => ArgType::String,
                _ => ArgType::Float,
            };
            if cmd.flags.iter().any(|x| x.name == long) {
                continue;
            }
            cmd = crate::history_qe::upsert(cmd, f(long, short, ty, doc));
        }
        let mut common = vec![
            ("constructionHistory", "ch", ArgType::Bool, "Keep a history node (on)."),
            ("name", "n", ArgType::String, "The node's name."),
            ("uvSetName", "uvs", ArgType::String, "The UV set to work on (the current one)."),
        ];
        if name == "polyNormalizeUV" {
            common.push(("createNewMap", "cm", ArgType::Bool, "Make the UV set -uvs names, a copy of the current one, when the mesh lacks it."));
            common.push(("insertBeforeDeformers", "ibd", ArgType::Bool, "Accepted: the node goes into the mesh's history as any UV edit does."));
        }
        for (long, short, ty, doc) in common {
            if !cmd.flags.iter().any(|x| x.name == long) {
                cmd = crate::history_qe::upsert(cmd, f(long, short, ty, doc));
            }
        }
        if node_type == "polyMoveUV" {
            for &(pair, _, _) in PAIRS {
                if let Some(fl) = cmd.flags.iter_mut().find(|x| x.name == pair) {
                    fl.ty = ArgType::List;
                    fl.arity = 2;
                }
            }
        }
        cmd.positional_doc = "UVs, edges, faces or meshes; with -q and -e, the node or a mesh it feeds".into();
        let (cname, label) = (name.to_string(), label.to_string());
        cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
            if let Some(r) = live_qe(s, a, &cname, node_type) {
                return r;
            }
            each_live(s, a, &cname, &label, node_type)
        });
        c.register(cmd);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// The UV edits

fn uv_at(m: &Mesh, c: Index) -> DVec2 {
    m.corner_attrs().get::<[f32; 2]>(uv_ops::UV).get(c as usize).map_or(DVec2::ZERO, |u| DVec2::new(u[0] as f64, u[1] as f64))
}

/// The edges between the faces and the rest of the mesh.
fn region_border(m: &Mesh, faces: &[Index]) -> Vec<Index> {
    let set: BTreeSet<Index> = faces.iter().copied().collect();
    let mut out = Vec::new();
    for &f in faces {
        if !m.face_valid(f) {
            continue;
        }
        for c in m.face_corners(f) {
            let r = m.corner_radial(c);
            if r != c && r != aztek_geo::INVALID && !set.contains(&m.corner_face(r)) {
                out.push(m.corner_edge(c));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn flip_uv(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let vertical = a.int("flipType", 0) == 1;
    let faces = if t.whole { uv_ops::all_faces(m) } else { t.faces.clone() };
    if a.bool("cutUV") && !t.whole && !faces.is_empty() {
        let border = region_border(m, &faces);
        uv_ops::cut(m, &border);
    }
    let corners = uv_ops::corners_of(m, tp, &faces, &t.maps);
    let pivot = if a.bool("usePivot") {
        DVec2::new(a.float("pivotU", 0.0), a.float("pivotV", 0.0))
    } else if a.bool("local") {
        let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), &corners);
        (lo + hi) * 0.5
    } else {
        DVec2::splat(0.5)
    };
    let sc = if vertical { DVec2::new(1.0, -1.0) } else { DVec2::new(-1.0, 1.0) };
    uv_ops::transform(m, &corners, DVec2::ZERO, sc, 0.0, pivot);
}

/// A shell's UV area with its sign: negative when its UVs face backwards.
fn signed_uv_area(m: &Mesh, faces: &[Index]) -> f64 {
    let mut area = 0.0;
    for &f in faces {
        if !m.face_valid(f) {
            continue;
        }
        let cs = m.face_corners(f);
        for i in 0..cs.len() {
            let (p, q) = (uv_at(m, cs[i]), uv_at(m, cs[(i + 1) % cs.len()]));
            area += p.x * q.y - q.x * p.y;
        }
    }
    area * 0.5
}

/// Fits the UVs of the corners into 0 to 1: the same scale both ways (`uniform`) or each its own.
fn fit_unit(m: &mut Mesh, corners: &[Index], uniform: bool) {
    let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), corners);
    let size = (hi - lo).max(DVec2::splat(1e-12));
    let mut sc = DVec2::new(1.0 / size.x, 1.0 / size.y);
    if uniform {
        sc = DVec2::splat(sc.x.min(sc.y));
    }
    uv_ops::transform(m, corners, -lo, sc, 0.0, lo);
}

fn layout_uv(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let faces = uv_ops::target_faces(m, t, tp);
    let shells = uv_ops::shells_of(tp, &faces);
    if a.bool("flipReversed") {
        for sh in &shells {
            if signed_uv_area(m, sh) < 0.0 {
                let corners = uv_ops::corners_of(m, tp, sh, &[]);
                let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), &corners);
                uv_ops::transform(m, &corners, DVec2::ZERO, DVec2::new(-1.0, 1.0), 0.0, (lo + hi) * 0.5);
            }
        }
    }
    let all: Vec<Index> = shells.concat();
    let corners = uv_ops::corners_of(m, tp, &all, &[]);
    let spacing = (a.float("percentageSpace", 0.2).clamp(0.0, 5.0) / 100.0).max(0.002) * 2.0;
    let layout = a.int("layout", 2);
    let scale = a.int("scale", 1).clamp(0, 2);
    let mut p = uv_ops::Pack { spacing, margin: spacing, relative: true, rotate: a.int("rotateForBestFit", 1) != 0, translate: layout != 0, scale, ..Default::default() };
    if layout == 1 {
        // Along U: one long row, then fitted as asked.
        p.region = (DVec2::ZERO, DVec2::new(1.0e4, 1.0));
        p.scale = 0;
        p.margin = 0.0;
    }
    uv_ops::layout_with(m, &all, &p);
    if (layout == 1 || layout == 0) && scale != 0 {
        fit_unit(m, &corners, scale == 1);
    }
}

fn merge_uv(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let d = a.float("distance", 0.0).max(1e-7);
    let faces = if t.whole { uv_ops::all_faces(m) } else { t.faces.clone() };
    let corners = uv_ops::corners_of(m, tp, &faces, &t.maps);
    let mut by_point: std::collections::BTreeMap<Index, Vec<Index>> = Default::default();
    for &c in &corners {
        by_point.entry(m.corner_point(c)).or_default().push(c);
    }
    let uv = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
    for group in by_point.values() {
        let mut used = vec![false; group.len()];
        for i in 0..group.len() {
            if used[i] {
                continue;
            }
            let base = uv[group[i] as usize];
            let mut members = Vec::new();
            let mut sum = DVec2::ZERO;
            for (j, &c) in group.iter().enumerate() {
                let q = uv[c as usize];
                if !used[j] && ((q[0] - base[0]) as f64).hypot((q[1] - base[1]) as f64) <= d {
                    used[j] = true;
                    sum += DVec2::new(q[0] as f64, q[1] as f64);
                    members.push(c);
                }
            }
            let mean = sum / members.len() as f64;
            for c in members {
                uv[c as usize] = [mean.x as f32, mean.y as f32];
            }
        }
    }
    // Edges whose two sides now meet in UVs are no longer seams.
    let edges: Vec<Index> = corners.iter().map(|&k| m.corner_edge(k)).collect::<BTreeSet<_>>().into_iter().collect();
    let mut joined = Vec::new();
    for e in edges {
        let c0 = m.edge_corner(e);
        let r = m.corner_radial(c0);
        if r == c0 || r == aztek_geo::INVALID || m.corner_radial(r) != c0 {
            continue;
        }
        let at = |k: Index, p: Index| if m.corner_point(k) == p { k } else { m.corner_next(k) };
        let (p, q) = (m.corner_point(c0), m.corner_next_point(c0));
        if uv_at(m, c0).distance(uv_at(m, at(r, p))) < 1e-6 && uv_at(m, m.corner_next(c0)).distance(uv_at(m, at(r, q))) < 1e-6 {
            joined.push(e);
        }
    }
    let seam = m.edge_attrs_mut().get_or_add::<bool>("seam");
    for e in joined {
        if let Some(x) = seam.get_mut(e as usize) {
            *x = false;
        }
    }
}

/// A number in -1 to 1 that stays the same for the same seed.
fn noise(seed: u64) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(0x6A09_E667);
    x ^= x >> 31;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 29;
    (x % 2_000_001) as f64 / 1_000_000.0 - 1.0
}

fn move_uv(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let faces = if t.whole { uv_ops::all_faces(m) } else { t.faces.clone() };
    let corners = uv_ops::corners_of(m, tp, &faces, &t.maps);
    let tr = DVec2::new(a.float("translateU", 0.0), a.float("translateV", 0.0));
    let sc = DVec2::new(a.float("scaleU", 1.0), a.float("scaleV", 1.0));
    let pv = DVec2::new(a.float("pivotU", 0.5), a.float("pivotV", 0.5));
    uv_ops::transform(m, &corners, tr, sc, a.float("rotationAngle", 0.0), pv);
    let r = a.float("random", 0.0);
    if r != 0.0 {
        // The same offset for every corner of a UV, so UVs stay joined.
        let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
        for &c in &corners {
            let id = tp.corner_map.get(c as usize).copied().unwrap_or(c) as u64;
            layer[c as usize][0] += (noise(id * 2) * r) as f32;
            layer[c as usize][1] += (noise(id * 2 + 1) * r) as f32;
        }
    }
}

fn normalize_uv(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let each_shell = a.int("normalizeType", 1) == 0;
    let keep = a.bool("preserveAspectRatio");
    let dir = a.int("normalizeDirection", 0);
    let on_tile = a.bool("centerOnTile");
    let faces = uv_ops::target_faces(m, t, tp);
    let groups: Vec<Vec<Index>> = if each_shell { uv_ops::shells_of(tp, &faces) } else { vec![faces] };
    for fs in groups {
        let corners = uv_ops::corners_of(m, tp, &fs, &[]);
        let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), &corners);
        let size = hi - lo;
        if size.x <= 0.0 && size.y <= 0.0 {
            continue;
        }
        let tile = if on_tile { ((lo + hi) * 0.5).floor() } else { DVec2::ZERO };
        let mut sc = DVec2::new(1.0 / size.x.max(1e-12), 1.0 / size.y.max(1e-12));
        let mut off = tile - lo;
        match dir {
            1 => {
                sc.y = 1.0;
                off.y = 0.0;
            }
            2 => {
                sc.x = 1.0;
                off.x = 0.0;
            }
            _ if keep => {
                sc = DVec2::splat(sc.x.min(sc.y));
                // The shorter side is centred in the tile.
                let fill = size * sc;
                off += (DVec2::ONE - fill) * 0.5;
            }
            _ => {}
        }
        uv_ops::transform(m, &corners, off, sc, 0.0, lo);
    }
}

fn map_cut(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let edges = uv_ops::target_edges(m, t, tp);
    uv_ops::cut(m, &edges);
    let ratio = a.float("moveratio", 0.0);
    if ratio == 0.0 {
        return;
    }
    // The cut opens: each side's UVs on the cut move toward the middle of their face.
    let mut moved: std::collections::BTreeMap<Index, DVec2> = Default::default();
    for &e in &edges {
        if !m.edge_valid(e) {
            continue;
        }
        let c0 = m.edge_corner(e);
        let mut c = c0;
        loop {
            let f = m.corner_face(c);
            let cs = m.face_corners(f);
            let mid = cs.iter().map(|&k| uv_at(m, k)).sum::<DVec2>() / cs.len().max(1) as f64;
            for k in [c, m.corner_next(c)] {
                let p = uv_at(m, k);
                moved.entry(k).or_insert(p + (mid - p) * ratio.clamp(-1.0, 1.0));
            }
            c = m.corner_radial(c);
            if c == c0 || c == aztek_geo::INVALID {
                break;
            }
        }
    }
    let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
    for (k, p) in moved {
        layer[k as usize] = [p.x as f32, p.y as f32];
    }
}

fn map_sew(_a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let edges = uv_ops::target_edges(m, t, tp);
    uv_ops::sew(m, &edges);
}

fn map_sew_move(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let mut edges = uv_ops::target_edges(m, t, tp);
    if a.bool("limitPieceSize") {
        let most = a.int("numberFaces", 10).max(0) as usize;
        // A merge is refused when the smaller of the two pieces has more faces than numberFaces.
        edges.retain(|&e| {
            if !m.edge_valid(e) {
                return false;
            }
            let c = m.edge_corner(e);
            let r = m.corner_radial(c);
            if r == c || r == aztek_geo::INVALID {
                return true;
            }
            let size = |f: Index| tp.face_shell.get(f as usize).and_then(|&s| tp.shells.get(s as usize)).map_or(0, Vec::len);
            size(m.corner_face(c)).min(size(m.corner_face(r))) <= most
        });
    }
    uv_ops::move_and_sew(m, &edges);
}

fn opt_uvs(a: &Args, m: &mut Mesh, t: &Target, tp: &UvTopo) {
    let mut faces = uv_ops::target_faces(m, t, tp);
    let mut o = uv_ops::UnfoldOpts { iterations: (a.int("iterations", 100) / 8).clamp(1, 200) as i32, pin_border: a.bool("pinUvBorder"), ..Default::default() };
    if !t.maps.is_empty() && !t.whole {
        let shells: Vec<Index> = uv_ops::shells_of(tp, &faces).concat();
        let named: BTreeSet<Index> = uv_ops::corners_of(m, tp, &[], &t.maps).into_iter().collect();
        if a.bool("pinSelected") {
            o.pinned = named.iter().copied().collect();
        } else if !a.bool("applyToShell") {
            // Only the UVs named move: the rest of their pieces hold.
            o.pinned = uv_ops::corners_of(m, tp, &shells, &[]).into_iter().filter(|c| !named.contains(c)).collect();
        }
        faces = shells;
    }
    uv_ops::unfold_with(m, &faces, &o);
}

// ---------------------------------------------------------------------------------------------------------------------
// polyPlanarProjection: the projection's settings on its polyPlanarProj node

/// Each single-value flag and the polyPlanarProj attribute it reads and writes.
const PLANAR_SINGLE: &[(&str, &str)] = &[
    ("projectionCenterX", "projectionCenterX"),
    ("projectionCenterY", "projectionCenterY"),
    ("projectionCenterZ", "projectionCenterZ"),
    ("projectionHeight", "projectionHeight"),
    ("projectionHorizontalSweep", "projectionHorizontalSweep"),
    ("projectionScaleU", "projectionWidth"),
    ("projectionScaleV", "projectionHeight"),
    ("imageCenterX", "imageCenterX"),
    ("imageCenterY", "imageCenterY"),
    ("imageScaleU", "imageScaleX"),
    ("imageScaleV", "imageScaleY"),
    ("rotationAngle", "rotationAngle"),
    ("seamCorrect", "seamCorrect"),
    ("worldSpace", "worldSpace"),
    ("rotateX", "rotateX"),
    ("rotateY", "rotateY"),
    ("rotateZ", "rotateZ"),
    ("caching", "caching"),
    ("nodeState", "nodeState"),
];

/// The flags of several values and the attributes each value goes to.
const PLANAR_MULTI: &[(&str, &[&str])] = &[
    ("projectionCenter", &["projectionCenterX", "projectionCenterY", "projectionCenterZ"]),
    ("projectionScale", &["projectionWidth", "projectionHeight"]),
    ("imageCenter", &["imageCenterX", "imageCenterY"]),
    ("imageScale", &["imageScaleX", "imageScaleY"]),
    ("rotate", &["rotateX", "rotateY", "rotateZ"]),
];

fn planar(c: &mut Commands<Session>) {
    use aztek_ops::units::Unit;
    let Some(mut cmd) = c.find("polyPlanarProjection").cloned() else { return };
    let fl = |n: &str, sh: &str, t: ArgType, unit: Unit, doc: &str| {
        let mut x = f(n, sh, t, doc);
        x.unit = unit;
        x
    };
    for x in [
        fl("projectionCenter", "pc", ArgType::Vec3, Unit::Linear, "The point on the object the projection is centred on (0 0 0)."),
        fl("projectionHeight", "ph", ArgType::Float, Unit::Linear, "The map's height along the projection."),
        fl("projectionHorizontalSweep", "phs", ArgType::Float, Unit::Angle, "The angle the projection sweeps across."),
        fl("projectionScale", "ps", ArgType::Any, Unit::None, "The map's width and height along the projection (1 1).").multi(2),
        fl("imageCenter", "ic", ArgType::Any, Unit::None, "Where the middle of the UVs goes (0.5 0.5).").multi(2),
        fl("imageScale", "is", ArgType::Any, Unit::None, "How the UVs scale about that middle (1 1).").multi(2),
        fl("rotate", "ro", ArgType::Vec3, Unit::Angle, "The projection's rotation (0 0 0)."),
        fl("rotationAngle", "ra", ArgType::Float, Unit::Angle, "How far the UVs turn, counterclockwise."),
        fl("seamCorrect", "sc", ArgType::Bool, Unit::None, "Cut the seam where the map wraps."),
        fl("worldSpace", "ws", ArgType::Bool, Unit::None, "Values in world space (on) or the object's (off)."),
        fl("perInstance", "pi", ArgType::Bool, Unit::None, "A new map for this instance only; UV sets are shared by every instance here."),
        fl("name", "n", ArgType::String, Unit::None, "The node's name."),
    ] {
        cmd = crate::history_qe::upsert(cmd, x);
    }
    let cmd = crate::history_qe::node_flags(crate::history_qe::qe_flags(cmd));
    crate::poly_extra::projection_qe(c, cmd, "polyPlanarProj", (PLANAR_SINGLE, PLANAR_MULTI));
}

// ---------------------------------------------------------------------------------------------------------------------
// polyHole and polyPinUV: -createHistory, query and edit

/// Whether any of the meshes has history.
fn has_history(s: &Session, meshes: &[NodeId]) -> bool {
    meshes.iter().any(|&m| !crate::history::history_of(&s.graph, m).is_empty())
}

fn asked(a: &Args) -> Vec<String> {
    a.get("queried").and_then(Value::as_array).map(|q| q.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn hole(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyHole").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = crate::history_qe::upsert(
        old,
        f("createHistory", "ch", ArgType::Bool, "Make a history node even on a mesh with none (off); a mesh with history always gets one."),
    );
    cmd = crate::history_qe::upsert(cmd, f("edit", "e", ArgType::Bool, "Edit mode: set or clear holes as in create mode."));
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        let (targets, _) = crate::mesh_edit::ref_poly::gather(s, a, crate::components::ComponentMode::Face)?;
        let meshes: Vec<NodeId> = targets.keys().copied().collect();
        if a.bool("query") {
            if asked(a).iter().any(|k| k == "createHistory") {
                return Ok(json!(has_history(s, &meshes)));
            }
            return run(s, a);
        }
        let mut b = a.clone();
        b.flags.remove("edit");
        let forced = b.flags.remove("createHistory").and_then(|v| v.as_bool().or_else(|| v.as_i64().map(|i| i != 0)));
        let ch = forced == Some(true) || has_history(s, &meshes);
        b.flags.insert("constructionHistory".into(), json!(ch));
        run(s, &b)
    });
    c.register(cmd);
}

fn pin_uv(c: &mut Commands<Session>) {
    let Some(old) = c.find("polyPinUV").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = crate::history_qe::upsert(old, f("createHistory", "ch", ArgType::Bool, "Accepted: pins are written into the mesh, which carries them through its history."));
    cmd = crate::history_qe::qe_flags(cmd);
    if let Some(v) = cmd.flags.iter_mut().find(|x| x.name == "value") {
        v.ty = ArgType::Any;
        v.arity = 1;
    }
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if a.bool("query") {
            let what = asked(a);
            let targets = uv_ops::targets(s, a, "polyPinUV")?;
            let meshes: Vec<NodeId> = targets.keys().copied().collect();
            if what.iter().any(|k| k == "createHistory") {
                return Ok(json!(has_history(s, &meshes)));
            }
            if what.iter().any(|k| k == "operation") {
                return Ok(json!(0));
            }
            if what.iter().any(|k| k == "uvSetName") {
                let Some(&m) = meshes.first() else { return Ok(json!("")) };
                let path = s.graph.path(m);
                let cur = s.run("polyUVSet", json!({"args": [path], "query": true, "queried": ["currentUVSet"]}))?;
                return Ok(cur.as_array().and_then(|l| l.first()).cloned().unwrap_or(cur));
            }
            let unpinned = what.iter().any(|k| k == "unpinned");
            let mut out = Vec::new();
            for (mesh, t) in targets {
                let Some(m) = crate::modifiers::cage(&mut s.graph, mesh) else { continue };
                let tp = uv_ops::topo(&m);
                let maps: Vec<Index> = if t.maps.is_empty() {
                    let faces = uv_ops::target_faces(&m, &t, &tp);
                    let mut v: Vec<Index> = uv_ops::corners_of(&m, &tp, &faces, &[]).iter().map(|&c| tp.corner_map[c as usize]).collect();
                    v.sort_unstable();
                    v.dedup();
                    v
                } else {
                    t.maps.clone()
                };
                let pins = m.corner_attrs().get::<f32>("uvPin");
                let name = crate::components::display_path(&s.graph, crate::dag::transform_of(&s.graph, mesh));
                for mp in maps {
                    let Some(&k) = tp.maps.get(mp as usize).and_then(|v| v.first()) else { continue };
                    let w = pins.get(k as usize).copied().unwrap_or(0.0);
                    if unpinned {
                        if w == 0.0 {
                            out.push(json!(format!("{name}.map[{mp}]")));
                        }
                    } else {
                        out.push(json!(w as f64));
                    }
                }
            }
            return Ok(Value::Array(out));
        }
        let mut b = a.clone();
        b.flags.remove("edit");
        b.flags.remove("createHistory");
        // -v given again for each UV in turn.
        if let Some(Value::Array(vals)) = a.get("value").cloned() {
            if vals.len() > 1 {
                let targets = uv_ops::targets(s, a, "polyPinUV")?;
                let mut names = Vec::new();
                for (mesh, t) in &targets {
                    let name = crate::components::display_path(&s.graph, crate::dag::transform_of(&s.graph, *mesh));
                    names.extend(t.maps.iter().map(|mp| format!("{name}.map[{mp}]")));
                }
                s.graph.open_chunk("Pin UVs");
                let r = (|| -> CmdResult<()> {
                    for (i, n) in names.iter().enumerate() {
                        let v = vals.get(i).or(vals.last()).cloned().unwrap_or(json!(1.0));
                        let mut one = b.clone();
                        one.positional = vec![json!(n)];
                        one.flags.insert("value".into(), json!([v]));
                        run(s, &one)?;
                    }
                    Ok(())
                })();
                s.graph.close_chunk();
                r?;
                return Ok(json!(true));
            }
        }
        run(s, &b)
    });
    c.register(cmd);
}

// ---------------------------------------------------------------------------------------------------------------------
// polySelect -query

fn select_query(c: &mut Commands<Session>) {
    let Some(old) = c.find("polySelect").cloned() else { return };
    let run = old.run.clone();
    let mut cmd = crate::history_qe::upsert(old, f("query", "q", ArgType::Bool, "Query mode: answer what the flags would select, leaving the selection alone."));
    cmd = crate::history_qe::upsert(cmd, f("queried", "", ArgType::List, "Flags asked about in query mode."));
    cmd.run = std::sync::Arc::new(move |s: &mut Session, a: &Args| {
        if !a.bool("query") {
            return run(s, a);
        }
        let mut b = a.clone();
        b.flags.remove("query");
        b.flags.remove("queried");
        b.flags.insert("noSelection".into(), json!(true));
        run(s, &b)
    });
    c.register(cmd);
}

// ---------------------------------------------------------------------------------------------------------------------
// polyOutput

const OUTPUT_FLAGS: &[(&str, &str, &str)] = &[
    ("vert", "v", "Vertex positions."),
    ("edge", "e", "Each edge's two vertices and whether it is smooth."),
    ("face", "f", "Each face's vertices and edges."),
    ("faceNorm", "fn", "Each face's normal."),
    ("vertNorm", "vn", "The normals at each face's corners."),
    ("triangle", "t", "The triangles each face is drawn with."),
    ("uvValue", "uv", "UV positions, for each UV set."),
    ("uvDesc", "uvd", "Each face's UV ids, for each UV set."),
    ("color", "c", "The colour at each face's corners, for each colour set."),
    ("colorDesc", "cd", "Each face's colour ids, for each colour set."),
    ("normDesc", "nd", "Each face's corner normal ids."),
    ("edgeFace", "ef", "The faces on each edge."),
    ("vertEdge", "ve", "The edges at each vertex."),
    ("group", "g", "The shading groups holding the mesh and which of its faces."),
];

fn poly_output(c: &mut Commands<Session>) {
    let mut cmd = Cmd::new(
        "polyOutput",
        "A mesh's description as text: a summary, and with each flag one part of it (-a everything). A plug (polySubdFace1.output) \
         describes the mesh flowing through it. Returns the lines; -of writes them to a file, -no gives nothing.",
        output_run,
    )
    .positional("meshes or mesh plugs (the selection when none)")
    .flag(f("allValues", "a", ArgType::Bool, "Every part."))
    .flag(f("force", "fo", ArgType::Bool, "Bring the scene up to date first."))
    .flag(f("noOutput", "no", ArgType::Bool, "Work it out but give nothing."))
    .flag(f("outputFile", "of", ArgType::String, "Write the lines to this file."))
    .undoable(false);
    for &(n, sh, doc) in OUTPUT_FLAGS {
        cmd = cmd.flag(f(n, sh, ArgType::Bool, doc));
    }
    c.register(cmd);
}

fn output_mesh(s: &mut Session, name: &str) -> CmdResult<(NodeId, std::sync::Arc<Mesh>)> {
    if name.contains('.') && !name.contains('[') {
        // A mesh plug: what flows through it.
        let p = s.graph.plug(name).map_err(|e| aztek_ops::CommandError::new(format!("polyOutput: {name}: {e}")))?;
        s.graph.evaluate();
        return match s.graph.value(&p).map_err(|e| aztek_ops::CommandError::new(e.to_string()))? {
            PlugValue::Mesh(Some(m)) => Ok((p.node, m)),
            _ => fail!("polyOutput: {name} holds no mesh"),
        };
    }
    let g = &mut s.graph;
    let Some(id) = g.find(name)? else { fail!("polyOutput: no object {name}") };
    let Some(mesh) = crate::components::as_mesh(g, id) else { fail!("polyOutput: {name} is not a mesh") };
    let Some(m) = crate::modifiers::cage(g, mesh) else { fail!("polyOutput: {name} has no geometry") };
    Ok((mesh, m))
}

fn output_run(s: &mut Session, a: &Args) -> CmdResult {
    let names = aztek_ops::objects(s, a);
    if a.bool("force") {
        s.graph.evaluate();
    }
    let all = a.bool("allValues");
    let on = |k: &str| all || a.bool(k);
    let mut out: Vec<String> = Vec::new();
    for n in &names {
        let (node, m) = output_mesh(s, n)?;
        let valid_f: Vec<Index> = (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect();
        out.push(format!("{n}: {} vertices, {} edges, {} faces, {} face vertices", m.num_points(), m.num_edges(), m.num_faces(), valid_f.iter().map(|&f| m.face_corners(f).len()).sum::<usize>()));
        if on("vert") {
            for p in (0..m.point_capacity()).filter(|&p| m.point_valid(p)) {
                let q = m.position(p);
                out.push(format!("vtx {p}: {} {} {}", q.x, q.y, q.z));
            }
        }
        if on("edge") {
            let hard = m.edge_attrs().get::<bool>("hard");
            for e in (0..m.edge_capacity()).filter(|&e| m.edge_valid(e)) {
                let smooth = !hard.get(e as usize).copied().unwrap_or(false);
                out.push(format!("edge {e}: {} {} {}", m.edge_point(e, 0), m.edge_point(e, 1), if smooth { "smooth" } else { "hard" }));
            }
        }
        if on("face") {
            for &f in &valid_f {
                let pts: Vec<String> = m.face_points(f).iter().map(|x| x.to_string()).collect();
                let eds: Vec<String> = m.face_corners(f).iter().map(|&k| m.corner_edge(k).to_string()).collect();
                out.push(format!("face {f}: {} edges {}", pts.join(" "), eds.join(" ")));
            }
        }
        if on("faceNorm") {
            for &f in &valid_f {
                let q = m.face_normal(f);
                out.push(format!("faceNormal {f}: {} {} {}", q.x, q.y, q.z));
            }
        }
        if on("vertNorm") || on("normDesc") {
            let pn = aztek_geo::normals::point_normals(&m, aztek_geo::normals::NormalWeight::Area);
            let hard = m.edge_attrs().get::<bool>("hard");
            let mut k = 0usize;
            for &f in &valid_f {
                let mut ids = Vec::new();
                for c in m.face_corners(f) {
                    // A corner on a hard edge takes the face's normal.
                    let sharp = hard.get(m.corner_edge(c) as usize).copied().unwrap_or(false) || hard.get(m.corner_edge(m.corner_prev(c)) as usize).copied().unwrap_or(false);
                    let nrm = if sharp { m.face_normal(f) } else { pn.get(m.corner_point(c) as usize).map_or(glam::DVec3::Y, |v| v.as_dvec3()) };
                    if on("vertNorm") {
                        out.push(format!("vertNormal {f} {}: {} {} {}", m.corner_point(c), nrm.x, nrm.y, nrm.z));
                    }
                    ids.push(k.to_string());
                    k += 1;
                }
                if on("normDesc") {
                    out.push(format!("normDesc {f}: {}", ids.join(" ")));
                }
            }
        }
        if on("triangle") {
            for &f in &valid_f {
                let p = m.face_points(f);
                let tris: Vec<String> = (1..p.len().saturating_sub(1)).map(|i| format!("({} {} {})", p[0], p[i], p[i + 1])).collect();
                out.push(format!("triangles {f}: {}", tris.join(" ")));
            }
        }
        if on("uvValue") || on("uvDesc") {
            let layers: Vec<String> = m.corner_attrs().names().into_iter().filter(|l| l.starts_with("uv") && m.corner_attrs().find(l).is_some_and(|x| x.ty() == aztek_geo::AttrType::Float2)).collect();
            for layer in layers {
                let mut mm = (*m).clone();
                if layer != uv_ops::UV {
                    let vals = m.corner_attrs().get::<[f32; 2]>(&layer).to_vec();
                    mm.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV).copy_from_slice(&vals);
                }
                let tp = uv_ops::topo(&mm);
                out.push(format!("uvSet {layer}"));
                if on("uvValue") {
                    for (i, cs) in tp.maps.iter().enumerate() {
                        if let Some(&k) = cs.first() {
                            let q = uv_at(&mm, k);
                            out.push(format!("uv {i}: {} {}", q.x, q.y));
                        }
                    }
                }
                if on("uvDesc") {
                    for &f in &valid_f {
                        let ids: Vec<String> = m.face_corners(f).iter().map(|&k| tp.corner_map[k as usize].to_string()).collect();
                        out.push(format!("uvDesc {f}: {}", ids.join(" ")));
                    }
                }
            }
        }
        if on("color") || on("colorDesc") {
            for set in aztek_geo::colors::sets(&m) {
                let vals = aztek_geo::colors::values(&m, &set);
                out.push(format!("colorSet {set}"));
                let mut ids: Vec<[f32; 4]> = Vec::new();
                for &f in &valid_f {
                    let mut desc = Vec::new();
                    for k in m.face_corners(f) {
                        let v = vals.get(k as usize).copied().unwrap_or([1.0; 4]);
                        let id = ids.iter().position(|x| *x == v).unwrap_or_else(|| {
                            ids.push(v);
                            ids.len() - 1
                        });
                        desc.push(id.to_string());
                    }
                    if on("colorDesc") {
                        out.push(format!("colorDesc {f}: {}", desc.join(" ")));
                    }
                }
                if on("color") {
                    for (i, v) in ids.iter().enumerate() {
                        out.push(format!("color {i}: {} {} {} {}", v[0], v[1], v[2], v[3]));
                    }
                }
            }
        }
        if on("edgeFace") {
            for e in (0..m.edge_capacity()).filter(|&e| m.edge_valid(e)) {
                let mut faces = Vec::new();
                m.for_each_edge_corner(e, |k| faces.push(m.corner_face(k).to_string()));
                out.push(format!("edgeFace {e}: {}", faces.join(" ")));
            }
        }
        if on("vertEdge") {
            let mut by_point: std::collections::BTreeMap<Index, Vec<Index>> = Default::default();
            for e in (0..m.edge_capacity()).filter(|&e| m.edge_valid(e)) {
                by_point.entry(m.edge_point(e, 0)).or_default().push(e);
                by_point.entry(m.edge_point(e, 1)).or_default().push(e);
            }
            for (p, es) in by_point {
                out.push(format!("vertEdge {p}: {}", es.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ")));
            }
        }
        if on("group") {
            if s.graph.node(node).is_some_and(|x| x.ty().is_a("mesh")) {
                for (sg, faces) in crate::shading::membership_of(&s.graph, node) {
                    let what = match faces {
                        Some(ids) => crate::history::format_components(crate::components::ComponentMode::Face, &ids),
                        None => "all faces".into(),
                    };
                    out.push(format!("group {}: {what}", s.graph.path(sg)));
                }
            }
        }
    }
    if a.bool("noOutput") {
        return Ok(json!([]));
    }
    if let Some(p) = a.get("outputFile").and_then(Value::as_str) {
        std::fs::write(p, out.join("\n")).map_err(|e| aztek_ops::CommandError::new(format!("polyOutput: {p}: {e}")))?;
    }
    Ok(json!(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// polyGeoSampler: the surface's colour and transparency as lit, sampled when the command runs, then stored as colours
// or used to push the points along their normals

/// Each light shining on the scene, or a default light along the view when there are none.
fn sampling_lights(g: &mut crate::graph::Graph) -> Vec<crate::lights::SceneLight> {
    let mut out: Vec<crate::lights::SceneLight> = crate::lights::all_shapes(g).into_iter().filter_map(|id| crate::lights::scene_light(g, id)).filter(|l| l.diffuse).collect();
    if out.is_empty() {
        // The reference's default light shines along the camera's view.
        let world = g.find("persp").ok().flatten().map(|id| crate::rig::world(g, crate::dag::transform_of(g, id))).unwrap_or(glam::DMat4::IDENTITY);
        out.push(crate::lights::SceneLight {
            shape: crate::plug::NO_NODE,
            transform: crate::plug::NO_NODE,
            kind: crate::lights::Kind::Directional,
            world,
            color: [1.0; 3],
            diffuse: true,
            specular: false,
            decay: 0,
            cone_deg: 40.0,
            penumbra_deg: 0.0,
            dropoff: 0.0,
            ambient_shade: 0.45,
            by_area: false,
            volume_shape: 1,
            volume_dir: 0,
            volume_ambient: false,
            cast_shadows: false,
        });
    }
    out
}

/// A material's colour and alpha: color (or baseColor times base) and one minus its transparency (or its opacity).
fn material_colour(g: &mut crate::graph::Graph, mat: Option<NodeId>) -> [f64; 4] {
    let Some(mat) = mat else { return [0.5, 0.5, 0.5, 1.0] };
    let read = |g: &mut crate::graph::Graph, name: &str| -> Option<PlugValue> {
        let i = g.node(mat)?.ty().attr_index(name)?;
        g.value(&crate::plug::PlugRef::new(mat, i as i32)).ok()
    };
    let rgb = match (read(g, "color"), read(g, "baseColor")) {
        (Some(PlugValue::Vec3(c)), _) => c,
        (_, Some(PlugValue::Vec3(c))) => c * read(g, "base").and_then(|v| v.scalar()).unwrap_or(1.0),
        _ => glam::DVec3::splat(0.5),
    };
    let alpha = match (read(g, "transparency"), read(g, "opacity")) {
        (Some(PlugValue::Vec3(t)), _) => 1.0 - (t.x + t.y + t.z) / 3.0,
        (_, Some(PlugValue::Vec3(o))) => (o.x + o.y + o.z) / 3.0,
        _ => 1.0,
    };
    [rgb.x, rgb.y, rgb.z, alpha]
}

/// The samples for the faces of one mesh: [id, r, g, b, a] per face (sampleByFace), per point (averageColor) or per face
/// corner.
fn geo_samples(g: &mut crate::graph::Graph, shape: NodeId, m: &Mesh, faces: &[Index], a: &Args) -> Value {
    let world = g.node(shape).map_or(glam::DMat4::IDENTITY, |n| n.matrix(crate::transform::Transform::WORLD_MATRIX));
    let nmat = glam::DMat3::from_mat4(world).inverse().transpose();
    let lights = sampling_lights(g);
    let groups = crate::shading::membership_of(g, shape);
    let fg = crate::shading::face_groups(m, &groups);
    let mut mats: Vec<[f64; 4]> = Vec::new();
    for (sg, _) in &groups {
        let mat = crate::shading::material_of_group(g, *sg);
        mats.push(material_colour(g, mat));
    }
    let double = g.node(shape).and_then(|n| n.ty().attr_index("doubleSided").map(|i| n.bool(i))).unwrap_or(true) && !a.bool("ignoreDoubleSided");
    let lighting_only = a.bool("lightingOnly");
    let flat = a.bool("flatShading");
    let pn = aztek_geo::normals::point_normals(m, aztek_geo::normals::NormalWeight::Area);
    let sample = |p: glam::DVec3, n: glam::DVec3, f: Index| -> [f64; 4] {
        let pw = world.transform_point3(p);
        let nw = (nmat * n).normalize_or_zero();
        let light = |nn: glam::DVec3| lights.iter().map(|l| crate::lights::viewport_diffuse(l, pw, nn)).sum::<glam::DVec3>();
        let mut illum = light(nw);
        if double {
            illum = illum.max(light(-nw));
        }
        let base = fg.get(f as usize).and_then(|&k| mats.get(k)).copied().unwrap_or([0.5, 0.5, 0.5, 1.0]);
        let c = if lighting_only { illum } else { glam::DVec3::new(base[0], base[1], base[2]) * illum };
        [c.x, c.y, c.z, base[3]]
    };
    let mut out = Vec::new();
    if a.bool("sampleByFace") {
        for &f in faces.iter().filter(|&&f| m.face_valid(f)) {
            let pts = m.face_points(f);
            let mid = pts.iter().map(|&p| m.position(p)).sum::<glam::DVec3>() / pts.len().max(1) as f64;
            let s = sample(mid, m.face_normal(f), f);
            out.push(json!([f, s[0], s[1], s[2], s[3]]));
        }
        return json!({"by": "face", "values": out});
    }
    let mut per_point: std::collections::BTreeMap<Index, ([f64; 4], f64)> = Default::default();
    for &f in faces.iter().filter(|&&f| m.face_valid(f)) {
        for k in m.face_corners(f) {
            let p = m.corner_point(k);
            let n = if flat { m.face_normal(f) } else { pn.get(p as usize).map_or(m.face_normal(f), |v| v.as_dvec3()) };
            let s = sample(m.position(p), n, f);
            if a.bool("averageColor") {
                let e = per_point.entry(p).or_insert(([0.0; 4], 0.0));
                for i in 0..4 {
                    e.0[i] += s[i];
                }
                e.1 += 1.0;
            } else {
                out.push(json!([k, s[0], s[1], s[2], s[3]]));
            }
        }
    }
    if a.bool("averageColor") {
        let list: Vec<Value> = per_point.into_iter().map(|(p, (s, n))| json!([p, s[0] / n, s[1] / n, s[2] / n, s[3] / n])).collect();
        return json!({"by": "point", "values": list});
    }
    json!({"by": "corner", "values": out})
}

fn blend(mode: &str, old: f32, new: f32) -> f32 {
    match mode {
        "none" => old,
        "add" => old + new,
        "subtract" => old - new,
        "multiply" => old * new,
        "divide" => {
            if new.abs() > 1e-9 {
                old / new
            } else {
                old
            }
        }
        "average" => (old + new) * 0.5,
        _ => new,
    }
}

/// polyGeoSampler's node: the stored samples scaled, clamped and blended into the colour set, or the points pushed
/// along their normals by the samples' brightness.
pub(crate) fn geo_sample_apply(a: &Args, m: &mut Mesh, sel: &[Index], set: &str) {
    let samples = a.get("geoSamples").cloned().unwrap_or(Value::Null);
    let by = samples.get("by").and_then(Value::as_str).unwrap_or("corner").to_string();
    let values: Vec<(Index, [f32; 4])> = samples
        .get("values")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|e| {
            let n: Vec<f64> = e.as_array()?.iter().filter_map(Value::as_f64).collect();
            (n.len() == 5).then(|| (n[0] as Index, [n[1] as f32, n[2] as f32, n[3] as f32, n[4] as f32]))
        })
        .collect();
    let sf = a.float("scaleFactor", 1.0) as f32;
    let faces: BTreeSet<Index> = if sel.is_empty() { (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect() } else { sel.iter().copied().collect() };
    if a.bool("displaceGeometry") {
        // Each point moves along its normal by its samples' mean brightness times the scale.
        let mut lum: std::collections::BTreeMap<Index, (f64, f64)> = Default::default();
        for &(id, v) in &values {
            let l = (0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2]) as f64;
            let pts: Vec<Index> = match by.as_str() {
                "face" if m.face_valid(id) => m.face_points(id),
                "point" => vec![id],
                _ if m.corner_valid(id) => vec![m.corner_point(id)],
                _ => Vec::new(),
            };
            for p in pts {
                let e = lum.entry(p).or_insert((0.0, 0.0));
                e.0 += l;
                e.1 += 1.0;
            }
        }
        let normals = if a.bool("flatShading") { Vec::new() } else { aztek_geo::normals::point_normals(m, aztek_geo::normals::NormalWeight::Area) };
        for (p, (l, n)) in lum {
            if !m.point_valid(p) {
                continue;
            }
            let nrm = normals.get(p as usize).map_or(glam::DVec3::ZERO, |v| v.as_dvec3());
            m.set_position(p, m.position(p) + nrm * (l / n) * sf as f64);
        }
        return;
    }
    let set = if set.is_empty() { "colorSet1".to_string() } else { set.to_string() };
    if !aztek_geo::colors::has(m, &set) {
        aztek_geo::colors::add_white(m, &set);
    }
    let v3 = |k: &str, d: f32| -> [f32; 3] { a.vec3(k).map_or([d; 3], |v| v.map(|x| x as f32)) };
    let (cmn, cmx) = (v3("clampRGBMin", 0.0), v3("clampRGBMax", 1.0e30));
    let (amn, amx) = (a.float("clampAlphaMin", 0.0) as f32, a.float("clampAlphaMax", 1.0) as f32);
    let (cb, ab) = (a.string("colorBlend", "overwrite"), a.string("alphaBlend", "overwrite"));
    let mut per_corner: Vec<(Index, [f32; 4])> = Vec::new();
    for &(id, v) in &values {
        match by.as_str() {
            "face" if m.face_valid(id) && faces.contains(&id) => per_corner.extend(m.face_corners(id).into_iter().map(|k| (k, v))),
            "point" => {
                for &f in &faces {
                    per_corner.extend(m.face_corners(f).into_iter().filter(|&k| m.corner_point(k) == id).map(|k| (k, v)));
                }
            }
            "corner" if m.corner_valid(id) && faces.contains(&m.corner_face(id)) => per_corner.push((id, v)),
            _ => {}
        }
    }
    let colours = aztek_geo::colors::values_mut(m, &set);
    for (k, v) in per_corner {
        let Some(c) = colours.get_mut(k as usize) else { continue };
        for i in 0..3 {
            let new = (v[i] * sf).max(0.0).clamp(cmn[i].min(cmx[i]), cmx[i].max(cmn[i]));
            c[i] = blend(&cb, c[i], new);
        }
        c[3] = blend(&ab, c[3], v[3].clamp(amn.min(amx), amx.max(amn)));
    }
}

fn geo_sampler(c: &mut Commands<Session>) {
    let b = |n: &str, sh: &str, doc: &str| f(n, sh, ArgType::Bool, doc);
    let cmd = Cmd::new(
        "polyGeoSampler",
        "Sample the surface's colour and transparency as its lights (or a default light along the view) shine on it, at each \
         vertex of the faces named or selected (each face with -bf), and store them as colours (blended by -cbl and -abl), or \
         push the points along their normals by them (-dg). Returns the node.",
        |s, a| {
            if a.bool("edit") {
                return crate::history_qe::qe(s, a, "polyGeoSampler", &["polyGeoSampler"], &[]).unwrap_or(Ok(Value::Null));
            }
            crate::mesh_edit::ref_poly::run_op_with(s, "polyGeoSampler", a, &mut |g, node, m, ids, args| {
                let faces: Vec<Index> = if ids.is_empty() { (0..m.face_capacity()).filter(|&f| m.face_valid(f)).collect() } else { ids.clone() };
                let samples = geo_samples(g, node, m, &faces, args);
                args.flags.insert("geoSamples".into(), samples);
                if args.bool("colorDisplayOption") {
                    if let Some(i) = g.node(node).and_then(|n| n.ty().attr_index("displayColors")) {
                        g.set_value(crate::plug::PlugRef::new(node, i as i32), PlugValue::Bool(true))?;
                    }
                }
                Ok(())
            })
        },
    )
    .positional("faces, vertices or meshes (the selection when none)")
    .flag(f("alphaBlend", "abl", ArgType::String, "How the sampled alpha combines with the stored one: none, overwrite (the default), add, subtract, multiply, divide or average."))
    .flag(f("colorBlend", "cbl", ArgType::String, "How the sampled colour combines with the stored one, as -abl."))
    .flag(b("averageColor", "ac", "One colour per vertex, shared by its faces (off: each face's corner its own)."))
    .flag(f("clampAlphaMax", "amx", ArgType::Float, "The most the stored alpha may be."))
    .flag(f("clampAlphaMin", "amn", ArgType::Float, "The least the stored alpha may be."))
    .flag(aztek_ops::FlagDef::new("clampRGBMax", "cmx", ArgType::Vec3, json!([1.0e30, 1.0e30, 1.0e30]), "The most each stored colour channel may be."))
    .flag(aztek_ops::FlagDef::new("clampRGBMin", "cmn", ArgType::Vec3, json!([0.0, 0.0, 0.0]), "The least each stored colour channel may be."))
    .flag(b("colorDisplayOption", "cdo", "Show the mesh's vertex colours."))
    .flag(b("computeShadows", "cs", "Accepted: lights sample without shadows here."))
    .flag(b("reuseShadows", "rs", "Accepted: lights sample without shadows here."))
    .flag(b("useLightShadows", "ul", "Accepted: lights sample without shadows here."))
    .flag(b("displaceGeometry", "dg", "Push the points along their normals by the samples' brightness rather than store colours."))
    .flag(b("flatShading", "fs", "Sample with the faces' own normals rather than smooth ones."))
    .flag(b("ignoreDoubleSided", "ids", "Light only the side the normals face, even on a double-sided mesh."))
    .flag(b("lightingOnly", "lo", "Sample the light arriving rather than the lit colour."))
    .flag(b("sampleByFace", "bf", "One sample per face rather than per vertex."))
    .flag(aztek_ops::FlagDef::new("scaleFactor", "sf", ArgType::Float, json!(1.0), "Scale the samples (1); below 0 pushes points inward with -dg."))
    .flag(b("shareUV", "su", "Share UVs at each vertex; UVs are left as they are here."))
    .flag(b("edit", "e", "Edit mode: change the named polyGeoSampler node's settings."))
    .flag(f("baseColorName", "bcn", ArgType::String, "The colour set (the current one)."))
    .flag(f("geoSamples", "", ArgType::List, "The samples, worked out when the command ran."));
    let mut cmd = crate::mesh_edit::ref_poly::with_table(cmd, "polyGeoSampler", &[]);
    cmd.flags.retain(|x| !matches!(x.name.as_str(), "clampMin" | "clampMax"));
    for x in cmd.flags.iter_mut() {
        if x.name == "alphaBlend" || x.name == "colorBlend" {
            x.default_value = json!("overwrite");
        }
        if x.name == "clampAlphaMax" {
            x.default_value = json!(1.0);
        }
        if x.name == "clampAlphaMin" {
            x.default_value = json!(0.0);
        }
    }
    c.register(cmd);
}

// ---------------------------------------------------------------------------------------------------------------------
// polyMultiLayoutUV: the UVs of several meshes laid out together so none overlap

fn multi_layout(c: &mut Commands<Session>) {
    let i = |n: &str, sh: &str, doc: &str| f(n, sh, ArgType::Int, doc);
    let x = |n: &str, sh: &str, doc: &str| f(n, sh, ArgType::Float, doc);
    let cmd = Cmd::new(
        "polyMultiLayoutUV",
        "Lay out the UVs of the meshes named or selected together, so no two pieces overlap across them: each mesh's pieces packed, \
         then the meshes' blocks arranged (-l) and fitted (-sc), scaled by -su -sv and moved by -ou -ov.",
        multi_layout_run,
    )
    .positional("meshes or faces (the selection when none)")
    .flag(f("flipReversed", "fr", ArgType::Bool, "Mirror pieces whose UVs face backwards (on)."))
    .flag(i("gridU", "gu", "With -l 3: cells across."))
    .flag(i("gridV", "gv", "With -l 3: cells up."))
    .flag(i("layout", "l", "How pieces move: 0 not at all, 1 along U, 2 into a square (the default), 3 into a grid of cells, 4 to the nearest free space."))
    .flag(i("layoutMethod", "lm", "0 block stacking, 1 shape stacking."))
    .flag(x("offsetU", "ou", "Move the layout in U."))
    .flag(x("offsetV", "ov", "Move the layout in V."))
    .flag(x("percentageSpace", "ps", "With a square layout: space round each piece, percent of the square (5 at most)."))
    .flag(i("prescale", "psc", "Before: 0 pieces as they are, 1 to their sizes in object space, 2 in world space."))
    .flag(i("rotateForBestFit", "rbf", "0 no turning, 1 by right angles, 2 freely."))
    .flag(i("scale", "sc", "After: 0 no scaling, 1 uniform into the unit square (the default), 2 each way into it."))
    .flag(x("sizeU", "su", "Scale the layout in U (1)."))
    .flag(x("sizeV", "sv", "Scale the layout in V (1)."))
    .flag(f("uvSetName", "uvs", ArgType::String, "The UV set to lay out (the current one)."))
    .flag(f("createNewMap", "", ArgType::Bool, "Make the UV set named if a mesh lacks it."));
    c.register(cmd);
}

fn multi_layout_run(s: &mut Session, a: &Args) -> CmdResult {
    let targets = uv_ops::targets(s, a, "polyMultiLayoutUV")?;
    let layout = a.int("layout", 2);
    let scale = a.int("scale", 1).clamp(0, 2);
    let prescale = a.int("prescale", 0).clamp(0, 2);
    let rotate = a.int("rotateForBestFit", 1) != 0;
    let spacing = (a.float("percentageSpace", 0.2).clamp(0.0, 5.0) / 100.0).max(0.002) * 2.0;
    s.graph.evaluate();
    // Each mesh's pieces packed on their own first, at the sizes asked, into a block from 0.
    let mut jobs: Vec<(NodeId, Mesh, Vec<Index>, DVec2, DVec2)> = Vec::new();
    for (mesh, t) in &targets {
        let Some(cage) = crate::modifiers::cage(&mut s.graph, *mesh) else { continue };
        let mut m = (*cage).clone();
        let tp = uv_ops::topo(&m);
        let faces = uv_ops::target_faces(&m, t, &tp);
        let shells = uv_ops::shells_of(&tp, &faces);
        if a.get("flipReversed").and_then(Value::as_bool).unwrap_or(true) {
            for sh in &shells {
                if signed_uv_area(&m, sh) < 0.0 {
                    let corners = uv_ops::corners_of(&m, &tp, sh, &[]);
                    let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), &corners);
                    uv_ops::transform(&mut m, &corners, DVec2::ZERO, DVec2::new(-1.0, 1.0), 0.0, (lo + hi) * 0.5);
                }
            }
        }
        let all: Vec<Index> = shells.concat();
        let corners = uv_ops::corners_of(&m, &tp, &all, &[]);
        if layout != 0 {
            let p = uv_ops::Pack { spacing, margin: 0.0, relative: true, rotate, translate: true, scale: 0, pre_scale: if prescale == 0 { 0 } else { 1 }, region: (DVec2::ZERO, DVec2::splat(1.0e4)), ..Default::default() };
            uv_ops::layout_with(&mut m, &all, &p);
            if prescale == 2 {
                // World space: the object's own scale sizes its pieces too.
                let tr = crate::dag::transform_of(&s.graph, *mesh);
                let w = crate::rig::world(&mut s.graph, tr);
                let k = glam::DMat3::from_mat4(w).determinant().abs().cbrt().max(1e-9);
                uv_ops::transform(&mut m, &corners, DVec2::ZERO, DVec2::splat(k), 0.0, DVec2::ZERO);
            }
        }
        let (lo, hi) = uv_ops::bounds(m.corner_attrs().get::<[f32; 2]>(uv_ops::UV), &corners);
        jobs.push((*mesh, m, corners, lo, hi));
    }
    if jobs.is_empty() {
        fail!("polyMultiLayoutUV: select meshes with UVs");
    }
    // The blocks arranged: where each block's corner goes, and its scale.
    let n = jobs.len();
    let mut place: Vec<(DVec2, f64)> = vec![(DVec2::ZERO, 1.0); n];
    let gap = spacing;
    match layout {
        0 => {
            for (k, j) in jobs.iter().enumerate() {
                place[k] = (j.3, 1.0);
            }
        }
        1 => {
            let mut at = 0.0;
            for (k, j) in jobs.iter().enumerate() {
                place[k] = (DVec2::new(at, 0.0), 1.0);
                at += (j.4 - j.3).x + gap;
            }
        }
        3 => {
            let (gu, gv) = (a.int("gridU", 1).max(1) as usize, a.int("gridV", 1).max(1) as usize);
            let cols = gu.max(((n as f64) / gv as f64).ceil() as usize).max(1);
            let rows = n.div_ceil(cols).max(gv);
            let cell = DVec2::new(1.0 / cols as f64, 1.0 / rows as f64);
            for (k, j) in jobs.iter().enumerate() {
                let size = (j.4 - j.3).max(DVec2::splat(1e-12));
                let fit = ((cell.x - gap) / size.x).min((cell.y - gap) / size.y).max(1e-12);
                place[k] = (DVec2::new((k % cols) as f64 * cell.x, (k / cols) as f64 * cell.y), fit);
            }
        }
        _ => {
            // Rows of blocks, tallest first, about as wide as they are tall.
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by(|&p, &q| (jobs[q].4 - jobs[q].3).y.total_cmp(&(jobs[p].4 - jobs[p].3).y));
            let area: f64 = jobs.iter().map(|j| ((j.4 - j.3).x + gap) * ((j.4 - j.3).y + gap)).sum();
            let widest = jobs.iter().map(|j| (j.4 - j.3).x).fold(0.0, f64::max);
            let width = widest.max(area.sqrt() * 1.05);
            let (mut x, mut y, mut row) = (0.0f64, 0.0f64, 0.0f64);
            for k in order {
                let size = jobs[k].4 - jobs[k].3;
                if x > 0.0 && x + size.x > width {
                    x = 0.0;
                    y += row + gap;
                    row = 0.0;
                }
                place[k] = (DVec2::new(x, y), 1.0);
                x += size.x + gap;
                row = row.max(size.y);
            }
        }
    }
    // Where everything ends: fitted as asked, then sized and moved.
    let mut lo = DVec2::splat(f64::MAX);
    let mut hi = DVec2::splat(f64::MIN);
    for (k, j) in jobs.iter().enumerate() {
        let (at, sc) = place[k];
        lo = lo.min(at);
        hi = hi.max(at + (j.4 - j.3) * sc);
    }
    let size = (hi - lo).max(DVec2::splat(1e-12));
    let fit = match (scale, layout) {
        (0, _) | (_, 0) => DVec2::ONE,
        (2, _) => DVec2::new(1.0 / size.x, 1.0 / size.y),
        _ => DVec2::splat(1.0 / size.x.max(size.y)),
    };
    let sized = DVec2::new(a.float("sizeU", 1.0), a.float("sizeV", 1.0));
    let offset = DVec2::new(a.float("offsetU", 0.0), a.float("offsetV", 0.0));
    let g = &mut s.graph;
    g.open_chunk("Layout UVs");
    let r = (|| -> CmdResult<()> {
        for (k, (mesh, mut m, corners, jlo, _)) in jobs.into_iter().enumerate() {
            let (at, sc) = place[k];
            let origin = if layout == 0 { DVec2::ZERO } else { lo };
            {
                let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
                for &cn in &corners {
                    let q = DVec2::new(layer[cn as usize][0] as f64, layer[cn as usize][1] as f64);
                    let placed = if layout == 0 { q } else { (q - jlo) * sc + at };
                    let v = ((placed - origin) * fit) * sized + offset;
                    layer[cn as usize] = [v.x as f32, v.y as f32];
                }
            }
            let uvs: Vec<[f32; 2]> = m.corner_attrs().get::<[f32; 2]>(uv_ops::UV).to_vec();
            uv_ops::edit_mesh(g, mesh, "polyLayoutUV", |out| {
                let layer = out.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
                for &cn in &corners {
                    if let (Some(d), Some(v)) = (layer.get_mut(cn as usize), uvs.get(cn as usize)) {
                        *d = *v;
                    }
                }
            })?;
        }
        Ok(())
    })();
    g.close_chunk();
    r?;
    s.graph.evaluate();
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// polyProjectCurve: a live node projecting a curve onto a mesh

const PC_MESH: usize = 0;
const PC_MATRIX: usize = 1;
const PC_CURVE: usize = 2;
const PC_DIRECTION: usize = 3;
const PC_TOLERANCE: usize = 4;
const PC_UNDER: usize = 5;
const PC_OUT: usize = 6;

struct ProjectCurveNode;

/// The curve's shadow on the mesh along the direction, in world space: one point for each sample of the curve that
/// hits, kept where it bends by more than the tolerance.
fn project_curve(curve: &aztek_geo::nurbs::NurbsCurve, m: &Mesh, world: glam::DMat4, dir: glam::DVec3, tol: f64) -> Option<aztek_geo::nurbs::NurbsCurve> {
    let inverse = world.inverse();
    let dir = if dir.length_squared() < 1e-18 { glam::DVec3::NEG_Y } else { dir.normalize() };
    let local_dir = inverse.transform_vector3(dir).normalize_or_zero();
    let bvh = aztek_geo::bvh::MeshBvh::new(m);
    let reach = m.bounds().size().length() * 2.0 + 1.0 + inverse.transform_vector3(glam::DVec3::ONE).length();
    let mut pts: Vec<glam::DVec3> = Vec::new();
    for p in curve.tessellate(8) {
        let at = inverse.transform_point3(p);
        let mid = at - local_dir * local_dir.dot(at - m.bounds().center());
        if let Some(h) = bvh.raycast_with(mid - local_dir * reach, local_dir, reach * 3.0, true) {
            pts.push(world.transform_point3(h.point));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    // Points that lie within the tolerance of the line through their neighbours go.
    if tol > 0.0 && pts.len() > 2 {
        let mut kept = vec![pts[0]];
        for i in 1..pts.len() - 1 {
            let (a, b, c) = (*kept.last().unwrap(), pts[i], pts[i + 1]);
            let ac = c - a;
            let off = if ac.length_squared() < 1e-18 { (b - a).length() } else { (b - a).cross(ac).length() / ac.length() };
            if off > tol {
                kept.push(b);
            }
        }
        kept.push(*pts.last().unwrap());
        pts = kept;
    }
    Some(aztek_geo::nurbs::NurbsCurve::through(1, &pts))
}

impl crate::node::NodeBehavior for ProjectCurveNode {
    fn compute(&mut self, _ctx: &crate::node::EvalContext, d: &mut NodeData) {
        let mesh = d.value(PC_MESH).as_mesh().cloned();
        let curve = crate::curves::curve_value(d.value(PC_CURVE)).cloned();
        let out = match (mesh, curve) {
            (Some(m), Some(c)) => project_curve(&c, &m, d.matrix(PC_MATRIX), d.vec3(PC_DIRECTION), d.float(PC_TOLERANCE)).map(|mut out| {
                // Under the mesh's transform the curve is in the mesh's space.
                if d.bool(PC_UNDER) {
                    let inv = d.matrix(PC_MATRIX).inverse();
                    out.cvs.iter_mut().for_each(|p| *p = inv.transform_point3(*p));
                }
                out
            }),
            _ => None,
        };
        d.set_output(PC_OUT, out.map(crate::curves::to_value).unwrap_or(PlugValue::Data(None)));
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn register_project_curve_type() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let hidden_in = flags::INPUT | flags::STORABLE | flags::HIDDEN;
        let t = crate::node::NodeType {
            name: "polyProjectCurve".into(),
            icon: "chart-line".into(),
            doc: "A curve's shadow on a mesh along a direction, as a curve.".into(),
            own_attrs: vec![
                crate::attr_defs::mesh_attr("inputMesh", "im", flags::INPUT | flags::HIDDEN, "The mesh projected onto."),
                crate::attr_defs::matrix_attr("inputMatrix", flags::INPUT | flags::HIDDEN, "The mesh's world matrix."),
                crate::curves::curve_attr("inputCurve", "ic", hidden_in, "The curve projected, in world space."),
                crate::node::vec3_attr("direction", "d", glam::DVec3::new(0.0, 0.0, 1.0), flags::INPUT | flags::STORABLE | flags::KEYABLE, "The way the curve is projected."),
                crate::attr_defs::float_attr_short("tolerance", "tol", 0.01, "How closely the result follows the shadow.", Some(0.0), None),
                AttrDef { name: "addUnderTransform".into(), short_name: "aut".into(), ty: PlugType::Bool, default_value: PlugValue::Bool(true), flags: flags::INPUT | flags::STORABLE, doc: "The result was put under the mesh's transform.".into(), ..Default::default() },
                crate::curves::curve_attr("curveOut", "co", flags::OUTPUT | flags::HIDDEN, "The projected curve, in world space."),
            ],
            create: Some(std::sync::Arc::new(|| Box::new(ProjectCurveNode))),
            ..Default::default()
        };
        crate::node::types::register_type(t).expect("polyProjectCurve");
    });
}

fn project_curve_cmd(c: &mut Commands<Session>) {
    register_project_curve_type();
    let lin = aztek_ops::units::Unit::Linear;
    let mut d = aztek_ops::FlagDef::new("direction", "d", ArgType::Vec3, Value::Null, "The way to project (the camera's view).");
    d.unit = lin;
    let mut tol = aztek_ops::FlagDef::new("tolerance", "tol", ArgType::Float, Value::Null, "How closely the result follows the shadow (0.01).");
    tol.unit = lin;
    let mut cmd = Cmd::new(
        "polyProjectCurve",
        "Project a curve onto a mesh along -d (the camera's view when left out): a curve lying on the surface, made by a live \
         polyProjectCurve node, for polySplit -pc. Takes the curve and the mesh, either way round. Returns the curve and the node.",
        project_curve_run,
    )
    .positional("the curve and the mesh")
    .flag(f("addUnderTransform", "aut", ArgType::Bool, "Put the result under the mesh's transform (on)."))
    .flag(d)
    .flag(tol);
    for (n, sh) in [("directionX", "dx"), ("directionY", "dy"), ("directionZ", "dz")] {
        let mut x = f(n, sh, ArgType::Float, "A part of the direction.");
        x.unit = lin;
        cmd = cmd.flag(x);
    }
    let cmd = crate::mesh_edit::ref_poly::with_table(cmd, "polyProjectCurve", &[]);
    c.register(cmd);
    crate::history_qe::extend(c, "polyProjectCurve", &["polyProjectCurve"], &[], Vec::new());
}

fn project_curve_run(s: &mut Session, a: &Args) -> CmdResult {
    let names = a.named();
    let [x, y] = names.as_slice() else { fail!("polyProjectCurve: give a curve and a mesh") };
    let g = &mut s.graph;
    let is_curve = |g: &crate::graph::Graph, n: &str| g.find(n).ok().flatten().and_then(|id| crate::curves::as_curve(g, id)).is_some();
    let (curve_name, mesh_name) = if is_curve(g, x) { (x, y) } else { (y, x) };
    let Some(cid) = g.find(curve_name)? else { fail!("polyProjectCurve: no curve {curve_name}") };
    let Some(cshape) = crate::curves::as_curve(g, cid) else { fail!("polyProjectCurve: {curve_name} is not a curve") };
    let mesh = crate::mesh_edit::mesh_arg(g, mesh_name, "polyProjectCurve")?;
    let mesh_tr = crate::dag::transform_of(g, mesh);
    let folded = crate::history::fold_vectors(a);
    let dir = match folded.vec3("direction") {
        Some(v) => glam::DVec3::from_array(v),
        None => {
            // The camera's view.
            let cam = g.find("persp").ok().flatten().map(|id| crate::rig::world(g, crate::dag::transform_of(g, id)));
            cam.map_or(glam::DVec3::NEG_Y, |w| -w.z_axis.truncate())
        }
    };
    let t = crate::node::types::find("polyProjectCurve").expect("polyProjectCurve");
    let node = g.create_node(&t, "polyProjectCurve1", crate::plug::NO_NODE, true)?;
    let p = |n: NodeId, i: usize| crate::plug::PlugRef::new(n, i as i32);
    g.set_value(p(node, PC_DIRECTION), PlugValue::Vec3(dir))?;
    g.set_value(p(node, PC_TOLERANCE), PlugValue::Float(a.float("tolerance", 0.01)))?;
    let under = a.get("addUnderTransform").and_then(Value::as_bool).unwrap_or(true);
    g.set_value(p(node, PC_UNDER), PlugValue::Bool(under))?;
    g.connect(&p(mesh, crate::mesh_nodes::MeshNode::OUT_MESH), &p(node, PC_MESH), false)?;
    g.connect(&p(mesh_tr, crate::transform::Transform::WORLD_MATRIX), &p(node, PC_MATRIX), false)?;
    g.connect(&p(cshape, crate::curves::CurveShape::WORLD_SPACE), &p(node, PC_CURVE), false)?;
    g.evaluate();
    if crate::curves::curve_value(&g.value(&p(node, PC_OUT)).map_err(|e| aztek_ops::CommandError::new(e.to_string()))?).is_none() {
        g.delete_node(node)?;
        fail!("polyProjectCurve: {curve_name} misses {mesh_name} along that direction");
    }
    let base = g.node(cid).map(|n| n.name.clone()).unwrap_or_default();
    let (tr, sh) = crate::curves::create_curve(g, &format!("{base}_projected1"), aztek_geo::nurbs::NurbsCurve::default(), false)?;
    g.connect(&p(node, PC_OUT), &p(sh, crate::curves::CurveShape::CREATE), true)?;
    g.evaluate();
    let mut curve_path = g.path(tr);
    let node_path = g.path(node);
    if under {
        // Under the mesh's transform, keeping where it lies in the world.
        let r = s.run("parent", json!({"args": [curve_path.clone(), s.graph.path(mesh_tr)]}))?;
        if let Some(n) = r.as_array().and_then(|l| l.first()).and_then(Value::as_str).or(r.as_str()) {
            curve_path = n.to_string();
        }
        // The node gives the curve in the mesh's space: its transform stays at rest under the mesh.
        if let Some(tr) = s.graph.find(&curve_path).ok().flatten() {
            crate::transform::Transform::set_local_matrix(&mut s.graph, tr, &glam::DMat4::IDENTITY).map_err(|e| aztek_ops::CommandError::new(e.to_string()))?;
        }
        s.graph.evaluate();
    }
    crate::history_qe::apply_common(&mut s.graph, node, a)?;
    Ok(json!([curve_path, node_path]))
}
