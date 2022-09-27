use std::cell::RefCell;

use crate::{dom::{WeakHandle, Handle, new_handle, new_weak_handle}, node::Node};



pub(crate) struct LazyWeakHandle(RefCell<WeakHandle>);

impl Default for LazyWeakHandle {
  fn default() -> Self {
    Self(Default::default())
  }
}

impl LazyWeakHandle {
  pub(crate) fn get_or_init<T: Into<Node>>(&self, to_node: T) -> Handle {
    let mut weak_handle = self.0.borrow_mut();

    let maybe_handle = weak_handle.upgrade();

    match maybe_handle {
      Some(handle) => handle,
      None => {
        let node: Node = to_node.into();
        let handle = new_handle(node);
        *weak_handle = new_weak_handle(Some(handle.clone()));
        handle
      }
    }
  }
}