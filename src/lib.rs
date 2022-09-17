#![deny(clippy::all)]

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use html5ever::{serialize, tendril::TendrilSink};
use markup5ever_rcdom::{Node, NodeData, RcDom, SerializableHandle};
use napi::{bindgen_prelude::*, Result};

#[macro_use]
extern crate napi_derive;

fn get_node_name(data: &NodeData) -> String {
  match data {
    NodeData::Document => "#document".to_string(),
    NodeData::Doctype { name, .. } => name.to_string(),
    NodeData::Text { contents } => todo!(),
    NodeData::Comment { contents } => todo!(),
    NodeData::Element {
      name,
      attrs,
      template_contents,
      mathml_annotation_xml_integration_point,
    } => todo!(),
    NodeData::ProcessingInstruction { target, contents } => todo!(),
  }
}

#[napi]
pub struct Document {
  handle: SharedReference<Html5everDom, &'static Rc<Node>>,
}

#[napi]
impl Document {
  pub fn new(handle: SharedReference<Html5everDom, &Rc<Node>>) -> Self {
    Self { handle }
  }

  #[napi(getter)]
  pub fn doc_type(&self) -> Option<DocType> {
    let children: &RefCell<Vec<Rc<Node>>> = &self.handle.children;

    if let Some(first) = children.borrow().get(0) {
      if let NodeData::Doctype {
        name,
        public_id,
        system_id,
      } = first.data.borrow()
      {
        return Some(DocType {
          name: name.to_string(),
          public_id: public_id.to_string(),
          system_id: system_id.to_string(),
        });
      }
    }
    None
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    get_node_name(&self.handle.data)
  }
}

#[napi]
pub struct DocType {
  #[napi(writable = false)]
  pub name: String,

  #[napi(writable = false)]
  pub public_id: String,

  #[napi(writable = false)]
  pub system_id: String,
}

#[napi]
pub struct Html5everDom {
  rc_dom: RcDom,
}

#[napi]
impl From<RcDom> for Html5everDom {
  fn from(rc_dom: RcDom) -> Self {
    Self { rc_dom }
  }
}

#[napi]
impl Html5everDom {
  #[napi]
  pub fn serialize(&self) -> String {
    let mut serialized = Vec::new();
    let document: SerializableHandle = self.rc_dom.document.clone().into();
    serialize::serialize(&mut serialized, &document, Default::default()).unwrap();

    String::from_utf8(serialized).unwrap()
  }

  #[napi(getter)]
  pub fn quirks_mode(&self) -> QuirksMode {
    match self.rc_dom.quirks_mode {
      html5ever::tree_builder::QuirksMode::Quirks => QuirksMode::Quirks,
      html5ever::tree_builder::QuirksMode::LimitedQuirks => QuirksMode::LimitedQuirks,
      html5ever::tree_builder::QuirksMode::NoQuirks => QuirksMode::NoQuirks,
    }
  }

  #[napi(getter)]
  pub fn document(&self, reference: Reference<Html5everDom>, env: Env) -> Result<Document> {
    let shared = reference.share_with(env, |dom| {
      let handle = &dom.rc_dom.document;

      Ok(handle)
    })?;

    Ok(Document::new(shared))
  }
}

#[napi]
pub enum QuirksMode {
  Quirks,
  LimitedQuirks,
  NoQuirks,
}

#[napi]
pub fn parse_document(html: String) -> Result<Html5everDom> {
  let dom: Html5everDom = html5ever::parse_document(RcDom::default(), Default::default())
    .one(html)
    .into();

  Ok(dom)
}
