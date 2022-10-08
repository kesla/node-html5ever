use html5ever::{tendril::StrTendril, Attribute, LocalName, Namespace, QualName};

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
