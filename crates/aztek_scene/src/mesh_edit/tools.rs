//! What the interactive modelling tools run: polySlideEdge (Slide Edge Tool), polyAppendVertex (Create Polygon, Append to
//! Polygon, Quad Draw), polyCrease (Crease Tool), makeLive (Make Live, the surface Quad Draw draws on).

use std::collections::{BTreeMap, HashMap, HashSet};

use aztek_geo::symmetry::SymmetryMap;
use aztek_geo::{Index, Mesh, MeshRemap};
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use glam::DVec3;
use serde_json::{json, Value};

use super::{base_mesh, base_plug, edges, edit_selected_op, mesh_flag, store_edited, EditResult};
use crate::components::{self, ComponentMode as K, Ids};
use crate::session::Session;
use crate::transform::Transform;

type Cmd = CommandDef<Session>;

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

/// The edge a selected edge's point slides along within one of the edge's faces: the other edge at the point in that face.
fn side_edge(m: &Mesh, e: Index, face: Index, p: Index) -> Option<Index> {
    for c in m.face_corners(face) {
        let x = m.corner_edge(c);
        if x != e && (m.edge_point(x, 0) == p || m.edge_point(x, 1) == p) {
            return Some(x);
        }
    }
    None
}

/// Per point of the selected edges, its slide targets to each side: (point, toward side A, toward side B). Sides stay
/// consistent along a connected run of edges.
pub fn slide_targets(m: &Mesh, sel: &[Index]) -> Vec<(Index, Option<Index>, Option<Index>)> {
    let mut face_of: HashMap<Index, [Option<Index>; 2]> = HashMap::new();
    let set: HashSet<Index> = sel.iter().copied().collect();
    let mut seen = HashSet::new();
    // Starting from each unvisited edge: its faces in order give sides A and B; neighbours take theirs from the shared
    // slide edge.
    for &start in sel {
        if !seen.insert(start) {
            continue;
        }
        let mut faces = Vec::new();
        m.for_each_edge_corner(start, |c| faces.push(m.corner_face(c)));
        face_of.insert(start, [faces.first().copied(), faces.get(1).copied()]);
        let mut todo = vec![start];
        while let Some(e) = todo.pop() {
            let sides = face_of[&e];
            for i in 0..2 {
                let p = m.edge_point(e, i);
                let mut next = None;
                m.for_each_point_edge(p, |x| {
                    if x != e && set.contains(&x) && !seen.contains(&x) {
                        next = Some(x);
                    }
                });
                let Some(n) = next else { continue };
                seen.insert(n);
                let mut nf = Vec::new();
                m.for_each_edge_corner(n, |c| nf.push(m.corner_face(c)));
                // A neighbour's side A is the face sharing side A's slide edge at p.
                let via = sides[0].and_then(|fa| side_edge(m, e, fa, p));
                let a_first = match (via, nf.first()) {
                    (Some(v), Some(&f0)) => side_edge(m, n, f0, p) == Some(v),
                    _ => true,
                };
                let pair = if a_first { [nf.first().copied(), nf.get(1).copied()] } else { [nf.get(1).copied(), nf.first().copied()] };
                face_of.insert(n, pair);
                todo.push(n);
            }
        }
    }
    let mut out: BTreeMap<Index, (Option<Index>, Option<Index>)> = BTreeMap::new();
    for &e in sel {
        let sides = face_of[&e];
        for i in 0..2 {
            let p = m.edge_point(e, i);
            let side = |face: Option<Index>| -> Option<Index> {
                let x = side_edge(m, e, face?, p)?;
                // Sliding along another selected edge would fold the loop.
                if set.contains(&x) {
                    return None;
                }
                Some(m.edge_other_point(x, p))
            };
            let (a, b) = (side(sides[0]), side(sides[1]));
            let entry = out.entry(p).or_default();
            entry.0 = entry.0.or(a);
            entry.1 = entry.1.or(b);
        }
    }
    out.into_iter().map(|(p, (a, b))| (p, a, b)).collect()
}

fn slide_op(a: &Args, m: &mut Mesh, sel: &Ids, _sym: Option<&SymmetryMap>) -> CmdResult<EditResult> {
    let targets = slide_targets(m, sel);
    let before = m.positions().to_vec();
    // The reference's -edgeDirection: a share of the side edges, toward -direction 0 (one side), 1 (the other) or 2
    // (along the vertex normals); -absolute moves every vertex that share of the shortest side edge.
    let ed = a.get("edgeDirection").and_then(Value::as_f64).filter(|x| *x != 0.0);
    let Some(ed) = ed else {
        let v = a.float("value", 0.0).clamp(-1.0, 1.0);
        for (p, side_a, side_b) in targets {
            let to = if v >= 0.0 { side_a } else { side_b };
            let Some(t) = to else { continue };
            let at = before[p as usize].lerp(before[t as usize], v.abs());
            m.set_position(p, at);
        }
        return Ok(edges(sel.clone()));
    };
    let dir = a.int("direction", 0);
    let side_len = |p: Index, t: Option<Index>| t.map(|t| before[p as usize].distance(before[t as usize]));
    let shortest = targets
        .iter()
        .flat_map(|&(p, x, y)| [side_len(p, x), side_len(p, y)])
        .flatten()
        .filter(|l| *l > 1e-12)
        .fold(f64::INFINITY, f64::min);
    let absolute = a.bool("absolute") && shortest.is_finite();
    let normals = if dir == 2 { aztek_geo::normals::point_normals(m, aztek_geo::normals::NormalWeight::Area) } else { Vec::new() };
    for (p, side_a, side_b) in targets {
        let from = before[p as usize];
        let at = if dir == 2 {
            let mean = [side_len(p, side_a), side_len(p, side_b)].into_iter().flatten().sum::<f64>() / 2.0;
            let len = if absolute { shortest } else { mean };
            from + normals.get(p as usize).map_or(DVec3::ZERO, |n| n.as_dvec3()) * ed * len
        } else {
            let Some(t) = (if dir == 1 { side_a } else { side_b }) else { continue };
            let to = before[t as usize];
            if absolute {
                from + (to - from).normalize_or_zero() * ed * shortest
            } else {
                from.lerp(to, ed)
            }
        };
        m.set_position(p, at);
    }
    Ok(edges(sel.clone()))
}

#[derive(Clone, Debug)]
pub enum FaceItem {
    Vertex(Index),
    /// World.
    Position(DVec3),
}

/// Builds a face from existing vertices and new positions on a mesh, winding it to match any neighbour sharing an edge.
fn append_face(m: &mut Mesh, items: &[FaceItem], to_local: &glam::DMat4) -> Option<Index> {
    let mut pts = Vec::with_capacity(items.len());
    for it in items {
        pts.push(match it {
            FaceItem::Vertex(v) => {
                if !m.point_valid(*v) {
                    return None;
                }
                *v
            }
            FaceItem::Position(p) => m.add_point(to_local.transform_point3(*p)),
        });
    }
    pts.dedup();
    if pts.first() == pts.last() && pts.len() > 1 {
        pts.pop();
    }
    if pts.len() < 3 {
        return None;
    }
    // An existing edge already running a -> b means this face must run b -> a.
    let mut flip = false;
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        let e = m.find_edge(a, b);
        if e == aztek_geo::INVALID {
            continue;
        }
        let mut same = false;
        m.for_each_edge_corner(e, |c| {
            if m.corner_point(c) == a && m.corner_next_point(c) == b {
                same = true;
            }
        });
        if same {
            flip = true;
            break;
        }
    }
    if flip {
        pts.reverse();
    }
    let f = m.add_face(&pts);
    (f != aztek_geo::INVALID).then_some(f)
}

fn parse_items(a: &Args) -> CmdResult<Vec<FaceItem>> {
    let mut items = Vec::new();
    if let Some(list) = a.get("items").and_then(Value::as_array) {
        for it in list {
            if let Some(v) = it.get("vertex").and_then(Value::as_u64) {
                items.push(FaceItem::Vertex(v as Index));
            } else if let Some(p) = it.get("position") {
                items.push(FaceItem::Position(super::vec(p)));
            } else {
                fail!("polyAppendVertex: each item is {{vertex:}} or {{position:}}");
            }
        }
    }
    // The reference's -a and -p in order: all vertices, then all points, when given apart.
    if let Some(v) = a.get("append") {
        let list: Vec<i64> = match v {
            Value::Array(x) => x.iter().filter_map(Value::as_i64).collect(),
            x => x.as_i64().into_iter().collect(),
        };
        items.extend(list.into_iter().map(|i| FaceItem::Vertex(i as Index)));
    }
    if let Some(Value::Array(x)) = a.get("point") {
        let mut nums = Vec::new();
        fn walk(v: &Value, out: &mut Vec<f64>) {
            match v {
                Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
                y => out.extend(y.as_f64()),
            }
        }
        x.iter().for_each(|v| walk(v, &mut nums));
        items.extend(nums.chunks(3).filter(|c| c.len() == 3).map(|c| FaceItem::Position(DVec3::new(c[0], c[1], c[2]))));
    }
    Ok(items)
}

/// The registry's size when nothing has set it.
const REGISTRY_DEFAULT: usize = 10;

thread_local! {
    /// makeLive's registry: the sets of objects made live, newest first, and how many it keeps.
    static REGISTRY: std::cell::RefCell<(Vec<Vec<String>>, usize)> = const { std::cell::RefCell::new((Vec::new(), REGISTRY_DEFAULT)) };
}

fn set_live(s: &mut Session, list: Vec<crate::NodeId>) {
    // The live surface can't be selected: it leaves the selection, as in the reference (#116).
    let drop: Vec<crate::NodeId> = list.iter().flat_map(|&m| [m, crate::dag::transform_of(&s.graph, m)]).collect();
    s.graph.select(&drop, crate::graph::SelectMode::Remove);
    s.graph.live = list.clone();
    s.live_surface = list;
}

fn live_meshes(s: &Session, names: &[String]) -> CmdResult<Vec<crate::NodeId>> {
    let mut list = Vec::new();
    for n in names {
        let Some(id) = s.graph.find(n)? else { fail!("makeLive: no object named {n}") };
        let Some(m) = components::as_mesh(&s.graph, id) else { fail!("makeLive: {n} can't be made live: only meshes can") };
        if !list.contains(&m) {
            list.push(m);
        }
    }
    Ok(list)
}

fn make_live(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        if a.bool("registryCount") {
            return Ok(json!(REGISTRY.with(|r| r.borrow().0.len())));
        }
        if a.has("registrySize") && a.get("registrySize").and_then(Value::as_i64).is_none() || a.bool("registrySize") {
            return Ok(json!(REGISTRY.with(|r| r.borrow().1)));
        }
        if a.has("registry") {
            // In query mode the entry's number may come after the flag as an argument.
            let given = a.positional.iter().find_map(|v| v.as_i64().or_else(|| v.as_str().and_then(|t| t.parse().ok())));
            let Some(i) = given.or_else(|| a.get("registry").and_then(Value::as_i64)) else { fail!("makeLive: -q -registry takes the entry's number") };
            return Ok(json!(REGISTRY.with(|r| r.borrow().0.get(i.max(0) as usize).cloned().unwrap_or_default())));
        }
        return Ok(json!(s.live_surface.iter().map(|&n| s.graph.path(n)).collect::<Vec<_>>()));
    }
    if a.bool("registryReset") {
        REGISTRY.with(|r| *r.borrow_mut() = (Vec::new(), REGISTRY_DEFAULT));
        return Ok(Value::Null);
    }
    if let Some(n) = a.get("registrySize").and_then(Value::as_i64) {
        REGISTRY.with(|r| {
            let mut r = r.borrow_mut();
            r.1 = n.max(0) as usize;
            let keep = r.1;
            r.0.truncate(keep);
        });
        if a.named().is_empty() && !a.has("registry") {
            return Ok(Value::Null);
        }
    }
    if a.bool("none") {
        s.live_surface.clear();
        s.graph.live.clear();
        return Ok(Value::Null);
    }
    if let Some(i) = a.get("registry").and_then(Value::as_i64) {
        let Some(names) = REGISTRY.with(|r| r.borrow().0.get(i.max(0) as usize).cloned()) else { fail!("makeLive: the registry has no entry {i}") };
        let list = live_meshes(s, &names)?;
        set_live(s, list);
        return Ok(Value::Null);
    }
    let names: Vec<String> = if a.named().is_empty() { s.graph.selection().iter().map(|&i| s.graph.path(i)).collect() } else { a.named() };
    if a.bool("edit") && (a.bool("addObjects") || a.bool("removeObjects")) {
        let given = live_meshes(s, &names)?;
        let mut list = s.live_surface.clone();
        if a.bool("addObjects") {
            list.extend(given.into_iter().filter(|m| !s.live_surface.contains(m)));
        } else {
            list.retain(|m| !given.contains(m));
        }
        set_live(s, list);
        return Ok(Value::Null);
    }
    let list = live_meshes(s, &names)?;
    if !list.is_empty() {
        let paths: Vec<String> = list.iter().map(|&m| s.graph.path(crate::dag::transform_of(&s.graph, m))).collect();
        REGISTRY.with(|r| {
            let mut r = r.borrow_mut();
            r.0.retain(|x| *x != paths);
            r.0.insert(0, paths);
            let keep = r.1;
            r.0.truncate(keep);
        });
    }
    set_live(s, list);
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    register_crease_op();
    crate::history::register_op(crate::history::HistoryOp {
        node_type: "polySlideEdge",
        command: "polySlideEdge",
        label: "Slide Edge",
        kind: K::Edge,
        run: slide_op,
        skip_flags: &[],
        doc: "Slides edges along the edges beside them.",
    });
    c.register(
        Cmd::new(
            "polySlideEdge",
            "Slide the selected edges within the faces beside them: -v -1 to 1, a fraction of the way along the neighbouring edges to \
             one side or the other.",
            |s, a| edit_selected_op(s, "polySlideEdge", a, false, None),
        )
        .flag(f("value", "v", ArgType::Float, "How far: -1 to 1."))
        .flag(f("edgeDirection", "ed", ArgType::Float, "How far, as a share of the side edges (0.5 is half way; more than 1 goes past them)."))
        .flag(f("absolute", "a", ArgType::Bool, "With -ed: every vertex moves the same distance, that share of the shortest side edge."))
        .flag(f("direction", "d", ArgType::Int, "With -ed: 0 toward one side, 1 toward the other, 2 along the vertex normals."))
        .flag(f("edgeFlow", "ef", ArgType::Bool, "Accepted for existing scripts."))
        .flag(f("symmetry", "sym", ArgType::Bool, "Accepted for existing scripts.")),
    );

    c.register(
        Cmd::new(
            "polyAppendVertex",
            "Add a face to a mesh from existing vertices and new points: -a vertex (repeat), -p x y z (repeat), or -items [{vertex:}, \
             {position:}] in order. With no mesh named or selected and only points, makes a new mesh. Returns the new face and the mesh.",
            |s, a| {
                let items = parse_items(a)?;
                if items.len() < 3 {
                    fail!("polyAppendVertex: a face needs three points");
                }
                let g = &mut s.graph;
                g.evaluate();
                let target = mesh_flag(g, a, "polyAppendVertex").ok();
                let (n, fresh) = match target {
                    Some(n) => (n, false),
                    None => {
                        if items.iter().any(|i| matches!(i, FaceItem::Vertex(_))) {
                            fail!("polyAppendVertex: name a mesh to use its vertices");
                        }
                        let (_tr, sh) = crate::mesh_nodes::create_mesh_node(g, "polySurface1")?;
                        (sh, true)
                    }
                };
                g.open_chunk("Append to Polygon");
                let r = (|| -> CmdResult {
                    let base = base_plug(g, n)?;
                    let mut mesh = base_mesh(g, &base);
                    let world = g.node(n).unwrap().matrix(Transform::WORLD_MATRIX);
                    // With symmetry the face's image is made too, wound the other way.
                    let spec = if fresh { None } else { crate::mesh_nodes::symmetry_spec(g, n) };
                    let sym = spec.as_ref().filter(|s| !s.is_radial()).and_then(|_| crate::mesh_edit::symmetry_for(g, n, &mut mesh));
                    let to_local = world.inverse();
                    let image: Option<Vec<FaceItem>> = match (&spec, &sym) {
                        (Some(sp), Some(map)) => {
                            let mut out = Vec::new();
                            for i in items.iter().rev() {
                                out.push(match i {
                                    FaceItem::Vertex(v) => FaceItem::Vertex(map.image(1, *v)),
                                    FaceItem::Position(p) => FaceItem::Position(world.transform_point3(sp.apply(1, to_local.transform_point3(*p)))),
                                });
                            }
                            let lies_on_plane = items.iter().zip(out.iter().rev()).all(|(a, b)| match (a, b) {
                                (FaceItem::Vertex(x), FaceItem::Vertex(y)) => x == y,
                                (FaceItem::Position(x), FaceItem::Position(y)) => (*x - *y).length() < 1e-9,
                                _ => false,
                            });
                            let usable = out.iter().all(|i| !matches!(i, FaceItem::Vertex(v) if *v == aztek_geo::INVALID));
                            (usable && !lies_on_plane).then_some(out)
                        }
                        _ => None,
                    };
                    let Some(face) = append_face(&mut mesh, &items, &world.inverse()) else { fail!("polyAppendVertex: these points don't make a face") };
                    if let Some(other) = &image {
                        let _ = append_face(&mut mesh, other, &to_local);
                    }
                    store_edited(g, n, base, mesh, &MeshRemap::default())?;
                    let path = components::display_path(g, n);
                    if fresh {
                        let tr = crate::dag::transform_of(g, n);
                        g.select(&[tr], crate::graph::SelectMode::Replace);
                    }
                    Ok(json!([path, format!("f[{face}]")]))
                })();
                g.close_chunk();
                r
            },
        )
        .flag(f("mesh", "", ArgType::String, "The mesh; the selected one by default."))
        .flag(f("append", "a", ArgType::Any, "An existing vertex. Repeat it.").multi(1))
        .flag(f("point", "p", ArgType::List, "A new point: x y z. Repeat it.").multi(3))
        .flag(f("items", "", ArgType::List, "In order: {vertex: id} or {position: [x, y, z]}."))
        .flag(f("constructionHistory", "ch", ArgType::Bool, "Accepted for existing scripts.")),
    );

    c.register(
        Cmd::new(
            "polyCrease",
            "Set crease values on edges (-v) and vertices (-vv): they hold sharper under smoothing. -rv adds to what they have; \
             -op 1 takes the crease off the components, -op 2 off the whole mesh. -v and -vv repeat, one value per component \
             in order. -q -v and -q -vv read them back.",
            poly_crease,
        )
        .positional("edges or vertices (the selection when none)")
        .flag(f("value", "v", ArgType::Any, "Crease of the edges, 0 to 10; repeat it for one value per edge.").multi(1))
        .flag(f("vertexValue", "vv", ArgType::Any, "Crease of the vertices; repeat it for one value per vertex.").multi(1))
        .flag(f("relativeValue", "rv", ArgType::Float, "Add this to the crease of every edge and vertex named."))
        .flag(f("operation", "op", ArgType::Int, "0 crease the components, 1 take their crease off, 2 take every crease off the mesh."))
        .flag(f("createHistory", "ch", ArgType::Bool, "Make a polyCreaseEdge history node on a mesh that has no history (one with history always gets one)."))
        .flag(f("query", "q", ArgType::Bool, "Query mode."))
        .flag(f("edit", "e", ArgType::Bool, "Edit mode: the same as creasing."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("creaseVertices", "", ArgType::List, "The vertices a polyCreaseEdge node creases, worked out from the arguments.")),
    );

    c.register(
        Cmd::new(
            "makeLive",
            "Make meshes the live surface: Quad Draw and the snap to live draw on it. -none clears it; -e -ao adds, -e -ro takes off. \
             Each set made live is remembered in a registry of -registrySize entries, newest first; -registry n makes one live again.",
            make_live,
        )
        .undoable(false)
        .positional("meshes (the selection when none)")
        .flag(f("none", "n", ArgType::Bool, "Make the live objects dormant."))
        .flag(f("addObjects", "ao", ArgType::Bool, "With -e: add the objects named to the live ones."))
        .flag(f("removeObjects", "ro", ArgType::Bool, "With -e: take the objects named off the live ones."))
        .flag(f("registry", "r", ArgType::Int, "Make the objects of this registry entry live; with -q, the objects in it."))
        .flag(f("registryCount", "rc", ArgType::Bool, "With -q: how many entries the registry holds."))
        .flag(f("registryReset", "rr", ArgType::Bool, "Empty the registry and put its size back to the default."))
        .flag(f("registrySize", "rs", ArgType::Int, "The most entries the registry remembers; with -q, that number."))
        .flag(f("query", "q", ArgType::Bool, "Query mode."))
        .flag(f("edit", "e", ArgType::Bool, "Edit mode.")),
    );
}

/// Numbers of a flag given once or repeated.
fn numbers(a: &Args, flag: &str) -> Vec<f64> {
    match a.get(flag) {
        Some(Value::Array(l)) => l.iter().filter_map(Value::as_f64).collect(),
        Some(v) => v.as_f64().into_iter().collect(),
        None => Vec::new(),
    }
}

/// The value for the i-th component: one each, or the last for the rest.
fn nth(values: &[f64], i: usize) -> Option<f64> {
    values.get(i).or(values.last()).copied()
}

pub(crate) const VERTEX_CREASE: &str = "creaseVertex";

/// Creases the edges and vertices as the flags say, on a mesh.
pub(crate) fn apply_crease(a: &Args, m: &mut Mesh, edges: &[Index], verts: &[Index]) {
    use aztek_geo::attribute::{names::CREASE, AttrType};
    let op = a.int("operation", 0);
    m.edge_attrs_mut().add(CREASE, AttrType::Float);
    m.point_attrs_mut().add(VERTEX_CREASE, AttrType::Float);
    if op == 2 {
        m.edge_attrs_mut().get_mut::<f32>(CREASE).iter_mut().for_each(|x| *x = 0.0);
        m.point_attrs_mut().get_mut::<f32>(VERTEX_CREASE).iter_mut().for_each(|x| *x = 0.0);
        return;
    }
    let (ev, vv) = (numbers(a, "value"), numbers(a, "vertexValue"));
    // A node's relativeValue of 0 (its default) adds nothing: the values set it.
    let rel = a.get("relativeValue").and_then(Value::as_f64).filter(|r| *r != 0.0);
    let set = |list: &mut [f32], ids: &[Index], vals: &[f64]| {
        for (i, &id) in ids.iter().enumerate() {
            let Some(x) = list.get_mut(id as usize) else { continue };
            if op == 1 {
                *x = 0.0;
            } else if let Some(r) = rel {
                *x = (*x as f64 + r).clamp(0.0, 10.0) as f32;
            } else if let Some(v) = nth(vals, i) {
                *x = v.clamp(0.0, 10.0) as f32;
            }
        }
    };
    set(m.edge_attrs_mut().get_mut::<f32>(CREASE), edges, &ev);
    set(m.point_attrs_mut().get_mut::<f32>(VERTEX_CREASE), verts, &vv);
}

fn crease_op(a: &Args, m: &mut Mesh, sel: &Ids, _sym: Option<&SymmetryMap>) -> CmdResult<EditResult> {
    let verts: Vec<Index> = a.get("creaseVertices").and_then(Value::as_array).map(|l| l.iter().filter_map(Value::as_u64).map(|x| x as Index).collect()).unwrap_or_default();
    apply_crease(a, m, sel, &verts);
    Ok(EditResult::default())
}

pub(crate) fn register_crease_op() {
    crate::history::register_op(crate::history::HistoryOp {
        node_type: "polyCreaseEdge",
        command: "polyCrease",
        label: "Crease",
        kind: K::Edge,
        run: crease_op,
        skip_flags: &["edit"],
        doc: "Sets crease values on edges and vertices of its input mesh.",
    });
}

fn poly_crease(s: &mut Session, a: &Args) -> CmdResult {
    use aztek_geo::attribute::names::CREASE;
    let members = crate::mesh_edit::ref_poly::named_members(s, a)?;
    let g = &mut s.graph;
    g.evaluate();
    // Each mesh's edges and vertices: vertices stay vertices, the rest go to their edges.
    let mut picked: BTreeMap<crate::plug::NodeId, (Ids, Ids)> = BTreeMap::new();
    let mut add = |g: &mut crate::graph::Graph, node: crate::plug::NodeId, kind: K, ids: &Ids| {
        let Some(mesh) = components::as_mesh(g, node) else { return };
        let Some(cage) = crate::modifiers::cage(g, mesh) else { return };
        let e = picked.entry(mesh).or_default();
        match kind {
            K::Vertex => e.1.extend(ids.iter().copied()),
            K::Object => e.0.extend(components::all_ids(&cage, K::Edge)),
            k => e.0.extend(components::convert(&cage, k, ids, K::Edge, false)),
        }
    };
    if !members.is_empty() {
        for m in &members {
            add(g, m.node, m.kind, &m.ids);
        }
    } else {
        let mode = components::mode(g);
        if mode == K::Object {
            for id in g.selection().to_vec() {
                add(g, id, K::Object, &Vec::new());
            }
        } else {
            for (n, ids) in components::selected_all(g, mode).clone() {
                add(g, n, mode, &ids);
            }
        }
    }
    if picked.is_empty() {
        fail!("polyCrease: select or name edges or vertices");
    }
    for (e, v) in picked.values_mut() {
        e.sort_unstable();
        e.dedup();
        v.sort_unstable();
        v.dedup();
    }
    let asked = |f: &str| a.bool(f) || a.get("queried").and_then(Value::as_array).is_some_and(|q| q.iter().any(|x| x == f));
    if a.bool("query") {
        let mut out = Vec::new();
        for (&n, (edges, verts)) in &picked {
            let Some(cage) = crate::modifiers::cage(g, n) else { continue };
            if asked("vertexValue") {
                let cr = cage.point_attrs().get::<f32>(VERTEX_CREASE);
                let verts: Ids = if verts.is_empty() { components::convert(&cage, K::Edge, edges, K::Vertex, false) } else { verts.clone() };
                out.extend(verts.iter().map(|&p| json!(cr.get(p as usize).copied().unwrap_or(0.0) as f64)));
            } else {
                let cr = cage.edge_attrs().get::<f32>(CREASE);
                let edges: Ids = if edges.is_empty() { components::convert(&cage, K::Vertex, verts, K::Edge, false) } else { edges.clone() };
                out.extend(edges.iter().map(|&e| json!(cr.get(e as usize).copied().unwrap_or(0.0) as f64)));
            }
        }
        return Ok(Value::Array(out));
    }
    // Vertices named with only -value take it as their value.
    let mut call = a.clone();
    if !call.has("vertexValue") && call.has("value") && picked.values().all(|(e, _)| e.is_empty()) {
        if let Some(v) = call.get("value").cloned() {
            call.flags.insert("vertexValue".into(), v);
        }
    }
    let force = a.get("createHistory").is_some_and(|v| v.as_bool().unwrap_or(v.as_i64().unwrap_or(0) != 0));
    let op = crate::history::op("polyCreaseEdge").ok_or_else(|| aztek_ops::CommandError::new("no polyCreaseEdge op"))?;
    g.open_chunk("Crease");
    let r = (|| -> CmdResult<Vec<crate::plug::NodeId>> {
        let mut made = Vec::new();
        for (n, (edges, verts)) in picked {
            let has_history = !crate::history::history_of(g, n).is_empty();
            let mut args = call.clone();
            args.flags.insert("creaseVertices".into(), json!(verts.clone()));
            args.flags.insert("constructionHistory".into(), json!(force || has_history));
            let (v1, a1) = (verts.clone(), args.clone());
            let got = super::apply_targets(g, K::Edge, BTreeMap::from([(n, edges)]), &mut |m, sel, _| {
                apply_crease(&a1, m, sel, &v1);
                Ok(EditResult::default())
            }, Some((&*op, &args)), None)?;
            made.extend(got);
        }
        Ok(made)
    })();
    g.close_chunk();
    r?;
    Ok(json!(true))
}
