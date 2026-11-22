# Aztek: working notes

## What it is

Aztek is a modelling, UV, rigging and animation application that behaves like the reference application (Maya; everywhere else it is called "the reference"),
released by srlegrand at https://github.com/srlegrand/Aztek. PARITY.md holds the target and status.

## Rules

- The reference's behaviour is the spec: command names, flags and short names, query/edit modes, return values, node and
  attribute names, defaults, hotkeys, menus, marking menus, channel box colours. When in doubt, do what the reference does.
- Name the reference only where nothing else works: module, command, node, attribute, file-format and plug-in names that
  scripts and files depend on (`maya.cmds`, `mayaUSDImport`, `mayaAscii`, `MAYA_SCRIPT_PATH`, the `.ma` header). Everywhere
  else, code identifiers, comments, docs, help text, tests and the interface, write "the reference". No logo, icons,
  branding or artwork from it. The project's history and any earlier code it came from are not mentioned.
- The Python API is `aztek.cmds`; `maya.cmds` / `maya.mel` are thin shims onto it so existing scripts run.
- Scope: modelling, UVs, rigging, animation, viewport navigation, manipulators, script editor, preferences, one unified dynamics system (cloth, particles, rigid bodies), the Harold renderer, and Bicurious, which is part of the dynamics rather than a separate system. Dynamics and rendering deliberately depart from the reference where it carries old overlapping systems: `.ma` files using any of them load into the one system, and the interface says only Cloth, Particles and Harold.
- Never add AI or Claude attribution anywhere: no Co-Authored-By, session links or "Generated with" lines in commits,
  no credits in files, READMEs or comments.
- Pushing to `origin main` is allowed when tests pass. Every push builds and releases through GitHub Actions, so a push also carries the README, docs and website's download links up to date. No `git pull`; merge what's on origin with `git merge --ff-only`. Commit only what you mean to: check for stray files before `git add`.
- `parity/` is the parity ledger (parity/README.md). After adding a command, flag, node type or attribute, run
  `cargo run -p aztek_host --example parity -- --write` and commit what it changes; tick rows of the hand-checked areas
  as their features land.
- Writing style in docs and comments: dry and plain, no em dashes, dates as DD/MM/YYYY.
- Licence: MIT-0 (LICENSE). Keep the README, website, Cargo.toml and packaging saying the same. Copies of the repo on any
  other host are welcome: the README, the website and the About box say so; keep them saying it.

## Layout

- `crates/aztek_core` prefs, paths. `aztek_undo` undo stack. `aztek_ops` command layer (CommandDef, flags, execute).
- `crates/aztek_geo` mesh (half-edge style corners, attributes; UVs are per-corner `uv0`, seams edge `seam`), ops.
- `crates/aztek_uv` charts, unwrap (LSCM, SLIM, BFF), layout helpers, projections.
- `crates/aztek_render` Harold, the renderer, with no scene dependency: BVH, geometry and instances, camera, textures,
  shading networks, the standard surface's BSDF, lights, path tracer (`integrator.rs`, `path.rs`, `first_hit.rs`), AOV
  film, progressive job (`render.rs`), image output, denoiser. `aztek_scene/src/harold/` translates the scene into it and
  holds the render commands; `harold_nodes.rs` the settings, shader and light node types; `aztek_host/src/harold_view.rs`
  the render view's C functions and `aztek -render`; `app/src/renderview.cpp`, `rendersettings.cpp` the windows.
  docs/HAROLD.md.
- `crates/aztek_scene` the scene: graph, nodes, plugs, DAG (transform over shape: pCube1 / pCubeShape1),
  construction history (`history.rs`: one node per edit, polyTweak for point moves), components, mesh edits,
  `anim_layer*.rs` (animLayer, blends, merge), `anim.rs` (animCurveTL/TA/TU, setKeyframe, keyframe, cutKey, keyTangent, playbackOptions, currentTime in frames,
  autoKeyframe, held values), `uv_ops.rs` (maps and shells from corner UVs, projections, unfold, layout, cut, sew,
  polyEditUV; each a history node holding a UvEdit), `rig.rs` (joint, orientJoint, skinCluster, skinPercent,
  constraints, ikHandle), `ik.rs` (RP, SC, spline), `constraint.rs`, `skin.rs` (weights, painting, mirror, copy),
  `deform*.rs` (nonLinear, cluster, blendShape, lattice, wrap, wire, delta mush, softMod and the rest), `curves.rs` and
  `surfaces.rs` (NURBS), `sculpt.rs`, `hik*.rs` (HumanIK: nodes, slots, retarget, commands), `ma_file.rs` and `ma_file/deformers.rs` (.ma read and
  write), `references.rs`, `ref_cmds.rs` (polyEvaluate, listConnections, filterExpand, ...). New commands register in
  `session.rs`.
- `crates/aztek_dyn` the dynamics solver: one XPBD solve for cloth, particles and rigid bodies (`solver.rs`,
  `constraints.rs`, `contacts.rs`, `resolve.rs` for impossible states, `rigid.rs`, `fields.rs`, `emit.rs`, `cache.rs`).
  The scene wraps it in `dynamics.rs` (nodes), `dyn_solve.rs` (stepping with time from an evaluate hook),
  `dyn_cmds.rs` and `dyn_particles.rs` (commands) and `ma_file/dynamics.rs` (older systems' nodes read into it);
  `aztek_host/src/dyn_draw.rs` draws it. See docs/DYNAMICS.md.
- `crates/aztek_usd` USD on the native `openusd` crate: `export*.rs` and `import*.rs` (`mayaUSDExport`, `mayaUSDImport`),
  `stages.rs` (open stages by node, undo through the scene's stack), `proxy.rs` (`mayaUsdProxyShape`), `prims.rs` and
  `prim_cmds.rs` (prims named `proxy,/path`, `usdPrim`, `usdAttr`, wrapping `select`, `move` and the like), `layers.rs`,
  `translate.rs` (pull, merge), `draw.rs` (what a stage draws, no GPU), `panels.rs`, `py_api.rs` (`usdPy`), `extras.rs`.
  Tests write their own `.usda`. The host draws through `usd_view.rs`; `python/pxr_shim.py` is the `pxr` module;
  `app/src/usdui.cpp` is the interface. See `docs/USD.md`.
- `crates/aztek_host` static library the UI links: C ABI (`lib.rs`, header `app/src/aztek.h`), `core.rs` (Core:
  exec with MEL echo, exec_silent for previews, retract_preview, query/edit routing, UI runtime commands like
  GraphEditor, MoveTool), `mel.rs` (MEL parsing, the reference command and flag aliases), `query.rs` (JSON snapshots for the
  panels), `view.rs` / `viewport.rs` / `render.rs` / `gpu.rs` (ash Vulkan viewport, picking, manipulators, joints and
  handles drawn as lines, snapping X/V, pivot mode D), `tools.rs` (Multi-Cut, Quad Draw and other view tools),
  `python.rs` + `python/bootstrap.py` (pyo3), `scriptui.rs`, `panel_cmds.rs`, `ui_cmds.rs`, `ctx_cmds.rs` and
  `script_jobs.rs` (script-built UI, panels, tool contexts, scriptJob), `mel/` (the MEL interpreter).
- `app/src` Qt 6 Widgets shell in C++20: `mainwindow.*` (menus by menu set, shelves, hotkeys, time slider, editors),
  `panels.*` (outliner, channel box with picked channels and virtual slider, layer and attribute editors, script
  editor), `viewport.*`, `widgets.*` (option boxes, time and range sliders, splash, About), `grapheditor.*`,
  `uveditor.*`, `dopesheet.*`, `nodeeditor.*`, `toolsettings.*`, `shapeeditor.cpp`, `runtime.cpp` (script-built windows),
  `animlayers.cpp` (Anim tab), `hikwindow.cpp` (Human IK), `markingmenu.*`, `icons.cpp` (icons drawn with QPainter), `style.qss`.
- `site/` the website (GitHub Pages, `.github/workflows/pages.yml`; `sh site/build.sh _site` to preview). It uses the
  images in `docs/`; keep it and the README saying the same things.
- `app/icons` logo and app icon (SVG sources, PNG, ICO). `docs/` README screenshots.

The UI talks to the core only through commands (exec, exec_quiet, ask) and JSON queries; panels refresh when the
core's version counter moves (Host poller, 50 ms, or poke()). Drags preview with silent commands and run the final
command once on release so the history shows one reference-style line.

## Build and test

    ./build.sh                 # Linux: fetches Qt 6.8.3, Python 3.13 and PySide6 (the VFX Reference Platform
                               # CY2027's versions) into build/ the first time, builds, and stages build/Aztek; --run starts it
    build.bat                  # Windows (PowerShell script build.ps1), which fetches them into build\ and makes build\dist
    cargo test --workspace     # all Rust tests; keep them green

The build never uses the system's Qt or Python: `tools/bundle/versions.sh` holds the versions, `tools/qt/fetch-qt.sh`
and `tools/python/fetch-python.sh` fetch them once, and `tools/python/fetch-pyside.sh` installs PySide6 and shiboken6
against that Qt. `cargo test` still needs a Python 3.13 for pyo3: point `PYO3_PYTHON` at
`build/python/bin/python3.13` once `./build.sh` has fetched it, and add
`RUSTFLAGS="-L $PWD/build/python/lib -C link-arg=-Wl,-rpath,$PWD/build/python/lib"` so the test programs find its library.

The GitHub Action `.github/workflows/build.yml` builds Linux, Rocky Linux 8 and 9 and Windows on every push to main and attaches
the packages to the release. `packaging/` holds the RPM spec, the deb and Windows installer scripts and the install checks; see
`docs/PACKAGING.md`.

Checking a real `.ma` with no UI: `cargo run --release -p aztek_scene --example load_ma -- file.ma` lists warnings and how far the
rig is from what the file stored; `--example rig_diff -- a.ma b.ma node.attr=value` compares two files under a pose. Rig files and their
textures from elsewhere stay out of the repo: tests use small `.ma` text written by hand (`tests/rig_files.rs`).

Checking anything else without a display: `./build/Aztek/AppRun --batch --script file.py` (or `.mel`, `--command mel`, a
scene file) has no window and no Vulkan, and exits with the script's status.
`AZTEK_BIN=$PWD/build/Aztek/AppRun cargo test -p aztek_host --test batch` runs its tests through the bundle, and
`tools/check_bundle.sh build/Aztek` checks that Python, PySide6, shiboken6 and Qt all come from it.

Checking the UI without a display: run under Xvfb and look at the pictures.

    xvfb-run -a -s "-screen 0 1700x1050x24" ./build/Aztek/AppRun --script scene.mel --screenshot out.png

`--script` runs MEL or Python at start and leaves the camera where the script or file put it (`viewFit -all` frames); `--screenshot` saves the main window (viewports included) and quits. For
interaction (marking menus, drags, separate editor windows) drive a real Xvfb with xdotool and grab the screen with
ImageMagick `import -window root`. `AZTEK_ECHO=1` prints the script history to stderr. Runtime commands such as
`GraphEditor;`, `UVEditor;`, `FourViewLayout;`, `JointTool;`, `MoveTool;` let a script open editors and set tools.

Timing the poly tools on a dense mesh: `cargo build --release -p aztek_host --example poly_bench`, then
`tools/poly_bench.sh [faces] [seconds each] [name filter]` prints each command's milliseconds and the face count after.
Run it before and after a change to a mesh op; `crates/aztek_geo/tests/dense.rs` holds the dense cases as tests.

## Issues

The issue bot (docs/ISSUE_BOT.md) works on GitHub issues unattended by `.claude/skills/fix-issue/SKILL.md`; the same
skill applies when working on an issue by hand.

## Working loop

Pick the next gap from PARITY.md ("Not yet") or below, implement it to the reference's behaviour, add tests in the crate's
`tests/`, build, check it in the UI with screenshots, update PARITY.md's status with the date. Commit locally in small
steps with plain messages (no attribution); srlegrand pushes.

## Next

Work PLAN.md from the top: ground truth from the reference first, then the basics, then breadth. TICKETS.md wins when
srlegrand puts something there. PARITY.md stays the record of what is done and how it differs.
