use crate::{Comment, DocType, Document, Element, Text};
use napi::bindgen_prelude::Reference;

pub enum NodeReference {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

impl PartialEq for NodeReference {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Comment(left), Self::Comment(right)) => left.id == right.id,
      (Self::DocType(left), Self::DocType(right)) => left.id == right.id,
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

impl From<Reference<DocType>> for NodeReference {
  fn from(r: Reference<DocType>) -> Self {
    NodeReference::DocType(r)
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
      NodeReference::DocType(_) => "DocType".to_string(),
      NodeReference::Document(_) => "Document".to_string(),
      NodeReference::Element(element) => format!("Element <{}>", element.name.local),
      NodeReference::Text(_) => "Text".to_string(),
    };

    println!("Dropping NodeReference {:?}", node_type);
  }
}
