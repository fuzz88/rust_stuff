use std::cmp::PartialOrd;
use std::fmt::Display;

struct Heap<T> {
    items: Vec<T>,
}

impl<T: PartialOrd + Display> Heap<T> {
    fn new() -> Heap<T> {
        Heap { items: vec![] }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
        self.sift_up();
    }

    fn sift_up(&mut self) {
        let mut i = self.items.len();
        let mut p = i / 2;
        while p > 0 && self.items[i - 1] > self.items[p - 1] {
            self.items.swap(i - 1, p - 1);
            i = p;
            p = i / 2;
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
            for _ in 0..space_count - 1 {
                print!(" ");
            }
            print!("{}", item);
        }
        println!();
    }
}

fn main() {
    let mut heap: Heap<u32> = Heap::new();

    for i in 0..=9 {
        heap.push(i);
        heap.push(i);
    }

    println!();

    heap.print();
}
