//! Times the poly tools on a dense mesh, one command per run so a slow one can be stopped from outside:
//!
//!     cargo run --release -p aztek_host --example poly_bench -- <faces> "<setup MEL>" "<command MEL>"
//!
//! The mesh is `m`, a sphere of about <faces> quads, made and evaluated before the clock starts; the setup runs next
//! (selections, a second mesh), untimed; then the command, and the scene evaluated after it, are timed. Prints
//! `ms faces-after`. tools/poly_bench.sh runs the whole list.
use aztek_host::Core;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let faces: f64 = args.first().and_then(|a| a.parse().ok()).unwrap_or(250_000.0);
    let setup = args.get(1).cloned().unwrap_or_default();
    let cmd = args.get(2).cloned().unwrap_or_default();
    let n = (faces / 2.0).sqrt().round().max(4.0) as i64;
    let mut c = Core::new();
    let t = Instant::now();
    c.mel_checked(&format!("polySphere -n m -r 10 -sx {} -sy {};", 2 * n, n)).unwrap();
    let count = |c: &mut Core| c.mel_checked("polyEvaluate -f m;").ok().and_then(|v| v.as_i64().or_else(|| v.get(0).and_then(|x| x.as_i64()))).unwrap_or(-1);
    let before = count(&mut c);
    eprintln!("made {before} faces in {:.0} ms", t.elapsed().as_secs_f64() * 1e3);
    if !setup.is_empty() {
        if let Err(e) = c.mel_checked(&setup) {
            eprintln!("setup: {e}");
        }
    }
    let t = Instant::now();
    let r = c.mel_checked(&cmd);
    c.s.graph.evaluate();
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let after = count(&mut c);
    match r {
        Ok(_) => println!("{ms:.0} {after}"),
        Err(e) => println!("{ms:.0} error: {e}"),
    }
}
