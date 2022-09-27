use std::{
  cell::RefCell,
  rc::{Rc},
};

use html5ever::{Namespace, QualName};
use napi::{bindgen_prelude::Reference, Env, Result};

use crate::{
  doc_type::DocType,
  dom::{new_weak_handle, Handle, WeakHandle, new_handle},
  element::Element, node::Node,
};

#[napi]
pub struct Document {
  pub(crate) list: Rc<RefCell<Vec<Handle>>>,
  pub(crate) env: Env,
  weak_handle: RefCell<WeakHandle>,
}

#[napi]
impl Document {
  pub(crate) fn new(env: Env) -> Result<Reference<Self>> {
    let document = Self {
      list: Rc::new(RefCell::new(vec![])),
      env,
      weak_handle: RefCell::new(new_weak_handle(None)),
    };

    return Self::into_reference(document, env);
  }

  pub(crate) fn get_handle(&self, reference: Reference<Document>) -> Handle {
    let mut weak_handle = self.weak_handle.borrow_mut();

    let maybe_handle = weak_handle.upgrade();

    match maybe_handle {
      Some(handle) => handle,
      None => {
        let node: Node =  reference.into();
        let handle = new_handle(node);
        *weak_handle = new_weak_handle(Some(handle.clone()));
        handle
      },
    }
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
