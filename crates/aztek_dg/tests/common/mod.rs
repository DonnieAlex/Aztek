//! Test node types. Names here are test-only; real node types arrive with the node type port.
#![allow(dead_code)]

use aztek_dg::attr::{self, AttrSpec};
use aztek_dg::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Counts {
    pub adder: AtomicUsize,
    pub uncached: AtomicUsize,
    pub hook: AtomicUsize,
    pub made: AtomicUsize,
    pub broken: AtomicUsize,
    pub internal_set: AtomicUsize,
}

impl Counts {
    pub fn get(c: &AtomicUsize) -> usize {
        c.load(Ordering::SeqCst)
    }
}

fn find(db: &DataBlock, name: &str) -> AttrId {
    db.graph().find_attr(db.node(), name).unwrap()
}

macro_rules! tryf {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Status::Failure(e.to_string()),
        }
    };
}

/// sum = a + b
struct Adder(Arc<Counts>);
impl NodeImpl for Adder {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let sum = find(data, "sum");
        if plug.leaf() != sum {
            return Status::UnknownParameter;
        }
        self.0.adder.fetch_add(1, Ordering::SeqCst);
        let a = tryf!(data.input_value(find(data, "a"))).as_f64().unwrap();
        let b = tryf!(data.input_value(find(data, "b"))).as_f64().unwrap();
        tryf!(data.set_output(sum, Value::Double(a + b)));
        tryf!(data.set_clean(sum));
        Status::Success
    }
    fn set_dependents_dirty(&self, _graph: &Graph, _plug: &Plug, _affected: &mut Vec<Plug>) {
        self.0.hook.fetch_add(1, Ordering::SeqCst);
    }
    fn connection_made(&self, _p: &Plug, _o: &Plug, _s: bool) {
        self.0.made.fetch_add(1, Ordering::SeqCst);
    }
    fn connection_broken(&self, _p: &Plug, _o: &Plug, _s: bool) {
        self.0.broken.fetch_add(1, Ordering::SeqCst);
    }
}

/// matrix = translation by translate, row-major with the translation in the last row.
struct Xform;
impl NodeImpl for Xform {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let m = find(data, "matrix");
        if plug.leaf() != m {
            return Status::UnknownParameter;
        }
        let t = tryf!(data.input_value(find(data, "translate"))).as_tuple().unwrap();
        let mut out = aztek_dg::data::IDENTITY;
        out[12..15].copy_from_slice(&t);
        tryf!(data.set_output(m, Value::matrix(out)));
        tryf!(data.set_clean(m));
        Status::Success
    }
}

/// output = sum of inputs[i].value * inputs[i].weight
struct Blend;
impl NodeImpl for Blend {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let o = find(data, "output");
        if plug.leaf() != o {
            return Status::UnknownParameter;
        }
        let inputs = tryf!(data.input_value(find(data, "inputs")));
        let total: f64 = inputs
            .as_array()
            .unwrap()
            .iter()
            .map(|(_, e)| {
                let c = e.as_compound().unwrap();
                c[0].as_f64().unwrap() * c[1].as_f64().unwrap()
            })
            .sum();
        tryf!(data.set_output(o, Value::Double(total)));
        tryf!(data.set_clean(o));
        Status::Success
    }
}

/// out[i] = in[i] * 2, computed one element at a time.
struct Doubler;
impl NodeImpl for Doubler {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let out = find(data, "out");
        if plug.leaf() != out {
            return Status::UnknownParameter;
        }
        if plug.is_element() {
            let index = plug.path.0.last().unwrap().index.unwrap();
            let input = tryf!(data.plug_of(find(data, "in"))).element(index);
            let x = tryf!(data.input_plug_value(&input)).as_f64().unwrap();
            tryf!(data.set_output_plug(plug, Value::Double(x * 2.0)));
            tryf!(data.set_clean_plug(plug));
        } else {
            let ins = tryf!(data.input_value(find(data, "in")));
            let items = ins.as_array().unwrap().iter().map(|(i, v)| (*i, Value::Double(v.as_f64().unwrap() * 2.0)));
            tryf!(data.set_output_plug(plug, Value::Array(items.collect())));
            tryf!(data.set_clean_plug(plug));
        }
        Status::Success
    }
}

/// y = x + 1
struct Base;
impl NodeImpl for Base {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let y = find(data, "y");
        if plug.leaf() != y {
            return Status::UnknownParameter;
        }
        let x = tryf!(data.input_value(find(data, "x"))).as_f64().unwrap();
        tryf!(data.set_output(y, Value::Double(x + 1.0)));
        tryf!(data.set_clean(y));
        Status::Success
    }
}

/// w = z * 10; everything else goes to the base type.
struct Derived;
impl NodeImpl for Derived {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let w = find(data, "w");
        if plug.leaf() != w {
            return Status::UnknownParameter;
        }
        let z = tryf!(data.input_value(find(data, "z"))).as_f64().unwrap();
        tryf!(data.set_output(w, Value::Double(z * 10.0)));
        tryf!(data.set_clean(w));
        Status::Success
    }
}

/// out = in, not cached, counted.
struct Uncached(Arc<Counts>);
impl NodeImpl for Uncached {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let out = find(data, "out");
        if plug.leaf() != out {
            return Status::UnknownParameter;
        }
        self.0.uncached.fetch_add(1, Ordering::SeqCst);
        let x = tryf!(data.input_value(find(data, "in")));
        tryf!(data.set_output(out, x));
        Status::Success
    }
}

/// Keeps `val` itself, doubled on the way out.
struct Internal(Arc<Counts>, Mutex<f64>);
impl NodeImpl for Internal {
    fn get_internal_value(&self, _plug: &Plug) -> Option<Value> {
        Some(Value::Double(*self.1.lock().unwrap() * 2.0))
    }
    fn set_internal_value(&self, _plug: &Plug, v: &Value) {
        self.0.internal_set.fetch_add(1, Ordering::SeqCst);
        *self.1.lock().unwrap() = v.as_f64().unwrap();
    }
}

/// `trigger` dirties `result` through set_dependents_dirty only; result = other.
struct Hooked;
impl NodeImpl for Hooked {
    fn compute(&self, plug: &Plug, data: &mut DataBlock) -> Status {
        let r = find(data, "result");
        if plug.leaf() != r {
            return Status::UnknownParameter;
        }
        let t = tryf!(data.input_value(find(data, "trigger"))).as_f64().unwrap();
        let o = tryf!(data.input_value(find(data, "other"))).as_f64().unwrap();
        tryf!(data.set_output(r, Value::Double(t + o)));
        tryf!(data.set_clean(r));
        Status::Success
    }
    fn set_dependents_dirty(&self, graph: &Graph, plug: &Plug, affected: &mut Vec<Plug>) {
        if plug.leaf() == graph.find_attr(plug.node, "trigger").unwrap() {
            affected.push(graph.plug(plug.node, "result").unwrap());
        }
    }
}

/// Fails every compute.
struct Failing;
impl NodeImpl for Failing {
    fn compute(&self, _plug: &Plug, _data: &mut DataBlock) -> Status {
        Status::Failure("broken on purpose".into())
    }
}

fn out(s: AttrSpec) -> AttrSpec {
    s.writable(false).storable(false)
}

pub fn d(name: &str, short: &str) -> AttrSpec {
    attr::numeric(name, short, NumericType::Double)
}

pub fn graph() -> (Graph, Arc<Counts>) {
    let counts = Arc::new(Counts::default());
    let mut r = Registry::new();
    let c = counts.clone();
    r.register_class("adder", 1, None, Some(Arc::new(move || Arc::new(Adder(c.clone())) as Arc<dyn NodeImpl>)), |i| {
        let a = i.add_attribute(d("a", "a").keyable(true))?;
        let b = i.add_attribute(d("b", "b").keyable(true))?;
        let s = i.add_attribute(out(d("sum", "s")))?;
        i.attribute_affects(a, s)?;
        i.attribute_affects(b, s)
    })
    .unwrap();
    r.register_class("xform", 2, None, Some(Arc::new(|| Arc::new(Xform) as Arc<dyn NodeImpl>)), |i| {
        let unit = |n: &str, s: &str| attr::unit(n, s, UnitType::Distance).keyable(true);
        let t = i.add_attribute(attr::numeric_compound(
            "translate",
            "t",
            vec![unit("translateX", "tx"), unit("translateY", "ty"), unit("translateZ", "tz")],
        )?)?;
        let m = i.add_attribute(out(attr::matrix("matrix", "m", MatrixType::Double)))?;
        i.attribute_affects(t, m)
    })
    .unwrap();
    r.register_class("blend", 3, None, Some(Arc::new(|| Arc::new(Blend) as Arc<dyn NodeImpl>)), |i| {
        let inputs = i.add_attribute(attr::compound("inputs", "i", vec![d("value", "v"), d("weight", "w")]).array(true))?;
        let o = i.add_attribute(out(d("output", "o")))?;
        i.attribute_affects(inputs, o)
    })
    .unwrap();
    r.register_class("doubler", 4, None, Some(Arc::new(|| Arc::new(Doubler) as Arc<dyn NodeImpl>)), |i| {
        let a = i.add_attribute(d("in", "in").array(true))?;
        let b = i.add_attribute(out(d("out", "out").array(true).uses_array_data_builder(true)))?;
        i.attribute_affects(a, b)
    })
    .unwrap();
    r.register_class("base", 5, None, Some(Arc::new(|| Arc::new(Base) as Arc<dyn NodeImpl>)), |i| {
        let x = i.add_attribute(d("x", "x"))?;
        let y = i.add_attribute(out(d("y", "y")))?;
        i.attribute_affects(x, y)
    })
    .unwrap();
    r.register_class("derived", 6, Some("base"), Some(Arc::new(|| Arc::new(Derived) as Arc<dyn NodeImpl>)), |i| {
        let z = i.add_attribute(d("z", "z"))?;
        let w = i.add_attribute(out(d("w", "w")))?;
        i.attribute_affects(z, w)
    })
    .unwrap();
    let c = counts.clone();
    r.register_class("uncached", 7, None, Some(Arc::new(move || Arc::new(Uncached(c.clone())) as Arc<dyn NodeImpl>)), |i| {
        let a = i.add_attribute(d("in", "in"))?;
        let b = i.add_attribute(out(d("out", "out")).cached(false))?;
        i.attribute_affects(a, b)
    })
    .unwrap();
    let c = counts.clone();
    r.register_class(
        "internal",
        8,
        None,
        Some(Arc::new(move || Arc::new(Internal(c.clone(), Mutex::new(0.0))) as Arc<dyn NodeImpl>)),
        |i| i.add_attribute(d("val", "val").internal(true)).map(|_| ()),
    )
    .unwrap();
    r.register_class("failing", 9, None, Some(Arc::new(|| Arc::new(Failing) as Arc<dyn NodeImpl>)), |i| {
        let a = i.add_attribute(d("in", "in"))?;
        let b = i.add_attribute(out(d("out", "out")))?;
        i.attribute_affects(a, b)
    })
    .unwrap();
    r.register_class("shape", 10, None, None, |i| {
        i.set_abstract();
        i.add_attribute(attr::message("message", "msg")).map(|_| ())
    })
    .unwrap();
    r.register_class("hooked", 11, None, Some(Arc::new(|| Arc::new(Hooked) as Arc<dyn NodeImpl>)), |i| {
        i.add_attribute(d("trigger", "tr"))?;
        let o = i.add_attribute(d("other", "ot"))?;
        let r = i.add_attribute(out(d("result", "res")))?;
        i.attribute_affects(o, r)
    })
    .unwrap();
    (Graph::with_registry(r), counts)
}

pub fn p(g: &Graph, n: NodeId, s: &str) -> Plug {
    g.plug(n, s).unwrap_or_else(|e| panic!("{s}: {e}"))
}

pub fn f(g: &mut Graph, n: NodeId, s: &str) -> f64 {
    let pl = p(g, n, s);
    g.get(&pl).unwrap().as_f64().unwrap()
}

pub fn set(g: &mut Graph, n: NodeId, s: &str, v: impl Into<Value>) {
    let pl = p(g, n, s);
    g.set(&pl, v.into()).unwrap_or_else(|e| panic!("{s}: {e}"));
}

pub fn connect(g: &mut Graph, a: NodeId, sa: &str, b: NodeId, sb: &str) {
    let (x, y) = (p(g, a, sa), p(g, b, sb));
    g.connect(&x, &y, false).unwrap_or_else(|e| panic!("{sa} -> {sb}: {e}"));
}
