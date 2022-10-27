use html5ever::{
    Namespace,
    QualName,
};
use napi::{
    bindgen_prelude::Reference,
    Error,
    Result,
};

use crate::{
    DocumentType,
    Element,
    LazyReference,
    QuirksMode,
    Text,
};

#[create_node(has_children)]
pub struct Document {
    pub(crate) quirks_mode: QuirksMode,
}

#[napi]
impl Document {
    #[napi(getter)]
    pub fn get_doc_type(&self) -> Option<Reference<DocumentType>> {
        self.get_node_handler()
            .get_child_node::<Reference<DocumentType>, Error>(0)
    }

    #[napi(getter)]
    pub fn get_document_element(&self) -> Result<Reference<Element>> {
        let node_handler = self.get_node_handler();

        match node_handler.get_child_node::<Reference<Element>, Error>(0) {
            Some(r) => Ok(r),
            None => {
                match node_handler
                    .try_get_child_node::<Reference<Element>, Error>(1)
                {
                    Ok(Some(e)) => Ok(e),
                    Ok(None) => Err(Error::from_reason(
                        "Document has no document Element (<html>)".to_string(),
                    )),
                    Err(e) => Err(e),
                }
            },
        }
    }

    #[napi(getter)]
    pub fn get_head(&mut self) -> Result<Reference<Element>> {
        let document_element = self.get_document_element()?;

        let node_handler = document_element.get_node_handler();

        match node_handler.try_get_child_node::<Reference<Element>, Error>(0)? {
            Some(e) => Ok(e),
            None => Err(Error::from_reason(
                "Document has no head Element (<head>)".to_string(),
            )),
        }
    }

    #[napi(getter)]
    pub fn get_body(&mut self) -> Result<Reference<Element>> {
        let document_element = self.get_document_element()?;
        let node_handler = document_element.get_node_handler();

        match node_handler.try_get_child_node::<Reference<Element>, Error>(1)? {
            Some(e) => Ok(e),
            None => Err(Error::from_reason(
                "Document has no body Element (<body>)".to_string(),
            )),
        }
    }

    #[napi(getter)]
    pub fn get_text_content(&self) -> Option<String> {
        None
    }

    #[napi]
    pub fn create_element(
        &self,
        name: String,
    ) -> Result<Reference<Element>> {
        Element::new_reference(
            self.env,
            vec![].into(),
            QualName::new(None, Namespace::from("html"), name.into()),
            LazyReference::new(self.env),
            LazyReference::new(self.env),
        )
    }

    #[napi]
    pub fn create_text_node(
        &mut self,
        data: String,
    ) -> Result<Reference<Text>> {
        Text::new_reference(self.env, data)
    }
}
