# Retopologize

Mesh > Retopologize (`polyRetopo`) rebuilds a surface as quads laid along a smooth field, with four methods for
different kinds of model. It takes the reference's flags and node attributes, so its scripts and files work, and adds
a few of its own.

## Methods

| Method (`-method`) | For | What it does |
| --- | --- | --- |
| 0 Organic | Sculpts, scans, smooth shapes | Smooth flow, light pull toward the curvature directions, density following curvature. |
| 1 Hard Surface | Machined and modelled parts | Follows hard edges and edges sharper than `-angle` exactly, a point on every feature corner, even density, flat regions found by shape segmentation steering the flow. |
| 2 Character | Creatures, people, heads | Loops along the curvature, denser where the shape is detailed; with `-symmetry` exactly mirrored. |
| 3 Segmented | Models whose parts differ in colour or shape | Splits the surface by the colour set's colours (followed exactly) and by shape (flow along the borders), and follows UV seams and material borders. |

`-segmentByShape`, `-segmentByColor`, `-followSeams` (each -1 for the method's choice, 0 off, 1 on) and `-pureQuads`
(-1 or 1 all quads, 0 quads with some triangles and pentagons) change what a method does. The reference's
`-targetFaceCount`, `-targetFaceCountTolerance`, `-preserveHardEdges`, `-edgesByAngle`, `-angle`, `-faceUniformity`,
`-anisotropy`, `-topologyRegularity`, `-targetEdgeDeviation` (with no face count), `-symmetry` with `-axis`,
`-axisPosition` and `-axisOffset`, and `-preprocessMesh` work as their names say. `featureTags`, `useFeatureTags`,
`symmetryEdges`, `curveInfluenceDirection`, `curveSingularitySeparation`, `shortEdgeTolerance`,
`thinTriangleAngleTolerance` and `interactiveMode` are accepted and stored.

A selection of faces is retopologized as a piece of its own in their place; its border is not joined to the rest.

## How it works

The code is in `crates/aztek_geo/src/retopo/`.

1. **Input.** The faces are triangulated; a mesh far denser than needed is reduced first; edges longer than a third of
   a lattice cell are split. Features are collected: open borders, hard edges, edges sharper than the angle, UV seams,
   material borders, the borders of colour regions (k-means in Lab, then connected parts), and the borders of shape
   regions (Variational Shape Approximation with normal proxies). Shape-region borders are soft: the flow leans
   along them but nothing is pinned to them.
2. **Orientation field.** A 4-way direction field, smoothed extrinsically by Gauss-Seidel over a hierarchy of ever
   coarser point sets, coarse to fine (Instant Field-Aligned Meshes). Feature points hold the feature's direction;
   where the surface bends clearly more one way than the other across a cell, the principal curvature direction pulls
   lightly, on the coarse levels only, so it sets the layout without each point's noise making singularities.
3. **Position field.** A 4-way lattice of the wanted edge length, smoothed the same way. On a feature a lattice line is
   held through it; at a feature corner a lattice point is. The edge length can follow curvature (`faceUniformity`
   below 1), smoothed and rescaled so the count holds.
4. **Extraction.** Points whose lattice points coincide merge into one vertex, closest pairs first and only while the
   clusters agree; points one lattice step apart give an edge; the edges' cycles about the surface are the faces.
   Non-manifold faces go, triangle pairs and triangle-pentagon pairs become quads, holes are filled, every face is
   split into quads, the points are relaxed and put back on the surface, feature points onto their features and each
   feature corner onto its nearest point, folded corners are untangled, and crumbs cut off from the rest are dropped.
   When the count misses by more than the tolerance, the edge length is corrected and the solve runs again.
5. **Symmetry.** The half on the positive side of the plane is retopologized with the cut as a border, its border
   points put on the plane, then mirrored and welded.

## Measured

Five standard test models (from [common-3d-test-models](https://github.com/alecjacobson/common-3d-test-models)), the
previous Retopologize ("old": an isotropic remesh with triangle pairs joined) against the new methods. Irregular: the
share of inner points not of valence 4. Scaled Jacobian: the mean over quad corners, 1 for a square. Back error: the
mean distance from the source's points to the result, against the model's size, which shows lost features and holes.

| Model, target | Method | Quads | Irregular | Scaled Jacobian | Back error |
| --- | --- | --- | --- | --- | --- |
| spot, 2000 | old | 75.5% | 37.3% | 0.913 | 0.18% |
| | Organic | 100% | 8.9% | 0.978 | 0.165% |
| fandisk, 2000 | old | 77.3% | 37.2% | 0.904 | 0.164% |
| | Hard Surface | 100% | 10.1% | 0.964 | 0.026% |
| rocker arm, 2000 | old | 77.3% | 36.2% | 0.905 | 0.239% |
| | Organic | 100% | 8.5% | 0.978 | 0.254% |
| | Hard Surface | 100% | 14.5% | 0.892 | 0.214% |
| bunny, 3000 | old | 73.7% | 38.6% | 0.907 | 0.116% |
| | Organic | 100% | 8.9% | 0.952 | 0.112% |
| | Character | 100% | 10.8% | 0.950 | 0.098% |
| armadillo, 4000 | old | 75.5% | 39.2% | 0.904 | 0.169% |
| | Organic | 100% | 12.8% | 0.954 | 0.174% |

The tests in `crates/aztek_geo/tests/retopo.rs` hold the rest: a sphere comes out closed, all quads, within a quarter of
the count and on the surface; Hard Surface puts a point on every corner of a box and every face flat on one side;
symmetry mirrors every point; the same input gives the same result; colour borders get a loop.

## Where it could go next

Measured against the state of the art in 2026, the field and extraction are the Instant Meshes family. What would
lower the irregular points further:

- Quantizing the layout with a flow (QuadriFlow) or Bi-MDF on a T-mesh (QuadWild with Bi-MDF), which removes the
  position-field singularities that make the remaining triangles and pentagons.
- Integer-grid maps (MIQ) with QEx extraction, for the cleanest layouts at a higher cost.
- Learned cross fields (NeurCross, 2025; CrossLift, 2026, which lifts quad wireframes drawn by an image model onto the
  surface), and autoregressive quad mesh generators (QuadGPT, ICLR 2026; QuadLink; Q-MAGIC), which need trained
  models Aztek doesn't ship.

## Sources

- Jakob, Tarini, Panozzo, Sorkine-Hornung: Instant Field-Aligned Meshes, SIGGRAPH Asia 2015.
- Huang, Zhou, Tao, Guibas, Nießner: QuadriFlow, SGP 2018.
- Pietroni, Nuvoli, Alderighi, Cignoni, Tarini: [Reliable feature-line driven quad-remeshing](https://www.researchgate.net/publication/355019897_Reliable_feature-line_driven_quad-remeshing), SIGGRAPH 2021 (QuadWild).
- Cohen-Steiner, Alliez, Desbrun: Variational Shape Approximation, SIGGRAPH 2004.
- Dong et al.: [NeurCross: A Neural Approach to Computing Cross Fields for Quad Mesh Generation](https://arxiv.org/abs/2405.13745), ACM TOG 2025.
- [Look Both Ways Before You Cross: Lifting Cross Fields From 2D Visual Priors](https://arxiv.org/html/2605.26062v1), 2026.
- [QuadGPT: Native Quadrilateral Mesh Generation with Autoregressive Models](https://arxiv.org/abs/2509.21420), ICLR 2026.
- [QuadLink: Autoregressive Quad-Dominant Mesh Generation via Point-Relation Learning](https://arxiv.org/pdf/2605.16813), 2026.
- [Q-MAGIC: Quad-only Mesh Autoregressive Generation](https://arxiv.org/pdf/2610.19777), 2026.
- The reference's [polyRetopo node](https://help.autodesk.com/cloudhelp/2027/ENU/Maya-Tech-Docs/Nodes/polyRetopo.html) for its attributes.
