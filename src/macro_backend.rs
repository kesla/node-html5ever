pub(crate) mod children {

  use napi::{bindgen_prelude::Reference, Result};

  use crate::{ChildNode, Element, Node, NodeHandler};

  pub(crate) fn get_child_nodes(node_handler: NodeHandler) -> Vec<ChildNode> {
    node_handler.child_nodes_iter(false).collect()
  }

  pub(crate) fn get_children(node_handler: NodeHandler) -> Vec<Reference<Element>> {
    node_handler.children_iter(false).collect()
  }

  pub(crate) fn append_child(parent_node: Node, child_node: ChildNode) -> Result<()> {
    parent_node.append_node(&child_node)
  }

  pub(crate) fn remove_element(parent_node: Node, child_node: ChildNode) {
    parent_node.remove_node(&child_node);
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
  use crate::{ChildNode, Document, Element, NodeHandler, ParentNode};
  use napi::{
    bindgen_prelude::{Reference, WeakReference},
    Result,
  };

  pub(crate) fn get_parent_element(
    node_handler: NodeHandler,
  ) -> Result<Option<WeakReference<Element>>> {
    match node_handler.parent_iterator().next() {
      Some(Ok(ParentNode::Element(element))) => Ok(Some(element)),
      Some(Err(err)) => Err(err),
      _ => Ok(None),
    }
  }

  pub(crate) fn get_parent_node(node_handler: NodeHandler) -> Result<Option<ParentNode>> {
    node_handler.parent_iterator().next().transpose()
  }

  pub(crate) fn remove(child: ChildNode) -> Result<()> {
    child.remove()
  }

  pub(crate) fn owner_document(
    node_handler: NodeHandler,
  ) -> Result<Option<WeakReference<Document>>> {
    node_handler
      .parent_iterator()
      .find_map(|node| match node {
        Ok(ParentNode::Document(document)) => Some(Ok(document)),
        Err(err) => Some(Err(err)),
        _ => None,
      })
      .transpose()
  }

  pub(crate) fn get_previous_sibling(node_handler: NodeHandler) -> Result<Option<ChildNode>> {
    Ok(node_handler.previous_iterator()?.next())
  }

  pub(crate) fn get_previous_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    Ok(node_handler.previous_iterator()?.next())
  }

  pub(crate) fn get_next_sibling(node_handler: NodeHandler) -> Result<Option<ChildNode>> {
    Ok(node_handler.next_iterator()?.next())
  }

  pub(crate) fn get_next_element_sibling(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    Ok(node_handler.next_iterator()?.next())
  }
}

mod all_nodes {
  use crate::Node;

  pub(crate) fn get_node_name(handle: Node) -> String {
    handle.get_node_name()
  }
}

pub(crate) use all_nodes::*;
