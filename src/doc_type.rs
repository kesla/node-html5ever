use napi::{
  bindgen_prelude::{Reference, WeakReference},
  Either, Env, Result,
};

use crate::{document::Document, element::Element};

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
}
