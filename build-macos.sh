#!/usr/bin/env bash
# Builds Aztek on macOS: ./build-macos.sh [--deps] [--debug] [--run] [--package] [--install]
#   --deps     update the build tools with Homebrew even when they are already there
#   --debug    debug build in build-debug/
#   --run      start Aztek when the build is done
#   --package  put Aztek.app and dist/aztek-macos-<arch>.zip together for release
#   --install  copy Aztek.app to ~/Applications (implies --package)
# Whatever is missing is installed first: Xcode's command line tools, Homebrew, CMake, Ninja, MoltenVK and Rust.
# AZTEK_SIGN_IDENTITY signs with that Developer ID instead of ad hoc; with AZTEK_NOTARY_KEY, AZTEK_NOTARY_KEY_ID and
# AZTEK_NOTARY_ISSUER as well, --package notarizes the app (packaging/macos/sign.sh, notarize.sh).
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
    \#*) break ;; # the rest of a line pasted with its comment: zsh passes "# ..." on as arguments
    *) echo "unknown option $a"; exit 1 ;;
  esac
done

# Xcode's command line tools: the compiler, git, codesign. Their installer is a window of its own.
if ! xcode-select -p >/dev/null 2>&1; then
  xcode-select --install 2>/dev/null || true
  echo "Waiting for Xcode's command line tools: finish the installer window that just opened."
  until xcode-select -p >/dev/null 2>&1; do sleep 10; done
fi

# Homebrew, from where its installer puts it even when this shell hasn't been told yet, or installed now.
brew_env() {
  local b
  for b in /opt/homebrew/bin/brew /usr/local/bin/brew; do
    [ -x "$b" ] && { PATH="$(dirname "$b"):$PATH"; export PATH; return 0; }
  done
  return 1
}
if ! command -v brew >/dev/null; then
  if ! brew_env; then
    echo "Installing Homebrew (it asks for your password)."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    brew_env || { echo "Homebrew didn't install: see https://brew.sh"; exit 1; }
  fi
  # So that new terminals find it too, as Homebrew's installer asks.
  if ! grep -qs 'brew shellenv' "$HOME/.zprofile"; then
    printf '\neval "$(%s shellenv)"\n' "$(command -v brew)" >>"$HOME/.zprofile"
    echo "Added Homebrew to ~/.zprofile for new terminals."
  fi
fi

NEED=()
for t in cmake ninja; do
  { [ "$DEPS" = 1 ] || ! command -v "$t" >/dev/null; } && NEED+=("$t")
done
{ [ "$DEPS" = 1 ] || ! brew list --formula molten-vk >/dev/null 2>&1; } && NEED+=(molten-vk)
if [ ${#NEED[@]} -gt 0 ]; then brew install "${NEED[@]}"; fi

# Rust from rustup, in its default place, for shells that haven't read ~/.cargo/env yet.
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
if ! command -v cargo >/dev/null; then
  echo "Installing Rust with rustup."
  curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
  . "$HOME/.cargo/env"
fi

# Qt, Python and PySide6, fetched once into build/ and reused by later builds, debug ones too.
CACHE=$PWD/build
PYBUNDLE=$CACHE/python
PY=$PYBUNDLE/bin/python3.13
if [ ! -x "$PY" ]; then
  PYBUNDLE=$(tools/python/fetch-python.sh "$PYBUNDLE")
  PY=$PYBUNDLE/bin/python3.13
fi
QTDIR=$(tools/qt/fetch-qt.sh "$QT_VERSION" "$CACHE/qt" "$PY")
# Qt before 6.9 links the AGL framework, which the macOS 26 SDK no longer has (QTBUG-137687); nothing uses it.
WRAPGL="$QTDIR/lib/cmake/Qt6/FindWrapOpenGL.cmake"
[ -f "$WRAPGL" ] && sed -i '' '/target_link_libraries.*__opengl_agl_fw_path/d' "$WRAPGL"
SITE=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["purelib"])')
tools/python/fetch-pyside.sh "$PY" "$QT_VERSION" "$SITE"
PYINC=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["include"])')
PYLIB=$PYBUNDLE/lib/libpython3.13.dylib
[ -f "$PYLIB" ] || { echo "libpython3.13.dylib is missing from $PYBUNDLE/lib" >&2; exit 1; }
MVK="$(brew --prefix molten-vk 2>/dev/null || true)/lib/libMoltenVK.dylib"
[ -f "$MVK" ] || { echo "MoltenVK is missing from Homebrew: brew reinstall molten-vk"; exit 1; }

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
rm -rf "$PRES/lib/python3.13"
cp -R "$PYBUNDLE/lib/python3.13" "$PRES/lib/python3.13"
rm -rf "$PRES/lib/python3.13/test" "$PRES/lib/python3.13/tests" "$PRES/lib/python3.13/idlelib" \
  "$PRES/lib/python3.13/tkinter" "$PRES/lib/python3.13/turtledemo"
cp "$PYLIB" "$APP/Contents/Frameworks/"
OLDID=$(otool -D "$PYLIB" | tail -1)
install_name_tool -id "@rpath/libpython3.13.dylib" "$APP/Contents/Frameworks/libpython3.13.dylib"
case "$OLDID" in
  @rpath/*) ;;
  *) install_name_tool -change "$OLDID" "@rpath/libpython3.13.dylib" "$APP/Contents/MacOS/aztek" ;;
esac
install_name_tool -add_rpath "@executable_path/../Frameworks" "$APP/Contents/MacOS/aztek" 2>/dev/null || true
# PySide6 and shiboken6 link Python and their own Qt: the app's Frameworks and the wheel's Qt are on their path.
for f in $(find "$PRES/lib/python3.13/site-packages/PySide6" "$PRES/lib/python3.13/site-packages/shiboken6" -name '*.so'); do
  install_name_tool -add_rpath "@executable_path/../Frameworks" "$f" 2>/dev/null || true
  case "$OLDID" in
    @rpath/*) ;;
    *) install_name_tool -change "$OLDID" "@rpath/libpython3.13.dylib" "$f" 2>/dev/null || true ;;
  esac
done
install_name_tool -add_rpath "@executable_path/../Resources/python/lib/python3.13/site-packages/PySide6/Qt/lib" \
  "$APP/Contents/MacOS/aztek" 2>/dev/null || true

# Harold's command-line renderer, beside the program.
cp "$DIR/harold" "$APP/Contents/MacOS/"
# Ad hoc, or with a Developer ID when AZTEK_SIGN_IDENTITY names one (packaging/macos/sign.sh).
packaging/macos/sign.sh "$APP"

if [ "$PACKAGE" = 1 ]; then
  ARCH="$(uname -m)"
  rm -rf dist/Aztek && mkdir -p dist/Aztek
  cp -R "$APP" dist/Aztek/Aztek.app
  # Notarized when there is a Developer ID signature and an App Store Connect API key, so Gatekeeper lets it open.
  if [ -n "${AZTEK_SIGN_IDENTITY:-}" ] && [ -n "${AZTEK_NOTARY_KEY:-}" ]; then
    packaging/macos/notarize.sh dist/Aztek/Aztek.app
  fi
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
