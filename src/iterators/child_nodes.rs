use std::{
    collections::VecDeque,
    marker::PhantomData,
};

use crate::{
    ChildNode,
    NodeData,
};

pub struct DeepChildNodesIterator<T> {
    queue: Vec<ChildNode>,
    _phantom: PhantomData<T>,
}

impl<T> DeepChildNodesIterator<T> {
    pub fn new(node_data: NodeData) -> Self {
        let queue = ShallowChildNodesIterator::<ChildNode>::new(node_data)
            .rev()
            .collect();

        Self {
            queue,
            _phantom: PhantomData,
        }
    }
}

impl<T> Iterator for DeepChildNodesIterator<T>
where
    ChildNode: TryInto<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop() {
            if let ChildNode::Element(r) = &node {
                let node_data = r.get_node_data();
                self.queue.extend(
                    ShallowChildNodesIterator::<ChildNode>::new(node_data)
                        .rev(),
                );
            }

            if let Ok(child) = node.try_into() {
                return Some(child);
            }
        }
        None
    }
}

pub struct ShallowChildNodesIterator<T> {
    queue: VecDeque<ChildNode>,
    _phantom: PhantomData<T>,
}

impl<T> ShallowChildNodesIterator<T> {
    pub fn new(node_data: NodeData) -> Self {
        let queue = node_data
            .child_nodes
            .borrow(|child_nodes| child_nodes.iter().cloned().collect());

        Self {
            queue,
            _phantom: PhantomData,
        }
    }
}

impl<T> Iterator for ShallowChildNodesIterator<T>
where
    ChildNode: TryInto<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_front() {
            if let Ok(child) = node.try_into() {
                return Some(child);
            }
        }
        None
    }
}

impl<T> DoubleEndedIterator for ShallowChildNodesIterator<T>
where
    ChildNode: TryInto<T>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_back() {
            if let Ok(child) = node.try_into() {
                return Some(child);
            }
        }
        None
    }
}
