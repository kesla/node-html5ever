use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::{
  doc_type::{self, DocType},
  element::Element,
  node_list::NodeList,
};

#[napi]
pub struct Document {
  pub(crate) list: Reference<NodeList>,
  pub(crate) env: Env,
}

#[napi]
impl Document {
  pub(crate) fn new(env: Env) -> Result<Reference<Self>> {
    let document = Self {
      list: NodeList::new(env)?,
      env,
    };

    return Self::into_reference(document, env);
  }

  #[napi(getter)]
  pub fn get_doc_type(&self) -> Result<Option<Reference<DocType>>> {
    if let Ok(first) = self.list.get(0) {
      if let Ok(doc_type) = first.into_doc_type() {
        return Ok(Some(doc_type.clone(self.env)?));
      }
    }

    Ok(None)
  }

  #[napi(getter)]
  pub fn get_document_element(&self) -> Result<Reference<Element>> {
    let node = match self.list.len() {
      2 => self.list.get(1),
      _ => self.list.get(0),
    }?;
    let element = node.into_element()?;

    return element.clone(self.env);
  }

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
