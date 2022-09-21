#[macro_use]
extern crate napi_derive;

mod doc_type;
mod document;
mod element;
mod node;
mod node_list;
mod quirks_mode;
// mod serialize
mod text;

use doc_type::DocType;
use document::Document;
use element::Element;
use html5ever::{
  tendril::TendrilSink,
  tree_builder::{NodeOrText, TreeSink},
};
use napi::{Env, Result};
use node::{Inner, Node};
use quirks_mode::QuirksMode;
use text::Text;

#[napi]
pub struct Html5everDom {
  document: Node,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,

  env: Env,
}

impl Html5everDom {
  fn new(env: Env) -> Result<Html5everDom> {
    let document: Node = Document::new(env)?.into();

    Ok(Html5everDom {
      document,
      quirks_mode: QuirksMode::NoQuirks,
      errors: vec![],
      env,
    })
  }
}

impl TreeSink for Html5everDom {
  type Handle = Node;

  type Output = Self;

  fn finish(self) -> Self::Output {
    self
  }

  fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
    self.errors.push(msg.into_owned());
  }

  fn get_document(&mut self) -> Self::Handle {
    self.document.clone()
  }

  fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
    let element = target.into_element().unwrap();
    element.name.expanded()
  }

  fn create_element(
    &mut self,
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    // TODO: set flags
    _flags: html5ever::tree_builder::ElementFlags,
  ) -> Self::Handle {
    let element = Element::new(attrs, name, self.env).unwrap();
    element.into()
  }

  fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
    todo!()
  }

  fn create_pi(
    &mut self,
    target: html5ever::tendril::StrTendril,
    data: html5ever::tendril::StrTendril,
  ) -> Self::Handle {
    todo!()
  }

  fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
    // todo: concatenate already existing text node
    // let children = parent
    let mut list = match &parent.inner {
      Inner::Element(r) => r.list.clone(self.env).unwrap(),
      Inner::Document(r) => r.list.clone(self.env).unwrap(),
      _ => panic!("Node does not have children"),
    };

    let node = match child {
      NodeOrText::AppendNode(node) => node,
      NodeOrText::AppendText(content) => Text::new(content.to_string(), self.env).unwrap().into(),
    };
  }

  fn append_based_on_parent_node(
    &mut self,
    element: &Self::Handle,
    prev_element: &Self::Handle,
    child: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    todo!()
  }

  fn append_doctype_to_document(
    &mut self,
    name: html5ever::tendril::StrTendril,
    public_id: html5ever::tendril::StrTendril,
    system_id: html5ever::tendril::StrTendril,
  ) {
    let doc_type = DocType::new(
      name.to_string(),
      public_id.to_string(),
      system_id.to_string(),
      self.env,
    )
    .unwrap();
    let node: Node = doc_type.into();
    let child = NodeOrText::AppendNode(node);
    self.append(&self.document.clone(), child);
  }

  fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
    todo!()
  }

  fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
    todo!()
  }

  fn set_quirks_mode(&mut self, mode: html5ever::tree_builder::QuirksMode) {
    self.quirks_mode = mode.into()
  }

  fn append_before_sibling(
    &mut self,
    sibling: &Self::Handle,
    new_node: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    todo!()
  }

  fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
    todo!()
  }

  fn remove_from_parent(&mut self, target: &Self::Handle) {
    todo!()
  }

  fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
    todo!()
  }
}

#[napi]
pub fn parse_document(html: String, env: Env) -> Result<Html5everDom> {
  let sink = Html5everDom::new(env)?;
  let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

  Ok(dom)
}
