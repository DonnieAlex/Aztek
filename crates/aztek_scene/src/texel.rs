//! Texel density, as the reference's texGetTexelDensity and texSetTexelDensity: pixels of a map of a given size per
//! scene unit on the surface, from the faces' UV area against their world-space area.

use aztek_geo::{Index, Mesh};
use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use glam::{DMat4, DVec2, DVec3};
use serde_json::{json, Value};

use crate::session::Session;
use crate::uv_ops::{self, topo};

const UV: &str = "map1";

fn uv_at(m: &Mesh, c: Index) -> DVec2 {
    let uv = m.corner_attrs().get::<[f32; 2]>(aztek_geo::names::uv(0).as_str());
    uv.get(c as usize).map_or(DVec2::ZERO, |u| DVec2::new(u[0] as f64, u[1] as f64))
}

/// (UV area, world area) of faces.
pub fn areas(m: &Mesh, world: &DMat4, faces: &[Index]) -> (f64, f64) {
    let (mut a_uv, mut a_w) = (0.0, 0.0);
    for &f in faces {
        if !m.face_valid(f) {
            continue;
        }
        let c = m.face_corners(f);
        let p: Vec<DVec3> = c.iter().map(|&k| world.transform_point3(m.position(m.corner_point(k)))).collect();
        let u: Vec<DVec2> = c.iter().map(|&k| uv_at(m, k)).collect();
        for k in 1..c.len().saturating_sub(1) {
            a_w += (p[k] - p[0]).cross(p[k + 1] - p[0]).length() * 0.5;
            a_uv += ((u[k] - u[0]).perp_dot(u[k + 1] - u[0]) * 0.5).abs();
        }
    }
    (a_uv, a_w)
}

/// Pixels per scene unit for a map of map_size pixels.
pub fn density(a_uv: f64, a_w: f64, map_size: f64) -> f64 {
    if a_w <= 0.0 {
        0.0
    } else {
        (a_uv / a_w).sqrt() * map_size
    }
}

fn world_of(s: &mut Session, mesh: crate::plug::NodeId) -> DMat4 {
    s.graph.evaluate();
    s.graph.node(mesh).map(|n| n.data().matrix(crate::transform::DagNode::WORLD_MATRIX)).unwrap_or(DMat4::IDENTITY)
}

fn uv_set_ok(a: &aztek_ops::Args, cmd: &str) -> CmdResult<()> {
    let set = a.string("uvSetName", UV);
    if set != UV && !set.is_empty() {
        fail!("{cmd}: only the first UV set ({UV}) is read; {set} is not");
    }
    Ok(())
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "texGetTexelDensity",
            "The texel density of the faces named or selected (objects count all their faces): pixels per scene unit for a \
             map of -mapSize pixels (1024 by default), from their UV area against their world-space area.",
            |s, a| {
                uv_set_ok(&a, "texGetTexelDensity")?;
                let map = a.float("mapSize", 1024.0);
                let targets = uv_ops::targets(s, &a, "texGetTexelDensity")?;
                let (mut au, mut aw) = (0.0, 0.0);
                for (mesh, t) in targets {
                    let world = world_of(s, mesh);
                    let Some(m) = crate::modifiers::cage(&mut s.graph, mesh) else { continue };
                    let tp = topo(&m);
                    let faces = uv_ops::target_faces(&m, &t, &tp);
                    let (u, w) = areas(&m, &world, &faces);
                    au += u;
                    aw += w;
                }
                Ok(json!(density(au, aw, map)))
            },
        )
        .positional("components or objects")
        .flag(FlagDef::new("mapSize", "ms", ArgType::Int, Value::Null, "The map's size in pixels; 1024 by default.").min(1.0))
        .flag(FlagDef::new("uvSetName", "uvs", ArgType::String, Value::Null, "The UV set; the first."))
        .undoable(false),
    );
    c.register(
        CommandDef::<Session>::new(
            "texSetTexelDensity",
            "Scales each UV shell of the faces named or selected about its centre so that it has -texelDensity pixels per \
             scene unit for a map of -mapSize pixels. One undo step.",
            |s, a| {
                uv_set_ok(&a, "texSetTexelDensity")?;
                let Some(want) = a.get("texelDensity").and_then(Value::as_f64) else { fail!("texSetTexelDensity: give -texelDensity") };
                if want <= 0.0 {
                    fail!("texSetTexelDensity: -texelDensity must be above 0");
                }
                let map = a.float("mapSize", 1024.0);
                let targets = uv_ops::targets(s, &a, "texSetTexelDensity")?;
                let worlds: Vec<(crate::plug::NodeId, DMat4)> = targets.keys().map(|&m| (m, world_of(s, m))).collect::<Vec<_>>();
                let g = &mut s.graph;
                g.open_chunk("Set Texel Density");
                let r = (|| -> CmdResult<()> {
                    for (mesh, t) in targets {
                        let world = worlds.iter().find(|x| x.0 == mesh).map(|x| x.1).unwrap_or(DMat4::IDENTITY);
                        uv_ops::edit_mesh(g, mesh, "polyTweakUV", |m| {
                            let tp = topo(m);
                            let faces = uv_ops::target_faces(m, &t, &tp);
                            for shell in uv_ops::shells_of(&tp, &faces) {
                                let (au, aw) = areas(m, &world, &shell);
                                let now = density(au, aw, map);
                                if now <= 0.0 {
                                    continue;
                                }
                                let k = want / now;
                                let corners: Vec<Index> = shell.iter().flat_map(|&f| m.face_corners(f).to_vec()).collect();
                                let (mut lo, mut hi) = (DVec2::splat(f64::INFINITY), DVec2::splat(f64::NEG_INFINITY));
                                for &c in &corners {
                                    let u = uv_at(m, c);
                                    lo = lo.min(u);
                                    hi = hi.max(u);
                                }
                                uv_ops::transform(m, &corners, DVec2::ZERO, DVec2::splat(k), 0.0, (lo + hi) * 0.5);
                            }
                        })?;
                    }
                    Ok(())
                })();
                g.close_chunk();
                r?;
                Ok(Value::Null)
            },
        )
        .positional("components or objects")
        .flag(FlagDef::new("texelDensity", "td", ArgType::Float, Value::Null, "Pixels per scene unit wanted."))
        .flag(FlagDef::new("mapSize", "ms", ArgType::Int, Value::Null, "The map's size in pixels; 1024 by default.").min(1.0))
        .flag(FlagDef::new("uvSetName", "uvs", ArgType::String, Value::Null, "The UV set; the first.")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn density_is_root_of_area_ratio() {
        assert!((density(0.25, 1.0, 1024.0) - 512.0).abs() < 1e-9);
        assert_eq!(density(1.0, 0.0, 1024.0), 0.0);
    }
}
