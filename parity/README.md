# Parity ledger

Everything the reference exposes that Aztek has to match, one row per item, ticked off as it is added. The welcome
screen's parity bar is read from it.

## Files

| File | What it holds |
| --- | --- |
| `areas.tsv` | The areas: the columns that name a row (`key`), how it is ticked (`check`), its `weight` in the overall figure |
| `<area>.tsv` | One row per item of that area: the reference's columns, then `status` and `evidence` |
| `summary.tsv` | Each area's counts and percentage, and `all`, the overall figure. Built, never edited |
| `data/` | Reference data that isn't ticked: the inventory's `about.tsv` (which version), `node_affects.tsv`, `colors.tsv`, `ae_templates.tsv`, the default hotkey file, `inventory.json` (row counts and any part that failed) |

Tab-separated, UTF-8, LF line endings, a header row. Tabs, newlines and backslashes inside a cell are written `\t`,
`\n` and `\\`. Any spreadsheet opens them; `git diff` shows a tick as a one-line change.

## Status

| Status | Meaning |
| --- | --- |
| `done` | Aztek has it and it behaves as the reference's does |
| `partial` | There, but not all of it (counts half) |
| `missing` | Not there, or not looked at yet |
| `n/a` | Out of scope (left out of the figure). Say why in `evidence` |

`evidence` says where it is or why: a file, a test, an issue number, a reason. The checker writes `auto` in the rows it
ticks.

## Ticking

Areas whose `check` is `commands`, `flags`, `node_types` or `node_attributes` are ticked by the checker: a row is done
when Aztek registers that command (or runs it under another name, or MEL has it built in), that flag, that node type, or
that attribute on that type. It only touches rows whose evidence is empty or `auto`, so to overrule it (a command that
is there but only half works: `partial`, with a note) write the evidence.

The other areas (`hand`) are ticked by editing the row.

After adding a command, flag, node type or attribute, or ticking anything by hand:

    cargo run -p aztek_host --example parity -- --write

It rewrites the checked rows and `summary.tsv`. The `parity_ledger` tests fail while either is stale or a status is
not one of the four. Without `--write` it lists what is stale and prints the summary; `--missing <area>` lists that
area's missing rows.

## The figure

Each area's percentage is done, plus half of partial, over its rows that aren't `n/a`. The overall figure is the
areas' mean, weighted by `weight` in `areas.tsv` (1 each); an area with no rows yet doesn't count. Rows are counted
equally within an area, so a hundred thousand attributes don't outweigh a thousand commands.

## Filling it

The rows come from the reference itself. `tools/reference_capture/inventory.py`, run inside a copy of the reference,
lists everything it exposes (commands and flags, node types and attributes, both scripting APIs, the C++ headers,
environment variables, preferences, runtime and name commands, hotkeys, menus, shelves, panels, tool contexts,
translators, project rules, plug-ins) with the product's and vendor's names and the user's paths taken out, and zips
it. Then:

    python3 tools/parity/import_inventory.py reference_inventory_<stamp>.zip
    cargo run -p aztek_host --example parity -- --write

The importer rewrites each area from the inventory, keeping every row's status and evidence by its key, starting new
rows as missing and dropping rows the reference no longer has (listing those that had been ticked by hand). It refuses
the whole inventory if a cell still names the product or vendor or holds a home folder, skips an empty table, and skips
one less than half its area's size unless given `--force`.

Until the first inventory comes in, `commands.tsv` and `command_flags.tsv` hold the reference's commands and their
flags by name only (long and short names as separate rows), from the command stubs of the `types-maya` package,
version 0.6.1 (MIT, `tools/reference_capture/types-maya-LICENSE`).
