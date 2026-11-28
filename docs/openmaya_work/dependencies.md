# OpenMaya dependency analysis

## Why this document exists

The module documents in `reference/` are independent lists, but the implementation is not independent. A
class or method that looks small can pull in types from another module. This document records the dependency edges
that should decide the order of work, including edges that cross modules.

The raw data is `data/dependency_edges.json`: every class-to-class edge found in a base class or in a public member
signature. The counts below are the number of distinct referenced classes, not the number of mentions. A class that
appears once in a signature and a class that appears fifty times both count once.

## Module dependency graph

| Source module | Target module | Weight |
| --- | --- | ---: |
| OpenMayaUI | OpenMaya | 1,234 |
| OpenMayaAnim | OpenMaya | 1,115 |
| OpenMayaRender | OpenMaya | 1,080 |
| OpenMayaFX | OpenMaya | 616 |
| OpenMayaUI | OpenMayaRender | 44 |
| OpenMaya | OpenMayaAnim | 4 |
| OpenMaya | OpenMayaRender | 1 |

Reading the graph:

- OpenMaya is the foundation. Every other module depends on it, heavily. It should be implemented first.
- OpenMayaAnim, OpenMayaFX and OpenMayaRender depend on OpenMaya, not on each other in the current signatures. They
  can run in parallel once OpenMaya's foundation is stable.
- OpenMayaUI depends on OpenMaya and on OpenMayaRender. The 44 UI-to-Render edges are concentrated in `MUIDrawManager`,
  `MFrameContext`, `MGeometry`, `MSwatchRenderBase` and the draw-override classes. UI work that touches drawing cannot
  finish before those Render types exist.
- The small back-edges are real cycles and should be broken or accepted deliberately:
  - `MFnFalloffData` (OpenMaya) references `MFalloffContext` and `MFalloffFunction` (OpenMayaAnim).
  - `MFnPlugin` (OpenMaya) references `MRenderPassDef` (OpenMayaRender).
  - `MPxAnimCurveInterpolator` (OpenMaya) references `MFnAnimCurve` (OpenMayaAnim).

  The first two can be handled with forward declarations and an interface boundary. The third is a proxy that belongs
  to OpenMayaAnim in practice; if it stays in OpenMaya, it needs the Anim class.

## Foundation ranking

The most referenced classes across all five modules, by distinct incoming class edges:

| Class | Module | Referenced by |
| --- | --- | ---: |
| MStatus | OpenMaya | 400 |
| MObject | OpenMaya | 262 |
| MString | OpenMaya | 210 |
| MFn | OpenMaya | 142 |
| MDagPath | OpenMaya | 109 |
| MStringArray | OpenMaya | 59 |
| MVector | OpenMaya | 57 |
| MPlug | OpenMaya | 57 |
| MMatrix | OpenMaya | 56 |
| MPoint | OpenMaya | 53 |
| MIntArray | OpenMaya | 41 |
| MObjectArray | OpenMaya | 38 |
| MDoubleArray | OpenMaya | 35 |
| MColor | OpenMaya | 31 |
| MPxNode | OpenMaya | 29 |
| MTime | OpenMaya | 26 |
| MFloatArray | OpenMaya | 26 |
| MFnData | OpenMaya | 26 |
| MSelectionList | OpenMaya | 25 |
| MFnDependencyNode | OpenMaya | 24 |
| MDataBlock | OpenMaya | 20 |
| MMessage | OpenMaya | 20 |
| MPointArray | OpenMaya | 20 |
| MVectorArray | OpenMaya | 19 |
| MTypeId | OpenMaya | 18 |
| MFnDagNode | OpenMaya | 16 |
| MImage | OpenMaya | 16 |
| MUIDrawManager | OpenMayaRender | 14 |
| MEvaluationNode | OpenMaya | 13 |
| MFloatVector | OpenMaya | 13 |

`MStatus`, `MObject`, `MString` and `MFn` are not optional. Every serious implementation phase should start after those
four and the arrays they use are stable.

## Key class dependencies

These are the direct prerequisites extracted from each class's public signatures and base clause. A missing entry
means the class has no distinct class reference in its own declarations.

| Class | Direct dependencies |
| --- | --- |
| MFnMesh | MColor, MColorArray, MDGModifier, MDagPath, MDoubleArray, MFloatArray, MFloatPoint, MFloatPointArray, MFloatVector, MFloatVectorArray, MFn, MFnDagNode, MIntArray, MMeshSmoothOptions, MObject, MObjectArray, MPoint, MPointArray, MSelectionList, MStatus, MString, MStringArray, MUintArray, MVector, MVectorArray |
| MFnNurbsSurface | MDagPath, MDoubleArray, MFloatArray, MFn, MFnDagNode, MIntArray, MObject, MObjectArray, MPoint, MPointArray, MStatus, MTesselationParams, MTrimBoundaryArray, MVector |
| MFnNurbsCurve | MDagPath, MDoubleArray, MFn, MFnDagNode, MObject, MObjectArray, MPoint, MPointArray, MStatus, MVector |
| MFnTransform | MDagPath, MEulerRotation, MFn, MFnDagNode, MObject, MPoint, MQuaternion, MStatus, MTransformationMatrix, MVector |
| MFnIkJoint | MDagPath, MEulerRotation, MFn, MFnTransform, MObject, MQuaternion, MStatus, MString, MTransformationMatrix |
| MFnSkinCluster | MDagPath, MDagPathArray, MDoubleArray, MFloatArray, MFn, MFnGeometryFilter, MIntArray, MObject, MSelectionList, MStatus |
| MFnAnimCurve | MAngle, MAnimCurveChange, MDGModifier, MDoubleArray, MFn, MFnDependencyNode, MIntArray, MObject, MPlug, MStatus, MTime, MTimeArray |
| MFnParticleSystem | MDagPath, MDoubleArray, MFn, MFnDagNode, MIntArray, MObject, MPoint, MPointArray, MStatus, MString, MTime, MVector, MVectorArray |
| MFnPlugin | MFn, MFnBase, MObject, MPluginCallableInfo, MPxData, MPxNode, MRenderPassDef, MStatus, MString, MStringArray, MTypeId |
| MPxNode | MDGContext, MDataBlock, MDataHandle, MEvaluationNode, MExternalContentInfoTable, MExternalContentLocationTable, MObject, MObjectArray, MPlug, MPlugArray, MStatus, MString, MStringArray, MTypeId |
| MPxCommand | MArgList, MDoubleArray, MIntArray, MStatus, MString, MStringArray, MSyntax |
| MPxData | MArgList, MStatus, MString, MTypeId |
| MPxTransform | MBoundingBox, MDGContext, MDataBlock, MDataHandle, MEulerRotation, MFnTransform, MMatrix, MObject, MPlug, MPoint, MPxNode, MPxTransformationMatrix, MQuaternion, MStatus, MTransformationMatrix, MVector |
| MPxDeformerNode | MDataBlock, MPxGeometryFilter, MPxNode, MStatus |
| MPxSurfaceShapeUI | MDagPath, MDrawData, MDrawInfo, MDrawRequest, MDrawRequestQueue, MMaterial, MMaterialArray, MObjectArray, MPointArray, MPxSurfaceShape, MSelectInfo, MSelectionList, MStatus, MTextureEditorDrawInfo |
| M3dView | MColor, MDagPath, MImage, MMatrix, MPoint, MPxGlBuffer, MSelectionList, MStatus, MString, MVector |
| MUIDrawManager | MBlendState, MColor, MColorArray, MFloatPointArray, MFloatVectorArray, MPoint, MPointArray, MSamplerState, MStatus, MString, MStringArray, MTexture, MUintArray, MVector, MVectorArray |
| MPxDrawOverride | MBoundingBox, MDagPath, MDrawContext, MFrameContext, MIndexBuffer, MMatrix, MObject, MPoint, MPointArray, MRenderItem, MSelectionContext, MSelectionInfo, MSelectionList, MStatus, MString, MUIDrawManager, MUserData, MVertexBufferArray |
| MPxLocatorNode | MBoundingBox, MColor, MDagPath, MEvaluationNode, MObjectArray, MPoint, MPxNode, MVector |
| MDataBlock | MArrayDataHandle, MDGContext, MDataHandle, MObject, MPlug, MStatus |
| MDataHandle | MAngle, MDataBlock, MDistance, MFloatMatrix, MFloatVector, MFnData, MFnNumericData, MMatrix, MObject, MPlug, MPxData, MStatus, MString, MTime, MTypeId, MUintArray, MVector |
| MPlug | MAngle, MDGContext, MDataBlock, MDataHandle, MDistance, MIntArray, MObject, MPlugArray, MPxData, MStatus, MString, MStringArray, MTime |
| MDagPath | MDAGDrawOverrideInfo, MDagPathArray, MFn, MMatrix, MObject, MStatus, MString, MTransformationMatrix |
| MGlobal | MCommandResult, MDagPath, MDoubleArray, MIntArray, MObject, MObjectArray, MRichSelection, MSelectionList, MStatus, MString, MStringArray, MTime, MTransformationMatrix, MVector |
| MSelectionList | MDagPath, MObject, MPlug, MStatus, MString, MStringArray, MUuid |

## Compelling work order

A dependency edge means the target should already work, or at least have a stable declaration and a passing test, before
the source is implemented. These are the edges that most often make a small task bigger than it looked.

### 1. Constants, status, strings and arrays

Do first:

- `MFn::Type`, the per-class enums and every numeric constant.
- `MStatus`, `MString`, `MStringArray`, `MIntArray`, `MFloatArray`, `MDoubleArray`, `MUintArray`, `MObjectArray`,
  `MPointArray`, `MVectorArray`, `MFloatPointArray`, `MFloatVectorArray`, `MMatrixArray`, `MColorArray`, `MTimeArray`.
- `MObject`, `MTypeId`, `MFn`, `MAngle`, `MDistance`, `MTime`, `MPoint`, `MVector`, `MMatrix`, `MFloatPoint`,
  `MFloatVector`, `MFloatMatrix`, `MColor`, `MBoundingBox`, `MQuaternion`, `MEulerRotation`, `MTransformationMatrix`.

Why: `MStatus` alone is referenced by 400 classes. `MString` by 210. Every array missing from the kit blocks dozens of
signatures. Implementing a mesh method before `MIntArray` exists means inventing a temporary type and doing the work
twice.

### 2. Object, path, plug and scene handles

Then:

- `MDagPath`, `MDagPathArray`, `MObjectHandle`, `MSelectionList`, `MPlug`, `MPlugArray`, `MGlobal`, `MDGContext`.
- `MDataHandle`, `MDataBlock`, `MArrayDataHandle`, `MArrayDataBuilder`.
- `MFnBase`, `MFnDependencyNode`, `MFnDagNode`, `MFnTransform`, `MFnAttribute` and its typed subclasses.

Why: every function set that takes or returns a node, path, plug or attribute depends on these. `MPlug` is referenced
by 57 classes and by the data block; `MDagPath` by 109.

### 3. Plugin bases before the plugin registry

`MPxNode` depends on `MDataBlock`, `MDataHandle`, `MPlug`, `MObject`, `MStatus`, `MTypeId` and the evaluation and
external-content types. `MPxCommand` depends on `MArgList`, `MSyntax` and the result arrays. `MPxData` depends on
`MArgList`, `MString` and `MTypeId`.

Implement the proxy base classes and their data/argument dependencies before `MFnPlugin`'s registration calls.
`MFnPlugin.registerNode` cannot be tested without `MPxNode`; `registerCommand` cannot be tested without `MPxCommand`;
`registerData` cannot be tested without `MPxData`.

`MPxDeformerNode` depends on `MPxGeometryFilter`, which depends on `MPxNode` and `MItGeometry`. Implement
`MItGeometry` and the data block before the deformer proxy, not after.

### 4. Mesh and NURBS after their data and array types

`MFnMesh` references 25 other classes. The blocking ones are the array classes (`MIntArray`, `MObjectArray`,
`MPointArray`, `MFloatPointArray`, `MFloatVectorArray`, `MColorArray`, `MDoubleArray`, `MFloatArray`), the value types
(`MPoint`, `MVector`, `MColor`, `MBoundingBox`) and `MFnDagNode`. Its construction and editing methods also need
`MFnMeshData`, `MMeshIsectAccelParams`, `MMeshSmoothOptions` and the component iterators.

`MFnNurbsSurface` and `MFnNurbsCurve` depend on the same arrays plus `MTesselationParams`, `MTrimBoundaryArray` and
`MObjectArray`. `MTesselationParams` is itself a class with dependencies on the tessellation enums. Implementing
`create` or `getCVs` before the arrays and data function sets exist will not compile against the reference signatures.

### 5. Render and UI drawing before the draw overrides

`MUIDrawManager` depends on `MPoint`, `MVector`, `MColor`, `MString`, `MTexture`, `MStateManager` and the buffer state
types. `MPxDrawOverride` depends on `MFrameContext`, `MDrawContext`, `MRenderItem`, `MVertexBufferArray`,
`MIndexBuffer`, `MSelectionContext`, `MSelectionInfo`, `MUIDrawManager` and `MUserData`.

Do not start `MPxDrawOverride` or `M3dView` before `MUIDrawManager`, `MFrameContext`, `MRenderItem` and the buffer
state types exist. The UI-to-Render edges are only 44, but they are all in the classes that make the viewport work.

### 6. Animation and effects after the scene

`MFnAnimCurve` depends on `MTimeArray`, `MAnimCurveChange`, `MDGModifier`, `MPlug`, `MAngle`, `MTime` and the array
classes. `MFnSkinCluster` depends on `MFnGeometryFilter`, `MDagPathArray`, `MSelectionList` and the numeric arrays.
`MFnParticleSystem` depends on the point and vector arrays, `MTime`, `MDagPath` and `MFnDagNode`.

Once OpenMaya's foundation and geometry are stable, Anim and FX can proceed in parallel. The only cross-module cycle
to decide first is `MFnFalloffData` (OpenMaya) and `MFalloffContext`/`MFalloffFunction` (OpenMayaAnim).

### 7. Python 1.0 aliases after the underlying method exists

A 1.0 alias is a thin wrapper over a 2.0 method or a 1.0 implementation. It depends on the target callable and on the
1.0 out-parameter convention. Do not add an alias before the target callable works; the alias will pass a name test and
fail at runtime.

The 1.0 class also inherits from the 2.0 class. The inheritance edge is a hard build dependency: `maya.OpenMaya.MFnMesh`
cannot exist before `aztek.api.OpenAztek.MFnMesh`. The Python dependency order is therefore 2.0 class, 2.0 method, 1.0
wrapper, 1.0 name.

### 8. Cycles to break

- `MFnFalloffData` (OpenMaya) -> `MFalloffContext`, `MFalloffFunction` (OpenMayaAnim). Move the shared falloff types to
  OpenMaya, or forward-declare and keep the Anim types behind an interface. Do not copy the types into both modules.
- `MFnPlugin` (OpenMaya) -> `MRenderPassDef` (OpenMayaRender). A forward declaration is enough if the method only takes
  a pointer or reference. If it takes a value, the type must move or the method waits for Render.
- `MPxAnimCurveInterpolator` (OpenMaya) -> `MFnAnimCurve` (OpenMayaAnim). Confirm the module placement first. If the
  reference truly puts the proxy in OpenMaya, the Anim function set is a hard prerequisite.

## Work packages this implies

| Package | Included classes | Blocked by |
| --- | --- | --- |
| P1 Constants and value types | MFn, status, strings, arrays, math | Nothing |
| P2 Scene handles | MObject, MDagPath, MPlug, MSelectionList, modifiers | P1 |
| P3 Data blocks | MDataHandle, MDataBlock, MArrayDataHandle, MArrayDataBuilder | P1, P2 |
| P4 Function set bases | MFnBase, MFnDependencyNode, MFnDagNode, MFnTransform, attributes | P1, P2, P3 |
| P5 Geometry data | MFnMeshData, MFnNurbsCurveData, MFnNurbsSurfaceData, array data sets | P1 |
| P6 Mesh and NURBS | MFnMesh, MFnNurbsCurve, MFnNurbsSurface, iterators | P1 to P5 |
| P7 Plugin bases | MPxNode, MPxCommand, MPxData, MPxDeformerNode | P3, P4, P5 |
| P8 Plugin registry | MFnPlugin, command args, syntax, messages | P7 |
| P9 Render foundation | MFrameContext, MUIDrawManager, MRenderItem, buffers | P1, P2 |
| P10 UI and overrides | M3dView, MPxDrawOverride, MPxLocatorNode, manipulators | P9 |
| P11 Animation | MFnAnimCurve, MFnSkinCluster, MFnIkJoint | P1 to P6 |
| P12 Effects | particles, fields, cloth/rigid | P1 to P6 |
| P13 Python 1.0 | aliases and missing 1.0 classes | P1 to P12 as needed |

## Practical rule for a subagent

Before starting a class, read its row in this document and the `data/dependency_edges.json` entries whose `source` is
that class. If any target is not implemented, either:

1. Implement the target first and update this document, or
2. Get an explicit decision that the target is out of scope and the source will be a documented stub.

Never add a source class that compiles only because it invents a local type with the same name. The 1.0 name is the
contract; a local substitute makes the class look complete and breaks the next class that needs the real one.
