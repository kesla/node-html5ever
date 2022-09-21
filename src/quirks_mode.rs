use napi::bindgen_prelude::ToNapiValue;

#[napi]
pub enum QuirksMode {
  Quirks,
  LimitedQuirks,
  NoQuirks,
}
