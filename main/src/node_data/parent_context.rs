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
    pub(crate) position: usize,
    pub(crate) env: Env,
}

impl ParentContext {
    pub(crate) fn new(
        env: Env,
        node: ParentNode,
        position: usize,
    ) -> Self {
        ParentContext {
            env,
            node,
            position,
        }
    }

    pub(crate) fn get_node(&self) -> Result<Node> {
        self.node.upgrade(self.env)
    }
}
