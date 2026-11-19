# scripts

Global MEL procedures that come with Aztek, laid out as the reference's installation lays out its own: `startup/` holds
those sourced as the program starts, `others/` those sourced the first time they're called. They're built into the
program (`crates/aztek_host/src/mel/bundled.rs` lists them) and found after the user's script path, so a script of the
same name on `MAYA_SCRIPT_PATH` or `AZTEK_SCRIPT_PATH` wins. `whatIs` names the file each one came from.

Each file is Aztek's own, written to take the arguments the reference's procedure of that name takes and do what it does
with Aztek's commands. Add a procedure here when scripts people use call it.
