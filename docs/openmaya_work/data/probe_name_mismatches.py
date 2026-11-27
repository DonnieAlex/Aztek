# Run with: build/Aztek/AppRun --batch --script docs/openmaya_work/data/probe_name_mismatches.py
# Checks the confirmed 1.0 names on a live instance. It prints NO for each still-missing name.
import maya.OpenMaya as om
import maya.OpenMayaMPx as mpx
import maya.cmds as cmds


def report(label, obj, names):
    for name in names:
        try:
            getattr(obj, name)
            print(f"{label}.{name}=YES")
        except Exception:
            print(f"{label}.{name}=NO")


arglist = om.MArgList()
report("MArgParser", om.MArgParser(om.MSyntax(), arglist), [
    "commandArgumentMAngle", "commandArgumentMDistance", "commandArgumentMTime",
    "flagArgumentMAngle", "flagArgumentMDistance", "flagArgumentMTime",
])

report("MFnNurbsSurface", om.MFnNurbsSurface(), [
    "degreeU", "degreeV", "getDerivativesAtParm", "getCVs", "getCV",
])
report("MFnDagNode", om.MFnDagNode(), ["setIntermediateObject"])
report("MQuaternion", om.MQuaternion(), ["getAxisAngle", "setAxisAngle"])
report("MStringArray", om.MStringArray(), ["indexOf"])

cmds.polySphere(n="probeSphere")
sel = om.MSelectionList()
om.MGlobal.getSelectionListByName("probeSphere", sel)
path = om.MDagPath()
sel.getDagPath(0, path)
path.extendToShape()
report("MItMeshFaceVertex", om.MItMeshFaceVertex(path), ["vertId", "faceVertId", "vertexId", "faceVertexId"])
report("MItMeshPolygon", om.MItMeshPolygon(path), [
    "isConnectedToEdge", "isConnectedToFace", "isConnectedToVertex",
    "connectedEdges", "connectedFaces", "connectedVertices",
])
report("MPxCommand", mpx.MPxCommand, [
    "currentIntResult", "currentDoubleResult", "currentStringResult", "getCurrentResult", "currentResult",
])
