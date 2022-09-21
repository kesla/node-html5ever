use napi::{bindgen_prelude::WeakReference, Either};

use crate::{document::Document, element::Element};

pub fn clone_parent_node(
  maybe_reference: Option<&Either<WeakReference<Element>, WeakReference<Document>>>,
) -> Option<Either<WeakReference<Element>, WeakReference<Document>>> {
  maybe_reference.map(|value| match value {
    Either::A(element) => Either::A(element.clone()),
    Either::B(document) => Either::B(document.clone()),
  })
}
