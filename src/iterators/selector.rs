use napi::{bindgen_prelude::Reference, Result};

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
  type Item = Result<Reference<Element>>;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(element_ref) = self.iter.next() {
      match self.selectors.matches(&element_ref) {
        Ok(true) => return Some(Ok(element_ref.into())),
        Ok(false) => continue,
        Err(err) => return Some(Err(err)),
      }
    }

    None
  }
}

impl SelectorsIterator {
  pub fn try_next(&mut self) -> Result<Option<Reference<Element>>> {
    self.next().transpose()
  }
}
