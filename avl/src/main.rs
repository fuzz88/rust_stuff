use std::clone::Clone;
use std::cmp::max;
use std::fmt::Debug;
use std::mem;

type NodeRef<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Clone)]
struct Node<K, V>
where
    K: Clone,
    V: Clone,
{
    key: K,
    value: V,
    height: u8,
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key: key,
            value: value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn in_order(&self, visit: &mut dyn FnMut(&K, &V, u8)) {
        if let Some(ref left) = self.left {
            left.in_order(visit);
        }
        visit(&self.key, &self.value, self.height);
        if let Some(ref right) = self.right {
            right.in_order(visit);
        }
    }

    fn print_keys(&self, level: usize) {
        if let Some(left) = &self.left {
            left.print_keys(level + 1);
        }

        (0..level).for_each(|_| print!("\t"));
        println!("{:?}", self.key);

        if let Some(right) = &self.right {
            right.print_keys(level + 1);
        }
    }

    fn minimum(&self) -> &Node<K, V> {
        let mut current = self;
        while let Some(minimum) = &current.left {
            current = &minimum;
        }
        current
    }

    fn maximum(&self) -> &Node<K, V> {
        let mut current = self;
        while let Some(maximum) = &current.right {
            current = &maximum;
        }
        current
    }

    fn bfactor(&self) -> i8 {
        match (&self.left, &self.right) {
            (None, None) => 0,
            (Some(left), Some(right)) => right.height as i8 - left.height as i8,
            (Some(left), None) => -(left.height as i8),
            (None, Some(right)) => right.height as i8,
        }
    }

    fn fix_height(&mut self) {
        match (&self.left, &self.right) {
            (None, None) => self.height = 1,
            (Some(left), Some(right)) => self.height = max(left.height, right.height) + 1,
            (Some(left), None) => self.height = left.height + 1,
            (None, Some(right)) => self.height = right.height + 1,
        }
    }

    fn rotate_right(&mut self) {
        let mut left = self.left.take().unwrap();

        self.left = left.right.take();

        let mut new_root = *left;
        mem::swap(&mut self.key, &mut new_root.key);
        mem::swap(&mut self.value, &mut new_root.value);
        mem::swap(&mut self.left, &mut new_root.left);
        mem::swap(&mut self.right, &mut new_root.right);

        self.right = Some(Box::new(new_root.clone()));

        if let Some(ref mut right) = self.right {
            right.fix_height();
        }
        self.fix_height();
        println!("rotated right");
    }

    fn rotate_left(&mut self) {
        let mut right = self.right.take().unwrap();

        self.right = right.left.take();

        let mut new_root = *right;
        mem::swap(&mut self.key, &mut new_root.key);
        mem::swap(&mut self.value, &mut new_root.value);
        mem::swap(&mut self.left, &mut new_root.left);
        mem::swap(&mut self.right, &mut new_root.right);

        self.left = Some(Box::new(new_root.clone()));

        if let Some(ref mut left) = self.left {
            left.fix_height();
        }
        self.fix_height();
        println!("rotated left");
    }

    fn balance(&mut self) {
        self.fix_height();

        if self.bfactor() == 2 {
            if let Some(ref mut right) = self.right {
                if right.bfactor() < 0 {
                    right.rotate_right();
                }
            }
            self.rotate_left();
        } else if self.bfactor() == -2 {
            if let Some(ref mut left) = self.left {
                if left.bfactor() > 0 {
                    left.rotate_left();
                }
            }
            self.rotate_right();
        }
    }

    fn insert(&mut self, key: K, value: V) {
        if key < self.key {
            match self.left {
                Some(ref mut left_child) => {
                    left_child.insert(key, value);
                }
                None => {
                    self.left = Some(Box::new(Node::new(key, value)));
                }
            }
        } else if key > self.key {
            match self.right {
                Some(ref mut right_child) => {
                    right_child.insert(key, value);
                }
                None => {
                    self.right = Some(Box::new(Node::new(key, value)));
                }
            }
        } else {
            self.value = value;
        }
        self.balance();
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(7, ("hey", "there"));
        assert_eq!(node.key, 7);
        assert_eq!(node.value.0, "hey");
    }

    #[test]
    fn test_minimal_node() {
        let mut node = Node::new(3, 7);
        assert_eq!(node.minimum().key, 3);

        node.left = Some(Box::new(Node::new(2, 19)));
        assert_eq!(node.minimum().key, 2);
    }

    #[test]
    fn test_maximal_node() {
        let mut node = Node::new(3, 7);
        assert_eq!(node.maximum().key, 3);

        node.right = Some(Box::new(Node::new(4, 20)));
        assert_eq!(node.maximum().key, 4);

        if let Some(ref mut right) = node.right {
            right.right = Some(Box::new(Node::new(10, 50)));
        }
        assert_eq!(node.maximum().key, 10);
    }
}

macro_rules! node {
    ($key:literal, $value:literal) => {
        Some(Box::new(Node::new($key, $value.to_string())))
    };
}

fn main() {
    let mut root = Box::new(Node::new(10, "hey, boy!".to_string()));

    root.left = node!(5, "yes");
    root.right = node!(13, "boom");

    if let Some(ref mut right) = root.right {
        right.left = node!(11, "go go");
        right.right = node!(14, "booyaka");
    }

    root.in_order(&mut |key, value, h| println!("{key}: {value} -- {h}"));
    root.print_keys(0);

    root.rotate_left();

    root.in_order(&mut |key, value, h| println!("{key}: {value} -- {h}"));
    root.print_keys(0);

    root.rotate_right();

    root.in_order(&mut |key, value, h| println!("{key}: {value} -- {h}"));
    root.print_keys(0);

    root.insert(18, "hello".to_string());

    root.in_order(&mut |key, value, h| println!("{key}: {value} -- {h}"));
    root.print_keys(0);
}
