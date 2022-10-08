use std::fmt::Debug;

use napi::bindgen_prelude::WeakReference;

#[derive(Clone)]
struct ElementRef(WeakReference<crate::Element>);

impl Debug for ElementRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ElementRef").finish()
  }
}

impl selectors::Element for ElementRef {
  type Impl = crate::Selectors;

  fn opaque(&self) -> selectors::OpaqueElement {
    todo!()
  }

  fn parent_element(&self) -> Option<Self> {
    todo!()
  }

  fn parent_node_is_shadow_root(&self) -> bool {
    todo!()
  }

  fn containing_shadow_host(&self) -> Option<Self> {
    todo!()
  }

  fn is_pseudo_element(&self) -> bool {
    todo!()
  }

  fn prev_sibling_element(&self) -> Option<Self> {
    todo!()
  }

  fn next_sibling_element(&self) -> Option<Self> {
    todo!()
  }

  fn is_html_element_in_html_document(&self) -> bool {
    todo!()
  }

  fn has_local_name(
    &self,
    local_name: &<Self::Impl as selectors::SelectorImpl>::BorrowedLocalName,
  ) -> bool {
    todo!()
  }

  fn has_namespace(
    &self,
    ns: &<Self::Impl as selectors::SelectorImpl>::BorrowedNamespaceUrl,
  ) -> bool {
    todo!()
  }

  fn is_same_type(&self, other: &Self) -> bool {
    todo!()
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
    todo!()
  }

  fn match_non_ts_pseudo_class<F>(
    &self,
    pc: &<Self::Impl as selectors::SelectorImpl>::NonTSPseudoClass,
    context: &mut selectors::matching::MatchingContext<Self::Impl>,
    flags_setter: &mut F,
  ) -> bool
  where
    F: FnMut(&Self, selectors::matching::ElementSelectorFlags),
  {
    todo!()
  }

  fn match_pseudo_element(
    &self,
    pe: &<Self::Impl as selectors::SelectorImpl>::PseudoElement,
    context: &mut selectors::matching::MatchingContext<Self::Impl>,
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
    todo!()
  }

  fn has_class(
    &self,
    name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
    case_sensitivity: selectors::attr::CaseSensitivity,
  ) -> bool {
    todo!()
  }

  fn imported_part(
    &self,
    name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
  ) -> Option<<Self::Impl as selectors::SelectorImpl>::Identifier> {
    todo!()
  }

  fn is_part(&self, name: &<Self::Impl as selectors::SelectorImpl>::Identifier) -> bool {
    todo!()
  }

  fn is_empty(&self) -> bool {
    todo!()
  }

  fn is_root(&self) -> bool {
    todo!()
  }
}
