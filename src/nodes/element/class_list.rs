use napi::{bindgen_prelude::WeakReference, Env, Error, Result};

use crate::Element;

#[napi]
pub struct ClassList {
  list: Vec<String>,
  owner: WeakReference<Element>,
  env: Env,
}

#[napi]
impl ClassList {
  pub(crate) fn new(
    owner: WeakReference<Element>,
    env: Env,
    initial_value: Option<String>,
  ) -> Self {
    Self {
      owner,
      env,
      list: initial_value.map(|s| as_list(&s)).unwrap_or_default(),
    }
  }

  fn update_owner_attribute(&self, value: &str) -> Result<()> {
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

  pub(crate) fn iter(&self) -> impl Iterator<Item = &String> {
    self.list.iter()
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
      self.update_owner_attribute(&self.get_value())
    } else {
      Ok(())
    }
  }

  #[napi]
  pub fn remove(&mut self, token: String) -> Result<()> {
    validate_token(&token)?;

    if self.list.contains(&token) {
      self.list.retain(|t| t != &token);
      self.update_owner_attribute(&self.get_value())
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

    self.update_owner_attribute(&self.get_value())?;
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
    self.update_owner_attribute(&value)
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
