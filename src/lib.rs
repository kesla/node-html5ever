#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod comment;
mod doc_type;
mod document;
mod dom;
mod element;
mod node;
mod id;
mod quirks_mode;
mod serialize;
mod text;

use dom::Html5everDom;
use html5ever::tendril::TendrilSink;
use napi::{Env, Result};

#[napi]
pub fn parse(html: String, env: Env) -> Result<Html5everDom> {
  let sink = Html5everDom::new(env)?;
  let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

  Ok(dom)
}
