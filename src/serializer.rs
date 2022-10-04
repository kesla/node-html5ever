use std::collections::VecDeque;

use html5ever::{
  serialize::{SerializeOpts, TraversalScope},
  QualName,
};

use crate::{Handle, NodeData};

struct SerializableHandle(Handle);

enum SerializeOp {
  Open(Handle),
  Close(QualName),
}

impl html5ever::serialize::Serialize for SerializableHandle {
  fn serialize<S>(
    &self,
    serializer: &mut S,
    traversal_scope: html5ever::serialize::TraversalScope,
  ) -> std::io::Result<()>
  where
    S: html5ever::serialize::Serializer,
  {
    let mut ops = VecDeque::new();
    match traversal_scope {
      html5ever::serialize::TraversalScope::IncludeNode => {
        ops.push_back(SerializeOp::Open(self.0.clone()))
      }
      html5ever::serialize::TraversalScope::ChildrenOnly(_) => {
        if let Some(children) = self.0.get_children() {
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
        SerializeOp::Open(handle) => {
          let node_data: &NodeData = &handle;
          match node_data {
            NodeData::Comment(comment) => serializer.write_comment(&comment.content)?,
            NodeData::DocType(doc_type) => serializer.write_doctype(&doc_type.name)?,
            NodeData::Element(element) => {
              let list = element.list.borrow();
              serializer.start_elem(
                // TODO: Is this actually copying the data? Need to figure that out
                element.name.clone(),
                element
                  .attributes_wrapper
                  .iter()
                  .map(|at| (&at.name, &at.value[..])),
              )?;
              ops.reserve(1 + list.len());
              ops.push_front(SerializeOp::Close(element.name.clone()));

              for child in list.iter().rev() {
                ops.push_front(SerializeOp::Open(child.clone()));
              }
            }
            NodeData::Document(_) => panic!("Can't serialize Document node itself"),
            NodeData::Text(text) => serializer.write_text(&text.content)?,
            NodeData::None => panic!("Can't serialize None node"),
          }
        }
        SerializeOp::Close(name) => serializer.end_elem(name)?,
      }
    }

    Ok(())
  }
}

pub fn serialize(handle: Handle, traversal_scope: TraversalScope) -> String {
  let serializable_handle: SerializableHandle = SerializableHandle(handle);
  let mut serialized = Vec::new();
  html5ever::serialize::serialize(
    &mut serialized,
    &serializable_handle,
    SerializeOpts {
      traversal_scope,
      ..Default::default()
    },
  )
  .unwrap();

  String::from_utf8(serialized).unwrap()
}