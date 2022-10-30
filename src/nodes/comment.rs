use napi::{
    bindgen_prelude::Reference,
    Result,
};

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

    #[napi]
    pub fn clone_node(&self) -> Result<Reference<Self>> {
        Self::new_reference(self.env, self.data.clone())
    }
}
