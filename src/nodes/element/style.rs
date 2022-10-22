use std::collections::HashMap;

#[napi]
struct Style {
  map: HashMap<String, String>,
}

macro_rules! getter_and_setter {
  ($x:tt) => {
    #[napi(getter)]
    // pub fn get_$x(&self) -> Option<String> {
    //   self.map.get(stringify!($x)).map(|s| s.to_string())
    // }
    #[napi(setter)]
    pub fn set_$x(&mut self, value: String) {
      self.map.insert(stringify!($x).to_string(), value);
    }
  };
}

impl Style {
  getter_and_setter!(color);
}
