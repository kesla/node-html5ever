use std::{fmt::Debug, ops::Deref};

use html5ever::{namespace_url, ns};
use napi::{bindgen_prelude::Reference, Env};

use crate::{ChildNode, Element, ParentNode};

pub struct ElementRef {
  r: Reference<Element>,
  env: Env,
}

impl ElementRef {
  pub fn new(env: Env, r: Reference<Element>) -> Self {
    Self { r, env }
  }
}

impl Clone for ElementRef {
  fn clone(&self) -> Self {
    Self {
      r: self.r.clone(self.env).unwrap(),
      env: self.env,
    }
  }
}

impl Debug for ElementRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ElementRef").finish()
  }
}

impl Deref for ElementRef {
  type Target = Element;

  fn deref(&self) -> &Self::Target {
    self.r.deref()
  }
}

impl selectors::Element for ElementRef {
  type Impl = crate::Selectors;

  fn opaque(&self) -> selectors::OpaqueElement {
    selectors::OpaqueElement::new(self)
  }

  fn parent_element(&self) -> Option<Self> {
    self.get_parent_element().unwrap().map(|r| ElementRef {
      r: r.upgrade(self.env).unwrap().unwrap(),
      env: self.env,
    })
  }

  fn parent_node_is_shadow_root(&self) -> bool {
    false
  }

  fn containing_shadow_host(&self) -> Option<Self> {
    None
  }

  fn is_pseudo_element(&self) -> bool {
    false
  }

  fn prev_sibling_element(&self) -> Option<Self> {
    self
      .get_previous_element_sibling()
      .unwrap()
      .map(|r| ElementRef { r, env: self.env })
  }

  fn next_sibling_element(&self) -> Option<Self> {
    self
      .get_next_element_sibling()
      .unwrap()
      .map(|r| ElementRef { r, env: self.env })
  }

  fn is_html_element_in_html_document(&self) -> bool {
    self.name.ns == ns!(html)
  }

  fn has_local_name(
    &self,
    local_name: &<Self::Impl as selectors::SelectorImpl>::BorrowedLocalName,
  ) -> bool {
    self.name.local == local_name.to_string()
  }

  fn has_namespace(
    &self,
    ns: &<Self::Impl as selectors::SelectorImpl>::BorrowedNamespaceUrl,
  ) -> bool {
    self.name.ns == ns.to_string()
  }

  fn is_same_type(&self, other: &Self) -> bool {
    self.name == other.name
  }

  fn attr_matches(
    &self,
    ns: &selectors::attr::NamespaceConstraint<
      &<Self::Impl as selectors::SelectorImpl>::NamespaceUrl,
    >,
    local_name: &<Self::Impl as selectors::SelectorImpl>::LocalName,
    operation: &selectors::attr::AttrSelectorOperation<
      &<Self::Impl as selectors::SelectorImpl>::AttrValue,
    >,
  ) -> bool {
    match ns {
      selectors::attr::NamespaceConstraint::Any => self
        .attributes_wrapper
        .iter()
        .any(|attr| self.name.local == local_name.to_string() && operation.eval_str(&attr.value)),
      selectors::attr::NamespaceConstraint::Specific(namespace_url) => {
        self.attributes_wrapper.iter().any(|attr| {
          self.name.ns == namespace_url.to_string()
            && self.name.local == local_name.to_string()
            && operation.eval_str(&attr.value)
        })
      }
    }
  }

  fn match_non_ts_pseudo_class<F>(
    &self,
    _pc: &<Self::Impl as selectors::SelectorImpl>::NonTSPseudoClass,
    _context: &mut selectors::matching::MatchingContext<Self::Impl>,
    _flags_setter: &mut F,
  ) -> bool
  where
    F: FnMut(&Self, selectors::matching::ElementSelectorFlags),
  {
    todo!()
  }

  fn match_pseudo_element(
    &self,
    _pe: &<Self::Impl as selectors::SelectorImpl>::PseudoElement,
    _context: &mut selectors::matching::MatchingContext<Self::Impl>,
  ) -> bool {
    todo!()
  }

  fn is_link(&self) -> bool {
    todo!()
  }

  fn is_html_slot_element(&self) -> bool {
    todo!()
  }

  fn has_id(
    &self,
    id: &<Self::Impl as selectors::SelectorImpl>::Identifier,
    case_sensitivity: selectors::attr::CaseSensitivity,
  ) -> bool {
    let id_attr = self.get_id();

    case_sensitivity.eq(id_attr.as_bytes(), id.as_bytes())
  }

  fn has_class(
    &self,
    name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
    case_sensitivity: selectors::attr::CaseSensitivity,
  ) -> bool {
    self
      .get_class_name()
      .split_ascii_whitespace()
      .any(|class_name_attr| case_sensitivity.eq(class_name_attr.as_bytes(), name.as_bytes()))
  }

  fn imported_part(
    &self,
    _name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
  ) -> Option<<Self::Impl as selectors::SelectorImpl>::Identifier> {
    // TODO: Implement this (shadow DOM related)
    None
  }

  fn is_part(&self, _name: &<Self::Impl as selectors::SelectorImpl>::Identifier) -> bool {
    // TODO: Implement this (shadow DOM related)
    false
  }

  fn is_empty(&self) -> bool {
    self
      .get_node_handler()
      .shallow_child_nodes_iter()
      .all(|ref child| match child {
        ChildNode::Element(ref _element) => false,
        ChildNode::Text(ref text) => text.data.is_empty(),
        _ => true,
      })
  }

  fn is_root(&self) -> bool {
    self
      .get_parent_node()
      .unwrap()
      .map_or(false, |parent| matches!(parent, ParentNode::Document(_)))
  }
}

impl Into<Reference<Element>> for ElementRef {
  fn into(self) -> Reference<Element> {
    self.r
  }
}
