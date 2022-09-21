use napi::{bindgen_prelude::Reference, Env, Error, Result};

use crate::node::Node;

#[napi]
pub struct NodeList {
  children: Vec<Node>,
  env: Env,
}

#[napi]
impl NodeList {
  pub fn new(env: Env) -> Result<Reference<Self>> {
    NodeList::into_reference(
      Self {
        children: vec![],
        env,
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

  pub fn len(&self) -> usize {
    self.children.len()
  }
}
