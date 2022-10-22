mod properties;

use convert_case::{Case, Casing};

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
  fn get_data(&self, property: String) -> Option<&Data> {
    self.data.iter().find(|data| data.property == property)
  }

  #[napi]
  pub fn get_property_value(&self, property: String) -> String {
    self
      .get_data(property)
      .map_or_else(|| String::from(""), |data| data.value.to_owned())
  }

  #[napi]
  pub fn get_property_priority(&self, property: String) -> String {
    self.get_data(property).map_or_else(
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
    let pos = self.data.iter().position(|data| data.property == property);

    if let Some(pos) = pos {
      self.data.remove(pos).value
    } else {
      String::from("")
    }
  }

  #[napi]
  pub fn set_property(&mut self, property: String, value: String, priority: Option<String>) {
    let important = priority.map_or(false, |priority| priority == "important");

    match self.data.iter_mut().find(|data| data.property == property) {
      Some(data) => {
        data.value = value;
        data.important = important;
      }
      None => {
        self.data.push(Data {
          property,
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
        let property = data.property.from_case(Case::Camel).to_case(Case::Kebab);

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
    self.data = css_text
      .split(';')
      .filter_map(|s| {
        let s = s.trim();
        if s.is_empty() {
          None
        } else {
          Some(s)
        }
      })
      .map(|item| {
        let mut parts = item.split(':');
        let mut property = parts
          .next()
          .unwrap()
          .trim()
          .from_case(Case::Kebab)
          .to_case(Case::Camel);

        if property.starts_with("webkit") {
          property = "-webkit-".to_owned() + &property[6..];
        }

        let mut value = parts.next().unwrap().trim().to_string();

        let important = value.ends_with("!important");

        if important {
          value = value.replace("!important", "").trim().to_string();
        }

        let data = Data {
          property: property.into(),
          value: value.into(),
          important,
        };
        data
      })
      .collect();
  }

  #[napi(getter)]
  pub fn get_css_float(&self) -> String {
    self.get_property_value("cssFloat".into())
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
