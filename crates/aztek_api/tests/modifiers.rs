//! Modifiers: do, undo, redo, and the undo queue.

mod common;
use aztek_api::{dag, node, DagModifier, DgModifier, UndoStep};
use aztek_dg::*;
use aztek_undo::UndoStack;
use common::*;

#[test]
fn created_nodes_have_ids_before_do_it() {
    let mut g = graph();
    let mut m = DgModifier::new();
    let a = m.create_node(&mut g, "adder", None).unwrap();
    assert!(!g.exists(a) && g.is_alive(a), "alive, not yet valid");
    m.do_it(&mut g).unwrap();
    assert_eq!(node::name(&g, a).unwrap(), "adder1");
    m.undo_it(&mut g).unwrap();
    assert!(!g.exists(a) && g.is_alive(a));
    m.do_it(&mut g).unwrap();
    assert!(g.exists(a), "redo brings back the same node");
    drop(m);
    assert!(g.exists(a));
}

#[test]
fn a_network_undoes_and_redoes() {
    let mut g = graph();
    let mut m = DgModifier::new();
    let a = m.create_node(&mut g, "adder", Some("a")).unwrap();
    let b = m.create_node(&mut g, "adder", Some("b")).unwrap();
    m.do_it(&mut g).unwrap();
    m.new_plug_value(&p(&g, a, "a"), Value::Double(3.0));
    m.connect(&p(&g, a, "sum"), &p(&g, b, "a"));
    m.new_plug_value(&p(&g, b, "b"), Value::Double(4.0));
    m.do_it(&mut g).unwrap();
    assert_eq!(f(&mut g, b, "sum"), 7.0);
    m.undo_it(&mut g).unwrap();
    assert!(!g.exists(a) && !g.exists(b));
    m.do_it(&mut g).unwrap();
    assert_eq!(f(&mut g, b, "sum"), 7.0);
    assert_eq!(g.source(&p(&g, b, "a")), Some(p(&g, a, "sum")));
}

#[test]
fn undoing_a_connection_restores_the_old_value() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    g.set(&p(&g, b, "a"), Value::Double(9.0)).unwrap();
    g.set(&p(&g, a, "a"), Value::Double(1.0)).unwrap();
    let mut m = DgModifier::new();
    m.connect(&p(&g, a, "sum"), &p(&g, b, "a"));
    m.connect(&p(&g, a, "sum"), &p(&g, b, "list[5]"));
    m.do_it(&mut g).unwrap();
    assert_eq!(f(&mut g, b, "a"), 1.0);
    assert_eq!(g.elements(&p(&g, b, "list")).unwrap(), [5]);
    m.undo_it(&mut g).unwrap();
    assert_eq!(f(&mut g, b, "a"), 9.0);
    assert!(g.elements(&p(&g, b, "list")).unwrap().is_empty(), "an element the connection made goes");
}

#[test]
fn deleting_restores_connections_values_and_hierarchy() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", Some("a")).unwrap();
    let b = node::create(&mut g, "adder", Some("b")).unwrap();
    g.set(&p(&g, a, "a"), Value::Double(2.0)).unwrap();
    g.set(&p(&g, a, "items[1].weight"), Value::Double(5.0)).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "a"), false).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "list[2]"), false).unwrap();
    let top = node::create(&mut g, "place", Some("top")).unwrap();
    let mid = node::create(&mut g, "place", Some("mid")).unwrap();
    let leaf = node::create(&mut g, "place", Some("leaf")).unwrap();
    let other = node::create(&mut g, "place", Some("other")).unwrap();
    let shared = node::create(&mut g, "place", Some("shared")).unwrap();
    dag::add_child(&mut g, Some(top), mid, None, false).unwrap();
    dag::add_child(&mut g, Some(mid), leaf, None, false).unwrap();
    dag::add_child(&mut g, Some(top), shared, Some(0), false).unwrap();
    dag::add_child(&mut g, Some(other), shared, None, true).unwrap();
    let mut m = DgModifier::new();
    m.delete_node(a);
    m.delete_node(top);
    m.do_it(&mut g).unwrap();
    assert!(!g.exists(a) && !g.exists(top) && !g.exists(mid) && !g.exists(leaf));
    assert!(g.is_alive(a), "held by the modifier");
    assert_eq!(g.dag_parents(shared), [Some(other)]);
    assert!(g.elements(&p(&g, b, "list")).unwrap().is_empty(), "delete behaviour on the element");
    m.undo_it(&mut g).unwrap();
    assert!(g.exists(a) && g.exists(mid) && g.exists(leaf));
    assert_eq!(f(&mut g, b, "a"), 2.0);
    assert_eq!(f(&mut g, a, "items[1].weight"), 5.0);
    assert_eq!(g.elements(&p(&g, b, "list")).unwrap(), [2]);
    assert_eq!(g.dag_children(Some(top)), [shared, mid], "child order comes back");
    assert_eq!(g.dag_parents(shared), [Some(top), Some(other)]);
    assert_eq!(dag::full_path_name(&g, leaf).unwrap(), "|top|mid|leaf");
    assert_eq!(g.dag_children(None), [top, other]);
    m.do_it(&mut g).unwrap();
    assert!(!g.exists(top));
    drop(m);
    assert!(!g.is_alive(a), "the deleted node ends with the step that held it");
}

#[test]
fn locked_nodes_are_not_deleted() {
    let mut g = graph();
    let top = node::create(&mut g, "place", None).unwrap();
    let leaf = node::create(&mut g, "place", None).unwrap();
    dag::add_child(&mut g, Some(top), leaf, None, false).unwrap();
    g.set_node_locked(leaf, true).unwrap();
    let mut m = DgModifier::new();
    m.delete_node(top);
    assert!(matches!(m.do_it(&mut g), Err(DgError::Locked(_))));
    assert!(g.exists(top) && g.exists(leaf), "nothing was touched");
}

#[test]
fn attributes_come_back_with_their_values_and_ids() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    let mut m = DgModifier::new();
    let ex = m.add_attribute(&mut g, a, d("extra", "ex")).unwrap();
    m.do_it(&mut g).unwrap();
    assert_eq!(node::attribute_named(&g, a, "extra").unwrap(), ex);
    g.set(&p(&g, a, "extra"), Value::Double(6.0)).unwrap();
    g.connect(&p(&g, a, "extra"), &p(&g, b, "b"), false).unwrap();
    let mut r = DgModifier::new();
    r.remove_attribute(a, ex);
    r.do_it(&mut g).unwrap();
    assert!(!node::has_attribute(&g, a, "extra"));
    r.undo_it(&mut g).unwrap();
    assert_eq!(node::attribute_named(&g, a, "extra").unwrap(), ex);
    assert_eq!(f(&mut g, a, "extra"), 6.0);
    assert_eq!(g.source(&p(&g, b, "b")), Some(p(&g, a, "extra")));
    let mut n = DgModifier::new();
    n.rename_attribute(a, ex, "xx", "renamed");
    n.do_it(&mut g).unwrap();
    assert!(node::has_attribute(&g, a, "renamed") && !node::has_attribute(&g, a, "extra"));
    n.undo_it(&mut g).unwrap();
    assert!(node::has_attribute(&g, a, "extra"));
}

#[test]
fn renames_locks_and_elements_undo() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", Some("a")).unwrap();
    let _taken = node::create(&mut g, "adder", Some("b")).unwrap();
    g.set(&p(&g, a, "items[7].value"), Value::Double(3.0)).unwrap();
    let mut m = DgModifier::new();
    m.rename_node(a, "b");
    m.set_plug_lock_state(&p(&g, a, "a"), true);
    m.set_node_lock_state(a, true);
    m.remove_multi_instance(&p(&g, a, "items[7]"), false);
    m.new_plug_value(&p(&g, a, "items[9].weight"), Value::Double(1.0));
    m.do_it(&mut g).unwrap();
    assert_eq!(node::name(&g, a).unwrap(), "b1");
    assert!(g.is_locked(&p(&g, a, "a")) && g.is_node_locked(a).unwrap());
    assert_eq!(g.elements(&p(&g, a, "items")).unwrap(), [9]);
    m.undo_it(&mut g).unwrap();
    assert_eq!(node::name(&g, a).unwrap(), "a");
    assert!(!g.is_locked(&p(&g, a, "a")) && !g.is_node_locked(a).unwrap());
    assert_eq!(g.elements(&p(&g, a, "items")).unwrap(), [7]);
    assert_eq!(f(&mut g, a, "items[7].value"), 3.0);
}

#[test]
fn connected_elements_need_breaking() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "items[0].value"), false).unwrap();
    let mut m = DgModifier::new();
    m.remove_multi_instance(&p(&g, b, "items[0]"), false);
    assert!(m.do_it(&mut g).is_err());
    let mut m = DgModifier::new();
    m.remove_multi_instance(&p(&g, b, "items[0]"), true);
    m.do_it(&mut g).unwrap();
    assert!(g.elements(&p(&g, b, "items")).unwrap().is_empty());
    m.undo_it(&mut g).unwrap();
    assert_eq!(g.source(&p(&g, b, "items[0].value")), Some(p(&g, a, "sum")));
}

#[test]
fn dag_modifier_creates_under_parents_and_reparents() {
    let mut g = graph();
    let mut m = DagModifier::new();
    let top = m.create_dag_node(&mut g, "place", Some("top"), None).unwrap();
    let kid = m.create_dag_node(&mut g, "place", Some("kid"), Some(top)).unwrap();
    let other = m.create_dag_node(&mut g, "place", Some("other"), None).unwrap();
    assert!(m.create_dag_node(&mut g, "adder", None, None).is_err());
    m.do_it(&mut g).unwrap();
    assert_eq!(dag::full_path_name(&g, kid).unwrap(), "|top|kid");
    m.reparent_node(kid, Some(other));
    m.add_child(Some(top), kid, Some(0), true);
    m.do_it(&mut g).unwrap();
    assert_eq!(g.dag_parents(kid), [Some(other), Some(top)]);
    m.reparent_node(top, Some(kid));
    assert!(m.do_it(&mut g).is_err(), "no cycles");
    m.undo_it(&mut g).unwrap();
    assert!(!g.exists(top));
}

#[test]
fn modifier_steps_on_the_undo_queue() {
    let mut g = graph();
    let mut stack: UndoStack<Graph> = UndoStack::new();
    let mut m = DgModifier::new();
    let a = m.create_node(&mut g, "adder", None).unwrap();
    m.do_it(&mut g).unwrap();
    stack.record(UndoStep(m));
    let mut m = DgModifier::new();
    m.new_plug_value(&p(&g, a, "a"), Value::Double(5.0));
    m.do_it(&mut g).unwrap();
    stack.record(UndoStep(m));
    assert_eq!(f(&mut g, a, "sum"), 5.0);
    assert!(stack.undo(&mut g));
    assert_eq!(f(&mut g, a, "sum"), 0.0);
    assert!(stack.undo(&mut g));
    assert!(!g.exists(a));
    assert!(stack.redo(&mut g));
    assert!(stack.redo(&mut g));
    assert_eq!(f(&mut g, a, "sum"), 5.0);
    assert!(g.take_warnings().is_empty());
}
