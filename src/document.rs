use std::{cell::RefCell, rc::Rc};

use html5ever::{Namespace, QualName};
use napi::{bindgen_prelude::Reference, Env, Result};

use crate::{doc_type::DocType, dom::Handle, element::Element, lazy_weak_handle::LazyWeakHandle};

#[napi]
pub struct Document {
  pub(crate) list: Rc<RefCell<Vec<Handle>>>,
  pub(crate) env: Env,
  lazy_weak_handle: LazyWeakHandle,
}

#[napi]
impl Document {
  pub(crate) fn new(env: Env) -> Result<Reference<Self>> {
    let document = Self {
      list: Rc::new(RefCell::new(vec![])),
      env,
      lazy_weak_handle: LazyWeakHandle::default(),
    };

    return Self::into_reference(document, env);
  }

  pub(crate) fn get_handle(&self, reference: Reference<Document>) -> Handle {
    self.lazy_weak_handle.get_or_init(reference)
  }

  #[napi(getter)]
  pub fn get_doc_type(&self) -> Result<Option<Reference<DocType>>> {
    if let Some(first) = self.list.borrow().get(0) {
      if let Ok(doc_type) = first.into_doc_type() {
        return Ok(Some(doc_type.clone(self.env)?));
      }
    }

    Ok(None)
  }

  #[napi(getter)]
  pub fn get_document_element(&self) -> Result<Reference<Element>> {
    let list = self.list.borrow();
    let node = match list.len() {
      2 => list.get(1),
      _ => list.get(0),
    }
    .unwrap();
    let element = node.into_element()?;

    return element.clone(self.env);
  }

  #[napi(getter)]
  pub fn get_head(&mut self, env: Env) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    let list = document_element.list.borrow();
    list.get(0).unwrap().into_element()?.clone(env)
  }

  #[napi(getter)]
  pub fn get_body(&mut self, env: Env) -> Result<Reference<Element>> {
    let document_element = self.get_document_element()?;

    let list = document_element.list.borrow();
    list.get(1).unwrap().into_element()?.clone(env)
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    "#document".to_string()
  }

  #[napi]
  pub fn create_element(&mut self, env: Env, name: String) -> Result<Reference<Element>> {
    Element::new_reference(
      env,
      vec![].into(),
      QualName::new(None, Namespace::from("html"), name.into()),
    )
  }
}

impl Drop for Document {
  fn drop(&mut self) {
    println!("Dropping Document");
  }
}
