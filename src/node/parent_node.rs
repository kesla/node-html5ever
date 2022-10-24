use std::fmt::{Debug, Formatter};

use crate::{Document, DocumentFragment, Element, Node};
use napi::{
  bindgen_prelude::{Error, FromNapiValue, Result, ToNapiValue, WeakReference},
  Either, Status,
};

#[derive(Clone)]
pub enum ParentNode {
  Document(WeakReference<Document>),
  DocumentFragment(WeakReference<DocumentFragment>),
  Element(WeakReference<Element>),
}

impl ToNapiValue for ParentNode {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    match val {
      ParentNode::Document(r) => WeakReference::<Document>::to_napi_value(env, r),
      ParentNode::DocumentFragment(r) => WeakReference::<DocumentFragment>::to_napi_value(env, r),
      ParentNode::Element(r) => WeakReference::<Element>::to_napi_value(env, r),
    }
  }
}

impl FromNapiValue for ParentNode {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    use napi::bindgen_prelude::ValidateNapiValue;
    if <&Element>::validate(env, napi_val).is_ok() {
      <&Element>::from_napi_value(env, napi_val).map(|r| r.into())
    } else if <&Document>::validate(env, napi_val).is_ok() {
      <&Document>::from_napi_value(env, napi_val).map(|r| r.into())
    } else if <&DocumentFragment>::validate(env, napi_val).is_ok() {
      <&DocumentFragment>::from_napi_value(env, napi_val).map(|r| r.into())
    } else {
      Err(Error::new(
        Status::InvalidArg,
        "Could not convert napi_value to ParentNode (Element, Document or DocumentFragment)"
          .to_string(),
      ))
    }
  }
}

impl Debug for ParentNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "ParentNode(")?;
    match self {
      ParentNode::Document(_) => write!(f, "Document"),
      ParentNode::DocumentFragment(_) => write!(f, "DocumentFragment"),
      ParentNode::Element(_) => write!(f, "Element"),
    }?;
    write!(f, ")")
  }
}

macro_rules! impl_from {
  ($type:ty, $variant:ident) => {
    impl From<&$type> for ParentNode {
      fn from(value: &$type) -> Self {
        ParentNode::$variant(value.cyclic_reference.get_weak().unwrap())
      }
    }

    impl From<WeakReference<$type>> for ParentNode {
      fn from(value: WeakReference<$type>) -> Self {
        ParentNode::$variant(value)
      }
    }

    impl TryInto<WeakReference<$type>> for ParentNode {
      type Error = Error;

      fn try_into(self) -> Result<WeakReference<$type>> {
        match self {
          ParentNode::$variant(r) => Ok(r),
          _ => Err(Error::new(
            Status::InvalidArg,
            format!("Could not convert {:?} to {}", self, stringify!($type)),
          )),
        }
      }
    }
  };
}

impl_from!(Document, Document);
impl_from!(DocumentFragment, DocumentFragment);
impl_from!(Element, Element);

impl From<&Node> for ParentNode {
  fn from(val: &Node) -> Self {
    match val {
      Node::Comment(_) => panic!("Comment cannot be a parent node"),
      Node::DocumentType(_) => panic!("DocumentType cannot be a parent node"),
      Node::Element(r) => ParentNode::Element(r.downgrade()),
      Node::Text(_) => panic!("Text nodes cannot be a parent node"),
      Node::Document(r) => ParentNode::Document(r.downgrade()),
      Node::DocumentFragment(r) => ParentNode::DocumentFragment(r.downgrade()),
    }
  }
}

impl TryInto<Either<WeakReference<Document>, WeakReference<DocumentFragment>>> for ParentNode {
  type Error = Error;

  fn try_into(self) -> Result<Either<WeakReference<Document>, WeakReference<DocumentFragment>>> {
    match self {
      ParentNode::Document(r) => Ok(Either::A(r.clone())),
      ParentNode::DocumentFragment(r) => Ok(Either::B(r.clone())),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!(
          "Could not convert {:?} to Document or DocumentFragment",
          self
        ),
      )),
    }
  }
}
