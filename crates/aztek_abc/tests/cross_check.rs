//! What Aztek writes, read back by Aztek and by ogawa-rs, an independent reader of the reference's Ogawa files.
//! ogawa-rs maps animated samples differently from the reference's reader (it takes a property written with no
//! first and last changed index as starting to change at sample 0, and does not step over array dimensions), so it
//! is given constant geometry; Aztek's own reader checks the animated case.

use aztek_abc::abc::{self, Archive, TimeSampling};
use aztek_abc::geom;

fn sample_archive() -> Archive {
    archive_with(true)
}

fn archive_with(animated: bool) -> Archive {
    let mut a = Archive::default();
    a.meta = "_ai_AlembicVersion=Alembic 1.8.0;_ai_Application=Aztek".into();
    a.samplings.push(TimeSampling::Uniform { start: 1.0 / 24.0, step: 1.0 / 24.0 });
    let quad = geom::polymesh(
        "quadShape",
        1,
        &[geom::MeshSample { points: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]] }, geom::MeshSample { points: if animated { vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]] } else { vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]] } }],
        &[4],
        &[0, 1, 2, 3],
        Some(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]),
    );
    let mut m = geom::identity();
    m[12] = 2.0;
    let mut m2 = m;
    m2[13] = 3.0;
    let xf = geom::xform("quad", 1, &[m, m2], vec![quad]);
    let hair = geom::curves("hair", 0, &[vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 2.0, 0.0]]], &[vec![0.1, 0.05, 0.1, 0.07, 0.01]], &[2, 3], false);
    a.top.children = vec![xf, hair];
    a
}

#[test]
fn aztek_reads_what_it_writes() {
    let a = sample_archive();
    let b = abc::read(abc::write(&a)).unwrap();
    assert_eq!(b.samplings, a.samplings);
    assert_eq!(b.top.children.len(), 2);
    let xf = &b.top.children[0];
    assert_eq!(geom::schema(xf).as_deref(), Some(geom::XFORM));
    let ms = geom::xform_matrices(xf);
    assert_eq!(ms.len(), 2);
    assert_eq!((ms[0][12], ms[1][13]), (2.0, 3.0));
    let mesh = geom::read_mesh(&xf.children[0]).unwrap();
    assert_eq!(mesh.indices, vec![0, 1, 2, 3]);
    assert_eq!(mesh.points.len(), 2);
    assert_eq!(mesh.points[1][2], [1.0, 1.0, 1.0]);
    assert_eq!(mesh.uvs.unwrap()[2], [1.0, 1.0]);
    let c = geom::read_curves(&b.top.children[1]).unwrap();
    assert_eq!(c.counts, vec![2, 3]);
    assert_eq!(c.widths[0][3], 0.07);
    assert!(!c.cubic);
    // Faces are stored clockwise.
    let raw = xf.children[0].props[0].child(".faceIndices").unwrap().samples()[0].clone();
    assert_eq!(geom::i32s(&raw), vec![3, 2, 1, 0]);
}

#[test]
fn ogawa_rs_reads_what_aztek_writes() {
    use ogawa_rs::*;
    let path = std::env::temp_dir().join(format!("aztek_abc_{}.abc", std::process::id()));
    std::fs::write(&path, abc::write(&archive_with(false))).unwrap();
    let file = std::fs::File::open(&path).unwrap();
    let mut reader = MemMappedReader::new(file).unwrap();
    let archive = ogawa_rs::Archive::new(&mut reader).unwrap();
    assert_eq!(archive.time_samplings.len(), 2);
    assert_eq!(archive.max_samples[1], 2);
    assert!(archive.root_header.meta_data.tokens.get("_ai_Application").is_some_and(|v| v == "Aztek"));
    let root = archive.load_root_object(&mut reader).unwrap();
    assert_eq!(root.child_count(), 2);
    let xf = root.load_child(0, &mut reader, &archive.indexed_meta_data, &archive.time_samplings).unwrap();
    assert_eq!(xf.header.name, "quad");
    match Schema::parse(&xf, &mut reader, &archive).unwrap() {
        Schema::Xform(x) => assert!(!x.is_constant_identity()),
        _ => panic!("not an xform"),
    }
    let mesh = xf.load_child(0, &mut reader, &archive.indexed_meta_data, &archive.time_samplings).unwrap();
    match Schema::parse(&mesh, &mut reader, &archive).unwrap() {
        Schema::PolyMesh(m) => {
            let p = m.load_vertices_sample(0, &mut reader).unwrap();
            assert_eq!(p[2], [1.0, 0.0, 1.0]);
            assert_eq!(m.load_faceindices_sample(0, &mut reader).unwrap(), vec![3, 2, 1, 0]);
            assert_eq!(m.load_facecounts_sample(0, &mut reader).unwrap(), vec![4]);
            let b = m.load_bounds_sample(0, &mut reader).unwrap();
            assert_eq!((b.min, b.max), ([0.0, 0.0, 0.0], [1.0, 0.0, 1.0]));
        }
        _ => panic!("not a mesh"),
    }
    let hair = root.load_child(1, &mut reader, &archive.indexed_meta_data, &archive.time_samplings).unwrap();
    match Schema::parse(&hair, &mut reader, &archive).unwrap() {
        Schema::Curves(c) => {
            assert_eq!(c.load_n_vertices_sample(0, &mut reader).unwrap(), vec![2, 3]);
            let (ty, _, basis) = c.load_curve_type_sample(0, &mut reader).unwrap();
            assert_eq!(ty, CurveType::Linear);
            assert_eq!(basis, BasisType::None);
            assert_eq!(c.load_positions_sample(0, &mut reader).unwrap()[4], [1.0, 2.0, 0.0]);
            assert!(c.has_width());
        }
        _ => panic!("not curves"),
    }
}
