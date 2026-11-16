//! File > Optimize Scene Size (cleanUpScene) and the Hypershade's Delete Unused Nodes (MLdeleteUnused).

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

fn exists(c: &mut Core, n: &str) -> bool {
    mel(c, &format!("objExists {n};")) == json!(true)
}

#[test]
fn optimize_removes_what_is_unused_and_keeps_the_rest() {
    let mut c = Core::new();
    mel(&mut c, "polyCube -n box; group -em -n emptyA; group -em -n emptyB; parent emptyB emptyA;");
    mel(&mut c, "sets -em -n emptySet; sets -n fullSet box;");
    mel(&mut c, "createDisplayLayer -e -n emptyLayer; select box; createDisplayLayer -n fullLayer;");
    mel(&mut c, "spaceLocator -n loneLoc; spaceLocator -n usedLoc; connectAttr usedLoc.tx box.ty;");
    mel(&mut c, "setKeyframe -at tx box; createNode animCurveTL -n strayCurve;");
    mel(&mut c, "shadingNode -asShader lambert -n unusedMat; shadingNode -asTexture checker -n unusedChecker; connectAttr unusedChecker.outColor unusedMat.color;");
    mel(&mut c, "shadingNode -asShader lambert -n usedMat; sets -renderable true -noSurfaceShader true -empty -n usedMatSG; connectAttr usedMat.outColor usedMatSG.surfaceShader; sets -e -forceElement usedMatSG box;");
    mel(&mut c, "createNode unitConversion -n strayConv; createNode pairBlend -n strayBlend;");
    let gone = mel(&mut c, "cleanUpScene;");
    for n in ["emptyA", "emptyB", "emptySet", "emptyLayer", "loneLoc", "strayCurve", "unusedMat", "unusedChecker", "strayConv", "strayBlend"] {
        assert!(!exists(&mut c, n), "{n} kept: {gone}");
    }
    for n in ["box", "fullSet", "fullLayer", "usedLoc", "box_translateX", "usedMat", "usedMatSG", "lambert1", "initialShadingGroup", "defaultLightSet", "persp"] {
        assert!(exists(&mut c, n), "{n} went: {gone}");
    }
    // One undo brings it all back.
    c.mel("undo;");
    assert!(exists(&mut c, "emptyA") && exists(&mut c, "unusedChecker"));
}

#[test]
fn categories_and_delete_unused_nodes() {
    let mut c = Core::new();
    mel(&mut c, "group -em -n emptyA; shadingNode -asShader blinn -n unusedBlinn;");
    mel(&mut c, "cleanUpScene -emptyTransforms;");
    assert!(!exists(&mut c, "emptyA") && exists(&mut c, "unusedBlinn"));
    mel(&mut c, "group -em -n emptyB; MLdeleteUnused;");
    assert!(exists(&mut c, "emptyB") && !exists(&mut c, "unusedBlinn"));
}

#[test]
fn archive_scene_and_file_list() {
    let tmp = std::env::temp_dir().join(format!("aztek_archive_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(tmp.join("sourceimages")).unwrap();
    let outside = std::env::temp_dir().join(format!("aztek_archive_out_{}", std::process::id()));
    std::fs::create_dir_all(&outside).unwrap();
    let tex = tmp.join("sourceimages/wood.png");
    std::fs::write(&tex, b"not really a png").unwrap();
    let far = outside.join("far.png");
    std::fs::write(&far, b"far").unwrap();
    let scenes = tmp.join("scenes");
    std::fs::create_dir_all(&scenes).unwrap();
    let scene = scenes.join("shot.ma").to_string_lossy().replace('\\', "/");
    let p = |x: &std::path::Path| x.to_string_lossy().replace('\\', "/");
    let mut c = Core::new();
    mel(&mut c, &format!("workspace -o \"{}\";", p(&tmp)));
    mel(&mut c, &format!("shadingNode -asTexture file -n wood; setAttr -type \"string\" wood.fileTextureName \"{}\";", p(&tex)));
    mel(&mut c, &format!("shadingNode -asTexture file -n far; setAttr -type \"string\" far.fileTextureName \"{}\";", p(&far)));
    mel(&mut c, "shadingNode -asTexture file -n lost; setAttr -type \"string\" lost.fileTextureName \"/nowhere/lost.png\";");
    assert!(c.mel_checked("archiveScene;").is_err(), "an unsaved scene can't be archived");
    mel(&mut c, &format!("file -rename \"{scene}\"; file -save -type \"mayaAscii\";"));
    let list = mel(&mut c, "file -q -list;");
    assert_eq!(list[0], json!(scene));
    assert!(list.as_array().unwrap().iter().any(|x| x == &json!(p(&tex))), "{list}");
    assert!(list.as_array().unwrap().iter().any(|x| x == &json!("/nowhere/lost.png")), "{list}");
    let zip_path = mel(&mut c, "archiveScene;");
    assert_eq!(zip_path, json!(p(&scenes.join("shot.zip"))));
    let mut names: Vec<String> = {
        let f = std::fs::File::open(scenes.join("shot.zip")).unwrap();
        let z = zip::ZipArchive::new(f).unwrap();
        z.file_names().map(|n| n.unwrap().to_string()).collect()
    };
    names.sort();
    assert_eq!(names, vec!["external/far.png", "shot.ma", "sourceimages/wood.png"]);
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::remove_dir_all(&outside);
}
