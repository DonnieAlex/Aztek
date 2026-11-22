#!/usr/bin/env bash
# Installs what an RPM build needs on Rocky Linux 8 or 9 (run as root in the container).
# Rust is not here: it comes from rustup, the distribution's cargo is too old.
set -euo pipefail

. /etc/os-release
EL=${VERSION_ID%%.*}

dnf -y --setopt=retries=10 install epel-release dnf-plugins-core
if [ "$EL" = 8 ]; then
  dnf config-manager --set-enabled powertools
else
  dnf config-manager --set-enabled crb
fi

COMMON="git-core tar gzip xz findutils which file rpm-build cmake ninja-build desktop-file-utils
        gcc gcc-c++ make vulkan-loader-devel"

# Python 3.13, the only version Aztek embeds, from AppStream where it has it; otherwise build-rpm.sh fetches one with
# curl and the package carries it.
PYPKG=""
dnf -q list python3.13-devel >/dev/null 2>&1 && PYPKG="python3.13 python3.13-devel"

if [ "$EL" = 8 ]; then
  # No Qt 6 in any Rocky 8 repository: Qt comes from packaging/linux/install-qt.sh and is bundled. Its prebuilt
  # kit wants a newer compiler than gcc 8 for its headers (gcc 14, the VFX Reference Platform CY2027 one), and the
  # system libraries its CMake files look for. xcb-util-cursor is only in EPEL here, so the package bundles it.
  # python3.12 runs aqtinstall.
  dnf -y --setopt=retries=10 install $COMMON $PYPKG python3.12 python3.12-pip gcc-toolset-14-gcc-c++ libglvnd-devel libxkbcommon-devel \
    fontconfig freetype dbus-libs glib2 libxkbcommon-x11 xcb-util-cursor xcb-util-image xcb-util-keysyms \
    xcb-util-renderutil xcb-util-wm
else
  dnf -y --setopt=retries=10 install $COMMON $PYPKG qt6-qtbase-devel
fi
