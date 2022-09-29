#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod id;
mod lazy_weak_handle;
mod node_wrapper;
mod nodes;
mod quirks_mode;
mod serializer;

use std::rc::{Rc, Weak};

pub use dom::Html5everDom;
pub(crate) use id::get_id;
pub(crate) use lazy_weak_handle::LazyWeakHandle;
pub(crate) use node_wrapper::{NodeData, NodeWrapper};
pub(crate) use nodes::*;
pub(crate) use quirks_mode::QuirksMode;
pub(crate) use serializer::serialize;

pub(crate) type Handle = Rc<NodeWrapper>;

pub(crate) type WeakHandle = Weak<NodeWrapper>;
