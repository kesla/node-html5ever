use std::{
  cell::RefMut,
  fmt::{Debug, Formatter},
  ops::Deref,
};

use crate::{
  Comment, Document, DocumentFragment, DocumentType, Element, NodeHandler, ParentContext, Text,
};
use napi::{
  bindgen_prelude::{Either3, Error, FromNapiValue, Reference, Result, ToNapiValue, WeakReference},
  Status,
};

pub enum Handle {
  Comment(Reference<Comment>),
  DocumentType(Reference<DocumentType>),
  Document(Reference<Document>),
  DocumentFragment(Reference<DocumentFragment>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

pub enum ChildNode {
  Comment(Reference<Comment>),
  DocumentType(Reference<DocumentType>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

impl ToNapiValue for ChildNode {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    match val {
      ChildNode::Comment(r) => Reference::<Comment>::to_napi_value(env, r),
      ChildNode::DocumentType(r) => Reference::<DocumentType>::to_napi_value(env, r),
      ChildNode::Element(r) => Reference::<Element>::to_napi_value(env, r),
      ChildNode::Text(r) => Reference::<Text>::to_napi_value(env, r),
    }
  }
}

impl FromNapiValue for ChildNode {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    println!("from_napi_value");
    use napi::bindgen_prelude::ValidateNapiValue;
    if <&Element>::validate(env, napi_val).is_ok() {
      <&Element>::from_napi_value(env, napi_val).map(|r| r.into())
    } else if <&Text>::validate(env, napi_val).is_ok() {
      <&Text>::from_napi_value(env, napi_val).map(|r| r.into())
    } else if <&Comment>::validate(env, napi_val).is_ok() {
      <&Comment>::from_napi_value(env, napi_val).map(|r| r.into())
    } else if <&DocumentType>::validate(env, napi_val).is_ok() {
      <&DocumentType>::from_napi_value(env, napi_val).map(|r| r.into())
    } else {
      Err(Error::new(
        Status::InvalidArg,
        "Could not convert napi_value to ChildNode (Element, Text, Comment or DocumentType)"
          .to_string(),
      ))
    }
  }
}

impl Debug for ChildNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "ChildNode(")?;
    match self {
      ChildNode::Comment(r) => write!(f, "{:?}", r.deref()),
      ChildNode::DocumentType(r) => write!(f, "{:?}", r.deref()),
      ChildNode::Element(r) => write!(f, "{:?}", r.deref()),
      ChildNode::Text(r) => write!(f, "{:?}", r.deref()),
    }?;
    write!(f, ")")
  }
}

macro_rules! impl_from_child_node {
  ($type:ty, $variant:ident) => {
    impl From<&$type> for ChildNode {
      fn from(value: &$type) -> Self {
        ChildNode::$variant(
          value
            .weak_reference
            .as_ref()
            .unwrap()
            .upgrade(value.env)
            .unwrap()
            .unwrap(),
        )
      }
    }

    impl From<Reference<$type>> for ChildNode {
      fn from(value: Reference<$type>) -> Self {
        ChildNode::$variant(value)
      }
    }
  };
}

impl_from_child_node!(Comment, Comment);
impl_from_child_node!(DocumentType, DocumentType);
impl_from_child_node!(Element, Element);
impl_from_child_node!(Text, Text);

impl From<ChildNode> for Handle {
  fn from(value: ChildNode) -> Self {
    match value {
      ChildNode::Comment(r) => Handle::Comment(r),
      ChildNode::DocumentType(r) => Handle::DocumentType(r),
      ChildNode::Element(r) => Handle::Element(r),
      ChildNode::Text(r) => Handle::Text(r),
    }
  }
}

impl From<&Handle>
  for Either3<WeakReference<Document>, WeakReference<DocumentFragment>, WeakReference<Element>>
{
  fn from(val: &Handle) -> Self {
    match val {
      Handle::Document(document) => Either3::A(document.downgrade()),
      Handle::DocumentFragment(document_fragment) => Either3::B(document_fragment.downgrade()),
      Handle::Element(element) => Either3::C(element.downgrade()),
      _ => panic!("Invalid handle"),
    }
  }
}

impl From<&Handle> for ChildNode {
  fn from(val: &Handle) -> Self {
    match val {
      Handle::Comment(r) => ChildNode::Comment(r.clone(r.env).unwrap()),
      Handle::DocumentType(r) => ChildNode::DocumentType(r.clone(r.env).unwrap()),
      Handle::Element(r) => ChildNode::Element(r.clone(r.env).unwrap()),
      Handle::Text(r) => ChildNode::Text(r.clone(r.env).unwrap()),
      Handle::Document(_) => panic!("Document is not a Node"),
      &Handle::DocumentFragment(_) => panic!("DocumentFragment is not a Node"),
    }
  }
}

macro_rules! impl_from {
  ($type:ty, $variant:ident) => {
    impl From<&$type> for Handle {
      fn from(value: &$type) -> Self {
        Handle::$variant(
          value
            .weak_reference
            .as_ref()
            .unwrap()
            .upgrade(value.env)
            .unwrap()
            .unwrap(),
        )
      }
    }

    impl From<Reference<$type>> for Handle {
      fn from(value: Reference<$type>) -> Self {
        Handle::$variant(value)
      }
    }
  };
}

impl_from!(Comment, Comment);
impl_from!(DocumentType, DocumentType);
impl_from!(Document, Document);
impl_from!(DocumentFragment, DocumentFragment);
impl_from!(Element, Element);
impl_from!(Text, Text);

impl PartialEq for Handle {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Comment(left), Self::Comment(right)) => left.id == right.id,
      (Self::DocumentType(left), Self::DocumentType(right)) => left.id == right.id,
      (Self::Document(left), Self::Document(right)) => left.id == right.id,
      (Self::DocumentFragment(left), Self::DocumentFragment(right)) => left.id == right.id,
      (Self::Element(left), Self::Element(right)) => left.id == right.id,
      (Self::Text(left), Self::Text(right)) => left.id == right.id,
      _ => false,
    }
  }
}

impl Eq for Handle {}

impl Clone for Handle {
  fn clone(&self) -> Self {
    match self {
      Self::Comment(arg0) => Self::Comment(arg0.clone(arg0.env).unwrap()),
      Self::DocumentType(arg0) => Self::DocumentType(arg0.clone(arg0.env).unwrap()),
      Self::Document(arg0) => Self::Document(arg0.clone(arg0.env).unwrap()),
      Self::DocumentFragment(arg0) => Self::DocumentFragment(arg0.clone(arg0.env).unwrap()),
      Self::Element(arg0) => Self::Element(arg0.clone(arg0.env).unwrap()),
      Self::Text(arg0) => Self::Text(arg0.clone(arg0.env).unwrap()),
    }
  }
}

impl Handle {
  pub(crate) fn as_element(&self) -> Result<&Reference<Element>> {
    match &self {
      Handle::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn as_doc_type(&self) -> Result<&Reference<DocumentType>> {
    match &self {
      Handle::DocumentType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocumentType".to_string(),
      )),
    }
  }

  pub(crate) fn append_handle(&self, child_handle: &Handle) -> Result<()> {
    // remove from old parent
    {
      child_handle.remove()?;
    }
    // TODO: concatenate already existing text node

    let node_handler = NodeHandler::from(self);
    let mut children = node_handler.get_child_nodes_mut();
    children.append_handle(child_handle);

    let parent_reference: Either3<
      WeakReference<Document>,
      WeakReference<DocumentFragment>,
      WeakReference<Element>,
    > = self.into();

    let parent_context = Some(ParentContext::new(
      node_handler.get_env(),
      parent_reference,
      children.len() - 1,
    ));
    let node_handler = NodeHandler::from(child_handle);
    let mut parent = node_handler.get_parent_mut();
    *parent = parent_context;
    Ok(())
  }

  pub(crate) fn remove_handle(&self, child_handle: &Handle) {
    let child_node_handler: NodeHandler = child_handle.into();
    let parent = child_node_handler.get_parent_mut();

    remove_handle(self, parent, child_handle);
  }

  pub(crate) fn remove(&self) -> Result<()> {
    let node_handler = NodeHandler::from(self);
    let maybe_parent = node_handler.get_parent_mut();

    match maybe_parent.as_ref() {
      Some(parent) => remove_handle(&parent.try_into()?, maybe_parent, self),
      None => {}
    }

    Ok(())
  }

  pub(crate) fn get_node_name(&self) -> String {
    match self {
      Handle::Comment(_) => "#comment".to_string(),
      Handle::DocumentType(_) => "#docType".to_string(),
      Handle::Document(_) => "#document".to_string(),
      Handle::DocumentFragment(_) => "#document-fragment".to_string(),
      Handle::Element(r) => r.name.local.to_string().to_uppercase(),
      Handle::Text(_) => "#text".to_string(),
    }
  }
}

fn remove_handle(parent: &Handle, mut parent_ref: RefMut<Option<ParentContext>>, child: &Handle) {
  let parent_node_handler: NodeHandler = parent.into();

  let mut children = parent_node_handler.get_child_nodes_mut();
  children.remove_handle(child);

  *parent_ref = None;
}
