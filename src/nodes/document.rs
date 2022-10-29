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
    DocumentFragment,
    DocumentType,
    Element,
    Html5everDom,
    LazyReference,
    Node,
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
        let node: Node = self.into();

        node.try_get_child_node(0).ok().flatten()
    }

    #[napi(getter)]
    pub fn get_document_element(&self) -> Result<Reference<Element>> {
        let node: Node = self.into();

        if let Ok(Some(r)) = node.try_get_child_node(0) {
            Ok(r)
        } else if let Ok(Some(r)) = node.try_get_child_node(1) {
            Ok(r)
        } else {
            Err(Error::from_reason(
                "Document has no document Element (<html>)".to_string(),
            ))
        }
    }

    #[napi(getter)]
    pub fn get_head(&mut self) -> Result<Reference<Element>> {
        let document_element = self.get_document_element()?;
        let node: Node = document_element.into();

        match node.try_get_child_node(0) {
            Ok(Some(e)) => Ok(e),
            _ => Err(Error::from_reason(
                "Document has no head Element (<head>)".to_string(),
            )),
        }
    }

    #[napi(getter)]
    pub fn get_body(&mut self) -> Result<Reference<Element>> {
        let document_element = self.get_document_element()?;
        let node: Node = document_element.into();

        match node.try_get_child_node(1) {
            Ok(Some(e)) => Ok(e),
            _ => Err(Error::from_reason(
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

    #[napi]
    pub fn create_document_fragment(
        &self,
        html: String,
    ) -> Result<Reference<DocumentFragment>> {
        Html5everDom::create_document_fragment(
            self.env,
            html,
            Some(self.quirks_mode),
        )
    }
}
