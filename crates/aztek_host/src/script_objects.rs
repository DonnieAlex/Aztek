//! Interface objects scripts make that are not widgets: itemFilter, selectionConnection, panelConfiguration,
//! editorTemplate and baseTemplate. Each keeps its state here and answers with no interface.

use std::cell::RefCell;
use std::collections::BTreeMap;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Map, Value};

type Cmd = CommandDef<Session>;

#[derive(Default)]
struct Objects {
    filters: BTreeMap<String, Map<String, Value>>,
    connections: BTreeMap<String, Connection>,
    configs: BTreeMap<String, Map<String, Value>>,
    /// What editorTemplate has been told since the template began.
    template: Vec<Value>,
    base_templates: Vec<String>,
    counter: u32,
}

#[derive(Clone, Debug, Default)]
struct Connection {
    kind: String,
    objects: Vec<String>,
    filter: String,
    parent: String,
    locked: bool,
    /// An editorList's editor: the connection reflects its main list connection.
    editor: String,
}

thread_local! {
    static OBJECTS: RefCell<Objects> = RefCell::new(Objects::default());
}

fn with<R>(f: impl FnOnce(&mut Objects) -> R) -> R {
    OBJECTS.with(|o| {
        let mut o = o.borrow_mut();
        // The lists the reference has from the start.
        if o.connections.is_empty() {
            for k in ["worldList", "modelList", "keyframeList", "highlightList", "activeList"] {
                o.connections.insert(k.to_string(), Connection { kind: k.to_string(), ..Default::default() });
            }
        }
        f(&mut o)
    })
}

fn add(mut c: Cmd, spec: &str) -> Cmd {
    for t in spec.split_whitespace() {
        let mut it = t.split(':');
        let (Some(long), Some(short), Some(ty)) = (it.next(), it.next(), it.next()) else { continue };
        if c.flags.iter().any(|x| x.name == long) {
            continue;
        }
        let mut d = FlagDef::new(long, short, if ty == "b" { ArgType::Bool } else { ArgType::Any }, Value::Null, "");
        if let Some(n) = ty.strip_prefix('a').and_then(|n| n.parse::<u8>().ok()) {
            d = d.multi(n);
        }
        c = c.flag(d);
    }
    c
}

fn asked(a: &Args) -> Option<String> {
    a.get("queried").and_then(Value::as_array).and_then(|q| q.first()).and_then(Value::as_str).map(str::to_string).or_else(|| a.flags.keys().find(|k| !matches!(k.as_str(), "query" | "queried")).cloned())
}

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        o => o.to_string(),
    }
}

fn flat(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a.iter().flat_map(flat).collect(),
        o => vec![o.clone()],
    }
}

fn strings(v: Option<&Value>) -> Vec<String> {
    v.map(flat).unwrap_or_default().iter().map(text).collect()
}

/// The next free name base1, base2...: the counter is read and bumped outside `taken`, which may itself look at the
/// objects.
fn fresh(base: &str, taken: impl Fn(&str) -> bool) -> String {
    loop {
        let n = with(|o| {
            o.counter += 1;
            format!("{base}{}", o.counter)
        });
        if !taken(&n) {
            break n;
        }
    }
}

/// `*` and `?`, as the reference's name patterns have them.
fn glob(pat: &str, name: &str) -> bool {
    fn go(p: &[char], n: &[char]) -> bool {
        match p.first() {
            None => n.is_empty(),
            Some('*') => (0..=n.len()).any(|i| go(&p[1..], &n[i..])),
            Some('?') => !n.is_empty() && go(&p[1..], &n[1..]),
            Some(c) => n.first() == Some(c) && go(&p[1..], &n[1..]),
        }
    }
    go(&pat.chars().collect::<Vec<_>>(), &name.chars().collect::<Vec<_>>())
}

// ---------------------------------------------------------------------------------------------------------------------
// itemFilter

fn filter_matches(filters: &BTreeMap<String, Map<String, Value>>, name: &str, node: &str, types: &[String], depth: u32) -> bool {
    let Some(x) = filters.get(name) else { return false };
    if depth > 16 {
        return false;
    }
    let sub = |n: &str| filter_matches(filters, n, node, types, depth + 1);
    let mut ok = true;
    if let Some(p) = x.get("byName") {
        ok &= strings(Some(p)).iter().any(|q| glob(q, node));
    }
    if let Some(t) = x.get("byType") {
        ok &= strings(Some(t)).iter().any(|q| types.contains(q));
    }
    for (flag, combine) in [("union", 0), ("intersect", 1), ("difference", 2)] {
        let parts = strings(x.get(flag));
        if let [l, r, ..] = parts.as_slice() {
            ok &= match combine {
                0 => sub(l) || sub(r),
                1 => sub(l) && sub(r),
                _ => sub(l) && !sub(r),
            };
        }
    }
    if x.get("negate").is_some_and(|v| v.as_bool().unwrap_or(false)) {
        ok = !ok;
    }
    ok
}

fn item_filter(a: &Args) -> CmdResult {
    let name = a.pos_str(0).map(str::to_string);
    if a.bool("listBuiltInFilters") || a.bool("listOtherFilters") {
        return Ok(json!(Vec::<String>::new()));
    }
    if a.bool("listUserFilters") {
        return Ok(json!(with(|o| o.filters.keys().cloned().collect::<Vec<_>>())));
    }
    if a.bool("exists") && !a.bool("query") {
        return Ok(json!(name.is_some_and(|n| with(|o| o.filters.contains_key(&n)))));
    }
    let mut flags = a.flags.clone();
    flags.retain(|k, v| !matches!(k.as_str(), "query" | "edit" | "queried" | "exists") && !v.is_null());
    if a.bool("query") {
        let Some(n) = name else { fail!("itemFilter: name a filter.") };
        let Some(q) = asked(a) else { fail!("itemFilter: name what to ask about.") };
        return with(|o| {
            let Some(x) = o.filters.get(&n) else { fail!("itemFilter: Object '{n}' not found.") };
            Ok(x.get(&q).cloned().unwrap_or(Value::Null))
        });
    }
    if a.bool("edit") {
        let Some(n) = name else { fail!("itemFilter: name a filter.") };
        return with(|o| {
            let Some(x) = o.filters.get_mut(&n) else { fail!("itemFilter: Object '{n}' not found.") };
            x.extend(flags);
            Ok(json!(n))
        });
    }
    let n = name.unwrap_or_else(|| fresh("itemFilter", |t| with(|o| o.filters.contains_key(t))));
    with(|o| o.filters.insert(n.clone(), flags));
    Ok(json!(n))
}

fn ls_through_filter(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.pos_str(0).map(str::to_string) else { fail!("lsThroughFilter: name a filter.") };
    if !with(|o| o.filters.contains_key(&name)) {
        fail!("lsThroughFilter: Object '{name}' not found.");
    }
    let g = &s.graph;
    let given = strings(a.get("item"));
    let selected: Vec<String> = if a.bool("selection") { g.selection().iter().map(|&i| g.path(i)).collect() } else { Vec::new() };
    let mut out = Vec::new();
    for id in g.nodes() {
        let Some(n) = g.node(id) else { continue };
        let path = g.path(id);
        if !given.is_empty() && !given.contains(&path) {
            continue;
        }
        if a.bool("selection") && !selected.contains(&path) {
            continue;
        }
        // A node passes as its own type or any it inherits from.
        let mut types = Vec::new();
        let mut t = Some(n.ty());
        while let Some(x) = t {
            types.push(x.name.clone());
            t = x.base.as_deref();
        }
        let short = path.rsplit('|').next().unwrap_or(&path).to_string();
        if with(|o| filter_matches(&o.filters, &name, &short, &types, 0)) {
            out.push(path);
        }
    }
    if a.bool("sort") {
        out.sort();
    }
    if a.bool("reverse") {
        out.reverse();
    }
    Ok(json!(out))
}

// ---------------------------------------------------------------------------------------------------------------------
// selectionConnection

const LIST_KINDS: &[&str] = &["activeList", "modelList", "keyframeList", "worldList", "objectList", "highlightList", "connectionList", "characterList", "setList", "activeCharacterList", "activeCacheList"];

/// What an activeCharacterList holds: the characters the selected objects belong to.
fn active_characters(s: &Session) -> Vec<String> {
    let g = &s.graph;
    let picked: Vec<_> = g.selection().to_vec();
    let mut out = Vec::new();
    for id in g.nodes() {
        if !aztek_scene::characters::is_character(g, id) {
            continue;
        }
        let members = aztek_scene::characters::member_plugs(g, id);
        if members.iter().any(|p| picked.contains(&p.node) || g.node(p.node).is_some_and(|n| n.ty().dag && picked.contains(&g.parent(p.node)))) {
            out.push(g.path(id));
        }
    }
    out
}

/// What an activeCacheList holds: the caches feeding the selected objects.
fn active_caches(s: &Session) -> Vec<String> {
    let g = &s.graph;
    let mut out = Vec::new();
    for &id in g.selection() {
        let mut start = vec![id];
        start.extend(aztek_scene::dag::shapes(g, id, false));
        for n in start {
            for h in aztek_scene::history::history_of(g, n) {
                if g.node(h).is_some_and(|x| x.ty().is_a("cacheFile") || x.ty().name.to_lowercase().contains("cache")) {
                    let p = g.path(h);
                    if !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

/// What a connection holds now, followed through editors and the special lists.
fn objects_of(s: &Session, name: &str, depth: usize) -> Vec<String> {
    let active: Vec<String> = s.graph.selection().iter().map(|&i| s.graph.path(i)).collect();
    let Some((kind, objects, editor)) = with(|o| o.connections.get(name).map(|c| (c.kind.clone(), c.objects.clone(), c.editor.clone()))) else {
        // The lists every scene has.
        return match name {
            "activeList" | "modelList" => active,
            _ => Vec::new(),
        };
    };
    match kind.as_str() {
        "activeList" | "modelList" => active,
        "activeCharacterList" => active_characters(s),
        "activeCacheList" => active_caches(s),
        "editorList" if depth < 8 => {
            let mlc = crate::scriptui::control_prop(&editor, "mainListConnection").and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
            if mlc.is_empty() {
                Vec::new()
            } else {
                objects_of(s, &mlc, depth + 1)
            }
        }
        _ => objects,
    }
}

fn selection_connection(s: &mut Session, a: &Args) -> CmdResult {
    let name = a.pos_str(0).map(str::to_string);
    if a.bool("exists") && !a.bool("query") {
        return Ok(json!(name.is_some_and(|n| with(|o| o.connections.contains_key(&n)))));
    }
    // -defineTemplate: the flags are kept as defaults for later calls.
    if let Some(t) = a.get("defineTemplate").and_then(Value::as_str) {
        if !a.bool("query") && !a.bool("edit") {
            crate::scriptui::define_template(t, "selectionConnection", &a.flags);
            return Ok(json!("selectionConnection"));
        }
    }
    if a.bool("query") {
        let Some(n) = name else { fail!("selectionConnection: name a connection.") };
        let Some(q) = asked(a) else { fail!("selectionConnection: name what to ask about.") };
        if q == "object" && with(|o| o.connections.contains_key(&n)) {
            let mut list = objects_of(s, &n, 0);
            if let Some(m) = a.get("maxObjects").and_then(Value::as_i64) {
                list.truncate(m.max(0) as usize);
            }
            return Ok(json!(list));
        }
        return with(|o| {
            let Some(x) = o.connections.get(&n) else { fail!("selectionConnection: Object '{n}' not found.") };
            Ok(match q.as_str() {
                "editor" => json!(x.editor),
                "filter" => json!(x.filter),
                "parent" => json!(x.parent),
                "lock" => json!(x.locked),
                "identify" => json!(if x.kind.is_empty() { "connection" } else { x.kind.as_str() }),
                k if LIST_KINDS.contains(&k) => json!(x.kind == k),
                _ => Value::Null,
            })
        });
    }
    // What a template gives comes first; what the call says wins.
    let merged;
    let a = if a.bool("edit") {
        a
    } else {
        let mut flags = crate::scriptui::template_defaults("selectionConnection", a.get("useTemplate").and_then(Value::as_str));
        for (k, v) in &a.flags {
            if !v.is_null() || !flags.contains_key(k) {
                flags.insert(k.clone(), v.clone());
            }
        }
        merged = Args { positional: a.positional.clone(), flags };
        &merged
    };
    let editor = a.get("editor").and_then(Value::as_str).map(str::to_string);
    let kind = LIST_KINDS.iter().find(|k| a.bool(k)).map(|k| k.to_string()).or_else(|| editor.as_ref().map(|_| "editorList".to_string()));
    if !a.bool("edit") {
        let n = name.unwrap_or_else(|| fresh("selectionConnection", |t| with(|o| o.connections.contains_key(t))));
        let taken = with(|o| o.connections.contains_key(&n));
        if taken {
            fail!("selectionConnection: Object's name '{n}' is not unique.");
        }
        let c = Connection { kind: kind.unwrap_or_else(|| "objectList".to_string()), editor: editor.unwrap_or_default(), ..Default::default() };
        with(|o| o.connections.insert(n.clone(), c));
        edit_connection(s, &n, a)?;
        return Ok(json!(n));
    }
    if let Some(e) = &editor {
        with(|o| {
            if let Some(c) = o.connections.get_mut(name.as_deref().unwrap_or("")) {
                c.editor = e.clone();
            }
        });
    }
    let Some(n) = name else { fail!("selectionConnection: name a connection.") };
    if !with(|o| o.connections.contains_key(&n)) {
        fail!("selectionConnection: Object '{n}' not found.");
    }
    if let Some(k) = kind {
        with(|o| o.connections.get_mut(&n).unwrap().kind = k);
    }
    edit_connection(s, &n, a)?;
    Ok(json!(n))
}

fn edit_connection(s: &mut Session, n: &str, a: &Args) -> CmdResult<()> {
    let to_scene = with(|o| o.connections.get(n).is_some_and(|c| c.kind == "activeList"));
    let locked = with(|o| o.connections.get(n).is_some_and(|c| c.locked));
    if a.has("lock") {
        with(|o| o.connections.get_mut(n).unwrap().locked = a.bool("lock"));
    }
    if let Some(f) = a.get("filter").map(text) {
        with(|o| o.connections.get_mut(n).unwrap().filter = f);
    }
    if let Some(p) = a.get("parent").map(text) {
        with(|o| o.connections.get_mut(n).unwrap().parent = p);
    }
    if locked {
        return Ok(());
    }
    let added: Vec<String> = ["object", "select", "addTo"].iter().flat_map(|k| strings(a.get(k))).collect();
    let removed: Vec<String> = ["deselect", "remove"].iter().flat_map(|k| strings(a.get(k))).collect();
    if a.bool("clear") {
        if to_scene {
            s.run("select", json!({"clear": true}))?;
        }
        with(|o| o.connections.get_mut(n).unwrap().objects.clear());
    }
    if to_scene {
        if !added.is_empty() {
            s.run("select", json!({"args": added, "add": true}))?;
        }
        if !removed.is_empty() {
            s.run("select", json!({"args": removed, "deselect": true}))?;
        }
        return Ok(());
    }
    with(|o| {
        let c = o.connections.get_mut(n).unwrap();
        for x in added {
            if !c.objects.contains(&x) {
                c.objects.push(x);
            }
        }
        c.objects.retain(|x| !removed.contains(x));
    });
    Ok(())
}

/// What a connection lists for the editors reading it: None for one that means the whole scene.
pub fn connection_objects(name: &str) -> Option<Vec<String>> {
    with(|o| o.connections.get(name).filter(|c| !matches!(c.kind.as_str(), "worldList" | "activeList" | "modelList")).map(|c| c.objects.clone()))
}

/// Whether a selection connection has this name, and what it holds: None for one that follows the selection.
pub fn connection_members(name: &str) -> Option<Option<Vec<String>>> {
    with(|o| {
        o.connections.get(name).map(|c| {
            if matches!(c.kind.as_str(), "activeList" | "modelList") || (c.kind != "worldList" && c.objects.is_empty() && c.kind.is_empty()) {
                None
            } else {
                Some(c.objects.clone())
            }
        })
    })
}

// ---------------------------------------------------------------------------------------------------------------------
// panelConfiguration

fn panel_configuration(a: &Args) -> CmdResult {
    let name = a.pos_str(0).map(str::to_string);
    if a.bool("exists") && !a.bool("query") {
        return Ok(json!(name.is_some_and(|n| with(|o| o.configs.contains_key(&n)))));
    }
    if a.bool("query") {
        let Some(n) = name else { fail!("panelConfiguration: name a configuration.") };
        let Some(q) = asked(a) else { fail!("panelConfiguration: name what to ask about.") };
        return with(|o| {
            let Some(x) = o.configs.get(&n) else { fail!("panelConfiguration: Object '{n}' not found.") };
            let panels: Vec<Value> = x.get("panels").and_then(Value::as_array).cloned().unwrap_or_default();
            let column = |i: usize| Value::Array(panels.iter().map(|p| p.get(i).cloned().unwrap_or(Value::Null)).collect());
            Ok(match q.as_str() {
                // Each panel as -addPanel gives it: fixed, label, type, create string, edit string.
                "isFixedState" => Value::Array(panels.iter().map(|p| json!(p.get(0).map(|v| v.as_bool().map_or(v.as_i64().unwrap_or(0), |b| b as i64)).unwrap_or(0))).collect()),
                "labelStrings" => column(1),
                "typeStrings" => column(2),
                "createStrings" => column(3),
                "editStrings" => column(4),
                "numberOfPanels" => json!(panels.len()),
                "label" => x.get("label").cloned().unwrap_or(json!(n)),
                k => x.get(k).cloned().unwrap_or(Value::Null),
            })
        });
    }
    let n = if a.bool("edit") {
        let Some(n) = name else { fail!("panelConfiguration: name a configuration.") };
        if !with(|o| o.configs.contains_key(&n)) {
            fail!("panelConfiguration: Object '{n}' not found.");
        }
        n
    } else {
        let n = name.unwrap_or_else(|| fresh("panelConfiguration", |t| with(|o| o.configs.contains_key(t))));
        with(|o| { o.configs.entry(n.clone()).or_default(); });
        n
    };
    with(|o| {
        let x = o.configs.get_mut(&n).unwrap();
        let mut panels: Vec<Value> = x.get("panels").and_then(Value::as_array).cloned().unwrap_or_default();
        if a.bool("removeAllPanels") {
            panels.clear();
        }
        if a.bool("removeLastPanel") {
            panels.pop();
        }
        for chunk in a.get("addPanel").map(flat).unwrap_or_default().chunks(5) {
            panels.push(json!(chunk));
        }
        // -replacePanel: the panel numbered from 1 given again whole.
        let whole = a.get("replacePanel").map(flat).unwrap_or_default();
        for chunk in whole.chunks(6).filter(|c| c.len() == 6) {
            let at = chunk[0].as_i64().unwrap_or(1).max(1) as usize - 1;
            if at < panels.len() {
                panels[at] = json!(chunk[1..].to_vec());
            } else if at == panels.len() {
                panels.push(json!(chunk[1..].to_vec()));
            }
        }
        // -replace... name a panel by its number from 1, then the new value.
        for (flag, col) in [("replaceFixedState", 0), ("replaceLabel", 1), ("replaceTypeString", 2), ("replaceCreateString", 3), ("replaceEditString", 4)] {
            let v = a.get(flag).map(flat).unwrap_or_default();
            if let [i, val, ..] = v.as_slice() {
                let at = i.as_i64().unwrap_or(1).max(1) as usize - 1;
                if let Some(row) = panels.get_mut(at).and_then(Value::as_array_mut) {
                    if col < row.len() {
                        row[col] = val.clone();
                    }
                }
            }
        }
        x.insert("panels".to_string(), Value::Array(panels));
        for k in ["label", "configString", "defaultImage", "image", "sceneConfig", "userCreated"] {
            // The default image, once set, stays.
            if k == "defaultImage" && x.contains_key(k) {
                continue;
            }
            if let Some(v) = a.get(k) {
                x.insert(k.to_string(), v.clone());
            }
        }
    });
    Ok(json!(n))
}

pub fn all_configs() -> Vec<String> {
    with(|o| o.configs.keys().cloned().collect())
}

pub fn config_with_label(label: &str) -> Option<String> {
    with(|o| o.configs.iter().find(|(n, x)| x.get("label").map_or(n.as_str() == label, |l| text(l) == label)).map(|(n, _)| n.clone()))
}

// ---------------------------------------------------------------------------------------------------------------------
// editorTemplate

fn editor_template(a: &Args) -> CmdResult {
    let item = |kind: &str| -> Map<String, Value> {
        let mut m = Map::from_iter([("kind".to_string(), json!(kind))]);
        for k in ["label", "annotation"] {
            if let Some(v) = a.get(k) {
                m.insert(k.to_string(), v.clone());
            }
        }
        // -annotateFieldOnly: the annotation shows over the value field alone, not the whole row.
        if a.has("annotation") && a.bool("annotateFieldOnly") {
            m.insert("annotateFieldOnly".to_string(), json!(true));
        }
        m
    };
    if a.has("forceRebuild") {
        // The editor builds the template afresh each time rather than replacing what it built.
        let mut m = item("forceRebuild");
        m.insert("on".to_string(), json!(a.get("forceRebuild").map_or(true, |v| v.as_bool().unwrap_or(v.as_i64() != Some(0)))));
        with(|o| o.template.push(Value::Object(m)));
        return Ok(Value::Null);
    }
    if a.bool("beginNoOptimize") || a.bool("endNoOptimize") {
        // The rows between keep their own spacing, not packed to save room.
        let kind = if a.bool("beginNoOptimize") { "beginNoOptimize" } else { "endNoOptimize" };
        with(|o| o.template.push(Value::Object(item(kind))));
        return Ok(Value::Null);
    }
    if a.bool("addComponents") {
        with(|o| o.template.push(Value::Object(item("components"))));
        return Ok(Value::Null);
    }
    let positional: Vec<String> = a.named();
    let push = |m: Map<String, Value>| with(|o| o.template.push(Value::Object(m)));
    if a.bool("beginScrollLayout") {
        with(|o| o.template.clear());
    } else if a.bool("endScrollLayout") {
        // The template stays until the editor has read it.
    } else if a.has("beginLayout") {
        let mut m = item("beginLayout");
        m.insert("title".to_string(), json!(text(a.get("beginLayout").unwrap())));
        m.insert("collapse".to_string(), json!(a.has("collapse") && a.bool("collapse")));
        push(m);
    } else if a.bool("endLayout") {
        push(item("endLayout"));
    } else if a.bool("addSeparator") {
        push(item("separator"));
    } else if a.bool("addExtraControls") {
        let mut m = item("extraControls");
        if let Some(t) = a.get("extraControlsLabel") {
            m.insert("title".to_string(), t.clone());
        }
        push(m);
    } else if let Some(attrs) = a.get("addControl") {
        let names = strings(Some(attrs));
        let mut m = item("control");
        m.insert("attr".to_string(), json!(names.first().cloned().or_else(|| positional.first().cloned()).unwrap_or_default()));
        if let Some(cc) = names.get(1) {
            m.insert("changeCommand".to_string(), json!(cc));
        }
        push(m);
    } else if a.bool("addAdskAssetControls") {
        // Controls for an adskMaterial's dynamic attributes: there are no adskMaterial nodes here, so none to add.
    } else if let Some(attr) = a.get("suppress") {
        let mut m = item("suppress");
        m.insert("attr".to_string(), json!(text(attr)));
        push(m);
    } else if let Some(v) = a.get("callCustom") {
        let mut m = item("custom");
        let parts = strings(Some(v));
        m.insert("newProc".to_string(), json!(parts.first().cloned().unwrap_or_default()));
        m.insert("replaceProc".to_string(), json!(parts.get(1).cloned().unwrap_or_default()));
        m.insert("attrs".to_string(), json!(parts.get(2..).map(<[String]>::to_vec).unwrap_or_default()));
        push(m);
    } else if let Some(v) = a.get("dimControl") {
        let parts = strings(Some(v));
        let mut m = item("dim");
        m.insert("node".to_string(), json!(parts.first().cloned().unwrap_or_default()));
        m.insert("attr".to_string(), json!(parts.get(1).cloned().unwrap_or_default()));
        m.insert("on".to_string(), json!(parts.get(2).is_some_and(|x| x == "true" || x == "1")));
        push(m);
    } else if a.bool("query") {
        if a.has("listExtraAttributes") {
            return Ok(json!(Vec::<String>::new()));
        }
        return Ok(Value::Null);
    }
    Ok(Value::Null)
}

/// The Attribute Editor's layout for a node, from a script's `AE<type>Template` procedure; empty with no such procedure.
/// `arg` is the type and the node, with a space between.
pub fn ae_template(core: &mut crate::core::Core, arg: &str) -> Value {
    let Some((ty, node)) = arg.split_once(' ') else { return json!([]) };
    let name = format!("AE{ty}Template");
    if !crate::mel::has_global_proc(core, &name) {
        return json!([]);
    }
    take_template();
    if let Err(m) = crate::mel::call_for_value(core, &format!("{name} \"{node}\";")) {
        core.push(format!("// Error: {m} //"), crate::core::LineKind::Error);
    }
    Value::Array(take_template())
}

/// What the last template said, and forgets it.
pub fn take_template() -> Vec<Value> {
    with(|o| std::mem::take(&mut o.template))
}

// ---------------------------------------------------------------------------------------------------------------------
// baseTemplate

fn base_template(a: &Args) -> CmdResult {
    let name = a.pos_str(0).map(str::to_string);
    if a.bool("exists") {
        return Ok(json!(name.is_some_and(|n| with(|o| o.base_templates.contains(&n)))));
    }
    if a.bool("viewList") {
        return Ok(json!(Vec::<String>::new()));
    }
    if a.bool("load") {
        let file = a.string("fileName", "");
        if !a.bool("silent") && !file.is_empty() && !std::path::Path::new(&file).exists() {
            fail!("baseTemplate: {file} not found.");
        }
        let n = name.unwrap_or_else(|| std::path::Path::new(&file).file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default());
        with(|o| {
            if !o.base_templates.contains(&n) {
                o.base_templates.push(n.clone());
            }
        });
        return Ok(json!(n));
    }
    if a.bool("unload") {
        if let Some(n) = name {
            with(|o| o.base_templates.retain(|x| *x != n));
        }
        return Ok(Value::Null);
    }
    if a.bool("matchFile") {
        return Ok(json!(false));
    }
    Ok(json!(with(|o| o.base_templates.clone())))
}

pub fn register(s: &mut Session) {
    *aztek_scene::ref_scene::INTERNAL_NODES.write().unwrap() = Some(|| with(|o| o.filters.keys().cloned().collect()));
    s.register(add(
        Cmd::new("itemFilter", "A named test of nodes for editors and lsThroughFilter: -byName, -byType, -union, -intersect, -difference, -negate.", |_s, a| item_filter(a)).positional("[name]").undoable(false),
        "byName:bn:a byType:bt:a1 byBin:bk:a1 byScript:bs:a secondScript:ss:a classification:cls:a clearByBin:cbk:b clearByType:cbt:b category:cat:a1 customCategory:ctg:a difference:di:a2 intersect:in:a2 union:un:a2 negate:neg:b listBuiltInFilters:lbf:b listOtherFilters:lof:b listUserFilters:luf:b parent:p:a text:t:a uniqueNodeNames:unn:b exists:ex:b query:q:b edit:e:b queried::a",
    ));
    s.register(add(
        Cmd::new("lsThroughFilter", "The nodes a filter passes: -item limits it to those named, -selection to the selected, -sort, -reverse.", ls_through_filter).positional("filter").undoable(false),
        "item:it:a1 nodeArray:na:b reverse:rv:b selection:sl:b sort:so:b",
    ));
    s.register(add(
        Cmd::new("selectionConnection", "A named list of objects for editors to show and follow: -activeList is the scene's selection, -worldList the scene, -objectList its own.", selection_connection).positional("[name]").undoable(false),
        "activeList:act:b activeCharacterList:acl:b activeCacheList:atc:b editor:ed:a modelList:mdl:b keyframeList:key:b worldList:wl:b objectList:ol:b highlightList:hl:b connectionList:lst:b characterList:cl:b setList:sl:b maxObjects:mxo:a defineTemplate:dt:a useTemplate:ut:a addScript:as:a removeScript:rs:a addTo:add:a1 clear:clr:b deselect:d:a1 select:s:a1 remove:rm:a1 object:obj:a1 filter:f:a findObject:fo:a global:g:a identify:id:b lock:lck:b parent:p:a switch:sw:b exists:ex:b query:q:b edit:e:b queried::a",
    ));
    s.register(add(
        Cmd::new("panelConfiguration", "A named arrangement of panels: -addPanel (fixed, label, type, create, edit), -replacePanel (number, then the same), -configString, -label; getPanel -allConfigs lists them.", |_s, a| panel_configuration(a)).positional("[name]").undoable(false),
        "addPanel:ap:a5 configString:cfs:a defaultImage:di:a image:i:a label:l:a labelStrings:ls:b createStrings:cs:b editStrings:es:b typeStrings:ts:b isFixedState:if:b numberOfPanels:np:b removeAllPanels:rap:b removeLastPanel:rlp:b replaceCreateString:rcs:a2 replaceEditString:res:a2 replaceFixedState:rfs:a2 replaceLabel:rl:a2 replaceTypeString:rts:a2 replacePanel:rp:a6 sceneConfig:sc:b userCreated:uc:b defineTemplate:dt:a useTemplate:ut:a exists:ex:b query:q:b edit:e:b queried::a",
    ));
    s.register(add(
        Cmd::new("editorTemplate", "Inside an AE<type>Template procedure: lays out the Attribute Editor with -beginLayout, -addControl, -suppress, -addSeparator, -addExtraControls, -callCustom.", |_s, a| editor_template(a)).positional("[attribute]").undoable(false),
        "addAdskAssetControls:aac:b addComponents:acp:b annotateFieldOnly:afo:b forceRebuild:fr:b addControl:ac:a addDynamicControl:adc:a addExtraControls:aec:b addSeparator:as:b annotation:ann:a beginLayout:bl:a beginNoOptimize:bn:b beginScrollLayout:bsl:b callCustom:ccu:a callback:cb:a2 collapse:cl:b debugMode:dbm:b dimControl:dc:a endLayout:el:b endNoOptimize:en:b endScrollLayout:esl:b extraControlsLabel:ecl:a interruptOptimize:io:b label:l:a listExtraAttributes:lea:a preventOverride:po:b queryControl:qc:a queryLabel:ql:a queryName:qn:a removeControl:rc:a suppress:s:a query:q:b edit:e:b queried::a",
    ));
    s.register(add(
        Cmd::new("baseTemplate", "Base node templates: -load -fileName, -unload, -exists. Their contents are not read here: the name is kept.", |_s, a| base_template(a)).positional("[name]").undoable(false),
        "exists:ex:b fileName:fn:a force:f:b load:l:b matchFile:mf:b silent:si:b unload:u:b viewList:vl:b query:q:b edit:e:b",
    ));
}
