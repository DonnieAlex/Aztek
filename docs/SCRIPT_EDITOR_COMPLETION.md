# Script Editor completion

The input pane's completion follows a JetBrains editor: a popup at the caret with the names that continue what is
typed, a detail line for the selected entry, and a hint for a call's arguments. This is the specification the
implementation in `app/src/completion.cpp` and the host's completion query are written against.

## The switch

`Command > Command Completion` is a checkable item, on by default off. Its state is kept in
`QSettings` under `scriptEditor/commandCompletion`. With it off, no popup appears and typing behaves exactly as before.
`Object Path Completion`, `Show Tooltip Help` and `Show quick help` stay greyed: Aztek has no object path completion and
no Quick Help window. They are shown rather than hidden, as the menu has always shown them.

## The popup

- Opens under the caret, or above it when there is no room below. It is clamped to the Script Editor window, never to
  the screen edge.
- A row is 18 pixels high, in the editor's monospace font. Rows show an icon, the name, and the entry's category at the
  right in a muted colour. The selected row uses the editor's selection blue.
- Icons carry the kind: module, class, function, method, flag, property, constant.
- A detail area under the list shows the selected entry's signature and a one-line description. Descriptions are
  elided to the popup's width.
- A call shows an argument hint line above the list: the parameter list, with the parameter under the caret in white and
  the rest muted. The hint follows the commas typed in the call.
- Long lists scroll with the wheel, Page Up and Page Down, and show a slim scrollbar. The selection always stays
  visible. The popup has at most 12 rows; more are scrolled.

## Filtering and ranking

The typed prefix is matched against every candidate, without rebuilding the index:

1. case-sensitive prefix, best;
2. case-insensitive prefix;
3. camel-case initials, so `pc` finds `polyCube` and `msl` finds `MSelectionList`;
4. case-insensitive substring, worst.

With nothing typed the list is alphabetical; with a prefix, shorter names come first within a rank, then alphabetical
order. Filtering is case-insensitive for matching but the rank keeps an exact-case prefix above the rest. A greyed
entry keeps its fit, after enabled entries of the same fit and before a worse fit.

## Keys and mouse

- Up and Down move the selection by one row, without wrapping.
- Page Up and Page Down move by a page.
- Enter or Tab accepts the selection; Tab indents as usual when no popup is up.
- Escape dismisses the popup and nothing else.
- Moving the caret with the keyboard re-evaluates the context; clicking in the text dismisses the popup.
- Hovering a row selects it; a click accepts it.
- The wheel moves the selection three rows at a time and scrolls the list to keep it in view.

Accepting replaces the typed prefix only: the word for a name, the `-word` for a MEL flag, or the current argument
segment for a Python keyword. Accepting closes the popup. Undo takes the insertion back in one step.

## What is offered, and where

### MEL tabs

- At the start of a statement, after `;`, or after `=`, `(`, `[`, `{`, `,` or a backtick: command names, the bundled
  and live MEL procedures, and MEL's built-in functions. That covers a command inside a variable assignment and inside
  a backtick expression.
- After a command and a space, or after a `-`: that command's flags, both long and short forms, with the flag's type in
  the category column and its help in the detail area. Filtering matches either form. The command may be anywhere in
  the statement, so `$x = `polyCube -w` completes the flags too.
- After `command (` or between its arguments: the argument hint; the current parameter is the one at the caret.

### Python tabs

- `maya.` offers `cmds`, `mel`, `OpenMaya`, `OpenMayaAnim`, `OpenMayaMPx`, `OpenMayaUI`, `api`, `standalone`, `utils`,
  `app` and `internal`, as Aztek's importer provides them.
- `maya.api.` offers `OpenMaya`, `OpenMayaAnim`, `OpenMayaUI` and `OpenMayaRender`.
- `maya.cmds.` offers the command registry: every command Aztek registers, with its flags and help. The nine MEL-only
  commands (`isParentOf`, `lsType`, `listTransforms`, `findRelatedDeformer`, `getChain`, `getAllChains`, `gotoBindPose`,
  `setConstraintRestPosition`, `moveIKtoFK`) are shown greyed, because `maya.cmds` has no function for them.
- `maya.mel.` offers what that module has, which is `eval`.
- `maya.OpenMaya.` and `maya.api.OpenMaya.` offer the classes of each API.
- A class offers its members, inherited ones included, with the method signature and any docstring the class carries.
  A class's entry shows its constructor signature.
- A command name followed by `(` shows the argument hint and offers the command's flags as keyword arguments. The
  hint's current parameter follows the commas typed.
- `import maya.cmds as mc` makes `mc.` resolve to `maya.cmds`, and likewise for `from ... import ... as ...`.

### Both

- The index is built once from the host: the command registry, the procedure lists, and the Python API, and kept in
  memory. Filtering runs over that memory. The session's own MEL procedures are re-read when the command list is
  prepared, so one defined while working appears without reopening the panel.
- A module Aztek ships as an empty package is listed greyed with "Empty package in Aztek" rather than hidden or
  invented. Nothing that Aztek does not register or ship is offered.
- Completion is suppressed inside a comment or a string, where names mean nothing.

## The data

From the host:

- `completionIndex`: every command with its positional help and its flags (long name, short name, type, help, arity,
  choices); every global MEL procedure with its signature (bundled scripts and the session's own); MEL's built-in
  functions. Its `python` member holds each shipped Python module, its module level entries, and the names of the
  modules under it.
- `completionClass(module, class)`: a Python class's constructor signature, docstring and members, loaded when a class
  is first completed and kept afterwards.

Python API members come from the real embedded modules (`maya.api.OpenMaya`, `maya.OpenMaya`, and the sibling API
modules), not from a hand-written list. A class that the module carries is what is offered; the module's classes
without docstrings show their signature only.

## What this does not do

- No object path completion; it has no index in Aztek.
- No completion in a multi-line expression that continues on the previous line.
- No type inference for an arbitrary Python expression before a dot; only module and class names, import aliases and
  the command names resolve.
- No docstrings for most API members; they are not in the modules Aztek ships.
