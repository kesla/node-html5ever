#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Comment {
  pub(crate) content: String,
}
