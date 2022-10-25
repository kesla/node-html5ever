use napi::bindgen_prelude::ToNapiValue;

#[napi]
pub enum QuirksMode {
  Quirks,
  LimitedQuirks,
  NoQuirks,
}

impl From<html5ever::tree_builder::QuirksMode> for QuirksMode {
  fn from(q: html5ever::tree_builder::QuirksMode) -> Self {
    match q {
      html5ever::tree_builder::QuirksMode::Quirks => QuirksMode::Quirks,
      html5ever::tree_builder::QuirksMode::LimitedQuirks => {
        QuirksMode::LimitedQuirks
      }
      html5ever::tree_builder::QuirksMode::NoQuirks => QuirksMode::NoQuirks,
    }
  }
}

impl Into<selectors::matching::QuirksMode> for QuirksMode {
  fn into(self) -> selectors::matching::QuirksMode {
    match self {
      QuirksMode::Quirks => selectors::matching::QuirksMode::Quirks,
      QuirksMode::LimitedQuirks => {
        selectors::matching::QuirksMode::LimitedQuirks
      }
      QuirksMode::NoQuirks => selectors::matching::QuirksMode::NoQuirks,
    }
  }
}
