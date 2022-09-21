use std::cell::RefCell;

use html5ever::{Attribute, QualName};
use napi::{bindgen_prelude::Reference, Env, Result};

use crate::{node_list::NodeList};

#[napi]
pub struct Element {
  attrs: RefCell<Vec<Attribute>>,
  child_nodes: Reference<NodeList>,
  name: QualName,
  env: Env,
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
    self.name.local.to_string().to_uppercase()
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> String {
    self.node_name()
  }

  // #[napi(getter)]
  // pub fn get_children(&self) -> Result<Reference<NodeList>> {
  //   self.children.clone(self.env)
  // }

  // #[napi(getter)]
  // pub fn outer_html(&self) -> String {
  //   serialize(self)
  // }
}