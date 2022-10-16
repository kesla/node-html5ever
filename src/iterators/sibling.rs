use crate::ChildNode;

use std::marker::PhantomData;

use crate::NodeHandler;

pub enum SiblingIteratorType {
  Next,
  Previous,
}

pub struct SiblingIterator<T> {
  pub(crate) data: Option<(NodeHandler, usize)>,
  pub(crate) next_index: &'static dyn Fn(usize) -> Option<usize>,
  pub(crate) _phantom: PhantomData<T>,
}

impl<T> SiblingIterator<T> {
  pub fn new(data: Option<(NodeHandler, usize)>, sibling_type: SiblingIteratorType) -> Self {
    SiblingIterator {
      data,
      next_index: match sibling_type {
        SiblingIteratorType::Next => &|index: usize| index.checked_add(1),
        SiblingIteratorType::Previous => &|index: usize| index.checked_sub(1),
      },
      _phantom: PhantomData,
    }
  }

  pub(crate) fn next_child_node(&mut self) -> Option<ChildNode> {
    let (node_handler, index) = match self.data {
      Some((ref node_handler, ref mut index)) => (node_handler, index),
      None => return None,
    };

    let next_index_fn = self.next_index;
    let next_index = match next_index_fn(*index) {
      Some(i) => i,
      None => return None,
    };

    match node_handler.get_child_node(next_index) {
      Some(child_node) => {
        *index = next_index;
        Some(child_node)
      }
      None => None,
    }
  }
}

impl<T> Iterator for SiblingIterator<T>
where
  ChildNode: TryInto<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(child) = self.next_child_node() {
      if let Ok(child) = child.try_into() {
        return Some(child);
      }
    }

    None
  }
}
