//! Python 3.12 only, the reference's version: a build against any other stops here, naming the one it found.

fn main() {
    // The build's date and stamp, for about -date and -cutIdentifier.
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    let (y, m, d) = civil(secs.div_euclid(86_400));
    let r = secs.rem_euclid(86_400);
    println!("cargo:rustc-env=AZTEK_BUILD_DATE={y:04}/{m:02}/{d:02}");
    println!("cargo:rustc-env=AZTEK_BUILD_STAMP={y:04}{m:02}{d:02}{:02}{:02}", r / 3600, r / 60 % 60);
    #[cfg(feature = "python")]
    {
        let v = pyo3_build_config::get().version();
        if (v.major, v.minor) != (3, 12) {
            panic!(
                "Aztek embeds Python 3.12, and pyo3 found Python {}.{}. Point PYO3_PYTHON at a python3.12 (./build.sh --deps \
                 installs one, tools/python/fetch-python.sh fetches one), or build with --no-default-features.",
                v.major, v.minor
            );
        }
    }
}

/// Days since 1970 to a civil date.
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + if m <= 2 { 1 } else { 0 }, m, d)
}
