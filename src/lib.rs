#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate node_html5ever_derive;

mod dom;
mod handle;
mod id;
mod lazy_weak_handle;
mod macro_backend;
mod nodes;
mod quirks_mode;
mod serializer;

pub use dom::Html5everDom;
pub use handle::Handle;
pub(crate) use handle::{NodeReference, ParentContext, WeakHandle};
pub(crate) use id::get_id;
pub(crate) use lazy_weak_handle::LazyWeakHandle;
pub(crate) use nodes::{Comment, DocType, Document, Element, Text};
pub(crate) use quirks_mode::QuirksMode;
pub(crate) use serializer::serialize;
