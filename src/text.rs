use napi::{Env, bindgen_prelude::Reference, Result};

#[napi]
pub struct Text {
  pub(crate) content: String,
  pub(crate) env: Env
}

impl Text {
  pub(crate) fn new(content: String, env: Env) -> Result<Reference<Self>> {
    Self::into_reference(Self {content, env}, env)
  }
}