use std::cell::RefCell;

use napi::Env;

use crate::{Handle, NodeReference};

pub(crate) struct LazyWeakHandle(RefCell<Option<Handle>>, Env);

impl LazyWeakHandle {
  pub(crate) fn new(env: Env) -> Self {
    Self(Default::default(), env)
  }

  pub(crate) fn get_or_init<T: Into<NodeReference>>(&self, to_node: T) -> Handle {
    let mut maybe_handle = self.0.borrow_mut();

    match maybe_handle.as_mut() {
      Some(handle) => handle.to_owned(),
      None => {
        let node: NodeReference = to_node.into();
        let handle = Handle::new(self.1, node);
        *maybe_handle = Some(handle.clone());
        handle
      }
    }
  }
}
