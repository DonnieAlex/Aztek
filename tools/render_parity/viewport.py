"""Sets the view the parity capture takes: through the scene's shotCam, with the scene's lights and shadows.
Run by tools/render_parity/capture.sh with --viewport-screenshot, which takes the picture 1500 ms after start-up.
"""
import maya.cmds as mc

mc.lookThru("shotCam")
panel = mc.playblast(q=True, activeEditor=True)
mc.modelEditor(panel, edit=True, displayLights="all", shadows=True)
