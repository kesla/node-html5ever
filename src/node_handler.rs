use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

use napi::{
  bindgen_prelude::{Either4, Reference, WeakReference},
  Either, Env, Error, Result, Status,
};

use crate::{get_id, Comment, DocType, Document, Element, Text};

mod child_node_list;
mod iterators;
mod node_reference;
mod parent_context;
mod weak;

pub(crate) use self::{
  node_reference::NodeReference, parent_context::ParentContext, weak::WeakNodeHandler,
};

use self::{
  child_node_list::ChildNodeList,
  iterators::{NextIterator, PreviousIterator},
};

struct NodeHandlerInner {
  env: Env,
  id: usize,
  node: NodeReference,
  list: RefCell<ChildNodeList>,
  parent_context: RefCell<Option<ParentContext>>,
}

impl Drop for NodeHandlerInner {
  fn drop(&mut self) {
    let node_type: String = match &self.node {
      NodeReference::Comment(_) => "Comment".to_string(),
      NodeReference::DocType(_) => "DocType".to_string(),
      NodeReference::Document(_) => "Document".to_string(),
      NodeReference::Element(element) => format!("Element <{}>", element.name.local),
      NodeReference::Text(_) => "Text".to_string(),
    };

    println!("Dropping NodeHandlerInner {:?}", node_type);
  }
}

#[derive(Clone)]
pub struct NodeHandler(Rc<NodeHandlerInner>);

impl NodeHandler {
  pub(crate) fn new(env: Env, node: NodeReference) -> Self {
    NodeHandler(Rc::new(NodeHandlerInner {
      env,
      id: get_id(),
      list: Default::default(),
      node,
      parent_context: RefCell::new(None),
    }))
  }

  pub(crate) fn get_child_nodes(&self) -> Ref<ChildNodeList> {
    self.0.list.borrow()
  }

  pub(crate) fn get_child_nodes_mut(&self) -> RefMut<ChildNodeList> {
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

  pub(crate) fn get_parent_node_handler(&self) -> Result<Option<NodeHandler>> {
    match self.get_parent_node()? {
      Some(Either::A(element)) => Ok(Some(element.get_node_handler())),
      Some(Either::B(document)) => Ok(Some(document.get_node_handler())),
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

  pub(crate) fn append_node_handler(&self, child: &NodeHandler) -> Result<()> {
    // remove from old parent
    child.remove()?;

    // TODO: concatenate already existing text node

    let mut children = self.get_child_nodes_mut();
    children.append_node_handler(child.clone());

    let parent_reference = match &self.get_node_reference() {
      NodeReference::Document(r) => Either::A(r.downgrade()),
      NodeReference::Element(r) => Either::B(r.downgrade()),
      _ => panic!("Wrong type"),
    };
    let parent_context = Some(ParentContext::new(
      self.0.env,
      parent_reference,
      children.len() - 1,
    ));
    let mut parent = child.get_parent_mut();
    *parent = parent_context;
    Ok(())
  }

  pub(crate) fn remove_node_handler(&self, child: &NodeHandler) {
    let mut children = self.get_child_nodes_mut();
    children.remove_node_handler(child);

    let mut parent = child.get_parent_mut();
    *parent = None;
  }

  pub(crate) fn remove(&self) -> Result<()> {
    let maybe_node_handler = self.get_parent_node_handler()?;

    match maybe_node_handler {
      Some(parent) => parent.remove_node_handler(self),
      None => {}
    }

    Ok(())
  }

  pub(crate) fn downgrade(&self) -> WeakNodeHandler {
    self.into()
  }

  pub(crate) fn previous_iterator(&self) -> Result<PreviousIterator> {
    let maybe_parent_context = self.get_parent();
    let maybe_parent_context = maybe_parent_context.as_ref();

    match maybe_parent_context {
      Some(ctx) => Ok(PreviousIterator::Data {
        node_handler: ctx.try_into()?,
        index: ctx.index,
      }),
      None => Ok(PreviousIterator::None),
    }
  }

  pub(crate) fn next_iterator(&self) -> Result<NextIterator> {
    let maybe_parent_context = self.get_parent();
    let maybe_parent_context = maybe_parent_context.as_ref();

    match maybe_parent_context {
      Some(ctx) => Ok(NextIterator::Data {
        node_handler: ctx.try_into()?,
        index: ctx.index,
      }),
      None => Ok(NextIterator::None),
    }
  }
}

impl TryFrom<&ParentContext> for NodeHandler {
  type Error = Error;

  fn try_from(parent_context: &ParentContext) -> Result<Self> {
    match &parent_context.node {
      Either::A(document) => {
        let document = document
          .upgrade(parent_context.env)?
          .expect("Document is gone");
        Ok(document.get_node_handler())
      }
      Either::B(element) => {
        let element = element
          .upgrade(parent_context.env)?
          .expect("Element is gone");
        Ok(element.get_node_handler())
      }
    }
  }
}

impl From<Either<&Document, &Element>> for NodeHandler {
  fn from(e: Either<&Document, &Element>) -> Self {
    match e {
      Either::A(r) => r.into(),
      Either::B(r) => r.into(),
    }
  }
}

impl From<Either4<&Comment, &DocType, &Element, &Text>> for NodeHandler {
  fn from(e: Either4<&Comment, &DocType, &Element, &Text>) -> Self {
    match e {
      Either4::A(r) => r.into(),
      Either4::B(r) => r.into(),
      Either4::C(r) => r.into(),
      Either4::D(r) => r.into(),
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
  > for &NodeHandler
{
  fn into(
    self,
  ) -> Either4<
    WeakReference<Comment>,
    WeakReference<DocType>,
    WeakReference<Element>,
    WeakReference<Text>,
  > {
    match self.get_node_reference() {
      NodeReference::Comment(r) => Either4::A(r.downgrade()),
      NodeReference::DocType(r) => Either4::B(r.downgrade()),
      NodeReference::Element(r) => Either4::C(r.downgrade()),
      NodeReference::Text(r) => Either4::D(r.downgrade()),
      NodeReference::Document(_) => unreachable!("Document is not a Node"),
    }
  }

  // fn into(
  //   self,
  // ) ->
  //   Either4<
  //     WeakReference<Comment>,
  //     WeakReference<DocType>,
  //     WeakReference<Element>,
  //     WeakReference<Text>,
  //   >,
  //  {
  //   match self.get_node_reference() {
  //     NodeReference::Comment(r) => Either4::A(r.downgrade()),
  //     NodeReference::DocType(r) => Either4::B(r.downgrade()),
  //     NodeReference::Element(r) => Either4::C(r.downgrade()),
  //     NodeReference::Text(r) => Either4::D(r.downgrade()),
  //     NodeReference::Document(_) => unreachable!("Document is not a Node"),
  //   }
  // }
}

impl PartialEq for NodeHandler {
  fn eq(&self, other: &Self) -> bool {
    self.0.id == other.0.id
  }
}

impl Eq for NodeHandler {}
