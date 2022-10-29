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
    Document,
    DocumentFragment,
    DocumentType,
    Element,
    NodeHandler,
    ParentContext,
    ParentNode,
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
        position: InsertPosition,
    ) -> Result<()> {
        // remove from old parent
        let node_handler: NodeHandler = child_node.into();

        if let Some(parent) = node_handler.parent_context.replace(None) {
            let parent_node_handler: NodeHandler = parent.get_node()?.into();

            parent_node_handler.child_nodes.borrow_mut(|child_nodes| {
                child_nodes.remove_node(child_node)
            })?;
        }

        // TODO: concatenate already existing text node

        let node_handler: NodeHandler = self.into();
        node_handler
            .child_nodes
            .borrow_mut(|child_nodes| match position {
                InsertPosition::Prepend => child_nodes.prepend_node(child_node),
                InsertPosition::Append => child_nodes.append_node(child_node),
                InsertPosition::InsertBefore(pos) => {
                    child_nodes.insert_node(child_node, pos)
                },
            });

        let parent_node: ParentNode = self.into();

        let parent_context = Some(ParentContext::new(
            node_handler.env,
            parent_node,
            node_handler.child_nodes_len() - 1,
        ));
        let node_handler = NodeHandler::from(child_node);
        node_handler.parent_context.set(parent_context);

        node_handler
            .child_nodes
            .borrow_mut(|child_nodes| child_nodes.sync_parent_context());

        Ok(())
    }

    pub(crate) fn remove_node(
        &self,
        child_node: &ChildNode,
    ) -> Result<()> {
        let parent_node_handler: NodeHandler = self.into();
        parent_node_handler
            .child_nodes
            .borrow_mut(|child_nodes| child_nodes.remove_node(child_node))?;

        let child_node_handler: NodeHandler = child_node.into();
        child_node_handler.parent_context.set(None);

        Ok(())
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
}
