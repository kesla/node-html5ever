use std::marker::PhantomData;

use napi::bindgen_prelude::Reference;

use crate::{ChildNode, Element, Node, NodeHandler};

pub struct ElementIterator<T> {
  data: Option<SiblingIterator>,
  _phantom: PhantomData<T>,
}

impl<T> Iterator for ElementIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .data
      .as_mut()
      .map(|i| {
        i.find_map(|child| match child.try_into() {
          Ok(v) => Some(v),
          Err(_) => None,
        })
      })
      .flatten()
  }
}

struct SiblingIterator {
  node_handler: NodeHandler,
  index: usize,
  next_index: &'static dyn Fn(usize) -> Option<usize>,
}

impl Iterator for SiblingIterator {
  type Item = ChildNode;

  fn next(&mut self) -> Option<ChildNode> {
    let child_nodes = self.node_handler.get_child_nodes();
    let next_index = match (self.next_index)(self.index) {
      Some(i) => i,
      None => return None,
    };

    let node = match child_nodes.get(next_index) {
      Some(node) => node,
      None => return None,
    };
    self.index = next_index;

    Some(node.into())
  }
}

pub struct NextIterator<T> {
  inner: Option<SiblingIterator>,
  _phantom: PhantomData<T>,
}

impl<T> NextIterator<T> {
  pub fn new(input: Option<(NodeHandler, usize)>) -> Self {
    NextIterator {
      inner: input.map(|(node_handler, index)| SiblingIterator {
        node_handler,
        index,
        next_index: &|index: usize| index.checked_add(1),
      }),
      _phantom: PhantomData,
    }
  }
}

impl<T> Iterator for NextIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .inner
      .as_mut()
      .map(|i| {
        i.find_map(|child| match child.try_into() {
          Ok(v) => Some(v),
          Err(_) => None,
        })
      })
      .flatten()
  }
}

pub struct PrevIterator<T> {
  inner: Option<SiblingIterator>,
  _phantom: PhantomData<T>,
}

impl<T> PrevIterator<T> {
  pub fn new(input: Option<(NodeHandler, usize)>) -> Self {
    PrevIterator {
      inner: input.map(|(node_handler, index)| SiblingIterator {
        node_handler,
        index,
        next_index: &|index: usize| index.checked_sub(1),
      }),
      _phantom: PhantomData,
    }
  }
}

impl<T> Iterator for PrevIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .inner
      .as_mut()
      .map(|i| {
        i.find_map(|child| match child.try_into() {
          Ok(v) => Some(v),
          Err(_) => None,
        })
      })
      .flatten()
  }
}

pub(crate) struct ChildNodesIterator {
  queue: Vec<Node>,
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
      if let Node::Element(r) = &node {
        let node_handler = r.get_node_handler();
        let child_nodes = node_handler.get_child_nodes();
        self.queue.extend(child_nodes.iter().rev().cloned());
      }
    }

    Some((&node).into())
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
