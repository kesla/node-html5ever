use napi::{Either, bindgen_prelude::WeakReference};

use crate::{element::Element, document::Document};

pub trait Node {
  fn get_parent_node(&self) -> Option<Either<WeakReference<Element>, WeakReference<Document>>>;
}