# Data and diagnostic scripts

These files support the documents in the parent folder. They are not product code.

| File | Use |
| --- | --- |
| `cpp_class_gaps.tsv` | Every C++ reference class, its missing methods, signature and overload gaps |
| `python_class_gaps.tsv` | Every Python 1.0 class, missing methods, and missing or extra constants |
| `enum_gaps.tsv` | Every enum or constant set that differs between the reference and Aztek |
| `reference_class_matrix.tsv` | All reference classes with their C++ and Python status |
| `name_candidates.tsv` | Raw pairing of missing 1.0 names with the nearest implemented names, including rejected pairs |
| `module_dependencies.tsv` | Module-to-module edge weights |
| `dependency_edges.json` | Every class-to-class base or signature edge |
| `probe_name_mismatches.py` | Live check for the confirmed Python 1.0 name aliases |

## probe_name_mismatches.py

Purpose: confirm that the 1.0 names in `../name_mismatches.md` resolve on a running build, and that the 2.0 controls
still resolve. It is a quick check for an agent working on one alias. It is not the full reference audit.

Scope:

- 18 required names, the confirmed mismatches.
- 6 probable names, the rows that still need a signature check.
- 2.0 control names that must keep working while an alias is added.
- It does not check signatures, argument order, return values or behaviour.
- It does not cover the other 961 missing names or the missing classes. Those are in `python_class_gaps.tsv`.

Run from the repository root, with a built app:

```sh
build/Aztek/AppRun --batch --script docs/openmaya_work/data/probe_name_mismatches.py
```

Output: one `Class.name=YES` or `Class.name=NO (kind)` line per check, then a summary. Exit status is 0 when every
required name resolves, 1 when any required name is still missing. Probable and control failures are printed but do
not fail the run.

Side effects: creates a sphere named `probeSphere` in the batch scene. The batch process exits without saving, so no
file or scene outside the process is changed.

When to use it:

- Before starting a name work item, to see the current state.
- After adding an alias, to confirm the required name resolves and the control still does.
- In a pull request note, paste the summary and the exit status.

Do not use it as the only test for a change. The unit test for an alias belongs in
`crates/aztek_host/tests/python_api_v1.rs`, as `SUBAGENT_RULES.md` says.
