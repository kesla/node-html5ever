use napi::{Env, bindgen_prelude::{Reference, WeakReference}, Result, Either};

use crate::{element::Element, document::Document};

#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Text {
  pub(crate) content: String,
}

#[napi]
impl Text {
  pub(crate) fn new(env: Env, content: String) -> Result<Reference<Self>> {
    Self::new_reference(env, content)
  }
}