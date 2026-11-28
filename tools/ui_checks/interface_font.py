#!/usr/bin/env python3
"""Check viewport menus and shelf tabs at two interface font sizes (#323)."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

with tempfile.TemporaryDirectory(prefix="aztek_font_") as directory:
    work = Path(directory)
    config = work / "config/aztek/Aztek.conf"
    config.parent.mkdir(parents=True)
    result = work / "result.json"
    probe = work / "probe.py"
    probe.write_text("""
import json
import os
from pathlib import Path
from PySide6.QtWidgets import QApplication, QMenuBar, QTabWidget

app = QApplication.instance()
window = next(w for w in app.topLevelWidgets() if w.objectName() == "MayaWindow")
panel = window.findChild(QMenuBar, "panelMenus")
tabs = window.findChild(QTabWidget, "shelfTabs").tabBar()
Path(os.environ["AZTEK_FONT_RESULT"]).write_text(json.dumps([
    window.menuBar().font().pointSize(), panel.font().pointSize(), tabs.tabRect(0).width(),
]))
app.quit()
""")
    env = dict(os.environ, XDG_CONFIG_HOME=str(work / "config"), XDG_DATA_HOME=str(work / "data"),
               AZTEK_PREFS=str(work / "prefs"), AZTEK_APP_DIR=str(work / "aztek"), AZTEK_FONT_RESULT=str(result))
    sizes = []
    for size in (9, 14):
        config.write_text(f"[prefs]\nfontSize={size}\n")
        result.unlink(missing_ok=True)
        subprocess.run([str(Path(sys.argv[1]).resolve()), "--script", str(probe)], env=env, check=True, timeout=30)
        sizes.append(json.loads(result.read_text()))
    assert [size[:2] for size in sizes] == [[9, 9], [14, 14]], sizes
    assert sizes[1][2] > sizes[0][2], sizes
    print("Viewport menus and shelf tabs follow the interface font size.")
