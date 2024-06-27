pub mod binary_search_tree;
pub mod graph;
pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::{binary_search_tree::BSTNode, graph::Node, linked_list::LinkedList};

    #[test]
    fn push_test() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.print(), 3);
    }

    #[test]
    fn split_test() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        ll.push(4);

        let ll2 = ll.split(2).expect("ll2 should not be None!");

        assert_eq!(ll.print(), 2);
        assert_eq!(ll2.print(), 2);
    }

    #[test]
    fn nodes() {
        let n2 = Box::new(Node::new(2, 2));
        let mut n1 = Box::new(Node::new(1, 1));

        n1.add_adj(&*n2);
        n1.remove_adj(&*n2);
        assert_eq!(n1.adj.len(), 0);
    }

    #[test]
    fn bin_tree() {
        let mut n1 = BSTNode::new(1, 3);
        let mut n2 = Box::new(BSTNode::new(2, 4));
        let mut n3 = BSTNode::new(0, 0);
        n1.insert(&mut n2); // Box has auto deref (*x) to act as if it is on the stack
        n1.insert(&mut n3);
        println!("{}", n1.left.is_some());
        println!("{}", n1.right.is_some());
    }
}
