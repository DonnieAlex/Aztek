#!/usr/bin/env bash
# PySide6 and shiboken6 for the bundled Python, the release built for the Qt the app runs. PySide6 requires its exact
# shiboken6 and stops the program as it loads when the two differ, so both are installed here, together, at the Qt
# version. Qt's own libraries that come in the wheel are left for the staging step to trim against the bundled Qt.
#   tools/python/fetch-pyside.sh <python> <qt-version> <site-packages>
set -euo pipefail

PY=${1:?usage: fetch-pyside.sh <python> <qt-version> <site-packages>}
VER=${2:?usage: fetch-pyside.sh <python> <qt-version> <site-packages>}
DEST=${3:?usage: fetch-pyside.sh <python> <qt-version> <site-packages>}

have() {
  [ -f "$DEST/PySide6/_config.py" ] && [ -f "$DEST/shiboken6/_config.py" ] &&
    grep -q "^version = \"$VER\"" "$DEST/PySide6/_config.py" &&
    grep -q "^version = \"$VER\"" "$DEST/shiboken6/_config.py"
}
if have; then
  exit 0
fi

mkdir -p "$DEST"
if [ -d "$DEST/PySide6" ] || [ -d "$DEST/shiboken6" ]; then
  rm -rf "$DEST/PySide6" "$DEST/shiboken6"
  rm -rf "$DEST"/PySide6_Essentials-*.dist-info "$DEST"/shiboken6-*.dist-info
fi
"$PY" -m pip install --quiet --no-deps --target "$DEST" "PySide6-Essentials==$VER" "shiboken6==$VER"

# The two must be the same version as each other and as the Qt the app links.
GOT=$(PYTHONPATH="$DEST" "$PY" -c 'import PySide6, shiboken6; print(PySide6.__version__, shiboken6.__version__)')
[ "$GOT" = "$VER $VER" ] || {
  echo "PySide6 and shiboken6 must both be $VER; got $GOT" >&2
  exit 1
}
