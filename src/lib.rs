#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod case;
mod cyclic_reference;
mod dom;
mod einar_cell;
mod id;
mod iterators;
mod lazy_reference;
mod node;
mod node_data;
mod nodes;
mod quirks_mode;
mod selectors;
mod serializer;
mod style_declaration;
mod traits;

pub use case::*;
pub use cyclic_reference::CyclicReference;
pub use dom::Html5everDom;
pub use einar_cell::EinarCell;
pub use id::get_id;
pub use iterators::*;
pub use lazy_reference::LazyReference;
pub use node::{
    ChildNode,
    InsertPosition,
    Node,
    ParentNode,
};
pub use node_data::{
    NodeData,
    ParentContext,
};
pub use nodes::{
    Comment,
    Document,
    DocumentFragment,
    DocumentType,
    Element,
    ElementRef,
    Text,
};
pub use quirks_mode::QuirksMode;
pub use serializer::serialize;
pub use style_declaration::StyleDeclaration;
pub use traits::*;

pub use crate::selectors::Selectors;
