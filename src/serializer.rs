use std::collections::VecDeque;

use html5ever::{
  serialize::{SerializeOpts, TraversalScope},
  QualName,
};

use crate::{Node, NodeHandler};

struct SerializableNodeHandler(Node);

enum SerializeOp {
  Open(Node),
  Close(QualName),
}

impl html5ever::serialize::Serialize for SerializableNodeHandler {
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
        let node_handler: NodeHandler = (&self.0).into();
        let children = node_handler.get_child_nodes();
        ops.extend(
          children
            .iter()
            .map(|child| SerializeOp::Open(child.clone())),
        );
      }
    };

    while let Some(op) = ops.pop_front() {
      match op {
        SerializeOp::Open(node) => {
          match &node {
            Node::Comment(comment) => serializer.write_comment(&comment.data)?,
            Node::DocumentType(doc_type) => serializer.write_doctype(&doc_type.name)?,
            Node::Element(element) => {
              let node_handler = element.get_node_handler();
              let list = node_handler.get_child_nodes();
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
            Node::Document(_) => panic!("Can't serialize Document node itself"),
            Node::DocumentFragment(_) => panic!("Can't serialize DocumentFragment node itself"),
            Node::Text(text) => serializer.write_text(&text.data)?,
          }
        }
        SerializeOp::Close(name) => serializer.end_elem(name)?,
      }
    }

    Ok(())
  }
}

pub fn serialize(handle: Node, traversal_scope: TraversalScope) -> String {
  let serializable_node_handler: SerializableNodeHandler = SerializableNodeHandler(handle);
  let mut serialized = Vec::new();
  html5ever::serialize::serialize(
    &mut serialized,
    &serializable_node_handler,
    SerializeOpts {
      traversal_scope,
      ..Default::default()
    },
  )
  .unwrap();

  String::from_utf8(serialized).unwrap()
}
