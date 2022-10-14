#[create_node(is_child)]
pub struct Text {
  #[napi(writable = false)]
  pub data: String,
}
