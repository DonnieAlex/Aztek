"""Builds the lighting parity scenes in examples/rendering/.
Run from the repository root:
  ./build/Aztek/AppRun --batch --script examples/tools/make_rendering_parity.py
Each scene is opened by Aztek, saved by Aztek, and rendered through its shotCam by the Render View's path.
"""
import math
import os
import traceback

import maya.cmds as mc

ROOT = os.environ.get("AZTEK_EXAMPLES", os.path.join(os.getcwd(), "examples"))
OUT = os.path.join(ROOT, "rendering")
LOG = []


def scene(name):
    def wrap(fn):
        try:
            mc.file(new=True, force=True)
            mc.currentUnit(l="cm", a="deg", t="film")
            fn()
            mc.select(clear=True)
            mc.currentTime(mc.playbackOptions(q=True, min=True))
            mc.file(rename=os.path.join(OUT, name + ".ma"))
            mc.file(save=True, type="mayaAscii", force=True)
            LOG.append("ok   " + name)
        except Exception:
            LOG.append("FAIL " + name + "\n" + traceback.format_exc())
        return fn
    return wrap


def material(name, kind="standardSurface", color=(0.5, 0.5, 0.5), **attrs):
    s = mc.shadingNode(kind, asShader=True, n=name)
    attr = "baseColor" if kind in ("standardSurface", "openPBRSurface") else "color"
    mc.setAttr(s + "." + attr, *color, type="double3")
    for a, v in attrs.items():
        if isinstance(v, (tuple, list)):
            mc.setAttr(s + "." + a, *v, type="double3")
        else:
            mc.setAttr(s + "." + a, v)
    sg = mc.sets(renderable=True, noSurfaceShader=True, empty=True, n=name + "SG")
    mc.connectAttr(s + ".outColor", sg + ".surfaceShader")
    return s, sg


def assign(obj, sg):
    mc.sets(obj, e=True, forceElement=sg)


def floor(size=30, name="floor", color=(0.42, 0.44, 0.47), rough=0.2, metal=0.0):
    f = mc.polyCube(n=name, w=size, h=0.02, d=size, sx=20, sy=1, sz=20)[0]
    mc.move(0, -0.01, 0, f)
    m, sg = material(name + "Mat", color=color, specularRoughness=rough, metalness=metal)
    assign(f, sg)
    return f


def sphere(name, pos, radius=1.2, color=(0.5, 0.5, 0.5), **attrs):
    s = mc.polySphere(n=name, r=radius, sx=48, sy=32)[0]
    mc.move(pos[0], pos[1], pos[2], s)
    m, sg = material(name + "Mat", color=color, **attrs)
    assign(s, sg)
    return s


def camera(pos=(0, 4.0, 12.0), rot=(-12, 0, 0), fl=50, coi=10.0):
    c = mc.camera(n="shotCam")[0]
    mc.setAttr(c + ".translateX", pos[0])
    mc.setAttr(c + ".translateY", pos[1])
    mc.setAttr(c + ".translateZ", pos[2])
    mc.setAttr(c + ".rotateX", rot[0])
    mc.setAttr(c + ".rotateY", rot[1])
    mc.setAttr(c + ".rotateZ", rot[2])
    mc.setAttr(c + ".focalLength", fl)
    mc.setAttr(c + ".centerOfInterest", coi)
    mc.setAttr("perspShape.focalLength", fl)
    mc.viewPlace("persp", eye=pos, la=(0, 1.2, 0))
    return c


def point(name, pos=(-4.5, 7.5, 4.5), intensity=40.0, decay=2):
    p = mc.pointLight(n=name, position=pos, intensity=intensity)
    mc.setAttr(p + ".decayRate", decay)
    mc.setAttr(p + ".useRayTraceShadows", 1)
    return p


def area(name, pos=(-3.0, 6.5, 4.0), intensity=200.0, size=5.0):
    p = mc.shadingNode("aiAreaLight", asLight=True, n=name)
    mc.setAttr(p + ".intensity", intensity)
    mc.setAttr(p + ".aiTranslator", 0)
    mc.setAttr(p + ".useRayTraceShadows", 1)
    mc.move(pos[0], pos[1], pos[2], p)
    mc.setAttr(p + ".scaleX", size * 0.5)
    mc.setAttr(p + ".scaleY", size * 0.5)
    mc.rotate(-45, 35, 0, p)
    return p


def sky(image=None, intensity=0.8, rotation=0.0):
    t = mc.createNode("transform", n="sky")
    s = mc.createNode("aiSkyDomeLight", n="skyShape", p=t)
    mc.setAttr(s + ".intensity", intensity)
    mc.setAttr(t + ".rotateY", rotation)
    if image:
        f = mc.shadingNode("file", asTexture=True, n="skyImage")
        mc.setAttr(f + ".fileTextureName", image, type="string")
        mc.connectAttr(f + ".outColor", s + ".cl")
    return s


# ===================================================================================================== the issue
@scene("reflections")
def _():
    floor(30, color=(0.55, 0.57, 0.60), rough=0.04, metal=0.75)
    sphere("redSphere", (-2.0, 1.5, 0.0), 1.5, (0.8, 0.03, 0.02), specularRoughness=0.10)
    sphere("whiteSphere", (2.4, 1.5, 0.6), 1.5, (0.8, 0.8, 0.78), specularRoughness=0.10)
    key = mc.directionalLight(n="keyLight", rotation=(-45, 35, 0), intensity=2.5)
    mc.setAttr(key + ".useRayTraceShadows", 1)
    camera((0, 3.7, 12.0), (-15, 0, 0), 42, 10.0)


# ===================================================================================================== metals
@scene("metal_roughness_sweep")
def _():
    floor(40, color=(0.20, 0.21, 0.23), rough=0.3, metal=0.6)
    roughs = [0.02, 0.08, 0.15, 0.25, 0.40, 0.60, 0.85]
    for i, r in enumerate(roughs):
        x = -6.0 + i * 2.0
        sphere("metal%d" % i, (x, 1.0, 0.0), 1.0, (0.95, 0.72, 0.35), specularRoughness=r, metalness=1.0)
    sphere("whiteBall", (0.0, 1.2, -3.0), 1.2, (0.9, 0.9, 0.9), specularRoughness=0.2)
    point("keyPoint", (-4.0, 7.0, 5.0), 60.0)
    sky("../textures/sky.hdr", 0.15, 45.0)
    camera((0, 5.0, 16.0), (-13, 0, 0), 50, 14.0)


# ===================================================================================================== glass
@scene("dielectric_glass")
def _():
    floor(30, color=(0.30, 0.32, 0.35), rough=0.15, metal=0.2)
    wall = mc.polyCube(n="wall", w=24, h=10, d=0.2)[0]
    mc.move(0, 5, -8, wall)
    wm, wsg = material("wallMat", color=(0.0, 0.0, 0.0), specularRoughness=0.5, emission=1.0, emissionColor=(0.55, 0.55, 0.55))
    assign(wall, wsg)
    left = mc.polyCube(n="leftBlock", w=2, h=4, d=2)[0]
    mc.move(-3.6, 2, -5, left)
    m, sg = material("leftMat", color=(0.95, 0.65, 0.08), specularRoughness=0.5, emission=0.35, emissionColor=(0.8, 0.5, 0.05))
    assign(left, sg)
    right = mc.polyCube(n="rightBlock", w=2, h=4, d=2)[0]
    mc.move(3.6, 2, -5, right)
    m, sg = material("rightMat", color=(0.08, 0.55, 0.95), specularRoughness=0.5, emission=0.35, emissionColor=(0.05, 0.4, 0.8))
    assign(right, sg)
    for i, ior in enumerate([1.33, 1.5, 1.7]):
        sphere("glass%d" % i, (-3.0 + i * 3.0, 1.5, 0.0), 1.4, (1.0, 1.0, 1.0),
               specularRoughness=0.02, transmission=1.0, transmissionColor=(0.85, 0.25, 0.2), specularIOR=ior)
    point("keyPoint", (-4.0, 7.0, 5.0), 12.0)
    camera((0, 3.6, 11.0), (-8, 0, 0), 50)


# ===================================================================================================== tinted glass
@scene("tinted_glass")
def _():
    floor(30, color=(0.30, 0.32, 0.35), rough=0.15, metal=0.2)
    wall = mc.polyCube(n="wall", w=24, h=10, d=0.2)[0]
    mc.move(0, 5, -8, wall)
    wm, wsg = material("wallMat", color=(0.0, 0.0, 0.0), specularRoughness=0.5, emission=1.0, emissionColor=(0.6, 0.6, 0.6))
    assign(wall, wsg)
    cols = [(0.85, 0.10, 0.08), (0.10, 0.70, 0.25), (0.10, 0.35, 0.85), (0.85, 0.65, 0.10)]
    for i, c in enumerate(cols):
        x = -4.5 + i * 3.0
        sphere("tint%d" % i, (x, 1.4, 0.0), 1.3, (1.0, 1.0, 1.0), specularRoughness=0.03,
               transmission=1.0, transmissionColor=c, specularIOR=1.5)
    point("keyPoint", (-4.0, 7.0, 5.0), 12.0)
    camera((0, 3.6, 13.0), (-8, 0, 0), 50)


# ===================================================================================================== emission
@scene("emissive_object")
def _():
    floor(30, color=(0.45, 0.45, 0.45), rough=0.3)
    wall = mc.polyCube(n="backWall", w=24, h=10, d=0.2)[0]
    mc.move(0, 5, -7, wall)
    m, sg = material("backMat", color=(0.5, 0.5, 0.5), specularRoughness=0.4)
    assign(wall, sg)
    sphere("litSphere", (0.0, 1.4, 0.0), 1.4, (0.8, 0.8, 0.8), specularRoughness=0.2)
    panel = mc.polyCube(n="emitter", w=5, h=3, d=0.2)[0]
    mc.move(-3.5, 3.0, 3.0, panel)
    em, esg = material("emitterMat", color=(0.0, 0.0, 0.0), specularRoughness=0.5, emission=8.0, emissionColor=(1.0, 0.35, 0.05))
    assign(panel, esg)
    point("fill", (4.0, 5.0, 4.0), 8.0)
    camera((0, 3.5, 11.0), (-8, 0, 0), 50)


# ===================================================================================================== area lights
@scene("area_lights")
def _():
    floor(30, color=(0.38, 0.39, 0.41), rough=0.2)
    sphere("redBall", (-2.2, 1.3, 0.0), 1.3, (0.8, 0.10, 0.08), specularRoughness=0.15)
    sphere("blueBall", (2.2, 1.3, 0.0), 1.3, (0.08, 0.25, 0.75), specularRoughness=0.15)
    area("keyArea", (-3.2, 5.5, 4.0), 250.0, 5.0)
    area("fillArea", (4.0, 4.5, 3.0), 90.0, 4.0)
    camera((0, 3.6, 11.0), (-8, 0, 0), 50)


# ===================================================================================================== environment
@scene("environment_hdri")
def _():
    floor(50, color=(0.22, 0.23, 0.25), rough=0.08, metal=0.7)
    sphere("goldBall", (-2.0, 1.5, 0.0), 1.5, (0.95, 0.72, 0.35), specularRoughness=0.05, metalness=1.0)
    sphere("chromeBall", (2.2, 1.5, 0.0), 1.5, (0.85, 0.85, 0.85), specularRoughness=0.05, metalness=1.0)
    sphere("redBall", (0.2, 1.0, -3.2), 1.0, (0.8, 0.03, 0.02), specularRoughness=0.15)
    sky("../textures/sky.hdr", 0.12, 30.0)
    camera((0, 4.0, 12.0), (-10, 0, 0), 50)


# ===================================================================================================== checker
@scene("checker_sphere")
def _():
    floor(30, color=(0.35, 0.36, 0.38), rough=0.2)
    ball = sphere("checkerBall", (0.0, 1.5, 0.0), 1.5, (1.0, 1.0, 1.0), specularRoughness=0.25)
    for i, x in enumerate([-3.4, 3.4]):
        sphere("plain%d" % i, (x, 1.2, -1.0), 1.2, (0.8, 0.8, 0.8), specularRoughness=0.2)
    s = mc.shadingNode("lambert", asShader=True, n="checkerMat")
    f = mc.shadingNode("file", asTexture=True, n="checkerFile")
    p = mc.shadingNode("place2dTexture", asUtility=True, n="checkerPlace")
    mc.connectAttr(p + ".outUV", f + ".uvCoord")
    mc.connectAttr(p + ".outUvFilterSize", f + ".uvFilterSize")
    mc.setAttr(p + ".repeatU", 6)
    mc.setAttr(p + ".repeatV", 3)
    mc.setAttr(f + ".fileTextureName", "../textures/checker.png", type="string")
    mc.connectAttr(f + ".outColor", s + ".color")
    sg = mc.sets(renderable=True, noSurfaceShader=True, empty=True, n="checkerMatSG")
    mc.connectAttr(s + ".outColor", sg + ".surfaceShader")
    mc.sets(ball, e=True, forceElement=sg)
    point("keyPoint", (-4.5, 7.0, 4.5), 45.0)
    camera((0, 3.5, 12.0), (-10, 0, 0), 50)


print("\n".join(LOG))
