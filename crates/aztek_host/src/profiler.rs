//! The profiler: timed events recorded while sampling is on (each command run, and the MEL procedures and commands
//! instrumented with -instrumentMel), in categories that can be turned off one by one, saved to and read from a file.
//! profilerTool drives the profiler panel's view of them: which events show, how they are laid out and what is framed.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Value};

/// The categories there are from the start, as the profiler window lists them.
const BUILT_IN: &[(&str, &str)] = &[
    ("Animation", "Animation evaluation and playback"),
    ("Command", "Commands run"),
    ("Dirty Propagation", "Dirty propagation through the dependency graph"),
    ("Evaluation", "Node evaluation"),
    ("File IO", "Reading and writing files"),
    ("Maya Qt", "Interface events"),
    ("MEL", "MEL procedures and commands"),
    ("Python", "Python scripts"),
    ("Rendering", "Rendering"),
    ("Viewport", "Viewport drawing"),
    ("Polygons", "Polygon operations"),
    ("Deformers", "Deformer evaluation"),
    ("Dynamics", "Dynamics simulation"),
    ("Idle", "Idle events"),
];

const DEFAULT_BUFFER: i64 = 20;
/// Events kept per unit of -bufferSize (a megabyte of 64 byte events).
const EVENTS_PER_UNIT: usize = 16384;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub category: usize,
    pub color: i64,
    /// Microseconds since the profiler started.
    pub start: f64,
    pub duration: f64,
    pub thread: i64,
    pub cpu: i64,
    pub signal: bool,
    /// The frame the scene was at: events of one frame make a segment.
    pub frame: f64,
}

#[derive(Clone, Debug)]
struct Category {
    name: String,
    info: String,
    recording: bool,
}

#[derive(Clone, Debug)]
struct Instrument {
    category: usize,
    color: i64,
    description: String,
    signal: bool,
}

struct Open {
    name: String,
    start: f64,
    category: usize,
    color: i64,
    description: String,
    signal: bool,
    frame: f64,
}

struct Prof {
    sampling: bool,
    buffer_size: i64,
    capacity: usize,
    events: Vec<Event>,
    /// None where a category was removed, so later indices keep their numbers.
    cats: Vec<Option<Category>>,
    instr: HashMap<String, Instrument>,
    open: Vec<Option<Open>>,
    output: String,
    load: String,
}

impl Prof {
    fn new() -> Prof {
        Prof {
            sampling: false,
            buffer_size: DEFAULT_BUFFER,
            capacity: DEFAULT_BUFFER as usize * EVENTS_PER_UNIT,
            events: Vec::new(),
            cats: BUILT_IN.iter().map(|(n, i)| Some(Category { name: n.to_string(), info: i.to_string(), recording: true })).collect(),
            instr: HashMap::new(),
            open: Vec::new(),
            output: String::new(),
            load: String::new(),
        }
    }

    fn cat_index(&self, name: &str) -> Option<usize> {
        self.cats.iter().position(|c| c.as_ref().is_some_and(|c| c.name == name))
    }

    fn cat(&self, i: usize) -> Option<&Category> {
        self.cats.get(i).and_then(Option::as_ref)
    }

    fn records(&self, cat: usize) -> bool {
        self.sampling && self.cat(cat).is_some_and(|c| c.recording)
    }

    fn push(&mut self, e: Event) {
        if self.events.len() >= self.capacity {
            // A full buffer stops the recording.
            self.sampling = false;
            return;
        }
        self.events.push(e);
    }
}

thread_local! {
    static PROF: std::cell::RefCell<Prof> = std::cell::RefCell::new(Prof::new());
    static VIEW: std::cell::RefCell<View> = std::cell::RefCell::new(View { mode: "thread".into(), ..Default::default() });
}

fn with<R>(f: impl FnOnce(&mut Prof) -> R) -> R {
    PROF.with(|p| f(&mut p.borrow_mut()))
}

fn now_us() -> f64 {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    EPOCH.get_or_init(Instant::now).elapsed().as_secs_f64() * 1e6
}

fn thread_number() -> i64 {
    static NEXT: Mutex<i64> = Mutex::new(0);
    thread_local! {
        static ME: i64 = {
            let mut n = NEXT.lock().unwrap_or_else(|e| e.into_inner());
            let me = *n;
            *n += 1;
            me
        };
    }
    ME.with(|m| *m)
}

fn cpu_number() -> i64 {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: sched_getcpu takes nothing and only reads the calling thread's state.
        let c = unsafe { libc::sched_getcpu() };
        if c >= 0 {
            return c as i64;
        }
    }
    0
}

/// The colour index events of a category get when nothing gives them one.
fn category_color(cat: usize) -> i64 {
    (cat % 23) as i64 + 1
}

// ---------------------------------------------------------------------------------------------------------------------
// Recording, called by the host around each command and by the MEL interpreter around each procedure call

/// A command (or an instrumented procedure) is about to run. Every `begin` is followed by one `end`.
pub fn begin(name: &str, frame: f64, procedure: bool) {
    with(|p| {
        let open = if !p.sampling || name == "profiler" || name == "profilerTool" {
            None
        } else {
            match p.instr.get(name).cloned() {
                Some(i) if p.records(i.category) => Some(Open { name: name.into(), start: now_us(), category: i.category, color: i.color, description: i.description, signal: i.signal, frame }),
                Some(_) => None,
                None if procedure => None,
                None => {
                    let cat = p.cat_index("Command").unwrap_or(1);
                    p.records(cat).then(|| Open { name: name.into(), start: now_us(), category: cat, color: category_color(cat), description: "Command".into(), signal: false, frame })
                }
            }
        };
        p.open.push(open);
    })
}

/// What the last `begin` opened has finished.
pub fn end() {
    let t = now_us();
    with(|p| {
        if let Some(Some(o)) = p.open.pop() {
            if !p.sampling {
                return;
            }
            let duration = if o.signal { 0.0 } else { (t - o.start).max(0.0) };
            let e = Event { name: o.name, description: o.description, category: o.category, color: o.color, start: o.start, duration, thread: thread_number(), cpu: cpu_number(), signal: o.signal, frame: o.frame };
            p.push(e);
        }
    });
}

/// Whether a MEL procedure is instrumented, so the interpreter times it.
pub fn instrumented(name: &str) -> bool {
    with(|p| p.sampling && p.instr.contains_key(name))
}

pub fn events() -> Vec<Event> {
    with(|p| p.events.clone())
}

fn category_names() -> Vec<String> {
    with(|p| p.cats.iter().map(|c| c.as_ref().map(|c| c.name.clone()).unwrap_or_default()).collect())
}

// ---------------------------------------------------------------------------------------------------------------------
// The file

fn clean(t: &str) -> String {
    t.replace(['\t', '\n', '\r'], " ")
}

fn write_file(path: &str) -> Result<(), String> {
    let (cats, events) = with(|p| (p.cats.clone(), p.events.clone()));
    let mut out = String::from("# Aztek profiler events 1\n");
    for (i, c) in cats.iter().enumerate() {
        if let Some(c) = c {
            out.push_str(&format!("category\t{i}\t{}\t{}\t{}\n", clean(&c.name), clean(&c.info), c.recording as i32));
        }
    }
    for e in &events {
        out.push_str(&format!(
            "event\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            clean(&e.name),
            clean(&e.description),
            e.category,
            e.color,
            e.start,
            e.duration,
            e.thread,
            e.cpu,
            e.signal as i32,
            e.frame
        ));
    }
    std::fs::write(path, out).map_err(|e| format!("profiler: {path}: {e}"))
}

fn read_file(path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("profiler: {path}: {e}"))?;
    if !text.starts_with("# Aztek profiler events") {
        return Err(format!("profiler: {path} is not a profiler events file"));
    }
    with(|p| {
        // The file's categories by its own numbers, matched to ours by name.
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut events = Vec::new();
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            match f.first().copied() {
                Some("category") if f.len() >= 4 => {
                    let at = f[1].parse::<usize>().unwrap_or(0);
                    let ours = match p.cat_index(f[2]) {
                        Some(i) => i,
                        None => {
                            p.cats.push(Some(Category { name: f[2].into(), info: f[3].into(), recording: true }));
                            p.cats.len() - 1
                        }
                    };
                    map.insert(at, ours);
                }
                Some("event") if f.len() >= 11 => {
                    let num = |i: usize| f[i].parse::<f64>().unwrap_or(0.0);
                    let at = f[3].parse::<usize>().unwrap_or(0);
                    events.push(Event {
                        name: f[1].into(),
                        description: f[2].into(),
                        category: map.get(&at).copied().unwrap_or(at),
                        color: num(4) as i64,
                        start: num(5),
                        duration: num(6),
                        thread: num(7) as i64,
                        cpu: num(8) as i64,
                        signal: f[9] == "1",
                        frame: num(10),
                    });
                }
                _ => {}
            }
        }
        p.events = events;
    });
    crate::profiler::view_reset();
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
// profiler

fn queried(a: &Args) -> Vec<String> {
    a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn on(v: &Value) -> bool {
    v.as_bool().unwrap_or(v.as_f64().is_some_and(|x| x != 0.0) || v.as_str().is_some_and(|s| matches!(s, "true" | "on" | "yes" | "1")))
}

fn int_of(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)).or_else(|| v.as_str().and_then(|t| t.trim().parse().ok()))
}

/// The category -categoryIndex or -categoryName names.
fn named_category(p: &Prof, a: &Args) -> Result<usize, String> {
    if let Some(i) = a.get("categoryIndex").and_then(int_of) {
        if i < 0 || p.cat(i as usize).is_none() {
            return Err(format!("profiler: no category {i}"));
        }
        return Ok(i as usize);
    }
    if let Some(n) = a.get("categoryName").and_then(Value::as_str) {
        return p.cat_index(n).ok_or_else(|| format!("profiler: no category named \"{n}\""));
    }
    Err("profiler: give -categoryIndex or -categoryName".into())
}

fn profiler(_s: &mut Session, a: &Args) -> CmdResult {
    let q = queried(a);
    let query = a.bool("query");
    let wants = |f: &str| q.iter().any(|x| x == f) || (query && a.get(f).and_then(Value::as_bool) == Some(true));
    if query {
        return with(|p| -> CmdResult {
            if wants("sampling") {
                return Ok(json!(p.sampling));
            }
            if wants("bufferSize") {
                return Ok(json!(p.buffer_size));
            }
            if wants("eventCount") {
                return Ok(json!(p.events.len()));
            }
            if wants("output") {
                return Ok(json!(p.output));
            }
            if wants("load") {
                return Ok(json!(p.load));
            }
            if wants("reset") {
                return Ok(json!(p.events.is_empty()));
            }
            if wants("categoryCount") {
                return Ok(json!(p.cats.iter().filter(|c| c.is_some()).count()));
            }
            if wants("allCategories") {
                let info = wants("categoryInfo");
                let mut v = Vec::new();
                for c in p.cats.iter().flatten() {
                    v.push(c.name.clone());
                    if info {
                        v.push(c.info.clone());
                    }
                }
                return Ok(json!(v));
            }
            if let Some(i) = a.get("categoryIndexToName").and_then(int_of) {
                let Some(c) = (i >= 0).then(|| p.cat(i as usize)).flatten() else { fail!("profiler: no category {i}") };
                return Ok(json!(c.name));
            }
            if let Some(n) = a.get("categoryNameToIndex").and_then(Value::as_str) {
                let Some(i) = p.cat_index(n) else { fail!("profiler: no category named \"{n}\"") };
                return Ok(json!(i));
            }
            if wants("categoryRecording") {
                let i = named_category(p, a).map_err(aztek_ops::CommandError::new)?;
                return Ok(json!(p.cat(i).is_some_and(|c| c.recording)));
            }
            if wants("categoryInfo") {
                let i = named_category(p, a).map_err(aztek_ops::CommandError::new)?;
                return Ok(json!(p.cat(i).map(|c| c.info.clone()).unwrap_or_default()));
            }
            let event_flags = ["eventStartTime", "eventDuration", "eventName", "eventDescription", "eventCategory", "eventColor", "eventThreadId", "eventCPUId", "signalEvent"];
            if let Some(f) = event_flags.iter().find(|f| wants(f)) {
                let Some(i) = a.get("eventIndex").and_then(int_of) else { fail!("profiler: -{f} needs -eventIndex") };
                let Some(e) = (i >= 0).then(|| p.events.get(i as usize)).flatten() else { fail!("profiler: no event {i}; there are {}", p.events.len()) };
                return Ok(match *f {
                    "eventStartTime" => json!(e.start),
                    "eventDuration" => json!(if e.signal { 0.0 } else { e.duration }),
                    "eventName" => json!(e.name),
                    "eventDescription" => json!(e.description),
                    "eventCategory" => json!(e.category),
                    "eventColor" => json!(e.color),
                    "eventThreadId" => json!(e.thread),
                    "eventCPUId" => json!(e.cpu),
                    _ => json!(e.signal),
                });
            }
            fail!("profiler: name what to query")
        });
    }
    // Saving wins over loading in one command.
    if let Some(path) = a.get("output").and_then(Value::as_str) {
        write_file(path).map_err(aztek_ops::CommandError::new)?;
        with(|p| p.output = path.to_string());
        return Ok(Value::Null);
    }
    if let Some(path) = a.get("load").and_then(Value::as_str) {
        if with(|p| p.sampling) {
            fail!("profiler: stop sampling before loading");
        }
        read_file(path).map_err(aztek_ops::CommandError::new)?;
        with(|p| p.load = path.to_string());
        return Ok(Value::Null);
    }
    if let Some(name) = a.get("addCategory").and_then(Value::as_str) {
        let info = a.get("categoryInfo").and_then(Value::as_str).unwrap_or("").to_string();
        return with(|p| {
            if let Some(i) = p.cat_index(name) {
                fail!("profiler: there is already a category named \"{name}\" ({i})");
            }
            p.cats.push(Some(Category { name: name.into(), info, recording: true }));
            Ok(json!(p.cats.len() - 1))
        });
    }
    if let Some(name) = a.get("removeCategory").and_then(Value::as_str) {
        return with(|p| {
            let Some(i) = p.cat_index(name) else { fail!("profiler: no category named \"{name}\"") };
            p.cats[i] = None;
            p.instr.retain(|_, x| x.category != i);
            Ok(json!(i))
        });
    }
    if a.bool("clearAllMelInstrumentation") {
        with(|p| p.instr.clear());
        return Ok(Value::Null);
    }
    if let Some(v) = a.get("instrumentMel").filter(|v| !v.is_null()) {
        let Some(proc_name) = a.get("procedureName").and_then(Value::as_str).map(str::to_string) else { fail!("profiler: -instrumentMel needs -procedureName") };
        if !on(v) {
            with(|p| p.instr.remove(&proc_name));
            return Ok(Value::Null);
        }
        return with(|p| {
            let cat = match a.get("categoryIndex").and_then(int_of) {
                Some(i) if i >= 0 && p.cat(i as usize).is_some() => i as usize,
                Some(i) => fail!("profiler: no category {i}"),
                None => match a.get("categoryName").and_then(Value::as_str) {
                    Some(n) => match p.cat_index(n) {
                        Some(i) => i,
                        None => fail!("profiler: no category named \"{n}\""),
                    },
                    None => p.cat_index("MEL").unwrap_or(0),
                },
            };
            let color = a.get("colorIndex").and_then(int_of).unwrap_or_else(|| category_color(cat));
            let description = a.get("procedureDescription").and_then(Value::as_str).unwrap_or("").to_string();
            let signal = a.get("signalMelEvent").is_some_and(on);
            p.instr.insert(proc_name, Instrument { category: cat, color, description, signal });
            Ok(Value::Null)
        });
    }
    if let Some(v) = a.get("categoryRecording").filter(|v| !v.is_null()) {
        let rec = on(v);
        return with(|p| {
            let i = named_category(p, a).map_err(aztek_ops::CommandError::new)?;
            if let Some(Some(c)) = p.cats.get_mut(i) {
                c.recording = rec;
            }
            Ok(Value::Null)
        });
    }
    if let Some(n) = a.get("bufferSize").and_then(int_of) {
        return with(|p| {
            if p.sampling {
                fail!("profiler: the buffer size can't change while sampling");
            }
            if n < 1 {
                fail!("profiler: the buffer size must be at least 1");
            }
            // Taken up at the next start of sampling.
            p.buffer_size = n;
            Ok(Value::Null)
        });
    }
    if a.bool("reset") {
        return with(|p| {
            if p.sampling {
                fail!("profiler: stop sampling before resetting");
            }
            p.events.clear();
            drop_view();
            Ok(Value::Null)
        });
    }
    if let Some(v) = a.get("sampling").filter(|v| !v.is_null()) {
        let start = on(v);
        with(|p| {
            if start && !p.sampling {
                p.capacity = p.buffer_size.max(1) as usize * EVENTS_PER_UNIT;
                p.events.clear();
            }
            p.sampling = start;
        });
        if start {
            drop_view();
        }
        return Ok(Value::Null);
    }
    // Without -addCategory, -categoryInfo sets the description of the category named.
    if let Some(info) = a.get("categoryInfo").and_then(Value::as_str) {
        return with(|p| {
            let i = named_category(p, a).map_err(aztek_ops::CommandError::new)?;
            if let Some(Some(c)) = p.cats.get_mut(i) {
                c.info = info.to_string();
            }
            Ok(Value::Null)
        });
    }
    if let Some(i) = a.get("categoryIndexToName").and_then(int_of) {
        return with(|p| match (i >= 0).then(|| p.cat(i as usize)).flatten() {
            Some(c) => Ok(json!(c.name)),
            None => fail!("profiler: no category {i}"),
        });
    }
    if let Some(n) = a.get("categoryNameToIndex").and_then(Value::as_str) {
        return with(|p| match p.cat_index(n) {
            Some(i) => Ok(json!(i)),
            None => fail!("profiler: no category named \"{n}\""),
        });
    }
    Ok(Value::Null)
}

// ---------------------------------------------------------------------------------------------------------------------
// profilerTool: the view

#[derive(Default)]
struct View {
    made: bool,
    control: String,
    mode: String,
    match_whole_word: bool,
    selected: Vec<usize>,
    hidden: HashSet<usize>,
    collapsed: HashSet<usize>,
    isolated: Option<usize>,
    framed: (f64, f64),
    critical: Vec<usize>,
    hotspot: Vec<usize>,
}

fn view<R>(f: impl FnOnce(&mut View) -> R) -> R {
    VIEW.with(|v| f(&mut v.borrow_mut()))
}

/// New events: what the view picked out of the old ones goes.
fn drop_view() {
    view(|v| {
        v.selected.clear();
        v.hidden.clear();
        v.collapsed.clear();
        v.isolated = None;
        v.critical.clear();
        v.hotspot.clear();
        v.framed = (0.0, 0.0);
    });
}

pub(crate) fn view_reset() {
    drop_view();
}

/// The segments: runs of events at one frame, in the order recorded. Each is (first event, one past the last).
fn segments(events: &[Event]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (i, e) in events.iter().enumerate() {
        match out.last_mut() {
            Some(s) if events[s.0].frame == e.frame => s.1 = i + 1,
            _ => out.push((i, i + 1)),
        }
    }
    out
}

/// The events inside one, on its thread: those that start in it and end by its end.
fn inside(events: &[Event], i: usize) -> Vec<usize> {
    let e = &events[i];
    let end = e.start + e.duration;
    events.iter().enumerate().filter(|(j, x)| *j != i && x.thread == e.thread && x.start >= e.start && x.start + x.duration <= end && (x.start > e.start || x.duration < e.duration)).map(|(j, _)| j).collect()
}

fn range_of(events: &[Event], idx: &[usize]) -> (f64, f64) {
    let mut r = (f64::MAX, f64::MIN);
    for &i in idx {
        if let Some(e) = events.get(i) {
            r.0 = r.0.min(e.start);
            r.1 = r.1.max(e.start + e.duration);
        }
    }
    if r.0 > r.1 {
        (0.0, 0.0)
    } else {
        r
    }
}

fn name_matches(name: &str, pat: &str, whole: bool) -> bool {
    if whole {
        name == pat
    } else {
        name.contains(pat)
    }
}

/// The segment the selection is in, or the isolated one, or the first.
fn focus_segment(events: &[Event], v: &View) -> Option<(usize, usize)> {
    let segs = segments(events);
    if let Some(i) = v.isolated {
        return segs.get(i).copied();
    }
    if let Some(&s) = v.selected.first() {
        return segs.iter().copied().find(|(a, b)| s >= *a && s < *b);
    }
    segs.first().copied()
}

fn profiler_tool(_s: &mut Session, a: &Args) -> CmdResult {
    let q = queried(a);
    let query = a.bool("query");
    let wants = |f: &str| q.iter().any(|x| x == f) || (query && a.get(f).and_then(Value::as_bool) == Some(true));
    let events = events();
    if a.bool("destroy") {
        view(|v| {
            v.made = false;
            v.control.clear();
        });
        return Ok(Value::Null);
    }
    if a.bool("make") {
        let made = crate::scriptui::create_as("profilerTool", "profilerTool", &Args::default())?;
        view(|v| {
            v.made = true;
            v.control = made.as_str().unwrap_or("").to_string();
        });
        return Ok(made);
    }
    if wants("exists") || (!query && a.bool("exists")) {
        return Ok(json!(view(|v| v.made)));
    }
    if query {
        if wants("segmentCount") {
            return Ok(json!(segments(&events).len()));
        }
        if a.has("eventTypes") || wants("eventTypes") {
            // true (or the flag alone): the selected events', when some are selected; false: all of them.
            let sel: HashSet<usize> = view(|v| v.selected.iter().copied().collect());
            let only_selected = a.get("eventTypes").and_then(Value::as_bool) != Some(false) && !sel.is_empty();
            let names = category_names();
            let mut kinds: Vec<(String, String, i64, usize, usize)> = Vec::new();
            for (i, e) in events.iter().enumerate() {
                if only_selected && !sel.contains(&i) {
                    continue;
                }
                match kinds.iter_mut().find(|k| k.0 == e.name && k.1 == e.description && k.3 == e.category) {
                    Some(k) => k.4 += 1,
                    None => kinds.push((e.name.clone(), e.description.clone(), e.color, e.category, 1)),
                }
            }
            let list: Vec<Value> = kinds
                .into_iter()
                .map(|(n, d, c, cat, count)| {
                    let mut o = serde_json::Map::new();
                    if !n.is_empty() {
                        o.insert("type".into(), json!(n));
                    }
                    if !d.is_empty() {
                        o.insert("description".into(), json!(d));
                    }
                    o.insert("color".into(), json!(c));
                    o.insert("category".into(), json!(names.get(cat).cloned().unwrap_or_default()));
                    o.insert("count".into(), json!(count));
                    Value::Object(o)
                })
                .collect();
            return Ok(json!(Value::Array(list).to_string()));
        }
        if let Some(pat) = a.get("searchEvent").and_then(Value::as_str) {
            let whole = view(|v| v.match_whole_word);
            let found: Vec<usize> = events.iter().enumerate().filter(|(_, e)| name_matches(&e.name, pat, whole)).map(|(i, _)| i).collect();
            let next = wants("findNext") || a.bool("findNext");
            let prev = wants("findPrevious") || a.bool("findPrevious");
            if !next && !prev {
                if !found.is_empty() {
                    view(|v| v.selected = found.clone());
                }
                return Ok(json!(found));
            }
            let cur = view(|v| v.selected.last().copied());
            let pick = if next {
                found.iter().copied().find(|&i| cur.is_none_or(|c| i > c))
            } else {
                found.iter().rev().copied().find(|&i| cur.is_none_or(|c| i < c))
            };
            return Ok(match pick {
                Some(i) => {
                    view(|v| v.selected = vec![i]);
                    json!([i])
                }
                None => json!([]),
            });
        }
        if wants("matchWholeWord") {
            return Ok(json!(view(|v| v.match_whole_word)));
        }
        fail!("profilerTool: name what to query");
    }
    // Edits.
    if let Some(v) = a.get("matchWholeWord").filter(|v| !v.is_null()) {
        let w = on(v);
        view(|x| x.match_whole_word = w);
    }
    for (flag, mode) in [("categoryView", "category"), ("cpuView", "cpu"), ("threadView", "thread")] {
        if a.bool(flag) {
            view(|v| v.mode = mode.to_string());
            }
    }
    if let Some(i) = a.get("isolateSegment").and_then(int_of) {
        let n = segments(&events).len();
        if i < 0 || i as usize >= n {
            fail!("profilerTool: no segment {i}; there are {n}");
        }
        let (s0, s1) = segments(&events)[i as usize];
        view(|v| {
            v.isolated = Some(i as usize);
            v.framed = range_of(&events, &(s0..s1).collect::<Vec<_>>());
        });
    }
    if a.bool("unisolateSegment") {
        view(|v| v.isolated = None);
    }
    if a.bool("frameAll") {
        let all: Vec<usize> = (0..events.len()).collect();
        view(|v| v.framed = range_of(&events, &all));
    }
    if a.bool("frameSelected") {
        view(|v| v.framed = range_of(&events, &v.selected.clone()));
    }
    if let Some(v) = a.get("showAllEvent").filter(|v| !v.is_null()) {
        let show = on(v);
        view(|x| {
            x.hidden = if show { HashSet::new() } else { (0..events.len()).collect() };
            x.collapsed.clear();
        });
    }
    if let Some(v) = a.get("showSelectedEvents").filter(|v| !v.is_null()) {
        let show = on(v);
        view(|x| {
            let sel: HashSet<usize> = x.selected.iter().copied().collect();
            if show {
                x.hidden = (0..events.len()).filter(|i| !sel.contains(i)).collect();
            } else {
                x.hidden.extend(sel);
            }
        });
    }
    if let Some(v) = a.get("showSelectedEventsRepetition").filter(|v| !v.is_null()) {
        let show = on(v);
        view(|x| {
            let comments: HashSet<String> = x.selected.iter().filter_map(|&i| events.get(i)).map(|e| e.description.clone()).collect();
            let same: HashSet<usize> = events.iter().enumerate().filter(|(_, e)| comments.contains(&e.description)).map(|(i, _)| i).collect();
            if show {
                x.hidden = (0..events.len()).filter(|i| !same.contains(i)).collect();
            } else {
                x.hidden.extend(same);
            }
        });
    }
    if a.bool("collapseSelectedEvents") || a.bool("expandSelectedEvents") {
        let collapse = a.bool("collapseSelectedEvents");
        view(|x| {
            for &i in &x.selected.clone() {
                if i >= events.len() {
                    continue;
                }
                for j in inside(&events, i) {
                    if collapse {
                        x.hidden.insert(j);
                    } else {
                        x.hidden.remove(&j);
                    }
                }
                if collapse {
                    x.collapsed.insert(i);
                } else {
                    x.collapsed.remove(&i);
                }
            }
        });
    }
    if a.bool("collapseSelectedEventsRepetition") || a.bool("expandSelectedEventsRepetition") {
        let collapse = a.bool("collapseSelectedEventsRepetition");
        view(|x| {
            for &i in &x.selected.clone() {
                if i >= events.len() {
                    continue;
                }
                // The sub-events with the comment of one already shown under it.
                let mut seen: HashSet<String> = HashSet::new();
                for j in inside(&events, i) {
                    let c = events[j].description.clone();
                    if !seen.insert(c) {
                        if collapse {
                            x.hidden.insert(j);
                        } else {
                            x.hidden.remove(&j);
                        }
                    }
                }
            }
        });
    }
    if a.bool("showCriticalPath") || a.bool("showHotspot") {
        let critical = a.bool("showCriticalPath");
        view(|x| {
            let Some((s0, s1)) = focus_segment(&events, x) else { return };
            let seg: Vec<usize> = (s0..s1).collect();
            if critical {
                // From the longest top-level event down, the longest event inside each.
                let mut path = Vec::new();
                let top = seg.iter().copied().filter(|&i| !seg.iter().any(|&j| j != i && inside(&events, j).contains(&i))).max_by(|&a, &b| events[a].duration.total_cmp(&events[b].duration));
                let mut cur = top;
                while let Some(c) = cur {
                    path.push(c);
                    let kids = inside(&events, c);
                    let direct: Vec<usize> = kids.iter().copied().filter(|&k| !kids.iter().any(|&o| o != k && inside(&events, o).contains(&k))).collect();
                    cur = direct.into_iter().max_by(|&a, &b| events[a].duration.total_cmp(&events[b].duration));
                }
                x.critical = path;
            } else {
                // The event whose own time, less what runs inside it, is the most.
                let own = |i: usize| {
                    let kids = inside(&events, i);
                    let direct: f64 = kids.iter().filter(|&&k| !kids.iter().any(|&o| o != k && inside(&events, o).contains(&k))).map(|&k| events[k].duration).sum();
                    events[i].duration - direct
                };
                x.hotspot = seg.iter().copied().max_by(|&a, &b| own(a).total_cmp(&own(b))).into_iter().collect();
            }
        });
    }
    Ok(Value::Null)
}

/// What the profiler panel draws: the events with the view's state.
pub fn view_json() -> Value {
    let events = events();
    let names = category_names();
    view(|v| {
        let list: Vec<Value> = events
            .iter()
            .enumerate()
            .filter(|(i, _)| !v.hidden.contains(i))
            .filter(|(i, _)| v.isolated.is_none_or(|s| segments(&events).get(s).is_some_and(|(a, b)| i >= a && i < b)))
            .map(|(i, e)| json!({"index": i, "name": e.name, "description": e.description, "category": e.category, "color": e.color, "start": e.start, "duration": e.duration, "thread": e.thread, "cpu": e.cpu, "signal": e.signal}))
            .collect();
        json!({
            "mode": v.mode,
            "events": list,
            "categories": names,
            "selected": v.selected,
            "critical": v.critical,
            "hotspot": v.hotspot,
            "collapsed": v.collapsed.iter().copied().collect::<Vec<_>>(),
            "framed": [v.framed.0, v.framed.1],
            "segments": segments(&events).len(),
        })
    })
}

/// The panel picked events by hand.
pub fn select_events(idx: Vec<usize>) {
    view(|v| v.selected = idx);
    crate::script_jobs::note_event("profilerSelectionChanged");
}

pub fn register(s: &mut Session) {
    let f = |n: &str, sh: &str, ty: ArgType, doc: &str| FlagDef::new(n, sh, ty, Value::Null, doc);
    let b = ArgType::Bool;
    s.register(
        CommandDef::new(
            "profiler",
            "Record timed events while -sampling is on (each command run, and the MEL procedures and commands instrumented with -instrumentMel), and query them by -eventIndex. Times are in microseconds. Events go in categories, each of which can be turned off. -output saves them, -load reads them back.",
            profiler,
        )
        .undoable(false)
        .flag(f("query", "q", b, "Query mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("addCategory", "a", ArgType::String, "Add a category; returns its index. -categoryInfo gives its description."))
        .flag(f("allCategories", "ac", b, "Query: every category's name; with -categoryInfo, each name then its description."))
        .flag(f("bufferSize", "b", ArgType::Int, "How many events the buffer holds, in megabytes of events (20). Only while sampling is off; taken up when sampling next starts."))
        .flag(f("categoryCount", "cc", b, "Query: how many categories there are."))
        .flag(f("categoryIndex", "ci", ArgType::Int, "The category the other flags are about, by index."))
        .flag(f("categoryIndexToName", "cin", ArgType::Int, "The name of the category of this index."))
        .flag(f("categoryInfo", "cf", ArgType::String, "With -addCategory, the new category's description; query: the description of the category named."))
        .flag(f("categoryName", "cn", ArgType::String, "The category the other flags are about, by name."))
        .flag(f("categoryNameToIndex", "cni", ArgType::String, "The index of the category of this name."))
        .flag(f("categoryRecording", "cr", b, "Whether the category named records events."))
        .flag(f("clearAllMelInstrumentation", "cam", b, "Take every MEL procedure and command off instrumentation."))
        .flag(f("colorIndex", "coi", ArgType::Int, "With -instrumentMel true: the colour index its events are drawn in."))
        .flag(f("eventCPUId", "eci", b, "Query: the processor the event ran on."))
        .flag(f("eventCategory", "eca", b, "Query: the event's category index."))
        .flag(f("eventColor", "eco", b, "Query: the event's colour index."))
        .flag(f("eventCount", "ec", b, "Query: how many events the buffer holds."))
        .flag(f("eventDescription", "ed", b, "Query: the event's description."))
        .flag(f("eventDuration", "edu", b, "Query: the event's duration in microseconds; 0 for a signal event."))
        .flag(f("eventIndex", "ei", ArgType::Int, "The event the query is about."))
        .flag(f("eventName", "en", b, "Query: the event's name."))
        .flag(f("eventStartTime", "et", b, "Query: when the event started, in microseconds."))
        .flag(f("eventThreadId", "eti", b, "Query: the thread the event ran on."))
        .flag(f("instrumentMel", "in", b, "Time a MEL procedure or command (true, with -procedureName, -categoryIndex and -colorIndex) or stop (false, with -procedureName)."))
        .flag(f("load", "l", ArgType::String, "Read events from a file -output wrote, in place of the buffer's."))
        .flag(f("output", "o", ArgType::String, "Write the recorded events to a file. With -load as well, only this is done."))
        .flag(f("procedureDescription", "pd", ArgType::String, "With -instrumentMel true: the description its events get."))
        .flag(f("procedureName", "pn", ArgType::String, "The MEL procedure or command -instrumentMel is about."))
        .flag(f("removeCategory", "rc", ArgType::String, "Remove the category of this name; returns its index."))
        .flag(f("reset", "r", b, "Clear the recorded events. Only while sampling is off."))
        .flag(f("sampling", "s", b, "Start (true) or stop (false) recording events. Starting clears the buffer."))
        .flag(f("signalEvent", "sig", b, "Query: whether the event is a signal event, which has a start but no duration."))
        .flag(f("signalMelEvent", "sim", b, "With -instrumentMel true: its events are signal events.")),
    );
    s.register(
        CommandDef::new(
            "profilerTool",
            "The profiler panel's view of the recorded events: thread, category or CPU layout, framing, which events show, segments (the events of one frame), search and the critical path or hotspot of a frame.",
            profiler_tool,
        )
        .undoable(false)
        .flag(f("query", "q", b, "Query mode."))
        .flag(f("edit", "e", b, "Edit mode."))
        .flag(f("queried", "", ArgType::List, "Flags asked about in query mode."))
        .flag(f("categoryView", "cat", b, "Lay the events out by category."))
        .flag(f("collapseSelectedEvents", "cs", b, "Hide what runs inside the selected events."))
        .flag(f("collapseSelectedEventsRepetition", "csr", b, "Hide the repeats, by comment, of what runs inside the selected events."))
        .flag(f("cpuView", "cpu", b, "Lay the events out by processor."))
        .flag(f("destroy", "dtr", b, "Destroy the tool's view (for the panel)."))
        .flag(f("eventTypes", "et", b, "Query: the kinds of event there are, as JSON (type, description, color, category, count); true: of the selected events only."))
        .flag(f("exists", "ex", b, "Query: whether the view has been made with -make."))
        .flag(f("expandSelectedEvents", "es", b, "Show what runs inside the selected events."))
        .flag(f("expandSelectedEventsRepetition", "esr", b, "Show the repeats, by comment, of what runs inside the selected events."))
        .flag(f("findNext", "fn", b, "With -searchEvent: the next match after the selected event, which it selects."))
        .flag(f("findPrevious", "fp", b, "With -searchEvent: the match before the selected event, which it selects."))
        .flag(f("frameAll", "fa", b, "Frame every event."))
        .flag(f("frameSelected", "fs", b, "Frame the selected events."))
        .flag(f("isolateSegment", "is", ArgType::Int, "Show only this segment (from 0)."))
        .flag(f("make", "mk", b, "Make the tool's view in the current layout (for the panel)."))
        .flag(f("matchWholeWord", "mww", b, "Whether -searchEvent matches whole names only (false)."))
        .flag(f("searchEvent", "se", ArgType::String, "Query: the events whose names contain this (or are it, with -matchWholeWord); they are selected."))
        .flag(f("segmentCount", "sc", b, "Query: how many segments the events make."))
        .flag(f("showAllEvent", "sa", b, "Show (true) or hide (false) every event."))
        .flag(f("showCriticalPath", "scp", b, "Pick out the critical path of the selected frame."))
        .flag(f("showHotspot", "sh", b, "Pick out the hotspot of the selected frame."))
        .flag(f("showSelectedEvents", "ss", b, "Show only (true) or hide (false) the selected events."))
        .flag(f("showSelectedEventsRepetition", "ssr", b, "Show only (true) or hide (false) the events with the selected events' comments."))
        .flag(f("threadView", "thd", b, "Lay the events out by thread."))
        .flag(f("unisolateSegment", "uis", b, "Show every segment again.")),
    );
}
