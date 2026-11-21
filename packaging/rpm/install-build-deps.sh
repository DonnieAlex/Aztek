#!/usr/bin/env bash
# Installs what an RPM build needs on Rocky Linux 8 or 9 (run as root in the container).
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

# The libraries Qt's own kit asks for when CMake looks at it, and Python 3.12 for `cargo test` on the checkout. The
# build itself uses the Python it fetches.
COMMON="git-core tar gzip xz findutils which file rpm-build cmake ninja-build desktop-file-utils
        gcc gcc-c++ make curl vulkan-loader-devel python3.12 python3.12-devel
        libglvnd-devel libxkbcommon-devel libxkbcommon-x11 fontconfig freetype dbus-libs glib2
        xcb-util-cursor xcb-util-image xcb-util-keysyms xcb-util-renderutil xcb-util-wm"

if [ "$EL" = 8 ]; then
  # Qt's prebuilt kit wants a newer compiler than gcc 8 for its headers.
  dnf -y --setopt=retries=10 install $COMMON python3.12-pip gcc-toolset-13-gcc-c++
else
  dnf -y --setopt=retries=10 install $COMMON
fi
