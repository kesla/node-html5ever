use std::{
    fmt::Debug,
    ops::Deref,
};

use html5ever::{
    local_name,
    namespace_url,
    ns,
};
use napi::{
    bindgen_prelude::Reference,
    Error,
    Result,
    Status,
};
use selectors::{
    attr::{
        AttrSelectorOperation,
        CaseSensitivity,
        NamespaceConstraint,
    },
    matching::{
        ElementSelectorFlags,
        MatchingContext,
    },
    OpaqueElement,
    SelectorImpl,
};

use crate::{
    ChildNode,
    Element,
    ParentNode,
};

pub struct ElementRef {
    inner: Reference<Element>,
}

impl Clone for ElementRef {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(self.inner.env).unwrap(),
        }
    }
}

impl Debug for ElementRef {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("ElementRef").finish()
    }
}

impl Deref for ElementRef {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl Into<Reference<Element>> for ElementRef {
    fn into(self) -> Reference<Element> {
        self.inner
    }
}

impl From<Reference<Element>> for ElementRef {
    fn from(r: Reference<Element>) -> Self {
        ElementRef {
            inner: r,
        }
    }
}

impl TryFrom<ChildNode> for ElementRef {
    type Error = Error;

    fn try_from(child_node: ChildNode) -> Result<Self> {
        match child_node {
            ChildNode::Element(element) => Ok(element.into()),
            _ => Err(Error::new(
                Status::InvalidArg,
                "Could not convert ChildNode to ElementRef".to_string(),
            )),
        }
    }
}

impl selectors::Element for ElementRef {
    type Impl = crate::Selectors;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self)
    }

    fn parent_element(&self) -> Option<Self> {
        self.get_parent_element()
            .unwrap()
            .map(|r| r.upgrade(self.inner.env).unwrap().unwrap().into())
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
        self.get_previous_element_sibling()
            .unwrap()
            .map(|r| r.into())
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.get_next_element_sibling().unwrap().map(|r| r.into())
    }

    fn is_html_element_in_html_document(&self) -> bool {
        self.name.ns == ns!(html)
    }

    fn has_local_name(
        &self,
        local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName,
    ) -> bool {
        self.name.local == local_name.to_string()
    }

    fn has_namespace(
        &self,
        ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl,
    ) -> bool {
        self.name.ns == ns.to_string()
    }

    fn is_same_type(
        &self,
        other: &Self,
    ) -> bool {
        self.name == other.name
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<
            &<Self::Impl as SelectorImpl>::AttrValue,
        >,
    ) -> bool {
        match ns {
            NamespaceConstraint::Any => {
                self.attributes_wrapper.iter().any(|attr| {
                    attr.name.local == local_name.to_string()
                        && operation.eval_str(&attr.value)
                })
            },

            NamespaceConstraint::Specific(namespace_url) => {
                self.attributes_wrapper.iter().any(|attr| {
                    attr.name.ns == namespace_url.to_string()
                        && attr.name.local == local_name.to_string()
                        && operation.eval_str(&attr.value)
                })
            },
        }
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        _pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut MatchingContext<Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        todo!()
    }

    fn match_pseudo_element(
        &self,
        _pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        _context: &mut MatchingContext<Self::Impl>,
    ) -> bool {
        todo!()
    }

    fn is_link(&self) -> bool {
        matches!(
            self.name.local,
            local_name!("a") | local_name!("area") | local_name!("link")
        ) && self.has_attribute("href".to_string())
    }

    fn is_html_slot_element(&self) -> bool {
        todo!()
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let id_attr = self.get_id();

        case_sensitivity.eq(id_attr.as_bytes(), id.as_bytes())
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let class_name = match self.get_attribute("class".to_string()) {
            Some(class) => class,
            None => return false,
        };

        class_name
            .split_ascii_whitespace()
            .any(|class| case_sensitivity.eq(class.as_bytes(), name.as_bytes()))
    }

    fn imported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier> {
        // TODO: Implement this (shadow DOM related)
        None
    }

    fn is_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> bool {
        // TODO: Implement this (shadow DOM related)
        false
    }

    fn is_empty(&self) -> bool {
        self.get_node_handler()
            .shallow_child_nodes_iter()
            .all(|ref child| match child {
                ChildNode::Element(ref _element) => false,
                ChildNode::Text(ref text) => text.data.is_empty(),
                _ => true,
            })
    }

    fn is_root(&self) -> bool {
        self.get_parent_node()
            .unwrap()
            .map_or(false, |parent| matches!(parent, ParentNode::Document(_)))
    }
}
