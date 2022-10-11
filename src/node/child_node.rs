use std::{
  fmt::{Debug, Formatter},
  ops::Deref,
};

use crate::{Comment, DocumentType, Element, Node, Text};
use napi::{
  bindgen_prelude::{Error, FromNapiValue, Reference, Result, ToNapiValue},
  Status,
};

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

macro_rules! impl_from {
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

impl_from!(Comment, Comment);
impl_from!(DocumentType, DocumentType);
impl_from!(Element, Element);
impl_from!(Text, Text);

impl From<&Node> for ChildNode {
  fn from(val: &Node) -> Self {
    match val {
      Node::Comment(r) => ChildNode::Comment(r.clone(r.env).unwrap()),
      Node::DocumentType(r) => ChildNode::DocumentType(r.clone(r.env).unwrap()),
      Node::Element(r) => ChildNode::Element(r.clone(r.env).unwrap()),
      Node::Text(r) => ChildNode::Text(r.clone(r.env).unwrap()),
      Node::Document(_) => panic!("Document is not a Node"),
      Node::DocumentFragment(_) => panic!("DocumentFragment is not a Node"),
    }
  }
}
