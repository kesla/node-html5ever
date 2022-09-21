use napi::{
  bindgen_prelude::{Reference, ToNapiValue},
  Either, Env, Error, Result, Status,
};

use crate::{doc_type::DocType, element::Element};

type Inner = Either<Reference<DocType>, Reference<Element>>;

pub struct Node {
  pub(crate) inner: Inner,
  env: Env,
}

impl ToNapiValue for Node {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    Inner::to_napi_value(env, val.inner)
  }
}

impl Clone for Node {
  fn clone(&self) -> Self {
    // Self { inner: self.inner.clone(), env: self.env.clone() }
    let cloned_inner = match &self.inner {
      Inner::A(r) => Inner::A(r.clone(self.env).unwrap()),
      Inner::B(r) => Inner::B(r.clone(self.env).unwrap()),
    };

    Self {
      inner: cloned_inner,
      env: self.env.clone(),
    }
  }
}

impl Node {
  pub fn as_element(&self) -> Result<Reference<Element>> {
    match &self.inner {
      Either::B(r) => r.clone(self.env),
      _ => Err(Error::new(Status::InvalidArg, "not an Element".to_string())),
    }
  }

  pub fn as_doc_type(&self) -> Result<Reference<DocType>> {
    match &self.inner {
      Either::A(r) => r.clone(self.env),
      _ => Err(Error::new(Status::InvalidArg, "not a DocType".to_string())),
    }
  }
}
