use crate::QuirksMode;

#[create_node(has_children)]
pub struct DocumentFragment {
  pub(crate) quirks_mode: QuirksMode,
}

#[napi]
impl DocumentFragment {
  #[napi(getter)]
  pub fn get_text_content(&self) -> Option<String> {
    None
  }
}
