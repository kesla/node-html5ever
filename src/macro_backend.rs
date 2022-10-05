pub(crate) mod children {

  use napi::{bindgen_prelude::Reference, Result};

  use crate::{Element, Handle};

  pub(crate) fn get_children(handle: Handle) -> Result<Vec<Reference<Element>>> {
    handle
      .get_children()
      .iter()
      .filter_map(|handle| handle.as_element().ok().map(|r| r.clone(r.env)))
      .collect()
  }

  pub(crate) fn append_child(parent_handle: Handle, child_handle: Handle) -> Result<()> {
    parent_handle.append_handle(&child_handle)
  }

  pub(crate) fn remove_element(parent_handle: Handle, child_handle: Handle) {
    parent_handle.remove_handle(&child_handle);
  }

  pub(crate) fn get_element_by_id(
    handle: Handle,
    id: String,
  ) -> Result<Option<Reference<Element>>> {
    let mut q: Vec<Handle> = handle.get_children().iter().rev().cloned().collect();

    while let Some(handle) = q.pop() {
      if let Ok(element) = handle.as_element() {
        if element.get_id() == id {
          return Ok(Some(element.clone(element.env)?));
        }

        q.extend(handle.get_children().iter().rev().cloned());
      }
    }

    Ok(None)
  }

  pub(crate) fn get_elements_by_class_name(
    handle: Handle,
    class_name: String,
  ) -> Result<Vec<Reference<Element>>> {
    let mut q: Vec<Handle> = handle.get_children().iter().rev().cloned().collect();
    let mut elements = vec![];

    while let Some(handle) = q.pop() {
      if let Ok(element) = handle.as_element() {
        if element.get_class_name() == class_name {
          elements.push(element.clone(element.env)?);
        }

        q.extend(handle.get_children().iter().rev().cloned());
      }
    }

    Ok(elements)
  }
}

pub(crate) mod parent {
  use crate::{Comment, DocType, Document, Element, Handle, ParentContext, Text};
  use napi::{
    bindgen_prelude::{Either4, Reference, WeakReference},
    Either, Env, Result,
  };

  pub(crate) fn get_parent_element(handle: Handle) -> Result<Option<Reference<Element>>> {
    let parent_node = handle.get_parent_node();

    match parent_node {
      Ok(Some(Either::B(element))) => Ok(Some(element)),
      _ => Ok(None),
    }
  }

  pub(crate) fn get_parent_node(
    handle: Handle,
  ) -> Result<Option<Either<Reference<Document>, Reference<Element>>>> {
    handle.get_parent_node()
  }

  pub(crate) fn remove(child: Handle) -> Result<()> {
    child.remove()
  }

  pub(crate) fn owner_document(handle: Handle) -> Result<Option<Reference<Document>>> {
    let maybe_parent = handle.get_parent_node()?;

    match maybe_parent {
      Some(Either::A(document)) => Ok(Some(document)),
      Some(Either::B(element)) => element.owner_document(),
      None => Ok(None),
    }
  }

  pub(crate) fn get_previous_sibling(
    handle: Handle,
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
    Ok(handle.previous_iterator()?.next())
  }

  pub(crate) fn get_previous_element_sibling(
    handle: Handle,
  ) -> Result<Option<WeakReference<Element>>> {
    Ok(handle.previous_iterator()?.find_map(|s| match s {
      Either4::C(r) => Some(r),
      _ => None,
    }))
  }

  pub(crate) fn get_next_sibling(
    handle: Handle,
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
    Ok(handle.next_iterator()?.next())
  }

  pub(crate) fn get_next_element_sibling(handle: Handle) -> Result<Option<WeakReference<Element>>> {
    Ok(handle.next_iterator()?.find_map(|s| match s {
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
