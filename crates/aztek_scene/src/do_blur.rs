//! doBlur: 2.5D motion blur of a colour image along a motion vector image, written beside it as name_blur.ext, as the
//! reference's blur2d does. The vector image holds each pixel's screen motion in its red (x, right) and green (y, down)
//! channels, mid grey being still and the full range 256 pixels either way.

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::session::Session;

/// The image a name stands for: as given, or with an image extension added.
fn find_image(name: &str) -> Option<std::path::PathBuf> {
    let p = std::path::PathBuf::from(name);
    if p.is_file() {
        return Some(p);
    }
    ["iff", "png", "tif", "tiff", "jpg", "jpeg", "tga", "bmp"].iter().map(|e| std::path::PathBuf::from(format!("{name}.{e}"))).find(|q| q.is_file())
}

/// Blurs `rgba` (w by h) along `motion` (pixels, x then y per pixel), the vectors scaled by `length`; `sharpness` narrows
/// the filter, `smooth` (1 to 5) is the size of the box that softens the alpha (and the colour with `smooth_color`)
/// afterwards. Rows go in strips that fit `cap` bytes.
pub fn blur(rgba: &[f32], motion: &[[f32; 2]], w: usize, h: usize, length: f32, sharpness: f32, smooth: f32, smooth_color: bool, cap: usize) -> Vec<f32> {
    let mut out = vec![0f32; w * h * 4];
    let strip = (cap / (w.max(1) * 16)).clamp(1, h.max(1));
    let mut y0 = 0;
    while y0 < h {
        let y1 = (y0 + strip).min(h);
        for y in y0..y1 {
            for x in 0..w {
                let m = motion[y * w + x];
                let (dx, dy) = (m[0] * length, m[1] * length);
                let reach = (dx.abs().max(dy.abs())).ceil() as i32;
                let n = (reach * 2 + 1).max(1);
                let mut acc = [0f32; 4];
                let mut total = 0f32;
                for k in 0..n {
                    // Along the vector, centred on the pixel.
                    let t = if n == 1 { 0.0 } else { k as f32 / (n - 1) as f32 - 0.5 };
                    let wgt = (1.0 - 2.0 * t.abs()).max(0.0).powf(sharpness.max(0.0)).max(1e-6);
                    let (sx, sy) = ((x as f32 + dx * t).round() as i32, (y as f32 + dy * t).round() as i32);
                    if sx < 0 || sy < 0 || sx >= w as i32 || sy >= h as i32 {
                        continue;
                    }
                    let at = (sy as usize * w + sx as usize) * 4;
                    for c in 0..4 {
                        acc[c] += rgba[at + c] * wgt;
                    }
                    total += wgt;
                }
                let at = (y * w + x) * 4;
                for c in 0..4 {
                    out[at + c] = if total > 0.0 { acc[c] / total } else { rgba[at + c] };
                }
            }
        }
        y0 = y1;
    }
    let size = smooth.clamp(1.0, 5.0).round() as i32;
    if size > 1 {
        let r = size / 2;
        let src = out.clone();
        let channels: Vec<usize> = if smooth_color { vec![0, 1, 2, 3] } else { vec![3] };
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                for &c in &channels {
                    let (mut s, mut n) = (0f32, 0f32);
                    for j in -r..=r {
                        for i in -r..=r {
                            let (sx, sy) = (x + i, y + j);
                            if sx >= 0 && sy >= 0 && sx < w as i32 && sy < h as i32 {
                                s += src[(sy as usize * w + sx as usize) * 4 + c];
                                n += 1.0;
                            }
                        }
                    }
                    out[(y as usize * w + x as usize) * 4 + c] = s / n.max(1.0);
                }
            }
        }
    }
    out
}

fn do_blur(_s: &mut Session, a: &Args) -> CmdResult {
    let color_name = a.get("colorFile").and_then(Value::as_str).or_else(|| a.pos_str(0)).map(str::to_string);
    let Some(color_name) = color_name else { fail!("doBlur: give the colour image with -c") };
    let Some(vector_name) = a.get("vectorFile").and_then(Value::as_str).map(str::to_string) else { fail!("doBlur: give the motion vector image with -v") };
    let Some(color_path) = find_image(&color_name) else { fail!("doBlur: no image {color_name}") };
    let Some(vector_path) = find_image(&vector_name) else { fail!("doBlur: no motion vector image {vector_name}") };
    let read = |p: &std::path::Path| image::open(p).map_err(|e| aztek_ops::CommandError::new(format!("doBlur: {}: {e}", p.display())));
    let color = read(&color_path)?.to_rgba32f();
    let vectors = read(&vector_path)?.to_rgba32f();
    let (w, h) = (color.width() as usize, color.height() as usize);
    if (vectors.width() as usize, vectors.height() as usize) != (w, h) {
        fail!("doBlur: the motion vectors are {}x{}, the image {w}x{h}", vectors.width(), vectors.height());
    }
    let motion: Vec<[f32; 2]> = vectors.pixels().map(|p| [(p[0] - 0.5) * 256.0, (p[1] - 0.5) * 256.0]).collect();
    let cap = a.get("memCapSize").and_then(Value::as_f64).filter(|x| *x > 0.0).map_or(usize::MAX, |x| x as usize);
    let out = blur(color.as_raw(), &motion, w, h, a.float("length", 1.0) as f32, a.float("sharpness", 1.0) as f32, a.float("smooth", 1.0) as f32, a.bool("smoothColor"), cap);
    // name_blur.ext, beside the colour image; the name can't be chosen.
    let stem = color_path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = color_path.extension().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "png".into());
    let ext = if matches!(ext.as_str(), "png" | "tif" | "tiff" | "jpg" | "jpeg" | "tga" | "bmp") { ext } else { "png".into() };
    let dest = color_path.with_file_name(format!("{stem}_blur.{ext}"));
    let bytes: Vec<u8> = out.iter().map(|v| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8).collect();
    let saved = if matches!(ext.as_str(), "jpg" | "jpeg") {
        let rgb: Vec<u8> = bytes.chunks(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
        image::save_buffer(&dest, &rgb, w as u32, h as u32, image::ExtendedColorType::Rgb8)
    } else {
        image::save_buffer(&dest, &bytes, w as u32, h as u32, image::ExtendedColorType::Rgba8)
    };
    saved.map_err(|e| aztek_ops::CommandError::new(format!("doBlur: {}: {e}", dest.display())))?;
    Ok(json!(dest.to_string_lossy().replace('\\', "/")))
}

fn f(name: &str, short: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(name, short, ty, Value::Null, doc)
}

pub(crate) fn register(c: &mut Commands<Session>) {
    c.register(
        CommandDef::<Session>::new(
            "doBlur",
            "2.5D motion blur: blurs the -c colour image along the -v motion vector image (red and green the screen motion, mid grey \
             still, 256 pixels each way) and writes name_blur.ext beside it. Returns that file.",
            do_blur,
        )
        .flag(f("colorFile", "c", ArgType::String, "The colour image to blur."))
        .flag(f("vectorFile", "v", ArgType::String, "The motion vector image."))
        .flag(f("length", "l", ArgType::Float, "Scales the motion vectors, 0 and up (1)."))
        .flag(f("sharpness", "s", ArgType::Float, "The filter's shape, 0 and up: higher, a narrower, sharper blur (1)."))
        .flag(f("smooth", "m", ArgType::Float, "The box that smooths the result, 1 to 5: higher softens the alpha's edges more (1)."))
        .flag(f("smoothColor", "r", ArgType::Bool, "Smooth the colour as well as the alpha."))
        .flag(f("memCapSize", "o", ArgType::Float, "How many bytes to work in at once; the image goes in strips that fit."))
        .undoable(false),
    );
}
