mod attributes;
mod class_list;
mod element_ref;

use html5ever::{LocalName, QualName};
use napi::{
  bindgen_prelude::{Object, Reference},
  Result,
};
use regex::Regex;

use crate::{serialize, Text};

use attributes::{Attr, AttributesWrapper};
use class_list::ClassList;
pub use element_ref::ElementRef;

#[create_node(has_children, is_child)]
pub struct Element {
  pub(crate) attributes_wrapper: AttributesWrapper,

  pub(crate) class_list: Option<Reference<ClassList>>,

  pub(crate) name: QualName,
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
    if name == "class".to_string() {
      if let Some(class_list) = &mut self.class_list {
        class_list.clear()?;
      }
    }

    self.attributes_wrapper.remove_attribute(name.into());

    Ok(())
  }

  #[napi]
  pub fn set_attribute(&mut self, name: String, value: String) -> Result<()> {
    if name == "class".to_string() {
      if let Some(class_list) = &mut self.class_list {
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
  pub fn get_class_list(&mut self, element: Reference<Element>) -> Result<Reference<ClassList>> {
    if let Some(class_list) = &self.class_list {
      class_list.clone(self.env)
    } else {
      let class_list = ClassList::new(
        element.downgrade(),
        self.env,
        self.get_attribute("class".to_string()),
      )?;

      self.class_list = Some(class_list.clone(self.env)?);
      Ok(class_list)
    }
  }

  #[napi(getter)]
  pub fn get_style(&self) -> Result<Object> {
    let style = self
      .attributes_wrapper
      .get_attribute(LocalName::from("style"))
      .map(|attribute| attribute.value.to_string())
      .unwrap_or_default();

    style
      .split(';')
      .try_fold(self.env.create_object()?, |mut obj, style| {
        let mut style = style.split(':').map(|s| s.trim());
        let key = style.next();
        let value = style.next();

        if let (Some(key), Some(value)) = (key, value) {
          if key.len() > 0 && value.len() > 0 {
            let re = Regex::new(r"(-?\w+)-?(.*)").unwrap();

            let caps = re.captures(key).unwrap();

            let mut key = String::new();
            key.push_str(match &caps[1] {
              "-webkit" => "Webkit",
              "-moz" => "Moz",
              "-ms" => "ms",
              "-o" => "O",
              _ => &caps[1],
            });

            if let Some(rest) = caps.get(2) {
              rest
                .as_str()
                .split('-')
                .filter(|part| part.trim().len() > 0)
                .for_each(|part| {
                  let mut chars = part.chars();
                  let first = chars.next().unwrap().to_uppercase().to_string();
                  let rest = chars.collect::<String>();
                  key.push_str(&first);
                  key.push_str(&rest);
                });
            }

            obj.set(key, value.trim().to_string())?;
          }
        }

        Ok(obj)
      })
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
