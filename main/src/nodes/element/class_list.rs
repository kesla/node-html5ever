use indexmap::IndexSet;
use itertools::join;
use napi::{
    bindgen_prelude::Reference,
    Env,
    Error,
    Result,
};

use crate::{
    CyclicReference,
    Element,
    WeakReference,
    WithDataInBrackets,
};

#[napi]
pub struct ClassList {
    data: IndexSet<String>,
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
        self.data.get_index(index).cloned()
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
                    data: initial_value.map(|s| as_set(&s)).unwrap_or_default(),
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

        let mut owner = self.owner.upgrade(self.env)?;
        owner
            .attributes_wrapper
            .set_attribute("class".into(), value.into());

        Ok(())
    }

    pub(crate) fn clear(&mut self) -> Result<()> {
        self.data.clear();
        self.set_properties()
    }

    fn as_string(&self) -> String {
        join(self.data.iter(), " ")
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
        token1: Option<String>,
        token2: Option<String>,
        token3: Option<String>,
        token4: Option<String>,
        token5: Option<String>,
    ) -> Result<()> {
        let tokens = vec![token1, token2, token3, token4, token5]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if tokens.is_empty() {
            return Ok(());
        }

        for token in tokens {
            validate_token(&token)?;
            self.data.insert(token);
        }

        self.sync(&self.as_string())
    }

    #[napi]
    pub fn remove(
        &mut self,
        token1: Option<String>,
        token2: Option<String>,
        token3: Option<String>,
        token4: Option<String>,
        token5: Option<String>,
    ) -> Result<()> {
        let tokens = vec![token1, token2, token3, token4, token5]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if tokens.is_empty() {
            return Ok(());
        }

        for token in tokens {
            validate_token(&token)?;
            self.data.shift_remove(&token);
        }

        self.sync(&self.as_string())
    }

    #[napi]
    pub fn toggle(
        &mut self,
        token: String,
        force: Option<bool>,
    ) -> Result<bool> {
        let add = force.unwrap_or_else(|| !self.data.contains(&token));

        if add {
            self.add(Some(token), None, None, None, None)?;
        } else {
            self.remove(Some(token), None, None, None, None)?;
        }

        Ok(add)
    }

    #[napi]
    pub fn contains(
        &self,
        token: String,
    ) -> bool {
        self.data.contains(&token)
    }

    #[napi(getter)]
    pub fn get_length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    #[napi(getter)]
    pub fn get_value(&self) -> Result<String> {
        self.owner.upgrade(self.env).map(|owner| {
            owner.get_attribute("class".into()).unwrap_or_default()
        })
    }

    #[napi]
    pub fn to_string(&self) -> Result<String> {
        self.get_value()
    }

    #[napi(setter)]
    pub fn set_value(
        &mut self,
        value: String,
    ) -> Result<()> {
        if self.get_value()? == value {
            return Ok(());
        }

        self.data = as_set(&value);
        self.sync(&value)
    }
}

fn as_set(value: &str) -> IndexSet<String> {
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
