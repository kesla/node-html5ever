use std::cell::RefCell;

use napi::Env;

use crate::{Handle, NodeReference, WeakHandle};

pub(crate) struct LazyWeakHandle(RefCell<WeakHandle>, Env);

impl LazyWeakHandle {
  pub(crate) fn new(env: Env) -> Self {
    Self(Default::default(), env)
  }

  pub(crate) fn get_or_init<T: Into<NodeReference>>(&self, to_node: T) -> Handle {
    let mut weak_handle = self.0.borrow_mut();

    let maybe_handle = weak_handle.upgrade();

    match maybe_handle {
      Some(handle) => handle,
      None => {
        let node: NodeReference = to_node.into();
        let handle = Handle::new(self.1, node);
        *weak_handle = handle.downgrade();
        handle
      }
    }
  }
}
