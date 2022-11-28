mod properties;
mod property_cache;

use napi::{
    bindgen_prelude::{
        Reference,
        WeakReference,
    },
    Env,
    Error,
    Result,
};
use property_cache::{
    get_index,
    get_kebab,
};

use crate::{
    CyclicReference,
    Element,
    WithDataInBrackets,
};

#[derive(Debug)]
struct Data {
    property_index: usize,
    value: String,
    important: bool,
}

impl Data {
    #[inline]
    fn get_kebab_property(&self) -> String {
        get_kebab(self.property_index)
    }
}

#[napi]
pub struct StyleDeclaration {
    list: Vec<Data>,
    owner: WeakReference<Element>,
    env: Env,
    cyclic_reference: CyclicReference<Self>,
}

impl WithDataInBrackets for StyleDeclaration {
    #[inline]
    fn raw_item(
        &self,
        index: usize,
    ) -> Option<String> {
        self.list.get(index).map(|data| data.get_kebab_property())
    }

    #[inline]
    fn get_reference(&self) -> Result<Reference<Self>> {
        self.cyclic_reference.get()
    }

    #[inline]
    fn get_env(&self) -> Env {
        self.env
    }
}

#[napi]
impl StyleDeclaration {
    pub(crate) fn new(
        owner: WeakReference<Element>,
        env: Env,
        initial_value: Option<String>,
    ) -> Result<Reference<Self>> {
        let r = CyclicReference::<Self>::new_cyclic(env, |cyclic_reference| {
            Self::into_reference(
                Self {
                    owner,
                    env,
                    list: initial_value.map_or_else(Vec::new, string_to_data),
                    cyclic_reference,
                },
                env,
            )
        })?;

        r.clone(env)?.set_properties()?;
        r.clone(env)
    }

    fn sync(
        &self,
        value: &str,
    ) -> Result<()> {
        self.set_properties()?;

        self.owner.upgrade(self.env)?.map_or_else(
            || Err(Error::from_reason("Element not found")),
            |mut owner| {
                owner
                    .attributes_wrapper
                    .set_attribute("style".into(), value.into());

                Ok(())
            },
        )
    }

    pub(crate) fn clear(&mut self) -> Result<()> {
        self.list.clear();
        self.set_properties()
    }

    fn get_data_mut(
        &mut self,
        property: &str,
    ) -> Option<&mut Data> {
        get_index(property).and_then(|property_index| {
            self.list
                .iter_mut()
                .find(|data| data.property_index == property_index)
        })
    }

    fn get_data(
        &self,
        property: &str,
    ) -> Option<&Data> {
        get_index(property).and_then(|property_index| {
            self.list
                .iter()
                .find(|data| data.property_index == property_index)
        })
    }

    #[napi]
    pub fn get_property_value(
        &self,
        property: String,
    ) -> String {
        self.get_data(&property)
            .map_or_else(|| String::from(""), |data| data.value.to_owned())
    }

    #[napi]
    pub fn get_property_priority(
        &self,
        property: String,
    ) -> String {
        self.get_data(&property).map_or_else(
            || String::from(""),
            |data| {
                if data.important {
                    String::from("important")
                } else {
                    String::from("")
                }
            },
        )
    }

    #[napi]
    pub fn remove_property(
        &mut self,
        property: String,
    ) -> Result<String> {
        let property_index = match get_index(&property) {
            Some(index) => index,
            None => return Ok(String::from("")),
        };

        let pos = self
            .list
            .iter()
            .position(|data| data.property_index == property_index);

        if let Some(pos) = pos {
            let result = self.list.remove(pos).value;
            self.sync(&self.get_css_text())?;
            Ok(result)
        } else {
            Ok(String::from(""))
        }
    }

    #[napi]
    pub fn set_property(
        &mut self,
        property: String,
        value: String,
        priority: Option<String>,
    ) -> Result<()> {
        let important =
            priority.map_or(false, |priority| priority == "important");

        match self.get_data_mut(&property) {
            Some(data) => {
                data.value = value;
                data.important = important;
            },
            None => {
                let property_index = match get_index(&property) {
                    Some(index) => index,
                    None => return Ok(()),
                };

                self.list.push(Data {
                    property_index,
                    value,
                    important,
                });
            },
        };

        self.sync(&self.get_css_text())
    }

    #[napi(getter)]
    pub fn get_css_text(&self) -> String {
        self.list
            .iter()
            .map(|data| {
                let property = get_kebab(data.property_index);

                if data.important {
                    format!("{}: {} !important;", property, data.value)
                } else {
                    format!("{}: {};", property, data.value)
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[napi(setter)]
    pub fn set_css_text(
        &mut self,
        css_text: String,
    ) -> Result<()> {
        self.list = string_to_data(css_text);
        self.sync(&self.get_css_text())
    }

    #[napi(getter)]
    pub fn get_css_float(&self) -> String {
        self.get_property_value("css-float".into())
    }

    #[napi(setter)]
    pub fn set_css_float(
        &mut self,
        value: String,
    ) -> Result<()> {
        self.set_property("cssFloat".to_string(), value, None)
    }

    #[napi]
    pub fn item(
        &self,
        index: i64,
    ) -> Option<String> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.raw_item(index))
    }

    #[napi(getter)]
    pub fn get_length(&self) -> u32 {
        self.list.len() as u32
    }
}

fn string_to_data(css_text: String) -> Vec<Data> {
    css_text
        .split(';')
        .filter_map(|item| {
            let (property_index, mut value) = {
                let mut parts = item.split(':');

                let (property, value): (&str, String) =
                    match (parts.next(), parts.next(), parts.next()) {
                        (Some(property), Some(value), None) => {
                            (property.trim(), value.trim().to_string())
                        },
                        _ => return None,
                    };

                if property.is_empty() || value.is_empty() {
                    return None;
                }

                let property_index = match get_index(property) {
                    Some(index) => index,
                    None => return None,
                };

                (property_index, value)
            };

            let important = value.ends_with("!important");

            if important {
                value = value.replace("!important", "").trim().to_string();
            }

            let data = Data {
                property_index,
                value,
                important,
            };

            Some(data)
        })
        .collect()
}
