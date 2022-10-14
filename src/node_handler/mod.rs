use std::{cell::Cell, ops::Deref, rc::Rc};

use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::{
  get_id, ChildNode, Comment, Document, DocumentFragment, DocumentType, Element, Node, ParentNode,
  Text,
};

mod child_node_list;
mod iterators;
mod parent_context;

pub use self::parent_context::ParentContext;

use self::{
  child_node_list::ChildNodeList,
  iterators::{ChildNodesIterator, SiblingIterator, SiblingIteratorType},
};

pub struct NodeHandlerInner {
  pub(crate) env: Env,
  id: usize,
  pub(crate) child_nodes: Cell<ChildNodeList>,
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
      child_nodes: Default::default(),
      parent_context: Default::default(),
    }))
  }

  pub(crate) fn previous_iterator<T>(&self) -> Result<SiblingIterator<T>> {
    let maybe_parent = self.parent_context.take();
    let input: Option<(NodeHandler, usize)> = match maybe_parent.as_ref() {
      Some(parent) => Some((parent.try_into()?, parent.index)),
      None => None,
    };
    self.parent_context.set(maybe_parent);

    Ok(SiblingIterator::new(input, SiblingIteratorType::Previous))
  }

  pub(crate) fn next_iterator<T>(&self) -> Result<SiblingIterator<T>> {
    let maybe_parent = self.parent_context.take();
    let input: Option<(NodeHandler, usize)> = match maybe_parent.as_ref() {
      Some(parent) => Some((parent.try_into()?, parent.index)),
      None => None,
    };
    self.parent_context.set(maybe_parent);

    Ok(SiblingIterator::new(input, SiblingIteratorType::Next))
  }

  pub(crate) fn try_get_child_node<T, U>(&self, index: usize) -> std::result::Result<Option<T>, U>
  where
    ChildNode: TryInto<T, Error = U>,
  {
    let child_nodes = self.child_nodes.take();
    let child_node = child_nodes.get(index).cloned();
    self.child_nodes.set(child_nodes);

    let result = if let Some(child_node) = child_node {
      Some(child_node.try_into()?)
    } else {
      None
    };

    Ok(result)
  }

  pub(crate) fn get_child_node<T, U>(&self, index: usize) -> Option<T>
  where
    ChildNode: TryInto<T, Error = U>,
  {
    match self.try_get_child_node(index) {
      Ok(Some(child_node)) => Some(child_node),
      _ => None,
    }
  }

  pub(crate) fn child_nodes_iter(&self, deep: bool) -> ChildNodesIterator<ChildNode> {
    ChildNodesIterator::new(self, deep)
  }

  pub(crate) fn children_iter(&self, deep: bool) -> ChildNodesIterator<Reference<Element>> {
    ChildNodesIterator::new(self, deep)
  }

  pub(crate) fn append_node(&self, child: &ChildNode) {
    let mut child_nodes = self.child_nodes.take();
    child_nodes.append_node(child);
    self.child_nodes.set(child_nodes);
  }

  pub(crate) fn remove_node(&self, child: &ChildNode) {
    let mut child_nodes = self.child_nodes.take();
    child_nodes.remove_node(child);
    self.child_nodes.set(child_nodes);
  }

  pub(crate) fn child_nodes_len(&self) -> usize {
    let child_nodes = self.child_nodes.take();
    let len = child_nodes.len();
    self.child_nodes.set(child_nodes);
    len
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

impl From<&ChildNode> for NodeHandler {
  fn from(e: &ChildNode) -> Self {
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
