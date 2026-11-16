//! Sound: an audio node from a WAV file on the timeline, its length and end, the waveform the time slider shows, and
//! scene files.

use aztek_scene::Session;
use serde_json::{json, Value};

fn run(s: &mut Session, name: &str, call: Value) -> Value {
    s.run(name, call).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// One second of a 300 Hz tone at 8 kHz, 16-bit mono.
fn wav() -> String {
    let rate = 8000u32;
    let samples: Vec<i16> = (0..rate).map(|i| ((i as f64 * 300.0 * std::f64::consts::TAU / rate as f64).sin() * 16000.0) as i16).collect();
    let data: Vec<u8> = samples.iter().flat_map(|x| x.to_le_bytes()).collect();
    let mut b = b"RIFF".to_vec();
    b.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    for v in [16u32.to_le_bytes().to_vec(), 1u16.to_le_bytes().to_vec(), 1u16.to_le_bytes().to_vec(), rate.to_le_bytes().to_vec(), (rate * 2).to_le_bytes().to_vec(), 2u16.to_le_bytes().to_vec(), 16u16.to_le_bytes().to_vec()] {
        b.extend(v);
    }
    b.extend_from_slice(b"data");
    b.extend_from_slice(&(data.len() as u32).to_le_bytes());
    b.extend(data);
    let p = std::env::temp_dir().join(format!("tone_{}.wav", std::process::id()));
    std::fs::write(&p, b).unwrap();
    p.to_string_lossy().replace('\\', "/")
}

#[test]
fn a_wav_on_the_timeline() {
    let mut s = Session::new();
    let f = wav();
    let n = run(&mut s, "sound", json!({"file": f, "offset": 10.0, "name": "tone"}));
    assert_eq!(n, json!("tone"));
    assert_eq!(run(&mut s, "sound", json!({"args": ["tone"], "query": true, "queried": ["length"]})), json!(24.0));
    assert_eq!(run(&mut s, "sound", json!({"args": ["tone"], "query": true, "queried": ["endTime"]})), json!(34.0));
    run(&mut s, "timeControl", json!({"sound": "tone", "displaySound": true}));
    assert_eq!(run(&mut s, "timeControl", json!({"query": true, "queried": ["sound"]})), json!("tone"));
    let p = run(&mut s, "aztekSoundPeaks", json!({"start": 0.0, "end": 40.0, "count": 40}));
    let p: Vec<(f64, f64)> = p.as_array().unwrap().iter().map(|x| (x[0].as_f64().unwrap(), x[1].as_f64().unwrap())).collect();
    assert_eq!(p.len(), 40);
    assert!(p[..10].iter().all(|x| *x == (0.0, 0.0)));
    assert!(p[11..33].iter().all(|x| x.0 < -0.4 && x.1 > 0.4), "{:?}", &p[11..14]);
    assert!(p[35..].iter().all(|x| *x == (0.0, 0.0)));
    // Half of it, from the file's middle.
    run(&mut s, "sound", json!({"args": ["tone"], "edit": true, "sourceStart": 12.0}));
    assert_eq!(run(&mut s, "sound", json!({"args": ["tone"], "query": true, "queried": ["length"]})), json!(12.0));
    let path = std::env::temp_dir().join(format!("sound_{}.ma", std::process::id())).to_string_lossy().to_string();
    run(&mut s, "file", json!({"args": [path.clone()], "save": true}));
    let mut t = Session::new();
    run(&mut t, "file", json!({"args": [path], "open": true, "force": true}));
    assert_eq!(run(&mut t, "sound", json!({"args": ["tone"], "query": true, "queried": ["file"]})), json!(f));
    assert_eq!(run(&mut t, "sound", json!({"args": ["tone"], "query": true, "queried": ["endTime"]})), json!(22.0));
    assert!(s.run("sound", json!({"file": "/no/such.wav"})).is_err());
}
