#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod id;
mod macro_backend;
mod node;
mod node_handler;
mod nodes;
mod quirks_mode;
mod selectors;
mod serializer;

pub use crate::selectors::Selectors;
pub use dom::Html5everDom;
pub use id::get_id;
pub use node::{ChildNode, Node, ParentNode};
pub use node_handler::{NodeHandler, ParentContext};
pub use nodes::{Comment, Document, DocumentFragment, DocumentType, Element, Text};
pub use quirks_mode::QuirksMode;
pub use serializer::serialize;
