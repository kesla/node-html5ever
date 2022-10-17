use crate::NodeHandler;

use std::{collections::VecDeque, marker::PhantomData};

use crate::ChildNode;

pub struct DeepChildNodesIterator<T> {
  queue: Vec<ChildNode>,
  _phantom: PhantomData<T>,
}

impl<T> DeepChildNodesIterator<T> {
  pub fn new(node_handler: &NodeHandler) -> Self {
    let queue = ShallowChildNodesIterator::<ChildNode>::new(node_handler)
      .rev()
      .collect();

    Self {
      queue,
      _phantom: PhantomData,
    }
  }
}

impl<T> Iterator for DeepChildNodesIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(node) = self.queue.pop() {
      if let ChildNode::Element(r) = &node {
        let node_handler = r.get_node_handler();
        self
          .queue
          .extend(ShallowChildNodesIterator::<ChildNode>::new(&node_handler).rev());
      }

      if let Ok(child) = node.try_into() {
        return Some(child);
      }
    }
    None
  }
}

pub struct ShallowChildNodesIterator<T> {
  queue: VecDeque<ChildNode>,
  _phantom: PhantomData<T>,
}

impl<T> ShallowChildNodesIterator<T> {
  pub fn new(node_handler: &NodeHandler) -> Self {
    let queue = node_handler
      .child_nodes
      .borrow(|child_nodes| child_nodes.iter().cloned().collect());

    Self {
      queue,
      _phantom: PhantomData,
    }
  }
}

impl<T> Iterator for ShallowChildNodesIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(node) = self.queue.pop_front() {
      if let Ok(child) = node.try_into() {
        return Some(child);
      }
    }
    None
  }
}

impl<T> DoubleEndedIterator for ShallowChildNodesIterator<T>
where
  ChildNode: TryInto<T>,
{
  fn next_back(&mut self) -> Option<Self::Item> {
    while let Some(node) = self.queue.pop_back() {
      if let Ok(child) = node.try_into() {
        return Some(child);
      }
    }
    None
  }
}
