#!/usr/bin/env bash
# Installs the system packages ./build.sh and the Xvfb checks need on a bare Ubuntu or Debian machine with no desktop:
# OpenGL and Vulkan headers for configuring against the bundled Qt, the xcb libraries its platform plugin loads,
# a software Vulkan driver, Xvfb and xdotool. Desktop installs usually have all of these already.
set -euo pipefail
sudo=""
[ "$(id -u)" -ne 0 ] && sudo="sudo"
$sudo apt-get update -qq
DEBIAN_FRONTEND=noninteractive $sudo apt-get install -y -qq --no-install-recommends \
  cmake ninja-build clang pkg-config \
  libgl-dev libglx-dev libegl-dev libvulkan-dev libvulkan1 mesa-vulkan-drivers \
  libxkbcommon-dev libxkbcommon-x11-0 libxcb-cursor0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1 \
  libxcb-render-util0 libxcb-xinerama0 libxcb-xkb1 libxcb-shape0 \
  xvfb xauth xdotool imagemagick
echo "Build dependencies installed."
