use std::{self, slice::Iter};

use crate::{Node, NodeHandler};

#[derive(Default)]
pub(crate) struct ChildNodeList(Vec<Node>);

impl ChildNodeList {
  pub(crate) fn get(&self, index: usize) -> Option<&Node> {
    self.0.get(index)
  }

  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn iter(&self) -> Iter<Node> {
    self.0.iter()
  }

  pub(crate) fn remove_node(&mut self, node: &Node) {
    self.0.retain(|h| h != node);

    self.sync_parent_context();
  }

  pub(crate) fn sync_parent_context(&mut self) {
    for index in 0..self.0.len() {
      let node_handler: NodeHandler = (&self.0[index]).into();
      let mut parent_context = node_handler.parent_context.take();
      assert!(parent_context.is_some());
      parent_context.as_mut().map(|mut ctx| ctx.index = index);
      node_handler.parent_context.set(parent_context);
    }
  }

  pub(crate) fn append_node(&mut self, child: &Node) {
    self.0.push(child.to_owned());
  }
}

impl From<ChildNodeList> for Vec<Node> {
  fn from(child_node_list: ChildNodeList) -> Self {
    child_node_list.0
  }
}
