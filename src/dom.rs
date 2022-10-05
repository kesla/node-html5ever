use crate::{serialize, Comment, DocType, Document, Element, Handle, QuirksMode, Text};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{NodeOrText, TreeSink};
use napi::{bindgen_prelude::Reference, Env, Result};

#[napi]
pub struct Html5everDom {
  document_reference: Reference<Document>,
  document_handle: Handle,

  #[napi(writable = false)]
  pub quirks_mode: QuirksMode,

  #[napi(writable = false)]
  pub errors: Vec<String>,

  env: Env,
}

#[napi]
impl Html5everDom {
  #[napi(constructor)]
  pub fn new(env: Env, html: String) -> Result<Html5everDom> {
    let document_reference = Document::new_reference(env)?;
    let sink = Html5everDom {
      document_handle: document_reference.get_handle(),
      document_reference,
      quirks_mode: QuirksMode::NoQuirks,
      errors: vec![],
      env,
    };

    let dom: Html5everDom = html5ever::parse_document(sink, Default::default()).one(html);

    Ok(dom)
  }

  #[napi(getter)]
  pub fn document(&mut self) -> Result<Reference<Document>> {
    self.document_reference.clone(self.env)
  }

  #[napi]
  #[must_use]
  pub fn serialize(&self) -> String {
    serialize(
      self.document_handle.clone(),
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }
}

#[allow(unused_variables)]
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
    let element: &Reference<Element> = target.as_element().unwrap();
    element.name.expanded()
  }

  fn create_element(
    &mut self,
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    // TODO: set flags
    _flags: html5ever::tree_builder::ElementFlags,
  ) -> Self::Handle {
    let r = Element::new_reference(self.env, attrs.into(), name).unwrap();
    r.get_handle()
  }

  fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
    let r = Comment::new_reference(self.env, text.to_string()).unwrap();
    r.get_handle()
  }

  fn create_pi(
    &mut self,
    target: html5ever::tendril::StrTendril,
    data: html5ever::tendril::StrTendril,
  ) -> Self::Handle {
    todo!()
  }

  fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
    let handle = match child {
      NodeOrText::AppendNode(handle) => handle,
      NodeOrText::AppendText(content) => {
        let r = Text::new_reference(self.env, content.to_string()).unwrap();
        r.get_handle()
      }
    };

    parent.append_handle(handle);
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
    let r = DocType::new_reference(
      self.env,
      name.to_string(),
      public_id.to_string(),
      system_id.to_string(),
    )
    .unwrap();
    let doc_type = r.get_handle();
    let child = NodeOrText::AppendNode(doc_type);
    let handle = self.get_document();
    self.append(&handle, child);
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
