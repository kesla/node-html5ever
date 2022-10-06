use std::cell::RefCell;

use napi::Env;

use crate::{NodeHandler, NodeReference};

pub(crate) struct LazyWeakNodeHandler(RefCell<Option<NodeHandler>>, Env);

impl LazyWeakNodeHandler {
  pub(crate) fn new(env: Env) -> Self {
    Self(Default::default(), env)
  }

  pub(crate) fn get_or_init<T: Into<NodeReference>>(&self, to_node: T) -> NodeHandler {
    let mut maybe_node_handler = self.0.borrow_mut();

    match maybe_node_handler.as_mut() {
      Some(node_handler) => node_handler.to_owned(),
      None => {
        let node: NodeReference = to_node.into();
        let node_handler = NodeHandler::new(self.1, node);
        *maybe_node_handler = Some(node_handler.clone());
        node_handler
      }
    }
  }
}
