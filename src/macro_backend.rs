pub(crate) mod children {

  use napi::{
    bindgen_prelude::{Either4, Reference, WeakReference},
    Result,
  };

  use crate::{Comment, DocType, Element, NodeHandler, Text};

  pub(crate) fn get_child_nodes(
    node_handler: NodeHandler,
  ) -> Vec<
    Either4<
      WeakReference<Comment>,
      WeakReference<DocType>,
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

  pub(crate) fn get_children(node_handler: NodeHandler) -> Result<Vec<Reference<Element>>> {
    node_handler
      .get_child_nodes()
      .iter()
      .filter_map(|node_handler| node_handler.as_element().ok().map(|r| r.clone(r.env)))
      .collect()
  }

  pub(crate) fn append_child(
    parent_node_handler: NodeHandler,
    child_node_handler: NodeHandler,
  ) -> Result<()> {
    parent_node_handler.append_node_handler(&child_node_handler)
  }

  pub(crate) fn remove_element(parent_node_handler: NodeHandler, child_node_handler: NodeHandler) {
    parent_node_handler.remove_node_handler(&child_node_handler);
  }

  pub(crate) fn get_element_by_id(
    node_handler: NodeHandler,
    id: String,
  ) -> Result<Option<Reference<Element>>> {
    let mut q: Vec<NodeHandler> = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();

    while let Some(node_handler) = q.pop() {
      if let Ok(element) = node_handler.as_element() {
        if element.get_id() == id {
          return Ok(Some(element.clone(element.env)?));
        }

        q.extend(node_handler.get_child_nodes().iter().rev().cloned());
      }
    }

    Ok(None)
  }

  pub(crate) fn get_elements_by_class_name(
    node_handler: NodeHandler,
    class_name: String,
  ) -> Result<Vec<Reference<Element>>> {
    let mut q: Vec<NodeHandler> = node_handler
      .get_child_nodes()
      .iter()
      .rev()
      .cloned()
      .collect();
    let mut elements = vec![];

    while let Some(node_handler) = q.pop() {
      if let Ok(element) = node_handler.as_element() {
        if element.get_class_name() == class_name {
          elements.push(element.clone(element.env)?);
        }

        q.extend(node_handler.get_child_nodes().iter().rev().cloned());
      }
    }

    Ok(elements)
  }
}

pub(crate) mod parent {
  use crate::{Comment, DocType, Document, Element, NodeHandler, ParentContext, Text};
  use napi::{
    bindgen_prelude::{Either4, Reference, WeakReference},
    Either, Env, Result,
  };

  pub(crate) fn get_parent_element(
    node_handler: NodeHandler,
  ) -> Result<Option<Reference<Element>>> {
    let parent_node = node_handler.get_parent_node();

    match parent_node {
      Ok(Some(Either::B(element))) => Ok(Some(element)),
      _ => Ok(None),
    }
  }

  pub(crate) fn get_parent_node(
    node_handler: NodeHandler,
  ) -> Result<Option<Either<Reference<Document>, Reference<Element>>>> {
    node_handler.get_parent_node()
  }

  pub(crate) fn remove(child: NodeHandler) -> Result<()> {
    child.remove()
  }

  pub(crate) fn owner_document(node_handler: NodeHandler) -> Result<Option<Reference<Document>>> {
    let maybe_parent = node_handler.get_parent_node()?;

    match maybe_parent {
      Some(Either::A(document)) => Ok(Some(document)),
      Some(Either::B(element)) => element.owner_document(),
      None => Ok(None),
    }
  }

  pub(crate) fn get_previous_sibling(
    node_handler: NodeHandler,
  ) -> Result<
    Option<
      Either4<
        WeakReference<Comment>,
        WeakReference<DocType>,
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
        WeakReference<DocType>,
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

mod base {
  use napi::{
    bindgen_prelude::{Reference, WeakReference},
    Env, Result,
  };

  pub(crate) fn upgrade_weak_reference<T>(
    env: Env,
    weak: &Option<WeakReference<T>>,
  ) -> Result<Reference<T>> {
    let weak: &WeakReference<T> = weak.as_ref().unwrap();
    weak.upgrade(env).map(std::option::Option::unwrap)
  }
}

pub(crate) use base::*;
