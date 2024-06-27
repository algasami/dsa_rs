use crate::linked_list::{LinkedList, LinkedListIterator};

type NodesT<'a> = LinkedList<&'a Node<'a>>;

/**
 * Node<'a>
 * 'a means that the adj references to node refs that live at least as long as 'a
 */
pub struct Node<'a> {
    pub val: i32,
    adj: NodesT<'a>,
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node {}", self.val)
    }
}

impl Node<'_> {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            adj: LinkedList::new(),
        }
    }
}

impl<'a> Node<'a> {
    pub fn add_adj(&mut self, node: &'a Node<'a>) {
        if self.adj.find(node) {
            return;
        }
        self.adj.push(node);
    }

    pub fn iter(&'a self) -> LinkedListIterator<'a, &'a Node<'a>> {
        self.adj.iter()
    }
}
