use std::{borrow::Borrow, collections::VecDeque};

use html5ever::QualName;
// use markup5ever_rcdom::SerializabeHandle;

use crate::{node::Node, element::Element};

struct SerializableNode(Element);

enum SerializeOp {
  Open(Node),
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
    let mut ops = VecDeque::new();
    match traversal_scope {
      html5ever::serialize::TraversalScope::IncludeNode => {
        ops.push_back(SerializeOp::Open(self.0.clone()))
      }
      html5ever::serialize::TraversalScope::ChildrenOnly(_) => ops.extend(
        self
          .0
          .children()
          .unwrap()
          .child_nodes
          .iter()
          .map(|child| SerializeOp::Open(child.clone())),
      ),
    };

    while let Some(op) = ops.pop_front() {
      match op {
        SerializeOp::Open(node) => match node.inner {
          napi::Either::A(doc_type) => serializer.write_doctype(&doc_type.name)?,
          napi::Either::B(element) => {
            serializer.start_elem(
              element.name,
              element
                .attrs
                .borrow()
                .iter()
                .map(|at| (&at.name, &at.value[..])),
            )?;
            ops.reserve(1 + element.children.child_nodes.len());
            ops.push_back(SerializeOp::Close(element.name));

            for child in element.children.child_nodes.iter().rev() {
              ops.push_front(SerializeOp::Open(child.clone()));
            }
          }
        },
        SerializeOp::Close(name) => serializer.end_elem(name)?,
      }
    }

    Ok(())
  }
}

pub fn serialize(element: &Element) -> String {
  let serializable_node: SerializableNode = SerializableNode(element);
  let mut serialized = Vec::new();
  html5ever::serialize::serialize(&mut serialized, &serializable_node, Default::default()).unwrap();

  String::from_utf8(serialized).unwrap()
}
