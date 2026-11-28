# Python 1.0 member name mismatches

## Scope and precedence

The reference is the 2022 C++ API reference, module by module:

https://help.autodesk.com/view/MAYAUL/2022/ENU/?guid=Maya_SDK_cpp_ref_group_open_maya_html

Python API 1.0 (`maya.OpenMaya`, `maya.OpenMayaAnim`, `maya.OpenMayaMPx`) takes precedence. API 2.0 is the
implementation substrate under those modules, not the specification. When a 1.0 name and a 2.0 name disagree, the 1.0
name is the one a script written for the reference calls.

This document records names that Aztek implements under the 2.0 spelling while the documented 1.0 spelling is absent.
It does not repeat every missing method. The full missing-method and missing-class records are in
`data/python_class_gaps.tsv` and `data/cpp_class_gaps.tsv`.

## Root cause

The 1.0 modules are built in `crates/aztek_host/python/om1.py` by `_v1(base)`, which copies the 2.0 class's callables
and adds `getX()`, `setX()` and `isX()` aliases for 2.0 properties. It does not carry names that 2.0 renamed, shortened
or replaced. A 2.0 method such as `vertexId` is copied as `vertexId`, but the 1.0 method `vertId` is never created.
The result is the reported error:

```
'MItMeshFaceVertex' object has no attribute 'vertId'. Did you mean: 'vertexId'?
```

The fix is an explicit 1.0 alias or wrapper in the class body, not a rename of the 2.0 method. Several aliases cannot be
a plain assignment because the 1.0 method fills out-parameters or returns a different type.

## Audit method

- Reference names: all public member functions on every C++ class page for the five C++ modules, read from the 2022
  pages and stored in `data/reference_class_matrix.tsv`.
- Aztek names: `dir()` and `inspect.signature()` on every class in `maya.OpenMaya`, `maya.OpenMayaAnim` and
  `maya.OpenMayaMPx`, run from `build/Aztek/AppRun --batch --script`.
- Pairing: for every class, a documented 1.0 name missing from Aztek was compared with the class's extra names. A pair
  was proposed when the names matched after ignoring `get`/`set`/`is`, or when their string similarity was high.
- Review: every proposal was checked against the C++ declaration and the Aztek callable. Pairs that were merely
  different methods (a getter paired with an unrelated setter) were rejected. The full proposal list, including the
  rejected pairs, is `data/name_candidates.tsv`.
- Counts: 105 reference classes are present somewhere in Python 1.0. 961 documented names are absent from those
  classes, and 2,078 more belong to classes that are absent entirely. This document covers the subset where Aztek has
  a differently named equivalent.

## Result

18 mappings are confirmed. 6 are probable and need a signature check before they are aliased. The confirmed rows are the ones that raise AttributeError on a real instance, not the ones merely absent from `dir()`.

| Class | Documented 1.0 name | Aztek name | Verdict | Evidence and note |
| --- | --- | --- | --- | --- |
| `MArgParser` | `commandArgumentMAngle` | `commandArgumentAngle` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. Aztek exposes the 2.0 name. |
| `MArgParser` | `commandArgumentMDistance` | `commandArgumentDistance` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. |
| `MArgParser` | `commandArgumentMTime` | `commandArgumentTime` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. |
| `MArgParser` | `flagArgumentMAngle` | `flagArgumentAngle` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. |
| `MArgParser` | `flagArgumentMDistance` | `flagArgumentDistance` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. |
| `MArgParser` | `flagArgumentMTime` | `flagArgumentTime` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_arg_parser.html). The 1.0 name keeps the M prefix. |
| `MFnDagNode` | `setIntermediateObject` | `setIsIntermediateObject` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_dag_node.html). The 2.0 property is isIntermediateObject, so the 1.0 wrapper generated setIsIntermediateObject. The documented 1.0 setter is setIntermediateObject. |
| `MFnNurbsSurface` | `degreeU` | `degreeInU` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html). 2.0 renamed degreeU to degreeInU. The 1.0 name is degreeU. |
| `MFnNurbsSurface` | `degreeV` | `degreeInV` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html). 2.0 renamed degreeV to degreeInV. |
| `MFnNurbsSurface` | `getDerivativesAtParm` | `getDerivativesAtParam` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html). The 1.0 spelling is Parm, the 2.0 spelling is Param. |
| `MItMeshFaceVertex` | `faceVertId` | `faceVertexId` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_face_vertex.html). The same pattern as the reported vertId. 2.0 renamed faceVertId to faceVertexId and the 1.0 alias is missing. |
| `MItMeshFaceVertex` | `vertId` | `vertexId` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_face_vertex.html). The reported case. 2.0 renamed vertId to vertexId and the 1.0 alias is missing. |
| `MItMeshPolygon` | `isConnectedToEdge` | `connectedEdges` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_polygon.html). 2.0 replaced the predicate with connectedEdges. The 1.0 method is isConnectedToEdge. |
| `MItMeshPolygon` | `isConnectedToFace` | `connectedFaces` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_polygon.html). 2.0 replaced the predicate with connectedFaces. |
| `MItMeshPolygon` | `isConnectedToVertex` | `connectedVertices` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_it_mesh_polygon.html). 2.0 replaced the predicate with connectedVertices. |
| `MQuaternion` | `getAxisAngle` | `asAxisAngle` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_quaternion.html). The 1.0 getter is getAxisAngle. Aztek only has the 2.0 asAxisAngle. |
| `MQuaternion` | `setAxisAngle` | `asAxisAngle` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_quaternion.html). The 1.0 setter is setAxisAngle. Aztek only has the 2.0 asAxisAngle; the setter needs its own wrapper because the return convention differs. |
| `MStringArray` | `indexOf` | `index` | confirmed | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_string_array.html). 2.0 uses list.index. The 1.0 method is indexOf, which returns -1 rather than raising. |
| `MFnNurbsSurface` | `getCVs` | `cvPositions` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html). 2.0 exposes cvPositions. The 1.0 getter is getCVs. Verify the array form before aliasing. |
| `MFnNurbsSurface` | `getCV` | `cvPosition` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_fn_nurbs_surface.html). 2.0 exposes cvPosition. The 1.0 getter is getCV. Verify the two-index form. |
| `MPxCommand` | `currentIntResult` | `currentResult` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_command.html). 1.0 has typed current result getters. Aztek has one currentResult. Check the return type before aliasing. |
| `MPxCommand` | `currentDoubleResult` | `currentResult` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_command.html). 1.0 has typed current result getters. |
| `MPxCommand` | `currentStringResult` | `currentResult` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_command.html). 1.0 has typed current result getters. |
| `MPxCommand` | `getCurrentResult` | `currentResult` | probable | [class page](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/class_m_px_command.html). 1.0 getCurrentResult fills an MStringArray. currentResult may not have the same contract. |
## Related missing names that are not renames

These documented 1.0 names are absent and Aztek has no equivalent callable. They are missing methods, not renamed
ones, and should be handled from the C++ gap records:

- `MFnSet.setAnnotation`, `MFnDagNode.setObjectColor`, `MFnDagNode.setUseObjectColor`, `MPlug.setCaching`.
- `MFnNurbsSurface.cvsInU`, `cvsInV`, `getKnotDomain`, `setCVs`, `setCV`.
- `MPxTransform.rotateBy`, `rotateTo`, `scaleBy`, `scaleTo`, `translateBy`, `translateTo`, `getMatrix`,
  `getRotationOrder`, `setRotationOrder`, `getRotatePivot`, `setRotatePivot` and the pivot translation pair. The
  entries named `matrix`, `rotatePivot`, `translate`, `scale` and `shear` are static MObject attributes on
  `MPxTransform`, not callables, so they are not aliases.
- `MFnNurbsSurface.getKnotsInU`, `getKnotsInV`, `getTangents`, `MRampAttribute.getNumEntries` and `MFnTransform.getTranslation` work through the dynamic `getX` path even though `dir()` does not list them. They are not mismatches. `MFnNurbsSurface.getCVs` and `getCV` do not resolve and are listed as probable.
- `MItMeshFaceVertex.faceVertId` and `vertId` are the same kind of case as the confirmed report. `faceVertexId` and
  `vertexId` exist and are the 2.0 names.
- `MStringArray.indexOf` needs a wrapper over the list `index`, not a class-body assignment, because the 1.0 method
  returns `-1` rather than raising `ValueError`.

## What matches

- All 105 present classes exist under the reference's names, in the module the reference uses.
- 1.0 constructors and out-parameter conventions are present for the covered classes: `MDagPath`, `MObject`,
  `MSelectionList`, `MPlug`, the arrays and `MScriptUtil` pointers.
- `MStatus`, `MString`, `MCommandResult`, `MScriptUtil` and the 1.0 plugin base classes are present.
- The remaining 961 missing names are mostly methods that do not exist at all, not aliases under another spelling.
  The full list is in `data/python_class_gaps.tsv`.

## Fix pattern for the later agent

For each row, add the 1.0 name to the `_v1` body for that class in `crates/aztek_host/python/om1.py` (or `om1_anim.py`,
`om1_mpx.py`). Do not rename the 2.0 method and do not change `om_fn.py`, `om_iter.py` or `om_values.py` for this work.

Example shape:

```python
def vertId(self, *args):
    return self.vertexId(*args)
```

Rules for the wrappers:

- If the 1.0 method fills an out-parameter, accept the out argument and write the 2.0 result into it. Do not return the
  value where 1.0 returns an `MStatus`.
- If the 1.0 method returns an `MStatus` or `None`, keep that contract even when the 2.0 method returns a value.
- If the 1.0 method returns `-1` for not found (`MStringArray.indexOf`), do not let a Python exception escape.
- If the 1.0 method is a getter/setter pair, add both names. A getter alias alone leaves scripts that set a value
  broken.
- Add a test for each alias to `crates/aztek_host/tests/python_api_v1.rs`, with a call that would have raised
  `AttributeError` before the change.
- Run the Python introspection dump again and confirm the name is present in `maya.OpenMaya`.
- Update `data/python_class_gaps.tsv` or add an allowlist entry so the audit no longer reports the name as missing.

## Reproduction

From the repository root, with a built app, run the committed live probe:

```sh
build/Aztek/AppRun --batch --script docs/openmaya_work/data/probe_name_mismatches.py
```

It prints one `Class.name=YES` or `Class.name=NO (kind)` line per check, then a summary. Exit status is 0 when every
required name resolves and 1 while any confirmed name is still missing. It checks the 18 confirmed names, the 6
probable names and the 2.0 controls, and it creates a sphere named `probeSphere` in the batch scene. It does not check
signatures, return values or the rest of the 961 missing names. `data/README.md` has the full scope and usage.

The full audit used a separate introspector that walks `maya.OpenMaya`, `maya.OpenMayaAnim` and `maya.OpenMayaMPx`,
records `dir()` and `inspect.signature()`, and writes JSON. That script is not committed; the resulting records are
`data/python_class_gaps.tsv`, `data/reference_class_matrix.tsv` and `data/name_candidates.tsv`. The reference manifest
is generated from the cached C++ pages under `help.autodesk.com/cloudhelp/2022/ENU/Maya-SDK/cpp_ref/`. The tables in
this document are the reviewed subset.

## Next steps

1. Add the 18 confirmed aliases first. They are small, testable and directly unblock scripts.
2. Verify the 6 probable rows against a real call signature before coding. `MQuaternion.setAxisAngle`,
   `MPxCommand.currentIntResult` and `MFnNurbsSurface.getCVs` are the ones most likely to need a full wrapper.
3. Re-run `probe_name_mismatches.py` and the 1.0 unit tests after each alias.
4. For the remaining 961 missing names, use the C++ gap records and the dependency document rather than treating them
   as naming work.
