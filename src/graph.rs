use std::vec;

type NodesT<'a, T> = Vec<&'a Node<'a, T>>;

/**
 * Node<'a>
 * 'a means that the adj references to node refs that live at least as long as 'a
 */
pub struct Node<'a, T> {
    pub key: i32,
    pub val: T,
    pub adj: NodesT<'a, T>,
}

impl<T> PartialEq for Node<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> std::fmt::Display for Node<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node {}", self.key)
    }
}

impl<T> Node<'_, T> {
    pub fn new(key: i32, val: T) -> Self {
        Node {
            key,
            val,
            adj: vec![],
        }
    }
}

impl<'a, T> Node<'a, T> {
    pub fn add_adj(&mut self, node: &'a Node<'a, T>) {
        if self.adj.iter().find(|x| ***x == *node).is_some() {
            return;
        }
        self.adj.push(node);
    }

    pub fn remove_adj(&mut self, node: &'a Node<'a, T>) {
        if let Some(index) = self.adj.iter().position(|x| **x == *node) {
            self.adj.remove(index);
        }
    }
}
