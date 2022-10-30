use std::{
    ops::Deref,
    rc::Rc,
};

use napi::{
    bindgen_prelude::Reference,
    Env,
    Error,
    Result,
};

use crate::{
    ChildNode,
    Comment,
    Document,
    DocumentFragment,
    DocumentType,
    EinarCell,
    Element,
    Node,
    ParentNode,
    Text,
};

mod child_node_list;
mod parent_context;

use self::child_node_list::ChildNodeList;
pub use self::parent_context::ParentContext;

pub struct NodeDataInner {
    pub(crate) env: Env,
    pub(crate) child_nodes: EinarCell<ChildNodeList>,
    pub(crate) parent_context: EinarCell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct NodeData(Rc<NodeDataInner>);

impl Deref for NodeData {
    type Target = NodeDataInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NodeData {
    pub(crate) fn new(env: Env) -> Self {
        NodeData(Rc::new(NodeDataInner {
            env,
            child_nodes: Default::default(),
            parent_context: Default::default(),
        }))
    }
}

impl TryFrom<&ParentContext> for NodeData {
    type Error = Error;

    fn try_from(parent_context: &ParentContext) -> Result<Self> {
        match &parent_context.node {
            ParentNode::Document(document) => {
                let document = document
                    .upgrade(parent_context.env)?
                    .expect("Document is gone");
                Ok(document.into())
            },
            ParentNode::DocumentFragment(document_fragment) => {
                let document_fragment = document_fragment
                    .upgrade(parent_context.env)?
                    .expect("DocumentFragment is gone");
                Ok(document_fragment.into())
            },
            ParentNode::Element(element) => {
                let element = element
                    .upgrade(parent_context.env)?
                    .expect("Element is gone");
                Ok(element.into())
            },
        }
    }
}

impl From<ChildNode> for NodeData {
    fn from(node: ChildNode) -> Self {
        From::from(&node)
    }
}

impl From<&ChildNode> for NodeData {
    fn from(node: &ChildNode) -> Self {
        match node {
            ChildNode::Comment(r) => From::from(r),
            ChildNode::DocumentType(r) => From::from(r),
            ChildNode::Element(r) => From::from(r),
            ChildNode::Text(r) => From::from(r),
        }
    }
}

impl From<Node> for NodeData {
    fn from(node: Node) -> Self {
        From::from(&node)
    }
}

impl From<&Node> for NodeData {
    fn from(node: &Node) -> Self {
        match node {
            Node::Comment(r) => From::from(r),
            Node::DocumentType(r) => From::from(r),
            Node::Document(r) => From::from(r),
            Node::DocumentFragment(r) => From::from(r),
            Node::Element(r) => From::from(r),
            Node::Text(r) => From::from(r),
        }
    }
}

macro_rules! impl_from {
    ($type:ty) => {
        impl From<Reference<$type>> for NodeData {
            fn from(r: Reference<$type>) -> Self {
                From::from(&r)
            }
        }

        impl From<&Reference<$type>> for NodeData {
            fn from(r: &Reference<$type>) -> Self {
                From::from(r.deref())
            }
        }

        impl From<&$type> for NodeData {
            fn from(value: &$type) -> Self {
                value.node_data.clone()
            }
        }
    };
}

impl_from!(Comment);
impl_from!(Document);
impl_from!(DocumentFragment);
impl_from!(DocumentType);
impl_from!(Element);
impl_from!(Text);
