#[create_node(parent)]
pub struct Text {
  pub(crate) content: String,
}

impl Drop for Text {
  fn drop(&mut self) {
    println!("Dropping Text {:?}", self.content);
  }
}
