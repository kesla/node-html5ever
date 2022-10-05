use super::Handle;

use super::HandleInner;

use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct WeakHandle(Weak<HandleInner>);

impl WeakHandle {
  pub(crate) fn upgrade(&self) -> Option<Handle> {
    self.0.upgrade().map(|h| Handle(h))
  }
}

impl From<&Handle> for WeakHandle {
  fn from(handle: &Handle) -> Self {
    WeakHandle(Rc::downgrade(&handle.0))
  }
}
