# scripts

Global MEL procedures that come with Aztek, laid out as the reference's installation lays out its own: `startup/` holds
those sourced as the program starts, `others/` those sourced the first time they're called. They're built into the
program (`crates/aztek_host/src/mel/bundled.rs` lists them) and found after the user's script path, so a script of the
same name on `MAYA_SCRIPT_PATH` or `AZTEK_SCRIPT_PATH` wins. `whatIs` names the file each one came from.

Each file is Aztek's own, written from a description of what the reference's script of that name does (its procedures,
their arguments and results, the windows it opens), never from the reference's code, and doing it with Aztek's
commands. Where the reference builds a window Aztek has natively (Script Editor, Render View, Plug-in Manager, Hotkey
Editor and the rest), the procedure opens Aztek's. The procedures of the scripts in `startup/` are known by name from
the start, as the reference's are, whatever file they sit in, and `source` finds these files by name too. What they
still need from Aztek is in [docs/SCRIPT_GAPS.md](../docs/SCRIPT_GAPS.md). Add a procedure here when scripts people use
call it.
