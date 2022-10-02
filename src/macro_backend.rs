pub(crate) mod children {
  use std::cell::Ref;

  use napi::{bindgen_prelude::Reference, Result};

  use crate::{Element, Handle};

  pub fn get_children(list: Ref<Vec<Handle>>) -> Result<Vec<Reference<Element>>> {
    list
      .iter()
      .filter_map(|handle| handle.into_element().ok().map(|r| r.clone(r.env)))
      .collect()
  }
}
