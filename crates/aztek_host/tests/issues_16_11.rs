//! Issues filed on 16/11/2026, as scripts reach them.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

/// #103: Delete on edges dissolves them, the faces either side merged; the node says withFaces off.
#[test]
fn delete_edges_dissolves() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 2 -sy 1 -n p;");
    assert_eq!(mel(&mut c, "polyEvaluate -f p;"), json!(2));
    mel(&mut c, "select p.e[1]; deleteComponents;");
    assert_eq!(mel(&mut c, "polyEvaluate -f p;"), json!(1));
    let node = mel(&mut c, "ls -type polyDelEdge;")[0].as_str().unwrap().to_string();
    assert_eq!(mel(&mut c, &format!("getAttr {node}.withFaces;")), json!(false));
}

/// #91: an OBJ with no object names comes in as polySurface, its own "default" group passed over; a named group names it.
#[test]
fn obj_names() {
    let dir = std::env::temp_dir().join(format!("az_obj_names_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let plain = dir.join("plain.obj");
    std::fs::write(&plain, "g default\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n").unwrap();
    let grouped = dir.join("grouped.obj");
    std::fs::write(&grouped, "g default\nv 0 0 0\nv 1 0 0\nv 0 1 0\ng tri\nf 1 2 3\n").unwrap();
    let mut c = Core::new();
    mel(&mut c, "loadPlugin objExport;");
    let a = mel(&mut c, &format!("importObj \"{}\";", plain.to_string_lossy().replace('\\', "/")));
    assert_eq!(a, json!(["polySurface1"]));
    let b = mel(&mut c, &format!("importObj \"{}\";", grouped.to_string_lossy().replace('\\', "/")));
    assert_eq!(b, json!(["tri"]));
    let _ = std::fs::remove_dir_all(&dir);
}

/// #89: default nodes among the ones deleted are passed over; alone they are an error.
#[test]
fn delete_passes_over_default_nodes() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a; polySphere -n b;");
    mel(&mut c, "delete a defaultLightSet b;");
    assert_eq!(mel(&mut c, "objExists a;"), json!(false));
    assert_eq!(mel(&mut c, "objExists b;"), json!(false));
    assert_eq!(mel(&mut c, "objExists defaultLightSet;"), json!(true));
    assert!(c.mel_checked("delete defaultLightSet;").is_err());
}

/// #94: the pieces of a separated mesh keep its materials, whole or by face.
#[test]
fn separate_keeps_materials() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n a; polyCube -n b; move 3 0 0 b; polyUnite -n ab a b; delete -ch ab;");
    mel(&mut c, "shadingNode -asShader lambert -n red; sets -renderable true -noSurfaceShader true -empty -n redSG; connectAttr red.outColor redSG.surfaceShader;");
    mel(&mut c, "shadingNode -asShader lambert -n blue; sets -renderable true -noSurfaceShader true -empty -n blueSG; connectAttr blue.outColor blueSG.surfaceShader;");
    // The first cube's faces red, the second's top face blue, the rest of it red.
    mel(&mut c, "sets -e -fe redSG ab; sets -e -fe blueSG ab.f[7];");
    let made = mel(&mut c, "polySeparate ab;");
    let pieces: Vec<String> = made.as_array().unwrap().iter().filter_map(Value::as_str).filter(|n| !n.contains("polySeparate")).map(str::to_string).collect();
    assert_eq!(pieces.len(), 2);
    let red = mel(&mut c, "sets -q redSG;");
    let blue = mel(&mut c, "sets -q blueSG;");
    let first_shape = mel(&mut c, &format!("listRelatives -s {};", pieces[0]))[0].as_str().unwrap().to_string();
    let second = pieces[1].trim_start_matches('|');
    let red_s = red.to_string();
    assert!(red_s.contains(&format!("\"{first_shape}\"")), "{red}");
    assert!(red_s.contains(&format!("{second}.f[")), "{red}");
    assert!(blue.to_string().contains(&format!("{second}.f[1]")), "{blue}");
    // -ch 0 as well.
    mel(&mut c, "polyCube -n c; polyCube -n d; move 3 0 0 d; polyUnite -n cd c d; delete -ch cd; sets -e -fe blueSG cd;");
    let plain = mel(&mut c, "polySeparate -ch 0 cd;");
    assert_eq!(plain.as_array().unwrap().len(), 2);
    let blue = mel(&mut c, "sets -q blueSG;").to_string();
    for p in plain.as_array().unwrap() {
        let shape = mel(&mut c, &format!("listRelatives -s {};", p.as_str().unwrap()))[0].as_str().unwrap().to_string();
        assert!(blue.contains(&format!("\"{shape}\"")), "{blue} {shape}");
    }
}

/// #102: move -r -co -ls pushes components along their normals by the first value.
#[test]
fn move_along_normals() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 1 -sy 1 -n p;");
    mel(&mut c, "move -r -co -ls -wd 2 0 0 p.vtx[0];");
    let v = mel(&mut c, "pointPosition -w p.vtx[0];");
    assert!((v[1].as_f64().unwrap() - 2.0).abs() < 1e-6, "{v}");
    assert!((v[0].as_f64().unwrap() + 0.5).abs() < 1e-6, "{v}");
    // From the selection too.
    mel(&mut c, "polyPlane -sx 1 -sy 1 -n q;");
    mel(&mut c, "select q.vtx[1]; move -r -co -ls -wd -1 0 0;");
    let v = mel(&mut c, "pointPosition -w q.vtx[1];");
    assert!((v[1].as_f64().unwrap() + 1.0).abs() < 1e-6, "{v}");
}

/// #92: any type an importer reads can be referenced: its nodes come in under the namespace, tracked as the reference's.
#[test]
fn reference_an_obj() {
    let dir = std::env::temp_dir().join(format!("az_ref_obj_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("tri.obj");
    std::fs::write(&path, "o tri\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n").unwrap();
    let p = path.to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, "loadPlugin objExport;");
    mel(&mut c, &format!("file -r -ns \"mocap\" \"{p}\";"));
    assert_eq!(mel(&mut c, "objExists mocap:tri;"), json!(true));
    assert_eq!(mel(&mut c, "referenceQuery -isNodeReferenced mocap:tri;"), json!(true));
    // Unloaded, it goes; loaded again, it comes back.
    let rfn = mel(&mut c, "referenceQuery -rfn mocap:tri;").as_str().unwrap().to_string();
    mel(&mut c, &format!("file -unloadReference {rfn};"));
    assert_eq!(mel(&mut c, "objExists mocap:tri;"), json!(false));
    mel(&mut c, &format!("file -loadReference {rfn};"));
    assert_eq!(mel(&mut c, "objExists mocap:tri;"), json!(true));
    let _ = std::fs::remove_dir_all(&dir);
}

/// #100: a stage referencing an Alembic file reads it, animation and all, as the reference's usdAbc does.
#[test]
fn usd_references_alembic() {
    let dir = std::env::temp_dir().join(format!("az_usdabc_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let d = dir.to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, "polyCube -n box; setKeyframe -t 1 -v 0 box.ty; setKeyframe -t 10 -v 5 box.ty;");
    mel(&mut c, &format!("AbcExport -j \"-frameRange 1 10 -uvWrite -worldSpace -root |box -file {d}/box.abc\";"));
    std::fs::write(dir.join("shot.usda"), "#usda 1.0\n\ndef Xform \"set\"\n{\n    def \"box\" (\n        references = @./box.abc@</box>\n    )\n    {\n    }\n}\n").unwrap();
    mel(&mut c, "file -f -new;");
    mel(&mut c, &format!("mayaUSDImport -file \"{d}/shot.usda\" -readAnimData 1;"));
    let meshes = mel(&mut c, "ls -type mesh;");
    assert!(!meshes.as_array().unwrap().is_empty(), "{meshes}");
    // The cube's animation came through: it moves between frames 1 and 10.
    let top = mel(&mut c, "ls -assemblies;");
    let set = top.as_array().unwrap().iter().filter_map(Value::as_str).find(|n| n.starts_with("set")).unwrap_or("set").to_string();
    mel(&mut c, "currentTime 1;");
    let a = mel(&mut c, &format!("xform -q -ws -bb {set};"));
    mel(&mut c, "currentTime 10;");
    let b = mel(&mut c, &format!("xform -q -ws -bb {set};"));
    assert!((b[1].as_f64().unwrap() - a[1].as_f64().unwrap() - 5.0).abs() < 1e-3, "{a} {b}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// #108: the UV Toolkit's further tools by their MEL names and short flags.
#[test]
fn uv_toolkit_from_mel() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 2 -sy 2 -n p;");
    mel(&mut c, "polyNormalizeUV -nt 1 -nd 1 p;");
    mel(&mut c, "unfold -oa 2 p;");
    mel(&mut c, "polyRotateUVsByVertex p.f[0];");
    mel(&mut c, "polyForceUV -uni p.f[1];");
    mel(&mut c, "texCreateShell -g 1 p.f[2:3];");
    mel(&mut c, "texLayoutAlong -d v p;");
    mel(&mut c, "texStraightenShell p.map[0];");
    mel(&mut c, "texBrushUVs -m smooth -r 0.3 -sm {0.5, 0.5, 0.0, 0.0} p;");
    mel(&mut c, "polyProjection -t planar -md n p.f[*];");
    assert_eq!(mel(&mut c, "texSelectByType -t unmapped p;"), json!([]));
}

/// #107: Camera Based Selection: with selectPref -useDepth on, a marquee takes only the vertices the camera sees.
#[test]
fn camera_based_selection() {
    use aztek_host::camera::CamKind;
    use aztek_host::viewport::{gather, Viewport};
    let mut c = Core::new();
    mel(&mut c, "polySphere -r 1 -n s; select s; selectMode -component; selectType -vertex 1;");
    let mut vp = Viewport::new(CamKind::Persp);
    vp.rect = [0.0, 0.0, 800.0, 800.0];
    vp.cam.coi = glam::DVec3::ZERO;
    vp.cam.yaw = 0.0;
    vp.cam.pitch = 0.0;
    vp.cam.distance = 10.0;
    let count = |c: &mut Core, vp: &Viewport| {
        let items = gather(&mut c.s.graph);
        let picked = vp.pick_rect(&c.s, &items, glam::DVec2::ZERO, glam::DVec2::new(800.0, 800.0));
        let n = picked.len();
        mel(c, &format!("select -r {};", picked.join(" ")));
        let _ = n;
        mel(c, "size(`ls -sl -fl`);").as_i64().unwrap()
    };
    let all = count(&mut c, &vp);
    mel(&mut c, "select s; selectPref -useDepth 1;");
    let seen = count(&mut c, &vp);
    mel(&mut c, "selectPref -useDepth 0;");
    assert!(all > 300 && seen < all * 2 / 3 && seen > all / 4, "{all} {seen}");
}

/// #107: the Modeling Toolkit's Object menu: hide, freeze, X-ray and backfaces of the selection.
#[test]
fn modeling_toolkit_object_menu() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n b; select b;");
    mel(&mut c, "string $s[] = `ls -sl -o -tr`; for ($o in $s) { int $on = `getAttr ($o + \".overrideDisplayType\")` == 2 && `getAttr ($o + \".overrideEnabled\")`; setAttr ($o + \".overrideEnabled\") (!$on); setAttr ($o + \".overrideDisplayType\") ($on ? 0 : 2); }");
    assert_eq!(mel(&mut c, "getAttr b.overrideDisplayType;"), json!(2));
    mel(&mut c, "string $s[] = `ls -sl -o`; for ($o in $s) { int $x[] = `displaySurface -q -xRay $o`; displaySurface -xRay (!$x[0]) $o; }");
    assert_eq!(mel(&mut c, "displaySurface -q -xRay b;"), json!([true]));
    mel(&mut c, "string $s[] = `ls -sl -o`; for ($o in $s) { int $c = `displayCull -q -bfc $o`; displayCull -bfc (!$c) $o; }");
    assert_eq!(mel(&mut c, "displayCull -q -bfc b;"), json!(true));
    mel(&mut c, "string $s[] = `ls -sl -o`; for ($o in $s) { if (`objExists ($o + \".visibility\")`) setAttr ($o + \".visibility\") (!`getAttr ($o + \".visibility\")`); }");
    assert_eq!(mel(&mut c, "getAttr b.visibility;"), json!(false));
}

/// PLAN 8.4: Time Slider bookmarks are timeSliderBookmark nodes, kept in the scene file.
#[test]
fn time_slider_bookmarks() {
    let dir = std::env::temp_dir().join(format!("az_bookmarks_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("b.ma").to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, "string $b = `createNode timeSliderBookmark`; setAttr ($b + \".name\") -type \"string\" \"walk\"; setAttr ($b + \".timeRangeStart\") 10; setAttr ($b + \".timeRangeStop\") 24; setAttr ($b + \".color\") 1 0 0;");
    mel(&mut c, &format!("file -rename \"{p}\"; file -save -type \"mayaAscii\";"));
    mel(&mut c, "file -f -new;");
    assert_eq!(mel(&mut c, "ls -type timeSliderBookmark;"), json!([]));
    mel(&mut c, &format!("file -f -open \"{p}\";"));
    assert_eq!(mel(&mut c, "ls -type timeSliderBookmark;"), json!(["timeSliderBookmark1"]));
    assert_eq!(mel(&mut c, "getAttr timeSliderBookmark1.name;"), json!("walk"));
    assert_eq!(mel(&mut c, "getAttr timeSliderBookmark1.timeRangeStop;"), json!(24.0));
    let all = aztek_scene::bookmarks::all(&c.s.graph);
    assert_eq!(all[0]["start"], json!(10.0));
    let _ = std::fs::remove_dir_all(&dir);
}
