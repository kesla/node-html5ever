use fallible_iterator::FallibleIterator;
use napi::{
  bindgen_prelude::{Either4, Reference},
  Env, Error, Result,
};

use crate::{Comment, DocumentType, Element, Handle, NodeHandler, Text};

pub(crate) enum PreviousIterator {
  Data {
    env: Env,
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl FallibleIterator for PreviousIterator {
  type Item =
    Either4<Reference<Comment>, Reference<DocumentType>, Reference<Element>, Reference<Text>>;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>> {
    let (env, node_handler, index) = match self {
      Self::Data {
        env,
        node_handler,
        index,
      } => (env.clone(), node_handler, index),
      Self::None => return Ok(None),
    };

    if *index == 0 {
      return Ok(None);
    }

    *index -= 1;

    let child_nodes = node_handler.get_child_nodes();
    let handle = child_nodes.get(*index).unwrap();
    let e = match handle {
      Handle::Comment(r) => Either4::A(r.clone(env)?),
      Handle::DocumentType(r) => Either4::B(r.clone(env)?),
      Handle::Element(r) => Either4::C(r.clone(env)?),
      Handle::Text(r) => Either4::D(r.clone(env)?),
      _ => panic!("Invalid handle"),
    };
    Ok(Some(e))
  }
}

pub(crate) enum NextIterator {
  Data {
    env: Env,
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl FallibleIterator for NextIterator {
  type Item =
    Either4<Reference<Comment>, Reference<DocumentType>, Reference<Element>, Reference<Text>>;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>> {
    let (env, node_handler, index) = match self {
      Self::Data {
        env,
        node_handler,
        index,
      } => (env.clone(), node_handler, index),
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
    let e = match handle {
      Handle::Comment(r) => Either4::A(r.clone(env)?),
      Handle::DocumentType(r) => Either4::B(r.clone(env)?),
      Handle::Element(r) => Either4::C(r.clone(env)?),
      Handle::Text(r) => Either4::D(r.clone(env)?),
      _ => panic!("Invalid handle"),
    };
    Ok(Some(e))
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
  type Item =
    Either4<Reference<Comment>, Reference<DocumentType>, Reference<Element>, Reference<Text>>;

  fn next(&mut self) -> Option<Self::Item> {
    let handle = match self.queue.pop() {
      Some(handle) => handle,
      None => return None,
    };

    let e = match handle {
      Handle::Comment(r) => Either4::A(r),
      Handle::DocumentType(r) => Either4::B(r),
      Handle::Element(r) => {
        if self.deep {
          let node_handler = r.get_node_handler();
          let child_nodes = node_handler.get_child_nodes();
          self.queue.extend(child_nodes.iter().rev().cloned());
        }

        Either4::C(r.clone(r.env).unwrap())
      }
      Handle::Text(r) => Either4::D(r),
      _ => panic!("Invalid handle"),
    };
    Some(e)
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
      Either4::C(e) => Some(e),
      _ => None,
    })
  }
}
