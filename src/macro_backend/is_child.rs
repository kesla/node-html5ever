use napi::Result;

use crate::{
    ChildNode,
    NodeHandler,
    ParentNode,
};

pub(crate) fn parent<T>(node_handler: NodeHandler) -> Result<Option<T>>
where
    ParentNode: TryInto<T>,
{
    node_handler.parent_iterator().try_next()
}

pub(crate) fn remove(child: ChildNode) -> Result<()> {
    child.remove()
}

pub(crate) fn previous<T>(node_handler: NodeHandler) -> Result<Option<T>>
where
    ChildNode: TryInto<T>,
{
    Ok(node_handler.previous_iterator()?.next())
}

pub(crate) fn next<T>(node_handler: NodeHandler) -> Result<Option<T>>
where
    ChildNode: TryInto<T>,
{
    Ok(node_handler.next_iterator()?.next())
}
