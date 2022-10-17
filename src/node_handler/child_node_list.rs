use std::{self, slice::Iter};

use crate::{ChildNode, NodeHandler};
use napi::{Error, Result};

#[derive(Default)]
pub(crate) struct ChildNodeList(Vec<ChildNode>);

impl ChildNodeList {
  pub(crate) fn get(&self, index: usize) -> Option<&ChildNode> {
    self.0.get(index)
  }

  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn iter(&self) -> Iter<ChildNode> {
    self.0.iter()
  }

  pub(crate) fn remove_node(&mut self, node: &ChildNode) -> Result<()> {
    let index = self
      .0
      .iter()
      .position(|child_node| child_node == node)
      .ok_or_else(|| Error::from_reason("Node not found"))?;

    self.0.remove(index);

    self.sync_parent_context();
    Ok(())
  }

  pub(crate) fn sync_parent_context(&mut self) {
    for index in 0..self.0.len() {
      let node_handler: NodeHandler = (&self.0[index]).into();

      node_handler.parent_context.borrow_mut(|parent_context| {
        assert!(parent_context.is_some());

        if let Some(mut ctx) = parent_context.as_mut() {
          ctx.index = index;
        }
      })
    }
  }

  pub(crate) fn append_node(&mut self, child: &ChildNode) {
    self.0.push(child.to_owned());
  }
}

impl From<ChildNodeList> for Vec<ChildNode> {
  fn from(child_node_list: ChildNodeList) -> Self {
    child_node_list.0
  }
}
