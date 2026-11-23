//! Quad Draw and Multi-Cut as the reference's mode tools (#140): their options, set from the Modeling Toolkit, and the
//! drags beyond a click: quad strips, extending border edges by edge, loop or border, relaxing, centred loops, deleting
//! loops, and Multi-Cut's slice across the surface or along a plane.

use aztek_geo::{Index, Mesh};
use aztek_scene::components;
use aztek_scene::plug::NodeId;
use glam::{DVec2, DVec3};
use serde_json::{json, Value};

use crate::core::Core;
use crate::view::View;
use crate::viewport::DrawItem;

const GREEN: [u8; 4] = [70, 255, 120, 255];
const ORANGE: [u8; 4] = [235, 160, 30, 255];

/// Quad Draw Options, as the Modeling Toolkit sets them.
#[derive(Clone, Debug)]
pub struct QuadOpts {
    pub auto_weld: bool,
    /// Pixels: a dragged vertex let go this near another welds to it.
    pub weld_px: f64,
    /// Which points Shift+drag relaxes: 0 auto-lock (border points stay when there are inner ones), 1 interior, 2 border,
    /// 3 all.
    pub relax: u8,
    /// What Tab+drag on a border edge extends: 0 the edge, 1 its loop along the border, 2 the whole border.
    pub extend: u8,
    /// Pixels across a quad strip.
    pub strip_px: f64,
}

impl Default for QuadOpts {
    fn default() -> Self {
        QuadOpts { auto_weld: true, weld_px: 10.0, relax: 0, extend: 0, strip_px: 20.0 }
    }
}

/// Multi-Cut Options.
#[derive(Clone, Debug)]
pub struct CutOpts {
    /// Shift snaps a point along an edge in steps of this percentage.
    pub snap_step: f64,
    pub smoothing_angle: f64,
    pub edge_flow: bool,
    pub edge_flow_amount: f64,
    pub subdivisions: i64,
    pub ignore_backfaces: bool,
    pub delete_faces: bool,
    pub extract_faces: bool,
    pub extract_offset: [f64; 3],
    /// Force Cage + Smooth: smooth previews show their cage while the tool is on.
    pub force_cage: bool,
    /// Color Settings: the cut line, and points on a vertex, an edge or a face.
    pub line_color: [u8; 4],
    pub vertex_color: [u8; 4],
    pub edge_color: [u8; 4],
    pub face_color: [u8; 4],
}

impl Default for CutOpts {
    fn default() -> Self {
        CutOpts { snap_step: 10.0, smoothing_angle: 180.0, edge_flow: false, edge_flow_amount: 1.0, subdivisions: 1, ignore_backfaces: false, delete_faces: false, extract_faces: false, extract_offset: [0.5; 3], force_cage: false, line_color: [230, 160, 0, 255], vertex_color: [230, 220, 0, 255], edge_color: [0, 0, 220, 255], face_color: [220, 220, 220, 255] }
    }
}

/// A quad strip being drawn: where its last edge is on screen and which way it went, and that edge's vertices once
/// they exist.
#[derive(Clone, Debug, Default)]
pub struct Strip {
    pub last_px: DVec2,
    pub dir: DVec2,
    pub edge: Option<(Index, Index)>,
    pub started: bool,
}

/// Border edges being extended: the mesh, the edges, and where the drag started.
#[derive(Clone, Debug)]
pub struct Extend {
    pub mesh: NodeId,
    pub edges: Vec<Index>,
    pub from: DVec3,
}

/// What a middle drag moves along its normal.
#[derive(Clone, Debug)]
pub enum Dragged {
    Dot(usize),
    Vertex(NodeId, Index),
}

/// A middle drag along a normal: what, from where, along which world direction, and where the mouse went down.
#[derive(Clone, Debug)]
pub struct NormalDrag {
    pub what: Dragged,
    pub from: DVec3,
    pub normal: DVec3,
    pub press: DVec2,
}

fn f(v: &Value, k: &str, d: f64) -> f64 {
    v.get(k).and_then(Value::as_f64).unwrap_or(d)
}
fn b(v: &Value, k: &str, d: bool) -> bool {
    v.get(k).and_then(Value::as_bool).unwrap_or(d)
}

impl QuadOpts {
    pub fn set(&mut self, v: &Value) {
        self.auto_weld = b(v, "autoWeld", self.auto_weld);
        self.weld_px = f(v, "autoWeldDistance", self.weld_px).max(0.0);
        self.relax = f(v, "relax", self.relax as f64).clamp(0.0, 3.0) as u8;
        self.extend = f(v, "extend", self.extend as f64).clamp(0.0, 2.0) as u8;
        self.strip_px = f(v, "stripWidth", self.strip_px).max(1.0);
    }
}

impl CutOpts {
    pub fn set(&mut self, v: &Value) {
        self.snap_step = f(v, "snapStep", self.snap_step).clamp(0.0, 100.0);
        self.smoothing_angle = f(v, "smoothingAngle", self.smoothing_angle);
        self.edge_flow = b(v, "edgeFlow", self.edge_flow);
        self.edge_flow_amount = f(v, "edgeFlowAmount", self.edge_flow_amount);
        self.subdivisions = f(v, "subdivisions", self.subdivisions as f64).max(1.0) as i64;
        self.ignore_backfaces = b(v, "ignoreBackfaces", self.ignore_backfaces);
        self.delete_faces = b(v, "deleteFaces", self.delete_faces);
        self.extract_faces = b(v, "extractFaces", self.extract_faces);
        self.force_cage = b(v, "forceCage", self.force_cage);
        let col = |k: &str, d: [u8; 4]| crate::viewport::hex_color(v.get(k)).map_or(d, |c| [(c[0] * 255.0).round() as u8, (c[1] * 255.0).round() as u8, (c[2] * 255.0).round() as u8, 255]);
        self.line_color = col("lineColor", self.line_color);
        self.vertex_color = col("vertexColor", self.vertex_color);
        self.edge_color = col("edgeColor", self.edge_color);
        self.face_color = col("faceColor", self.face_color);
        if let Some(a) = v.get("extractOffset").and_then(Value::as_array) {
            for (k, x) in a.iter().take(3).enumerate() {
                self.extract_offset[k] = x.as_f64().unwrap_or(self.extract_offset[k]);
            }
        }
    }

    /// A fraction along an edge snapped to the step.
    pub fn snap(&self, t: f64) -> f64 {
        let step = (self.snap_step / 100.0).max(1e-6);
        ((t / step).round() * step).clamp(0.0, 1.0)
    }
}

/// How many edges meet at a point.
fn valence(m: &Mesh, p: Index) -> usize {
    let mut n = 0;
    m.for_each_point_edge(p, |_| n += 1);
    n
}

/// The border edges Tab+drag extends from `e`: the edge alone, its run along the border while the border goes straight
/// on through points of three edges, or the whole border it is on.
pub fn extend_edges(m: &Mesh, e: Index, mode: u8) -> Vec<Index> {
    if !m.is_boundary_edge(e) {
        return Vec::new();
    }
    match mode {
        2 => aztek_geo::topology::boundary_loops(m).into_iter().find(|l| l.edges.contains(&e)).map(|l| l.edges).unwrap_or_else(|| vec![e]),
        1 => {
            let mut out = vec![e];
            for end in 0..2 {
                let (mut cur, mut p) = (e, m.edge_point(e, end));
                loop {
                    if valence(m, p) != 3 {
                        break;
                    }
                    let mut next = None;
                    m.for_each_point_edge(p, |x| {
                        if x != cur && m.is_boundary_edge(x) {
                            next = Some(x);
                        }
                    });
                    let Some(x) = next.filter(|x| !out.contains(x)) else { break };
                    out.push(x);
                    p = m.edge_other_point(x, p);
                    cur = x;
                }
            }
            out
        }
        _ => vec![e],
    }
}

/// The points Shift+drag relaxes among those near the brush, by the Relax setting.
pub fn relax_points(m: &Mesh, near: &[Index], mode: u8) -> Vec<Index> {
    let border = |p: Index| {
        let mut on = false;
        m.for_each_point_edge(p, |e| on |= m.is_boundary_edge(e));
        on
    };
    let inner: Vec<Index> = near.iter().copied().filter(|&p| !border(p)).collect();
    match mode {
        1 => inner,
        2 => near.iter().copied().filter(|&p| border(p)).collect(),
        3 => near.to_vec(),
        // Auto-lock: the border holds while there are inner points to move; a strip with none relaxes its border.
        _ => {
            if inner.is_empty() {
                near.to_vec()
            } else {
                inner
            }
        }
    }
}

/// Each point moved a quarter of the way to the middle of its neighbours: one step of a relax. A border point only
/// looks along the border, so it slides along it and the border keeps its shape; a corner stays.
pub fn relaxed(m: &Mesh, pts: &[Index]) -> Vec<(Index, DVec3)> {
    pts.iter()
        .filter_map(|&p| {
            let mut on_border = false;
            m.for_each_point_edge(p, |e| on_border |= m.is_boundary_edge(e));
            if on_border && valence(m, p) <= 2 {
                return None;
            }
            let mut sum = DVec3::ZERO;
            let mut n = 0.0;
            m.for_each_point_edge(p, |e| {
                if !on_border || m.is_boundary_edge(e) {
                    sum += m.position(m.edge_other_point(e, p));
                    n += 1.0;
                }
            });
            (n > 0.0).then(|| (p, m.position(p).lerp(sum / n, 0.25)))
        })
        .collect()
}

/// The plane through the eye and a line across the screen: a point on it and its normal.
pub fn screen_plane(v: &View, a: DVec2, b: DVec2) -> (DVec3, DVec3) {
    let size = v.vp.size();
    let (oa, da) = v.vp.cam.ray(a, size);
    let (ob, db) = v.vp.cam.ray(b, size);
    let n = da.cross((ob + db) - oa).normalize_or_zero();
    (oa, n)
}

/// A screen line's end turned to the nearest 45 degrees from its start.
pub fn snap45(from: DVec2, to: DVec2) -> DVec2 {
    let d = to - from;
    let len = d.length();
    let step = std::f64::consts::FRAC_PI_4;
    let a = (d.y.atan2(d.x) / step).round() * step;
    from + DVec2::new(a.cos(), a.sin()) * len
}

/// polyCut's plane rotation for a plane with this normal, in radians: the tools run their commands in stored units.
pub fn cut_rotation(n: DVec3) -> [f64; 3] {
    let n = n.normalize_or_zero();
    [(-n.y).clamp(-1.0, 1.0).asin(), n.x.atan2(n.z), 0.0]
}

/// A mesh shape's evaluated mesh.
fn mesh_of(g: &aztek_scene::graph::Graph, shape: NodeId) -> Option<std::sync::Arc<Mesh>> {
    g.node(shape)?.data().mesh(aztek_scene::mesh_nodes::MeshNode::OUT_MESH).cloned()
}

/// The vertex of a mesh at a world place, as polyAppendVertex left it.
fn vertex_at(core: &mut Core, shape: NodeId, w: DVec3) -> Option<Index> {
    let g = &mut core.s.graph;
    g.evaluate();
    let m = mesh_of(g, shape)?;
    let xf = aztek_scene::dag::transform_of(g, shape);
    let world = aztek_scene::rig::world(g, xf);
    let local = world.inverse().transform_point3(w);
    (0..m.point_capacity()).filter(|&p| m.point_valid(p)).min_by(|&x, &y| (m.position(x) - local).length().total_cmp(&(m.position(y) - local).length())).filter(|&p| (m.position(p) - local).length() < 1e-5)
}

/// How many points a mesh shape has room for now: points numbered from here on are new.
pub(crate) fn point_count(core: &mut Core, shape: NodeId) -> Index {
    core.s.graph.evaluate();
    mesh_of(&core.s.graph, shape).map(|m| m.point_capacity()).unwrap_or(0)
}

/// With a live surface, the points an edit just made on a mesh, numbered from `from` on, go onto it at their closest
/// point (#163): Quad Draw's and Multi-Cut's new loops and cuts lie on the surface, not across its curve. Part of the
/// edit's undo step when the caller opened one.
pub(crate) fn snap_new_points_to_live(core: &mut Core, shape: NodeId, from: Index) {
    let Some(&live) = core.s.live_surface.first() else { return };
    if live == shape {
        return;
    }
    let g = &mut core.s.graph;
    g.evaluate();
    let (Some(surface), Some(m)) = (mesh_of(g, live), mesh_of(g, shape)) else { return };
    let live_world = { let xf = aztek_scene::dag::transform_of(g, live); aztek_scene::rig::world(g, xf) };
    let world = { let xf = aztek_scene::dag::transform_of(g, shape); aztek_scene::rig::world(g, xf) };
    let mut ws = (*surface).clone();
    for p in 0..ws.point_capacity() {
        if ws.point_valid(p) {
            let at = live_world.transform_point3(ws.position(p));
            ws.set_position(p, at);
        }
    }
    let bvh = aztek_geo::bvh::MeshBvh::new(&ws);
    let name = components::display_path(g, shape);
    let moves: Vec<(Index, DVec3)> = (from..m.point_capacity())
        .filter(|&p| m.point_valid(p))
        .filter_map(|p| bvh.closest_point(world.transform_point3(m.position(p))).map(|c| (p, c.point)))
        .collect();
    for (p, at) in moves {
        let _ = core.exec_silent("xform", json!({"args": [format!("{name}.vtx[{p}]")], "worldSpace": true, "translation": at.to_array()}));
    }
}

impl View {
    /// A middle press in Quad Draw with no live surface: a dot or a vertex under the mouse starts moving along its
    /// normal, a dot's being the grid's up.
    pub(crate) fn quad_normal_press(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) -> bool {
        if !core.s.live_surface.is_empty() {
            return false;
        }
        let screen = self.vp.screen();
        let reach = 12.0 * self.dpr;
        if let Some(i) = self.ts.dots.iter().position(|d| screen.project(*d).is_some_and(|q| (q - px).length() < reach)) {
            self.ts.normal_drag = Some(NormalDrag { what: Dragged::Dot(i), from: self.ts.dots[i], normal: DVec3::Y, press: px });
            return true;
        }
        let Some(it) = self.quad_target(core, items) else { return false };
        let (shape, world, m) = (it.shape, it.world, it.mesh.clone());
        let Some(p) = (0..m.point_capacity()).filter(|&p| m.point_valid(p)).find(|&p| screen.project(world.transform_point3(m.position(p))).is_some_and(|q| (q - px).length() < reach)) else { return false };
        let n = m.point_faces(p).into_iter().map(|f| m.face_area_vector(f)).sum::<DVec3>();
        let n = world.inverse().transpose().transform_vector3(n).normalize_or_zero();
        if n == DVec3::ZERO {
            return false;
        }
        self.ts.applied = false;
        self.ts.normal_drag = Some(NormalDrag { what: Dragged::Vertex(shape, p), from: world.transform_point3(m.position(p)), normal: n, press: px });
        let weights = aztek_scene::mesh_edit::transform::soft_weights(&core.s.graph, &m, &world, &[p]);
        let moving: Vec<(Index, f32)> = (0..m.point_capacity()).filter(|&q| m.point_valid(q) && weights[q as usize] > 0.0).map(|q| (q, weights[q as usize])).collect();
        self.ts.normal_soft = (moving.len() > 1).then(|| ((0..m.point_capacity()).map(|q| if m.point_valid(q) { m.position(q) } else { DVec3::ZERO }).collect(), world, moving));
        true
    }

    /// The middle drag along the normal: as far along it as the mouse has gone along its line on screen.
    pub(crate) fn quad_normal_drag(&mut self, core: &mut Core, px: DVec2) {
        let Some(d) = self.ts.normal_drag.clone() else { return };
        let screen = self.vp.screen();
        let len = self.vp.handle_len(d.from, self.dpr).max(1e-6);
        let (Some(a), Some(b)) = (screen.project(d.from), screen.project(d.from + d.normal * len)) else { return };
        let dir = b - a;
        if dir.length_squared() < 1e-6 {
            return;
        }
        let t = (px - d.press).dot(dir) / dir.length_squared();
        let to = d.from + d.normal * len * t;
        match d.what {
            Dragged::Dot(i) => {
                if let Some(x) = self.ts.dots.get_mut(i) {
                    *x = to;
                }
            }
            Dragged::Vertex(mesh, v) => {
                let name = components::display_path(&core.s.graph, mesh);
                // With soft selection on, the points round it go the same way by its falloff (#166).
                if let Some((lo, hi, vals)) = self.ts.normal_soft.as_ref().map(|(start, world, moving)| {
                    let inv = world.inverse();
                    let by = to - d.from;
                    let lo = moving.iter().map(|x| x.0).min().unwrap_or(0);
                    let hi = moving.iter().map(|x| x.0).max().unwrap_or(0);
                    let mut vals = Vec::new();
                    for p in lo..=hi {
                        let w = moving.iter().find(|x| x.0 == p).map(|x| x.1).unwrap_or(0.0) as f64;
                        let at = inv.transform_point3(world.transform_point3(start[p as usize]) + by * w);
                        vals.extend([json!(at.x), json!(at.y), json!(at.z)]);
                    }
                    (lo, hi, vals)
                }) {
                    let mut args = vec![json!(format!("{name}.vtx[{lo}:{hi}]"))];
                    args.extend(vals);
                    self.apply_preview(core, "setAttr", json!({"args": args}));
                    return;
                }
                self.apply_preview(core, "xform", json!({"args": [format!("{name}.vtx[{v}]")], "worldSpace": true, "translation": to.to_array()}));
            }
        }
    }

    /// Shift+middle drag: one relax step on the picked points of the quad mesh, by the Relax setting.
    pub(crate) fn quad_relax_picked(&mut self, core: &mut Core, items: &[DrawItem]) {
        let Some(it) = self.quad_target(core, items) else { return };
        let (shape, world, mesh) = (it.shape, it.world, it.mesh.clone());
        let g = &core.s.graph;
        let mode = components::mode(g);
        if mode == aztek_scene::components::ComponentMode::Object {
            return;
        }
        let picked = components::selected(g, shape, mode).to_vec();
        let pts = components::convert(&mesh, mode, &picked, aztek_scene::components::ComponentMode::Vertex, false);
        let pts = relax_points(&mesh, &pts, self.ts.qd.relax);
        let name = components::display_path(&core.s.graph, shape);
        let screen = self.vp.screen();
        for (p, local) in relaxed(&mesh, &pts) {
            let mut w = world.transform_point3(local);
            if !core.s.live_surface.is_empty() {
                if let Some(on) = screen.project(w).and_then(|q| self.live_point(core, items, q)) {
                    w = on;
                }
            }
            let _ = core.exec_silent("xform", json!({"args": [format!("{name}.vtx[{p}]")], "worldSpace": true, "translation": w.to_array()}));
        }
    }

    /// The four corners of a strip quad on screen, round its middle along `dir`, `w` pixels a side.
    fn strip_corners(&self, at: DVec2, dir: DVec2, w: f64) -> [DVec2; 4] {
        let d = if dir.length_squared() < 1e-9 { DVec2::X } else { dir.normalize() };
        let side = DVec2::new(-d.y, d.x) * w * 0.5;
        let along = d * w * 0.5;
        [at - along - side, at - along + side, at + along + side, at + along - side]
    }

    /// Tab held over an empty part of the live surface: the strip's next quad, along the last way the mouse moved.
    pub(crate) fn quad_strip_preview(&mut self, core: &Core, items: &[DrawItem], px: DVec2) {
        let w = self.ts.qd.strip_px * self.dpr;
        let corners = self.strip_corners(px, self.ts.last_dir, w);
        let pts: Vec<DVec3> = corners.iter().filter_map(|&c| self.live_point(core, items, c)).collect();
        if pts.len() == 4 {
            for i in 0..4 {
                self.vp.tool_lines.push((pts[i], pts[(i + 1) % 4], GREEN));
            }
        }
    }

    /// Tab+drag on an empty part of the surface: a quad each strip width the mouse moves, each on the last.
    pub(crate) fn quad_strip_drag(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) {
        let w = self.ts.qd.strip_px * self.dpr;
        let Some(mut s) = self.ts.strip.clone() else { return };
        let mut guard = 0;
        while (px - s.last_px).length() >= w && guard < 64 {
            guard += 1;
            let dir = (px - s.last_px).normalize();
            let back = s.last_px;
            let front = back + dir * w;
            let side = DVec2::new(-dir.y, dir.x) * w * 0.5;
            let (bl, br, fl, fr) = (back - side, back + side, front - side, front + side);
            let world = |v: &mut View, c: DVec2| v.live_point(core, items, c);
            let (Some(wfl), Some(wfr)) = (world(self, fl), world(self, fr)) else { break };
            let target = self.quad_target(core, items).map(|i| i.shape);
            let mut call_items = Vec::new();
            match s.edge {
                Some((a, b)) => {
                    call_items.push(json!({"vertex": a}));
                    call_items.push(json!({"vertex": b}));
                }
                None => {
                    let (Some(wbl), Some(wbr)) = (world(self, bl), world(self, br)) else { break };
                    call_items.push(json!({"position": wbl.to_array()}));
                    call_items.push(json!({"position": wbr.to_array()}));
                }
            }
            call_items.push(json!({"position": wfr.to_array()}));
            call_items.push(json!({"position": wfl.to_array()}));
            let mut call = json!({"items": call_items});
            if let Some(t) = target {
                call["mesh"] = json!(core.s.graph.path(t));
            } else {
                let _ = core.exec_silent("select", json!({"clear": true}));
            }
            let Ok(v) = core.exec("polyAppendVertex", call) else { break };
            if self.ts.quad_mesh.is_none() {
                if let Some(n) = v.get(0).and_then(Value::as_str) {
                    if let Ok(Some(id)) = core.s.graph.find(n) {
                        self.ts.quad_mesh = components::as_mesh(&core.s.graph, id);
                    }
                }
            }
            let Some(shape) = self.ts.quad_mesh.or(target) else { break };
            // The quad's front edge is the next one's back.
            s.edge = match (vertex_at(core, shape, wfl), vertex_at(core, shape, wfr)) {
                (Some(l), Some(r)) => Some((l, r)),
                _ => None,
            };
            s.last_px = front;
            s.dir = dir;
            s.started = true;
        }
        self.ts.strip = Some(s);
    }

    /// Tab+drag on a border edge: the edges the Extend setting picks, extruded and moved with the mouse, live.
    pub(crate) fn quad_extend_drag(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) {
        let Some(x) = self.ts.extend.clone() else { return };
        let Some(p) = self.live_point(core, items, px) else { return };
        let name = components::display_path(&core.s.graph, x.mesh);
        let list: Vec<String> = x.edges.iter().map(|e| format!("{name}.e[{e}]")).collect();
        let d = p - x.from;
        self.apply_preview(core, "polyExtrudeEdge", json!({"args": list, "worldSpace": true, "translate": d.to_array(), "keepFacesTogether": true}));
    }

    /// Shift+drag inside the quad mesh: the points near the brush relax toward their neighbours, kept on the surface.
    pub(crate) fn quad_relax(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) {
        let Some(it) = self.quad_target(core, items) else { return };
        let (shape, world, mesh) = (it.shape, it.world, it.mesh.clone());
        let screen = self.vp.screen();
        let reach = 60.0 * self.dpr;
        let near: Vec<Index> = (0..mesh.point_capacity()).filter(|&p| mesh.point_valid(p) && screen.project(world.transform_point3(mesh.position(p))).is_some_and(|q| (q - px).length() < reach)).collect();
        let pts = relax_points(&mesh, &near, self.ts.qd.relax);
        if pts.is_empty() {
            return;
        }
        let moved = relaxed(&mesh, &pts);
        let name = components::display_path(&core.s.graph, shape);
        let _ = core.exec_silent("undoInfo", json!({"openChunk": true}));
        for (p, local) in moved {
            let mut w = world.transform_point3(local);
            if let Some(q) = screen.project(w) {
                if let Some(on) = self.live_point(core, items, q) {
                    if !core.s.live_surface.is_empty() {
                        w = on;
                    }
                }
            }
            let _ = core.exec_silent("xform", json!({"args": [format!("{name}.vtx[{p}]")], "worldSpace": true, "translation": w.to_array()}));
        }
        let _ = core.exec_silent("undoInfo", json!({"closeChunk": true}));
    }

    /// A dragged vertex let go within the auto-weld distance of another of its mesh's points welds to it.
    pub(crate) fn quad_auto_weld(&mut self, core: &mut Core, mesh: NodeId, v: Index) {
        if !self.ts.qd.auto_weld {
            return;
        }
        core.s.graph.evaluate();
        let Some(m) = mesh_of(&core.s.graph, mesh) else { return };
        let world = { let xf = aztek_scene::dag::transform_of(&core.s.graph, mesh); aztek_scene::rig::world(&mut core.s.graph, xf) };
        let screen = self.vp.screen();
        let Some(at) = m.point_valid(v).then(|| world.transform_point3(m.position(v))).and_then(|w| screen.project(w)) else { return };
        let reach = self.ts.qd.weld_px * self.dpr;
        let other = (0..m.point_capacity())
            .filter(|&p| p != v && m.point_valid(p))
            .filter_map(|p| screen.project(world.transform_point3(m.position(p))).map(|q| (p, (q - at).length())))
            .filter(|x| x.1 < reach)
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((p, _)) = other {
            // Onto the other point, then welded there.
            let name = components::display_path(&core.s.graph, mesh);
            let target = world.transform_point3(m.position(p));
            let _ = core.exec("xform", json!({"args": [format!("{name}.vtx[{v}]")], "worldSpace": true, "translation": target.to_array()}));
            let _ = core.exec("polyMergeVertex", json!({"args": [format!("{name}.vtx[{v}]"), format!("{name}.vtx[{p}]")], "distance": 1e-4}));
        }
    }

    /// Ctrl+Shift+click on an edge in Quad Draw: its whole loop goes.
    pub(crate) fn quad_delete_loop(&mut self, core: &mut Core, shape: NodeId, m: &Mesh, e: Index) {
        let lp = aztek_geo::topology::edge_loop(m, e);
        let list: Vec<Index> = if lp.is_empty() { vec![e] } else { lp };
        let name = components::display_path(&core.s.graph, shape);
        let args: Vec<String> = list.iter().map(|e| format!("{name}.e[{e}]")).collect();
        let _ = core.exec("polyDelEdge", json!({"args": args, "cleanVertices": true}));
    }

    /// An edge loop through an edge: by Multi-Cut's settings, with edge flow and more than one loop through
    /// polySplitRing, otherwise the knife's, which keeps to symmetry.
    pub(crate) fn insert_loop_at(&mut self, core: &mut Core, shape: NodeId, e: Index, t: f64) {
        let name = core.s.graph.path(shape);
        let c = self.ts.mc.clone();
        let live = !core.s.live_surface.is_empty();
        if (c.edge_flow && !live) || c.subdivisions > 1 {
            let edge = format!("{}.e[{e}]", components::display_path(&core.s.graph, shape));
            let mut call = json!({"args": [edge], "weight": t, "smoothingAngle": c.smoothing_angle.to_radians()});
            if c.subdivisions > 1 {
                call["splitType"] = json!(2);
                call["divisions"] = json!(c.subdivisions);
            }
            if c.edge_flow && !live {
                call["insertWithEdgeFlow"] = json!(true);
                call["adjustEdgeFlow"] = json!(c.edge_flow_amount);
            }
            let from = point_count(core, shape);
            let _ = core.exec_silent("undoInfo", json!({"openChunk": true}));
            let _ = core.exec("polySplitRing", call);
            snap_new_points_to_live(core, shape, from);
            let _ = core.exec_silent("undoInfo", json!({"closeChunk": true}));
            let _ = core.exec_silent("select", json!({"clear": true}));
            return;
        }
        let from = point_count(core, shape);
        let _ = core.exec_silent("undoInfo", json!({"openChunk": true}));
        let _ = core.exec("polyKnife", json!({"mesh": name, "loopEdge": e, "fraction": t, "select": false}));
        snap_new_points_to_live(core, shape, from);
        let _ = core.exec_silent("undoInfo", json!({"closeChunk": true}));
    }

    /// Multi-Cut's middle drag across the surface: the line drawn, Shift turning it in 45 degree steps.
    pub(crate) fn slice_preview(&mut self, items: &[DrawItem], from: DVec2, to: DVec2) {
        let _ = items;
        let a = self.vp.place_point(from);
        let b = self.vp.place_point(to);
        self.vp.tool_lines.push((a, b, ORANGE));
    }

    /// Slices the meshes under a screen line through the eye, or the given meshes along a world plane, as the Slice Tool
    /// settings say: faces deleted or pulled off on the plane's far side, back faces left alone if asked.
    pub(crate) fn slice(&mut self, core: &mut Core, shapes: &[(NodeId, DVec3, DVec3)]) -> Option<String> {
        let mut last = None;
        let c = self.ts.mc.clone();
        let eye = self.vp.cam.eye();
        let _ = core.exec_silent("undoInfo", json!({"openChunk": true}));
        for &(shape, at, n) in shapes {
            core.s.graph.evaluate();
            let Some(m) = mesh_of(&core.s.graph, shape) else { continue };
            let world = { let xf = aztek_scene::dag::transform_of(&core.s.graph, shape); aztek_scene::rig::world(&mut core.s.graph, xf) };
            let name = components::display_path(&core.s.graph, shape);
            let faces: Vec<String> = (0..m.face_capacity())
                .filter(|&fc| m.face_valid(fc))
                .filter(|&fc| {
                    if !c.ignore_backfaces {
                        return true;
                    }
                    let mid = world.transform_point3(m.face_centroid(fc));
                    let nn = world.transform_vector3(m.face_normal(fc));
                    nn.dot(eye - mid) > 0.0
                })
                .map(|fc| format!("{name}.f[{fc}]"))
                .collect();
            if faces.is_empty() {
                continue;
            }
            let mut call = json!({"args": faces, "cutPlaneCenter": at.to_array(), "cutPlaneRotate": cut_rotation(n), "worldSpace": true});
            if c.delete_faces {
                call["deleteFaces"] = json!(true);
            } else if c.extract_faces {
                call["extractFaces"] = json!(true);
                call["extractOffset"] = json!(c.extract_offset);
            }
            if let Ok(v) = core.exec("polyCut", call) {
                last = v.as_str().map(str::to_string).or_else(|| v.get(0).and_then(Value::as_str).map(str::to_string)).or(last);
            }
        }
        let _ = core.exec_silent("undoInfo", json!({"closeChunk": true}));
        // The sliced objects stay selected, to see the cut and go on.
        let names: Vec<String> = shapes.iter().map(|s| core.s.graph.path(aztek_scene::dag::transform_of(&core.s.graph, s.0))).collect();
        if !names.is_empty() {
            let _ = core.exec_silent("select", json!({"args": names}));
        }
        last
    }

    /// The meshes a screen line crosses, by samples along it.
    pub(crate) fn meshes_under_line(&mut self, items: &[DrawItem], a: DVec2, b: DVec2) -> Vec<NodeId> {
        let mut out = Vec::new();
        for k in 0..=48 {
            let p = a.lerp(b, k as f64 / 48.0);
            if let Some((i, _, _)) = self.vp.raycast(items, p) {
                if !out.contains(&items[i].shape) {
                    out.push(items[i].shape);
                }
            }
        }
        out
    }

    /// Slice Along Plane: each Multi-Cut target cut through its middle across X (YZ), Y (ZX) or Z (XY), in world space.
    pub(crate) fn slice_along(&mut self, core: &mut Core, plane: &str) {
        let n = match plane {
            "yz" => DVec3::X,
            "zx" => DVec3::Y,
            _ => DVec3::Z,
        };
        let targets = crate::tools::multi_cut_targets(core).to_vec();
        let g = &mut core.s.graph;
        g.evaluate();
        let mut shapes = Vec::new();
        for shape in targets {
            if core.s.live_surface.contains(&shape) {
                continue;
            }
            let Some(m) = mesh_of(g, shape) else { continue };
            let xf = aztek_scene::dag::transform_of(g, shape);
            let world = aztek_scene::rig::world(g, xf);
            let (mut lo, mut hi) = (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN));
            for p in (0..m.point_capacity()).filter(|&p| m.point_valid(p)) {
                let w = world.transform_point3(m.position(p));
                lo = lo.min(w);
                hi = hi.max(w);
            }
            if lo.x <= hi.x && !shapes.iter().any(|s: &(NodeId, DVec3, DVec3)| s.0 == shape) {
                shapes.push((shape, (lo + hi) * 0.5, n));
            }
        }
        let keep: Vec<String> = core.s.graph.selection().iter().map(|&id| core.s.graph.path(id)).collect();
        let cut = self.slice(core, &shapes);
        if !keep.is_empty() {
            let _ = core.exec_silent("select", json!({"args": keep}));
        }
        // The plane's handles straight away, as the reference's gizmo: its centre moves and it turns, the cut following.
        if let Some(node) = cut {
            crate::node_manip::show_node(core, &node);
            crate::core::push_ui_request("setTool|showManip".to_string());
        }
    }
}

/// A Quad Draw drag of a vertex, an edge or a face (#164), soft selection carrying the points around it (#166): every
/// point's place when the drag began, the points that move and by how much, and where on the surface it was grabbed.
#[derive(Clone)]
pub struct CompDrag {
    pub shape: NodeId,
    pub world: glam::DMat4,
    pub start: Vec<DVec3>,
    pub moving: Vec<(Index, f32)>,
    pub grab: DVec3,
    /// The vertex dragged alone, which snaps onto another within the auto weld distance.
    pub single: Option<Index>,
    pub surface: Option<std::sync::Arc<(aztek_geo::bvh::MeshBvh, glam::DMat4)>>,
}

/// Tab+drag on a border vertex: a quad out of the corner, from the vertex's two border neighbours to the new point.
#[derive(Clone)]
pub struct VertexExtrude {
    pub shape: NodeId,
    pub ring: [Index; 3],
}

/// The live surface in world space, for putting points onto it.
fn live_bvh(core: &mut Core) -> Option<std::sync::Arc<(aztek_geo::bvh::MeshBvh, glam::DMat4)>> {
    let &live = core.s.live_surface.first()?;
    let g = &mut core.s.graph;
    g.evaluate();
    let m = mesh_of(g, live)?;
    let w = { let xf = aztek_scene::dag::transform_of(g, live); aztek_scene::rig::world(g, xf) };
    let mut ws = (*m).clone();
    for p in 0..ws.point_capacity() {
        if ws.point_valid(p) {
            let at = w.transform_point3(ws.position(p));
            ws.set_position(p, at);
        }
    }
    Some(std::sync::Arc::new((aztek_geo::bvh::MeshBvh::new(&ws), w)))
}

/// What's under the pointer in Quad Draw: a vertex within reach, else an edge, else the face.
enum Under {
    Vertex(Index),
    Edge(Index),
    Face(Index),
}

impl View {
    fn quad_under(&mut self, it: &DrawItem, px: DVec2) -> Option<Under> {
        let list = [DrawItem { shape: it.shape, xf: it.xf, world: it.world, mesh: it.mesh.clone(), smooth: 0 }];
        let screen = self.vp.screen();
        let reach = 12.0 * self.dpr;
        let m = &it.mesh;
        if let Some(p) = (0..m.point_capacity()).filter(|&p| m.point_valid(p)).min_by(|&a, &b| {
            let d = |p: Index| screen.project(it.world.transform_point3(m.position(p))).map(|q| (q - px).length()).unwrap_or(f64::INFINITY);
            d(a).total_cmp(&d(b))
        }) {
            if screen.project(it.world.transform_point3(m.position(p))).is_some_and(|q| (q - px).length() < reach) {
                return Some(Under::Vertex(p));
            }
        }
        let s = crate::tools::snap_at(self, &list, px)?;
        if s.edge.3 < 6.0 * self.dpr {
            return Some(Under::Edge(s.edge.0));
        }
        Some(Under::Face(s.face))
    }

    /// Hovering in Quad Draw with no keys held: what a press would take lights up, as the reference shows it (#164).
    pub(crate) fn quad_highlight(&mut self, core: &Core, items: &[DrawItem], px: DVec2) {
        let Some(it) = self.quad_target(core, items).map(|i| DrawItem { shape: i.shape, xf: i.xf, world: i.world, mesh: i.mesh.clone(), smooth: 0 }) else { return };
        const HI: [u8; 4] = [255, 60, 60, 255];
        let (m, w) = (&it.mesh, it.world);
        let line = |v: &mut View, e: Index| {
            let (a, b) = (m.position(m.edge_point(e, 0)), m.position(m.edge_point(e, 1)));
            v.vp.tool_lines.push((w.transform_point3(a), w.transform_point3(b), HI));
        };
        match self.quad_under(&it, px) {
            Some(Under::Vertex(p)) => self.vp.tool_points.push((w.transform_point3(m.position(p)), HI)),
            Some(Under::Edge(e)) => line(self, e),
            Some(Under::Face(f)) => {
                let mut edges = Vec::new();
                m.for_each_face_corner(f, |c| edges.push(m.corner_edge(c)));
                for e in edges {
                    line(self, e);
                }
            }
            None => {}
        }
    }

    /// A left press, or a middle one with a live surface, on the quad mesh: its vertex, edge or face starts moving
    /// over the surface.
    pub(crate) fn quad_comp_press(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) -> bool {
        let Some(it) = self.quad_target(core, items).map(|i| DrawItem { shape: i.shape, xf: i.xf, world: i.world, mesh: i.mesh.clone(), smooth: 0 }) else { return false };
        let m = it.mesh.clone();
        let picked: Vec<Index> = match self.quad_under(&it, px) {
            Some(Under::Vertex(p)) => vec![p],
            Some(Under::Edge(e)) => vec![m.edge_point(e, 0), m.edge_point(e, 1)],
            Some(Under::Face(f)) => {
                // Only a press on the face itself, not the empty surface beside the mesh.
                let list = [DrawItem { shape: it.shape, xf: it.xf, world: it.world, mesh: m.clone(), smooth: 0 }];
                if self.vp.raycast(&list, px).is_none() {
                    return false;
                }
                let mut ps = Vec::new();
                m.for_each_face_corner(f, |c| ps.push(m.corner_point(c)));
                ps
            }
            None => return false,
        };
        let Some(grab) = (if picked.len() == 1 { Some(it.world.transform_point3(m.position(picked[0]))) } else { self.live_point(core, items, px) }) else { return false };
        // Soft selection, when on, carries the points round what's dragged by its falloff (#166).
        let weights = aztek_scene::mesh_edit::transform::soft_weights(&core.s.graph, &m, &it.world, &picked);
        let moving: Vec<(Index, f32)> = (0..m.point_capacity()).filter(|&p| m.point_valid(p) && weights[p as usize] > 0.0).map(|p| (p, weights[p as usize])).collect();
        let start = (0..m.point_capacity()).map(|p| if m.point_valid(p) { m.position(p) } else { DVec3::ZERO }).collect();
        self.ts.comp_drag = Some(CompDrag {
            shape: it.shape,
            world: it.world,
            start,
            moving,
            grab,
            single: (picked.len() == 1).then(|| picked[0]),
            surface: live_bvh(core),
        });
        self.ts.applied = false;
        true
    }

    pub(crate) fn quad_comp_drag(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) {
        let Some(d) = self.ts.comp_drag.clone() else { return };
        let Some(now) = self.live_point(core, items, px) else { return };
        let mut delta = now - d.grab;
        let screen = self.vp.screen();
        // One vertex near another within the auto weld distance: onto it, to be welded on release.
        if let (Some(v), true) = (d.single, self.ts.qd.auto_weld) {
            let reach = self.ts.qd.weld_px * self.dpr;
            let near = (0..d.start.len() as Index)
                .filter(|&p| p != v && d.moving.iter().all(|x| x.0 != p))
                .filter_map(|p| screen.project(d.world.transform_point3(d.start[p as usize])).map(|q| (p, (q - px).length())))
                .filter(|x| x.1 < reach)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((p, _)) = near {
                delta = d.world.transform_point3(d.start[p as usize]) - d.grab;
            }
        }
        let inv = d.world.inverse();
        let mut lo = Index::MAX;
        let mut hi = 0;
        let mut to: std::collections::HashMap<Index, DVec3> = std::collections::HashMap::new();
        for &(p, w) in &d.moving {
            let mut at = d.world.transform_point3(d.start[p as usize]) + delta * w as f64;
            if let Some(s) = &d.surface {
                if let Some(c) = s.0.closest_point(at) {
                    at = c.point;
                }
            }
            to.insert(p, inv.transform_point3(at));
            lo = lo.min(p);
            hi = hi.max(p);
        }
        if to.is_empty() {
            return;
        }
        // One setAttr over the run of points, those not moving keeping their place.
        let mut vals = Vec::with_capacity(((hi - lo + 1) * 3) as usize);
        for p in lo..=hi {
            let at = to.get(&p).copied().unwrap_or(d.start[p as usize]);
            vals.extend([json!(at.x), json!(at.y), json!(at.z)]);
        }
        let name = components::display_path(&core.s.graph, d.shape);
        let mut args = vec![json!(format!("{name}.vtx[{lo}:{hi}]"))];
        args.extend(vals);
        self.apply_preview(core, "setAttr", json!({"args": args}));
    }

    /// The drag's end: one undo step, and a lone vertex left on another welded to it.
    pub(crate) fn quad_comp_release(&mut self, core: &mut Core) {
        let Some(d) = self.ts.comp_drag.take() else { return };
        self.commit_preview(core);
        if let Some(v) = d.single {
            self.quad_auto_weld(core, d.shape, v);
        }
    }

    /// Tab+left press on a border vertex: its extrusion starts (#164).
    pub(crate) fn quad_vertex_extrude_press(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) -> bool {
        let Some(it) = self.quad_target(core, items).map(|i| DrawItem { shape: i.shape, xf: i.xf, world: i.world, mesh: i.mesh.clone(), smooth: 0 }) else { return false };
        let Some(Under::Vertex(v)) = self.quad_under(&it, px) else { return false };
        let m = &it.mesh;
        let mut border = Vec::new();
        m.for_each_point_edge(v, |e| {
            if m.is_boundary_edge(e) {
                let o = if m.edge_point(e, 0) == v { m.edge_point(e, 1) } else { m.edge_point(e, 0) };
                border.push(o);
            }
        });
        if border.len() != 2 {
            return false;
        }
        // The new face runs its border edges the other way from the faces already on them: when a face goes from v
        // to the first neighbour, the new one goes from that neighbour to v.
        let mut v_to_first = false;
        m.for_each_point_corner(v, |c| {
            if m.corner_point(m.corner_next(c)) == border[0] {
                v_to_first = true;
            }
        });
        let ring = if v_to_first { [border[0], v, border[1]] } else { [border[1], v, border[0]] };
        self.ts.vertex_extrude = Some(VertexExtrude { shape: it.shape, ring });
        self.ts.applied = false;
        true
    }

    pub(crate) fn quad_vertex_extrude_drag(&mut self, core: &mut Core, items: &[DrawItem], px: DVec2) {
        let Some(x) = self.ts.vertex_extrude.clone() else { return };
        let Some(at) = self.live_point(core, items, px) else { return };
        let name = core.s.graph.path(x.shape);
        let [a, v, b] = x.ring;
        let items_json = json!([{"vertex": a}, {"vertex": v}, {"vertex": b}, {"position": at.to_array()}]);
        self.apply_preview(core, "polyAppendVertex", json!({"mesh": name, "items": items_json}));
    }
}
