use napi::{Env, bindgen_prelude::Reference, Result};

#[napi]
pub struct DocType {
  #[napi(writable = false)]
  pub name: String,

  #[napi(writable = false)]
  pub public_id: String,

  #[napi(writable = false)]
  pub system_id: String,

  pub(crate) env: Env,
}

impl DocType {
  pub(crate) fn new(name: String, public_id: String, system_id: String, env: Env) -> Result<Reference<Self>> {
    let s = Self {
      name, public_id, system_id, env
    };
    Self::into_reference(s, env)
  }
}