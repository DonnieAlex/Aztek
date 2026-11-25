#!/usr/bin/env python3
"""Check workspace layouts under Xvfb. Usage: workspace_startup.py build/Aztek/AppRun."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile


app = Path(sys.argv[1]).resolve()
with tempfile.TemporaryDirectory(prefix="aztek_workspace_") as directory:
    work = Path(directory)
    settings = work / "config/aztek/Aztek.conf"
    settings.parent.mkdir(parents=True)
    settings.write_text("[General]\nworkspaces_current=General\n")
    probe = work / "probe.py"
    probe.write_text("""
import json
import os
from pathlib import Path
from PySide6.QtCore import Qt
from PySide6.QtWidgets import QApplication, QComboBox, QDockWidget

window = next(w for w in QApplication.topLevelWidgets() if w.objectName() == "MayaWindow")
if os.environ.get("AZTEK_WORKSPACE_SWITCH"):
    # Rendering docks the Script Editor under the views; back in General the sliders keep their own height (#312).
    box = window.findChild(QComboBox, "workspaceBox")
    sliders = [window.findChild(QDockWidget, name) for name in ("TimeSlider", "RangeSlider")]
    for name in ("Rendering - Standard", "General"):
        box.activated.emit(box.findText(name))
        for _ in range(10):
            QApplication.processEvents()
    check = [d.height() <= d.widget().sizeHint().height() + 8 for d in sliders]
elif os.environ.get("AZTEK_WORKSPACE_LEGACY"):
    sliders = [window.findChild(QDockWidget, name) for name in ("TimeSlider", "RangeSlider")]
    side_docks = [window.findChild(QDockWidget, name) for name in ("outlinerDock", "attrDock")]
    check = [window.isMaximized(), all(d.isFloating() for d in side_docks),
             all(window.dockWidgetArea(d) == Qt.BottomDockWidgetArea and d.isVisible() for d in sliders)]
else:
    uv = window.findChild(QDockWidget, "workspace_uvEditor")
    check = [window.findChild(QComboBox, "workspaceBox").currentText(),
             window.findChild(QDockWidget, "TimeSlider").isVisible(),
             uv is not None and uv.isVisible()]
Path(os.environ["AZTEK_WORKSPACE_PROBE"]).write_text(json.dumps(check))
window.close()
QApplication.instance().quit()
""")
    result = work / "result.json"
    env = dict(os.environ, XDG_CONFIG_HOME=str(work / "config"), AZTEK_PREFS=str(work / "prefs"),
               AZTEK_APP_DIR=str(work / "aztek"), AZTEK_WORKSPACE_PROBE=str(result))
    cases = [("General", True, False), ("Modeling - Expert", False, False),
             ("UV Editing", False, True), ("UV Editing", False, True), ("General", True, False)]
    for name, timeline, uv in cases:
        # Change the chosen workspace while retaining the previous session's dock state.
        lines = settings.read_text().splitlines()
        settings.write_text("\n".join(f"workspaces_current={name}" if line.startswith("workspaces_current=")
                                      else line for line in lines) + "\n")
        result.unlink(missing_ok=True)
        subprocess.run([str(app), "--script", str(probe)], env=env, check=True, timeout=20)
        observed = json.loads(result.read_text())
        assert observed == [name, timeline, uv], observed
    # From Rendering back to General, the Time and Range Sliders at their own height (#312).
    result.unlink(missing_ok=True)
    subprocess.run([str(app), "--script", str(probe)], env=dict(env, AZTEK_WORKSPACE_SWITCH="1"), check=True, timeout=20)
    observed = json.loads(result.read_text())
    assert observed == [True, True], observed
    # A maximized 2.513 window with floating side docks, saved before the sliders were docks (#289).
    settings.write_bytes(Path(__file__).with_name("legacy_dock_layout.conf").read_bytes())
    result.unlink()
    subprocess.run([str(app), "--script", str(probe)],
                   env=dict(env, QT_SCALE_FACTOR="1.5", AZTEK_WORKSPACE_LEGACY="1"), check=True, timeout=20)
    observed = json.loads(result.read_text())
    assert observed == [True, True, True], observed
print("Startup workspace and dock restore passed.")
