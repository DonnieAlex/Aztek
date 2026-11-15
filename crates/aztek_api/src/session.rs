//! A session: the scene (graph and active selection), its undo queue and its callbacks. Commands run
//! modifiers through it so that every change is one undo step. Callbacks hear each change as it
//! happens; a callback cannot add or remove callbacks while it runs.

use crate::messages::Messages;
use crate::modifier::DgModifier;
use crate::selection::SelectionList;
use aztek_dg::{Graph, Result};
use aztek_undo::{UndoAction, UndoStack};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

/// What undo steps change.
pub struct Scene {
    pub graph: Graph,
    pub selection: SelectionList,
}

pub struct Session {
    pub scene: Scene,
    pub undo: UndoStack<Scene>,
    messages: Rc<RefCell<Messages>>,
}

/// A modifier on the undo queue.
pub struct ModifierStep(pub DgModifier);

impl UndoAction<Scene> for ModifierStep {
    fn undo(&mut self, s: &mut Scene) {
        if let Err(e) = self.0.undo_it(&mut s.graph) {
            s.graph.warn(format!("undo: {e}"));
        }
    }

    fn redo(&mut self, s: &mut Scene) {
        if let Err(e) = self.0.do_it(&mut s.graph) {
            s.graph.warn(format!("redo: {e}"));
        }
    }
}

/// A change of the active selection on the undo queue.
pub struct SelectionStep {
    pub before: SelectionList,
    pub after: SelectionList,
}

impl UndoAction<Scene> for SelectionStep {
    fn undo(&mut self, s: &mut Scene) {
        s.selection = self.before.clone();
    }

    fn redo(&mut self, s: &mut Scene) {
        s.selection = self.after.clone();
    }

    fn selection_only(&self) -> bool {
        true
    }
}

impl Session {
    pub fn new(mut graph: Graph) -> Self {
        let messages: Rc<RefCell<Messages>> = Rc::default();
        let m = messages.clone();
        graph.set_listener(Some(Box::new(move |g, e| {
            if let Ok(mut m) = m.try_borrow_mut() {
                m.dispatch(g, e);
            }
        })));
        Session { scene: Scene { graph, selection: SelectionList::new() }, undo: UndoStack::new(), messages }
    }

    /// The callbacks, to add or remove some.
    pub fn messages(&self) -> RefMut<'_, Messages> {
        self.messages.borrow_mut()
    }

    pub fn graph(&self) -> &Graph {
        &self.scene.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.scene.graph
    }

    /// Applies a modifier as one undo step. On an error what it applied is undone and nothing is
    /// recorded.
    pub fn run(&mut self, mut m: DgModifier) -> Result<()> {
        let r = m.do_it(&mut self.scene.graph);
        if let Err(e) = r {
            let _ = m.undo_it(&mut self.scene.graph);
            return Err(e);
        }
        self.undo.record(ModifierStep(m));
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        let before = self.scene.selection.clone();
        let done = self.undo.undo(&mut self.scene);
        self.after_replay(before);
        done
    }

    pub fn redo(&mut self) -> bool {
        let before = self.scene.selection.clone();
        let done = self.undo.redo(&mut self.scene);
        self.after_replay(before);
        done
    }

    fn after_replay(&mut self, before: SelectionList) {
        if self.scene.selection != before {
            self.post_event("SelectionChanged");
        }
    }

    /// Calls the callbacks of a named event.
    pub fn post_event(&mut self, name: &str) {
        self.messages.borrow_mut().post_event(&self.scene.graph, name);
    }

    pub fn active_selection(&self) -> &SelectionList {
        &self.scene.selection
    }

    /// Replaces the active selection; `undoable` makes it an undo step.
    pub fn set_active_selection(&mut self, list: SelectionList, undoable: bool) {
        if list == self.scene.selection {
            return;
        }
        let before = std::mem::replace(&mut self.scene.selection, list.clone());
        if undoable {
            self.undo.record(SelectionStep { before, after: list });
        }
        self.post_event("SelectionChanged");
    }
}
