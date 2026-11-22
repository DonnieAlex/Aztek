# The versions every build bundles. One place, so the build scripts and the fetch scripts agree: the Python Aztek embeds
# and the Qt and PySide6 built for it, all as the VFX Reference Platform CY2027 gives them (Python 3.13.x, Qt and
# PySide6 6.8.x). PySide6 works only with the very Qt it was built for, and its shiboken6 must be the same version, so
# both carry the Qt version. 6.8.3 is the last open source 6.8 release; its Linux wheels need glibc 2.28, so Rocky 8
# takes the same version as everything else.
PYTHON_VERSION=3.13
QT_VERSION=6.8.3
