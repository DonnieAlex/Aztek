//! Test node types; real types arrive with the node type port.
#![allow(dead_code)]

use aztek_dg::attr::{self, AttrSpec};
use aztek_dg::*;
use std::sync::Arc;

/// sum = a + b
struct Adder;
impl NodeImpl for Adder {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let g = data.graph();
        let n = data.node();
        let (a, b, s) = (g.find_attr(n, "a").unwrap(), g.find_attr(n, "b").unwrap(), g.find_attr(n, "sum").unwrap());
        if plug.leaf() != s {
            return Status::UnknownParameter;
        }
        let x = data.input_value(a).unwrap().as_f64().unwrap() + data.input_value(b).unwrap().as_f64().unwrap();
        data.set_output(s, Value::Double(x)).unwrap();
        data.set_clean(s).unwrap();
        Status::Success
    }
}

pub fn d(name: &str, short: &str) -> AttrSpec {
    attr::numeric(name, short, NumericType::Double)
}

pub fn graph() -> Graph {
    let mut r = Registry::new();
    r.register_class("adder", 1, None, Some(Arc::new(|| Arc::new(Adder) as Arc<dyn NodeImpl>)), |i| {
        let a = i.add_attribute(d("a", "a").keyable(true))?;
        let b = i.add_attribute(d("b", "b"))?;
        let s = i.add_attribute(d("sum", "s").writable(false))?;
        i.add_attribute(attr::compound("pair", "pr", vec![d("first", "fi"), d("second", "se")]))?;
        i.add_attribute(attr::compound("items", "it", vec![d("value", "v"), d("weight", "w")]).array(true))?;
        i.add_attribute(d("list", "ls").array(true).disconnect_behavior(DisconnectBehavior::Delete))?;
        i.attribute_affects(a, s)?;
        i.attribute_affects(b, s)
    })
    .unwrap();
    r.register_class("place", 2, None, None, |i| {
        i.set_dag();
        i.add_attribute(d("tx", "tx").affects_world_space(true))?;
        i.add_attribute(d("worldX", "wx").array(true).world_space(true).writable(false))?;
        Ok(())
    })
    .unwrap();
    Graph::with_registry(r)
}

pub fn p(g: &Graph, n: NodeId, s: &str) -> Plug {
    g.plug(n, s).unwrap_or_else(|e| panic!("{s}: {e}"))
}

pub fn f(g: &mut Graph, n: NodeId, s: &str) -> f64 {
    let pl = p(g, n, s);
    g.get(&pl).unwrap().as_f64().unwrap()
}
