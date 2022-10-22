mod properties;

use std::collections::HashMap;

#[napi]
pub struct StyleDeclaration {
  map: HashMap<String, String>,
}

impl StyleDeclaration {
  fn get(&self, key: &str) -> String {
    self
      .map
      .get(key)
      .map_or_else(|| String::from(""), |s| s.to_owned())
  }

  fn set(&mut self, key: String, value: String) {
    self.map.insert(key, value);
  }
}
