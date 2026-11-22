#!/usr/bin/env bash
# Checks that a staged Linux app is self-contained: its Python, PySide6, shiboken6 and Qt all come from the tree, and no
# distribution site-packages is on its path. Needs no display.
#   tools/check_bundle.sh [appdir]
set -euo pipefail

APP=${1:-build/Aztek}
[ -x "$APP/AppRun" ] || { echo "no AppRun in $APP (run ./build.sh first)" >&2; exit 1; }
APP=$(cd "$APP" && pwd)
CHECK=$(mktemp "${TMPDIR:-/tmp}/aztek-bundle-XXXXXX.py")
trap 'rm -f "$CHECK"' EXIT
cat >"$CHECK" <<'PY'
import sys

import PySide6
import shiboken6
from PySide6 import QtCore

root = sys.prefix  # .../usr
problems = []
if sys.version_info[:2] != (3, 13):
    problems.append("Python %d.%d, not 3.13" % sys.version_info[:2])
for name, mod in (("PySide6", PySide6), ("shiboken6", shiboken6)):
    if not mod.__file__.startswith(root):
        problems.append("%s comes from %s" % (name, mod.__file__))
if PySide6.__version__ != shiboken6.__version__:
    problems.append("PySide6 %s and shiboken6 %s differ" % (PySide6.__version__, shiboken6.__version__))
if QtCore.qVersion().rsplit(".", 1)[0] != PySide6.__version__.rsplit(".", 1)[0]:
    problems.append("PySide6 %s is for Qt %s" % (PySide6.__version__, QtCore.qVersion()))
maps = open("/proc/self/maps").read()
for lib in ("libpython3.13", "libQt6Core", "libshiboken6"):
    paths = {line.split()[-1] for line in maps.splitlines() if lib in line}
    if not paths:
        problems.append("%s is not loaded" % lib)
    for path in paths:
        if not path.startswith(root):
            problems.append("%s loaded from %s" % (lib, path))
for path in sys.path:
    if path.startswith("/usr/lib/python") or path.startswith("/usr/local/lib/python"):
        problems.append("distribution site-packages on the path: %s" % path)
if problems:
    print("bundle check failed:")
    for problem in problems:
        print("  " + problem)
    sys.exit(1)
print("bundle check: Python %s, PySide6 %s, shiboken6 %s, Qt %s, all inside %s"
      % (sys.version.split()[0], PySide6.__version__, shiboken6.__version__, QtCore.qVersion(), root))
PY

env -u LD_LIBRARY_PATH -u PYTHONHOME -u PYTHONPATH "$APP/AppRun" --batch --script "$CHECK"
