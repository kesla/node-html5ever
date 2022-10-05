use std;

use crate::Handle;

#[derive(Default)]
pub(crate) struct ChildNodeList(Vec<Handle>);

impl ChildNodeList {
  pub(crate) fn get(&self, index: usize) -> Option<&Handle> {
    self.0.get(index)
  }

  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn iter(&self) -> std::slice::Iter<Handle> {
    self.0.iter()
  }

  pub(crate) fn remove_handle(&mut self, handle: &Handle) {
    self.0.retain(|h| h != handle);

    let mut index = 0;
    self.0.iter().for_each(|h| {
      let mut borrow_mut = h.0.parent_context.borrow_mut();
      borrow_mut.as_mut().unwrap().index = index;
      index += 1;
    });

    self.sync_parent_context();
  }

  pub(crate) fn sync_parent_context(&mut self) {
    for index in 0..self.0.len() {
      let mut borrow_mut = self.0[index].0.parent_context.borrow_mut();
      borrow_mut.as_mut().unwrap().index = index;
    }
  }

  pub(crate) fn append_handle(&mut self, child: Handle) {
    self.0.push(child);
  }
}

impl Into<Vec<Handle>> for ChildNodeList {
  fn into(self) -> Vec<Handle> {
    self.0
  }
}
