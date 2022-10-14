use std::{self, slice::Iter};

use crate::{ChildNode, NodeHandler};

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

  pub(crate) fn remove_node(&mut self, node: &ChildNode) {
    self.0.retain(|h| h != node);

    self.sync_parent_context();
  }

  pub(crate) fn sync_parent_context(&mut self) {
    for index in 0..self.0.len() {
      let node_handler: NodeHandler = (&self.0[index]).into();
      let mut parent_context = node_handler.parent_context.take();
      assert!(parent_context.is_some());

      if let Some(mut ctx) = parent_context.as_mut() {
        ctx.index = index;
      }

      node_handler.parent_context.set(parent_context);
    }
  }

  pub(crate) fn append_node(&mut self, child: &ChildNode) {
    self.0.push(child.to_owned());
  }

  pub(crate) fn clear(&mut self) {
    println!("Clearing child node list");
    // self.0.clear();
    // self.0 = Vec::new();
  }
}

impl From<ChildNodeList> for Vec<ChildNode> {
  fn from(child_node_list: ChildNodeList) -> Self {
    child_node_list.0
  }
}
