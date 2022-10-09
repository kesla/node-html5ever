#[create_node(is_child)]
pub struct Comment {
  #[napi(writable = false)]
  pub data: String,
}
