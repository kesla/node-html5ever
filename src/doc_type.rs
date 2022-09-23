use napi::{Env, bindgen_prelude::{Reference, WeakReference}, Result, Either};

use crate::{element::Element, document::Document};

#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct DocType {
  #[napi(writable = false)]
  pub name: String,

  #[napi(writable = false)]
  pub public_id: String,

  #[napi(writable = false)]
  pub system_id: String,

  pub(crate) env: Env,
}

#[napi]
impl DocType {
  pub(crate) fn new(name: String, public_id: String, system_id: String, env: Env) -> Result<Reference<Self>> {
    let s = Self {
      name, public_id, system_id, env, parent: None
    };
    Self::into_reference(s, env)
  }
}