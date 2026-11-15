//! Naming, and the node, DAG and plug function sets.

mod common;
use aztek_api::plug::{FreeToChange, PartialName};
use aztek_api::{dag, node, plug};
use aztek_dg::*;
use common::*;

#[test]
fn default_names_count_up_from_one() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    assert_eq!(node::name(&g, a).unwrap(), "adder1");
    assert_eq!(node::name(&g, b).unwrap(), "adder2");
}

#[test]
fn clashing_names_raise_their_numeric_suffix() {
    let mut g = graph();
    let names: Vec<String> = ["foo", "foo", "foo5", "foo5", "x#y", "x#y"]
        .into_iter()
        .map(|n| {
            let id = node::create(&mut g, "adder", Some(n)).unwrap();
            node::name(&g, id).unwrap()
        })
        .collect();
    assert_eq!(names, ["foo", "foo1", "foo5", "foo6", "x1y", "x2y"]);
    assert!(node::create(&mut g, "adder", Some("1bad")).is_err());
    assert!(node::create(&mut g, "adder", Some("bad name")).is_err());
    let a = g.nodes_named("foo")[0];
    assert_eq!(node::set_name(&mut g, a, "foo6").unwrap(), "foo7");
    assert_eq!(node::set_name(&mut g, a, "foo7").unwrap(), "foo7", "its own name does not clash");
    assert!(node::has_unique_name(&g, a).unwrap());
}

#[test]
fn dag_names_clash_only_between_siblings() {
    let mut g = graph();
    let a = node::create(&mut g, "place", Some("a")).unwrap();
    let b = node::create(&mut g, "place", Some("b")).unwrap();
    let c1 = node::create(&mut g, "place", Some("c")).unwrap();
    dag::add_child(&mut g, Some(a), c1, None, false).unwrap();
    let c2 = node::create(&mut g, "place", Some("c")).unwrap();
    assert_eq!(node::name(&g, c2).unwrap(), "c", "c under a is not a sibling");
    dag::add_child(&mut g, Some(b), c2, None, false).unwrap();
    let c3 = node::create(&mut g, "place", Some("c")).unwrap();
    assert_eq!(node::name(&g, c3).unwrap(), "c");
    let dg = node::create(&mut g, "adder", Some("c")).unwrap();
    assert_eq!(node::name(&g, dg).unwrap(), "c1", "a non-DAG node clashes with any node");
    let top = node::create(&mut g, "place", Some("c1")).unwrap();
    assert_eq!(node::name(&g, top).unwrap(), "c2");
    assert_eq!(dag::full_path_name(&g, c1).unwrap(), "|a|c");
    assert_eq!(dag::partial_path_name(&g, c1).unwrap(), "a|c");
    assert_eq!(dag::partial_path_name(&g, a).unwrap(), "a");
}

#[test]
fn attributes_in_order_with_children() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    node::add_attribute(&mut g, a, d("extra", "ex")).unwrap();
    let names: Vec<String> = node::attributes(&g, a).unwrap().iter().map(|x| g.attr(*x).name.clone()).collect();
    assert_eq!(names, ["a", "b", "sum", "pair", "first", "second", "items", "value", "weight", "list", "extra"]);
    assert_eq!(node::attribute_count(&g, a).unwrap(), 11);
    let ex = node::attribute_named(&g, a, "ex").unwrap();
    assert!(node::is_new_attribute(&g, a, ex).unwrap());
    assert!(!node::is_new_attribute(&g, a, node::attribute_named(&g, a, "a").unwrap()).unwrap());
    let s = node::attribute_named(&g, a, "sum").unwrap();
    let av = node::attribute_named(&g, a, "a").unwrap();
    assert_eq!(node::affected_attributes(&g, a, av).unwrap(), [s]);
    assert_eq!(node::affected_by_attributes(&g, a, s).unwrap().len(), 2);
    assert_eq!(node::type_name(&g, a).unwrap(), "adder");
    assert_eq!(node::type_id(&g, a).unwrap(), 1);
}

#[test]
fn connections_list_this_nodes_plugs() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "a"), false).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "b"), false).unwrap();
    assert_eq!(node::connections(&g, a).unwrap(), [p(&g, a, "sum")]);
    assert_eq!(node::connections(&g, b).unwrap(), [p(&g, b, "a"), p(&g, b, "b")]);
}

#[test]
fn dag_function_set() {
    let mut g = graph();
    let a = node::create(&mut g, "place", Some("a")).unwrap();
    let b = node::create(&mut g, "place", Some("b")).unwrap();
    let x = node::create(&mut g, "place", Some("x")).unwrap();
    let y = node::create(&mut g, "place", Some("y")).unwrap();
    dag::add_child(&mut g, Some(a), x, None, false).unwrap();
    dag::add_child(&mut g, Some(a), y, Some(0), false).unwrap();
    assert_eq!(dag::child(&g, Some(a), 0).unwrap(), y, "the index places the child");
    dag::add_child(&mut g, Some(b), x, None, true).unwrap();
    assert_eq!(dag::parent_count(&g, x).unwrap(), 2);
    assert!(dag::is_instanced(&g, x, false).unwrap());
    assert_eq!(dag::instance_count(&g, x, true).unwrap(), 2);
    assert!(dag::has_parent(&g, x, Some(b)));
    assert!(dag::is_child_of(&g, x, a) && dag::is_parent_of(&g, a, x));
    let paths = dag::all_paths(&g, x).unwrap();
    assert_eq!(paths.len(), 2);
    assert_eq!(dag::path_full_name(&g, &paths[1]).unwrap(), "|b|x");
    assert_eq!(dag::path_partial_name(&g, &paths[1]).unwrap(), "b|x", "x alone names two paths");
    assert_eq!(dag::path_instance_number(&g, &paths[1]).unwrap(), 1);
    assert!(dag::path_is_valid(&g, &paths[1]));
    let up = dag::path_pop(&paths[1], 1).unwrap();
    assert_eq!(dag::path_push(&g, &up, x).unwrap(), paths[1]);
    assert!(dag::path_push(&g, &up, y).is_err());
    dag::remove_child(&mut g, Some(b), x).unwrap();
    assert!(!dag::path_is_valid(&g, &paths[1]));
    assert_eq!(dag::child_count(&g, None).unwrap(), 2);
    let dg = node::create(&mut g, "adder", None).unwrap();
    assert!(dag::parent_count(&g, dg).is_err());
}

#[test]
fn plug_structure() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", Some("n")).unwrap();
    let items = node::find_plug(&g, a, "items").unwrap();
    assert!(plug::is_array(&g, &items) && plug::is_compound(&g, &items));
    let e = plug::element_by_logical_index(&g, &items, 4).unwrap();
    assert!(plug::is_element(&e) && !plug::is_array(&g, &e));
    assert_eq!(plug::logical_index(&e).unwrap(), 4);
    let w = plug::child(&g, &e, 1).unwrap();
    assert!(plug::is_child(&g, &w));
    assert_eq!(plug::parent(&g, &w).unwrap(), e);
    assert_eq!(plug::array(&e).unwrap(), items);
    assert_eq!(plug::name(&g, &w), "n.items[4].weight");
    assert_eq!(plug::partial_name(&g, &w, PartialName::default()), "it[4].w");
    let long = PartialName { use_long_names: true, include_node_name: true, ..Default::default() };
    assert_eq!(plug::partial_name(&g, &w, long), "n.items[4].weight");
    let fi = node::find_plug(&g, a, "first").unwrap();
    assert_eq!(plug::partial_name(&g, &fi, PartialName::default()), "fi");
    let full = PartialName { use_full_attribute_path: true, ..Default::default() };
    assert_eq!(plug::partial_name(&g, &fi, full), "pr.fi");
    g.set(&p(&g, a, "items[2].value"), Value::Double(1.0)).unwrap();
    g.set(&w, Value::Double(1.0)).unwrap();
    assert_eq!(plug::existing_indices(&g, &items).unwrap(), [2, 4]);
    assert_eq!(plug::element_by_physical_index(&g, &items, 1).unwrap(), e);
    assert_eq!(plug::num_children(&g, &e), 2);
    assert_eq!(plug::num_children(&g, &items), 0, "an array plug has elements, not children");
}

#[test]
fn plug_connections_and_flags() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let b = node::create(&mut g, "adder", None).unwrap();
    let (s, ba) = (p(&g, a, "sum"), p(&g, b, "a"));
    g.connect(&s, &ba, false).unwrap();
    g.connect(&s, &p(&g, b, "items[3].value"), false).unwrap();
    assert!(plug::is_source(&g, &s) && !plug::is_destination(&g, &s));
    assert!(plug::is_destination(&g, &ba) && plug::is_connected(&g, &ba));
    assert_eq!(plug::connected_to(&g, &ba, true, false), std::slice::from_ref(&s));
    assert_eq!(plug::num_connected_elements(&g, &p(&g, b, "items")).unwrap(), 1);
    assert_eq!(plug::num_connected_children(&g, &p(&g, b, "items[3]")), 1);
    assert!(plug::is_keyable(&g, &ba));
    plug::set_keyable(&mut g, &ba, false).unwrap();
    assert!(!plug::is_keyable(&g, &ba), "the plug overrides its attribute");
    assert!(plug::is_keyable(&g, &p(&g, a, "a")), "other nodes keep the attribute's setting");
    let pb = p(&g, a, "b");
    plug::set_channel_box(&mut g, &pb, true).unwrap();
    assert!(plug::is_channel_box(&g, &p(&g, a, "b")));
    assert_eq!(plug::is_free_to_change(&g, &ba, true, true), FreeToChange::NotFreeToChange);
    let pair = p(&g, a, "pair");
    let second = p(&g, a, "second");
    plug::set_locked(&mut g, &second, true).unwrap();
    assert_eq!(plug::is_free_to_change(&g, &pair, true, true), FreeToChange::ChildrenNotFreeToChange);
    assert_eq!(plug::is_free_to_change(&g, &pair, true, false), FreeToChange::FreeToChange);
    plug::set_locked(&mut g, &pair, true).unwrap();
    assert_eq!(plug::is_free_to_change(&g, &p(&g, a, "first"), true, true), FreeToChange::NotFreeToChange);
    assert_eq!(plug::is_free_to_change(&g, &p(&g, a, "first"), false, true), FreeToChange::FreeToChange);
}

#[test]
fn plug_values() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", None).unwrap();
    let pa = p(&g, a, "a");
    assert!(plug::is_default_value(&mut g, &pa).unwrap());
    plug::set_value(&mut g, &pa, Value::Double(2.5)).unwrap();
    assert!(!plug::is_default_value(&mut g, &pa).unwrap());
    let sum = p(&g, a, "sum");
    assert_eq!(plug::as_double(&mut g, &sum).unwrap(), 2.5);
    assert_eq!(plug::as_int(&mut g, &pa).unwrap(), 2);
    assert!(plug::as_bool(&mut g, &pa).unwrap());
    assert!(plug::as_string(&mut g, &pa).is_err());
}
