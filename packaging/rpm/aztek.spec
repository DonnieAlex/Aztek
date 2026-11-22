# Built by packaging/rpm/build-rpm.sh, which passes aztek_src (the checkout), aztek_build (the CMake build folder),
# aztek_version and dist. This builds from the checkout rather than a source tarball, so a build already done in that
# folder is reused.
#
# --with bundled_qt: Qt from aztek_qt (a kit from aqtinstall) goes into the private folder _libdir/aztek. Rocky 8 needs
# this: none of its repositories has Qt 6. Rocky 9 uses EPEL's Qt.
#
# --with bundled_python: the Python 3.13 in aztek_python (from tools/python/fetch-python.sh) goes into the private folder
# _prefix/lib/aztek/python, for a Rocky with no python3.13 in AppStream.

%bcond_with bundled_qt
%bcond_with bundled_python

%global debug_package %{nil}
# No /usr/lib/.build-id links: a bundled Qt library would clash with the same library in any other package.
%global _build_id_links none
%global python_bin /usr/bin/python3.13

# The bundled libraries are private: nothing else may resolve against them, and rpm must not look for them elsewhere.
%if %{with bundled_qt} && %{with bundled_python}
%global __provides_exclude_from ^(%{_libdir}/aztek|%{_prefix}/lib/aztek/python)/.*$
%global __requires_exclude ^(libQt6|libicu(data|i18n|uc)\\.so\\.[0-9]+\\(|libxcb-cursor\\.so|libpython3\\.13\\.so)
%else
%if %{with bundled_qt}
%global __provides_exclude_from ^%{_libdir}/aztek/.*$
%global __requires_exclude ^(libQt6|libicu(data|i18n|uc)\\.so\\.[0-9]+\\(|libxcb-cursor\\.so)
%endif
%if %{with bundled_python}
%global __provides_exclude_from ^%{_prefix}/lib/aztek/python/.*$
%global __requires_exclude ^libpython3\\.13\\.so
%endif
%endif

%if %{with bundled_python}
# The bundled Python is left as it comes: its own byte-code and shebangs, not the system Python's. The shebang exclusion
# comes first: on Rocky 8 the line below copies __os_install_post with its macros already expanded.
%global __brp_mangle_shebangs_exclude_from ^%{_prefix}/lib/aztek/python/.*$
%global __brp_python_bytecompile %{nil}
%global __os_install_post %(echo '%{__os_install_post}' | sed -e 's!/usr/lib[^[:space:]]*/brp-python-bytecompile[[:space:]].*$!!g')
%endif

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
%if %{without bundled_python}
BuildRequires:  python3.13-devel
%endif
BuildRequires:  vulkan-loader-devel
BuildRequires:  desktop-file-utils
%if %{without bundled_qt}
BuildRequires:  qt6-qtbase-devel
Requires:       qt6-qtbase-gui
%endif

# The viewport loads libvulkan.so.1 with dlopen, so rpm's scan of the binary cannot see it.
Requires:       vulkan-loader
Requires:       hicolor-icon-theme
# A Vulkan driver is what draws. NVIDIA's own driver brings one, and is preferred over Mesa's when both are there.
Recommends:     mesa-vulkan-drivers

%description
Aztek is a modelling, UV, rigging and animation application, with MEL and Python scripting and .ma files.
The viewport needs a Vulkan driver: the NVIDIA driver's own, or Mesa's.

%build
cd %{aztek_src}
%if %{with bundled_qt}
QT_ARGS='-DCMAKE_PREFIX_PATH=%{aztek_qt} -DCMAKE_INSTALL_RPATH=$ORIGIN/../%{_lib}/aztek/lib -DCMAKE_EXE_LINKER_FLAGS=-Wl,--disable-new-dtags'
%else
QT_ARGS=
%endif
%if %{with bundled_python}
PY_ARGS='-DAZTEK_PYTHON_BUNDLE=%{aztek_python}'
%else
PY_ARGS='-DPython3_EXECUTABLE=%{python_bin}'
%endif
cmake -S . -B %{aztek_build} -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=%{_prefix} \
  $PY_ARGS $QT_ARGS
cmake --build %{aztek_build}

%install
DESTDIR=%{buildroot} cmake --install %{aztek_build}
install -d %{buildroot}%{_datadir}/aztek/examples
%if %{with bundled_qt}
%{aztek_src}/packaging/linux/bundle-qt.sh %{aztek_qt} %{buildroot}%{_bindir}/aztek %{buildroot}%{_libdir}/aztek
%endif
desktop-file-validate %{buildroot}%{_datadir}/applications/aztek.desktop
install -Dm644 %{aztek_src}/LICENSE %{buildroot}%{_datadir}/licenses/aztek/LICENSE

%files
%{_datadir}/licenses/aztek/LICENSE
%{_bindir}/aztek
# aztekpy, the reference's mayapy.
%{_bindir}/aztekpy
# Harold, the command-line renderer.
%{_bindir}/harold
%{_datadir}/applications/aztek.desktop
%{_datadir}/icons/hicolor/*/apps/aztek.*
%{_datadir}/aztek
# The plug-ins that ship with Aztek (plugins/CMakeLists.txt installs them to lib/aztek/plug-ins).
%{_prefix}/lib/aztek
%if %{with bundled_qt}
%{_libdir}/aztek
%endif
