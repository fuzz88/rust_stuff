struct Heap<T> {
    items: Vec<T>,
}

impl<T> Heap<T> {
    fn new() -> Heap<T> {
        Heap {
            items: vec![],
        }
    }
}

fn main() {
    let heap: Heap<u32> = Heap::new();
}
