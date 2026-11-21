//! Sound, as the reference's audio node and sound command: a WAV file placed on the timeline at an offset, its length in
//! frames, and the waveform the time slider draws (timeControl -sound and -displaySound). Reading WAV is Aztek's own:
//! PCM at 8, 16, 24 or 32 bits and 32-bit float, any number of channels.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aztek_ops::{fail, ArgType, CmdResult, CommandDef, Commands, FlagDef};
use serde_json::{json, Value};

use crate::graph::Graph;
use crate::node::{types, NodeType};
use crate::plug::{flags, AttrDef, NodeId, PlugType, PlugValue};
use crate::session::Session;

pub const TYPE: &str = "audio";

/// A WAV file's samples, mixed to mono, as -1 to 1.
#[derive(Clone, Debug)]
pub struct Wave {
    pub rate: u32,
    pub channels: u16,
    pub samples: Vec<f32>,
}

impl Wave {
    pub fn seconds(&self) -> f64 {
        self.samples.len() as f64 / self.rate.max(1) as f64
    }
}

pub fn read_wav(bytes: &[u8]) -> Result<Wave, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not a WAV file".into());
    }
    let mut p = 12;
    let (mut fmt, mut data): (Option<(u16, u16, u32, u16)>, Option<&[u8]>) = (None, None);
    while p + 8 <= bytes.len() {
        let id = &bytes[p..p + 4];
        let n = u32::from_le_bytes(bytes[p + 4..p + 8].try_into().unwrap()) as usize;
        let body = &bytes[p + 8..(p + 8 + n).min(bytes.len())];
        match id {
            b"fmt " if body.len() >= 16 => {
                let mut format = u16::from_le_bytes([body[0], body[1]]);
                let channels = u16::from_le_bytes([body[2], body[3]]);
                let rate = u32::from_le_bytes(body[4..8].try_into().unwrap());
                let bits = u16::from_le_bytes([body[14], body[15]]);
                // WAVE_FORMAT_EXTENSIBLE: the real format is the sub-format's first two bytes.
                if format == 0xfffe && body.len() >= 26 {
                    format = u16::from_le_bytes([body[24], body[25]]);
                }
                fmt = Some((format, channels, rate, bits));
            }
            b"data" => data = Some(body),
            _ => {}
        }
        p += 8 + n + (n & 1);
    }
    let (format, channels, rate, bits) = fmt.ok_or("no fmt chunk")?;
    let data = data.ok_or("no data chunk")?;
    let ch = channels.max(1) as usize;
    let width = (bits as usize).div_ceil(8);
    if width == 0 {
        return Err("bad sample size".into());
    }
    let sample = |b: &[u8]| -> f32 {
        match (format, bits) {
            (3, 32) => f32::from_le_bytes(b.try_into().unwrap()),
            (1, 8) => (b[0] as f32 - 128.0) / 128.0,
            (1, 16) => i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0,
            (1, 24) => (((b[2] as i32) << 24 | (b[1] as i32) << 16 | (b[0] as i32) << 8) >> 8) as f32 / 8_388_608.0,
            (1, 32) => i32::from_le_bytes(b.try_into().unwrap()) as f32 / 2_147_483_648.0,
            _ => 0.0,
        }
    };
    if !matches!((format, bits), (3, 32) | (1, 8) | (1, 16) | (1, 24) | (1, 32)) {
        return Err(format!("WAV format {format} at {bits} bits is not read"));
    }
    let frame = width * ch;
    let samples = data.chunks_exact(frame).map(|f| f.chunks_exact(width).map(sample).sum::<f32>() / ch as f32).collect();
    Ok(Wave { rate, channels, samples })
}

fn cache() -> &'static Mutex<HashMap<String, (std::time::SystemTime, Arc<Wave>)>> {
    static C: std::sync::OnceLock<Mutex<HashMap<String, (std::time::SystemTime, Arc<Wave>)>>> = std::sync::OnceLock::new();
    C.get_or_init(Default::default)
}

/// A file's wave, read once while it stays the same.
pub fn wave(path: &str) -> Result<Arc<Wave>, String> {
    let stamp = std::fs::metadata(path).and_then(|m| m.modified()).map_err(|e| format!("{path}: {e}"))?;
    if let Some((t, w)) = cache().lock().unwrap().get(path) {
        if *t == stamp {
            return Ok(w.clone());
        }
    }
    let w = Arc::new(read_wav(&std::fs::read(path).map_err(|e| format!("{path}: {e}"))?).map_err(|e| format!("{path}: {e}"))?);
    cache().lock().unwrap().insert(path.to_string(), (stamp, w.clone()));
    Ok(w)
}

fn store(name: &str, short: &str, ty: PlugType, d: PlugValue, doc: &str) -> AttrDef {
    AttrDef { name: name.into(), short_name: short.into(), ty, default_value: d, flags: flags::INPUT | flags::STORABLE, doc: doc.into(), ..Default::default() }
}

pub fn register_types() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = types::register_type(NodeType {
            name: TYPE.into(),
            icon: "volume".into(),
            doc: "A sound on the timeline: a WAV file starting at its offset.".into(),
            own_attrs: vec![
                store("filename", "f", PlugType::String, PlugValue::String(String::new()), "The WAV file."),
                store("offset", "o", PlugType::Float, PlugValue::Float(0.0), "The frame the sound starts at."),
                store("sourceStart", "ss", PlugType::Float, PlugValue::Float(0.0), "Where in the file it starts, frames."),
                store("sourceEnd", "se", PlugType::Float, PlugValue::Float(0.0), "Where in the file it ends, frames; 0 the end of the file."),
                store("mute", "mu", PlugType::Bool, PlugValue::Bool(false), "Silent."),
                store("order", "ord", PlugType::Int, PlugValue::Int(0), "Its place among sounds."),
            ],
            create: Some(Arc::new(|| Box::new(crate::node::Plain))),
            ..Default::default()
        });
    });
}

fn val(g: &Graph, id: NodeId, attr: &str) -> PlugValue {
    g.node(id).and_then(|n| n.ty().attr_index(attr).map(|i| n.data().value(i).clone())).unwrap_or(PlugValue::Float(0.0))
}

/// (start frame, end frame, file seconds at the start) of an audio node, or None when its file can't be read.
pub fn span(g: &Graph, id: NodeId) -> Option<(f64, f64, f64, Arc<Wave>)> {
    let path = val(g, id, "filename").as_str().to_string();
    let w = wave(&path).ok()?;
    let fps = crate::anim::fps();
    let total = w.seconds() * fps;
    let ss = val(g, id, "sourceStart").as_float().clamp(0.0, total);
    let se = val(g, id, "sourceEnd").as_float();
    let se = if se <= ss { total } else { se.min(total) };
    let off = val(g, id, "offset").as_float();
    Some((off, off + (se - ss), ss / fps, w))
}

/// The waveform between two frames in count buckets: each bucket's lowest and highest sample.
pub fn peaks(g: &Graph, id: NodeId, from: f64, to: f64, count: usize) -> Vec<[f32; 2]> {
    let Some((start, end, src0, w)) = span(g, id) else { return Vec::new() };
    let fps = crate::anim::fps();
    let count = count.clamp(1, 20_000);
    (0..count)
        .map(|k| {
            let f0 = from + (to - from) * k as f64 / count as f64;
            let f1 = from + (to - from) * (k + 1) as f64 / count as f64;
            let (a, b) = (f0.max(start), f1.min(end));
            if b <= a {
                return [0.0, 0.0];
            }
            let s0 = ((src0 + (a - start) / fps) * w.rate as f64) as usize;
            let s1 = (((src0 + (b - start) / fps) * w.rate as f64) as usize).min(w.samples.len());
            let slice = &w.samples[s0.min(s1)..s1];
            let lo = slice.iter().copied().fold(0.0f32, f32::min);
            let hi = slice.iter().copied().fold(0.0f32, f32::max);
            [lo, hi]
        })
        .collect()
}

/// What the time slider shows: (sound, shown).
static DISPLAY: Mutex<Option<(String, bool)>> = Mutex::new(None);

pub fn displayed() -> (String, bool) {
    DISPLAY.lock().unwrap().clone().unwrap_or((String::new(), true))
}

pub fn set_displayed(sound: Option<String>, shown: Option<bool>) {
    let mut d = DISPLAY.lock().unwrap();
    let mut v = d.clone().unwrap_or((String::new(), true));
    if let Some(s) = sound {
        v.0 = s;
    }
    if let Some(b) = shown {
        v.1 = b;
    }
    *d = Some(v);
}

fn audio_node(s: &Session, n: &str) -> CmdResult<NodeId> {
    let id = s.graph.find(n)?.ok_or_else(|| aztek_ops::CommandError::new(format!("sound: no {n}")))?;
    if !s.graph.node(id).is_some_and(|x| x.ty().is_a(TYPE)) {
        fail!("sound: {n} is not a sound");
    }
    Ok(id)
}

pub fn register(c: &mut Commands<Session>) {
    register_types();
    c.register(
        CommandDef::<Session>::new(
            "sound",
            "Makes a sound (an audio node) from -file, starting at frame -offset, or with -edit or -query changes or reads one: \
             -file, -offset, -sourceStart, -sourceEnd, -mute, -endTime and -length (frames, query). Only WAV files are read.",
            |s, a| {
                let names = a.named();
                if a.bool("query") {
                    let Some(n) = names.first() else { fail!("sound: name a sound") };
                    let id = audio_node(s, n)?;
                    let q: Vec<String> = a.get("queried").and_then(Value::as_array).map(|v| v.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
                    let asked = |f: &str| a.bool(f) || q.iter().any(|x| x == f);
                    let g = &s.graph;
                    return Ok(if asked("file") {
                        json!(val(g, id, "filename").as_str())
                    } else if asked("offset") {
                        json!(val(g, id, "offset").as_float())
                    } else if asked("mute") {
                        json!(val(g, id, "mute").as_bool())
                    } else if asked("length") {
                        json!(span(g, id).map_or(0.0, |x| x.1 - x.0))
                    } else if asked("endTime") {
                        json!(span(g, id).map_or(0.0, |x| x.1))
                    } else if asked("sourceStart") {
                        json!(val(g, id, "sourceStart").as_float())
                    } else if asked("sourceEnd") {
                        json!(val(g, id, "sourceEnd").as_float())
                    } else {
                        fail!("sound: query what?")
                    });
                }
                let node = if a.bool("edit") {
                    let Some(n) = names.first() else { fail!("sound: name a sound") };
                    s.graph.path(audio_node(s, n)?)
                } else {
                    let Some(file) = a.get("file").and_then(Value::as_str) else { fail!("sound: give -file") };
                    wave(file).map_err(|e| aztek_ops::CommandError::new(format!("sound: {e}")))?;
                    let stem = std::path::Path::new(file).file_stem().and_then(|x| x.to_str()).unwrap_or("sound").replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "_");
                    let name = a.string("name", &stem);
                    let r = s.run("createNode", json!({"args": [TYPE], "name": name, "skipSelect": true}))?;
                    r.as_str().unwrap_or_default().to_string()
                };
                if let Some(f) = a.get("file").and_then(Value::as_str) {
                    s.run("setAttr", json!({"args": [format!("{node}.filename"), f], "type": "string"}))?;
                }
                for (flag, attr) in [("offset", "offset"), ("sourceStart", "sourceStart"), ("sourceEnd", "sourceEnd")] {
                    if let Some(v) = a.get(flag).and_then(Value::as_f64) {
                        s.run("setAttr", json!({"args": [format!("{node}.{attr}"), v]}))?;
                    }
                }
                if let Some(v) = a.get("mute").and_then(Value::as_bool) {
                    s.run("setAttr", json!({"args": [format!("{node}.mute"), v]}))?;
                }
                Ok(json!(node))
            },
        )
        .positional("sound")
        .flag(FlagDef::new("file", "f", ArgType::String, Value::Null, "The WAV file."))
        .flag(FlagDef::new("offset", "o", ArgType::Float, Value::Null, "The frame it starts at."))
        .flag(FlagDef::new("sourceStart", "ss", ArgType::Float, Value::Null, "Where in the file it starts, frames."))
        .flag(FlagDef::new("sourceEnd", "se", ArgType::Float, Value::Null, "Where in the file it ends, frames."))
        .flag(FlagDef::new("mute", "m", ArgType::Bool, Value::Null, "Silent."))
        .flag(FlagDef::new("name", "n", ArgType::String, Value::Null, "Its name."))
        .flag(FlagDef::new("length", "l", ArgType::Bool, Value::Null, "Query: its length in frames."))
        .flag(FlagDef::new("endTime", "et", ArgType::Bool, Value::Null, "Query: the frame it ends at."))
        .flag(FlagDef::new("edit", "e", ArgType::Bool, Value::Null, "Change one."))
        .flag(FlagDef::new("query", "q", ArgType::Bool, Value::Null, "Read one."))
        .flag(FlagDef::new("queried", "", ArgType::List, Value::Null, "What -q asks for.")),
    );
    c.register(
        CommandDef::<Session>::new(
            "aztekSoundPeaks",
            "For the time slider: the waveform of the sound timeControl shows (or -sound's), from -start to -end frame in -count \
             buckets, each bucket's lowest and highest sample. Empty when no sound is shown.",
            |s, a| {
                // -sound: that sound, as a soundControl shows it; else the time slider's.
                let (name, shown) = match a.get("sound").and_then(Value::as_str) {
                    Some(n) => (n.to_string(), true),
                    None => displayed(),
                };
                if !shown || name.is_empty() {
                    return Ok(json!([]));
                }
                let Ok(Some(id)) = s.graph.find(&name) else { return Ok(json!([])) };
                let (from, to) = (a.float("start", 1.0), a.float("end", 120.0));
                Ok(json!(peaks(&s.graph, id, from, to, a.int("count", 200).max(1) as usize)))
            },
        )
        .flag(FlagDef::new("start", "s", ArgType::Float, Value::Null, "First frame."))
        .flag(FlagDef::new("end", "e", ArgType::Float, Value::Null, "Last frame."))
        .flag(FlagDef::new("count", "c", ArgType::Int, Value::Null, "Buckets."))
        .flag(FlagDef::new("sound", "snd", ArgType::String, Value::Null, "The sound (audio node) to read, instead of the time slider's."))
        .undoable(false),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn wav16(rate: u32, samples: &[i16]) -> Vec<u8> {
        let data: Vec<u8> = samples.iter().flat_map(|x| x.to_le_bytes()).collect();
        let mut b = b"RIFF".to_vec();
        b.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        b.extend_from_slice(b"WAVEfmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * 2).to_le_bytes());
        b.extend_from_slice(&2u16.to_le_bytes());
        b.extend_from_slice(&16u16.to_le_bytes());
        b.extend_from_slice(b"data");
        b.extend_from_slice(&(data.len() as u32).to_le_bytes());
        b.extend(data);
        b
    }

    #[test]
    fn reads_pcm16() {
        let w = read_wav(&wav16(8000, &[0, 16384, -32768, 32767])).unwrap();
        assert_eq!(w.rate, 8000);
        assert_eq!(w.samples, vec![0.0, 0.5, -1.0, 32767.0 / 32768.0]);
        assert!(read_wav(b"nope").is_err());
    }
}
