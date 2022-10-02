use std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

use crate::{Handle, NodeData, WeakHandle};

fn new_handle(node: NodeData) -> Handle {
  Rc::new(node)
}

fn new_weak_handle(maybe_handle: Option<Handle>) -> WeakHandle {
  match maybe_handle {
    Some(handle) => Rc::downgrade(&handle),
    None => Weak::new(),
  }
}

#[derive(Default)]
pub(crate) struct LazyWeakHandle(RefCell<WeakHandle>);

impl LazyWeakHandle {
  pub(crate) fn get_or_init<T: Into<NodeData>>(&self, to_node: T) -> Handle {
    let mut weak_handle = self.0.borrow_mut();

    let maybe_handle = weak_handle.upgrade();

    match maybe_handle {
      Some(handle) => handle,
      None => {
        let node: NodeData = to_node.into();
        let handle = new_handle(node);
        *weak_handle = new_weak_handle(Some(handle.clone()));
        handle
      }
    }
  }
}
