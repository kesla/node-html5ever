#[create_node(is_child)]
pub struct DocumentType {
    #[napi(writable = false)]
    pub name: String,

    #[napi(writable = false)]
    pub public_id: String,

    #[napi(writable = false)]
    pub system_id: String,
}

#[napi]
impl DocumentType {
    #[napi(getter)]
    pub fn get_text_content(&self) -> Option<String> {
        None
    }
}
