//! PLAN 5.3, the 2.0 API's gaps: Python deformers (MPxDeformerNode in OpenAztekAnim and the 1.0 API's OpenMayaMPx,
//! made with the deformer command, deform() moving the mesh's points by the envelope and the painted weights), arrays
//! of compounds on plug-in nodes, and MRampAttribute.

use std::ffi::{CStr, CString};

use aztek_host::*;
use serde_json::Value;

fn log() -> Vec<(i64, String)> {
    let p = aztek_take_log();
    let s = unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned();
    aztek_free(p);
    let v: Value = serde_json::from_str(&s).unwrap();
    v.as_array().unwrap().iter().map(|l| (l["kind"].as_i64().unwrap(), l["text"].as_str().unwrap().to_string())).collect()
}

fn expect(src: &str, want: &[&str]) {
    log();
    let c = CString::new(src).unwrap();
    aztek_python(c.as_ptr());
    let l = log();
    let errors: Vec<&String> = l.iter().filter(|(k, _)| *k == 2).map(|(_, t)| t).collect();
    assert!(errors.is_empty(), "Python failed:\n{}\nrunning:\n{src}", errors.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"));
    let got: Vec<String> = l.into_iter().filter(|(k, _)| *k == 3).map(|(_, t)| t).collect();
    assert_eq!(got, want, "running:\n{src}");
}

const API2: &str = r#"
import maya.api.OpenMaya as om
import maya.api.OpenMayaAnim as oma

def maya_useNewAPI():
    pass

class Lift(oma.MPxDeformerNode):
    amount = None

    def deform(self, block, it, matrix, index):
        env = block.inputValue(oma.MPxGeometryFilter.envelope).asFloat()
        amount = block.inputValue(Lift.amount).asDouble()
        while not it.isDone():
            p = it.position()
            w = self.weightValue(block, index, it.index())
            p.y += amount * env * w
            it.setPosition(p)
            it.next()

def creator():
    return Lift()

def init():
    n = om.MFnNumericAttribute()
    Lift.amount = n.create("amount", "am", om.MFnNumericData.kDouble, 1.0)
    n.keyable = True
    Lift.addAttribute(Lift.amount)
    Lift.attributeAffects(Lift.amount, oma.MPxGeometryFilter.outputGeom)

def initializePlugin(obj):
    om.MFnPlugin(obj, "Testers", "1.0").registerNode("liftDeformer", om.MTypeId(0x7f001), creator, init, om.MPxNode.kDeformerNode)

def uninitializePlugin(obj):
    om.MFnPlugin(obj).deregisterNode(om.MTypeId(0x7f001))
"#;

const API1: &str = r#"
import maya.OpenMaya as OpenMaya
import maya.OpenMayaMPx as OpenMayaMPx

class Push(OpenMayaMPx.MPxDeformerNode):
    def __init__(self):
        OpenMayaMPx.MPxDeformerNode.__init__(self)

    def deform(self, dataBlock, geomIter, matrix, multiIndex):
        env = dataBlock.inputValue(OpenMayaMPx.cvar.MPxGeometryFilter_envelope).asFloat()
        geomIter.reset()
        while not geomIter.isDone():
            pt = geomIter.position()
            pt.x = pt.x + 2.0 * env
            geomIter.setPosition(pt)
            geomIter.next()

def creator():
    return OpenMayaMPx.asMPxPtr(Push())

def init():
    pass

def initializePlugin(obj):
    OpenMayaMPx.MFnPlugin(obj, "Testers", "1.0").registerNode("pushDeformer", OpenMaya.MTypeId(0x7f002), creator, init, OpenMayaMPx.MPxNode.kDeformerNode)

def uninitializePlugin(obj):
    OpenMayaMPx.MFnPlugin(obj).deregisterNode(OpenMaya.MTypeId(0x7f002))
"#;

const RAMPS: &str = r#"
import maya.api.OpenMaya as om

def maya_useNewAPI():
    pass

class Ramped(om.MPxNode):
    def compute(self, plug, block):
        t = block.inputValue(Ramped.inp).asFloat()
        r = om.MRampAttribute(self.thisMObject(), Ramped.curve)
        total = 0.0
        arr = block.inputArrayValue(Ramped.items)
        for i in range(len(arr)):
            arr.jumpToPhysicalElement(i)
            h = arr.inputValue()
            total += h.child(Ramped.weight).asDouble() * h.child(Ramped.factor).asDouble()
        block.outputValue(Ramped.out).setDouble(r.getValueAtPosition(t) + total)
        block.setClean(plug)

def creator():
    return Ramped()

def init():
    n = om.MFnNumericAttribute()
    Ramped.inp = n.create("input", "in", om.MFnNumericData.kFloat, 0.0)
    Ramped.out = n.create("output", "out", om.MFnNumericData.kDouble, 0.0)
    n.writable = False
    Ramped.curve = om.MRampAttribute.createCurveRamp("curve", "cv")
    c = om.MFnCompoundAttribute()
    Ramped.weight = n.create("weight", "w", om.MFnNumericData.kDouble, 0.0)
    Ramped.factor = n.create("factor", "f", om.MFnNumericData.kDouble, 1.0)
    Ramped.items = c.create("items", "it")
    c.addChild(Ramped.weight)
    c.addChild(Ramped.factor)
    c.array = True
    for a in (Ramped.inp, Ramped.out, Ramped.curve, Ramped.items):
        Ramped.addAttribute(a)
    for a in (Ramped.inp, Ramped.curve, Ramped.items):
        Ramped.attributeAffects(a, Ramped.out)

def initializePlugin(obj):
    om.MFnPlugin(obj, "Testers", "1.0").registerNode("rampedNode", om.MTypeId(0x7f003), creator, init)

def uninitializePlugin(obj):
    om.MFnPlugin(obj).deregisterNode(om.MTypeId(0x7f003))
"#;

const TRANSLATOR: &str = r#"
import maya.api.OpenMaya as om
import maya.cmds as cmds

def maya_useNewAPI():
    pass

class Points(om.MPxFileTranslator):
    def haveReadMethod(self):
        return True
    def haveWriteMethod(self):
        return True
    def defaultExtension(self):
        return "pts"
    def writer(self, f, options, mode):
        objs = cmds.ls(sl=True) if mode == om.MPxFileTranslator.kExportActiveAccessMode else cmds.ls(type="transform")
        with open(f.resolvedFullName(), "w") as out:
            for o in objs:
                if o in ("persp", "top", "front", "side"):
                    continue
                out.write("%s %s\n" % (o, " ".join(str(x) for x in cmds.xform(o, q=True, ws=True, t=True))))
    def reader(self, f, options, mode):
        for line in open(f.resolvedFullName()):
            parts = line.split()
            loc = cmds.spaceLocator(n=parts[0] + "_pt")[0]
            cmds.move(*[float(x) for x in parts[1:4]], loc)

def creator():
    return Points()

def initializePlugin(obj):
    om.MFnPlugin(obj, "Testers", "1.0").registerFileTranslator("pointsFile", None, creator)

def uninitializePlugin(obj):
    om.MFnPlugin(obj).deregisterFileTranslator("pointsFile")
"#;

const MESHES: &str = r#"
import maya.api.OpenMaya as om

def maya_useNewAPI():
    pass

class Inflate(om.MPxNode):
    def compute(self, plug, block):
        src = block.inputValue(Inflate.inMesh).asMesh()
        amount = block.inputValue(Inflate.amount).asDouble()
        fn = om.MFnMesh(src)
        pts = fn.getPoints()
        out = om.MFnMeshData().create()
        counts, connects = fn.getVertices()
        om.MFnMesh().create([om.MPoint(p.x * (1 + amount), p.y * (1 + amount), p.z * (1 + amount)) for p in pts], counts, connects, parent=out)
        block.outputValue(Inflate.outMesh).setMObject(out)
        block.setClean(plug)

def creator():
    return Inflate()

def init():
    t = om.MFnTypedAttribute()
    Inflate.inMesh = t.create("inMesh", "im", om.MFnData.kMesh)
    Inflate.outMesh = t.create("outMesh", "om", om.MFnData.kMesh)
    t.writable = False
    n = om.MFnNumericAttribute()
    Inflate.amount = n.create("amount", "am", om.MFnNumericData.kDouble, 1.0)
    for a in (Inflate.inMesh, Inflate.outMesh, Inflate.amount):
        Inflate.addAttribute(a)
    Inflate.attributeAffects(Inflate.inMesh, Inflate.outMesh)
    Inflate.attributeAffects(Inflate.amount, Inflate.outMesh)

def initializePlugin(obj):
    om.MFnPlugin(obj, "Testers", "1.0").registerNode("inflateMesh", om.MTypeId(0x7f004), creator, init)

def uninitializePlugin(obj):
    om.MFnPlugin(obj).deregisterNode(om.MTypeId(0x7f004))
"#;

#[test]
fn python_plugin_kinds() {
    let dir = std::env::temp_dir().join(format!("aztek_pydef_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("liftDeformer.py"), API2).unwrap();
    std::fs::write(dir.join("pushDeformer.py"), API1).unwrap();
    std::fs::write(dir.join("rampedNode.py"), RAMPS).unwrap();
    std::fs::write(dir.join("pointsFile.py"), TRANSLATOR).unwrap();
    std::fs::write(dir.join("inflateMesh.py"), MESHES).unwrap();
    let dirs = dir.to_str().unwrap().replace('\\', "/");
    expect(
        &format!(
            r#"
import os
import maya.cmds as cmds
os.environ["AZTEK_PLUG_IN_PATH"] = "{dirs}"
cmds.newScene()
cmds.loadPlugin("liftDeformer"); cmds.loadPlugin("pushDeformer")
cmds.polyPlane(n="p", sx=1, sy=1)
d = cmds.deformer("p", type="liftDeformer")[0]
print(cmds.nodeType(d), round(cmds.pointPosition("p.vtx[0]")[1], 4))
cmds.setAttr(d + ".amount", 3.0)
print(round(cmds.pointPosition("p.vtx[0]")[1], 4))
cmds.setAttr(d + ".envelope", 0.5)
print(round(cmds.pointPosition("p.vtx[0]")[1], 4))
cmds.setAttr(d + ".envelope", 1.0)
cmds.percent(d, "p.vtx[1]", v=0.0)
print(round(cmds.pointPosition("p.vtx[1]")[1], 4), round(cmds.pointPosition("p.vtx[2]")[1], 4))
cmds.polyCube(n="c")
x0 = cmds.pointPosition("c.vtx[0]")[0]
e = cmds.deformer("c", type="pushDeformer")[0]
print(cmds.nodeType(e), round(cmds.pointPosition("c.vtx[0]")[0] - x0, 4))
"#
        ),
        &["liftDeformer 1.0", "3.0", "1.5", "0.0 3.0", "pushDeformer 2.0"],
    );
    expect(
        r#"
import maya.api.OpenMaya as om
cmds.loadPlugin("rampedNode")
n = cmds.createNode("rampedNode")
sel = om.MSelectionList(); sel.add(n)
node = sel.getDependNode(0)
fn = om.MFnDependencyNode(node)
r = om.MRampAttribute(node, fn.attribute("curve"))
r.addEntries([0.0, 1.0], [0.0, 10.0], [om.MRampAttribute.kLinear, om.MRampAttribute.kLinear])
print(r.numEntries(), r.isCurveRamp(), round(r.getValueAtPosition(0.25), 4))
cmds.setAttr(n + ".input", 0.5)
print(round(cmds.getAttr(n + ".output"), 4))
cmds.setAttr(n + ".items[0].weight", 2.0)
cmds.setAttr(n + ".items[3].weight", 1.0)
cmds.setAttr(n + ".items[3].factor", 4.0)
print(round(cmds.getAttr(n + ".output"), 4))
ids, pos, vals, ints = r.getEntries()
print(list(ids), [round(x, 3) for x in pos], [round(x, 3) for x in vals], list(ints))
"#,
        &["2 True 2.5", "5.0", "11.0", "[0, 1] [0.0, 1.0] [0.0, 10.0] [1, 1]"],
    );
    expect(
        &format!(
            r#"
cmds.newScene()
cmds.loadPlugin("pointsFile")
print("pointsFile" in cmds.translator(q=True, list=True), cmds.translator("pointsFile", q=True, extension=True))
cmds.polyCube(n="a"); cmds.move(1, 2, 3, "a")
cmds.polyCube(n="b"); cmds.move(4, 0, 0, "b")
cmds.select("a")
cmds.file("{dirs}/sel.pts", exportSelected=True, type="pointsFile")
print(open("{dirs}/sel.pts").read().split())
cmds.file("{dirs}/sel.pts", i=True)
print(cmds.xform("a_pt", q=True, ws=True, t=True))
cmds.unloadPlugin("pointsFile")
print("pointsFile" in cmds.translator(q=True, list=True))
"#
        ),
        &["True pts", "['a', '1.0', '2.0', '3.0']", "[1.0, 2.0, 3.0]", "False"],
    );
    // Meshes into and out of a compute.
    expect(
        r#"
cmds.newScene()
cmds.loadPlugin("inflateMesh")
cmds.polyCube(n="src")
n = cmds.createNode("inflateMesh")
cmds.connectAttr("srcShape.outMesh", n + ".inMesh")
out = cmds.createNode("mesh", n="outShape")
cmds.connectAttr(n + ".outMesh", "outShape.inMesh")
print(cmds.polyEvaluate("outShape", v=True), [round(x, 3) for x in cmds.pointPosition("outShape.vtx[0]")])
cmds.setAttr(n + ".amount", 0.0)
print([round(x, 3) for x in cmds.pointPosition("outShape.vtx[0]")])
"#,
        &["8 [-1.0, -1.0, 1.0]", "[-0.5, -0.5, 0.5]"],
    );
    // Several UV sets and shear through the function sets.
    expect(
        r#"
cmds.newScene()
cmds.polyPlane(n="p", sx=1, sy=1)
sel = om.MSelectionList(); sel.add("p")
fn = om.MFnMesh(sel.getDagPath(0))
s2 = fn.createUVSet("second")
cmds.polyUVSet("p", copy=True, uvSet="map1", newUVSet="second")
cmds.polyUVSet("p", currentUVSet=True, uvSet="second")
cmds.polyEditUV("p.map[0]", u=0.5, v=0.0)
fn.setCurrentUVSetName("map1")
print(fn.getUVSetNames(), fn.getCurrentUVSetName(), fn.numUVSets())
print([round(x, 3) for x in fn.getUVs()[0]], [round(x, 3) for x in fn.getUVs("second")[0]])
t = om.MFnTransform(sel.getDagPath(0))
t.setShear([0.5, 0.0, 0.25])
print(t.shear(), cmds.getAttr("p.shearXY"))
"#,
        &["['map1', 'second'] map1 2", "[0.0, 1.0, 1.0, 0.0] [0.5, 1.0, 1.0, 0.0]", "[0.5, 0.0, 0.25] 0.5"],
    );
    let _ = std::fs::remove_dir_all(&dir);
}
