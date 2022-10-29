use std::{
    self,
    slice::Iter,
};

use napi::{
    Error,
    Result,
};

use crate::ChildNode;

#[derive(Default)]
pub(crate) struct ChildNodeList(Vec<ChildNode>);

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
        child: &ChildNode,
    ) {
        self.0.push(child.to_owned());
    }

    pub(crate) fn prepend_node(
        &mut self,
        child: &ChildNode,
    ) {
        self.0.insert(0, child.to_owned());
    }

    pub(crate) fn insert_node(
        &mut self,
        child: &ChildNode,
        position: usize,
    ) {
        self.0.insert(position, child.to_owned());
    }
}

impl From<ChildNodeList> for Vec<ChildNode> {
    fn from(child_node_list: ChildNodeList) -> Self {
        child_node_list.0
    }
}
