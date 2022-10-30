mod attributes;
mod class_list;
mod element_ref;

use attributes::{
    Attr,
    AttributesWrapper,
};
use class_list::ClassList;
pub use element_ref::ElementRef;
use html5ever::{
    LocalName,
    QualName,
};
use napi::{
    bindgen_prelude::Reference,
    Result,
};

use crate::{
    serialize,
    Html5everDom,
    LazyReference,
    Node,
    StyleDeclaration,
    Text,
};

#[create_node(has_children, is_child)]
pub struct Element {
    pub(crate) attributes_wrapper: AttributesWrapper,

    pub(crate) name: QualName,

    pub(crate) lazy_class_list: LazyReference<ClassList>,
    pub(crate) lazy_style: LazyReference<StyleDeclaration>,
}

#[napi]
impl Element {
    #[napi(getter)]
    pub fn get_attributes(
        &self,
        r: Reference<Element>,
    ) -> Vec<Attr> {
        self.attributes_wrapper.get_attributes(r)
    }

    #[napi]
    pub fn get_attribute(
        &self,
        name: String,
    ) -> Option<String> {
        self.attributes_wrapper
            .get_attribute(LocalName::from(name))
            .map(|attribute| attribute.value.to_string())
    }

    #[napi]
    pub fn remove_attribute(
        &mut self,
        name: String,
    ) -> Result<()> {
        if name == *"class" {
            if let Some(class_list) = &mut self.lazy_class_list.get_mut() {
                class_list.clear()?;
            }
        }

        if name == *"style" {
            if let Some(style) = &mut self.lazy_style.get_mut() {
                style.clear()?;
            }
        }

        self.attributes_wrapper.remove_attribute(name.into());

        Ok(())
    }

    #[napi]
    pub fn set_attribute(
        &mut self,
        name: String,
        value: String,
    ) -> Result<()> {
        if name == *"class" {
            if let Some(class_list) = &mut self.lazy_class_list.get_mut() {
                // attribute is set in ClassList::set_value
                class_list.set_value(value)?;

                return Ok(());
            }
        }

        if name == *"style" {
            if let Some(style) = &mut self.lazy_style.get_mut() {
                // attribute is set in StyleDeclaration::set_css_text
                style.set_css_text(value)?;

                return Ok(());
            }
        }

        self.attributes_wrapper
            .set_attribute(LocalName::from(name), value.into());

        Ok(())
    }

    #[napi]
    pub fn has_attribute(
        &self,
        name: String,
    ) -> bool {
        self.attributes_wrapper.has_attribute(name.into())
    }

    #[napi(getter)]
    pub fn get_class_list(
        &mut self,
        element: Reference<Element>,
    ) -> Result<Reference<ClassList>> {
        let initial_value = self.get_attribute("class".to_string());
        self.lazy_class_list.get_or_init(|| {
            ClassList::new(element.downgrade(), self.env, initial_value)
        })
    }

    #[napi(getter)]
    pub fn get_style(
        &mut self,
        element: Reference<Element>,
    ) -> Result<Reference<StyleDeclaration>> {
        let initial_value = self.get_attribute("style".to_string());
        self.lazy_style.get_or_init(|| {
            StyleDeclaration::new(element.downgrade(), self.env, initial_value)
        })
    }

    #[napi(getter)]
    pub fn get_tag_name(&self) -> String {
        self.get_node_name()
    }

    #[napi(getter, js_name = "innerHTML")]
    pub fn get_inner_html(&self) -> String {
        serialize(
            self.into(),
            html5ever::serialize::TraversalScope::ChildrenOnly(None),
        )
    }

    #[napi(setter, js_name = "innerHTML")]
    pub fn set_innner_html(
        &self,
        html: String,
    ) -> Result<()> {
        while let Some(child) = self.get_first_child() {
            self.remove_child(child)?;
        }

        Html5everDom::parse_and_append(self.env, self.into(), html)
    }

    #[napi(getter, js_name = "outerHTML")]
    pub fn get_outer_html(&self) -> String {
        serialize(
            self.into(),
            html5ever::serialize::TraversalScope::IncludeNode,
        )
    }

    // #[napi(setter, js_name = "outerHTML")]
    // pub fn set_outer_html(
    //     &self,
    //     html: String,
    // ) -> Result<()> {
    //     let maybe_parent = self.get_parent_node()?;
    //     let parent = match maybe_parent {
    //         Some(parent) => parent,
    //         None => return Ok(()),
    //     };

    //     if matches!(parent, crate::ParentNode::Document(_)) {
    //         return Err(napi::Error::new(
    //             napi::Status::InvalidArg,
    //             "Cannot set outerHTML on document".to_string(),
    //         ));
    //     }

    //     let cloned = self.clone_node(Some(false))?;
    //     let node: Node = cloned.clone(self.env)?.into();

    //     Html5everDom::parse_and_append(self.env, node, html)?;

    //     cloned.get_child_nodes().into_iter().for_each(|child| {
    //         parent
    //             .insert_before(child, Some(self.clone(self.env).unwrap()))
    //             .unwrap();
    //     });

    //     Ok(())
    // }

    #[napi(getter)]
    pub fn get_text_content(&self) -> Option<String> {
        let node: Node = self.into();

        let text = node
            .deep_child_nodes_iter::<Reference<Text>>()
            .map(|text| text.data.clone())
            .collect();

        Some(text)
    }

    #[napi(getter)]
    pub fn get_class_name(&self) -> String {
        self.attributes_wrapper
            .get_attribute(LocalName::from("class"))
            .map(|attribute| attribute.value.to_string())
            .unwrap_or_default()
    }

    #[napi(setter)]
    pub fn set_class_name(
        &mut self,
        class_name: String,
    ) {
        self.attributes_wrapper
            .set_attribute(LocalName::from("class"), class_name.into());
    }

    #[napi(getter)]
    pub fn get_id(&self) -> String {
        self.attributes_wrapper
            .get_attribute(LocalName::from("id"))
            .map(|attribute| attribute.value.to_string())
            .unwrap_or_default()
    }

    #[napi(setter)]
    pub fn set_id(
        &mut self,
        id: String,
    ) {
        self.attributes_wrapper
            .set_attribute(LocalName::from("id"), id.into());
    }

    #[napi]
    pub fn clone_node(
        &self,
        deep: Option<bool>,
    ) -> Result<Reference<Element>> {
        let deep = deep.unwrap_or(false);

        let clone = Self::new_reference(
            self.env,
            self.attributes_wrapper.clone(),
            self.name.clone(),
            LazyReference::new(self.env),
            LazyReference::new(self.env),
        )?;

        if deep {
            for child in self.get_child_nodes() {
                let child_clone = child.clone_node(Some(true))?;
                clone.append_child(child_clone)?;
            }
        }

        Ok(clone)
    }
}
