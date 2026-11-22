//! System and preference commands: makebot, melInfo, melOptions, memory, movIn, movOut, movieInfo,
//! nurbsToPolygonsPref, openCLInfo, openGLExtension and openMayaPref, with the API preferences the plug-in loader and
//! the Python API read.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Map, Value};

use crate::core::{Core, LineKind};

fn f(n: &str, sh: &str, ty: ArgType, doc: &str) -> FlagDef {
    FlagDef::new(n, sh, ty, Value::Null, doc)
}

fn asked(a: &Args) -> Vec<String> {
    a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or(v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|s| matches!(s, "true" | "on" | "yes" | "1")))
}

fn say(text: impl Into<String>, kind: LineKind) {
    crate::native_plugin::defer(text.into(), kind);
}

// ---------------------------------------------------------------------------------------------------------------------
// openMayaPref

static ERRLOG: AtomicBool = AtomicBool::new(false);
static LAZY_LOAD: AtomicBool = AtomicBool::new(false);
static OLD_PLUGIN_WARNING: AtomicBool = AtomicBool::new(true);
static ERRLOG_FILE: Mutex<()> = Mutex::new(());

pub fn errlog_on() -> bool {
    ERRLOG.load(Ordering::Relaxed)
}

pub fn lazy_load() -> bool {
    LAZY_LOAD.load(Ordering::Relaxed)
}

pub fn old_plugin_warning() -> bool {
    OLD_PLUGIN_WARNING.load(Ordering::Relaxed)
}

/// A failed API call, written to OpenMayaErrorLog in the current folder while -errlog is on: what failed, then where
/// it was called from.
pub fn log_api_error(text: &str, stack: &str) {
    if !errlog_on() {
        return;
    }
    let _g = ERRLOG_FILE.lock().unwrap_or_else(|e| e.into_inner());
    let path = std::env::current_dir().unwrap_or_default().join("OpenMayaErrorLog");
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{text}");
        for l in stack.lines().filter(|l| !l.trim().is_empty()) {
            let _ = writeln!(file, "    {l}");
        }
    }
}

/// The warning a plug-in built for an older release gets: an older kit's ABI, or an API version before the reference's.
pub fn old_plugin_text(name: &str, abi: u32, current: u32, api: &str) -> Option<String> {
    if abi < current {
        return Some(format!("// Warning: Plug-in \"{name}\" was built with an older plug-in kit (ABI {abi}, this release has {current}) and may not work properly. //"));
    }
    let year = |t: &str| t.get(..4).and_then(|y| y.parse::<u32>().ok());
    if let (Some(a), Some(r)) = (year(api.trim()), year(aztek_core::REFERENCE_VERSION)) {
        if a > 1990 && a < r {
            return Some(format!("// Warning: Plug-in \"{name}\" was built for an older release ({api}) and may not work properly. //"));
        }
    }
    None
}

fn open_maya_pref(_s: &mut Session, a: &Args) -> CmdResult {
    let prefs: [(&str, &AtomicBool); 3] = [("errlog", &ERRLOG), ("lazyLoad", &LAZY_LOAD), ("oldPluginWarning", &OLD_PLUGIN_WARNING)];
    if a.bool("query") {
        let q = asked(a);
        let Some((_, p)) = prefs.iter().find(|(n, _)| q.iter().any(|x| x == n) || a.get(n).is_some()) else { fail!("openMayaPref: name the preference to query") };
        return Ok(json!(p.load(Ordering::Relaxed)));
    }
    for (n, p) in prefs {
        if let Some(v) = a.get(n).filter(|v| !v.is_null()) {
            p.store(on(v), Ordering::Relaxed);
        }
    }
    Ok(json!(1))
}

// ---------------------------------------------------------------------------------------------------------------------
// openCLInfo

struct ClPrefs {
    limit_minimum_verts: bool,
    min_vertex_buffer: i64,
    double: bool,
}

static CL: Mutex<ClPrefs> = Mutex::new(ClPrefs { limit_minimum_verts: true, min_vertex_buffer: 2000, double: true });

fn opencl_info(_s: &mut Session, a: &Args) -> CmdResult {
    let mut cl = CL.lock().unwrap_or_else(|e| e.into_inner());
    if a.bool("query") {
        let q = asked(a);
        let first = q.first().cloned().or_else(|| ["valid", "limitMinimumVerts", "minVertexBuffer", "supportsDoublePrecision"].iter().find(|n| a.get(n).is_some()).map(|s| s.to_string()));
        return Ok(match first.as_deref() {
            Some("limitMinimumVerts") => json!(cl.limit_minimum_verts),
            Some("minVertexBuffer") => json!(cl.min_vertex_buffer),
            Some("supportsDoublePrecision") => json!(cl.double),
            // Deformers evaluate on the CPU's threads, which are always there.
            _ => json!(true),
        });
    }
    if let Some(v) = a.get("limitMinimumVerts").filter(|v| !v.is_null()) {
        cl.limit_minimum_verts = on(v);
    }
    if let Some(v) = a.get("minVertexBuffer").and_then(Value::as_f64) {
        cl.min_vertex_buffer = v as i64;
    }
    if let Some(v) = a.get("supportsDoublePrecision").filter(|v| !v.is_null()) {
        cl.double = on(v);
    }
    Ok(json!(true))
}

// ---------------------------------------------------------------------------------------------------------------------
// openGLExtension

#[derive(Default)]
struct GpuInfo {
    renderer: String,
    vendor: String,
    version: String,
    extensions: Vec<String>,
}

static GPU: Mutex<Option<GpuInfo>> = Mutex::new(None);

fn vendor_name(id: u32) -> String {
    match id {
        0x10DE => "NVIDIA Corporation".into(),
        0x1002 | 0x1022 => "ATI Technologies Inc.".into(),
        0x8086 => "Intel".into(),
        0x13B5 => "ARM".into(),
        0x5143 => "Qualcomm".into(),
        0x106B => "Apple".into(),
        0x1010 => "Imagination Technologies".into(),
        0x10005 => "Mesa".into(),
        other => format!("0x{other:04X}"),
    }
}

/// The device the views draw with, as the viewport found it.
pub fn set_gpu_info(renderer: &str, vendor_id: u32, version: &str, extensions: &[String]) {
    *GPU.lock().unwrap_or_else(|e| e.into_inner()) = Some(GpuInfo { renderer: renderer.into(), vendor: vendor_name(vendor_id), version: version.into(), extensions: extensions.to_vec() });
}

fn opengl_extension(_s: &mut Session, a: &Args) -> CmdResult {
    let g = GPU.lock().unwrap_or_else(|e| e.into_inner());
    // Before a view has been made nothing is known.
    let Some(g) = g.as_ref() else { return Ok(json!("")) };
    if a.bool("renderer") {
        return Ok(json!(g.renderer));
    }
    if a.bool("vendor") {
        return Ok(json!(g.vendor));
    }
    if a.bool("version") {
        return Ok(json!(g.version));
    }
    let want = a.string("extension", "");
    if want.is_empty() {
        return Ok(json!(g.extensions.join(" ")));
    }
    Ok(json!(if g.extensions.iter().any(|e| *e == want) { want } else { String::new() }))
}

// ---------------------------------------------------------------------------------------------------------------------
// melOptions

pub const DUP_WARNINGS_VAR: &str = "melDuplicateVariableWarnings";

fn mel_options(s: &mut Session, a: &Args) -> CmdResult {
    if a.bool("query") {
        return Ok(json!(s.option_vars.get(DUP_WARNINGS_VAR).is_some_and(on)));
    }
    if let Some(v) = a.get("duplicateVariableWarnings").filter(|v| !v.is_null()) {
        s.option_vars.insert(DUP_WARNINGS_VAR.into(), json!(on(v) as i64));
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// memory

#[derive(Default, Clone, Copy)]
struct Mem {
    physical: f64,
    free: f64,
    heap: f64,
    process_virtual: f64,
    resident: f64,
    page_faults: f64,
    page_reclaims: f64,
    swaps: f64,
    swap_total: f64,
    swap_free: f64,
    commit_limit: f64,
    committed: f64,
}

#[cfg(target_os = "linux")]
fn read_mem() -> Mem {
    let kv = |text: &str, key: &str| text.lines().find(|l| l.starts_with(key)).and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0) * 1024.0;
    let info = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let mut m = Mem {
        physical: kv(&info, "MemTotal:"),
        free: kv(&info, "MemAvailable:"),
        heap: kv(&status, "VmData:"),
        process_virtual: kv(&status, "VmSize:"),
        resident: kv(&status, "VmRSS:"),
        swap_total: kv(&info, "SwapTotal:"),
        swap_free: kv(&info, "SwapFree:"),
        commit_limit: kv(&info, "CommitLimit:"),
        committed: kv(&info, "Committed_AS:"),
        ..Default::default()
    };
    rusage(&mut m);
    m
}

#[cfg(all(unix, not(target_os = "linux")))]
fn read_mem() -> Mem {
    let mut m = Mem::default();
    #[cfg(target_os = "macos")]
    unsafe {
        let mut v: u64 = 0;
        let mut len = std::mem::size_of::<u64>();
        if libc::sysctlbyname(c"hw.memsize".as_ptr(), &mut v as *mut u64 as *mut libc::c_void, &mut len, std::ptr::null_mut(), 0) == 0 {
            m.physical = v as f64;
        }
    }
    rusage(&mut m);
    m
}

#[cfg(unix)]
fn rusage(m: &mut Mem) {
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) } == 0 {
        m.page_faults = ru.ru_majflt as f64;
        m.page_reclaims = ru.ru_minflt as f64;
        m.swaps = ru.ru_nswap as f64;
        if m.resident == 0.0 {
            // ru_maxrss: kilobytes on Linux, bytes on macOS.
            m.resident = ru.ru_maxrss as f64 * if cfg!(target_os = "macos") { 1.0 } else { 1024.0 };
        }
    }
}

#[cfg(windows)]
fn read_mem() -> Mem {
    #[repr(C)]
    struct MemoryStatusEx {
        length: u32,
        load: u32,
        total_phys: u64,
        avail_phys: u64,
        total_page_file: u64,
        avail_page_file: u64,
        total_virtual: u64,
        avail_virtual: u64,
        avail_extended_virtual: u64,
    }
    #[repr(C)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set: usize,
        working_set: usize,
        quota_peak_paged_pool: usize,
        quota_paged_pool: usize,
        quota_peak_non_paged_pool: usize,
        quota_non_paged_pool: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
        private_usage: usize,
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> i32;
        fn GetCurrentProcess() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut ProcessMemoryCounters, cb: u32) -> i32;
    }
    let mut m = Mem::default();
    unsafe {
        let mut st: MemoryStatusEx = std::mem::zeroed();
        st.length = std::mem::size_of::<MemoryStatusEx>() as u32;
        if GlobalMemoryStatusEx(&mut st) != 0 {
            m.physical = st.total_phys as f64;
            m.free = st.avail_phys as f64;
            m.swap_total = st.total_page_file.saturating_sub(st.total_phys) as f64;
            m.swap_free = st.avail_page_file.saturating_sub(st.avail_phys) as f64;
            m.commit_limit = st.total_page_file as f64;
            m.committed = st.total_page_file.saturating_sub(st.avail_page_file) as f64;
            m.process_virtual = (st.total_virtual - st.avail_virtual) as f64;
        }
        let mut pc: ProcessMemoryCounters = std::mem::zeroed();
        pc.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
        if K32GetProcessMemoryInfo(GetCurrentProcess(), &mut pc, pc.cb) != 0 {
            m.resident = pc.working_set as f64;
            m.heap = pc.private_usage as f64;
            m.page_faults = pc.page_fault_count as f64;
        }
    }
    m
}

/// What the process had reserved but not touched at launch: taken off its virtual size by -adjustedVirtualMemory.
static LAUNCH_OFFSET: std::sync::OnceLock<f64> = std::sync::OnceLock::new();

fn launch_offset() -> f64 {
    *LAUNCH_OFFSET.get_or_init(|| {
        let m = read_mem();
        (m.process_virtual - m.resident).max(0.0)
    })
}

const MEM_FLAGS: &[(&str, &str, &str, bool)] = &[
    ("adjustedVirtualMemory", "av", "The process's memory use: its virtual size less what was reserved at launch.", true),
    ("freeMemory", "fr", "Free memory.", true),
    ("heapMemory", "he", "The heap's size.", true),
    ("pageFaults", "pf", "The number of page faults.", false),
    ("pageReclaims", "pr", "The number of page reclaims.", false),
    ("physicalMemory", "phy", "Physical memory.", true),
    ("processVirtualMemory", "pv", "Virtual memory the process has.", true),
    ("swapFree", "swf", "Free swap.", true),
    ("swapLogical", "swl", "Logical swap: memory committed.", true),
    ("swapMax", "swm", "The most swap there can be.", true),
    ("swapPhysical", "swp", "Physical swap.", true),
    ("swapReserved", "swr", "Reserved swap: swap in use.", true),
    ("swapVirtual", "swv", "Virtual swap: the commit limit.", true),
    ("swaps", "sw", "The number of swaps.", false),
];

fn mem_value(m: &Mem, flag: &str) -> f64 {
    match flag {
        "adjustedVirtualMemory" => (m.process_virtual - launch_offset()).max(m.resident),
        "freeMemory" => m.free,
        "heapMemory" => m.heap,
        "pageFaults" => m.page_faults,
        "pageReclaims" => m.page_reclaims,
        "physicalMemory" => m.physical,
        "processVirtualMemory" => m.process_virtual,
        "swapFree" => m.swap_free,
        "swapLogical" => m.committed,
        "swapMax" | "swapPhysical" => m.swap_total,
        "swapReserved" => (m.swap_total - m.swap_free).max(0.0),
        "swapVirtual" => m.commit_limit,
        "swaps" => m.swaps,
        _ => 0.0,
    }
}

fn memory(_s: &mut Session, a: &Args) -> CmdResult {
    let m = read_mem();
    const MB: f64 = 1024.0 * 1024.0;
    if a.bool("summary") {
        let lines: Vec<String> = MEM_FLAGS
            .iter()
            .map(|(n, _, doc, size)| if *size { format!("{n}: {:.3} MB ({})", mem_value(&m, n) / MB, doc.trim_end_matches('.')) } else { format!("{n}: {}", mem_value(&m, n) as i64) })
            .collect();
        for l in &lines {
            say(l.clone(), LineKind::Info);
        }
        return Ok(json!(lines.join("\n")));
    }
    if a.bool("debug") {
        say("memory: no arena allocator is in use; nothing to report.", LineKind::Info);
        return Ok(Value::Null);
    }
    let Some((flag, _, _, size)) = MEM_FLAGS.iter().find(|(n, ..)| a.bool(n)) else { fail!("memory: name what to report, as -freeMemory or -physicalMemory") };
    let mut v = mem_value(&m, flag);
    if *size {
        if a.bool("gigaByte") {
            v /= 1024.0 * MB;
        } else if a.bool("megaByte") {
            v /= MB;
        } else if a.bool("kiloByte") {
            v /= 1024.0;
        }
    }
    if a.bool("asFloat") {
        return Ok(json!(v));
    }
    let v = v.floor();
    if v > i32::MAX as f64 {
        say(format!("// Warning: memory: {v} doesn't fit in an int and was cut to {}; use a unit flag or -asFloat. //", i32::MAX), LineKind::Warning);
        return Ok(json!(i32::MAX));
    }
    Ok(json!(v as i64))
}

// ---------------------------------------------------------------------------------------------------------------------
// nurbsToPolygonsPref

/// The settings: (long, short, kind 0 uint 1 float 2 bool, factory value).
const NTP: &[(&str, &str, u8, f64)] = &[
    ("chordHeight", "cht", 1, 1.0),
    ("chordHeightRatio", "chr", 1, 0.1),
    ("delta3D", "d", 1, 0.1),
    ("edgeSwap", "es", 2, 0.0),
    ("format", "f", 0, 1.0),
    ("fraction", "ft", 1, 0.01),
    ("matchRenderTessellation", "mrt", 0, 0.0),
    ("merge", "m", 0, 0.0),
    ("mergeTolerance", "mt", 1, 0.1),
    ("minEdgeLen", "mel", 1, 0.001),
    ("polyCount", "pc", 0, 200.0),
    ("polyType", "pt", 0, 0.0),
    ("uNumber", "un", 0, 3.0),
    ("uType", "ut", 0, 1.0),
    ("useChordHeight", "uch", 2, 0.0),
    ("useChordHeightRatio", "ucr", 2, 0.0),
    ("vNumber", "vn", 0, 3.0),
    ("vType", "vt", 0, 1.0),
];

static NTP_VALUES: Mutex<Option<Map<String, Value>>> = Mutex::new(None);

fn ntp_file() -> PathBuf {
    crate::user_folder::user_pref_dir().join("nurbsToPolygonsPrefs.json")
}

fn ntp_load() -> Map<String, Value> {
    let saved: Map<String, Value> = std::fs::read_to_string(ntp_file()).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    NTP.iter().map(|(n, _, kind, def)| (n.to_string(), saved.get(*n).cloned().unwrap_or_else(|| ntp_json(*kind, *def)))).collect()
}

fn ntp_json(kind: u8, v: f64) -> Value {
    match kind {
        0 => json!(v as i64),
        1 => json!(v),
        _ => json!(v != 0.0),
    }
}

/// The NURBS to polygons preference, as menu items read it.
pub fn nurbs_to_polygons_pref(name: &str) -> Value {
    let mut g = NTP_VALUES.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(ntp_load).get(name).cloned().unwrap_or(Value::Null)
}

fn nurbs_to_polygons_pref_cmd(_s: &mut Session, a: &Args) -> CmdResult {
    let mut g = NTP_VALUES.lock().unwrap_or_else(|e| e.into_inner());
    let vals = g.get_or_insert_with(ntp_load);
    if a.bool("query") {
        let q = asked(a);
        let Some(n) = q.first().cloned().or_else(|| NTP.iter().find(|(n, ..)| a.get(n).is_some()).map(|(n, ..)| n.to_string())) else { fail!("nurbsToPolygonsPref: name the setting to query") };
        return Ok(vals.get(&n).cloned().unwrap_or(Value::Null));
    }
    let mut changed = false;
    for (n, _, kind, _) in NTP {
        let Some(v) = a.get(n).filter(|v| !v.is_null()) else { continue };
        let x = v.as_f64().or_else(|| v.as_bool().map(|b| b as i64 as f64)).unwrap_or(0.0);
        if *n == "format" && !(0.0..=3.0).contains(&x) {
            fail!("nurbsToPolygonsPref: -format takes 0, 1, 2 or 3");
        }
        if *kind == 0 && x < 0.0 {
            fail!("nurbsToPolygonsPref: -{n} takes a whole number of 0 or more");
        }
        let nv = ntp_json(*kind, x);
        if vals.get(*n) != Some(&nv) {
            vals.insert(n.to_string(), nv);
            changed = true;
        }
    }
    if changed {
        let file = ntp_file();
        if let Some(d) = file.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let _ = std::fs::write(file, Value::Object(vals.clone()).to_string());
        crate::script_jobs::note_event("nurbsToPolygonsPrefsChanged");
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// makebot

/// A block ordered texture: a header, then the image in square tiles of RGBA bytes, left to right and top to bottom,
/// each tile's rows top to bottom. Edge tiles repeat the image's last row and column.
pub const BOT_MAGIC: &[u8; 8] = b"AZTKBOT1";
pub const BOT_TILE: u32 = 32;

pub fn write_bot(img: &image::RgbaImage, out: &Path) -> std::io::Result<u64> {
    let (w, h) = img.dimensions();
    let (tx, ty) = (w.div_ceil(BOT_TILE), h.div_ceil(BOT_TILE));
    let mut data = Vec::with_capacity(24 + (tx * ty * BOT_TILE * BOT_TILE * 4) as usize);
    data.extend_from_slice(BOT_MAGIC);
    for v in [w, h, BOT_TILE, 4] {
        data.extend_from_slice(&v.to_le_bytes());
    }
    for by in 0..ty {
        for bx in 0..tx {
            for y in 0..BOT_TILE {
                for x in 0..BOT_TILE {
                    let px = (bx * BOT_TILE + x).min(w - 1);
                    let py = (by * BOT_TILE + y).min(h - 1);
                    data.extend_from_slice(&img.get_pixel(px, py).0);
                }
            }
        }
    }
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(out, &data)?;
    Ok(u64::from(tx * ty))
}

fn makebot(s: &mut Session, a: &Args) -> CmdResult {
    let verbose = a.bool("verbose");
    let input = a.string("input", "");
    if input.is_empty() {
        fail!("makebot: give the image with -input");
    }
    let expand = |s: &mut Session, p: &str| -> String {
        if Path::new(p).is_absolute() {
            return p.to_string();
        }
        s.run("workspace", json!({"expandName": p})).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_else(|| p.to_string())
    };
    let in_path = PathBuf::from(expand(s, &input));
    if !in_path.is_file() {
        fail!("makebot: {} not found", in_path.display());
    }
    let mut output = a.string("output", "");
    if output.is_empty() {
        output = format!("{}.bot", in_path.file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_else(|| "image".into()));
    }
    let out_path = if Path::new(&output).is_absolute() {
        PathBuf::from(&output)
    } else {
        // A relative output goes in the project's source images folder.
        let rule = s.run("workspace", json!({"fileRuleEntry": "sourceImages"})).ok().and_then(|v| v.as_str().map(str::to_string)).filter(|r| !r.is_empty()).unwrap_or_else(|| "sourceimages".into());
        PathBuf::from(expand(s, &rule)).join(&output)
    };
    let exists = out_path.is_file();
    let check_depends = a.bool("checkdepends");
    let min_res = a.get("checkres").and_then(Value::as_f64).map(|v| v.max(0.0) as u32);
    let note = |t: String| {
        if verbose {
            say(format!("makebot: {t}"), LineKind::Info);
        }
    };
    if check_depends && exists {
        let newer = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
        if let (Some(src), Some(bot)) = (newer(&in_path), newer(&out_path)) {
            if bot >= src {
                note(format!("{} is up to date", out_path.display()));
                return Ok(Value::Null);
            }
        }
    }
    note(format!("reading {}", in_path.display()));
    let img = image::open(&in_path).map_err(|e| aztek_ops::CommandError::new(format!("makebot: {}: {e}", in_path.display())))?.to_rgba8();
    let (w, h) = img.dimensions();
    if let Some(r) = min_res {
        if w.max(h) < r {
            note(format!("{} is {w} x {h}, under {r}: no BOT file made", in_path.display()));
            return Ok(Value::Null);
        }
    }
    if exists && a.bool("nooverwrite") && (check_depends || min_res.is_some()) {
        note(format!("{} is there and is not overwritten", out_path.display()));
        return Ok(Value::Null);
    }
    let tiles = write_bot(&img, &out_path).map_err(|e| aztek_ops::CommandError::new(format!("makebot: {}: {e}", out_path.display())))?;
    note(format!("wrote {} ({w} x {h}, {tiles} tiles of {BOT_TILE} x {BOT_TILE})", out_path.display()));
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// movieInfo

#[derive(Default, Debug)]
struct Movie {
    quicktime: bool,
    frames: i64,
    width: i64,
    height: i64,
    /// The timecode track: its flags, time scale, frame duration, frames per second and first frame number.
    tc: Option<(u32, u32, u32, u32, i64)>,
}

fn be32(b: &[u8], at: usize) -> u32 {
    b.get(at..at + 4).map_or(0, |x| u32::from_be_bytes([x[0], x[1], x[2], x[3]]))
}

fn le32(b: &[u8], at: usize) -> u32 {
    b.get(at..at + 4).map_or(0, |x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
}

/// The boxes in a QuickTime or MP4 range: (type, start of its contents, end).
fn boxes(b: &[u8], from: usize, to: usize) -> Vec<([u8; 4], usize, usize)> {
    let mut out = Vec::new();
    let mut at = from;
    while at + 8 <= to {
        let mut size = be32(b, at) as usize;
        let kind = [b[at + 4], b[at + 5], b[at + 6], b[at + 7]];
        let mut head = 8;
        if size == 1 {
            let hi = be32(b, at + 8) as usize;
            let lo = be32(b, at + 12) as usize;
            size = (hi << 32) | lo;
            head = 16;
        } else if size == 0 {
            size = to - at;
        }
        if size < head || at + size > to {
            break;
        }
        out.push((kind, at + head, at + size));
        at += size;
    }
    out
}

fn find<'a>(list: &'a [([u8; 4], usize, usize)], kind: &[u8; 4]) -> Option<&'a ([u8; 4], usize, usize)> {
    list.iter().find(|(k, ..)| k == kind)
}

fn read_quicktime(b: &[u8]) -> Option<Movie> {
    let top = boxes(b, 0, b.len());
    let ftyp = find(&top, b"ftyp");
    let moov = find(&top, b"moov")?;
    let quicktime = ftyp.is_none_or(|(_, s, _)| b.get(*s..*s + 4) == Some(b"qt  "));
    let mut m = Movie { quicktime, ..Default::default() };
    for (k, s, e) in boxes(b, moov.1, moov.2) {
        if &k != b"trak" {
            continue;
        }
        let trak = boxes(b, s, e);
        let Some(mdia) = find(&trak, b"mdia") else { continue };
        let mdia = boxes(b, mdia.1, mdia.2);
        let handler = find(&mdia, b"hdlr").map(|(_, s, _)| [b[s + 8], b[s + 9], b[s + 10], b[s + 11]]).unwrap_or_default();
        let Some(minf) = find(&mdia, b"minf") else { continue };
        let minf = boxes(b, minf.1, minf.2);
        let Some(stbl) = find(&minf, b"stbl") else { continue };
        let stbl = boxes(b, stbl.1, stbl.2);
        let samples = find(&stbl, b"stsz").map_or(0, |(_, s, _)| be32(b, s + 8)) as i64;
        match &handler {
            b"vide" if m.width == 0 => {
                if let Some((_, s, _)) = find(&trak, b"tkhd") {
                    // Version 1 has 64-bit times; width and height are the last two 16.16 numbers either way.
                    let v1 = b[*s] == 1;
                    let at = s + if v1 { 88 } else { 76 };
                    m.width = (be32(b, at) >> 16) as i64;
                    m.height = (be32(b, at + 4) >> 16) as i64;
                }
                m.frames = samples;
            }
            b"tmcd" => {
                let Some((_, s, _)) = find(&stbl, b"stsd") else { continue };
                // stsd: version and flags, count, then the entry: size, 'tmcd', 6 reserved, data reference index.
                let entry = s + 8;
                let d = entry + 16;
                let (flags, scale, duration, fps) = (be32(b, d + 4), be32(b, d + 8), be32(b, d + 12), b.get(d + 16).copied().unwrap_or(0) as u32);
                // The first timecode sample: a frame number, where the chunk offset table says.
                let first = find(&stbl, b"stco").map(|(_, s, _)| be32(b, s + 8) as usize).or_else(|| find(&stbl, b"co64").map(|(_, s, _)| ((be32(b, s + 8) as usize) << 32) | be32(b, s + 12) as usize)).map_or(0, |off| be32(b, off) as i32 as i64);
                m.tc = Some((flags, scale, duration, fps, first));
            }
            _ => {}
        }
    }
    Some(m)
}

fn read_avi(b: &[u8]) -> Option<Movie> {
    if b.get(0..4) != Some(b"RIFF") || b.get(8..12) != Some(b"AVI ") {
        return None;
    }
    let at = b.windows(4).position(|w| w == b"avih")?;
    let d = at + 8;
    Some(Movie { quicktime: false, frames: le32(b, d + 16) as i64, width: le32(b, d + 32) as i64, height: le32(b, d + 36) as i64, tc: None })
}

fn read_movie(path: &Path) -> Result<Movie, String> {
    let b = std::fs::read(path).map_err(|e| format!("movieInfo: {}: {e}", path.display()))?;
    read_avi(&b).or_else(|| read_quicktime(&b)).ok_or_else(|| format!("movieInfo: {} is not a movie this can read (QuickTime, MPEG-4 or AVI)", path.display()))
}

fn movie_info(s: &mut Session, a: &Args) -> CmdResult {
    let mut frame = 0i64;
    let path = if let Some(node) = a.get("movieTexture").and_then(Value::as_str) {
        let file = s.run("getAttr", json!({"args": [format!("{node}.fileTextureName")]})).map_err(|_| aztek_ops::CommandError::new(format!("movieInfo: {node} is not a movie texture")))?;
        // The frame the texture shows, from its first.
        if let Ok(v) = s.run("getAttr", json!({"args": [format!("{node}.frameExtension")]})) {
            frame = v.as_f64().map_or(0, |x| (x as i64 - 1).max(0));
        }
        file.as_str().unwrap_or("").to_string()
    } else {
        a.pos_str(0).unwrap_or("").to_string()
    };
    if path.is_empty() {
        fail!("movieInfo: name a movie file");
    }
    let m = read_movie(Path::new(&path)).map_err(aztek_ops::CommandError::new)?;
    let (flags, scale, duration, fps, first) = m.tc.unwrap_or((0, 0, 0, 0, 0));
    let bit = |b: u32| json!(flags & b != 0);
    for flag in ["frameCount", "width", "height", "quickTime", "timeCodeTrack", "timeScale", "frameDuration", "numFrames", "dropFrame", "twentyFourHourMax", "negTimesOK", "counter", "timeCode"] {
        if !a.bool(flag) {
            continue;
        }
        return Ok(match flag {
            "frameCount" => json!(m.frames),
            "width" => json!(m.width),
            "height" => json!(m.height),
            "quickTime" => json!(m.quicktime),
            "timeCodeTrack" => json!(m.tc.is_some()),
            "timeScale" => json!(scale),
            "frameDuration" => json!(duration),
            "numFrames" => json!(fps),
            "dropFrame" => bit(0x1),
            "twentyFourHourMax" => bit(0x2),
            "negTimesOK" => bit(0x4),
            "counter" => bit(0x8),
            _ => {
                if m.tc.is_none() {
                    fail!("movieInfo: {path} has no timecode track");
                }
                let n = first + frame;
                if flags & 0x8 != 0 {
                    json!(n)
                } else {
                    let fps = fps.max(1) as i64;
                    let mut n = n;
                    if flags & 0x1 != 0 && fps >= 30 {
                        // Drop frame: two frame numbers (four at 60) are skipped each minute but every tenth.
                        let drop = 2 * (fps / 30);
                        let per_10 = fps * 600 - drop * 9;
                        let per_min = fps * 60 - drop;
                        let (d, r) = (n / per_10, n % per_10);
                        n += drop * 9 * d + if r > drop { drop * ((r - drop) / per_min) } else { 0 };
                    }
                    let (fr, secs) = (n % fps, n / fps);
                    let mut hours = secs / 3600;
                    if flags & 0x2 != 0 {
                        hours %= 24;
                    }
                    json!([hours, secs / 60 % 60, secs % 60, fr])
                }
            }
        });
    }
    fail!("movieInfo: name what to report, as -frameCount or -width")
}

// ---------------------------------------------------------------------------------------------------------------------
// movIn and movOut

/// A plug and the plugs under it that hold numbers, in order: a compound's children, a multi's elements.
fn leaves(s: &mut Session, plug: &str) -> Result<Vec<String>, String> {
    let Some((node, attr)) = plug.split_once('.') else { return Err(format!("{plug} is not an attribute")) };
    let base = attr.trim_end_matches(']');
    let bare = base.rsplit('.').next().unwrap_or(base).split('[').next().unwrap_or(base).to_string();
    let q = |s: &mut Session, flag: &str| s.run("attributeQuery", json!({"args": [bare.clone()], "node": node, flag: true}));
    if !attr.ends_with(']') && q(s, "multi").ok().is_some_and(|v| on(&v)) {
        let idx = s.run("getAttr", json!({"args": [plug], "multiIndices": true})).ok();
        let mut out = Vec::new();
        for i in idx.as_ref().and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_i64) {
            out.extend(leaves(s, &format!("{plug}[{i}]"))?);
        }
        return Ok(out);
    }
    let kids: Vec<String> = q(s, "listChildren").ok().and_then(|v| v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())).unwrap_or_default();
    if !kids.is_empty() {
        let mut out = Vec::new();
        for k in kids {
            out.extend(leaves(s, &format!("{plug}.{k}").replace(&format!(".{bare}.{k}"), &format!(".{k}")))?);
        }
        return Ok(out);
    }
    let v = aztek_ops::units::as_script(|| s.run("getAttr", json!({"args": [plug]}))).map_err(|e| e.to_string())?;
    match v {
        Value::Number(_) | Value::Bool(_) => Ok(vec![plug.to_string()]),
        Value::Array(ref a) if a.len() == 1 && (a[0].is_number() || a[0].is_boolean()) => Ok(vec![plug.to_string()]),
        _ => Err(format!("{plug} is not a numeric, time, linear or angular attribute")),
    }
}

fn all_leaves(s: &mut Session, a: &Args, cmd: &str) -> CmdResult<Vec<String>> {
    let targets: Vec<String> = a.positional.iter().flat_map(|v| match v {
        Value::Array(l) => l.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        Value::String(t) => vec![t.clone()],
        _ => Vec::new(),
    }).collect();
    if targets.is_empty() {
        fail!("{cmd}: name the attributes");
    }
    let mut out = Vec::new();
    for t in targets {
        out.extend(leaves(s, &t).map_err(|e| aztek_ops::CommandError::new(format!("{cmd}: {e}")))?);
    }
    Ok(out)
}

fn mov_path(a: &Args, cmd: &str) -> CmdResult<PathBuf> {
    let file = a.string("file", "");
    if file.is_empty() {
        fail!("{cmd}: give the file with -file");
    }
    let mut p = PathBuf::from(&file);
    if p.extension().is_none() {
        p.set_extension("mov");
    }
    Ok(p)
}

fn number(v: &Value) -> f64 {
    match v {
        Value::Array(a) => a.first().map(number).unwrap_or(0.0),
        Value::Bool(b) => *b as i64 as f64,
        other => other.as_f64().unwrap_or(0.0),
    }
}

fn time_range(s: &mut Session, a: &Args) -> CmdResult<(f64, f64)> {
    let parse = |t: &str| t.trim().trim_matches(|c| c == '"').parse::<f64>().ok();
    match a.get("time") {
        Some(Value::String(t)) => {
            let (lo, hi) = t.split_once(':').map_or((t.as_str(), t.as_str()), |(x, y)| (x, y));
            let cur = aztek_scene::anim::frame(&s.graph);
            let lo = if lo.trim().is_empty() { Some(cur) } else { parse(lo) };
            let hi = if hi.trim().is_empty() { lo } else { parse(hi) };
            match (lo, hi) {
                (Some(l), Some(h)) => Ok((l, h)),
                _ => fail!("movOut: -time takes a range such as \"1:24\""),
            }
        }
        Some(Value::Array(v)) if v.len() >= 2 => Ok((number(&v[0]), number(&v[1]))),
        Some(v) if v.is_number() => Ok((number(v), number(v))),
        _ => {
            let lo = s.run("playbackOptions", json!({"query": true, "queried": ["minTime"]})).map(|v| number(&v))?;
            let hi = s.run("playbackOptions", json!({"query": true, "queried": ["maxTime"]})).map(|v| number(&v))?;
            Ok((lo, hi))
        }
    }
}

fn mov_out(s: &mut Session, a: &Args) -> CmdResult {
    let path = mov_path(a, "movOut")?;
    let plugs = all_leaves(s, a, "movOut")?;
    let precision = a.get("precision").and_then(Value::as_f64).map_or(6, |p| p.max(0.0) as usize);
    let (lo, hi) = time_range(s, a)?;
    let mut text = String::new();
    if a.get("comment").is_some_and(on) {
        text.push_str("# movOut\n");
        for p in &plugs {
            text.push_str(&format!("# {p}\n"));
        }
        text.push_str("#\n");
    }
    let mut t = lo;
    while t <= hi + 1e-9 {
        let mut row = Vec::with_capacity(plugs.len());
        for p in &plugs {
            let v = aztek_ops::units::as_script(|| s.run("getAttr", json!({"args": [p], "time": t}))).map_err(|e| aztek_ops::CommandError::new(format!("movOut: {p}: {e}")))?;
            row.push(format!("{:.*}", precision, number(&v)));
        }
        text.push_str(&row.join(" "));
        text.push('\n');
        t += 1.0;
    }
    std::fs::write(&path, text).map_err(|e| aztek_ops::CommandError::new(format!("movOut: {}: {e}", path.display())))?;
    Ok(Value::Null)
}

fn mov_in(s: &mut Session, a: &Args) -> CmdResult {
    let path = mov_path(a, "movIn")?;
    let text = std::fs::read_to_string(&path).map_err(|e| aztek_ops::CommandError::new(format!("movIn: {}: {e}", path.display())))?;
    let plugs = all_leaves(s, a, "movIn")?;
    let start = match a.get("startTime") {
        Some(v) if !v.is_null() => number(v),
        _ => aztek_scene::anim::frame(&s.graph),
    };
    let mut rows = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with("//") {
            continue;
        }
        let vals: Vec<f64> = l.split_whitespace().map(|w| w.parse::<f64>()).collect::<Result<_, _>>().map_err(|_| aztek_ops::CommandError::new(format!("movIn: line {} of {} has something that isn't a number", i + 1, path.display())))?;
        if vals.len() != plugs.len() {
            fail!("movIn: line {} of {} has {} values for {} attributes", i + 1, path.display(), vals.len(), plugs.len());
        }
        rows.push(vals);
    }
    for (k, vals) in rows.iter().enumerate() {
        let t = start + k as f64;
        for (p, v) in plugs.iter().zip(vals) {
            aztek_ops::units::as_script(|| s.run("setKeyframe", json!({"args": [p], "time": t, "value": v})))?;
        }
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// melInfo and the MEL side

/// melInfo: the global procedures defined now.
fn mel_info(core: &mut Core) -> Value {
    json!(crate::mel::global_proc_names(core))
}

pub(crate) fn intercept(core: &mut Core, name: &str, _call: &Value) -> Option<Result<Value, String>> {
    match name {
        "melInfo" => Some(Ok(mel_info(core))),
        _ => None,
    }
}

pub fn register(s: &mut Session) {
    let b = ArgType::Bool;
    s.register(
        CommandDef::new("makebot", "Make a block ordered texture (BOT) file of an image, for texture caching. A relative -input is found by the project's rules; a relative -output goes in its source images folder.", makebot)
            .flag(f("checkdepends", "c", b, "Only when the BOT file isn't there or is older than the image."))
            .flag(f("checkres", "r", ArgType::Int, "Only when the image's width or height is at least this."))
            .flag(f("input", "i", ArgType::String, "The image."))
            .flag(f("nooverwrite", "nov", b, "Where -c or -r would make the BOT file again, leave one that is there."))
            .flag(f("output", "o", ArgType::String, "The BOT file."))
            .flag(f("verbose", "v", b, "Say what it does.")),
    );
    s.register(CommandDef::new("melInfo", "The names of the global MEL procedures defined now. whatIs gives each one's definition.", |_s, _a| Ok(json!([]))).undoable(false));
    s.register(
        CommandDef::new("melOptions", "Options of the MEL language. -duplicateVariableWarnings: warn, as a script is sourced, of a variable declared twice in one scope.", mel_options)
            .undoable(false)
            .flag(f("query", "q", b, "Query mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
            .flag(f("duplicateVariableWarnings", "dvw", b, "Warn of variables declared twice in one scope, as scripts are sourced (optionVar melDuplicateVariableWarnings).")),
    );
    let mut mem = CommandDef::new("memory", "Memory available and in use. Sizes are in bytes unless a unit flag says otherwise; an int past 2147483647 is cut to it.", memory)
        .flag(f("asFloat", "af", b, "The number as a float, keeping what a unit would round off."))
        .flag(f("debug", "dbg", b, "Print the arena allocator's statistics, if there is one."))
        .flag(f("gigaByte", "gb", b, "In gigabytes (1024^3 bytes)."))
        .flag(f("kiloByte", "kb", b, "In kilobytes (1024 bytes)."))
        .flag(f("megaByte", "mb", b, "In megabytes (1024^2 bytes)."))
        .flag(f("summary", "sum", b, "A summary of it all, in megabytes; size flags are ignored."));
    for (n, sh, doc, _) in MEM_FLAGS {
        mem = mem.flag(f(n, sh, b, doc));
    }
    s.register(mem);
    s.register(
        CommandDef::new("movIn", "Read a .mov file of numbers into animation curves on the attributes named, a row per frame from the current time (or -startTime). Keys there are replaced.", mov_in)
            .positional("attributes")
            .flag(f("file", "f", ArgType::String, "The .mov file; .mov is added to a name without an extension."))
            .flag(f("startTime", "st", ArgType::Float, "The frame the first row goes on; the current time when not given.")),
    );
    s.register(
        CommandDef::new("movOut", "Write the attributes named to a .mov file: a row per frame of the range, values in current units.", mov_out)
            .undoable(false)
            .positional("attributes")
            .flag(f("comment", "c", ArgType::Any, "Write the attributes in a comment at the top (false)."))
            .flag(f("file", "f", ArgType::String, "The .mov file; .mov is added to a name without an extension."))
            .flag(f("precision", "pre", ArgType::Int, "Digits after the point (6)."))
            .flag(f("time", "t", ArgType::Any, "The range, as \"start:end\"; the playback range when not given.")),
    );
    let mut mi = CommandDef::new("movieInfo", "Facts about a movie file (QuickTime, MPEG-4 or AVI): frames, size, and its timecode track's format.", movie_info).undoable(false).positional("movie file").flag(f(
        "movieTexture",
        "mt",
        ArgType::String,
        "A movie texture node: the movie it has loaded.",
    ));
    for (n, sh, doc) in [
        ("counter", "cn", "Whether the timecode is a counter: -timeCode is then a frame count rather than hours, minutes, seconds and frames."),
        ("dropFrame", "df", "The timecode format's drop frame flag."),
        ("frameCount", "f", "The number of frames."),
        ("frameDuration", "fd", "The timecode format's frame duration."),
        ("height", "h", "The height."),
        ("negTimesOK", "nt", "The timecode format's negative times OK flag."),
        ("numFrames", "nf", "The timecode format's whole frames per second."),
        ("quickTime", "qt", "Whether it is a QuickTime movie."),
        ("timeCode", "tc", "The timecode of the current frame."),
        ("timeCodeTrack", "tt", "Whether it has a timecode track."),
        ("timeScale", "ts", "The timecode format's time scale."),
        ("twentyFourHourMax", "tf", "The timecode format's 24 hour maximum flag."),
        ("width", "w", "The width."),
    ] {
        mi = mi.flag(f(n, sh, b, doc));
    }
    s.register(mi);
    let mut ntp = CommandDef::new("nurbsToPolygonsPref", "The NURBS to polygons (tessellate) preference the menu items use, kept between sessions. The settings are the nurbsTessellate node's.", nurbs_to_polygons_pref_cmd)
        .flag(f("query", "q", b, "Query mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."));
    for (n, sh, kind, def) in NTP {
        let ty = match kind {
            0 => ArgType::Int,
            1 => ArgType::Float,
            _ => ArgType::Bool,
        };
        ntp = ntp.flag(f(n, sh, ty, &format!("The nurbsTessellate setting of this name ({def} at first).")));
    }
    s.register(ntp);
    s.register(
        CommandDef::new("openCLInfo", "The GPU deformer evaluation's settings. -q -valid: whether it is set up (1).", opencl_info)
            .undoable(false)
            .flag(f("query", "q", b, "Query mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
            .flag(f("limitMinimumVerts", "lmv", b, "Whether geometry under -minVertexBuffer stays off the GPU. For debugging; not saved."))
            .flag(f("minVertexBuffer", "mvb", ArgType::Int, "The fewest vertices geometry has to go on the GPU. For debugging; not saved."))
            .flag(f("supportsDoublePrecision", "dbl", b, "Whether double precision is used; it can't be turned on where it isn't supported."))
            .flag(f("valid", "v", b, "Query only: whether it is set up.")),
    );
    s.register(
        CommandDef::new("openGLExtension", "The views' graphics device: -renderer, -vendor, -version, or whether an extension is supported (its name, or \"\"). Empty before a view has been made.", opengl_extension)
            .undoable(false)
            .flag(f("extension", "ext", ArgType::String, "The extension asked about; \"\" lists them all."))
            .flag(f("renderer", "rnd", b, "The device."))
            .flag(f("vendor", "vnd", b, "Who made it."))
            .flag(f("version", "ver", b, "The graphics API version.")),
    );
    s.register(
        CommandDef::new("openMayaPref", "The API preferences: the error log of failed API calls, lazy binding of compiled plug-ins, and the warning about plug-ins built for older releases. Returns 1.", open_maya_pref)
            .flag(f("query", "q", b, "Query mode."))
            .flag(f("edit", "e", b, "Edit mode."))
            .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
            .flag(f("errlog", "el", b, "Log failed API calls, with where they were called from, to OpenMayaErrorLog in the current folder (off)."))
            .flag(f("lazyLoad", "lz", b, "Load compiled plug-ins with lazy binding: an unresolved reference fails when it is called, not when loading (off)."))
            .flag(f("oldPluginWarning", "ow", b, "Warn when loadPlugin loads a plug-in built for an older release (on).")),
    );
    aztek_scene::perf_options::register(s);
    let _ = launch_offset();
}
