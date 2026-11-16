//! Python 3.12 only, the reference's version: a build against any other stops here, naming the one it found.

fn main() {
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
