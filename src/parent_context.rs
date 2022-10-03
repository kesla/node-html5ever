use std::{
  borrow::Borrow,
  cell::{Ref, RefCell},
  rc::Rc,
};

use napi::{
  bindgen_prelude::{Either, Either4, Result, WeakReference},
  Env,
};

use crate::{node_data::NodeData, Comment, DocType, Document, Element, Handle, Text};

pub(crate) struct ParentContext {
  pub(crate) node: Either<WeakReference<Element>, WeakReference<Document>>,
  index: usize,
}

impl ParentContext {
  pub(crate) fn new(
    node: Either<WeakReference<Element>, WeakReference<Document>>,
    index: usize,
  ) -> Self {
    ParentContext { node, index }
  }

  pub(crate) fn previous_iterator(&self, env: Env) -> Result<PreviousIterator> {
    let child_nodes: Rc<RefCell<Vec<Handle>>> = self.get_child_nodes(env)?;

    Ok(PreviousIterator {
      child_nodes,
      index: self.index,
    })
  }

  pub(crate) fn next_iterator(&self, env: Env) -> Result<NextIterator> {
    let child_nodes: Rc<RefCell<Vec<Handle>>> = self.get_child_nodes(env)?;

    Ok(NextIterator {
      child_nodes,
      index: self.index,
    })
  }

  fn get_child_nodes(&self, env: Env) -> Result<Rc<RefCell<Vec<Handle>>>> {
    match &self.node {
      Either::A(element) => Ok(element.upgrade(env)?.unwrap().list.clone()),
      Either::B(document) => Ok(document.upgrade(env)?.unwrap().list.clone()),
    }
  }
}

pub(crate) struct PreviousIterator {
  child_nodes: Rc<RefCell<Vec<Handle>>>,
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
      let borrow: &RefCell<_> = self.child_nodes.borrow();
      let borrow: Ref<_> = borrow.borrow();
      let data: &NodeData = borrow[self.index].borrow();
      match data {
        NodeData::Comment(ref comment) => Some(Either4::A(comment.downgrade())),
        NodeData::DocType(ref doc_type) => Some(Either4::B(doc_type.downgrade())),
        NodeData::Element(ref element) => Some(Either4::C(element.downgrade())),
        NodeData::Text(ref text) => Some(Either4::D(text.downgrade())),
        _ => unreachable!(),
      }
    }
  }
}

pub(crate) struct NextIterator {
  child_nodes: Rc<RefCell<Vec<Handle>>>,
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
    let borrow: &RefCell<_> = self.child_nodes.borrow();
    let borrow: Ref<_> = borrow.borrow();
    let last = borrow.len() - 1;

    if self.index == last {
      None
    } else {
      self.index -= 1;

      let data: &NodeData = borrow[self.index].borrow();
      match data {
        NodeData::Comment(ref comment) => Some(Either4::A(comment.downgrade())),
        NodeData::DocType(ref doc_type) => Some(Either4::B(doc_type.downgrade())),
        NodeData::Element(ref element) => Some(Either4::C(element.downgrade())),
        NodeData::Text(ref text) => Some(Either4::D(text.downgrade())),
        _ => unreachable!(),
      }
    }
  }
}
