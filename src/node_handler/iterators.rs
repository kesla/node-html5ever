use std::marker::PhantomData;

use crate::{ChildNode, NodeHandler};

pub enum SiblingIteratorType {
  Next,
  Previous,
}

pub struct SiblingIterator<T> {
  data: Option<(NodeHandler, usize)>,
  next_index: &'static dyn Fn(usize) -> Option<usize>,
  _phantom: PhantomData<T>,
}

impl<T> SiblingIterator<T> {
  pub fn new(data: Option<(NodeHandler, usize)>, sibling_type: SiblingIteratorType) -> Self {
    SiblingIterator {
      data,
      next_index: match sibling_type {
        SiblingIteratorType::Next => &|index: usize| index.checked_add(1),
        SiblingIteratorType::Previous => &|index: usize| index.checked_sub(1),
      },
      _phantom: PhantomData,
    }
  }

  fn next_child_node(&mut self) -> Option<ChildNode> {
    let (node_handler, index) = match self.data {
      Some((ref node_handler, ref mut index)) => (node_handler, index),
      None => return None,
    };

    let child_nodes = node_handler.get_child_nodes();
    let next_index = match (self.next_index)(*index) {
      Some(i) => i,
      None => return None,
    };

    let node = match child_nodes.get(next_index) {
      Some(node) => node,
      None => return None,
    };
    *index = next_index;

    Some(node.clone())
  }
}

impl<T> Iterator for SiblingIterator<T>
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

pub(crate) struct ChildNodesIterator<T> {
  queue: Vec<ChildNode>,
  deep: bool,
  _phantom: PhantomData<T>,
}

impl<T> ChildNodesIterator<T> {
  pub(crate) fn new(node_handler: &NodeHandler, deep: bool) -> Self {
    let queue = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();
    Self {
      queue,
      deep,
      _phantom: PhantomData,
    }
  }

  fn next_child_node(&mut self) -> Option<ChildNode> {
    let node = match self.queue.pop() {
      Some(handle) => handle,
      None => return None,
    };

    if self.deep {
      if let ChildNode::Element(r) = &node {
        let node_handler = r.get_node_handler();
        let child_nodes = node_handler.get_child_nodes();
        self.queue.extend(child_nodes.iter().rev().cloned());
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
