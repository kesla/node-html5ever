use std::slice::Iter;

use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::node::Node;

#[napi]
pub struct NodeList {
  children: Vec<Node>,
}

#[napi]
impl NodeList {
  pub fn new(env: Env) -> Result<Reference<Self>> {
    NodeList::into_reference(
      Self {
        children: vec![],
      },
      env,
    )
  }

  #[napi]
  pub fn get(&self, index: u32) -> Result<Node> {
    let index: usize =
      usize::try_from(index).map_err(|err| Error::from_reason(format!("{}", err)))?;

    match self.children.get(index) {
      Some(value) => Ok(value.clone()),
      None => Err(Error::from_reason("Node not found".to_string())),
    }
  }

  pub(crate) fn push(&mut self, node: Node) {
    self.children.push(node)
  }

  pub(crate) fn len(&self) -> usize {
    self.children.len()
  }

  pub(crate) fn iter(&self) -> Iter<Node> {
    self.children.iter()
  }
}
