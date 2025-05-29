use std::cmp::PartialOrd;
use std::fmt::Display;

struct Heap<T> {
    items: Vec<T>,
}

impl<T: PartialOrd + Display + Clone> Heap<T> {
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
        let mut i = pos;
        let mut p = i / 2;
        while p > start_pos && self.items[i - 1] < self.items[p - 1] {
            self.items.swap(i - 1, p - 1);
            i = p;
            p = i / 2;
        }
    }

    fn pop(&mut self) -> T {
        let item = self.items[0].clone();
        self.items[0] = self.items.pop().expect("expecting non empty");
        self.sift_up(0);
        item
    }

    fn sift_up(&mut self, start: usize) {
        let mut current = start;
        let last = self.items.len() - 1;
        loop {
            let child1 = 2 * current + 1;
            let child2 = child1 + 1;
            let child_to_swap_with;

            if child1 <= last {
                if child2 <= last {
                    child_to_swap_with = if self.items[child1] < self.items[child2] {
                        child1
                    } else {
                        child2
                    };
                } else {
                    child_to_swap_with = child1;
                }
            } else {
                break;
            // # We *could* break out of the loop as soon as we find a pos where newitem <=
            // # both its children, but turns out that's not a good idea, and despite that
            // # many books write the algorithm that way.  During a heap pop, the last array
            // # element is sifted in, and that tends to be large, so that comparing it
            // # against values starting from the root usually doesn't pay (= usually doesn't
            // # get us out of the loop early).  See Knuth, Volume 3, where this is
            // # explained and quantified in an exercise.
            }

            self.items.swap(current, child_to_swap_with);
            current = child_to_swap_with;
        }
    }

    fn heapify(&mut self) {
        let n = self.items.len();
        for i in (0..n).rev() {
            self.sift_down(0, i);
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
    }
}

fn main() {
    let mut heap: Heap<u32> = Heap::new();

    for i in 1..=12 {
        heap.push(i);
    }

    heap.print();

    heap.pop();

    heap.print();

    let numbers = vec![3, 9, 2, 1, 8, 5];

    let mut numbers_heap = Heap::new_from(&numbers);

    numbers_heap.print();

    numbers_heap.heapify();

    numbers_heap.print();
}
