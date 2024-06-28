pub mod binary_search_tree;
pub mod graph;
pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::{binary_search_tree::BST, graph, linked_list::LinkedList};

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
        let mut g: graph::Graph<graph::NodeT> = graph::Graph::new();
        let nodes: graph::NodeT = 5;
        let mut ids: Vec<graph::NodeT> = vec![];
        for x in 0..nodes {
            ids.push(g.add_node(x * 2));
        }
        for i in 0..ids.len() {
            for j in i + 1..ids.len() {
                match g.add_edge(ids[i], ids[j]) {
                    Err(_) => {
                        panic!("Graph::add_edge has failed!");
                    }
                    Ok(_) => {}
                }
            }
        }
        println!("{}", g);
    }

    #[test]
    fn bin_tree_delete() {
        let mut tree: BST<i32> = BST::new(2, 2);
        tree.insert(1, 2);
        tree.insert(3, 3);
        tree.remove(2);
        assert!(tree.find_ref(2).is_none(), "2 should have been deleted!");
    }
}
