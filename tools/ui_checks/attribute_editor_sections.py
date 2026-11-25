#!/usr/bin/env python3
"""A section keeps its state, and the page its scroll, when a value in it changes.

Usage: attribute_editor_sections.py build/aztek [shots dir]

Runs the built app on the current display (or the Xvfb display the other UI checks start) with a script that expands
sections, edits a value in each, then checks the sections, the scroll position, undo and redo, a rename and a tab
switch. Exits 0 when the panel gave what it should. No xdotool: the script drives the widgets itself.
"""
import os
import subprocess
import sys
import tempfile

BIN = os.path.abspath(sys.argv[1])
SHOTS = sys.argv[2] if len(sys.argv) > 2 else None
work = tempfile.mkdtemp(prefix="aztek_ae_sections_")
result = os.path.join(work, "result.txt")
driver = os.path.join(work, "driver.py")
shot = os.path.join(SHOTS or work, "attribute_editor_sections.png")
os.makedirs(os.path.dirname(shot), exist_ok=True)

DRIVER = r'''
import maya.cmds as cmds
import maya.mel as mel
import os, traceback
RESULT = os.environ["AE_SECTIONS_RESULT"]
open(RESULT, "w").write("RUNNING\n")

def log(s):
    with open(RESULT, "a") as f:
        f.write(s + "\n")

try:
    cmds.file(new=True, force=True)
    cmds.polyCube(name="pCube1")
    cmds.select("pCube1")
    mel.eval("AttributeEditor")
    from PySide6.QtCore import QTimer, Qt
    from PySide6.QtWidgets import QApplication, QLineEdit, QCheckBox, QTabWidget, QScrollBar
    from PySide6.QtTest import QTest

    def head(title):
        for w in QApplication.instance().allWidgets():
            if w.objectName() == "aeSectionHead" and w.property("title") == title and w.isVisible():
                return w
        return None

    def opened(title):
        h = head(title)
        return None if h is None else bool(h.property("open"))

    def click(title):
        h = head(title)
        if h is None:
            raise RuntimeError("no section " + title)
        QTest.mouseClick(h, Qt.LeftButton)

    def field(plug):
        for w in QApplication.instance().allWidgets():
            if isinstance(w, QLineEdit) and w.property("plugKey") == plug and w.isVisible():
                return w
        return None

    def bars():
        return [w for w in QApplication.instance().allWidgets()
                if isinstance(w, QScrollBar) and w.orientation() == Qt.Vertical and w.isVisible()]

    def page_bar():
        found = bars()
        return max(found, key=lambda b: b.maximum()) if found else None

    def tabs():
        for w in QApplication.instance().allWidgets():
            if isinstance(w, QTabWidget) and w.objectName() == "aeTabs":
                return w
        return None

    def check(what, ok):
        log(("ok " if ok else "FAIL ") + what)

    scroll_target = [0]

    def t500():
        try:
            click("Pivots"); click("Limit Information"); click("Display")
            check("expanded", opened("Pivots") and opened("Limit Information") and opened("Display"))
            b = page_bar()
            if b is not None:
                scroll_target[0] = 100 if b.maximum() >= 100 else b.maximum()
                b.setValue(scroll_target[0])
            for w in QApplication.instance().allWidgets():
                if isinstance(w, QCheckBox) and w.property("plugKey") == "pCube1.displayRotatePivot" and w.isVisible():
                    w.setChecked(True)
                    break
            else:
                raise RuntimeError("no displayRotatePivot")
        except Exception:
            log("FAIL t500\n" + traceback.format_exc())

    def t650():
        try:
            check("open after checkbox", opened("Pivots") and opened("Limit Information") and opened("Display"))
            w = field("pCube1.selectHandleX")
            if w is None:
                raise RuntimeError("no selectHandleX")
            w.setText("2.5"); w.textEdited.emit("2.5"); w.editingFinished.emit()
        except Exception:
            log("FAIL t650\n" + traceback.format_exc())

    def t750():
        try:
            check("open after text", opened("Pivots") and opened("Limit Information") and opened("Display"))
            check("text written", cmds.getAttr("pCube1.selectHandleX") == 2.5)
            b = page_bar()
            check("scroll kept", b is not None and b.value() == min(scroll_target[0], b.maximum()))
            cmds.undo(); cmds.redo()
        except Exception:
            log("FAIL t750\n" + traceback.format_exc())

    def t850():
        try:
            check("open after undo/redo", opened("Pivots") and opened("Limit Information") and opened("Display"))
            cmds.rename("pCube1", "myCube")
        except Exception:
            log("FAIL t850\n" + traceback.format_exc())

    def t900():
        try:
            t = tabs()
            names = [t.tabText(i) for i in range(t.count())]
            check("rename follows", "myCube" in names and "pCube1" not in names)
            check("open after rename", opened("Pivots") and opened("Limit Information") and opened("Display"))
            if "polyCube1" in names:
                t.setCurrentIndex(names.index("polyCube1"))
            if "myCube" in names:
                t.setCurrentIndex(names.index("myCube"))
            check("open after tab switch", opened("Pivots") and opened("Limit Information") and opened("Display"))
            b = page_bar()
            check("scroll after tab switch", b is not None and b.value() == min(scroll_target[0], b.maximum()))
            click("Display")
            check("collapsed on click", opened("Display") is False)
            cmds.setAttr("myCube.selectHandleX", 7.0)
        except Exception:
            log("FAIL t900\n" + traceback.format_exc())

    def t950():
        try:
            check("stays collapsed after a change", opened("Display") is False)
            check("open sections after a change", opened("Pivots") and opened("Limit Information"))
            check("value from the scene", cmds.getAttr("myCube.selectHandleX") == 7.0)
            failed = "FAIL" in open(RESULT).read()
            with open(RESULT, "a") as f:
                f.write("DONE " + ("FAIL" if failed else "PASS") + "\n")
        except Exception:
            log("FAIL t950\n" + traceback.format_exc())
            with open(RESULT, "a") as f:
                f.write("DONE FAIL\n")

    QTimer.singleShot(500, t500)
    QTimer.singleShot(650, t650)
    QTimer.singleShot(750, t750)
    QTimer.singleShot(850, t850)
    QTimer.singleShot(900, t900)
    QTimer.singleShot(950, t950)
except Exception:
    with open(RESULT, "a") as f:
        f.write("FAIL setup\n" + traceback.format_exc() + "\nDONE FAIL\n")
'''

open(driver, "w").write(DRIVER)
env = dict(os.environ)
env["XDG_CONFIG_HOME"] = work
env["XDG_DATA_HOME"] = work
env["AE_SECTIONS_RESULT"] = result
env["QT_QPA_PLATFORM"] = "xcb"
if "DISPLAY" not in env and "WAYLAND_DISPLAY" not in env:
    sys.exit("run on a display, as the other UI checks do")
subprocess.run([BIN, "-geometry", "1200x800+0+0", "--script", driver, "--screenshot", shot], env=env, timeout=90)
try:
    text = open(result).read()
except FileNotFoundError:
    sys.exit("the app wrote no result")
print(text)
if "DONE PASS" not in text:
    sys.exit(1)
