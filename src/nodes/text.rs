#[create_node(is_child)]
pub struct Text {
  pub(crate) content: String,
}

impl Drop for Text {
  fn drop(&mut self) {
    log::debug!("Dropping Text {:?}", self.content);
  }
}
