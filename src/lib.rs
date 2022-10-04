#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod id;
mod lazy_weak_handle;
mod macro_backend;
mod node;
mod nodes;
mod parent_context;
mod quirks_mode;
mod serializer;

use std::rc::{Rc, Weak};

pub use dom::Html5everDom;
pub(crate) use id::get_id;
pub(crate) use lazy_weak_handle::LazyWeakHandle;
pub(crate) use node::NodeData;
pub(crate) use nodes::{Comment, DocType, Document, Element, Text};
pub(crate) use parent_context::ParentContext;
pub(crate) use quirks_mode::QuirksMode;
pub(crate) use serializer::serialize;

pub(crate) type Handle = Rc<NodeData>;

pub(crate) type WeakHandle = Weak<NodeData>;
