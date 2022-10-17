use std::cell::UnsafeCell;

pub struct EinarCell<T> {
  value: UnsafeCell<T>,
}

impl<T> EinarCell<T> {
  pub fn new(value: T) -> Self {
    Self {
      value: UnsafeCell::new(value),
    }
  }

  pub fn replace(&self, value: T) -> T {
    unsafe { std::mem::replace(&mut *self.value.get(), value) }
  }

  pub fn set(&self, value: T) {
    unsafe {
      *self.value.get() = value;
    }
  }

  pub fn borrow<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&T) -> R,
  {
    let value = unsafe { &*self.value.get() };
    f(value)
  }

  pub fn borrow_mut<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&mut T) -> R,
  {
    let value = unsafe { &mut *self.value.get() };
    f(value)
  }
}

impl<T: Clone> EinarCell<T> {
  pub fn cloned(&self) -> T {
    self.borrow(|value| value.clone())
  }
}

impl<T: Default> Default for EinarCell<T> {
  fn default() -> Self {
    Self::new(Default::default())
  }
}

impl<T> From<T> for EinarCell<T> {
  fn from(value: T) -> Self {
    Self::new(value)
  }
}
