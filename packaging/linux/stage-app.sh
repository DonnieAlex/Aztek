#!/usr/bin/env bash
# Stages a self-contained Linux build: the program, the Qt 6 it was built against, Python with its standard library and
# PySide6/shiboken6, and the libraries they load that a desktop may not have. AppRun starts it with PYTHONHOME,
# PYTHONNOUSERSITE and LD_LIBRARY_PATH set to the tree, so nothing comes from the distribution's Qt, PySide or Python.
#   packaging/linux/stage-app.sh <build dir> <output dir> <python> <qt>
# The output is an AppDir: AppRun, usr/bin, usr/lib, usr/plugins.
set -euo pipefail

BUILD=${1:?usage: stage-app.sh <build dir> <output dir> <python> <qt>}
OUT=${2:?usage: stage-app.sh <build dir> <output dir> <python> <qt>}
PY=${3:?usage: stage-app.sh <build dir> <output dir> <python> <qt>}
QT=${4:?usage: stage-app.sh <build dir> <output dir> <python> <qt>}

# ld.so takes relative LD_LIBRARY_PATH entries from wherever it is run, so the three are made absolute here.
BUILD=$(cd "$BUILD" && pwd)
PY=$(cd "$(dirname "$PY")" && pwd)/$(basename "$PY")
QT=$(cd "$QT" && pwd)
mkdir -p "$(dirname "$OUT")"
OUT=$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")

rm -rf "$OUT"
# A debug build keeps its symbols, so a backtrace from the staged AppRun names Aztek's own functions (#289).
STRIP=(--strip)
case "$(sed -n 's/^CMAKE_BUILD_TYPE:[A-Z]*=//p' "$BUILD/CMakeCache.txt" 2>/dev/null)" in
  Debug | RelWithDebInfo) STRIP=() ;;
esac
DESTDIR=$OUT cmake --install "$BUILD" --prefix /usr ${STRIP[@]+"${STRIP[@]}"}
LIB=$OUT/usr/lib
mkdir -p "$LIB"

# Qt's plugins: the kit's.
for p in platforms/libqxcb.so platforms/libqoffscreen.so \
         platforminputcontexts/libcomposeplatforminputcontextplugin.so platforminputcontexts/libibusplatforminputcontextplugin.so \
         xcbglintegrations/libqxcb-glx-integration.so xcbglintegrations/libqxcb-egl-integration.so \
         imageformats/libqjpeg.so imageformats/libgif.so imageformats/libqico.so imageformats/libqsvg.so iconengines/libqsvgicon.so; do
  [ -e "$QT/plugins/$p" ] || continue
  mkdir -p "$OUT/usr/plugins/$(dirname "$p")"
  cp -pL "$QT/plugins/$p" "$OUT/usr/plugins/$p"
done
# Qt looks for its plugins where it was installed unless told otherwise.
printf '[Paths]\nPrefix = ..\nPlugins = plugins\n' >"$OUT/usr/bin/qt.conf"

# Python's standard library, without the wrappers Aztek does not use and without the packages, which come next.
STDLIB=$("$PY" -c 'import sysconfig; print(sysconfig.get_path("stdlib"))')
PYPREFIX=$("$PY" -c 'import sys; print(sys.base_prefix)')
PYLIB=$(dirname "$STDLIB")
PYDEST=$OUT/usr/${STDLIB#"$PYPREFIX"/}
SITE=$("$PY" -c 'import sysconfig; print(sysconfig.get_paths()["purelib"])')
mkdir -p "$PYDEST"
(cd "$STDLIB" && tar -cf - --exclude=__pycache__ --exclude=test --exclude=tests --exclude=idlelib --exclude=tkinter \
  --exclude=turtledemo --exclude=ensurepip --exclude=site-packages --exclude=dist-packages --exclude='config-*' .) | tar -xf - -C "$PYDEST"
mkdir -p "$PYDEST/site-packages"
# Debian keeps the compiled modules of the standard library beside it under another name.
DYNLOAD=$("$PY" -c 'import sysconfig; print(sysconfig.get_config_var("DESTSHARED"))')
if [ ! -d "$PYDEST/lib-dynload" ] && [ -d "$DYNLOAD" ]; then cp -a "$DYNLOAD" "$PYDEST/lib-dynload"; fi

# PySide6 and shiboken6, installed for this Python by fetch-pyside.sh: the build's own copies go, and the parts of the
# wheel the program has no use for (its Qt plugins, QML, translations and tools).
for p in PySide6 shiboken6; do
  [ -d "$SITE/$p" ] || { echo "PySide6 is missing from $SITE: run tools/python/fetch-pyside.sh" >&2; exit 1; }
  cp -a "$SITE/$p" "$PYDEST/site-packages/"
done
PS=$PYDEST/site-packages/PySide6
rm -rf "$PS/Qt/plugins" "$PS/Qt/qml" "$PS/Qt/translations" "$PS/Qt/libexec" "$PS/scripts" \
  "$PS"/{assistant,designer,linguist,lrelease,lupdate,qmllint,qmlformat,qmlls,qmlcachegen,qmlimportscanner,qmltyperegistrar,rcc,uic,svgtoqml}
# RPM's shebang mangling refuses this stdlib file as it is ("python", not "python3"); the bundle's Python runs it.
if [ -f "$PYDEST/encodings/rot_13.py" ]; then
  sed -i '1s|^#!/usr/bin/env python$|#!/usr/bin/env python3|' "$PYDEST/encodings/rot_13.py"
fi

# Every library the program, the plugins and Python's modules load, and what those load, except what every desktop has
# and what has to match the machine: the C library, the graphics drivers' side (GL, Vulkan, DRM), X11 and Wayland's own,
# fontconfig and freetype, and the like. The list follows the AppImage project's excludelist.
SKIP='^(ld-linux.*|libc|libm|libdl|libpthread|librt|libutil|libresolv|libnsl|libanl|libBrokenLocale|libnss_.*|libcrypt|libgcc_s|libstdc\+\+|libGL|libGLX|libGLdispatch|libEGL|libOpenGL|libGLESv2|libvulkan|libdrm|libgbm|libglapi|libX11|libX11-xcb|libxcb|libxcb-dri2|libxcb-dri3|libxcb-glx|libxcb-present|libxshmfence|libXau|libXdmcp|libXext|libXrender|libXfixes|libXi|libXcursor|libXrandr|libXdamage|libXxf86vm|libICE|libSM|libwayland-client|libwayland-cursor|libwayland-egl|libwayland-server|libfontconfig|libfreetype|libharfbuzz|libfribidi|libexpat|libz|libuuid|libcom_err|libgpg-error|libglib-2\.0|libgobject-2\.0|libgio-2\.0|libgmodule-2\.0|libgthread-2\.0|libdbus-1|libasound|libjack|libpipewire.*|libp11-kit|libusb-1\.0|libthai|libgmp|libselinux|libsystemd|libudev|libcap|libmount|libblkid|libtinfo)\.so'
copy_needed() {
  local lib name
  for lib in $(ldd "$1" 2>/dev/null | awk '/=> \// {print $3}'); do
    name=$(basename "$lib")
    if echo "$name" | grep -Eq "$SKIP" || [ -e "$LIB/$name" ]; then continue; fi
    cp -pL "$lib" "$LIB/$name"
    chmod 755 "$LIB/$name"
    copy_needed "$lib"
  done
}
export LD_LIBRARY_PATH="$QT/lib:$PYLIB${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
copy_needed "$OUT/usr/bin/aztek"
[ -e "$OUT/usr/bin/harold" ] && copy_needed "$OUT/usr/bin/harold"
for f in $(find "$OUT/usr/plugins" "$PYDEST/lib-dynload" -name '*.so'); do copy_needed "$f"; done
# PySide6 and shiboken6 bring what they load too: a wheel's Qt libraries are the same as the bundle's and go, the rest
# (OpenSSL, libffi) stay inside rather than being taken from whatever the machine has.
for f in $(find "$PYDEST/site-packages/PySide6" "$PYDEST/site-packages/shiboken6" -name '*.so*'); do copy_needed "$f"; done
for f in "$PS/Qt/lib"/*.so*; do
  [ -e "$f" ] && [ -e "$LIB/$(basename "$f")" ] && rm -f "$f"
done
find "$LIB" -maxdepth 1 -name '*.so*' -exec strip --strip-unneeded {} + 2>/dev/null || true

cat >"$OUT/AppRun" <<'RUN'
#!/bin/sh
# Starts Aztek from wherever it sits, with the Qt, Python and PySide6 inside it.
HERE=$(dirname "$(readlink -f "$0")")
export PYTHONHOME="$HERE/usr"
export PYTHONNOUSERSITE=1
export LD_LIBRARY_PATH="$HERE/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
exec "$HERE/usr/bin/aztek" "$@"
RUN
chmod 755 "$OUT/AppRun"

# Nothing inside may be left looking outside for a library that was meant to come along.
if LD_LIBRARY_PATH="$LIB" ldd "$OUT/usr/bin/aztek" | grep "not found"; then
  echo "unresolved libraries" >&2
  exit 1
fi
echo "$OUT"
