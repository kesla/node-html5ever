use html5ever::{Attribute, QualName};
use napi::{
  bindgen_prelude::{Reference, WeakReference},
  Either, Env, Result,
};

use crate::{document::Document, handle::Handle, node_list::NodeList, serialize::serialize};
// use crate::node::Node;

#[napi]
#[derive(Node)]
#[add_node_fields]
pub struct Element {
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) list: Reference<NodeList>,
  pub(crate) name: QualName,
  pub(crate) env: Env,
}

#[napi]
impl Element {
  pub(crate) fn new(
    env: Env,
    attrs: Vec<Attribute>,
    name: QualName,
  ) -> Result<Reference<Self>> {
    let r = Self {
      env,
      attrs,
      list: NodeList::new(env)?,
      name,
      parent: None,
    };
    
    r.get_parent_node();

    Self::into_reference(r, env)
  }

  #[napi]
  pub fn get_attribute(&self, key: String) -> Option<String> {
    let b = &self.attrs;
    let mut iter = b.iter();
    while let Some(attr) = iter.next() {
      if attr.name.local == key {
        return Some(attr.value.to_string());
      }
    }

    None
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
      .filter_map(|handle| handle.into_element().ok().map(|r| r.clone(self.env).unwrap()))
      .collect()
  }

  #[napi(getter)]
  pub fn inner_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(&handle, html5ever::serialize::TraversalScope::ChildrenOnly(None))
  }

  #[napi(getter)]
  pub fn outer_html(&self, reference: Reference<Element>) -> String {
    let handle: Handle = reference.into();
    serialize(&handle, html5ever::serialize::TraversalScope::IncludeNode)
  }
}
