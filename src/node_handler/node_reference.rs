use crate::{Comment, Document, DocumentType, Element, Text};
use napi::bindgen_prelude::Reference;

pub enum NodeReference {
  Comment(Reference<Comment>),
  DocumentType(Reference<DocumentType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

impl PartialEq for NodeReference {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Comment(left), Self::Comment(right)) => left.id == right.id,
      (Self::DocumentType(left), Self::DocumentType(right)) => left.id == right.id,
      (Self::Document(left), Self::Document(right)) => left.id == right.id,
      (Self::Element(left), Self::Element(right)) => left.id == right.id,
      (Self::Text(left), Self::Text(right)) => left.id == right.id,
      _ => false,
    }
  }
}

impl Eq for NodeReference {}

impl From<Reference<Comment>> for NodeReference {
  fn from(r: Reference<Comment>) -> Self {
    NodeReference::Comment(r)
  }
}

impl From<Reference<Element>> for NodeReference {
  fn from(r: Reference<Element>) -> Self {
    NodeReference::Element(r)
  }
}

impl From<Reference<Document>> for NodeReference {
  fn from(r: Reference<Document>) -> Self {
    NodeReference::Document(r)
  }
}

impl From<Reference<DocumentType>> for NodeReference {
  fn from(r: Reference<DocumentType>) -> Self {
    NodeReference::DocumentType(r)
  }
}

impl From<Reference<Text>> for NodeReference {
  fn from(r: Reference<Text>) -> Self {
    NodeReference::Text(r)
  }
}

impl Drop for NodeReference {
  fn drop(&mut self) {
    let node_type: String = match &self {
      NodeReference::Comment(_) => "Comment".to_string(),
      NodeReference::DocumentType(_) => "DocumentType".to_string(),
      NodeReference::Document(_) => "Document".to_string(),
      NodeReference::Element(element) => format!("Element <{}>", element.name.local),
      NodeReference::Text(_) => "Text".to_string(),
    };

    println!("Dropping NodeReference {:?}", node_type);
  }
}
