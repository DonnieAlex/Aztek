"""Live probe for the Python 1.0 name mismatches in this folder.

Purpose
    Confirm that the 1.0 names listed in ``name_mismatches.md`` resolve on a running Aztek build, and that the 2.0
    controls still resolve. This is a quick manual check for an implementation agent, not the full reference audit.

Scope
    Required names are the 18 confirmed mismatches. Probable names are the 6 rows that still need a signature check.
    Controls are the 2.0 names that must keep working while a 1.0 alias is added. The probe does not check method
    signatures, argument order, return values or behaviour. The full missing-method and missing-constant records are in
    ``python_class_gaps.tsv`` and ``enum_gaps.tsv``; the full name search is ``name_candidates.tsv``.

Run
    From the repository root, with a built app:

        build/Aztek/AppRun --batch --script docs/openmaya_work/data/probe_name_mismatches.py

Output
    One ``Class.name=YES`` or ``Class.name=NO`` line per check, then a summary. Exit status is 0 when every required
    name resolves, 1 when any required name is still missing. Probable and control failures are reported but do not
    fail the run.

Side effects
    Creates a sphere named ``probeSphere`` in the batch scene. The batch process exits without saving, so no file or
    scene is changed outside the process.
"""
import sys

import maya.OpenMaya as om
import maya.OpenMayaMPx as mpx
import maya.cmds as cmds


def names(obj, required=(), probable=(), controls=()):
    return {"obj": obj, "required": list(required), "probable": list(probable), "controls": list(controls)}


def check(label, group):
    results = {}
    for kind in ("required", "probable", "controls"):
        for name in group[kind]:
            try:
                getattr(group["obj"], name)
                ok = True
            except Exception:
                ok = False
            results[(kind, name)] = ok
            print(f"{label}.{name}={('YES' if ok else 'NO')} ({kind})")
    return results


arglist = om.MArgList()
checks = {
    "MArgParser": names(
        om.MArgParser(om.MSyntax(), arglist),
        required=[
            "commandArgumentMAngle", "commandArgumentMDistance", "commandArgumentMTime",
            "flagArgumentMAngle", "flagArgumentMDistance", "flagArgumentMTime",
        ],
        controls=["commandArgumentAngle", "flagArgumentTime"],
    ),
    "MFnNurbsSurface": names(
        om.MFnNurbsSurface(),
        required=["degreeU", "degreeV", "getDerivativesAtParm"],
        probable=["getCVs", "getCV"],
        controls=["degreeInU", "degreeInV", "getDerivativesAtParam", "cvPositions", "cvPosition"],
    ),
    "MFnDagNode": names(
        om.MFnDagNode(),
        required=["setIntermediateObject"],
        controls=["setIsIntermediateObject"],
    ),
    "MQuaternion": names(
        om.MQuaternion(),
        required=["getAxisAngle", "setAxisAngle"],
        controls=["asAxisAngle"],
    ),
    "MStringArray": names(
        om.MStringArray(),
        required=["indexOf"],
        controls=["index"],
    ),
}

cmds.polySphere(n="probeSphere")
selection = om.MSelectionList()
om.MGlobal.getSelectionListByName("probeSphere", selection)
path = om.MDagPath()
selection.getDagPath(0, path)
path.extendToShape()
checks["MItMeshFaceVertex"] = names(
    om.MItMeshFaceVertex(path),
    required=["vertId", "faceVertId"],
    controls=["vertexId", "faceVertexId"],
)
checks["MItMeshPolygon"] = names(
    om.MItMeshPolygon(path),
    required=["isConnectedToEdge", "isConnectedToFace", "isConnectedToVertex"],
    controls=["connectedEdges", "connectedFaces", "connectedVertices"],
)
checks["MPxCommand"] = names(
    mpx.MPxCommand,
    probable=["currentIntResult", "currentDoubleResult", "currentStringResult", "getCurrentResult"],
    controls=["currentResult"],
)

missing_required = []
missing_probable = []
for label, group in checks.items():
    results = check(label, group)
    for (kind, name), ok in results.items():
        if ok:
            continue
        if kind == "required":
            missing_required.append(f"{label}.{name}")
        elif kind == "probable":
            missing_probable.append(f"{label}.{name}")

print()
print(f"required missing: {len(missing_required)}")
for name in missing_required:
    print("  " + name)
print(f"probable missing: {len(missing_probable)}")
for name in missing_probable:
    print("  " + name)

sys.exit(1 if missing_required else 0)
