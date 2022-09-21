#[macro_use]
extern crate napi_derive;

mod doc_type;
mod document;
mod element;
mod node;
mod node_list;
mod quirks_mode;
// mod serialize;

use node::Node;
use quirks_mode::QuirksMode;

#[napi]
pub struct Html5everDom {
  document: Node,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,
}