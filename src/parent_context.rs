use napi::{bindgen_prelude::WeakReference, Either};

use crate::{Document, Element};

pub(crate) struct ParentContext {
  pub(crate) node: Either<WeakReference<Element>, WeakReference<Document>>,
  pub(crate) index: usize,
}
