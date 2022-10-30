use napi::{
    bindgen_prelude::Reference,
    Either,
    Env,
    Result,
};

use crate::{
    ChildNode,
    Element,
    InsertPosition,
    Node,
    Text,
};

pub(crate) fn children<T>(node: Node) -> Vec<T>
where
    ChildNode: TryInto<T>,
{
    node.shallow_child_nodes_iter().collect()
}

pub(crate) fn append_child(
    parent_node: Node,
    child_node: ChildNode,
) -> Result<ChildNode> {
    parent_node.insert_node(&child_node, &InsertPosition::Append)?;
    Ok(child_node)
}

pub(crate) fn prepend(
    env: Env,
    parent_node: Node,
    node: Either<ChildNode, String>,
) -> Result<()> {
    let child_node: ChildNode = match node {
        Either::A(child_node) => child_node,
        Either::B(data) => {
            let text = Text::new_reference(env, data)?;
            text.into()
        },
    };

    parent_node.insert_node(&child_node, &InsertPosition::Prepend)?;

    Ok(())
}

pub(crate) fn append(
    env: Env,
    parent_node: Node,
    node: Either<ChildNode, String>,
) -> Result<()> {
    let child_node: ChildNode = match node {
        Either::A(child_node) => child_node,
        Either::B(data) => {
            let text = Text::new_reference(env, data)?;
            text.into()
        },
    };

    parent_node.insert_node(&child_node, &InsertPosition::Append)?;

    Ok(())
}

pub(crate) fn insert_before(
    parent: Node,
    new_node: ChildNode,
    reference_node: Node,
) -> Result<ChildNode> {
    let position = reference_node.get_position()?;

    parent.insert_node(&new_node, &InsertPosition::InsertBefore(position))?;

    Ok(new_node)
}

pub(crate) fn remove_child(
    parent_node: Node,
    child_node: ChildNode,
) -> Result<ChildNode> {
    parent_node.remove_node(&child_node)?;
    Ok(child_node)
}

pub(crate) fn get_element_by_id(
    node: Node,
    id: String,
) -> Option<Reference<Element>> {
    node.deep_child_nodes_iter()
        .find(|e: &Reference<Element>| e.get_id() == id)
}

pub(crate) fn get_elements_by_class_name(
    node: Node,
    class_name: String,
) -> Vec<Reference<Element>> {
    node.deep_child_nodes_iter()
        .filter(|e: &Reference<Element>| e.get_class_name() == class_name)
        .collect()
}

pub(crate) fn get_elements_by_tag_name(
    node: Node,
    tag_name: String,
) -> Vec<Reference<Element>> {
    let tag_name: &str = &tag_name;

    node.deep_child_nodes_iter()
        .filter(|e: &Reference<Element>| {
            e.get_tag_name().eq_ignore_ascii_case(tag_name)
        })
        .collect()
}

pub(crate) fn first_child<T>(node: Node) -> Option<T>
where
    ChildNode: TryInto<T>,
{
    node.shallow_child_nodes_iter().next()
}

pub(crate) fn last_child<T>(node: Node) -> Option<T>
where
    ChildNode: TryInto<T>,
{
    node.shallow_child_nodes_iter().next_back()
}

pub(crate) fn query_selector_all(
    node: Node,
    selectors: String,
) -> Result<Vec<Reference<Element>>> {
    node.selectors_iter(selectors)?.collect()
}

pub(crate) fn query_selector(
    node: Node,
    selectors: String,
) -> Result<Option<Reference<Element>>> {
    node.selectors_iter(selectors)?.try_next()
}

pub(crate) fn normalize(node: Node) -> Result<()> {
    node.normalize()
}
