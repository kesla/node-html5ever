use std::collections::VecDeque;

use html5ever::{
  serialize::{SerializeOpts, TraversalScope},
  QualName,
};

use crate::handle::{self, Handle};

struct SerializableNode(Handle);

enum SerializeOp {
  Open(Handle),
  Close(QualName),
}

impl html5ever::serialize::Serialize for SerializableNode {
  fn serialize<S>(
    &self,
    serializer: &mut S,
    traversal_scope: html5ever::serialize::TraversalScope,
  ) -> std::io::Result<()>
  where
    S: html5ever::serialize::Serializer,
  {
    let env = self.0.env;

    let mut ops = VecDeque::new();
    match traversal_scope {
      html5ever::serialize::TraversalScope::IncludeNode => {
        ops.push_back(SerializeOp::Open(self.0.clone()))
      }
      html5ever::serialize::TraversalScope::ChildrenOnly(_) => {
        let maybe_children = match &self.0.inner {
          handle::Inner::Document(r) => Some(r.list.clone(env).unwrap()),
          handle::Inner::Element(r) => Some(r.list.clone(env).unwrap()),
          _ => None,
        };

        if let Some(children) = maybe_children {
          ops.extend(
            children
              .iter()
              .map(|child| SerializeOp::Open(child.clone())),
          );
        }
      }
    };

    while let Some(op) = ops.pop_front() {
      match op {
        SerializeOp::Open(handle) => match handle.inner {
          handle::Inner::Comment(comment) => serializer.write_comment(&comment.content)?,
          handle::Inner::DocType(doc_type) => serializer.write_doctype(&doc_type.name)?,
          handle::Inner::Element(element) => {
            serializer.start_elem(
              // TODO: Is this actually copying the data? Need to figure that out
              element.name.clone(),
              element.attrs.iter().map(|at| (&at.name, &at.value[..])),
            )?;
            ops.reserve(1 + element.list.len());
            ops.push_front(SerializeOp::Close(element.name.clone()));

            for child in element.list.iter().rev() {
              ops.push_front(SerializeOp::Open(child.clone()));
            }
          }
          handle::Inner::Document(_) => panic!("Can't serialize Document node itself"),
          handle::Inner::Text(text) => serializer.write_text(&text.content)?,
        },
        SerializeOp::Close(name) => serializer.end_elem(name)?,
      }
    }

    Ok(())
  }
}

pub fn serialize(handle: &Handle, traversal_scope: TraversalScope) -> String {
  let serializable_node: SerializableNode = SerializableNode(handle.clone());
  let mut serialized = Vec::new();
  html5ever::serialize::serialize(
    &mut serialized,
    &serializable_node,
    SerializeOpts {
      traversal_scope,
      ..Default::default()
    },
  )
  .unwrap();

  String::from_utf8(serialized).unwrap()
}
