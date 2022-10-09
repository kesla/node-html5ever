#[create_node(is_child)]
pub struct Text {
  #[napi(writable = false)]
  pub data: String,
}

impl Drop for Text {
  fn drop(&mut self) {
    println!("Dropping Text {:?}", self.data);
  }
}
