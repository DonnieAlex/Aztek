# OpenMaya 1.0 compliance audit

Audited on 27/11/2026 against the reference's C++ API reference page for the OpenMaya module:

https://help.autodesk.com/view/MAYAUL/2022/ENU/?guid=Maya_SDK_cpp_ref_group_open_maya_html

The page is the 2022 edition and lists 271 classes, the MRandom namespace and the OPENMAYA_DEPRECATED macro. The
count includes template classes (MSharedPtr, MTemplateCommand and the like) and small helper types, because the page
lists them. This audit compares both of Aztek's OpenMaya surfaces against it:

- The C++ plug-in kit in `devkit/`, which compiled plug-ins build against.
- The Python API 1.0 modules `maya.OpenMaya`, `maya.OpenMayaAnim` and `maya.OpenMayaMPx`.

The work is on branch `docs/openmaya-1.0-compliance`, from `main` at 3b74c28.

## How the audit was made

The reference pages were fetched from the same site, every class page parsed for its public members, enums, constants
and signatures, and saved as the reference manifest described in `reference_class_matrix.tsv`.

The C++ kit was parsed with clang's AST dump, so inherited methods, constructors, operators, default arguments and
overloads come from the headers rather than from a text search. A class's effective public API includes public methods
of its public bases. Each reference member was then matched by name and arity. An overload counts as present only when
the kit has a method of the same name with the same number of parameters. This is coarse: a default argument, an
implicit conversion or a typedef with a different spelling can make an unmatched overload callable. Every such row in
the tables needs a second look before it is treated as a hard break.

The Python modules were run in Aztek's batch mode and introspected with `dir()` and `inspect.signature()`. Python 1.0
has its own calling conventions, so the Python comparison is by class and member name against the C++ page, not a
signature comparison. The reference's Python 1.0 documentation is a separate document set; it was not used here
because the task named the C++ page. Where Python 1.0 deliberately omits a C++ member, the row is still a difference a
script sees, but it may not be a bug.

The machine-readable results are next to this file:

| File | Contents |
| --- | --- |
| `reference_class_matrix.tsv` | All 271 reference classes with their C++ and Python status and counts |
| `cpp_class_gaps.tsv` | Every C++ class: missing methods, flagged signature differences, documented overloads and unmatched overloads |
| `python_class_gaps.tsv` | Every Python 1.0 class: module, missing methods, missing and extra constants |
| `enum_gaps.tsv` | Every enum or constant set that differs, with the missing and extra names |

## Headline figures

| Measure | C++ kit | Python 1.0 |
| --- | ---: | ---: |
| Reference classes | 271 | 271 |
| Present | 111 | 105 |
| Missing | 160 | 166 |
| Reference method names missing | 3,250 | 3,039 |
| Missing methods on classes that are present | 1,187 | 961 |
| Missing enum constants | 1,148 | 84 |
| Extra enum constants | 0 | 207 |
| Documented overloads without a same-arity counterpart | 4,053 of 6,259 | not measured |

C++: 4,053 is 1,658 of the 3,864 overloads on classes that already exist, plus 2,395 overloads on classes that do not
exist at all. Python: the 3,039 names are 961 on present classes and 2,078 on absent classes.

These figures count names, not working behaviour. They are the size of the documentation gap, not a count of bugs. A
missing method can be a one-line forwarding call or a subsystem of its own. The plan and estimate later in this
document treat them accordingly.

## What already matches

The kit is not a blank page. These areas have a usable API surface and, in the tests and samples, working behaviour:

- Value and math types: MVector, MFloatVector, MPoint, MFloatPoint, MColor, MMatrix, MFloatMatrix, MQuaternion,
  MEulerRotation, MTransformationMatrix, MBoundingBox, MTime, MAngle, MDistance, MPlane, MSpace, MTypeId, MString and
  the array classes that exist.
- Object and scene basics: MObject, MObjectHandle, MDagPath, MDagPathArray, MSelectionList, MPlug, MPlugArray,
  MGlobal, MDGContext, MDGModifier, MDagModifier, MFnBase, MFnDependencyNode, MFnDagNode, MFnTransform, MFnIkJoint.
- Geometry: MFnMesh over scene meshes and mesh data, MFnNurbsCurve, MFnNurbsSurface, MItMeshVertex, MItMeshPolygon,
  MItMeshEdge, MFnComponent, MFnSingleIndexedComponent, MFnGeometryFilter, MFnWeightGeometryFilter, MFnSkinCluster,
  MFnAnimCurve, MMeshIntersector, MPointOnMesh, the mesh data function sets, MItGeometry.
- Plug-ins: MPxCommand, MPxNode, MPxDeformerNode, MPxGeometryFilter, MPxTransform, MPxTransformationMatrix,
  MPxConstraint, MPxConstraintCommand, MPxSurfaceShape, MPxLocatorNode, MPxIkSolverNode, MPxData, MFnPlugin,
  MFnAttribute and its numeric, unit, typed, enum, compound, matrix, message and generic forms, MDataHandle,
  MDataBlock, MArrayDataHandle, MArrayDataBuilder, MArgList, MSyntax, MArgParser, MArgDatabase.
- Messages: MMessage, MSceneMessage, MDGMessage, MDagMessage, MNodeMessage, MEventMessage, MMessage::CallableInfo
  registration.
- Python 1.0: the three modules import under the reference's names, 105 classes are present across them, the 1.0
  out-parameter style works (an MDagPath, array or MScriptUtil pointer is filled), MStatus and MCommandResult are
  there, and the 2.0 classes underneath give the 1.0 wrappers their values.

## C++ kit differences

### Classes

111 of 271 classes exist. The 160 missing classes are listed in full in the appendix. By area they are:

- Edits and reference tracking: MAddRemoveAttrEdit, MConnectDisconnectAttrEdit, MFcurveEdit, MEdit, MSetAttrEdit,
  MParentingEdit, MItEdits, MCurveAttribute, MPxEditData, MRichSelection.
- Messages and callbacks: MCallbackIdArray, MCameraMessage, MCameraSetMessage, MCommandMessage, MConditionMessage,
  MContainerMessage, MLockMessage, MModelMessage, MObjectSetMessage, MPaintMessage, MPolyMessage, MTimerMessage,
  MUserEventMessage, MNodeClass, MNamespace.
- Evaluation, profiling and threading: MEvaluationGraph, MEvaluationManager, MEvaluationNodeIterator,
  MCustomEvaluatorClusterNode, MGraphNodeIterator, MTopologyClusterNode, MPxCustomEvaluator,
  MPxTopologyEvaluator, MPxAnimCurveInterpolator, MProfiler, MProfilingScope, MTimer, MThreadAsync, MMutexLock,
  MSpinLock, MAtomic.
- Geometry and component iterators not present: MItCurveCV, MItSurfaceCV, MItMeshFaceVertex, MItInstancer,
  MItDependencyNodes, MItSubdEdge, MItSubdFace, MItSubdVertex, MFnSubd, MFnSubdData, MFnSubdNames,
  MMeshSmoothOptions, MNurbsIntersector, MTrimBoundaryArray, MWeight, MFnTripleIndexedComponent,
  MFnDoubleIndexedComponent, MFnUint64SingleIndexedComponent.
- Data and arrays: MIntArray, MFloatArray, MDoubleArray, MUintArray, MUint64Array, MInt64Array,
  MFnDoubleArrayData, MFnFloatArrayData, MFnIntArrayData, MFnStringArrayData, MFnPointArrayData,
  MFnVectorArrayData, MFnMatrixArrayData, MFnUInt64ArrayData, MFnUintArrayData, MFnSphereData,
  MFnNurbsSurfaceData, MFnComponentListData, MFnArrayAttrsData, MFnFalloffData, MFnLatticeData,
  MFnContainerNode, MFnPartition, MFnReference, MFnExpression, MFnAssembly, MPxAssembly,
  MPxAttributePatternFactory.
- Shaders and lights: MFnLight, MFnAmbientLight, MFnAreaLight, MFnDirectionalLight, MFnPointLight, MFnSpotLight,
  MFnVolumeLight, MFnNonAmbientLight, MFnNonExtendedLight, MFnLambertShader, MFnBlinnShader, MFnPhongShader,
  MFnPhongEShader, MFnAnisotropyShader, MFnLayeredShader, MFnReflectShader, MFnStandardSurfaceShader,
  MFnLightDataAttribute, MaterialInputData.
- Plug-in proxies and file handling: MPxFileTranslator, MPxFileResolver, MPxMayaAsciiFilter,
  MPxMayaAsciiFilterOutput, MPxCacheFormat, MPxCacheConfigRuleFilter, MPxBakeEngine, MPxCameraSet,
  MPxComponentShape, MPxGeometryData, MPxGeometryIterator, MPxImageFile, MPxImagePlane, MPxMaterialInformation,
  MPxMotionPathNode, MPxMultiPolyTweakUVCommand, MPxObjectSet, MPxPolyTrg, MPxPolyTweakUVCommand,
  MPxRepresentation, MPxTransformPreRotation, MPxTransformationMatrixPreRotation, MFileIO, MImage,
  MImageFileInfo, MIffFile, MIffTag, MCommonSystemUtils.
- Utilities: MScriptUtil, MStreamUtils, MStringResource, MStringResourceId, MUniqueString, MURI,
  MPluginCallableInfo, MSharedPtr, MTesselationParams, MPlane, MComputation, MAttributeIndex,
  MAttributePattern, MAttributePatternArray, MAttributeSpec, MAttributeSpecArray, MOpenCLBuffer,
  MCacheConfigRuleRegistry, MCacheFormatDescription, MDAGDrawOverrideInfo, MDGContextGuard,
  MFnToolContext, MFnSet, MIteratorType, MExternalContentInfoTable, MExternalContentLocationTable,
  MUserEventMessage, MPxCommand templates.

The kit also has 38 classes that are not in this module. They are not gaps: they belong to OpenMayaAnim, OpenMayaUI,
OpenMayaRender or are support types. Examples: MFnAnimCurve, MFnSkinCluster, MFnIkHandle, MFnIkEffector, MFnIkJoint,
MIkHandleGroup, MPxDeformerNode, MPxGeometryFilter, MPxIkSolverNode, MPxConstraint, MPxConstraintCommand, M3dView,
MDrawRegistry, MRenderer, MUIDrawManager, MFrameContext, MDrawContext, MUserData, MPxDrawOverride, MSelectionMask,
MSpace, MMeshIsectAccelParams, MThreadRootTask. They are listed in `cpp_class_gaps.tsv` as extras and should be
audited against their own module pages, not this one.

### Methods

3,250 reference method names have no kit counterpart. The largest gaps on classes that do exist:

| Class | Reference methods | Missing |
| --- | ---: | ---: |
| MFnMesh | 214 | 182 |
| MFnCamera | 137 | 126 |
| MGlobal | 92 | 60 |
| MFnNurbsSurface | 86 | 57 |
| MFnDependencyNode | 69 | 42 |
| MFnPlugin | 70 | 42 |
| MPxTransform | 53 | 35 |
| MItMeshPolygon | 65 | 29 |
| MFnGeometryData | 32 | 27 |
| MItDependencyGraph | 38 | 26 |
| MPxNode | 49 | 26 |
| MDataHandle | 96 | 25 |
| MFnNurbsCurve | 43 | 23 |
| MFnDagNode | 53 | 22 |
| MFnAttribute | 59 | 19 |

The pattern is clear. The kit covers the calls Aztek's own plug-ins and the samples need, and the bulk of the
construction, editing and query surface is absent. The full list for every class is in `cpp_class_gaps.tsv`; it is
the authoritative count.

Some classes are present but almost entirely stubs. MFnCamera has 137 documented methods and 11 of its own in the
kit. MFnGeometryData has 32 and 5. MFnNurbsCurve has 43 and 20. MScriptUtil is absent from the C++ kit entirely, which
also hides the 1.0 out-parameter helpers a C++ plug-in ported from older code expects.

### Signatures and overloads

The coarse same-arity match flags 26 names that exist in both but with a different parameter count. Some are real
breaks, some are default arguments or parser noise and need confirmation:

| Class | Method | Reference arity | Kit arity |
| --- | --- | ---: | ---: |
| MAngle | as, asUnits | 2 | 1 |
| MDistance | as, asUnits | 2 | 1 |
| MFnData | constructor | 0 or 2 | 1 |
| MFnMesh | create | 7, 8 or 9 | 2, 3 or 4 |
| MFnMesh | getClosestPoint, getClosestNormal | 5 | 4 |
| MFnNurbsSurface | create | 10 | 2, 3 or 4 |
| MFnNurbsCurve | create | 7 | 2, 3 or 4 |
| MPxNode | name | 0 | 1 |
| MPxTransformationMatrix | transformBy | 1 | 3 |
| MPxTransformationMatrix | rotation, eulerRotation | 2 | 1 |
| MRampAttribute | setRamp | 3 | 4 |
| MRampAttribute | createRamp | 7 | 4 |
| MPlug | setAttribute, setLocked | 1 | 2 |
| MMeshIntersector | constructor, create, isCreated | varies | varies |
| MFnMesh | constructor overloads | varies | fewer |

The overload column in `cpp_class_gaps.tsv` is the more useful measure. 1,658 of the 3,864 documented overloads on
existing classes have no same-arity kit method. MFnMesh alone has 242 unmatched of 277, MFnCamera 133 of 147,
MPxTransform 71 of 91, MGlobal 67 of 116, MFnNurbsSurface 63 of 94, MFnPlugin 47 of 78. A few examples are false
positives caused by default arguments, but the scale is real.

The common signatures that do exist mostly follow the reference's order and types. The audit did not find a
systematic convention break such as swapped out-parameters or missing `MStatus*`; the breaks are missing overloads
and missing methods, not a different calling style.

### Aztek additions

The kit also has public members the reference does not document. 31 are named `az` and are the kit's own plumbing.
They appear in the public headers, so a plug-in author can see and call them:

```
MArgList::azAdd, MArgList::azItems, MDGMessage::azNewId, MDagMessage::azNewId, MDagPath::azNode, MDagPath::azOf,
MDataBlock::azBlock, MEventMessage::azNewId, MFnMesh::azCornerEdges, MFnMesh::azWorld, MItGeometry::azMesh,
MMessage::azNewId, MNodeMessage::azNewId, MObject::azData, MPlug::azAttrPath, MPlug::azMake, MPlug::azNode,
MPlug::azOuter, MPlug::azOwnElement, MPlug::azPath, MPlug::azSpec, MSceneMessage::azNewId, MSelectionList::azAdd,
MSelectionList::azComponents, MSelectionList::azItems, MSyntax::azArgs, MSyntax::azFlag, MSyntax::azFlags,
MSyntax::azHostFlags, MSyntax::azObjectFormat, MSyntax::azSelectionDefault
```

The rest of the extra names are 2027 additions or inherited conveniences that the 2022 page does not list
(niceNameOverride, setReadOnly, setSoftMin, isEmpty, substituteW, getCallableInfo and the like). They are not defects,
but they mean the kit is not a strict subset of the 2022 page. Phase 0 should decide whether `az` members stay public
for internal wiring or move behind a private interface, and the 2027 manifest should classify the rest.

### Constants and enum values

1,148 documented enum constants are absent. The distribution matters more than the total:

| Class and enum | Reference | Kit | Missing |
| --- | ---: | ---: | ---: |
| MFn::Type | 1,128 | 109 | 1,019 |
| MTime::Unit | 60 | 22 | 38 |
| MFnTransform::LimitType | 24 | 0 | 24 |
| MFnDependencyNode::MdgTimerMetric | 10 | 0 | 10 |
| MFnCamera::FilmFit | 5 | 0 | 5 |
| MFnNurbsSurface::BoundaryType | 5 | 0 | 5 |
| MPxSurfaceShape::MVertexCachingMode | 5 | 4 | 5 |
| MFnDependencyNode::MdgTimerState, MdgTimerType | 4 each | 0 | 4 each |
| MPxSurfaceShape::MVertexOffsetMode | 4 | 0 | 4 |

The MFn::Type gap is the largest single compliance item. The kit has 109 values with the reference's numbers, enough
for the node kinds Aztek exposes, but a plug-in that switches on `MFn::kBlinn`, `MFn::kPhong`,
`MFn::kReverseCurve` or any of the other missing kinds either fails to compile or falls into a default.
Because MFn::Type values are ABI, this table must be generated from the reference and frozen, not typed by hand.

MStatus::MStatusCode matches. MFnNumericData::Type matches. MFnData::Type has the same count but not the same names:
the kit has kLast where the reference has kFalloffFunction. MPxNode::Type matches exactly.
MPxNode::SchedulingType lacks kSerialize and kGloballySerialize, and its kDefaultScheduling alias is kUntrusted
where the reference makes it kSerial.

### Behavioural differences

The API surface is only part of the difference. These are the places where a method exists but does not do what the
reference's documentation describes:

- Base and scene: MObject holds a host handle. There is one path per DAG node, as in Python 1.0, where the reference
  can have several. Node identity, rename and reparent follow the host's model, not the reference's internal
  pointers.
- MPlug: array and child traversal, connections and attribute lookup work over the host, but several documented
  states and helpers are missing (isProcedural, isFromReferencedFile, isIgnoredWhenRendering, isDefaultValue,
  constructHandle, destructHandle, setNumElements, setMPxData, setMDataHandle, getSetAttrCmds). setAttribute and
  setLocked take an extra MStatus*.
- MFnMesh: read and write points, normals, UVs, polygons, triangles and closest point work. create has only the
  small overloads. Construction and editing (addPolygon, addHoles, deleteFace, deleteEdge, deleteVertex, split,
  subdivide, extrude, duplicate, extract, collapse, boolean, smooth mesh, colours, blind data, intersect, copy,
  updateSurface) are absent. closestIntersection and allIntersections accept the acceleration parameters and ignore
  them; every face is tried. This is correct but not the documented performance contract.
- MFnNurbsCurve and MFnNurbsSurface: read and some edit calls exist, but create, createWithEditPoints, copy, CV
  access, knot editing, derivatives, planar tests, trimming and attachment are largely absent. Non-rational curves
  and surfaces are the scene's model, so a rational result from the reference cannot be reproduced exactly.
- Iterators: MItMeshVertex, MItMeshPolygon and MItMeshEdge cover positions, normals, UVs and connectivity, but not
  colours, smoothing, tangents, updateSurface/geomChanged, or the component and face-vertex iterators. MItDag,
  MItDependencyGraph and MItDependencyNodes have a small subset of their traversal modes, filters and pruning.
- MFnPlugin: registerCommand, registerNode, registerTransform, registerShape, registerData, registerConstraintCommand
  and registerUI work. The registry calls for control commands, model editor commands, context commands, evaluators,
  topology evaluators, devices, file translators, URI resolvers, cache formats and drag and drop behaviours are
  missing, as are their deregistration pairs. registerIkSolver fails, as the reference's 2027 documentation calls it
  obsolete, but the 2022 page still documents it as working.
- Messages: callbacks are delivered when the host sends them, after the command that made the change and inside its
  undo step, not in the middle of the operation. Some documented messages never fire because the corresponding host
  event does not exist: connection refusal while a file is read, UUID changes, instance messages, keyability changes.
  A callback id for those is accepted and never called.
- Data blocks and handles: numeric, string, matrix, mesh and plug-in data values work. The generic, address, NURBS,
  subd, double4 and MPxData setters and getters on MDataHandle are missing. Array builders and handles cover the
  common cases but not every reference call.
- Threads: MThreadPool runs tasks one after another on the calling thread, in creation order. The reference runs them
  in parallel. A plug-in that depends on parallel speed or on thread-local state sees different behaviour. This is
  deliberate because a compute may not reach the host from another thread.
- GPU and OpenCL: Aztek deforms on the CPU. A GPU deformer registration is accepted and kept, but the CPU deform is
  what runs. The MAutoCLMem, MAutoCLKernel and MAutoCLEvent objects are always empty and the OpenCL calls are no-ops.
  MGPUDeformerRegistry and MGPUDeformerRegistrationInfo exist as names, not as a working pipeline.
- Viewport: MPxLocatorNode's Viewport 2.0 draw override is called (prepareForDraw, addUIDrawables) and the lines,
  circles, cones and text are collected. The legacy M3dView draw path is not called. MPxSurfaceShapeUI is kept for
  compatibility and never called. MPxDrawOverride and MUIDrawManager are thin.
- Transform proxies: MPxTransformationMatrix builds the matrix in the reference's order and supports the common
  translate, rotate, scale, shear, pivot and orientation calls. MPxTransform::validateAndSetValue is accepted and
  never called, so a plug-in cannot intercept channel values first. The pre-rotation classes are absent.
- File objects: MFileObject handles raw, expanded and resolved names, but pathCount, ithPath, ithFullName,
  setResolveMethod, resolveMethod, isAbsolutePath and getResolvedFullName are missing, and several resolve methods
  are not implemented.
- Units and angles: MAngle and MDistance lack the pointer forms of as and asUnits, internalToUI, uiToInternal and
  setInternalUnit. MTime lacks the compound assignment and increment operators, className and ticksPerSecond.
- Arrays: the array classes that exist provide the common append, length, indexing and copy operations, but many are
  missing the reference's full set (setLength, remove, insert, clear, get, setSizeIncrement, sizeIncrement, begin,
  end, cbegin, cend, className). Some arrays (MIntArray, MFloatArray, MDoubleArray, MUintArray and the 64-bit
  variants) do not exist at all, which blocks many signatures that take them.
- String and status: MString lacks setUTF8 and setWChar. MStatus lacks pAPIerror, set and setSuccess, and its
  constructor from MStatusCode is present in a different set of overloads. MStatus behaves as a value with an error
  string, not as the reference's status with a per-thread last error.
- Documentation and diagnostics: most kit methods carry no comment describing the reference's behaviour, and there is
  no generated API reference. A plug-in author has the headers and the samples, not a searchable class reference.

## Python 1.0 differences

### Modules and classes

The reference's C++ page puts all 271 classes in one module. Python 1.0 splits them the way the reference's Python
modules do. Aztek has:

| Module | Classes | Common with the C++ page | Missing from the page's list |
| --- | ---: | ---: | ---: |
| maya.OpenMaya | 100 | 98 | 2 extras |
| maya.OpenMayaAnim | 7 | 0 | 7 extras |
| maya.OpenMayaMPx | 16 | 7 | 9 extras |
| all three | 121 | 105 | 16 extras |

105 of the 271 reference classes exist somewhere in Python 1.0. The two extras in OpenMaya are MSpace and
MMeshIsectAccelParams; the others are OpenMayaAnim, OpenMayaUI or OpenMayaRender classes that the C++ page does not
list. The full matrix is in `reference_class_matrix.tsv`.

The 166 missing classes include most of the same families as the C++ list, plus classes the kit has but Python 1.0
does not:

- MFnCamera, MFnMeshData, MFnNurbsCurveData, MFnNurbsSurfaceData, the array data function sets,
  MFnExpression, MFnPartition, MFnReference, MFnAssembly, MFnContainerNode, MFnToolContext, MFnSubd and the subd
  iterators, MMeshIntersector, MPointOnMesh, MMeshSmoothOptions, MNurbsIntersector.
- MFileIO, MImage, MImageFileInfo, MIffFile, MIffTag, MCommonSystemUtils, MNamespace, MNodeClass, MPluginCallableInfo,
  MProfiler, MProfilingScope, MTimer, MThreadPool, MThreadUtils, MThreadAsync, MMutexLock, MSpinLock, MAtomic,
  MStreamUtils, MStringResource, MStringResourceId, MUniqueString, MURI, MSharedPtr, MTesselationParams, MWeight,
  MTrimBoundaryArray, MRichSelection, MScriptUtil is present (it is one of the 100), but the C++ page's list also
  has edit classes that Python lacks.
- MPx classes beyond commands, nodes, transforms, constraints, shapes, locators and IK solvers: bake engines, cache
  formats and filters, camera sets, component shapes, custom evaluators, edit data, file resolvers, geometry data and
  iterators, image files and planes, material information, Maya ASCII filters, motion path nodes, object sets,
  poly triggers, poly tweak UV commands, representations, topology evaluators and pre-rotation matrices.

### Methods

961 reference method names are missing from classes Python 1.0 already has, and 2,078 more belong to classes that do
not exist, for 3,039 in total. The largest gaps are:

| Class | Module | Reference methods | Missing |
| --- | --- | ---: | ---: |
| MFnMesh | OpenMaya | 214 | 153 |
| MGlobal | OpenMaya | 92 | 63 |
| MFnNurbsSurface | OpenMaya | 86 | 59 |
| MFnPlugin | OpenMaya | 70 | 50 |
| MDataHandle | OpenMaya | 96 | 44 |
| MFnDependencyNode | OpenMaya | 69 | 44 |
| MPxTransform | OpenMayaMPx | 53 | 37 |
| MPxNode | OpenMayaMPx | 49 | 34 |
| MFnGeometryData | OpenMaya | 32 | 25 |
| MItMeshPolygon | OpenMaya | 65 | 25 |
| MString | OpenMaya | 40 | 25 |
| MItDependencyGraph | OpenMaya | 38 | 23 |
| MFnDagNode | OpenMaya | 53 | 22 |
| MFnNumericData | OpenMaya | 26 | 19 |
| MPlug | OpenMaya | 90 | 18 |

Some of these are C++ conveniences that Python 1.0 never exposes, such as className, typeString, isValid,
destructors and operator overloads. Those were removed before counting. The remainder is the same construction,
editing, file, camera, shader and evaluator surface missing from the C++ kit, seen through the 1.0 calling style.
The full list is in `python_class_gaps.tsv`.

### Constants

84 documented constants are missing from Python classes and 207 constants Python exposes are not in the C++ page.
The extras are not automatically wrong: Aztek targets the 2027 API, and some of its constants are newer than the 2022
page. The missing set includes:

- MFnTransform: 24 of 24 LimitType values (kTranslateX, kRotateX, kScaleX and the rest).
- MFnDependencyNode: 16 timer metric, state and type values.
- MFnMesh: 9 values across its nested enums.
- MTime: 9 unit values.
- MGlobal: 8 values.
- MFnData: 6 values, including kFalloffFunction, kDynSweptGeometry, kMatrixArray, kNObject, kNId and kSubdSurface.
- MFileObject: 3 resolve methods.
- MFnDagNode: 3 object colour values.
- MItDependencyGraph: 3 traversal values.
- MCommandResult: 2 result types.

The MFn class itself has 1,215 constant names, against 1,128 on the C++ page, so the large MFn::Type gap in the C++
kit is not repeated in Python. The comparison includes inherited constants, because `dir()` shows them, and the C++
page's inherited enum rows were excluded. That makes some of the 207 extras look worse than they are; the per-class
missing names are the reliable part. The remaining differences are concentrated in classes whose enum sets were not
copied over. `enum_gaps.tsv` has every name.

### Behavioural differences

Python 1.0 is a wrapper over the 2.0 classes, not a separate implementation. That produces these differences:

- Every 1.0 class is generated from its 2.0 class: properties become `getX()` calls, setters become `setX()`, and
  `isX()` aliases are added where the 2.0 class has a property. A method that exists in neither 2.0 nor the 1.0 body
  is absent, even if the reference's 1.0 API has it.
- Results are converted back to 1.0 classes as they are returned, and out-parameters are filled through MScriptUtil
  pointers, arrays, MDagPath, MObject and MPlug. The pointer forms are the part most likely to differ from the
  reference in edge cases, especially null pointers, wrong pointer types and arrays of pointers.
- Some classes are thin. `om_thin.py` creates the reference's names so imports and plug-in registration succeed,
  but any method that is not written out raises NotImplementedError when called. Registration calls are accepted and
  do nothing. Constants of thin classes that are not written out are Aztek's own running numbers, not the reference's
  values.
- The 1.0 callback style is supported: a check callback returns False to stop what is coming (2.0) or sets its first
  argument (1.0, through MScriptUtil). MEventMessage runs through a script job. Callback delivery follows the host's
  event order, not the reference's internal order.
- The known 2.0 differences carry into 1.0: one path per DAG node, setKeyable only on added attributes, cycles
  refused node by node rather than plug by plug, and curves not handed to Python deformers.
- The MPx surface is narrower than the C++ page. Commands, plain nodes, deformers, transforms, constraints, shapes,
  locators and IK solvers work. File translators, custom evaluators, bake engines, cache formats, camera sets,
  geometry data and the rest are thin or absent. MPxFileTranslator is present but identifyFile is not asked: a file
  goes to a translator by type or extension.
- Python 1.0 has no `MFnCamera`, `MFnMeshData` or `MMeshIntersector`, which are common in scripts that
  inspect a scene. It also has no `MFileIO`, `MNamespace` or `MNodeClass`. Those are C++ class names a 1.0 script
  would expect from the page.
- Documentation: docstrings are sparse. The completion index shows a class's members and the constructor signature,
  but most members have no description. The reference's Python pages are not reproduced.

## Cross-cutting differences

- Version skew. The page is the 2022 edition. Aztek's kit comments and PARITY.md target the 2027 API in several
  places, for example MFn::Type values, MPxNode::SchedulingType aliases and registerIkSolver being obsolete. A 2022
  audit and a 2027 implementation will disagree on names and numbers that changed between the releases. Before closing
  gaps, the reference manifest should be regenerated from the 2027 pages and this report re-run. For now the 2022 page
  is the spec the task named.
- C++ and Python are separate surfaces. Adding a C++ method does not add the Python 1.0 method, and the Python 1.0
  wrapper is built on 2.0, not on the C++ kit. The shared work is the reference manifest, the host calls and the
  conformance tests, not the class code.
- Extras are not gaps. 38 C++ kit classes belong to other modules. 16 Python classes are extras. They need their own
  module audits, not deletion.
- Metrics are name-level. A class can have every method name and still be behaviourally wrong. The behavioural
  section is therefore part of the compliance target, not an appendix.
- No API reference. The kit's headers are the documentation. Compliance should include generated reference pages or
  at least a method-level doc comment, otherwise a plug-in author cannot discover the implemented subset.

## Compliance plan

The plan is on branch `docs/openmaya-1.0-compliance`. It adds only the audit and the plan; no API code changes are in
this branch. Implementation branches should be cut from `main` one phase at a time, and each phase should keep the
manifest and the ledger current.

### Principles

1. The reference's 2027 pages are the target. Regenerate the manifest from them before the first implementation
   phase, then keep the 2022 manifest as a compatibility check.
2. Constants, types and enum values are generated, never hand-written. They are ABI.
3. A method is done only when it compiles, links, runs and matches a reference-derived test. A stub that returns
   kNotImplemented is not done.
4. Behaviour follows the reference where the host can express it. Where it cannot (legacy viewport, OpenCL, parallel
   threads), the difference is documented in the header, in the class reference and in PARITY.md, not left implicit.
5. The C++ kit and Python 1.0 are tracked separately. Shared host operations should be added once and exposed twice
   where possible.
6. No class is added just to make an import succeed. Thin classes are allowed only for modules that are explicitly
   marked thin, with a test that every such class raises the documented NotImplementedError.

### Phase 0: manifest and gates

Deliverables:

- `tools/reference_capture` gains an OpenMaya extractor that writes classes, methods, overloads, enums, constants and
  the module each belongs to. It runs against the 2027 pages and stores a canonical manifest in `parity/data/`.
- A CI job builds every kit sample and a new compile-only probe that includes all public headers.
- A CI job runs the Python introspection dump and compares class, method and constant names with the manifest. It
  reports additions, removals and renames, with a checked-in allowlist for intentional differences.
- A conformance test crate for the C++ kit, using the host's batch mode, and a Python conformance script for 1.0.

Exit criteria: the audit can be re-run with one command; the current numbers are reproducible; CI fails on an
unlisted regression.

Estimate: 4 to 6 engineer-days.

### Phase 1: foundation types and constants

Scope: MFn::Type (all values), MTime::Unit, MFnData::Type, MFnNumericData::Type, MPxNode::Type and
SchedulingType, MStatus, MString, all array classes, MAngle, MDistance, MTime, MBoundingBox, the matrix and vector
classes, MTypeId and the support types.

Work:

- Generate MFn::Type and the other enums from the manifest with the reference's explicit values and aliases.
- Complete the array classes over one template, including the 64-bit and colour arrays and the iterator surface.
- Fill the missing value-class methods and pointer forms.
- Add ABI tests that assert every enum value and every class size where the reference documents one.

Exit criteria: every value, math and array class on the page has its documented methods, constants and operators;
the Python 1.0 constants for those classes match the manifest.

Estimate: C++ 25 to 40 days, Python 15 to 25 days.

### Phase 2: scene and dependency graph

Scope: MObject, MObjectHandle, MDagPath, MDagPathArray, MSelectionList, MPlug, MPlugArray, MGlobal, MDGContext,
MDGModifier, MDagModifier, MFnBase, MFnDependencyNode, MFnDagNode, MFnTransform, MFnAttribute and its subclasses,
MDataHandle, MDataBlock, MArrayDataHandle, MArrayDataBuilder, MItDag, MItDependencyGraph, MItDependencyNodes.

Work:

- Complete the plug state and connection query surface, including physical and logical indices, attribute lookup,
  aliases, metadata and the callback flags.
- Complete the DAG path API: multiple paths, instances, all paths below, template and draw override queries.
- Complete the selection list API: replace, intersect, partly selected items, rich selection.
- Complete the modifier API: attribute add and remove edits, extensions, parenting, metadata, multi-instance removal.
- Complete the data handle and block generic, address, double4 and plug-in data paths.
- Add the dependency graph traversal filters, pruning and relationship modes.

Exit criteria: a ported node, transform or deformer that uses only these classes compiles and runs the same as in the
reference for the covered calls. Tests cover rename, reparent, instance, array plug and undo behaviour.

Estimate: C++ 60 to 90 days, Python 40 to 60 days.

### Phase 3: geometry

Scope: MFnMesh, MFnNurbsCurve, MFnNurbsSurface, the mesh, curve and surface data function sets, MFnComponent and its
indexed forms, MItMeshVertex, MItMeshPolygon, MItMeshEdge, MItMeshFaceVertex, MItCurveCV, MItSurfaceCV, MItGeometry,
MMeshIntersector, MPointOnMesh, MNurbsIntersector, MMeshSmoothOptions, MFnSubd and the subd iterators, MFnSet.

Work:

- Complete MFnMesh construction, editing, boolean, smoothing, colour, blind data and intersection surface. The host
  already stores the mesh; the work is the API and the history nodes behind the editing calls.
- Complete NURBS creation, CV and knot editing, derivatives, planar and trim queries. The scene is non-rational; mark
  rational-only results as an explicit difference.
- Complete the iterators, including colours, tangents, smoothing, UV sets and updateSurface/geomChanged.
- Add the missing data function sets and the component list, lattice and falloff data.
- Make MMeshIntersector and MNurbsIntersector real, with acceleration parameters that affect the result or are
  documented as ignored.

Exit criteria: the mesh and NURBS conformance suite covers every documented method that the scene can express, with
reference-derived expected values; the rest are listed as host limits.

Estimate: C++ 90 to 130 days, Python 50 to 75 days.

### Phase 4: plug-ins, proxies and messages

Scope: MFnPlugin, MPxCommand, MPxNode, MPxDeformerNode, MPxGeometryFilter, MPxTransform, MPxTransformationMatrix,
MPxConstraint, MPxConstraintCommand, MPxSurfaceShape, MPxLocatorNode, MPxIkSolverNode, MPxData, the missing MPx
classes, MArgList, MSyntax, MArgParser, MArgDatabase, MMessage and its subclasses, MPluginCallableInfo.

Work:

- Complete MFnPlugin's registry: control, model editor and context commands, evaluators, topology evaluators,
  devices, file translators, URI resolvers, cache formats and drag and drop behaviours.
- Add the missing proxy classes, starting with the ones real plug-ins use: file translators, custom evaluators, bake
  engines, cache formats, camera sets, geometry data and iterators, image planes and pre-rotation transforms.
- Decide and document the viewport contract. Keep the Viewport 2.0 path; either implement the legacy M3dView calls or
  mark them removed with the reference's deprecation version.
- Decide and document the GPU contract. Keep CPU deformation; make the OpenCL and GPU classes either functional or
  explicitly unsupported, not silently empty.
- Align message delivery with the reference where the host has the event. List the events the host cannot produce
  and make those callbacks fail visibly or return an id that is documented as never called.
- Complete the transform proxy hooks, including validateAndSetValue, or document that channel interception is not
  offered.

Exit criteria: the samples and the mGear and cvwrap plug-ins build unchanged; a new test plug-in exercises every
registration call and every proxy virtual; callback order tests match the documented order for the events the host
has.

Estimate: C++ 90 to 130 days, Python 45 to 65 days.

### Phase 5: breadth

Scope: shaders and lights, image and file I/O, editing and reference classes, evaluation and profiling, OpenCL,
threads, string resources, URIs, shared pointers, tessellation parameters, the remaining utility classes.

Work:

- Shader and light function sets over the host's shading nodes. This is a large but shallow surface: most methods map
  to attributes.
- MFileIO, MImage, MImageFileInfo, MIffFile and MIffTag, with the same resolution rules as MFileObject.
- The edit classes and MItEdits, if the host's undo and reference model can represent them. Otherwise mark them out
  of scope with a reason.
- MEvaluationGraph, MEvaluationManager, MEvaluationNode and their iterators. The host has no evaluation manager in
  the reference's sense; implement the query surface over the host's graph or document the difference.
- MProfiler, MProfilingScope, MTimer, MThreadAsync and the lock classes. The thread pool stays serial unless the host
  gains a safe parallel path; document that.
- MStringResource, MStringResourceId, MUniqueString, MURI, MSharedPtr, MTesselationParams and the remaining support
  types.

Exit criteria: every class on the page is either implemented or listed as out of scope with a reference citation and
a user-visible error.

Estimate: C++ 120 to 180 days, Python 70 to 100 days.

### Phase 6: Python 1.0 parity

Scope: the 166 missing classes, the 961 missing methods on present classes and the 84 missing constants.

Work:

- Let the 1.0 module derive from the completed 2.0 classes. Most of the missing methods arrive when the 2.0 class is
  completed, so do this phase after Phases 1 to 5.
- Add the 1.0-only spellings, pointer forms and out-parameter conventions that the reference's 1.0 API has and the
  2.0 class does not.
- Replace thin classes with real ones where the reference has behaviour. Keep a thin list only for modules that are
  deliberately names-only, and give every thin class a test that its unsupported calls raise.
- Copy the reference's constants, including the timer, limit, object colour and traversal enums.
- Add the missing module split: MFnCamera, MFnMeshData, MMeshIntersector, MFileIO, MNamespace, MNodeClass and
  the rest go in the module the reference puts them in.

Exit criteria: the Python introspection diff is empty except for the allowlist; a ported 1.0 script that uses the
covered classes runs unchanged; the 1.0 and 2.0 tests both pass.

Estimate: Python 150 to 220 days, less where Phase 2 to 5 already added the underlying 2.0 method.

### Phase 7: hardening and documentation

Work:

- Compile matrix on Linux, Windows and macOS, with the kit's C++ standard and the reference's include style.
- ABI tests for class layout, enum values, virtual order where the reference documents it, and symbol visibility.
- Negative tests: null objects, wrong function sets, failed statuses, callbacks on dead nodes, undo and redo.
- Generated API reference from the headers and the manifest, plus a short guide for each phase.
- Update PARITY.md and the plug-in docs with the exact remaining differences.

Exit criteria: the manifest diff is clean or allowlisted; the samples and real plug-ins build and run; the docs
name every remaining difference.

Estimate: 20 to 30 days.

### Suggested implementation order

1. Phase 0, then Phase 1. Without generated constants and a diff gate, later phases will drift.
2. Phase 2, because every plug-in uses the scene and DG classes.
3. Phase 3, because mesh and NURBS plug-ins are the largest real-world consumer.
4. Phase 4, because proxy registration is what makes a plug-in load at all.
5. Phase 6 in parallel with Phases 2 to 5 once the 2.0 classes are stable, then Phase 5 for the long tail.
6. Phase 7 continuously, not at the end.

## Estimate

### Basis

The estimate is in engineer-days for one experienced engineer who knows both the reference API and Aztek's host. It
assumes 5 productive days a week, no release work, no upstream API changes and no major redesign. The ranges are
deliberately wide because a missing method can be a forwarding call or a subsystem.

The count of missing methods is not a direct labor estimate. Many are simple getters and setters, especially in the
shader, light and array classes; a few (MFnMesh editing, MFnNurbsSurface, the evaluator, file translators, the
viewport) are large. The ranges below use approximately 0.15 to 0.4 engineer-days per missing method, 1 to 3 days per
missing class for surface and host wiring, and separate allowances for behaviour, tests and documentation.

### By work package

| Package | Low days | High days |
| --- | ---: | ---: |
| Phase 0, manifest and gates | 4 | 6 |
| Phase 1, C++ foundation | 25 | 40 |
| Phase 1, Python foundation | 15 | 25 |
| Phase 2, C++ scene and DG | 60 | 90 |
| Phase 2, Python scene and DG | 40 | 60 |
| Phase 3, C++ geometry | 90 | 130 |
| Phase 3, Python geometry | 50 | 75 |
| Phase 4, C++ plug-ins and proxies | 90 | 130 |
| Phase 4, Python plug-ins and proxies | 45 | 65 |
| Phase 5, C++ breadth | 120 | 180 |
| Phase 5, Python breadth | 70 | 100 |
| Phase 6, Python 1.0 parity | 150 | 220 |
| Phase 7, hardening and docs | 20 | 30 |
| **Total** | **779** | **1,151** |

The total is about 37 to 55 person-months, or 3.1 to 4.6 person-years. Adding 25 percent contingency for the parts
that turn out to be subsystems rather than surfaces gives 974 to 1,439 engineer-days.

### Scenarios

| Scenario | Scope | Engineer-days | One engineer | Three engineers | Four engineers |
| --- | --- | ---: | ---: | ---: | ---: |
| Surface only | Every class and method name exists, constants are correct, unimplemented calls fail visibly | 90 to 140 | 4 to 7 months | 1.5 to 2.5 months | 1 to 2 months |
| Tier 1, practical plug-in API | The classes used by the samples, mGear, cvwrap and common node, mesh, deformer and command plug-ins | 230 to 340 | 11 to 16 months | 4 to 5.5 months | 3 to 4.5 months |
| Tier 2, full core module | Everything on the page except obsolete, legacy viewport, OpenCL, subd and evaluation-manager internals | 520 to 760 | 2 to 3 years | 8 to 12 months | 6 to 9 months |
| Tier 3, full parity | Every class, method, constant and documented behaviour on the 2022 page, both surfaces | 974 to 1,439 | 4 to 6 years | 15 to 22 months | 11 to 17 months |

`Tier 1` is the recommended target for the next release cycle. `Tier 2` is a realistic two-year programme with a small
team. `Tier 3` is full API parity and should be planned as a multi-year effort, not a sprint. The calendar figures are
working time and exclude holidays, release freezes and review latency.

### What changes the estimate most

- Whether the target is 2022 or 2027. Regenerating the manifest against 2027 can remove obsolete classes and add new
  ones. This audit's numbers will move.
- Whether "compliance" means callable surface or reference behaviour. The surface-only scenario is 7 to 10 times
  cheaper than full behaviour.
- Whether the host can represent the concept. File references, the evaluation manager, the legacy viewport, parallel
  threads and OpenCL may be documented differences rather than implementations. Deciding that early removes large
  blocks from Tier 3.
- Whether Python 1.0 must be complete or only complete for the classes 2.0 already has. The 1.0 wrapper is cheap per
  method once 2.0 is done; the missing classes and their host calls are the expensive part.
- Real plug-in demand. If Tier 1 is defined by the plug-ins that must load, the work is much smaller than the page's
  271 classes.

## Recommendation

1. Freeze the 2027 manifest and re-run this audit against it before writing API code. Keep this 2022 report as the
   compatibility baseline.
2. Adopt Tier 1 as the release target: constants, scene and DG, mesh and NURBS basics, command and node proxies,
   messages, and the Python classes those plug-ins touch.
3. Make the diff gate and the generated constant tables the first commits. They prevent the surface from drifting
   again.
4. Treat the legacy viewport, parallel threads, OpenCL and the evaluation manager as explicit host differences with a
   citation, not as silent stubs.
5. Re-estimate after Phase 0. The manifest will replace this document's hand-counted numbers with a reproducible
   count, and that count is the only honest basis for a schedule.

## Appendix A: every missing C++ class

```
MAddRemoveAttrEdit, MAtomic, MAttributeIndex, MAttributePattern, MAttributePatternArray, MAttributeSpec,
MAttributeSpecArray, MCacheConfigRuleRegistry, MCacheFormatDescription, MCallbackIdArray, MCameraMessage,
MCameraSetMessage, MCommandMessage, MCommandResult, MCommonSystemUtils, MComputation, MConditionMessage,
MConnectDisconnectAttrEdit, MContainerMessage, MCurveAttribute, MCustomEvaluatorClusterNode, MDAGDrawOverrideInfo,
MDGContextGuard, MDoubleArray, MEdit, MEvaluationGraph, MEvaluationManager, MEvaluationNodeIterator,
MExternalContentInfoTable, MExternalContentLocationTable, MFcurveEdit, MFileIO, MFloatArray, MFnAmbientLight,
MFnAnisotropyShader, MFnAreaLight, MFnArrayAttrsData, MFnAssembly, MFnBlinnShader, MFnCameraSet,
MFnComponentListData, MFnContainerNode, MFnDirectionalLight, MFnDoubleIndexedComponent, MFnExpression,
MFnFalloffData, MFnFloatVectorArrayData, MFnLambertShader, MFnLatticeData, MFnLayeredShader, MFnLight,
MFnLightDataAttribute, MFnMatrixArrayData, MFnNonAmbientLight, MFnNonExtendedLight, MFnNurbsSurfaceData,
MFnPartition, MFnPhongEShader, MFnPhongShader, MFnPointLight, MFnReference, MFnReflectShader, MFnSet, MFnSphereData,
MFnSpotLight, MFnStandardSurfaceShader, MFnSubd, MFnSubdData, MFnSubdNames, MFnToolContext, MFnTripleIndexedComponent,
MFnUInt64ArrayData, MFnUint64SingleIndexedComponent, MFnUintArrayData, MFnVolumeLight, MGraphNodeIterator, MIffFile,
MIffTag, MImage, MImageFileInfo, MInt64Array, MIntArray, MItCurveCV, MItDependencyNodes, MItEdits, MItInstancer,
MItMeshFaceVertex, MItSubdEdge, MItSubdFace, MItSubdVertex, MItSurfaceCV, MIteratorType, MLockMessage,
MMeshSmoothOptions, MModelMessage, MMutexLock, MNamespace, MNodeClass, MNurbsIntersector, MObjectSetMessage,
MOpenCLBuffer, MPaintMessage, MParentingEdit, MPlane, MPluginCallableInfo, MPolyMessage, MProfiler, MProfilingScope,
MPxAnimCurveInterpolator, MPxAssembly, MPxAttributePatternFactory, MPxBakeEngine, MPxCacheConfigRuleFilter,
MPxCacheFormat, MPxCameraSet, MPxComponentShape, MPxCustomEvaluator, MPxEditData, MPxFileResolver, MPxFileTranslator,
MPxGeometryData, MPxGeometryIterator, MPxImageFile, MPxImagePlane, MPxMaterialInformation, MPxMayaAsciiFilter,
MPxMayaAsciiFilterOutput, MPxMotionPathNode, MPxMultiPolyTweakUVCommand, MPxObjectSet, MPxPolyTrg,
MPxPolyTweakUVCommand, MPxRepresentation, MPxTopologyEvaluator, MPxTransformPreRotation,
MPxTransformationMatrixPreRotation, MRichSelection, MScriptUtil, MSetAttrEdit, MSharedPtr<T>, MSpinLock, MStreamUtils,
MStringResource, MStringResourceId, MTemplateAction, MTemplateCommand, MTemplateCreateNodeCommand, MTesselationParams,
MThreadAsync, MTimer, MTimerMessage, MTopologyClusterNode, MTrimBoundaryArray, MURI, MUint64Array, MUintArray,
MUniqueString, MUserEventMessage, MWeight, MaterialInputData
```

The template classes are written without their template parameters here. `cpp_class_gaps.tsv` has the exact
spellings.

## Appendix B: every missing Python 1.0 class

```
MAddRemoveAttrEdit, MArrayDataBuilder, MAtomic, MAttributeIndex, MAttributePattern, MAttributePatternArray,
MAttributeSpec, MAttributeSpecArray, MAutoCLEvent, MAutoCLKernel, MAutoCLMem, MCacheConfigRuleRegistry,
MCacheFormatDescription, MCameraMessage, MCameraSetMessage, MCommandMessage, MCommonSystemUtils, MComputation,
MConditionMessage, MConnectDisconnectAttrEdit, MContainerMessage, MCurveAttribute, MCustomEvaluatorClusterNode,
MDAGDrawOverrideInfo, MDGContextGuard, MEdit, MEvaluationGraph, MEvaluationManager, MEvaluationNode,
MEvaluationNodeIterator, MExternalContentInfoTable, MExternalContentLocationTable, MFcurveEdit, MFileIO,
MFnAmbientLight, MFnAnisotropyShader, MFnAreaLight, MFnArrayAttrsData, MFnAssembly, MFnBlinnShader, MFnCamera,
MFnCameraSet, MFnComponentListData, MFnContainerNode, MFnDirectionalLight, MFnDoubleArrayData, MFnExpression,
MFnFalloffData, MFnFloatArrayData, MFnFloatVectorArrayData, MFnIntArrayData, MFnLambertShader, MFnLatticeData,
MFnLayeredShader, MFnLight, MFnLightDataAttribute, MFnMatrixArrayData, MFnMeshData, MFnNonAmbientLight,
MFnNonExtendedLight, MFnNurbsCurveData, MFnNurbsSurfaceData, MFnPartition, MFnPhongEShader, MFnPhongShader,
MFnPointArrayData, MFnPointLight, MFnReference, MFnReflectShader, MFnSphereData, MFnSpotLight,
MFnStandardSurfaceShader, MFnStringArrayData, MFnSubd, MFnSubdData, MFnSubdNames, MFnToolContext,
MFnTripleIndexedComponent, MFnUInt64ArrayData, MFnUint64SingleIndexedComponent, MFnUintArrayData,
MFnVectorArrayData, MFnVolumeLight, MGraphNodeIterator, MIffFile, MIffTag, MImage, MImageFileInfo, MInt64Array,
MItCurveCV, MItEdits, MItInstancer, MItSubdEdge, MItSubdFace, MItSubdVertex, MItSurfaceCV, MIteratorType,
MLockMessage, MMeshIntersector, MMeshSmoothOptions, MModelMessage, MMutexLock, MNamespace, MNodeClass,
MNurbsIntersector, MObjectSetMessage, MOpenCLBuffer, MOpenCLInfo, MPaintMessage, MParentingEdit,
MPluginCallableInfo, MPointOnMesh, MPolyMessage, MProfiler, MProfilingScope, MPxAnimCurveInterpolator, MPxAssembly,
MPxAttributePatternFactory, MPxBakeEngine, MPxCacheConfigRuleFilter, MPxCacheFormat, MPxCameraSet, MPxComponentShape,
MPxCustomEvaluator, MPxEditData, MPxFileResolver, MPxGeometryData, MPxGeometryIterator, MPxImageFile,
MPxImagePlane, MPxMaterialInformation, MPxMayaAsciiFilter, MPxMayaAsciiFilterOutput, MPxMotionPathNode,
MPxMultiPolyTweakUVCommand, MPxObjectSet, MPxPolyTrg, MPxPolyTweakUVCommand, MPxRepresentation, MPxTopologyEvaluator,
MPxTransformPreRotation, MPxTransformationMatrixPreRotation, MRichSelection, MSetAttrEdit, MSharedPtr<T>, MSpinLock,
MStreamUtils, MStringResource, MStringResourceId, MTemplateAction, MTemplateCommand, MTemplateCreateNodeCommand,
MTesselationParams, MThreadAsync, MThreadPool, MThreadUtils, MTimer, MTimerMessage, MTopologyClusterNode,
MTrimBoundaryArray, MURI, MUint64Array, MUniqueString, MUserEventMessage, MWeight, MaterialInputData
```

`python_class_gaps.tsv` is the corrected record. The count is 166 missing classes.

## Appendix C: reproducing the audit

The reference pages are public and do not need a reference installation. The audit uses:

- The C++ API pages under `https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/`.
- Clang 22's AST dump for the C++ headers (`-Xclang -ast-dump=json -ast-dump-filter=<class>`).
- `build/Aztek/AppRun --batch --script` for Python introspection, with `inspect.signature` on every public member.

The manifest and comparison scripts are not committed yet. Phase 0 of the plan puts them in
`tools/reference_capture` and adds the CI gate. Until then, this branch records the findings and the raw tables, not
a rerunnable pipeline.
