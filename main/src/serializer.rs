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
};

enum SerializeOp {
    Open(ChildNode),
    Close(QualName),
}

impl html5ever::serialize::Serialize for Node {
    fn serialize<S>(
        &self,
        serializer: &mut S,
        traversal_scope: html5ever::serialize::TraversalScope,
    ) -> std::io::Result<()>
    where
        S: html5ever::serialize::Serializer,
    {
        let mut ops: VecDeque<SerializeOp> = match traversal_scope {
            html5ever::serialize::TraversalScope::IncludeNode => {
                VecDeque::from([SerializeOp::Open((self).into())])
            },
            html5ever::serialize::TraversalScope::ChildrenOnly(_) => {
                if let Node::Element(element) = &self {
                    element
                        .get_all_child_nodes()
                        .iter()
                        .cloned()
                        .map(SerializeOp::Open)
                        .collect()
                } else {
                    self.shallow_child_nodes_iter()
                        .map(SerializeOp::Open)
                        .collect()
                }
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
                            serializer.start_elem(
                                // TODO: Is this actually copying the data? Need to figure that out
                                element.name.clone(),
                                element
                                    .attributes_wrapper
                                    .iter()
                                    .map(|at| (&at.name, &at.value[..])),
                            )?;

                            ops.push_front(SerializeOp::Close(
                                element.name.clone(),
                            ));

                            element
                                .get_all_child_nodes()
                                .into_iter()
                                .rev()
                                .for_each(|child_node| {
                                    ops.push_front(SerializeOp::Open(
                                        child_node,
                                    ))
                                });
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
    node: Node,
    traversal_scope: TraversalScope,
) -> String {
    let mut serialized = Vec::new();
    html5ever::serialize::serialize(
        &mut serialized,
        &node,
        SerializeOpts {
            traversal_scope,
            ..Default::default()
        },
    )
    .unwrap();

    String::from_utf8(serialized).unwrap()
}
