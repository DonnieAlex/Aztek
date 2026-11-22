#!/usr/bin/env bash
# Packs a staged Linux app (the tree packaging/linux/stage-app.sh leaves behind) into an AppImage, without the FUSE
# requirement of the tool: it unpacks itself and runs.
#   packaging/appimage/pack-appimage.sh <appdir> <output file>
set -euo pipefail

SRC=$(cd "$(dirname "$0")/../.." && pwd)
APPDIR=$(cd "${1:?usage: pack-appimage.sh <appdir> <output file>}" && pwd)
OUT=${2:?usage: pack-appimage.sh <appdir> <output file>}
[ -x "$APPDIR/AppRun" ] || { echo "no AppRun in $APPDIR" >&2; exit 1; }
mkdir -p "$(dirname "$OUT")"
OUT=$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")

cp "$APPDIR/usr/share/applications/aztek.desktop" "$APPDIR/aztek.desktop"
cp "$APPDIR/usr/share/icons/hicolor/256x256/apps/aztek.png" "$APPDIR/aztek.png"
ln -sf aztek.png "$APPDIR/.DirIcon"
install -Dm644 "$SRC/LICENSE" "$APPDIR/usr/share/doc/aztek/LICENSE"

MACHINE=$(uname -m)
# A pinned appimagetool, checked against its SHA-256, unless APPIMAGETOOL names one.
TOOL=${APPIMAGETOOL:-$("$SRC/tools/appimage/fetch-appimagetool.sh" "$APPDIR/../appimagetool-$MACHINE.AppImage")}
# Run unpacked: a container has no FUSE to mount the tool with.
ARCH=$MACHINE "$TOOL" --appimage-extract-and-run --no-appstream "$APPDIR" "$OUT"
chmod 755 "$OUT"
ls -l "$OUT"
