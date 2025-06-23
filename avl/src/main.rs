#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: value,
            left: None,
            right: None,
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
}

fn main() {
    let mut root = Some(Box::new(Node::new(10)));

    if let Some(ref mut r) = root {
        r.in_order(&mut |el| println!("{el}"));
    }
}
