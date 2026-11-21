#!/usr/bin/env bash
# Builds Aztek on macOS: ./build-macos.sh [--deps] [--debug] [--run] [--package] [--install]
#   --deps     install the build tools with Homebrew first
#   --debug    debug build in build-debug/
#   --run      start Aztek when the build is done
#   --package  put Aztek.app and dist/aztek-macos-<arch>.zip together for release
#   --install  copy Aztek.app to ~/Applications (implies --package)
# Every build is self-contained: Qt, Python and PySide6/shiboken6 are fetched into build/ when they are not there, and
# are copied into Aztek.app, which runs with nothing else installed. tools/bundle/versions.sh holds the versions.
set -euo pipefail
cd "$(dirname "$0")"
. tools/bundle/versions.sh

[ "$(uname -s)" = Darwin ] || { echo "This is the macOS build; on Linux use ./build.sh"; exit 1; }

DEPS=0 RUN=0 PACKAGE=0 INSTALL=0 TYPE=Release DIR=build
for a in "$@"; do
  case "$a" in
    --deps) DEPS=1 ;;
    --debug) TYPE=Debug DIR=build-debug ;;
    --run) RUN=1 ;;
    --package) PACKAGE=1 ;;
    --install) INSTALL=1 PACKAGE=1 ;;
    *) echo "unknown option $a"; exit 1 ;;
  esac
done

command -v brew >/dev/null || { echo "Install Homebrew first: https://brew.sh"; exit 1; }
if [ "$DEPS" = 1 ]; then
  xcode-select -p >/dev/null 2>&1 || { echo "Installing Xcode's command line tools; run this again when they're in."; xcode-select --install; exit 1; }
  brew install cmake ninja molten-vk
  [ -f "$HOME/.cargo/env" ] || command -v cargo >/dev/null || curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
fi

# Cargo from rustup's default place, for shells that haven't read ~/.cargo/env yet.
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
command -v cargo >/dev/null || { echo "Rust is missing: ./build-macos.sh --deps, or curl https://sh.rustup.rs -sSf | sh"; exit 1; }
for t in cmake ninja; do
  command -v "$t" >/dev/null || { echo "$t is missing (./build-macos.sh --deps installs it)"; exit 1; }
done

# Qt, Python and PySide6, fetched once into build/ and reused by later builds, debug ones too.
CACHE=$PWD/build
PYBUNDLE=$CACHE/python
PY=$PYBUNDLE/bin/python3.12
if [ ! -x "$PY" ]; then
  PYBUNDLE=$(tools/python/fetch-python.sh "$PYBUNDLE")
  PY=$PYBUNDLE/bin/python3.12
fi
QTDIR=$(tools/qt/fetch-qt.sh "$QT_VERSION" "$CACHE/qt" "$PY")
SITE=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["purelib"])')
tools/python/fetch-pyside.sh "$PY" "$QT_VERSION" "$SITE"
PYINC=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["include"])')
PYLIB=$PYBUNDLE/lib/libpython3.12.dylib
[ -f "$PYLIB" ] || { echo "libpython3.12.dylib is missing from $PYBUNDLE/lib" >&2; exit 1; }
MVK="$(brew --prefix molten-vk 2>/dev/null || true)/lib/libMoltenVK.dylib"
[ -f "$MVK" ] || { echo "MoltenVK is missing (./build-macos.sh --deps installs it)"; exit 1; }

EXTRA=(-DPython3_EXECUTABLE="$PY" -DPython3_INCLUDE_DIR="$PYINC" -DPython3_LIBRARY="$PYLIB" -DCMAKE_PREFIX_PATH="$QTDIR")
# Conda's libraries would shadow Homebrew's and the bundle's.
[ -n "${CONDA_PREFIX:-}" ] && EXTRA+=(-DCMAKE_IGNORE_PREFIX_PATH="$CONDA_PREFIX")
export PYO3_PYTHON="$PY"

cmake -B "$DIR" -G Ninja -DCMAKE_BUILD_TYPE="$TYPE" "${EXTRA[@]}"
cmake --build "$DIR"
APP="$DIR/aztek.app"
echo "Built $APP"

# Everything the app needs inside it: MoltenVK, the icon, Qt, Python with its standard library and PySide6, the plug-ins
# that ship with Aztek, and Harold's renderer. The signature comes last: everything before it changes the bundle.
mkdir -p "$APP/Contents/Frameworks" "$APP/Contents/Resources"
cp "$MVK" "$APP/Contents/Frameworks/"
ICONS="$DIR/aztek.iconset"
rm -rf "$ICONS" && mkdir -p "$ICONS"
for s in 16 32 128 256 512; do
  sips -z $s $s app/icons/aztek-icon.png --out "$ICONS/icon_${s}x${s}.png" >/dev/null
  sips -z $((s * 2)) $((s * 2)) app/icons/aztek-icon.png --out "$ICONS/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$ICONS" -o "$APP/Contents/Resources/aztek.icns"
"$QTDIR/bin/macdeployqt" "$APP"

# Python beside the app: its library in Frameworks, its standard library and PySide6 in Resources, and PYTHONHOME is
# pointed there at start-up (crates/aztek_host/src/python.rs).
PRES="$APP/Contents/Resources/python"
mkdir -p "$PRES/lib"
rm -rf "$PRES/lib/python3.12"
cp -R "$PYBUNDLE/lib/python3.12" "$PRES/lib/python3.12"
rm -rf "$PRES/lib/python3.12/test" "$PRES/lib/python3.12/tests" "$PRES/lib/python3.12/idlelib" \
  "$PRES/lib/python3.12/tkinter" "$PRES/lib/python3.12/turtledemo"
cp "$PYLIB" "$APP/Contents/Frameworks/"
OLDID=$(otool -D "$PYLIB" | tail -1)
install_name_tool -id "@rpath/libpython3.12.dylib" "$APP/Contents/Frameworks/libpython3.12.dylib"
case "$OLDID" in
  @rpath/*) ;;
  *) install_name_tool -change "$OLDID" "@rpath/libpython3.12.dylib" "$APP/Contents/MacOS/aztek" ;;
esac
install_name_tool -add_rpath "@executable_path/../Frameworks" "$APP/Contents/MacOS/aztek" 2>/dev/null || true
# PySide6 and shiboken6 link Python and their own Qt: the app's Frameworks and the wheel's Qt are on their path.
for f in $(find "$PRES/lib/python3.12/site-packages/PySide6" "$PRES/lib/python3.12/site-packages/shiboken6" -name '*.so'); do
  install_name_tool -add_rpath "@executable_path/../Frameworks" "$f" 2>/dev/null || true
  case "$OLDID" in
    @rpath/*) ;;
    *) install_name_tool -change "$OLDID" "@rpath/libpython3.12.dylib" "$f" 2>/dev/null || true ;;
  esac
done
install_name_tool -add_rpath "@executable_path/../Resources/python/lib/python3.12/site-packages/PySide6/Qt/lib" \
  "$APP/Contents/MacOS/aztek" 2>/dev/null || true

# The plug-ins that ship with Aztek (plugins/), where it looks for them inside the app.
if [ -d "$DIR/plug-ins" ]; then
  mkdir -p "$APP/Contents/PlugIns/aztek"
  cp "$DIR"/plug-ins/*.so "$APP/Contents/PlugIns/aztek/"
fi
# Harold's command-line renderer, beside the program.
cp "$DIR/harold" "$APP/Contents/MacOS/"
codesign --force --deep --sign - "$APP"
codesign --verify --deep --strict "$APP"

if [ "$PACKAGE" = 1 ]; then
  ARCH="$(uname -m)"
  rm -rf dist/Aztek && mkdir -p dist/Aztek
  cp -R "$APP" dist/Aztek/Aztek.app
  cp -R examples dist/Aztek/examples
  rm -f "dist/aztek-macos-$ARCH.zip"
  ditto -c -k --keepParent dist/Aztek "dist/aztek-macos-$ARCH.zip"
  echo "Packaged dist/Aztek/Aztek.app and dist/aztek-macos-$ARCH.zip"
fi

if [ "$INSTALL" = 1 ]; then
  mkdir -p "$HOME/Applications"
  rm -rf "$HOME/Applications/Aztek.app"
  cp -R dist/Aztek/Aztek.app "$HOME/Applications/Aztek.app"
  echo "Installed to $HOME/Applications/Aztek.app"
fi

if [ "$RUN" = 1 ]; then
  if [ "$INSTALL" = 1 ]; then
    open "$HOME/Applications/Aztek.app"
  else
    exec "$APP/Contents/MacOS/aztek"
  fi
fi
