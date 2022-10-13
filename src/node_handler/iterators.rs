use std::marker::PhantomData;

use napi::bindgen_prelude::Reference;

use crate::{ChildNode, Element, Node, NodeHandler};

pub struct ElementIterator<T> {
  data: Option<SiblingsInnerIterator>,
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

struct SiblingsInnerIterator {
  node_handler: NodeHandler,
  index: usize,
  next_index: &'static dyn Fn(usize) -> Option<usize>,
}

impl Iterator for SiblingsInnerIterator {
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

pub enum SiblingIteratorType {
  Next,
  Previous,
}

pub struct SiblingIterator<T> {
  inner: Option<SiblingsInnerIterator>,
  _phantom: PhantomData<T>,
}

impl<T> SiblingIterator<T> {
  pub fn new(input: Option<(NodeHandler, usize)>, sibling_type: SiblingIteratorType) -> Self {
    SiblingIterator {
      inner: input.map(|(node_handler, index)| SiblingsInnerIterator {
        node_handler,
        index,
        next_index: match sibling_type {
          SiblingIteratorType::Next => &|index: usize| index.checked_add(1),
          SiblingIteratorType::Previous => &|index: usize| index.checked_sub(1),
        },
      }),
      _phantom: PhantomData,
    }
  }
}

impl<T> Iterator for SiblingIterator<T>
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
