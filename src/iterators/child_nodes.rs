use crate::NodeHandler;

use std::marker::PhantomData;

use crate::ChildNode;

pub struct ChildNodesIterator<T> {
  pub queue: Vec<ChildNode>,
  pub deep: bool,
  pub _phantom: PhantomData<T>,
}

impl<T> ChildNodesIterator<T> {
  pub fn new(node_handler: &NodeHandler, deep: bool) -> Self {
    let queue = node_handler
      .child_nodes
      .borrow(|child_nodes| child_nodes.iter().rev().cloned().collect());

    Self {
      queue,
      deep,
      _phantom: PhantomData,
    }
  }

  pub fn next_child_node(&mut self) -> Option<ChildNode> {
    let node = match self.queue.pop() {
      Some(handle) => handle,
      None => return None,
    };

    if self.deep {
      if let ChildNode::Element(r) = &node {
        let node_handler = r.get_node_handler();
        node_handler
          .child_nodes
          .borrow(|child_nodes| self.queue.extend(child_nodes.iter().rev().cloned()));
      }
    }

    Some(node)
  }
}

impl<T> Iterator for ChildNodesIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(child) = self.next_child_node() {
      if let Ok(child) = child.try_into() {
        return Some(child);
      }
    }

    None
  }
}
