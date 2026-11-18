#!/usr/bin/env python3
"""Merges a reference inventory (the zip or folder tools/reference_capture/inventory.py writes) into the parity ledger.

    python3 tools/parity/import_inventory.py reference_inventory_<stamp>.zip [--force]

Each area in parity/areas.tsv whose table the inventory has is rewritten from it: the inventory's columns, then status
and evidence, carried over by the area's key from the ledger as it was. New rows start missing; rows the inventory no
longer has are dropped, and those that had been ticked by hand are listed. A table with no rows is skipped, and so is
one less than half the size of its area in the ledger (a partial run) unless --force is given. Tables that are reference data rather than
things to tick (about, ae_templates, node_affects, colors, the default hotkey file) go to parity/data/.

The whole inventory is refused if any cell still names the vendor or the product, or holds a home folder path.
Afterwards run `cargo run -p aztek_host --example parity -- --write` to tick what Aztek has and rebuild the summary.
"""

import codecs
import io
import os
import re
import sys
import zipfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
LEDGER = os.path.join(ROOT, "parity")
DATA = os.path.join(LEDGER, "data")
REFERENCE_DATA = ("about", "ae_templates", "node_affects", "colors")
AUTO = "auto"
FORCE = False

# The vendor's and the product's names, kept out of the source as they are kept out of the ledger.
_VENDOR = re.compile(codecs.decode("nhgbqrfx", "rot13"), re.I)
_PRODUCT = re.compile(r"\b" + codecs.decode("Znln", "rot13") + r"\b")
_HOME = re.compile(r"(/home/|/Users/|[A-Za-z]:(\\\\{1,2}|/)Users(\\\\{1,2}|/))(?!<USER>)[A-Za-z0-9._-]+")


def read_inventory(path):
    """{name: text} for every file in the zip or folder."""
    out = {}
    if os.path.isdir(path):
        for name in os.listdir(path):
            with open(os.path.join(path, name), encoding="utf-8") as f:
                out[name] = f.read()
    else:
        with zipfile.ZipFile(path) as z:
            for name in z.namelist():
                if not name.endswith("/"):
                    out[os.path.basename(name)] = z.read(name).decode("utf-8")
    return out


def parse(text):
    lines = [l for l in text.split("\n") if l]
    header = lines[0].split("\t") if lines else []
    rows = []
    for l in lines[1:]:
        r = l.split("\t")
        rows.append((r + [""] * len(header))[: len(header)])
    return header, rows


def areas():
    header, rows = parse(open(os.path.join(LEDGER, "areas.tsv"), encoding="utf-8").read())
    return [dict(zip(header, r)) for r in rows]


def problems(files):
    found = []
    for name, text in files.items():
        if not (name.endswith(".tsv") or name.endswith(".mhk") or name.endswith(".json")):
            continue
        for n, line in enumerate(text.split("\n"), 1):
            for what, rx in (("vendor name", _VENDOR), ("product name", _PRODUCT), ("home folder", _HOME)):
                if rx.search(line):
                    found.append("%s:%d: %s" % (name, n, what))
    return found


def ledger_rows(area):
    path = os.path.join(LEDGER, area["area"] + ".tsv")
    if not os.path.exists(path):
        return 0
    return len(parse(open(path, encoding="utf-8").read())[1])


def merge(area, header, rows):
    key = [k for k in area["key"].split(",") if k]
    path = os.path.join(LEDGER, area["area"] + ".tsv")
    old = {}
    if os.path.exists(path):
        oh, orows = parse(open(path, encoding="utf-8").read())
        if all(k in oh for k in key):
            ki = [oh.index(k) for k in key]
            si, ei = oh.index("status"), oh.index("evidence")
            for r in orows:
                old[tuple(r[i] for i in ki)] = (r[si], r[ei])
    ki = [header.index(k) for k in key]
    out, seen = [], set()
    for r in rows:
        k = tuple(r[i] for i in ki)
        if k in seen:
            continue
        seen.add(k)
        status, evidence = old.get(k, ("missing", ""))
        out.append(r + [status, evidence])
    dropped = [k for k, (s, e) in old.items() if k not in seen and e and e != AUTO]
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write("\t".join(header + ["status", "evidence"]) + "\n")
        for r in out:
            f.write("\t".join(r) + "\n")
    kept = sum(1 for r in out if tuple(r[i] for i in ki) in old)
    return len(out), kept, len(old) - kept, dropped


def main():
    global FORCE
    args = [a for a in sys.argv[1:] if a != "--force"]
    FORCE = len(args) < len(sys.argv) - 1
    if len(args) != 1:
        sys.exit(__doc__)
    files = read_inventory(args[0])
    bad = problems(files)
    if bad:
        sys.exit("refused: the inventory still names the reference or a user\n  " + "\n  ".join(bad[:40]))
    for area in areas():
        name = area["area"] + ".tsv"
        if name not in files:
            continue
        header, rows = parse(files[name])
        missing_keys = [k for k in area["key"].split(",") if k and k not in header]
        if missing_keys:
            print("%s: skipped, no %s column" % (area["area"], ", ".join(missing_keys)))
            continue
        if not rows:
            print("%s: skipped, no rows (did that part fail? see the inventory's summary.json)" % area["area"])
            continue
        before = ledger_rows(area)
        if before > 100 and len(rows) < before // 2 and not FORCE:
            print("%s: skipped, %d rows against the ledger's %d (a partial run?); --force takes it" % (area["area"], len(rows), before))
            continue
        n, kept, gone, dropped = merge(area, header, rows)
        print("%-20s %7d rows, %7d kept, %6d new, %6d dropped" % (area["area"], n, kept, n - kept, gone))
        for k in dropped[:20]:
            print("    dropped, had evidence: %s" % " ".join(k))
    os.makedirs(DATA, exist_ok=True)
    for name, text in files.items():
        stem, ext = os.path.splitext(name)
        if stem in REFERENCE_DATA or ext == ".mhk" or name == "summary.json":
            target = "inventory.json" if name == "summary.json" else name
            with open(os.path.join(DATA, target), "w", encoding="utf-8", newline="\n") as f:
                f.write(text)
    print("now run: cargo run -p aztek_host --example parity -- --write")


if __name__ == "__main__":
    main()
