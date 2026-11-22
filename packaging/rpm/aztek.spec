# Built by packaging/rpm/build-rpm.sh, which passes aztek_src (the checkout) and aztek_version. The spec builds from the
# checkout rather than a source tarball: ./build.sh fetches Qt, Python and PySide6 into build/ and stages a self-contained
# tree in build/Aztek, which is installed under /opt/aztek. Nothing comes from a Qt, Python or PySide6 package.

%global debug_package %{nil}
# PySide6's wheel carries templates ending in .py (its Android deploy recipes), which byte-compiling rejects. The bundle's
# Python compiles what it needs when it first runs.
%global __brp_python_bytecompile %{nil}
# The bundle's libraries are private: nothing else may resolve against them, and rpm must not look for them elsewhere.
%global _build_id_links none
%global __provides_exclude_from ^/opt/aztek/.*$
%global __requires_exclude_from ^/opt/aztek/.*$
%global __requires_exclude ^(libQt6|libicu|libpython3|libshiboken6|libpyside6|libtcl|libtk).*$

Name:           aztek
Version:        %{aztek_version}
Release:        1%{?dist}
Summary:        Modelling, UV, rigging and animation application
License:        MIT-0
URL:            https://github.com/srlegrand/AKA-Crest-Aztek
ExclusiveArch:  x86_64

BuildRequires:  cmake >= 3.21
BuildRequires:  ninja-build
BuildRequires:  gcc-c++
BuildRequires:  git-core
BuildRequires:  curl
BuildRequires:  tar gzip xz findutils
BuildRequires:  desktop-file-utils
BuildRequires:  libglvnd-devel
BuildRequires:  libxkbcommon-devel libxkbcommon-x11
BuildRequires:  fontconfig freetype dbus-libs glib2
BuildRequires:  xcb-util-cursor xcb-util-image xcb-util-keysyms xcb-util-renderutil xcb-util-wm

# The viewport loads libvulkan.so.1 with dlopen, so rpm's scan of the binary cannot see it.
Requires:       vulkan-loader
Requires:       hicolor-icon-theme
# What the bundle's Qt and Python still take from the machine: its desktop, fonts and C++ runtime. The xcb-util
# libraries the xcb plugin needs are inside the bundle.
Requires:       libX11 libxcb fontconfig freetype glib2 dbus-libs libstdc++ zlib libglvnd-egl libglvnd-glx
# A Vulkan driver is what draws. NVIDIA's own driver brings one, and is preferred over Mesa's when both are there.
Recommends:     mesa-vulkan-drivers

%description
Aztek is a modelling, UV, rigging and animation application, with MEL and Python scripting and .ma files. The Qt, Python
and PySide6 it runs on are inside it, in /opt/aztek. The viewport needs a Vulkan driver: the NVIDIA driver's own, or
Mesa's.

%build
cd %{aztek_src}
%{aztek_src}/build.sh

%install
mkdir -p %{buildroot}/opt/aztek %{buildroot}%{_bindir} %{buildroot}%{_datadir}/applications \
  %{buildroot}%{_datadir}/icons/hicolor %{buildroot}%{_datadir}/licenses/aztek
cp -a %{aztek_src}/build/Aztek/. %{buildroot}/opt/aztek/
cat >%{buildroot}%{_bindir}/aztek <<'EOF'
#!/bin/sh
exec /opt/aztek/AppRun "$@"
EOF
cat >%{buildroot}%{_bindir}/aztekpy <<'EOF'
#!/bin/sh
exec /opt/aztek/AppRun --python "$@"
EOF
cat >%{buildroot}%{_bindir}/harold <<'EOF'
#!/bin/sh
exec /opt/aztek/usr/bin/harold "$@"
EOF
chmod 755 %{buildroot}%{_bindir}/aztek %{buildroot}%{_bindir}/aztekpy %{buildroot}%{_bindir}/harold
sed 's|^Exec=.*|Exec=/opt/aztek/AppRun|' %{aztek_src}/build/Aztek/usr/share/applications/aztek.desktop \
  >%{buildroot}%{_datadir}/applications/aztek.desktop
cp -a %{aztek_src}/build/Aztek/usr/share/icons/. %{buildroot}%{_datadir}/icons/
desktop-file-validate %{buildroot}%{_datadir}/applications/aztek.desktop
install -Dm644 %{aztek_src}/LICENSE %{buildroot}%{_datadir}/licenses/aztek/LICENSE
install -Dm644 %{aztek_src}/THIRD_PARTY.md %{buildroot}%{_datadir}/licenses/aztek/THIRD_PARTY.md

%files
%{_datadir}/licenses/aztek/LICENSE
%{_datadir}/licenses/aztek/THIRD_PARTY.md
%{_bindir}/aztek
# aztekpy, the reference's mayapy.
%{_bindir}/aztekpy
# Harold, the command-line renderer.
%{_bindir}/harold
%{_datadir}/applications/aztek.desktop
%{_datadir}/icons/hicolor/*/apps/aztek.*
/opt/aztek
