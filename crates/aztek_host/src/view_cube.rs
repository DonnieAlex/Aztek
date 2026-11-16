//! The ViewCube, as the reference has it in a corner of each view: a cube turned with the camera, its faces named.
//! A click on a face looks straight at it, on an edge or corner from between its faces; a drag turns the camera; the
//! house goes to the camera's home and the triangle opens the cube's menu. viewManip sets whether it shows, its corner
//! and size, and the compass under it.

use std::cell::RefCell;
use std::collections::HashMap;

use aztek_scene::world::to_world;
use glam::{DMat3, DVec2, DVec3, IVec3};
use serde_json::Value;

use crate::camera::Camera;

/// What a point on the cube's area is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// A face, edge or corner: each part -1, 0 or 1 along the cube's own X, Y and Z (Y up, Z front).
    Region(IVec3),
    Home,
    Menu,
}

/// The cube as a view shows it.
pub struct Cube {
    pub center: DVec2,
    pub half: f64,
    /// World to view rotation.
    rot: DMat3,
    /// The turn about up that makes a face the front (Set Current View as Front), degrees.
    front: f64,
}

thread_local! {
    /// Each camera's home, by camera name (Set Current View as Home).
    static HOMES: RefCell<HashMap<String, Camera>> = RefCell::new(HashMap::new());
    /// Set Current View as Front's turn, degrees.
    static FRONT: RefCell<f64> = const { RefCell::new(0.0) };
}

fn setting(key: &str) -> Value {
    crate::ctx_cmds::setting("viewManip", key)
}

/// The region whose view is nearest a direction from the centre of interest to the eye.
pub fn nearest_region(cube: &Cube, to_eye: DVec3) -> IVec3 {
    let mut best = (IVec3::new(0, 0, 1), -2.0);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let r = IVec3::new(x, y, z);
                if r == IVec3::ZERO {
                    continue;
                }
                let d = cube.direction(r).dot(to_eye.normalize_or_zero());
                if d > best.1 {
                    best = (r, d);
                }
            }
        }
    }
    best.0
}

/// A viewManip switch, or its default.
pub fn flag(key: &str) -> bool {
    setting(key).as_bool().unwrap_or(false)
}

pub fn visible() -> bool {
    setting("visible").as_bool().unwrap_or(true)
}

pub fn front() -> f64 {
    FRONT.with(|f| *f.borrow())
}

pub fn set_front(deg: f64) {
    FRONT.with(|f| *f.borrow_mut() = deg);
}

pub fn home_of(cam: &str) -> Option<Camera> {
    HOMES.with(|h| h.borrow().get(cam).cloned())
}

pub fn set_home(cam: &str, c: Option<Camera>) {
    HOMES.with(|h| match c {
        Some(c) => {
            h.borrow_mut().insert(cam.to_string(), c);
        }
        None => {
            h.borrow_mut().remove(cam);
        }
    });
}

/// The cube's own axis directions in the world: Y up and Z front, as the up axis and the front turn have them.
fn frame(front: f64) -> [DVec3; 3] {
    let (s, c) = front.to_radians().sin_cos();
    let turn = |v: DVec3| DVec3::new(v.x * c + v.z * s, v.y, -v.x * s + v.z * c);
    [to_world(turn(DVec3::X)), to_world(turn(DVec3::Y)), to_world(turn(DVec3::Z))]
}

/// Where the cube goes in a view w by h pixels, or None when it is hidden.
pub fn layout(w: f64, h: f64, dpr: f64, cam: &Camera) -> Option<Cube> {
    if !visible() || w < 120.0 * dpr || h < 120.0 * dpr {
        return None;
    }
    let size = match setting("size").as_str().unwrap_or("normal") {
        "tiny" => 0.5,
        "small" => 0.75,
        "large" => 1.4,
        _ => 1.0,
    };
    let half = 24.0 * dpr * size;
    let margin = half * 2.4;
    let on = |k: &str| setting(k).as_bool().unwrap_or(false);
    let x = if on("topLeft") || on("bottomLeft") { margin } else { w - margin };
    let y = if on("bottomLeft") || on("bottomRight") { h - margin } else { margin };
    let v = cam.view();
    let rot = DMat3::from_cols(v.x_axis.truncate(), v.y_axis.truncate(), v.z_axis.truncate());
    Some(Cube { center: DVec2::new(x, y), half, rot, front: front() })
}

/// Face names along the cube's axes.
fn label(axis: usize, sign: i32) -> &'static str {
    match (axis, sign > 0) {
        (0, true) => "RIGHT",
        (0, false) => "LEFT",
        (1, true) => "TOP",
        (1, false) => "BOTTOM",
        (2, true) => "FRONT",
        _ => "BACK",
    }
}

impl Cube {
    fn axes(&self) -> [DVec3; 3] {
        frame(self.front)
    }

    /// A point of the cube, in its own coordinates (-1 to 1), on the screen, and how near the viewer it is.
    fn screen(&self, local: DVec3) -> (DVec2, f64) {
        let a = self.axes();
        let world = a[0] * local.x + a[1] * local.y + a[2] * local.z;
        let q = self.rot * world;
        (self.center + DVec2::new(q.x, -q.y) * self.half, q.z)
    }

    /// The faces turned to the viewer: axis, sign, and how squarely.
    fn faces(&self) -> Vec<(usize, i32, f64)> {
        let a = self.axes();
        let mut out = Vec::new();
        for (i, ax) in a.iter().enumerate() {
            for sign in [1, -1] {
                let z = (self.rot * (*ax * sign as f64)).z;
                if z > 1e-3 {
                    out.push((i, sign, z));
                }
            }
        }
        out
    }

    fn unit(i: usize) -> DVec3 {
        [DVec3::X, DVec3::Y, DVec3::Z][i]
    }

    /// The point on a face at (u, w), each -1 to 1.
    fn on_face(axis: usize, sign: i32, u: f64, w: f64) -> DVec3 {
        let (b, c) = ((axis + 1) % 3, (axis + 2) % 3);
        Self::unit(axis) * sign as f64 + Self::unit(b) * u + Self::unit(c) * w
    }

    pub fn home_at(&self) -> DVec2 {
        self.center + DVec2::new(-1.75, -1.75) * self.half
    }

    pub fn menu_at(&self) -> DVec2 {
        self.center + DVec2::new(1.65, 1.65) * self.half
    }

    /// What is under a pixel.
    pub fn hit(&self, px: DVec2) -> Option<Part> {
        if (px - self.home_at()).abs().max_element() < self.half * 0.35 {
            return Some(Part::Home);
        }
        if (px - self.menu_at()).abs().max_element() < self.half * 0.3 {
            return Some(Part::Menu);
        }
        for (axis, sign, _) in self.faces() {
            // The face's corner and sides on the screen; (u, w) from the pixel.
            let (o, _) = self.screen(Self::on_face(axis, sign, 0.0, 0.0));
            let (pu, _) = self.screen(Self::on_face(axis, sign, 1.0, 0.0));
            let (pw, _) = self.screen(Self::on_face(axis, sign, 0.0, 1.0));
            let (eu, ew) = (pu - o, pw - o);
            let det = eu.x * ew.y - eu.y * ew.x;
            if det.abs() < 1e-9 {
                continue;
            }
            let d = px - o;
            let u = (d.x * ew.y - d.y * ew.x) / det;
            let w = (eu.x * d.y - eu.y * d.x) / det;
            if u.abs() <= 1.0 && w.abs() <= 1.0 {
                let cell = |t: f64| if t < -0.6 { -1 } else if t > 0.6 { 1 } else { 0 };
                let mut r = IVec3::ZERO;
                r[axis] = sign;
                r[(axis + 1) % 3] = cell(u);
                r[(axis + 2) % 3] = cell(w);
                return Some(Part::Region(r));
            }
        }
        None
    }

    /// Whether a pixel is over the cube's corner of the view at all: the cube, its house and its menu.
    pub fn over(&self, px: DVec2) -> bool {
        (px - self.center).abs().max_element() < self.half * 2.1
    }

    /// The world direction from the centre of interest to the eye that looks at a region.
    pub fn direction(&self, r: IVec3) -> DVec3 {
        let a = self.axes();
        (a[0] * r.x as f64 + a[1] * r.y as f64 + a[2] * r.z as f64).normalize_or_zero()
    }

    /// Draws the cube: triangles and line segments in pixels, and its words through `text` (the words, where their
    /// baseline starts, their size, their colour).
    #[allow(clippy::type_complexity)]
    pub fn draw(&self, hover: Option<Part>, active: bool, tri: &mut dyn FnMut(DVec2, DVec2, DVec2, [u8; 4]), seg: &mut dyn FnMut(DVec2, DVec2, [u8; 4]), text: &mut dyn FnMut(&str, DVec2, f64, [u8; 4]), font_size: f64) {
        // Away from the mouse as opaque as -minOpacity says; under it, fully.
        let min = setting("minOpacity").as_f64().unwrap_or(0.5).clamp(0.0, 1.0);
        let alpha = if active { 255 } else { (min * 255.0).round() as u8 };
        let edges = [-1.0, -0.6, 0.6, 1.0];
        let compass = setting("drawCompass").as_bool().unwrap_or(false);
        if compass {
            // A ring on the ground under the cube, with north (the back) marked.
            let n = 48;
            let ring = |t: f64| DVec3::new(t.cos() * 1.9, -1.0, t.sin() * 1.9);
            for k in 0..n {
                let (a0, a1) = (k as f64 / n as f64 * std::f64::consts::TAU, (k + 1) as f64 / n as f64 * std::f64::consts::TAU);
                seg(self.screen(ring(a0)).0, self.screen(ring(a1)).0, [200, 200, 200, alpha]);
            }
            let (at, _) = self.screen(DVec3::new(0.0, -1.0, -2.3));
            text("N", at - DVec2::new(crate::text::width("N", font_size) * 0.5, -font_size * 0.5), font_size, [220, 220, 220, alpha]);
        }
        for (axis, sign, facing) in self.faces() {
            for i in 0..3 {
                for j in 0..3 {
                    let (u0, u1, w0, w1) = (edges[i], edges[i + 1], edges[j], edges[j + 1]);
                    let mut r = IVec3::ZERO;
                    r[axis] = sign;
                    r[(axis + 1) % 3] = i as i32 - 1;
                    r[(axis + 2) % 3] = j as i32 - 1;
                    let lit = hover == Some(Part::Region(r));
                    let shade = (170.0 + 60.0 * facing) as u8;
                    let col = if lit { [110, 170, 235, alpha] } else { [shade, shade, shade.saturating_add(6), alpha] };
                    let q = [(u0, w0), (u1, w0), (u1, w1), (u0, w1)].map(|(u, w)| self.screen(Self::on_face(axis, sign, u, w)).0);
                    tri(q[0], q[1], q[2], col);
                    tri(q[0], q[2], q[3], col);
                }
            }
            let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].map(|(u, w)| self.screen(Self::on_face(axis, sign, u, w)).0);
            for k in 0..4 {
                seg(corners[k], corners[(k + 1) % 4], [60, 60, 66, alpha]);
            }
            if facing > 0.3 {
                let name = label(axis, sign);
                let size = font_size * (0.55 + 0.45 * facing);
                let (at, _) = self.screen(Self::on_face(axis, sign, 0.0, 0.0));
                let start = at + DVec2::new(-crate::text::width(name, size) * 0.5, size * 0.5);
                text(name, start, size, [35, 35, 40, alpha]);
            }
        }
        // The house: home.
        let h = self.home_at();
        let k = self.half * 0.22;
        let col = if hover == Some(Part::Home) { [110, 170, 235, 255] } else { [210, 210, 210, alpha] };
        let roof = [h + DVec2::new(-k * 1.3, 0.0), h + DVec2::new(0.0, -k * 1.2), h + DVec2::new(k * 1.3, 0.0)];
        seg(roof[0], roof[1], col);
        seg(roof[1], roof[2], col);
        let body = [h + DVec2::new(-k, 0.0), h + DVec2::new(-k, k), h + DVec2::new(k, k), h + DVec2::new(k, 0.0)];
        for i in 0..3 {
            seg(body[i], body[i + 1], col);
        }
        // The triangle: the menu.
        let m = self.menu_at();
        let col = if hover == Some(Part::Menu) { [110, 170, 235, 255] } else { [210, 210, 210, alpha] };
        let k = self.half * 0.2;
        tri(m + DVec2::new(-k, -k * 0.5), m + DVec2::new(k, -k * 0.5), m + DVec2::new(0.0, k * 0.7), col);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::CamKind;

    #[test]
    fn a_click_on_the_cube_names_what_it_looks_at() {
        let cam = Camera::new(CamKind::Persp);
        let cube = layout(800.0, 600.0, 1.0, &cam).unwrap();
        // The middle of the cube, seen from front right above: the corner between front, right and top.
        assert_eq!(cube.hit(cube.center), Some(Part::Region(IVec3::new(1, 1, 1))));
        assert_eq!(cube.hit(cube.home_at()), Some(Part::Home));
        assert_eq!(cube.hit(cube.menu_at()), Some(Part::Menu));
        assert_eq!(cube.hit(cube.center + DVec2::new(cube.half * 3.0, 0.0)), None);
        // Straight at the front face from the front camera.
        let front = Camera::new(CamKind::Front);
        let cube = layout(800.0, 600.0, 1.0, &front).unwrap();
        assert_eq!(cube.hit(cube.center), Some(Part::Region(IVec3::new(0, 0, 1))));
        assert!((cube.direction(IVec3::new(0, 0, 1)) - DVec3::Z).length() < 1e-9);
        assert!((cube.direction(IVec3::new(1, 1, 0)) - DVec3::new(1.0, 1.0, 0.0).normalize()).length() < 1e-9);
        // Set Current View as Front turns which way is the front.
        set_front(90.0);
        let cube = layout(800.0, 600.0, 1.0, &front).unwrap();
        assert!((cube.direction(IVec3::new(0, 0, 1)) - DVec3::X).length() < 1e-9);
        set_front(0.0);
    }
}
