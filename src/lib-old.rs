mod node;
mod node_list;
mod serialize;

use std::{borrow::Borrow, cell::RefCell};

use html5ever::{tendril::TendrilSink, tree_builder::TreeSink, Attribute, QualName};
use napi::{bindgen_prelude::*, Result};

use node::Node;
use node_list::NodeList;
use serialize::serialize;

#[macro_use]
extern crate napi_derive;

#[napi]
pub enum QuirksMode {
  Quirks,
  LimitedQuirks,
  NoQuirks,
}

#[napi]
pub struct Document {
  document_element: Option<Reference<Element>>,
}

#[napi]
pub struct Element {
  attrs: RefCell<Vec<Attribute>>,
  children: Reference<NodeList>,
  name: QualName
}

#[napi]
impl Element {
  #[napi]
  pub fn get_attribute(&self, key: String) -> Option<String> {
    let b = self.attrs.borrow();
    let mut iter = b.iter();
    while let Some(attr) = iter.next() {
      if attr.name.local == key {
        return Some(attr.value.to_string());
      }
    }

    None
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    self.name.local.to_string().to_uppercase()
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> String {
    self.node_name()
  }

  #[napi(getter)]
  pub fn get_children(&self, env: Env) -> Result<Reference<NodeList>> {
    self.children.clone(env)
  }

  #[napi(getter)]
  pub fn outer_html(&self, reference: Reference<Element>, env: Env) -> String {
    let node = Node { inner: Either::B(reference), env };
    serialize(node)
  }
}

#[inline]
fn lazy<T, F: FnOnce() -> Result<Reference<T>>>(
  option: &mut Option<Reference<T>>,
  env: Env,
  create: F,
) -> Result<Reference<T>> {
  match option {
    Some(value) => value.clone(env),
    None => {
      let r = create()?;
      *option = Some(r.clone(env)?);

      r.clone(env)
    }
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

  document: Node,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,
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
    target.as_element().unwrap()

    // self.rc_dom.elem_name(target)
  }

  fn create_element(
    &mut self,
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    flags: html5ever::tree_builder::ElementFlags,
  ) -> Self::Handle {
    self.rc_dom.create_element(name, attrs, flags)
  }

  fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
    self.rc_dom.create_comment(text)
  }

  fn create_pi(
    &mut self,
    target: html5ever::tendril::StrTendril,
    data: html5ever::tendril::StrTendril,
  ) -> Self::Handle {
    self.rc_dom.create_pi(target, data)
  }

  fn append(
    &mut self,
    parent: &Self::Handle,
    child: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    self.rc_dom.append(parent, child)
  }

  fn append_based_on_parent_node(
    &mut self,
    element: &Self::Handle,
    prev_element: &Self::Handle,
    child: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    self
      .rc_dom
      .append_based_on_parent_node(element, prev_element, child)
  }

  fn append_doctype_to_document(
    &mut self,
    name: html5ever::tendril::StrTendril,
    public_id: html5ever::tendril::StrTendril,
    system_id: html5ever::tendril::StrTendril,
  ) {
    self
      .rc_dom
      .append_doctype_to_document(name, public_id, system_id)
  }

  fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
    self.rc_dom.get_template_contents(target)
  }

  fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
    self.rc_dom.same_node(x, y)
  }

  fn set_quirks_mode(&mut self, mode: html5ever::tree_builder::QuirksMode) {
    self.quirks_mode = match mode {
      html5ever::tree_builder::QuirksMode::Quirks => QuirksMode::Quirks,
      html5ever::tree_builder::QuirksMode::LimitedQuirks => QuirksMode::LimitedQuirks,
      html5ever::tree_builder::QuirksMode::NoQuirks => QuirksMode::NoQuirks,
    }
  }

  fn append_before_sibling(
    &mut self,
    sibling: &Self::Handle,
    new_node: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    self.rc_dom.append_before_sibling(sibling, new_node)
  }

  fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
    self.rc_dom.add_attrs_if_missing(target, attrs)
  }

  fn remove_from_parent(&mut self, target: &Self::Handle) {
    self.rc_dom.remove_from_parent(target)
  }

  fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
    self.rc_dom.reparent_children(node, new_parent)
  }
}

#[napi]
impl Html5everDom {
  fn new(env: Env) -> Result<Self> {
    let mut rc_dom: RcDom = RcDom::default();
    Ok(Self {
      document: Document::new(&mut rc_dom, env)?,
      rc_dom,
      quirks_mode: QuirksMode::NoQuirks,
      errors: vec![],
    })
  }

  #[napi(getter)]
  pub fn document(&mut self, env: Env) -> Result<Reference<Document>> {
    self.document.clone(env)
  }

  #[napi]
  pub fn serialize(&self) -> String {
    se
  }
}

#[napi]
pub fn parse_document(html: String, env: Env) -> Result<Html5everDom> {
  let sink = Html5everDom::new(env)?;
  let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

  Ok(dom)
}
