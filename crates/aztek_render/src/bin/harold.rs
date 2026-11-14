//! `harold`: renders a Harold scene file (`.hss`) to an image, as the reference's renderer's command-line program
//! renders `.ass` files. Flags follow that program's: `-i` the scene, `-o` the image, `-r` width and height, `-as`
//! camera samples, `-t` threads, `-v` verbosity; `-dw` and `-dp` are accepted, as there is no window.

use std::sync::Arc;

const HELP: &str = "harold [-i] scene.hss [-o image.exr] [-r width height] [-as samples] [-t threads] [-v level]

Renders a Harold scene file. Images: .exr, .png, .jpg, .tif, .tga, .bmp, .hdr.
  -i  the scene file (or give it on its own)
  -o  the image; the scene's name with .exr when not given
  -r  width and height in pixels
  -as camera (AA) samples
  -t  threads; 0 is every core
  -v  0 quiet, 1 the summary (default), 2 also the warnings
  -dw, -dp  accepted: no window is opened and the render is not progressive";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("harold: {e}");
            std::process::exit(1);
        }
    }
}

fn number<T: std::str::FromStr>(args: &[String], at: usize, flag: &str) -> Result<T, String> {
    args.get(at).and_then(|v| v.parse().ok()).ok_or_else(|| format!("{flag} needs a number"))
}

fn run(args: &[String]) -> Result<(), String> {
    let (mut input, mut output) = (None::<String>, None::<String>);
    let (mut size, mut samples, mut threads, mut verbosity) = (None::<(u32, u32)>, None::<u32>, 0usize, 1u32);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("{HELP}");
                return Ok(());
            }
            "-i" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "-o" => {
                output = args.get(i + 1).cloned();
                i += 1;
            }
            "-r" => {
                size = Some((number(args, i + 1, "-r")?, number(args, i + 2, "-r")?));
                i += 2;
            }
            "-as" => {
                samples = Some(number(args, i + 1, "-as")?);
                i += 1;
            }
            "-t" => {
                threads = number(args, i + 1, "-t")?;
                i += 1;
            }
            "-v" => {
                verbosity = number(args, i + 1, "-v")?;
                i += 1;
            }
            "-dw" | "-dp" => {}
            a if a.starts_with('-') => return Err(format!("unknown flag {a}\n\n{HELP}")),
            a => input = Some(a.to_string()),
        }
        i += 1;
    }
    let input = input.ok_or_else(|| format!("no scene file given\n\n{HELP}"))?;
    let mut scene = aztek_render::hss::read(std::path::Path::new(&input))?;
    if let Some((w, h)) = size {
        scene.options.width = w.max(1);
        scene.options.height = h.max(1);
        scene.options.region = None;
    }
    if let Some(n) = samples {
        scene.options.aa_samples = n.max(1);
    }
    let output = output.unwrap_or_else(|| std::path::Path::new(&input).with_extension("exr").display().to_string());
    let out = std::path::Path::new(&output);
    let format = aztek_render::output::Format::of_path(out).ok_or_else(|| format!("{output}: no image format for that extension"))?;
    let (film, st) = aztek_render::render::render(Arc::new(scene), threads);
    if verbosity >= 1 {
        eprintln!("harold: {input}: {:.2} s, {} rays, {:.1} M rays/s, {:.0} MB", st.elapsed.as_secs_f64(), st.rays, st.rays_per_second / 1e6, st.memory_bytes as f64 / 1_048_576.0);
    }
    if verbosity >= 2 {
        for w in &st.warnings {
            eprintln!("harold: warning: {w}");
        }
    }
    let files = aztek_render::output::write_film(&film, out, format, false, &[])?;
    if verbosity >= 1 {
        for f in files {
            eprintln!("harold: wrote {f}");
        }
    }
    Ok(())
}
