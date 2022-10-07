use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

use napi::{bindgen_prelude::Either4, Either, Env, Error, Result};

use crate::{get_id, Comment, DocType, Document, Element, Handle, Text};

mod child_node_list;
mod iterators;
mod node_reference;
mod parent_context;

pub use self::parent_context::ParentContext;

use self::{
  child_node_list::ChildNodeList,
  iterators::{NextIterator, PreviousIterator},
};

struct NodeHandlerInner {
  env: Env,
  id: usize,
  list: RefCell<ChildNodeList>,
  parent_context: RefCell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct NodeHandler(Rc<NodeHandlerInner>);

impl NodeHandler {
  pub(crate) fn new(env: Env) -> Self {
    NodeHandler(Rc::new(NodeHandlerInner {
      env,
      id: get_id(),
      list: Default::default(),
      parent_context: RefCell::new(None),
    }))
  }

  pub(crate) fn get_env(&self) -> Env {
    self.0.env
  }

  pub(crate) fn get_child_nodes(&self) -> Ref<ChildNodeList> {
    self.0.list.borrow()
  }

  pub(crate) fn get_child_nodes_mut(&self) -> RefMut<ChildNodeList> {
    self.0.list.borrow_mut()
  }

  pub(crate) fn get_parent(&self) -> Ref<Option<ParentContext>> {
    self.0.parent_context.borrow()
  }

  pub(crate) fn get_parent_mut(&self) -> RefMut<Option<ParentContext>> {
    self.0.parent_context.borrow_mut()
  }

  pub(crate) fn previous_iterator(&self) -> Result<PreviousIterator> {
    let maybe_parent_context = self.get_parent();
    let maybe_parent_context = maybe_parent_context.as_ref();

    match maybe_parent_context {
      Some(ctx) => Ok(PreviousIterator::Data {
        node_handler: ctx.try_into()?,
        index: ctx.index,
      }),
      None => Ok(PreviousIterator::None),
    }
  }

  pub(crate) fn next_iterator(&self) -> Result<NextIterator> {
    let maybe_parent_context = self.get_parent();
    let maybe_parent_context = maybe_parent_context.as_ref();

    match maybe_parent_context {
      Some(ctx) => Ok(NextIterator::Data {
        node_handler: ctx.try_into()?,
        index: ctx.index,
      }),
      None => Ok(NextIterator::None),
    }
  }
}

impl TryFrom<&ParentContext> for NodeHandler {
  type Error = Error;

  fn try_from(parent_context: &ParentContext) -> Result<Self> {
    match &parent_context.node {
      Either::A(document) => {
        let document = document
          .upgrade(parent_context.env)?
          .expect("Document is gone");
        Ok(document.get_node_handler())
      }
      Either::B(element) => {
        let element = element
          .upgrade(parent_context.env)?
          .expect("Element is gone");
        Ok(element.get_node_handler())
      }
    }
  }
}

impl From<Either<&Document, &Element>> for NodeHandler {
  fn from(e: Either<&Document, &Element>) -> Self {
    match e {
      Either::A(r) => r.into(),
      Either::B(r) => r.into(),
    }
  }
}

impl From<Either4<&Comment, &DocType, &Element, &Text>> for NodeHandler {
  fn from(e: Either4<&Comment, &DocType, &Element, &Text>) -> Self {
    match e {
      Either4::A(r) => r.into(),
      Either4::B(r) => r.into(),
      Either4::C(r) => r.into(),
      Either4::D(r) => r.into(),
    }
  }
}

impl From<Handle> for NodeHandler {
  fn from(handle: Handle) -> Self {
    match handle {
      Handle::Comment(r) => r.get_node_handler(),
      Handle::DocType(r) => r.get_node_handler(),
      Handle::Document(r) => r.get_node_handler(),
      Handle::Element(r) => r.get_node_handler(),
      Handle::Text(r) => r.get_node_handler(),
    }
  }
}

impl From<&Handle> for NodeHandler {
  fn from(handle: &Handle) -> Self {
    match handle {
      Handle::Comment(r) => r.get_node_handler(),
      Handle::DocType(r) => r.get_node_handler(),
      Handle::Document(r) => r.get_node_handler(),
      Handle::Element(r) => r.get_node_handler(),
      Handle::Text(r) => r.get_node_handler(),
    }
  }
}

impl PartialEq for NodeHandler {
  fn eq(&self, other: &Self) -> bool {
    self.0.id == other.0.id
  }
}

impl Eq for NodeHandler {}