#[create_node(is_child)]
pub struct Comment {
    #[napi(writable = false)]
    pub data: String,
}

#[napi]
impl Comment {
    #[napi(getter)]
    pub fn get_text_content(&self) -> Option<String> {
        Some(self.data.clone())
    }
}
