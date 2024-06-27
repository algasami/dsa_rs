pub struct BSTNode<'a, T> {
    pub left: Option<&'a mut BSTNode<'a, T>>,
    pub right: Option<&'a mut BSTNode<'a, T>>,
    pub key: i32,
    pub val: T,
}

impl<T> PartialEq for BSTNode<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> BSTNode<'_, T> {
    pub fn new(key: i32, val: T) -> Self {
        Self {
            left: None,
            right: None,
            key,
            val,
        }
    }
}

impl<'a, T> BSTNode<'a, T> {
    pub fn insert(&mut self, node_ref: &'a mut BSTNode<'a, T>) {
        assert_ne!(
            self.key, node_ref.key,
            "Binary Tree nodes should have unique ids/keys!"
        );
        if self.key > node_ref.key {
            if let Some(left_node) = self.left.as_mut() {
                // (**left_node) deref coercion
                left_node.insert(node_ref);
            } else {
                self.left = Some(node_ref);
            }
        } else if let Some(right_node) = self.right.as_mut() {
            right_node.insert(node_ref);
        } else {
            self.right = Some(node_ref);
        }
    }
    pub fn remove(&mut self, node_ref: &'a BSTNode<'a, T>) {
        unimplemented!();
    }
}
