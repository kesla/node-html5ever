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
pub use id::get_id;
pub use node_handler::{NodeHandler, ParentContext};
pub use nodes::{Comment, DocType, Document, DocumentFragment, Element, Text};
pub use quirks_mode::QuirksMode;
pub use serializer::serialize;
