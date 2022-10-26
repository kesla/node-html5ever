mod properties;

use napi::{bindgen_prelude::Reference, Env, Result};

use crate::{
  to_css_camel_case, to_css_kebab_case, CyclicReference, WithDataInBrackets,
};

#[derive(Debug)]
struct Data {
  property: String,
  value: String,
  important: bool,
}

#[napi]
pub struct StyleDeclaration {
  data: Vec<Data>,
  env: Env,
  cyclic_reference: CyclicReference<Self>,
}

impl WithDataInBrackets for StyleDeclaration {
  #[inline]
  fn raw_item(&self, index: usize) -> Option<String> {
    self.data.get(index).map(|d| to_css_kebab_case(&d.property))
  }

  #[inline]
  fn get_reference(&self) -> Result<Reference<Self>> {
    self.cyclic_reference.get()
  }

  #[inline]
  fn get_env(&self) -> Env {
    self.env
  }
}

#[napi]
impl StyleDeclaration {
  pub(crate) fn new_reference(
    env: Env,
    initial_value: Option<String>,
  ) -> Result<Reference<Self>> {
    let data = initial_value.map_or_else(Vec::new, string_to_data);
    let r = CyclicReference::<Self>::new_cyclic(env, |cyclic_reference| {
      Self::into_reference(
        Self {
          data,
          env,
          cyclic_reference,
        },
        env,
      )
    })?;

    r.clone(env)?.set_properties()?;
    r.clone(env)
  }

  fn get_data_mut(&mut self, property: &String) -> Option<&mut Data> {
    let property = to_css_camel_case(property);

    self.data.iter_mut().find(|data| data.property == property)
  }

  fn get_data(&self, property: &String) -> Option<&Data> {
    let property = to_css_camel_case(property);

    self.data.iter().find(|data| data.property == property)
  }

  #[napi]
  pub fn get_property_value(&self, property: String) -> String {
    self
      .get_data(&property)
      .map_or_else(|| String::from(""), |data| data.value.to_owned())
  }

  #[napi]
  pub fn get_property_priority(&self, property: String) -> String {
    self.get_data(&property).map_or_else(
      || String::from(""),
      |data| {
        if data.important {
          String::from("important")
        } else {
          String::from("")
        }
      },
    )
  }

  #[napi]
  pub fn remove_property(&mut self, property: String) -> Result<String> {
    let camel = to_css_camel_case(property);

    let pos = self.data.iter().position(|data| data.property == camel);

    if let Some(pos) = pos {
      let result = self.data.remove(pos).value;
      self.set_properties()?;
      Ok(result)
    } else {
      Ok(String::from(""))
    }
  }

  #[napi]
  pub fn set_property(
    &mut self,
    property: String,
    value: String,
    priority: Option<String>,
  ) -> Result<()> {
    let important = priority.map_or(false, |priority| priority == "important");

    match self.get_data_mut(&property) {
      Some(data) => {
        data.value = value;
        data.important = important;
      }
      None => {
        self.data.push(Data {
          property: to_css_camel_case(property),
          value,
          important,
        });
      }
    };

    self.set_properties()
  }

  #[napi(getter)]
  pub fn get_css_text(&self) -> String {
    self
      .data
      .iter()
      .map(|data| {
        let property = to_css_kebab_case(&data.property);

        if data.important {
          format!("{}: {} !important;", property, data.value)
        } else {
          format!("{}: {};", property, data.value)
        }
      })
      .collect::<Vec<String>>()
      .join(" ")
  }

  #[napi(setter)]
  pub fn set_css_text(&mut self, css_text: String) -> Result<()> {
    self.data = string_to_data(css_text);
    self.set_properties()
  }

  #[napi(getter)]
  pub fn get_css_float(&self) -> String {
    self.get_property_value("css-float".into())
  }

  #[napi(setter)]
  pub fn set_css_float(&mut self, value: String) -> Result<()> {
    self.set_property("cssFloat".to_string(), value, None)
  }

  #[napi]
  pub fn item(&self, index: i64) -> Option<String> {
    usize::try_from(index)
      .ok()
      .and_then(|index| self.raw_item(index))
  }

  #[napi(getter)]
  pub fn get_length(&self) -> u32 {
    self.data.len() as u32
  }
}

fn string_to_data(css_text: String) -> Vec<Data> {
  css_text
    .split(';')
    .filter_map(|item| {
      let (property, mut value) = {
        let mut parts = item.split(':');

        let (property, value): (String, String) =
          match (parts.next(), parts.next(), parts.next()) {
            (Some(property), Some(value), None) => (
              to_css_camel_case(property.trim().to_string()),
              value.trim().to_string(),
            ),
            _ => return None,
          };

        if property.is_empty() || value.is_empty() {
          return None;
        }

        (property, value)
      };

      let important = value.ends_with("!important");

      if important {
        value = value.replace("!important", "").trim().to_string();
      }

      let data = Data {
        property,
        value,
        important,
      };

      Some(data)
    })
    .collect()
}
