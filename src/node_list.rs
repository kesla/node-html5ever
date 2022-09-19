use napi::{bindgen_prelude::Reference, Env, Error, Result, Either};

use crate::Element;

#[napi]
pub struct NodeList {
  children: Vec<Reference<Element>>,
}

#[napi]
impl NodeList {
  pub fn new(env: Env, children: Vec<Reference<Element>>) -> Result<Reference<Self>> {
    NodeList::into_reference(Self { children: children }, env)
  }

  #[napi]
  pub fn get(&self, index: u32, env: Env) -> Result<Option<Reference<Element>>> {
    let index: usize =
      usize::try_from(index).map_err(|err| Error::from_reason(format!("{}", err)))?;

    match self.children.get(index) {
      Some(value) => match value.clone(env) {
        Ok(cloned) => Ok(Some(cloned)),
        Err(error) => Err(error),
      },
      None => Ok(None),
    }
  }
}
