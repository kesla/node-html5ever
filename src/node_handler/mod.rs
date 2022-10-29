use std::{
    ops::Deref,
    rc::Rc,
};

use napi::{
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

pub struct NodeHandlerInner {
    pub(crate) env: Env,
    pub(crate) child_nodes: EinarCell<ChildNodeList>,
    pub(crate) parent_context: EinarCell<Option<ParentContext>>,
}

#[derive(Clone)]
pub struct NodeHandler(Rc<NodeHandlerInner>);

impl Deref for NodeHandler {
    type Target = NodeHandlerInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NodeHandler {
    pub(crate) fn new(env: Env) -> Self {
        NodeHandler(Rc::new(NodeHandlerInner {
            env,
            child_nodes: Default::default(),
            parent_context: Default::default(),
        }))
    }
}

impl TryFrom<&ParentContext> for NodeHandler {
    type Error = Error;

    fn try_from(parent_context: &ParentContext) -> Result<Self> {
        match &parent_context.node {
            ParentNode::Document(document) => {
                let document = document
                    .upgrade(parent_context.env)?
                    .expect("Document is gone");
                Ok(document.get_node_handler())
            },
            ParentNode::DocumentFragment(document_fragment) => {
                let document_fragment = document_fragment
                    .upgrade(parent_context.env)?
                    .expect("DocumentFragment is gone");
                Ok(document_fragment.get_node_handler())
            },
            ParentNode::Element(element) => {
                let element = element
                    .upgrade(parent_context.env)?
                    .expect("Element is gone");
                Ok(element.get_node_handler())
            },
        }
    }
}

impl From<ChildNode> for NodeHandler {
    fn from(e: ChildNode) -> Self {
        match e {
            ChildNode::Comment(r) => r.get_node_handler(),
            ChildNode::DocumentType(r) => r.get_node_handler(),
            ChildNode::Element(r) => r.get_node_handler(),
            ChildNode::Text(r) => r.get_node_handler(),
        }
    }
}

impl From<&ChildNode> for NodeHandler {
    fn from(e: &ChildNode) -> Self {
        match e {
            ChildNode::Comment(r) => r.get_node_handler(),
            ChildNode::DocumentType(r) => r.get_node_handler(),
            ChildNode::Element(r) => r.get_node_handler(),
            ChildNode::Text(r) => r.get_node_handler(),
        }
    }
}

impl From<Node> for NodeHandler {
    fn from(node: Node) -> Self {
        match node {
            Node::Comment(r) => r.get_node_handler(),
            Node::DocumentType(r) => r.get_node_handler(),
            Node::Document(r) => r.get_node_handler(),
            Node::DocumentFragment(r) => r.get_node_handler(),
            Node::Element(r) => r.get_node_handler(),
            Node::Text(r) => r.get_node_handler(),
        }
    }
}

impl From<&Node> for NodeHandler {
    fn from(node: &Node) -> Self {
        match node {
            Node::Comment(r) => r.get_node_handler(),
            Node::DocumentType(r) => r.get_node_handler(),
            Node::Document(r) => r.get_node_handler(),
            Node::DocumentFragment(r) => r.get_node_handler(),
            Node::Element(r) => r.get_node_handler(),
            Node::Text(r) => r.get_node_handler(),
        }
    }
}

macro_rules! impl_from {
    ($type:ty) => {
        impl From<&$type> for NodeHandler {
            fn from(value: &$type) -> Self {
                value.get_node_handler()
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
