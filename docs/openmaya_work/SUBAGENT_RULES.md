# Subagent rules for the OpenMaya work

These rules are the last step of the documentation set. They govern how several agents split the implementation
without overwriting each other or duplicating work. The documentation in this folder is the source of truth.

## Authority and precedence

1. The 2022 C++ reference pages, whose classes and signatures are listed in `reference/`, are the specification.
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

## Claiming on GitHub comes first

A work item is claimed on GitHub before any branch, worktree or code exists. The local queue is only a mirror of the
GitHub issue. It is not a claim system of its own.

The claim follows `CONTRIBUTING.md`, "Before you start on an issue". That section is the authority; the steps below are
the subagent version of it.

### Step 0: check the issue tracker first

Before creating an issue or claiming anything, search the repository's issues and pull requests for the same work.
This is the first action of a work item, ahead of authentication, branching or editing.

```sh
gh issue list --repo srlegrand/Aztek --state all --search "OM-<area>-<number>"
gh issue list --repo srlegrand/Aztek --state all --search "<class or method>"
gh pr list --repo srlegrand/Aztek --state all --search "<class or method>"
```

Also search for the documented 1.0 name and the Aztek name when the item is a name fix, and for the class name alone
when the work is a class. Read every plausible hit, including closed issues and merged pull requests.

Then choose one of these outcomes:

- An open issue already covers the work. Claim that issue. Do not open a second one.
- A closed issue or merged pull request already covers it. Record the link and stop, or reopen only if the work is
  genuinely incomplete. Do not duplicate a settled item.
- An open issue overlaps but is not the same. Link it in the new issue and say what is different.
- Nothing matches. Create the issue in the claim step below.

The work item record must carry the search queries and their result:

```
issue check: <queries>, found #<N> | none
```

An agent that cannot show this check has not started the item correctly and must not create a branch.

### Preconditions

An agent must satisfy all of these before it claims anything:

- It is authenticated: `gh auth status` succeeds for the account that will do the work.
- The target repository is reachable: `gh repo view srlegrand/Aztek` succeeds.
- The issue is open and publicly visible: `gh issue view <N> --repo srlegrand/Aztek --json number,url,state,labels`
  returns it. A private, deleted or forbidden issue stops the claim.
- The issue is not owned by the issue bot or by srlegrand: no `auto:working`, `auto:pr` or `needs-srlegrand` label.
- No open pull request already fixes it, and no unexpired claim exists. Search the comments and pull requests as
  `CONTRIBUTING.md` says. A claim lapses after 3 days with no comment, push or pull request update from its claimant.
- The work item's dependencies in `dependencies.md` are merged or explicitly out of scope.

If any precondition fails, the agent records the item as `blocked` and stops. It does not start work on a local branch
and it does not use the queue as a fallback.

### The claim

1. Use the open issue found by Step 0. Create one only when the search found nothing that covers the work. The title
   should name the work item and the class or method. The body should link this folder, the relevant reference row and
   the search result, including any overlapping issue or pull request. Use the repository's issue template if one
   applies.
2. Post the claim with the exact `Claimed:` prefix from `CONTRIBUTING.md`:

   ```sh
   gh issue comment <N> --repo srlegrand/Aztek \
     --body "Claimed: OM-<area>-<number>, <one-line plan>"
   ```

3. Read the issue back and confirm the comment is visible and public:

   ```sh
   gh issue view <N> --repo srlegrand/Aztek --comments
   ```

4. Record the issue number, issue URL and claim comment URL on the work item. Only then change the local status from
   `queued` to `claimed`, set `owner`, and create the branch and worktree.
5. If the claim comment cannot be written, cannot be read back, or the issue is not public, the claim did not happen.
   Report the exact command and error, mark the item `blocked`, and stop.

A takeover follows the same rule with the wording from `CONTRIBUTING.md`:
`Claimed: taking over from @name, <plan>`. Do not push to the previous claimant's branch.

### Releasing a claim

- If work stops, comment `Unclaimed:` on the issue with the reason, then mark the local item `queued` or `blocked`.
- If work is done, push the branch, open the draft pull request with `Fixes #N` as soon as the first commit exists, and
  leave the issue open until the pull request is merged. The merge closes it as `CONTRIBUTING.md` describes.
- The coordinator may reassign an item only after the issue shows an `Unclaimed:` comment or the 3 day lapse. Record
  the reassignment on the issue, not only in the local queue.

### Why this order

The issue tracker is the only place every contributor looks. Checking it first prevents duplicate issues, duplicate
claims and work on something already settled. A local claim or a new issue is invisible to a person working from
GitHub, a fork or another worktree. Making the search and the issue comment the first actions means a duplicate, a
failed login, a private issue or a stale claim is found before a branch exists, not after an agent has spent time on
work it cannot publish.

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
issue check: <queries>, found #<N> | none
issue: <N> <url>
claim comment: <url> | none
owner:
status: queued | claimed | implemented | verified | merged | blocked
```

The coordinator keeps the queue. The GitHub issue is authoritative. An item may be `claimed` locally only after the
public `Claimed:` comment is visible on the issue and its URL is recorded. If the two disagree, the issue wins and the
local item is corrected to match it.

## Roles

### Coordinator

- Owns the local queue and the merge order, but treats the GitHub issue as authoritative.
- Releases an item only when its issue search is recorded, its issue is public, open and carries the claimant's
  `Claimed:` comment.
- Rejects a new issue that duplicates an existing open issue or a merged pull request.
- Verifies the claim URL before assigning files or approving a branch.
- Reads `dependencies.md` and releases items only when their prerequisites are merged.
- Assigns non-overlapping files. Two agents may not edit the same header, source file or test file at the same time.
- Runs the final verification and updates `README.md` and `PARITY.md`.
- Rejects a change that adds a stub, renames a 2.0 method for a 1.0 fix, or edits outside its declared files.
- Rejects a branch whose work item has no public claim, or whose claim was never read back from GitHub.

### Reference and audit agent

- Maintains `reference/` and the TSV records.
- Re-runs the reference capture when the target version changes.
- Reviews name candidates and moves confirmed rows into `name_mismatches.md`.
- Does not implement API code.

### Implementation agent

- Implements exactly one work item.
- Before creating a branch, shows the issue search result, the public GitHub issue URL and the `Claimed:` comment URL.
  Without all three, it stops and reports `blocked`.
- Reads the class's module document and its dependency row before editing.
- Adds the tests named in the item.
- Keeps the public header and the implementation in sync.
- Does not touch another agent's files. If a shared file is unavoidable, the coordinator serialises the change.
- Comments `Unclaimed:` on the issue if work stops, then updates the local item.

### Verification agent

- Runs the build and the tests in a clean worktree.
- Checks that the item records the issue search and that no open issue or merged pull request already covers it.
- Checks that the GitHub issue shows the claimant's `Claimed:` comment and is still open.
- Checks that the branch's pull request says `Fixes #N` and points at the same issue.
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
- A one-line comment in our own words, or a short host-limitation note, is in the header.
- `data/cpp_class_gaps.tsv` no longer lists the method, or the item notes why the gap remains.

A Python 1.0 work item is done only when all of these hold:

- The documented 1.0 name exists on the class.
- The 2.0 method still exists under its own name.
- A 1.0-style call works, including the out-parameter and `MStatus` contract.
- A test in `crates/aztek_host/tests/python_api_v1.rs` would have failed before the change.
- The introspection dump shows the name in the expected module.
- `docs/openmaya_work/data/probe_name_mismatches.py` exits 0, and its output is pasted into the handoff note.
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

For a name-alias work item, run the committed live probe first and after the change:

```sh
build/Aztek/AppRun --batch --script docs/openmaya_work/data/probe_name_mismatches.py
```

It prints `YES`/`NO` per name and exits 1 while any confirmed alias is still missing. It is a manual proof, not a
substitute for the unit test in `crates/aztek_host/tests/python_api_v1.rs`. Its scope, output, exit status and side
effects are in `docs/openmaya_work/data/README.md`.

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
