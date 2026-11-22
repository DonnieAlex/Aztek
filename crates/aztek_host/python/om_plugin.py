# aztek.api.OpenAztek, plugins: MPxCommand, MSyntax, MArgList, MArgDatabase, MPxNode, MDataBlock, MFnPlugin, and the
# entry points the host calls: loadPlugin, unloadPlugin, pluginInfo, running commands, computing nodes.
import os as _os
import traceback as _traceback


class MTypeId:
    def __init__(self, a=0, b=None):
        if isinstance(a, MTypeId):
            self._id = a._id
        elif b is not None:
            self._id = (int(a) << 8) | int(b)
        else:
            self._id = int(a)

    def id(self):
        return self._id

    def __eq__(self, o):
        return isinstance(o, MTypeId) and self._id == o._id

    def __ne__(self, o):
        return not self == o

    def __hash__(self):
        return hash(self._id)

    def __repr__(self):
        return "aztek.api.OpenAztek.MTypeId(0x%x)" % self._id


def _type_id_of(type_name):
    e = _node_classes.get(type_name)
    return MTypeId(e["id"]) if e else MTypeId(0)


def _node_type_of_id(type_id):
    for k, e in _node_classes.items():
        if e["id"] == type_id:
            return k
    _fail(message="No node type has that id")


class MArgList:
    def __init__(self, items=None):
        self._a = list(items) if items is not None else []

    def __len__(self):
        return len(self._a)

    def length(self):
        return len(self._a)

    def _at(self, i):
        if not 0 <= i < len(self._a):
            raise IndexError("MArgList index out of range")
        return self._a[i]

    def addArg(self, v):
        self._a.append(v)
        return self

    def flagIndex(self, shortFlag, longFlag=None):
        names = {"-" + shortFlag.lstrip("-")}
        if longFlag:
            names.add("-" + longFlag.lstrip("-"))
        for i, v in enumerate(self._a):
            if isinstance(v, str) and v in names:
                return i
        return 4294967295

    def asBool(self, i):
        v = self._at(i)
        return v.lower() in ("true", "on", "yes", "1") if isinstance(v, str) else bool(v)

    def asInt(self, i):
        return int(self._at(i))

    def asDouble(self, i):
        return float(self._at(i))

    asFloat = asDouble

    def asString(self, i):
        return str(self._at(i))

    def asAngle(self, i):
        return MAngle(float(self._at(i)), MAngle.uiUnit())

    def asDistance(self, i):
        return MDistance(float(self._at(i)), MDistance.uiUnit())

    def asTime(self, i):
        return MTime(float(self._at(i)))

    def asVector(self, i, numElements=3):
        return MVector(*[float(self._at(i + k)) for k in range(numElements)])

    def asPoint(self, i, numElements=3):
        return MPoint(*[float(self._at(i + k)) for k in range(numElements)])

    def asMatrix(self, i):
        return MMatrix([float(self._at(i + k)) for k in range(16)])

    def asStringArray(self, i):
        v = self._at(i)
        return list(v) if isinstance(v, (list, tuple)) else [str(v)]

    def asIntArray(self, i):
        v = self._at(i)
        return MIntArray(v if isinstance(v, (list, tuple)) else [v])

    def asDoubleArray(self, i):
        v = self._at(i)
        return MDoubleArray(v if isinstance(v, (list, tuple)) else [v])


class MSyntax:
    kInvalidArgType = 0
    kNoArg = 1
    kBoolean = 2
    kLong = 3
    kDouble = 4
    kString = 5
    kUnsigned = 6
    kDistance = 7
    kAngle = 8
    kTime = 9
    kSelectionItem = 10
    kLastArgType = 11
    kInvalidObjectFormat = 0
    kNone = 1
    kStringObjects = 2
    kSelectionList = 3
    kLastObjectFormat = 4

    def __init__(self):
        self._flags = []
        self._args = []
        self._object_format = MSyntax.kNone
        self._min = 0
        self._max = 0
        self._selection_default = False
        self.enableQuery = False
        self.enableEdit = False

    def addFlag(self, shortName, longName, *argTypes):
        types = []
        for t in argTypes:
            types.extend(t if isinstance(t, (list, tuple)) else [t])
        self._flags.append({"short": shortName.lstrip("-"), "long": longName.lstrip("-"), "types": types, "multi": False})
        return self

    def _flag(self, name):
        n = name.lstrip("-")
        for f in self._flags:
            if n in (f["short"], f["long"]):
                return f
        return None

    def addArg(self, t):
        self._args.append(t)
        return self

    def makeFlagMultiUse(self, name):
        f = self._flag(name)
        if f:
            f["multi"] = True
        return self

    def makeFlagQueryWithFullArgs(self, name, optional):
        return self

    def setObjectType(self, format, minimum=0, maximum=4294967295):
        self._object_format = format
        self._min, self._max = minimum, maximum
        return self

    def setMinObjects(self, n):
        self._min = n
        return self

    def setMaxObjects(self, n):
        self._max = n
        return self

    def minObjects(self):
        return self._min

    def maxObjects(self):
        return self._max

    def useSelectionAsDefault(self, on):
        self._selection_default = on
        return self

    def host_flags(self):
        out = []
        kinds = {MSyntax.kBoolean: "bool", MSyntax.kLong: "int", MSyntax.kUnsigned: "int", MSyntax.kDouble: "float", MSyntax.kDistance: "float", MSyntax.kAngle: "float", MSyntax.kTime: "float", MSyntax.kString: "string", MSyntax.kSelectionItem: "string"}
        for f in self._flags:
            t = f["types"]
            if not t or t == [MSyntax.kNoArg]:
                ty = "bool"
            elif len(t) == 1 and not f["multi"]:
                ty = kinds.get(t[0], "any")
            else:
                ty = "any"
            out.append({"name": f["long"], "short": f["short"], "type": ty})
        if self.enableQuery:
            out.append({"name": "query", "short": "q", "type": "bool"})
            # The flags a query asks about arrive here, without values.
            out.append({"name": "queried", "short": "", "type": "list"})
        if self.enableEdit:
            out.append({"name": "edit", "short": "e", "type": "bool"})
        return out


class MArgParser:
    def __init__(self, syntax, args):
        self._syntax = syntax
        self._uses = {}
        self._rest = []
        self._query = False
        self._edit = False
        items = list(args._a)
        i = 0
        while i < len(items):
            v = items[i]
            f = syntax._flag(v) if isinstance(v, str) and v.startswith("-") and len(v) > 1 and not _is_number(v) else None
            if isinstance(v, str) and v in ("-q", "-query") and syntax.enableQuery:
                self._query = True
                i += 1
                continue
            if isinstance(v, str) and v in ("-e", "-edit") and syntax.enableEdit:
                self._edit = True
                i += 1
                continue
            if f is None:
                self._rest.append(v)
                i += 1
                continue
            n = len(f["types"]) if f["types"] != [MSyntax.kNoArg] else 0
            vals = items[i + 1 : i + 1 + n]
            if len(vals) < n and not self._query:
                _fail(message="Flag -%s needs %d values" % (f["long"], n))
            self._uses.setdefault(f["long"], []).append(vals)
            i += 1 + n

    def isFlagSet(self, name):
        f = self._syntax._flag(name)
        return bool(f) and f["long"] in self._uses

    def numberOfFlagUses(self, name):
        f = self._syntax._flag(name)
        return len(self._uses.get(f["long"], [])) if f else 0

    def numberOfFlagsUsed(self):
        return sum(len(v) for v in self._uses.values())

    @property
    def isQuery(self):
        return _CBool(self._query)

    @property
    def isEdit(self):
        return _CBool(self._edit)

    def _value(self, name, index):
        f = self._syntax._flag(name)
        uses = self._uses.get(f["long"]) if f else None
        if not uses:
            _fail(status="kFailure", message="Flag -%s was not given" % (name.lstrip("-"),))
        vals = uses[0]
        if not 0 <= index < len(vals):
            _fail(message="Flag -%s has no value %d" % (name.lstrip("-"), index))
        return vals[index]

    def flagArgumentBool(self, name, index):
        return MArgList([self._value(name, index)]).asBool(0)

    def flagArgumentInt(self, name, index):
        return int(self._value(name, index))

    def flagArgumentDouble(self, name, index):
        return float(self._value(name, index))

    flagArgumentFloat = flagArgumentDouble

    def flagArgumentString(self, name, index):
        return str(self._value(name, index))

    def flagArgumentAngle(self, name, index):
        return MAngle(float(self._value(name, index)), MAngle.uiUnit())

    def flagArgumentDistance(self, name, index):
        return MDistance(float(self._value(name, index)), MDistance.uiUnit())

    def flagArgumentTime(self, name, index):
        return MTime(float(self._value(name, index)))

    def getFlagArgumentList(self, name, occurrence):
        f = self._syntax._flag(name)
        uses = self._uses.get(f["long"], []) if f else []
        if not 0 <= occurrence < len(uses):
            _fail(message="Flag was not used that many times")
        return MArgList(uses[occurrence])

    def getFlagArgumentPosition(self, name, occurrence):
        return occurrence

    def commandArgumentBool(self, index):
        return MArgList(self._rest).asBool(index)

    def commandArgumentInt(self, index):
        return MArgList(self._rest).asInt(index)

    def commandArgumentDouble(self, index):
        return MArgList(self._rest).asDouble(index)

    commandArgumentFloat = commandArgumentDouble

    def commandArgumentString(self, index):
        return MArgList(self._rest).asString(index)

    def commandArgumentAngle(self, index):
        return MArgList(self._rest).asAngle(index)

    def commandArgumentDistance(self, index):
        return MArgList(self._rest).asDistance(index)

    def commandArgumentTime(self, index):
        return MArgList(self._rest).asTime(index)

    def getObjectStrings(self):
        out = [str(v) for v in self._rest]
        if not out and self._syntax._selection_default:
            out = _cmds.ls(sl=True) or []
        return out


class MArgDatabase(MArgParser):
    def getObjectList(self):
        s = MSelectionList()
        for n in self.getObjectStrings():
            s.add(n)
        return s

    def getFlagArgument(self, name, index):
        return self._value(name, index)

    def selectionItem(self, name, index):
        s = MSelectionList()
        s.add(str(self._value(name, index)))
        return s

    def flagArgumentMSelectionList(self, name, index):
        return self.selectionItem(name, index)


def _is_number(s):
    try:
        float(s)
        return True
    except ValueError:
        return False


def _plain_result(v):
    if isinstance(v, (MAngle,)):
        return v.asRadians()
    if isinstance(v, MDistance):
        return v.asCentimeters()
    if isinstance(v, MTime):
        return v.value
    if isinstance(v, (MVector, MPoint)):
        return [v[0], v[1], v[2]]
    if isinstance(v, _MatrixBase):
        return list(v)
    if isinstance(v, (list, tuple)):
        return [_plain_result(x) for x in v]
    return v


class MPxCommand:
    kDouble = 0
    kLong = 1
    kString = 2
    kNoArg = 3
    # What the running command has set; commands nest.
    _result = None
    _has_result = False

    def __init__(self):
        self._syntax_obj = None
        self.commandString = ""

    def doIt(self, args):
        pass

    def undoIt(self):
        pass

    def redoIt(self):
        pass

    def isUndoable(self):
        return False

    def hasSyntax(self):
        return self._syntax_obj is not None

    def syntax(self):
        return self._syntax_obj if self._syntax_obj is not None else MSyntax()

    @classmethod
    def setResult(cls, value):
        MPxCommand._result = _plain_result(value)
        MPxCommand._has_result = True

    @classmethod
    def appendToResult(cls, value):
        if not isinstance(MPxCommand._result, list):
            MPxCommand._result = [] if MPxCommand._result is None else [MPxCommand._result]
        MPxCommand._result.append(_plain_result(value))
        MPxCommand._has_result = True

    @classmethod
    def clearResult(cls):
        MPxCommand._result = None
        MPxCommand._has_result = False

    @classmethod
    def currentResult(cls):
        return MPxCommand._result

    @classmethod
    def isCurrentResultArray(cls):
        return isinstance(MPxCommand._result, list)

    @classmethod
    def currentResultType(cls):
        r = MPxCommand._result
        if isinstance(r, bool) or isinstance(r, int):
            return MPxCommand.kLong
        if isinstance(r, float):
            return MPxCommand.kDouble
        if isinstance(r, str):
            return MPxCommand.kString
        return MPxCommand.kNoArg

    @staticmethod
    def displayInfo(msg):
        MGlobal.displayInfo(msg)

    @staticmethod
    def displayWarning(msg, showLineNumber=False):
        MGlobal.displayWarning(msg)

    @staticmethod
    def displayError(msg, showLineNumber=False):
        MGlobal.displayError(msg)


# ---------------------------------------------------------------------------------------------------------------------
# Nodes


class MDataHandle:
    def __init__(self, block, attr, value, element=None):
        self._block = block
        self._attr = attr
        self._v = value
        self._element = element

    def _unit(self):
        return self._block._units.get(self._attr)

    def _num(self):
        v = self._v
        if isinstance(v, (list, tuple)) or v is None:
            return 0.0
        return v

    def asBool(self):
        return bool(self._num())

    def asChar(self):
        return int(self._num())

    asShort = asChar
    asInt = asChar
    asLong = asChar
    asInt64 = asChar

    def asDouble(self):
        v = float(self._num())
        return v

    def asFloat(self):
        return _f32(self.asDouble())

    def asAngle(self):
        return MAngle(float(self._num()), MAngle.kRadians)

    def asDistance(self):
        return MDistance(float(self._num()), MDistance.kCentimeters)

    def asTime(self):
        return MTime(float(self._num()))

    def asString(self):
        return "" if self._v is None else str(self._v)

    def _triple(self):
        return list(self._v) if isinstance(self._v, (list, tuple)) and len(self._v) == 3 else [0.0, 0.0, 0.0]

    def asVector(self):
        return MVector(self._triple())

    def asFloatVector(self):
        return MFloatVector(self._triple())

    def asDouble3(self):
        return tuple(self._triple())

    asFloat3 = asDouble3

    def asInt3(self):
        return tuple(int(x) for x in self._triple())

    asShort3 = asInt3

    def asMatrix(self):
        return MMatrix(self._v) if isinstance(self._v, (list, tuple)) and len(self._v) == 16 else MMatrix()

    def asFloatMatrix(self):
        return MFloatMatrix(self.asMatrix())

    def child(self, attr):
        name = _attr_name(attr)
        if isinstance(self._v, dict) and name in self._v:
            # An element of an array of compounds: the child at this element.
            host, v = self._v[name]
            return MDataHandle(self._block, host, v, element=self._element)
        kids = self._block._children.get(self._attr)
        if isinstance(self._v, (list, tuple)) and kids and name in kids:
            return MDataHandle(self._block, name, self._v[kids.index(name)])
        return self._block.inputValue(attr)

    def _write(self, v):
        self._v = v
        self._block._write(self._attr, v, self._element)
        return self

    def setBool(self, v):
        return self._write(bool(v))

    def setChar(self, v):
        return self._write(int(v))

    setShort = setChar
    setInt = setChar
    setInt64 = setChar

    def setDouble(self, v):
        return self._write(float(v))

    setFloat = setDouble

    def setMAngle(self, a):
        return self._write(a.asRadians())

    def setMDistance(self, d):
        return self._write(d.asCentimeters())

    def setMTime(self, t):
        return self._write(t.value)

    def setString(self, v):
        return self._write(str(v))

    def set3Double(self, x, y, z):
        return self._write([float(x), float(y), float(z)])

    set3Float = set3Double

    def set3Int(self, x, y, z):
        return self._write([int(x), int(y), int(z)])

    set3Short = set3Int

    def setMVector(self, v):
        return self._write([v[0], v[1], v[2]])

    setMFloatVector = setMVector

    def setMMatrix(self, m):
        return self._write(list(m))

    setMFloatMatrix = setMMatrix
    setMatrix = setMMatrix

    def copy(self, other):
        return self._write(other._v)

    # A plug-in's data (MPxData): the object itself, handed across by the host.

    def asPluginData(self):
        v = self._v
        if isinstance(v, MPxData):
            return v
        if isinstance(v, dict) and "$data" in v:
            obj = _az.data_get(v["$data"])
            if obj is not None:
                self._v = obj
            return obj
        return None

    def data(self):
        if isinstance(self._v, dict) and "$mesh" in self._v:
            return self.asMesh()
        d = self.asPluginData()
        if d is None:
            _fail(status="kFailure", message="The handle holds no plug-in data")
        return _plugin_data_object(d)

    def setMPxData(self, data):
        if not isinstance(data, MPxData):
            _fail()
        return self._write(data)

    def asMesh(self):
        v = self._v
        if isinstance(v, dict) and "$mesh" in v:
            return _mesh_value_object(v["$mesh"])
        if isinstance(v, MObject):
            return v
        return MObject()

    asMeshTransformed = asMesh

    def setMObject(self, obj):
        if isinstance(obj, MObject) and obj._kind == "data" and obj._fn == MFn.kMeshData:
            if obj._data.key is None:
                _fail(message="The mesh data holds no mesh")
            return self._write({"$mesh": obj._data.key})
        if isinstance(obj, MObject) and obj._kind == "data" and isinstance(obj._data, MPxData):
            return self._write(obj._data)
        if isinstance(obj, MObject) and obj._kind == "data":
            d = obj._data
            return self._write(list(d) if isinstance(d, MMatrix) else d)
        _fail()

    def asMObject(self):
        return self.data()

    def setClean(self):
        return self

    def isNumeric(self):
        return isinstance(self._v, (int, float))


class MArrayDataHandle:
    def __init__(self, block, attr, elements):
        self._block = block
        self._attr = attr
        self._els = elements
        self._k = 0

    def __len__(self):
        return len(self._els)

    def elementCount(self):
        return len(self._els)

    def elementLogicalIndex(self):
        if not self._els:
            _fail(status="kFailure", message="Array has no elements")
        return self._els[self._k][0]

    def jumpToPhysicalElement(self, i):
        if not 0 <= i < len(self._els):
            _fail(message="Physical index out of range")
        self._k = i
        return self

    def jumpToLogicalElement(self, i):
        for k, e in enumerate(self._els):
            if e[0] == i:
                self._k = k
                return self
        _fail(message="No element at that index")

    def jumpToArrayElement(self, i):
        return self.jumpToPhysicalElement(i)

    def next(self):
        if self._k + 1 >= len(self._els):
            return False
        self._k += 1
        return True

    def isDone(self):
        return self._k >= len(self._els)

    def inputValue(self):
        if not self._els:
            _fail(status="kFailure", message="Array has no elements")
        e = self._els[self._k]
        return MDataHandle(self._block, self._attr, e[1], element=e[0])

    outputValue = inputValue

    def addElement(self, i):
        kids = self._block._groups.get(self._attr)
        if kids:
            v = {c: (h, d) for c, (h, d) in kids.items()}
            self._els.append([i, v])
            self._els.sort(key=lambda e: e[0])
            for k, e in enumerate(self._els):
                if e[0] == i:
                    self._k = k
            return MDataHandle(self._block, self._attr, v, element=i)
        self._els.append([i, None])
        self._els.sort(key=lambda e: e[0])
        for k, e in enumerate(self._els):
            if e[0] == i:
                self._k = k
        return MDataHandle(self._block, self._attr, None, element=i)

    def setAllClean(self):
        return self

    setClean = setAllClean


class MDataBlock:
    def __init__(self, values, units, children, time):
        self._values = values
        self._units = units
        self._children = children
        self._out = {}
        self._time = time
        self._clean = set()
        # Arrays of compounds: group -> {child: (the host's name for it, its default)}.
        self._groups = {}

    def _name(self, a):
        if isinstance(a, MPlug):
            return a._leaf_attr()
        n = _attr_name(a)
        if n is None:
            _fail()
        return n

    def _handle(self, a):
        n = self._name(a)
        if n in self._values:
            return MDataHandle(self, n, self._values[n])
        for parent, kids in self._children.items():
            if n in kids:
                v = self._values.get(parent)
                return MDataHandle(self, parent, v).child(n) if isinstance(v, (list, tuple)) else MDataHandle(self, n, 0.0)
        _fail(message="Attribute is not on this node")

    def inputValue(self, a):
        return self._handle(a)

    outputValue = inputValue

    def inputArrayValue(self, a):
        n = self._name(a)
        if n in self._groups:
            kids = self._groups[n]
            els = {}
            for child, (host, _) in kids.items():
                for k, v in self._values.get(host) or []:
                    els.setdefault(k, {})[child] = (host, v)
            for k in els:
                for child, (host, default) in kids.items():
                    els[k].setdefault(child, (host, default))
            return MArrayDataHandle(self, n, [[k, els[k]] for k in sorted(els)])
        v = self._values.get(n)
        if not isinstance(v, list):
            _fail(message="Attribute is not an array")
        return MArrayDataHandle(self, n, [list(e) for e in v])

    outputArrayValue = inputArrayValue

    def setClean(self, a):
        self._clean.add(self._name(a))
        return self

    def isClean(self, a):
        return self._name(a) in self._clean

    def context(self):
        return MDGContext()

    def _write(self, attr, v, element):
        if element is not None:
            self._out.setdefault(attr, {})[element] = v
        else:
            self._values[attr] = v
            self._out[attr] = v


class MDGContext:
    kNormal = None

    def __init__(self, time=None):
        self._time = time

    def isNormal(self):
        return self._time is None

    def getTime(self):
        return self._time if self._time is not None else MTime()


class MPxNode:
    # The reference's node kinds, in its order.
    kDependNode = 0
    kLocatorNode = 1
    kDeformerNode = 2
    kManipContainer = 3
    kSurfaceShape = 4
    kFieldNode = 5
    kEmitterNode = 6
    kSpringNode = 7
    kIkSolverNode = 8
    kHardwareShader = 9
    kHwShaderNode = 10
    kTransformNode = 11
    kObjectSet = 12
    kFluidEmitterNode = 13
    kImagePlaneNode = 14
    kParticleAttributeMapperNode = 15
    kCameraSetNode = 16
    kConstraintNode = 17
    kManipulatorNode = 18
    kMotionPathNode = 19
    kClientDeviceNode = 20
    kThreadedDeviceNode = 21
    kAssembly = 22
    kSkinCluster = 23
    kGeometryFilter = 24
    kBlendShape = 25
    kLast = 26
    # What postEvaluation is told.
    kEvaluatedIndirectly = 0
    kEvaluatedDirectly = 1
    kLeaveDirty = 2
    kPostEvaluationTypeLast = 3

    _registering = None

    def __init__(self):
        self._node_id = 0

    def compute(self, plug, dataBlock):
        return None

    def postConstructor(self):
        pass

    def thisMObject(self):
        return _node_object(self._node_id)

    def typeName(self):
        return self._type_name

    def typeId(self):
        return _type_id_of(self._type_name)

    def name(self):
        return _api("node.info", id=self._node_id)["name"]

    def isAbstractClass(self):
        return False

    def existWithoutInConnections(self, *a):
        return True

    def existWithoutOutConnections(self, *a):
        return True

    @classmethod
    def addAttribute(cls, attr):
        reg = MPxNode._registering
        if reg is None:
            _fail(status="kFailure", message="addAttribute belongs in the node's initialize function")
        if not isinstance(attr._attr, dict):
            _fail()
        reg["attrs"].append(attr._attr)

    @classmethod
    def attributeAffects(cls, whenChanges, isAffected):
        reg = MPxNode._registering
        if reg is None:
            _fail(status="kFailure", message="attributeAffects belongs in the node's initialize function")
        reg["affects"].append((_attr_name(whenChanges), _attr_name(isAffected)))

    @classmethod
    def inheritAttributesFrom(cls, name):
        _fail(status="kNotImplemented", message="Plug-in node types can't inherit attributes")


def _flatten_attrs(attrs):
    """The attrs the host holds: three numeric children make a vec3; other compounds hold their children flat."""
    out = []
    for s in attrs:
        if s["type"] == "compound":
            kids = s["children"]
            if len(kids) == 3 and all(k["type"] == "float" for k in kids) and not s.get("array"):
                d = dict(s)
                d["type"] = "vec3"
                d["default"] = [k.get("default") or 0.0 for k in kids]
                out.append(d)
            elif s.get("array"):
                # An array of compounds: each child an array of its own, named group.child, as the scene's ramps are
                # (value[3].value_Position is value.value_Position, element 3).
                for k in kids:
                    if k["type"] == "compound":
                        sub = k["children"]
                        if len(sub) == 3 and all(c["type"] == "float" for c in sub):
                            d = dict(k)
                            d["type"] = "vec3"
                            d["default"] = [c.get("default") or 0.0 for c in sub]
                            d["children"] = [{"name": s["name"] + "." + c["name"], "short": s["short"] + "." + c["short"]} for c in sub]
                        else:
                            _fail(status="kNotImplemented", message="Compounds inside an array of compounds aren't available: %s" % (k["name"],))
                    else:
                        d = dict(k)
                    d["name"] = s["name"] + "." + k["name"]
                    d["short"] = (s["short"] or s["name"]) + "." + (k["short"] or k["name"])
                    d["array"] = True
                    d["$group"] = s["name"]
                    d["$child"] = k["name"]
                    d.pop("parent", None)
                    out.append(d)
            else:
                out.extend(_flatten_attrs(kids))
        elif not s.get("parent") or s not in out:
            out.append(s)
    # Children of a compound were also added on their own, as the reference's initializers do.
    names = set()
    unique = []
    inside = set()
    for s in attrs:
        if s["type"] == "compound" and len(s["children"]) == 3 and not s.get("array"):
            inside.update(k["name"] for k in s["children"])
        if s["type"] == "compound" and s.get("array"):
            # Its children live as group.child: added on their own as well, they don't count again.
            for k in s["children"]:
                inside.add(k["name"])
                if k["type"] == "compound":
                    inside.update(c["name"] for c in k["children"])
    for s in out:
        if s["name"] in names or (s["name"] in inside and (s["type"] != "vec3" or "$group" not in s and s.get("parent"))):
            continue
        names.add(s["name"])
        unique.append(s)
    return unique


# The host's numbers for the kinds a plug-in node can be (the kit's ABI): a dependency node, a locator, a transform,
# a constraint, a shape, an IK solver.
_HOST_KINDS = {
    MPxNode.kDependNode: 0,
    MPxNode.kDeformerNode: 1,
    MPxNode.kGeometryFilter: 1,
    MPxNode.kLocatorNode: 2,
    MPxNode.kTransformNode: 3,
    MPxNode.kConstraintNode: 4,
    MPxNode.kSurfaceShape: 5,
    MPxNode.kIkSolverNode: 6,
}

# type name -> {"creator", "id", "owner", "units", "children", "outputs", "attrs", "kind"}
_node_classes = {}
# data type name -> {"creator", "id", "owner"}
_data_classes = {}
# file translator name -> {"creator", "owner", "options"}
_translator_classes = {}
_node_instances = {}
_command_classes = {}
# plugin name -> {"path", "vendor", "version", "api", "module", "commands", "nodes", "object"}
_plugins = {}
_loading = []


def _plugin_instance(node_id):
    info = _api("node.info", id=node_id)
    if not info or info["type"] not in _node_classes:
        _fail()
    return _instance(info["type"], node_id)


def _instance(type_name, node_id):
    inst = _node_instances.get(node_id)
    # Node ids are used again once a node has gone (a new scene, a file opened): an instance made for another type
    # under the same id is not this node's.
    if inst is not None and inst._type_name != type_name:
        inst = None
    if inst is None:
        e = _node_classes[type_name]
        inst = e["creator"]()
        inst._node_id = node_id
        inst._type_name = type_name
        _node_instances[node_id] = inst
        inst.postConstructor()
    return inst


class MFnPlugin(MFnBase):
    def __init__(self, obj=None, vendor="Unknown", version="Unknown", apiVersion="Any"):
        self._obj = obj if isinstance(obj, MObject) else MObject()
        if self._obj._kind != "plugin":
            _fail()
        p = self._obj._data
        self._p = p
        if vendor != "Unknown":
            p["vendor"] = vendor
        if version != "Unknown":
            p["version"] = version
        if apiVersion != "Any":
            p["api"] = apiVersion

    def vendor(self):
        return self._p["vendor"]

    def version(self):
        return self._p["version"]

    def apiVersion(self):
        return self._p["api"]

    def name(self):
        return self._p["name"]

    def setName(self, name, allowRename=True):
        return self

    def loadPath(self):
        return _os.path.dirname(self._p["path"])

    def registerUI(self, *a):
        return self

    def registerCommand(self, name, creator, syntaxCreator=None):
        if _az.exists(name):
            _fail(status="kFailure", message="A command named %s already exists" % (name,))
        syntax = syntaxCreator() if syntaxCreator is not None else None
        _az.register_command(name, self._p["name"], _json.dumps(syntax.host_flags() if syntax else []))
        _command_classes[name] = {"creator": creator, "syntax": syntax, "owner": self._p["name"]}
        if syntax is None:
            _main._plain_arg_commands.add(name)
        self._p["commands"].append(name)
        _forget_cached_command(name)
        return self

    def deregisterCommand(self, name):
        if name not in _command_classes:
            _fail(status="kFailure", message="No plug-in command named %s" % (name,))
        _az.deregister_command(name)
        del _command_classes[name]
        _main._plain_arg_commands.discard(name)
        if name in self._p["commands"]:
            self._p["commands"].remove(name)
        _forget_cached_command(name)
        return self

    def registerNode(self, name, typeId, creator, initialize=None, nodeType=MPxNode.kDependNode, classification=None):
        kind = _HOST_KINDS.get(nodeType)
        if kind is None:
            _fail(status="kNotImplemented", message="Plug-in nodes of that kind aren't available: %s" % (name,))
        return self._register(name, typeId, creator, initialize, kind)

    def registerShape(self, name, typeId, creator, initialize, uiCreator=None, classification=None):
        """A shape. The reference's 2027 API also takes no UI creator; one given is kept and not used."""
        return self._register(name, typeId, creator, initialize, _HOST_KINDS[MPxNode.kSurfaceShape])

    def registerData(self, name, typeId, creator, dataType=0):
        tid = MTypeId(typeId)
        if name in _data_classes or any(e["id"] == tid for e in _data_classes.values()):
            _fail(status="kFailure", message="A data type named %s or with that id already exists" % (name,))
        _az.register_data(name)
        _data_classes[name] = {"creator": creator, "id": tid, "owner": self._p["name"]}
        self._p.setdefault("data", []).append(name)
        return self

    def registerFileTranslator(self, name, pixmapName, creator, optionsScriptName=None, defaultOptionsString=None, requiresFullMel=False, dataStorage=None):
        t = creator()
        if not isinstance(t, MPxFileTranslator):
            _fail(status="kInvalidParameter", message="The creator didn't make an MPxFileTranslator")
        if name in _translator_classes:
            _fail(status="kFailure", message="A file translator named %s already exists" % (name,))
        _az.register_translator(name, self._p["name"], t.defaultExtension() or "", bool(t.haveReadMethod()), bool(t.haveWriteMethod()), defaultOptionsString or "", optionsScriptName or "")
        _translator_classes[name] = {"creator": creator, "owner": self._p["name"], "options": optionsScriptName}
        self._p.setdefault("translators", []).append(name)
        return self

    def deregisterFileTranslator(self, name):
        if name not in _translator_classes:
            _fail(status="kFailure", message="No file translator named %s" % (name,))
        _az.deregister_translator(name)
        del _translator_classes[name]
        if name in self._p.get("translators", []):
            self._p["translators"].remove(name)
        return self

    def deregisterData(self, typeId):
        tid = MTypeId(typeId)
        for name, e in list(_data_classes.items()):
            if e["id"] == tid:
                _az.deregister_data(name)
                del _data_classes[name]
                if name in self._p.get("data", []):
                    self._p["data"].remove(name)
                return self
        _fail(status="kFailure", message="No plug-in data type has that id")

    def _register(self, name, typeId, creator, initialize, kind, extra=None):
        reg = {"attrs": [], "affects": []}
        was = MPxNode._registering
        MPxNode._registering = reg
        try:
            if initialize is not None:
                initialize()
        finally:
            MPxNode._registering = was
        attrs = _flatten_attrs(reg["attrs"])
        # What gets computed: what something affects, and whatever can't be written from outside.
        affected = {b for _, b in reg["affects"]}
        outputs = []
        children = {}
        units = {}
        for s in attrs:
            if s["name"] in affected and "input" not in s:
                s["input"] = False
            if not s.get("input", True):
                outputs.append(s["name"])
            if s.get("unit"):
                units[s["name"]] = s["unit"]
                if s["unit"] == "angle":
                    _PLUGIN_ANGLES.add((name, s["name"]))
            if s["type"] == "vec3":
                kids = s.get("children") or [{"name": s["name"] + a, "short": s["short"] + a.lower()} for a in "XYZ"]
                children[s["name"]] = [k["name"] for k in kids]
        groups = {}
        for s in attrs:
            if "$group" in s:
                groups.setdefault(s["$group"], {})[s["$child"]] = (s["name"], s.get("default"))
        host_attrs = [{k: v for k, v in a.items() if not k.startswith("$")} for a in attrs]
        _az.register_node(name, self._p["name"], _json.dumps(host_attrs), kind, MTypeId(typeId).id() & 0xFFFFFFFF)
        _node_classes[name] = {
            "creator": creator, "id": MTypeId(typeId), "owner": self._p["name"], "outputs": outputs, "units": units, "children": children,
            "attrs": attrs, "kind": kind, "groups": groups,
        }
        _node_classes[name].update(extra or {})
        self._p["nodes"].append(name)
        return self

    def deregisterNode(self, typeId):
        tid = MTypeId(typeId)
        for name, e in list(_node_classes.items()):
            if e["id"] == tid:
                _az.deregister_node(name)
                del _node_classes[name]
                if name in self._p["nodes"]:
                    self._p["nodes"].remove(name)
                return self
        _fail(status="kFailure", message="No plug-in node has that id")


def _forget_cached_command(name):
    if name in _cmds.__dict__:
        del _cmds.__dict__[name]


def _plugin_search_path():
    out = []
    for var in ("AZTEK_PLUG_IN_PATH", "MAYA_PLUG_IN_PATH"):
        out.extend(p for p in _os.environ.get(var, "").split(_os.pathsep) if p)
    return out


def _find_plugin_file(name):
    if _os.path.isfile(name):
        return _os.path.abspath(name)
    file = name if name.endswith(".py") else name + ".py"
    for d in _plugin_search_path():
        p = _os.path.join(d, file)
        if _os.path.isfile(p):
            return p
    return None


def _plugin_key(name):
    base = _os.path.basename(name)
    return base[:-3] if base.endswith(".py") else base


def _error_answer(e):
    return _json.dumps({"error": str(e) or type(e).__name__})


def _show_traceback(e):
    lines = _traceback.format_exception(type(e), e, e.__traceback__)
    for l in "".join(lines[1:-1]).rstrip().splitlines():
        _az.write("# " + l, 2)


class _Hooks:
    """What the host calls."""

    def translate(self, name, mode, path, options):
        """A plug-in translator's reader or writer for a file command."""
        e = _translator_classes.get(name)
        if e is None:
            return _error_answer("No file translator %s" % (name,))
        modes = {"open": MPxFileTranslator.kOpenAccessMode, "import": MPxFileTranslator.kImportAccessMode,
                 "reference": MPxFileTranslator.kReferenceAccessMode, "save": MPxFileTranslator.kSaveAccessMode,
                 "exportAll": MPxFileTranslator.kExportAccessMode, "exportSelected": MPxFileTranslator.kExportActiveAccessMode}
        try:
            t = e["creator"]()
            f = MFileObject()
            f.setRawFullName(path)
            # The file the command names, even one still to be written.
            f.overrideResolvedFullName(path)
            m = modes.get(mode, MPxFileTranslator.kUnknownAccessMode)
            MPxFileTranslator._mode = m
            if mode in ("open", "import", "reference"):
                if not t.haveReadMethod():
                    return _error_answer("The %s translator can't read" % (name,))
                got = t.reader(f, options, m)
            else:
                if not t.haveWriteMethod():
                    return _error_answer("The %s translator can't write" % (name,))
                got = t.writer(f, options, m)
            if got is False:
                return _error_answer("The %s translator failed" % (name,))
            return _json.dumps({"ok": path})
        except Exception as ex:
            _show_traceback(ex)
            return _error_answer(ex)
        finally:
            MPxFileTranslator._mode = MPxFileTranslator.kUnknownAccessMode
            _sys.stdout.flush()

    def load_plugin(self, input):
        a = _json.loads(input)
        try:
            names = []
            for n in a["args"]:
                names.append(_load_one(n, a["flags"].get("quiet")))
            return _json.dumps({"ok": [n for n in names if n]})
        except Exception as e:
            _show_traceback(e)
            return _error_answer(e)

    def unload_plugin(self, input):
        a = _json.loads(input)
        try:
            for n in a["args"]:
                _unload_one(n)
            return _json.dumps({"ok": None})
        except Exception as e:
            _show_traceback(e)
            return _error_answer(e)

    def plugin_info(self, input):
        a = _json.loads(input)
        try:
            return _json.dumps({"ok": _plugin_info(a["args"], a["flags"])})
        except Exception as e:
            return _error_answer(e)

    def command_new(self, name):
        e = _command_classes.get(name)
        if e is None:
            raise RuntimeError("(kFailure): The plug-in command %s is gone" % (name,))
        inst = e["creator"]()
        inst._syntax_obj = e["syntax"]
        return (inst, bool(inst.isUndoable()))

    def command_do(self, inst, input):
        a = _json.loads(input)
        items = []
        syntax = inst._syntax_obj
        if syntax is not None:
            for k, v in a["flags"].items():
                if k == "queried":
                    continue
                if k == "query":
                    items.append("-q")
                    continue
                if k == "edit":
                    items.append("-e")
                    continue
                f = syntax._flag(k)
                if f is None:
                    continue
                items.append("-" + f["long"])
                if f["types"] and f["types"] != [MSyntax.kNoArg]:
                    items.extend(v if isinstance(v, list) else [v])
                elif v is False:
                    items.pop()
        for v in a["args"]:
            items.extend(v if isinstance(v, list) else [v])
        # Last, so nothing after them reads as their values.
        for k in a["flags"].get("queried") or []:
            items.append("-" + k)
        saved = (MPxCommand._result, MPxCommand._has_result)
        MPxCommand._result, MPxCommand._has_result = None, False
        try:
            try:
                inst.doIt(MArgList(items))
                _flush_points()
            except Exception as e:
                _show_traceback(e)
                return _error_answer(e)
            return _json.dumps({"ok": MPxCommand._result})
        finally:
            MPxCommand._result, MPxCommand._has_result = saved
            _sys.stdout.flush()

    def command_replay(self, inst, method):
        try:
            getattr(inst, method)()
            _flush_points()
        except Exception as e:
            _show_traceback(e)
            _az.write("# Error: %s #" % (e,), 2)
        finally:
            _sys.stdout.flush()
        return ""

    def compute(self, type_name, node_id, name, time, values):
        e = _node_classes.get(type_name)
        # Transforms, shapes and locators give their matrix and bounds even with no outputs of their own.
        if e is None or not (e["outputs"] or e["kind"] in (1, 2, 3, 4, 5)):
            return "{}"
        vals = _json.loads(values)
        if e["kind"] == 1:
            return self._deform(type_name, node_id, name, time, e, vals)
        ctx = {"id": node_id, "name": name, "type": type_name, "owner": e["owner"], "attrs": e["attrs"], "values": vals, "children": e["children"]}
        ctx["command"] = lambda n, a, k: _compute_command(ctx, n, a, k)
        _compute_local.ctx = ctx
        try:
            inst = _instance(type_name, node_id)
            block = MDataBlock(vals, e["units"], e["children"], time)
            block._groups = e.get("groups", {})
            for out in e["outputs"]:
                inst.compute(MPlug._of(node_id, out), block)
            res = _host_outputs(block._out)
            local = getattr(inst, "_az_local_matrix", None)
            if local is not None:
                m = local(block)
                if m is not None:
                    m = MMatrix(m)
                    res["$matrix"] = [m[i] for i in range(16)]
            if e["kind"] in (2, 5) and inst.isBounded():
                b = inst.boundingBox()
                lo, hi = b.min, b.max
                res["$bbox"] = [lo.x, lo.y, lo.z, hi.x, hi.y, hi.z]
            return _json.dumps(res)
        except Exception as ex:
            _show_traceback(ex)
            _az.write("# Error: %s: %s #" % (type(ex).__name__, ex), 2)
            return "{}"
        finally:
            _compute_local.ctx = None
            _sys.stdout.flush()

    def _deform(self, type_name, node_id, name, time, e, vals):
        """A deformer: its deform() moves the mesh's points, which go back as $points."""
        pts = vals.pop("$points", None)
        weights = vals.pop("$weights", None)
        gm = vals.pop("$geomMatrix", None)
        if pts is None:
            return "{}"
        ctx = {"id": node_id, "name": name, "type": type_name, "owner": e["owner"], "attrs": e["attrs"], "values": vals, "children": e["children"]}
        ctx["command"] = lambda n, a, k: _compute_command(ctx, n, a, k)
        _compute_local.ctx = ctx
        try:
            inst = _instance(type_name, node_id)
            block = MDataBlock(vals, e["units"], e["children"], time)
            block._groups = e.get("groups", {})
            block._az_weights = weights
            it = _DeformIterator([MPoint(pts[i], pts[i + 1], pts[i + 2]) for i in range(0, len(pts), 3)])
            m = MMatrix(gm) if gm else MMatrix()
            inst.deform(block, it, m, 0)
            res = _host_outputs(block._out)
            out = []
            for p in it._pts:
                out.extend((p.x, p.y, p.z))
            res["$points"] = out
            return _json.dumps(res)
        except Exception as ex:
            _show_traceback(ex)
            _az.write("# Error: %s: %s #" % (type(ex).__name__, ex), 2)
            return "{}"
        finally:
            _compute_local.ctx = None
            _sys.stdout.flush()

    def data_write(self, obj):
        """A plug-in data value as its type writes it to a file."""
        try:
            return _data_text(obj)
        except Exception as ex:
            _show_traceback(ex)
            return ""

    def data_read(self, kind, text):
        """A plug-in data value read from a file; None when its type refuses it."""
        e = _data_classes.get(kind)
        if e is None:
            return None
        try:
            obj = e["creator"]()
            got = obj.readASCII(MArgList(_data_words(text)), 0)
            return None if got is False else obj
        except Exception as ex:
            _show_traceback(ex)
            return None


def _compute_command(ctx, name, args, kwargs):
    """The commands a compute can run: getAttr of its own node's values."""
    if name == "getAttr" and args and not kwargs:
        node, _, attr = str(args[0]).partition(".")
        if node.split("|")[-1] == ctx["name"].split("|")[-1]:
            vals = ctx["values"]
            if attr in vals:
                v = vals[attr]
                return [tuple(v)] if isinstance(v, list) and len(v) == 3 and all(isinstance(x, (int, float)) for x in v) else v
            for parent, kids in ctx["children"].items():
                if attr in kids and isinstance(vals.get(parent), list):
                    return vals[parent][kids.index(attr)]
    raise RuntimeError("(kFailure): Commands cannot run inside compute")


def _data_words(text):
    """A data value's words as the reference's file reader gives them to readASCII: quoted strings whole."""
    import shlex

    try:
        return shlex.split(text or "")
    except ValueError:
        return (text or "").split()


class _AsciiOut:
    """What a 1.0 writeASCII writes into."""

    def __init__(self):
        self.parts = []

    def write(self, s):
        self.parts.append(str(s))

    def __lshift__(self, s):
        self.write(s)
        return self


def _data_text(obj):
    try:
        out = obj.writeASCII()
    except TypeError:
        # 1.0: writeASCII(ostream).
        o = _AsciiOut()
        obj.writeASCII(o)
        out = "".join(o.parts)
    return "" if out is None else str(out)


def _data_type_name(obj):
    try:
        tid = MTypeId(obj.typeId())
        for name, e in _data_classes.items():
            if e["id"] == tid:
                return name
    except Exception:
        pass
    return obj.name()


def _plugin_data_object(d):
    o = MObject()
    o._kind, o._fn, o._data = "data", MFn.kPluginData, d
    return o


# ---------------------------------------------------------------------------------------------------------------------
# Plug-in data and shapes


class MPxData:
    """A plug-in's own data type: what flows through its attributes and is written to files."""

    kData = 0
    kGeometryData = 1
    kLast = 2

    def __init__(self):
        pass

    def copy(self, src):
        _fail(status="kNotImplemented", message="MPxData.copy must be implemented by the data class")

    def typeId(self):
        _fail(status="kNotImplemented", message="MPxData.typeId must be implemented by the data class")

    def name(self):
        _fail(status="kNotImplemented", message="MPxData.name must be implemented by the data class")

    def readASCII(self, argList, endOfTheLastParsedElement):
        return endOfTheLastParsedElement

    def writeASCII(self):
        return ""

    def readBinary(self, data, length):
        return -1

    def writeBinary(self):
        return bytearray()


class MFnPluginData(MFnBase):
    def _check(self, obj):
        if obj._kind != "data" or not isinstance(obj._data, MPxData):
            _fail()

    def create(self, typeId):
        tid = MTypeId(typeId)
        for e in _data_classes.values():
            if e["id"] == tid:
                self._obj = _plugin_data_object(e["creator"]())
                return MObject(self._obj)
        _fail(message="No plug-in data type has that id")

    def data(self):
        self._check(self._obj)
        return self._obj._data

    def typeId(self):
        return MTypeId(self.data().typeId())


def _builtin_attr(name):
    """An attribute every node of a kind has, as the base classes' static members hold them."""
    o = MObject()
    o._kind, o._attr, o._fn = "attr", name, MFn.kAttribute
    return o


class MPxSurfaceShape(MPxNode):
    """A shape of a plug-in's own: what it draws and selects is the plug-in's; here its bounds count."""

    kObjectChanged = 0
    kBoundingBoxChanged = 1
    kMatchOk = 0
    kMatchNone = 1
    kMatchTooMany = 2
    kMatchInvalidName = 3
    kMatchInvalidAttribute = 4
    kMatchInvalidAttributeIndex = 5
    kMatchInvalidAttributeRange = 6
    kMatchInvalidAttributeDim = 7
    kNoPointCaching = 0
    kSavePoints = 1
    kRestorePoints = 2
    kUpdatePoints = 3
    kTransformOriginalPoints = 4
    kNormal = 0
    kUTangent = 1
    kVTangent = 2
    kUVNTriad = 3

    def isBounded(self):
        return False

    def boundingBox(self):
        return MBoundingBox()

    def geometryData(self):
        return MObject()

    def localShapeInAttr(self):
        return MObject()

    def localShapeOutAttr(self):
        return MObject()

    def worldShapeOutAttr(self):
        return MObject()

    def cachedShapeAttr(self):
        return MObject()

    def childChanged(self, state=0):
        pass

    def deleteComponents(self, componentList, undoInfo):
        return False

    def undeleteComponents(self, componentList, undoInfo):
        return False

    def componentToPlugs(self, component, selectionList):
        pass

    def match(self, mask, componentList):
        return False

    def matchComponent(self, item, spec, list):
        return MPxSurfaceShape.kMatchNone

    def createFullVertexGroup(self):
        return MObject()

    def createFullRenderGroup(self):
        return MObject()

    def hasActiveComponents(self):
        return False

    def activeComponents(self):
        return MObjectArray()

    def acceptsGeometryIterator(self, *a):
        return False

    def excludeAsPluginShape(self):
        return True

    def canMakeLive(self):
        return False

    def getWorldMatrix(self, block, instance):
        return MMatrix(_api("dag.matrices", id=self._node_id)["world"])


for _n in (
    "boundingBoxCenterX", "boundingBoxCenterY", "boundingBoxCenterZ", "center", "instObjGroups", "intermediateObject",
    "inverseMatrix", "isTemplated", "matrix", "nodeBoundingBox", "nodeBoundingBoxMax", "nodeBoundingBoxMaxX",
    "nodeBoundingBoxMaxY", "nodeBoundingBoxMaxZ", "nodeBoundingBoxMin", "nodeBoundingBoxMinX", "nodeBoundingBoxMinY",
    "nodeBoundingBoxMinZ", "nodeBoundingBoxSize", "nodeBoundingBoxSizeX", "nodeBoundingBoxSizeY", "nodeBoundingBoxSizeZ",
    "objectColor", "objectGroupColor", "objectGroupId", "objectGroups", "objectGrpCompList", "parentInverseMatrix",
    "parentMatrix", "visibility", "worldInverseMatrix", "worldMatrix", "mControlPoints", "mControlValueX",
    "mControlValueY", "mControlValueZ", "mHasHistoryOnCreate",
):
    # The reference's nodeBoundingBox members are the boundingBox attributes.
    setattr(MPxSurfaceShape, _n, _builtin_attr(_n.replace("nodeBoundingBox", "boundingBox")))
del _n


def _host_value(v):
    """An output as the host takes it: a plug-in's data object by the key the host keeps it under."""
    if isinstance(v, MPxData):
        return {"$data": _az.data_put(v, _data_type_name(v))}
    return v


def _host_outputs(out):
    res = {}
    for k, v in out.items():
        # A dict is an array's elements by index, but for a mesh handed on by key.
        array = isinstance(v, dict) and "$mesh" not in v
        res[k] = sorted([[i, _host_value(x)] for i, x in v.items()]) if array else _host_value(v)
    return res


def _load_one(name, quiet):
    key = _plugin_key(name)
    if key in _plugins:
        return key
    path = _find_plugin_file(name)
    if path is None:
        if quiet:
            return None
        raise RuntimeError('Plug-in "%s" was not found on the plug-in path.' % (name,))
    import importlib.util

    spec = importlib.util.spec_from_file_location("_aztek_plugin_" + key, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    # Without maya_useNewAPI() a plug-in is an API 1.0 one: it registers through maya.OpenMayaMPx.MFnPlugin.
    if not hasattr(module, "initializePlugin"):
        raise RuntimeError('Plug-in "%s" has no initializePlugin.' % (key,))
    p = {"name": key, "path": path, "vendor": "Unknown", "version": "Unknown", "api": "Any", "module": module, "commands": [], "nodes": []}
    obj = MObject()
    obj._kind, obj._fn, obj._data = "plugin", MFn.kInvalid, p
    p["object"] = obj
    try:
        module.initializePlugin(obj)
    except Exception:
        _take_back(p)
        raise
    _plugins[key] = p
    return key


def _take_back(p):
    """What a plug-in still has registered goes when it does."""
    for name in list(p["commands"]):
        if name in _command_classes:
            _az.deregister_command(name)
            del _command_classes[name]
            _main._plain_arg_commands.discard(name)
            _forget_cached_command(name)
    for name in list(p["nodes"]):
        if name in _node_classes:
            _az.deregister_node(name)
            del _node_classes[name]
    for name in list(p.get("data", [])):
        if name in _data_classes:
            _az.deregister_data(name)
            del _data_classes[name]
    for name in list(p.get("translators", [])):
        if name in _translator_classes:
            _az.deregister_translator(name)
            del _translator_classes[name]
    p["commands"], p["nodes"], p["data"], p["translators"] = [], [], [], []


def _unload_one(name):
    key = _plugin_key(name)
    p = _plugins.get(key)
    if p is None:
        raise RuntimeError('Plug-in "%s" is not loaded.' % (name,))
    if hasattr(p["module"], "uninitializePlugin"):
        p["module"].uninitializePlugin(p["object"])
    _take_back(p)
    del _plugins[key]


def _plugin_info(args, flags):
    if flags.get("listPlugins"):
        return sorted(_plugins)
    name = args[0] if args else None
    if name is None:
        _fail(message="pluginInfo: name a plug-in")
    p = _plugins.get(_plugin_key(name))
    if p is None:
        p = next((x for x in _plugins.values() if x["path"] == name), None)
    if flags.get("__record__"):
        # Everything it registered, for the host's pluginInfo.
        if p is None:
            return None
        return {
            "name": p["name"], "path": p["path"], "vendor": p["vendor"], "version": p["version"], "api": p["api"],
            "commands": list(p["commands"]), "nodes": list(p["nodes"]), "data": list(p.get("data", [])),
            "translators": list(p.get("translators", [])),
        }
    if flags.get("loaded"):
        return p is not None
    if flags.get("registered"):
        return p is not None or _find_plugin_file(name) is not None
    if p is None:
        raise RuntimeError('Plug-in "%s" is not loaded.' % (name,))
    for flag, key in (("version", "version"), ("vendor", "vendor"), ("path", "path"), ("name", "name"), ("apiVersion", "api")):
        if flags.get(flag):
            return p[key]
    if flags.get("command"):
        return list(p["commands"])
    if flags.get("dependNode"):
        return list(p["nodes"])
    _fail(message="pluginInfo: nothing asked")


_az.set_hooks(_Hooks())


# ---------------------------------------------------------------------------------------------------------------------
# Deformers (MPxGeometryFilter, MPxDeformerNode, in OpenAztekAnim as the reference's are in OpenMayaAnim)


class _DeformIterator:
    """The points a deform() is handed: MItGeometry's walk over the deformed mesh's points."""

    def __init__(self, pts):
        self._pts = pts
        self._k = 0

    def count(self):
        return len(self._pts)

    exactCount = count

    def index(self):
        return self._k

    def isDone(self):
        return self._k >= len(self._pts)

    def next(self):
        self._k += 1
        return self

    def reset(self, *a):
        self._k = 0
        return self

    def position(self, space=0):
        return MPoint(self._pts[self._k])

    def setPosition(self, point, space=0):
        self._pts[self._k] = MPoint(point.x, point.y, point.z)
        return self

    def normal(self, space=0):
        return MVector()

    def allPositions(self, space=0):
        return MPointArray([MPoint(p) for p in self._pts])

    def setAllPositions(self, points, space=0):
        if len(points) != len(self._pts):
            _fail(message="setAllPositions: %d points for %d" % (len(points), len(self._pts)))
        self._pts = [MPoint(p.x, p.y, p.z) for p in points]
        return self

    def currentItem(self):
        return _component_object(MFn.kMeshVertComponent, [self._k])

    def component(self):
        return self.currentItem()


class _MPxGeometryFilter(MPxNode):
    """A deformer: deform() moves the points of the mesh in; envelope, weights and the mesh's world matrix come with
    them. A compute of its own is not called."""

    def deform(self, dataBlock, geomIter, localToWorldMatrix, multiIndex):
        return None

    def accessoryAttribute(self):
        return MObject()

    def accessoryNodeSetup(self, cmd):
        return None

    def setUseExistingConnectionWhenSetEditing(self, state):
        return None

    def getDeformationDetails(self):
        return 0

    def setDeformationDetails(self, details):
        return None

    def getInputGeom(self, dataBlock, multiIndex):
        return MObject()

    def weightValue(self, dataBlock, multiIndex, index):
        """The painted weight of a point: 1 where none was painted."""
        w = getattr(dataBlock, "_az_weights", None)
        if not w or index < 0 or index >= len(w):
            return 1.0
        return float(w[index])


class _MPxDeformerNode(_MPxGeometryFilter):
    pass


for _n, _a in (("input", "inputPolymesh"), ("inputGeom", "inputPolymesh"), ("outputGeom", "output"), ("envelope", "envelope"),
               ("groupId", "deformerSet"), ("originalGeometry", "inputPolymesh")):
    setattr(_MPxGeometryFilter, _n, _builtin_attr(_a))
for _n in ("weightList", "weights"):
    setattr(_MPxDeformerNode, _n, _builtin_attr("weightList"))
del _n, _a


# ---------------------------------------------------------------------------------------------------------------------
# Ramps (MRampAttribute): an array of compounds, Position, FloatValue (or Color) and Interp, as the reference's.


def _ramp_eval(pts, t):
    """The ramp at t from sorted (position, value, interp) points: None holds, Linear, Smooth, Spline (Catmull-Rom)."""
    if not pts:
        return None
    if t <= pts[0][0]:
        return pts[0][1]
    if t >= pts[-1][0]:
        return pts[-1][1]
    k = max(i for i, p in enumerate(pts) if p[0] <= t)
    a, c = pts[k], pts[k + 1]
    span = c[0] - a[0]
    u = (t - a[0]) / span if span > 0 else 0.0

    def mix(x, y, f):
        return [x[i] + (y[i] - x[i]) * f for i in range(len(x))]

    if a[2] == 0:
        return a[1]
    if a[2] == 2:
        return mix(a[1], c[1], u * u * (3 - 2 * u))
    if a[2] == 3:
        before = pts[k - 1][1] if k > 0 else a[1]
        after = pts[k + 2][1] if k + 2 < len(pts) else c[1]
        u2, u3 = u * u, u * u * u
        return [a[1][i] * (2 * u3 - 3 * u2 + 1) + (c[1][i] - before[i]) * 0.5 * (u3 - 2 * u2 + u) + c[1][i] * (-2 * u3 + 3 * u2)
                + (after[i] - a[1][i]) * 0.5 * (u3 - u2) for i in range(len(a[1]))]
    return mix(a[1], c[1], u)


class MRampAttribute:
    kNone = 0
    kLinear = 1
    kSmooth = 2
    kSpline = 3

    @staticmethod
    def _create(longName, shortName, color):
        fc = MFnCompoundAttribute()
        o = fc.create(longName, shortName)
        n = MFnNumericAttribute()
        fc.addChild(n.create(longName + "_Position", shortName + "p", MFnNumericData.kFloat, 0.0))
        if color:
            fc.addChild(n.createColor(longName + "_Color", shortName + "c"))
        else:
            fc.addChild(n.create(longName + "_FloatValue", shortName + "fv", MFnNumericData.kFloat, 0.0))
        e = MFnEnumAttribute()
        i = e.create(longName + "_Interp", shortName + "i", 1)
        for k, name in enumerate(("None", "Linear", "Smooth", "Spline")):
            e.addField(name, k)
        fc.addChild(i)
        fc.array = True
        o._attr["ramp"] = "color" if color else "curve"
        return o

    @staticmethod
    def createCurveRamp(longName, shortName):
        return MRampAttribute._create(longName, shortName, False)

    @staticmethod
    def createColorRamp(longName, shortName):
        return MRampAttribute._create(longName, shortName, True)

    def __init__(self, *args):
        self._node = None
        self._name = None
        if len(args) == 1 and isinstance(args[0], MPlug):
            p = args[0]
            self._node = MFnDependencyNode(p.node()).name()
            self._name = p.partialName(useLongNames=True).split("[")[0]
        elif len(args) == 2:
            self._node = MFnDependencyNode(args[0]).name()
            self._name = _attr_name(args[1])
        elif len(args) == 1 and isinstance(args[0], MRampAttribute):
            self._node, self._name = args[0]._node, args[0]._name
        if self._name is not None and "." in self._name:
            self._name = self._name.split(".")[-1]

    def _kids(self):
        color = self._value_kid().endswith("_Color")
        return self._name + "_Position", self._value_kid(), self._name + "_Interp", color

    def _value_kid(self):
        if getattr(self, "_vk", None) is None:
            try:
                _cmds.getAttr("%s.%s[0].%s_Color" % (self._node, self._name, self._name))
                self._vk = self._name + "_Color"
            except Exception:
                self._vk = self._name + "_FloatValue"
                ctx = getattr(_compute_local, "ctx", None)
                if ctx is not None and (self._name + "." + self._name + "_Color") in ctx["values"]:
                    self._vk = self._name + "_Color"
        return self._vk

    def _entries(self):
        """{index: [position, value (a list), interp]}"""
        pos, val, interp, color = self._kids()
        ctx = getattr(_compute_local, "ctx", None)
        out = {}
        if ctx is not None and self._node and ctx["name"].split("|")[-1] == self._node.split("|")[-1]:
            vals = ctx["values"]
            g = self._name + "."
            for k, v in vals.get(g + pos) or []:
                out.setdefault(k, [0.0, [0.0, 0.0, 0.0] if color else [0.0], 1])[0] = float(v or 0.0)
            for k, v in vals.get(g + val) or []:
                e = out.setdefault(k, [0.0, [0.0, 0.0, 0.0] if color else [0.0], 1])
                e[1] = list(v) if isinstance(v, (list, tuple)) else [float(v or 0.0)]
            for k, v in vals.get(g + interp) or []:
                out.setdefault(k, [0.0, [0.0, 0.0, 0.0] if color else [0.0], 1])[2] = int(v or 0)
            return out
        ids = _cmds.getAttr("%s.%s" % (self._node, self._name), multiIndices=True) or []
        for k in ids:
            base = "%s.%s[%d]." % (self._node, self._name, k)
            v = _cmds.getAttr(base + val)
            if isinstance(v, list) and v and isinstance(v[0], (list, tuple)):
                v = list(v[0])
            out[k] = [float(_cmds.getAttr(base + pos)), v if isinstance(v, list) else [float(v)], int(_cmds.getAttr(base + interp))]
        return out

    def _sorted(self):
        return sorted(([e[0], e[1], e[2]] for e in self._entries().values()), key=lambda e: e[0])

    def isCurveRamp(self):
        return not self._kids()[3]

    def isColorRamp(self):
        return self._kids()[3]

    def numEntries(self):
        return len(self._entries())

    def getEntries(self):
        e = self._entries()
        ids = sorted(e)
        color = self._kids()[3]
        values = MColorArray([MColor(e[k][1]) for k in ids]) if color else MFloatArray([e[k][1][0] for k in ids])
        return MIntArray(ids), MFloatArray([e[k][0] for k in ids]), values, MIntArray([e[k][2] for k in ids])

    def getValueAtPosition(self, position):
        v = _ramp_eval(self._sorted(), float(position))
        return 0.0 if v is None else float(v[0])

    def getColorAtPosition(self, position):
        v = _ramp_eval(self._sorted(), float(position))
        return MColor() if v is None else MColor(v[0], v[1], v[2])

    def hasIndex(self, index):
        return index in self._entries()

    def _set(self, index, kid, value):
        attr = "%s.%s[%d].%s" % (self._node, self._name, index, kid)
        if isinstance(value, (list, tuple)):
            _cmds.setAttr(attr, *value, type="double3")
        else:
            _cmds.setAttr(attr, value)

    def setPositionAtIndex(self, position, index):
        self._set(index, self._kids()[0], float(position))

    def setValueAtIndex(self, value, index):
        self._set(index, self._kids()[1], float(value))

    def setColorAtIndex(self, color, index):
        self._set(index, self._kids()[1], [color.r, color.g, color.b])

    def setInterpolationAtIndex(self, interp, index):
        self._set(index, self._kids()[2], int(interp))

    def addEntries(self, positions, values, interps):
        e = self._entries()
        nxt = (max(e) + 1) if e else 0
        color = self._kids()[3]
        for i in range(len(positions)):
            k = nxt + i
            self.setPositionAtIndex(positions[i], k)
            if color:
                self.setColorAtIndex(values[i], k)
            else:
                self.setValueAtIndex(values[i], k)
            self.setInterpolationAtIndex(interps[i], k)

    def deleteEntries(self, indices):
        if len(indices) >= self.numEntries():
            _fail(status="kFailure", message="A ramp keeps at least one entry")
        for k in indices:
            _cmds.removeMultiInstance("%s.%s[%d]" % (self._node, self._name, k), b=True)

    def setRamp(self, values, positions=None, interps=None):
        """setRamp(value) sets every entry to one value; with positions and interps, the ramp becomes those points."""
        if positions is None:
            color = self._kids()[3]
            for k in self._entries():
                if color:
                    self.setColorAtIndex(values, k)
                else:
                    self.setValueAtIndex(values, k)
            return
        for k in list(self._entries()):
            _cmds.removeMultiInstance("%s.%s[%d]" % (self._node, self._name, k), b=True)
        self.addEntries(positions, values, interps)

    def sort(self, ascending=True):
        pts = sorted(self._entries().values(), key=lambda e: e[0], reverse=not ascending)
        positions = [p[0] for p in pts]
        color = self._kids()[3]
        values = [MColor(p[1]) for p in pts] if color else [p[1][0] for p in pts]
        self.setRamp(values, positions, [p[2] for p in pts])

    def pack(self):
        self.sort()

    @staticmethod
    def createRamp(*a):
        _fail(status="kNotImplemented", message="createRamp: use createCurveRamp or createColorRamp")



class MPxFileTranslator:
    """A file type of a plug-in's own: file -open, -import, -exportAll and -exportSelected with its type (or a file
    with its extension) run reader() or writer()."""

    kUnknownAccessMode = 0
    kOpenAccessMode = 1
    kReferenceAccessMode = 2
    kImportAccessMode = 3
    kSaveAccessMode = 4
    kExportAccessMode = 5
    kExportActiveAccessMode = 6
    kNotMyFileType = 0
    kIsMyFileType = 1
    kCouldBeMyFileType = 2
    kNone = 0
    kOverwriteFile = 1
    kOpen = 1
    kImport = 3
    kReference = 2

    _mode = 0

    def __init__(self):
        pass

    def haveReadMethod(self):
        return False

    def haveWriteMethod(self):
        return False

    def haveReferenceMethod(self):
        return False

    def haveNamespaceSupport(self):
        return False

    def canBeOpened(self):
        return True

    def defaultExtension(self):
        return ""

    def filter(self):
        ext = self.defaultExtension()
        return "*." + ext if ext else "*.*"

    def identifyFile(self, file, buffer, size):
        return MPxFileTranslator.kNotMyFileType

    def reader(self, file, optionsString, mode):
        _fail(status="kNotImplemented", message="This translator has no reader")

    def writer(self, file, optionsString, mode):
        _fail(status="kNotImplemented", message="This translator has no writer")

    def allowMultipleFileOptimization(self):
        return False

    @staticmethod
    def fileAccessMode():
        return MPxFileTranslator._mode
