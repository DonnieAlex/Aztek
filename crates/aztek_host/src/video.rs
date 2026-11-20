//! The report window's short videos (#153): frames of the window as RGBA, encoded as H.264 and written as an MP4 that
//! browsers and GitHub play. GitHub takes videos up to 10 MB: the encoder is held to a rate well under that, may drop a
//! frame to keep to it (the frame before is shown longer), and a recording that still reaches the limit stops there
//! (#196).

use std::ffi::{c_char, c_void, CStr};

use openh264::encoder::{BitRate, Encoder, EncoderConfig, FrameRate, RateControlMode, UsageType};
use openh264::formats::{RgbaSliceU8, YUVBuffer};
use openh264::OpenH264API;

/// The most a recording's file may be: under GitHub's 10 MB (10,485,760 bytes) with room for the MP4's own tables.
pub const MAX_BYTES: usize = 9_800_000;

pub struct Recording {
    path: String,
    width: usize,
    height: usize,
    fps: u32,
    encoder: Encoder,
    sps: Vec<u8>,
    pps: Vec<u8>,
    /// Each frame as the MP4 holds it (length-prefixed NAL units), whether it starts a group, and how many frames
    /// it is shown for (more than one where the encoder dropped the next).
    samples: Vec<(Vec<u8>, bool, u32)>,
    bytes: usize,
    /// Reached MAX_BYTES: frames after are left out.
    pub full: bool,
}

impl Recording {
    pub fn new(path: &str, width: usize, height: usize, fps: u32, bits_per_second: u32) -> Result<Self, String> {
        if width == 0 || height == 0 || width % 2 != 0 || height % 2 != 0 {
            return Err(format!("{width}x{height}: a video's sides must be even"));
        }
        let config = EncoderConfig::new()
            .bitrate(BitRate::from_bps(bits_per_second))
            .max_frame_rate(FrameRate::from_hz(fps as f32))
            .rate_control_mode(RateControlMode::Bitrate)
            .skip_frames(true)
            .usage_type(UsageType::ScreenContentRealTime)
            .adaptive_quantization(false)
            .background_detection(false);
        let encoder = Encoder::with_api_config(OpenH264API::from_source(), config).map_err(|e| e.to_string())?;
        Ok(Recording { path: path.to_string(), width, height, fps, encoder, sps: Vec::new(), pps: Vec::new(), samples: Vec::new(), bytes: 0, full: false })
    }

    /// One frame, width x height RGBA rows `stride` bytes apart. Once the file has reached MAX_BYTES, frames are
    /// left out and `full` says so.
    pub fn frame(&mut self, rgba: &[u8], stride: usize) -> Result<(), String> {
        if self.full {
            return Ok(());
        }
        let row = self.width * 4;
        if stride < row || rgba.len() < stride * (self.height - 1) + row {
            return Err("frame smaller than the video".into());
        }
        let packed: Vec<u8> = if stride == row { rgba[..row * self.height].to_vec() } else { (0..self.height).flat_map(|y| rgba[y * stride..y * stride + row].iter().copied()).collect() };
        let yuv = YUVBuffer::from_rgb_source(RgbaSliceU8::new(&packed, (self.width, self.height)));
        let stream = self.encoder.encode(&yuv).map_err(|e| e.to_string())?.to_vec();
        let mut sample = Vec::new();
        let mut sync = false;
        for nal in openh264::nal_units(&stream) {
            // Past the start code.
            let zeros = nal.iter().take_while(|&&b| b == 0).count();
            let Some(body) = nal.get(zeros + 1..).filter(|b| !b.is_empty()) else { continue };
            match body[0] & 0x1f {
                7 => {
                    if self.sps.is_empty() {
                        self.sps = body.to_vec();
                    }
                }
                8 => {
                    if self.pps.is_empty() {
                        self.pps = body.to_vec();
                    }
                }
                kind => {
                    sync |= kind == 5;
                    sample.extend_from_slice(&(body.len() as u32).to_be_bytes());
                    sample.extend_from_slice(body);
                }
            }
        }
        if sample.is_empty() {
            // Dropped by the encoder to keep to its rate: the last frame stays up a frame longer.
            if let Some(last) = self.samples.last_mut() {
                last.2 += 1;
            }
            return Ok(());
        }
        let overhead = 64 * 1024 + self.sps.len() + self.pps.len() + 32 * (self.samples.len() + 1);
        if self.bytes + sample.len() + overhead > MAX_BYTES {
            self.full = true;
            return Ok(());
        }
        self.bytes += sample.len();
        self.samples.push((sample, sync, 1));
        Ok(())
    }

    /// Writes the file. Returns its size in bytes.
    pub fn finish(self) -> Result<u64, String> {
        if self.samples.is_empty() || self.sps.is_empty() || self.pps.is_empty() {
            return Err("no frames".into());
        }
        let brand = |b: &str| b.parse::<mp4::FourCC>().unwrap();
        let config = mp4::Mp4Config { major_brand: brand("isom"), minor_version: 512, compatible_brands: vec![brand("isom"), brand("iso2"), brand("avc1"), brand("mp41")], timescale: 1000 };
        let file = std::fs::File::create(&self.path).map_err(|e| format!("{}: {e}", self.path))?;
        let mut w = mp4::Mp4Writer::write_start(std::io::BufWriter::new(file), &config).map_err(|e| e.to_string())?;
        let avc = mp4::AvcConfig { width: self.width as u16, height: self.height as u16, seq_param_set: self.sps, pic_param_set: self.pps };
        w.add_track(&mp4::TrackConfig { track_type: mp4::TrackType::Video, timescale: self.fps * 1000, language: "und".into(), media_conf: mp4::MediaConfig::AvcConfig(avc) }).map_err(|e| e.to_string())?;
        let mut at = 0u64;
        for (bytes, is_sync, frames) in self.samples {
            let sample = mp4::Mp4Sample { start_time: at, duration: frames * 1000, rendering_offset: 0, is_sync, bytes: mp4::Bytes::from(bytes) };
            w.write_sample(1, &sample).map_err(|e| e.to_string())?;
            at += frames as u64 * 1000;
        }
        w.write_end().map_err(|e| e.to_string())?;
        drop(w);
        std::fs::metadata(&self.path).map(|m| m.len()).map_err(|e| e.to_string())
    }
}

/// Starts a video at path; null if it can't.
#[no_mangle]
pub extern "C" fn aztek_video_begin(path: *const c_char, width: i32, height: i32, fps: i32, bits_per_second: i32) -> *mut c_void {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();
    match Recording::new(&path, width.max(0) as usize, height.max(0) as usize, fps.max(1) as u32, bits_per_second.max(100_000) as u32) {
        Ok(r) => Box::into_raw(Box::new(r)) as *mut c_void,
        Err(e) => {
            log::warn!("video: {e}");
            std::ptr::null_mut()
        }
    }
}

/// Adds a frame: width x height RGBA (QImage::Format_RGBA8888) with rows `stride` bytes apart. 0 when it went in, 2
/// when the video has reached the most a file may be and takes no more, 1 on an error.
#[no_mangle]
pub extern "C" fn aztek_video_frame(video: *mut c_void, rgba: *const u8, stride: i32) -> i32 {
    if video.is_null() || rgba.is_null() {
        return 1;
    }
    let r = unsafe { &mut *(video as *mut Recording) };
    let len = stride.max(0) as usize * r.height;
    let data = unsafe { std::slice::from_raw_parts(rgba, len) };
    match r.frame(data, stride.max(0) as usize) {
        Ok(()) if r.full => 2,
        Ok(()) => 0,
        Err(e) => {
            log::warn!("video: {e}");
            1
        }
    }
}

/// Writes the file and frees the video; with `keep` 0, frees it and writes nothing. Returns the file's size, or -1.
#[no_mangle]
pub extern "C" fn aztek_video_end(video: *mut c_void, keep: i32) -> i64 {
    if video.is_null() {
        return -1;
    }
    let r = unsafe { Box::from_raw(video as *mut Recording) };
    if keep == 0 {
        return -1;
    }
    match r.finish() {
        Ok(n) => n as i64,
        Err(e) => {
            log::warn!("video: {e}");
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_playable_mp4() {
        let dir = std::env::temp_dir().join(format!("aztek-video-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("v.mp4");
        let (w, h) = (64, 48);
        let mut r = Recording::new(path.to_str().unwrap(), w, h, 24, 500_000).unwrap();
        for f in 0..12 {
            let rgba: Vec<u8> = (0..w * h).flat_map(|i| [(i + f * 3) as u8, (i / w) as u8, 90, 255]).collect();
            r.frame(&rgba, w * 4).unwrap();
        }
        let size = r.finish().unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, size);
        assert_eq!(&bytes[4..8], b"ftyp");
        assert!(bytes.windows(4).any(|x| x == b"avcC"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn busy(r: &mut Recording, w: usize, h: usize, frames: usize) {
        let mut seed = 12345u32;
        for _ in 0..frames {
            let rgba: Vec<u8> = (0..w * h * 4)
                .map(|_| {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    (seed >> 24) as u8
                })
                .collect();
            r.frame(&rgba, w * 4).unwrap();
        }
    }

    /// Ten seconds of 720p where every pixel changes every frame, the worst a recording can be, at the report window's
    /// rate: under GitHub's 10 MB however the encoder copes.
    #[test]
    fn ten_busy_seconds_stay_under_ten_megabytes() {
        let dir = std::env::temp_dir().join(format!("aztek-video-busy-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("v.mp4");
        let (w, h) = (1280, 720);
        let mut r = Recording::new(path.to_str().unwrap(), w, h, 24, 2_000_000).unwrap();
        busy(&mut r, w, h, 240);
        assert!(!r.full);
        let size = r.finish().unwrap();
        assert!(size < 10 * 1024 * 1024, "{size} bytes");

        // At a rate far past what the file may hold, the recording stops at the limit and is still a whole file.
        let mut r = Recording::new(path.to_str().unwrap(), w, h, 24, 60_000_000).unwrap();
        busy(&mut r, w, h, 240);
        assert!(r.full);
        let size = r.finish().unwrap();
        assert!(size <= MAX_BYTES as u64 && size < 10 * 1024 * 1024, "{size} bytes");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
