use napi::{
    bindgen_prelude::{
        Reference,
        WeakReference,
    },
    Env,
    Error,
    Result,
};

use crate::{
    CyclicReference,
    Element,
    WithDataInBrackets,
};

#[napi]
pub struct ClassList {
    list: Vec<String>,
    owner: WeakReference<Element>,
    env: Env,
    cyclic_reference: CyclicReference<Self>,
}

impl WithDataInBrackets for ClassList {
    #[inline]
    fn raw_item(
        &self,
        index: usize,
    ) -> Option<String> {
        self.list.get(index).cloned()
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
impl ClassList {
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
                    list: initial_value
                        .map(|s| as_list(&s))
                        .unwrap_or_default(),
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
                    .set_attribute("class".into(), value.into());

                Ok(())
            },
        )
    }

    pub(crate) fn clear(&mut self) -> Result<()> {
        self.list.clear();
        self.set_properties()
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

    #[napi]
    pub fn add(
        &mut self,
        token: String,
    ) -> Result<()> {
        validate_token(&token)?;

        if !self.list.contains(&token) {
            self.list.push(token);
            self.sync(&self.get_value())
        } else {
            Ok(())
        }
    }

    #[napi]
    pub fn remove(
        &mut self,
        token: String,
    ) -> Result<()> {
        validate_token(&token)?;

        if self.list.contains(&token) {
            self.list.retain(|t| t != &token);
            self.sync(&self.get_value())
        } else {
            Ok(())
        }
    }

    #[napi]
    pub fn toggle(
        &mut self,
        token: String,
    ) -> Result<bool> {
        validate_token(&token)?;
        let contains = self.list.contains(&token);

        if contains {
            self.list.retain(|t| t != &token);
        } else {
            self.list.push(token);
        }

        self.sync(&self.get_value())?;
        Ok(!contains)
    }

    #[napi]
    pub fn contains(
        &self,
        token: String,
    ) -> bool {
        self.list.contains(&token)
    }

    #[napi(getter)]
    pub fn get_length(&self) -> u32 {
        self.list.len().try_into().unwrap()
    }

    #[napi(getter)]
    pub fn get_value(&self) -> String {
        self.list.join(" ")
    }

    #[napi(setter)]
    pub fn set_value(
        &mut self,
        value: String,
    ) -> Result<()> {
        if self.get_value() == value {
            return Ok(());
        }

        self.list = as_list(&value);
        self.sync(&value)
    }
}

fn as_list(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .filter_map(|token| (!token.is_empty()).then(|| token.to_string()))
        .collect()
}

fn validate_token(token: &str) -> Result<()> {
    if token.is_empty() {
        Err(Error::from_reason("Token must not be empty"))
    } else if token.contains(char::is_whitespace) {
        Err(Error::from_reason("Token must not contain whitespace"))
    } else {
        Ok(())
    }
}
