use fallible_iterator::FallibleIterator;
use napi::{bindgen_prelude::Reference, Error, Result};

use crate::{ChildNode, Element, Handle, NodeHandler};

pub(crate) enum PreviousIterator {
  Data {
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl FallibleIterator for PreviousIterator {
  type Item = ChildNode;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>> {
    let (node_handler, index) = match self {
      Self::Data {
        node_handler,
        index,
      } => (node_handler, index),
      Self::None => return Ok(None),
    };

    if *index == 0 {
      return Ok(None);
    }

    *index -= 1;

    let child_nodes = node_handler.get_child_nodes();
    let handle = child_nodes.get(*index).unwrap();
    let e = handle_to_child_node(handle)?;
    Ok(Some(e))
  }
}

fn handle_to_child_node(handle: &Handle) -> Result<ChildNode> {
  let e = match handle {
    Handle::Comment(r) => ChildNode::Comment(r.clone(r.env)?),
    Handle::DocumentType(r) => ChildNode::DocumentType(r.clone(r.env)?),
    Handle::Element(r) => ChildNode::Element(r.clone(r.env)?),
    Handle::Text(r) => ChildNode::Text(r.clone(r.env)?),
    _ => panic!("Invalid handle"),
  };
  Ok(e)
}

pub(crate) enum NextIterator {
  Data {
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl FallibleIterator for NextIterator {
  type Item = ChildNode;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>> {
    let (node_handler, index) = match self {
      Self::Data {
        node_handler,
        index,
      } => (node_handler, index),
      Self::None => return Ok(None),
    };

    let child_nodes = node_handler.get_child_nodes();
    let last = child_nodes.len() - 1;

    if *index == last {
      return Ok(None);
    }

    *index += 1;

    let child_nodes = node_handler.get_child_nodes();
    let handle = child_nodes.get(*index).unwrap();

    Ok(Some(handle_to_child_node(handle)?))
  }
}

pub(crate) struct ChildNodesIterator {
  queue: Vec<Handle>,
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
    let handle = match self.queue.pop() {
      Some(handle) => handle,
      None => return None,
    };

    if self.deep {
      if let Handle::Element(r) = &handle {
        let node_handler = r.get_node_handler();
        let child_nodes = node_handler.get_child_nodes();
        self.queue.extend(child_nodes.iter().rev().cloned());
      }
    }

    Some(handle_to_child_node(&handle).unwrap())
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
