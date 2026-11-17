//! Arranging UV shells, the UV Toolkit's Arrange and Layout, and Align and Snap tools (#108): texOrientShells,
//! texOrientEdge, texStackShells, texUnstackShells, texGatherShells, texDistributeShells, texAlignShells,
//! texSnapShells, texRandomizeShells, texLinearAlignUVs, texStraightenUVs, texMatchUVs, texMatchGrid and
//! texSnapTogether; and texSelectByType, the Select By Type picks (back and front facing, texture borders). The command names are the reference's; their flags are
//! Aztek's own, as the reference's flag lists for these were not to hand.
//!
//! Shells are those of the UVs, edges or faces named (or selected), across every mesh named; one polyTweakUV node per
//! mesh, as the other UV commands make.

use std::collections::BTreeSet;

use aztek_geo::{Index, Mesh};
use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands};
use glam::DVec2;

use crate::ref_core::f;
use crate::session::Session;
use crate::uv_misc::{bounds_of, corners_of_faces, poly_flag, shells_of, uv_at};
use crate::uv_ops::{self, each, Target, UvTopo};

type Cmd = CommandDef<Session>;
use ArgType::{Float as Fl, Int as I, String as S};

/// One shell: its corners and UV bounds.
#[derive(Clone)]
struct Shell {
    corners: Vec<Index>,
    lo: DVec2,
    hi: DVec2,
}

impl Shell {
    fn centre(&self) -> DVec2 {
        (self.lo + self.hi) * 0.5
    }
    fn size(&self) -> DVec2 {
        self.hi - self.lo
    }
}

/// A shell's move: offset, then turned by degrees about pivot and scaled by scale about it.
#[derive(Clone, Copy)]
struct Move {
    offset: DVec2,
    scale: DVec2,
    degrees: f64,
    pivot: DVec2,
}

impl Move {
    fn by(offset: DVec2) -> Self {
        Move { offset, scale: DVec2::ONE, degrees: 0.0, pivot: DVec2::ZERO }
    }
}

fn shells_in(m: &Mesh, t: &Target, tp: &UvTopo) -> Vec<Shell> {
    shells_of(m, t, tp)
        .into_iter()
        .map(|faces| {
            let corners = corners_of_faces(m, &faces);
            let (lo, hi) = bounds_of(m, &corners);
            Shell { corners, lo, hi }
        })
        .collect()
}

/// Every named shell, per mesh in the order `each` visits them.
fn all_shells(s: &mut Session, a: &Args, cmd: &str) -> CmdResult<Vec<Vec<Shell>>> {
    let targets = uv_ops::targets(s, a, cmd)?;
    let g = &mut s.graph;
    let mut out = Vec::new();
    for (mesh, t) in targets {
        let Some(m) = crate::modifiers::cage(g, mesh) else {
            out.push(Vec::new());
            continue;
        };
        let tp = uv_ops::topo(&m);
        out.push(shells_in(&m, &t, &tp));
    }
    if out.iter().all(Vec::is_empty) {
        fail!("{cmd}: no UV shells named or selected");
    }
    Ok(out)
}

/// Applies the moves worked out for each shell, mesh by mesh.
fn apply(s: &mut Session, a: &Args, cmd: &str, label: &str, moves: Vec<Vec<Move>>) -> CmdResult {
    let mut at = 0;
    each(s, a, cmd, label, "polyTweakUV", |m, t, tp| {
        let mine = moves.get(at).cloned().unwrap_or_default();
        at += 1;
        for (shell, mv) in shells_in(m, t, tp).iter().zip(mine) {
            if mv.degrees != 0.0 || mv.scale != DVec2::ONE {
                uv_ops::transform(m, &shell.corners, DVec2::ZERO, mv.scale, mv.degrees, mv.pivot);
            }
            if mv.offset != DVec2::ZERO {
                uv_ops::transform(m, &shell.corners, mv.offset, DVec2::ONE, 0.0, DVec2::ZERO);
            }
        }
    })
}

/// Moves for every shell from one rule.
fn per_shell(shells: &[Vec<Shell>], rule: impl Fn(&Shell) -> Move) -> Vec<Vec<Move>> {
    shells.iter().map(|l| l.iter().map(&rule).collect()).collect()
}

fn hull(mut pts: Vec<DVec2>) -> Vec<DVec2> {
    pts.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    pts.dedup_by(|a, b| (*a - *b).length_squared() < 1e-24);
    if pts.len() < 3 {
        return pts;
    }
    let cross = |o: DVec2, a: DVec2, b: DVec2| (a - o).perp_dot(b - o);
    let mut lower: Vec<DVec2> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<DVec2> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// The turn, in degrees between -45 and 45, that sets a shell square to U and V: the one that makes its bounding box
/// smallest, its hull's edges being the angles to try.
fn orient_angle(m: &Mesh, sh: &Shell) -> f64 {
    let pts: Vec<DVec2> = sh.corners.iter().map(|&c| uv_at(m, c)).collect();
    let h = hull(pts);
    if h.len() < 2 {
        return 0.0;
    }
    let mut best = (f64::MAX, 0.0);
    for i in 0..h.len() {
        let e = h[(i + 1) % h.len()] - h[i];
        if e.length_squared() < 1e-20 {
            continue;
        }
        let ang = e.y.atan2(e.x);
        let (c, s) = (ang.cos(), ang.sin());
        let (mut lo, mut hi) = (DVec2::splat(f64::MAX), DVec2::splat(f64::MIN));
        for p in &h {
            let q = DVec2::new(p.x * c + p.y * s, -p.x * s + p.y * c);
            lo = lo.min(q);
            hi = hi.max(q);
        }
        let area = (hi - lo).x * (hi - lo).y;
        if area < best.0 - 1e-12 {
            best = (area, ang);
        }
    }
    nearest_square(-best.1.to_degrees())
}

/// An angle brought to the smallest turn that ends on a multiple of 90.
fn nearest_square(deg: f64) -> f64 {
    let r = deg.rem_euclid(90.0);
    if r > 45.0 {
        r - 90.0
    } else {
        r
    }
}

fn orient_shells(s: &mut Session, a: &Args) -> CmdResult {
    let _ = all_shells(s, a, "texOrientShells")?;
    each(s, a, "texOrientShells", "Orient Shells", "polyTweakUV", |m, t, tp| {
        for sh in shells_in(m, t, tp) {
            let deg = orient_angle(m, &sh);
            if deg.abs() > 1e-9 {
                uv_ops::transform(m, &sh.corners, DVec2::ZERO, DVec2::ONE, deg, sh.centre());
            }
        }
    })
}

fn orient_edge(s: &mut Session, a: &Args) -> CmdResult {
    let _ = all_shells(s, a, "texOrientEdge")?;
    each(s, a, "texOrientEdge", "Orient Shells to Edges", "polyTweakUV", |m, t, tp| {
        // Each shell turns so the first named edge in it lies along U or V.
        let mut done: BTreeSet<Index> = BTreeSet::new();
        for &e in &t.edges {
            if !m.edge_valid(e) {
                continue;
            }
            let mut corner = None;
            m.for_each_edge_corner(e, |c| {
                if corner.is_none() {
                    corner = Some(c);
                }
            });
            let Some(c) = corner else { continue };
            let shell = tp.face_shell.get(m.corner_face(c) as usize).copied().unwrap_or(aztek_geo::INVALID);
            if shell == aztek_geo::INVALID || !done.insert(shell) {
                continue;
            }
            let (p, q) = (uv_at(m, c), uv_at(m, m.corner_next(c)));
            let d = q - p;
            if d.length_squared() < 1e-20 {
                continue;
            }
            let deg = nearest_square(-d.y.atan2(d.x).to_degrees());
            let faces = tp.shells.get(shell as usize).cloned().unwrap_or_default();
            let corners = corners_of_faces(m, &faces);
            let (lo, hi) = bounds_of(m, &corners);
            uv_ops::transform(m, &corners, DVec2::ZERO, DVec2::ONE, deg, (lo + hi) * 0.5);
        }
    })
}

fn stack_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texStackShells")?;
    // Onto the middle of them all.
    let flat: Vec<&Shell> = shells.iter().flatten().collect();
    let (lo, hi) = flat.iter().fold((DVec2::splat(f64::MAX), DVec2::splat(f64::MIN)), |(lo, hi), sh| (lo.min(sh.lo), hi.max(sh.hi)));
    let at = (lo + hi) * 0.5;
    let moves = per_shell(&shells, |sh| Move::by(at - sh.centre()));
    apply(s, a, "texStackShells", "Stack Shells", moves)
}

fn overlaps(a: (DVec2, DVec2), b: (DVec2, DVec2), gap: f64) -> bool {
    a.0.x < b.1.x + gap && b.0.x < a.1.x + gap && a.0.y < b.1.y + gap && b.0.y < a.1.y + gap
}

fn unstack_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texUnstackShells")?;
    let gap = a.float("spacing", 0.01).max(0.0);
    // Largest first; each stays where it is unless it overlaps one placed already, then goes to the nearest free place
    // on a ring of steps around where it was.
    let mut order: Vec<(usize, usize)> = shells.iter().enumerate().flat_map(|(i, l)| (0..l.len()).map(move |j| (i, j))).collect();
    order.sort_by(|x, y| {
        let (a, b) = (&shells[x.0][x.1], &shells[y.0][y.1]);
        (b.size().x * b.size().y).total_cmp(&(a.size().x * a.size().y))
    });
    let mut placed: Vec<(DVec2, DVec2)> = Vec::new();
    let mut moves: Vec<Vec<Move>> = shells.iter().map(|l| vec![Move::by(DVec2::ZERO); l.len()]).collect();
    for (i, j) in order {
        let sh = &shells[i][j];
        let step = sh.size().max(DVec2::splat(1e-4)) + DVec2::splat(gap);
        let free = |off: DVec2, placed: &[(DVec2, DVec2)]| !placed.iter().any(|p| overlaps((sh.lo + off, sh.hi + off), *p, gap * 0.5));
        let mut chosen = DVec2::ZERO;
        if !free(chosen, &placed) {
            let mut best: Option<(f64, DVec2)> = None;
            'rings: for ring in 1..64i32 {
                for dx in -ring..=ring {
                    for dy in -ring..=ring {
                        if dx.abs() != ring && dy.abs() != ring {
                            continue;
                        }
                        let off = DVec2::new(dx as f64 * step.x, dy as f64 * step.y);
                        if free(off, &placed) {
                            let d = off.length();
                            if best.is_none_or(|b| d < b.0) {
                                best = Some((d, off));
                            }
                        }
                    }
                }
                if best.is_some() {
                    break 'rings;
                }
            }
            chosen = best.map(|b| b.1).unwrap_or(DVec2::ZERO);
        }
        placed.push((sh.lo + chosen, sh.hi + chosen));
        moves[i][j] = Move::by(chosen);
    }
    apply(s, a, "texUnstackShells", "Unstack Shells", moves)
}

fn gather_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texGatherShells")?;
    // Whole tiles, so the texture each shell shows stays the same: its middle lands in 0 to 1.
    let moves = per_shell(&shells, |sh| Move::by(-sh.centre().floor()));
    apply(s, a, "texGatherShells", "Gather Shells", moves)
}

fn axis_of(a: &Args, cmd: &str) -> CmdResult<usize> {
    match a.string("direction", "u").to_ascii_lowercase().as_str() {
        "u" | "x" | "horizontal" => Ok(0),
        "v" | "y" | "vertical" => Ok(1),
        d => fail!("{cmd}: -direction is u or v, not {d}"),
    }
}

fn distribute_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texDistributeShells")?;
    let axis = axis_of(a, "texDistributeShells")?;
    let spacing = a.float("spacing", 0.0);
    // In the order they lie, from the first one's edge, each the spacing on from the one before.
    let mut order: Vec<(usize, usize)> = shells.iter().enumerate().flat_map(|(i, l)| (0..l.len()).map(move |j| (i, j))).collect();
    order.sort_by(|x, y| shells[x.0][x.1].centre()[axis].total_cmp(&shells[y.0][y.1].centre()[axis]));
    let mut moves: Vec<Vec<Move>> = shells.iter().map(|l| vec![Move::by(DVec2::ZERO); l.len()]).collect();
    let mut edge = None;
    for (i, j) in order {
        let sh = &shells[i][j];
        let start = edge.unwrap_or(sh.lo[axis]);
        let mut off = DVec2::ZERO;
        off[axis] = start - sh.lo[axis];
        moves[i][j] = Move::by(off);
        edge = Some(start + sh.size()[axis] + spacing);
    }
    apply(s, a, "texDistributeShells", "Distribute Shells", moves)
}

fn align_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texAlignShells")?;
    let flat: Vec<&Shell> = shells.iter().flatten().collect();
    let (lo, hi) = flat.iter().fold((DVec2::splat(f64::MAX), DVec2::splat(f64::MIN)), |(lo, hi), sh| (lo.min(sh.lo), hi.max(sh.hi)));
    let how = a.string("alignTo", "left");
    let rule: Box<dyn Fn(&Shell) -> DVec2> = match how.as_str() {
        "left" => Box::new(move |sh: &Shell| DVec2::new(lo.x - sh.lo.x, 0.0)),
        "right" => Box::new(move |sh: &Shell| DVec2::new(hi.x - sh.hi.x, 0.0)),
        "bottom" => Box::new(move |sh: &Shell| DVec2::new(0.0, lo.y - sh.lo.y)),
        "top" => Box::new(move |sh: &Shell| DVec2::new(0.0, hi.y - sh.hi.y)),
        "centerU" => Box::new(move |sh: &Shell| DVec2::new((lo.x + hi.x) * 0.5 - sh.centre().x, 0.0)),
        "centerV" => Box::new(move |sh: &Shell| DVec2::new(0.0, (lo.y + hi.y) * 0.5 - sh.centre().y)),
        o => fail!("texAlignShells: -alignTo is left, right, top, bottom, centerU or centerV, not {o}"),
    };
    let moves = per_shell(&shells, |sh| Move::by(rule(sh)));
    apply(s, a, "texAlignShells", "Align Shells", moves)
}

fn snap_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texSnapShells")?;
    // Nine places on the tile, 0 top left to 8 bottom right, as the toolkit's grid of buttons.
    let at = a.int("position", 4);
    if !(0..=8).contains(&at) {
        fail!("texSnapShells: -position is 0 (top left) to 8 (bottom right)");
    }
    let (u0, u1) = (a.float("uMin", 0.0), a.float("uMax", 1.0));
    let (v0, v1) = (a.float("vMin", 0.0), a.float("vMax", 1.0));
    let (col, row) = (at % 3, at / 3);
    let moves = per_shell(&shells, |sh| {
        let x = match col {
            0 => u0 - sh.lo.x,
            1 => (u0 + u1) * 0.5 - sh.centre().x,
            _ => u1 - sh.hi.x,
        };
        let y = match row {
            0 => v1 - sh.hi.y,
            1 => (v0 + v1) * 0.5 - sh.centre().y,
            _ => v0 - sh.lo.y,
        };
        Move::by(DVec2::new(x, y))
    });
    apply(s, a, "texSnapShells", "Snap Shells", moves)
}

fn randomize_shells(s: &mut Session, a: &Args) -> CmdResult {
    let shells = all_shells(s, a, "texRandomizeShells")?;
    let (tr, rot, sc) = (a.float("translate", 0.0).abs(), a.float("rotate", 0.0).abs(), a.float("scale", 0.0).abs() / 100.0);
    let mut seed = (a.int("seed", 1) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut rnd = move || {
        // xorshift64*: -1 to 1.
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        let x = seed.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11;
        (x as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    };
    let mut moves = Vec::new();
    for l in &shells {
        let mut row = Vec::new();
        for sh in l {
            let k = 1.0 + rnd() * sc;
            row.push(Move { offset: DVec2::new(rnd() * tr, rnd() * tr), scale: DVec2::splat(k.max(0.01)), degrees: rnd() * rot, pivot: sh.centre() });
        }
        moves.push(row);
    }
    apply(s, a, "texRandomizeShells", "Randomize Shells", moves)
}

fn linear_align(s: &mut Session, a: &Args) -> CmdResult {
    each(s, a, "texLinearAlignUVs", "Linear Align", "polyTweakUV", |m, t, tp| {
        // The UVs named onto the line that runs best through them.
        let corners: Vec<Index> = t.maps.iter().filter_map(|&mp| tp.maps.get(mp as usize)).flatten().copied().collect();
        let maps: Vec<DVec2> = t.maps.iter().filter_map(|&mp| tp.maps.get(mp as usize).and_then(|c| c.first()).map(|&c| uv_at(m, c))).collect();
        if maps.len() < 2 {
            return;
        }
        let mean = maps.iter().copied().sum::<DVec2>() / maps.len() as f64;
        let (mut xx, mut xy, mut yy) = (0.0, 0.0, 0.0);
        for p in &maps {
            let d = *p - mean;
            xx += d.x * d.x;
            xy += d.x * d.y;
            yy += d.y * d.y;
        }
        let ang = 0.5 * (2.0 * xy).atan2(xx - yy);
        let dir = DVec2::new(ang.cos(), ang.sin());
        let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
        for c in corners {
            let p = DVec2::new(layer[c as usize][0] as f64, layer[c as usize][1] as f64);
            let q = mean + dir * (p - mean).dot(dir);
            layer[c as usize] = [q.x as f32, q.y as f32];
        }
    })
}

/// The UVs named, as maps with their corners, per mesh.
fn named_maps(_m: &Mesh, t: &Target, tp: &UvTopo) -> Vec<(Index, Vec<Index>)> {
    if t.maps.is_empty() && t.whole {
        // A whole mesh (mesh.map[*], or the mesh itself): every UV.
        return tp.maps.iter().enumerate().map(|(i, c)| (i as Index, c.clone())).collect();
    }
    t.maps.iter().filter_map(|&mp| tp.maps.get(mp as usize).map(|c| (mp, c.clone()))).collect()
}

fn set_corners(m: &mut Mesh, corners: &[Index], p: DVec2) {
    let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
    for &c in corners {
        layer[c as usize] = [p.x as f32, p.y as f32];
    }
}

/// Each UV edge between two of the maps named that runs within `limit` degrees of U (or V) is made to run along it: both
/// ends take the middle of their V (or U). A few passes, so runs of edges settle on one line.
fn straighten(m: &mut Mesh, tp: &UvTopo, named: &BTreeSet<Index>, limit: f64, along_u: bool, along_v: bool, passes: usize) {
    let mut edges: BTreeSet<(Index, Index)> = BTreeSet::new();
    for &mp in named {
        let Some(corners) = tp.maps.get(mp as usize) else { continue };
        for &c in corners {
            let n = tp.corner_map[m.corner_next(c) as usize];
            if named.contains(&n) && mp != n {
                edges.insert((mp.min(n), mp.max(n)));
            }
        }
    }
    for _ in 0..passes {
        for &(p, q) in &edges {
            let (a, b) = (tp.maps[p as usize][0], tp.maps[q as usize][0]);
            let (pa, pb) = (uv_at(m, a), uv_at(m, b));
            let d = pb - pa;
            if d.length_squared() < 1e-20 {
                continue;
            }
            let ang = d.y.abs().atan2(d.x.abs()).to_degrees();
            let (mut na, mut nb) = (pa, pb);
            if along_u && ang <= limit {
                let y = (pa.y + pb.y) * 0.5;
                na.y = y;
                nb.y = y;
            } else if along_v && 90.0 - ang <= limit {
                let x = (pa.x + pb.x) * 0.5;
                na.x = x;
                nb.x = x;
            } else {
                continue;
            }
            set_corners(m, &tp.maps[p as usize], na);
            set_corners(m, &tp.maps[q as usize], nb);
        }
    }
}

fn straighten_uvs(s: &mut Session, a: &Args) -> CmdResult {
    let limit = a.float("angle", 30.0).clamp(0.0, 90.0);
    let (along_u, along_v) = (a.get("u").is_none_or(|v| v.as_bool().unwrap_or(true)), a.get("v").is_none_or(|v| v.as_bool().unwrap_or(true)));
    each(s, a, "texStraightenUVs", "Straighten UVs", "polyTweakUV", |m, t, tp| {
        let named: BTreeSet<Index> = t.maps.iter().copied().collect();
        straighten(m, tp, &named, limit, along_u, along_v, 8);
    })
}

fn match_uvs(s: &mut Session, a: &Args) -> CmdResult {
    let tol = a.float("tolerance", 0.01).max(0.0);
    each(s, a, "texMatchUVs", "Match UVs", "polyTweakUV", |m, t, tp| {
        // UVs named that lie within the tolerance of one another go to where they are on average, without joining.
        let maps = named_maps(m, t, tp);
        let pos: Vec<DVec2> = maps.iter().map(|(_, c)| uv_at(m, c[0])).collect();
        let mut group: Vec<usize> = (0..maps.len()).collect();
        fn root(g: &mut Vec<usize>, i: usize) -> usize {
            let mut r = i;
            while g[r] != r {
                r = g[r];
            }
            g[i] = r;
            r
        }
        for i in 0..maps.len() {
            for j in i + 1..maps.len() {
                if (pos[i] - pos[j]).length() <= tol {
                    let (ri, rj) = (root(&mut group, i), root(&mut group, j));
                    group[ri] = rj;
                }
            }
        }
        let mut sums: std::collections::BTreeMap<usize, (DVec2, f64)> = Default::default();
        for i in 0..maps.len() {
            let r = root(&mut group, i);
            let e = sums.entry(r).or_insert((DVec2::ZERO, 0.0));
            e.0 += pos[i];
            e.1 += 1.0;
        }
        for (i, (_, corners)) in maps.iter().enumerate() {
            let r = root(&mut group, i);
            let (sum, n) = sums[&r];
            set_corners(m, corners, sum / n);
        }
    })
}

fn match_grid(s: &mut Session, a: &Args) -> CmdResult {
    let (gu, gv) = (a.float("gridU", 16.0).max(1.0), a.float("gridV", 16.0).max(1.0));
    each(s, a, "texMatchGrid", "Match Grid", "polyTweakUV", |m, t, tp| {
        // Each UV named to the nearest crossing of a grid of gridU by gridV over the 0 to 1 tile.
        for (_, corners) in named_maps(m, t, tp) {
            let p = uv_at(m, corners[0]);
            set_corners(m, &corners, DVec2::new((p.x * gu).round() / gu, (p.y * gv).round() / gv));
        }
    })
}

fn snap_together(s: &mut Session, a: &Args) -> CmdResult {
    // Two UVs, on two shells: the second's shell moves so its UV lands on the first (-order ab), or the other way.
    let names = a.named();
    if names.len() != 2 {
        fail!("texSnapTogether: name two UVs, one on each shell, in order");
    }
    let ba = a.string("order", "ab") == "ba";
    let (fixed, moving) = if ba { (&names[1], &names[0]) } else { (&names[0], &names[1]) };
    let at = |s: &mut Session, n: &str| -> CmdResult<DVec2> {
        let v = s.run("polyEditUV", serde_json::json!({"args": [n], "query": true}))?;
        let l = v.as_array().cloned().unwrap_or_default();
        match (l.first().and_then(|x| x.as_f64()), l.get(1).and_then(|x| x.as_f64())) {
            (Some(u), Some(v)) => Ok(DVec2::new(u, v)),
            _ => fail!("texSnapTogether: {n} is not a UV"),
        }
    };
    let (to, from) = (at(s, fixed)?, at(s, moving)?);
    let by = to - from;
    s.run("polyEditUVShell", serde_json::json!({"args": [moving], "u": by.x, "v": by.y}))
}

/// The faces of a mesh whose UVs run clockwise, seen from above in UV: shown back to front in a texture.
fn backwards(m: &Mesh, f: Index) -> bool {
    let pts: Vec<DVec2> = m.face_corners(f).into_iter().map(|c| uv_at(m, c)).collect();
    let mut area = 0.0;
    for i in 0..pts.len() {
        area += pts[i].perp_dot(pts[(i + 1) % pts.len()]);
    }
    area < 0.0
}

fn select_by_type(s: &mut Session, a: &Args) -> CmdResult {
    let kind = a.string("type", "backFacing");
    let targets = uv_ops::targets(s, a, "texSelectByType")?;
    let g = &mut s.graph;
    let mut out = Vec::new();
    for (mesh, t) in targets {
        let Some(m) = crate::modifiers::cage(g, mesh) else { continue };
        let tp = uv_ops::topo(&m);
        let path = crate::components::display_path(g, crate::dag::transform_of(g, mesh));
        let faces = crate::uv_misc::target_faces(&m, &t, &tp);
        match kind.as_str() {
            "backFacing" | "frontFacing" => {
                let want_back = kind == "backFacing";
                for f in faces {
                    if backwards(&m, f) == want_back {
                        out.push(serde_json::json!(format!("{path}.f[{f}]")));
                    }
                }
            }
            "textureBorders" => {
                // UVs on an edge where the UVs split or nothing lies across: the shells' borders.
                let mut maps: BTreeSet<Index> = BTreeSet::new();
                for f in faces {
                    for c in m.face_corners(f) {
                        let e = m.corner_edge(c);
                        if tp.split.get(e as usize).copied().unwrap_or(true) {
                            maps.insert(tp.corner_map[c as usize]);
                            maps.insert(tp.corner_map[m.corner_next(c) as usize]);
                        }
                    }
                }
                out.extend(maps.into_iter().map(|mp| serde_json::json!(format!("{path}.map[{mp}]"))));
            }
            "unmapped" => {
                // Faces with no mapping: all their corners on one UV.
                for f in faces {
                    let cs = m.face_corners(f);
                    let first = uv_at(&m, cs[0]);
                    if cs.iter().all(|&c| (uv_at(&m, c) - first).length_squared() < 1e-14) {
                        out.push(serde_json::json!(format!("{path}.f[{f}]")));
                    }
                }
            }
            o => fail!("texSelectByType: -type is backFacing, frontFacing, textureBorders or unmapped, not {o}"),
        }
    }
    Ok(serde_json::Value::Array(out))
}

pub fn register(c: &mut Commands<Session>) {
    register_more(c);
    c.register(
        poly_flag(Cmd::new("texStraightenUVs", "Make each UV edge between the UVs named that runs within -angle of U or V run along it. Returns the nodes.", straighten_uvs))
            .flag(f("angle", "a", Fl, "Most an edge may lean and still be straightened, degrees (30)."))
            .flag(f("u", "u", ArgType::Bool, "Straighten along U (on)."))
            .flag(f("v", "v", ArgType::Bool, "Straighten along V (on).")),
    );
    c.register(
        poly_flag(Cmd::new("texMatchUVs", "Bring the UVs named that lie within -tolerance of one another to where they are on average, unjoined. Returns the nodes.", match_uvs))
            .flag(f("tolerance", "tol", Fl, "How near, in UV units (0.01).")),
    );
    c.register(
        poly_flag(Cmd::new("texMatchGrid", "Move each UV named to the nearest crossing of a grid over the 0 to 1 tile. Returns the nodes.", match_grid))
            .flag(f("gridU", "gu", Fl, "Grid divisions across U (16)."))
            .flag(f("gridV", "gv", Fl, "Grid divisions across V (16).")),
    );
    c.register(
        Cmd::new("texSnapTogether", "Two UVs on two shells, in order: the second's shell moves so its UV lands on the first; -order ba the other way. Returns the node.", snap_together)
            .positional("two UVs")
            .flag(f("order", "o", S, "ab (the default) or ba.").choices(&["ab", "ba"])),
    );
    c.register(
        Cmd::new("texSelectByType", "The faces (backFacing, frontFacing) or UVs (textureBorders) of the meshes or components named, for Select By Type. Returns them.", select_by_type)
            .positional("meshes or components (the selection when none)")
            .flag(f("type", "t", S, "backFacing, frontFacing, textureBorders or unmapped.").choices(&["backFacing", "frontFacing", "textureBorders", "unmapped"])),
    );
    c.register(poly_flag(Cmd::new("texOrientShells", "Turn each shell named to lie square to U and V: the turn that makes its bounding box smallest. Returns the nodes.", orient_shells)));
    c.register(poly_flag(Cmd::new("texOrientEdge", "Turn each shell so the first edge named in it lies along U or V, whichever is nearer. Returns the nodes.", orient_edge)));
    c.register(poly_flag(Cmd::new("texStackShells", "Move the shells named onto one another, their middles on the middle of them all. Returns the nodes.", stack_shells)));
    c.register(
        poly_flag(Cmd::new("texUnstackShells", "Move the shells named so none overlap, each as little as it can, the largest staying put. Returns the nodes.", unstack_shells))
            .flag(f("spacing", "sp", Fl, "Gap kept between shells, in UV units (0.01).")),
    );
    c.register(poly_flag(Cmd::new("texGatherShells", "Move each shell named by whole tiles so its middle is in 0 to 1. Returns the nodes.", gather_shells)));
    c.register(
        poly_flag(Cmd::new("texDistributeShells", "Space the shells named along U or V, in the order they lie, from the first one's edge, -spacing apart. Returns the nodes.", distribute_shells))
            .flag(f("direction", "d", S, "u or v.").choices(&["u", "v"]))
            .flag(f("spacing", "sp", Fl, "Gap between shells, in UV units (0).")),
    );
    c.register(
        poly_flag(Cmd::new("texAlignShells", "Line the shells named up on the outermost one's edge or the middle of them all. Returns the nodes.", align_shells))
            .flag(f("alignTo", "at", S, "left, right, top, bottom, centerU or centerV.").choices(&["left", "right", "top", "bottom", "centerU", "centerV"])),
    );
    c.register(
        poly_flag(Cmd::new("texSnapShells", "Move each shell named to one of nine places of a UV range (0 to 1 by default): a corner, the middle of a side, or the middle. Returns the nodes.", snap_shells))
            .flag(f("position", "p", I, "0 top left, 1 top, 2 top right, 3 left, 4 middle, 5 right, 6 bottom left, 7 bottom, 8 bottom right."))
            .flag(f("uMin", "u0", Fl, "The range's lowest U (0)."))
            .flag(f("uMax", "u1", Fl, "Its highest U (1)."))
            .flag(f("vMin", "v0", Fl, "The range's lowest V (0)."))
            .flag(f("vMax", "v1", Fl, "Its highest V (1).")),
    );
    c.register(
        poly_flag(Cmd::new("texRandomizeShells", "Move, turn and scale each shell named by a random amount up to the most given. Returns the nodes.", randomize_shells))
            .flag(f("translate", "t", Fl, "Furthest a shell moves, in U and in V."))
            .flag(f("rotate", "r", Fl, "Most it turns, degrees."))
            .flag(f("scale", "s", Fl, "Most it grows or shrinks, as a percent of its size."))
            .flag(f("seed", "sd", I, "Seed: the same seed gives the same result.")),
    );
    c.register(poly_flag(Cmd::new("texLinearAlignUVs", "Line the UVs named up along the straight line that runs best through them. Returns the nodes.", linear_align)));
}

// ---- The rest of the toolkit's Modify and Cut/Sew tools: Cycle, Symmetrize, Straighten Shell, Snap and Stack, Layout
// Along, Distribute UVs, Create UV Shell and Create Shell (Grid).

fn cycle_uvs(s: &mut Session, a: &Args) -> CmdResult {
    each(s, a, "polyRotateUVsByVertex", "Cycle UVs", "polyTweakUV", |m, t, tp| {
        // Each face's UVs move on one corner round it.
        for f in crate::uv_misc::target_faces(m, t, tp) {
            let cs = m.face_corners(f);
            let uvs: Vec<DVec2> = cs.iter().map(|&c| uv_at(m, c)).collect();
            let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
            for (i, &c) in cs.iter().enumerate() {
                let q = uvs[(i + 1) % uvs.len()];
                layer[c as usize] = [q.x as f32, q.y as f32];
            }
        }
    })
}

/// The UVs in the order named, or as selected.
fn ordered_names(s: &mut Session, a: &Args) -> CmdResult<Vec<String>> {
    let mut names = a.named();
    if names.is_empty() {
        let v = s.run("ls", serde_json::json!({"selection": true, "flatten": true}))?;
        names = v.as_array().into_iter().flatten().filter_map(|x| x.as_str().map(str::to_string)).collect();
    }
    Ok(names)
}

/// Where each UV named sits among the shells `all_shells` gives: (mesh number, shell number, its UV).
fn locate(s: &mut Session, a: &Args, names: &[String]) -> CmdResult<Vec<(usize, usize, DVec2)>> {
    let all = uv_ops::targets(s, a, "uv")?;
    let order: Vec<_> = all.keys().copied().collect();
    let mut out = Vec::new();
    for n in names {
        let one = Args { positional: vec![serde_json::json!(n)], flags: Default::default() };
        let Ok(t) = uv_ops::targets(s, &one, "uv") else { continue };
        for (mesh, t) in t {
            let Some(i) = order.iter().position(|&x| x == mesh) else { continue };
            let Some(m) = crate::modifiers::cage(&mut s.graph, mesh) else { continue };
            let tp = uv_ops::topo(&m);
            let shells = shells_in(&m, &all[&mesh], &tp);
            for &mp in &t.maps {
                let Some(&c) = tp.maps.get(mp as usize).and_then(|l| l.first()) else { continue };
                if let Some(j) = shells.iter().position(|sh| sh.corners.contains(&c)) {
                    out.push((i, j, uv_at(&m, c)));
                }
            }
        }
    }
    Ok(out)
}

fn snap_stack(s: &mut Session, a: &Args) -> CmdResult {
    // UVs picked on several shells: each shell moves, turns and scales so its UVs land on those picked on the shell of
    // the last UV picked; one UV a shell only moves it.
    let names = ordered_names(s, a)?;
    let sel = Args { positional: names.iter().map(|n| serde_json::json!(n)).collect(), flags: a.flags.clone() };
    let shells = all_shells(s, &sel, "texSnapStackShells")?;
    let at = locate(s, &sel, &names)?;
    let Some(&(ti, tj, _)) = at.last() else { fail!("texSnapStackShells: pick UVs on two or more shells") };
    let target: Vec<DVec2> = at.iter().filter(|x| x.0 == ti && x.1 == tj).map(|x| x.2).collect();
    let mut groups: Vec<((usize, usize), Vec<DVec2>)> = Vec::new();
    for &(i, j, p) in &at {
        if (i, j) == (ti, tj) {
            continue;
        }
        match groups.iter_mut().find(|g| g.0 == (i, j)) {
            Some(g) => g.1.push(p),
            None => groups.push(((i, j), vec![p])),
        }
    }
    if groups.is_empty() {
        fail!("texSnapStackShells: pick UVs on two or more shells");
    }
    let mut moves: Vec<Vec<Move>> = shells.iter().map(|l| vec![Move::by(DVec2::ZERO); l.len()]).collect();
    for ((i, j), src) in groups {
        let (p0, q0) = (src[0], target[0]);
        let mut mv = Move { offset: q0 - p0, scale: DVec2::ONE, degrees: 0.0, pivot: p0 };
        if src.len() > 1 && target.len() > 1 {
            let (dp, dq) = (src[1] - p0, target[1] - q0);
            if dp.length() > 1e-12 && dq.length() > 1e-12 {
                mv.degrees = (dq.y.atan2(dq.x) - dp.y.atan2(dp.x)).to_degrees();
                mv.scale = DVec2::splat(dq.length() / dp.length());
            }
        }
        if let Some(slot) = moves.get_mut(i).and_then(|l| l.get_mut(j)) {
            *slot = mv;
        }
    }
    apply(s, &sel, "texSnapStackShells", "Snap and Stack", moves)
}

fn layout_along(s: &mut Session, a: &Args) -> CmdResult {
    // The shells in a row along U (or V), in the order they lie, -spacing apart from 0, then scaled together to fit the
    // tile.
    let shells = all_shells(s, a, "texLayoutAlong")?;
    let axis = axis_of(a, "texLayoutAlong")?;
    let gap = a.float("spacing", 0.01).max(0.0);
    let mut order: Vec<(usize, usize)> = shells.iter().enumerate().flat_map(|(i, l)| (0..l.len()).map(move |j| (i, j))).collect();
    order.sort_by(|x, y| shells[x.0][x.1].centre()[axis].total_cmp(&shells[y.0][y.1].centre()[axis]));
    let (mut run, mut across) = (gap, 0.0f64);
    let mut starts = Vec::new();
    for &(i, j) in &order {
        let sh = &shells[i][j];
        starts.push(run);
        run += sh.size()[axis] + gap;
        across = across.max(sh.size()[1 - axis]);
    }
    let k = (1.0 / run.max(1e-12)).min(1.0 / (across + 2.0 * gap).max(1e-12));
    let mut moves: Vec<Vec<Move>> = shells.iter().map(|l| vec![Move::by(DVec2::ZERO); l.len()]).collect();
    for (&(i, j), start) in order.iter().zip(starts) {
        let sh = &shells[i][j];
        let mut to = DVec2::splat(gap * k);
        to[axis] = start * k;
        moves[i][j] = Move { offset: to - sh.lo, scale: DVec2::splat(k), degrees: 0.0, pivot: sh.lo };
    }
    apply(s, a, "texLayoutAlong", "Layout Along", moves)
}

fn distribute_to_target(s: &mut Session, a: &Args) -> CmdResult {
    // The shells evenly spaced on the line from the one furthest from the target (the shell of the last UV picked) to it.
    let names = ordered_names(s, a)?;
    let sel = Args { positional: names.iter().map(|n| serde_json::json!(n)).collect(), flags: a.flags.clone() };
    let shells = all_shells(s, &sel, "texDistributeShells")?;
    let at = locate(s, &sel, &names)?;
    let Some(&(ti, tj, _)) = at.last() else { fail!("texDistributeShells: pick shells, the target last") };
    let goal = shells[ti][tj].centre();
    let mut rest: Vec<(usize, usize)> = shells.iter().enumerate().flat_map(|(i, l)| (0..l.len()).map(move |j| (i, j))).filter(|&x| x != (ti, tj)).collect();
    rest.sort_by(|x, y| (shells[y.0][y.1].centre() - goal).length().total_cmp(&(shells[x.0][x.1].centre() - goal).length()));
    let mut moves: Vec<Vec<Move>> = shells.iter().map(|l| vec![Move::by(DVec2::ZERO); l.len()]).collect();
    if let Some(&(fi, fj)) = rest.first() {
        let from = shells[fi][fj].centre();
        let n = rest.len();
        for (k, &(i, j)) in rest.iter().enumerate() {
            let p = from + (goal - from) * (k as f64 / n as f64);
            moves[i][j] = Move::by(p - shells[i][j].centre());
        }
    }
    apply(s, &sel, "texDistributeShells", "Distribute Shells", moves)
}

fn distribute_uvs(s: &mut Session, a: &Args) -> CmdResult {
    let axis = axis_of(a, "texDistributeUVs")?;
    let spacing = a.float("spacing", 0.0);
    each(s, a, "texDistributeUVs", "Distribute UVs", "polyTweakUV", |m, t, tp| {
        // The UVs named in the order they lie along the axis: the first and last stay and the rest spread evenly
        // between, or each -spacing on from the first.
        let mut maps = named_maps(m, t, tp);
        if maps.len() < 2 {
            return;
        }
        maps.sort_by(|x, y| uv_at(m, x.1[0])[axis].total_cmp(&uv_at(m, y.1[0])[axis]));
        let first = uv_at(m, maps[0].1[0])[axis];
        let last = uv_at(m, maps[maps.len() - 1].1[0])[axis];
        let n = (maps.len() - 1) as f64;
        for (k, (_, corners)) in maps.iter().enumerate() {
            let mut p = uv_at(m, corners[0]);
            p[axis] = if spacing > 0.0 { first + spacing * k as f64 } else { first + (last - first) * (k as f64 / n) };
            set_corners(m, corners, p);
        }
    })
}

fn straighten_shell(s: &mut Session, a: &Args) -> CmdResult {
    let limit = a.float("angle", 30.0).clamp(0.0, 90.0);
    each(s, a, "texStraightenShell", "Straighten Shell", "polyTweakUV", |m, t, tp| {
        // Every UV of each shell named: its rows and columns run straight along U and V.
        for sh in shells_in(m, t, tp) {
            let named: BTreeSet<Index> = sh.corners.iter().map(|&c| tp.corner_map[c as usize]).collect();
            straighten(m, tp, &named, limit, true, true, 16);
        }
    })
}

fn symmetrize(s: &mut Session, a: &Args) -> CmdResult {
    let Some(edge_name) = a.get("edge").and_then(|v| v.as_str()).map(str::to_string) else { fail!("texSymmetrizeUVs: -edge names the edge to mirror about") };
    let edge = edge_name.rsplit_once(".e[").and_then(|(_, r)| r.trim_end_matches(']').parse::<Index>().ok());
    let Some(edge) = edge else { fail!("texSymmetrizeUVs: {edge_name} is not an edge") };
    each(s, a, "texSymmetrizeUVs", "Symmetrize UVs", "polyTweakUV", |m, t, tp| {
        if !m.edge_valid(edge) {
            return;
        }
        // The mirror in 3D: the plane through the edge, square to its faces. In UV: the line through the edge's UVs.
        let (pa, pb) = (m.edge_point(edge, 0), m.edge_point(edge, 1));
        let (a3, b3) = (m.position(pa), m.position(pb));
        let mut n = glam::DVec3::ZERO;
        let mut uv_line = None;
        m.for_each_edge_corner(edge, |c| {
            n += m.face_normal(m.corner_face(c));
            if uv_line.is_none() {
                uv_line = Some((uv_at(m, c), uv_at(m, m.corner_next(c))));
            }
        });
        let Some((ua, ub)) = uv_line else { return };
        let normal = (b3 - a3).cross(n).normalize_or_zero();
        let dir = (ub - ua).normalize_or_zero();
        if normal == glam::DVec3::ZERO || dir == DVec2::ZERO {
            return;
        }
        let size = m.bounds().size().max_element().max(1e-9);
        let pts: Vec<glam::DVec3> = (0..m.point_capacity()).map(|p| if m.point_valid(p) { m.position(p) } else { glam::DVec3::splat(f64::MAX) }).collect();
        let shell_of_edge = {
            let mut sh = aztek_geo::INVALID;
            m.for_each_edge_corner(edge, |c| sh = tp.face_shell[m.corner_face(c) as usize]);
            sh
        };
        // Each UV named goes across the line to the corners of the point opposite its own.
        for (_, corners) in named_maps(m, t, tp) {
            let p = m.corner_point(corners[0]);
            let q3 = pts[p as usize] - normal * 2.0 * (pts[p as usize] - a3).dot(normal);
            let Some((other, d)) = pts.iter().enumerate().map(|(i, x)| (i as Index, (*x - q3).length())).min_by(|x, y| x.1.total_cmp(&y.1)) else { continue };
            if d > size * 1e-3 || other == p {
                continue;
            }
            let u = uv_at(m, corners[0]);
            let rel = u - ua;
            let mirrored = ua + dir * 2.0 * rel.dot(dir) - rel;
            let mut targets = Vec::new();
            m.for_each_point_corner(other, |c| {
                if shell_of_edge == aztek_geo::INVALID || tp.face_shell[m.corner_face(c) as usize] == shell_of_edge {
                    targets.push(c);
                }
            });
            set_corners(m, &targets, mirrored);
        }
    })
}

fn create_shell(s: &mut Session, a: &Args) -> CmdResult {
    let grid = a.bool("grid");
    each(s, a, "texCreateShell", if grid { "Create Shell (Grid)" } else { "Create UV Shell" }, "polyMapCut", |m, t, tp| {
        let faces = crate::uv_misc::target_faces(m, t, tp);
        let inside: BTreeSet<Index> = faces.iter().copied().collect();
        // Cut round the faces: their own shell.
        let mut outer = Vec::new();
        for &f in &faces {
            for c in m.face_corners(f) {
                let e = m.corner_edge(c);
                let mut all_in = true;
                m.for_each_edge_corner(e, |k| all_in &= inside.contains(&m.corner_face(k)));
                if !all_in {
                    outer.push(e);
                }
            }
        }
        uv_ops::cut(m, &outer);
        if !grid {
            return;
        }
        // Grid: quads walked across from the first, each a unit square beside the one it was reached from, then the
        // whole fitted to 0 to 1.
        let mut at: std::collections::BTreeMap<Index, DVec2> = Default::default();
        let mut done: BTreeSet<Index> = BTreeSet::new();
        let Some(&f0) = faces.iter().find(|&&f| m.face_size(f) == 4) else { return };
        let cs = m.face_corners(f0);
        for (i, uv) in [DVec2::new(0.0, 0.0), DVec2::new(1.0, 0.0), DVec2::new(1.0, 1.0), DVec2::new(0.0, 1.0)].into_iter().enumerate() {
            at.insert(m.corner_point(cs[i]), uv);
        }
        done.insert(f0);
        let mut queue = std::collections::VecDeque::from([f0]);
        while let Some(f) = queue.pop_front() {
            let cs = m.face_corners(f);
            for (i, &c) in cs.iter().enumerate() {
                let e = m.corner_edge(c);
                let (pa, pb) = (m.corner_point(c), m.corner_point(cs[(i + 1) % 4]));
                let (pc, pd) = (m.corner_point(cs[(i + 3) % 4]), m.corner_point(cs[(i + 2) % 4]));
                let mut next = None;
                m.for_each_edge_corner(e, |k| {
                    let g = m.corner_face(k);
                    if g != f && inside.contains(&g) && !done.contains(&g) && m.face_size(g) == 4 {
                        next = Some(g);
                    }
                });
                let Some(g) = next else { continue };
                let (Some(&ua), Some(&ub), Some(&uc), Some(&ud)) = (at.get(&pa), at.get(&pb), at.get(&pc), at.get(&pd)) else { continue };
                // pa's neighbour in g away from f lies past pa from pc; pb's past pb from pd.
                for k in m.face_corners(g) {
                    let p = m.corner_point(k);
                    if p == pa || p == pb || at.contains_key(&p) {
                        continue;
                    }
                    let (prev, nxt) = (m.corner_point(m.corner_prev(k)), m.corner_point(m.corner_next(k)));
                    if prev == pa || nxt == pa {
                        at.insert(p, ua + (ua - uc));
                    } else if prev == pb || nxt == pb {
                        at.insert(p, ub + (ub - ud));
                    }
                }
                done.insert(g);
                queue.push_back(g);
            }
        }
        let (mut lo, mut hi) = (DVec2::splat(f64::MAX), DVec2::splat(f64::MIN));
        for p in at.values() {
            lo = lo.min(*p);
            hi = hi.max(*p);
        }
        let size = (hi - lo).max(DVec2::splat(1e-12));
        let sets: Vec<(Index, DVec2)> = done.iter().flat_map(|&f| m.face_corners(f)).filter_map(|k| at.get(&m.corner_point(k)).map(|p| (k, (*p - lo) / size))).collect();
        let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
        for (k, q) in sets {
            layer[k as usize] = [q.x as f32, q.y as f32];
        }
    })
}


fn brush(s: &mut Session, a: &Args) -> CmdResult {
    let mode = a.string("mode", "grab");
    let radius = a.float("radius", 0.05).max(1e-6);
    let strength = a.float("strength", 1.0).clamp(0.0, 1.0);
    let samples: Vec<[f64; 4]> = a
        .get("samples")
        .and_then(|v| v.as_array())
        .map(|l| l.iter().filter_map(|x| x.as_f64()).collect::<Vec<f64>>())
        .unwrap_or_default()
        .chunks_exact(4)
        .map(|c| [c[0], c[1], c[2], c[3]])
        .collect();
    if samples.is_empty() {
        fail!("texBrushUVs: -samples gives the stroke, u v du dv for each step");
    }
    let label = match mode.as_str() {
        "grab" => "Grab UVs",
        "pinch" => "Pinch UVs",
        "smear" => "Smear UVs",
        "smooth" | "optimize" | "unfold" => "Smooth UVs",
        "pin" | "unpin" => "Pin UVs",
        o => fail!("texBrushUVs: -mode is grab, pinch, smear, smooth, optimize, unfold, pin or unpin, not {o}"),
    };
    let falloff = |d: f64| {
        let x = (d / radius).min(1.0);
        let w = 1.0 - x * x;
        w * w * strength
    };
    if mode == "pin" || mode == "unpin" {
        // The UVs the stroke passes over, pinned (or unpinned) as Pin does.
        let targets = uv_ops::targets(s, a, "texBrushUVs")?;
        let mut names = Vec::new();
        for (mesh, _) in targets {
            let Some(m) = crate::modifiers::cage(&mut s.graph, mesh) else { continue };
            let tp = uv_ops::topo(&m);
            let path = crate::components::display_path(&s.graph, crate::dag::transform_of(&s.graph, mesh));
            for (i, cs) in tp.maps.iter().enumerate() {
                let p = uv_at(&m, cs[0]);
                if samples.iter().any(|x| (p - DVec2::new(x[0], x[1])).length() <= radius) {
                    names.push(serde_json::json!(format!("{path}.map[{i}]")));
                }
            }
        }
        if names.is_empty() {
            return Ok(serde_json::json!([]));
        }
        return s.run("polyPinUV", serde_json::json!({"args": names, "value": if mode == "pin" { 1.0 } else { 0.0 }}));
    }
    each(s, a, "texBrushUVs", label, "polyTweakUV", |m, _t, tp| {
        let n = tp.maps.len();
        let pinned: Vec<bool> = {
            let pin = m.corner_attrs().get::<f32>("uvPin");
            tp.maps.iter().map(|cs| cs.iter().any(|&c| pin.get(c as usize).is_some_and(|&p| p > 0.5))).collect()
        };
        let mut pos: Vec<DVec2> = tp.maps.iter().map(|cs| uv_at(m, cs[0])).collect();
        let mut near: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, cs) in tp.maps.iter().enumerate() {
            for &c in cs {
                for k in [m.corner_next(c), m.corner_prev(c)] {
                    let j = tp.corner_map[k as usize] as usize;
                    if j < n && j != i && !near[i].contains(&j) {
                        near[i].push(j);
                    }
                }
            }
        }
        match mode.as_str() {
            "grab" => {
                // The UVs under the brush where the stroke began, carried the whole way.
                let start = DVec2::new(samples[0][0], samples[0][1]);
                let total: DVec2 = samples.iter().map(|x| DVec2::new(x[2], x[3])).sum();
                for i in 0..n {
                    if !pinned[i] {
                        let w = falloff((pos[i] - start).length());
                        pos[i] += total * w;
                    }
                }
            }
            _ => {
                for smp in &samples {
                    let c = DVec2::new(smp[0], smp[1]);
                    let d = DVec2::new(smp[2], smp[3]);
                    let was = pos.clone();
                    for i in 0..n {
                        if pinned[i] {
                            continue;
                        }
                        let w = falloff((was[i] - c).length());
                        if w <= 0.0 {
                            continue;
                        }
                        match mode.as_str() {
                            "smear" => pos[i] += d * w,
                            "pinch" => pos[i] += (c - was[i]) * w * 0.1,
                            _ => {
                                if near[i].is_empty() {
                                    continue;
                                }
                                let avg = near[i].iter().map(|&j| was[j]).sum::<DVec2>() / near[i].len() as f64;
                                pos[i] += (avg - was[i]) * w * 0.5;
                            }
                        }
                    }
                }
            }
        }
        let layer = m.corner_attrs_mut().get_or_add::<[f32; 2]>(uv_ops::UV);
        for (i, cs) in tp.maps.iter().enumerate() {
            for &c in cs {
                layer[c as usize] = [pos[i].x as f32, pos[i].y as f32];
            }
        }
    })
}


fn lattice(s: &mut Session, a: &Args) -> CmdResult {
    // A lattice of columns x rows control points over a box: each UV named moves as the lattice cell it is in moved,
    // bilinear between its corners.
    let (nu, nv) = (a.int("columns", 3).max(2) as usize, a.int("rows", 3).max(2) as usize);
    let nums = |k: &str| a.get(k).and_then(|v| v.as_array()).map(|l| l.iter().filter_map(|x| x.as_f64()).collect::<Vec<f64>>()).unwrap_or_default();
    let (b, moved) = (nums("box"), nums("points"));
    if b.len() != 4 || moved.len() != nu * nv * 2 {
        fail!("texLatticeUVs: -box u0 v0 u1 v1 and -points with columns x rows u v pairs");
    }
    let (lo, hi) = (DVec2::new(b[0], b[1]), DVec2::new(b[2], b[3]));
    let size = (hi - lo).max(DVec2::splat(1e-12));
    let ctrl = |i: usize, j: usize| DVec2::new(moved[(j * nu + i) * 2], moved[(j * nu + i) * 2 + 1]);
    each(s, a, "texLatticeUVs", "Lattice UVs", "polyTweakUV", |m, t, tp| {
        for (_, corners) in named_maps(m, t, tp) {
            let p = uv_at(m, corners[0]);
            let q = ((p - lo) / size).clamp(DVec2::ZERO, DVec2::ONE);
            let (fx, fy) = (q.x * (nu - 1) as f64, q.y * (nv - 1) as f64);
            let (i, j) = ((fx.floor() as usize).min(nu - 2), (fy.floor() as usize).min(nv - 2));
            let (s0, t0) = (fx - i as f64, fy - j as f64);
            let at = ctrl(i, j) * (1.0 - s0) * (1.0 - t0) + ctrl(i + 1, j) * s0 * (1.0 - t0) + ctrl(i, j + 1) * (1.0 - s0) * t0 + ctrl(i + 1, j + 1) * s0 * t0;
            // Where the cell's corners were: the UV keeps its place relative to them.
            let rest = |i: usize, j: usize| lo + size * DVec2::new(i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64);
            let was = rest(i, j) * (1.0 - s0) * (1.0 - t0) + rest(i + 1, j) * s0 * (1.0 - t0) + rest(i, j + 1) * (1.0 - s0) * t0 + rest(i + 1, j + 1) * s0 * t0;
            set_corners(m, &corners, p + (at - was));
        }
    })
}


fn warp_image(s: &mut Session, a: &Args) -> CmdResult {
    // An image laid on a mesh by one UV set, drawn again by another: each face of the mesh, in the output set, takes
    // the image as it lies under the same face in the input set.
    let input = a.string("inputName", "");
    let output = a.string("outputName", "");
    if input.is_empty() || output.is_empty() {
        fail!("polyWarpImage: give -inputName and -outputName");
    }
    let (w, h) = (a.int("xResolution", 1024).clamp(1, 16384) as usize, a.int("yResolution", 1024).clamp(1, 16384) as usize);
    let samples = if a.bool("antiAlias") { 3 } else { 1 };
    let cache = aztek_render::texture::TextureCache::new(256);
    let Some(img) = cache.image(&input, aztek_render::texture::ColorSpace::Srgb) else { fail!("polyWarpImage: can't read {input}") };
    let names = a.named();
    let Some(name) = names.first() else { fail!("polyWarpImage: name a mesh") };
    let Some(id) = s.graph.find(name)? else { fail!("polyWarpImage: no object {name}") };
    let Some(mesh) = crate::components::as_mesh(&s.graph, id) else { fail!("polyWarpImage: {name} is not a mesh") };
    let Some(m) = crate::modifiers::cage(&mut s.graph, mesh) else { fail!("polyWarpImage: {name} has no mesh") };
    let sets = crate::uv_sets::all(&m);
    let cur = crate::uv_sets::current(&m);
    let in_set = a.string("inputUvSetName", &cur);
    let out_set = a.string("outputUvSetName", &cur);
    let uvs_of = |set: &str| -> CmdResult<Vec<DVec2>> {
        if !sets.iter().any(|x| x == set) {
            fail!("polyWarpImage: {name} has no UV set {set}");
        }
        let mut c = (*m).clone();
        crate::uv_sets::set_current(&mut c, set);
        Ok((0..c.corner_capacity()).map(|k| if c.corner_valid(k) { uv_at(&c, k) } else { DVec2::ZERO }).collect())
    };
    let (src, dst) = (uvs_of(&in_set)?, uvs_of(&out_set)?);
    let fill = glam::DVec3::ZERO;
    let mut px = vec![glam::Vec4::new(fill.x as f32, fill.y as f32, fill.z as f32, 1.0); w * h];
    for f in 0..m.face_capacity() {
        if !m.face_valid(f) {
            continue;
        }
        let cs = m.face_corners(f);
        for t in 1..cs.len().saturating_sub(1) {
            let tri = [cs[0], cs[t], cs[t + 1]];
            let o: Vec<DVec2> = tri.iter().map(|&k| dst[k as usize] * DVec2::new(w as f64, h as f64)).collect();
            let i: Vec<DVec2> = tri.iter().map(|&k| src[k as usize]).collect();
            let area = (o[1] - o[0]).perp_dot(o[2] - o[0]);
            if area.abs() < 1e-12 {
                continue;
            }
            let (lo, hi) = (o[0].min(o[1]).min(o[2]), o[0].max(o[1]).max(o[2]));
            let (x0, x1) = (lo.x.floor().max(0.0) as usize, (hi.x.ceil() as usize).min(w));
            let (y0, y1) = (lo.y.floor().max(0.0) as usize, (hi.y.ceil() as usize).min(h));
            // A footprint for the lookup: how much of the input one output pixel spans.
            let span = ((i[1] - i[0]).perp_dot(i[2] - i[0]).abs() / area.abs()).sqrt() as f32;
            for y in y0..y1 {
                for x in x0..x1 {
                    let mut sum = glam::Vec4::ZERO;
                    let mut n = 0.0;
                    for sy in 0..samples {
                        for sx in 0..samples {
                            let p = DVec2::new(x as f64 + (sx as f64 + 0.5) / samples as f64, y as f64 + (sy as f64 + 0.5) / samples as f64);
                            let b1 = (p - o[0]).perp_dot(o[2] - o[0]) / area;
                            let b2 = (o[1] - o[0]).perp_dot(p - o[0]) / area;
                            let b0 = 1.0 - b1 - b2;
                            if b0 < -1e-9 || b1 < -1e-9 || b2 < -1e-9 {
                                continue;
                            }
                            let q = i[0] * b0 + i[1] * b1 + i[2] * b2;
                            sum += img.sample(glam::Vec2::new(q.x as f32, q.y as f32), span);
                            n += 1.0;
                        }
                    }
                    if n > 0.0 {
                        // Image rows go down, V up.
                        px[(h - 1 - y) * w + x] = sum / n;
                    }
                }
            }
        }
    }
    let path = std::path::Path::new(&output);
    let fmt = aztek_render::output::Format::from_name(&a.string("fileFormat", "")).or_else(|| aztek_render::output::Format::of_path(path)).unwrap_or(aztek_render::output::Format::Png);
    aztek_render::output::write_ldr(path, w as u32, h as u32, &px, fmt).map_err(aztek_ops::CommandError::new)?;
    Ok(serde_json::json!(output))
}

pub fn register_more(c: &mut Commands<Session>) {
    c.register(
        Cmd::new("polyWarpImage", "Warp Image: an image laid on a mesh by one UV set (-iuv) drawn again for another (-ouv), written to -outputName at -xr by -yr. Returns the file.", warp_image)
            .positional("a mesh")
            .flag(f("inputName", "in", S, "The image as the input set lays it."))
            .flag(f("outputName", "on", S, "The image to write."))
            .flag(f("inputUvSetName", "iuv", S, "The UV set the image fits (the current one)."))
            .flag(f("outputUvSetName", "ouv", S, "The UV set to draw it for (the current one)."))
            .flag(f("xResolution", "xr", I, "Width in pixels (1024)."))
            .flag(f("yResolution", "yr", I, "Height in pixels (1024)."))
            .flag(f("antiAlias", "aa", ArgType::Bool, "Nine samples a pixel."))
            .flag(f("bilinear", "bi", ArgType::Bool, "Accepted: lookups are filtered."))
            .flag(f("fileFormat", "ff", S, "png, jpg, tif, tga or exr; from the name otherwise."))
            .flag(f("overwrite", "o", ArgType::Bool, "Accepted: the file is written over.")),
    );
    c.register(
        poly_flag(Cmd::new("texLatticeUVs", "The UV Lattice Tool: the UVs named move with a lattice of -columns x -rows points over -box u0 v0 u1 v1, as its points moved to -points (u v each, row by row from the bottom). Returns the nodes.", lattice))
            .flag(f("columns", "c", I, "Lattice points across U (3)."))
            .flag(f("rows", "r", I, "Lattice points across V (3)."))
            .flag(f("box", "b", ArgType::List, "The lattice's box: u0 v0 u1 v1."))
            .flag(f("points", "p", ArgType::List, "Its points where they are now, u v each, row by row from the bottom.")),
    );
    c.register(
        poly_flag(Cmd::new("texBrushUVs", "A UV brush stroke over the meshes named: -mode grab, pinch, smear, smooth, optimize, unfold, pin or unpin, -radius in UV units, -strength 0 to 1, -samples u v du dv for each step of the stroke. Pinned UVs stay. Returns the nodes.", brush))
            .flag(f("mode", "m", S, "grab, pinch, smear, smooth, optimize, unfold, pin or unpin.").choices(&["grab", "pinch", "smear", "smooth", "optimize", "unfold", "pin", "unpin"]))
            .flag(f("radius", "r", Fl, "The brush's radius in UV units (0.05)."))
            .flag(f("strength", "st", Fl, "0 to 1 (1)."))
            .flag(f("samples", "sm", ArgType::List, "The stroke: u v du dv for each step.")),
    );
    c.register(poly_flag(Cmd::new("polyRotateUVsByVertex", "Cycle: each face's UVs move on one corner round it. Returns the nodes.", cycle_uvs)));
    c.register(
        poly_flag(Cmd::new("texSymmetrizeUVs", "Mirror the UVs named across the edge -edge: each sets the UV of the point opposite its own, across the line through the edge's UVs. Returns the nodes.", symmetrize))
            .flag(f("edge", "e", S, "The edge on the mesh's line of symmetry.")),
    );
    c.register(
        poly_flag(Cmd::new("texStraightenShell", "Straighten Shell: every UV edge of each shell named that runs within -angle of U or V runs along it. Returns the nodes.", straighten_shell))
            .flag(f("angle", "a", Fl, "Most an edge may lean and still be straightened, degrees (30).")),
    );
    c.register(
        Cmd::new("texSnapStackShells", "Snap and Stack: each shell with UVs picked moves, turns and scales so its first two UVs picked land on the first two of the shell of the last UV picked. Returns the nodes.", snap_stack)
            .positional("UVs, in order (the selection when none)"),
    );
    c.register(
        poly_flag(Cmd::new("texLayoutAlong", "Layout Along: the shells named in a row along U or V, -spacing apart, scaled together to fit 0 to 1. Returns the nodes.", layout_along))
            .flag(f("direction", "d", S, "u or v.").choices(&["u", "v"]))
            .flag(f("spacing", "sp", Fl, "Gap between shells and round them, in UV units (0.01).")),
    );
    c.register(
        Cmd::new("texDistributeShellsToTarget", "Distribute to Target: the shells evenly spaced on the line from the one furthest from the target (the shell of the last UV picked) to it. Returns the nodes.", distribute_to_target)
            .positional("UVs, faces or edges, the target's last (the selection when none)"),
    );
    c.register(
        poly_flag(Cmd::new("texDistributeUVs", "Distribute UVs: the UVs named evenly spaced along U or V between the first and last, or -spacing apart from the first. Returns the nodes.", distribute_uvs))
            .flag(f("direction", "d", S, "u or v.").choices(&["u", "v"]))
            .flag(f("spacing", "sp", Fl, "Distance between UVs; 0 (the default) spreads them between the ends.")),
    );
    c.register(
        poly_flag(Cmd::new("texCreateShell", "Create UV Shell: the faces named cut out as a shell of their own; -grid also lays their quads out as an even grid filling 0 to 1. Returns the nodes.", create_shell))
            .flag(f("grid", "g", ArgType::Bool, "Create Shell (Grid).")),
    );
}
