#![deny(clippy::all)]

use std::{
  borrow::Borrow,
  cell::{Ref, RefCell},
  convert::{TryFrom, TryInto},
  rc::Rc,
};

use html5ever::{serialize, tendril::TendrilSink, tree_builder::TreeSink, Attribute};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use napi::{bindgen_prelude::*, Result};

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
  handle: Handle,
  document_element: Option<Reference<Element>>,
  head: Option<Reference<Element>>,
  body: Option<Reference<Element>>,
}

#[napi]
pub struct Element {
  attrs: RefCell<Vec<Attribute>>,
  handle: Handle,
}

impl TryFrom<Handle> for Element {
  type Error = Error;

  fn try_from(handle: Handle) -> Result<Self> {
    match handle.data.borrow() {
      NodeData::Element {
        name,
        attrs,
        template_contents,
        mathml_annotation_xml_integration_point,
      } => Ok(Element {
        handle: handle.clone(),
        attrs: attrs.clone(),
      }),
      _ => Err(Error::from_reason("Handle not an element!")),
    }
  }
}

impl TryFrom<&Handle> for Element {
  type Error = Error;

  fn try_from(handle: &Handle) -> Result<Self> {
    handle.clone().try_into()
  }
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
    get_node_name(&self.handle.data)
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> String {
    self.node_name()
  }

  #[napi(getter)]
  pub fn child_nodes(&self) -> Vec<Element> {
    get_child_nodes(self.handle.clone())
  }

  #[napi(getter)]
  pub fn outer_html(&self) -> String {
    serialize_handle(self.handle.clone())
  }
}

fn get_child_nodes(handle: Handle) -> Vec<Element> {
  let children = handle.children.borrow();
  children
    .iter()
    .filter_map(|child| {
      let x: Handle = child.clone();
      let bar: Option<Element> = x.try_into().ok();
      bar
    })
    .collect()
}

fn get_node_name(node_data: &NodeData) -> String {
  match node_data {
    NodeData::Document => "#document".to_string(),
    NodeData::Doctype { name, .. } => name.to_string(),
    NodeData::Text { contents } => todo!(),
    NodeData::Comment { contents } => todo!(),
    NodeData::Element { name, .. } => name.local.to_string().to_uppercase(),
    NodeData::ProcessingInstruction { target, contents } => todo!(),
  }
}

#[napi]
impl Document {
  fn new(rc_dom: &mut RcDom, env: Env) -> Result<Reference<Self>> {
    let handle = rc_dom.get_document();

    let document = Self {
      handle,
      document_element: None,
      head: None,
      body: None,
    };

    return Document::into_reference(document, env);
  }

  #[napi(getter)]
  pub fn get_doc_type(&self) -> Option<DocType> {
    let children = self.handle.children.borrow();

    if let Some(first) = children.get(0) {
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
  pub fn get_document_element(&mut self, env: Env) -> Result<Reference<Element>> {
    match &self.document_element {
      Some(document_element) => document_element.clone(env),
      None => {
        let document_element = {
          let children = self.handle.children.borrow();
          let element: Element = match children.len() {
            2 => children.get(1),
            _ => children.get(0),
          }
          .unwrap()
          .clone()
          .try_into()
          .unwrap();
          Element::into_reference(element, env)?
        };
        self.document_element = Some(document_element.clone(env)?);

        Ok(document_element)
      }
    }

    // self.document_element.clone(env)
  }

  #[napi(getter)]
  pub fn get_head(&mut self, env: Env) -> Result<Reference<Element>> {
    match &self.head {
      Some(head) => head.clone(env),
      None => {
        let head = self
          .get_document_element(env)?
          .handle
          .children
          .borrow()
          .get(0)
          .unwrap()
          .clone()
          .try_into()
          .unwrap();
        let head_reference = Element::into_reference(head, env)?;

        self.head = Some(head_reference.clone(env)?);

        Ok(head_reference)
      }
    }
  }

  #[napi(getter)]
  pub fn get_body(&mut self, env: Env) -> Result<Reference<Element>> {
    match &self.body {
      Some(body) => body.clone(env),
      None => {
        let body = self
          .get_document_element(env)?
          .handle
          .children
          .borrow()
          .get(1)
          .unwrap()
          .clone()
          .try_into()
          .unwrap();
        let body_reference = Element::into_reference(body, env)?;

        self.body = Some(body_reference.clone(env)?);

        Ok(body_reference)
      }
    }
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

  document: Reference<Document>,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,
}

impl TreeSink for Html5everDom {
  type Handle = Handle;

  type Output = Self;

  fn finish(self) -> Self::Output {
    self
  }

  fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
    self.errors.push(msg.into_owned());
  }

  fn get_document(&mut self) -> Self::Handle {
    self.document.handle.clone()
  }

  fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
    self.rc_dom.elem_name(target)
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
    serialize_handle(self.document.handle.clone())
  }
}

fn serialize_handle(handle: Handle) -> String {
  let serializable_handle: SerializableHandle = handle.into();
  let mut serialized = Vec::new();
  serialize::serialize(&mut serialized, &serializable_handle, Default::default()).unwrap();

  String::from_utf8(serialized).unwrap()
}

#[napi]
pub fn parse_document(html: String, env: Env) -> Result<Html5everDom> {
  let sink = Html5everDom::new(env)?;
  let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

  Ok(dom)
}
