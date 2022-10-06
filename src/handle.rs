use crate::{node_handler::ParentContext, Comment, DocType, Document, Element, NodeHandler, Text};
use napi::{
  bindgen_prelude::{Either4, Error, Reference, Result, WeakReference},
  Either, Status,
};

pub enum Handle {
  Comment(Reference<Comment>),
  DocType(Reference<DocType>),
  Document(Reference<Document>),
  Element(Reference<Element>),
  Text(Reference<Text>),
}

impl From<Either4<&Comment, &DocType, &Element, &Text>> for Handle {
  fn from(value: Either4<&Comment, &DocType, &Element, &Text>) -> Self {
    match value {
      Either4::A(comment) => comment.into(),
      Either4::B(doc_type) => doc_type.into(),
      Either4::C(element) => element.into(),
      Either4::D(text) => text.into(),
    }
  }
}

impl Into<Either<WeakReference<Document>, WeakReference<Element>>> for &Handle {
  fn into(self) -> Either<WeakReference<Document>, WeakReference<Element>> {
    match self {
      Handle::Document(document) => Either::A(document.downgrade()),
      Handle::Element(element) => Either::B(element.downgrade()),
      _ => panic!("Invalid handle"),
    }
  }
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
      Handle::Document(_) => panic!("Document is not a Node"),
    }
  }
}

impl From<Reference<Comment>> for Handle {
  fn from(r: Reference<Comment>) -> Self {
    Self::Comment(r)
  }
}

impl From<&Comment> for Handle {
  fn from(r: &Comment) -> Self {
    Self::Comment(
      r.weak_reference
        .as_ref()
        .unwrap()
        .upgrade(r.env)
        .unwrap()
        .unwrap(),
    )
  }
}

impl From<Reference<Element>> for Handle {
  fn from(r: Reference<Element>) -> Self {
    Self::Element(r)
  }
}

impl From<&Element> for Handle {
  fn from(r: &Element) -> Self {
    Self::Element(
      r.weak_reference
        .as_ref()
        .unwrap()
        .upgrade(r.env)
        .unwrap()
        .unwrap(),
    )
  }
}

impl From<Reference<Document>> for Handle {
  fn from(r: Reference<Document>) -> Self {
    Self::Document(r)
  }
}

impl From<&Document> for Handle {
  fn from(r: &Document) -> Self {
    Self::Document(
      r.weak_reference
        .as_ref()
        .unwrap()
        .upgrade(r.env)
        .unwrap()
        .unwrap(),
    )
  }
}

impl From<Reference<DocType>> for Handle {
  fn from(r: Reference<DocType>) -> Self {
    Self::DocType(r)
  }
}

impl From<&DocType> for Handle {
  fn from(r: &DocType) -> Self {
    Self::DocType(
      r.weak_reference
        .as_ref()
        .unwrap()
        .upgrade(r.env)
        .unwrap()
        .unwrap(),
    )
  }
}

impl From<Reference<Text>> for Handle {
  fn from(r: Reference<Text>) -> Self {
    Self::Text(r)
  }
}

impl From<&Text> for Handle {
  fn from(r: &Text) -> Self {
    Self::Text(
      r.weak_reference
        .as_ref()
        .unwrap()
        .upgrade(r.env)
        .unwrap()
        .unwrap(),
    )
  }
}

impl PartialEq for Handle {
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

impl Eq for Handle {}

impl Clone for Handle {
  fn clone(&self) -> Self {
    match self {
      Self::Comment(arg0) => Self::Comment(arg0.clone(arg0.env).unwrap()),
      Self::DocType(arg0) => Self::DocType(arg0.clone(arg0.env).unwrap()),
      Self::Document(arg0) => Self::Document(arg0.clone(arg0.env).unwrap()),
      Self::Element(arg0) => Self::Element(arg0.clone(arg0.env).unwrap()),
      Self::Text(arg0) => Self::Text(arg0.clone(arg0.env).unwrap()),
    }
  }
}

impl Handle {
  pub(crate) fn as_element(&self) -> Result<&Reference<Element>> {
    match &self {
      Handle::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn as_doc_type(&self) -> Result<&Reference<DocType>> {
    match &self {
      Handle::DocType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocType".to_string(),
      )),
    }
  }

  pub(crate) fn append_handle(&self, child_handle: &Handle) -> Result<()> {
    // remove from old parent
    {
      child_handle.remove()?;
    }
    // TODO: concatenate already existing text node

    let node_handler = NodeHandler::from(self);
    let mut children = node_handler.get_child_nodes_mut();
    children.append_handle(child_handle);

    let parent_reference: Either<WeakReference<Document>, WeakReference<Element>> = self.into();

    let parent_context = Some(ParentContext::new(
      node_handler.get_env(),
      parent_reference,
      children.len() - 1,
    ));
    let node_handler = NodeHandler::from(child_handle);
    let mut parent = node_handler.get_parent_mut();
    *parent = parent_context;
    Ok(())
  }

  pub(crate) fn remove_handle(&self, child_handle: &Handle) {
    let child_node_handler: NodeHandler = child_handle.into();

    let node_handler = NodeHandler::from(self);
    let mut children = node_handler.get_child_nodes_mut();
    children.remove_handle(child_handle);

    let mut parent = child_node_handler.get_parent_mut();
    *parent = None;
  }

  pub(crate) fn remove(&self) -> Result<()> {
    let node_handler = NodeHandler::from(self);
    let maybe_parent = node_handler.get_parent();

    match maybe_parent.as_ref() {
      Some(parent) => {
        let handle: Handle = parent.try_into()?;
        handle.remove_handle(&self)
      }
      None => {}
    }

    Ok(())
  }
}
