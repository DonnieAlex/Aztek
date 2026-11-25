#!/usr/bin/env python3
"""Check Attribute Editor colors after switching themes. Run with AppRun under Xvfb."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

with tempfile.TemporaryDirectory(prefix="aztek_ae_theme_") as directory:
    work = Path(directory)
    probe = work / "probe.py"
    probe.write_text("""
import json
import os
from pathlib import Path
import traceback
import maya.cmds as cmds
import maya.mel as mel
from PySide6.QtCore import QTimer
from PySide6.QtGui import QPalette
from PySide6.QtWidgets import QApplication, QComboBox, QTabWidget, QToolButton, QWidget

app = QApplication.instance()
result = Path(os.environ["AZTEK_THEME_RESULT"])

def setup():
    cmds.polyCube(name="themeCube")
    cmds.select("themeCube")
    mel.eval("AttributeEditor")
    window = next(w for w in app.topLevelWidgets() if w.objectName() == "MayaWindow")
    QTimer.singleShot(500, lambda: finish(window))

def sample(widget, x, y):
    return widget.grab().toImage().pixelColor(x, y).lightness()

def check_themes(window):
    themes = window.findChild(QComboBox, "themeBox")
    tabs = window.findChild(QTabWidget, "aeTabs")
    head = next(w for w in tabs.findChildren(QWidget, "aeSectionHead") if w.isVisible())
    menu = next(b.menu() for b in window.findChildren(QToolButton, "aeMenuItem") if b.text() == "Help")
    link = next(b for b in tabs.findChildren(QToolButton, "aeLinkButton") if b.isVisible())
    colors = {}
    for theme in ("classic", "obsidian", "paper"):
        index = themes.findData(theme)
        assert index >= 0, theme
        themes.setCurrentIndex(index)
        themes.activated.emit(index)
        app.processEvents()
        bar = tabs.tabBar()
        tab = bar.tabRect(bar.currentIndex())
        menu.ensurePolished()
        menu.adjustSize()
        header = head.grab().toImage()
        icon = link.icon().pixmap(14, 14).toImage()
        colors[theme] = [
            header.pixelColor(head.width() - 12, head.height() // 2).name(),
            head.palette().color(QPalette.WindowText).name(),
            sample(bar, tab.right() - 5, tab.center().y()),
            sample(menu, 3, menu.height() // 2),
            icon.pixelColor(7, 3).lightness(),
        ]
    result.write_text(json.dumps(colors))

def finish(window):
    try:
        check_themes(window)
    except Exception:
        result.write_text(json.dumps({"error": traceback.format_exc()}))
    finally:
        app.exit(0)

try:
    setup()
except Exception:
    result.write_text(json.dumps({"error": traceback.format_exc()}))
    app.exit(0)
""")
    result = work / "result.json"
    env = dict(os.environ, XDG_CONFIG_HOME=str(work / "config"), AZTEK_PREFS=str(work / "prefs"),
               AZTEK_APP_DIR=str(work / "aztek"), XDG_DATA_HOME=str(work / "data"), AZTEK_THEME_RESULT=str(result))
    subprocess.run([str(Path(sys.argv[1]).resolve()), "--script", str(probe)], env=env, check=True, timeout=25)
    colors = json.loads(result.read_text())
    assert "error" not in colors, colors.get("error")
    dark, light = colors["obsidian"], colors["paper"]
    assert dark[:2] == ["#00e5ff", "#000000"], dark
    assert light[:2] == ["#2b2b2b", "#f6ecd2"], light
    assert light[2] - dark[2] > 40 and light[3] - dark[3] > 40, colors
    assert dark[4] - light[4] > 40, colors
    print("Attribute Editor section, tab, menu and link icon follow the selected theme.")
