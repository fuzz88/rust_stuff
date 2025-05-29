use std::cmp::PartialOrd;
use std::fmt::Display;

struct Heap<T> {
    items: Vec<T>,
}

impl<T: PartialOrd + Display + Copy> Heap<T> {
    fn new() -> Heap<T> {
        Heap { items: vec![] }
    }

    fn new_from(data: &Vec<T>) -> Heap<T> {
        Heap {
            items: data.to_owned(),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
        self.sift_down(0, self.items.len());
    }

    fn sift_down(&mut self, start_pos: usize, pos: usize) {
        println!("{} {} siftdown", start_pos, pos);
        let mut i = pos;
        let mut p = i / 2;
        while p > start_pos && self.items[i - 1] < self.items[p - 1] {
            println!("{} {} swap", i - 1, p - 1);
            self.items.swap(i - 1, p - 1);
            i = p;
            p = i / 2;
        }
    }

    fn pop(&mut self) -> T {
        println!("popping");
        let item = self.items[0];
        self.items[0] = self.items.pop().expect("expecting non empty");
        self.sift_up(0);
        item
    }

    fn sift_up(&mut self, start: usize) {
        println!("{} siftup", start);
        let mut i = start;
        let n = self.items.len();

        let mut l = 2 * i + 1;
        let mut r = l + 1;

        if r < n && self.items[r] < self.items[l] {
            l = r;
        }
        while l < n && self.items[l] < self.items[i] {
            self.items.swap(i, l);
            println!("{} {} swap", i, l);
            i = l;
            l = i * 2 + 1;
            r = l + 1;
            if r < n && self.items[r] < self.items[l] {
                l = r;
            }
        }
    }

    fn heapify(&mut self) {
        let n = self.items.len();
        for i in (0..n / 2).rev() {
            self.sift_up(i);
        }
    }

    fn print(&self) {
        let n = self.items.len() as u32;

        let mut nk = 0;
        while n > u32::pow(2, nk) {
            nk += 1;
        }
        let width = nk * 10;

        let mut k = 1;
        for (idx, item) in self.items.iter().enumerate() {
            while !((idx as u32 + 1) < u32::pow(2, k)) {
                k += 1;
                println!();
                println!();
            }
            let mut space_count = width / u32::pow(2, k - 1);
            let elements = u32::pow(2, k - 1);
            if (idx + 1) as u32 % elements != 0 {
                space_count *= 2;
            }
            for _ in 0..space_count - 2 {
                print!(" ");
            }
            print!("{}", item);
        }
        println!();
        println!("----------------------------------------------------------------------");
    }
}

fn main() {
    let mut heap: Heap<u32> = Heap::new();

    heap.push(3);
    heap.push(1);
    heap.push(10);
    heap.push(8);
    heap.push(5);
    heap.push(6);
    heap.push(2);
    heap.push(7);
    heap.push(4);
    heap.push(9);

    heap.print();

    println!("'{}' popped", heap.pop());

    heap.print();

    let numbers = vec![33, 9, 5, 1, 8, 2, 9, 5, 1, 8, 2];

    let mut numbers_heap = Heap::new_from(&numbers);

    numbers_heap.print();

    numbers_heap.heapify();

    numbers_heap.print();
}
