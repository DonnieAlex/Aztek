#!/usr/bin/env bash
# Builds the RPM for this Rocky Linux release: packaging/rpm/build-rpm.sh [build dir] [output dir]
# Needs packaging/rpm/install-build-deps.sh done and cargo on PATH.
set -euo pipefail

SRC=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "${1:-$SRC/build}" "${2:-$SRC/dist}"
BUILD=$(cd "${1:-$SRC/build}" && pwd)
OUT=$(cd "${2:-$SRC/dist}" && pwd)
QT_VERSION=${AZTEK_QT_VERSION:-6.8.3}
QT_DIR=${AZTEK_QT_DIR:-/opt/qt}

. /etc/os-release
EL=${VERSION_ID%%.*}
VERSION=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$SRC/Cargo.toml" | head -n1)

ARGS=()
# Python 3.13, the only version Aztek embeds: AppStream's where it has one, else one fetched into the build folder that
# the package carries in /usr/lib/aztek/python.
if [ -x /usr/bin/python3.13 ] && [ -f /usr/include/python3.13/Python.h ]; then
  export PYO3_PYTHON=/usr/bin/python3.13
else
  BUNDLE=$("$SRC/tools/python/fetch-python.sh" "$BUILD/python")
  export PYO3_PYTHON="$BUNDLE/bin/python3.13"
  ARGS+=(--with bundled_python --define "aztek_python $BUNDLE")
fi
if [ "$EL" = 8 ]; then
  # Off for the toolset's script, which reads variables it does not set.
  set +u
  . /opt/rh/gcc-toolset-14/enable
  set -u
  "$SRC/packaging/linux/install-qt.sh" python3.12 "$QT_VERSION" "$QT_DIR"
  ARGS+=(--with bundled_qt --define "aztek_qt $QT_DIR/$QT_VERSION/gcc_64")
fi

rpmbuild -bb "$SRC/packaging/rpm/aztek.spec" "${ARGS[@]}" \
  --define "_topdir $BUILD/rpmbuild" \
  --define "_rpmdir $OUT" \
  --define "_build_name_fmt %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm" \
  --define "dist .el$EL" \
  --define "aztek_src $SRC" \
  --define "aztek_build $BUILD" \
  --define "aztek_version $VERSION"
ls -l "$OUT"/*.rpm
