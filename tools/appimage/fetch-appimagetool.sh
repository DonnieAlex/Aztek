#!/usr/bin/env bash
# appimagetool for this machine's kind, a pinned release checked against its published SHA-256, as
# tools/python/fetch-python.sh does for Python.
#   tools/appimage/fetch-appimagetool.sh <file>
# Downloads to <file> unless it is already there with the right checksum, and prints <file>.
set -euo pipefail

DEST=${1:?usage: fetch-appimagetool.sh <file>}
VERSION=1.9.1

case "$(uname -m)" in
  x86_64) MACHINE=x86_64 SUM=ed4ce84f0d9caff66f50bcca6ff6f35aae54ce8135408b3fa33abfc3cb384eb0 ;;
  aarch64 | arm64) MACHINE=aarch64 SUM=f0837e7448a0c1e4e650a93bb3e85802546e60654ef287576f46c71c126a9158 ;;
  *) echo "No appimagetool $VERSION for $(uname -m)" >&2; exit 1 ;;
esac

sum() { sha256sum "$1" | cut -d' ' -f1; }
if [ -f "$DEST" ] && [ "$(sum "$DEST")" = "$SUM" ]; then
  chmod 755 "$DEST"
  echo "$DEST"
  exit 0
fi

URL="https://github.com/AppImage/appimagetool/releases/download/$VERSION/appimagetool-$MACHINE.AppImage"
mkdir -p "$(dirname "$DEST")"
# A file of this run's own beside the destination: two builds at once never check or move each other's download, and
# the rename is atomic.
PART=$(mktemp "$DEST.XXXXXX")
trap 'rm -f "$PART"' EXIT
echo "Downloading appimagetool $VERSION: $URL" >&2
curl -fsSL --retry 5 --retry-delay 10 -o "$PART" "$URL" >&2
[ "$(sum "$PART")" = "$SUM" ] || { echo "appimagetool download: checksum mismatch" >&2; exit 1; }
chmod 755 "$PART"
mv "$PART" "$DEST"
echo "$DEST"
