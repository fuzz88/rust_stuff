use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::Debug;

use rand;

#[derive(Debug, Clone)]
struct Node<T: Ord + Clone + Debug> {
    key: T,
    priority: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone + Debug> Node<T> {
    fn new(key: T) -> Self {
        let priority = rand::random_range(0..1024);
        Node {
            key,
            priority,
            left: None,
            right: None,
        }
    }

    fn search(node: &Option<Box<Node<T>>>, key: T) -> Option<Box<Node<T>>> {
        if let Some(n) = node {
            if key < n.key {
                return Self::search(&n.left, key);
            } else if key > n.key {
                return Self::search(&n.right, key);
            } else {
                return node.clone();
            }
        }
        None
    }

    fn split(node: Option<Box<Node<T>>>, key: T) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        if let Some(mut n) = node {
            if key < n.key {
                let (left, right) = Self::split(n.left.take(), key);
                n.left = right;
                (left, Some(n))
            } else {
                let (left, right) = Self::split(n.right.take(), key);
                n.right = left;
                (Some(n), right)
            }
        } else {
            (None, None)
        }
    }

    fn merge(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match (left, right) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.priority > r.priority {
                    l.right = Self::merge(l.right.take(), Some(r));
                    Some(l)
                } else {
                    r.left = Self::merge(Some(l), r.left.take());
                    Some(r)
                }
            }
        }
    }

    fn insert(node: Option<Box<Node<T>>>, new_node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        if let Some(mut n) = node {
            if new_node.priority > n.priority {
                let (left, right) = Self::split(Some(n), new_node.key.clone());
                let mut new_n = new_node;
                new_n.left = left;
                new_n.right = right;
                Some(new_n)
            } else if new_node.key < n.key {
                n.left = Self::insert(n.left.take(), new_node);
                Some(n)
            } else {
                n.right = Self::insert(n.right.take(), new_node);
                Some(n)
            }
        } else {
            Some(new_node)
        }
    }
    fn erase(node: Option<Box<Node<T>>>, key: T) -> Option<Box<Node<T>>> {
        if let Some(mut n) = node {
            match key.cmp(&n.key) {
                Ordering::Less => {
                    n.left = Self::erase(n.left.take(), key);
                    Some(n)
                }
                Ordering::Greater => {
                    n.right = Self::erase(n.right.take(), key);
                    Some(n)
                }
                Ordering::Equal => Self::merge(n.left.take(), n.right.take()),
            }
        } else {
            None
        }
    }

    fn print_inorder(node: &Option<Box<Node<T>>>) {
        if let Some(n) = node {
            Self::print_inorder(&n.left);
            println!("key: {:?}, priority: {}", n.key, n.priority);
            Self::print_inorder(&n.right);
        }
    }
}

fn main() {
    let mut root: Option<Box<Node<i32>>> = None;

    let values = vec![10, 5, 15, 3, 7, 12, 18];

    for val in values {
        let new_node = Box::new(Node::new(val));
        root = Node::insert(root, new_node);
    }

    println!("Inorder traversal (by key):");
    Node::print_inorder(&root);

    let search_key = 7;
    if let Some(found) = Node::search(&root, search_key) {
        println!(
            "Found node with key {}: priority {}",
            found.key, found.priority
        );
    } else {
        println!("Key {} not found.", search_key);
    }

    root = Node::erase(root, 15);
    Node::print_inorder(&root);
}
