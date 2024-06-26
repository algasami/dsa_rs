use crate::linked_list::{LinkedList, LinkedListIterator};

type NodesT<'a> = LinkedList<&'a Node<'a>>;

pub struct Node<'a> {
    pub val: i32,
    adj: NodesT<'a>,
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node {}", self.val)
    }
}

impl<'a> Node<'a> {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            adj: LinkedList::new(),
        }
    }
    pub fn add_adj(&mut self, node: &'a Node<'a>) {
        if self.adj.find(&self) {
            return;
        }
        self.adj.push(node);
    }

    pub fn iter(&'a self) -> LinkedListIterator<'a, &'a Node<'a>> {
        self.adj.iter()
    }
}
