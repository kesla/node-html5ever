mod attributes;
mod class_list;
mod element_ref;

use crate::StyleDeclaration;
use html5ever::{LocalName, QualName};
use napi::{bindgen_prelude::Reference, Result};

use crate::{serialize, LazyReference, Text};

use attributes::{Attr, AttributesWrapper};
use class_list::ClassList;
pub use element_ref::ElementRef;

#[create_node(has_children, is_child)]
pub struct Element {
  pub(crate) attributes_wrapper: AttributesWrapper,

  pub(crate) name: QualName,

  pub(crate) lazy_class_list: LazyReference<ClassList>,
  pub(crate) lazy_style: LazyReference<StyleDeclaration>,
}

#[napi]
impl Element {
  #[napi(getter)]
  pub fn get_attributes(&self, r: Reference<Element>) -> Vec<Attr> {
    self.attributes_wrapper.get_attributes(r)
  }

  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .attributes_wrapper
      .get_attribute(LocalName::from(name))
      .map(|attribute| attribute.value.to_string())
  }

  #[napi]
  pub fn remove_attribute(&mut self, name: String) -> Result<()> {
    if name == *"class" {
      if let Some(class_list) = &mut self.lazy_class_list.get_mut() {
        class_list.clear()?;
      }
    }

    self.attributes_wrapper.remove_attribute(name.into());

    Ok(())
  }

  #[napi]
  pub fn set_attribute(&mut self, name: String, value: String) -> Result<()> {
    if name == *"class" {
      if let Some(class_list) = &mut self.lazy_class_list.get_mut() {
        // attribute is set in ClassList::set_value
        class_list.set_value(value)?;

        return Ok(());
      }
    }

    self
      .attributes_wrapper
      .set_attribute(LocalName::from(name), value.into());

    Ok(())
  }

  #[napi]
  pub fn has_attribute(&self, name: String) -> bool {
    self.attributes_wrapper.has_attribute(name.into())
  }

  #[napi(getter)]
  pub fn get_class_list(
    &mut self,
    element: Reference<Element>,
  ) -> Result<Reference<ClassList>> {
    let initial_value = self.get_attribute("class".to_string());
    self.lazy_class_list.get_or_init(|| {
      ClassList::new(element.downgrade(), self.env, initial_value)
    })
  }

  #[napi(getter)]
  pub fn get_style(&mut self) -> Result<Reference<StyleDeclaration>> {
    let initial_value = self.get_attribute("style".to_string());
    self
      .lazy_style
      .get_or_init(|| StyleDeclaration::new_reference(self.env, initial_value))
  }

  #[napi(getter)]
  pub fn get_tag_name(&self) -> String {
    self.get_node_name()
  }

  #[napi(getter, js_name = "innerHTML")]
  pub fn get_inner_html(&self) -> String {
    serialize(
      self.into(),
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter, js_name = "outerHTML")]
  pub fn gete_outer_html(&self) -> String {
    serialize(
      self.into(),
      html5ever::serialize::TraversalScope::IncludeNode,
    )
  }

  #[napi(getter)]
  pub fn get_text_content(&self) -> Option<String> {
    let text = self
      .get_node_handler()
      .deep_child_nodes_iter::<Reference<Text>>()
      .map(|text| text.data.clone())
      .collect();

    Some(text)
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
