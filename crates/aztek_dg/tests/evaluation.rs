//! Push-dirty and pull evaluation.

mod common;
use aztek_dg::*;
use common::*;

#[test]
fn outputs_compute_lazily_and_once() {
    let (mut g, c) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let sum = p(&g, a, "sum");
    assert!(g.is_dirty(&sum), "outputs start dirty");
    set(&mut g, a, "a", 2.0);
    set(&mut g, a, "b", 3.0);
    assert_eq!(Counts::get(&c.adder), 0, "setting inputs computes nothing");
    assert_eq!(f(&mut g, a, "sum"), 5.0);
    assert_eq!(f(&mut g, a, "sum"), 5.0);
    assert_eq!(Counts::get(&c.adder), 1);
    assert!(!g.is_dirty(&sum));
    set(&mut g, a, "a", 4.0);
    assert!(g.is_dirty(&sum));
    assert_eq!(f(&mut g, a, "sum"), 7.0);
    assert_eq!(Counts::get(&c.adder), 2);
}

#[test]
fn dirty_pushes_through_connections_and_values_pull_back() {
    let (mut g, c) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    let e = g.create_node("adder", "c").unwrap();
    connect(&mut g, a, "sum", b, "a");
    connect(&mut g, b, "sum", e, "b");
    set(&mut g, a, "a", 1.0);
    set(&mut g, b, "b", 10.0);
    assert_eq!(f(&mut g, e, "sum"), 11.0);
    assert_eq!(Counts::get(&c.adder), 3);
    set(&mut g, a, "b", 5.0);
    assert!(g.is_dirty(&p(&g, e, "sum")));
    assert_eq!(f(&mut g, e, "sum"), 16.0);
    assert_eq!(f(&mut g, b, "a"), 6.0, "the destination holds the pulled value");
}

#[test]
fn dirty_propagation_stops_at_dirty_plugs() {
    let (mut g, c) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    connect(&mut g, a, "sum", b, "a");
    f(&mut g, b, "sum");
    let before = Counts::get(&c.hook);
    set(&mut g, a, "a", 1.0);
    let first = Counts::get(&c.hook) - before;
    set(&mut g, a, "a", 2.0);
    let second = Counts::get(&c.hook) - before - first;
    assert!(first > second, "the second change stops at a.sum, already dirty: {first} then {second}");
    assert_eq!(f(&mut g, b, "sum"), 2.0);
}

#[test]
fn numeric_compound_inputs_affect_through_children() {
    let (mut g, _) = graph();
    let x = g.create_node("xform", "x").unwrap();
    set(&mut g, x, "ty", 4.0);
    let m = p(&g, x, "matrix");
    assert_eq!(g.get(&m).unwrap().as_matrix().unwrap()[13], 4.0);
    set(&mut g, x, "tz", 2.0);
    assert!(g.is_dirty(&m), "a child dirties what its parent affects");
    assert_eq!(g.get(&m).unwrap().as_matrix().unwrap()[14], 2.0);
}

#[test]
fn compound_arrays_compute_over_their_elements() {
    let (mut g, _) = graph();
    let b = g.create_node("blend", "b").unwrap();
    set(&mut g, b, "inputs[0].value", 2.0);
    set(&mut g, b, "inputs[0].weight", 0.5);
    set(&mut g, b, "inputs[5].value", 4.0);
    set(&mut g, b, "inputs[5].weight", 2.0);
    assert_eq!(f(&mut g, b, "output"), 9.0);
    assert_eq!(g.elements(&p(&g, b, "inputs")).unwrap(), [0, 5]);
    let e = p(&g, b, "inputs[5]");
    g.remove_element(&e, false).unwrap();
    assert_eq!(g.elements(&p(&g, b, "inputs")).unwrap(), [0]);
    assert_eq!(f(&mut g, b, "output"), 1.0, "removing an element dirties what the array affects");
}

#[test]
fn array_elements_compute_one_at_a_time() {
    let (mut g, _) = graph();
    let n = g.create_node("doubler", "n").unwrap();
    set(&mut g, n, "in[3]", 1.5);
    set(&mut g, n, "in[7]", 4.0);
    assert_eq!(f(&mut g, n, "out[3]"), 3.0, "the element takes its index from the plug being computed");
    let out = p(&g, n, "out");
    let v = g.get(&out).unwrap();
    assert_eq!(v.as_array().unwrap(), &[(3, Value::Double(3.0)), (7, Value::Double(8.0))]);
}

#[test]
fn derived_types_hand_unknown_plugs_to_their_base() {
    let (mut g, _) = graph();
    let n = g.create_node("derived", "n").unwrap();
    set(&mut g, n, "x", 1.0);
    set(&mut g, n, "z", 2.0);
    assert_eq!(f(&mut g, n, "y"), 2.0);
    assert_eq!(f(&mut g, n, "w"), 20.0);
}

#[test]
fn uncached_outputs_compute_on_every_read() {
    let (mut g, c) = graph();
    let n = g.create_node("uncached", "n").unwrap();
    let m = g.create_node("adder", "m").unwrap();
    connect(&mut g, n, "out", m, "a");
    set(&mut g, n, "in", 3.0);
    assert_eq!(f(&mut g, n, "out"), 3.0);
    assert_eq!(f(&mut g, n, "out"), 3.0);
    assert_eq!(Counts::get(&c.uncached), 2);
    assert_eq!(f(&mut g, m, "sum"), 3.0);
    set(&mut g, n, "in", 5.0);
    assert_eq!(f(&mut g, m, "sum"), 5.0, "an uncached output still pushes dirty downstream");
}

#[test]
fn cycles_warn_and_end() {
    let (mut g, _) = graph();
    let a = g.create_node("adder", "a").unwrap();
    let b = g.create_node("adder", "b").unwrap();
    connect(&mut g, a, "sum", b, "a");
    connect(&mut g, b, "sum", a, "a");
    set(&mut g, a, "b", 1.0);
    f(&mut g, a, "sum");
    assert!(g.take_warnings().iter().any(|w| w.starts_with("cycle on")));
}

#[test]
fn failed_computes_warn_and_keep_the_datablock_value() {
    let (mut g, _) = graph();
    let n = g.create_node("failing", "n").unwrap();
    assert_eq!(f(&mut g, n, "out"), 0.0);
    let w = g.take_warnings();
    assert!(w.iter().any(|w| w.contains("broken on purpose")), "{w:?}");
}

#[test]
fn internal_attributes_go_through_the_node() {
    let (mut g, c) = graph();
    let n = g.create_node("internal", "n").unwrap();
    set(&mut g, n, "val", 3.0);
    assert_eq!(Counts::get(&c.internal_set), 1);
    assert_eq!(f(&mut g, n, "val"), 6.0);
}

#[test]
fn set_dependents_dirty_adds_plugs() {
    let (mut g, _) = graph();
    let n = g.create_node("hooked", "n").unwrap();
    assert_eq!(f(&mut g, n, "result"), 0.0);
    set(&mut g, n, "trigger", 2.0);
    assert!(g.is_dirty(&p(&g, n, "result")));
    assert_eq!(f(&mut g, n, "result"), 2.0);
    set(&mut g, n, "other", 1.0);
    assert_eq!(f(&mut g, n, "result"), 3.0);
}

#[test]
fn marking_dirty_forces_a_compute() {
    let (mut g, c) = graph();
    let a = g.create_node("adder", "a").unwrap();
    f(&mut g, a, "sum");
    let pl = p(&g, a, "sum");
    g.mark_dirty(&pl).unwrap();
    f(&mut g, a, "sum");
    assert_eq!(Counts::get(&c.adder), 2);
}

#[test]
fn whole_compounds_connect_and_their_children_follow() {
    let (mut g, _) = graph();
    let a = g.create_node("xform", "a").unwrap();
    let b = g.create_node("xform", "b").unwrap();
    connect(&mut g, a, "translate", b, "translate");
    set(&mut g, a, "tx", 5.0);
    assert_eq!(f(&mut g, b, "tx"), 5.0);
    assert_eq!(g.get(&p(&g, b, "matrix")).unwrap().as_matrix().unwrap()[12], 5.0);
    set(&mut g, a, "ty", 6.0);
    assert_eq!(g.get(&p(&g, b, "translate")).unwrap().as_tuple(), Some(vec![5.0, 6.0, 0.0]));
}

#[test]
fn children_connect_on_their_own() {
    let (mut g, _) = graph();
    let s = g.create_node("adder", "s").unwrap();
    let x = g.create_node("xform", "x").unwrap();
    connect(&mut g, s, "sum", x, "ty");
    set(&mut g, s, "a", 7.0);
    assert_eq!(g.get(&p(&g, x, "translate")).unwrap().as_tuple(), Some(vec![0.0, 7.0, 0.0]));
    assert_eq!(g.get(&p(&g, x, "matrix")).unwrap().as_matrix().unwrap()[13], 7.0);
}

#[test]
fn whole_arrays_connect() {
    let (mut g, _) = graph();
    let a = g.create_node("doubler", "a").unwrap();
    let b = g.create_node("doubler", "b").unwrap();
    connect(&mut g, a, "out", b, "in");
    set(&mut g, a, "in[2]", 1.0);
    set(&mut g, a, "in[4]", 2.0);
    let out = p(&g, b, "out");
    assert_eq!(g.get(&out).unwrap().as_array().unwrap(), &[(2, Value::Double(4.0)), (4, Value::Double(8.0))]);
}

#[test]
fn elements_connect_into_compound_arrays() {
    let (mut g, _) = graph();
    let s = g.create_node("adder", "s").unwrap();
    let b = g.create_node("blend", "b").unwrap();
    connect(&mut g, s, "sum", b, "inputs[2].value");
    set(&mut g, b, "inputs[2].weight", 3.0);
    set(&mut g, s, "a", 2.0);
    assert_eq!(f(&mut g, b, "output"), 6.0);
    assert_eq!(g.elements(&p(&g, b, "inputs")).unwrap(), [2]);
    set(&mut g, s, "a", 1.0);
    assert_eq!(f(&mut g, b, "output"), 3.0);
}
