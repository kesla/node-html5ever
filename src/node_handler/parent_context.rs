use napi::{
  bindgen_prelude::{Either, WeakReference},
  Env,
};

use crate::{Document, Element};

pub(crate) struct ParentContext {
  pub(crate) node: Either<WeakReference<Document>, WeakReference<Element>>,
  pub(crate) index: usize,
  pub(crate) env: Env,
}

impl ParentContext {
  pub(crate) fn new(
    env: Env,
    node: Either<WeakReference<Document>, WeakReference<Element>>,
    index: usize,
  ) -> Self {
    ParentContext { env, node, index }
  }
}
