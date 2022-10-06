#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod id;
mod lazy_weak_handle;
mod macro_backend;
mod node_handler;
mod nodes;
mod quirks_mode;
mod serializer;

pub use dom::Html5everDom;
pub(crate) use id::get_id;
pub(crate) use lazy_weak_handle::LazyWeakNodeHandler;
pub use node_handler::NodeHandler;
pub(crate) use node_handler::{NodeReference, ParentContext};
pub(crate) use nodes::{Comment, DocType, Document, Element, Text};
pub(crate) use quirks_mode::QuirksMode;
pub(crate) use serializer::serialize;
