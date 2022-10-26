mod properties;

use convert_case::{Case, Converter as CaseConverter};
use napi::{bindgen_prelude::Reference, Env, Result};

lazy_static! {
  static ref TO_CAMEL_CASE: CaseConverter =
    CaseConverter::new().to_case(Case::Camel);
  static ref TO_KEBAB_CASE: CaseConverter =
    CaseConverter::new().to_case(Case::Kebab);
}

#[derive(Debug)]
struct Data {
  property: String,
  value: String,
  important: bool,
}

#[napi]
pub struct StyleDeclaration {
  data: Vec<Data>,
}

#[napi]
impl StyleDeclaration {
  pub(crate) fn new(initial_value: Option<String>) -> Self {
    let data = initial_value.map_or_else(Vec::new, string_to_data);
    Self { data }
  }

  pub(crate) fn new_reference(
    env: Env,
    initial_value: Option<String>,
  ) -> Result<Reference<Self>> {
    let style_declaration = Self::new(initial_value);
    Self::into_reference(style_declaration, env)
  }

  fn get_data_mut(&mut self, property: &String) -> Option<&mut Data> {
    let property = TO_CAMEL_CASE.convert(property);

    self.data.iter_mut().find(|data| data.property == property)
  }

  fn get_data(&self, property: &String) -> Option<&Data> {
    let property = TO_CAMEL_CASE.convert(property);

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
  pub fn remove_property(&mut self, property: String) -> String {
    let camel = TO_CAMEL_CASE.convert(property);

    let pos = self.data.iter().position(|data| data.property == camel);

    if let Some(pos) = pos {
      self.data.remove(pos).value
    } else {
      String::from("")
    }
  }

  #[napi]
  pub fn set_property(
    &mut self,
    property: String,
    value: String,
    priority: Option<String>,
  ) {
    let important = priority.map_or(false, |priority| priority == "important");

    match self.get_data_mut(&property) {
      Some(data) => {
        data.value = value;
        data.important = important;
      }
      None => {
        self.data.push(Data {
          property: TO_CAMEL_CASE.convert(property),
          value,
          important,
        });
      }
    }
  }

  #[napi(getter)]
  pub fn get_css_text(&self) -> String {
    self
      .data
      .iter()
      .map(|data| {
        let mut property = TO_KEBAB_CASE.convert(&data.property);

        if property.starts_with("webkit") {
          property = "-webkit".to_owned() + &property[6..];
        }

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
  pub fn set_css_text(&mut self, css_text: String) {
    self.data = string_to_data(css_text);
  }

  #[napi(getter)]
  pub fn get_css_float(&self) -> String {
    self.get_property_value("css-float".into())
  }

  #[napi(setter)]
  pub fn set_css_float(&mut self, value: String) {
    self.set_property("cssFloat".to_string(), value, None);
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
              TO_CAMEL_CASE.convert(property.trim().to_string()),
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
