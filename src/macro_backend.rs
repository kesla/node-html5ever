pub(crate) mod children {

  use napi::{
    bindgen_prelude::{Either4, Reference, WeakReference},
    Result,
  };

  use crate::{Comment, DocumentType, Element, Handle, NodeHandler, Text};

  pub(crate) fn get_child_nodes(
    node_handler: NodeHandler,
  ) -> Vec<
    Either4<
      WeakReference<Comment>,
      WeakReference<DocumentType>,
      WeakReference<Element>,
      WeakReference<Text>,
    >,
  > {
    node_handler
      .get_child_nodes()
      .iter()
      .map(|child| child.into())
      .collect()
  }

  pub(crate) fn get_children(node_handler: NodeHandler) -> Vec<WeakReference<Element>> {
    node_handler
      .get_child_nodes()
      .iter()
      .filter_map(|node_handler| node_handler.as_element().ok().map(|r| r.downgrade()))
      .collect()
  }

  pub(crate) fn append_child(parent_handle: Handle, child_handle: Handle) -> Result<()> {
    parent_handle.append_handle(&child_handle)
  }

  pub(crate) fn remove_element(parent_handle: Handle, child_handle: Handle) {
    parent_handle.remove_handle(&child_handle);
  }

  pub(crate) fn get_element_by_id(
    node_handler: NodeHandler,
    id: String,
  ) -> Result<Option<Reference<Element>>> {
    let mut q: Vec<Handle> = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();

    while let Some(handle) = q.pop() {
      if let Ok(element) = handle.as_element() {
        if element.get_id() == id {
          return Ok(Some(element.clone(element.env)?));
        }

        let node_handler: NodeHandler = handle.into();
        q.extend(node_handler.get_child_nodes().iter().rev().cloned());
      }
    }

    Ok(None)
  }

  pub(crate) fn get_elements_by_class_name(
    node_handler: NodeHandler,
    class_name: String,
  ) -> Result<Vec<Reference<Element>>> {
    let mut q: Vec<Handle> = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();
    let mut elements = vec![];

    while let Some(handle) = q.pop() {
      if let Ok(element) = handle.as_element() {
        if element.get_class_name() == class_name {
          elements.push(element.clone(element.env)?);
        }

        let node_handler: NodeHandler = handle.into();
        q.extend(node_handler.get_child_nodes().iter().rev().cloned());
      }
    }

    Ok(elements)
  }
}

pub(crate) mod parent {
  use crate::{
    Comment, Document, DocumentFragment, DocumentType, Element, Handle, NodeHandler, Text,
  };
  use napi::{
    bindgen_prelude::{Either3, Either4, Reference, WeakReference},
    Result,
  };

  pub(crate) fn get_parent_element(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    let parent_node = get_parent_node(node_handler);

    match parent_node {
      Ok(Some(Either3::C(element))) => Ok(Some(element)),
      _ => Ok(None),
    }
  }

  pub(crate) fn get_parent_node(
    node_handler: NodeHandler,
  ) -> Result<Option<Either3<Reference<Document>, Reference<DocumentFragment>, Reference<Element>>>>
  {
    let parent = node_handler.get_parent();
    let parent = parent.as_ref();
    let parent = match parent {
      Some(parent) => parent,
      None => return Ok(None),
    };
    let parent: Either3<Reference<Document>, Reference<DocumentFragment>, Reference<Element>> =
      parent.try_into()?;
    Ok(Some(parent))
  }

  pub(crate) fn remove(child: Handle) -> Result<()> {
    child.remove()
  }

  pub(crate) fn owner_document(node_handler: NodeHandler) -> Result<Option<Reference<Document>>> {
    let maybe_parent = get_parent_node(node_handler)?;

    match maybe_parent {
      Some(Either3::A(document)) => Ok(Some(document)),
      Some(Either3::B(_document_fragment)) => Ok(None),
      Some(Either3::C(element)) => element.owner_document(),
      None => Ok(None),
    }
  }

  pub(crate) fn get_previous_sibling(
    node_handler: NodeHandler,
  ) -> Result<
    Option<
      Either4<
        WeakReference<Comment>,
        WeakReference<DocumentType>,
        WeakReference<Element>,
        WeakReference<Text>,
      >,
    >,
  > {
    Ok(node_handler.previous_iterator()?.next())
  }

  pub(crate) fn get_previous_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<WeakReference<Element>>> {
    Ok(node_handler.previous_iterator()?.find_map(|s| match s {
      Either4::C(r) => Some(r),
      _ => None,
    }))
  }

  pub(crate) fn get_next_sibling(
    node_handler: NodeHandler,
  ) -> Result<
    Option<
      Either4<
        WeakReference<Comment>,
        WeakReference<DocumentType>,
        WeakReference<Element>,
        WeakReference<Text>,
      >,
    >,
  > {
    Ok(node_handler.next_iterator()?.next())
  }

  pub(crate) fn get_next_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<WeakReference<Element>>> {
    Ok(node_handler.next_iterator()?.find_map(|s| match s {
      Either4::C(r) => Some(r),
      _ => None,
    }))
  }
}

mod all_nodes {
  use crate::Handle;

  pub(crate) fn get_node_name(handle: Handle) -> String {
    handle.get_node_name()
  }
}

pub(crate) use all_nodes::*;
