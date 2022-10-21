use napi::{
  bindgen_prelude::{Object, Reference, WeakReference},
  Env, Error, JsString, NapiValue, Result,
};

use crate::Element;

#[napi(skip)]
pub struct ClassList {
  list: Vec<String>,
  owner: WeakReference<Element>,
  env: Env,
  weak: Option<WeakReference<Self>>,
}

#[napi]
impl ClassList {
  pub(crate) fn new(
    owner: WeakReference<Element>,
    env: Env,
    initial_value: Option<String>,
  ) -> Result<Reference<Self>> {
    let s = Self {
      owner,
      env,
      list: initial_value.map(|s| as_list(&s)).unwrap_or_default(),
      weak: None,
    };
    let r = Self::into_reference(s, env)?;
    r.clone(env)?.weak = Some(r.downgrade());

    r.clone(env)?.set_properties()?;

    r.clone(env)
  }

  fn set_properties(&self) -> Result<()> {
    let val = self.weak.as_ref().unwrap().upgrade(self.env)?.unwrap();

    let mut this = unsafe {
      let ptr = <Reference<ClassList> as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
        self.env.raw(),
        val,
      )?;
      Object::from_raw(self.env.raw(), ptr)?
    };

    let mut index: u32 = 0;
    loop {
      if let Some(s) = self.list.get(index as usize) {
        let value: JsString = self.env.create_string(s)?;
        this.set_element(index, value)?;
      } else if this.has_element(index)? {
        this.delete_element(index)?;
      } else {
        break;
      }
      index += 1;
    }

    Ok(())
  }

  fn sync(&self, value: &str) -> Result<()> {
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
  pub fn item(&self, index: i64) -> Option<String> {
    usize::try_from(index)
      .ok()
      .and_then(|index| self.list.get(index).cloned())
  }

  #[napi]
  pub fn add(&mut self, token: String) -> Result<()> {
    validate_token(&token)?;

    if !self.list.contains(&token) {
      self.list.push(token);
      self.sync(&self.get_value())
    } else {
      Ok(())
    }
  }

  #[napi]
  pub fn remove(&mut self, token: String) -> Result<()> {
    validate_token(&token)?;

    if self.list.contains(&token) {
      self.list.retain(|t| t != &token);
      self.sync(&self.get_value())
    } else {
      Ok(())
    }
  }

  #[napi]
  pub fn toggle(&mut self, token: String) -> Result<bool> {
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
  pub fn contains(&self, token: String) -> bool {
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
  pub fn set_value(&mut self, value: String) -> Result<()> {
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
