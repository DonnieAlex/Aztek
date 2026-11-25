# Small modules of the reference's that tools import at the top of their files. One source, run under each name.
_name = __name__

if _name == "maya.standalone":
    # aztekpy's commands work from the start; initialize brings in the user folder as the reference's does: its
    # paths, modules, plug-ins to load and userSetup scripts. In a window it has nothing to do.
    def initialize(name="python"):
        import _aztek

        _aztek.standalone_initialize()

    def uninitialize():
        import sys

        sys.stdout.flush()
        sys.stderr.flush()

elif _name == "maya.app.flux.core":
    def dpi(value):
        import aztek.OpenAztekUI as omui

        return omui.MQtUtil.dpiScale(value)

    def pix(value):
        return int(dpi(value))

    def getPixmap(name, width=None, height=None):
        """A named image as a QPixmap: a file of that name on the icon path (XBMLANGPATH), else Qt's resource of that
        name. Aztek carries none of the reference's own images, so one of those comes back empty, as a missing image
        does there. Scaled to width and height when given, as dpi scales them."""
        import os
        try:
            from PySide6 import QtGui
        except ImportError:
            from PySide2 import QtGui
        name = str(name)
        found = None
        if os.path.isfile(name):
            found = name
        else:
            for d in os.environ.get("XBMLANGPATH", "").split(os.pathsep):
                d = d.replace("%B", "")
                for ext in ("", ".png", ".svg", ".xpm"):
                    f = os.path.join(d, name + ext)
                    if d and os.path.isfile(f):
                        found = f
                        break
                if found:
                    break
        px = QtGui.QPixmap(found) if found else QtGui.QPixmap(":/" + name if name.endswith((".png", ".svg")) else ":/" + name + ".png")
        if not px.isNull() and (width or height):
            px = px.scaled(pix(width or px.width()), pix(height or px.height()))
        return px

elif _name == "maya.internal.nodes.proximitywrap.node_interface":
    class NodeInterface(object):
        def __init__(self, *args, **kwargs):
            raise RuntimeError("The proximity wrap deformer is not in Aztek")

elif _name.startswith("maya.app.renderSetup.model"):
    def __getattr__(attr):
        if attr.startswith("__"):
            raise AttributeError(attr)
        raise RuntimeError("Render Setup's Python model is not in Aztek (%s.%s)" % (_name, attr))
