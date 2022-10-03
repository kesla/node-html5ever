pub(crate) mod children {
  use std::{cell::RefCell, collections::VecDeque, rc::Rc};

  use napi::{
    bindgen_prelude::{Either4, Reference},
    Result,
  };

  use crate::{Comment, DocType, Element, Handle, Text};

  pub(crate) fn get_children(list: Rc<RefCell<Vec<Handle>>>) -> Result<Vec<Reference<Element>>> {
    list
      .borrow()
      .iter()
      .filter_map(|handle| handle.as_element().ok().map(|r| r.clone(r.env)))
      .collect()
  }

  pub(crate) fn append_child(
    parent_handle: Handle,
    child: Either4<&Comment, &DocType, &Element, &Text>,
  ) {
    let child_handle = match child {
      Either4::A(r) => r.get_handle(),
      Either4::B(r) => r.get_handle(),
      Either4::C(r) => r.get_handle(),
      Either4::D(r) => r.get_handle(),
    };
    parent_handle.append_handle(child_handle);
  }

  pub(crate) fn remove_element(parent_handle: Handle, element: &Element) {
    let child: Handle = element.get_handle();

    parent_handle.remove_handle(child);
  }

  pub(crate) fn get_element_by_id(
    list: Rc<RefCell<Vec<Handle>>>,
    id: String,
  ) -> Result<Option<Reference<Element>>> {
    let mut q: VecDeque<Handle> = list.borrow().iter().cloned().collect();

    while let Some(handle) = q.pop_front() {
      if let Ok(element) = handle.as_element() {
        if element.get_id() == id {
          return Ok(Some(element.clone(element.env)?));
        }

        q.extend(element.list.borrow().iter().cloned());
      }
    }

    Ok(None)
  }

  pub(crate) fn get_elements_by_class_name(
    list: Rc<RefCell<Vec<Handle>>>,
    class_name: String,
  ) -> Result<Vec<Reference<Element>>> {
    let mut q: Vec<Handle> = list.borrow().iter().rev().cloned().collect();
    let mut elements = vec![];

    while let Some(handle) = q.pop() {
      if let Ok(element) = handle.as_element() {
        if element.get_class_name() == class_name {
          elements.push(element.clone(element.env)?);
        }

        q.extend(element.list.borrow().iter().rev().cloned());
      }
    }

    Ok(elements)
  }
}

pub(crate) mod parent {
  use std::{cell::RefCell, rc::Rc};

  use crate::{
    node_data::NodeData,
    nodes::{Comment, DocType, Text},
    Document, Element, Handle,
  };
  use napi::{
    bindgen_prelude::{Either4, Reference, WeakReference},
    Either, Env, Result,
  };

  type Parent = RefCell<Option<Either<WeakReference<Element>, WeakReference<Document>>>>;

  pub(crate) fn get_parent_element(
    env: Env,
    parent: &Parent,
  ) -> Result<Option<Reference<Element>>> {
    let parent_node = get_parent_node(env, parent);

    match parent_node {
      Ok(Some(Either::A(element))) => Ok(Some(element)),
      _ => Ok(None),
    }
  }

  pub(crate) fn get_parent_node(
    env: Env,
    parent: &Parent,
  ) -> Result<Option<Either<Reference<Element>, Reference<Document>>>> {
    let maybe_reference = parent.borrow();

    let r = match maybe_reference.as_ref() {
      Some(Either::A(weak_reference)) => weak_reference.upgrade(env)?.map(Either::A),
      Some(Either::B(weak_reference)) => weak_reference.upgrade(env)?.map(Either::B),
      None => None,
    };

    Ok(r)
  }

  pub(crate) fn remove(env: Env, parent: &Parent, child: &Handle) -> Result<()> {
    let maybe_handle = get_parent_handle(parent, env)?;

    match maybe_handle {
      Some(parent) => parent.remove_handle(child.clone()),
      None => {}
    }

    Ok(())
  }

  pub(crate) fn get_previous_sibling(
    env: Env,
    parent: &Parent,
    child: &Handle,
  ) -> Result<
    Option<Either4<Reference<Comment>, Reference<DocType>, Reference<Element>, Reference<Text>>>,
  > {
    let parent_node = get_parent_node(env, parent)?;

    let children = get_child_nodes(parent_node);
    let children = children.borrow();
    let sibling: &NodeData = match find_next(children.iter().rev(), child) {
      Some(handle) => handle,
      None => return Ok(None),
    };

    let previous = node_data_to_either(sibling, env)?;

    Ok(Some(previous))
  }

  pub(crate) fn get_next_sibling(
    env: Env,
    parent: &Parent,
    child: &Handle,
  ) -> Result<
    Option<Either4<Reference<Comment>, Reference<DocType>, Reference<Element>, Reference<Text>>>,
  > {
    let parent_node = get_parent_node(env, parent)?;

    let children = get_child_nodes(parent_node);
    let children = children.borrow();
    let sibling: &NodeData = match find_next(children.iter(), child) {
      Some(handle) => handle,
      None => return Ok(None),
    };

    let previous = node_data_to_either(sibling, env)?;

    Ok(Some(previous))
  }

  fn find_next<'a, I: Iterator<Item = &'a Handle>>(
    mut iter: I,
    child: &'a Handle,
  ) -> Option<&Handle> {
    while let Some(handle) = iter.next() {
      if handle == child {
        return iter.next();
      }
    }

    None
  }

  fn node_data_to_either(
    data: &NodeData,
    env: Env,
  ) -> Result<Either4<Reference<Comment>, Reference<DocType>, Reference<Element>, Reference<Text>>>
  {
    let either = match data {
      NodeData::Comment(r) => Either4::A(r.clone(env)?),
      NodeData::DocType(r) => Either4::B(r.clone(env)?),
      NodeData::Element(r) => Either4::C(r.clone(env)?),
      NodeData::Text(r) => Either4::D(r.clone(env)?),
      _ => unreachable!(),
    };
    Ok(either)
  }

  fn get_child_nodes(
    parent_node: Option<Either<Reference<Element>, Reference<Document>>>,
  ) -> Rc<RefCell<Vec<Handle>>> {
    match parent_node {
      Some(Either::A(element)) => element.list.clone(),
      Some(Either::B(document)) => document.list.clone(),
      None => unreachable!(),
    }
  }

  fn get_parent_handle(parent: &Parent, env: Env) -> Result<Option<Handle>> {
    match get_parent_node(env, parent)? {
      Some(Either::A(element)) => Ok(Some(element.get_handle())),
      Some(Either::B(document)) => Ok(Some(document.get_handle())),
      None => Ok(None),
    }
  }

  pub(crate) fn owner_document(
    env: Env,
    parent: &Parent,
  ) -> Result<Option<WeakReference<Document>>> {
    match parent.borrow().as_ref() {
      Some(Either::A(r)) => match r.upgrade(env)? {
        Some(element) => element.owner_document(),
        None => Ok(None),
      },
      Some(Either::B(document)) => Ok(Some(document.clone())),
      None => Ok(None),
    }
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
