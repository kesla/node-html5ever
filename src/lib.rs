#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod handle;
mod id;
mod macro_backend;
mod node_handler;
mod nodes;
mod quirks_mode;
mod serializer;

pub use dom::Html5everDom;
pub use handle::Handle;
pub(crate) use id::get_id;
pub use node_handler::NodeHandler;
pub(crate) use node_handler::ParentContext;
pub(crate) use nodes::{Comment, DocType, Document, Element, Text};
pub(crate) use quirks_mode::QuirksMode;
pub(crate) use serializer::serialize;
