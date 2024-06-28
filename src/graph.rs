use core::fmt;
use std::collections::{HashMap, HashSet};

pub type NodeT = usize;
pub struct Graph<T> {
    nodes: HashMap<NodeT, Box<GraphNode<T>>>,
    current_id: NodeT,
}

struct GraphNode<T> {
    val: T,
    edges: HashSet<NodeT>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn add_node(&mut self, val: T) -> NodeT {
        let node_id = self.current_id;
        self.nodes.insert(
            node_id,
            Box::new(GraphNode {
                val,
                edges: HashSet::new(),
            }),
        );
        self.current_id += 1;
        node_id
    }

    pub fn is_dangling(&self, key: &NodeT) -> bool {
        !self.nodes.contains_key(key)
    }

    pub fn add_edge(&mut self, key1: NodeT, key2: NodeT) -> Result<(), ()> {
        if self.is_dangling(&key1) || self.is_dangling(&key2) {
            return Err(());
        }
        let Some(node1) = self.nodes.get_mut(&key1) else {
            return Err(());
        };
        node1.edges.insert(key2);
        let Some(node2) = self.nodes.get_mut(&key2) else {
            return Err(());
        };
        node2.edges.insert(key1);
        Ok(())
    }

    pub fn remove_edge(&mut self, key1: &NodeT, key2: &NodeT) {
        if let Some(node1) = self.nodes.get_mut(&key1) {
            node1.edges.remove(&key2);
        };
        if let Some(node2) = self.nodes.get_mut(&key1) {
            node2.edges.remove(&key1);
        };
    }

    pub fn remove_node(&mut self, key: &NodeT) {
        let Some(node1) = self.nodes.remove(key) else {
            return;
        };
        for key2 in node1.edges {
            let Some(node2) = self.nodes.get_mut(&key2) else {
                continue;
            };
            node2.edges.remove(key);
        }
    }
}

impl<T: fmt::Display> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, node) in &self.nodes {
            writeln!(f, "Node {} - Val {}", key, &node.val)?;
            writeln!(f, "--------")?;
            for to_node in &node.edges {
                writeln!(f, "{}", to_node)?;
            }
        }
        Ok(())
    }
}
