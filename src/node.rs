use napi::{bindgen_prelude::Reference, Either, Error, Result, Status};

use crate::{
  comment::Comment, doc_type::DocType, document::Document, dom::Handle, element::Element,
  text::Text,
};

pub(crate) enum NodeData {
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

#[derive(PartialEq, Eq)]
pub struct Node {
  pub(crate) data: NodeData,
}

impl Default for Node {
  fn default() -> Self {
    Node {
      data: NodeData::None,
    }
  }
}

impl From<Reference<Comment>> for Node {
  fn from(r: Reference<Comment>) -> Self {
    Self {
      data: NodeData::Comment(r),
    }
  }
}

impl From<Reference<Element>> for Node {
  fn from(r: Reference<Element>) -> Self {
    Self {
      data: NodeData::Element(r),
    }
  }
}

impl From<Reference<Document>> for Node {
  fn from(r: Reference<Document>) -> Self {
    Self {
      data: NodeData::Document(r),
    }
  }
}

impl From<Reference<DocType>> for Node {
  fn from(r: Reference<DocType>) -> Self {
    Self {
      data: NodeData::DocType(r),
    }
  }
}

impl From<Reference<Text>> for Node {
  fn from(r: Reference<Text>) -> Self {
    Self {
      data: NodeData::Text(r),
    }
  }
}

impl Node {
  pub(crate) fn append_handle(&self, child: Handle) {
    // TODO: concatenate already existing text node
    let (mut list, parent_reference) = match &self.data {
      NodeData::Element(r) => (r.list.borrow_mut(), Some(Either::A(r.downgrade()))),
      NodeData::Document(r) => (r.list.borrow_mut(), Some(Either::B(r.downgrade()))),
      _ => panic!("Node does not have children"),
    };
    match &child.data {
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
    let mut list = match &self.data {
      NodeData::Element(r) => r.list.borrow_mut(),
      NodeData::Document(r) => r.list.borrow_mut(),
      _ => panic!("Node does not have children"),
    };
    let index = list.iter().position(|c| c == &child).unwrap();
    list.remove(index);

    match &child.data {
      NodeData::Comment(comment) => *comment.parent.borrow_mut() = None,
      NodeData::DocType(doc_type) => *doc_type.parent.borrow_mut() = None,
      NodeData::Element(element) => *element.parent.borrow_mut() = None,
      NodeData::Text(text) => *text.parent.borrow_mut() = None,
      NodeData::Document(_document) => panic!("Document cannot be a child of another node"),
      NodeData::None => panic!("Cannot remove None"),
    }
  }

  pub(crate) fn into_element(&self) -> Result<&Reference<Element>> {
    match &self.data {
      NodeData::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn into_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self.data {
      NodeData::DocType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocType".to_string(),
      )),
    }
  }
}

impl Drop for Node {
  fn drop(&mut self) {
    let node_type: String = match &self.data {
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
