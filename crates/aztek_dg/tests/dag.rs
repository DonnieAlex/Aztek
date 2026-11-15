//! The DAG: hierarchy, instances, world-space values.

mod common;
use aztek_dg::attr;
use aztek_dg::*;
use common::*;
use std::sync::Arc;

/// worldX[i] = (parent's worldX along instance i) + tx
struct Place;
impl NodeImpl for Place {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let g = data.graph();
        let (wx, tx) = (g.find_attr(data.node(), "worldX").unwrap(), g.find_attr(data.node(), "tx").unwrap());
        if plug.leaf() != wx {
            return Status::UnknownParameter;
        }
        let count = g.instance_count(data.node()) as u32;
        let local = data.input_value(tx).unwrap().as_f64().unwrap();
        let mut items = Vec::new();
        for i in 0..count {
            let up = data.parent_value(i, wx).unwrap().and_then(|v| v.as_f64()).unwrap_or(0.0);
            items.push((i, Value::Double(up + local)));
        }
        let whole = data.graph().plug_for(data.node(), wx).unwrap();
        data.set_output_plug(&whole, Value::Array(items)).unwrap();
        data.set_clean_plug(&whole).unwrap();
        Status::Success
    }
}

fn dag() -> Graph {
    let (mut g, _) = graph();
    g.registry
        .register_class("place", 50, None, Some(Arc::new(|| Arc::new(Place) as Arc<dyn NodeImpl>)), |i| {
            i.set_dag();
            let tx = i.add_attribute(d("tx", "tx").affects_world_space(true))?;
            let w = i.add_attribute(
                d("worldX", "wx").array(true).world_space(true).writable(false).storable(false).uses_array_data_builder(true),
            )?;
            i.attribute_affects(tx, w)
        })
        .unwrap();
    g
}

fn wx(g: &mut Graph, n: NodeId, i: u32) -> f64 {
    let pl = p(g, n, "worldX").element(i);
    g.get(&pl).unwrap().as_f64().unwrap()
}

#[test]
fn new_dag_nodes_go_under_the_world() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let k = g.create_node("adder", "k").unwrap();
    assert!(g.is_dag(a) && !g.is_dag(k));
    assert_eq!(g.dag_parents(a), [None]);
    assert_eq!(g.dag_children(None), [a]);
    assert_eq!(g.dag_paths(a), [DagPath(vec![a])]);
    assert!(g.set_parent(k, None, false).is_err());
    assert!(g.set_parent(a, Some(k), false).is_err());
}

#[test]
fn children_follow_their_parents() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let b = g.create_node("place", "b").unwrap();
    g.set_parent(b, Some(a), false).unwrap();
    assert_eq!(g.dag_children(None), [a]);
    assert_eq!(g.dag_paths(b), [DagPath(vec![a, b])]);
    set(&mut g, a, "tx", 2.0);
    set(&mut g, b, "tx", 3.0);
    assert_eq!(wx(&mut g, b, 0), 5.0);
    set(&mut g, a, "tx", 10.0);
    assert!(g.is_dirty(&p(&g, b, "worldX")), "moving a parent dirties its children's world space");
    assert_eq!(wx(&mut g, b, 0), 13.0);
    assert!(g.set_parent(a, Some(b), false).is_err(), "no cycles");
    assert!(g.set_parent(a, Some(a), false).is_err());
}

#[test]
fn instances_have_one_world_value_each() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let c = g.create_node("place", "c").unwrap();
    let b = g.create_node("place", "b").unwrap();
    g.set_parent(b, Some(a), false).unwrap();
    g.set_parent(b, Some(c), true).unwrap();
    assert!(g.set_parent(b, Some(c), true).is_err());
    set(&mut g, a, "tx", 1.0);
    set(&mut g, c, "tx", 100.0);
    set(&mut g, b, "tx", 5.0);
    let paths = g.dag_paths(b);
    assert_eq!(paths, [DagPath(vec![a, b]), DagPath(vec![c, b])]);
    assert_eq!(g.instance_number(&paths[1]), Some(1));
    assert_eq!(wx(&mut g, b, 0), 6.0);
    assert_eq!(wx(&mut g, b, 1), 105.0);
    assert_eq!(g.elements(&p(&g, b, "worldX")).unwrap(), [0, 1]);
    g.remove_parent(b, Some(a)).unwrap();
    assert_eq!(g.elements(&p(&g, b, "worldX")).unwrap(), [0], "the gone instance's element goes too");
    assert_eq!(wx(&mut g, b, 0), 105.0);
    assert!(g.remove_parent(b, Some(c)).is_err(), "the last parent stays");
}

#[test]
fn instances_multiply_down_the_hierarchy() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let b = g.create_node("place", "b").unwrap();
    let leaf = g.create_node("place", "leaf").unwrap();
    g.set_parent(leaf, Some(b), false).unwrap();
    g.set_parent(b, Some(a), true).unwrap();
    assert_eq!(g.dag_paths(leaf), [DagPath(vec![b, leaf]), DagPath(vec![a, b, leaf])]);
    set(&mut g, a, "tx", 7.0);
    set(&mut g, leaf, "tx", 1.0);
    assert_eq!(wx(&mut g, leaf, 0), 1.0);
    assert_eq!(wx(&mut g, leaf, 1), 8.0);
}

#[test]
fn deleting_takes_children_but_not_other_instances() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let c = g.create_node("place", "c").unwrap();
    let only = g.create_node("place", "only").unwrap();
    let shared = g.create_node("place", "shared").unwrap();
    g.set_parent(only, Some(a), false).unwrap();
    g.set_parent(shared, Some(a), false).unwrap();
    g.set_parent(shared, Some(c), true).unwrap();
    g.delete_node(a).unwrap();
    assert!(!g.exists(only));
    assert!(g.exists(shared));
    assert_eq!(g.dag_parents(shared), [Some(c)]);
    assert_eq!(g.dag_children(None), [c]);
}

#[test]
fn reparenting_to_the_world() {
    let mut g = dag();
    let a = g.create_node("place", "a").unwrap();
    let b = g.create_node("place", "b").unwrap();
    g.set_parent(b, Some(a), false).unwrap();
    set(&mut g, a, "tx", 4.0);
    assert_eq!(wx(&mut g, b, 0), 4.0);
    g.set_parent(b, None, false).unwrap();
    assert_eq!(g.dag_children(None), [a, b]);
    assert_eq!(wx(&mut g, b, 0), 0.0);
    let _ = attr::message("m", "m");
}
