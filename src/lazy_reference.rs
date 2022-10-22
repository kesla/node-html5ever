use napi::{bindgen_prelude::Reference, Env, Result};

pub struct LazyReference<T: 'static> {
  data: Option<Reference<T>>,
  env: Env,
}

impl<T> LazyReference<T> {
  pub fn new(env: Env) -> Self {
    Self { data: None, env }
  }

  pub fn get(&self) -> Option<&Reference<T>> {
    self.data.as_ref()
  }

  pub fn get_mut(&mut self) -> Option<&mut Reference<T>> {
    self.data.as_mut()
  }

  pub fn get_or_init(
    &mut self,
    init: impl FnOnce() -> Result<Reference<T>>,
  ) -> Result<Reference<T>> {
    if let Some(data) = &self.data {
      return data.clone(self.env);
    }

    let data = init()?;
    self.data = Some(data.clone(self.env)?);
    Ok(data)
  }
}
