type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

pub struct LinkedListIterator<'a, T> {
    current: &'a Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, val: T) {
        let n = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(n);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.val
        })
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: &self.head,
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.current {
            self.current = &x.next;
            Some(&x.val)
        } else {
            None
        }
    }
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    pub fn find(&self, val: T) -> bool {
        for x in self.iter() {
            if *x == val {
                return true;
            }
        }
        false
    }
    pub fn split(&mut self, val: T) -> Option<Self> {
        let mut cur = &mut self.head;
        while cur.is_some() {
            if cur.as_ref().unwrap().val == val {
                let mut ll = LinkedList::new();
                ll.head = cur.take();
                return Some(ll);
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        None
    }
    pub fn remove(&mut self, _val: T) {
        /*
        Linked List is difficult to implement in Rust due to its unsafe nature.
        */
        unimplemented!()
    }
}

impl<T: std::fmt::Display> LinkedList<T> {
    pub fn print(&self) -> i32 {
        let mut cur = &self.head;
        let mut size: i32 = 0;
        while let Some(box_node) = cur {
            println!("{}", box_node.val);
            cur = &box_node.next;
            size += 1;
        }
        size
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut box_node) = cur {
            cur = box_node.next.take();
        }
    }
}
