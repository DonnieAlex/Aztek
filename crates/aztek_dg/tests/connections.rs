//! Connection rules, disconnect behaviours, locks, node deletion.

mod common;
use aztek_dg::attr::{self, numeric};
use aztek_dg::data::IntArrayData;
use aztek_dg::*;
use common::*;
use std::sync::Arc;

fn try_connect(g: &mut Graph, a: NodeId, sa: &str, b: NodeId, sb: &str, force: bool) -> Result<()> {
    let (x, y) = (p(g, a, sa), p(g, b, sb));
    g.connect(&x, &y, force)
}

/// A node with attributes for the kind rules.
fn kinds(g: &mut Graph) -> NodeId {
    let n = g.create_node("adder", "k").unwrap();
    for s in [
        attr::message("msg", "msg"),
        numeric("d3", "d3", NumericType::Double3),
        numeric("d2", "d2", NumericType::Double2),
        numeric("i", "i", NumericType::Int),
        numeric("flag", "fl", NumericType::Boolean),
        attr::matrix("mat", "mat", MatrixType::Double),
        attr::typed("tmat", "tmat", DataType::Matrix),
        attr::typed("str", "str", DataType::String),
        attr::typed("str2", "str2", DataType::String),
        attr::typed("ints", "ints", DataType::IntArray),
        attr::typed("any", "any", DataType::Any),
        attr::generic("gen", "gen").accept_numeric(NumericType::Double3),
        attr::compound("c3", "c3", vec![d("c3a", "c3a"), d("c3b", "c3b"), d("c3c", "c3c")]),
        attr::compound("cs", "cs", vec![d("csa", "csa"), attr::typed("csb", "csb", DataType::String)]),
        attr::compound("cs2", "cs2", vec![d("cs2a", "cs2a"), attr::typed("cs2b", "cs2b", DataType::String)]),
        d("arr", "arr").array(true),
        d("arr2", "arr2").array(true),
        d("hidden", "hid").readable(false),
        d("noconn", "nc").connectable(false),
    ] {
        g.add_attribute(n, s).unwrap();
    }
    n
}

#[test]
fn readable_writable_connectable() {
    let (mut g, _) = graph();
    let k = kinds(&mut g);
    let a = g.create_node("adder", "a").unwrap();
    assert!(matches!(try_connect(&mut g, k, "hidden", a, "a", false), Err(DgError::NotReadable(_))));
    assert!(matches!(try_connect(&mut g, a, "sum", a, "sum", false), Err(DgError::NotWritable(_))));
    assert!(matches!(try_connect(&mut g, a, "a", a, "sum", false), Err(DgError::NotWritable(_))), "outputs here are not writable");
    assert!(matches!(try_connect(&mut g, k, "noconn", a, "a", false), Err(DgError::NotConnectable(_))));
    assert!(matches!(try_connect(&mut g, a, "a", a, "a", false), Err(DgError::Invalid(_))));
}

#[test]
fn kinds_that_connect() {
    let (mut g, _) = graph();
    let k = kinds(&mut g);
    let k2 = kinds(&mut g);
    let ok = [
        ("msg", "msg"),
        ("d3", "d3"),
        ("d3", "c3"),
        ("c3", "d3"),
        ("i", "flag"),
        ("flag", "i"),
        ("mat", "tmat"),
        ("tmat", "mat"),
        ("str", "str2"),
        ("ints", "any"),
        ("any", "str"),
        ("d3", "gen"),
        ("cs", "cs2"),
        ("arr", "arr2"),
    ];
    for (s, t) in ok {
        try_connect(&mut g, k, s, k2, t, true).unwrap_or_else(|e| panic!("{s} -> {t}: {e}"));
    }
    let bad = [
        ("msg", "i"),
        ("i", "msg"),
        ("d3", "d2"),
        ("d2", "c3"),
        ("str", "ints"),
        ("i", "str"),
        ("mat", "d3"),
        ("i", "gen"),
        ("cs", "c3"),
        ("arr", "i"),
        ("i", "arr"),
    ];
    for (s, t) in bad {
        assert!(
            matches!(try_connect(&mut g, k, s, k2, t, true), Err(DgError::Incompatible(..))),
            "{s} -> {t} should not connect"
        );
    }
    assert!(try_connect(&mut g, k, "i", k2, "arr[3]", true).is_ok(), "a scalar connects to one element");
}

#[test]
fn values_convert_across_numeric_kinds() {
    let (mut g, _) = graph();
    let k = kinds(&mut g);
    let k2 = kinds(&mut g);
    let a = g.create_node("adder", "a").unwrap();
    connect(&mut g, a, "sum", k2, "i");
    connect(&mut g, a, "sum", k2, "flag");
    set(&mut g, a, "a", 2.75);
    assert_eq!(g.get(&p(&g, k2, "i")).unwrap(), Value::Int(2));
    assert_eq!(g.get(&p(&g, k2, "flag")).unwrap(), Value::Bool(true));
    connect(&mut g, k, "d3", k2, "c3");
    let d3 = p(&g, k, "d3");
    g.set(&d3, Value::Tuple(vec![1.0, 2.0, 3.0])).unwrap();
    assert_eq!(f(&mut g, k2, "c3b"), 2.0);
}

#[test]
fn one_source_per_destination() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    let c = g.create_node("adder", "c").unwrap();
    connect(&mut g, a, "sum", c, "a");
    assert!(matches!(try_connect(&mut g, a, "sum", c, "a", false), Err(DgError::AlreadyConnected(_))));
    assert!(matches!(try_connect(&mut g, b, "sum", c, "a", false), Err(DgError::AlreadyConnected(_))));
    try_connect(&mut g, b, "sum", c, "a", true).unwrap();
    assert_eq!(g.source(&p(&g, c, "a")), Some(p(&g, b, "sum")));
    connect(&mut g, a, "sum", c, "b");
    assert_eq!(g.destinations(&p(&g, a, "sum")), [p(&g, c, "b")], "a source feeds many");
}

#[test]
fn parents_and_children_share_one_incoming_connection() {
    let (mut g, _) = graph();
    let x = g.create_node("xform", "x").unwrap();
    let y = g.create_node("xform", "y").unwrap();
    let s = g.create_node("adder", "s").unwrap();
    connect(&mut g, x, "translate", y, "translate");
    assert!(matches!(try_connect(&mut g, s, "sum", y, "tx", false), Err(DgError::AlreadyConnected(_))));
    try_connect(&mut g, s, "sum", y, "tx", true).unwrap();
    assert!(g.source(&p(&g, y, "translate")).is_none(), "forcing the child broke the parent's connection");
}

#[test]
fn destinations_cannot_be_set() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let x = g.create_node("xform", "x").unwrap();
    let y = g.create_node("xform", "y").unwrap();
    connect(&mut g, a, "sum", x, "tx");
    connect(&mut g, a, "sum", y, "translate.translateY");
    assert!(matches!(g.set(&p(&g, x, "tx"), Value::Double(1.0)), Err(DgError::ConnectedDestination(_))));
    assert!(
        matches!(g.set(&p(&g, x, "translate"), Value::Tuple(vec![0.0; 3])), Err(DgError::ConnectedDestination(_))),
        "a parent with a connected child"
    );
    let z = g.create_node("xform", "z").unwrap();
    connect(&mut g, x, "translate", z, "translate");
    assert!(matches!(g.set(&p(&g, z, "tz"), Value::Double(1.0)), Err(DgError::ConnectedDestination(_))), "a child of a connected parent");
    set(&mut g, x, "ty", 1.0);
}

#[test]
fn disconnect_behaviours() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let k = g.create_node("adder", "k").unwrap();
    g.add_attribute(k, d("keep", "kp")).unwrap();
    g.add_attribute(k, d("reset", "rs").default_value(Value::Double(9.0)).unwrap().disconnect_behavior(DisconnectBehavior::Reset)).unwrap();
    g.add_attribute(k, d("list", "ls").array(true).disconnect_behavior(DisconnectBehavior::Delete)).unwrap();
    for t in ["keep", "reset", "list[4]"] {
        connect(&mut g, a, "sum", k, t);
    }
    set(&mut g, a, "a", 3.0);
    let (src, keep, reset, el) = (p(&g, a, "sum"), p(&g, k, "keep"), p(&g, k, "reset"), p(&g, k, "list[4]"));
    g.disconnect(&src, &keep).unwrap();
    g.disconnect(&src, &reset).unwrap();
    g.disconnect(&src, &el).unwrap();
    assert_eq!(f(&mut g, k, "keep"), 3.0, "nothing: the last value stays, pulled at disconnect");
    assert_eq!(f(&mut g, k, "reset"), 9.0);
    assert!(g.elements(&p(&g, k, "list")).unwrap().is_empty());
    assert!(matches!(g.disconnect(&src, &keep), Err(DgError::NotConnected(..))));
}

#[test]
fn deleting_a_node_breaks_its_connections() {
    let (mut g, c) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    connect(&mut g, a, "sum", b, "a");
    set(&mut g, a, "a", 4.0);
    let made = Counts::get(&c.made);
    assert_eq!(made, 2, "both ends hear of the connection");
    g.delete_node(a).unwrap();
    assert_eq!(Counts::get(&c.broken), 2);
    assert!(!g.exists(a));
    assert!(g.connections(b).is_empty());
    assert_eq!(f(&mut g, b, "sum"), 4.0);
    assert!(matches!(g.set(&Plug { node: a, path: p(&g, b, "a").path }, Value::Double(1.0)), Err(DgError::NoNode)));
    let n = g.create_node("adder", "n").unwrap();
    assert_ne!(n, a, "ids are not reused");
}

#[test]
fn locks() {
    let (mut g, _) = graph();
    let x = g.create_node("xform", "x").unwrap();
    let a = g.create_node("adder", "a").unwrap();
    let t = p(&g, x, "translate");
    g.set_locked(&t, true).unwrap();
    assert!(g.is_locked(&p(&g, x, "tx")), "children of a locked plug are locked");
    assert!(matches!(g.set(&p(&g, x, "tx"), Value::Double(1.0)), Err(DgError::Locked(_))));
    assert!(matches!(try_connect(&mut g, a, "sum", x, "ty", false), Err(DgError::Locked(_))));
    g.set_locked(&t, false).unwrap();
    g.set_locked(&p(&g, x, "tz"), true).unwrap();
    assert!(!g.is_locked(&t));
    assert!(matches!(g.set(&t, Value::Tuple(vec![0.0; 3])), Err(DgError::Locked(_))), "a locked child stops its parent changing");
    set(&mut g, x, "tx", 1.0);
    g.set_node_locked(a, true).unwrap();
    assert!(matches!(g.delete_node(a), Err(DgError::Locked(_))));
    assert!(matches!(g.rename(a, "b"), Err(DgError::Locked(_))));
    assert!(matches!(g.add_attribute(a, d("z", "z")), Err(DgError::Locked(_))));
    set(&mut g, a, "a", 1.0);
}

#[test]
fn next_available_and_connected_elements() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("blend", "b").unwrap();
    let arr = p(&g, b, "inputs");
    assert_eq!(g.next_available(&arr).unwrap(), 0);
    connect(&mut g, a, "sum", b, "inputs[0].value");
    connect(&mut g, a, "sum", b, "inputs[1].value");
    assert_eq!(g.next_available(&arr).unwrap(), 2);
    assert_eq!(g.elements(&arr).unwrap(), [0, 1]);
    let e = p(&g, b, "inputs[1]");
    assert!(g.remove_element(&e, false).is_err(), "connected elements need breaking");
    g.remove_element(&e, true).unwrap();
    assert_eq!(g.next_available(&arr).unwrap(), 1);
}

#[test]
fn typed_data_passes_through_connections() {
    let (mut g, _) = graph();
    let k = kinds(&mut g);
    let k2 = kinds(&mut g);
    connect(&mut g, k, "ints", k2, "any");
    let ints = p(&g, k, "ints");
    g.set(&ints, Value::Data(Arc::new(IntArrayData(vec![1, 2])))).unwrap();
    assert_eq!(g.get(&p(&g, k2, "any")).unwrap().as_data::<IntArrayData>().unwrap().0, vec![1, 2]);
}
