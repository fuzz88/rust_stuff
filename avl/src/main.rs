use std::fmt::Debug;

type NodeRef<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    height: usize,
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
}

impl<K: Debug, V: Debug> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key: key, 
            value: value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn in_order(&self, visit: &mut dyn FnMut(&K, &V)) {
        if let Some(ref left) = self.left {
            left.in_order(visit);
        }
        visit(&self.key, &self.value);
        if let Some(ref right) = self.right {
            right.in_order(visit);
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

    #[test]
    fn test_visit_all_nodes() {

    }
}

fn main() {
    let mut root = Some(Box::new(Node::new(10, "hey, boy!".to_string())));

    if let Some(ref mut r) = root {
        r.in_order(&mut |key, value| println!("{key}: {value}"));
    }
}
