//! Aztek's native API over the dependency graph, in the shape of the reference's: function sets for
//! nodes, DAG nodes and plugs; DAG paths; modifiers as the undoable way to change the graph;
//! selection lists; callbacks; iterators; a session tying graph, selection, undo and callbacks.
//! The C++ kit is a thin layer over this; commands are built on it.

pub mod dag;
pub mod iter;
pub mod messages;
pub mod modifier;
pub mod naming;
pub mod node;
pub mod plug;
pub mod selection;
pub mod session;

pub use aztek_dg::{DagPath, Graph, NodeId, Plug, Value};
pub use modifier::{DagModifier, DgModifier, UndoStep};
pub use selection::{Component, Item, MergeStrategy, SelectionList};
pub use session::{Scene, Session};
