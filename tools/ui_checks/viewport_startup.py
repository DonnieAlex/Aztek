#!/usr/bin/env python3
"""Check that viewport startup returns to the event loop. Usage: viewport_startup.py /path/to/aztek"""
import os
from pathlib import Path
import subprocess
import sys
import tempfile


app = str(Path(sys.argv[1]).resolve())
with tempfile.TemporaryDirectory(prefix="aztek_viewport_") as directory:
    work = Path(directory)
    for name in ("home", "maya"):
        (work / name).mkdir()
    script = work / "scene.mel"
    script.write_text("polyCube; viewFit -all;\n")
    screenshot = work / "viewport.png"
    env = dict(os.environ, AZTEK_PREFS=str(work / "prefs"), AZTEK_APP_DIR=str(work / "aztek"),
               MAYA_APP_DIR=str(work / "maya"), XDG_CONFIG_HOME=str(work / "config"),
               CFFIXED_USER_HOME=str(work / "home"))
    # Both startup scripts and screenshots use timers. A completed frame alone does not prove the UI responds.
    try:
        subprocess.run([app, "--script", str(script), "--screenshot", str(screenshot)], env=env,
                       stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=True, timeout=30)
    except subprocess.TimeoutExpired:
        sys.exit("Viewport startup starved the event loop: screenshot did not finish within 30 seconds.")
    assert screenshot.is_file(), "No screenshot was saved."
    assert screenshot.read_bytes().startswith(b"\x89PNG\r\n\x1a\n"), "The screenshot is not a PNG."
print("Viewport startup: script, screenshot and clean exit passed.")
