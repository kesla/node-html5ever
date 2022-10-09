use napi::{
  bindgen_prelude::{Either3, Reference, WeakReference},
  Env, Error, Result,
};

use crate::{Document, DocumentFragment, Element, Handle};

pub struct ParentContext {
  pub(crate) node:
    Either3<WeakReference<Document>, WeakReference<DocumentFragment>, WeakReference<Element>>,
  pub(crate) index: usize,
  pub(crate) env: Env,
}

impl ParentContext {
  pub(crate) fn new(
    env: Env,
    node: Either3<WeakReference<Document>, WeakReference<DocumentFragment>, WeakReference<Element>>,
    index: usize,
  ) -> Self {
    ParentContext { env, node, index }
  }

  pub(crate) fn get_handle(&self) -> Result<Handle> {
    let handle: Handle = match &self.node {
      Either3::A(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
      Either3::B(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
      Either3::C(weak_reference) => weak_reference.upgrade(self.env)?.unwrap().into(),
    };
    Ok(handle)
  }

  pub(crate) fn is_document(&self) -> bool {
    matches!(&self.node, Either3::A(_))
  }
}

impl TryInto<Handle> for &ParentContext {
  type Error = Error;

  fn try_into(self) -> Result<Handle> {
    self.get_handle()
  }
}

impl TryInto<Either3<Reference<Document>, Reference<DocumentFragment>, Reference<Element>>>
  for &ParentContext
{
  type Error = Error;

  fn try_into(
    self,
  ) -> Result<Either3<Reference<Document>, Reference<DocumentFragment>, Reference<Element>>> {
    let handle = self.get_handle()?;
    let reference: Either3<Reference<Document>, Reference<DocumentFragment>, Reference<Element>> =
      match handle {
        Handle::Document(document) => Either3::A(document),
        Handle::DocumentFragment(document_fragment) => Either3::B(document_fragment),
        Handle::Element(element) => Either3::C(element),
        _ => panic!("Invalid handle"),
      };
    Ok(reference)
  }
}
