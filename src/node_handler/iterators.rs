use napi::bindgen_prelude::{Either4, WeakReference};

use crate::{Comment, DocType, Element, NodeHandler, Text};

pub(crate) enum PreviousIterator {
  Data {
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl Iterator for PreviousIterator {
  type Item = Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  >;

  fn next(&mut self) -> Option<Self::Item> {
    let (node_handler, index) = match self {
      PreviousIterator::Data {
        node_handler,
        index,
      } => (node_handler, index),
      PreviousIterator::None => return None,
    };

    if *index == 0 {
      None
    } else {
      *index -= 1;
      let child_nodes = node_handler.get_child_nodes();
      let handle = child_nodes.get(*index).unwrap();
      Some(handle.into())
    }
  }
}

pub(crate) enum NextIterator {
  Data {
    node_handler: NodeHandler,
    index: usize,
  },
  None,
}

impl Iterator for NextIterator {
  type Item = Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  >;

  fn next(&mut self) -> Option<Self::Item> {
    let (node_handler, index) = match self {
      Self::Data {
        node_handler,
        index,
      } => (node_handler, index),
      Self::None => return None,
    };

    let child_nodes = node_handler.get_child_nodes();
    let last = child_nodes.len() - 1;

    if *index == last {
      None
    } else {
      *index += 1;

      let handle = child_nodes.get(*index).unwrap();
      Some(handle.into())
    }
  }
}
