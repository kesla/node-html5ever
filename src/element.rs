use html5ever::{Attribute, QualName};
use napi::{
  bindgen_prelude::{Reference, WeakReference},
  Either, Env, Result,
};

use crate::{document::Document, handle::Handle, node_list::NodeList, serialize::serialize, parent::clone_parent_node};

#[napi]
pub struct Element {
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) list: Reference<NodeList>,
  pub(crate) name: QualName,
  pub(crate) env: Env,
  pub(crate) parent: Option<Either<WeakReference<Element>, WeakReference<Document>>>,
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

  #[napi(getter)]
  pub fn get_parent_node(&self) -> Option<Either<WeakReference<Element>, WeakReference<Document>>> {
    clone_parent_node(self.parent.as_ref())
  }
}
