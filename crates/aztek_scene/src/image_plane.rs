//! Image planes, as the reference's imagePlane node and command: a picture (or a numbered sequence of them) shown on a
//! camera, at a depth in front of it and fitted to its film gate, or free in the scene at a width and height. The
//! node's attributes and defaults are the reference's 2027 ones (its node documentation); the views draw them.

use std::sync::Arc;

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec3};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::node::{types, EvalContext, NodeBehavior, NodeData, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue, NO_NODE};
use crate::session::Session;
use crate::transform::{DagNode, Shape};

pub const TYPE: &str = "imagePlane";

const STORE: u32 = flags::INPUT | flags::STORABLE;

fn a(name: &str, short: &str, ty: PlugType, d: PlugValue, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: d, flags: STORE, doc: doc.into(), ..Default::default() }
}
fn fl(name: &str, short: &str, d: f64, doc: &str) -> AttrDef {
    a(name, short, PlugType::Float, PlugValue::Float(d), doc)
}
fn dist(name: &str, short: &str, d: f64, doc: &str) -> AttrDef {
    AttrDef { unit: aztek_ops::units::Unit::Linear, ..fl(name, short, d, doc) }
}
fn int(name: &str, short: &str, d: i64, doc: &str) -> AttrDef {
    a(name, short, PlugType::Int, PlugValue::Int(d), doc)
}
fn boolean(name: &str, short: &str, d: bool, doc: &str) -> AttrDef {
    a(name, short, PlugType::Bool, PlugValue::Bool(d), doc)
}
fn en(name: &str, short: &str, d: i64, names: &[&str], doc: &str) -> AttrDef {
    AttrDef { enum_names: names.iter().map(|x| x.to_string()).collect(), ..a(name, short, PlugType::Enum, PlugValue::Int(d), doc) }
}
fn text(name: &str, short: &str, doc: &str) -> AttrDef {
    a(name, short, PlugType::String, PlugValue::String(String::new()), doc)
}
fn vec(name: &str, short: &str, kids: [(&str, &str); 3], d: [f64; 3], color: bool, doc: &str) -> AttrDef {
    let mut x = a(name, short, PlugType::Vec3, PlugValue::Vec3(DVec3::from_array(d)), doc);
    if color {
        x.flags |= flags::COLOR;
    }
    for (k, (n, sh)) in kids.into_iter().enumerate() {
        x.child_names[k] = n.into();
        x.child_short[k] = sh.into();
    }
    x
}

struct ImagePlane;

impl NodeBehavior for ImagePlane {
    fn compute(&mut self, _ctx: &EvalContext, d: &mut NodeData) {
        Shape::compute_matrices(d);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let shape = types::find("shape").expect("shape");
        let own = vec![
            text("imageName", "imn", "The picture: a file, or the first of a numbered sequence."),
            en("type", "t", 0, &["Image File", "Texture", "Movie"], "What it shows. Image files are drawn."),
            en("displayMode", "dm", 3, &["None", "Outline", "RGB", "RGBA", "Luminance", "RGBA Alpha"], "How it draws."),
            fl("alphaGain", "ag", 1.0, "A multiplier on its alpha."),
            boolean("alreadyPremult", "pre", false, "Its colour is already multiplied by its alpha."),
            vec("colorGain", "cg", [("colorGainR", "cgr"), ("colorGainG", "cgg"), ("colorGainB", "cgb")], [1.0; 3], true, "A multiplier on its colour."),
            vec("colorOffset", "cof", [("colorOffsetR", "cor"), ("colorOffsetG", "cog"), ("colorOffsetB", "cob")], [0.0; 3], true, "Added to its colour."),
            dist("depth", "d", 100.0, "How far in front of the camera it sits."),
            en("fit", "f", 1, &["Fill", "Best", "Horizontal", "Vertical", "To Size"], "How it fits the camera's film gate."),
            boolean("displayOnlyIfCurrent", "dic", false, "Drawn only when looking through its camera."),
            boolean("lockedToCamera", "dlc", true, "It follows its camera; off, it stays where it is in the scene."),
            boolean("useFrameExtension", "ufe", false, "A numbered sequence: the frame's number in the file name."),
            int("frameExtension", "fe", 1, "The frame of the sequence shown, when nothing else drives it."),
            int("frameOffset", "fo", 0, "Added to the frame."),
            int("frameCache", "fc", 24, "Frames kept loaded."),
            int("frameIn", "fin", -1, "First frame of the sequence shown; -1 none."),
            int("frameOut", "fot", -1, "Last frame shown; -1 none."),
            dist("width", "w", 0.0, "Width of a free plane."),
            dist("height", "h", 0.0, "Height of a free plane."),
            boolean("maintainRatio", "mr", true, "Width and height keep the picture's proportions."),
            vec("imageCenter", "ic", [("imageCenterX", "icx"), ("imageCenterY", "icy"), ("imageCenterZ", "icz")], [0.0; 3], false, "Where a free plane's middle is."),
            vec("offset", "o", [("offsetX", "ox"), ("offsetY", "oy"), ("", "")], [0.0; 3], false, "Offset in the film gate, inches."),
            vec("size", "s", [("sizeX", "sx"), ("sizeY", "sy"), ("", "")], [1.417_322_834_645_669_3, 0.944_881_889_763_779_6, 0.0], false, "Size in the film gate for To Size, inches."),
            AttrDef { unit: aztek_ops::units::Unit::Angle, ..fl("rotate", "r", 0.0, "Turn in the gate.") },
            fl("squeezeCorrection", "sqc", 1.0, "Stretch across for anamorphic pictures."),
            boolean("flipInX", "fix", false, "Mirrored across."),
            boolean("flipInY", "fiy", false, "Mirrored up and down."),
            en("textureFilter", "tf", 0, &["Default", "Unfiltered", "Bilinear", "Trilinear"], "Filtering in the views."),
            boolean("visibleInReflections", "vir", false, "Seen in reflections when rendered."),
            boolean("visibleInRefractions", "vif", false, "Seen in refractions when rendered."),
            AttrDef { flags: flags::INPUT | flags::MESSAGE | flags::HIDDEN, ..int("lookThroughCamera", "ltc", 0, "The camera it is shown on.") },
        ];
        types::register_type(NodeType {
            name: TYPE.into(),
            base: Some(shape),
            dag: true,
            icon: "photo".into(),
            doc: "An image plane: a picture on a camera or free in the scene, drawn in the views.".into(),
            own_attrs: own,
            create: Some(Arc::new(|| Box::new(ImagePlane))),
            ..Default::default()
        })
        .expect(TYPE);
    });
}

fn val(g: &Graph, id: NodeId, attr: &str) -> PlugValue {
    g.node(id).and_then(|n| n.ty().attr_index(attr).map(|i| n.data().value(i).clone())).unwrap_or(PlugValue::Float(0.0))
}

/// The camera an image plane is attached to.
pub fn camera_of(g: &Graph, id: NodeId) -> Option<NodeId> {
    let p = crate::harold::network::attr(g, id, "message")?;
    g.destinations(&p).into_iter().find(|d| g.node(d.node).is_some_and(|n| n.ty().is_a(crate::cameras::TYPE))).map(|d| d.node)
}

/// The file shown at a frame: the name itself, or with useFrameExtension the frame (frameExtension, or the scene's
/// frame when nothing drives it, plus frameOffset) in place of the last run of digits before the extension, padded as
/// that run is.
pub fn file_at(g: &Graph, id: NodeId, scene_frame: f64) -> String {
    let name = val(g, id, "imageName").as_str().to_string();
    if !val(g, id, "useFrameExtension").as_bool() {
        return name;
    }
    let driven = crate::harold::network::attr(g, id, "frameExtension").is_some_and(|p| g.source(&p).is_some());
    let frame = if driven { val(g, id, "frameExtension").as_int() } else { scene_frame.round() as i64 } + val(g, id, "frameOffset").as_int();
    with_frame(&name, frame)
}

pub fn with_frame(name: &str, frame: i64) -> String {
    let (dir, file) = match name.rfind(['/', '\\']) {
        Some(i) => (&name[..=i], &name[i + 1..]),
        None => ("", name),
    };
    let stem_end = file.rfind('.').unwrap_or(file.len());
    let stem = &file[..stem_end];
    let digits_end = stem.len();
    let digits_start = stem.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits_start == digits_end {
        return name.to_string();
    }
    let pad = digits_end - digits_start;
    let n = if frame < 0 { format!("-{:0pad$}", -frame, pad = pad.saturating_sub(1)) } else { format!("{frame:0pad$}") };
    format!("{dir}{}{n}{}", &stem[..digits_start], &file[stem_end..])
}

/// What a view needs to draw one: the gate's middle and half extents in world space (the picture is fitted into it
/// by fit, with the picture's own proportions), and how to show it.
#[derive(Clone, Debug)]
pub struct Draw {
    pub node: NodeId,
    pub file: String,
    pub center: DVec3,
    /// Half the gate across and up, world vectors.
    pub right: DVec3,
    pub up: DVec3,
    pub fit: i64,
    /// The size in the gate for To Size, as fractions of the gate's half extents.
    pub to_size: (f64, f64),
    pub display_mode: i64,
    pub alpha_gain: f64,
    pub gain: [f32; 3],
    pub flip: (bool, bool),
    pub rotate: f64,
}

/// The image planes a view looking through `view_camera` (a camera shape) draws.
pub fn draws(g: &Graph, view_camera: NodeId, scene_frame: f64) -> Vec<Draw> {
    let mut ids: Vec<NodeId> = g.nodes().into_iter().filter(|&id| g.node(id).is_some_and(|n| n.ty().is_a(TYPE))).collect();
    ids.sort_unstable();
    let mut out = Vec::new();
    for id in ids {
        let cam = camera_of(g, id);
        // On a camera, its own visibility and its transform's count (the default cameras are hidden themselves);
        // free, as any object's.
        let shown = match cam {
            Some(_) => [id, g.parent(id)].iter().all(|&n| n == NO_NODE || g.node(n).is_some_and(|x| !x.ty().dag || x.bool(DagNode::VISIBILITY))),
            None => crate::harold::visible(g, id),
        };
        if !shown || val(g, id, "displayMode").as_int() == 0 {
            continue;
        }
        let locked = val(g, id, "lockedToCamera").as_bool();
        if cam.is_some() && val(g, id, "displayOnlyIfCurrent").as_bool() && cam != Some(view_camera) {
            continue;
        }
        let file = file_at(g, id, scene_frame);
        let base = Draw {
            node: id,
            file,
            center: DVec3::ZERO,
            right: DVec3::X,
            up: DVec3::Y,
            fit: val(g, id, "fit").as_int(),
            to_size: (1.0, 1.0),
            display_mode: val(g, id, "displayMode").as_int(),
            alpha_gain: val(g, id, "alphaGain").as_float(),
            gain: val(g, id, "colorGain").as_vec3().as_vec3().to_array(),
            flip: (val(g, id, "flipInX").as_bool(), val(g, id, "flipInY").as_bool()),
            rotate: val(g, id, "rotate").as_float(),
        };
        match cam.filter(|_| locked) {
            Some(c) => {
                // On the camera: the film gate at depth, in the camera's space.
                let cw = g.node(c).map(|n| n.data().matrix(DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY);
                let depth = val(g, id, "depth").as_float().max(1e-6);
                let focal_cm = val(g, c, "focalLength").as_float().max(1e-6) / 10.0;
                let (hfa, vfa) = (val(g, c, "horizontalFilmAperture").as_float() * 2.54, val(g, c, "verticalFilmAperture").as_float() * 2.54);
                let ortho = val(g, c, "orthographic").as_bool();
                let (hw, hh) = if ortho {
                    let w = val(g, c, "orthographicWidth").as_float() * 0.5;
                    (w, w * vfa / hfa.max(1e-9))
                } else {
                    (depth * hfa * 0.5 / focal_cm, depth * vfa * 0.5 / focal_cm)
                };
                let off = val(g, id, "offset").as_vec3();
                let size = val(g, id, "size").as_vec3();
                let (ox, oy) = (off.x * 2.54 / hfa.max(1e-9) * 2.0 * hw, off.y * 2.54 / vfa.max(1e-9) * 2.0 * hh);
                let center = cw.transform_point3(DVec3::new(ox, oy, -depth));
                out.push(Draw {
                    center,
                    right: cw.transform_vector3(DVec3::new(hw, 0.0, 0.0)),
                    up: cw.transform_vector3(DVec3::new(0.0, hh, 0.0)),
                    to_size: (size.x * 2.54 / hfa.max(1e-9), size.y * 2.54 / vfa.max(1e-9)),
                    ..base
                });
            }
            None => {
                // Free: width by height about imageCenter, in the plane's own space.
                let w = g.node(id).map(|n| n.data().matrix(DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY);
                let (pw, ph) = (val(g, id, "width").as_float(), val(g, id, "height").as_float());
                let c = val(g, id, "imageCenter").as_vec3();
                out.push(Draw {
                    center: w.transform_point3(c),
                    right: w.transform_vector3(DVec3::new(pw.max(1e-6) * 0.5, 0.0, 0.0)),
                    up: w.transform_vector3(DVec3::new(0.0, ph.max(1e-6) * 0.5, 0.0)),
                    // A free plane is its own gate: the picture fills it.
                    fit: if val(g, id, "maintainRatio").as_bool() && (pw <= 0.0 || ph <= 0.0) { 1 } else { 0 },
                    ..base
                });
            }
        }
    }
    out
}

/// The picture's rectangle in the gate, as fractions of the gate's half extents: (across, up).
pub fn fitted(fit: i64, gate_aspect: f64, image_aspect: f64, to_size: (f64, f64)) -> (f64, f64) {
    let ia = image_aspect.max(1e-9);
    let ga = gate_aspect.max(1e-9);
    match fit {
        // Fill: the gate covered, the picture's proportions kept (it overhangs).
        0 => {
            if ia > ga {
                (ia / ga, 1.0)
            } else {
                (1.0, ga / ia)
            }
        }
        // Best: the whole picture inside the gate.
        1 => {
            if ia > ga {
                (1.0, ga / ia)
            } else {
                (ia / ga, 1.0)
            }
        }
        2 => (1.0, ga / ia),
        3 => (ia / ga, 1.0),
        _ => to_size,
    }
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        CommandDef::<Session>::new(
            "imagePlane",
            "Makes an image plane: on -camera (shown on it, at its depth, fitted to its gate) or free in the scene, showing \
             -fileName, -width by -height. Returns its transform and shape. With -edit or -query on an image plane: \
             -fileName, -width, -height, -camera, -showInAllViews (displayOnlyIfCurrent off), -lookThrough. -detach frees \
             one from its camera.",
            |s, a| {
                let names = a.named();
                let edit = a.bool("edit") || a.bool("query") || a.bool("detach");
                if edit {
                    let Some(n) = names.first() else { fail!("imagePlane: name an image plane") };
                    let id = s.graph.find(n)?.ok_or_else(|| aztek_ops::CommandError::new(format!("imagePlane: no {n}")))?;
                    let shape = if s.graph.node(id).is_some_and(|x| x.ty().is_a(TYPE)) { id } else { s.graph.children(id).iter().copied().find(|&k| s.graph.node(k).is_some_and(|x| x.ty().is_a(TYPE))).unwrap_or(NO_NODE) };
                    if shape == NO_NODE {
                        fail!("imagePlane: {n} is not an image plane");
                    }
                    let sp = s.graph.path(shape);
                    if a.bool("query") {
                        let q: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
                        let asked = |f: &str| a.bool(f) || q.iter().any(|x| x == f);
                        let g = &s.graph;
                        return Ok(if asked("fileName") {
                            json!(val(g, shape, "imageName").as_str())
                        } else if asked("width") {
                            json!(val(g, shape, "width").as_float())
                        } else if asked("height") {
                            json!(val(g, shape, "height").as_float())
                        } else if asked("camera") {
                            json!(camera_of(g, shape).map(|c| g.path(g.parent(c))).unwrap_or_default())
                        } else if asked("showInAllViews") {
                            json!(!val(g, shape, "displayOnlyIfCurrent").as_bool())
                        } else if asked("maintainRatio") {
                            json!(val(g, shape, "maintainRatio").as_bool())
                        } else if asked("lookThrough") {
                            json!(camera_of(g, shape).map(|c| g.path(g.parent(c))).unwrap_or_default())
                        } else if asked("name") {
                            json!(g.node(shape).map(|n| n.name().to_string()).unwrap_or_default())
                        } else if asked("imageSize") {
                            let file = file_at(g, shape, crate::anim::frame(g));
                            let (w, h) = image::image_dimensions(&file).map(|(w, h)| (w as i64, h as i64)).unwrap_or((0, 0));
                            json!([w, h])
                        } else if let Some(v) = movie_query(g, shape, &asked) {
                            v
                        } else {
                            fail!("imagePlane: query what?")
                        });
                    }
                    if a.bool("detach") {
                        if let Some(c) = camera_of(&s.graph, shape) {
                            let cp = s.graph.path(c);
                            let msg = format!("{sp}.message");
                            let conns = s.run("listConnections", json!({"args": [msg.clone()], "plugs": true, "source": false}))?;
                            for d in conns.as_array().into_iter().flatten().filter_map(Value::as_str) {
                                if d.starts_with(&cp) || d.contains("imagePlane[") {
                                    let _ = s.run("disconnectAttr", json!({"args": [msg.clone(), d]}));
                                }
                            }
                            s.run("setAttr", json!({"args": [format!("{sp}.lockedToCamera"), false]}))?;
                            let tr = s.graph.path(s.graph.parent(shape));
                            let _ = s.run("parent", json!({"args": [tr], "world": true}));
                        }
                        return Ok(Value::Null);
                    }
                    apply_flags(s, &sp, &a)?;
                    return Ok(Value::Null);
                }
                let name = a.string("name", "imagePlane1");
                let cam = match a.get("camera").and_then(Value::as_str) {
                    Some(c) => {
                        let id = s.graph.find(c)?.ok_or_else(|| aztek_ops::CommandError::new(format!("imagePlane: no camera {c}")))?;
                        Some(crate::cameras::shape_of(&s.graph, id).ok_or_else(|| aztek_ops::CommandError::new(format!("imagePlane: {c} is not a camera")))?)
                    }
                    None => None,
                };
                let parent = cam.map_or(NO_NODE, |c| s.graph.parent(c));
                let (tr, sh) = crate::dag::create_with_shape(&mut s.graph, TYPE, &name, parent)?;
                let (trp, shp) = (s.graph.path(tr), s.graph.path(sh));
                if let Some(c) = cam {
                    let cp = s.graph.path(c);
                    s.run("connectAttr", json!({"args": [format!("{shp}.message"), format!("{cp}.imagePlane")], "nextAvailable": true}))?;
                    s.run("connectAttr", json!({"args": [format!("{cp}.message"), format!("{shp}.lookThroughCamera")]}))?;
                } else {
                    s.run("setAttr", json!({"args": [format!("{shp}.lockedToCamera"), false]}))?;
                    // A free plane's size, when none is given: Aztek's default, 10 by the picture's proportions.
                    if !a.has("width") {
                        s.run("setAttr", json!({"args": [format!("{shp}.width"), 10.0]}))?;
                    }
                    if !a.has("height") {
                        s.run("setAttr", json!({"args": [format!("{shp}.height"), 10.0]}))?;
                    }
                }
                apply_flags(s, &shp, &a)?;
                Ok(json!([trp, shp]))
            },
        )
        .positional("image plane")
        .flag(FlagDef::new("camera", "c", ArgType::String, Value::Null, "The camera it goes on."))
        .flag(FlagDef::new("name", "n", ArgType::String, Value::Null, "Its name."))
        .flag(FlagDef::new("fileName", "fn", ArgType::String, Value::Null, "The picture."))
        .flag(FlagDef::new("width", "w", ArgType::Float, Value::Null, "Width of a free plane."))
        .flag(FlagDef::new("height", "h", ArgType::Float, Value::Null, "Height of a free plane."))
        .flag(FlagDef::new("maintainRatio", "mr", ArgType::Bool, Value::Null, "Keep the picture's proportions."))
        .flag(FlagDef::new("showInAllViews", "sia", ArgType::Bool, Value::Null, "Shown in every view, not only looking through its camera."))
        .flag(FlagDef::new("lookThrough", "lt", ArgType::String, Value::Null, "The camera it is shown on, by name."))
        .flag(FlagDef::new("detach", "d", ArgType::Bool, Value::Null, "Free it from its camera."))
        .flag(FlagDef::new("imageSize", "iz", ArgType::Bool, Value::Null, "Query: the picture's width and height in pixels."))
        .flag(FlagDef::new("quickTime", "qt", ArgType::Bool, Value::Null, "Query: whether the picture is a movie."))
        .flag(FlagDef::new("timeCodeTrack", "tt", ArgType::Bool, Value::Null, "Query: whether the movie has a timecode track."))
        .flag(FlagDef::new("timeCode", "tc", ArgType::Bool, Value::Null, "Query: the timecode, as a frame count."))
        .flag(FlagDef::new("counter", "cn", ArgType::Bool, Value::Null, "Query: whether the timecode is a frame count rather than hours, minutes, seconds and frames."))
        .flag(FlagDef::new("numFrames", "nf", ArgType::Bool, Value::Null, "Query: the timecode format's whole frames per second."))
        .flag(FlagDef::new("timeScale", "ts", ArgType::Bool, Value::Null, "Query: the timecode format's time scale."))
        .flag(FlagDef::new("frameDuration", "fd", ArgType::Bool, Value::Null, "Query: the timecode format's frame duration."))
        .flag(FlagDef::new("dropFrame", "df", ArgType::Bool, Value::Null, "Query: the timecode format's drop frame flag."))
        .flag(FlagDef::new("negTimesOK", "nt", ArgType::Bool, Value::Null, "Query: the timecode format's negative times flag."))
        .flag(FlagDef::new("twentyFourHourMax", "tf", ArgType::Bool, Value::Null, "Query: the timecode format's 24 hour maximum flag."))
        .flag(FlagDef::new("edit", "e", ArgType::Bool, Value::Null, "Change one."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Read one."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What -q asks for.")),
    );
}

/// Whether the picture is a movie file.
fn is_movie(g: &Graph, id: NodeId) -> bool {
    let name = val(g, id, "imageName").as_str().to_ascii_lowercase();
    val(g, id, "type").as_int() == 2 || [".mov", ".mp4", ".avi", ".m4v", ".mkv", ".qt"].iter().any(|e| name.ends_with(e))
}

/// The movie and timecode questions. Pictures here carry no timecode track, so the timecode format is the scene's:
/// its frame rate as the reference writes it (frames per second, a time scale and a frame duration), drop frame for
/// the NTSC rates, and the timecode as a frame count.
fn movie_query(g: &Graph, id: NodeId, asked: &dyn Fn(&str) -> bool) -> Option<Value> {
    let fps = crate::anim::fps();
    let ntsc = (fps * 1001.0 / 1000.0 - (fps * 1001.0 / 1000.0).round()).abs() < 1e-3 && (fps.round() - fps).abs() > 1e-3;
    let whole = if ntsc { (fps * 1001.0 / 1000.0).round() } else { fps.round() };
    let (scale, duration) = if ntsc { (whole as i64 * 1000, 1001) } else { (whole as i64 * 100, 100) };
    Some(if asked("quickTime") {
        json!(is_movie(g, id))
    } else if asked("timeCodeTrack") {
        json!(false)
    } else if asked("numFrames") {
        json!(whole as i64)
    } else if asked("timeScale") {
        json!(scale)
    } else if asked("frameDuration") {
        json!(duration)
    } else if asked("dropFrame") {
        json!(ntsc && (whole as i64) % 30 == 0)
    } else if asked("negTimesOK") || asked("twentyFourHourMax") {
        json!(false)
    } else if asked("counter") {
        json!(true)
    } else if asked("timeCode") {
        json!(crate::anim::frame(g).round() as i64)
    } else {
        return None;
    })
}

fn apply_flags(s: &mut Session, shp: &str, a: &aztek_ops::Args) -> CmdResult<()> {
    if let Some(f) = a.get("fileName").and_then(Value::as_str) {
        s.run("setAttr", json!({"args": [format!("{shp}.imageName"), f], "type": "string"}))?;
    }
    for (flag, attr) in [("width", "width"), ("height", "height")] {
        if let Some(v) = a.get(flag).and_then(Value::as_f64) {
            s.run("setAttr", json!({"args": [format!("{shp}.{attr}"), v]}))?;
        }
    }
    if let Some(v) = a.get("maintainRatio").and_then(Value::as_bool) {
        s.run("setAttr", json!({"args": [format!("{shp}.maintainRatio"), v]}))?;
    }
    if let Some(v) = a.get("showInAllViews").and_then(Value::as_bool) {
        s.run("setAttr", json!({"args": [format!("{shp}.displayOnlyIfCurrent"), !v]}))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_go_in_the_last_digits() {
        assert_eq!(with_frame("/a/plate.0001.jpg", 12), "/a/plate.0012.jpg");
        assert_eq!(with_frame("plate_7.png", 120), "plate_120.png");
        assert_eq!(with_frame("still.png", 3), "still.png");
    }

    #[test]
    fn fits() {
        // A 2:1 picture in a 1.5 gate.
        assert_eq!(fitted(1, 1.5, 2.0, (1.0, 1.0)), (1.0, 0.75));
        assert_eq!(fitted(0, 1.5, 2.0, (1.0, 1.0)), (2.0 / 1.5, 1.0));
        assert_eq!(fitted(3, 1.5, 2.0, (1.0, 1.0)), (2.0 / 1.5, 1.0));
    }
}
