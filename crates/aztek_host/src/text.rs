//! The viewport's text: glyphs of a real font (DejaVu Sans, shipped with Aztek), drawn as anti-aliased pixels so the
//! heads up display, camera names and labels read on any screen. Same calls as the line font they replace: `size` is
//! the height of a capital, `origin` the start of the baseline, in screen pixels with y down.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use glam::DVec2;

use crate::render::LineVertex;

const FONT: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");

fn font() -> &'static FontRef<'static> {
    static F: OnceLock<FontRef<'static>> = OnceLock::new();
    F.get_or_init(|| FontRef::try_from_slice(FONT).expect("the viewport font"))
}

/// A glyph's covered pixels at one size: their place from the pen on the baseline, and how much each is covered.
type Coverage = (i32, i32, Vec<(i32, i32, f32)>);

fn cache() -> &'static Mutex<HashMap<(char, u32), Coverage>> {
    static C: OnceLock<Mutex<HashMap<(char, u32), Coverage>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn scale(size: f64) -> PxScale {
    // The size asked is a capital's height; the font's scale is its whole height, ascender to descender.
    static CAP: OnceLock<f32> = OnceLock::new();
    let cap = *CAP.get_or_init(|| {
        let f = font();
        f.outline_glyph(f.glyph_id('H').with_scale(PxScale::from(100.0))).map_or(0.73, |o| o.px_bounds().height() / 100.0)
    });
    PxScale::from((size / cap.max(0.1) as f64) as f32)
}

/// How wide the text is drawn.
pub fn width(text: &str, size: f64) -> f64 {
    let f = font().as_scaled(scale(size));
    let mut w = 0.0f32;
    let mut last = None;
    for c in text.chars() {
        let g = f.glyph_id(c);
        if let Some(p) = last {
            w += f.kern(p, g);
        }
        w += f.h_advance(g);
        last = Some(g);
    }
    w as f64
}

fn coverage(c: char, s: PxScale) -> Coverage {
    let key = (c, (s.y * 8.0).round() as u32);
    if let Some(v) = cache().lock().unwrap().get(&key) {
        return v.clone();
    }
    let f = font();
    let g = f.glyph_id(c).with_scale_and_position(s, ab_glyph::point(0.0, 0.0));
    let out = match f.outline_glyph(g) {
        Some(o) => {
            let b = o.px_bounds();
            let mut px = Vec::new();
            o.draw(|x, y, v| {
                if v > 0.02 {
                    px.push((x as i32, y as i32, v));
                }
            });
            (b.min.x as i32, b.min.y as i32, px)
        }
        None => (0, 0, Vec::new()),
    };
    cache().lock().unwrap().insert(key, out.clone());
    out
}

/// The text as triangles, two to a covered pixel, coloured with the coverage as alpha.
pub fn draw(text: &str, origin: DVec2, size: f64, color: [f32; 4], out: &mut Vec<LineVertex>) {
    let s = scale(size);
    let f = font().as_scaled(s);
    let mut x = origin.x.round() as f32;
    let y = origin.y.round() as i32;
    let mut last = None;
    for c in text.chars() {
        let g = f.glyph_id(c);
        if let Some(p) = last {
            x += f.kern(p, g);
        }
        let (ox, oy, px) = coverage(c, s);
        let pen = x.round() as i32;
        for (dx, dy, v) in px {
            let (x0, y0) = ((pen + ox + dx) as f32, (y + oy + dy) as f32);
            // Coverage a little heavier than linear, so thin stems at small sizes don't wash out to grey.
            let col = [color[0], color[1], color[2], color[3] * v.min(1.0).powf(0.75)];
            for (a, b) in [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
                out.push(LineVertex { pos: [x0 + a, y0 + b, 0.5], color: col });
            }
        }
        x += f.h_advance(g);
        last = Some(g);
    }
}

/// Text over the scene: a soft dark edge under it, so it reads on a light background as well as a dark one.
pub fn label(text: &str, origin: DVec2, size: f64, color: [f32; 4], out: &mut Vec<LineVertex>) {
    let shadow = [0.0, 0.0, 0.0, 0.3 * color[3]];
    draw(text, origin + DVec2::new(1.0, 1.0), size, shadow, out);
    draw(text, origin, size, color, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_covers_pixels_near_its_baseline_and_grows_with_its_size() {
        let mut v = Vec::new();
        draw("Verts: 382", DVec2::new(10.0, 30.0), 9.0, [1.0; 4], &mut v);
        assert!(!v.is_empty());
        let (lo, hi) = v.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(p.pos[1]), hi.max(p.pos[1])));
        // Capitals 9 pixels tall stand on the baseline at 30.
        assert!(lo >= 19.0 && lo <= 22.0, "{lo}");
        assert!(hi <= 31.5, "{hi}");
        assert!(width("Verts: 382", 18.0) > 1.8 * width("Verts: 382", 9.0));
    }
}
