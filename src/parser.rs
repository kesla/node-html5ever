use html5ever::tendril::TendrilSink;
use napi::{Env, Result};

use crate::Html5everDom;

#[napi]
pub fn parse(html: String, env: Env) -> Result<Html5everDom> {
  let sink = Html5everDom::new(env)?;
  let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

  Ok(dom)
}
