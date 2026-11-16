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
