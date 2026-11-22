#!/usr/bin/env bash
# Builds the RPM for this Rocky Linux release from a self-contained build: packaging/rpm/build-rpm.sh [build dir] [output dir]
# The spec runs ./build.sh, which fetches Qt, Python and PySide6 into <checkout>/build, and installs build/Aztek under
# /opt/aztek, so the package carries its own Qt, Python and PySide6.
# Needs packaging/rpm/install-build-deps.sh done and cargo on PATH.
set -euo pipefail

SRC=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "${1:-$SRC/build}" "${2:-$SRC/dist}"
BUILD=$(cd "${1:-$SRC/build}" && pwd)
OUT=$(cd "${2:-$SRC/dist}" && pwd)

. /etc/os-release
EL=${VERSION_ID%%.*}
VERSION=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$SRC/Cargo.toml" | head -n1)
# Qt's prebuilt kit wants a newer compiler than Rocky 8's gcc 8 for its headers.
if [ "$EL" = 8 ]; then
  set +u
  . /opt/rh/gcc-toolset-14/enable
  set -u
fi

rpmbuild -bb "$SRC/packaging/rpm/aztek.spec" \
  --define "_topdir $BUILD/rpmbuild" \
  --define "_rpmdir $OUT" \
  --define "_build_name_fmt %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm" \
  --define "dist .el$EL" \
  --define "aztek_src $SRC" \
  --define "aztek_version $VERSION"
ls -l "$OUT"/*.rpm
