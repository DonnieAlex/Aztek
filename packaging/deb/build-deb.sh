#!/usr/bin/env bash
# Builds the Ubuntu package with dpkg-deb: packaging/deb/build-deb.sh [build dir] [output dir]
# Needs the packages from ./build.sh --deps, dpkg-dev and cargo on PATH. A build already in that folder is reused.
set -euo pipefail

SRC=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "${1:-$SRC/build}" "${2:-$SRC/dist}"
BUILD=$(cd "${1:-$SRC/build}" && pwd)
OUT=$(cd "${2:-$SRC/dist}" && pwd)

. /etc/os-release
VERSION=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$SRC/Cargo.toml" | head -n1)
ARCH=$(dpkg --print-architecture)
FILE="aztek_${VERSION}-1_ubuntu${VERSION_ID}_${ARCH}.deb"

# Python 3.13, the only version Aztek embeds: the system's where Ubuntu has it (24.04), else one inside the package in
# /usr/lib/aztek/python (24.04 has 3.12, 26.04 3.14).
if [ -x /usr/bin/python3.13 ] && [ -f /usr/include/python3.13/Python.h ]; then
  export PYO3_PYTHON=/usr/bin/python3.13
  PYARGS=(-DPython3_EXECUTABLE=/usr/bin/python3.13 -DAZTEK_PYTHON_BUNDLE=)
else
  BUNDLE=$("$SRC/tools/python/fetch-python.sh" "$BUILD/python")
  export PYO3_PYTHON="$BUNDLE/bin/python3.13"
  PYARGS=(-DAZTEK_PYTHON_BUNDLE="$BUNDLE")
fi
cmake -S "$SRC" -B "$BUILD" -G Ninja -DCMAKE_BUILD_TYPE=Release "${PYARGS[@]}"
cmake --build "$BUILD"

ROOT=$BUILD/debroot
rm -rf "$ROOT"
DESTDIR=$ROOT cmake --install "$BUILD" --prefix /usr --strip
mkdir -p "$ROOT/usr/share/aztek/examples" "$ROOT/usr/share/doc/aztek" "$ROOT/DEBIAN"
install -m644 "$SRC/LICENSE" "$ROOT/usr/share/doc/aztek/copyright"
PRIVATE=()
if [ -d "$ROOT/usr/lib/aztek/python/lib" ]; then
  # The bundled Python comes with its symbols: stripped, it is a fraction of the size.
  find "$ROOT/usr/lib/aztek/python/lib" -name '*.so*' -type f -exec strip --strip-unneeded {} +
  PRIVATE=(-l"$ROOT/usr/lib/aztek/python/lib")
fi

# dpkg-shlibdeps wants a debian/control beside it, and only looks at the binary. What is loaded with dlopen goes in
# control.in: libvulkan1 for the viewport, qt6-qpa-plugins for the xcb platform plugin.
WORK=$(mktemp -d)
mkdir "$WORK/debian"
printf 'Source: aztek\n' >"$WORK/debian/control"
# A bundled libpython is the package's own: no other package provides it.
SHLIBS=$(cd "$WORK" && dpkg-shlibdeps "${PRIVATE[@]}" --ignore-missing-info -O "$ROOT/usr/bin/aztek" | sed 's/^shlibs:Depends=//')
rm -rf "$WORK"

SIZE=$(du -sk "$ROOT" | cut -f1)
sed -e "s/@VERSION@/${VERSION}-1ubuntu${VERSION_ID}/" -e "s/@ARCH@/$ARCH/" -e "s/@SIZE@/$SIZE/" -e "s/@SHLIBS@/$SHLIBS/" \
  "$SRC/packaging/deb/control.in" >"$ROOT/DEBIAN/control"

dpkg-deb --root-owner-group --build "$ROOT" "$OUT/$FILE"
ls -l "$OUT/$FILE"
