use napi::{Env, bindgen_prelude::{Reference, WeakReference}, Result, Either};

use crate::{element::Element, document::Document, parent::clone_parent_node};

#[napi]
pub struct Text {
  pub(crate) content: String,
  pub(crate) env: Env,
  pub(crate) parent: Option<Either<WeakReference<Element>, WeakReference<Document>>>,
}

#[napi]
impl Text {
  pub(crate) fn new(content: String, env: Env) -> Result<Reference<Self>> {
    Self::into_reference(Self {content, env, parent: None}, env)
  }

  #[napi(getter)]
  pub fn get_parent_node(&self) -> Option<Either<WeakReference<Element>, WeakReference<Document>>> {
    clone_parent_node(self.parent.as_ref())
  }
}