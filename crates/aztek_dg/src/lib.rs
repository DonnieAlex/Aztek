//! Aztek's dependency graph: the reference's attribute model and evaluation, with no names in it.
//!
//! - Attributes of every kind the reference has (numeric, unit, enum, typed, matrix, message,
//!   compound, numeric compound, generic, light data), with the reference's properties and defaults.
//! - The DAG: hierarchy, instancing, paths and instance numbers, world-space dirty propagation.
//! - Node types with inheritance, attribute affects, extension attributes; dynamic attributes per node.
//! - Plugs down through compound children and sparse array elements.
//! - Connections checked by kind and direction, with the three disconnect behaviours.
//! - Push-dirty, pull evaluation; computes read and write through a datablock and hand plugs they do
//!   not own to the type they derive from.

pub mod attr;
pub mod dag;
pub mod events;
pub mod data;
pub mod datablock;
pub mod graph;
pub mod node;
pub mod plug;
pub mod registry;
mod store;
pub mod types;
pub mod value;

pub use attr::{AttrDef, AttrId, AttrKind, AttrSpec, Limits, Props};
pub use dag::DagPath;
pub use events::Event;
pub use datablock::DataBlock;
pub use graph::{Connection, DagLinks, Detached, DetachedAttr, Graph, Listener};
pub use node::{NodeId, NodeImpl};
pub use plug::{Plug, PlugPath, Step};
pub use registry::{ClassId, ClassInit, Factory, Registry};
pub use types::{DataType, DgError, DisconnectBehavior, MatrixType, NumericType, Result, Status, UnitType};
pub use value::Value;
