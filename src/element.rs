use std::{cell::RefCell, rc::Rc};

use html5ever::{tendril::StrTendril, Attribute, LocalName, Namespace, QualName};
use napi::{bindgen_prelude::Reference, Either};

use crate::{
  dom::{append_handle, Handle},
  serialize::serialize,
};

#[napi]
#[derive(NodeType)]
#[add_node_fields]
pub struct Element {
  #[default(Rc::new(RefCell::new(vec![])))]
  pub(crate) list: Rc<RefCell<Vec<Handle>>>,

  pub(crate) attributes_wrapper: AttributesWrapper,

  pub(crate) name: QualName,
}

#[napi]
impl Element {
  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .attributes_wrapper
      .get_attribute(LocalName::from(name))
      .map(|attribute| attribute.value.to_string())
  }

  #[napi]
  pub fn remove_attribute(&mut self, name: String) {
    self.attributes_wrapper.remove_attribute(name.into());
  }

  #[napi]
  pub fn set_attribute(&mut self, name: String, value: String) {
    let local = LocalName::from(name);

    self.attributes_wrapper.remove_attribute(local.clone());
    self
      .attributes_wrapper
      .add_attribute(local.clone(), value.into())
  }

  #[napi]
  pub fn has_attribute(&self, name: String) -> bool {
    self.attributes_wrapper.has_attribute(name.into())
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    self.name.local.to_string().to_uppercase()
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> String {
    self.node_name()
  }

  // TODO: cache this & perhaps return something nicer
  // remove .unwrap
  #[napi(getter)]
  pub fn get_children(&self) -> Vec<Reference<Element>> {
    self
      .list
      .borrow()
      .iter()
      .filter_map(|node| node.into_element().ok().map(|r| r.clone(self.env).unwrap()))
      .collect()
  }

  #[napi(getter)]
  pub fn inner_html(&self, reference: Reference<Element>) -> String {
    serialize(
      self.get_handle(reference),
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter)]
  pub fn outer_html(&self, reference: Reference<Element>) -> String {
    serialize(
      self.get_handle(reference),
      html5ever::serialize::TraversalScope::IncludeNode,
    )
  }

  #[napi]
  pub fn append_element(&mut self, me: Reference<Element>, element: &Element) {
    let child_weak_reference = element.r.as_ref().unwrap();
    let child: Handle =
      element.get_handle(child_weak_reference.upgrade(self.env).unwrap().unwrap());

    let parent: Handle = self.get_handle(me);

    append_handle(parent, child);
  }
}

impl Drop for Element {
  fn drop(&mut self) {
    println!("Dropping element <{}>", self.name.local);
  }
}

pub(crate) struct AttributesWrapper {
  attrs: Vec<Attribute>,
}

impl From<Vec<Attribute>> for AttributesWrapper {
  fn from(attrs: Vec<Attribute>) -> Self {
    Self { attrs }
  }
}

impl AttributesWrapper {
  pub(crate) fn get_attribute(&self, name: LocalName) -> Option<&Attribute> {
    self.iter().find(|attribute| attribute.name.local == name)
  }

  pub(crate) fn has_attribute(&self, name: LocalName) -> bool {
    self.get_attribute(name).is_some()
  }

  pub(crate) fn remove_attribute(&mut self, name: LocalName) {
    self.attrs.retain(|attribute| attribute.name.local != name)
  }

  pub(crate) fn add_attribute(&mut self, name: LocalName, value: StrTendril) {
    let attribute_name = QualName::new(None, Namespace::from(""), name);
    let new_attribute = Attribute {
      name: attribute_name,
      value,
    };
    self.push(new_attribute);
  }

  pub(crate) fn push(&mut self, attribute: Attribute) {
    self.attrs.push(attribute)
  }

  pub(crate) fn iter(&self) -> std::slice::Iter<Attribute> {
    let attrs = &self.attrs;
    attrs.into_iter()
  }
}
