//! Character sets and their clips, in the reference's layout (Maya2glTF's TwoJointsLogicalParentAnim.ma, saved by the
//! reference, under tests/data/ma_characters).
//!
//! A `character` lists its members through `.dnsm` and drives each through a value of its own (`.lv[i]` linear,
//! `.av[i]` angular, `.uv[i]` unitless), named by `.am` (characterMapping: plug, kind, index) and `.aal`. Keys on a
//! member go to that value. A `clipLibrary` holds the source clips: one `animClip` each on `.sc[s]`, with a curve
//! per member on `.cel[s].cev[k].cevr`. A `clipScheduler` places the clip instances: each an `animClip` whose times,
//! scale, cycles and weight feed the scheduler's arrays at the clip's index; tracks are `.tr[j]`.
//!
//! Aztek keeps a character's clips as data on its library and scheduler (`characters.rs`); this file turns that into
//! the layout above and back.

use super::*;
use crate::characters as ch;

/// Which value a member goes through, by its attribute's unit: (kind, the reference's short name, long name).
fn kind_of(g: &Graph, p: &PlugRef) -> (u8, &'static str, &'static str) {
    match g.attr(p).unit {
        units::Unit::Linear => (1, "lv", "linearValues"),
        units::Unit::Angle => (2, "av", "angularValues"),
        _ => (0, "uv", "unitlessValues"),
    }
}

fn kind_unit(kind: u8) -> units::Unit {
    match kind {
        1 => units::Unit::Linear,
        2 => units::Unit::Angle,
        _ => units::Unit::None,
    }
}

fn curve_type(kind: u8) -> &'static str {
    match kind {
        1 => "animCurveTL",
        2 => "animCurveTA",
        _ => "animCurveTU",
    }
}

/// A member's attribute by its long name: translateX, not translate.
fn member_attr(g: &Graph, p: &PlugRef) -> String {
    let a = g.attr(p);
    if p.component >= 0 && a.ty == PlugType::Vec3 {
        a.child_names[p.component as usize].clone()
    } else {
        a.name.clone()
    }
}

fn yes_no(list: &[bool]) -> String {
    list.iter().map(|b| if *b { "yes" } else { "no" }).collect::<Vec<_>>().join(" ")
}

impl Writer<'_> {
    /// Before anything is written: which member each character value drives, so a member's keys and its clip
    /// channel's are written into the character instead.
    pub(super) fn plan_characters(&mut self) {
        let g = &self.s.graph;
        for c in g.nodes() {
            if !self.keep.contains(&c) || !ch::is_character(g, c) {
                continue;
            }
            let name = g.path(c);
            let mut counts = [0usize; 3];
            for p in ch::saved(g, c).members {
                let (kind, short, _) = kind_of(g, &p);
                counts[kind as usize] += 1;
                let target = format!("{name}.{short}[{}]", counts[kind as usize]);
                // A member that clips play: its channel's keys go to the character.
                if let Some(src) = g.source(&p).or_else(|| g.source(&p.whole())) {
                    if ch::is_clip_channel(g, src.node) {
                        self.char_channels.insert(src.node, target.clone());
                    }
                }
                self.char_driven.insert(p, target);
            }
        }
    }

    /// The character, its library and scheduler, their animClips and the curves of the source clips.
    pub(super) fn write_character(&mut self, c: NodeId) {
        struct Member {
            plug: String,
            alias_node: String,
            attr: String,
            kind: u8,
            short: &'static str,
            long: &'static str,
            index: usize,
            value: f64,
        }
        let name = self.s.graph.path(c);
        let saved = ch::saved(&self.s.graph, c);
        let mut counts = [0usize; 3];
        let mut members = Vec::new();
        for p in &saved.members {
            let value = match self.s.graph.value(p) {
                Ok(v) => v.scalar().unwrap_or(0.0),
                Err(_) => 0.0,
            };
            let g = &self.s.graph;
            let (kind, short, long) = kind_of(g, p);
            counts[kind as usize] += 1;
            let node = g.path(p.node);
            let attr = member_attr(g, p);
            let alias_node = node.rsplit('|').next().unwrap_or(&node).replace(':', "_");
            members.push(Member { plug: format!("{node}.{attr}"), alias_node, attr, kind, short, long, index: counts[kind as usize], value: units::to_user(kind_unit(kind), value) });
        }
        let n = members.len();
        let mut text = format!("createNode character -n {};\n", quoted(&name));
        text.push_str("\taddAttr -ci true -h true -sn \"aal\" -ln \"attributeAliasList\" -dt \"attributeAlias\";\n");
        if n > 0 {
            let _ = writeln!(text, "\tsetAttr -s {n} \".dnsm\";");
        }
        for m in &members {
            let _ = writeln!(text, "\tsetAttr \".{}[{}]\" {};", m.short, m.index, m.value);
        }
        for (kind, short) in [(1usize, "lv"), (2, "av"), (0, "uv")] {
            if counts[kind] > 0 {
                let _ = writeln!(text, "\tsetAttr -s {} \".{short}\";", counts[kind]);
            }
        }
        if n > 0 {
            let mapping: Vec<String> = members.iter().map(|m| format!("{} {} {}", quoted(&m.plug), m.kind, m.index)).collect();
            let _ = writeln!(text, "\tsetAttr \".am\" -type \"characterMapping\" {n} {} ;", mapping.join(" "));
            let ids: Vec<String> = (0..n).map(|k| k.to_string()).collect();
            let _ = writeln!(text, "\tsetAttr \".cim\" -type \"Int32Array\" {n} {} ;", ids.join(" "));
            let aliases: Vec<String> = members.iter().map(|m| format!("{},{}", quoted(&format!("{}_{}", m.alias_node, m.attr)), quoted(&format!("{}[{}]", m.long, m.index)))).collect();
            let _ = writeln!(text, "\tsetAttr \".aal\" -type \"attributeAlias\" {{{}}} ;", aliases.join(","));
        }
        for (k, m) in members.iter().enumerate() {
            self.conn(&m.plug, &format!("{name}.dnsm[{k}]"));
            self.conn(&format!("{name}.{}[{}]", m.short, m.index), &m.plug);
        }
        if saved.has_poses {
            self.warnings.push(format!("{name}: poses are not written yet; left out"));
        }
        self.out.push_str(&text);

        let (Some((lib, sources)), Some((sched, clips))) = (saved.library, saved.scheduler) else { return };
        let sources: Vec<(String, Value)> = sources.as_object().map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default();
        let clips: Vec<Value> = clips.as_array().cloned().unwrap_or_default();

        // The library, its source clips and their curves.
        let mut lt = format!("createNode clipLibrary -n {};\n", quoted(&lib));
        if !sources.is_empty() {
            let _ = writeln!(lt, "\tsetAttr -s {} \".cel\";", sources.len());
            for s in 0..sources.len() {
                let _ = writeln!(lt, "\tsetAttr -s {n} \".cel[{s}].cev\";");
            }
            let _ = writeln!(lt, "\tsetAttr -s {} \".sc\";", sources.len());
        }
        if !clips.is_empty() {
            let _ = writeln!(lt, "\tsetAttr -s {} \".cl\";", clips.len());
        }
        self.out.push_str(&lt);
        self.conn(&format!("{name}.am"), &format!("{lib}.cd[0].cm"));
        self.conn(&format!("{name}.cim"), &format!("{lib}.cd[0].cim"));
        for (s, (src, info)) in sources.iter().enumerate() {
            let f = |k: &str| info.get(k).and_then(Value::as_f64).unwrap_or(0.0);
            let _ = write!(self.out, "createNode animClip -n {};\n\tsetAttr \".ihi\" 0;\n\tsetAttr \".ss\" {};\n\tsetAttr \".se\" {};\n\tsetAttr \".ci\" no;\n", quoted(src), f("start"), f("end"));
            self.conn(&format!("{src}.cl"), &format!("{lib}.sc[{s}]"));
            for (k, m) in members.iter().enumerate() {
                let Some(keys) = info.get("curves").and_then(|c| c.get(k.to_string())) else { continue };
                let mut keys = anim::Keys::from_json(keys);
                keys.weighted = keys.keys.iter().any(|x| x.in_weight != 1.0 || x.out_weight != 1.0);
                let cname = self.unique_name(&format!("{src}_{}_{}", m.alias_node, m.attr));
                let lines = curve_lines(curve_type(m.kind), &cname, &keys, 0, 0);
                self.out.push_str(&lines);
                self.conn(&format!("{cname}.a"), &format!("{lib}.cel[{s}].cev[{k}].cevr"));
            }
        }

        // The scheduler and the clips it places.
        let mut st = format!("createNode clipScheduler -n {};\n", quoted(&sched));
        let m = clips.len();
        if m > 0 {
            for a in ["cl", "st", "ss", "se", "sc", "h", "w", "ws", "cb", "ca", "ea"] {
                let _ = writeln!(st, "\tsetAttr -s {m} \".{a}\";");
            }
            let tracks: Vec<String> = clips.iter().map(|c| (c.get("track").and_then(Value::as_i64).unwrap_or(0) + 1).to_string()).collect();
            let _ = writeln!(st, "\tsetAttr -s {m} \".tr[0:{}]\"  {};", m - 1, tracks.join(" "));
            let top = clips.iter().map(|c| c.get("track").and_then(Value::as_i64).unwrap_or(0) + 1).max().unwrap_or(1);
            let _ = writeln!(st, "\tsetAttr \".nt\" {top};");
            // Mute and solo are a track's state in the reference: 2 soloed, 4 muted.
            let mut states: BTreeMap<i64, i64> = BTreeMap::new();
            for c in &clips {
                let t = c.get("track").and_then(Value::as_i64).unwrap_or(0) + 1;
                let b = |k: &str| c.get(k).and_then(Value::as_bool).unwrap_or(false);
                let bits = if b("solo") { 2 } else { 0 } | if b("mute") { 4 } else { 0 } | if b("lock") { 1 } else { 0 };
                *states.entry(t).or_default() |= bits;
            }
            for (t, bits) in states.into_iter().filter(|(_, b)| *b != 0) {
                let _ = writeln!(st, "\tsetAttr \".ts[{t}]\" {bits};");
            }
        }
        self.out.push_str(&st);
        self.conn(&format!("{lib}.cf"), &format!("{sched}.cfr"));
        self.conn(&format!("{sched}.ce"), &format!("{name}.cer"));
        for (j, c) in clips.iter().enumerate() {
            let f = |k: &str, d: f64| c.get(k).and_then(Value::as_f64).unwrap_or(d);
            let b = |k: &str, d: bool| c.get(k).and_then(Value::as_bool).unwrap_or(d);
            let cname = c.get("name").and_then(Value::as_str).unwrap_or("clip1").to_string();
            let src = c.get("source").and_then(Value::as_str).unwrap_or("").to_string();
            let mut t = format!("createNode animClip -n {};\n", quoted(&cname));
            if n > 0 {
                let _ = writeln!(t, "\tsetAttr -s {n} \".ac[0:{}]\" {};", n - 1, yes_no(&vec![!b("relative", false); n]));
            }
            let _ = writeln!(t, "\tsetAttr \".sf\" {};", f("start", 1.0));
            let _ = writeln!(t, "\tsetAttr \".ss\" {};", f("sourceStart", 1.0));
            let _ = writeln!(t, "\tsetAttr \".se\" {};", f("sourceEnd", 1.0));
            if f("scale", 1.0) != 1.0 {
                let _ = writeln!(t, "\tsetAttr \".sc\" {};", f("scale", 1.0));
            }
            if f("preCycle", 0.0) != 0.0 {
                let _ = writeln!(t, "\tsetAttr \".cb\" {};", f("preCycle", 0.0));
            }
            // Aztek counts the clip's own play as a cycle; the reference's postCycle counts the repeats after it.
            let post = (f("postCycle", 1.0) - 1.0).max(0.0);
            if post != 0.0 {
                let _ = writeln!(t, "\tsetAttr \".ca\" {post};");
            }
            if f("hold", 0.0) != 0.0 {
                let _ = writeln!(t, "\tsetAttr \".h\" {};", f("hold", 0.0));
            }
            if f("weight", 1.0) != 1.0 {
                let _ = writeln!(t, "\tsetAttr \".w\" {};", f("weight", 1.0));
            }
            if !b("enabled", true) {
                t.push_str("\tsetAttr \".ea\" no;\n");
            }
            self.out.push_str(&t);
            if !src.is_empty() {
                self.conn(&format!("{src}.cl"), &format!("{cname}.cl"));
            }
            self.conn(&format!("{lib}.cl[{j}]"), &format!("{sched}.cl[{j}]"));
            for (from, to) in [("sf", "st"), ("ss", "ss"), ("se", "se"), ("w", "w"), ("ws", "ws"), ("ea", "ea"), ("cb", "cb"), ("ca", "ca"), ("sc", "sc"), ("h", "h")] {
                self.conn(&format!("{cname}.{from}"), &format!("{sched}.{to}[{j}]"));
            }
        }
    }

    /// A name no other node in the file has.
    fn unique_name(&mut self, wanted: &str) -> String {
        let base: String = wanted.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect();
        let mut name = base.clone();
        let mut k = 1;
        while self.used.contains(&name) || self.s.graph.find(&name).ok().flatten().is_some() {
            name = format!("{base}{k}");
            k += 1;
        }
        self.used.insert(name.clone());
        name
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Reading

/// A character as the file has it, taken out of the document's connections before the scene is built.
#[derive(Debug)]
pub(super) struct CharFile {
    node: usize,
    /// Each member's node and attribute, in the order of `.am`.
    members: Vec<(String, String, u8)>,
    library: Option<usize>,
    scheduler: Option<usize>,
    /// Each source clip: its animClip and, per member position, the curve.
    sources: Vec<(usize, Vec<(usize, usize)>)>,
    /// Each scheduled clip: its index in the scheduler, its animClip and its source's.
    clips: Vec<(usize, usize, Option<usize>)>,
}

/// "lv[3]" → (kind 1, 3).
fn value_plug(attr: &str) -> Option<(u8, usize)> {
    let (name, rest) = attr.split_once('[')?;
    let i: usize = rest.strip_suffix(']')?.parse().ok()?;
    let kind = match name {
        "lv" | "linearValues" => 1,
        "av" | "angularValues" => 2,
        "uv" | "unitlessValues" => 0,
        _ => return None,
    };
    Some((kind, i))
}

/// The indices in "cel[0].cev[3].cevr" or "sc[2]".
fn indices(attr: &str) -> Vec<usize> {
    attr.split('[').skip(1).filter_map(|p| p.split(']').next()?.parse().ok()).collect()
}

fn base(attr: &str) -> &str {
    attr.split(['[', '.']).next().unwrap_or("")
}

/// Takes the characters, libraries, schedulers and clips out of the document: their connections are read here and
/// leave the list, and keys on a character's value go to the member it drives, as Aztek keys a member.
pub(super) fn take(d: &mut Doc) -> Vec<CharFile> {
    let mut out = Vec::new();
    let chars: Vec<usize> = (0..d.nodes.len()).filter(|&i| d.nodes[i].ty == "character").collect();
    if chars.is_empty() {
        return out;
    }
    let is = |d: &Doc, i: usize, ty: &str| d.nodes[i].ty == ty;
    for c in chars {
        let n = &d.nodes[c];
        // characterMapping: count, then "node.attr" kind index for each.
        let mut members = Vec::new();
        if let Some(set) = n.sets.iter().rev().find(|s| s.path.len() == 1 && matches!(s.path[0].name.as_str(), "am" | "animationMapping") && !s.values.is_empty()) {
            for e in set.values[1..].chunks(3) {
                if let [Tok::Str(plug), kind, idx] = e {
                    if let (Some((node, attr)), Some(k), Some(i)) = (plug.split_once('.'), kind.num(), idx.num()) {
                        members.push((node.to_string(), attr.to_string(), k as u8, i as usize));
                    }
                }
            }
        }
        let library = d.conns.iter().find(|x| x.src == c && base(&x.src_attr) == "am" && is(d, x.dst, "clipLibrary")).map(|x| x.dst);
        let scheduler = d.conns.iter().find(|x| x.dst == c && matches!(base(&x.dst_attr), "cer" | "clipEvaluate_Raw" | "ce") && is(d, x.src, "clipScheduler")).map(|x| x.src).or_else(|| library.and_then(|l| d.conns.iter().find(|x| x.src == l && matches!(base(&x.src_attr), "cf" | "clipFunction") && is(d, x.dst, "clipScheduler")).map(|x| x.dst)));
        // Cim maps a member's position to its index in the library.
        let cim: Vec<usize> = n.sets.iter().rev().find(|s| s.path.len() == 1 && matches!(s.path[0].name.as_str(), "cim" | "clipIndexMap")).map(|s| s.values.iter().skip(1).filter_map(|t| t.num().map(|v| v as usize)).collect()).unwrap_or_default();
        let position_of = |k: usize| cim.iter().position(|&x| x == k).unwrap_or(k);
        let mut sources = Vec::new();
        let mut clips = Vec::new();
        if let Some(l) = library {
            let mut by_index: BTreeMap<usize, usize> = BTreeMap::new();
            for x in d.conns.iter().filter(|x| x.dst == l && base(&x.dst_attr) == "sc" && is(d, x.src, "animClip")) {
                if let Some(&s) = indices(&x.dst_attr).first() {
                    by_index.insert(s, x.src);
                }
            }
            for (s, clip) in by_index {
                let curves: Vec<(usize, usize)> = d.conns.iter().filter(|x| x.dst == l && base(&x.dst_attr) == "cel" && indices(&x.dst_attr).first() == Some(&s)).filter_map(|x| Some((position_of(*indices(&x.dst_attr).get(1)?), x.src))).collect();
                sources.push((clip, curves));
            }
        }
        if let Some(sc) = scheduler {
            let mut by_index: BTreeMap<usize, usize> = BTreeMap::new();
            for x in d.conns.iter().filter(|x| x.dst == sc && matches!(base(&x.dst_attr), "st" | "start") && is(d, x.src, "animClip")) {
                if let Some(&j) = indices(&x.dst_attr).first() {
                    by_index.insert(j, x.src);
                }
            }
            for (j, clip) in by_index {
                let source = d.conns.iter().find(|x| x.dst == clip && base(&x.dst_attr) == "cl" && is(d, x.src, "animClip")).map(|x| x.src);
                clips.push((j, clip, source));
            }
        }
        out.push(CharFile { node: c, members: members.iter().map(|m| (m.0.clone(), m.1.clone(), m.2)).collect(), library, scheduler, sources, clips });
        // Keys on the character's values go to the members they drive.
        let target: HashMap<(u8, usize), (String, String)> = members.iter().map(|m| ((m.2, m.3), (m.0.clone(), m.1.clone()))).collect();
        for x in d.conns.iter_mut() {
            if x.dst != c {
                continue;
            }
            if let Some((node, attr)) = value_plug(&x.dst_attr).and_then(|k| target.get(&k)) {
                // Resolved by name below.
                x.dst_attr = format!("\u{1}{node}\u{1}{attr}");
            }
        }
        let renamed: Vec<(usize, String, String)> = d.conns.iter().enumerate().filter_map(|(k, x)| x.dst_attr.strip_prefix('\u{1}').and_then(|r| r.split_once('\u{1}')).map(|(n, a)| (k, n.to_string(), a.to_string()))).collect();
        for (k, node, attr) in renamed {
            match d.find(&node) {
                Some(i) => {
                    d.conns[k].dst = i;
                    d.conns[k].dst_attr = attr;
                }
                None => d.conns[k].dst_attr = String::new(),
            }
        }
    }
    // Everything else into or out of these nodes is held by the data built from them.
    let held = |d: &Doc, i: usize| matches!(d.nodes[i].ty.as_str(), "character" | "clipLibrary" | "clipScheduler" | "animClip");
    let keep: Vec<bool> = d.conns.iter().map(|x| !x.dst_attr.is_empty() && !held(d, x.src) && !held(d, x.dst) && !(d.nodes[x.dst].ty == "clipLibrary")).collect();
    let mut it = keep.into_iter();
    d.conns.retain(|_| it.next().unwrap_or(true));
    out
}

impl Build<'_> {
    /// The characters taken out of the document, made with their clips once the members and their keys are in.
    pub(super) fn read_characters(&mut self, files: &[CharFile], fps: f64) {
        let doc = self.doc;
        for f in files {
            for i in [Some(f.node), f.library, f.scheduler].into_iter().flatten() {
                self.used[i] = true;
            }
            let mut plugs = Vec::new();
            for (node, attr, _) in &f.members {
                let p = doc.find(node).and_then(|i| self.ids[i]).and_then(|id| self.s.graph.plug(&format!("{}.{attr}", self.s.graph.path(id))).ok());
                match p {
                    Some(p) => plugs.push(p),
                    None => self.warnings.push(format!("{}: member {node}.{attr} not found; left out", doc.nodes[f.node].name)),
                }
            }
            let library = f.library.map(|l| {
                let mut o = serde_json::Map::new();
                for (clip, curves) in &f.sources {
                    self.used[*clip] = true;
                    let n = &doc.nodes[*clip];
                    let mut cs = serde_json::Map::new();
                    for &(k, curve) in curves {
                        self.used[curve] = true;
                        cs.insert(k.to_string(), crate::plug::Data::to_json(&curve_keys(&doc.nodes[curve], doc, fps)));
                    }
                    o.insert(n.name.clone(), json!({"start": n.num(&["ss", "sourceStart"]).unwrap_or(0.0), "end": n.num(&["se", "sourceEnd"]).unwrap_or(0.0), "curves": Value::Object(cs)}));
                }
                (doc.nodes[l].name.clone(), Value::Object(o))
            });
            let scheduler = f.scheduler.map(|sc| {
                let s = &doc.nodes[sc];
                let tracks: HashMap<usize, f64> = s.elements(&["tr", "track"], 1).into_iter().filter_map(|(i, v)| Some((i, v[0].num()?))).collect();
                let states: HashMap<usize, i64> = s.elements(&["ts", "trackState"], 1).into_iter().filter_map(|(i, v)| Some((i, v[0].num()? as i64))).collect();
                let mut list = Vec::new();
                for &(j, clip, source) in &f.clips {
                    self.used[clip] = true;
                    let n = &doc.nodes[clip];
                    let g = |names: &[&str], d: f64| n.num(names).unwrap_or(d);
                    let track = tracks.get(&j).copied().unwrap_or(1.0) as i64;
                    let state = states.get(&(track as usize)).copied().unwrap_or(0);
                    let relative = !n.elements(&["ac", "absoluteChannel"], 1).is_empty() && n.elements(&["ac", "absoluteChannel"], 1).iter().all(|(_, v)| v[0].truth() == Some(false));
                    list.push(json!({
                        "name": n.name, "source": source.map(|x| doc.nodes[x].name.clone()).unwrap_or_default(),
                        "start": g(&["sf", "startFrame"], 0.0), "sourceStart": g(&["ss", "sourceStart"], 0.0), "sourceEnd": g(&["se", "sourceEnd"], 0.0),
                        "scale": g(&["sc", "scale"], 1.0), "preCycle": g(&["cb", "preCycle"], 0.0), "postCycle": g(&["ca", "postCycle"], 0.0) + 1.0,
                        "hold": g(&["h", "hold"], 0.0), "weight": g(&["w", "weight"], 1.0), "enabled": n.truth(&["ea", "enable"]).unwrap_or(true),
                        "track": (track - 1).max(0), "relative": relative, "solo": state & 2 != 0, "mute": state & 4 != 0, "lock": state & 1 != 0,
                    }));
                }
                (s.name.clone(), Value::Array(list))
            });
            let name = doc.nodes[f.node].name.clone();
            if let Err(e) = ch::restore(&mut self.s.graph, &name, &plugs, library, scheduler) {
                self.warnings.push(format!("{name}: {e}"));
            }
        }
        // Clips no character names.
        for (i, n) in doc.nodes.iter().enumerate() {
            if n.ty == "animClip" {
                self.used[i] = true;
            }
        }
    }
}
