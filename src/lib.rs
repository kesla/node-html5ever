#![deny(clippy::all)]

use std::{
  borrow::Borrow,
  cell::RefCell,
  collections::HashSet,
  convert::{TryFrom, TryInto},
  rc::Rc,
};

use html5ever::{
  serialize,
  tendril::TendrilSink,
  tree_builder::{NodeOrText, TreeSink},
  Attribute,
};
use markup5ever_rcdom::{Node, NodeData, SerializableHandle};
use napi::{bindgen_prelude::*, Result};

#[macro_use]
extern crate napi_derive;

#[napi]
pub enum QuirksMode {
  Quirks,
  LimitedQuirks,
  NoQuirks,
}

type Handle = Rc<Node>;

#[napi]
pub struct Document {
  handle: Handle,
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
  let foo = handle.children.borrow();
  let foo2 = foo.iter().filter_map(|child| {
    let x: Handle = child.clone();
    let bar: Option<Element> = x.try_into().ok();
    bar
  });
  foo2.collect()
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

impl From<Handle> for Document {
  fn from(handle: Handle) -> Self {
    Document::new(handle)
  }
}

#[napi]
impl Document {
  pub fn new(handle: Handle) -> Self {
    Self { handle }
  }

  #[napi(getter)]
  pub fn doc_type(&self) -> Option<DocType> {
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
  pub fn document_element(&self) -> Element {
    let children = self.handle.children.borrow();
    match children.len() {
      2 => children.get(1),
      _ => children.get(0),
    }
    .unwrap()
    .clone()
    .try_into()
    .unwrap()
  }

  #[napi(getter)]
  pub fn head(&self) -> Element {
    self
      .document_element()
      .handle
      .children
      .borrow()
      .get(0)
      .unwrap()
      .clone()
      .try_into()
      .unwrap()
  }

  #[napi(getter)]
  pub fn body(&self) -> Element {
    self
      .document_element()
      .handle
      .children
      .borrow()
      .get(1)
      .unwrap()
      .clone()
      .try_into()
      .unwrap()
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
  document_handle: Handle,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,
}

impl Default for Html5everDom {
  fn default() -> Self {
    Self {
      document_handle: Node::new(NodeData::Document),
      quirks_mode: QuirksMode::NoQuirks,
      errors: vec![],
    }
  }
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
    self.document_handle.clone()
  }

  fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
    match target.data {
      NodeData::Element { ref name, .. } => name.expanded(),
      _ => panic!("not an element!"),
    }
  }

  fn create_element(
    &mut self,
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    flags: html5ever::tree_builder::ElementFlags,
  ) -> Self::Handle {
    Node::new(NodeData::Element {
      name,
      attrs: RefCell::new(attrs),
      template_contents: RefCell::new(if flags.template {
        Some(Node::new(NodeData::Document))
      } else {
        None
      }),
      mathml_annotation_xml_integration_point: flags.mathml_annotation_xml_integration_point,
    })
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

  fn append(
    &mut self,
    parent: &Self::Handle,
    child: html5ever::tree_builder::NodeOrText<Self::Handle>,
  ) {
    if let NodeOrText::AppendText(ref text) = child {
      if let Some(last) = parent.children.borrow().last() {
        if let NodeData::Text { ref contents } = last.data {
          contents.borrow_mut().push_slice(text);
          return;
        }
      }
    }

    let child_handle = match child {
      NodeOrText::AppendNode(node) => node,
      NodeOrText::AppendText(text) => Node::new(NodeData::Text {
        contents: RefCell::new(text),
      }),
    };
    parent.children.borrow_mut().push(child_handle);
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
    self
      .document_handle
      .children
      .borrow_mut()
      .push(Node::new(NodeData::Doctype {
        name,
        public_id,
        system_id,
      }));
  }

  fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
    todo!()
  }

  fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
    todo!()
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
    todo!()
  }

  fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
    let element: Element = target.try_into().unwrap();
    let mut existing = element.attrs.borrow_mut();
    let existing_names: HashSet<_> = existing.iter().map(|attr| attr.name.clone()).collect();

    existing.extend(
      attrs
        .into_iter()
        .filter(|attr| !existing_names.contains(&attr.name)),
    );
  }

  fn remove_from_parent(&mut self, target: &Self::Handle) {
    todo!()
  }

  fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
    todo!()
  }
}

#[napi]
impl Html5everDom {
  #[napi(getter)]
  pub fn document(&mut self) -> Document {
    self.get_document().into()
  }

  #[napi]
  pub fn serialize(&self) -> String {
    serialize_handle(self.document_handle.clone())
  }
}

fn serialize_handle(handle: Handle) -> String {
  let serializable_handle: SerializableHandle = handle.into();
  let mut serialized = Vec::new();
  serialize::serialize(&mut serialized, &serializable_handle, Default::default()).unwrap();

  String::from_utf8(serialized).unwrap()
}

#[napi]
pub fn parse_document(html: String) -> Result<Html5everDom> {
  let dom: Html5everDom =
    html5ever::parse_document(Html5everDom::default(), Default::default()).one(html);

  Ok(dom)
}
