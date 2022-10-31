use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::Deref,
};

use napi::{
    bindgen_prelude::{
        FromNapiValue,
        Reference,
        Result,
        ToNapiValue,
        TypeName,
        ValidateNapiValue,
    },
    Error,
    Status,
    ValueType,
};

use crate::{
    Comment,
    DocumentType,
    Element,
    Node,
    NodeData,
    ParentContext,
    Text,
};

pub enum ChildNode {
    Comment(Reference<Comment>),
    DocumentType(Reference<DocumentType>),
    Element(Reference<Element>),
    Text(Reference<Text>),
}

impl ChildNode {
    pub(crate) fn remove(&self) -> Result<()> {
        let node_data: NodeData = self.into();

        let parent_ctx: ParentContext =
            match node_data.parent_context.replace(None) {
                Some(parent) => parent,
                None => return Ok(()),
            };

        let parent_node = parent_ctx.get_node()?;
        parent_node.remove_node(self)
    }

    pub(crate) fn clone_node(
        &self,
        deep: Option<bool>,
    ) -> Result<Self> {
        let cloned = match self {
            ChildNode::Comment(r) => r.clone_node()?.into(),
            ChildNode::DocumentType(r) => r.clone_node(deep)?.into(),
            ChildNode::Element(r) => r.clone_node(deep)?.into(),
            ChildNode::Text(r) => r.clone_node()?.into(),
        };
        Ok(cloned)
    }
}

impl PartialEq for ChildNode {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::Comment(left), Self::Comment(right)) => left.id == right.id,
            (Self::DocumentType(left), Self::DocumentType(right)) => {
                left.id == right.id
            },
            (Self::Element(left), Self::Element(right)) => left.id == right.id,
            (Self::Text(left), Self::Text(right)) => left.id == right.id,
            _ => false,
        }
    }
}

impl Eq for ChildNode {}

impl Clone for ChildNode {
    fn clone(&self) -> Self {
        match self {
            Self::Comment(arg0) => Self::Comment(arg0.clone(arg0.env).unwrap()),
            Self::DocumentType(arg0) => {
                Self::DocumentType(arg0.clone(arg0.env).unwrap())
            },
            Self::Element(arg0) => Self::Element(arg0.clone(arg0.env).unwrap()),
            Self::Text(arg0) => Self::Text(arg0.clone(arg0.env).unwrap()),
        }
    }
}

impl ToNapiValue for ChildNode {
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        val: Self,
    ) -> Result<napi::sys::napi_value> {
        match val {
            ChildNode::Comment(r) => {
                Reference::<Comment>::to_napi_value(env, r)
            },
            ChildNode::DocumentType(r) => {
                Reference::<DocumentType>::to_napi_value(env, r)
            },
            ChildNode::Element(r) => {
                Reference::<Element>::to_napi_value(env, r)
            },
            ChildNode::Text(r) => Reference::<Text>::to_napi_value(env, r),
        }
    }
}

impl ValidateNapiValue for ChildNode {
    unsafe fn validate(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> Result<napi::sys::napi_value> {
        <&Element>::validate(env, napi_val)
            .or_else(|_| <&Text>::validate(env, napi_val))
            .or_else(|_| <&DocumentType>::validate(env, napi_val))
            .or_else(|_| <&Text>::validate(env, napi_val))
    }
}

impl FromNapiValue for ChildNode {
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> Result<Self> {
        if <&Element>::validate(env, napi_val).is_ok() {
            <&Element>::from_napi_value(env, napi_val).map(|r| r.into())
        } else if <&Text>::validate(env, napi_val).is_ok() {
            <&Text>::from_napi_value(env, napi_val).map(|r| r.into())
        } else if <&Comment>::validate(env, napi_val).is_ok() {
            <&Comment>::from_napi_value(env, napi_val).map(|r| r.into())
        } else if <&DocumentType>::validate(env, napi_val).is_ok() {
            <&DocumentType>::from_napi_value(env, napi_val).map(|r| r.into())
        } else {
            Err(Error::new(
        Status::InvalidArg,
        "Could not convert napi_value to ChildNode (Element, Text, Comment or DocumentType)"
          .to_string(),
      ))
        }
    }
}

impl TypeName for ChildNode {
    fn type_name() -> &'static str {
        "ChildNode"
    }

    fn value_type() -> ValueType {
        ValueType::Unknown
    }
}

impl Debug for ChildNode {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "ChildNode(")?;
        match self {
            ChildNode::Comment(r) => write!(f, "{:?}", r.deref()),
            ChildNode::DocumentType(r) => write!(f, "{:?}", r.deref()),
            ChildNode::Element(r) => write!(f, "{:?}", r.deref()),
            ChildNode::Text(r) => write!(f, "{:?}", r.deref()),
        }?;
        write!(f, ")")
    }
}

macro_rules! impl_into_from {
    ($type:ty, $variant:ident) => {
        impl From<&$type> for ChildNode {
            fn from(value: &$type) -> Self {
                ChildNode::$variant(value.cyclic_reference.get().unwrap())
            }
        }

        impl From<Reference<$type>> for ChildNode {
            fn from(value: Reference<$type>) -> Self {
                ChildNode::$variant(value)
            }
        }

        impl TryFrom<ChildNode> for Reference<$type> {
            type Error = Error;

            fn try_from(value: ChildNode) -> Result<Self> {
                match value {
                    ChildNode::$variant(r) => Ok(r),
                    _ => Err(Error::new(
                        Status::InvalidArg,
                        format!(
                            "Could not convert ChildNode to {}",
                            stringify!($type)
                        ),
                    )),
                }
            }
        }
    };
}

impl_into_from!(Comment, Comment);
impl_into_from!(DocumentType, DocumentType);
impl_into_from!(Element, Element);
impl_into_from!(Text, Text);

impl From<Node> for ChildNode {
    fn from(val: Node) -> Self {
        match val {
            Node::Comment(r) => ChildNode::Comment(r.clone(r.env).unwrap()),
            Node::DocumentType(r) => {
                ChildNode::DocumentType(r.clone(r.env).unwrap())
            },
            Node::Element(r) => ChildNode::Element(r.clone(r.env).unwrap()),
            Node::Text(r) => ChildNode::Text(r.clone(r.env).unwrap()),
            Node::Document(_) => panic!("Document is not a Node"),
            Node::DocumentFragment(_) => {
                panic!("DocumentFragment is not a Node")
            },
        }
    }
}

impl From<&Node> for ChildNode {
    fn from(val: &Node) -> Self {
        match val {
            Node::Comment(r) => ChildNode::Comment(r.clone(r.env).unwrap()),
            Node::DocumentType(r) => {
                ChildNode::DocumentType(r.clone(r.env).unwrap())
            },
            Node::Element(r) => ChildNode::Element(r.clone(r.env).unwrap()),
            Node::Text(r) => ChildNode::Text(r.clone(r.env).unwrap()),
            Node::Document(_) => panic!("Document is not a Node"),
            Node::DocumentFragment(_) => {
                panic!("DocumentFragment is not a Node")
            },
        }
    }
}
