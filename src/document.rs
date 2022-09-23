use html5ever::{QualName, Namespace};
use napi::{bindgen_prelude::Reference, Env, Result};

use crate::{doc_type::DocType,element::Element, node_list::NodeList};



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
    let handle = match self.list.len() {
      2 => self.list.get(1),
      _ => self.list.get(0),
    }?;
    let element = handle.into_element()?;

    return element.clone(self.env);
  }

  #[napi(getter)]
  pub fn get_head(&mut self, env: Env) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    document_element.list.get(0)?.into_element()?.clone(env)
  }

  #[napi(getter)]
  pub fn get_body(&mut self, env: Env) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    document_element.list.get(1)?.into_element()?.clone(env)
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    "#document".to_string()
  }

  #[napi]
  pub fn create_element(&mut self, env: Env, name: String) -> Result<Reference<Element>> {
    Element::new_reference(
      env,
      vec![],
      QualName::new(None, Namespace::from("html"), name.into()),
    )
  }
}
