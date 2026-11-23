//! selectType and selectPref: the selection masks the select tools pick through, and the selection preferences.
//! The masks are the application's, shared by every scene; the preferences live with ui_cmds' other settings.

use std::collections::BTreeMap;
use std::sync::Mutex;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::graph::Graph;
use aztek_scene::plug::NodeId;
use aztek_scene::Session;
use serde_json::{json, Value};

type Cmd = CommandDef<Session>;

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

/// The object masks: (flag, short, what it masks).
const OBJECT_MASKS: &[(&str, &str, &str)] = &[
    ("camera", "ca", "Cameras."),
    ("cluster", "cl", "Cluster and soft modification handles."),
    ("collisionModel", "clm", "Collision models."),
    ("curve", "c", "Curves: NURBS curves and curves on surfaces."),
    ("curveOnSurface", "cos", "Curves on surfaces."),
    ("dimension", "dim", "Dimension shapes: distances, angles, arc lengths and parameters."),
    ("dynamicConstraint", "dc", "Dynamic constraints."),
    ("emitter", "em", "Emitters."),
    ("field", "fi", "Fields."),
    ("fluid", "fl", "Fluids."),
    ("follicle", "fo", "Follicles."),
    ("hairSystem", "hs", "Hair systems."),
    ("handle", "ha", "Object handles."),
    ("ikEndEffector", "iee", "IK end effectors."),
    ("ikHandle", "ikh", "IK handles."),
    ("implicitGeometry", "ig", "Implicit geometry: implicit spheres, boxes and cones."),
    ("joint", "j", "Joints."),
    ("lattice", "la", "Lattices and their bases."),
    ("light", "lt", "Lights."),
    ("locator", "lc", "Every kind of locator."),
    ("locatorUV", "luv", "UV locators."),
    ("locatorXYZ", "xyz", "XYZ locators."),
    ("nCloth", "ncl", "Cloth."),
    ("nParticleShape", "nps", "Particle shapes (nParticle)."),
    ("nRigid", "nr", "Passive colliders (nRigid)."),
    ("nonlinear", "nl", "Nonlinear deformer handles."),
    ("nurbsCurve", "nc", "NURBS curves."),
    ("nurbsSurface", "ns", "NURBS surfaces."),
    ("orientationLocator", "ol", "Orientation locators."),
    ("particleShape", "ps", "Particle shapes."),
    ("plane", "pl", "Sketch planes."),
    ("polymesh", "p", "Polygon meshes."),
    ("rigidBody", "rb", "Rigid bodies."),
    ("rigidConstraint", "rc", "Rigid constraints."),
    ("sculpt", "sc", "Sculpt objects."),
    ("spring", "spr", "Spring shapes."),
    ("stroke", "str", "Strokes."),
    ("subdiv", "sd", "Subdivision surfaces."),
    ("texture", "tx", "Texture placements."),
    ("group", "gr", "Transforms with no shape (groups)."),
];

/// The component masks: (flag, short, what it masks).
const COMPONENT_MASKS: &[(&str, &str, &str)] = &[
    ("animBreakdown", "abd", "Breakdown keys."),
    ("animCurve", "ac", "Animation curves."),
    ("animInTangent", "ait", "In tangents of keys."),
    ("animKeyframe", "ak", "Keys."),
    ("animOutTangent", "aot", "Out tangents of keys."),
    ("controlVertex", "cv", "Control vertices."),
    ("curveKnot", "ck", "Curve knots."),
    ("curveParameterPoint", "cpp", "Curve points: a click on a curve takes the place clicked."),
    ("edge", "eg", "Mesh edges."),
    ("editPoint", "ep", "Curve edit points."),
    ("facet", "fc", "Mesh faces."),
    ("hull", "hl", "Hulls."),
    ("imagePlane", "ip", "Image planes."),
    ("isoparm", "iso", "Surface isoparms."),
    ("jointPivot", "jp", "Joint pivots."),
    ("latticePoint", "lp", "Lattice points."),
    ("localRotationAxis", "ra", "Local rotation axes."),
    ("meshUVShell", "msh", "UV shells: a UV pick takes its whole shell."),
    ("motionTrailPoint", "mtp", "Motion trail points."),
    ("motionTrailTangent", "mtt", "Motion trail point tangents."),
    ("nParticle", "npr", "Particle points (nParticle)."),
    ("particle", "pr", "Particle points."),
    ("polymeshEdge", "pe", "Mesh edges."),
    ("polymeshFace", "pf", "Mesh faces."),
    ("polymeshFreeEdge", "pfe", "Mesh border edges."),
    ("polymeshUV", "puv", "Mesh UVs."),
    ("polymeshVertex", "pv", "Mesh vertices."),
    ("polymeshVtxFace", "pvf", "Mesh vertex faces."),
    ("rotatePivot", "rp", "Rotate pivots."),
    ("scalePivot", "sp", "Scale pivots."),
    ("selectHandle", "sh", "Selection handles."),
    ("springComponent", "spc", "Single springs."),
    ("subdivMeshEdge", "sme", "Subdivision surface edges."),
    ("subdivMeshFace", "smf", "Subdivision surface faces."),
    ("subdivMeshPoint", "smp", "Subdivision surface points."),
    ("subdivMeshUV", "smu", "Subdivision surface UVs."),
    ("surfaceEdge", "se", "Surface edges."),
    ("surfaceFace", "sf", "Surface patches."),
    ("surfaceKnot", "sk", "Surface knots."),
    ("surfaceParameterPoint", "spp", "Surface points."),
    ("surfaceRange", "sr", "Surface ranges."),
    ("surfaceUV", "suv", "Surface UVs."),
    ("vertex", "v", "Mesh vertices."),
];

/// The component kinds Aztek's component modes stand for: these switch the mode and are read back from it.
const MODE_KINDS: &[&str] = &["controlVertex", "vertex", "polymeshVertex", "edge", "polymeshEdge", "facet", "polymeshFace", "polymeshUV", "polymeshVtxFace", "meshUVShell", "editPoint", "curveParameterPoint"];

/// The point masks, on in component mode as the reference starts.
const POINT_MASKS: &[&str] = &["controlVertex", "vertex", "polymeshVertex", "latticePoint", "particle", "nParticle", "subdivMeshPoint"];

#[derive(Default)]
struct Masks {
    /// Object masks set; unset ones are on.
    objects: BTreeMap<String, bool>,
    /// Component masks set in component mode; unset ones follow POINT_MASKS.
    components: BTreeMap<String, bool>,
    /// Component masks that apply in object mode (-objectComponent); unset ones are off.
    object_components: BTreeMap<String, bool>,
    /// User-defined masks by name (-byName); unset ones are on.
    by_name: BTreeMap<String, bool>,
    before_preset: Option<(BTreeMap<String, bool>, BTreeMap<String, bool>)>,
}

static MASKS: Mutex<Option<Masks>> = Mutex::new(None);

fn with_masks<R>(f: impl FnOnce(&mut Masks) -> R) -> R {
    let mut g = MASKS.lock().unwrap();
    f(g.get_or_insert_with(Masks::default))
}

/// Set a Hotbox mask using the objects this picker supports. Polygons also picks faces.
pub(crate) fn hotbox_mask(s: &mut Session, preset: &str) -> Option<bool> {
    let flags: &[&str] = match preset {
        "nurbsModeling" => &["curve", "nurbsSurface", "plane"],
        "animation" => &["joint", "handle", "ikHandle"],
        "deformations" => &["lattice", "cluster", "sculpt", "nonlinear", "curve"],
        "dynamics" => &["particleShape", "nParticleShape", "emitter", "field", "fluid", "hairSystem", "follicle", "collisionModel",
            "spring", "rigidBody", "rigidConstraint", "nCloth", "nRigid", "dynamicConstraint"],
        "polyModeling" => &["polymesh", "subdiv", "plane"],
        "rendering" => &["light", "camera", "plane"],
        "hierarchy" | "object" | "component" => {
            with_masks(|m| {
                if let Some((objects, components)) = m.before_preset.take() {
                    m.objects = objects;
                    m.components = components;
                }
            });
            return None;
        }
        _ => return None,
    };
    with_masks(|m| {
        if m.before_preset.is_none() {
            m.before_preset = Some((m.objects.clone(), m.components.clone()));
        }
    });
    let polygons = preset == "polyModeling";
    let _ = s.run("selectMode", json!({"args": ["object"]}));
    let mut call = json!({"allObjects": false, "allComponents": false});
    for &flag in flags {
        call[flag] = json!(true);
    }
    if preset == "deformations" {
        call["curveOnSurface"] = json!(false);
    }
    if polygons {
        call["polymeshFace"] = json!(true);
    }
    let _ = s.run("selectType", call);
    Some(polygons)
}

/// Whether an object mask is on.
pub fn object_mask(name: &str) -> bool {
    with_masks(|m| m.objects.get(name).copied().unwrap_or(true))
}

/// Whether a component mask is on (in component mode, or in object mode with `object_mode`).
pub fn component_mask(name: &str, object_mode: bool) -> bool {
    with_masks(|m| {
        if object_mode {
            m.object_components.get(name).copied().unwrap_or(false)
        } else {
            m.components.get(name).copied().unwrap_or(POINT_MASKS.contains(&name))
        }
    })
}

/// A user-defined mask, by name.
pub fn named_mask(name: &str) -> bool {
    with_masks(|m| m.by_name.get(name).copied().unwrap_or(true))
}

/// The mask that governs picking a node: a transform takes its shape's.
pub fn node_mask(g: &Graph, id: NodeId) -> &'static str {
    let Some(n) = g.node(id) else { return "" };
    let ty = n.ty();
    let is = |names: &[&str]| names.iter().any(|t| ty.is_a(t));
    if is(&["joint"]) {
        return "joint";
    }
    if is(&["ikHandle"]) {
        return "ikHandle";
    }
    if is(&["ikEffector"]) {
        return "ikEndEffector";
    }
    if is(&["clusterHandle", "softModHandle"]) {
        return "cluster";
    }
    if aztek_scene::deform::handle_shape_kind(&ty.name).is_some_and(|k| k != "cluster" && k != "sculpt" && k != "textureDeformer") {
        return "nonlinear";
    }
    if is(&["implicitSphere"]) && g.connections().iter().any(|(d, s)| s.node == id && g.node(d.node).is_some_and(|x| x.ty().name == "sculpt")) {
        return "sculpt";
    }
    if is(&["implicitSphere", "implicitBox", "implicitCone", "renderBox", "renderSphere", "renderCone"]) {
        return "implicitGeometry";
    }
    if is(&["lattice", "baseLattice"]) {
        return "lattice";
    }
    if is(&["nurbsSurface"]) {
        return "nurbsSurface";
    }
    if is(&["subdiv"]) {
        return "subdiv";
    }
    if is(&["nCloth"]) {
        return "nCloth";
    }
    if is(&["nRigid"]) {
        return "nRigid";
    }
    if is(&["mesh"]) {
        return "polymesh";
    }
    if is(&["nurbsCurve"]) {
        // A curve on a surface sits under the surface's transform beside the surface.
        let parent = g.parent(id);
        let on_surface = parent != aztek_scene::plug::NO_NODE && aztek_scene::dag::shapes(g, parent, false).iter().any(|&s| s != id && g.node(s).is_some_and(|x| x.ty().is_a("nurbsSurface")));
        return if on_surface { "curveOnSurface" } else { "nurbsCurve" };
    }
    if is(&["distanceDimShape", "angleDimension", "arcLengthDimension", "paramDimension"]) {
        return "dimension";
    }
    if is(&["locator"]) {
        return "locator";
    }
    if is(&["camera"]) {
        return "camera";
    }
    if is(&["light"]) || aztek_scene::lights::is_light_type(&ty.name) {
        return "light";
    }
    if is(&["nParticle"]) {
        return "nParticleShape";
    }
    if is(&["particle"]) {
        return "particleShape";
    }
    if is(&["pointEmitter"]) {
        return "emitter";
    }
    if is(&["field"]) || ty.name.ends_with("Field") {
        return "field";
    }
    if is(&["follicle"]) {
        return "follicle";
    }
    if is(&["hairSystem"]) {
        return "hairSystem";
    }
    if is(&["rigidBody"]) {
        return "rigidBody";
    }
    if is(&["rigidConstraint"]) {
        return "rigidConstraint";
    }
    if is(&["dynamicConstraint"]) {
        return "dynamicConstraint";
    }
    if is(&["spring"]) {
        return "spring";
    }
    if is(&["fluidShape"]) {
        return "fluid";
    }
    if is(&["sketchPlane"]) {
        return "plane";
    }
    if is(&["stroke"]) {
        return "stroke";
    }
    if is(&["place3dTexture"]) {
        return "texture";
    }
    if is(&["transform"]) {
        if let Some(&s) = aztek_scene::dag::shapes(g, id, false).first() {
            if s != id {
                return node_mask(g, s);
            }
        }
        return "group";
    }
    ""
}

/// Whether the selection masks let a click or marquee pick this node.
pub fn pickable(g: &Graph, id: NodeId) -> bool {
    let m = node_mask(g, id);
    if m.is_empty() {
        return true;
    }
    if !object_mask(m) {
        return false;
    }
    // The curve mask covers both kinds of curve.
    if matches!(m, "nurbsCurve" | "curveOnSurface") && !object_mask("curve") {
        return false;
    }
    true
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or_else(|| v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|s| matches!(s, "1" | "true" | "on" | "yes")))
}

fn select_type(s: &mut Session, a: &Args) -> CmdResult {
    let ocm = a.bool("objectComponent");
    let objects = || OBJECT_MASKS.iter().map(|x| x.0);
    let comps = || COMPONENT_MASKS.iter().map(|x| x.0);
    if a.bool("query") {
        if let Some(n) = a.get("queryByName").and_then(Value::as_str) {
            return Ok(json!(named_mask(n)));
        }
        if a.has("allObjects") {
            return Ok(json!(objects().all(object_mask)));
        }
        if a.has("allComponents") {
            return Ok(json!(comps().all(|c| component_mask(c, ocm))));
        }
        if let Some(name) = objects().find(|n| a.has(n)) {
            return Ok(json!(object_mask(name)));
        }
        if let Some(name) = comps().find(|n| a.has(n)) {
            if !ocm && MODE_KINDS.contains(&name) {
                let mode = aztek_scene::surfaces::select_type_components(&mut s.graph, a).and_then(|v| v.as_bool()).unwrap_or(false);
                // The mode decides, unless the mask was turned off since (-allComponents false).
                return Ok(json!(mode && with_masks(|m| m.components.get(name).copied().unwrap_or(true))));
            }
            return Ok(json!(component_mask(name, ocm)));
        }
        fail!("selectType: query which mask?");
    }
    let mut turned_off = false;
    with_masks(|m| {
        if let Some(v) = a.get("allObjects") {
            let v = on(v);
            turned_off |= !v;
            for n in objects() {
                m.objects.insert(n.to_string(), v);
            }
        }
        if let Some(v) = a.get("allComponents") {
            let v = on(v);
            let map = if ocm { &mut m.object_components } else { &mut m.components };
            for n in comps() {
                map.insert(n.to_string(), v);
            }
        }
        for n in objects() {
            if let Some(v) = a.get(n) {
                let v = on(v);
                turned_off |= !v;
                m.objects.insert(n.to_string(), v);
                // The locator mask covers every kind of locator; the curve mask both kinds of curve.
                let kin: &[&str] = match n {
                    "locator" => &["locatorUV", "locatorXYZ", "orientationLocator"],
                    "curve" => &["nurbsCurve", "curveOnSurface"],
                    _ => &[],
                };
                for k in kin {
                    m.objects.insert(k.to_string(), v);
                }
            }
        }
        for n in comps() {
            if let Some(v) = a.get(n) {
                let v = on(v);
                let map = if ocm { &mut m.object_components } else { &mut m.components };
                map.insert(n.to_string(), v);
            }
        }
        if let Some(Value::Array(v)) = a.get("byName") {
            for pair in v.chunks(2) {
                if let (Some(n), Some(x)) = (pair.first().and_then(Value::as_str), pair.get(1)) {
                    m.by_name.insert(n.to_string(), on(x));
                }
            }
        }
    });
    // Aztek's component modes: a kind turned on switches to its mode.
    if !ocm {
        let _ = aztek_scene::surfaces::select_type_components(&mut s.graph, a);
        if a.get("allComponents").is_some_and(on) {
            aztek_scene::components::set_multi(&mut s.graph);
        }
    }
    // selectPref -selectTypeChangeAffectsActive: what can no longer be picked leaves the selection.
    if turned_off && crate::ui_cmds::pref_bool("selectTypeChangeAffectsActive", false) {
        let g = &s.graph;
        let keep: Vec<String> = g.selection().iter().filter(|&&id| pickable(g, id)).map(|&id| g.path(id)).collect();
        if keep.len() != g.selection().len() {
            if keep.is_empty() {
                s.run("select", json!({"clear": true}))?;
            } else {
                s.run("select", json!({"args": keep}))?;
            }
        }
    }
    Ok(Value::Null)
}

/// selectPref's settings: (flag, short, type, default, help).
const PREFS: &[(&str, &str, ArgType, f64, &str)] = &[
    ("affectsActive", "aa", ArgType::Bool, 0.0, "Switching between object and component selection changes the active list."),
    ("allowHiliteSelection", "ahs", ArgType::Bool, 1.0, "In component mode, picking another object hilites it for editing."),
    ("autoSelectContainer", "asc", ArgType::Bool, 0.0, "With container centric selection, picking a root transform also selects its container."),
    ("autoSelectOutlinerSetMembers", "asm", ArgType::Bool, 0.0, "Picking a set in the Outliner selects its members instead."),
    ("autoUseDepth", "aud", ArgType::Bool, 0.0, "useDepth and paintSelectWithDepth on in shaded views and off in wireframe."),
    ("clickBoxSize", "cbs", ArgType::Int, 4.0, "Half the side, in pixels, of the square a click picks in; must be positive (4)."),
    ("clickDrag", "cld", ArgType::Bool, 0.0, "Click and drag selection."),
    ("containerCentricSelection", "ccs", ArgType::Bool, 0.0, "Picking a node in a container selects its root transform, or its highest node."),
    ("disableComponentPopups", "dcp", ArgType::Bool, 0.0, "No popup menus when picking components (with popupMenuSelection)."),
    ("expandPopupList", "epl", ArgType::Bool, 0.0, "In popup selection, each object or component of a candidate is its own item."),
    ("ignoreSelectionPriority", "isp", ArgType::Bool, 0.0, "A marquee takes every kind it covers, not only the highest in selection priority."),
    ("lockSelection", "lck", ArgType::Bool, 0.0, "The selection doesn't change on a click."),
    ("manipClickBoxSize", "mcb", ArgType::Int, 8.0, "Half the side, in pixels, of the square a click picks a manipulator handle in; must be positive (8)."),
    ("paintSelect", "ps", ArgType::Bool, 0.0, "The select tool paints a selection with a brush instead of dragging a marquee."),
    ("paintSelectWithDepth", "psd", ArgType::Bool, 0.0, "Paint selection leaves out components behind surfaces."),
    ("popupMenuSelection", "pms", ArgType::Bool, 0.0, "A marquee over several candidates shows a popup to pick one."),
    ("preSelectBackfacing", "psb", ArgType::Bool, 0.0, "Preselection highlights back-facing components too."),
    ("preSelectClosest", "psc", ArgType::Bool, 0.0, "Over a surface, preselection highlights the closest component however far."),
    ("preSelectDeadSpace", "pds", ArgType::Int, 8.0, "Pixels round the cursor preselection looks in off the surface (8)."),
    ("preSelectHilite", "psh", ArgType::Bool, 0.0, "Highlights the component under the cursor that a click would pick."),
    ("preSelectHiliteSize", "phs", ArgType::Float, 7.0, "Pixels round the cursor the closest component is highlighted in (7)."),
    ("preSelectTweakDeadSpace", "pdt", ArgType::Int, 120.0, "As preSelectDeadSpace, in tweak mode (120)."),
    ("selectTypeChangeAffectsActive", "stc", ArgType::Bool, 0.0, "A selectType change takes what can no longer be picked out of the selection."),
    ("selectionChildHighlightMode", "sch", ArgType::Int, 0.0, "Children of a selected object highlighted: 0 always, 1 never, 2 by each object's Selection Child Highlighting."),
    ("singleBoxSelection", "sbs", ArgType::Bool, 0.0, "A marquee over several objects selects only one."),
    ("straightLineDistance", "sld", ArgType::Bool, 0.0, "Nearness for selection measured in a straight line."),
    ("trackSelectionOrder", "tso", ArgType::Bool, 0.0, "Keeps the order things are picked in, for ls -orderedSelection."),
    ("useDepth", "ud", ArgType::Bool, 0.0, "A marquee leaves out components behind surfaces."),
    ("xformNoSelect", "xns", ArgType::Bool, 0.0, "The transform tools don't select."),
];

fn pref_value(name: &str) -> Value {
    let Some(p) = PREFS.iter().find(|p| p.0 == name) else { return Value::Null };
    match p.2 {
        ArgType::Bool => json!(crate::ui_cmds::pref_bool(name, p.3 != 0.0)),
        ArgType::Int => json!(crate::ui_cmds::pref_f64(name, p.3) as i64),
        _ => json!(crate::ui_cmds::pref_f64(name, p.3)),
    }
}

fn select_pref(_s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        let asked: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
        let Some(p) = PREFS.iter().find(|p| asked.iter().any(|q| q == p.0 || q == p.1)).or_else(|| PREFS.iter().find(|p| a.has(p.0))) else { fail!("selectPref: query which setting?") };
        return Ok(pref_value(p.0));
    }
    for p in PREFS {
        let Some(v) = a.get(p.0) else { continue };
        let v = match p.2 {
            ArgType::Bool => json!(on(v)),
            ArgType::Int => {
                let n = v.as_f64().map(|x| x as i64).unwrap_or(if on(v) { 1 } else { 0 });
                if matches!(p.0, "clickBoxSize" | "manipClickBoxSize" | "preSelectDeadSpace" | "preSelectTweakDeadSpace") && n <= 0 {
                    fail!("selectPref: -{} must be positive", p.0);
                }
                if p.0 == "selectionChildHighlightMode" && !(0..=2).contains(&n) {
                    fail!("selectPref: -selectionChildHighlightMode is 0, 1 or 2");
                }
                json!(n)
            }
            _ => {
                let x = v.as_f64().unwrap_or(0.0);
                if x <= 0.0 {
                    fail!("selectPref: -{} must be positive", p.0);
                }
                json!(x)
            }
        };
        crate::ui_cmds::set_pref(p.0, v);
    }
    Ok(Value::Null)
}

pub fn register(s: &mut Session) {
    let mut st = Cmd::new(
        "selectType",
        "Which kinds of object and component the select tools pick. Each object flag (-polymesh, -joint, -nurbsCurve, -light...) turns its mask on or off; \
         component flags (-vertex, -polymeshEdge, -controlVertex, -hull...) set the component masks, and the kinds Aztek has component modes for switch \
         to that mode (-pv -pe -pf together are Multi). -objectComponent sets and reads the component masks that apply in object mode. -allObjects and \
         -allComponents set every mask at once, before the single flags. -byName name on|off sets a user-defined mask; -q -queryByName name reads it. -q reads one mask.",
        select_type,
    )
    .undoable(false)
    .flag(f("query", "q", ArgType::Bool, "Read a mask."))
    .flag(f("allObjects", "alo", ArgType::Bool, "Every object mask on or off."))
    .flag(f("allComponents", "alc", ArgType::Bool, "Every component mask on or off."))
    .flag(f("objectComponent", "ocm", ArgType::Bool, "The component flags given apply to object selection mode."))
    .flag(f("byName", "bn", ArgType::Any, "A user-defined mask and whether it is on: -bn name on.").multi(2))
    .flag(f("queryByName", "qbn", ArgType::String, "With -q: a user-defined mask, by name."));
    for (n, sh, doc) in OBJECT_MASKS {
        st = st.flag(f(n, sh, ArgType::Bool, doc));
    }
    for (n, sh, doc) in COMPONENT_MASKS {
        st = st.flag(f(n, sh, ArgType::Bool, doc));
    }
    s.register(st);

    let mut sp = Cmd::new("selectPref", "Selection preferences: how clicks, marquees, preselection and the select tools behave. -q reads one.", select_pref)
        .undoable(false)
        .flag(f("query", "q", ArgType::Bool, "Read a setting."))
        .flag(f("queried", "", ArgType::List, "The settings a query asks for."));
    for (n, sh, ty, _, doc) in PREFS {
        sp = sp.flag(f(n, sh, *ty, doc));
    }
    s.register(sp);
}
