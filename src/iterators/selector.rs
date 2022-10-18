use napi::bindgen_prelude::Reference;

use crate::{DeepChildNodesIterator, Element, ElementRef, Selectors};

pub struct SelectorsIterator {
  selectors: Selectors,
  iter: DeepChildNodesIterator<ElementRef>,
}

impl SelectorsIterator {
  pub fn new(selectors: Selectors, iter: DeepChildNodesIterator<ElementRef>) -> Self {
    Self { selectors, iter }
  }
}

impl Iterator for SelectorsIterator {
  type Item = Reference<Element>;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.find_map(|element_ref| {
      self
        .selectors
        .matches(&element_ref)
        .then(|| element_ref.into())
    })
  }
}
