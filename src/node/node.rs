use html5ever::{
    namespace_url,
    ns,
    QualName,
};
use napi::{
    bindgen_prelude::{
        Error,
        Reference,
        Result,
    },
    Status,
};

use crate::{
    ChildNode,
    Comment,
    DeepChildNodesIterator,
    Document,
    DocumentFragment,
    DocumentType,
    Element,
    NodeData,
    ParentContext,
    ParentIterator,
    SelectorsIterator,
    ShallowChildNodesIterator,
    SiblingIterator,
    SiblingIteratorType,
    Text,
};

pub enum Node {
    Comment(Reference<Comment>),
    DocumentType(Reference<DocumentType>),
    Document(Reference<Document>),
    DocumentFragment(Reference<DocumentFragment>),
    Element(Reference<Element>),
    Text(Reference<Text>),
}

impl From<ChildNode> for Node {
    fn from(value: ChildNode) -> Self {
        match value {
            ChildNode::Comment(r) => Node::Comment(r),
            ChildNode::DocumentType(r) => Node::DocumentType(r),
            ChildNode::Element(r) => Node::Element(r),
            ChildNode::Text(r) => Node::Text(r),
        }
    }
}

macro_rules! impl_from {
    ($type:ty, $variant:ident) => {
        impl From<&$type> for Node {
            fn from(value: &$type) -> Self {
                Node::$variant(value.cyclic_reference.get().unwrap())
            }
        }

        impl From<Reference<$type>> for Node {
            fn from(value: Reference<$type>) -> Self {
                Node::$variant(value)
            }
        }
    };
}

impl_from!(Comment, Comment);
impl_from!(DocumentType, DocumentType);
impl_from!(Document, Document);
impl_from!(DocumentFragment, DocumentFragment);
impl_from!(Element, Element);
impl_from!(Text, Text);

impl PartialEq for Node {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::Comment(left), Self::Comment(right)) => left.id == right.id,
            (Self::DocumentType(left), Self::DocumentType(right)) => {
                left.id == right.id
            },
            (Self::Document(left), Self::Document(right)) => {
                left.id == right.id
            },
            (Self::DocumentFragment(left), Self::DocumentFragment(right)) => {
                left.id == right.id
            },
            (Self::Element(left), Self::Element(right)) => left.id == right.id,
            (Self::Text(left), Self::Text(right)) => left.id == right.id,
            _ => false,
        }
    }
}

impl Eq for Node {}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Self::Comment(arg0) => Self::Comment(arg0.clone(arg0.env).unwrap()),
            Self::DocumentType(arg0) => {
                Self::DocumentType(arg0.clone(arg0.env).unwrap())
            },
            Self::Document(arg0) => {
                Self::Document(arg0.clone(arg0.env).unwrap())
            },
            Self::DocumentFragment(arg0) => {
                Self::DocumentFragment(arg0.clone(arg0.env).unwrap())
            },
            Self::Element(arg0) => Self::Element(arg0.clone(arg0.env).unwrap()),
            Self::Text(arg0) => Self::Text(arg0.clone(arg0.env).unwrap()),
        }
    }
}

pub enum InsertPosition {
    Prepend,
    Append,
    InsertBefore(usize),
}

impl Node {
    pub(crate) fn as_element(&self) -> Result<&Reference<Element>> {
        match &self {
            Node::Element(r) => Ok(r),
            _ => Err(Error::new(
                Status::InvalidArg,
                "Node is not an Element".to_string(),
            )),
        }
    }

    pub(crate) fn insert_node(
        &self,
        child_node: &ChildNode,
        position: &InsertPosition,
    ) -> Result<()> {
        // remove from old parent
        child_node.remove()?;

        // TODO: concatenate already existing text node

        let node_data: NodeData = self.into();
        node_data
            .child_nodes
            .borrow_mut(|child_nodes| match position {
                InsertPosition::Prepend => child_nodes.prepend_node(child_node),
                InsertPosition::Append => child_nodes.append_node(child_node),
                InsertPosition::InsertBefore(position) => {
                    child_nodes.insert_node(child_node, position.to_owned())
                },
            });

        self.sync_parent_context();

        Ok(())
    }

    pub(crate) fn remove_node(
        &self,
        child_node: &ChildNode,
    ) -> Result<()> {
        let parent_node_data: NodeData = self.into();
        parent_node_data
            .child_nodes
            .borrow_mut(|child_nodes| child_nodes.remove_node(child_node))?;

        self.sync_parent_context();

        let child_node_data: NodeData = child_node.into();
        child_node_data.parent_context.set(None);

        Ok(())
    }

    fn sync_parent_context(&self) {
        let parent_node_data: NodeData = self.into();
        parent_node_data.child_nodes.borrow(|child_nodes| {
            for index in 0..child_nodes.len() {
                let node_data: NodeData =
                    child_nodes.get(index).unwrap().into();

                node_data.parent_context.borrow_mut(|parent_context| {
                    if let Some(mut ctx) = parent_context.as_mut() {
                        ctx.position = index;
                        ctx.node = self.into();
                    } else {
                        *parent_context = Some(ParentContext::new(
                            node_data.env,
                            self.into(),
                            index,
                        ));
                    }
                })
            }
        });
    }

    pub(crate) fn get_position(&self) -> Result<usize> {
        let node_data: NodeData = self.into();
        node_data
            .parent_context
            .borrow(|maybe_ctx| maybe_ctx.as_ref().map(|ctx| ctx.position))
            .ok_or_else(|| {
                Error::new(Status::InvalidArg, "Node has no parent".to_string())
            })
    }

    pub(crate) fn get_node_name(&self) -> String {
        match self {
            Node::Comment(_) => "#comment".to_string(),
            Node::DocumentType(_) => "#docType".to_string(),
            Node::Document(_) => "#document".to_string(),
            Node::DocumentFragment(_) => "#document-fragment".to_string(),
            Node::Element(r) => r.name.local.to_string().to_uppercase(),
            Node::Text(_) => "#text".to_string(),
        }
    }

    pub(crate) fn get_qual_name(&self) -> QualName {
        match self {
            Node::Element(r) => r.name.clone(),
            _ => QualName::new(None, ns!(html), self.get_node_name().into()),
        }
    }

    pub(crate) fn parent_iterator<T>(&self) -> ParentIterator<T> {
        let node_data: NodeData = self.into();

        ParentIterator::new(node_data.parent_context.cloned())
    }

    pub(crate) fn deep_child_nodes_iter<T>(&self) -> DeepChildNodesIterator<T>
    where
        ChildNode: TryInto<T>,
    {
        DeepChildNodesIterator::new(self.into())
    }

    pub(crate) fn shallow_child_nodes_iter<T>(
        &self
    ) -> ShallowChildNodesIterator<T>
    where
        ChildNode: TryInto<T>,
    {
        ShallowChildNodesIterator::new(self.into())
    }

    fn new_sibling_iterator<T>(
        &self,
        sibling_type: SiblingIteratorType,
    ) -> Result<SiblingIterator<T>> {
        let node_data: NodeData = self.into();

        SiblingIterator::new(node_data.parent_context.cloned(), sibling_type)
    }

    pub(crate) fn previous_iterator<T>(&self) -> Result<SiblingIterator<T>> {
        self.new_sibling_iterator(SiblingIteratorType::Previous)
    }

    pub(crate) fn next_iterator<T>(&self) -> Result<SiblingIterator<T>> {
        self.new_sibling_iterator(SiblingIteratorType::Next)
    }

    pub(crate) fn selectors_iter(
        &self,
        selectors: String,
    ) -> Result<SelectorsIterator> {
        Ok(SelectorsIterator::new(
            crate::Selectors::compile(selectors)?,
            self.deep_child_nodes_iter(),
        ))
    }

    pub(crate) fn try_get_child_node<T>(
        &self,
        index: usize,
    ) -> Result<Option<T>>
    where
        ChildNode: TryInto<T, Error = Error>,
    {
        let maybe_child_node = self.get_child_node(index);
        maybe_child_node
            .map(|child_node| child_node.try_into())
            .transpose()
    }

    pub(crate) fn get_child_node(
        &self,
        index: usize,
    ) -> Option<ChildNode> {
        let node_data: NodeData = self.into();

        node_data
            .child_nodes
            .borrow(|child_nodes| child_nodes.get(index).cloned())
    }

    pub(crate) fn normalize(&self) -> Result<()> {
        let mut iter: ShallowChildNodesIterator<ChildNode> =
            self.shallow_child_nodes_iter();

        while let Some(ref mut child) = iter.next() {
            if let ChildNode::Element(element) = child {
                element.normalize()?;
                continue;
            }

            if let ChildNode::Text(ref mut text) = child {
                if text.data.is_empty() {
                    text.remove()?;
                    continue;
                }

                while let Some(ref next_child) = iter.next() {
                    if let ChildNode::Text(next_text) = next_child {
                        text.data.push_str(&next_text.data);
                        next_text.remove()?;
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
