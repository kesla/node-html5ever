use napi::{
    Env,
    Result,
};

use crate::{
    Node,
    ParentNode,
};

#[derive(Clone)]
pub struct ParentContext {
    pub(crate) node: ParentNode,
    pub(crate) index: usize,
    pub(crate) env: Env,
}

impl ParentContext {
    pub(crate) fn new(
        env: Env,
        node: ParentNode,
        index: usize,
    ) -> Self {
        ParentContext {
            env,
            node,
            index,
        }
    }

    pub(crate) fn get_node(&self) -> Result<Node> {
        let node: Node = match &self.node {
            ParentNode::Document(weak_reference) => {
                weak_reference.upgrade(self.env)?.unwrap().into()
            },
            ParentNode::DocumentFragment(weak_reference) => {
                weak_reference.upgrade(self.env)?.unwrap().into()
            },
            ParentNode::Element(weak_reference) => {
                weak_reference.upgrade(self.env)?.unwrap().into()
            },
        };
        Ok(node)
    }
}
