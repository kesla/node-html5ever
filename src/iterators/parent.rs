use napi::Result;

use crate::{NodeHandler, ParentContext, ParentNode};

pub struct ParentIterator {
  pub(crate) maybe_parent_ctx: Option<ParentContext>,
}

impl ParentIterator {
  pub(crate) fn new(maybe_parent_ctx: Option<ParentContext>) -> Self {
    Self { maybe_parent_ctx }
  }
}

impl Iterator for ParentIterator {
  type Item = Result<ParentNode>;

  fn next(&mut self) -> Option<Self::Item> {
    let parent_ctx = match &self.maybe_parent_ctx {
      Some(ctx) => ctx,
      None => return None,
    };
    let result = Some(Ok(parent_ctx.node.clone()));

    let node_handler: NodeHandler = match parent_ctx.try_into() {
      Ok(node_handler) => node_handler,
      Err(err) => return Some(Err(err)),
    };

    self.maybe_parent_ctx = node_handler.parent_context.cloned();
    result
  }
}
