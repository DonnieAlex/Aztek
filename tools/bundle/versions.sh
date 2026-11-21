# The versions every build bundles. One place, so the build scripts and the fetch scripts agree: the Python Aztek embeds
# (3.12, the reference's version) and the Qt and PySide6 built for it. PySide6 works only with the very Qt it was built
# for, and its shiboken6 must be the same version, so both carry the Qt version.
PYTHON_VERSION=3.12

# PySide6's Linux wheels name the oldest C library they run on: 6.12's need glibc 2.34. Rocky 8 has 2.28, so it keeps
# 6.8.3, whose wheels need 2.28; everything else, macOS too, takes 6.12.0.
QT_VERSION=6.12.0
if [ "$(uname -s)" = Linux ]; then
  glibc=$(getconf GNU_LIBC_VERSION 2>/dev/null | awk '{print $2}')
  if [ -n "$glibc" ] && [ "$(printf '%s\n%s\n' "$glibc" 2.34 | sort -V | head -n1)" = "$glibc" ] && [ "$glibc" != 2.34 ]; then
    QT_VERSION=6.8.3
  fi
fi
