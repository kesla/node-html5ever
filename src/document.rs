use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::{doc_type::DocType, element::Element, node_list::NodeList};

#[napi]
pub struct Document {
  list: Reference<NodeList>,
}

#[napi]
impl Document {
  fn new(env: Env) -> Result<Reference<Self>> {
    let document = Self {
      list: NodeList::new(env)?,
    };

    return Self::into_reference(document, env);
  }

  // #[napi(getter)]
  // pub fn get_doc_type(&self, env: Env) -> Option<Reference<DocType>> {
  //   if let Ok(Some(first)) = self.list.get(0, env) {
  //     if let Ok(doc_type) = first.as_doc_type() {
  //       return Some(doc_type);
  //     }
  //   }
  //   None
  // }

  // #[napi(getter)]
  // pub fn get_document_element(&self, env: Env) -> Result<Reference<Element>> {
  //   let element = match self.list.len() {
  //     2 => self.list.get(1),
  //     _ => self.list.get(0),
  //   }?
  //   .as_element();

  //   return element;
  // }

  // #[napi(getter)]
  // pub fn get_head(&mut self, env: Env) -> Result<Reference<Element>> {
  //   let document_element = self.get_document_element(env)?;

  //   document_element
  //     .get_children(env)?
  //     .get(0, env)?
  //     .ok_or_else(|| Error::from_reason("head element should exists"))
  // }

  // #[napi(getter)]
  // pub fn get_body(&mut self, env: Env) -> Result<Reference<Element>> {
  //   let document_element = self.get_document_element(env)?;

  //   document_element
  //     .get_children(env)?
  //     .get(1, env)?
  //     .ok_or_else(|| Error::from_reason("body element should exists"))
  // }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    "#document".to_string()
  }
}
