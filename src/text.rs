#[napi]
#[derive(NodeType)]
#[add_node_fields]
pub struct Text {
  pub(crate) content: String,
}

impl Drop for Text {
  fn drop(&mut self) {
    println!("Dropping Text {:?}", self.content);
  }
}
