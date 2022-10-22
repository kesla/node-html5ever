mod properties;

use convert_case::{Case, Casing};
use std::collections::HashMap;

struct Data {
  value: String,
  important: bool,
}

#[napi]
pub struct StyleDeclaration {
  map: HashMap<String, Data>,
}

#[napi]
impl StyleDeclaration {
  fn get(&self, property: &str) -> String {
    self
      .map
      .get(property)
      .map_or_else(|| String::from(""), |data| data.value.to_owned())
  }

  fn set(&mut self, property: String, value: String) {
    let data = Data {
      value,
      important: false,
    };
    self.map.insert(property, data);
  }

  #[napi]
  pub fn get_property_value(&self, property: String) -> String {
    self
      .map
      .get(&property.to_string())
      .map_or_else(|| String::from(""), |data| data.value.to_owned())
  }

  #[napi]
  pub fn get_property_priority(&self, property: String) -> String {
    self.map.get(&property.to_string()).map_or_else(
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
    self
      .map
      .remove(&property.to_string())
      .map_or_else(|| String::from(""), |data| data.value.to_owned())
  }

  #[napi]
  pub fn set_property(&mut self, property: String, value: String, priority: Option<String>) {
    let data = Data {
      value,
      important: priority.map_or(false, |priority| priority == "important"),
    };
    self.map.insert(property, data);
  }

  #[napi(getter)]
  pub fn get_css_text(&self) -> String {
    self
      .map
      .iter()
      .map(|(key, data)| {
        let key = key.from_case(Case::Camel).to_case(Case::Kebab);

        if data.important {
          format!("{}: {} !important;", key, data.value)
        } else {
          format!("{}: {};", key, data.value)
        }
      })
      .collect::<Vec<String>>()
      .join(" ")
  }

  #[napi(setter)]
  pub fn set_css_text(&mut self, css_text: String) {
    self.map.clear();

    css_text
      .split(';')
      .filter_map(|s| {
        let s = s.trim();
        if s.is_empty() {
          None
        } else {
          Some(s)
        }
      })
      .for_each(|item| {
        let mut parts = item.split(':');
        let mut key = parts
          .next()
          .unwrap()
          .trim()
          .from_case(Case::Kebab)
          .to_case(Case::Camel);

        if key.starts_with("webkit") {
          key = "-webkit-".to_owned() + &key[6..];
        }

        let mut value = parts.next().unwrap().trim().to_string();

        let important = value.ends_with("!important");

        if important {
          value = value.replace("!important", "").trim().to_string();
        }

        let data = Data {
          value: value.into(),
          important,
        };
        self.map.insert(key.into(), data);
      });
  }

  #[napi(getter)]
  pub fn get_css_float(&self) -> String {
    self.get("cssFloat")
  }

  #[napi(setter)]
  pub fn set_css_float(&mut self, value: String) {
    self.set("cssFloat".to_string(), value);
  }

  #[napi(getter)]
  pub fn get_length(&self) -> u32 {
    self.map.len() as u32
  }
}
