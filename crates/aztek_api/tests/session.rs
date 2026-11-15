//! Selection lists, callbacks, iterators, aliases and the session.

mod common;
use aztek_api::iter::{self, Direction, Traversal};
use aztek_api::messages::{attr_msg, Message};
use aztek_api::plug::PartialName;
use aztek_api::*;
use common::*;
use std::cell::RefCell;
use std::rc::Rc;

fn tree(g: &mut Graph) -> (NodeId, NodeId, NodeId, NodeId) {
    let top = node::create(g, "place", Some("top")).unwrap();
    let a = node::create(g, "place", Some("a")).unwrap();
    let b = node::create(g, "place", Some("b")).unwrap();
    let other = node::create(g, "place", Some("other")).unwrap();
    dag::add_child(g, Some(top), a, None, false).unwrap();
    dag::add_child(g, Some(a), b, None, false).unwrap();
    dag::add_child(g, Some(other), a, None, true).unwrap();
    (top, a, b, other)
}

#[test]
fn selection_adds_merges_and_names() {
    let mut g = graph();
    let (top, a, _b, _other) = tree(&mut g);
    let n = node::create(&mut g, "adder", Some("n")).unwrap();
    let mut s = SelectionList::new();
    s.add_node(&g, n, false).unwrap();
    s.add_node(&g, n, true).unwrap();
    assert_eq!(s.len(), 1, "merging skips what is there");
    s.add_node(&g, top, false).unwrap();
    assert_eq!(s.strings(&g).unwrap(), ["n", "top"]);
    assert!(s.has_node(top) && s.has_node(n));
    let path = dag::all_paths(&g, a).unwrap()[1].clone();
    s.add_path(path.clone(), Some(Component::single("vtx", [0, 1, 2, 5])), false);
    s.add_path(path.clone(), Some(Component::single("vtx", [3])), true);
    assert_eq!(s.item_strings(&g, 2).unwrap(), ["other|a.vtx[0:3]", "other|a.vtx[5]"]);
    assert!(s.has_path(&path, Some(&Component::single("vtx", [1, 5]))));
    assert!(!s.has_path(&path, Some(&Component::single("vtx", [4]))));
    assert!(s.has_path_partly(&path, &Component::single("vtx", [4, 5])));
    s.toggle(&path, Some(&Component::single("vtx", [5, 6])));
    assert_eq!(s.item_strings(&g, 2).unwrap(), ["other|a.vtx[0:3]", "other|a.vtx[6]"]);
    s.add_plug(p(&g, n, "items[2].weight"), false);
    assert_eq!(s.plug(3).unwrap(), p(&g, n, "items[2].weight"));
    assert_eq!(s.depend_node(2).unwrap(), a);
    assert!(s.dag_path(0).is_err() && s.plug(0).is_err());
}

#[test]
fn selection_from_names_and_patterns() {
    let mut g = graph();
    let (top, a, b, _other) = tree(&mut g);
    let n1 = node::create(&mut g, "adder", Some("blendA")).unwrap();
    let n2 = node::create(&mut g, "adder", Some("blendB")).unwrap();
    let mut s = SelectionList::new();
    s.add_matching(&g, "blend*").unwrap();
    assert!(s.has_node(n1) && s.has_node(n2) && s.len() == 2);
    let mut s = SelectionList::new();
    s.add_matching(&g, "|top|a|b").unwrap();
    assert_eq!(s.len(), 1);
    assert_eq!(s.dag_path(0).unwrap().0.0, vec![top, a, b]);
    let mut s = SelectionList::new();
    s.add_matching(&g, "a|b").unwrap();
    assert_eq!(s.len(), 2, "both instances of b");
    let mut s = SelectionList::new();
    s.add_matching(&g, "blendA.items[3].value").unwrap();
    assert_eq!(s.plug(0).unwrap(), p(&g, n1, "items[3].value"));
    assert!(SelectionList::new().add_matching(&g, "nothing*").is_err());
}

#[test]
fn selection_merge_strategies_and_intersect() {
    let mut g = graph();
    let x = node::create(&mut g, "adder", Some("x")).unwrap();
    let y = node::create(&mut g, "adder", Some("y")).unwrap();
    let z = node::create(&mut g, "adder", Some("z")).unwrap();
    let list = |ids: &[NodeId]| {
        let mut s = SelectionList::new();
        for i in ids {
            s.add_node(&g, *i, true).unwrap();
        }
        s
    };
    let mut s = list(&[x, y]);
    s.merge(&list(&[y, z]), MergeStrategy::MergeNormal);
    assert_eq!(s, list(&[x, y, z]));
    let mut s = list(&[x, y]);
    s.merge(&list(&[y, z]), MergeStrategy::XorWithList);
    assert_eq!(s, list(&[x, z]));
    let mut s = list(&[x, y]);
    s.merge(&list(&[y, z]), MergeStrategy::RemoveFromList);
    assert_eq!(s, list(&[x]));
    let mut s = list(&[x, y]);
    s.intersect(&list(&[y, z]));
    assert_eq!(s, list(&[y]));
}

#[test]
fn callbacks_hear_changes() {
    let mut ses = Session::new(graph());
    let heard: Rc<RefCell<Vec<String>>> = Rc::default();
    let h = heard.clone();
    ses.messages().add_node_added_callback(Some("place"), move |g, m| {
        if let Message::NodeAdded(n) = m {
            h.borrow_mut().push(format!("added {}", g.name(*n).unwrap()));
        }
    });
    let mut m = DgModifier::new();
    let a = m.create_node(ses.graph_mut(), "adder", Some("a")).unwrap();
    let b = m.create_node(ses.graph_mut(), "adder", Some("b")).unwrap();
    m.create_node(ses.graph_mut(), "place", Some("pl")).unwrap();
    ses.run(m).unwrap();
    assert_eq!(*heard.borrow(), ["added pl"], "the type filter keeps adders out");
    let h = heard.clone();
    ses.messages().add_attribute_changed_callback(b, move |g, m| {
        if let Message::AttributeChanged { msg, plug, other } = m {
            h.borrow_mut().push(format!("{msg:#x} {} {:?}", g.plug_name(plug), other.as_ref().map(|o| g.plug_name(o))));
        }
    });
    let h = heard.clone();
    let id = ses.messages().add_name_changed_callback(None, move |g, m| {
        if let Message::NameChanged { node, previous } = m {
            h.borrow_mut().push(format!("{previous} -> {}", g.name(*node).unwrap()));
        }
    });
    heard.borrow_mut().clear();
    let mut m = DgModifier::new();
    m.connect(&p(ses.graph(), a, "sum"), &p(ses.graph(), b, "a"));
    m.new_plug_value(&p(ses.graph(), b, "items[4].value"), Value::Double(1.0));
    m.rename_node(a, "first");
    m.set_plug_lock_state(&p(ses.graph(), b, "b"), true);
    ses.run(m).unwrap();
    let made = attr_msg::CONNECTION_MADE | attr_msg::INCOMING_DIRECTION;
    assert_eq!(
        *heard.borrow(),
        [
            format!("{made:#x} b.a Some(\"a.sum\")"),
            format!("{:#x} b.items[4] None", attr_msg::ATTRIBUTE_ARRAY_ADDED),
            format!("{:#x} b.items[4].value None", attr_msg::ATTRIBUTE_SET),
            "a -> first".to_string(),
            format!("{:#x} b.b None", attr_msg::ATTRIBUTE_LOCKED),
        ]
    );
    assert!(ses.messages().remove_callback(id));
    assert!(!ses.messages().remove_callback(id));
}

#[test]
fn undo_and_redo_go_through_the_session() {
    let mut ses = Session::new(graph());
    let removed: Rc<RefCell<usize>> = Rc::default();
    let r = removed.clone();
    ses.messages().add_node_removed_callback(None, move |_, _| *r.borrow_mut() += 1);
    let mut m = DgModifier::new();
    let a = m.create_node(ses.graph_mut(), "adder", None).unwrap();
    ses.run(m).unwrap();
    let mut sel = SelectionList::new();
    sel.add_node(ses.graph(), a, false).unwrap();
    let changes: Rc<RefCell<usize>> = Rc::default();
    let c = changes.clone();
    ses.messages().add_event_callback("SelectionChanged", move |_, _| *c.borrow_mut() += 1);
    ses.set_active_selection(sel.clone(), true);
    assert_eq!(*changes.borrow(), 1);
    assert!(ses.undo());
    assert!(ses.active_selection().is_empty());
    assert_eq!(*changes.borrow(), 2);
    assert!(ses.undo());
    assert!(!ses.graph().exists(a));
    assert_eq!(*removed.borrow(), 1, "callbacks hear undo too");
    assert!(ses.redo() && ses.redo());
    assert!(ses.graph().exists(a));
    assert_eq!(ses.active_selection(), &sel);
}

#[test]
fn a_failed_modifier_leaves_nothing() {
    let mut ses = Session::new(graph());
    let mut m = DgModifier::new();
    let a = m.create_node(ses.graph_mut(), "adder", None).unwrap();
    ses.run(m).unwrap();
    let mut m = DgModifier::new();
    m.new_plug_value(&p(ses.graph(), a, "a"), Value::Double(2.0));
    m.connect(&p(ses.graph(), a, "a"), &p(ses.graph(), a, "sum"));
    assert!(ses.run(m).is_err());
    assert_eq!(f(ses.graph_mut(), a, "a"), 0.0, "the value set before the failure is undone");
    assert_eq!(ses.undo.undo_count(), 1);
}

#[test]
fn dag_iteration_follows_instances() {
    let mut g = graph();
    let (top, a, b, other) = tree(&mut g);
    let names = |paths: Vec<DagPath>| -> Vec<String> { paths.iter().map(|p| dag::path_full_name(&g, p).unwrap()).collect() };
    assert_eq!(
        names(iter::dag(&g, None, Traversal::DepthFirst, None)),
        ["|top", "|top|a", "|top|a|b", "|other", "|other|a", "|other|a|b"]
    );
    assert_eq!(
        names(iter::dag(&g, None, Traversal::BreadthFirst, None)),
        ["|top", "|other", "|top|a", "|other|a", "|top|a|b", "|other|a|b"]
    );
    let root = dag::all_paths(&g, a).unwrap()[1].clone();
    assert_eq!(names(iter::dag(&g, Some(&root), Traversal::DepthFirst, None)), ["|other|a", "|other|a|b"]);
    let _ = (top, b, other);
    let n = node::create(&mut g, "adder", None).unwrap();
    assert_eq!(iter::dependency_nodes(&g, Some("adder")), [n]);
    assert_eq!(iter::dependency_nodes(&g, None).len(), 5);
}

#[test]
fn graph_iteration_up_and_down() {
    let mut g = graph();
    let a = node::create(&mut g, "adder", Some("a")).unwrap();
    let b = node::create(&mut g, "adder", Some("b")).unwrap();
    let c = node::create(&mut g, "adder", Some("c")).unwrap();
    g.connect(&p(&g, a, "sum"), &p(&g, b, "a"), false).unwrap();
    g.connect(&p(&g, b, "sum"), &p(&g, c, "b"), false).unwrap();
    assert_eq!(iter::dependency_graph_nodes(&g, a, Direction::Downstream, Traversal::DepthFirst), [a, b, c]);
    assert_eq!(iter::dependency_graph_nodes(&g, c, Direction::Upstream, Traversal::BreadthFirst), [c, b, a]);
    let plugs = iter::dependency_graph_plugs(&g, &p(&g, a, "a"), Direction::Downstream, Traversal::DepthFirst);
    let names: Vec<String> = plugs.iter().map(|x| g.plug_name(x)).collect();
    assert_eq!(names, ["a.a", "a.sum", "b.a", "b.sum", "c.b", "c.sum"]);
}

#[test]
fn aliases_name_plugs() {
    let mut g = graph();
    let n = node::create(&mut g, "adder", Some("n")).unwrap();
    let w = p(&g, n, "items[0].weight");
    assert!(g.set_alias(n, "heavy", &w, true).unwrap());
    assert_eq!(g.plug(n, "heavy").unwrap(), w);
    assert_eq!(g.plug_alias(&w).as_deref(), Some("heavy"));
    let o = PartialName { use_alias: true, ..Default::default() };
    assert_eq!(plug::partial_name(&g, &w, o), "heavy");
    assert!(g.set_alias(n, "sum", &w, true).is_err(), "an attribute name");
    assert!(g.set_alias(n, "heavy", &p(&g, n, "a"), true).is_err(), "another plug's alias");
    g.set_alias(n, "lighter", &w, true).unwrap();
    assert_eq!(g.aliases(n).unwrap().len(), 1, "one alias per plug");
    assert!(g.set_alias(n, "lighter", &w, false).unwrap());
    assert!(g.plug(n, "lighter").is_err());
}
