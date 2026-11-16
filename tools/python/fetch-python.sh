#!/usr/bin/env bash
# Python 3.12, the version Aztek embeds (as the reference's own Python is 3.12), for systems that have no 3.12 of
# their own: a relocatable build from python-build-standalone, checked against its published SHA-256.
#   tools/python/fetch-python.sh <folder>
# Unpacks into <folder> (bin/, include/, lib/) and prints <folder>. Nothing is downloaded when it is already there.
set -euo pipefail

DEST=${1:?usage: fetch-python.sh <folder>}
RELEASE=20260924
VERSION=3.12.14

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) TRIPLE=x86_64-unknown-linux-gnu SUM=5eae8cf79dd47fc2496a4fccc892936be831ce7a84d984b2299dfb1cdb592682 ;;
  Linux-aarch64) TRIPLE=aarch64-unknown-linux-gnu SUM=c0574c9c9fec561eb192ad510e4e79aaad039929b3531b3bb873e2f630fb14a9 ;;
  Darwin-arm64) TRIPLE=aarch64-apple-darwin SUM=9763f43db2481a6af36af82ec40302aab7a73632f880129d07a6e81aec846277 ;;
  Darwin-x86_64) TRIPLE=x86_64-apple-darwin SUM=0d6a4a299908123f00bc844df737603f047ff9eba14fda6cad83f3cf3cb3a2af ;;
  *) echo "No Python $VERSION build for $(uname -s) $(uname -m): install Python 3.12 with its development files" >&2; exit 1 ;;
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
