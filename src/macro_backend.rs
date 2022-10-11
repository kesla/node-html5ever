pub(crate) mod children {

  use napi::{bindgen_prelude::Reference, Result};

  use crate::{ChildNode, Element, Handle, NodeHandler};

  pub(crate) fn get_child_nodes(node_handler: NodeHandler) -> Vec<ChildNode> {
    node_handler.child_nodes_iter(false).collect()
  }

  pub(crate) fn get_children(node_handler: NodeHandler) -> Vec<Reference<Element>> {
    node_handler.children_iter(false).collect()
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
  ) -> Option<Reference<Element>> {
    node_handler.children_iter(true).find(|e| e.get_id() == id)
  }

  pub(crate) fn get_elements_by_class_name(
    node_handler: NodeHandler,
    class_name: String,
  ) -> Vec<Reference<Element>> {
    node_handler
      .children_iter(true)
      .filter(|e| e.get_class_name() == class_name)
      .collect()
  }

  pub(crate) fn get_first_child(node_handler: NodeHandler) -> Option<ChildNode> {
    node_handler.child_nodes_iter(false).next()
  }

  pub(crate) fn get_last_child(node_handler: NodeHandler) -> Option<ChildNode> {
    node_handler.child_nodes_iter(false).last()
  }

  pub(crate) fn get_first_element_child(node_handler: NodeHandler) -> Option<Reference<Element>> {
    node_handler.children_iter(false).next()
  }

  pub(crate) fn get_last_element_child(node_handler: NodeHandler) -> Option<Reference<Element>> {
    node_handler.children_iter(false).last()
  }
}

pub(crate) mod parent {
  use crate::{ChildNode, Document, DocumentFragment, Element, Handle, NodeHandler};
  use fallible_iterator::FallibleIterator;
  use napi::{
    bindgen_prelude::{Either3, Reference},
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

  pub(crate) fn get_previous_sibling(node_handler: NodeHandler) -> Result<Option<ChildNode>> {
    node_handler.previous_iterator()?.next()
  }

  pub(crate) fn get_previous_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    node_handler.previous_iterator()?.find_map(|s| match s {
      ChildNode::Element(element) => Ok(Some(element)),
      _ => Ok(None),
    })
  }

  pub(crate) fn get_next_sibling(node_handler: NodeHandler) -> Result<Option<ChildNode>> {
    node_handler.next_iterator()?.next()
  }

  pub(crate) fn get_next_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    node_handler.next_iterator()?.find_map(|s| match s {
      ChildNode::Element(element) => Ok(Some(element)),
      _ => Ok(None),
    })
  }
}

mod all_nodes {
  use crate::Handle;

  pub(crate) fn get_node_name(handle: Handle) -> String {
    handle.get_node_name()
  }
}

pub(crate) use all_nodes::*;
