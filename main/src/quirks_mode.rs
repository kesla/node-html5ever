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
            },
            html5ever::tree_builder::QuirksMode::NoQuirks => {
                QuirksMode::NoQuirks
            },
        }
    }
}

impl From<QuirksMode> for selectors::matching::QuirksMode {
    fn from(val: QuirksMode) -> Self {
        match val {
            QuirksMode::Quirks => selectors::matching::QuirksMode::Quirks,
            QuirksMode::LimitedQuirks => {
                selectors::matching::QuirksMode::LimitedQuirks
            },
            QuirksMode::NoQuirks => selectors::matching::QuirksMode::NoQuirks,
        }
    }
}
