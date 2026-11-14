//! Harold scene files: a scene written and read back renders the same, and the `harold` program renders one.

use std::sync::Arc;

use aztek_render::camera::Camera;
use aztek_render::geom::{quad, sphere, Instance};
use aztek_render::lights::{Light, Shape};
use aztek_render::scene::{Options, Scene};
use aztek_render::shader::{Input, Material, Standard};
use glam::{Mat4, Vec3, Vec4};

fn scene() -> Scene {
    let mut s = Scene::new(Options { width: 24, height: 16, aa_samples: 2, diffuse_samples: 1, specular_samples: 1, seed: 7, ..Default::default() });
    s.camera = Camera::looking(Vec3::new(0.0, 1.0, 5.0), Vec3::ZERO, 45.0);
    s.geo.meshes.push(Arc::new(sphere(Vec3::ZERO, 1.0, 16, 32)));
    s.geo.meshes.push(Arc::new(quad(Vec3::new(-4.0, -1.0, -4.0), Vec3::new(4.0, -1.0, -4.0), Vec3::new(4.0, -1.0, 4.0), Vec3::new(-4.0, -1.0, 4.0))));
    let red = s.add_material("red", Material::Standard(Box::new(Standard { base: Input::c(1.0), base_color: Input::Const(Vec3::new(0.8, 0.2, 0.1)), specular: Input::c(0.5), ..Default::default() })));
    let grey = s.add_material("grey", Material::Standard(Box::new(Standard { base: Input::c(1.0), base_color: Input::c(0.5), ..Default::default() })));
    s.geo.instances.push(Instance::new("ball", 0, Mat4::IDENTITY, vec![red]));
    s.geo.instances.push(Instance::new("floor", 1, Mat4::IDENTITY, vec![grey]));
    s.lights.push(Light::new("key", Shape::Point { pos: Vec3::new(2.0, 4.0, 3.0), radius: 0.2 }, Vec3::splat(60.0)));
    s.lights.push(Light::new("sky", Shape::Dome { to_local: Mat4::IDENTITY, image: None, dist: None, source: None }, Vec3::new(0.2, 0.25, 0.3)));
    s.prepare();
    s
}

fn pixels(s: Scene) -> Vec<Vec4> {
    let (film, _) = aztek_render::render::render(Arc::new(s), 1);
    film.pixels(film.index_of("RGBA").unwrap())
}

#[test]
fn written_and_read_back_it_renders_the_same() {
    let text = aztek_render::hss::to_string(&scene()).unwrap();
    assert!(text.starts_with("{\"harold\":1,"), "{}", &text[..40]);
    let again = aztek_render::hss::from_str(&text).unwrap();
    assert_eq!(again.geo.instances.len(), 2);
    assert_eq!(again.lights.len(), 2);
    assert_eq!(pixels(scene()), pixels(again));
}

#[test]
fn later_versions_and_other_files_are_refused() {
    let refused = |t: &str| aztek_render::hss::from_str(t).err().unwrap_or_default();
    assert!(refused("{\"harold\": 99, \"scene\": {}}").contains("version 99"));
    assert!(!refused("createNode transform;").is_empty());
}

#[test]
fn the_harold_program_renders_a_file() {
    let dir = std::env::temp_dir().join(format!("aztek_hss_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("ball.hss");
    aztek_render::hss::write(&scene(), &file).unwrap();
    let out = dir.join("ball.png");
    let st = std::process::Command::new(env!("CARGO_BIN_EXE_harold")).args(["-i", file.to_str().unwrap(), "-o", out.to_str().unwrap(), "-r", "12", "8", "-as", "1", "-t", "1", "-v", "0"]).status().unwrap();
    assert!(st.success());
    let img = image_size(&out);
    assert_eq!(img, (12, 8));
    // No output named: the scene's name with .exr.
    let st = std::process::Command::new(env!("CARGO_BIN_EXE_harold")).args([file.to_str().unwrap(), "-v", "0", "-dw"]).status().unwrap();
    assert!(st.success());
    assert!(dir.join("ball.exr").exists());
    let bad = std::process::Command::new(env!("CARGO_BIN_EXE_harold")).args(["-x"]).output().unwrap();
    assert!(!bad.status.success());
}

/// Width and height from a PNG's header.
fn image_size(p: &std::path::Path) -> (u32, u32) {
    let b = std::fs::read(p).unwrap();
    (u32::from_be_bytes(b[16..20].try_into().unwrap()), u32::from_be_bytes(b[20..24].try_into().unwrap()))
}
