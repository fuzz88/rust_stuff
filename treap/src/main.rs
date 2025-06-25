use std::cmp::Ord;

use rand;

#[derive(Debug, Clone)]
struct Node<T: Ord + Clone> {
    key: T,
    priority: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(key: T) -> Self {
        let priority = rand::random_range(0..1024);
        Node {
            key,
            priority,
            left: None,
            right: None,
        }
    }

    fn search(&self, key: T) -> Option<Box<Node<T>>> {
        if key < self.key {
            if let Some(left) = &self.left {
                left.search(key)
            } else {
                None
            }
        } else if key > self.key {
            if let Some(right) = &self.right {
                right.search(key)
            } else {
                None
            }
        } else {
            Some(Box::new((*self).clone()))
        }
    }
}

fn main() {
    let root = Node::new(10);

    println!("{root:?}");

    let found = root.search(10);

    println!("{found:?}");
}
