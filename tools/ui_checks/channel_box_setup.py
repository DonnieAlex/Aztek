# Two cubes selected, and whenever a value changes or an undo runs, the values the check reads written to a file. Not on
# idle: a job that fires then moves the core's version, the panels rebuild and a field being typed in loses the keyboard.
import json
import os

import maya.cmds as cmds

OUT = os.environ["CHECK_STATE"]
cmds.polyCube()
cmds.polyCube()
cmds.select("pCube1", "pCube2")


def write_state():
    state = {
        c: [cmds.getAttr("%s.s%s" % (c, a)) for a in "xyz"] + [cmds.getAttr("%s.t%s" % (c, a)) for a in "xyz"] for c in ("pCube1", "pCube2")
    }
    with open(OUT + ".tmp", "w") as f:
        json.dump(state, f)
    os.replace(OUT + ".tmp", OUT)


write_state()
for c in ("pCube1", "pCube2"):
    for a in ("sx", "sy", "sz", "tx", "ty", "tz"):
        cmds.scriptJob(attributeChange=["%s.%s" % (c, a), write_state])
cmds.scriptJob(event=["Undo", write_state])


# The UI check clicks the label and value columns by position. The layout has changed (the button row, one label column
# for every section), so write where those are once the window has drawn, rather than keeping numbers in the check.
def write_geometry():
    from PySide6.QtCore import QPoint
    from PySide6.QtWidgets import QApplication, QLabel, QLineEdit

    main = next((w for w in QApplication.topLevelWidgets() if w.isWindow() and w.objectName() == "MayaWindow"), None)
    if not main:
        return
    field = next((w for w in main.findChildren(QLineEdit) if w.objectName() == "channelValue"), None)
    label = next((w for w in main.findChildren(QLabel) if w.objectName() == "channelName"), None)
    if not field or not label:
        return
    names = {"Translate X": "tx", "Translate Y": "ty", "Translate Z": "tz", "Rotate X": "rx", "Rotate Y": "ry",
             "Rotate Z": "rz", "Scale X": "sx", "Scale Y": "sy", "Scale Z": "sz"}
    rows = {}
    for w in main.findChildren(QLabel):
        key = names.get(w.text()) if w.objectName() == "channelName" else None
        if key and key not in rows:
            at = w.mapTo(main, QPoint(0, 0))
            rows[key] = at.y() + w.height() // 2
    headers = [w.text() for w in main.findChildren(QLineEdit) if w.objectName() == "channelHeader"]
    data = {"label_x": label.mapTo(main, QPoint(0, 0)).x() + 8,
            "field_x": field.mapTo(main, QPoint(0, 0)).x() + 6,
            "headers": headers,
            "rows": rows}
    with open(OUT + ".geom.tmp", "w") as f:
        json.dump(data, f)
    os.replace(OUT + ".geom.tmp", OUT + ".geom")


from PySide6.QtCore import QTimer
QTimer.singleShot(900, write_geometry)
