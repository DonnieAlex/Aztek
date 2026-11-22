//! geomToBBox: polygon boxes standing for the selected geometry's world bounds, one per shape or one per selected
//! hierarchy, with a lambert of their own, optionally combined, the originals kept or deleted, and the bounds' motion
//! baked as keys.

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::plug::NodeId;
use crate::session::Session;

fn names(v: &Value) -> Vec<String> {
    match v {
        Value::String(s) => vec![s.clone()],
        Value::Array(a) => a.iter().flat_map(names).collect(),
        _ => Vec::new(),
    }
}

fn is_geometry(s: &Session, id: NodeId) -> bool {
    let g = &s.graph;
    g.node(id).is_some_and(|n| {
        let t = n.ty();
        (t.is_a("mesh") || t.is_a("nurbsSurface") || t.is_a("nurbsCurve") || t.is_a("subdiv")) && !n.bool(crate::transform::DagNode::INTERMEDIATE_OBJECT)
    })
}

fn bounds(s: &mut Session, of: &str) -> CmdResult<[f64; 6]> {
    let v = s.run("exactWorldBoundingBox", json!({"args": [of]}))?;
    let a: Vec<f64> = v.as_array().map(|x| x.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
    if a.len() != 6 {
        fail!("geomToBBox: no bounds for {of}");
    }
    Ok([a[0], a[1], a[2], a[3], a[4], a[5]])
}

/// Where a box goes and how big it is.
fn place(b: [f64; 6]) -> ([f64; 3], [f64; 3]) {
    let c = [(b[0] + b[3]) / 2.0, (b[1] + b[4]) / 2.0, (b[2] + b[5]) / 2.0];
    let size = [(b[3] - b[0]).max(1e-6), (b[4] - b[1]).max(1e-6), (b[5] - b[2]).max(1e-6)];
    (c, size)
}

fn set_box(s: &mut Session, name: &str, b: [f64; 6]) -> CmdResult<()> {
    let (c, size) = place(b);
    for (i, ax) in ["X", "Y", "Z"].iter().enumerate() {
        s.run("setAttr", json!({"args": [format!("{name}.translate{ax}"), c[i]]}))?;
        s.run("setAttr", json!({"args": [format!("{name}.scale{ax}"), size[i]]}))?;
    }
    Ok(())
}

fn geom_to_bbox(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("combineMesh") && a.bool("single") {
        fail!("geomToBBox: -combineMesh can't be used with -single");
    }
    let picked: Vec<NodeId> = s.graph.selection().to_vec();
    if picked.is_empty() {
        fail!("geomToBBox: select the geometry to make boxes for");
    }
    let suffix = a.string("nameSuffix", "_BBox");
    let base = a.string("name", "");
    // What each box stands for: a selected hierarchy, or each shape in one.
    let mut targets: Vec<(String, String)> = Vec::new();
    for &id in &picked {
        let path = s.graph.path(id);
        let leaf = |p: &str| p.rsplit('|').next().unwrap_or(p).rsplit(':').next().unwrap_or(p).to_string();
        if a.bool("single") {
            targets.push((path.clone(), leaf(&path)));
            continue;
        }
        let mut under = vec![id];
        under.extend(s.graph.descendants(id));
        for n in under {
            if is_geometry(s, n) {
                let tr = crate::dag::transform_of(&s.graph, n);
                let p = s.graph.path(tr);
                if !targets.iter().any(|t| t.0 == s.graph.path(n)) {
                    targets.push((s.graph.path(n), leaf(&p)));
                }
            }
        }
    }
    if targets.is_empty() {
        fail!("geomToBBox: the selection holds no geometry");
    }
    let c = a.get("shaderColor").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_f64).collect::<Vec<_>>()).filter(|v| v.len() == 3).unwrap_or(vec![0.5, 0.5, 0.5]);
    let mat = s.run("shadingNode", json!({"args": ["lambert"], "asShader": true, "name": format!("{}{}_lambert", if base.is_empty() { "BBox" } else { &base }, suffix)}))?;
    let mat = mat.as_str().unwrap_or_default().to_string();
    s.run("setAttr", json!({"args": [format!("{mat}.color"), c[0], c[1], c[2]], "type": "double3"}))?;
    let sg = s.run("sets", json!({"renderable": true, "noSurfaceShader": true, "empty": true, "name": format!("{mat}SG")}))?;
    let sg = names(&sg).into_iter().next().unwrap_or_default();
    s.run("connectAttr", json!({"args": [format!("{mat}.outColor"), format!("{sg}.surfaceShader")], "force": true}))?;
    let bake = a.bool("bakeAnimation");
    let mut boxes: Vec<String> = Vec::new();
    for (i, (of, leaf)) in targets.iter().enumerate() {
        let stem = if base.is_empty() { leaf.clone() } else if targets.len() > 1 { format!("{base}{}", i + 1) } else { base.clone() };
        let made = s.run("polyCube", json!({"width": 1.0, "height": 1.0, "depth": 1.0, "constructionHistory": false, "name": format!("{stem}{suffix}")}))?;
        let name = names(&made).into_iter().next().unwrap_or_default();
        let b = bounds(s, of)?;
        set_box(s, &name, b)?;
        s.run("sets", json!({"forceElement": sg, "args": [name.clone()]}))?;
        boxes.push(name);
    }
    if bake {
        let range = |s: &mut Session, k: &str| s.run("playbackOptions", json!({"query": true, "queried": [k]})).ok().and_then(|v| v.as_f64());
        let start = a.get("startTime").and_then(Value::as_f64).or_else(|| range(s, "minTime")).unwrap_or(1.0);
        let end = a.get("endTime").and_then(Value::as_f64).or_else(|| range(s, "maxTime")).unwrap_or(start);
        let by = a.float("sampleBy", 1.0).abs().max(1e-3);
        let was = crate::anim::frame(&s.graph);
        let mut t = start;
        while t <= end + 1e-9 {
            s.run("currentTime", json!({"args": [t]}))?;
            for (i, (of, _)) in targets.iter().enumerate() {
                let b = bounds(s, of)?;
                let (c, size) = place(b);
                for (k, ax) in ["X", "Y", "Z"].iter().enumerate() {
                    s.run("setKeyframe", json!({"args": [boxes[i].clone()], "attribute": format!("translate{ax}"), "time": t, "value": c[k]}))?;
                    s.run("setKeyframe", json!({"args": [boxes[i].clone()], "attribute": format!("scale{ax}"), "time": t, "value": size[k]}))?;
                }
            }
            t += by;
        }
        s.run("currentTime", json!({"args": [was]}))?;
    }
    if a.bool("combineMesh") && boxes.len() > 1 {
        let name = if base.is_empty() { format!("combined{suffix}") } else { format!("{base}{suffix}") };
        let r = s.run("polyUnite", json!({"args": boxes.clone(), "constructionHistory": false, "name": name}))?;
        boxes = names(&r).into_iter().take(1).collect();
        if let Some(b) = boxes.first() {
            s.run("sets", json!({"forceElement": sg, "args": [b.clone()]}))?;
        }
    }
    if !a.bool("keepOriginal") {
        let doomed: Vec<String> = picked.iter().filter(|&&i| s.graph.node(i).is_some()).map(|&i| s.graph.path(i)).collect();
        s.run("delete", json!({"args": doomed}))?;
    }
    let ids: Vec<NodeId> = boxes.iter().filter_map(|b| s.graph.find(b).ok().flatten()).collect();
    s.graph.select(&ids, crate::graph::SelectMode::Replace);
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    c.register(
        CommandDef::new("geomToBBox", "Polygon boxes for the selected geometry's world bounds: one per shape, or one per selected hierarchy with -single.", geom_to_bbox)
            .flag(f("bakeAnimation", "ba", ArgType::Bool, "Key the boxes to follow the bounds from -startTime to -endTime by -sampleBy (the time slider's range by default)."))
            .flag(f("combineMesh", "cm", ArgType::Bool, "Combine the boxes into one mesh. Not with -single."))
            .flag(f("endTime", "et", ArgType::Float, "Where baking ends."))
            .flag(f("keepOriginal", "ko", ArgType::Bool, "Keep the selected objects; otherwise they are deleted."))
            .flag(f("name", "n", ArgType::String, "The boxes' name."))
            .flag(f("nameSuffix", "ns", ArgType::String, "Put after each box's name (_BBox)."))
            .flag(f("sampleBy", "sb", ArgType::Float, "The step between baked samples (1)."))
            .flag(f("shaderColor", "sc", ArgType::Vec3, "The boxes' lambert colour, 0 to 1 (0.5 0.5 0.5)."))
            .flag(f("single", "s", ArgType::Bool, "One box per selected hierarchy."))
            .flag(f("startTime", "st", ArgType::Float, "Where baking starts.")),
    );
}
