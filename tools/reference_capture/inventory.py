# Reference inventory: lists everything the reference application exposes, so Aztek's parity ledger can hold a row for
# each thing it has to match. Names and interfaces only: commands and their flags, node types and their attributes,
# the scripting APIs and their members, the C++ headers' classes and methods, environment variables, preferences,
# menus, hotkeys, runtime commands, shelves, panels, tools, file translators and plug-ins.
#
# Run it in the reference, in a Python tab of the Script Editor (menus and hotkeys need the full interface):
#
#     exec(open("/path/to/inventory.py").read())
#
# or without the interface (menus, hotkeys and shelves are then skipped):
#
#     mayapy inventory.py [output folder]
#
# It writes a folder of tab-separated files, one row per item, and a zip of it beside the folder, in your home folder
# unless you name another. Product and vendor names are replaced in every text field; identifiers (command, node,
# attribute, variable names) are kept as they are. Paths are written as <INSTALL>, <HOME>, <APPDIR>, <TEMP>, and your
# user name as <USER>. Nothing is sent anywhere: send back the zip. In Aztek's repository,
# `python3 tools/parity/import_inventory.py <zip>` merges it into the parity ledger (parity/README.md).
#
# Best run with fresh preferences, so the preference values are the defaults: start the application once with the
# environment variable for its application folder pointed at an empty folder.
#
# Settings, as environment variables before running:
#   INVENTORY_PLUGINS=0    don't load the bundled plug-ins (their nodes and commands are then missed)
#   INVENTORY_AFFECTS=0    skip which attributes affect which (the slowest part)
#   INVENTORY_VALUES=0     leave out environment and preference values, names only
#   INVENTORY_DEVKIT=path  the developer kit's folder, if the C++ headers aren't in the install
#   INVENTORY_ONLY=a,b     run only these parts (names as in PARTS below)

import codecs
import os
import re
import sys
import time
import json
import glob
import getpass
import platform
import tempfile
import traceback
import zipfile

_standalone = False
try:
    import maya.standalone

    maya.standalone.initialize(name="python")
    _standalone = True
except Exception:
    pass

import maya.cmds as cmds
import maya.mel as mel

FORMAT = 1
GUI = not cmds.about(batch=True)
ENV = os.environ.get


def flag(name, default=True):
    v = ENV(name)
    return default if v is None else v not in ("0", "false", "no", "")


LOAD_PLUGINS = flag("INVENTORY_PLUGINS")
AFFECTS = flag("INVENTORY_AFFECTS")
VALUES = flag("INVENTORY_VALUES")
ONLY = [p for p in (ENV("INVENTORY_ONLY") or "").split(",") if p]

INSTALL = ENV("MAYA_LOCATION") or os.path.dirname(os.path.dirname(sys.executable))
HOME = os.path.expanduser("~")
APPDIR = ENV("MAYA_APP_DIR") or ""
TEMP = tempfile.gettempdir()
USER = getpass.getuser() or ""

# ---------------------------------------------------------------------------------------------------------------------
# Anonymising

# The vendor's name, kept out of the source as it is kept out of the output.
_VENDOR = re.compile(codecs.decode("nhgbqrfx", "rot13"), re.I)
_PRODUCT = re.compile(r"(?<![A-Za-z0-9_])maya(?![A-Za-z0-9_])", re.I)


_HOMES = re.compile(r"(/home/|/Users/|[A-Za-z]:[\\/]Users[\\/])(?!<)[^/\\:;\s\t]+")


def paths(s):
    """Install, home, application and temp folders and the user name as placeholders, longest first."""
    for real, token in sorted(((INSTALL, "<INSTALL>"), (APPDIR, "<APPDIR>"), (TEMP, "<TEMP>"), (HOME, "<HOME>")), key=lambda x: -len(x[0])):
        if real and len(real) > 3:
            s = s.replace(real, token).replace(real.replace("\\", "/"), token)
    if USER and len(USER) > 2:
        s = re.sub(r"(?<![A-Za-z0-9])" + re.escape(USER) + r"(?![A-Za-z0-9])", "<USER>", s)
    # Any other home folder (another account's, or the real one when HOME points elsewhere).
    return _HOMES.sub(lambda m: m.group(1) + "<USER>", s)


def text(s):
    """A free-text field (a label, a note, a value): paths as placeholders, product and vendor names replaced. A whole
    word only: identifiers that contain the product's name stay as they are."""
    if s is None:
        return ""
    s = paths(str(s))
    s = _VENDOR.sub("Vendor", s)
    return _PRODUCT.sub("Ref", s)


def ident(s):
    """An identifier: kept, but no paths or user name in it."""
    return "" if s is None else paths(str(s))


def cell(v):
    if v is None:
        return ""
    if isinstance(v, bool):
        return "1" if v else "0"
    if isinstance(v, (list, tuple)):
        v = " ".join(cell(x) for x in v)
    return str(v).replace("\\", "\\\\").replace("\t", "\\t").replace("\r", "\\r").replace("\n", "\\n")


# ---------------------------------------------------------------------------------------------------------------------
# Output

OUT = None
COUNTS = {}
ERRORS = {}


class Table:
    """One tab-separated file: a header row, then one row per item, written as it goes."""

    def __init__(self, name, columns):
        self.name = name
        self.columns = columns
        self.f = open(os.path.join(OUT, name + ".tsv"), "w", encoding="utf-8", newline="\n")
        self.f.write("\t".join(columns) + "\n")
        self.n = 0

    def row(self, *values):
        assert len(values) == len(self.columns), (self.name, values)
        self.f.write("\t".join(cell(v) for v in values) + "\n")
        self.n += 1

    def close(self):
        self.f.close()
        COUNTS[self.name] = self.n


def say(msg):
    sys.stdout.write("[inventory] %s\n" % msg)
    sys.stdout.flush()


def safe(fn, *a, **k):
    try:
        return fn(*a, **k)
    except Exception:
        return None


def progress(i, n, what, every=200):
    if i % every == 0:
        say("%s %d/%d" % (what, i, n))


# ---------------------------------------------------------------------------------------------------------------------
# Parts


def part_about():
    t = Table("about", ["key", "value"])
    for key in ("apiVersion", "majorVersion", "minorVersion", "version", "cutIdentifier", "operatingSystem", "operatingSystemVersion", "qtVersion", "uiLanguage", "codeset"):
        t.row(key, text(safe(cmds.about, **{key: True})))
    t.row("python", sys.version.split()[0])
    t.row("platform", platform.platform())
    t.row("interface", GUI)
    t.row("format", FORMAT)
    for k, v in (("linear", "linear"), ("angle", "angle"), ("time", "time")):
        t.row("unit." + k, safe(cmds.currentUnit, q=True, **{v: True}))
    t.close()


def part_plugins():
    """Every plug-in in the install's plug-in folders: loaded first (unless asked not to), so what they register is in
    the other parts too."""
    t = Table("plugins", ["plugin", "loaded", "version", "commands", "nodes", "contexts", "translators", "error"])
    found = []
    for d in (ENV("MAYA_PLUG_IN_PATH") or "").split(os.pathsep):
        if not d or not os.path.isdir(d) or not _inside_install(d):
            continue
        for f in sorted(os.listdir(d)):
            stem, ext = os.path.splitext(f)
            if ext.lower() in (".so", ".mll", ".bundle", ".py", ".pyc", ".dylib") and stem not in found:
                found.append(stem)
    for i, p in enumerate(found):
        progress(i, len(found), "plug-ins", 20)
        err = ""
        if LOAD_PLUGINS and not safe(cmds.pluginInfo, p, q=True, loaded=True):
            try:
                cmds.loadPlugin(p, quiet=True)
            except Exception as e:
                err = text(str(e).strip().splitlines()[0] if str(e).strip() else type(e).__name__)
        loaded = bool(safe(cmds.pluginInfo, p, q=True, loaded=True))
        q = lambda **kw: (safe(cmds.pluginInfo, p, q=True, **kw) or []) if loaded else []
        t.row(ident(p), loaded, text(safe(cmds.pluginInfo, p, q=True, version=True) if loaded else ""), sorted(q(command=True)), sorted(q(dependNode=True)), sorted(q(tool=True)), sorted(q(translator=True)), err)
    t.close()


def _help(name):
    try:
        return cmds.help(name) or ""
    except Exception:
        return ""


_FLAG = re.compile(r"^\s+-(\S+)\s+-(\S+)(.*)$")


def part_commands():
    """Each command and its flags as its help lists them, with what kind of command it is."""
    names = sorted(set(mel.eval('help -list "*"') or []))
    cmd = Table("commands", ["command", "kind", "queryable", "editable", "flags"])
    flg = Table("command_flags", ["command", "flag", "short", "arguments", "notes"])
    for i, name in enumerate(names):
        progress(i, len(names), "commands", 300)
        kind = text(safe(mel.eval, 'whatIs "%s"' % name) or "")
        h = _help(name)
        n = 0
        longs = set()
        for line in h.splitlines():
            m = _FLAG.match(line)
            if not m:
                continue
            short, long_, rest = m.group(1), m.group(2), m.group(3).strip()
            notes = " ".join(re.findall(r"\(([^)]*)\)", rest))
            args = re.sub(r"\([^)]*\)", "", rest).strip()
            flg.row(name, long_, short, args, text(notes))
            longs.add(long_)
            n += 1
        cmd.row(name, kind, "query" in longs, "edit" in longs, n)
    cmd.close()
    flg.close()
    py = Table("python_commands", ["name", "in_mel"])
    mel_names = set(names)
    for name in sorted(n for n in dir(cmds) if not n.startswith("_")):
        py.row(name, name in mel_names)
    py.close()


_PROC = re.compile(r"^\s*global\s+proc\s+(?:([\w\[\]]+)\s+)?(\w+)\s*\(([^)]*)\)", re.M)


def _inside_install(d):
    return os.path.normcase(os.path.abspath(d)).startswith(os.path.normcase(os.path.abspath(INSTALL)))


def _install_script_dirs():
    out = []
    for d in (ENV("MAYA_SCRIPT_PATH") or "").split(os.pathsep):
        if d and os.path.isdir(d) and _inside_install(d):
            out.append(d)
    for d in (os.path.join(INSTALL, "scripts"),):
        if os.path.isdir(d) and d not in out:
            out.append(d)
    return out


def part_procs():
    """The global procedures of the install's own scripts: name, return type, parameters, which file. Their names are
    what scripts and menus call."""
    t = Table("mel_procs", ["proc", "returns", "parameters", "file"])
    seen = set()
    files = []
    for d in _install_script_dirs():
        for root, _, fs in os.walk(d):
            files += [os.path.join(root, f) for f in fs if f.endswith(".mel")]
    for i, path in enumerate(sorted(set(files))):
        progress(i, len(files), "script files", 500)
        try:
            with open(path, encoding="utf-8", errors="replace") as f:
                src = f.read()
        except Exception:
            continue
        for m in _PROC.finditer(src):
            if m.group(2) in seen:
                continue
            seen.add(m.group(2))
            params = ", ".join(" ".join(p.split()) for p in m.group(3).split(",") if p.strip())
            t.row(m.group(2), m.group(1) or "", params, ident(os.path.basename(path)))
    t.close()
    # The AE templates and option-box procedures are found by name above; the template files say which node types have
    # a layout of their own.
    ae = Table("ae_templates", ["node_type"])
    for p in sorted(set(files)):
        b = os.path.basename(p)
        if b.startswith("AE") and b.endswith("Template.mel"):
            ae.row(b[2:-len("Template.mel")])
    ae.close()


def part_nodes():
    """Each node type, what it derives from, what it is, and its own attributes (not those it inherits) with their
    types, flags, ranges, defaults and enum fields; then which attributes affect which."""
    import maya.api.OpenMaya as om

    types = sorted(set(cmds.allNodeTypes(includeAbstract=True) or []))
    nt = Table("node_types", ["node_type", "abstract", "parent", "inherits", "classification", "plugin"])
    at = Table("node_attributes", ["node_type", "attribute", "short", "parent", "kind", "data", "array", "flags", "default", "min", "max", "soft_min", "soft_max", "enum", "nice"])
    af = Table("node_affects", ["node_type", "attribute", "affects"]) if AFFECTS else None
    plugin_of = {}
    for p in safe(cmds.pluginInfo, q=True, listPlugins=True) or []:
        for n in safe(cmds.pluginInfo, p, q=True, dependNode=True) or []:
            plugin_of[n] = p
    names_cache = {}

    def attr_names(t):
        """The names of every attribute a type has, its inherited ones included."""
        if t not in names_cache:
            got = set()
            for a in _class_attrs(om, t):
                n = safe(lambda: om.MFnAttribute(a).name)
                if n:
                    got.add(n)
            names_cache[t] = got
        return names_cache[t]

    for i, raw in enumerate(types):
        progress(i, len(types), "node types", 100)
        name = raw.split(" ")[0]
        abstract = raw.endswith("(abstract)")
        chain = safe(cmds.nodeType, name, inherited=True, isTypeName=True) or []
        parent = chain[-2] if len(chain) > 1 else ""
        cls = safe(cmds.getClassification, name) or []
        nt.row(name, abstract, parent, chain[:-1], text(cls[0] if cls else ""), ident(plugin_of.get(name, "")))
        mine = []
        for a in _class_attrs(om, name):
            try:
                fa = om.MFnAttribute(a)
            except Exception:
                continue
            mine.append((fa.name, a, fa))
        inherited = attr_names(parent) if parent else set()
        for attr_name, a, fa in mine:
            if attr_name in inherited:
                continue
            at.row(name, attr_name, *_attr(om, a, fa))
            if af is not None:
                hit = safe(cmds.affects, attr_name, t=name) or []
                if hit:
                    af.row(name, attr_name, sorted(hit))
    nt.close()
    at.close()
    if af is not None:
        af.close()


def _class_attrs(om, t):
    """Every attribute of a node type, as objects."""
    try:
        nc = om.MNodeClass(t)
    except Exception:
        return []
    try:
        objs = nc.getAttributes()
        return [objs[k] for k in range(len(objs))]
    except Exception:
        pass
    try:
        return [nc.attribute(k) for k in range(nc.attributeCount)]
    except Exception:
        return []


def _attr(om, a, fa):
    """short, parent, kind, data, array, flags, default, min, max, soft_min, soft_max, enum, nice"""
    parent = ""
    try:
        if not fa.parent.isNull():
            parent = om.MFnAttribute(fa.parent).name
    except Exception:
        pass
    kind = a.apiTypeStr
    data = default = lo = hi = slo = shi = enum = ""
    try:
        if a.hasFn(om.MFn.kNumericAttribute):
            n = om.MFnNumericAttribute(a)
            data = _numeric_name(om, n.numericType())
            default = safe(lambda: n.default)
            lo = safe(n.getMin) if safe(n.hasMin) else ""
            hi = safe(n.getMax) if safe(n.hasMax) else ""
            slo = safe(n.getSoftMin) if safe(n.hasSoftMin) else ""
            shi = safe(n.getSoftMax) if safe(n.hasSoftMax) else ""
        elif a.hasFn(om.MFn.kEnumAttribute):
            e = om.MFnEnumAttribute(a)
            data = "enum"
            default = safe(lambda: e.default)
            fields = []
            for v in range(safe(e.getMin) or 0, (safe(e.getMax) or -1) + 1):
                f = safe(e.fieldName, v)
                if f:
                    fields.append("%s=%d" % (f, v))
            enum = ":".join(fields)
        elif a.hasFn(om.MFn.kTypedAttribute):
            data = _data_name(om, om.MFnTypedAttribute(a).attrType())
        elif a.hasFn(om.MFn.kUnitAttribute):
            u = om.MFnUnitAttribute(a)
            data = {0: "invalid", 1: "angle", 2: "distance", 3: "time"}.get(u.unitType(), str(u.unitType()))
            default = safe(lambda: u.default.value)
            lo = safe(lambda: u.getMin().value) if safe(u.hasMin) else ""
            hi = safe(lambda: u.getMax().value) if safe(u.hasMax) else ""
        elif a.hasFn(om.MFn.kMatrixAttribute):
            data = "matrix"
        elif a.hasFn(om.MFn.kMessageAttribute):
            data = "message"
        elif a.hasFn(om.MFn.kCompoundAttribute):
            data = "compound:%d" % om.MFnCompoundAttribute(a).numChildren()
        elif a.hasFn(om.MFn.kGenericAttribute):
            data = "generic"
    except Exception:
        pass
    flags = []
    for f in ("readable", "writable", "storable", "keyable", "channelBox", "hidden", "connectable", "cached", "internal", "affectsAppearance", "affectsWorldSpace", "usedAsColor", "usedAsFilename", "indeterminant", "disconnectBehavior", "indexMatters", "renderSource", "worldSpace", "usesArrayDataBuilder", "isProxyAttribute"):
        v = safe(lambda: getattr(fa, f))
        if v is True:
            flags.append(f)
        elif f == "disconnectBehavior" and v not in (None, False):
            flags.append("disconnect=%s" % v)
    if isinstance(default, (list, tuple)):
        default = ",".join(str(x) for x in default)
    nice = safe(lambda: fa.niceName) or ""
    return fa.shortName, parent, kind, data, bool(fa.array), flags, default, lo, hi, slo, shi, enum, text(nice)


def _numeric_name(om, t):
    for n in dir(om.MFnNumericData):
        if n.startswith("k") and getattr(om.MFnNumericData, n) == t:
            return n[1:]
    return str(t)


def _data_name(om, t):
    for n in dir(om.MFnData):
        if n.startswith("k") and getattr(om.MFnData, n) == t:
            return n[1:]
    return str(t)


API_MODULES = [
    "maya.api.OpenMaya",
    "maya.api.OpenMayaAnim",
    "maya.api.OpenMayaUI",
    "maya.api.OpenMayaRender",
    "maya.api.OpenMayaFX",
    "maya.OpenMaya",
    "maya.OpenMayaAnim",
    "maya.OpenMayaUI",
    "maya.OpenMayaRender",
    "maya.OpenMayaFX",
    "maya.OpenMayaMPx",
]


def part_python_api():
    """The scripting APIs: each class, its bases, and each member with its kind; numbers for constants and enums; the
    first line of a method's docstring where it gives the call's shape."""
    import importlib
    import inspect

    cl = Table("api_python_classes", ["module", "class", "bases"])
    mb = Table("api_python_members", ["module", "class", "member", "kind", "value", "signature"])
    for mod_name in API_MODULES:
        mod = safe(importlib.import_module, mod_name)
        if mod is None:
            continue
        short = mod_name.replace("maya.api.", "api2.").replace("maya.", "api1.")
        for cname in sorted(dir(mod)):
            if cname.startswith("_"):
                continue
            c = getattr(mod, cname, None)
            if inspect.isclass(c):
                cl.row(short, cname, [b.__name__ for b in c.__bases__ if b is not object])
                own = c.__dict__
                for m in sorted(own):
                    if m.startswith("__") and m not in ("__init__", "__getitem__", "__setitem__", "__len__", "__iter__", "__add__", "__sub__", "__mul__", "__eq__", "__contains__"):
                        continue
                    if m.startswith("_") and not m.startswith("__"):
                        continue
                    v = own[m]
                    kind, value, sig = _member(v, c, m)
                    mb.row(short, cname, m, kind, value, text(sig))
            elif not inspect.ismodule(c):
                kind, value, sig = _member(c, None, cname)
                mb.row(short, "", cname, kind, value, text(sig))
    cl.close()
    mb.close()


def _member(v, owner, name):
    if isinstance(v, (bool, int, float)):
        return "constant", v, ""
    if isinstance(v, str):
        return "constant", text(v), ""
    if isinstance(v, staticmethod):
        return "static", "", _sig(v.__func__)
    if isinstance(v, classmethod):
        return "classmethod", "", _sig(v.__func__)
    if isinstance(v, property) or type(v).__name__ in ("getset_descriptor", "member_descriptor"):
        return "property", "", ""
    if callable(v):
        return "method", "", _sig(v)
    return type(v).__name__, "", ""


def _sig(f):
    doc = getattr(f, "__doc__", None) or ""
    first = doc.strip().splitlines()[0] if doc.strip() else ""
    return first if "(" in first and len(first) < 300 else ""


_CLASS = re.compile(r"\bclass\s+(?:\w+_EXPORT\s+|\w+API\s+)*(\w+)\s*(?::\s*([^{;]+))?\{")
_ENUM = re.compile(r"\benum\s+(?:class\s+)?(\w*)\s*(?::\s*\w+\s*)?\{([^}]*)\}")


def _strip_comments(src):
    src = re.sub(r"/\*.*?\*/", " ", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def part_cpp_api():
    """The C++ headers: each class and what it derives from, each public declaration in it, each enum and its
    enumerators. Read as text, without a compiler, so an odd declaration may be split oddly; the names come through."""
    roots = [os.path.join(INSTALL, "include"), os.path.join(INSTALL, "devkit", "include")]
    if ENV("INVENTORY_DEVKIT"):
        roots.insert(0, os.path.join(ENV("INVENTORY_DEVKIT"), "include"))
        roots.insert(0, ENV("INVENTORY_DEVKIT"))
    headers = []
    root = ""
    for r in roots:
        if os.path.isdir(r):
            headers += glob.glob(os.path.join(r, "maya", "**", "*.h"), recursive=True)
            if headers:
                root = r
                break
    cl = Table("api_cpp_classes", ["header", "class", "bases"])
    mb = Table("api_cpp_members", ["header", "class", "access", "declaration"])
    en = Table("api_cpp_enums", ["header", "class", "enum", "enumerators"])
    for i, h in enumerate(sorted(set(headers))):
        progress(i, len(headers), "headers", 200)
        rel = os.path.relpath(h, root).replace("\\", "/")
        try:
            with open(h, encoding="utf-8", errors="replace") as f:
                src = _strip_comments(f.read())
        except Exception:
            continue
        for m in _CLASS.finditer(src):
            name, bases = m.group(1), (m.group(2) or "").strip()
            body, end = _block(src, m.end() - 1)
            cl.row(rel, name, " ".join(bases.split()))
            access = "private"
            for stmt in _statements(body):
                a = re.match(r"^(public|protected|private)\s*:\s*(.*)$", stmt, re.S)
                if a:
                    access = a.group(1)
                    stmt = a.group(2).strip()
                    if not stmt:
                        continue
                if access == "private" or not stmt or stmt.startswith(("friend", "typedef", "using")):
                    continue
                d = " ".join(stmt.split())
                if "(" in d or re.match(r"^(static\s+)?(const\s+)?[\w:<>,\s\*&]+\s+\w+(\[\w*\])?$", d):
                    mb.row(rel, name, access, d)
            for e in _ENUM.finditer(body):
                items = [x.split("=")[0].strip() for x in e.group(2).split(",") if x.strip()]
                en.row(rel, name, e.group(1), items)
        for e in _ENUM.finditer(src):
            if not any(m.start() < e.start() < _block(src, m.end() - 1)[1] for m in _CLASS.finditer(src)):
                items = [x.split("=")[0].strip() for x in e.group(2).split(",") if x.strip()]
                en.row(rel, "", e.group(1), items)
    cl.close()
    mb.close()
    en.close()
    if not headers:
        ERRORS["cpp_api"] = "no C++ headers found: set INVENTORY_DEVKIT to the developer kit's folder"


def _block(src, open_at):
    depth = 0
    for k in range(open_at, len(src)):
        if src[k] == "{":
            depth += 1
        elif src[k] == "}":
            depth -= 1
            if depth == 0:
                return src[open_at + 1 : k], k
    return src[open_at + 1 :], len(src)


def _statements(body):
    """The top-level declarations of a class body: up to ; or to the end of an inline body, nested classes skipped."""
    out, cur, depth = [], [], 0
    for ch in body:
        if ch == "{":
            depth += 1
            if depth == 1:
                cur.append(" {...}")
            continue
        if ch == "}":
            depth -= 1
            if depth == 0:
                out.append("".join(cur).strip())
                cur = []
            continue
        if depth:
            continue
        if ch == ";":
            out.append("".join(cur).strip())
            cur = []
        elif ch == ":" and re.search(r"(public|protected|private)\s*$", "".join(cur)):
            cur.append(":")
            out.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    return [s for s in out if s]


_GETENV = re.compile(r"""(?:getenv\s*\(?\s*|environ(?:\.get)?\s*[\(\[]\s*|getEnv\s*\(\s*)["']([A-Z][A-Z0-9_]{2,})["']""")


def part_environment():
    """Environment variables: those set now, those the environment file sets, and those the install's own scripts read.
    Values with paths as placeholders, or none at all with INVENTORY_VALUES=0."""
    t = Table("environment", ["variable", "set", "in_env_file", "read_by_scripts", "value"])
    env_file = safe(cmds.about, environmentFile=True) or ""
    in_file = set()
    if env_file and os.path.isfile(env_file):
        with open(env_file, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = re.match(r"\s*([A-Za-z_][A-Za-z0-9_]*)\s*=", line)
                if m:
                    in_file.add(m.group(1))
    read = set()
    dirs = _install_script_dirs() + glob.glob(os.path.join(INSTALL, "lib", "python*", "site-packages", "maya")) + glob.glob(os.path.join(INSTALL, "Python", "Lib", "site-packages", "maya"))
    for d in dirs:
        for root, _, fs in os.walk(d):
            for f in fs:
                if f.endswith((".mel", ".py")):
                    try:
                        with open(os.path.join(root, f), encoding="utf-8", errors="replace") as fh:
                            read.update(_GETENV.findall(fh.read()))
                    except Exception:
                        pass
    names = set(os.environ) | in_file | read
    for n in sorted(names):
        v = os.environ.get(n)
        # Only the application's own, and what its scripts read: not the rest of the machine's environment.
        if not (n in in_file or n in read or re.match(r"^(MAYA|MTOA|XGEN|BIFROST|PYTHON|QT|ADSK|LD_|DYLD_|PATH$|TMPDIR$|TEMP$|TMP$|HOME$|USER)", n)):
            continue
        t.row(n, v is not None, n in in_file, n in read, text(v) if (VALUES and v is not None) else "")
    t.close()


def part_preferences():
    """Preference variables (optionVar): name, type, and the value now (the defaults, with fresh preferences)."""
    t = Table("preferences", ["option", "type", "value"])
    for n in sorted(safe(cmds.optionVar, list=True) or []):
        v = safe(cmds.optionVar, q=n)
        kind = "array" if isinstance(v, list) else type(v).__name__
        t.row(n, kind, text(v) if VALUES else "")
    t.close()


def part_runtime():
    """Runtime commands: name, category, annotation, language and the command itself (short scripts calling procedures
    or commands; the call is what Aztek needs to match)."""
    t = Table("runtime_commands", ["runtime_command", "category", "language", "annotation", "command", "default"])
    names = safe(cmds.runTimeCommand, q=True, commandArray=True) or []
    for n in sorted(set(names)):
        q = lambda **kw: safe(cmds.runTimeCommand, n, q=True, **kw)
        t.row(n, text(q(category=True)), q(commandLanguage=True) or "", text(q(annotation=True)), text(q(command=True)), bool(q(default=True)))
    t.close()
    nc = Table("name_commands", ["name_command", "annotation", "command"])
    count = safe(cmds.assignCommand, q=True, numElements=True) or 0
    for i in range(1, count + 1):
        nc.row(safe(cmds.assignCommand, i, q=True, name=True) or "", text(safe(cmds.assignCommand, i, q=True, annotation=True)), text(safe(cmds.assignCommand, i, q=True, command=True)))
    nc.close()


def part_hotkeys():
    """The default hotkey set, exported as the application writes it, and as rows: key, modifiers, press or release, the
    name command it runs."""
    if not GUI:
        ERRORS["hotkeys"] = "needs the interface"
        return
    t = Table("hotkeys", ["key", "ctrl", "alt", "shift", "command_key", "release", "name_command"])
    sets = safe(cmds.hotkeySet, q=True, hotkeySetArray=True) or []
    default = next((s for s in sets if s.lower().endswith("_default")), None)
    current = safe(cmds.hotkeySet, q=True, current=True)
    if default:
        safe(cmds.hotkeySet, default, e=True, current=True)
        tmp = os.path.join(OUT, "hotkeys_default.mhk")
        safe(cmds.hotkeySet, default, e=True, export=tmp)
        if os.path.isfile(tmp):
            with open(tmp, encoding="utf-8", errors="replace") as f:
                src = text(f.read())
            with open(tmp, "w", encoding="utf-8", newline="\n") as f:
                f.write(src)
    keys = [chr(c) for c in range(33, 127)] + ["Up", "Down", "Left", "Right", "Home", "End", "Page_Up", "Page_Down", "Insert", "Delete", "Return", "Space", "Backspace", "Tab", "Esc"] + ["F%d" % k for k in range(1, 13)]
    for k in keys:
        for ctl in (False, True):
            for alt in (False, True):
                for sht in (False, True):
                    for rel in (False, True):
                        kw = dict(ctl=ctl, alt=alt, sht=sht)
                        name = safe(cmds.hotkey, keyShortcut=k, q=True, releaseName=True, **kw) if rel else safe(cmds.hotkey, keyShortcut=k, q=True, name=True, **kw)
                        if name:
                            t.row(k, ctl, alt, sht, False, rel, name)
    t.close()
    if current:
        safe(cmds.hotkeySet, current, e=True, current=True)


def part_menus():
    """The main window's menus in every menu set, each built as when opened: label, kind, option box, the command's
    first word (what it calls), and the annotation."""
    if not GUI:
        ERRORS["menus"] = "needs the interface"
        return
    t = Table("menus", ["menu_set", "path", "item", "kind", "option_box", "checked", "calls", "annotation"])
    main = mel.eval("$_inv_tmp = $gMainWindow")
    sets = safe(cmds.menuSet, q=True, allMenuSets=True) or [""]
    common = safe(cmds.window, main, q=True, menuArray=True) or []
    seen = set()
    for ms in sets:
        label = ms
        if ms:
            label = safe(cmds.menuSet, ms, q=True, label=True) or ms
            safe(cmds.menuSet, currentMenuSet=ms)
            safe(mel.eval, 'setMenuMode "%s"' % ms)
        menus = list(common) + list(safe(cmds.menuSet, ms, q=True, menuArray=True) or [] if ms else [])
        for m in menus:
            short = m.split("|")[-1]
            ml = text(safe(cmds.menu, m, q=True, label=True) or short)
            key = (label if short in (safe(cmds.menuSet, ms, q=True, menuArray=True) or []) else "", short)
            if key in seen:
                continue
            seen.add(key)
            _walk_menu(t, text(label) if key[0] else "", ml, m, 0)
    t.close()


def _post(m):
    pmc = safe(cmds.menu, m, q=True, postMenuCommand=True)
    if not pmc:
        return
    try:
        if callable(pmc):
            pmc()
        else:
            mel.eval(pmc)
    except Exception:
        pass


def _calls(cmd):
    if not cmd:
        return ""
    if callable(cmd):
        return getattr(cmd, "__name__", "python")
    s = str(cmd).strip()
    m = re.match(r"^(?:python\s*\(\s*\"?)?\s*([A-Za-z_][\w\.]*)", s)
    return m.group(1) if m else ""


def _walk_menu(t, menu_set, path, m, depth):
    if depth > 6:
        return
    _post(m)
    items = safe(cmds.menu, m, q=True, itemArray=True) or []
    parent = m if "|" in m else m
    for it in items:
        full = parent + "|" + it if not it.startswith(parent) else it
        if not safe(cmds.menuItem, full, q=True, exists=True):
            full = it
        q = lambda **kw: safe(cmds.menuItem, full, q=True, **kw)
        if q(divider=True):
            dl = text(q(dividerLabel=True) or "")
            t.row(menu_set, path, dl, "divider", False, "", "", "")
            continue
        label = text(q(label=True) or it)
        if q(optionBox=True):
            t.row(menu_set, path, label, "option box", True, "", _calls(q(command=True)), text(q(annotation=True)))
            continue
        kind = "submenu" if q(subMenu=True) else "check" if q(checkBox=True) is not None and q(isCheckBox=True) else "radio" if q(isRadioButton=True) else "item"
        checked = q(checkBox=True) if kind == "check" else q(radioButton=True) if kind == "radio" else ""
        t.row(menu_set, path, label, kind, False, checked, _calls(q(command=True)), text(q(annotation=True)))
        if kind == "submenu":
            _walk_menu(t, menu_set, path + " > " + label, full, depth + 1)


def part_shelves():
    """The install's default shelves: shelf, button label, annotation, icon, what the command calls."""
    t = Table("shelves", ["shelf", "label", "annotation", "image", "calls", "language"])
    files = []
    for d in [os.path.join(INSTALL, "scripts", "shelves")] + _install_script_dirs():
        files += glob.glob(os.path.join(d, "shelf_*.mel"))
    for path in sorted(set(files)):
        shelf = os.path.basename(path)[len("shelf_") : -len(".mel")]
        with open(path, encoding="utf-8", errors="replace") as f:
            src = f.read()
        for b in re.split(r"\bshelfButton\b", src)[1:]:
            def opt(n):
                m = re.search(r"-%s\s+\"((?:[^\"\\]|\\.)*)\"" % n, b)
                return m.group(1) if m else ""

            lang = opt("sourceType") or "mel"
            cmd = opt("command").encode("utf-8").decode("unicode_escape", errors="replace") if opt("command") else ""
            t.row(shelf, text(opt("label")), text(opt("annotation")), opt("image"), _calls(cmd), lang)
    t.close()


def part_ui():
    """Panel types, scripted panel types, tool contexts, file translators, workspace file rules, colour palette."""
    t = Table("panels", ["panel_type", "scripted"])
    for p in sorted(set(safe(cmds.getPanel, allTypes=True) or [])):
        t.row(p, False)
    for p in sorted(set(safe(cmds.getPanel, allScriptedTypes=True) or [])):
        t.row(p, True)
    t.close()
    c = Table("tool_contexts", ["context", "type"])
    for x in sorted(set(safe(cmds.lsUI, contexts=True) or [])):
        c.row(x, safe(cmds.contextInfo, x, c=True) or "")
    c.close()
    tr = Table("translators", ["translator", "extension", "filter", "read", "write", "default_options"])
    for x in sorted(set(safe(cmds.translator, q=True, list=True) or [])):
        q = lambda **kw: safe(cmds.translator, x, q=True, **kw)
        tr.row(x, q(extension=True) or "", text(q(filter=True)), bool(q(readSupport=True)), bool(q(writeSupport=True)), text(q(defaultOptions=True)))
    tr.close()
    w = Table("workspace_rules", ["rule", "default"])
    rules = safe(cmds.workspace, q=True, fileRuleList=True) or []
    for r in rules:
        w.row(r, text(safe(cmds.workspace, fileRuleEntry=r)))
    w.close()
    col = Table("colors", ["kind", "name", "value"])
    for i in range(1, 32):
        v = safe(cmds.colorIndex, i, q=True)
        if v:
            col.row("index", i, v)
    for line in safe(cmds.displayRGBColor, list=True) or []:
        parts = line.split()
        if parts:
            col.row("display", parts[0], parts[1:])
    for line in (safe(cmds.displayColor, q=True, list=True) or []) if hasattr(cmds, "displayColor") else []:
        parts = str(line).split()
        if parts:
            col.row("displayIndex", parts[0], parts[1:])
    col.close()


PARTS = [
    ("about", part_about),
    ("plugins", part_plugins),
    ("commands", part_commands),
    ("procs", part_procs),
    ("nodes", part_nodes),
    ("python_api", part_python_api),
    ("cpp_api", part_cpp_api),
    ("environment", part_environment),
    ("preferences", part_preferences),
    ("runtime", part_runtime),
    ("hotkeys", part_hotkeys),
    ("menus", part_menus),
    ("shelves", part_shelves),
    ("ui", part_ui),
]


def run(out=None):
    global OUT
    stamp = time.strftime("%Y%m%d-%H%M%S")
    OUT = out or os.path.join(HOME, "reference_inventory_" + stamp)
    os.makedirs(OUT, exist_ok=True)
    say("writing to " + OUT)
    t0 = time.time()
    for name, fn in PARTS:
        if ONLY and name not in ONLY:
            continue
        say("part: " + name)
        t = time.time()
        try:
            fn()
        except Exception:
            ERRORS[name] = paths(traceback.format_exc())
            say("part %s failed, the others go on" % name)
        say("part %s: %.0fs" % (name, time.time() - t))
    with open(os.path.join(OUT, "summary.json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump({"format": FORMAT, "interface": GUI, "plugins_loaded": LOAD_PLUGINS, "values": VALUES, "rows": COUNTS, "errors": ERRORS, "seconds": round(time.time() - t0)}, f, indent=1, sort_keys=True)
    zpath = OUT.rstrip("/\\") + ".zip"
    with zipfile.ZipFile(zpath, "w", zipfile.ZIP_DEFLATED) as z:
        for f in sorted(os.listdir(OUT)):
            z.write(os.path.join(OUT, f), os.path.join(os.path.basename(OUT), f))
    say("done in %.0fs: %s" % (time.time() - t0, zpath))
    for k in sorted(COUNTS):
        say("  %-22s %d rows" % (k, COUNTS[k]))
    for k in sorted(ERRORS):
        say("  error in %s (see summary.json)" % k)
    return zpath


if __name__ == "__main__":
    run(sys.argv[1] if _standalone and len(sys.argv) > 1 else None)
    if _standalone:
        safe(maya.standalone.uninitialize)
