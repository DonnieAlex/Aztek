#!/usr/bin/env bash
# Qt for the build, from Qt's own kit, fetched with aqtinstall: the kit PySide6 is built for, so a build never links the
# distribution's Qt. A kit already in <dest> is reused, so later builds download nothing.
#   tools/qt/fetch-qt.sh <version> <dest> [python]
# Prints <dest>/<version>/<kit>.
set -euo pipefail

VER=${1:?usage: fetch-qt.sh <version> <dest> [python]}
DEST=${2:?usage: fetch-qt.sh <version> <dest> [python]}
PY=${3:-}

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) HOST=linux; ARCH=linux_gcc_64; KIT=gcc_64 ;;
  Darwin-arm64 | Darwin-x86_64) HOST=mac; ARCH=clang_64; KIT=macos ;;
  Linux-aarch64)
    echo "No Qt $VER kit for Linux arm64: aqtinstall has none for this Qt and PySide6 has no arm64 Linux wheel." >&2
    echo "A bundled build needs Qt 6.9 or later, which this project does not pin yet." >&2
    exit 1 ;;
  *) echo "No Qt $VER kit for $(uname -s) $(uname -m)" >&2; exit 1 ;;
esac

QT=$DEST/$VER/$KIT
if [ -d "$QT/lib/cmake/Qt6" ]; then
  echo "$QT"
  exit 0
fi

if [ -z "$PY" ]; then
  PY=$(command -v python3 || true)
  [ -n "$PY" ] || { echo "Python 3 is missing: it is needed once to fetch Qt" >&2; exit 1; }
fi
mkdir -p "$DEST"
# aqtinstall in a venv of its own, so neither the build's Python nor the system's pip is changed. Their chatter goes to
# stderr: stdout carries the kit's path and nothing else.
"$PY" -m venv "$DEST/.aqt" >&2
"$DEST/.aqt/bin/python" -m pip install -q --retries 10 --timeout 120 aqtinstall >&2
# Qt's download servers drop connections now and then: a few tries, each starting clean.
for i in 1 2 3 4 5; do
  if "$DEST/.aqt/bin/python" -m aqt install-qt "$HOST" desktop "$VER" "$ARCH" --outputdir "$DEST" >&2; then
    break
  fi
  [ "$i" = 5 ] && { echo "Qt $VER: download failed 5 times" >&2; exit 1; }
  rm -rf "${DEST:?}/$VER"
  sleep $((i * 60))
done
rm -rf "$DEST/.aqt"
[ -d "$QT/lib/cmake/Qt6" ] || { echo "Qt $VER: the kit did not arrive in $QT" >&2; exit 1; }
echo "$QT"
