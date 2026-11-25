#!/usr/bin/env python3
"""Check bright emission on a Vulkan device that supports Realtime. Run under Xvfb with the built app path."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

app = Path(sys.argv[1]).resolve()
scene = Path(__file__).resolve().parents[2] / "examples/cornell_box.ma"
with tempfile.TemporaryDirectory(prefix="aztek_emission_") as directory:
    work = Path(directory)
    probe = work / "probe.py"
    probe.write_text("""
import json
import os
import sys
import traceback
from pathlib import Path
from aztek import cmds
from PySide6.QtCore import QTimer
from PySide6.QtWidgets import QApplication, QToolBar

sys.excepthook = lambda *exc: Path(os.environ["AZTEK_EMISSION_RESULT"]).write_text("".join(traceback.format_exception(*exc)))
main = next(w for w in QApplication.topLevelWidgets() if w.objectName() == "MayaWindow")
cmds.delete("lamp")
emitter = cmds.polyPlane(name="emitter", width=2.0, height=2.0, subdivisionsX=1, subdivisionsY=1)[0]
cmds.rotate(65, 0, 0, emitter)
cmds.move(0, 2.5, -1, emitter)
shader = cmds.shadingNode("standardSurface", asShader=True)
cmds.setAttr(shader + ".base", 0)
cmds.setAttr(shader + ".specular", 0)
cmds.setAttr(shader + ".emissionColor", 1.0, 0.12, 0.0, type="double3")
cmds.hyperShade(emitter, assign=shader)
cmds.select(clear=True)
cmds.lookThru("boxCam")
bounce = next(a for bar in main.findChildren(QToolBar) if bar.isVisible() for a in bar.actions()
              if a.text() == "Bounce Light (Default renderer)")
cases = iter([(10, False), (50, False), (10, True), (50, True)])
values = []

def capture():
    case = next(cases, None)
    if case is None:
        Path(os.environ["AZTEK_EMISSION_RESULT"]).write_text(json.dumps(values))
        cmds.file(new=True, force=True)
        main.close()
        return
    strength, enabled = case
    cmds.setAttr(shader + ".emission", strength)
    if bounce.isChecked() != enabled:
        bounce.trigger()
    QTimer.singleShot(1600, measure)

def measure():
    image = main.screen().grabWindow(main.winId()).toImage()
    w, h = image.width(), image.height()
    # The upper part of the back wall, above the emitting panel in the middle.
    pixels = [image.pixelColor(x, y).green() for y in range(int(h * .29), int(h * .41), 3)
              for x in range(int(w * .43), int(w * .60), 3)]
    values.append(sum(pixels) / len(pixels))
    capture()

capture()
""")
    result = work / "brightness.json"
    env = dict(os.environ, XDG_CONFIG_HOME=str(work / "config"), AZTEK_PREFS=str(work / "prefs"),
               AZTEK_APP_DIR=str(work / "aztek"), AZTEK_EMISSION_RESULT=str(result))
    try:
        subprocess.run([str(app), "--script", str(probe), str(scene)], env=env, check=True, timeout=45)
    except subprocess.TimeoutExpired:
        if result.is_file():
            print(result.read_text(), file=sys.stderr)
        raise
    low_off, high_off, low_on, high_on = json.loads(result.read_text())
    assert abs(high_off - low_off) < 1, (low_off, high_off)
    assert high_on > low_on + 1, (low_on, high_on)
print("A stronger emitter brightens the wall with Bounce Light enabled.")
