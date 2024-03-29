use html5ever::{
    tendril::StrTendril,
    Attribute,
    LocalName,
    Namespace,
    QualName,
};
use napi::{
    bindgen_prelude::Reference,
    Env,
    Result,
};

use crate::{
    Document,
    Element,
    WeakReference,
};

#[napi]
pub struct Attr {
    attribute: Attribute,
    owner_element: WeakReference<Element>,
    env: Env,
}

#[napi]
impl Attr {
    fn get_prefix_and_name(&self) -> (Option<String>, String) {
        let as_string: &str = &self.attribute.name.local;

        let mut split = as_string.split(':');

        match (split.next(), split.next()) {
            (Some(prefix), Some(name)) => {
                (Some(prefix.to_string()), name.to_string())
            },
            (Some(name), _) => (None, name.to_string()),
            _ => panic!("Invalid attribute name"),
        }
    }

    #[napi(getter)]
    pub fn get_local_name(&self) -> String {
        self.attribute.name.local.to_string()
    }

    #[napi(getter)]
    pub fn get_name(&self) -> String {
        self.get_prefix_and_name().1
    }

    #[napi(getter)]
    pub fn get_namespace_uri(&self) -> String {
        self.attribute.name.ns.to_string()
    }

    #[napi(getter)]
    pub fn get_owner_document(
        &self
    ) -> Result<Option<WeakReference<Document>>> {
        let element = self.get_owner_element().upgrade(self.env)?;

        element.get_owner_document()
    }

    #[napi(getter)]
    pub fn get_owner_element(&self) -> WeakReference<Element> {
        self.owner_element.clone()
    }

    #[napi(getter)]
    pub fn get_prefix(&self) -> Option<String> {
        self.get_prefix_and_name().0
    }

    #[napi(getter)]
    pub fn get_value(&self) -> String {
        self.attribute.value.to_string()
    }
}

#[derive(Clone)]
pub(crate) struct AttributesWrapper {
    attrs: Vec<Attribute>,
}

impl From<Vec<Attribute>> for AttributesWrapper {
    fn from(attrs: Vec<Attribute>) -> Self {
        Self {
            attrs,
        }
    }
}

impl AttributesWrapper {
    pub(crate) fn get_attributes(
        &self,
        r: Reference<Element>,
    ) -> Vec<Attr> {
        self.attrs
            .iter()
            .map(|attribute| Attr {
                attribute: attribute.clone(),
                owner_element: r.downgrade().into(),
                env: r.env,
            })
            .collect()
    }

    pub(crate) fn get_attribute(
        &self,
        name: LocalName,
    ) -> Option<&Attribute> {
        self.iter().find(|attribute| attribute.name.local == name)
    }

    pub(crate) fn has_attribute(
        &self,
        name: LocalName,
    ) -> bool {
        self.get_attribute(name).is_some()
    }

    pub(crate) fn remove_attribute(
        &mut self,
        name: LocalName,
    ) {
        self.attrs.retain(|attribute| attribute.name.local != name)
    }

    pub(crate) fn add_attribute(
        &mut self,
        name: LocalName,
        value: StrTendril,
    ) {
        let attribute_name = QualName::new(None, Namespace::from(""), name);
        let new_attribute = Attribute {
            name: attribute_name,
            value,
        };
        self.push(new_attribute);
    }

    pub(crate) fn set_attribute(
        &mut self,
        name: LocalName,
        value: StrTendril,
    ) {
        self.remove_attribute(name.clone());
        self.add_attribute(name, value);
    }

    pub(crate) fn push(
        &mut self,
        attribute: Attribute,
    ) {
        self.attrs.push(attribute)
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<Attribute> {
        self.attrs.iter()
    }
}
