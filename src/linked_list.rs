type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    next: Link,
}

pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, val: i32) {
        let n = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(n);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.val
        })
    }

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

    pub fn split(&mut self, val: i32) -> Option<Self> {
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
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut box_node) = cur {
            cur = box_node.next.take();
        }
    }
}
