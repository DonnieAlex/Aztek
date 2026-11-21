# The versions every build bundles. One place, so the build scripts and the fetch scripts agree: the Python Aztek embeds
# (3.12, the reference's version) and the Qt and PySide6 built for it. PySide6 works only with the very Qt it was built
# for, and its shiboken6 must be the same version, so both carry the Qt version.
PYTHON_VERSION=3.12
QT_VERSION=6.8.3
