use std::rc::Rc;

use lazycell::LazyCell;
use napi::{
  bindgen_prelude::{Reference, WeakReference},
  Env, Error, Result,
};

pub struct CyclicReference<T>
where
  T: 'static,
{
  env: Env,
  inner: Rc<LazyCell<WeakReference<T>>>,
}

impl<T> CyclicReference<T> {
  pub fn new_cyclic(
    env: Env,
    init: impl FnOnce(CyclicReference<T>) -> Result<Reference<T>>,
  ) -> Result<Reference<T>> {
    let lazy = Rc::new(LazyCell::new());
    let me = Self {
      env,
      inner: lazy.clone(),
    };
    let reference = init(me)?;
    let r = lazy.fill(reference.downgrade());
    assert!(r.is_ok());
    Ok(reference)
  }

  pub fn get(&self) -> Result<Reference<T>> {
    self
      .get_weak()?
      .upgrade(self.env)?
      .ok_or_else(|| Error::from_reason("self reference is not available anymore".to_string()))
  }

  pub fn get_weak(&self) -> Result<WeakReference<T>> {
    Ok(
      self
        .inner
        .borrow()
        .ok_or_else(|| Error::from_reason("cyclic_reference is not initialized"))?
        .clone(),
    )
  }
}
