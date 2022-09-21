use crate::doc_type::DocType;
use crate::document::Document;
use crate::element::Element;
use crate::node::{Inner, Node};
use crate::quirks_mode::QuirksMode;
use crate::serialize::serialize;
use crate::text::Text;
use html5ever::tree_builder::{NodeOrText, TreeSink};
use napi::Either;
use napi::{bindgen_prelude::Reference, Env, Result};

#[napi]
pub struct Html5everDom {
  document: Node,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,

  env: Env,
}

#[napi]
impl Html5everDom {
  pub(crate) fn new(env: Env) -> Result<Html5everDom> {
    let document: Node = Document::new(env)?.into();

    Ok(Html5everDom {
      document,
      quirks_mode: QuirksMode::NoQuirks,
      errors: vec![],
      env,
    })
  }

  #[napi(getter)]
  pub fn document(&mut self, env: Env) -> Result<Reference<Document>> {
    let r = self.document.into_document()?;
    r.clone(env)
  }

  #[napi]
  pub fn serialize(&self) -> String {
    serialize(&self.document)
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
    let element: &Reference<Element> = target.into_element().unwrap();
    element.name.expanded()
  }

  fn create_element(
    &mut self,
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    // TODO: set flags
    _flags: html5ever::tree_builder::ElementFlags,
  ) -> Self::Handle {
    let element = Element::new(self.env, attrs, name).unwrap();
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
    // TODO: concatenate already existing text node
    let (mut list, parent_reference) = match &parent.inner {
      Inner::Element(r) => (
        r.list.clone(self.env).unwrap(),
        Some(Either::A(r.downgrade())),
      ),
      Inner::Document(r) => (
        r.list.clone(self.env).unwrap(),
        Some(Either::B(r.downgrade())),
      ),
      _ => panic!("Node does not have children"),
    };

    let mut node = match child {
      NodeOrText::AppendNode(node) => node,
      NodeOrText::AppendText(content) => Text::new(content.to_string(), self.env).unwrap().into(),
    };

    match &mut node.inner {
        Inner::DocType(doc_type) => doc_type.parent = parent_reference,
        Inner::Document(document) => (),
        Inner::Element(element) => element.parent = parent_reference,
        Inner::Text(text) => text.parent = parent_reference,
    }

    list.push(node);
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
