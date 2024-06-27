use core::fmt;
use std::cmp::Ordering;

struct BSTNode<T> {
    left: BST<T>,
    right: BST<T>,
    pub key: i32,
    pub val: T,
}

pub struct BST<T> {
    node: Option<Box<BSTNode<T>>>,
}

impl<T> PartialEq for BSTNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> BSTNode<T> {
    fn new(key: i32, val: T) -> Self {
        Self {
            left: BST::new_empty(),
            right: BST::new_empty(),
            key,
            val,
        }
    }
}

impl<T> BST<T> {
    pub fn new_empty() -> Self {
        Self { node: None }
    }
    pub fn new(key: i32, val: T) -> Self {
        Self {
            node: Some(Box::new(BSTNode::new(key, val))),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.node.is_none()
    }
    pub fn insert(&mut self, key: i32, val: T) {
        if let Some(root) = &mut self.node {
            match root.key.cmp(&key) {
                Ordering::Equal => {
                    panic!("BST should not have same id!");
                }
                Ordering::Greater => {
                    root.left.insert(key, val);
                }
                Ordering::Less => {
                    root.right.insert(key, val);
                }
            }
        } else {
            self.node = Some(Box::new(BSTNode::new(key, val)));
        }
    }
}

impl<T: Clone> BST<T> {
    fn extract_min(&mut self) -> Option<Box<BSTNode<T>>> {
        if !self.is_empty() {
            if self.node.as_ref().unwrap().left.is_empty() == false {
                return self.node.as_mut().unwrap().left.extract_min();
            } else {
                return self.node.take();
            }
        }
        return None;
    }

    pub fn remove(&mut self, key: i32) {
        if let Some(root) = &mut self.node {
            match root.key.cmp(&key) {
                Ordering::Equal => match (root.left.node.as_ref(), root.right.node.as_ref()) {
                    (None, None) => {
                        self.node.take();
                    }
                    (Some(_), None) => {
                        self.node = root.left.node.take();
                    }
                    (None, Some(_)) => {
                        self.node = root.right.node.take();
                    }
                    (Some(_), Some(_)) => {
                        if let Some(x) = root.right.extract_min() {
                            root.key = x.key;
                            root.val = x.val.clone();
                        }
                    }
                },
                Ordering::Greater => {
                    root.left.remove(key);
                }
                Ordering::Less => {
                    root.right.remove(key);
                }
            }
        }
    }
}

impl<T: fmt::Display> BST<T> {
    fn fmt_helper(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        if let Some(current_node) = &self.node {
            current_node.left.fmt_helper(f, depth + 1)?;
            write!(f, "{}", ("\t").repeat(depth))?;
            writeln!(f, "node {}", current_node.key)?;
            current_node.right.fmt_helper(f, depth + 1)?;
        } else {
            write!(f, "{}", ("\t").repeat(depth))?;
            return writeln!(f, "leave");
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for BST<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_helper(f, 0)
    }
}
