#!/usr/bin/env bash
# Installs what a build on Rocky Linux 8 or 9 needs (run as root in the container).
#   install-build-deps.sh [--system-qt]
# Without arguments, for the RPM: the development libraries Qt's own kit looks for. With --system-qt, for the ARM
# AppImage, which builds against Rocky 9's own Qt instead.
# Rust is not here: it comes from rustup, the distribution's cargo is too old. Qt, Python and PySide6 are not here
# either: build.sh fetches them.
set -euo pipefail

. /etc/os-release
EL=${VERSION_ID%%.*}

dnf -y --setopt=retries=10 install epel-release dnf-plugins-core
if [ "$EL" = 8 ]; then
  dnf config-manager --set-enabled powertools
else
  dnf config-manager --set-enabled crb
fi

# The tools, and Python 3.12 for `cargo test` on the checkout. The build itself uses the Python it fetches.
BASE="git-core tar gzip xz findutils which file rpm-build cmake ninja-build desktop-file-utils
      gcc gcc-c++ make curl vulkan-loader-devel python3.12 python3.12-devel"
# What Qt's own kit asks for when CMake looks at it.
AQT="libglvnd-devel libxkbcommon-devel libxkbcommon-x11 fontconfig freetype dbus-libs glib2
     xcb-util-cursor xcb-util-image xcb-util-keysyms xcb-util-renderutil xcb-util-wm"

if [ "${1:-}" = --system-qt ]; then
  [ "$EL" != 8 ] || { echo "--system-qt: Rocky 8 has no Qt 6 packages" >&2; exit 1; }
  # The ARM AppImage, against Rocky 9's Qt.
  dnf -y --setopt=retries=10 install $BASE qt6-qtbase-devel
elif [ "$EL" = 8 ]; then
  # Qt's prebuilt kit wants a newer compiler than gcc 8 for its headers, and the development libraries its CMake files
  # look for, which are not part of any Rocky 8 Qt package because there is no Qt package.
  dnf -y --setopt=retries=10 install $BASE $AQT python3.12-pip gcc-toolset-13-gcc-c++
else
  # Rocky 9's Qt development package brings the same libraries, and pulls everything the fetched kit's CMake files ask
  # for. The fetched kit is what the RPM builds against; this package's own Qt is never linked.
  dnf -y --setopt=retries=10 install $BASE qt6-qtbase-devel
fi
