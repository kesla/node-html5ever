use super::NodeHandler;

use super::NodeHandlerInner;

use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct WeakNodeHandler(Weak<NodeHandlerInner>);

impl WeakNodeHandler {
  pub(crate) fn upgrade(&self) -> Option<NodeHandler> {
    self.0.upgrade().map(|h| NodeHandler(h))
  }
}

impl From<&NodeHandler> for WeakNodeHandler {
  fn from(node_handler: &NodeHandler) -> Self {
    WeakNodeHandler(Rc::downgrade(&node_handler.0))
  }
}
