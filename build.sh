#!/usr/bin/env bash
# Builds Aztek on Linux: ./build.sh [--deps] [--debug] [--run] [--install]
#   --deps     install what the build itself needs first (apt, dnf or pacman)
#   --debug    debug build in build-debug/
#   --run      start Aztek when the build is done
#   --install  put the app in ~/.local/share/aztek with a launcher in ~/.local/bin
# Every build is self-contained: the Qt, Python and PySide6/shiboken6 it uses are fetched into build/ when they are not
# there, the program is built against those and never against the system's, and the result runs with nothing else
# installed. tools/bundle/versions.sh holds the versions.
set -euo pipefail
cd "$(dirname "$0")"
. tools/bundle/versions.sh

[ "$(uname -s)" = Linux ] || { echo "This is the Linux build; on macOS use ./build-macos.sh"; exit 1; }

DEPS=0 RUN=0 INSTALL=0 TYPE=Release DIR=build
for a in "$@"; do
  case "$a" in
    --deps) DEPS=1 ;;
    --debug) TYPE=Debug DIR=build-debug ;;
    --run) RUN=1 ;;
    --install) INSTALL=1 ;;
    \#*) break ;; # the rest of a line pasted with its comment: zsh passes "# ..." on as arguments
    *) echo "unknown option $a"; exit 1 ;;
  esac
done

if [ "$DEPS" = 1 ]; then
  if command -v apt >/dev/null; then
    sudo apt install -y build-essential cmake ninja-build curl libglvnd-dev libxcb-cursor0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1 libxcb-render-util0 libxcb-shape0 libxcb-sync1 libxcb-xfixes0 libxcb-xkb1 libxcb-xinerama0 libxkbcommon-dev libxkbcommon-x11-dev \
      libfontconfig1-dev libfreetype-dev libdbus-1-dev libglib2.0-dev mesa-vulkan-drivers libvulkan1
  elif command -v dnf >/dev/null; then
    sudo dnf install -y gcc-c++ cmake ninja-build curl libglvnd-devel xcb-util-cursor xcb-util-image xcb-util-keysyms xcb-util-renderutil xcb-util-wm libxkbcommon-devel libxkbcommon-x11 \
      fontconfig freetype dbus-libs glib2 mesa-vulkan-drivers vulkan-loader
  elif command -v pacman >/dev/null; then
    sudo pacman -S --needed base-devel cmake ninja curl libglvnd libxkbcommon xcb-util-cursor xcb-util-image xcb-util-keysyms xcb-util-renderutil xcb-util-wm fontconfig freetype2 dbus glib2 \
      vulkan-icd-loader
  else
    echo "Install by hand: a C++20 compiler, CMake 3.21+, Ninja, curl, the OpenGL and XKB development files, and a"
    echo "Vulkan driver."
  fi
fi

command -v cargo >/dev/null || { echo "Rust is missing: curl https://sh.rustup.rs -sSf | sh"; exit 1; }
for t in cmake ninja; do
  command -v "$t" >/dev/null || { echo "$t is missing (./build.sh --deps installs it)"; exit 1; }
done

# Qt, Python and PySide6, fetched once into build/ and reused by later builds, debug ones too.
CACHE=$PWD/build
PYBUNDLE=$CACHE/python
PY=$PYBUNDLE/bin/python3.13
if [ ! -x "$PY" ]; then
  PYBUNDLE=$(tools/python/fetch-python.sh "$PYBUNDLE")
  PY=$PYBUNDLE/bin/python3.13
fi
QTDIR=$(tools/qt/fetch-qt.sh "$QT_VERSION" "$CACHE/qt" "$PY")
SITE=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["purelib"])')
tools/python/fetch-pyside.sh "$PY" "$QT_VERSION" "$SITE"
PYINC=$(PYTHONHOME= "$PY" -c 'import sysconfig; print(sysconfig.get_paths()["include"])')
PYLIB=$PYBUNDLE/lib/libpython3.13.so
[ -f "$PYLIB" ] || { echo "libpython3.13.so is missing from $PYBUNDLE/lib" >&2; exit 1; }

EXTRA=(-DCMAKE_PREFIX_PATH="$QTDIR" -DPython3_EXECUTABLE="$PY" -DPython3_INCLUDE_DIR="$PYINC" -DPython3_LIBRARY="$PYLIB")
[ -n "${CONDA_PREFIX:-}" ] && EXTRA+=(-DCMAKE_IGNORE_PREFIX_PATH="$CONDA_PREFIX")
export PYO3_PYTHON="$PY"

# A build folder configured against another Qt (the system's, from before Qt was fetched, or an older fetch) keeps
# that Qt in its cache whatever CMAKE_PREFIX_PATH says, and the program then mixes two Qts and crashes at start (#289).
# Forget the cached configuration in that case; the compiled objects are rebuilt against the right Qt.
if [ -f "$DIR/CMakeCache.txt" ]; then
  CACHED_QT=$(sed -n 's/^Qt6_DIR:PATH=//p' "$DIR/CMakeCache.txt")
  if [ -n "$CACHED_QT" ] && [ "${CACHED_QT#"$QTDIR"}" = "$CACHED_QT" ]; then
    echo "$DIR was configured against the Qt in $CACHED_QT; configuring it again against $QTDIR"
    rm -rf "$DIR/CMakeCache.txt" "$DIR/CMakeFiles"
  fi
fi
cmake -B "$DIR" -G Ninja -DCMAKE_BUILD_TYPE="$TYPE" "${EXTRA[@]}"
cmake --build "$DIR"
APP=$DIR/Aztek
packaging/linux/stage-app.sh "$DIR" "$APP" "$PY" "$QTDIR"
echo "Built $APP (run $APP/AppRun)"

if [ "$INSTALL" = 1 ]; then
  DATA=${XDG_DATA_HOME:-$HOME/.local/share}
  PREFIX=$DATA/aztek
  rm -rf "$PREFIX"
  mkdir -p "$PREFIX" "$HOME/.local/bin" "$DATA/applications" "$DATA/icons"
  cp -a "$APP/." "$PREFIX/"
  ln -sf "$PREFIX/AppRun" "$HOME/.local/bin/aztek"
  sed -e "s|^Exec=.*|Exec=$PREFIX/AppRun|" "$APP/usr/share/applications/aztek.desktop" >"$DATA/applications/aztek.desktop"
  cp -a "$APP/usr/share/icons/." "$DATA/icons/" 2>/dev/null || true
  command -v update-desktop-database >/dev/null && update-desktop-database "$DATA/applications" || true
  echo "Installed to $PREFIX (start it with $HOME/.local/bin/aztek)"
fi

if [ "$RUN" = 1 ]; then
  exec "$APP/AppRun"
fi
