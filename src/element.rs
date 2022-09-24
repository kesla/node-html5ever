use html5ever::{tendril::StrTendril, Attribute, LocalName, Namespace, QualName};
use napi::bindgen_prelude::Reference;

use crate::{handle::Handle, node_list::NodeList, serialize::serialize};

#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Element {
  #[default(NodeList::new(env)?)]
  pub(crate) list: Reference<NodeList>,

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
      .iter()
      .filter_map(|handle| {
        handle
          .into_element()
          .ok()
          .map(|r| r.clone(self.env).unwrap())
      })
      .collect()
  }

  #[napi(getter)]
  pub fn inner_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(
      &handle,
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter)]
  pub fn outer_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(&handle, html5ever::serialize::TraversalScope::IncludeNode)
  }

  #[napi]
  pub fn append_element(&mut self, element: &Element) -> u32 {
    element.id.try_into().unwrap()
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
