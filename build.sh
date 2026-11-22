#!/usr/bin/env bash
# Builds Aztek on Linux and macOS: ./build.sh [--deps] [--debug] [--run]
# On macOS, ./build-macos.sh also makes a self-contained Aztek.app (--package) and installs it (--install).
#   --deps   install the build dependencies first (apt, dnf, pacman, or Homebrew on macOS)
#   --debug  debug build in build-debug/
#   --run    start Aztek when the build is done
#   --install  install to ~/.local: the binary, a menu entry and the icon
# Python 3.13 is the only version Aztek embeds: the system's when it has one, else one fetched into the build folder.
set -euo pipefail
cd "$(dirname "$0")"

DEPS=0 RUN=0 INSTALL=0 TYPE=Release DIR=build
for a in "$@"; do
  case "$a" in
    --deps) DEPS=1 ;;
    --debug) TYPE=Debug DIR=build-debug ;;
    --run) RUN=1 ;;
    --install) INSTALL=1 ;;
    *) echo "unknown option $a"; exit 1 ;;
  esac
done

MAC=0
[ "$(uname -s)" = Darwin ] && MAC=1

if [ "$DEPS" = 1 ]; then
  # macOS first: its /usr/bin/apt is a stub for an old Java tool, not a package manager.
  if [ "$MAC" = 1 ]; then
    command -v brew >/dev/null || { echo "Install Homebrew first: https://brew.sh"; exit 1; }
    brew install cmake ninja qt python@3.13 molten-vk
  elif command -v apt >/dev/null; then
    # Python 3.13 from the distribution where it has one; elsewhere the build fetches it (below).
    PYPKG=""
    apt-cache show python3.13-dev >/dev/null 2>&1 && PYPKG=python3.13-dev
    sudo apt install -y build-essential cmake ninja-build qt6-base-dev libvulkan-dev mesa-vulkan-drivers curl $PYPKG
  elif command -v dnf >/dev/null; then
    PYPKG=""
    dnf -q list python3.13-devel >/dev/null 2>&1 && PYPKG=python3.13-devel
    sudo dnf install -y gcc-c++ cmake ninja-build qt6-qtbase-devel vulkan-loader-devel mesa-vulkan-drivers curl $PYPKG
  elif command -v pacman >/dev/null; then
    sudo pacman -S --needed base-devel cmake ninja qt6-base vulkan-icd-loader curl
  else
    echo "Install by hand: a C++20 compiler, CMake 3.21+, Ninja, Qt 6.2+ (base, dev), Vulkan loader, curl, and Python 3.13"
    echo "with its development files if the system has it (the build fetches Python 3.13 otherwise)."
  fi
fi

if ! command -v cargo >/dev/null; then
  echo "Rust is missing: curl https://sh.rustup.rs -sSf | sh"
  exit 1
fi
for t in cmake ninja; do
  command -v "$t" >/dev/null || { echo "$t is missing (./build.sh --deps installs it)"; exit 1; }
done

# Python 3.13, the only version Aztek embeds. Conda's Python and libraries shadow the system's: never those.
EXTRA=()
if [ "$MAC" = 1 ] && command -v brew >/dev/null; then
  # Homebrew's Python and Qt, not Xcode's Python or a conda one.
  PY="$(brew --prefix python@3.13 2>/dev/null)/bin/python3.13"
  [ -x "$PY" ] || { echo "Homebrew's python@3.13 is missing (./build.sh --deps installs it)"; exit 1; }
  EXTRA+=(-DPython3_EXECUTABLE="$PY")
  export PYO3_PYTHON="$PY"
  QTP="$(brew --prefix qt 2>/dev/null)"
  [ -d "$QTP" ] && EXTRA+=(-DCMAKE_PREFIX_PATH="$QTP")
  [ -n "${CONDA_PREFIX:-}" ] && EXTRA+=(-DCMAKE_IGNORE_PREFIX_PATH="$CONDA_PREFIX")
elif [ "$MAC" = 0 ]; then
  if [ -x /usr/bin/python3.13 ] && /usr/bin/python3.13 -c 'import sysconfig, os, sys; sys.exit(not os.path.isfile(os.path.join(sysconfig.get_paths()["include"], "Python.h")))' 2>/dev/null; then
    # The system's own 3.13 with its development files.
    PY=/usr/bin/python3.13
    EXTRA+=(-DPython3_EXECUTABLE="$PY")
  else
    # None on this system (Ubuntu 24.04 and 26.04, Arch and others): a 3.13 fetched into the build folder, linked
    # from there and installed beside the program.
    command -v curl >/dev/null || { echo "curl is missing (./build.sh --deps installs it)"; exit 1; }
    BUNDLE=$(tools/python/fetch-python.sh "$PWD/$DIR/python")
    PY="$BUNDLE/bin/python3.13"
    EXTRA+=(-DAZTEK_PYTHON_BUNDLE="$BUNDLE")
  fi
  export PYO3_PYTHON="$PY"
  [ -n "${CONDA_PREFIX:-}" ] && EXTRA+=(-DCMAKE_IGNORE_PREFIX_PATH="$CONDA_PREFIX")
fi

cmake -B "$DIR" -G Ninja -DCMAKE_BUILD_TYPE="$TYPE" "${EXTRA[@]}"
cmake --build "$DIR"
BIN="$DIR/aztek"
[ "$MAC" = 1 ] && BIN="$DIR/aztek.app/Contents/MacOS/aztek"
echo "Built $BIN"

if [ "$INSTALL" = 1 ]; then
  cmake --install "$DIR" --prefix "$HOME/.local"
  command -v update-desktop-database >/dev/null && update-desktop-database "$HOME/.local/share/applications" || true
  echo "Installed to $HOME/.local (bin/aztek, menu entry, icon)"
fi

if [ "$RUN" = 1 ]; then
  exec "$BIN"
fi
