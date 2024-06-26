pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

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

        println!("ll:");
        assert_eq!(ll.print(), 2);
        println!("ll2:");
        assert_eq!(ll2.print(), 2);
    }
}
