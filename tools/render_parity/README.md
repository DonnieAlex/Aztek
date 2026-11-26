# Lighting parity suite

Eight scenes in `examples/rendering/` compare the realtime viewport with the Render View's path (Harold):

| Scene | What it shows |
| --- | --- |
| `reflections` | The issue's scene: a grey floor and a red and a white sphere lit by one light. Both spheres reflect in the floor. |
| `metal_roughness_sweep` | Seven gold spheres, roughness 0.02 to 0.85, under an HDRI sky: sharp reflections at low roughness, blurred at high, the sky in both. |
| `dielectric_glass` | Three glass spheres at IOR 1.33, 1.5 and 1.7 over a wall and two coloured blocks. |
| `tinted_glass` | Four transmission colours over a light wall; the glass is tinted and what is behind it bends. |
| `emissive_object` | An emissive panel lighting a wall and a sphere. |
| `area_lights` | Two area lights on red and blue spheres. |
| `environment_hdri` | Metal spheres and a red ball under `examples/textures/sky.hdr`. |
| `checker_sphere` | A checker texture on a sphere, with two plain spheres beside it. |

Each scene has a `shotCam`; the reference render and the viewport capture both look through it. The pictures are kept
as `reference/<scene>.png` (640 by 360, the Render View) and `viewport/<scene>.png` (the realtime viewport, cropped to
the same 16:9 field and resized to 640 by 360).

## Taking the pictures again

From the repository root, with a build and a display (Xvfb is enough):

```
tools/render_parity/capture.sh build/Aztek/AppRun
```

Name scenes to redo only those:

```
tools/render_parity/capture.sh build/Aztek/AppRun reflections dielectric_glass
```

The reference render is Harold's batch path through `shotCam`:

```
build/Aztek/AppRun -render -s 1 -cam shotCam -x 640 -y 360 -rd examples/rendering/reference -im reflections -of png examples/rendering/reflections.ma
```

The viewport capture is `--viewport-screenshot`: the window is fixed, the view is set to a single panel, the ViewCube,
grid, light and camera icons, handles and selection highlights are hidden, textures and the camera's depth of field
are on, and the viewport looks through `shotCam` with the scene's lights and shadows. The host writes a PPM, the shell
saves the PNG. `tools/render_parity/viewport.py` sets the camera and the lighting before the capture's 1500 ms timer.

At another viewport size the crop is taken from the picture's own width, so the field stays the same: the camera's
horizontal field is fixed by its focal length, and cropping to 16:9 removes only the extra height a taller panel adds.

## Comparing

The pictures are the same size, so a difference is the two renderers, not the crop:

```
magick compare -metric RMSE examples/rendering/reference/reflections.png examples/rendering/viewport/reflections.png null:
magick examples/rendering/reference/reflections.png examples/rendering/viewport/reflections.png +append side-by-side.png
```

The scenes are built by `examples/tools/make_rendering_parity.py`, run the same way:

```
build/Aztek/AppRun --batch --script examples/tools/make_rendering_parity.py
```

## What the viewport does

- Screen-space reflections march the view's own colour and depth for objects; the sky dome stands in where the view
  shows nothing to reflect.
- The widest flat mesh at the ground plane is drawn a second time mirrored below it; the floor reads that image, so
  it reflects the underside of what stands on it, which a screen-space march cannot see.
- Metals and dielectrics blur their reflection with their roughness.
- Glass is drawn solid and taken apart in the post pass: the scene is drawn once more without its transmissive
  surfaces, and each glass pixel reads that image bent out towards the rim and tinted by its transmission colour.

## Known gaps

- A reflection of something the view does not show anywhere falls back to the sky; the Render View sees it.
- The reflection image is low-passed and copied down a mip chain; the gather samples a level from the roughness, so a
  speckle is spread rather than shown. The planar and screen-space tiers share one brightness and the same filtered
  copy, and their weights sum to one.
- The viewport's planar reflection of a standing sphere is smaller and sharper than the Render View's soft full one.
- Only the widest flat mesh at the world's ground is mirrored. Sloped or raised reflecting planes use the
  screen-space march.
- Aztek's standard surface has no transmission depth. The distance the refraction lookup travels stands in for it, and
  the glass is bent as a lens rather than traced through its back surface.
- The viewport does not draw the sky dome as a background; it lights and reflects from it, so the HDRI scene's sky is
  black behind the objects where the Render View shows it.
- Harold's reflection off one large single-face floor reflects the floor itself; the scenes give their floors
  subdivisions so the reference shows the reflection it should.
- At the same focal length the viewport frames about 12% tighter than the Render View, seen when the two pictures are
  overlaid.
