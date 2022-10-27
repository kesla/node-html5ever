use std::marker::PhantomData;

use napi::Result;

use crate::{
    NodeHandler,
    ParentContext,
    ParentNode,
};

pub struct ParentIterator<T> {
    pub(crate) maybe_parent_ctx: Option<ParentContext>,
    _phantom: PhantomData<T>,
}

impl<T> ParentIterator<T> {
    pub(crate) fn new(maybe_parent_ctx: Option<ParentContext>) -> Self {
        Self {
            maybe_parent_ctx,
            _phantom: PhantomData,
        }
    }

    fn next_parent_node(&mut self) -> Option<Result<ParentNode>> {
        let parent_ctx = match &self.maybe_parent_ctx {
            Some(ctx) => ctx,
            None => return None,
        };
        let result = Some(Ok(parent_ctx.node.clone()));

        let node_handler: NodeHandler = match parent_ctx.try_into() {
            Ok(node_handler) => node_handler,
            Err(err) => return Some(Err(err)),
        };

        self.maybe_parent_ctx = node_handler.parent_context.cloned();
        result
    }
}

impl<T> ParentIterator<T>
where
    ParentNode: TryInto<T>,
{
    pub fn try_next(&mut self) -> Result<Option<T>> {
        self.next().transpose()
    }
}

impl<T> Iterator for ParentIterator<T>
where
    ParentNode: TryInto<T>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_or_error) = self.next_parent_node() {
            let node = match node_or_error {
                Ok(node) => node,
                Err(err) => return Some(Err(err)),
            };

            if let Ok(parent) = node.try_into() {
                return Some(Ok(parent));
            }
        }

        None
    }
}
