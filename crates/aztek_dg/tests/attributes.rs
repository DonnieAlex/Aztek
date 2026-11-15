//! Every attribute kind: defaults, conversion, storage, and the reference's property defaults.

mod common;
use aztek_dg::attr::{self, numeric};
use aztek_dg::data::{DoubleArrayData, IntArrayData, MatrixData, StringData, IDENTITY};
use aztek_dg::*;
use common::*;
use std::sync::Arc;

#[test]
fn property_defaults_are_the_references() {
    let p = Props::default();
    assert!(p.readable && p.writable && p.connectable && p.storable && p.cached && p.index_matters);
    assert!(!p.array && !p.keyable && !p.channel_box && !p.hidden && !p.used_as_color && !p.indeterminant);
    assert!(!p.render_source && !p.world_space && !p.affects_world_space && !p.internal);
    assert!(!p.uses_array_data_builder);
    assert_eq!(p.disconnect_behavior, DisconnectBehavior::Nothing);
}

/// A node with one attribute of every kind, and the graph it lives in.
fn kinds() -> (Graph, NodeId) {
    use NumericType::*;
    let mut r = Registry::new();
    r.register_class("kinds", 100, None, None, |i| {
        for (n, t) in [
            ("bool", Boolean),
            ("byte", Byte),
            ("char", Char),
            ("short", Short),
            ("short2", Short2),
            ("short3", Short3),
            ("int", Int),
            ("int2", Int2),
            ("int3", Int3),
            ("int64", Int64),
            ("float", Float),
            ("float2", Float2),
            ("float3", Float3),
            ("double", Double),
            ("double2", Double2),
            ("double3", Double3),
            ("double4", Double4),
            ("addr", Addr),
        ] {
            i.add_attribute(numeric(n, &format!("{n}_s"), t))?;
        }
        i.add_attribute(numeric("withDefault", "wd", Double).default_value(Value::Double(2.5))?)?;
        i.add_attribute(attr::unit("angle", "ang", UnitType::Angle))?;
        i.add_attribute(attr::unit("distance", "dist", UnitType::Distance).default_value(Value::Double(4.0))?)?;
        i.add_attribute(attr::unit("time", "tm", UnitType::Time))?;
        i.add_attribute(attr::enumeration("mode", "md", 1).field("off", 0).field("on", 1).field("auto", 2))?;
        i.add_attribute(attr::typed("text", "txt", DataType::String))?;
        i.add_attribute(attr::typed("numbers", "nums", DataType::DoubleArray))?;
        i.add_attribute(attr::typed("anything", "any", DataType::Any))?;
        i.add_attribute(attr::typed("typedMatrix", "tmat", DataType::Matrix))?;
        i.add_attribute(attr::matrix("dmatrix", "dm", MatrixType::Double))?;
        i.add_attribute(attr::matrix("fmatrix", "fm", MatrixType::Float))?;
        i.add_attribute(attr::message("message", "msg"))?;
        i.add_attribute(attr::compound("pair", "pr", vec![d("first", "fi"), attr::typed("label", "lb", DataType::String)]))?;
        i.add_attribute(
            attr::generic("generic", "gen")
                .accept_numeric(Double)
                .accept_numeric(Double3)
                .accept_data(DataType::String),
        )?;
        let light = attr::light_data(
            "lightData",
            "ltd",
            [
                numeric("lightDirection", "ld", Float3),
                numeric("lightIntensity", "li", Float3),
                numeric("lightAmbient", "la", Boolean),
                numeric("lightDiffuse", "ldf", Boolean),
                numeric("lightSpecular", "ls", Boolean),
                numeric("lightShadowFraction", "lsf", Float),
                numeric("preShadowIntensity", "psi", Float),
                numeric("lightBlindData", "lbd", Addr),
            ],
        )?;
        i.add_attribute(light).map(|_| ())
    })
    .unwrap();
    let mut g = Graph::with_registry(r);
    let n = g.create_node("kinds", "k").unwrap();
    (g, n)
}

fn get(g: &mut Graph, n: NodeId, s: &str) -> Value {
    let pl = p(g, n, s);
    g.get(&pl).unwrap()
}

#[test]
fn numeric_defaults_are_zero_and_typed_by_kind() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "bool"), Value::Bool(false));
    assert_eq!(get(&mut g, n, "int"), Value::Int(0));
    assert_eq!(get(&mut g, n, "double"), Value::Double(0.0));
    assert_eq!(get(&mut g, n, "double3"), Value::Tuple(vec![0.0; 3]));
    assert_eq!(get(&mut g, n, "double4"), Value::Tuple(vec![0.0; 4]));
    assert_eq!(get(&mut g, n, "short2"), Value::Tuple(vec![0.0; 2]));
    assert_eq!(get(&mut g, n, "withDefault"), Value::Double(2.5));
}

#[test]
fn integers_wrap_to_their_width() {
    let (mut g, n) = kinds();
    set(&mut g, n, "byte", 300);
    assert_eq!(get(&mut g, n, "byte"), Value::Int(44));
    set(&mut g, n, "char", 200);
    assert_eq!(get(&mut g, n, "char"), Value::Int(-56));
    set(&mut g, n, "short", 70000);
    assert_eq!(get(&mut g, n, "short"), Value::Int(4464));
    set(&mut g, n, "int", 3_000_000_000i64);
    assert_eq!(get(&mut g, n, "int"), Value::Int(-1294967296));
    set(&mut g, n, "int64", 3_000_000_000i64);
    assert_eq!(get(&mut g, n, "int64"), Value::Int(3_000_000_000));
    set(&mut g, n, "int", 2.9);
    assert_eq!(get(&mut g, n, "int"), Value::Int(2), "doubles truncate");
    set(&mut g, n, "bool", 3);
    assert_eq!(get(&mut g, n, "bool"), Value::Bool(true));
    let pl = p(&g, n, "short3");
    g.set(&pl, Value::Tuple(vec![1.7, -2.2, 70000.0])).unwrap();
    assert_eq!(get(&mut g, n, "short3"), Value::Tuple(vec![1.0, -2.0, 4464.0]));
}

#[test]
fn floats_keep_single_precision() {
    let (mut g, n) = kinds();
    set(&mut g, n, "float", 0.1);
    assert_eq!(get(&mut g, n, "float"), Value::Double(0.1f32 as f64));
    set(&mut g, n, "double", 0.1);
    assert_eq!(get(&mut g, n, "double"), Value::Double(0.1));
    let pl = p(&g, n, "float2");
    g.set(&pl, Value::Tuple(vec![0.1, 0.2])).unwrap();
    assert_eq!(get(&mut g, n, "float2"), Value::Tuple(vec![0.1f32 as f64, 0.2f32 as f64]));
}

#[test]
fn tuples_need_their_arity() {
    let (mut g, n) = kinds();
    let pl = p(&g, n, "double3");
    assert!(matches!(g.set(&pl, Value::Tuple(vec![1.0, 2.0])), Err(DgError::WrongValue(_))));
    g.set(&pl, Value::Tuple(vec![1.0, 2.0, 3.0])).unwrap();
    assert_eq!(get(&mut g, n, "double3"), Value::Tuple(vec![1.0, 2.0, 3.0]));
}

#[test]
fn units_and_enums() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "distance"), Value::Double(4.0));
    set(&mut g, n, "angle", std::f64::consts::PI);
    assert_eq!(get(&mut g, n, "angle"), Value::Double(std::f64::consts::PI));
    set(&mut g, n, "time", 1.5);
    assert_eq!(get(&mut g, n, "time"), Value::Double(1.5));
    assert_eq!(get(&mut g, n, "mode"), Value::Int(1));
    set(&mut g, n, "mode", 2);
    assert_eq!(get(&mut g, n, "mode"), Value::Int(2));
    let a = g.find_attr(n, "mode").unwrap();
    let AttrKind::Enum { fields, .. } = &g.attr(a).kind else { panic!() };
    assert_eq!(fields.iter().map(|f| f.0.as_str()).collect::<Vec<_>>(), ["off", "on", "auto"]);
}

#[test]
fn typed_attributes_take_their_data_type_only() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "text"), Value::None);
    set(&mut g, n, "text", "hello");
    assert_eq!(get(&mut g, n, "text").as_str(), Some("hello"));
    let nums = p(&g, n, "numbers");
    assert!(g.set(&nums, Value::string("no")).is_err());
    g.set(&nums, Value::data(DoubleArrayData(vec![1.0, 2.0]))).unwrap();
    assert_eq!(get(&mut g, n, "numbers").as_data::<DoubleArrayData>().unwrap().0, vec![1.0, 2.0]);
    let any = p(&g, n, "anything");
    g.set(&any, Value::data(IntArrayData(vec![3]))).unwrap();
    assert_eq!(get(&mut g, n, "anything").as_data::<IntArrayData>().unwrap().0, vec![3]);
    let dbl = p(&g, n, "double");
    assert!(g.set(&dbl, Value::string("x")).is_err());
}

#[test]
fn matrices() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "dmatrix").as_matrix(), Some(IDENTITY));
    let mut m = IDENTITY;
    m[12] = 0.1;
    set(&mut g, n, "dmatrix", Value::matrix(m));
    assert_eq!(get(&mut g, n, "dmatrix").as_matrix().unwrap()[12], 0.1);
    set(&mut g, n, "fmatrix", Value::matrix(m));
    assert_eq!(get(&mut g, n, "fmatrix").as_matrix().unwrap()[12], 0.1f32 as f64);
    set(&mut g, n, "typedMatrix", Value::matrix(m));
    assert!(get(&mut g, n, "typedMatrix").as_data::<MatrixData>().is_some());
    assert_eq!(get(&mut g, n, "typedMatrix").as_matrix().unwrap()[12], 0.1);
}

#[test]
fn messages_hold_no_value() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "message"), Value::None);
    let pl = p(&g, n, "message");
    assert!(g.set(&pl, Value::Double(1.0)).is_err());
}

#[test]
fn compounds_read_and_write_through_children() {
    let (mut g, n) = kinds();
    assert_eq!(get(&mut g, n, "pair"), Value::Compound(vec![Value::Double(0.0), Value::None]));
    set(&mut g, n, "first", 3.0);
    set(&mut g, n, "pair.label", "x");
    let v = get(&mut g, n, "pair");
    assert_eq!(v.as_compound().unwrap()[0], Value::Double(3.0));
    assert_eq!(v.as_compound().unwrap()[1].as_str(), Some("x"));
    let pl = p(&g, n, "pair");
    g.set(&pl, Value::Compound(vec![Value::Int(7), Value::string("y")])).unwrap();
    assert_eq!(get(&mut g, n, "first"), Value::Double(7.0));
    assert!(g.set(&pl, Value::Compound(vec![Value::Int(7)])).is_err());
}

#[test]
fn numeric_compounds_read_as_tuples_or_children() {
    let (mut g, _) = graph_with_xform();
    let x = g.nodes()[0];
    let t = p(&g, x, "translate");
    g.set(&t, Value::Tuple(vec![1.0, 2.0, 3.0])).unwrap();
    assert_eq!(f(&mut g, x, "ty"), 2.0);
    set(&mut g, x, "translateZ", 9.0);
    assert_eq!(g.get(&t).unwrap().as_tuple(), Some(vec![1.0, 2.0, 9.0]));
    let a = g.find_attr(x, "translate").unwrap();
    assert!(matches!(g.attr(a).kind, AttrKind::Compound { numeric: Some(NumericType::Double3), .. }));
}

fn graph_with_xform() -> (Graph, NodeId) {
    let (mut g, _) = graph();
    let x = g.create_node("xform", "x").unwrap();
    (g, x)
}

#[test]
fn numeric_compounds_need_matching_numeric_children() {
    assert!(attr::numeric_compound("bad", "bad", vec![d("a", "a"), attr::typed("b", "b", DataType::String)]).is_err());
    assert!(attr::numeric_compound(
        "bad",
        "bad",
        vec![d("a", "a"), numeric("b", "b", NumericType::Int)]
    )
    .is_err());
    assert!(attr::numeric_compound("one", "one", vec![d("a", "a")]).is_err());
    assert!(attr::numeric_compound("four", "four", (0..4).map(|i| d(&format!("c{i}"), &format!("c{i}"))).collect()).is_err());
    assert!(attr::numeric_compound("two", "two", vec![d("a", "a"), d("b", "b")]).is_ok());
}

#[test]
fn generic_attributes_take_listed_types() {
    let (mut g, n) = kinds();
    let pl = p(&g, n, "generic");
    g.set(&pl, Value::Double(1.5)).unwrap();
    assert_eq!(g.get(&pl).unwrap(), Value::Double(1.5));
    g.set(&pl, Value::Tuple(vec![1.0, 2.0, 3.0])).unwrap();
    g.set(&pl, Value::string("s")).unwrap();
    assert!(g.set(&pl, Value::Tuple(vec![1.0, 2.0])).is_err());
    assert!(g.set(&pl, Value::data(IntArrayData(vec![1]))).is_err());
}

#[test]
fn light_data_is_a_compound_of_the_references_children() {
    let (mut g, n) = kinds();
    let v = get(&mut g, n, "lightData");
    assert_eq!(v.as_compound().unwrap().len(), 8);
    let bad = attr::light_data(
        "l",
        "l",
        [d("a", "a"), d("b", "b"), d("c", "c"), d("d", "d"), d("e", "e"), d("f", "f"), d("g", "g"), d("h", "h")],
    );
    assert!(bad.is_err());
}

#[test]
fn defaults_must_fit_their_kind() {
    assert!(attr::typed("t", "t", DataType::String).default_value(Value::Double(1.0)).is_err());
    let s = attr::typed("t", "t", DataType::String).default_value(Value::data(StringData("d".into()))).unwrap();
    let mut r = Registry::new();
    r.register_class("t", 1, None, None, |i| i.add_attribute(s).map(|_| ())).unwrap();
    let mut g = Graph::with_registry(r);
    let n = g.create_node("t", "t").unwrap();
    assert_eq!(get(&mut g, n, "t").as_str(), Some("d"));
}

#[test]
fn names_are_unique_through_the_inheritance_chain() {
    let mut r = Registry::new();
    r.register_class("a", 1, None, None, |i| i.add_attribute(d("x", "x")).map(|_| ())).unwrap();
    let e = r.register_class("b", 2, Some("a"), None, |i| i.add_attribute(d("y", "x")).map(|_| ()));
    assert!(matches!(e, Err(DgError::NameClash(_))));
    assert!(r.class_by_name("b").is_none(), "a failed registration leaves no type behind");
    let e = r.register_class("c", 3, None, None, |i| {
        i.add_attribute(attr::compound("p", "p", vec![d("q", "q"), d("q", "q2")])).map(|_| ())
    });
    assert!(matches!(e, Err(DgError::NameClash(_))));
    assert!(matches!(r.register_class("a", 9, None, None, |_| Ok(())), Err(DgError::NameClash(_))));
}

#[test]
fn plugs_parse_by_long_short_and_child_names() {
    let (g, _) = graph();
    let mut g = g;
    let b = g.create_node("blend", "b").unwrap();
    let x = g.create_node("xform", "x").unwrap();
    assert_eq!(p(&g, x, "tx"), p(&g, x, "translate.translateX"));
    assert_eq!(p(&g, x, "tx"), p(&g, x, "t.tx"));
    let e = p(&g, b, "inputs[3].weight");
    assert_eq!(e, p(&g, b, "i[3].w"));
    assert_eq!(g.plug_name(&e), "b.inputs[3].weight");
    assert_eq!(g.plug_name(&p(&g, x, "tx")), "x.translateX");
    assert!(matches!(g.plug(b, "inputs.weight"), Err(DgError::BadPlug(_))), "a child of an array needs an index");
    assert!(matches!(g.plug(x, "tx[2]"), Err(DgError::BadPlug(_))));
    assert!(matches!(g.plug(x, "nothing"), Err(DgError::NoAttribute(_))));
}

#[test]
fn attributes_list_base_type_first_then_dynamic() {
    let (mut g, _) = graph();
    let n = g.create_node("derived", "n").unwrap();
    g.add_attribute(n, d("extra", "ex")).unwrap();
    let names: Vec<String> = g.attributes(n).unwrap().iter().map(|a| g.attr(*a).name.clone()).collect();
    assert_eq!(names, ["x", "y", "z", "w", "extra"]);
}

#[test]
fn dynamic_attributes_belong_to_one_node() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    let extra = g.add_attribute(a, d("extra", "ex").keyable(true)).unwrap();
    assert!(g.plug(b, "extra").is_err());
    assert!(matches!(g.add_attribute(a, d("sum", "zz")), Err(DgError::NameClash(_))));
    assert!(matches!(g.add_attribute(a, d("other", "ex")), Err(DgError::NameClash(_))));
    set(&mut g, a, "extra", 4.0);
    connect(&mut g, a, "extra", b, "a");
    assert_eq!(f(&mut g, b, "sum"), 4.0);
    g.remove_attribute(a, extra).unwrap();
    assert!(g.plug(a, "extra").is_err());
    assert!(g.connections(b).is_empty());
    assert_eq!(f(&mut g, b, "a"), 4.0, "the destination keeps its value");
    let s = g.find_attr(a, "sum").unwrap();
    assert!(g.remove_attribute(a, s).is_err(), "only dynamic attributes can be removed");
}

#[test]
fn dynamic_compound_arrays() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    g.add_attribute(a, attr::compound("slots", "sl", vec![d("amount", "am"), attr::typed("tag", "tg", DataType::String)]).array(true))
        .unwrap();
    set(&mut g, a, "slots[4].amount", 2.0);
    set(&mut g, a, "slots[1].tag", "t");
    let pl = p(&g, a, "slots");
    assert_eq!(g.elements(&pl).unwrap(), [1, 4]);
    let v = g.get(&pl).unwrap();
    assert_eq!(v.as_array().unwrap()[1].1.as_compound().unwrap()[0], Value::Double(2.0));
}

#[test]
fn extension_attributes_reach_derived_types_and_existing_nodes() {
    let (mut g, _) = graph();
    let n = g.create_node("derived", "n").unwrap();
    g.registry.add_extension("base", d("ext", "ext")).unwrap();
    let a = g.find_attr(n, "ext").unwrap();
    assert!(g.attr(a).extension);
    set(&mut g, n, "ext", 3.0);
    assert_eq!(f(&mut g, n, "ext"), 3.0);
    assert!(matches!(g.registry.add_extension("base", d("other", "w")), Err(DgError::NameClash(_))), "w is on a derived type");
}

#[test]
fn abstract_types_are_not_created() {
    let (mut g, _) = graph();
    assert!(matches!(g.create_node("shape", "s"), Err(DgError::Abstract(_))));
    assert!(matches!(g.create_node("nothing", "s"), Err(DgError::NoClass(_))));
}

#[test]
fn types_know_their_lineage() {
    let (g, _) = graph();
    let c = g.registry.class_by_name("derived").unwrap();
    assert!(g.registry.is_a(c, "base"));
    assert!(!g.registry.is_a(c, "adder"));
    assert_eq!(g.registry.class_by_type_id(6), Some(c));
    let _ = Arc::new(0);
}
