//! vortex -axisX -axisY -axisZ (-ax -ay -az): the axis a vortex turns about, given on creation, changed with -e and
//! read with -q. The reference's defaults are 0 1 0, with magnitude 5 and attenuation 1.

use aztek_host::Core;
use serde_json::Value;

fn ok(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) if a.len() == 1 => num(&a[0]),
        x => x.as_f64().unwrap_or_else(|| panic!("not a number: {x}")),
    }
}

fn axis(c: &mut Core, field: &str) -> [f64; 3] {
    ["ax", "ay", "az"].map(|f| num(&ok(c, &format!("vortex -q -{f} {field};"))))
}

#[test]
fn vortex_takes_its_axis() {
    let mut c = Core::new();
    ok(&mut c, "vortex -n plain;");
    assert_eq!(axis(&mut c, "plain"), [0.0, 1.0, 0.0], "the default axis");
    assert_eq!(num(&ok(&mut c, "vortex -q -m plain;")), 5.0);
    assert_eq!(num(&ok(&mut c, "vortex -q -att plain;")), 1.0);

    ok(&mut c, "vortex -n tipped -ax 1 -ay 0 -az 0.5;");
    assert_eq!(axis(&mut c, "tipped"), [1.0, 0.0, 0.5]);
    // Long names, and an edit that changes one component keeps the others.
    ok(&mut c, "vortex -e -axisZ -2 tipped;");
    assert_eq!(axis(&mut c, "tipped"), [1.0, 0.0, -2.0]);
    assert_eq!(num(&ok(&mut c, "vortex -q -axisX tipped;")), 1.0);
}

#[test]
fn only_vortex_has_an_axis() {
    let mut c = Core::new();
    assert!(c.mel_checked("gravity -ax 1;").is_err(), "gravity has no -ax in the reference");
    // The other fields' flags query from a script the same way.
    ok(&mut c, "gravity -n g -m 3;");
    assert_eq!(num(&ok(&mut c, "gravity -q -m g;")), 3.0);
}
