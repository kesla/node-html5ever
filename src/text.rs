#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Text {
  pub(crate) content: String,
}
