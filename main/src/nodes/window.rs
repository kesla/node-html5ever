use napi::{
    bindgen_prelude::Reference,
    Env,
    Result,
};

use crate::Document;

#[napi]
pub struct Window {
    document_reference: Reference<Document>,
    env: Env,
}

#[napi]
impl Window {
    pub(crate) fn new_reference(
        env: Env,
        document: Reference<Document>,
    ) -> Result<Reference<Self>> {
        let window = Self {
            document_reference: document,
            env,
        };

        Self::into_reference(window, env)
    }

    #[napi(getter)]
    pub fn get_document(&self) -> Result<Reference<Document>> {
        self.document_reference.clone(self.env)
    }
}
