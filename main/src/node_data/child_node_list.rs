use std::slice::Iter;

use napi::{
    Error,
    Result,
};

use crate::ChildNode;

#[derive(Default)]
pub struct ChildNodeList(Vec<ChildNode>);

impl ChildNodeList {
    pub(crate) fn get(
        &self,
        index: usize,
    ) -> Option<&ChildNode> {
        self.0.get(index)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn iter(&self) -> Iter<ChildNode> {
        self.0.iter()
    }

    pub(crate) fn remove_node(
        &mut self,
        node: &ChildNode,
    ) -> Result<()> {
        let index = self
            .0
            .iter()
            .position(|child_node| child_node == node)
            .ok_or_else(|| Error::from_reason("Node not found"))?;

        self.0.remove(index);

        Ok(())
    }

    pub(crate) fn append_node(
        &mut self,
        child: ChildNode,
    ) {
        self.0.push(child);
    }

    pub(crate) fn prepend_node(
        &mut self,
        child: ChildNode,
    ) {
        self.insert_node(child, 0);
    }

    pub(crate) fn insert_node(
        &mut self,
        child: ChildNode,
        position: usize,
    ) {
        self.0.insert(position, child);
    }
}
