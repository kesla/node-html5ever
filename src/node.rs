use napi::{
  bindgen_prelude::{Either5, Reference, ToNapiValue},
  Error, Result, Status,
};

use crate::{
  comment::Comment, doc_type::DocType, document::Document, element::Element, text::Text,
};

pub(crate) enum NodeData {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

type EitherType = Either5<
  Reference<Comment>,
  Reference<DocType>,
  Reference<Document>,
  Reference<Element>,
  Reference<Text>,
>;

impl Into<EitherType> for NodeData {
  fn into(self) -> EitherType {
    match self {
      NodeData::Comment(i) => Either5::A(i),
      NodeData::DocType(i) => Either5::B(i),
      NodeData::Document(i) => Either5::C(i),
      NodeData::Element(i) => Either5::D(i),
      NodeData::Text(i) => Either5::E(i),
    }
  }
}

pub struct Node {
  pub(crate) data: NodeData,
}

impl ToNapiValue for Node {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    Either5::to_napi_value(env, val.data.into())
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
  pub(crate) fn into_element(&self) -> Result<&Reference<Element>> {
    match &self.data {
      NodeData::Element(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not an Element".to_string())),
    }
  }

  pub(crate) fn into_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self.data {
      NodeData::DocType(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a DocType".to_string())),
    }
  }

  pub(crate) fn into_document(&self) -> Result<&Reference<Document>> {
    match &self.data {
      NodeData::Document(r) => Ok(r),
      _ => Err(Error::new(Status::InvalidArg, "not a Document".to_string())),
    }
  }
}
