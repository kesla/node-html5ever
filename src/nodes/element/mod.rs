mod attributes_wrapper;
mod impl_selectors;

use html5ever::{LocalName, QualName};

use crate::serialize;

use attributes_wrapper::AttributesWrapper;

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
  pub fn get_tag_name(&self) -> String {
    self.get_node_name()
  }

  #[napi(getter, js_name = "innerHTML")]
  pub fn inner_html(&self) -> String {
    serialize(
      self.into(),
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter, js_name = "outerHTML")]
  pub fn outer_html(&self) -> String {
    serialize(
      self.into(),
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
    println!("Dropping element <{}>", self.name.local);
  }
}
