"""Skin Tools' Python API, the reference's ngSkinTools2.api: layers on a skinCluster and the tools that work on them,
over the skinLayers and skinLayersTool commands."""

import types as _types

import maya.cmds as _cmds


def _skin_target(target):
    return target


class NamedPaintTarget(object):
    """Targets that are not influences."""

    MASK = "mask"
    DUAL_QUATERNION = "dq"


class PaintMode(object):
    replace = "replace"
    add = "add"
    scale = "scale"
    smooth = "smooth"
    sharpen = "sharpen"

    all = [replace, add, scale, smooth, sharpen]


class MirrorOptions(object):
    directionPositiveToNegative = "positiveToNegative"
    directionNegativeToPositive = "negativeToPositive"
    directionFlip = "flip"

    def __init__(self):
        self.mirrorWeights = True
        self.mirrorMask = True
        self.mirrorDq = True
        self.direction = MirrorOptions.directionPositiveToNegative


class VertexTransferMode(object):
    closestPoint = "closestPoint"
    vertexId = "vertexId"


class InfluenceMappingConfig(object):
    """Accepted for the reference's signatures: influences are matched by name, then by bind place."""

    def __init__(self):
        self.use_distance_matching = True
        self.use_name_matching = True
        self.globs = []

    @classmethod
    def transfer_defaults(cls):
        return cls()


class FloodSettings(object):
    def __init__(self):
        self.mode = PaintMode.replace
        self.intensity = 1.0
        self.iterations = 1
        self.influences_limit = 0
        self.mirror = False
        self.distribute_to_other_influences = True
        self.limit_to_component_selection = False
        self.use_volume_neighbours = False


def _target_arg(t):
    if t is None:
        return None
    if isinstance(t, (int,)):
        return t
    return str(t)


class Layer(object):
    """One layer. Properties read and write the layer on its skin."""

    def __init__(self, target, layer_id):
        self._target = target
        self.id = int(layer_id)

    def __repr__(self):
        return "Layer(%r, %d: %r)" % (self._target, self.id, self.name)

    def __eq__(self, other):
        return isinstance(other, Layer) and other.id == self.id and other._target == self._target

    def __hash__(self):
        return hash((self._target, self.id))

    def _info(self):
        return _cmds.skinLayers(self._target, q=True, layer=self.id)

    def _edit(self, **kw):
        _cmds.skinLayers(self._target, e=True, layer=self.id, **kw)

    @property
    def name(self):
        return self._info()["name"]

    @name.setter
    def name(self, v):
        self._edit(name=v)

    @property
    def enabled(self):
        return bool(self._info()["enabled"])

    @enabled.setter
    def enabled(self, v):
        self._edit(enabled=bool(v))

    @property
    def opacity(self):
        return float(self._info()["opacity"])

    @opacity.setter
    def opacity(self, v):
        self._edit(opacity=float(v))

    @property
    def index(self):
        return int(self._info()["index"])

    @index.setter
    def index(self, v):
        self._edit(index=int(v))

    @property
    def parent_id(self):
        p = self._info()["parent"]
        return None if p is None or p < 0 else int(p)

    @property
    def parent(self):
        p = self.parent_id
        return None if p is None else Layer(self._target, p)

    @parent.setter
    def parent(self, v):
        self._edit(parent=-1 if v is None else int(v.id if isinstance(v, Layer) else v))

    @property
    def children(self):
        return [Layer(self._target, c) for c in self._info()["children"]]

    @property
    def num_children(self):
        return len(self._info()["children"])

    @property
    def paint_target(self):
        t = self._info()["paintTarget"]
        return t

    @paint_target.setter
    def paint_target(self, v):
        self._edit(paintTarget=_target_arg(v))

    @property
    def locked_influences(self):
        return list(self._info()["locked"])

    @locked_influences.setter
    def locked_influences(self, v):
        self._edit(lockInfluences=[_target_arg(x) for x in v])

    def get_used_influences(self):
        return list(self._info()["influences"])

    def get_weights(self, influence):
        return list(_cmds.skinLayers(self._target, q=True, layer=self.id, weights=True, target=_target_arg(influence)) or [])

    def set_weights(self, influence, weights_list, undo_enabled=True):
        self._edit(target=_target_arg(influence), weights=list(weights_list))

    def configure_mirror(self, mirror_weights=True, mirror_mask=True, mirror_dq=True, everything=None):
        if everything is not None:
            mirror_weights = mirror_mask = mirror_dq = bool(everything)
        self._edit(mirrorEffects=[bool(mirror_weights), bool(mirror_mask), bool(mirror_dq)])

    def set_current(self):
        _cmds.skinLayers(self._target, current=self.id)


class Layers(object):
    """The layers of a skinned mesh (or skinCluster)."""

    def __init__(self, target):
        self.mesh = target
        self._target = target

    def __repr__(self):
        return "Layers(%r)" % self._target

    def is_initialized(self):
        return bool(_cmds.skinLayers(self._target, q=True, exists=True))

    def list(self):
        if not self.is_initialized():
            return []
        return [Layer(self._target, x["id"]) for x in _cmds.skinLayers(self._target, q=True, layers=True)]

    def add(self, name, force_empty=False, parent=None):
        if not self.is_initialized():
            init_layers(self._target)
        kw = {}
        if parent is not None:
            kw["parent"] = int(parent.id if isinstance(parent, Layer) else parent)
        return Layer(self._target, _cmds.skinLayers(self._target, add=name, **kw))

    def delete(self, layer):
        _cmds.skinLayers(self._target, remove=int(layer.id if isinstance(layer, Layer) else layer))

    def clear(self):
        for l in [x for x in self.list() if x.parent_id is None]:
            self.delete(l)

    def current_layer(self):
        c = _cmds.skinLayers(self._target, q=True, current=True)
        return None if c is None or c < 0 else Layer(self._target, c)

    def set_current_layer(self, layer):
        _cmds.skinLayers(self._target, current=int(layer.id if isinstance(layer, Layer) else layer))

    def layer_by_name(self, name):
        for l in self.list():
            if l.name == name:
                return l
        return None

    def duplicate(self, layer):
        return Layer(self._target, _cmds.skinLayers(self._target, duplicate=int(layer.id)))

    def merge_down(self, layer):
        return Layer(self._target, _cmds.skinLayers(self._target, mergeDown=int(layer.id)))

    def config(self):
        return _cmds.skinLayers(self._target, q=True, mirrorAxis=True)


def init_layers(target):
    """Starts layers on a skinned mesh: a base layer with the weights it has. Returns its Layers."""
    _cmds.skinLayers(target, initialize=True)
    return Layers(target)


def get_layers_enabled(target):
    return bool(_cmds.skinLayers(target, q=True, exists=True))


def delete_layers(target):
    _cmds.skinLayers(target, delete=True)


def _layer_id(layer):
    return int(layer.id if isinstance(layer, Layer) else layer)


def flood_weights(target, influence=None, settings=None):
    """Floods the current layer (or the target's layer, when target is a Layer) with settings' mode and intensity."""
    s = settings or FloodSettings()
    if isinstance(target, Layer):
        layer, mesh = target, target._target
    else:
        mesh, layer = target, None
    kw = dict(flood=True, mode=s.mode, intensity=float(s.intensity), iterations=int(s.iterations), distribute=bool(s.distribute_to_other_influences),
              selectedComponents=bool(s.limit_to_component_selection), brushMirror=bool(s.mirror))
    if layer is not None:
        kw["layer"] = layer.id
    if influence is not None:
        kw["target"] = _target_arg(influence)
    _cmds.skinLayersTool(mesh, **kw)


def assign_from_closest_joint(target, layer, influences=None):
    kw = {"assignFromClosestJoint": True, "layer": _layer_id(layer)}
    if influences:
        kw["influences"] = [_target_arg(i) for i in influences]
    _cmds.skinLayersTool(target, **kw)


def unify_weights(target, layer, overall_effect=1.0, single_cluster_mode=False):
    _cmds.skinLayersTool(target, unify=True, layer=_layer_id(layer), selectedComponents=True)


class Mirror(object):
    """How a skinned mesh mirrors: the axis, the seam width and the direction, and influence pairs."""

    def __init__(self, target):
        self.target = target

    def _q(self):
        return _cmds.skinLayers(self.target, q=True, mirrorAxis=True)

    @property
    def axis(self):
        return self._q()["axis"]

    @axis.setter
    def axis(self, v):
        if isinstance(v, int):
            v = "xyz"[v]
        _cmds.skinLayers(self.target, mirrorAxis=v)

    @property
    def seam_width(self):
        return self._q()["seamWidth"]

    @seam_width.setter
    def seam_width(self, v):
        _cmds.skinLayers(self.target, seamWidth=float(v))

    @property
    def mirror_direction(self):
        return self._q()["direction"]

    @mirror_direction.setter
    def mirror_direction(self, v):
        _cmds.skinLayers(self.target, mirrorDirection=v)

    def set_influences_mapping(self, mapping):
        _cmds.skinLayers(self.target, influencePairs=[[int(a), int(b)] for a, b in dict(mapping).items()])

    def build_influences_mapping(self, *args, **kwargs):
        return {a: b for a, b in _cmds.skinLayers(self.target, q=True, influencePairs=True)}

    def mirror(self, options=None, layer=None):
        o = options or MirrorOptions()
        if getattr(o, "direction", None):
            _cmds.skinLayers(self.target, mirrorDirection=o.direction)
        kw = dict(mirror=True, mirrorWeights=bool(o.mirrorWeights), mirrorMask=bool(o.mirrorMask), mirrorDq=bool(o.mirrorDq))
        if layer is not None:
            kw["layer"] = _layer_id(layer)
        _cmds.skinLayersTool(self.target, **kw)


def mirror_layer(target, layer, options=None):
    Mirror(target).mirror(options, layer)


def export_json(target, file):
    _cmds.skinLayersTool(target, exportFile=file)


def import_json(target, file, vertex_transfer_mode=VertexTransferMode.closestPoint, influences_mapping_config=None):
    _cmds.skinLayersTool(target, importFile=file, vertexTransferMode=vertex_transfer_mode)


def transfer_layers(source, destination, vertex_transfer_mode=VertexTransferMode.closestPoint, influences_mapping_config=None):
    _cmds.skinLayersTool(destination, transferFrom=source, vertexTransferMode=vertex_transfer_mode)


def _skin_cluster_of(target):
    if _cmds.nodeType(target) == "skinCluster":
        return target
    hist = _cmds.listHistory(target) or []
    found = _cmds.ls(hist, type="skinCluster") or []
    return found[0] if found else None


def _list_influences(target):
    return [x["path"] for x in _cmds.skinLayers(target, q=True, influences=True)]


def _influence_indexes(target):
    return {x["path"]: x["index"] for x in _cmds.skinLayers(target, q=True, influences=True)}


target_info = _types.SimpleNamespace(
    get_related_skin_cluster=_skin_cluster_of,
    get_related_data_node=lambda t: _cmds.skinLayers(t, q=True, dataNode=True) if get_layers_enabled(t) else None,
    list_influences=_list_influences,
    influence_indexes=_influence_indexes,
    is_slow_mode_skin_cluster=lambda t: False,
)
get_related_skin_cluster = _skin_cluster_of
list_influences = _list_influences


class _Brush(object):
    """The layer brush's settings, as the Skin Tools window sets them."""

    @property
    def mode(self):
        return _cmds.skinLayersTool(q=True, mode=True)["mode"]

    @mode.setter
    def mode(self, v):
        _cmds.skinLayersTool(mode=v)

    @property
    def intensity(self):
        return _cmds.skinLayersTool(q=True, mode=True)["intensity"]

    @intensity.setter
    def intensity(self, v):
        _cmds.skinLayersTool(intensity=float(v))

    @property
    def iterations(self):
        return _cmds.skinLayersTool(q=True, mode=True)["iterations"]

    @iterations.setter
    def iterations(self, v):
        _cmds.skinLayersTool(iterations=int(v))

    @property
    def mirror(self):
        return _cmds.skinLayersTool(q=True, mode=True)["mirror"]

    @mirror.setter
    def mirror(self, v):
        _cmds.skinLayersTool(brushMirror=bool(v))


paint = _types.SimpleNamespace(brush=_Brush(), PaintMode=PaintMode)


def open_ui():
    """Opens the Skin Tools window."""
    import maya.mel as _mel

    _mel.eval("SkinToolsWindow;")
