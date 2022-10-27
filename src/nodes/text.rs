#[create_node(is_child)]
pub struct Text {
    #[napi(writable = false)]
    pub data: String,
}

#[napi]
impl Text {
    #[napi(getter)]
    pub fn get_text_content(&self) -> Option<String> {
        Some(self.data.clone())
    }
}
