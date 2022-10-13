use std::marker::PhantomData;

use napi::bindgen_prelude::Reference;

use crate::{ChildNode, Element, NodeHandler};

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

pub(crate) struct ChildNodesIterator {
  queue: Vec<ChildNode>,
  deep: bool,
}

impl ChildNodesIterator {
  pub(crate) fn new(node_handler: &NodeHandler, deep: bool) -> Self {
    let queue = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();
    Self { queue, deep }
  }
}

impl Iterator for ChildNodesIterator {
  type Item = ChildNode;

  fn next(&mut self) -> Option<Self::Item> {
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

pub(crate) struct ChildrenIterator(ChildNodesIterator);

impl ChildrenIterator {
  pub(crate) fn new(node_handler: &NodeHandler, deep: bool) -> Self {
    Self(ChildNodesIterator::new(node_handler, deep))
  }
}

impl Iterator for ChildrenIterator {
  type Item = Reference<Element>;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.find_map(|e| match e {
      ChildNode::Element(e) => Some(e),
      _ => None,
    })
  }
}
