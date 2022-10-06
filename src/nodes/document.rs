use html5ever::{Namespace, QualName};
use napi::{bindgen_prelude::Reference, Result};

use crate::{DocType, Element, Text};

#[create_node(has_children)]
pub struct Document {}

#[napi]
impl Document {
  #[napi(getter)]
  pub fn get_doc_type(&self) -> Result<Option<Reference<DocType>>> {
    if let Some(first) = self.get_handle().get_child_nodes().get(0) {
      if let Ok(doc_type) = first.as_doc_type() {
        return Ok(Some(doc_type.clone(self.env)?));
      }
    }

    Ok(None)
  }

  #[napi(getter)]
  pub fn get_document_element(&self) -> Result<Reference<Element>> {
    let handle = self.get_handle();
    let list = handle.get_child_nodes();
    let node = match list.len() {
      2 => list.get(1),
      _ => list.get(0),
    }
    .unwrap();
    let element = node.as_element()?;

    element.clone(self.env)
  }

  #[napi(getter)]
  pub fn get_head(&mut self) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    let handle = document_element.get_handle();
    let list = handle.get_child_nodes();
    list.get(0).unwrap().as_element()?.clone(self.env)
  }

  #[napi(getter)]
  pub fn get_body(&mut self) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    let handle = document_element.get_handle();
    let list = handle.get_child_nodes();
    list.get(1).unwrap().as_element()?.clone(self.env)
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    "#document".to_string()
  }

  #[napi]
  pub fn create_element(&mut self, name: String) -> Result<Reference<Element>> {
    Element::new_reference(
      self.env,
      vec![].into(),
      QualName::new(None, Namespace::from("html"), name.into()),
    )
  }

  #[napi]
  pub fn create_text_node(&mut self, data: String) -> Result<Reference<Text>> {
    Text::new_reference(self.env, data)
  }
}

impl Drop for Document {
  fn drop(&mut self) {
    println!("Dropping Document");
  }
}
