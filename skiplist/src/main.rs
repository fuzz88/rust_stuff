use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    key: Option<T>,                              // None for head sentinel
    forwards: Vec<Option<Rc<RefCell<Node<T>>>>>, // one slot per level
}

impl<T> Node<T> {
    fn new(key: Option<T>, max_level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            forwards: vec![None; max_level],
        }))
    }
}

struct SkipList<T> {
    head: Rc<RefCell<Node<T>>>,
    max_level: usize,
    p: f64,
    level: usize,
    rng: rand::rngs::ThreadRng,
}

impl<T: Ord + Clone> SkipList<T> {
    fn new(max_level: usize, p: f64) -> Self {
        SkipList {
            head: Node::new(None, max_level),
            max_level,
            p,
            level: 0,
            rng: rand::rng(),
        }
    }

    // return a level in 0..max_level-1 (0 = lowest)
    fn random_level(&mut self) -> usize {
        let mut lvl = 0;
        while self.rng.r#gen::<f64>() < self.p && (lvl + 1) < self.max_level {
            lvl += 1;
        }
        lvl
    }

    fn contains(&self, key: &T) -> bool {
        let mut x = Rc::clone(&self.head);
        // from top level down
        for i in (0..=self.level).rev() {
            loop {
                let next = x.borrow().forwards[i].as_ref().map(Rc::clone);
                match next {
                    Some(next_rc) => {
                        // unwrap is safe: only head has None key, forward nodes have Some
                        let nk = next_rc.borrow().key.as_ref().unwrap().clone();
                        if nk < *key {
                            x = next_rc;
                            continue;
                        }
                    }
                    None => {}
                }
                break;
            }
        }
        // check candidate at level 0
        if let Some(next) = x.borrow().forwards[0].as_ref().map(Rc::clone) {
            if let Some(k) = next.borrow().key.as_ref() {
                return k == key;
            }
        }
        false
    }

    fn insert(&mut self, key: T) {
        // update[i] will point to the node after which we should insert at level i
        let mut update: Vec<Rc<RefCell<Node<T>>>> = vec![Rc::clone(&self.head); self.max_level];
        let mut x = Rc::clone(&self.head);

        // find predecessors
        for i in (0..=self.level).rev() {
            loop {
                let next = x.borrow().forwards[i].as_ref().map(Rc::clone);
                if let Some(next_rc) = next {
                    let nk = next_rc.borrow().key.as_ref().unwrap().clone();
                    if nk < key {
                        x = next_rc;
                        continue;
                    }
                }
                break;
            }
            update[i] = Rc::clone(&x);
        }

        // check if present at level 0
        if let Some(next_rc) = x.borrow().forwards[0].as_ref().map(Rc::clone) {
            if next_rc.borrow().key.as_ref().unwrap() == &key {
                // already present; for now do nothing (could replace value if storing values)
                return;
            }
        }

        let lvl = self.random_level();
        if lvl > self.level {
            for i in (self.level + 1)..=lvl {
                update[i] = Rc::clone(&self.head);
            }
            self.level = lvl;
        }

        let new_node = Node::new(Some(key), self.max_level);
        for i in 0..=lvl {
            // splice: new.next = update[i].next; update[i].next = Some(new)
            let next = update[i].borrow().forwards[i].as_ref().map(Rc::clone);
            new_node.borrow_mut().forwards[i] = next;
            update[i].borrow_mut().forwards[i] = Some(Rc::clone(&new_node));
        }
    }

    fn remove(&mut self, key: &T) -> bool {
        let mut update: Vec<Rc<RefCell<Node<T>>>> = vec![Rc::clone(&self.head); self.max_level];
        let mut x = Rc::clone(&self.head);

        for i in (0..=self.level).rev() {
            loop {
                let next = x.borrow().forwards[i].as_ref().map(Rc::clone);
                if let Some(next_rc) = next {
                    let nk = next_rc.borrow().key.as_ref().unwrap().clone();
                    if nk < *key {
                        x = next_rc;
                        continue;
                    }
                }
                break;
            }
            update[i] = Rc::clone(&x);
        }

        let candidate_opt = x.borrow().forwards[0].as_ref().map(Rc::clone);
        if let Some(candidate) = candidate_opt {
            if candidate.borrow().key.as_ref().unwrap() == key {
                for i in 0..=self.level {
                    // if update[i].forwards[i] == candidate, unlink it
                    if let Some(ref u_fwd) = update[i].borrow().forwards[i] {
                        if Rc::ptr_eq(u_fwd, &candidate) {
                            let next = candidate.borrow().forwards[i].as_ref().map(Rc::clone);
                            update[i].borrow_mut().forwards[i] = next;
                        }
                    }
                }
                // reduce level if top levels are empty
                while self.level > 0 && self.head.borrow().forwards[self.level].is_none() {
                    self.level -= 1;
                }
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut sl = SkipList::new(16, 0.5);
    sl.insert(10);
    sl.insert(20);
    sl.insert(15);
    println!("contains 15? {}", sl.contains(&15));
    println!("contains 17? {}", sl.contains(&17));
}
