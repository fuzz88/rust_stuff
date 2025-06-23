#[derive(Debug)]
struct TreeNode<T: Ord + Clone> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: T) {
        if val < self.value {
            match self.left {
                Some(ref mut left) => left.insert(val),
                None => self.left = Some(Box::new(TreeNode::new(val))),
            }
        } else if val > self.value {
            match self.right {
                Some(ref mut right) => right.insert(val),
                None => self.right = Some(Box::new(TreeNode::new(val))),
            }
        }
    }

    fn in_order(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(ref left) = self.left {
            left.in_order(visit);
        }
        visit(&self.value);
        if let Some(ref right) = self.right {
            right.in_order(visit);
        }
    }

    fn delete(node: &mut Option<Box<TreeNode<T>>>, key: T) {
        if let Some(n) = node {
            if key < n.value {
                Self::delete(&mut n.left, key);
            } else if key > n.value {
                Self::delete(&mut n.right, key);
            } else {
                match (n.left.take(), n.right.take()) {
                    (None, None) => {
                        *node = None;
                    }
                    (Some(child), None) | (None, Some(child)) => {
                        *node = Some(child);
                    }
                    (Some(left), Some(right)) => {
                        // Find min in right subtree
                        let min_value = Self::find_min(&right);
                        n.value = min_value;
                        n.left = Some(left);
                        n.right = Some(right);
                        Self::delete(&mut n.right, n.value.clone());
                    }
                }
            }
        }
    }

    fn find_min(node: &Box<TreeNode<T>>) -> T
    where
        T: Clone,
    {
        let mut current = node;
        while let Some(ref left) = current.left {
            current = left;
        }
        current.value.clone()
    }
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(10)));
    if let Some(ref mut r) = root {
        r.insert(5);
        r.insert(15);
        r.insert(3);
        r.insert(7);
        r.insert(13);
        r.insert(17);
    }

    println!("Before deletion:");
    if let Some(ref r) = root {
        r.in_order(&mut |v| print!("{} ", v));
    }
    println!();

    TreeNode::delete(&mut root, 10); // Delete node with two children

    println!("After deletion of 10:");
    if let Some(ref r) = root {
        r.in_order(&mut |v| print!("{} ", v));
    }
    println!();
}

