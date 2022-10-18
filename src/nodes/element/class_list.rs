use napi::{bindgen_prelude::WeakReference, Env};

use crate::Element;

#[napi]
pub struct ClassList {
  owner: WeakReference<Element>,
  env: Env,
}

#[napi]
impl ClassList {
  pub(crate) fn new(owner: WeakReference<Element>, env: Env) -> Self {
    Self { owner, env }
  }

  #[napi]
  pub fn contains(&self, token: String) -> bool {
    let element = match self.owner.upgrade(self.env) {
      Ok(Some(element)) => element,
      _ => return false,
    };

    element
      .get_attribute("class".into())
      .map_or(false, |class_list| {
        class_list.split(' ').any(|c| c.len() > 0 && c == token)
      })
  }
}
