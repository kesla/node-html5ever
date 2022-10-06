use crate::{Comment, DocType, Document, Element, Text};
use napi::bindgen_prelude::{Either4, Reference, WeakReference};

pub enum Handle {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

impl
  Into<
    Either4<
      WeakReference<Comment>,
      WeakReference<DocType>,
      WeakReference<Element>,
      WeakReference<Text>,
    >,
  > for &Handle
{
  fn into(
    self,
  ) -> Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  > {
    match self {
      Handle::Comment(r) => Either4::A(r.downgrade()),
      Handle::DocType(r) => Either4::B(r.downgrade()),
      Handle::Element(r) => Either4::C(r.downgrade()),
      Handle::Text(r) => Either4::D(r.downgrade()),
      Handle::Document(_) => unreachable!("Document is not a Node"),
    }
  }
}

impl From<Reference<Comment>> for Handle {
  fn from(r: Reference<Comment>) -> Self {
    Self::Comment(r)
  }
}

impl From<Reference<Element>> for Handle {
  fn from(r: Reference<Element>) -> Self {
    Self::Element(r)
  }
}

impl From<Reference<Document>> for Handle {
  fn from(r: Reference<Document>) -> Self {
    Self::Document(r)
  }
}

impl From<Reference<DocType>> for Handle {
  fn from(r: Reference<DocType>) -> Self {
    Self::DocType(r)
  }
}

impl From<Reference<Text>> for Handle {
  fn from(r: Reference<Text>) -> Self {
    Self::Text(r)
  }
}
