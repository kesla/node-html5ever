#[create_node(has_children)]
pub struct DocumentFragment {}

#[napi]
impl DocumentFragment {
  #[napi(getter)]
  pub fn get_text_content(&self) -> Option<String> {
    None
  }
}
