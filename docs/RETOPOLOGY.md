# Retopologize

Mesh > Retopologize (`polyRetopo`) rebuilds a surface as clean quads: every face a quad, closed wherever the source
is closed, holes and thin parts kept, with long even loops and few irregular points. It has the reference's flags
and node attributes and no others, so its scripts and files work unchanged.

## Options

| Flag | What it does |
| --- | --- |
| `-targetFaceCount` (`tfc`) | About how many faces. 0 lets `-targetEdgeDeviation` set the density. |
| `-targetFaceCountTolerance` (`tft`) | Percent the count may miss by (10). |
| `-preserveHardEdges` (`phe`) | Edges follow the mesh's hard edges. |
| `-edgesByAngle` (`eba`), `-angle` (`ang`) | Edges follow edges whose faces bend more than the angle (30 degrees). Runs too short or too ragged to be a real edge are left out. |
| `-useFeatureTags` (`uft`), `-featureTags` (`ft`) | Edges follow the edges of the named component tags. |
| `-symmetry` (`sym`), `-axis` (`an`), `-axisPosition` (`ap`), `-axisOffset` (`ao`) | The result is exactly symmetric across the plane. |
| `-topologyRegularity` (`trg`) | 0 to 1 (0.5): higher for fewer irregular points and longer unbroken loops, lower to follow detail more closely. |
| `-faceUniformity` (`fun`) | 0 to 1 (0): from faces sized to the curvature to faces all one size. |
| `-anisotropy` (`a`) | 0 to 1 (0.5): how far quads stretch along the direction the surface bends least. |
| `-targetEdgeDeviation` (`ted`) | With no face count: how far edges may stray from the surface, against its local radius. |
| `-preprocessMesh` (`pm`), `-shortEdgeTolerance` (`se`), `-thinTriangleAngleTolerance` (`tt`) | Clean the input first: edges shorter than the tolerance (against the mesh's size) collapsed, triangles with a smaller angle losing their shortest edge, and a far denser mesh reduced. |

`symmetryEdges`, `curveInfluenceDirection`, `curveSingularitySeparation`, `interactiveMode` and the pivot are
accepted and stored.

A selection of faces is retopologized as a piece of its own in their place; its border is kept where it is.

## How it works

The code is in `crates/aztek_geo/src/retopo/`. The idea: lay out a coarse mesh of large quads whose topology is
right, then fill each coarse quad with a regular grid. Inside a grid every point has four neighbours, so irregular
points only sit at the coarse layout's corners, and neighbouring grids share the points along their common edge, so
the result is closed by construction.

1. **Input** (`tri.rs`). The faces are triangulated and the features collected: open borders always, hard edges,
   edges sharper than the angle and feature tags when asked, and a symmetry plane's cut. Angle features that are
   short, or that turn back and forth at every point (following the triangulation rather than a real edge), are
   dropped.
2. **Even triangles** (`trimesh.rs`). The surface is remeshed into even triangles (split, collapse, flip, slide;
   Botsch and Kobbelt 2004), finer where it bends tightly. Feature points slide only along their curves; points where
   a curve turns sharply, or where curves meet, stay put. Every collapse keeps the mesh manifold and keeps the source
   points it covers near its triangles, so thin parts such as ears and horns stay.
3. **All quads** (`quad.rs`). Each triangle splits into three quads.
4. **Coarse layout** (`quad.rs`). The quad mesh is simplified (after Tarini et al. 2010): quads collapse across their
   shorter diagonal, smallest against the wanted size first; points with two faces are removed; edges turn when that
   brings points nearer four neighbours; whole rows of narrow quads are removed, which is what lets the points along
   a feature curve thin out; and points slide along the surface between. Every change is checked before it is made:
   each point must keep a single fan of faces, no face may turn over, a collapse may not leave a point with more
   than five edges, and the source points a quad covers must stay near it. A change that fails any of these is not
   made, so the layout is always a closed, manifold all-quad mesh of the same shape and holes as the source.
5. **Rows and grids** (`quad.rs`). The layout's rows of quads (chords) each get a whole number of new rows, from the
   wanted edge length along them: smaller where the surface bends tightly (by `-faceUniformity`) and longer along the
   direction it bends least (by `-anisotropy`). The numbers are scaled together and then adjusted row by row to meet
   the count. When a layout's rows wind through all of it, every quad gets the same grid and only some counts can be
   met; layouts kept at several stages of the simplification are tried, the coarsest that meets the count within
   the tolerance used. Every coarse quad is filled with its grid (a Coons patch on its edges' points).
6. **Relaxing**. The points go onto the surface along their normals, then slide toward their neighbours' middle and
   back onto the surface; feature points along their curves; corners stay. Corners that turned over are untangled.
7. **Symmetry**. Half the surface, cut at the plane, is done as above with the cut as a border. No edge may join two
   points on the plane except along the cut. The half is then mirrored and joined along the cut's points only, so the
   join can't give any edge four faces.

## Measured

Standard test models (from [common-3d-test-models](https://github.com/alecjacobson/common-3d-test-models)), the
first Retopologize ("old": an isotropic remesh with triangle pairs joined) against this one at the default options
(and with `-edgesByAngle` on the fandisk). Irregular: the share of inner points not of valence 4. Scaled Jacobian:
the mean over quad corners, 1 for a square. Back error: the mean distance from the source's points to the result,
against the model's size, which shows lost parts and holes. Open borders: edges with one face, against the source's.

| Model, target | Version | Faces | Quads | Irregular | Scaled Jacobian | Back error | Open borders |
| --- | --- | --- | --- | --- | --- | --- | --- |
| spot, 2000 | old | 1954 | 75.5% | 37.3% | 0.913 | 0.169% | 0 of 0 |
| | new | 1854 | 100% | 2.9% | 0.976 | 0.170% | 0 of 0 |
| fandisk, 2000 | old | 1855 | 77.3% | 37.2% | 0.904 | 0.164% | 0 of 0 |
| | new, edges by angle | 1988 | 100% | 3.7% | 0.895 | 0.036% | 0 of 0 |
| rocker arm, 2000 | old | 1887 | 77.3% | 36.2% | 0.905 | 0.239% | 0 of 0 |
| | new | 1980 | 100% | 4.5% | 0.973 | 0.225% | 0 of 0 |
| bunny (open base), 3000 | old | 2976 | 73.7% | 38.6% | 0.907 | 0.116% | 223 of 223 |
| | new | 2992 | 100% | 2.7% | 0.947 | 0.097% | 98 of 223 (the same holes, fewer edges round them) |
| armadillo, 4000 | old | 3617 | 75.5% | 39.2% | 0.904 | 0.169% | 0 of 0 |
| | new | 3963 | 100% | 5.0% | 0.955 | 0.125% | 0 of 0 |

A box at 300 faces with edges by angle comes out as an even grid on each side, its only irregular points the eight
corners. Spot, fandisk, rocker arm and armadillo at 200, 800 and 4000 faces, each with the defaults, edges by angle,
symmetry, full uniformity with full regularity, and no regularity with no anisotropy: every result closed.

The tests in `crates/aztek_geo/tests/retopo.rs` hold the rest: a sphere and a thin torus come out closed and
manifold, all quads, with the right Euler characteristic and every point on the surface; edges by angle put a point
on every corner of a box and every face flat on one side; symmetry mirrors every point and stays closed; the same
input gives the same result; a selection keeps its border where it was.

## Sources

- Botsch, Kobbelt: A remeshing approach to multiresolution modeling, SGP 2004.
- Tarini, Pietroni, Cignoni, Panozzo, Puppo: Practical quad mesh simplification, Eurographics 2010.
- Daniels, Silva, Shepherd, Cohen: Quadrilateral mesh simplification, SIGGRAPH Asia 2008 (removing rows of quads).
- Bommes, Lévy, Pietroni, Puppo, Silva, Tarini, Zorin: Quad-mesh generation and processing: a survey, 2013.
- The reference's [polyRetopo node](https://help.autodesk.com/cloudhelp/2027/ENU/Maya-Tech-Docs/Nodes/polyRetopo.html) for its attributes.
