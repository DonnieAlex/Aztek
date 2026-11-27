# Subagent rules for the OpenMaya work

These rules are the last step of the documentation set. They govern how several agents split the implementation
without overwriting each other or duplicating work. The documentation in this folder is the source of truth.

## Authority and precedence

1. The 2022 C++ reference pages transcribed in `reference/` are the specification.
2. Python 1.0 names take precedence over Python 2.0 names. Never "fix" a 1.0 name by renaming the 2.0 method.
3. `dependencies.md` decides order. A class may not be marked done while a hard dependency is missing.
4. `data/cpp_class_gaps.tsv`, `data/python_class_gaps.tsv` and `data/name_candidates.tsv` decide scope. Do not expand a
   work item because a nearby method looks interesting.
5. If the reference and the existing code disagree, the reference wins. If the host cannot express the reference's
   behaviour, document the limitation in the header and in the work item before coding around it.

## Branch and worktree policy

- Each work item gets one branch and one worktree. Do not edit two worktrees that share a branch.
- Branch names: `om/<area>/<class-or-topic>`, for example `om/openmaya/mfn-mesh-create` or
  `om/python1/vertId-alias`.
- Base every branch on the current `main`. Do not base a work item on another unmerged item unless the dependency is
  explicit in `dependencies.md`.
- One logical change per commit. A class implementation, its tests and its documentation update can be one commit.
- Do not push to `main` from a subagent. The coordinator merges after verification.
- Do not force-push a shared branch. If a rebase is needed, announce it in the handoff.
- Remove the worktree only after the branch is merged or abandoned and the coordinator confirms it.

## Work item format

Every item is a short record with these fields:

```
id: OM-<area>-<number>
title:
surface: cpp | python1 | both
class or method:
reference: reference/<module>.md#<class>, plus the source URL
depends on: class or work item ids from dependencies.md
files expected:
acceptance: exact calls that must work
tests:
owner:
status: queued | claimed | implemented | verified | merged | blocked
```

The coordinator keeps the queue. An agent claims an item by changing `owner` and `status` in the queue file, not by
starting work silently. If two agents claim the same item, the first timestamp wins.

## Roles

### Coordinator

- Owns the queue and the merge order.
- Reads `dependencies.md` and releases items only when their prerequisites are merged.
- Assigns non-overlapping files. Two agents may not edit the same header, source file or test file at the same time.
- Runs the final verification and updates `README.md` and `PARITY.md`.
- Rejects a change that adds a stub, renames a 2.0 method for a 1.0 fix, or edits outside its declared files.

### Reference and audit agent

- Maintains `reference/` and the TSV records.
- Re-runs the reference capture when the target version changes.
- Reviews name candidates and moves confirmed rows into `name_mismatches.md`.
- Does not implement API code.

### Implementation agent

- Implements exactly one work item.
- Reads the class's module document and its dependency row before editing.
- Adds the tests named in the item.
- Keeps the public header and the implementation in sync.
- Does not touch another agent's files. If a shared file is unavoidable, the coordinator serialises the change.

### Verification agent

- Runs the build and the tests in a clean worktree.
- Checks the reference signature against the header.
- Checks the Python 1.0 name against the C++ page, not the 2.0 name.
- Checks that no unrelated files changed.
- Writes a verification note on the item. A failed check returns the item to implementation, not to the queue.

## Safe and unsafe parallel work

Safe in parallel:

- Different modules after OpenMaya's foundation is stable: OpenMayaAnim, OpenMayaFX, OpenMayaRender.
- Different classes in the same module when `dependencies.md` shows no edge between them and they do not edit the same
  files.
- Python 1.0 alias fixes in different class bodies, one file per agent where possible.
- Documentation updates in different files.

Unsafe in parallel:

- Constants or enum generation while another agent edits the same header.
- `MStatus`, `MString`, `MObject`, `MFn`, `MDagPath`, `MPlug` or the array classes with anything that uses them.
- `MDataBlock`, `MDataHandle` or `MArrayDataHandle` with node or deformer work.
- `MFnPlugin` with any proxy class it registers.
- `MUIDrawManager` or `MFrameContext` with `MPxDrawOverride`, `M3dView` or manipulator work.
- Two agents editing `crates/aztek_host/python/om1.py`, `om1_anim.py` or `om1_mpx.py` at the same time.

The coordinator should prefer one agent per file even when the classes are independent.

## Dependency gates

Before claiming an item:

1. Open `dependencies.md` and read the class row.
2. Check every direct dependency's status in the queue.
3. If a dependency is queued or claimed, either wait or split the item so the independent part can land first.
4. If a dependency is intentionally out of scope, record that in the item and add the documented stub or error before
   the dependent class. Do not invent a local type with the same name.

Hard gates that should never be bypassed:

- No mesh work before the numeric, point and colour arrays.
- No NURBS work before the double and point arrays and `MTesselationParams`.
- No deformer work before `MItGeometry`, `MDataBlock` and `MArrayDataHandle`.
- No draw override before `MUIDrawManager`, `MFrameContext`, `MRenderItem` and the buffer types.
- No Python 1.0 alias before the underlying 2.0 or 1.0 callable exists and is tested.

## Definition of done

A C++ work item is done only when all of these hold:

- The header declares the documented signature, including defaults and `MStatus*` where the reference has them.
- The implementation compiles in the kit, not only in the host.
- A test calls the method through the public header and checks the documented result.
- The sample plug-ins still build.
- The method's one-line description or a short host-limitation note is in the header.
- `data/cpp_class_gaps.tsv` no longer lists the method, or the item notes why the gap remains.

A Python 1.0 work item is done only when all of these hold:

- The documented 1.0 name exists on the class.
- The 2.0 method still exists under its own name.
- A 1.0-style call works, including the out-parameter and `MStatus` contract.
- A test in `crates/aztek_host/tests/python_api_v1.rs` would have failed before the change.
- The introspection dump shows the name in the expected module.
- The name is removed from the mismatch record or marked as an accepted difference.

A documentation work item is done only when the reference link, the signature and the module are present and the
document builds as plain Markdown.

## Testing and commands

Use the repository's commands, not ad hoc builds:

```sh
cargo test --workspace
cargo build -p aztek_host --example parity
cargo run -p aztek_host --example parity -- --write
```

For the kit:

```sh
cmake -S devkit/samples -B build-samples
cmake --build build-samples
```

For Python 1.0 on a built app:

```sh
build/Aztek/AppRun --batch --script <introspection-or-test-script>.py
```

If the change is UI-facing, run the relevant UI check under Xvfb and attach the screenshot to the item. A green unit
test is not enough for a viewport method.

## What not to do

- Do not rename a 2.0 method to satisfy a 1.0 name. Add the 1.0 alias to the 1.0 class body.
- Do not add a class that only imports and raises `NotImplementedError` and call it done. Thin classes are allowed only
  where the module is explicitly names-only and the item records that decision.
- Do not hand-write enum values that can be generated from the reference. Constants are ABI.
- Do not copy reference prose or examples into the codebase. Use the reference to derive behaviour.
- Do not use the reference's branding, icons or artwork.
- Do not add AI or agent attribution to commits, code, comments or docs.
- Do not edit `main` or push a shared branch.
- Do not reformat unrelated code, rename unrelated symbols or "clean up" a file while implementing one method.
- Do not mark a method done because it compiles. It must run and match a reference-derived test.
- Do not edit another agent's declared files without the coordinator reassigning ownership.

## Handoff note

Every agent leaves this note on the item when stopping, even if the work is complete:

```
status:
files changed:
what works:
what does not:
tests run:
known differences:
next step:
```

The next agent reads the note and the item's reference row before touching the branch. If the note is missing, the
verification agent rejects the item.

## Merge order

1. Constants, types and array prerequisites.
2. Scene and DG foundations.
3. Data blocks and function set bases.
4. Geometry data, then geometry.
5. Plugin bases, then the registry.
6. Render foundation, then UI overrides.
7. Anim and FX.
8. Python 1.0 aliases and missing classes.
9. Documentation and ledger refresh.

The coordinator may merge independent branches out of order only when `dependencies.md` shows no edge between them and
the verification agent has run the full test suite.

## Escalation

Stop and ask the coordinator when:

- the reference page and the existing header disagree on a signature;
- a dependency is missing and implementing it would double the item's size;
- the host cannot represent the documented behaviour;
- two work items need the same file;
- a test needs a real reference session, a display or a GPU that is not available.

Record the question in the item. Do not guess and do not hide the difference in a comment.
