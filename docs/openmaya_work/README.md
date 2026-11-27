# OpenMaya 1.0 working set

This folder holds the reference transcription and the analysis that later agents should work from. It is documentation,
not an implementation branch.

## Precedence

- Reference: the 2022 C++ API reference, module by module,
  https://help.autodesk.com/view/MAYAUL/2022/ENU/?guid=Maya_SDK_cpp_ref_group_open_maya_html
- Python 1.0 (`maya.OpenMaya`, `maya.OpenMayaAnim`, `maya.OpenMayaMPx`) is the scripting target.
- Python 2.0 is the implementation substrate under 1.0 and is an afterthought for this work. A 2.0 name never overrides
  a documented 1.0 name.

## Contents

| Path | What it is |
| --- | --- |
| `reference/OpenMaya.md` | Every class in the C++ OpenMaya module, with bases, public members, signatures and one-line descriptions |
| `reference/OpenMayaAnim.md` | The same for OpenMayaAnim |
| `reference/OpenMayaFX.md` | The same for OpenMayaFX |
| `reference/OpenMayaRender.md` | The same for OpenMayaRender |
| `reference/OpenMayaUI.md` | The same for OpenMayaUI |
| `reference/PythonOpenMaya.md` | The current `maya.OpenMaya` classes and members, for 1.0 implementation work |
| `reference/PythonOpenMayaAnim.md` | The current `maya.OpenMayaAnim` classes and members |
| `reference/PythonOpenMayaMPx.md` | The current `maya.OpenMayaMPx` classes and members |
| `name_mismatches.md` | Python 1.0 names that Aztek implements under a 2.0 spelling, plus the related missing names |
| `dependencies.md` | Cross-module and class-level dependencies, and the order they force |
| `SUBAGENT_RULES.md` | The orchestration rules for splitting the implementation across subagents |
| `data/cpp_class_gaps.tsv` | Every C++ class and its missing methods, signature gaps and overload counts |
| `data/python_class_gaps.tsv` | Every Python 1.0 class, missing methods, and missing or extra constants |
| `data/enum_gaps.tsv` | Every enum or constant set that differs |
| `data/name_candidates.tsv` | Raw missing-name to nearest-extra-name pairing, including rejected pairs |
| `data/reference_class_matrix.tsv` | All reference classes with C++ and Python status |
| `data/module_dependencies.tsv` | Module-to-module edge weights |
| `data/dependency_edges.json` | Every class-to-class base or signature edge |
| `data/probe_name_mismatches.py` | Live YES/NO check for the confirmed 1.0 name aliases. See `data/README.md` for scope, exit status and side effects |
| `data/README.md` | Description of every data file and of the probe's purpose, scope and limits |

## Status

- Five C++ modules transcribed: 271 + 53 + 33 + 120 + 65 class entries, with duplicates across modules.
- Python 1.0 audit: 105 reference classes present somewhere across the three 1.0 modules; 961 documented names missing
  from those classes; 2,078 names belong to absent classes.
- Name mismatches: 18 confirmed 1.0-name aliases and 6 probable aliases. The confirmed rows include the reported
  `MItMeshFaceVertex.vertId` and `faceVertId` cases.
- Dependencies: 2,577 class-level edges over 542 class entries. OpenMaya is the foundation; Anim, FX and Render depend
  on it; UI depends on both OpenMaya and Render.
- The C++ kit and the Python 1.0 module are separate surfaces. A method added in one does not appear in the other.

## How the transcriptions were made

The group page for each module was read, then every linked class page. The transcription records:

- the class name, module, direct base and one-line description;
- public types, public member functions, static public functions, public attributes and static attributes;
- the exact signature text and the short description shown on the class page.

The per-member detail prose (parameter descriptions, return values, examples) is not copied. Each class has a source
link to the reference page. The "Examples:" lists were removed because they are long, noisy and not useful for an
implementation agent.

The generated files are large, especially `reference/OpenMaya.md` because of `MFn::Type`. They are plain Markdown and
can be searched with `grep` or loaded by an agent in sections.

## Reading order for a new agent

1. `name_mismatches.md` for the Python 1.0 naming work.
2. `dependencies.md` for the order of implementation.
3. The module document for the class being implemented.
4. `data/cpp_class_gaps.tsv` and `data/python_class_gaps.tsv` for the exact gaps.
5. `data/README.md` before running `probe_name_mismatches.py`.
6. `SUBAGENT_RULES.md` before creating a branch or editing shared files.

## Source and date

The pages were read on 27/11/2026. The reference is the 2022 edition. Aztek's kit comments target the 2027 API in
several places; a 2027 pass will move some names and values. Keep this 2022 set as the compatibility baseline until the
2027 manifest exists.
