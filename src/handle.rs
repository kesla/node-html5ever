use napi::{
  bindgen_prelude::{Reference, ToNapiValue, Either5},
  Env, Error, Result, Status,
};

use crate::{doc_type::DocType, document::Document, element::Element, text::Text, comment::Comment};

pub(crate) enum Inner {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

type EitherType =
  Either5<Reference<Comment>, Reference<DocType>, Reference<Document>, Reference<Element>, Reference<Text>>;

impl Into<EitherType> for Inner {
  fn into(self) -> EitherType {
    match self {
      Inner::Comment(i) => Either5::A(i),
      Inner::DocType(i) => Either5::B(i),
      Inner::Document(i) => Either5::C(i),
      Inner::Element(i) => Either5::D(i),
      Inner::Text(i) => Either5::E(i),
    }
  }
}

pub struct Handle {
  pub(crate) inner: Inner,
  pub(crate) env: Env,
}

impl ToNapiValue for Handle {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    Either5::to_napi_value(env, val.inner.into())
  }
}

impl Clone for Handle {
  fn clone(&self) -> Self {
    let cloned_inner = match &self.inner {
      Inner::Comment(r) => Inner::Comment(r.clone(self.env).unwrap()),
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

impl From<Reference<Comment>> for Handle {
  fn from(r: Reference<Comment>) -> Self {
    let env = r.env;
    let inner = Inner::Comment(r);
    Self { inner, env }
  }
}

impl From<Reference<Element>> for Handle {
  fn from(r: Reference<Element>) -> Self {
    let env = r.env;
    let inner = Inner::Element(r);
    Self { inner, env }
  }
}

impl From<Reference<Document>> for Handle {
  fn from(r: Reference<Document>) -> Self {
    let env = r.env;
    let inner = Inner::Document(r);
    Self { inner, env }
  }
}

impl From<Reference<DocType>> for Handle {
  fn from(r: Reference<DocType>) -> Self {
    let env = r.env;
    let inner = Inner::DocType(r);
    Self { inner, env }
  }
}

impl From<Reference<Text>> for Handle {
  fn from(r: Reference<Text>) -> Self {
    let env = r.env;
    let inner = Inner::Text(r);
    Self { inner, env }
  }
}

impl Handle {
  pub(crate) fn into_element(&self) -> Result<&Reference<Element>> {
    match &self.inner {
      Inner::Element(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not an Element".to_string())),
    }
  }

  pub(crate) fn into_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self.inner {
      Inner::DocType(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a DocType".to_string())),
    }
  }

  pub(crate) fn into_document(&self) -> Result<&Reference<Document>> {
    match &self.inner {
      Inner::Document(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a Document".to_string())),
    }
  }
}
