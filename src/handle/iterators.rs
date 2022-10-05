use napi::bindgen_prelude::{Either4, WeakReference};

use crate::{Comment, DocType, Element, Handle, Text};

use super::NodeReference;

pub(crate) enum PreviousIterator {
  Data { handle: Handle, index: usize },
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
    let (handle, index) = match self {
      PreviousIterator::Data { handle, index } => (handle, index),
      PreviousIterator::None => return None,
    };

    if *index == 0 {
      None
    } else {
      *index -= 1;
      let child_nodes = handle.get_children();
      let handle = child_nodes.get(*index).unwrap();
      let data: &NodeReference = handle.get_node_reference();
      match data {
        NodeReference::Comment(ref comment) => Some(Either4::A(comment.downgrade())),
        NodeReference::DocType(ref doc_type) => Some(Either4::B(doc_type.downgrade())),
        NodeReference::Element(ref element) => Some(Either4::C(element.downgrade())),
        NodeReference::Text(ref text) => Some(Either4::D(text.downgrade())),
        _ => unreachable!(),
      }
    }
  }
}

pub(crate) enum NextIterator {
  Data { handle: Handle, index: usize },
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
    let (handle, index) = match self {
      Self::Data { handle, index } => (handle, index),
      Self::None => return None,
    };

    let child_nodes = handle.get_children();
    let last = child_nodes.len() - 1;

    if *index == last {
      None
    } else {
      *index += 1;

      let handle = child_nodes.get(*index).unwrap();
      let data: &NodeReference = handle.get_node_reference();
      match data {
        NodeReference::Comment(ref comment) => Some(Either4::A(comment.downgrade())),
        NodeReference::DocType(ref doc_type) => Some(Either4::B(doc_type.downgrade())),
        NodeReference::Element(ref element) => Some(Either4::C(element.downgrade())),
        NodeReference::Text(ref text) => Some(Either4::D(text.downgrade())),
        _ => unreachable!(),
      }
    }
  }
}
