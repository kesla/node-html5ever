use std::convert::TryInto;

use napi::{
  bindgen_prelude::{Either4, Reference, ToNapiValue},
  Either, Env, Error, Result, Status,
};

use crate::{doc_type::DocType, document::Document, element::Element, text::Text};

pub(crate) enum Inner {
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

type EitherType =
  Either4<Reference<DocType>, Reference<Document>, Reference<Element>, Reference<Text>>;

impl Into<EitherType> for Inner {
  fn into(self) -> EitherType {
    match self {
      Inner::DocType(i) => Either4::A(i),
      Inner::Document(i) => Either4::B(i),
      Inner::Element(i) => Either4::C(i),
      Inner::Text(i) => Either4::D(i),
    }
  }
}

pub struct Node {
  pub(crate) inner: Inner,
  env: Env,
}

impl ToNapiValue for Node {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    Either4::to_napi_value(env, val.inner.into())
  }
}

impl Clone for Node {
  fn clone(&self) -> Self {
    // Self { inner: self.inner.clone(), env: self.env.clone() }
    let cloned_inner = match &self.inner {
      Inner::DocType(r) => Inner::DocType(r.clone(self.env).unwrap()),
      Inner::Document(r) => Inner::Document(r.clone(self.env).unwrap()),
      Inner::Element(r) => Inner::Element(r.clone(self.env).unwrap()),
      Inner::Text(r) => Inner::Text(r.clone(self.env).unwrap()),
    };

    Self {
      inner: cloned_inner,
      env: self.env.clone(),
    }
  }
}

impl From<Reference<Element>> for Node {
  fn from(r: Reference<Element>) -> Self {
    let env = r.env;
    let inner = Inner::Element(r);
    Self { inner, env }
  }
}

impl From<Reference<Document>> for Node {
  fn from(r: Reference<Document>) -> Self {
    let env = r.env;
    let inner = Inner::Document(r);
    Self { inner, env }
  }
}

impl From<Reference<DocType>> for Node {
  fn from(r: Reference<DocType>) -> Self {
    let env = r.env;
    let inner = Inner::DocType(r);
    Self { inner, env }
  }
}

impl From<Reference<Text>> for Node {
  fn from(r: Reference<Text>) -> Self {
    let env = r.env;
    let inner = Inner::Text(r);
    Self { inner, env }
  }
}

impl Node {
  pub fn into_element(&self) -> Result<&Reference<Element>> {
    match &self.inner {
      Inner::Element(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not an Element".to_string())),
    }
  }

  pub fn into_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self.inner {
      Inner::DocType(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a DocType".to_string())),
    }
  }

  pub fn into_document(&self) -> Result<&Reference<Document>> {
    match &self.inner {
      Inner::Document(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a Document".to_string())),
    }
  }
}
