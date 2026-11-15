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

elif _name == "maya.internal.nodes.proximitywrap.node_interface":
    class NodeInterface(object):
        def __init__(self, *args, **kwargs):
            raise RuntimeError("The proximity wrap deformer is not in Aztek")

elif _name.startswith("maya.app.renderSetup.model"):
    def __getattr__(attr):
        if attr.startswith("__"):
            raise AttributeError(attr)
        raise RuntimeError("Render Setup's Python model is not in Aztek (%s.%s)" % (_name, attr))
