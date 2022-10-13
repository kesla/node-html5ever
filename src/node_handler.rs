use std::{
  cell::{Cell, Ref, RefCell, RefMut},
  ops::Deref,
  rc::Rc,
};

use napi::{Env, Error, Result};

use crate::{
  get_id, ChildNode, Comment, Document, DocumentFragment, DocumentType, Element, Node, ParentNode,
  Text,
};

mod child_node_list;
pub mod iterators;
mod parent_context;

pub use self::parent_context::ParentContext;

use self::{
  child_node_list::ChildNodeList,
  iterators::{ChildNodesIterator, ChildrenIterator, NextIterator, PrevIterator},
};

pub struct NodeHandlerInner {
  env: Env,
  id: usize,
  list: RefCell<ChildNodeList>,
  pub(crate) parent_context: Cell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct NodeHandler(Rc<NodeHandlerInner>);

impl Deref for NodeHandler {
  type Target = NodeHandlerInner;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl NodeHandler {
  pub(crate) fn new(env: Env) -> Self {
    NodeHandler(Rc::new(NodeHandlerInner {
      env,
      id: get_id(),
      list: Default::default(),
      parent_context: Cell::new(None),
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

  pub(crate) fn previous_iterator(&self) -> Result<PrevIterator> {
    let maybe_parent = self.parent_context.take();
    let input: Option<(NodeHandler, usize)> = match maybe_parent.as_ref() {
      Some(parent) => Some((parent.try_into()?, parent.index)),
      None => None,
    };
    self.parent_context.set(maybe_parent);

    Ok(PrevIterator::new(input))
  }

  pub(crate) fn next_iterator(&self) -> Result<NextIterator> {
    let maybe_parent = self.parent_context.take();
    let input: Option<(NodeHandler, usize)> = match maybe_parent.as_ref() {
      Some(parent) => Some((parent.try_into()?, parent.index)),
      None => None,
    };
    self.parent_context.set(maybe_parent);

    Ok(NextIterator::new(input))
  }

  pub(crate) fn child_nodes_iter(&self, deep: bool) -> ChildNodesIterator {
    ChildNodesIterator::new(self, deep)
  }

  pub(crate) fn children_iter(&self, deep: bool) -> ChildrenIterator {
    ChildrenIterator::new(self, deep)
  }
}

impl TryFrom<&ParentContext> for NodeHandler {
  type Error = Error;

  fn try_from(parent_context: &ParentContext) -> Result<Self> {
    match &parent_context.node {
      ParentNode::Document(document) => {
        let document = document
          .upgrade(parent_context.env)?
          .expect("Document is gone");
        Ok(document.get_node_handler())
      }
      ParentNode::DocumentFragment(document_fragment) => {
        let document_fragment = document_fragment
          .upgrade(parent_context.env)?
          .expect("DocumentFragment is gone");
        Ok(document_fragment.get_node_handler())
      }
      ParentNode::Element(element) => {
        let element = element
          .upgrade(parent_context.env)?
          .expect("Element is gone");
        Ok(element.get_node_handler())
      }
    }
  }
}

impl From<ChildNode> for NodeHandler {
  fn from(e: ChildNode) -> Self {
    match e {
      ChildNode::Comment(r) => r.get_node_handler(),
      ChildNode::DocumentType(r) => r.get_node_handler(),
      ChildNode::Element(r) => r.get_node_handler(),
      ChildNode::Text(r) => r.get_node_handler(),
    }
  }
}

impl From<Node> for NodeHandler {
  fn from(node: Node) -> Self {
    match node {
      Node::Comment(r) => r.get_node_handler(),
      Node::DocumentType(r) => r.get_node_handler(),
      Node::Document(r) => r.get_node_handler(),
      Node::DocumentFragment(r) => r.get_node_handler(),
      Node::Element(r) => r.get_node_handler(),
      Node::Text(r) => r.get_node_handler(),
    }
  }
}

impl From<&Node> for NodeHandler {
  fn from(node: &Node) -> Self {
    match node {
      Node::Comment(r) => r.get_node_handler(),
      Node::DocumentType(r) => r.get_node_handler(),
      Node::Document(r) => r.get_node_handler(),
      Node::DocumentFragment(r) => r.get_node_handler(),
      Node::Element(r) => r.get_node_handler(),
      Node::Text(r) => r.get_node_handler(),
    }
  }
}

impl PartialEq for NodeHandler {
  fn eq(&self, other: &Self) -> bool {
    self.0.id == other.0.id
  }
}

impl Eq for NodeHandler {}

macro_rules! impl_from {
  ($type:ty) => {
    impl From<&$type> for NodeHandler {
      fn from(value: &$type) -> Self {
        value.get_node_handler()
      }
    }
  };
}

impl_from!(Comment);
impl_from!(Document);
impl_from!(DocumentFragment);
impl_from!(DocumentType);
impl_from!(Element);
impl_from!(Text);
