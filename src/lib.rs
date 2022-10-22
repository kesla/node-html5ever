#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod einar_cell;
mod id;
mod iterators;
mod macro_backend;
mod node;
mod node_handler;
mod nodes;
mod quirks_mode;
mod selectors;
mod serializer;
mod style_declaration;

pub use crate::selectors::Selectors;
pub use dom::Html5everDom;
pub use einar_cell::EinarCell;
pub use id::get_id;
pub use iterators::*;
pub use node::{ChildNode, Node, ParentNode};
pub use node_handler::{NodeHandler, ParentContext};
pub use nodes::{Comment, Document, DocumentFragment, DocumentType, Element, ElementRef, Text};
pub use quirks_mode::QuirksMode;
pub use serializer::serialize;
pub use style_declaration::StyleDeclaration;
