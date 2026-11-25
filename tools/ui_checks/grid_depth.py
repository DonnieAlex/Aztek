#!/usr/bin/env python3
"""Check perspective grid depth under Xvfb. Usage: grid_depth.py build/Aztek/AppRun."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile


app = Path(sys.argv[1]).resolve()
with tempfile.TemporaryDirectory(prefix="aztek_grid_") as directory:
    work = Path(directory)
    script = work / "slab.py"
    script.write_text("""
import maya.cmds as cmds

cmds.polyCube(width=6, height=6, depth=0.2)
cmds.select(clear=True)
cmds.setAttr("persp.translate", 0, 5, 13, type="double3")
cmds.setAttr("persp.rotate", -20, 0, 0, type="double3")
for name in ("gridAxis", "gridHighlight", "gridColor"):
    cmds.displayRGBColor(name, 1, 0, 1)
""")
    shot = work / "grid.png"
    env = dict(os.environ, XDG_CONFIG_HOME=str(work / "config"), AZTEK_PREFS=str(work / "prefs"),
               AZTEK_APP_DIR=str(work / "aztek"))
    subprocess.run([str(app), "--script", str(script), "--screenshot", str(shot)],
                   env=env, check=True, timeout=30)
    width, height = map(int, subprocess.check_output(
        ["magick", "identify", "-format", "%w %h", str(shot)], text=True).split())
    pixels = subprocess.check_output(["magick", str(shot), "-depth", "8", "rgb:-"])

    # Sample well inside the slab, above and below the grid's intersection.
    def grid_pixels(top, bottom):
        count = 0
        for y in range(int(height * top), int(height * bottom)):
            for x in range(int(width * .45), int(width * .56)):
                i = 3 * (y * width + x)
                red, green, blue = pixels[i:i + 3]
                count += red > green + 30 and blue > green + 30
        return count

    assert grid_pixels(.42, .48) == 0, "Grid lines showed through the slab above the intersection."
    assert grid_pixels(.56, .68) > 100, "The slab hid grid lines in front of it."
print("Perspective grid is visible in front of the slab and hidden behind it.")
