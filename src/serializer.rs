use std::collections::VecDeque;

use html5ever::{
    serialize::{
        SerializeOpts,
        TraversalScope,
    },
    QualName,
};

use crate::{
    ChildNode,
    Node,
    NodeHandler,
};

struct SerializableNodeHandler(Node);

enum SerializeOp {
    Open(ChildNode),
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
                ops.push_back(SerializeOp::Open((&self.0).into()))
            },
            html5ever::serialize::TraversalScope::ChildrenOnly(_) => {
                let node_handler: NodeHandler = (&self.0).into();
                ops.extend(
                    node_handler
                        .shallow_child_nodes_iter()
                        .map(SerializeOp::Open),
                );
            },
        };

        while let Some(op) = ops.pop_front() {
            match op {
                SerializeOp::Open(node) => {
                    match &node {
                        ChildNode::Comment(comment) => {
                            serializer.write_comment(&comment.data)?
                        },
                        ChildNode::DocumentType(doc_type) => {
                            serializer.write_doctype(&doc_type.name)?
                        },
                        ChildNode::Element(element) => {
                            let node_handler = element.get_node_handler();

                            node_handler
                                .child_nodes
                                .borrow::<_, std::io::Result<()>>(
                                    |child_nodes| {
                                        serializer.start_elem(
                                            // TODO: Is this actually copying the data? Need to figure that out
                                            element.name.clone(),
                                            element
                                                .attributes_wrapper
                                                .iter()
                                                .map(|at| {
                                                    (&at.name, &at.value[..])
                                                }),
                                        )?;
                                        ops.reserve(1 + child_nodes.len());
                                        ops.push_front(SerializeOp::Close(
                                            element.name.clone(),
                                        ));

                                        for child in child_nodes.iter().rev() {
                                            ops.push_front(SerializeOp::Open(
                                                child.clone(),
                                            ));
                                        }
                                        Ok(())
                                    },
                                )?;
                        },
                        ChildNode::Text(text) => {
                            serializer.write_text(&text.data)?
                        },
                    }
                },
                SerializeOp::Close(name) => serializer.end_elem(name)?,
            }
        }

        Ok(())
    }
}

pub fn serialize(
    handle: Node,
    traversal_scope: TraversalScope,
) -> String {
    let serializable_node_handler: SerializableNodeHandler =
        SerializableNodeHandler(handle);
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
