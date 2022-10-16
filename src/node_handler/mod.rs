use std::{ops::Deref, rc::Rc};

use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::{
  ChildNode, ChildNodesIterator, Comment, Document, DocumentFragment, DocumentType, EinarCell,
  Element, Node, ParentNode, SiblingIterator, SiblingIteratorType, Text,
};

mod child_node_list;
mod parent_context;

pub use self::parent_context::ParentContext;

use self::child_node_list::ChildNodeList;

pub struct NodeHandlerInner {
  pub(crate) env: Env,
  pub(crate) child_nodes: EinarCell<ChildNodeList>,
  pub(crate) parent_context: EinarCell<Option<ParentContext>>,
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
      child_nodes: Default::default(),
      parent_context: Default::default(),
    }))
  }

  pub(crate) fn previous_iterator<T>(&self) -> Result<SiblingIterator<T>> {
    let input: Option<(NodeHandler, usize)> =
      self
        .parent_context
        .borrow::<_, Result<_>>(|maybe_parent| match maybe_parent.as_ref() {
          Some(parent) => Ok(Some((parent.try_into()?, parent.index))),
          None => Ok(None),
        })?;

    Ok(SiblingIterator::new(input, SiblingIteratorType::Previous))
  }

  pub(crate) fn next_iterator<T>(&self) -> Result<SiblingIterator<T>> {
    self
      .parent_context
      .borrow(|maybe_parent: &Option<ParentContext>| {
        let input: Option<(NodeHandler, usize)> = match maybe_parent.as_ref() {
          Some(parent) => Some((parent.try_into()?, parent.index)),
          None => None,
        };

        Ok(SiblingIterator::new(input, SiblingIteratorType::Next))
      })
  }

  pub(crate) fn try_get_child_node<T, U>(&self, index: usize) -> std::result::Result<Option<T>, U>
  where
    ChildNode: TryInto<T, Error = U>,
  {
    self.child_nodes.borrow(|child_nodes| {
      let child_node = child_nodes.get(index).cloned();

      let result = if let Some(child_node) = child_node {
        Some(child_node.try_into()?)
      } else {
        None
      };

      Ok(result)
    })
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
    self.child_nodes.borrow_mut(|child_nodes| {
      child_nodes.append_node(child);
    });
  }

  pub(crate) fn remove_node(&self, child: &ChildNode) {
    self.child_nodes.borrow_mut(|child_nodes| {
      child_nodes.remove_node(child);
    });
  }

  pub(crate) fn child_nodes_len(&self) -> usize {
    self.child_nodes.borrow(|child_nodes| child_nodes.len())
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
