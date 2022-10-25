pub(crate) mod children {

  use napi::{bindgen_prelude::Reference, Either, Env, Result};

  use crate::{ChildNode, Element, Node, NodeHandler, Text};

  pub(crate) fn children<T>(node_handler: NodeHandler) -> Vec<T>
  where
    ChildNode: TryInto<T>,
  {
    node_handler.shallow_child_nodes_iter().collect()
  }

  pub(crate) fn append_child(
    parent_node: Node,
    child_node: ChildNode,
  ) -> Result<ChildNode> {
    parent_node.append_node(&child_node)?;
    Ok(child_node)
  }

  pub(crate) fn prepend(
    env: Env,
    parent_node: Node,
    node: Either<ChildNode, String>,
  ) -> Result<()> {
    let child_node: ChildNode = match node {
      Either::A(child_node) => child_node,
      Either::B(data) => {
        let text = Text::new_reference(env, data)?;
        text.into()
      }
    };

    parent_node.prepend_node(&child_node)?;

    Ok(())
  }

  pub(crate) fn append(
    env: Env,
    parent_node: Node,
    node: Either<ChildNode, String>,
  ) -> Result<()> {
    let child_node: ChildNode = match node {
      Either::A(child_node) => child_node,
      Either::B(data) => {
        let text = Text::new_reference(env, data)?;
        text.into()
      }
    };

    parent_node.append_node(&child_node)?;

    Ok(())
  }

  pub(crate) fn remove_child(
    parent_node: Node,
    child_node: ChildNode,
  ) -> Result<ChildNode> {
    parent_node.remove_node(&child_node)?;
    Ok(child_node)
  }

  pub(crate) fn get_element_by_id(
    node_handler: NodeHandler,
    id: String,
  ) -> Option<Reference<Element>> {
    node_handler
      .deep_child_nodes_iter()
      .find(|e: &Reference<Element>| e.get_id() == id)
  }

  pub(crate) fn get_elements_by_class_name(
    node_handler: NodeHandler,
    class_name: String,
  ) -> Vec<Reference<Element>> {
    node_handler
      .deep_child_nodes_iter()
      .filter(|e: &Reference<Element>| e.get_class_name() == class_name)
      .collect()
  }

  pub(crate) fn get_elements_by_tag_name(
    node_handler: NodeHandler,
    tag_name: String,
  ) -> Vec<Reference<Element>> {
    let tag_name: &str = &tag_name;

    node_handler
      .deep_child_nodes_iter()
      .filter(|e: &Reference<Element>| {
        e.get_tag_name().eq_ignore_ascii_case(tag_name)
      })
      .collect()
  }

  pub(crate) fn first_child<T>(node_handler: NodeHandler) -> Option<T>
  where
    ChildNode: TryInto<T>,
  {
    node_handler.shallow_child_nodes_iter().next()
  }

  pub(crate) fn last_child<T>(node_handler: NodeHandler) -> Option<T>
  where
    ChildNode: TryInto<T>,
  {
    node_handler.shallow_child_nodes_iter().next_back()
  }

  pub(crate) fn query_selector_all(
    node_handler: NodeHandler,
    selectors: String,
  ) -> Result<Vec<Reference<Element>>> {
    node_handler.selectors_iter(selectors)?.collect()
  }

  pub(crate) fn query_selector(
    node_handler: NodeHandler,
    selectors: String,
  ) -> Result<Option<Reference<Element>>> {
    node_handler.selectors_iter(selectors)?.try_next()
  }
}

pub(crate) mod parent {
  use crate::{ChildNode, NodeHandler, ParentNode};
  use napi::Result;

  pub(crate) fn parent<T>(node_handler: NodeHandler) -> Result<Option<T>>
  where
    ParentNode: TryInto<T>,
  {
    node_handler.parent_iterator().try_next()
  }

  pub(crate) fn remove(child: ChildNode) -> Result<()> {
    child.remove()
  }

  pub(crate) fn previous<T>(node_handler: NodeHandler) -> Result<Option<T>>
  where
    ChildNode: TryInto<T>,
  {
    Ok(node_handler.previous_iterator()?.next())
  }

  pub(crate) fn next<T>(node_handler: NodeHandler) -> Result<Option<T>>
  where
    ChildNode: TryInto<T>,
  {
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
