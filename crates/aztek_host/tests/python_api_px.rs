//! The Python API's plug-in classes beyond plain nodes and commands: plug-in data, shapes, locators, transforms,
//! constraints and IK solvers (python_plugins/), the messages, NURBS surfaces, and the thin maya.api.OpenMayaUI and
//! maya.api.OpenMayaRender. Python runs once per process, so one test walks through.

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

/// Runs Python and gives back what it printed, with the rest of the log.
fn run_py(src: &str) -> (Vec<String>, Vec<String>) {
    log();
    let c = CString::new(src).unwrap();
    aztek_python(c.as_ptr());
    let l = log();
    let rest = l.iter().filter(|(k, _)| *k != 3).map(|(k, t)| format!("[{k}] {t}")).collect();
    (l.into_iter().filter(|(k, _)| *k == 3).map(|(_, t)| t).collect(), rest)
}

fn expect(src: &str, want: &[&str]) {
    log();
    let c = CString::new(src).unwrap();
    aztek_python(c.as_ptr());
    let l = log();
    let errors: Vec<&String> = l.iter().filter(|(k, _)| *k == 2).map(|(_, t)| t).collect();
    assert!(errors.is_empty(), "Python failed:\n{}\nrunning:\n{src}", errors.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"));
    let rest: Vec<String> = l.iter().filter(|(k, _)| *k != 3).map(|(k, t)| format!("[{k}] {t}")).collect();
    let got: Vec<String> = l.into_iter().filter(|(k, _)| *k == 3).map(|(_, t)| t).collect();
    // Warnings and the rest of the log come along, so a failure on another machine says why.
    assert_eq!(got, want, "running:\n{src}\nlog:\n{}", rest.join("\n"));
}

fn plugin_dir() -> String {
    let dir = std::env::temp_dir().join(format!("aztek_px_py_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("pxPy1.py"), include_str!("python_plugins/pxPy1.py")).unwrap();
    std::fs::write(dir.join("pxPy2.py"), include_str!("python_plugins/pxPy2.py")).unwrap();
    dir.to_str().unwrap().replace('\\', "/")
}

fn imports() {
    expect(
        r#"
import maya.api.OpenMayaUI as omui, maya.api.OpenMayaRender as omr, aztek.api.OpenAztekUI as azui
print(omui is azui, omui.MPxLocatorNode.__name__, omr.MPxDrawOverride.__name__, omr.MUIDrawManager.__name__)
print(omr.MGeometry.kTriangles == omr.MGeometry.kTriangles, omr.MRenderer.setGeometryDrawDirty(None))
try:
    omr.MRenderer.theRenderer().setPresentOnScreen(True)
except NotImplementedError as e:
    print(e)
import maya.OpenMayaMPx as mpx
print(mpx.MPxTransform.__name__, mpx.MPxConstraint.__name__, mpx.MPxIkSolverNode.__name__, mpx.MPxSurfaceShape is om.MPxSurfaceShape)
print(om.MPxNode.kTransformNode, om.MPxNode.kIkSolverNode, om.MPxNode.kConstraintNode, om.MPxNode.kLast)
print(om.MFn.kPluginDependNode, om.MFn.kPluginTransformNode, om.MFn.kPluginShape, om.MFn.kTransform, om.MFn.kMeshVertComponent)
o = om.MSelectionList().add(cmds.createNode("absoluteDL")).getDependNode(0)
print(o.apiType() == om.MFn.kAbsolute, o.apiTypeStr, om.MSelectionList().add(cmds.createNode("addDL")).getDependNode(0).apiTypeStr)
"#,
        &[
            "True MPxLocatorNode MPxDrawOverride MUIDrawManager",
            "True None",
            "MRenderer.setPresentOnScreen is not in Aztek yet",
            "MPxTransform MPxConstraint MPxIkSolverNode True",
            "11 8 17 26",
            "459 914 712 110 554",
            "True kAbsolute kAddDoubleLinear",
        ],
    );
}

fn data_shapes_locators(dir: &str) {
    // Where the reopened value went, should it not come back: printed with the failure.
    let probe = std::panic::catch_unwind(|| data_shapes_locators_run(dir));
    if let Err(e) = probe {
        let (out, rest) = run_py(
            r#"
s = "pySumPoints1"
print("plugins", cmds.pluginInfo(q=True, listPlugins=True))
print("nodes", cmds.ls(type="pySumPoints"), cmds.ls(type="pyMakePoints"))
print("attrs", cmds.listAttr(s))
print("connections", cmds.listConnections(s, c=True, p=True))
try:
    print("pts", cmds.getAttr(s + ".pts"))
except Exception as ex:
    print("pts error", ex)
o = om.MSelectionList().add(s).getDependNode(0)
p = om.MFnDependencyNode(o).findPlug("pts", False)
try:
    d = p.asMObject()
    print("pts data", d.apiTypeStr, getattr(om.MFnPluginData(d).data(), "values", None))
except Exception as ex:
    print("pts data error", ex)
print("sum again", cmds.getAttr(s + ".sum"))
cmds.setAttr(s + ".pts", type="pyPoints")
print("dirty sum", cmds.getAttr(s + ".sum"))
print("file", [l for l in open(cmds.file(q=True, sn=True)).read().splitlines() if "requires" in l or "py" in l.lower() or ".pts" in l])
"#,
        );
        let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|x| x.to_string())).unwrap_or_default();
        panic!("{msg}\n\nafter reopening:\n{}\n{}", out.join("\n"), rest.join("\n"));
    }
}

fn data_shapes_locators_run(dir: &str) {
    expect(
        &format!(
            r#"
import os
os.environ["AZTEK_PLUG_IN_PATH"] = "{dir}"
print(cmds.loadPlugin("pxPy2"))
m = cmds.createNode("pyMakePoints")
s = cmds.createNode("pySumPoints")
cmds.connectAttr(m + ".points", s + ".points")
cmds.setAttr(m + ".count", 4)
print(cmds.getAttr(s + ".sum"))
b = cmds.createNode("pyBoxShape")
cmds.setAttr(b + ".size", 3)
print(cmds.getAttr(b + ".boundingBoxMin"), cmds.getAttr(b + ".boundingBoxMax"))
o = om.MSelectionList().add(b).getDependNode(0)
print(o.hasFn(om.MFn.kPluginShape), o.hasFn(om.MFn.kShape))
loc = cmds.createNode("pyMarker")
print(cmds.nodeType(loc), om.MSelectionList().add(loc).getDependNode(0).hasFn(om.MFn.kPluginLocatorNode))
cmds.disconnectAttr(m + ".points", s + ".points")
path = os.path.join("{dir}", "points.ma")
cmds.file(rename=path)
cmds.file(save=True, type="mayaAscii", force=True)
print([l.strip() for l in open(path).read().splitlines() if "pyPoints" in l])
cmds.file(path, open=True, force=True)
fn = om.MFnDependencyNode(om.MSelectionList().add(s).getDependNode(0))
print(cmds.getAttr(s + ".sum"))
"#
        ),
        &[
            "['pxPy2']",
            "3.0",
            "[(-1.5, -1.5, -1.5)] [(1.5, 1.5, 1.5)]",
            "True True",
            "pyMarker True",
            r#"['setAttr ".pts" -type "pyPoints" 4 0.0 0.5 1.0 1.5;']"#,
            "3.0",
        ],
    );
}

fn transforms_constraints_solvers() {
    expect(
        r#"
print(cmds.loadPlugin("pxPy1"))
r = cmds.createNode("pyRockTransform", n="rock")
cmds.setAttr(r + ".translate", 1, 2, 3)
cmds.setAttr(r + ".rockX", math.pi / 2)
print([round(x, 4) for x in cmds.getAttr(r + ".matrix")])
print(om.MSelectionList().add(r).getDependNode(0).hasFn(om.MFn.kPluginTransformNode))
a = cmds.spaceLocator(n="a")[0]; b = cmds.spaceLocator(n="b")[0]; c = cmds.spaceLocator(n="c")[0]
cmds.move(2, 0, 0, a); cmds.move(0, 4, 0, b)
made = cmds.pyAvgConstraint(a, b, c)
print(made, cmds.listRelatives(made, p=True), cmds.getAttr(c + ".translate"))
cmds.setAttr(made + ".targetWeight[1]", 3)
print(cmds.getAttr(c + ".translate"))
cmds.select(cl=True)
j1 = cmds.joint(p=(0, 0, 0)); j2 = cmds.joint(p=(0, 2, 0))
h = cmds.ikHandle(sj=j1, ee=j2, solver="pyAimSolver")[0]
cmds.move(2, 0, 0, h)
print([round(x, 3) for x in cmds.xform(j2, q=True, ws=True, t=True)])
import maya.OpenMayaMPx as mpx
try:
    mpx.MFnPlugin.registerIkSolver(None, "x", None)
except RuntimeError as e:
    print("obsolete" in str(e))
"#,
        &[
            "['pxPy1']",
            "[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 1.0]",
            "True",
            "c_pyAvgConstraint1 ['c'] [(1.0, 2.0, 0.0)]",
            "[(0.5, 3.0, 0.0)]",
            "[2.0, 0.0, 0.0]",
            "True",
        ],
    );
}

fn messages() {
    expect(
        r#"
cmds.newScene()
log = []
ids = [
    om.MSceneMessage.addCallback(om.MSceneMessage.kAfterNew, lambda cd: log.append("afterNew %s" % cd), 7),
    om.MSceneMessage.addCheckCallback(om.MSceneMessage.kBeforeNewCheck, lambda cd: (log.append("check"), False)[1]),
]
try:
    cmds.file(new=True, force=True)
except RuntimeError as e:
    log.append(str(e))
om.MMessage.removeCallback(ids.pop())
cmds.file(new=True, force=True)
a = cmds.createNode("transform", n="a")
b = cmds.createNode("transform", n="b")
ids.append(om.MDGMessage.addNodeAddedCallback(lambda n, cd: log.append("added " + om.MFnDependencyNode(n).name()), "transform"))
ids.append(om.MDGMessage.addConnectionCallback(lambda s, d, made, cd: log.append("conn %s %s %s" % (s.name(), d.name(), made))))
ids.append(om.MDagMessage.addParentAddedCallback(lambda c, p, cd: log.append("parent %s %s" % (c.partialPathName(), p.partialPathName() if p.length() else "world"))))
node = om.MSelectionList().add("a").getDependNode(0)
ids.append(om.MNodeMessage.addAttributeChangedCallback(node, lambda m, p, o, cd: log.append("attr %d %s" % (m, p.name()))))
ids.append(om.MNodeMessage.addNameChangedCallback(node, lambda n, old, cd: log.append("name %s was %s" % (om.MFnDependencyNode(n).name(), old))))
def once(n, cd):
    log.append("once " + om.MFnDependencyNode(n).name())
    om.MMessage.removeCallback(om.MMessage.currentCallbackId())
om.MDGMessage.addNodeAddedCallback(once)
cmds.parent(b, a)
cmds.connectAttr(a + ".tx", b + ".ty")
cmds.setAttr("a.tz", 2)
cmds.rename("a", "a2")
cmds.createNode("transform", n="c")
cmds.createNode("transform", n="d")
print(len(om.MMessage.nodeCallbacks(node)))
om.MMessage.removeCallbacks(ids)
cmds.createNode("transform", n="e")
for l in log: print(l)
import maya.OpenMaya as om1
log = []
def check1(ret, cd):
    log.append("check1")
    om1.MScriptUtil.setBool(ret, False)
i1 = om1.MSceneMessage.addCheckCallback(om1.MSceneMessage.kBeforeNewCheck, check1)
i2 = om1.MDGMessage.addNodeAddedCallback(lambda n, cd: log.append("added1 %s %s" % (type(n).__module__, om1.MFnDependencyNode(n).name())))
try:
    cmds.file(new=True, force=True)
except RuntimeError:
    pass
cmds.createNode("transform", n="f")
om1.MMessage.removeCallback(i1)
om1.MMessage.removeCallback(i2)
print(log)
"#,
        &[
            "2",
            "check",
            "File operation cancelled by user supplied callback.",
            "afterNew 7",
            "parent b a",
            "conn a.translateX b.translateY True",
            "attr 1 a.translateX",
            "attr 2056 a.translateZ",
            "name a2 was a",
            "added c",
            "once c",
            "parent c world",
            "added d",
            "parent d world",
            "['check1', 'added1 aztek.OpenAztek f']",
        ],
    );
}

fn surfaces_and_values() {
    expect(
        r#"
s = cmds.nurbsPlane(n="pl", w=2, d=3, u=2, v=2, ax=(0, 1, 0))[0]
fn = om.MFnNurbsSurface(om.MSelectionList().add(s).getDagPath(0))
print(fn.numCVsInU, fn.numCVsInV, fn.degreeInU, fn.formInU == om.MFnNurbsSurface.kOpen, len(fn.cvPositions()), fn.numKnotsInU)
p, u, v = fn.closestPoint(om.MPoint(0.2, 1, 0.1))
print([round(x, 3) for x in (p.x, p.y, p.z)], fn.isPointOnSurface(p), round(fn.area(), 3))
print(fn.cvPosition(0, 0) == fn.cvPositions()[0], [round(x, 3) for x in fn.normal(u, v)])
u = om.MUuid(); u.generate(); print(u.valid(), len(u.asString()), om.MUuid(u.asString()) == u, om.MUuid().valid())
f = om.MFileObject(); f.setRawFullName("/tmp/dir/x.ma"); print(f.rawName(), f.rawPath(), f.exists())
print(isinstance(om.MCallbackIdArray([1, 2]), om.MIntArray))
"#,
        &[
            "5 5 3 True 25 7",
            "[0.2, 0.0, 0.1] True 4.0",
            "True [0.0, 1.0, 0.0]",
            "True 36 True False",
            "x.ma /tmp/dir/ False",
            "True",
        ],
    );
}

#[test]
fn python_api_px() {
    aztek_init();
    expect("import maya.cmds as cmds\nimport maya.api.OpenMaya as om\nimport math\ncmds.newScene()", &[]);
    let dir = plugin_dir();
    imports();
    data_shapes_locators(&dir);
    transforms_constraints_solvers();
    messages();
    surfaces_and_values();
}
