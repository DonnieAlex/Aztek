# Writes what XJEN's tests need from the reference's grooming system. Run it in the reference's Script Editor (Python tab), in a
# session where a groom was made by hand as below, or start the steps from the top: it records the commands the menus
# run while you work.
#
# 1. New scene, clean preferences. Run with STEP=start:
#      import os; os.environ["STEP"] = "start"; exec(open("/path/to/tools/reference_capture/xgen_capture.py").read())
#    This turns on Echo All Commands and writes the Script Editor's history to a file from then on.
# 2. By hand: polyPlane (defaults); Generate > Create Interactive Groom Splines with its defaults; in the Interactive
#    Groom Editor add one of each modifier the Add Modifier menu offers; with the Comb tool make one stroke; add a
#    second sculpt layer and set its weight to 0.5; select a few faces and use Select > Select Splines on Selected Face.
# 3. Run again with STEP=finish. It writes tests/data/reference/xgen/ (or CAPTURE_OUT): commit that folder.
import json
import os
import traceback

import maya.cmds as cmds
import maya.mel as mel

try:
    HERE = os.path.dirname(os.path.abspath(__file__))
except NameError:
    HERE = os.environ.get("CAPTURE_DIR", os.getcwd())
OUT = os.environ.get("CAPTURE_OUT") or os.path.join(HERE, "..", "..", "tests", "data", "reference", "xgen")
HISTORY = os.path.join(OUT, "history.mel")


def plain(v):
    if v is None or isinstance(v, (bool, int, float, str)):
        return v
    if isinstance(v, (list, tuple)):
        return [plain(x) for x in v]
    if isinstance(v, dict):
        return {str(k): plain(x) for k, x in v.items()}
    return repr(v)


def attempt(fn, *args, **kwargs):
    try:
        return plain(fn(*args, **kwargs))
    except Exception as e:
        return {"error": str(e).strip()}


def write(rel, data):
    path = os.path.join(OUT, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", newline="\n") as f:
        json.dump(data, f, indent=1, sort_keys=True)
        f.write("\n")


def start():
    os.makedirs(OUT, exist_ok=True)
    attempt(cmds.loadPlugin, "xgenToolkit", quiet=True)
    cmds.scriptEditorInfo(echoAllCommands=True)
    cmds.scriptEditorInfo(historyFilename=HISTORY, writeHistory=True)
    print("Recording to %s. Make the groom, then run with STEP=finish." % HISTORY)


QUERIES = ("shortName", "longName", "attributeType", "keyable", "channelBox", "storable", "hidden", "multi",
           "listParent", "listChildren", "listDefault", "listEnum", "softRange", "range")


def node_attributes(node):
    rows = {}
    for a in cmds.listAttr(node) or []:
        if "." in a:
            continue
        row = {q: attempt(cmds.attributeQuery, a, node=node, **{q: True}) for q in QUERIES}
        row["type"] = attempt(cmds.getAttr, node + "." + a, type=True)
        row["value"] = attempt(cmds.getAttr, node + "." + a)
        rows[a] = row
    return rows


def finish():
    cmds.scriptEditorInfo(writeHistory=False)
    cmds.scriptEditorInfo(echoAllCommands=False)
    write("meta.json", {"version": cmds.about(version=True), "api": cmds.about(apiVersion=True), "os": cmds.about(os=True)})
    plugin = {
        "commands": attempt(cmds.pluginInfo, "xgenToolkit", q=True, command=True),
        "nodes": attempt(cmds.pluginInfo, "xgenToolkit", q=True, dependNode=True),
        "nodeIds": attempt(cmds.pluginInfo, "xgenToolkit", q=True, dependNodeId=True),
        "translators": attempt(cmds.pluginInfo, "xgenToolkit", q=True, translator=True),
        "contexts": [c for c in (cmds.lsUI(contexts=True) or []) if c.lower().startswith(("xgm", "xg"))],
        "runtime": [r for r in (cmds.runTimeCommand(q=True, userCommandArray=True) or []) + (cmds.runTimeCommand(q=True, defaultCommandArray=True) or []) if "xg" in r.lower() or "groom" in r.lower()],
    }
    write("plugin.json", plugin)
    # Every node type of the plug-in, made on its own: attributes and defaults.
    for t in plugin["nodes"] if isinstance(plugin["nodes"], list) else []:
        info = {"inherited": attempt(cmds.nodeType, t, inherited=True, isTypeName=True)}
        node = attempt(cmds.createNode, t, skipSelect=True)
        if isinstance(node, str):
            info["default_name"] = node
            info["attributes"] = node_attributes(node)
            parents = cmds.listRelatives(node, parent=True) or []
            attempt(cmds.delete, parents or node)
        else:
            info["create_error"] = node
        write("types/%s.json" % t, info)
    # The groom made by hand: its nodes, their values and connections.
    types = set(plugin["nodes"]) if isinstance(plugin["nodes"], list) else set()
    nodes = [n for n in cmds.ls(long=True) if cmds.nodeType(n) in types or cmds.nodeType(n).startswith("xgm")]
    scene = {}
    for n in nodes:
        scene[n] = {
            "type": cmds.nodeType(n),
            "parent": attempt(cmds.listRelatives, n, parent=True, fullPath=True),
            "attributes": node_attributes(n),
            "connections": attempt(cmds.listConnections, n, connections=True, plugs=True),
        }
    write("scene_nodes.json", scene)
    descs = attempt(mel.eval, "xgmSplineQuery -listSplineDescriptions")
    queries = {"listSplineDescriptions": descs, "long": attempt(mel.eval, "xgmSplineQuery -listSplineDescriptions -long")}
    for d in descs if isinstance(descs, list) else []:
        queries[d] = {
            "isSplineDescription": attempt(mel.eval, 'xgmSplineQuery -isSplineDescription "%s"' % d),
            "splineCount": attempt(mel.eval, 'xgmSplineQuery -splineCount "%s"' % d),
        }
        dump = os.path.join(OUT, "spline_data_%s.txt" % d.split("|")[-1]).replace("\\", "/")
        queries[d]["export"] = attempt(mel.eval, 'xgmExportSplineDataInternal -output "%s" "%s.outRenderData"' % (dump, d))
    write("queries.json", queries)
    scene_path = cmds.file(q=True, sceneName=True)
    for kind, ext in (("mayaAscii", "ma"), ("mayaBinary", "mb")):
        attempt(cmds.file, rename=os.path.join(OUT, "groom.%s" % ext))
        attempt(cmds.file, save=True, type=kind, force=True)
    if scene_path:
        attempt(cmds.file, rename=scene_path)
    print("Written to %s" % OUT)


try:
    {"start": start, "finish": finish}[os.environ.get("STEP", "finish")]()
except Exception:
    traceback.print_exc()
