use fallible_iterator::FallibleIterator;
use napi::{bindgen_prelude::Reference, Error, Result};

use crate::{ChildNode, Element, Node, NodeHandler};

pub enum SiblingIterator {
  Data {
    node_handler: NodeHandler,
    index: usize,
    next_index: &'static dyn Fn(usize) -> Option<usize>,
  },
  None,
}

impl SiblingIterator {
  pub fn new_next_iterator(input: Option<(NodeHandler, usize)>) -> Self {
    match input {
      Some((node_handler, index)) => SiblingIterator::Data {
        node_handler,
        index,
        next_index: &|index: usize| index.checked_add(1),
      },
      None => SiblingIterator::None,
    }
  }

  pub fn new_prev_iterator(input: Option<(NodeHandler, usize)>) -> Self {
    match input {
      Some((node_handler, index)) => SiblingIterator::Data {
        node_handler,
        index,
        next_index: &|index: usize| index.checked_sub(1),
      },
      None => SiblingIterator::None,
    }
  }
}

impl FallibleIterator for SiblingIterator {
  type Item = ChildNode;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>> {
    let (node_handler, index, next_index) = match self {
      Self::Data {
        ref node_handler,
        index,
        ref next_index,
      } => (node_handler, index, next_index),
      Self::None => return Ok(None),
    };

    let child_nodes = node_handler.get_child_nodes();
    let next_index = match next_index(*index) {
      Some(i) => i,
      None => return Ok(None),
    };

    let node = match child_nodes.get(next_index) {
      Some(node) => node,
      None => return Ok(None),
    };
    *index = next_index;

    Ok(Some(handle_to_child_node(node)?))
  }
}

fn handle_to_child_node(node: &Node) -> Result<ChildNode> {
  let e = match node {
    Node::Comment(r) => ChildNode::Comment(r.clone(r.env)?),
    Node::DocumentType(r) => ChildNode::DocumentType(r.clone(r.env)?),
    Node::Element(r) => ChildNode::Element(r.clone(r.env)?),
    Node::Text(r) => ChildNode::Text(r.clone(r.env)?),
    _ => panic!("Invalid handle"),
  };
  Ok(e)
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

    Some(handle_to_child_node(&node).unwrap())
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
