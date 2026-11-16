//! Alembic's layer over Ogawa (the reference library's AbcCoreOgawa): an archive of objects, each with a compound of
//! properties, scalar or array, sampled over time. Laid out as that library lays it out:
//!
//! - Root group: format version (i32 0), library version (i32), the top object's group, archive metadata (text), time
//!   samplings, indexed metadata.
//! - Object group: its properties' compound group, a group per child object, then the children's headers (name and
//!   metadata each, then 32 bytes of hashes).
//! - Compound group: a group per property, then the properties' headers packed as the library packs them.
//! - Scalar property: a data block per written sample (a 16-byte key, then the bytes). Array property: the same, each
//!   followed by its dimensions (empty data when one-dimensional).
//!
//! A property whose samples are all the same writes one. Strings are written NUL-terminated.

use std::collections::HashMap;

use crate::ogawa::{self, err, Node, Reader, Result};

/// Plain data types, numbered as the file numbers them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pod {
    Bool = 0,
    U8 = 1,
    I8 = 2,
    U16 = 3,
    I16 = 4,
    U32 = 5,
    I32 = 6,
    U64 = 7,
    I64 = 8,
    F16 = 9,
    F32 = 10,
    F64 = 11,
    String = 12,
    WString = 13,
}

impl Pod {
    pub fn size(self) -> usize {
        match self {
            Pod::Bool | Pod::U8 | Pod::I8 | Pod::String => 1,
            Pod::U16 | Pod::I16 | Pod::F16 => 2,
            Pod::U32 | Pod::I32 | Pod::F32 | Pod::WString => 4,
            Pod::U64 | Pod::I64 | Pod::F64 => 8,
        }
    }
    fn from(v: u32) -> Result<Pod> {
        Ok(match v {
            0 => Pod::Bool,
            1 => Pod::U8,
            2 => Pod::I8,
            3 => Pod::U16,
            4 => Pod::I16,
            5 => Pod::U32,
            6 => Pod::I32,
            7 => Pod::U64,
            8 => Pod::I64,
            9 => Pod::F16,
            10 => Pod::F32,
            11 => Pod::F64,
            12 => Pod::String,
            13 => Pod::WString,
            x => return err(format!("unknown data type {x}")),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Compound(Vec<Property>),
    Scalar { pod: Pod, extent: u8, ts: u32, samples: Vec<Vec<u8>> },
    Array { pod: Pod, extent: u8, ts: u32, samples: Vec<Vec<u8>> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    pub name: String,
    /// "key=value;key=value".
    pub meta: String,
    pub kind: Kind,
}

impl Property {
    pub fn compound(name: &str, meta: &str, props: Vec<Property>) -> Self {
        Property { name: name.into(), meta: meta.into(), kind: Kind::Compound(props) }
    }
    pub fn scalar(name: &str, meta: &str, pod: Pod, extent: u8, ts: u32, samples: Vec<Vec<u8>>) -> Self {
        Property { name: name.into(), meta: meta.into(), kind: Kind::Scalar { pod, extent, ts, samples } }
    }
    pub fn array(name: &str, meta: &str, pod: Pod, extent: u8, ts: u32, samples: Vec<Vec<u8>>) -> Self {
        Property { name: name.into(), meta: meta.into(), kind: Kind::Array { pod, extent, ts, samples } }
    }
    pub fn child(&self, name: &str) -> Option<&Property> {
        match &self.kind {
            Kind::Compound(p) => p.iter().find(|x| x.name == name),
            _ => None,
        }
    }
    pub fn samples(&self) -> &[Vec<u8>] {
        match &self.kind {
            Kind::Scalar { samples, .. } | Kind::Array { samples, .. } => samples,
            Kind::Compound(_) => &[],
        }
    }
    pub fn pod(&self) -> Option<(Pod, u8)> {
        match &self.kind {
            Kind::Scalar { pod, extent, .. } | Kind::Array { pod, extent, .. } => Some((*pod, *extent)),
            Kind::Compound(_) => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    pub name: String,
    pub meta: String,
    pub props: Vec<Property>,
    pub children: Vec<Object>,
}

impl Object {
    pub fn prop(&self, name: &str) -> Option<&Property> {
        self.props.iter().find(|p| p.name == name)
    }
    /// A metadata value.
    pub fn meta_value(&self, key: &str) -> Option<String> {
        meta_value(&self.meta, key)
    }
}

pub fn meta_value(meta: &str, key: &str) -> Option<String> {
    meta.split(';').find_map(|kv| kv.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v.to_string()))
}

/// Times of the samples: the first sample's time and the time between samples (uniform), or every time (acyclic).
#[derive(Clone, Debug, PartialEq)]
pub enum TimeSampling {
    Uniform { start: f64, step: f64 },
    Acyclic(Vec<f64>),
}

impl TimeSampling {
    pub fn time(&self, k: usize) -> f64 {
        match self {
            TimeSampling::Uniform { start, step } => start + step * k as f64,
            TimeSampling::Acyclic(t) => t.get(k).copied().or(t.last().copied()).unwrap_or(0.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Archive {
    pub meta: String,
    /// Index 0 is the identity sampling (one sample at 0, a second apart); others follow.
    pub samplings: Vec<TimeSampling>,
    pub top: Object,
}

impl Default for Archive {
    fn default() -> Self {
        Archive { meta: String::new(), samplings: vec![TimeSampling::Uniform { start: 0.0, step: 1.0 }], top: Object { name: "ABC".into(), ..Default::default() } }
    }
}

/// The library version this writes as (Alembic 1.8.0 numbering, XXYYZZ).
pub const LIBRARY_VERSION: i32 = 10800;

// ---------------------------------------------------------------------------------------------------------------------
// The 16-byte key on each sample: MurmurHash3 x64 128 of its bytes.

pub fn murmur3_128(data: &[u8]) -> [u8; 16] {
    const C1: u64 = 0x87c3_7b91_1142_53d5;
    const C2: u64 = 0x4cf5_ad43_2745_937f;
    let (mut h1, mut h2) = (0u64, 0u64);
    let n = data.len();
    let blocks = n / 16;
    let fmix = |mut k: u64| {
        k ^= k >> 33;
        k = k.wrapping_mul(0xff51_afd7_ed55_8ccd);
        k ^= k >> 33;
        k = k.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        k ^ (k >> 33)
    };
    for i in 0..blocks {
        let mut k1 = u64::from_le_bytes(data[i * 16..i * 16 + 8].try_into().unwrap());
        let mut k2 = u64::from_le_bytes(data[i * 16 + 8..i * 16 + 16].try_into().unwrap());
        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
        h1 = h1.rotate_left(27).wrapping_add(h2).wrapping_mul(5).wrapping_add(0x52dc_e729);
        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
        h2 = h2.rotate_left(31).wrapping_add(h1).wrapping_mul(5).wrapping_add(0x3849_5ab5);
    }
    let tail = &data[blocks * 16..];
    let (mut k1, mut k2) = (0u64, 0u64);
    for (i, &b) in tail.iter().enumerate() {
        if i < 8 {
            k1 ^= (b as u64) << (8 * i);
        } else {
            k2 ^= (b as u64) << (8 * (i - 8));
        }
    }
    if tail.len() > 8 {
        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
    }
    if !tail.is_empty() {
        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
    }
    h1 ^= n as u64;
    h2 ^= n as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix(h1);
    h2 = fmix(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&h1.to_le_bytes());
    out[8..].copy_from_slice(&h2.to_le_bytes());
    out
}

// ---------------------------------------------------------------------------------------------------------------------
// Writing

struct MetaMap {
    entries: Vec<String>,
    index: HashMap<String, u8>,
}

impl MetaMap {
    /// The index of a metadata text: 0 for none, 1 to 254 for the indexed, 0xff to write it in place.
    fn index(&mut self, m: &str) -> u8 {
        if m.is_empty() {
            return 0;
        }
        if let Some(&i) = self.index.get(m) {
            return i;
        }
        if m.len() > 255 || self.entries.len() >= 254 {
            return 0xff;
        }
        self.entries.push(m.to_string());
        let i = self.entries.len() as u8;
        self.index.insert(m.to_string(), i);
        i
    }
    fn data(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for e in &self.entries {
            out.push(e.len() as u8);
            out.extend_from_slice(e.as_bytes());
        }
        out
    }
}

fn push_hint(out: &mut Vec<u8>, v: u32, hint: u32) {
    match hint {
        0 => out.push(v as u8),
        1 => out.extend_from_slice(&(v as u16).to_le_bytes()),
        _ => out.extend_from_slice(&v.to_le_bytes()),
    }
}

fn keyed(bytes: &[u8]) -> Vec<u8> {
    let mut v = murmur3_128(bytes).to_vec();
    v.extend_from_slice(bytes);
    v
}

struct Writer {
    meta: MetaMap,
    /// Per time sampling, the most samples a property using it has.
    max_samples: Vec<u32>,
}

impl Writer {
    fn compound(&mut self, props: &[Property]) -> Node {
        let mut kids = Vec::new();
        let mut headers = Vec::new();
        for p in props {
            let meta_index = self.meta.index(&p.meta);
            let (node, info_extra): (Node, Option<(u32, u32, bool, bool, Pod, u8, u32)>) = match &p.kind {
                Kind::Compound(sub) => (self.compound(sub), None),
                Kind::Scalar { pod, extent, ts, samples } | Kind::Array { pod, extent, ts, samples } => {
                    let array = matches!(p.kind, Kind::Array { .. });
                    let constant = samples.windows(2).all(|w| w[0] == w[1]);
                    let written: &[Vec<u8>] = if constant { &samples[..samples.len().min(1)] } else { samples };
                    let mut ch = Vec::new();
                    for s in written {
                        ch.push(Node::Data(keyed(s)));
                        if array {
                            if matches!(pod, Pod::String | Pod::WString) {
                                let count = s.iter().filter(|&&b| b == 0).count() as u64;
                                ch.push(Node::Data(count.to_le_bytes().to_vec()));
                            } else {
                                ch.push(Node::Data(Vec::new()));
                            }
                        }
                    }
                    let n = if constant { samples.len().min(1) } else { samples.len() } as u32;
                    let elem = pod.size() * (*extent as usize).max(1);
                    let counts: Vec<usize> = samples.iter().map(|s| if matches!(pod, Pod::String) { s.iter().filter(|&&b| b == 0).count() } else { s.len() / elem.max(1) }).collect();
                    let homogenous = counts.windows(2).all(|w| w[0] == w[1]);
                    let scalar_like = !array || counts.iter().all(|&c| c == 1);
                    if (*ts as usize) < self.max_samples.len() {
                        let m = &mut self.max_samples[*ts as usize];
                        *m = (*m).max(samples.len() as u32);
                    }
                    (Node::Group(ch), Some((n, if array { 2 } else { 1 }, homogenous, scalar_like || !array, *pod, *extent, *ts)))
                }
            };
            kids.push(node);
            // The header.
            let name = p.name.as_bytes();
            let meta_len = if meta_index == 0xff { p.meta.len() } else { 0 };
            let (n, ts) = info_extra.map_or((0, 0), |x| (x.0, x.6));
            let biggest = (meta_len as u64).max(name.len() as u64).max(n as u64).max(ts as u64);
            let hint: u32 = if biggest <= 255 { 0 } else if biggest <= 65535 { 1 } else { 2 };
            let mut info: u32 = (hint << 2) & 0xc;
            info |= (meta_index as u32) << 20;
            match info_extra {
                None => {
                    headers.extend_from_slice(&info.to_le_bytes());
                }
                Some((n, ptype, homogenous, scalar_like, pod, extent, ts)) => {
                    info |= ptype & 0x3;
                    if scalar_like {
                        info |= 1;
                    }
                    info |= ((pod as u32) << 4) & 0xf0;
                    if ts != 0 {
                        info |= 0x100;
                    }
                    // Constant: one sample, first and last changed both 0. Otherwise every sample is written,
                    // first changed 1 and last n - 1, which is what the header says with neither flag.
                    if n <= 1 {
                        info |= 0x800;
                    }
                    if homogenous {
                        info |= 0x400;
                    }
                    info |= ((extent as u32) << 12) & 0xff000;
                    headers.extend_from_slice(&info.to_le_bytes());
                    push_hint(&mut headers, n, hint);
                    if ts != 0 {
                        push_hint(&mut headers, ts, hint);
                    }
                }
            }
            push_hint(&mut headers, name.len() as u32, hint);
            headers.extend_from_slice(name);
            if meta_index == 0xff {
                push_hint(&mut headers, p.meta.len() as u32, hint);
                headers.extend_from_slice(p.meta.as_bytes());
            }
        }
        if !headers.is_empty() {
            kids.push(Node::Data(headers));
        }
        Node::Group(kids)
    }

    fn object(&mut self, o: &Object) -> Node {
        let mut kids = vec![self.compound(&o.props)];
        let mut headers = Vec::new();
        for c in &o.children {
            kids.push(self.object(c));
            headers.extend_from_slice(&(c.name.len() as u32).to_le_bytes());
            headers.extend_from_slice(c.name.as_bytes());
            let i = self.meta.index(&c.meta);
            headers.push(i);
            if i == 0xff {
                headers.extend_from_slice(&(c.meta.len() as u32).to_le_bytes());
                headers.extend_from_slice(c.meta.as_bytes());
            }
        }
        headers.extend_from_slice(&[0u8; 32]);
        kids.push(Node::Data(headers));
        Node::Group(kids)
    }
}

/// The archive as file bytes.
pub fn write(a: &Archive) -> Vec<u8> {
    let mut w = Writer { meta: MetaMap { entries: Vec::new(), index: HashMap::new() }, max_samples: vec![0; a.samplings.len().max(1)] };
    let top = w.object(&a.top);
    let mut ts = Vec::new();
    for (k, s) in a.samplings.iter().enumerate() {
        ts.extend_from_slice(&w.max_samples.get(k).copied().unwrap_or(0).max(if k == 0 { 1 } else { 0 }).to_le_bytes());
        match s {
            TimeSampling::Uniform { start, step } => {
                ts.extend_from_slice(&step.to_le_bytes());
                ts.extend_from_slice(&1u32.to_le_bytes());
                ts.extend_from_slice(&start.to_le_bytes());
            }
            TimeSampling::Acyclic(times) => {
                ts.extend_from_slice(&(f64::MAX / 32.0).to_le_bytes());
                ts.extend_from_slice(&(times.len() as u32).to_le_bytes());
                for t in times {
                    ts.extend_from_slice(&t.to_le_bytes());
                }
            }
        }
    }
    let root = Node::Group(vec![
        Node::Data(0i32.to_le_bytes().to_vec()),
        Node::Data(LIBRARY_VERSION.to_le_bytes().to_vec()),
        top,
        Node::Data(a.meta.as_bytes().to_vec()),
        Node::Data(ts),
        Node::Data(w.meta.data()),
    ]);
    ogawa::write(&root)
}

// ---------------------------------------------------------------------------------------------------------------------
// Reading

struct Cursor<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let s = self.b.get(self.p..self.p + n).ok_or_else(|| ogawa::Error("a header ends early".into()))?;
        self.p += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn hint(&mut self, h: u32) -> Result<u32> {
        Ok(match h {
            0 => self.u8()? as u32,
            1 => u16::from_le_bytes(self.take(2)?.try_into().unwrap()) as u32,
            _ => self.u32()?,
        })
    }
    fn text(&mut self, n: usize) -> Result<String> {
        Ok(String::from_utf8_lossy(self.take(n)?).into_owned())
    }
    fn done(&self) -> bool {
        self.p >= self.b.len()
    }
}

struct Loader {
    r: Reader,
    meta: Vec<String>,
}

impl Loader {
    fn sample(&self, entry: u64) -> Result<Vec<u8>> {
        let d = self.r.data(entry)?;
        Ok(if d.len() >= 16 { d[16..].to_vec() } else { Vec::new() })
    }

    fn compound(&self, entry: u64) -> Result<Vec<Property>> {
        let kids = self.r.group(entry)?;
        let Some(&last) = kids.last() else { return Ok(Vec::new()) };
        if !Reader::is_data(last) {
            return Ok(Vec::new());
        }
        let hb = self.r.data(last)?;
        let mut c = Cursor { b: hb, p: 0 };
        let mut out = Vec::new();
        let mut k = 0;
        while !c.done() {
            let info = c.u32()?;
            let ptype = info & 3;
            let hint = (info >> 2) & 3;
            let mut sample_info = None;
            if ptype != 0 {
                let pod = Pod::from((info >> 4) & 0xf)?;
                let extent = ((info >> 12) & 0xff) as u8;
                let n = c.hint(hint)?;
                let (first, last) = if info & 0x200 != 0 {
                    (c.hint(hint)?, c.hint(hint)?)
                } else if info & 0x800 != 0 {
                    (0, 0)
                } else {
                    (1, n.saturating_sub(1))
                };
                let ts = if info & 0x100 != 0 { c.hint(hint)? } else { 0 };
                sample_info = Some((pod, extent, n, first, last, ts));
            }
            let nlen = c.hint(hint)? as usize;
            let name = c.text(nlen)?;
            let mi = ((info >> 20) & 0xff) as usize;
            let meta = if mi == 0xff {
                let n = c.hint(hint)? as usize;
                c.text(n)?
            } else {
                self.meta.get(mi).cloned().unwrap_or_default()
            };
            let child = *kids.get(k).ok_or_else(|| ogawa::Error(format!("property {name} has no group")))?;
            k += 1;
            let kind = match sample_info {
                None => Kind::Compound(self.compound(child)?),
                Some((pod, extent, n, first, last, ts)) => {
                    let g = self.r.group(child)?;
                    let per = if ptype == 1 { 1 } else { 2 };
                    let mut samples = Vec::with_capacity(n as usize);
                    for i in 0..n {
                        // Written samples: the first, then those from first changed to last changed.
                        let w = if (first == 0 && last == 0) || i < first {
                            0
                        } else if i >= last {
                            (last - first + 1) as usize
                        } else {
                            (i - first + 1) as usize
                        };
                        let e = *g.get(w * per).ok_or_else(|| ogawa::Error(format!("property {name} lacks sample {i}")))?;
                        samples.push(self.sample(e)?);
                    }
                    if ptype == 1 {
                        Kind::Scalar { pod, extent, ts, samples }
                    } else {
                        Kind::Array { pod, extent, ts, samples }
                    }
                }
            };
            out.push(Property { name, meta, kind });
        }
        Ok(out)
    }

    fn object(&self, entry: u64, name: String, meta: String) -> Result<Object> {
        let kids = self.r.group(entry)?;
        let mut o = Object { name, meta, ..Default::default() };
        if kids.is_empty() {
            return Ok(o);
        }
        if !Reader::is_data(kids[0]) {
            o.props = self.compound(kids[0])?;
        }
        let last = *kids.last().unwrap();
        if kids.len() >= 2 && Reader::is_data(last) {
            let hb = self.r.data(last)?;
            let body = &hb[..hb.len().saturating_sub(32)];
            let mut c = Cursor { b: body, p: 0 };
            let mut i = 1;
            while !c.done() {
                let n = c.u32()? as usize;
                let cname = c.text(n)?;
                let mi = c.u8()?;
                let cmeta = if mi == 0xff {
                    let n = c.u32()? as usize;
                    c.text(n)?
                } else {
                    self.meta.get(mi as usize).cloned().unwrap_or_default()
                };
                let ce = *kids.get(i).ok_or_else(|| ogawa::Error(format!("object {cname} has no group")))?;
                o.children.push(self.object(ce, cname, cmeta)?);
                i += 1;
            }
        }
        Ok(o)
    }
}

pub fn read(bytes: Vec<u8>) -> Result<Archive> {
    let r = Reader::new(bytes)?;
    let root = r.group(r.root()?)?;
    if root.len() < 6 {
        return err("not an Alembic archive: its root has too few parts");
    }
    let meta_text = String::from_utf8_lossy(r.data(root[3])?).into_owned();
    let mut samplings = Vec::new();
    {
        let tsb = r.data(root[4])?;
        let mut c = Cursor { b: tsb, p: 0 };
        while !c.done() {
            let _max = c.u32()?;
            let tpc = c.f64()?;
            let n = c.u32()? as usize;
            let times: Vec<f64> = (0..n).map(|_| c.f64()).collect::<Result<_>>()?;
            if tpc == f64::MAX / 32.0 {
                samplings.push(TimeSampling::Acyclic(times));
            } else if n == 1 {
                samplings.push(TimeSampling::Uniform { start: times[0], step: tpc });
            } else {
                // Cyclic: written out as times for the first few cycles' worth is not needed here; keep the stored ones.
                samplings.push(TimeSampling::Acyclic(times));
            }
        }
    }
    let mut meta = vec![String::new()];
    {
        let mb = r.data(root[5])?;
        let mut c = Cursor { b: mb, p: 0 };
        while !c.done() {
            let n = c.u8()? as usize;
            meta.push(c.text(n)?);
        }
    }
    let l = Loader { r, meta };
    let top = l.object(root[2], "ABC".into(), meta_text.clone())?;
    Ok(Archive { meta: meta_text, samplings, top })
}
