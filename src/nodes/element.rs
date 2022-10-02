use html5ever::{tendril::StrTendril, Attribute, LocalName, Namespace, QualName};

use crate::serialize;

#[create_node(has_children, is_child)]
pub struct Element {
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
    self
      .attributes_wrapper
      .set_attribute(LocalName::from(name), value.into());
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

  #[napi(getter)]
  pub fn inner_html(&self) -> String {
    serialize(
      self.get_handle(),
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter)]
  pub fn outer_html(&self) -> String {
    serialize(
      self.get_handle(),
      html5ever::serialize::TraversalScope::IncludeNode,
    )
  }

  #[napi(getter)]
  pub fn get_class_name(&self) -> String {
    self
      .attributes_wrapper
      .get_attribute(LocalName::from("class"))
      .map(|attribute| attribute.value.to_string())
      .unwrap_or_default()
  }

  #[napi(setter)]
  pub fn set_class_name(&mut self, class_name: String) {
    self
      .attributes_wrapper
      .set_attribute(LocalName::from("class"), class_name.into());
  }

  #[napi(getter)]
  pub fn get_id(&self) -> String {
    self
      .attributes_wrapper
      .get_attribute(LocalName::from("id"))
      .map(|attribute| attribute.value.to_string())
      .unwrap_or_default()
  }

  #[napi(setter)]
  pub fn set_id(&mut self, id: String) {
    self
      .attributes_wrapper
      .set_attribute(LocalName::from("id"), id.into());
  }
}

impl Drop for Element {
  fn drop(&mut self) {
    log::debug!("Dropping element <{}>", self.name.local);
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

  pub(crate) fn set_attribute(&mut self, name: LocalName, value: StrTendril) {
    self.remove_attribute(name.clone());
    self.add_attribute(name, value);
  }

  pub(crate) fn push(&mut self, attribute: Attribute) {
    self.attrs.push(attribute)
  }

  pub(crate) fn iter(&self) -> std::slice::Iter<Attribute> {
    (&self.attrs).iter()
  }
}
