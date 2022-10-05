use napi::{
  bindgen_prelude::{Either, Either4, Result, WeakReference},
  Env,
};

use crate::{Comment, DocType, Document, Element, Handle, NodeReference, Text};

pub(crate) struct ParentContext {
  pub(crate) node: Either<WeakReference<Document>, WeakReference<Element>>,
  index: usize,
}

impl ParentContext {
  pub(crate) fn new(
    node: Either<WeakReference<Document>, WeakReference<Element>>,
    index: usize,
  ) -> Self {
    ParentContext { node, index }
  }

  pub(crate) fn previous_iterator(&self, env: Env) -> Result<PreviousIterator> {
    let child_nodes: Vec<Handle> = self.get_child_nodes(env)?;

    Ok(PreviousIterator {
      child_nodes,
      index: self.index,
    })
  }

  pub(crate) fn next_iterator(&self, env: Env) -> Result<NextIterator> {
    let child_nodes: Vec<Handle> = self.get_child_nodes(env)?;

    Ok(NextIterator {
      child_nodes,
      index: self.index,
    })
  }

  fn get_child_nodes(&self, env: Env) -> Result<Vec<Handle>> {
    match &self.node {
      Either::A(element) => {
        let handle = element.upgrade(env)?.unwrap().get_handle();
        // TODO: don't clone
        let vec = handle.get_children().clone();
        Ok(vec)
      }
      Either::B(document) => {
        let handle = document.upgrade(env)?.unwrap().get_handle();
        // TODO: don't clone
        let vec = handle.get_children().clone();
        Ok(vec)
      }
    }
  }
}

pub(crate) struct PreviousIterator {
  child_nodes: Vec<Handle>,
  index: usize,
}

impl Iterator for PreviousIterator {
  type Item = Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  >;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == 0 {
      None
    } else {
      self.index -= 1;

      let handle = &self.child_nodes[self.index];
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

pub(crate) struct NextIterator {
  child_nodes: Vec<Handle>,
  index: usize,
}

impl Iterator for NextIterator {
  type Item = Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  >;

  fn next(&mut self) -> Option<Self::Item> {
    let last = self.child_nodes.len() - 1;

    if self.index == last {
      None
    } else {
      self.index += 1;

      let handle = &self.child_nodes[self.index];
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
