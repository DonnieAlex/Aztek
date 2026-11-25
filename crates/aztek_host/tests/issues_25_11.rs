//! Issues of 25/11/2026.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    let v = c.mel(src);
    let errors: Vec<String> = c.take_log().as_array().unwrap().iter().filter(|l| l["kind"] == 2).map(|l| l["text"].as_str().unwrap().to_string()).collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    v
}

#[test]
fn layer_weight_ticks_show_on_the_time_slider_by_default() {
    // #293: Display Key Ticks > Show Layer Weight is on in a new session, as in the reference.
    let mut c = Core::new();
    assert_eq!(mel(&mut c, "timeControl -q -asw;"), json!(true));
    mel(&mut c, "polyCube;");
    mel(&mut c, "animLayer -aso L1;");
    mel(&mut c, "animLayer -e -sel 1 -prf 1 L1;");
    mel(&mut c, "setKeyframe -t 7 -v 0.5 L1.weight;");
    assert_eq!(mel(&mut c, "keyTimes;"), json!([7.0]));
}
