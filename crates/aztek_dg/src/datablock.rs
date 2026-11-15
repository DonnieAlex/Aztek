//! What a compute reads its inputs from and writes its outputs to.

use crate::attr::AttrId;
use crate::graph::Graph;
use crate::node::NodeId;
use crate::plug::Plug;
use crate::types::{DgError, Result};
use crate::value::Value;

/// A node's datablock during one compute. Reading an input pulls it, computing upstream as needed;
/// writing an output only stores it. Attributes inside arrays take their element from the plug being
/// computed when that plug is inside the same array.
pub struct DataBlock<'g> {
    pub(crate) graph: &'g mut Graph,
    pub(crate) plug: Plug,
}

impl DataBlock<'_> {
    pub fn node(&self) -> NodeId {
        self.plug.node
    }

    /// The plug being computed.
    pub fn plug(&self) -> &Plug {
        &self.plug
    }

    pub fn graph(&self) -> &Graph {
        self.graph
    }

    /// The plug for an attribute of this node, in the context of the plug being computed.
    pub fn plug_of(&self, attr: AttrId) -> Result<Plug> {
        let path = self.graph.path_for_attr(attr, Some(&self.plug.path));
        let p = Plug { node: self.plug.node, path };
        self.graph.validate(&p, false)?;
        Ok(p)
    }

    pub fn input_value(&mut self, attr: AttrId) -> Result<Value> {
        let p = self.plug_of(attr)?;
        self.graph.eval(&p)
    }

    pub fn input_plug_value(&mut self, plug: &Plug) -> Result<Value> {
        self.own(plug)?;
        self.graph.eval(plug)
    }

    pub fn set_output(&mut self, attr: AttrId, v: Value) -> Result<()> {
        let p = self.plug_of(attr)?;
        self.set_output_plug(&p, v)
    }

    /// Writes an output. Writing a whole array may only add elements when the attribute uses an
    /// array data builder.
    pub fn set_output_plug(&mut self, plug: &Plug, v: Value) -> Result<()> {
        self.own(plug)?;
        let d = self.graph.registry.attr(plug.leaf());
        if d.props.array && !plug.is_element() && !d.props.uses_array_data_builder {
            if let Value::Array(items) = &v {
                let have = self.graph.elements(plug)?;
                if items.iter().any(|(i, _)| !have.contains(i)) {
                    return Err(DgError::Invalid(format!("{}: adding elements needs an array data builder", d.name)));
                }
            }
        }
        self.graph.write_value(plug, v)
    }

    pub fn set_clean(&mut self, attr: AttrId) -> Result<()> {
        let p = self.plug_of(attr)?;
        self.graph.mark_clean(&p);
        Ok(())
    }

    pub fn set_clean_plug(&mut self, plug: &Plug) -> Result<()> {
        self.own(plug)?;
        self.graph.mark_clean(plug);
        Ok(())
    }

    /// The value of `attr` on the DAG parent along instance `instance` of this node, taking the
    /// parent's own instance element for world-space arrays; `None` under the world. This is how a
    /// node's parent-space inputs follow its parent.
    pub fn parent_value(&mut self, instance: u32, attr: AttrId) -> Result<Option<Value>> {
        let paths = self.graph.dag_paths(self.plug.node);
        let path = paths
            .get(instance as usize)
            .ok_or_else(|| DgError::BadPlug(format!("no instance {instance}")))?;
        let Some(parent_path) = path.parent() else { return Ok(None) };
        let parent = parent_path.node();
        let mut plug = self.graph.plug_for(parent, attr)?;
        if self.graph.registry.attr(attr).props.array {
            plug = plug.element(self.graph.instance_number(&parent_path).unwrap_or(0));
        }
        self.graph.eval(&plug).map(Some)
    }

    fn own(&self, plug: &Plug) -> Result<()> {
        if plug.node != self.plug.node {
            return Err(DgError::BadPlug("a datablock only reaches its own node".into()));
        }
        self.graph.validate(plug, false)
    }
}
