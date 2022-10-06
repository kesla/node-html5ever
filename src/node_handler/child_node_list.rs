use std;

use crate::NodeHandler;

#[derive(Default)]
pub(crate) struct ChildNodeList(Vec<NodeHandler>);

impl ChildNodeList {
  pub(crate) fn get(&self, index: usize) -> Option<&NodeHandler> {
    self.0.get(index)
  }

  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn iter(&self) -> std::slice::Iter<NodeHandler> {
    self.0.iter()
  }

  pub(crate) fn remove_node_handler(&mut self, node_handler: &NodeHandler) {
    self.0.retain(|h| h != node_handler);

    let mut index = 0;
    self.0.iter().for_each(|h| {
      let mut borrow_mut = h.0.parent_context.borrow_mut();
      borrow_mut.as_mut().unwrap().index = index;
      index += 1;
    });

    self.sync_parent_context();
  }

  pub(crate) fn sync_parent_context(&mut self) {
    for index in 0..self.0.len() {
      let mut borrow_mut = self.0[index].0.parent_context.borrow_mut();
      borrow_mut.as_mut().unwrap().index = index;
    }
  }

  pub(crate) fn append_node_handler(&mut self, child: NodeHandler) {
    self.0.push(child);
  }
}

impl Into<Vec<NodeHandler>> for ChildNodeList {
  fn into(self) -> Vec<NodeHandler> {
    self.0
  }
}
