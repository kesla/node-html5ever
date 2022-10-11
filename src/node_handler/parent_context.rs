use napi::{Env, Error, Result};

use crate::{Handle, ParentNode};

pub struct ParentContext {
  pub(crate) node: ParentNode,
  pub(crate) index: usize,
  pub(crate) env: Env,
}

impl ParentContext {
  pub(crate) fn new(env: Env, node: ParentNode, index: usize) -> Self {
    ParentContext { env, node, index }
  }

  pub(crate) fn get_handle(&self) -> Result<Handle> {
    let handle: Handle = match &self.node {
      ParentNode::Document(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
      ParentNode::DocumentFragment(weak_reference) => {
        weak_reference.upgrade(self.env)?.unwrap().into()
      }
      ParentNode::Element(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
    };
    Ok(handle)
  }

  pub(crate) fn is_document(&self) -> bool {
    matches!(&self.node, ParentNode::Document(_))
  }
}

impl TryInto<Handle> for &ParentContext {
  type Error = Error;

  fn try_into(self) -> Result<Handle> {
    self.get_handle()
  }
}
