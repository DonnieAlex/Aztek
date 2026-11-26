"""A moving check of the realtime viewport: one session, every feature on, four camera passes.

Run it with the app:

    MOTION_OUT=/tmp/aztek_motion build/Aztek/AppRun --script tools/render_parity/motion_capture.py \
        examples/rendering/reflections.ma

It turns on Realtime's lights, shadows, ambient occlusion, bounce light, textures and depth of field, then writes:

  orbit.####.png   a full 360-degree turntable at constant speed
  dolly.####.png   a dolly from far away to close and back, with the focus tracking the subject
  sweep.####.png   a low sweep from the lit side to the dark side, so the terminator crosses the spheres
  still.####.png   twelve frames of an unmoving camera; any difference between them is temporal instability

All but the last use keyframed cameras and the app's own playblast, so the motion is smooth and one launch serves the
whole pass. The frames are the viewport size at 100 percent. Hide the grid and the ornaments: they are guides, and at
grazing angles the grid's smooth lines can read over a sphere, which is a display layer and not the shaded picture.
"""
import os
import maya.cmds as cmds

OUT = os.environ.get("MOTION_OUT", "/tmp/aztek_motion")
os.makedirs(OUT, exist_ok=True)

panel = cmds.playblast(q=True, activeEditor=True)
cmds.modelEditor(panel, edit=True, displayLights="all", shadows=True, ambientOcclusion=True,
                 bounceLight=True, depthOfField=True, displayTextures=True, grid=False)
state = {
    "displayLights": cmds.modelEditor(panel, q=True, displayLights=True),
    "shadows": cmds.modelEditor(panel, q=True, shadows=True),
    "ambientOcclusion": cmds.modelEditor(panel, q=True, ambientOcclusion=True),
    "bounceLight": cmds.modelEditor(panel, q=True, bounceLight=True),
    "depthOfField": cmds.modelEditor(panel, q=True, depthOfField=True),
    "displayTextures": cmds.modelEditor(panel, q=True, displayTextures=True),
    "grid": cmds.modelEditor(panel, q=True, grid=True),
}
print("MOTION FEATURES", state)


def make(name, pos, focus):
    made = cmds.camera(n=name, focalLength=42)
    tr = made[0] if isinstance(made, (list, tuple)) else made
    sh = made[1] if isinstance(made, (list, tuple)) and len(made) > 1 else tr + "Shape"
    pivot = cmds.group(em=True, n=name + "Pivot")
    cmds.move(0, 1.5, 0, pivot)
    cmds.parent(tr, pivot, r=True)
    cmds.move(pos[0], pos[1], pos[2], tr, os=True, r=True)
    cmds.aimConstraint(pivot, tr, aim=(0, 0, -1), u=(0, 1, 0))
    cmds.setAttr(sh + ".depthOfField", 1)
    cmds.setAttr(sh + ".fStop", 1.4)
    cmds.setAttr(sh + ".focusDistance", focus)
    cmds.lookThru(tr)
    return tr, sh, pivot


def play(stem, last):
    cmds.playbackOptions(min=1, max=last)
    path = os.path.join(OUT, stem)
    cmds.playblast(st=1, et=last, f=path, fmt="image", c="png", v=False, fo=True, p=100, showOrnaments=False)
    print("MOTION WRITTEN", path)


# (a) Full 360 orbit at constant speed.
N = 90
tr, sh, pivot = make("orbitCam", (0, 2.2, 12), 12.0)
cmds.setKeyframe(pivot, at="ry", t=1, v=0)
cmds.setKeyframe(pivot, at="ry", t=N + 1, v=360)
cmds.keyTangent(pivot, at="ry", itt="linear", ott="linear")
play("orbit", N)

# (b) Dolly from very distant to close and back, focus tracking the subject.
N = 90
tr, sh, pivot = make("dollyCam", (0, 2.2, 60), 60.0)
cmds.setKeyframe(tr, at="tz", t=1, v=60)
cmds.setKeyframe(tr, at="tz", t=45, v=8)
cmds.setKeyframe(tr, at="tz", t=N + 1, v=60)
cmds.keyTangent(tr, at="tz", itt="linear", ott="linear")
cmds.setKeyframe(sh, at="focusDistance", t=1, v=60)
cmds.setKeyframe(sh, at="focusDistance", t=45, v=8.3)
cmds.setKeyframe(sh, at="focusDistance", t=N + 1, v=60)
cmds.keyTangent(sh, at="focusDistance", itt="linear", ott="linear")
play("dolly", N)

# (c) Low sweep from the lit side to the dark side, so the terminator crosses the spheres.
N = 90
tr, sh, pivot = make("sweepCam", (0, 0.1, 11), 11.0)
cmds.setKeyframe(pivot, at="ry", t=1, v=0)
cmds.setKeyframe(pivot, at="ry", t=N + 1, v=180)
cmds.keyTangent(pivot, at="ry", itt="linear", ott="linear")
play("sweep", N)

# (d) A still camera for twelve frames: any change here is temporal instability, not motion.
tr, sh, pivot = make("stillCam", (0, 2.2, 12), 12.0)
last = 12
cmds.playbackOptions(min=1, max=last)
cmds.playblast(st=1, et=last, f=os.path.join(OUT, "still"), fmt="image", c="png", v=False, fo=True, p=100, showOrnaments=False)
print("MOTION WRITTEN", os.path.join(OUT, "still"))

print("MOTION DONE")
