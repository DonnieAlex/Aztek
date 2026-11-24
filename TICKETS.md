# Tickets

Requests waiting or in progress, newest detail kept. Move to Done with the date when reported finished.

## In progress
- Sound output (srlegrand's call): which audio library Aztek plays sound through, for playback, scrubbing and sync.
- XJEN (top priority, srlegrand 16/11/2026): hair and fur at XGen parity. Target and order in docs/XJEN.md; ground truth
  from `tools/reference_capture/xgen_capture.py`, to be run by srlegrand. Steps 1 (strands), 2 (Interactive Groom
  core), 3 (grooming tools), 4 (modifiers), 5 (editor, shelf,
  menu) and 6 (Alembic, caches) in, 16/11/2026.
- Deformation speed: viewport staleness fixed; root control on Lou still about 115 ms a change; smooth preview subdivides from scratch each deformation.

- Unified dynamics: in (see docs/DYNAMICS.md). Left: speed at 20,000 vertices with self collision (contact search
  dominates), cloth springy under heavy rigid bodies, per particle expressions not run, dynamic constraints from files
  not rebuilt, rigid bodies follow their object by name, `.mcx` caches not read.
- C++ plug-in kit (agent).

## Waiting
- Attribute Editor (24/11/2026): the chrome and the transform, mesh, joint, skinCluster and blendShape layouts are in;
  controls the reference has and Aztek has no attribute for are greyed and empty (world pivots, Show Manip Default,
  Manip History Node, mesh component display detail, DoF scale, GPU options, a camera's Angle of View and millimetre
  aperture). skinCluster's input table and Manage
  Deformed Geometry, blendShape's in-betweens and target management, Load Selected Attributes, the Show filters and a
  tab's right-click menu are not built; a material's Type list and a light's Type list are present (a light's changes
  the shape), samples are drawn rather than rendered, and a light's disk-based depth map names and fog shadows are
  greyed. Translate and Scale limits are not attributes on a transform; Attribute
  Names' Short mode shows the long names, as Aztek keeps no short ones.
- Found 16/11/2026: `namespaceInfo -lon` at the root lists only the scene's namespaces; the reference also lists UI and
  shared, which scripts skip by name. Not changed: `core_forms.rs` and `core_cmds.rs` hold the current answer.
- Left from the issues closed 15/11 and 16/11/2026 (PARITY.md, Issues 37 to 83): Paint Skin Weights' ramp editing
  (and its colours checked against the reference's) and stylus pressure; Bind Skin's Colorize skeleton; the ViewCube's
  Animated transitions; the Outliner's Scene Assembly, Asset, Render Setup and Create Partition, greyed until those
  exist.
- #76 (open): install folder on PATH and a `bin` folder on Windows. The reference leaves PATH alone; srlegrand to decide on
  the `bin` folder.
- CI: `python_api_px` fails on rocky9 and ubuntu-2204 (sum 0.0 after reopening a plug-in's data); diagnostics are in,
  the "after reopening" output from the next failed run is wanted.
- Found making the examples (09/11/2026), still so on 16/11/2026: `skinCluster` and `blendShape` don't take NURBS
  curves (`cluster` does since #66); `nurbsSquare -sl2` and `vortex -ax/-ay/-az` are missing; `connectDynamic -f` with
  a field's transform fails; `scriptJob -listJobs` lists ids, not the Python function. (`displaySmoothness -po` works
  since PLAN 1.12.)
- Real plug cycles: `a.tx -> b.tx -> a.tx` is refused. The reference connects it and warns at evaluation; the evaluator needs a guard first.
- One shape node under several parents, in fact: each instance's parent holds its own node, kept the same by `setAttr` and `rename` (PLAN 1.14). A script sees one shape; the Python API's `instanceCount`, a direct graph write and non-mesh instances don't yet. A real multi-parent graph means every DAG walk and per-instance world matrices, picking and selection paths: a project of its own, srlegrand's call.
- A script's `scriptJob -idleEvent` moves the core's version on each idle, so the panels rebuild and a Channel Box field being typed in loses the keyboard. Found writing the multi-edit check; callbacks that changed nothing shouldn't move it.
- Missing commands found running the capture kit in Aztek (09/11/2026), still missing 16/11/2026: `assignCommand`, `hotkeySet`, `imagePlane`, `runTimeCommand -dca` and `-uca` (`allNodeTypes` is in).
- PLAN.md (08/11/2026): parity review's work queue. Phase 0 and 1 before anything else that isn't a ticket.
- Cloth feels wrong at scale: a cloth bigger than the grid acts stiff, small or heavy even as silk. Units, gravity, bending and presets must scale (agent, top priority). Real-world units by default from `currentUnit`: 1 cm of geometry is 1 cm of cloth, so nobody works in 10 cm = 1 m.
- Shift+click on an expand arrow fully expands (and collapses) everything below it, in every hierarchy: Outliner, Hypershade, Reference Editor, Node Editor lists, Graph Editor and Dope Sheet outlines, layer editors, Attribute Editor sections.
- Harold: render layers and render setup; hair and curves (XJEN step 1); quieter subsurface; USD stages in renders; an evicting texture tile cache; vector displacement and autobump; denoiser off the interface thread.
- Bicurious: something like Bifrost built into the dynamics, not a separate system. Liquids, smoke, fire and procedural graphs share the one solver, colliding with and affecting cloth, particles and rigid bodies. Start after the unified solver merges.
- README screenshots refreshed as features land.
- Menus still missing: UV Pin, Proximity Pin, Interactive Bind Skin, Split UVs, 3D Cut and Sew, Unsmooth, Slice, Hole Punch, Cut Out, Deform Edit submenus, Prune Membership, Relationship Editors, IK/FK Keys, Audio, Cache, Visualize.
- World symmetry: an object moved after symmetry was set keeps its old pairs.
- macOS tests: `python_forms` failed on the runner with `xform -q -ro` at 44.99999999999999 for 45; values shown in degrees are now tidied, so see if it passes (09/11/2026). `cargo test` stopped at that failure, so CI now runs `--no-fail-fast` to show the rest. The step is `continue-on-error`, so a green macOS job doesn't mean its tests passed: read the log.
- macOS: the early Apple-silicon build has to be tried on a Mac (viewport through MoltenVK, Cmd and Option in mouse and key handling), then made a condition of a release.
- Lou: `poseInterpolator` (node in since 16/11/2026; try it on Lou's file), blendShape on a lattice.
- Reference gaps: `exportEdits`, `-shd`, painted weights and tweaks on referenced meshes as edits; second reference of a large rig loads slower than the first.

- USD gaps: MaterialX draws grey, subdivision meshes draw their hull, no component selection in a stage, time samples not on the time slider or in the Graph Editor, Outliner collections.
- polyMergeUV, polyForceUV with no UV selection do nothing where the reference may still add a node: check.


- More examples once they merge: anim layers, HumanIK, USD.

- Shadows, spot cone manipulators, Light Linking editor; check whether keys 5 and 6 go back to default lighting.
- Waitress rig about 14 fps measured during playback in the viewport: see what else takes the time now the skin is faster.

- C++ plug-in SDK, so developers with a plug-in's source can rebuild it; precompiled `.so` and `.dll` can't load.
  - Headers written clean under the same paths and names (`maya/MFnMesh.h`, `MObject`, `MStatus`, `MPxCommand`), with no text copied from the reference's devkit, whose licence forbids it.
  - `aztek_api` library over the core, mirroring the Python API 2.0 subset. Same entry points (`initializePlugin`, `uninitializePlugin`, `MFnPlugin`), and `loadPlugin` accepts what's built against the kit.
  - CMake package: `find_package(AztekDevkit)`, `aztek_add_plugin(myPlugin SOURCES ...)`. Same MSVC as Aztek on Windows.
  - A missing class or method is a compile error naming it; a small tool turns those into a report for the backlog.
  - First: `MPxCommand`, `MArgDatabase`, `MSyntax`, `MPxNode`, `MPxDeformerNode`, mesh, transform, skin and animation function sets, iterators, math classes. Later: locators and custom drawing, `MPxContext`, `MQtUtil`. Then the dynamics and rendering classes, once the unified solver and Harold have landed.


## Session log

- 0.4 `--batch`. Done: no window, no Vulkan, scene, scripts and commands in order, exit status from uncaught errors and `sys.exit`, `about -batch` true, CI on all three platforms.
  Not done: `basics.rs` can't run through the binary as it stands, since it asserts in Rust; `tests/batch/basics.py` carries the same calls and grows with each item.
  Why: the suite drives one `Core` in process. CI runs both suites by name and `basics.py` through the binary.
- 0.1 Capture kit. Done: `tools/reference_capture/` with `capture.py`, `calls.json`, `scenes.json`, `node_types.txt`, `WANTED.md`; dry run through `aztek --batch` with no script errors.
  Not done: never run in the reference itself, and no fixtures exist yet.
  Why: needs srlegrand to run it: `mayapy tools/reference_capture/capture.py`, then commit `tests/data/reference/`. Nothing else waits on it.
- 0.2 Fixture harness. Done: `reference_fixtures.rs` compares 99 calls with hand-written readings, reports by area, and fails on a new mismatch or a stale entry on the open list (44 entries at the start).
  Not done: no comparison yet for attribute, hotkey or posed fixtures; 6 calls are skipped with no reading.
  Why: nothing captured to compare against, and writing those comparisons blind would guess at the fixture shapes' use.
- 1.1 Real units. Done: centimetres and radians stored, conversion at commands, `.ma`, panels, expressions, Python API and USD, `unitConversion` nodes, time units, audit in PARITY.md.
  Not done: three readings wait on a capture (matrix queries, primitive default sizes, `polyEvaluate -b`). The Channel Box check under Xvfb comes after 1.14 with its own item.
  Why: the reference has to answer those; the Channel Box works on what it shows, which is now in the current unit.
- 1.2. Done: `group1`, `group2` from `group`, `group -em`, `group -n` and the Ctrl+G menu item, which runs the same command
  Not done: nothing
  Why: one command
- 1.3. Done: `group` under the members' shared parent, pivots at the world origin, members keep their world place
  Not done: `-em` pivots stay local
  Why: the reference's empty group has no members to place it
- 1.4. Done: `rename` turns each run of # into the next free number, padded by its length
  Not done: `polyCube -n thing#` not done
  Why: the item names rename; ticket added
- 1.5. Done: `arm_01` copies to `arm_02`, `ctl_L_009` to `ctl_L_010`, `pCube2` has `pCubeShape2`, `listRelatives` leaf names
  Not done: padding rules unverified against a capture
  Why: calls.json holds them until srlegrand runs the kit
- 1.6. Done: `move -a 0 0 0 obj.rp obj.sp`, either pivot alone, relative and object space
  Not done: `-rpr` and `-spr` are accepted and do nothing on pivots
  Why: nothing in the reference's docs says what they mean there
- 1.7. Done: objects and components invert
  Not done: masks only by component kind, not selectType's object masks
  Why: needs capture of what a mask does to an invert
- 1.8. Done: `hotkey -q -k F12 -n`, `-rn`, from MEL and Python
  Not done: hotkey sets (`hotkeySet`) not there
  Why: ticket from the capture kit
- 1.9. Done: `polyEvaluate -e` and the 10 other commands with an own -e (`polyCheck`, `polyCompare`, `delete`, `findType`, `attributeInfo`, `createSet`, `createDisplayLayer`, `namingConvention`, `timer`, `scriptJob`), and `usdPy -q`, in MEL and Python
  Not done: nothing; -c never meant create in the parser, so it needed no change
  Why: audit in the session's flag dump; PARITY.md lists them
- 1.10. Done: `src.t -> dst.r -> src.s` connects; `polyBevel3` pin gone from PARITY.md; `shell` test now asks for UV shells
  Not done: `a.tx -> b.tx -> a.tx` still refused
  Why: the evaluator needs a cycle guard first
- 1.11. Done: `-uvShell`, `-uvShellIds`, `-uvsInShell`, `-activeUVShells`, `-activeShells`, `-uvComponent`, `-b2`, `-bc2`, units on `-b`, `-bc`, `-a`, `-wa`
  Not done: the shape of the UV list answers is from the docs, unverified
  Why: needs a capture
- 1.12. Done: `displaySmoothMesh`, `smoothLevel`, `useSmoothPreviewForRender`, `renderSmoothLevel`, `displaySmoothness -po` set and `-q -po` per object; the viewport and Harold read them
  Not done: the rest of the reference's smooth attributes (`continuity`, `smoothTessLevel` and so on) aren't there
  Why: they change nothing here until the preview honours them
- 1.13. Done: `shadingNode -isColorManaged`, `-asRendering`; `hyperShade -lus -lds -snw -d`
  Not done: `-rgp`, `-rs`, `-inc`, `-cwa`, `-n` accepted and do nothing
  Why: they drive the reference's Hypershade window, which Aztek doesn't have in that form
- 1.14. Done: `listRelatives -ap` gives every parent, attributes set on one instance's shape reach the others, a rename follows, `.ma` keeps the reference's form
  Not done: one node under two parents inside the graph; `instanceCount`; non-mesh instances
  Why: a true multi-parent graph means rewriting every DAG walk; ticketed for srlegrand's call
- After 1.14, every test and example under `--batch`. Done: the full suite green with no basics test ignored and `reference_open.txt` empty; all 51 example scenes open with no warnings; world matrices and mesh points at frames 1, 12 and 24 match the build from before the unit change on all 51; `aztekDynamics` warning gone; `tools/check_examples.sh` in CI.
  Not done: `examples/tools/make_examples.mel` only runs from the repo's root, as written.
  Why: it's the tool that writes the examples, not an example.
- Channel Box multi-edit under Xvfb. Done: `tools/ui_checks/channel_box.py`, by mouse and keyboard: Shift+click run, 2.5 on both cubes and all three channels, +=1, one undo, drag down the names, Ctrl+click; in CI on Linux with openbox.
  Not done: nothing.
  Why: found on the way, a script's idle job takes the keyboard from a field: ticketed.

## Done
- 16/11/2026: joints under an IK solver keep keyable FK rotates on the solver; keys survive making a handle, play where IK is off, and save on the joint.
- 16/11/2026 Issues 37 to 83 closed but #76 (PARITY.md, Issues 37 to 83): editors, views, UV, picking, rigging,
  skinning, `aztekpy`, themes. The UV and Vertex Face modes with Multi, and `.mb` reading, came off this list.
- 08/11/2026 One light model: the viewport's lights render the same in Harold (decay rates, intensity, colour, spot cone and dropoff, ambient, area, volume, default lighting, ambient colour, black background); parity test against the shared formula.
- 07/11/2026 Harold renderer, first parts: path tracer, standard surface, every material and light translated, Render View with IPR, Render Settings, AOVs to EXR, batch rendering; see docs/HAROLD.md for what is shallower.
- 07/11/2026 History from every route: Smooth, Mirror, Reduce, Extract no longer wipe it; `G` after each; Hypergraph shows the whole chain.
- 07/11/2026 Smooth Preview wire as asked.
- 07/11/2026 First class USD, priorities 1 to 8 with the gaps above.
- 07/11/2026 Skin core: parallel, flat table, shared topology; a deformed mesh can't go stale in the viewport.
- 07/11/2026 Anim layers and HumanIK retargeting; gaps in PARITY.md (Custom Rig, full-body solver, ghosts).
- 07/11/2026 File > Examples with modelling history, materials, a coloured keyed arm rig and nested references.
- 07/11/2026 All six default lights in the viewport, commands and `.ma`; no shadows yet.
- 07/11/2026 Sculpting icons drawn to show what each tool does.
- 07/11/2026 Symmetry on polygon actions, B fixed, hold B falloff circle, Global, Object and Surface falloff, many menu items and option boxes.
- 07/11/2026 Every build of main is a numbered release, 1.N with N the commits so far, titled with the commit; the app shows the same.
- 24/11/2026 Anim layer weight keys: the Anim tab's K button records the layer weight, but no key appears in the Graph Editor or the Time Slider.
