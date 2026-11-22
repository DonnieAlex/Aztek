#!/usr/bin/env bash
# Builds the Ubuntu package from a self-contained build: packaging/deb/build-deb.sh [build dir] [output dir]
# The bundle ./build.sh stages in <build dir>/Aztek is installed under /opt/aztek, with /usr/bin/aztek starting it, so
# the package carries its own Qt, Python and PySide6 and needs no Qt or Python from the system.
# Needs dpkg-dev and a finished ./build.sh; nothing is built here.
set -euo pipefail

SRC=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "${1:-$SRC/build}" "${2:-$SRC/dist}"
BUILD=$(cd "${1:-$SRC/build}" && pwd)
OUT=$(cd "${2:-$SRC/dist}" && pwd)
BUNDLE=$BUILD/Aztek
[ -x "$BUNDLE/AppRun" ] || { echo "no bundle in $BUNDLE: run ./build.sh first" >&2; exit 1; }

. /etc/os-release
VERSION=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$SRC/Cargo.toml" | head -n1)
ARCH=$(dpkg --print-architecture)
FILE="aztek_${VERSION}-1_ubuntu${VERSION_ID}_${ARCH}.deb"

ROOT=$BUILD/debroot
rm -rf "$ROOT"
mkdir -p "$ROOT/opt/aztek" "$ROOT/usr/bin" "$ROOT/usr/share/applications" "$ROOT/usr/share/icons" \
  "$ROOT/usr/share/doc/aztek" "$ROOT/DEBIAN"
cp -a "$BUNDLE/." "$ROOT/opt/aztek/"
install -m644 "$SRC/LICENSE" "$ROOT/usr/share/doc/aztek/copyright"
install -m644 "$SRC/THIRD_PARTY.md" "$ROOT/usr/share/doc/aztek/THIRD_PARTY.md"

# The names people run: the bundle's launcher, and aztekpy as the reference's mayapy.
cat >"$ROOT/usr/bin/aztek" <<'EOF'
#!/bin/sh
exec /opt/aztek/AppRun "$@"
EOF
cat >"$ROOT/usr/bin/aztekpy" <<'EOF'
#!/bin/sh
exec /opt/aztek/AppRun --python "$@"
EOF
cat >"$ROOT/usr/bin/harold" <<'EOF'
#!/bin/sh
exec /opt/aztek/usr/bin/harold "$@"
EOF
chmod 755 "$ROOT/usr/bin/aztek" "$ROOT/usr/bin/aztekpy" "$ROOT/usr/bin/harold"

sed 's|^Exec=.*|Exec=/opt/aztek/AppRun|' "$BUNDLE/usr/share/applications/aztek.desktop" \
  >"$ROOT/usr/share/applications/aztek.desktop"
cp -a "$BUNDLE/usr/share/icons/." "$ROOT/usr/share/icons/"

# dpkg-shlibdeps wants a debian/control beside it. The bundle's own libraries are found through -l and never become
# dependencies; what remains is the desktop the program and its libraries need.
WORK=$(mktemp -d)
mkdir "$WORK/debian"
printf 'Source: aztek\n' >"$WORK/debian/control"
SHLIBS=$(cd "$WORK" && dpkg-shlibdeps \
  -l"$ROOT/opt/aztek/usr/lib" -l"$ROOT/opt/aztek/usr/lib/python3.13/site-packages/PySide6/Qt/lib" \
  --ignore-missing-info -O "$ROOT/opt/aztek/usr/bin/aztek" | sed 's/^shlibs:Depends=//')
rm -rf "$WORK"

SIZE=$(du -sk "$ROOT" | cut -f1)
sed -e "s/@VERSION@/${VERSION}-1ubuntu${VERSION_ID}/" -e "s/@ARCH@/$ARCH/" -e "s/@SIZE@/$SIZE/" -e "s/@SHLIBS@/$SHLIBS/" \
  "$SRC/packaging/deb/control.in" >"$ROOT/DEBIAN/control"

dpkg-deb --root-owner-group --build "$ROOT" "$OUT/$FILE"
ls -l "$OUT/$FILE"
