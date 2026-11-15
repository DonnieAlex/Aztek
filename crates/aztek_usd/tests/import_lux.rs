//! Issues 41 and 42: Scopes merge on a second import; UsdLux lights (Arnold's too) come in as lights; a Scope of render
//! settings makes no group.

use aztek_scene::session::Session;
use serde_json::{json, Value};

fn session() -> Session {
    let mut s = Session::new();
    aztek_scene::cameras::make_defaults(&mut s.graph);
    aztek_usd::install(&mut s);
    s
}

fn write(name: &str, text: &str) -> String {
    let dir = std::env::temp_dir().join(format!("aztek_usd_lux_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join(name).to_string_lossy().into_owned();
    std::fs::write(&f, text).unwrap();
    f
}

fn get(s: &mut Session, plug: &str) -> Value {
    s.run("getAttr", json!({"args": [plug]})).unwrap()
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Array(a) => num(&a[0]),
        other => other.as_f64().unwrap_or(f64::NAN),
    }
}

const LIGHTS: &str = r#"#usda 1.0
(
    upAxis = "Y"
    metersPerUnit = 0.01
)
def Scope "lights"
{
    def SphereLight "key"
    {
        float inputs:intensity = 3
        float inputs:exposure = 2
        color3f inputs:color = (1, 0.5, 0.25)
        float inputs:radius = 4
        float inputs:shaping:cone:angle = 30
        float inputs:shaping:cone:softness = 0.5
    }
    def DistantLight "sun"
    {
        float inputs:angle = 0.53
    }
    def RectLight "card"
    {
        float inputs:width = 6
        float inputs:height = 2
    }
    def DiskLight "disc"
    {
        float inputs:radius = 3
    }
    def DomeLight "sky"
    {
        asset inputs:texture:file = @sky.hdr@
        token inputs:texture:format = "latlong"
    }
}
def Scope "Render"
{
    def RenderSettings "rendersettings1"
    {
    }
}
"#;

#[test]
fn usd_lux_lights_come_in_as_lights() {
    let mut s = session();
    let f = write("lights.usda", LIGHTS);
    s.run("mayaUSDImport", json!({"file": f, "primPath": "/"})).unwrap();
    let ty = |s: &mut Session, n: &str| s.run("nodeType", json!({"args": [n]})).unwrap();
    assert_eq!(ty(&mut s, "keyShape"), json!("spotLight"));
    assert_eq!(ty(&mut s, "sunShape"), json!("directionalLight"));
    assert_eq!(ty(&mut s, "cardShape"), json!("areaLight"));
    assert_eq!(ty(&mut s, "discShape"), json!("aiAreaLight"));
    assert_eq!(ty(&mut s, "skyShape"), json!("aiSkyDomeLight"));
    assert_eq!(num(&get(&mut s, "keyShape.intensity")), 3.0);
    assert_eq!(num(&get(&mut s, "keyShape.aiExposure")), 2.0);
    assert!((num(&get(&mut s, "keyShape.coneAngle")) - 60.0).abs() < 1e-4, "{}", get(&mut s, "keyShape.coneAngle"));
    assert_eq!(get(&mut s, "keyShape.color"), json!([1.0, 0.5, 0.25]));
    assert_eq!(get(&mut s, "card.scale"), json!([3.0, 1.0, 1.0]));
    assert_eq!(num(&get(&mut s, "discShape.aiTranslator")), 1.0);
    let tex = s.run("listConnections", json!({"args": ["skyShape.color"], "source": true, "destination": false})).unwrap();
    let tex = tex[0].as_str().unwrap().to_string();
    assert!(get(&mut s, &format!("{tex}.fileTextureName")).to_string().ends_with("sky.hdr\""));
    // The render settings' scope makes no empty group.
    assert_eq!(s.run("objExists", json!({"args": ["Render"]})).unwrap(), json!(false));
}

