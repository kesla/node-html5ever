use html5ever::{Attribute, QualName, Namespace};
use napi::bindgen_prelude::Reference;

use crate::{handle::Handle, node_list::NodeList, serialize::serialize};

#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Element {
  pub(crate) attrs: Vec<Attribute>,

  #[default(NodeList::new(env)?)]
  pub(crate) list: Reference<NodeList>,

  pub(crate) name: QualName,
}

#[napi]
impl Element {
  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    let b = &self.attrs;
    let mut iter = b.iter();
    while let Some(attr) = iter.next() {
      if attr.name.local == name {
        return Some(attr.value.to_string());
      }
    }

    None
  }

  #[napi]
  pub fn remove_attribute(&mut self, name: String) {
    self.attrs.retain(|attribute| {
      attribute.name.local != name
    })
  }

  #[napi]
  pub fn set_attribute(&mut self, name: String, value: String) {
    let maybe_existing_attribute = self.attrs.iter_mut().find(|attr| attr.name.local == name);

    match maybe_existing_attribute {
      Some(existing_attribute) => {
        existing_attribute.value = value.into();
      }
      None => {
        let new_attribute = Attribute {
          name: QualName::new(None, Namespace::from(""), name.into()),
          value: value.into(),
        };
        self.attrs.push(new_attribute);
      }
    }
  }

  #[napi]
  pub fn has_attribute(&self, name: String) -> bool {
    self.get_attribute(name).is_some()
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    self.name.local.to_string().to_uppercase()
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> String {
    self.node_name()
  }

  // TODO: cache this & perhaps return something nicer
  // remove .unwrap
  #[napi(getter)]
  pub fn get_children(&self) -> Vec<Reference<Element>> {
    self
      .list
      .iter()
      .filter_map(|handle| {
        handle
          .into_element()
          .ok()
          .map(|r| r.clone(self.env).unwrap())
      })
      .collect()
  }

  #[napi(getter)]
  pub fn inner_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(
      &handle,
      html5ever::serialize::TraversalScope::ChildrenOnly(None),
    )
  }

  #[napi(getter)]
  pub fn outer_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(&handle, html5ever::serialize::TraversalScope::IncludeNode)
  }
}
