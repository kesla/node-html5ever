use napi::{bindgen_prelude::Reference, Either, Error, Result, Status};

use crate::{Comment, DocType, Document, Element, Handle, Text};

pub enum NodeData {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
  None,
}

impl PartialEq for NodeData {
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

impl Eq for NodeData {}

impl Default for NodeData {
  fn default() -> Self {
    NodeData::None
  }
}

impl From<Reference<Comment>> for NodeData {
  fn from(r: Reference<Comment>) -> Self {
    NodeData::Comment(r)
  }
}

impl From<Reference<Element>> for NodeData {
  fn from(r: Reference<Element>) -> Self {
    NodeData::Element(r)
  }
}

impl From<Reference<Document>> for NodeData {
  fn from(r: Reference<Document>) -> Self {
    NodeData::Document(r)
  }
}

impl From<Reference<DocType>> for NodeData {
  fn from(r: Reference<DocType>) -> Self {
    NodeData::DocType(r)
  }
}

impl From<Reference<Text>> for NodeData {
  fn from(r: Reference<Text>) -> Self {
    NodeData::Text(r)
  }
}

impl NodeData {
  pub(crate) fn append_handle(&self, child: Handle) {
    // TODO: concatenate already existing text node
    let (mut list, parent_reference) = match &self {
      NodeData::Element(r) => (r.list.borrow_mut(), Some(Either::A(r.downgrade()))),
      NodeData::Document(r) => (r.list.borrow_mut(), Some(Either::B(r.downgrade()))),
      _ => panic!("Node does not have children"),
    };
    let child_node_data: &NodeData = &child;
    match &child_node_data {
      NodeData::Comment(comment) => *comment.parent.borrow_mut() = parent_reference,
      NodeData::DocType(doc_type) => *doc_type.parent.borrow_mut() = parent_reference,
      NodeData::Element(element) => *element.parent.borrow_mut() = parent_reference,
      NodeData::Text(text) => *text.parent.borrow_mut() = parent_reference,
      NodeData::Document(_document) => panic!("Document cannot be a child of another node"),
      NodeData::None => panic!("Cannot append None"),
    }
    list.push(child);
  }

  pub(crate) fn remove_handle(&self, child: Handle) {
    let mut list = match &self {
      NodeData::Element(r) => r.list.borrow_mut(),
      NodeData::Document(r) => r.list.borrow_mut(),
      _ => panic!("Node does not have children"),
    };
    let index = list.iter().position(|c| c == &child).unwrap();
    list.remove(index);

    let child_node_data: &NodeData = &child;
    match child_node_data {
      NodeData::Comment(comment) => *comment.parent.borrow_mut() = None,
      NodeData::DocType(doc_type) => *doc_type.parent.borrow_mut() = None,
      NodeData::Element(element) => *element.parent.borrow_mut() = None,
      NodeData::Text(text) => *text.parent.borrow_mut() = None,
      NodeData::Document(_document) => panic!("Document cannot be a child of another node"),
      NodeData::None => panic!("Cannot remove None"),
    }
  }

  pub(crate) fn into_element(&self) -> Result<&Reference<Element>> {
    match &self {
      NodeData::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn into_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self {
      NodeData::DocType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocType".to_string(),
      )),
    }
  }
}

impl Drop for NodeData {
  fn drop(&mut self) {
    let node_type: String = match &self {
      NodeData::Comment(_) => "Comment".to_string(),
      NodeData::DocType(_) => "DocType".to_string(),
      NodeData::Document(_) => "Document".to_string(),
      NodeData::Element(element) => format!("Element <{}>", element.name.local),
      NodeData::Text(_) => "Text".to_string(),
      NodeData::None => "None".to_string(),
    };

    println!("Dropping Node {:?}", node_type);
  }
}
