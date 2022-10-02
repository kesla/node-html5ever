pub(crate) mod children {
  use std::{cell::RefCell, rc::Rc};

  use napi::{bindgen_prelude::Reference, Result};

  use crate::{Element, Handle};

  pub(crate) fn get_children(list: Rc<RefCell<Vec<Handle>>>) -> Result<Vec<Reference<Element>>> {
    list
      .borrow()
      .iter()
      .filter_map(|handle| handle.into_element().ok().map(|r| r.clone(r.env)))
      .collect()
  }

  pub(crate) fn append_child(
    parent_handle: Handle,
    child: napi::bindgen_prelude::Either4<
      &crate::Comment,
      &crate::DocType,
      &crate::Element,
      &crate::Text,
    >,
  ) {
    let child_handle = match child {
      napi::bindgen_prelude::Either4::A(r) => r.get_handle(),
      napi::bindgen_prelude::Either4::B(r) => r.get_handle(),
      napi::bindgen_prelude::Either4::C(r) => r.get_handle(),
      napi::bindgen_prelude::Either4::D(r) => r.get_handle(),
    };
    parent_handle.append_handle(child_handle);
  }

  pub(crate) fn remove_element(parent_handle: Handle, element: &crate::Element) {
    let child: crate::Handle = element.get_handle();

    parent_handle.remove_handle(child);
  }
}

pub(crate) mod parent {
  use std::cell::RefCell;

  use crate::{Document, Element, Handle};
  use napi::{bindgen_prelude::WeakReference, Either, Env, Result};

  type Parent = RefCell<Option<Either<WeakReference<Element>, WeakReference<Document>>>>;

  pub(crate) fn get_parent_element(parent: &Parent) -> Option<WeakReference<Element>> {
    let parent_node = parent.borrow();

    match parent_node.as_ref() {
      Some(napi::Either::A(element)) => Some(element.clone()),
      _ => None,
    }
  }

  pub(crate) fn get_parent_node(
    parent: &Parent,
  ) -> Option<
    napi::Either<
      napi::bindgen_prelude::WeakReference<crate::Element>,
      napi::bindgen_prelude::WeakReference<crate::Document>,
    >,
  > {
    let maybe_reference = parent.borrow();

    maybe_reference.as_ref().map(|value| match value {
      napi::Either::A(element) => napi::Either::A(element.clone()),
      napi::Either::B(document) => napi::Either::B(document.clone()),
    })
  }

  pub(crate) fn remove(env: Env, parent: &Parent, child: &Handle) -> Result<()> {
    let maybe_handle = into_parent_handle(parent, env)?;

    match maybe_handle {
      Some(parent) => parent.remove_handle(child.clone()),
      None => {}
    }

    Ok(())
  }

  fn into_parent_handle(parent: &Parent, env: Env) -> Result<Option<Handle>> {
    let parent_node = parent.borrow();

    let maybe_handle: Option<Handle> = match parent_node.as_ref() {
      Some(Either::A(weak_reference)) => weak_reference.upgrade(env)?.map(|r| r.get_handle()),
      Some(Either::B(weak_reference)) => weak_reference.upgrade(env)?.map(|r| r.get_handle()),
      None => None,
    };
    Ok(maybe_handle)
  }

  pub(crate) fn owner_document(
    env: Env,
    parent: &Parent,
  ) -> Result<Option<WeakReference<Document>>> {
    match parent.borrow().as_ref() {
      Some(napi::Either::A(r)) => match r.upgrade(env)? {
        Some(element) => element.owner_document(),
        None => Ok(None),
      },
      Some(napi::Either::B(document)) => Ok(Some(document.clone())),
      None => Ok(None),
    }
  }
}
