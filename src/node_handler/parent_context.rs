use napi::{
  bindgen_prelude::{Either, Reference, WeakReference},
  Env, Error, Result,
};

use crate::{Document, Element, Handle};

pub(crate) struct ParentContext {
  pub(crate) node: Either<WeakReference<Document>, WeakReference<Element>>,
  pub(crate) index: usize,
  pub(crate) env: Env,
}

impl ParentContext {
  pub(crate) fn new(
    env: Env,
    node: Either<WeakReference<Document>, WeakReference<Element>>,
    index: usize,
  ) -> Self {
    ParentContext { env, node, index }
  }

  pub(crate) fn get_handle(&self) -> Result<Handle> {
    let handle: Handle = match &self.node {
      Either::A(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
      Either::B(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
    };
    Ok(handle)
  }
}

impl TryInto<Handle> for &ParentContext {
  type Error = Error;

  fn try_into(self) -> Result<Handle> {
    self.get_handle()
  }
}

impl TryInto<Either<Reference<Document>, Reference<Element>>> for &ParentContext {
  type Error = Error;

  fn try_into(self) -> Result<Either<Reference<Document>, Reference<Element>>> {
    let handle = self.get_handle()?;
    let reference: Either<Reference<Document>, Reference<Element>> = match handle {
      Handle::Document(document) => Either::A(document),
      Handle::Element(element) => Either::B(element),
      _ => panic!("Invalid handle"),
    };
    Ok(reference)
  }
}
