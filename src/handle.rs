use std::{
  cell::{Ref, RefCell, RefMut},
  rc::{Rc, Weak},
};

use napi::{
  bindgen_prelude::{Either4, Reference},
  Either, Env, Error, Result, Status,
};

use crate::{get_id, Comment, DocType, Document, Element, ParentContext, Text};

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

    log::debug!("Dropping Node {:?}", node_type);
  }
}

struct HandleInner {
  env: Env,
  id: usize,
  node: NodeReference,
  list: RefCell<Vec<Handle>>,
  parent_context: RefCell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct Handle(Rc<HandleInner>);

impl Handle {
  pub(crate) fn new(env: Env, node: NodeReference) -> Self {
    Handle(Rc::new(HandleInner {
      env,
      id: get_id(),
      list: RefCell::new(vec![]),
      node,
      parent_context: RefCell::new(None),
    }))
  }

  pub(crate) fn get_children(&self) -> Ref<Vec<Handle>> {
    self.0.list.borrow()
  }

  pub(crate) fn get_children_mut(&self) -> RefMut<Vec<Handle>> {
    self.0.list.borrow_mut()
  }

  pub(crate) fn get_parent(&self) -> Ref<Option<ParentContext>> {
    self.0.parent_context.borrow()
  }

  pub(crate) fn get_parent_mut(&self) -> RefMut<Option<ParentContext>> {
    self.0.parent_context.borrow_mut()
  }

  pub(crate) fn get_parent_node(
    &self,
  ) -> Result<Option<Either<Reference<Document>, Reference<Element>>>> {
    let parent = self.get_parent();
    let maybe_reference = parent.as_ref();

    let r = match maybe_reference {
      Some(parent_context) => match parent_context.node {
        Either::A(ref document) => {
          let document = document.upgrade(self.0.env)?;
          document.map(Either::A)
        }
        Either::B(ref element) => {
          let element = element.upgrade(self.0.env)?;
          element.map(Either::B)
        }
      },
      None => None,
    };

    Ok(r)
  }

  pub(crate) fn get_parent_handle(&self) -> Result<Option<Handle>> {
    match self.get_parent_node()? {
      Some(Either::A(element)) => Ok(Some(element.get_handle())),
      Some(Either::B(document)) => Ok(Some(document.get_handle())),
      None => Ok(None),
    }
  }

  pub(crate) fn get_node_reference(&self) -> &NodeReference {
    &self.0.node
  }

  pub(crate) fn as_element(&self) -> Result<&Reference<Element>> {
    match self.get_node_reference() {
      NodeReference::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn as_doc_type(&self) -> Result<&Reference<DocType>> {
    match self.get_node_reference() {
      NodeReference::DocType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocType".to_string(),
      )),
    }
  }

  pub(crate) fn append_handle(&self, child: Handle) {
    // TODO: concatenate already existing text node

    let mut children = self.get_children_mut();
    children.push(child.clone());

    let parent_reference = match &self.get_node_reference() {
      NodeReference::Document(r) => Either::A(r.downgrade()),
      NodeReference::Element(r) => Either::B(r.downgrade()),
      _ => panic!("Wrong type"),
    };
    let parent_context = Some(ParentContext::new(parent_reference, children.len() - 1));
    let mut parent = child.get_parent_mut();
    *parent = parent_context;
  }

  pub(crate) fn remove_handle(&self, child: Handle) {
    let mut children = self.get_children_mut();
    let index = children.iter().position(|c| c == &child).unwrap();
    children.remove(index);
    let mut parent = child.get_parent_mut();
    *parent = None;
  }

  pub(crate) fn downgrade(&self) -> WeakHandle {
    WeakHandle(Rc::downgrade(&self.0))
  }
}

impl From<Either<&Document, &Element>> for Handle {
  fn from(e: Either<&Document, &Element>) -> Self {
    match e {
      Either::A(r) => r.into(),
      Either::B(r) => r.into(),
    }
  }
}

impl From<Either4<&Comment, &DocType, &Element, &Text>> for Handle {
  fn from(e: Either4<&Comment, &DocType, &Element, &Text>) -> Self {
    match e {
      Either4::A(r) => r.into(),
      Either4::B(r) => r.into(),
      Either4::C(r) => r.into(),
      Either4::D(r) => r.into(),
    }
  }
}

impl PartialEq for Handle {
  fn eq(&self, other: &Self) -> bool {
    self.0.id == other.0.id
  }
}

impl Eq for Handle {}

pub(crate) struct WeakHandle(Weak<HandleInner>);

impl WeakHandle {
  pub(crate) fn upgrade(&self) -> Option<Handle> {
    self.0.upgrade().map(|h| Handle(h))
  }
}

impl Default for WeakHandle {
  fn default() -> Self {
    WeakHandle(Weak::new())
  }
}
