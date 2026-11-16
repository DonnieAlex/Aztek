//! Arranging UV shells, the UV Toolkit's Arrange and Layout, and Align and Snap tools (#108): texOrientShells,
//! texOrientEdge, texStackShells, texUnstackShells, texGatherShells, texDistributeShells, texAlignShells,
//! texSnapShells, texRandomizeShells and texLinearAlignUVs. The command names are the reference's; their flags are
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

pub fn register(c: &mut Commands<Session>) {
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
