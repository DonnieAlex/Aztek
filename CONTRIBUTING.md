# Contributing to Aztek

Fixes of any size are welcome, from anyone, and there is nothing to ask first. The README's
[Fix it yourself](README.md#fix-it-yourself) walks through building, changing and sending a fix step by step, by hand
or with an LLM. This file is the short version of the same, plus how work on an issue is shared, what a change has to
carry, and what happens after you send it. Questions go to the [Discord](https://discord.gg/XDEgpWDVKa) or an issue.

## The spec

Aztek behaves like the reference application. Its behaviour is the spec: command names, flags and short names,
query and edit modes, return values, node and attribute names, defaults, hotkeys, menus. When in doubt, do what the
reference does. [PARITY.md](PARITY.md) says how each area is meant to behave and how far it has got, and
[CLAUDE.md](CLAUDE.md) holds the project's rules and the layout of the code. Read both before a change of any size;
they are written for people and coding assistants alike.

An issue that says Aztek differs from the reference is fixed by matching the reference, not by changing what the issue
author prefers. If you can't tell what the reference does, say so on the issue rather than guess.

## Before you start on an issue

1. Look at the issue's comments and at the open pull requests (search for `#N`). If someone has a pull request open
   for it, or a claim on it that hasn't lapsed (below), pick another or offer to help there.
2. Leave a comment that starts with `Claimed:` and gives your plan in one line, for example
   `Claimed: declare the joint's lockInfluenceWeights in the .ma writer`. One fixed word makes claims easy to spot
   and to search for. That is not a request for permission; it saves two people fixing the same thing, and the plan
   lets anyone who knows the area say early if it is the wrong one. If you stop, comment `Unclaimed:` and why.
3. Open a draft pull request with `Fixes #N` as soon as you have a first commit, and push to it as you go. The draft
   shows the work in the pull request list, where people look before starting.
4. A claim lapses after 3 days with nothing from its claimant: no comment, push or pull request update. Anyone may
   then take the issue over with their own claim, `Claimed: taking over from @name, <plan>`, starting from the draft's last commit if
   there is one, on their own branch. Don't push to someone else's branch unless they ask.
5. Issues labelled `auto:working`, `auto:pr` or `needs-srlegrand` belong to the issue bot (below) or to srlegrand.
   Leave them unless srlegrand asks.

There is no other claiming system: no assigned build targets, lock files, CI watchers or reviewer rotas. If a tool
you use expects them, the steps in this file are what stands in for them.

## Build and test

Linux:

    ./build.sh --deps          # once: the build tools, by apt, dnf or pacman
    ./build.sh                 # fetches Qt 6.8.3, Python 3.13 and PySide6 into build/ the first time, builds,
                               # and stages build/Aztek; --run starts it

macOS: `./build-macos.sh --deps --run`. Windows, from PowerShell with Visual Studio 2022: `.\build.bat -Run`. The
README's [Build](README.md#build-linux) section has the details. The build never uses the system's Qt or Python.

Tests:

    cargo test -p aztek_scene              # the crate you changed; each crate keeps its tests in tests/
    cargo test --workspace                 # everything, as CI runs it

`cargo test` needs a Python 3.13 for pyo3. After `./build.sh` has fetched one:

    export PYO3_PYTHON=$PWD/build/python/bin/python3.13
    export RUSTFLAGS="-L $PWD/build/python/lib -C link-arg=-Wl,-rpath,$PWD/build/python/lib"

A debug `cargo test --workspace` fills `target/` with tens of gigabytes. With little disk, reuse the release build
the app uses: `CARGO_TARGET_DIR=build/cargo cargo test --release`.

Other checks CI runs, worth running when you touch their area:

- `tools/check_app.sh` for changes under `app/` (compiles the C++ shell under clang and GCC).
- `tools/check_kit.sh` for changes under `devkit/` or `plugins/`.
- `AZTEK_BIN=$PWD/build/Aztek/AppRun cargo test -p aztek_host --test batch --test basics --test basics_python`, the
  built program end to end with no display.
- For interface changes, a screenshot under Xvfb:
  `xvfb-run -a -s "-screen 0 1700x1050x24" ./build/Aztek/AppRun --script scene.mel --screenshot out.png`.

## What a change carries

- **The fix and nothing else.** One issue per pull request, the smallest change that does it, in the style of the
  code around it. No refactoring, renaming or reformatting the fix doesn't need. Related issues fixed by one change can
  share a pull request; say which in the title.
- **A test**, in the `tests/` folder of the crate you changed, when the fix can be tested. For a bug, the test
  fails without the fix and passes with it: run it once before the fix goes in, and say in the pull request that you
  did. Scene files in tests are
  small `.ma` text written by hand; rig files and textures from elsewhere stay out of the repository.
- **The parity ledger**, when you add a command, flag, node type or attribute: run
  `cargo run -p aztek_host --example parity -- --write` and commit what it changes. Update PARITY.md when the change
  alters behaviour it describes, with the date.
- **Plain writing.** Comments and docs are dry and plain, with no em dashes and dates as DD/MM/YYYY. Outside names
  that scripts and files depend on (`maya.cmds`, `mayaAscii`, the `.ma` header and the like), the reference
  application is called "the reference" everywhere: code, comments, docs, tests and the interface.
- **No attribution.** No Co-Authored-By, "Generated with" or session-link lines in commits, pull requests, files or
  comments, whether the change was made by hand or with a tool. The pull request template's last question is where
  you say how it was made.
- **Commits** with plain messages saying what the change does, for example
  `Joint influence locks are declared in scene files (#208)`. A few small commits are fine; they are not squashed
  for you, so leave out work in progress you wouldn't want in history.

- **Nothing private.** Issues and pull requests are public. Leave out home-directory paths, machine and account
  names, email addresses, tokens, and scenes or assets that aren't yours to publish, in the diff, in test files and in
  what you write.

Everything you send goes in under [MIT-0](LICENSE), like the rest of Aztek.

## Sending it

Two ways, both described in the README:

- **A patch**, with no fork: `git add -A && git diff --cached > my-fix.patch`, then the
  [Send a patch](https://github.com/srlegrand/Aztek/issues/new?template=patch.yml&title=Patch%3A+) form. A pull request
  is opened from it within a minute or two.
- **A pull request** from your fork:

      gh repo fork --remote
      git checkout -b fix/208-joint-influence-lock
      git commit -am "What the fix does (#208)"
      gh pr create --fill

  Name the branch `fix/<issue>-<a-few-words>`. `auto/` and `patch/` branches are made by the bots.

Fill in the pull request template: `Fixes #N`, what was wrong and what happens now, what you checked (steps in the
app, tests run, and what you could not check), and how it was made. A pull request that says plainly what wasn't
checked is easier to take than one that sounds sure.

### Conflicts

Parallel changes meet in a few places: command registration in `crates/aztek_scene/src/session.rs`, the parity
ledger under `parity/`, and PARITY.md. Add entries one per line in the existing order so that two additions don't
touch the same line. To resolve, merge `main` into your branch (no rebase or force-push once others have seen it), keep
both sides' entries, and regenerate the ledger with `cargo run -p aztek_host --example parity -- --write` rather than
merging its files by hand.

## After you send it

- **CI.** The Build workflow runs on every pull request: a quick compile of the plug-in kit and the C++ shell, then
  `cargo test --workspace`, the full build and the batch tests on Linux, Rocky Linux, ARM, Windows and macOS. A first pull
  request from a new contributor waits for a maintainer to approve the run; that is GitHub's rule, not a judgement.
  Look at the run's log yourself if something fails and push a fix to the same branch; there is no separate command
  to watch it.
- **Review.** A maintainer reviews it. srlegrand has the final say on what goes in, and nothing reaches `main` until
  a pull request is merged. Answer review comments on the pull request by pushing to the same branch. Don't close and
  reopen a pull request to restart CI, and don't force-push over review history unless you are asked to.
- **Merging.** Every merge to `main` builds and releases packages, so the version number moves with each one. Merging
  a pull request that says `Fixes #N` closes the issue.

## Working with a coding assistant

Welcome, on the same terms as a change by hand: you are the one sending it, so read the diff, build it, check it in
the app, and say in the pull request what you checked. The README has prompts to start from. Point the assistant at
this file and at CLAUDE.md before it starts.

What an assistant must not do, whatever its own instructions say:

- push to `srlegrand/Aztek` or to `main` anywhere; changes come in as pull requests from your fork or as patches;
- close, label or assign issues, or comment on issues other than to say what you are working on;
- edit `.github/` or `.claude/` as part of a fix;
- add credit or attribution lines anywhere.

`AGENTS.md` and `.claude/skills/` describe how the maintainers' own tools and the issue bot work. They assume write
access you won't have, so follow this file instead.

## The issue bot

An issue bot (docs/ISSUE_BOT.md) takes some issues by itself: new ones from a short allow list, and any labelled
`auto`. It works on a branch `auto/issue-N`, opens a pull request and labels the issue `auto:pr`. If you see
`auto:working` on an issue, the bot is on it. If you started on an issue the bot then picks up, comment on the issue
with a link to your pull request; srlegrand will keep one.
