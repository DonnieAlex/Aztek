#!/usr/bin/env bash
# Python 3.13, the version Aztek embeds (the VFX Reference Platform CY2027 version), for systems that have no 3.13 of
# their own: a relocatable build from python-build-standalone, checked against its published SHA-256.
#   tools/python/fetch-python.sh <folder>
# Unpacks into <folder> (bin/, include/, lib/) and prints <folder>. Nothing is downloaded when it is already there.
set -euo pipefail

DEST=${1:?usage: fetch-python.sh <folder>}
RELEASE=20261110
VERSION=3.13.18

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) TRIPLE=x86_64-unknown-linux-gnu SUM=50c774f6674740739f791c05a9177ed03e2fe02abebda9bca3512eb7309b37cc ;;
  Linux-aarch64) TRIPLE=aarch64-unknown-linux-gnu SUM=d25a65036d5e149db902fa52ad65878d0c00e9301360ec197832f99063087e9e ;;
  Darwin-arm64) TRIPLE=aarch64-apple-darwin SUM=1afd3983237c2a10277345f235be730e9cb49a123e0574ed252dbf3a841a24e4 ;;
  Darwin-x86_64) TRIPLE=x86_64-apple-darwin SUM=a46bcdeecd28ba253522b321dcb3806da1b0ff581479fc1d27e9613e3ac44764 ;;
  *) echo "No Python $VERSION build for $(uname -s) $(uname -m): install Python 3.13 with its development files" >&2; exit 1 ;;
esac

if [ -f "$DEST/.aztek-python-$VERSION-$RELEASE" ]; then
  echo "$DEST"
  exit 0
fi

NAME="cpython-$VERSION+$RELEASE-$TRIPLE-install_only.tar.gz"
URL="https://github.com/astral-sh/python-build-standalone/releases/download/$RELEASE/$NAME"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
echo "Downloading Python $VERSION: $URL" >&2
curl -fL --retry 5 --retry-delay 10 -o "$TMP/python.tar.gz" "$URL" >&2
# macOS has shasum rather than sha256sum.
if command -v sha256sum >/dev/null; then GOT=$(sha256sum "$TMP/python.tar.gz"); else GOT=$(shasum -a 256 "$TMP/python.tar.gz"); fi
[ "${GOT%% *}" = "$SUM" ] || { echo "Python download: checksum mismatch" >&2; exit 1; }
tar -xzf "$TMP/python.tar.gz" -C "$TMP"
rm -rf "$DEST"
mkdir -p "$(dirname "$DEST")"
mv "$TMP/python" "$DEST"
touch "$DEST/.aztek-python-$VERSION-$RELEASE"
echo "$DEST"
