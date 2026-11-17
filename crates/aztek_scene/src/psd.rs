//! Photoshop files, as the reference's PSD networks use them (#108): written with a layer set (a group) for each
//! shading attribute and the UV snapshot over them, and read back as Photoshop leaves them: layers raw, RLE or zip
//! compressed, in groups, with their opacity and visibility. A psdFileTex shows one layer set (all of the image when it
//! names none), turned into a PNG beside the other temporary files whenever the PSD changes.
//!
//! Layers are put together with the normal blend whatever mode Photoshop gives them; 8 bit RGB and greyscale files
//! only (16 and 32 bit, CMYK and PSB are refused with a message).

use std::path::{Path, PathBuf};

/// What a layer is in the stack: pixels, the top of a group (open or closed, with the group's name), or the hidden
/// marker under a group's layers.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Pixels,
    GroupStart { open: bool },
    GroupEnd,
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub name: String,
    /// top, left, bottom, right in pixels of the image.
    pub rect: [i32; 4],
    /// RGBA, the rect's width by height, top row first.
    pub rgba: Vec<u8>,
    pub opacity: u8,
    pub visible: bool,
    pub blend: [u8; 4],
    pub kind: Kind,
}

impl Layer {
    /// A full-size layer of pixels.
    pub fn pixels(name: &str, width: u32, height: u32, rgba: Vec<u8>) -> Layer {
        Layer { name: name.to_string(), rect: [0, 0, height as i32, width as i32], rgba, opacity: 255, visible: true, blend: *b"norm", kind: Kind::Pixels }
    }
    fn group(name: &str, kind: Kind) -> Layer {
        let blend = if kind == Kind::GroupEnd { *b"norm" } else { *b"pass" };
        Layer { name: name.to_string(), rect: [0; 4], rgba: Vec::new(), opacity: 255, visible: true, blend, kind }
    }
    fn w(&self) -> usize {
        (self.rect[3] - self.rect[1]).max(0) as usize
    }
    fn h(&self) -> usize {
        (self.rect[2] - self.rect[0]).max(0) as usize
    }
}

/// A document: its layers bottom first, as the file lists them, and the merged image.
#[derive(Clone, Debug, Default)]
pub struct Psd {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub merged: Vec<u8>,
}

/// One layer set of a document to write: its name and its layers, bottom first.
pub struct LayerSet {
    pub name: String,
    pub layers: Vec<Layer>,
}

impl Psd {
    /// A document of layer sets (bottom first) with loose layers over them (the UV snapshot), merged.
    pub fn from_sets(width: u32, height: u32, sets: Vec<LayerSet>, top: Vec<Layer>) -> Psd {
        let mut layers = Vec::new();
        for s in sets {
            layers.push(Layer::group("</Layer group>", Kind::GroupEnd));
            layers.extend(s.layers);
            layers.push(Layer::group(&s.name, Kind::GroupStart { open: true }));
        }
        layers.extend(top);
        let mut p = Psd { width, height, layers, merged: Vec::new() };
        p.merged = p.composite(None).unwrap_or_default();
        p
    }

    /// The names of the layer sets, top first, as Photoshop's Layers panel lists them.
    pub fn layer_sets(&self) -> Vec<String> {
        self.layers.iter().rev().filter(|l| matches!(l.kind, Kind::GroupStart { .. })).map(|l| l.name.clone()).collect()
    }

    /// RGBA of the visible layers put together, of every layer (None) or of one layer set's. None when there is no such
    /// set.
    pub fn composite(&self, set: Option<&str>) -> Option<Vec<u8>> {
        let (w, h) = (self.width as usize, self.height as usize);
        let mut out = vec![0u8; w * h * 4];
        // Each layer with the groups it sits in, outermost first: groups open at their top (last in the file), so the
        // stack is walked from the top down.
        let mut opened: Vec<(String, bool)> = Vec::new();
        let mut placed: Vec<(usize, Vec<(String, bool)>)> = Vec::new();
        for (i, l) in self.layers.iter().enumerate().rev() {
            match &l.kind {
                Kind::GroupStart { .. } => opened.push((l.name.clone(), l.visible)),
                Kind::GroupEnd => {
                    opened.pop();
                }
                Kind::Pixels => placed.push((i, opened.clone())),
            }
        }
        let mut found = set.is_none();
        // Put together bottom first.
        for (i, groups) in placed.into_iter().rev() {
            let l = &self.layers[i];
            // The groups that hide it: all of them, or inside the set asked for (the set shows whether or not it is
            // hidden itself).
            let inner = match set {
                None => &groups[..],
                Some(name) => match groups.iter().position(|(g, _)| g == name) {
                    Some(at) => {
                        found = true;
                        &groups[at + 1..]
                    }
                    None => continue,
                },
            };
            if !l.visible || inner.iter().any(|(_, v)| !v) {
                continue;
            }
            over(&mut out, w, h, l);
        }
        found.then_some(out)
    }
}

/// A layer over the canvas, normal blend at its opacity.
fn over(out: &mut [u8], w: usize, h: usize, l: &Layer) {
    let (lw, lh) = (l.w(), l.h());
    if l.rgba.len() < lw * lh * 4 {
        return;
    }
    let op = l.opacity as f32 / 255.0;
    for y in 0..lh {
        let cy = l.rect[0] + y as i32;
        if cy < 0 || cy as usize >= h {
            continue;
        }
        for x in 0..lw {
            let cx = l.rect[1] + x as i32;
            if cx < 0 || cx as usize >= w {
                continue;
            }
            let s = &l.rgba[(y * lw + x) * 4..(y * lw + x) * 4 + 4];
            let d = &mut out[(cy as usize * w + cx as usize) * 4..(cy as usize * w + cx as usize) * 4 + 4];
            let sa = s[3] as f32 / 255.0 * op;
            let da = d[3] as f32 / 255.0;
            let oa = sa + da * (1.0 - sa);
            if oa <= 0.0 {
                continue;
            }
            for k in 0..3 {
                let v = (s[k] as f32 * sa + d[k] as f32 * da * (1.0 - sa)) / oa;
                d[k] = v.round().clamp(0.0, 255.0) as u8;
            }
            d[3] = (oa * 255.0).round() as u8;
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Writing

fn be16(o: &mut Vec<u8>, v: u16) {
    o.extend_from_slice(&v.to_be_bytes());
}
fn be32(o: &mut Vec<u8>, v: u32) {
    o.extend_from_slice(&v.to_be_bytes());
}

/// The planes of an RGBA image: R, G, B, A.
fn planes(rgba: &[u8], n: usize) -> [Vec<u8>; 4] {
    let mut p: [Vec<u8>; 4] = Default::default();
    for k in 0..4 {
        p[k] = (0..n).map(|i| rgba[i * 4 + k]).collect();
    }
    p
}

/// The file's bytes: uncompressed, 8 bit RGB with transparency, layers and the merged image.
pub fn write(p: &Psd) -> Vec<u8> {
    let mut o = Vec::new();
    o.extend_from_slice(b"8BPS");
    be16(&mut o, 1);
    o.extend_from_slice(&[0; 6]);
    be16(&mut o, 4);
    be32(&mut o, p.height);
    be32(&mut o, p.width);
    be16(&mut o, 8);
    be16(&mut o, 3);
    be32(&mut o, 0); // colour mode data
    be32(&mut o, 0); // image resources

    let mut info = Vec::new();
    // A negative count says the merged image's first extra channel is its transparency.
    be16(&mut info, (-(p.layers.len() as i16)) as u16);
    let mut data = Vec::new();
    for l in &p.layers {
        let n = l.w() * l.h();
        for k in 0..4 {
            info.extend_from_slice(&l.rect[k].to_be_bytes());
        }
        be16(&mut info, 4);
        let pl = if n > 0 && l.rgba.len() >= n * 4 { planes(&l.rgba, n) } else { Default::default() };
        let ids: [i16; 4] = [0, 1, 2, -1];
        let order = [3usize, 0, 1, 2];
        for &k in &order {
            info.extend_from_slice(&ids[k].to_be_bytes());
            be32(&mut info, 2 + pl[k].len() as u32);
        }
        info.extend_from_slice(b"8BIM");
        info.extend_from_slice(&l.blend);
        info.push(l.opacity);
        info.push(0);
        info.push(if l.visible { 0 } else { 0x02 } | 0x08);
        info.push(0);
        let mut extra = Vec::new();
        be32(&mut extra, 0); // mask
        be32(&mut extra, 0); // blending ranges
        let name: Vec<u8> = l.name.chars().map(|c| if c.is_ascii() { c as u8 } else { b'_' }).take(255).collect();
        let mut pascal = vec![name.len() as u8];
        pascal.extend_from_slice(&name);
        while pascal.len() % 4 != 0 {
            pascal.push(0);
        }
        extra.extend_from_slice(&pascal);
        // The name in full, and what a group is.
        let utf16: Vec<u16> = l.name.encode_utf16().collect();
        let mut luni = Vec::new();
        be32(&mut luni, utf16.len() as u32);
        for u in &utf16 {
            be16(&mut luni, *u);
        }
        if luni.len() % 2 != 0 {
            luni.push(0);
        }
        extra.extend_from_slice(b"8BIMluni");
        be32(&mut extra, luni.len() as u32);
        extra.extend_from_slice(&luni);
        let section = match l.kind {
            Kind::Pixels => None,
            Kind::GroupStart { open } => Some(if open { 1u32 } else { 2 }),
            Kind::GroupEnd => Some(3),
        };
        if let Some(t) = section {
            extra.extend_from_slice(b"8BIMlsct");
            be32(&mut extra, 4);
            be32(&mut extra, t);
        }
        be32(&mut info, extra.len() as u32);
        info.extend_from_slice(&extra);
        for &k in &order {
            be16(&mut data, 0);
            data.extend_from_slice(&pl[k]);
        }
    }
    info.extend_from_slice(&data);
    if info.len() % 2 != 0 {
        info.push(0);
    }
    let mut lm = Vec::new();
    be32(&mut lm, info.len() as u32);
    lm.extend_from_slice(&info);
    be32(&mut lm, 0); // global mask
    be32(&mut o, lm.len() as u32);
    o.extend_from_slice(&lm);

    // The merged image: raw, planar.
    be16(&mut o, 0);
    let n = (p.width * p.height) as usize;
    let merged = if p.merged.len() >= n * 4 { p.merged.clone() } else { vec![0; n * 4] };
    let pl = planes(&merged, n);
    for k in 0..4 {
        o.extend_from_slice(&pl[k]);
    }
    o
}

// ---------------------------------------------------------------------------------------------------------------------
// Reading

struct Cur<'a> {
    b: &'a [u8],
    at: usize,
}

impl Cur<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.b.len()).ok_or("the file ends early")?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, String> {
        let s = self.take(2)?;
        Ok(u16::from_be_bytes([s[0], s[1]]))
    }
    fn i16(&mut self) -> Result<i16, String> {
        Ok(self.u16()? as i16)
    }
    fn u32(&mut self) -> Result<u32, String> {
        let s = self.take(4)?;
        Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(self.u32()? as i32)
    }
}

/// PackBits rows: their byte counts first, then the runs.
fn unpack_rle(c: &mut Cur, w: usize, h: usize) -> Result<Vec<u8>, String> {
    let mut counts = Vec::with_capacity(h);
    for _ in 0..h {
        counts.push(c.u16()? as usize);
    }
    let mut out = Vec::with_capacity(w * h);
    for n in counts {
        let row = c.take(n)?;
        let mut i = 0;
        let start = out.len();
        while i < row.len() {
            let k = row[i] as i8;
            i += 1;
            if k >= 0 {
                let len = k as usize + 1;
                out.extend_from_slice(row.get(i..i + len).ok_or("a bad RLE run")?);
                i += len;
            } else if k != -128 {
                let len = (1 - k as isize) as usize;
                let v = *row.get(i).ok_or("a bad RLE run")?;
                i += 1;
                out.extend(std::iter::repeat_n(v, len));
            }
        }
        out.resize(start + w, 0);
    }
    Ok(out)
}

fn channel(c: &mut Cur, len: usize, w: usize, h: usize) -> Result<Vec<u8>, String> {
    let end = c.at + len;
    if len < 2 {
        c.at = end.min(c.b.len());
        return Ok(vec![0; w * h]);
    }
    let comp = c.u16()?;
    let n = w * h;
    let out = match comp {
        0 => c.take(n)?.to_vec(),
        1 => unpack_rle(c, w, h)?,
        2 | 3 => {
            let z = c.take(len - 2)?;
            let mut d = miniz_oxide::inflate::decompress_to_vec_zlib(z).map_err(|_| "a zip channel that doesn't inflate".to_string())?;
            d.resize(n, 0);
            if comp == 3 {
                for y in 0..h {
                    for x in 1..w {
                        d[y * w + x] = d[y * w + x].wrapping_add(d[y * w + x - 1]);
                    }
                }
            }
            d
        }
        other => return Err(format!("compression {other} isn't known")),
    };
    c.at = end.min(c.b.len());
    Ok(out)
}

/// Reads a document. 8 bit RGB or greyscale.
pub fn read(bytes: &[u8]) -> Result<Psd, String> {
    let mut c = Cur { b: bytes, at: 0 };
    if c.take(4)? != b"8BPS" {
        return Err("not a Photoshop file".into());
    }
    let version = c.u16()?;
    if version != 1 {
        return Err("a large document (PSB) isn't read here".into());
    }
    c.take(6)?;
    let channels = c.u16()? as usize;
    let height = c.u32()? as usize;
    let width = c.u32()? as usize;
    let depth = c.u16()?;
    let mode = c.u16()?;
    if depth != 8 {
        return Err(format!("{depth} bit documents aren't read here: save it 8 bits a channel"));
    }
    if mode != 3 && mode != 1 {
        return Err("only RGB and greyscale documents are read here".into());
    }
    let grey = mode == 1;
    let n = c.u32()? as usize;
    c.take(n)?;
    let n = c.u32()? as usize;
    c.take(n)?;
    let lm_len = c.u32()? as usize;
    let lm_end = c.at + lm_len;
    let mut layers = Vec::new();
    if lm_len > 0 {
        let info_len = c.u32()? as usize;
        let info_end = c.at + info_len;
        if info_len > 0 {
            let count = (c.i16()? as i32).unsigned_abs() as usize;
            struct Rec {
                layer: Layer,
                chans: Vec<(i16, usize)>,
            }
            let mut recs = Vec::with_capacity(count);
            for _ in 0..count {
                let rect = [c.i32()?, c.i32()?, c.i32()?, c.i32()?];
                let nc = c.u16()? as usize;
                let mut chans = Vec::with_capacity(nc);
                for _ in 0..nc {
                    let id = c.i16()?;
                    let len = c.u32()? as usize;
                    chans.push((id, len));
                }
                c.take(4)?;
                let blend: [u8; 4] = c.take(4)?.try_into().unwrap();
                let opacity = c.u8()?;
                let _clip = c.u8()?;
                let flags = c.u8()?;
                c.u8()?;
                let extra = c.u32()? as usize;
                let extra_end = c.at + extra;
                let mask = c.u32()? as usize;
                c.take(mask)?;
                let ranges = c.u32()? as usize;
                c.take(ranges)?;
                let nl = c.u8()? as usize;
                let mut name = String::from_utf8_lossy(c.take(nl)?).to_string();
                let pad = (4 - (nl + 1) % 4) % 4;
                c.take(pad)?;
                let mut kind = Kind::Pixels;
                while c.at + 12 <= extra_end {
                    let sig = c.take(4)?.to_vec();
                    if &sig != b"8BIM" && &sig != b"8B64" {
                        break;
                    }
                    let key: [u8; 4] = c.take(4)?.try_into().unwrap();
                    let len = c.u32()? as usize;
                    let start = c.at;
                    match &key {
                        b"luni" if len >= 4 => {
                            let k = c.u32()? as usize;
                            let mut u = Vec::with_capacity(k);
                            for _ in 0..k {
                                u.push(c.u16()?);
                            }
                            name = String::from_utf16_lossy(&u).trim_end_matches('\0').to_string();
                        }
                        b"lsct" | b"lsdk" if len >= 4 => {
                            kind = match c.u32()? {
                                1 => Kind::GroupStart { open: true },
                                2 => Kind::GroupStart { open: false },
                                3 => Kind::GroupEnd,
                                _ => Kind::Pixels,
                            };
                        }
                        _ => {}
                    }
                    c.at = start + len + (len % 2);
                }
                c.at = extra_end;
                let layer = Layer { name, rect, rgba: Vec::new(), opacity, visible: flags & 0x02 == 0, blend, kind };
                recs.push(Rec { layer, chans });
            }
            for r in &mut recs {
                let (w, h) = (r.layer.w(), r.layer.h());
                let mut pl: [Option<Vec<u8>>; 4] = Default::default();
                for &(id, len) in &r.chans {
                    let ch = channel(&mut c, len, w, h)?;
                    match id {
                        0 => pl[0] = Some(ch),
                        1 if !grey => pl[1] = Some(ch),
                        2 if !grey => pl[2] = Some(ch),
                        -1 => pl[3] = Some(ch),
                        _ => {}
                    }
                }
                if grey {
                    pl[1] = pl[0].clone();
                    pl[2] = pl[0].clone();
                }
                let mut rgba = vec![0u8; w * h * 4];
                for i in 0..w * h {
                    for k in 0..4 {
                        rgba[i * 4 + k] = pl[k].as_ref().and_then(|p| p.get(i).copied()).unwrap_or(255);
                    }
                }
                r.layer.rgba = rgba;
            }
            layers = recs.into_iter().map(|r| r.layer).collect();
        }
        c.at = info_end;
    }
    c.at = lm_end;
    // The merged image.
    let mut merged = vec![0u8; width * height * 4];
    if c.at + 2 <= bytes.len() {
        let comp = c.u16()?;
        let n = width * height;
        let mut planes: Vec<Vec<u8>> = Vec::new();
        if comp == 1 {
            let mut counts = Vec::new();
            for _ in 0..height * channels {
                counts.push(c.u16()? as usize);
            }
            let mut all = Vec::new();
            for (row, k) in counts.iter().enumerate() {
                let data = c.take(*k)?;
                let mut tmp = Vec::new();
                be16(&mut tmp, data.len() as u16);
                tmp.extend_from_slice(data);
                let mut cc = Cur { b: &tmp, at: 0 };
                let _ = row;
                all.extend(unpack_rle(&mut cc, width, 1)?);
            }
            for k in 0..channels {
                planes.push(all.get(k * n..(k + 1) * n).map(<[u8]>::to_vec).unwrap_or_else(|| vec![0; n]));
            }
        } else if comp == 0 {
            for _ in 0..channels {
                planes.push(c.take(n).map(<[u8]>::to_vec).unwrap_or_else(|_| vec![0; n]));
            }
        }
        if !planes.is_empty() {
            let pick = |k: usize| -> Option<&Vec<u8>> {
                if grey {
                    match k {
                        0..=2 => planes.first(),
                        _ => planes.get(1),
                    }
                } else {
                    planes.get(k)
                }
            };
            for i in 0..n {
                for k in 0..4 {
                    merged[i * 4 + k] = pick(k).and_then(|p| p.get(i).copied()).unwrap_or(255);
                }
            }
        }
    }
    Ok(Psd { width: width as u32, height: height as u32, layers, merged })
}

pub fn read_file(path: &Path) -> Result<Psd, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    read(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn write_file(path: &Path, p: &Psd) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(path, write(p)).map_err(|e| format!("{}: {e}", path.display()))
}

/// The image a psdFileTex shows: the layer set it names (all of it when it names none, or the merged image when the
/// document has no layers), as a PNG made again whenever the PSD changes. None when it can't be read.
pub fn layer_set_png(psd: &Path, set: &str) -> Option<PathBuf> {
    use std::hash::{Hash, Hasher};
    let stamp = std::fs::metadata(psd).and_then(|m| m.modified()).ok()?;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    psd.to_string_lossy().hash(&mut h);
    stamp.hash(&mut h);
    set.hash(&mut h);
    let stem = psd.file_stem().and_then(|x| x.to_str()).unwrap_or("psd");
    let dir = std::env::temp_dir().join("aztek-psd");
    let out = dir.join(format!("{stem}_{}_{:016x}.png", if set.is_empty() { "all" } else { set }, h.finish()));
    if out.is_file() {
        return Some(out);
    }
    let doc = match read_file(psd) {
        Ok(d) => d,
        Err(e) => {
            log::warn!("psdFileTex: {e}");
            return None;
        }
    };
    let rgba = if set.is_empty() {
        if doc.layers.is_empty() { Some(doc.merged.clone()) } else { doc.composite(None) }
    } else {
        doc.composite(Some(set))
    };
    let Some(rgba) = rgba else {
        log::warn!("psdFileTex: {} has no layer set {set}", psd.display());
        return None;
    };
    std::fs::create_dir_all(&dir).ok()?;
    image::save_buffer(&out, &rgba, doc.width, doc.height, image::ExtendedColorType::Rgba8).ok()?;
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(w: u32, h: u32, c: [u8; 4]) -> Vec<u8> {
        (0..w * h).flat_map(|_| c).collect()
    }

    #[test]
    fn layer_sets_round_trip_and_composite() {
        let (w, h) = (8, 4);
        let sets = vec![
            LayerSet { name: "color".into(), layers: vec![Layer::pixels("color", w, h, solid(w, h, [200, 10, 10, 255]))] },
            LayerSet { name: "bump".into(), layers: vec![Layer::pixels("bump", w, h, solid(w, h, [128, 128, 128, 255]))] },
        ];
        let mut snap = solid(w, h, [0, 0, 0, 0]);
        snap[0..4].copy_from_slice(&[255, 255, 255, 255]);
        let doc = Psd::from_sets(w, h, sets, vec![Layer::pixels("UVSnapShot", w, h, snap)]);
        let back = read(&write(&doc)).unwrap();
        assert_eq!((back.width, back.height), (8, 4));
        assert_eq!(back.layer_sets(), vec!["bump".to_string(), "color".to_string()]);
        let c = back.composite(Some("color")).unwrap();
        assert_eq!(&c[0..4], &[200, 10, 10, 255]);
        // The snapshot is over everything, not in a set.
        assert_eq!(&back.composite(None).unwrap()[0..4], &[255, 255, 255, 255]);
        assert_eq!(&back.merged[0..4], &[255, 255, 255, 255]);
        assert!(back.composite(Some("nothing")).is_none());
    }

    #[test]
    fn rle_and_half_transparent_layers() {
        // A row "aaaabc" as PackBits: a run of 4, then 2 literals.
        let rows = [0xFDu8, b'a', 0x01, b'b', b'c'];
        let mut buf = Vec::new();
        be16(&mut buf, rows.len() as u16);
        buf.extend_from_slice(&rows);
        let mut c = Cur { b: &buf, at: 0 };
        assert_eq!(unpack_rle(&mut c, 6, 1).unwrap(), b"aaaabc".to_vec());
        let (w, h) = (2, 1);
        let mut base = Layer::pixels("base", w, h, solid(w, h, [0, 0, 255, 255]));
        base.opacity = 255;
        let mut top = Layer::pixels("top", w, h, solid(w, h, [255, 0, 0, 255]));
        top.opacity = 128;
        let doc = Psd::from_sets(w, h, vec![LayerSet { name: "color".into(), layers: vec![base, top] }], vec![]);
        let px = &doc.composite(Some("color")).unwrap()[0..4];
        assert!(px[0] > 120 && px[0] < 135 && px[2] > 120 && px[2] < 135, "{px:?}");
    }
}
