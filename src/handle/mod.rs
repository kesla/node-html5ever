use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

use napi::{
  bindgen_prelude::{Either4, Reference},
  Either, Env, Error, Result, Status,
};

use crate::{get_id, Comment, DocType, Document, Element, Text};

mod child_node_list;
mod node_reference;
mod parent_context;
mod weak;

pub(crate) use node_reference::NodeReference;
pub(crate) use parent_context::ParentContext;
pub(crate) use weak::WeakHandle;

struct HandleInner {
  env: Env,
  id: usize,
  node: node_reference::NodeReference,
  list: RefCell<child_node_list::ChildNodeList>,
  parent_context: RefCell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct Handle(Rc<HandleInner>);

impl Handle {
  pub(crate) fn new(env: Env, node: node_reference::NodeReference) -> Self {
    Handle(Rc::new(HandleInner {
      env,
      id: get_id(),
      list: Default::default(),
      node,
      parent_context: RefCell::new(None),
    }))
  }

  pub(crate) fn get_children(&self) -> Ref<child_node_list::ChildNodeList> {
    self.0.list.borrow()
  }

  pub(crate) fn get_children_mut(&self) -> RefMut<child_node_list::ChildNodeList> {
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

  pub(crate) fn get_node_reference(&self) -> &node_reference::NodeReference {
    &self.0.node
  }

  pub(crate) fn as_element(&self) -> Result<&Reference<Element>> {
    match self.get_node_reference() {
      node_reference::NodeReference::Element(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not an Element".to_string(),
      )),
    }
  }

  pub(crate) fn as_doc_type(&self) -> Result<&Reference<DocType>> {
    match self.get_node_reference() {
      node_reference::NodeReference::DocType(r) => Ok(r),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Node is not a DocType".to_string(),
      )),
    }
  }

  pub(crate) fn append_handle(&self, child: &Handle) -> Result<()> {
    // remove from old parent
    child.remove()?;

    // TODO: concatenate already existing text node

    let mut children = self.get_children_mut();
    children.append_handle(child.clone());

    let parent_reference = match &self.get_node_reference() {
      node_reference::NodeReference::Document(r) => Either::A(r.downgrade()),
      node_reference::NodeReference::Element(r) => Either::B(r.downgrade()),
      _ => panic!("Wrong type"),
    };
    let parent_context = Some(ParentContext::new(parent_reference, children.len() - 1));
    let mut parent = child.get_parent_mut();
    *parent = parent_context;
    Ok(())
  }

  pub(crate) fn remove_handle(&self, child: &Handle) {
    let mut children = self.get_children_mut();
    children.remove_handle(child);

    let mut parent = child.get_parent_mut();
    *parent = None;
  }

  pub(crate) fn remove(&self) -> Result<()> {
    let maybe_handle = self.get_parent_handle()?;

    match maybe_handle {
      Some(parent) => parent.remove_handle(self),
      None => {}
    }

    Ok(())
  }

  pub(crate) fn downgrade(&self) -> WeakHandle {
    self.into()
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
