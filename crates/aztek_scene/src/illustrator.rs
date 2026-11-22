//! illustratorCurves: NURBS curves from an Illustrator file's paths. Both kinds of file are read: the PostScript
//! based ones (path operators in the text) and the PDF based ones (the same operators in the page streams, inflated
//! when compressed). Each subpath becomes a cubic Bezier-knotted curve in the XY plane, one unit to a point. The
//! illustratorCurves node keeps the file and tolerance; editing it makes the curves again.

use std::any::Any;
use std::sync::Arc;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::DVec3;
use serde_json::{json, Value};

use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, PlugType, PlugValue};
use crate::session::Session;

pub const TYPE: &str = "illustratorCurves";

struct Plain;

impl NodeBehavior for Plain {
    fn compute(&mut self, _ctx: &EvalContext, _d: &mut NodeData) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let attr = |name: &str, short: &str, ty: PlugType, v: PlugValue| AttrDef { name: name.into(), short_name: short.into(), ty, default_value: v, flags: flags::INPUT | flags::STORABLE, ..Default::default() };
        let _ = types::register_type(NodeType {
            name: TYPE.into(),
            doc: "Curves read from an Illustrator file: the file, how close CVs snap together, and the group the curves are in.".into(),
            own_attrs: vec![
                attr("illustratorFilename", "ifn", PlugType::String, PlugValue::String(String::new())),
                attr("tolerance", "tl", PlugType::Float, PlugValue::Float(0.001)),
                attr("outputGroup", "og", PlugType::String, PlugValue::String(String::new())),
            ],
            create: Some(Arc::new(|| Box::new(Plain))),
            ..Default::default()
        });
    });
}

/// A subpath: Bezier control points, 3n+1 of them, and whether it closes.
#[derive(Debug, Default, Clone)]
pub struct Path {
    pub cvs: Vec<DVec3>,
}

fn inflate_streams(bytes: &[u8]) -> String {
    // PDF content: each `stream ... endstream`, inflated when it was deflated.
    let mut out = String::new();
    let mut at = 0;
    while let Some(i) = find(bytes, b"stream", at) {
        let mut start = i + 6;
        if bytes.get(start) == Some(&b'\r') {
            start += 1;
        }
        if bytes.get(start) == Some(&b'\n') {
            start += 1;
        }
        let Some(end) = find(bytes, b"endstream", start) else { break };
        let head_from = i.saturating_sub(400);
        let head = String::from_utf8_lossy(&bytes[head_from..i]);
        let data = &bytes[start..end];
        if head.rsplit("obj").next().unwrap_or("").contains("FlateDecode") {
            if let Ok(v) = miniz_oxide::inflate::decompress_to_vec_zlib(data) {
                out.push_str(&String::from_utf8_lossy(&v));
                out.push('\n');
            }
        } else {
            out.push_str(&String::from_utf8_lossy(data));
            out.push('\n');
        }
        at = end + 9;
    }
    out
}

fn find(h: &[u8], n: &[u8], from: usize) -> Option<usize> {
    if from >= h.len() {
        return None;
    }
    h[from..].windows(n.len()).position(|w| w == n).map(|p| p + from)
}

/// The paths in the text: m, l/L, c/C, v/V, y/Y, re and the closing and painting operators.
pub fn parse_paths(text: &str) -> Vec<Path> {
    let mut out: Vec<Path> = Vec::new();
    let mut nums: Vec<f64> = Vec::new();
    let mut cur: Option<Path> = None;
    let mut start = DVec3::ZERO;
    let p = |x: f64, y: f64| DVec3::new(x, y, 0.0);
    let line_to = |path: &mut Path, to: DVec3| {
        let from = *path.cvs.last().unwrap();
        path.cvs.push(from + (to - from) / 3.0);
        path.cvs.push(from + (to - from) * 2.0 / 3.0);
        path.cvs.push(to);
    };
    let finish = |cur: &mut Option<Path>, out: &mut Vec<Path>| {
        if let Some(path) = cur.take() {
            if path.cvs.len() >= 4 {
                out.push(path);
            }
        }
    };
    for line in text.lines() {
        // PostScript comments, and the prolog's procedures, are not paths.
        let line = line.split('%').next().unwrap_or("");
        for w in line.split_whitespace() {
            if let Ok(v) = w.parse::<f64>() {
                nums.push(v);
                continue;
            }
            let n = nums.len();
            match w {
                "m" if n >= 2 => {
                    finish(&mut cur, &mut out);
                    start = p(nums[n - 2], nums[n - 1]);
                    cur = Some(Path { cvs: vec![start] });
                }
                "l" | "L" if n >= 2 => {
                    if let Some(c) = cur.as_mut() {
                        line_to(c, p(nums[n - 2], nums[n - 1]));
                    }
                }
                "c" | "C" if n >= 6 => {
                    if let Some(c) = cur.as_mut() {
                        c.cvs.extend([p(nums[n - 6], nums[n - 5]), p(nums[n - 4], nums[n - 3]), p(nums[n - 2], nums[n - 1])]);
                    }
                }
                "v" | "V" if n >= 4 => {
                    if let Some(c) = cur.as_mut() {
                        let from = *c.cvs.last().unwrap();
                        c.cvs.extend([from, p(nums[n - 4], nums[n - 3]), p(nums[n - 2], nums[n - 1])]);
                    }
                }
                "y" | "Y" if n >= 4 => {
                    if let Some(c) = cur.as_mut() {
                        let end = p(nums[n - 2], nums[n - 1]);
                        c.cvs.extend([p(nums[n - 4], nums[n - 3]), end, end]);
                    }
                }
                "re" if n >= 4 => {
                    finish(&mut cur, &mut out);
                    let (x, y, w2, h) = (nums[n - 4], nums[n - 3], nums[n - 2], nums[n - 1]);
                    let mut r = Path { cvs: vec![p(x, y)] };
                    for to in [p(x + w2, y), p(x + w2, y + h), p(x, y + h), p(x, y)] {
                        line_to(&mut r, to);
                    }
                    out.push(r);
                }
                "h" | "H" | "s" | "b" | "f" | "n" | "b*" | "f*" => {
                    if let Some(c) = cur.as_mut() {
                        // A closing operator draws back to the start.
                        if matches!(w, "h" | "s" | "b" | "b*") || (matches!(w, "f" | "f*" | "n") && text.contains("%!PS-Adobe")) {
                            let last = *c.cvs.last().unwrap();
                            if last.distance(start) > 1e-9 {
                                line_to(c, start);
                            }
                        }
                    }
                    if w != "h" && w != "H" {
                        finish(&mut cur, &mut out);
                    }
                }
                "S" | "B" | "F" | "N" | "B*" => finish(&mut cur, &mut out),
                _ => {}
            }
            nums.clear();
        }
    }
    finish(&mut cur, &mut out);
    out
}

pub fn read_file(path: &str) -> Result<Vec<Path>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let text = if bytes.starts_with(b"%PDF") { inflate_streams(&bytes) } else { String::from_utf8_lossy(&bytes).into_owned() };
    Ok(parse_paths(&text))
}

/// CVs closer to the one before than the tolerance are snapped onto it.
fn snapped(mut cvs: Vec<DVec3>, tol: f64) -> Vec<DVec3> {
    for i in 1..cvs.len() {
        if cvs[i].distance(cvs[i - 1]) < tol {
            cvs[i] = cvs[i - 1];
        }
    }
    cvs
}

fn make_curves(s: &mut Session, path: &str, tol: f64, group: &str) -> CmdResult<Vec<String>> {
    let paths = read_file(path).map_err(aztek_ops::CommandError::new)?;
    if paths.is_empty() {
        fail!("illustratorCurves: {path} holds no paths");
    }
    let mut made = Vec::new();
    for (i, p) in paths.iter().enumerate() {
        let cvs = snapped(p.cvs.clone(), tol);
        let spans = (cvs.len() - 1) / 3;
        let mut knots = Vec::new();
        for k in 0..=spans {
            knots.extend([k as f64; 3]);
        }
        let pts: Vec<Vec<f64>> = cvs.iter().map(|v| vec![v.x, v.y, v.z]).collect();
        let r = s.run("curve", json!({"degree": 3, "point": pts, "knot": knots, "name": format!("illustratorCurve{}", i + 1)}))?;
        let name = r.as_str().map(str::to_string).or_else(|| r[0].as_str().map(str::to_string)).unwrap_or_default();
        let r = s.run("parent", json!({"args": [name, group], "relative": true}))?;
        made.push(r.as_array().and_then(|v| v.first()).and_then(Value::as_str).map(str::to_string).unwrap_or_default());
    }
    Ok(made)
}

fn set(s: &mut Session, plug: &str, v: PlugValue) -> CmdResult<()> {
    let p = s.graph.plug(plug).map_err(|e| aztek_ops::CommandError::new(e.to_string()))?;
    s.graph.set_value(p, v).map_err(|e| aztek_ops::CommandError::new(e.to_string()))?;
    Ok(())
}

fn get(s: &mut Session, plug: &str) -> Option<PlugValue> {
    let p = s.graph.plug(plug).ok()?;
    s.graph.value(&p).ok()
}

fn illustrator_curves(s: &mut Session, a: &Args) -> CmdResult {
    register_types();
    let asked = |n: &str| crate::ref_core::asked(a, n);
    if a.bool("query") || a.bool("edit") {
        let Some(node) = a.pos_str(0).map(str::to_string) else { fail!("illustratorCurves: name an illustratorCurves node") };
        if !s.graph.find(&node).ok().flatten().is_some_and(|id| s.graph.node(id).is_some_and(|n| n.ty().name == TYPE)) {
            fail!("illustratorCurves: {node} is not an illustratorCurves node");
        }
        if a.bool("query") {
            let v = if asked("tolerance") {
                get(s, &format!("{node}.tolerance"))
            } else if asked("caching") {
                get(s, &format!("{node}.caching"))
            } else if asked("nodeState") {
                get(s, &format!("{node}.nodeState"))
            } else {
                fail!("illustratorCurves: query -tolerance, -caching or -nodeState");
            };
            return Ok(match v {
                Some(PlugValue::Float(f)) => json!(f),
                Some(PlugValue::Int(i)) => json!(i),
                Some(PlugValue::Bool(b)) => json!(b),
                _ => Value::Null,
            });
        }
        if a.has("caching") {
            set(s, &format!("{node}.caching"), PlugValue::Bool(a.bool("caching")))?;
        }
        if a.has("nodeState") {
            set(s, &format!("{node}.nodeState"), PlugValue::Int(a.int("nodeState", 0)))?;
        }
        if a.has("tolerance") {
            set(s, &format!("{node}.tolerance"), PlugValue::Float(a.float("tolerance", 0.001)))?;
            // The curves again, with the new tolerance.
            let file = match get(s, &format!("{node}.illustratorFilename")) {
                Some(PlugValue::String(t)) => t,
                _ => String::new(),
            };
            let group = match get(s, &format!("{node}.outputGroup")) {
                Some(PlugValue::String(t)) => t,
                _ => String::new(),
            };
            if let Ok(Some(gid)) = s.graph.find(&group) {
                let kids: Vec<String> = s.graph.children(gid).iter().map(|&c| s.graph.path(c)).collect();
                if !kids.is_empty() {
                    s.run("delete", json!({"args": kids}))?;
                }
                make_curves(s, &file, a.float("tolerance", 0.001), &group)?;
            }
        }
        return Ok(json!(node));
    }
    let Some(file) = a.get("illustratorFilename").and_then(Value::as_str).map(str::to_string) else { fail!("illustratorCurves: give -illustratorFilename") };
    let tol = a.float("tolerance", 0.001);
    let history = !a.has("constructionHistory") || a.bool("constructionHistory");
    let object = !a.has("object") || a.bool("object");
    let make_node = |s: &mut Session, group: &str| -> CmdResult<String> {
        let n = s.run("createNode", json!({"args": [TYPE], "name": "illustratorCurves1"}))?;
        let n = n.as_str().unwrap_or_default().to_string();
        set(s, &format!("{n}.illustratorFilename"), PlugValue::String(file.clone()))?;
        set(s, &format!("{n}.tolerance"), PlugValue::Float(tol))?;
        set(s, &format!("{n}.outputGroup"), PlugValue::String(group.to_string()))?;
        if a.has("caching") {
            set(s, &format!("{n}.caching"), PlugValue::Bool(a.bool("caching")))?;
        }
        if a.has("nodeState") {
            set(s, &format!("{n}.nodeState"), PlugValue::Int(a.int("nodeState", 0)))?;
        }
        Ok(n)
    };
    if !object {
        // The node alone.
        read_file(&file).map_err(aztek_ops::CommandError::new)?;
        let n = make_node(s, "")?;
        return Ok(json!([n]));
    }
    let g = s.run("group", json!({"empty": true, "name": "illustratorCurvesGroup1"}))?;
    let group = g.as_str().map(str::to_string).or_else(|| g[0].as_str().map(str::to_string)).unwrap_or_default();
    make_curves(s, &file, tol, &group)?;
    let mut out = vec![group.clone()];
    if history {
        out.push(make_node(s, &group)?);
    }
    if let Ok(Some(id)) = s.graph.find(&group) {
        s.graph.select(&[id], crate::graph::SelectMode::Replace);
    }
    Ok(json!(out))
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    c.register(
        CommandDef::new("illustratorCurves", "NURBS curves from an Illustrator file's paths, grouped. Returns the group and the illustratorCurves node.", illustrator_curves)
            .positional("[node]")
            .flag(f("caching", "cch", ArgType::Bool, "Keep the node's results so they are not computed again."))
            .flag(f("constructionHistory", "ch", ArgType::Bool, "Make the illustratorCurves node (on)."))
            .flag(f("illustratorFilename", "ifn", ArgType::String, "The Illustrator file."))
            .flag(f("nodeState", "nds", ArgType::Int, "0 normal, 1 has no effect, 2 blocking."))
            .flag(f("object", "o", ArgType::Bool, "Make the curves (on); off makes only the node."))
            .flag(f("tolerance", "tl", ArgType::Float, "Contiguous CVs closer than this snap together (0.001)."))
            .flag(f("query", "q", ArgType::Bool, "Query mode."))
            .flag(f("edit", "e", ArgType::Bool, "Edit mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode.")),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn paths_from_postscript() {
        let t = "%!PS-Adobe-3.0\n0 0 m\n10 0 l\n10 10 20 10 20 0 c\nS\n5 5 10 10 re\nf\n";
        let p = super::parse_paths(t);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].cvs.len(), 7);
        assert_eq!(p[1].cvs.len(), 13);
    }
}
