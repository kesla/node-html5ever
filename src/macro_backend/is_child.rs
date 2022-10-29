use napi::Result;

use crate::{
    ChildNode,
    Node,
    ParentNode,
};

pub(crate) fn parent<T>(node: Node) -> Result<Option<T>>
where
    ParentNode: TryInto<T>,
{
    node.parent_iterator().try_next()
}

pub(crate) fn remove(child: ChildNode) -> Result<()> {
    child.remove()
}

pub(crate) fn previous<T>(node: Node) -> Result<Option<T>>
where
    ChildNode: TryInto<T>,
{
    Ok(node.previous_iterator()?.next())
}

pub(crate) fn next<T>(node: Node) -> Result<Option<T>>
where
    ChildNode: TryInto<T>,
{
    Ok(node.next_iterator()?.next())
}
