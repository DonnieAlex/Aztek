# Third-party material

Aztek is MIT-0. Six pieces of it come from elsewhere under the MIT licence, which asks for its notice to go with copies.

The programs and packages also carry Qt and PySide6/shiboken6, under the LGPL, and Python, under the PSF licence. They
are not part of the source: every build fetches them (`tools/qt/fetch-qt.sh`, `tools/python/fetch-pyside.sh`) and every
package installs them beside Aztek. Qt: https://www.qt.io/licensing. PySide6 and shiboken6:
https://doc.qt.io/qtforpython-6/licenses.html, LGPL-3.0-only or GPL-2.0-only or GPL-3.0-only. Python 3.13:
https://docs.python.org/3/license.html.

## Binary scene type ids

`crates/aztek_scene/data/mb/typeids.dat`, the four-character ids of node types in `.mb` files, is from
maya-scenefile-parser (https://github.com/mottosso/maya-scenefile-parser, also https://github.com/agibli/sansapp),
unmodified. It is built into the program.

Copyright (c) 2016 Alon Gibli. Licence: MIT, full text in `crates/aztek_scene/data/mb/LICENSE`.

## mGear's solvers

`plugins/mgear_solvers/` is the source of mGear's `mgear_solvers` plug-in (the `src/` folder of
https://github.com/mgear-dev/mgear at commit 62d9708 of 06/11/2026, after mGear 5.4.5), unmodified. It is built with
Aztek's plug-in kit to test the kit, and does not ship with the program.

Copyright (c) 2011-2018 Jeremie Passerin, Miquel Campos; 2018 onwards the mGear-Dev Organization. Licence: MIT, full
text in `plugins/mgear_solvers/LICENSE`.

## mGear's weight driver

`plugins/mGearWeightDriver/` is the source of mGear's `mGearWeightDriver` plug-in (the `source/` folder of
https://github.com/mgear-dev/mgear_weightDriver at commit 65a0133 of 06/04/2026, version 3.6.3), unmodified but for
its `CMakeLists.txt` and build environment folder, which are left out. It is built with Aztek's plug-in kit to test
the kit, and does not ship with the program.

Copyright (c) 2021 Ingo Clemens, brave rabbit. Licence: MIT, full text in `plugins/mGearWeightDriver/LICENSE`.

## cvWrap

`plugins/cvwrap/` is the source of the `cvwrap` plug-in as mGear publishes it (the `src/` folder of
https://github.com/mgear-dev/cvwrap at commit 882469a of 19/05/2021, a fork of https://github.com/chadmv/cvwrap),
unmodified but for its `CMakeLists.txt`, which is left out, with the repository's `LICENSE` beside it. It is built
with Aztek's plug-in kit to test the kit, and does not ship with the program.

Copyright (c) 2015 Chad Vernon. Licence: MIT, full text in `plugins/cvwrap/LICENSE`.

## openusd

`crates/vendor/openusd/` is the `openusd` crate, version 0.8.0 as published on crates.io (https://github.com/mxpv/openusd),
with its examples and tests left out and its `zip` dependency moved from 8 to 9, which takes a few small changes in
`src/usdz/reader.rs` and `src/usd_utils/package.rs`. It is built into the program in place of the crates.io copy until
a release past 0.8.0 makes the same move.

Copyright (c) 2024 Maksym Pavlenko. Licence: MIT, full text in `crates/vendor/openusd/LICENSE`.

## The reference's command list

`parity/commands.tsv` and `parity/command_flags.tsv`, the parity ledger's first rows, list the reference's commands and
their flags by name only, taken from the command stubs of the `types-maya` package, version 0.6.1, on PyPI. Licence:
MIT, full text in `tools/reference_capture/types-maya-LICENSE`.

## Test files

`crates/aztek_scene/tests/data/mb/` holds two `.mb` files used only by the tests, each with its MIT licence beside it;
`crates/aztek_scene/tests/data/mb/README.md` says where they are from. They are not part of the program.
