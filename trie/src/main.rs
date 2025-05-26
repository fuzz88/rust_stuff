use std::cell::RefCell;
use std::rc::Rc;

type TrieNode = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    children: Vec<TrieNode>,
    is_terminal: bool,
    value: Option<char>,
}

struct Trie {
    root: TrieNode,
    current_node: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        let root = Rc::new(RefCell::new(Node {
            children: vec![],
            is_terminal: false,
            value: None,
        }));

        Trie {
            root: Rc::clone(&root),
            current_node: Rc::clone(&root),
        }
    }

    fn insert(&mut self, word: &str) {
        self.current_node = Rc::clone(&self.root);
        for ch in word.chars() {
            println!("{}", ch);
            self.current_node = self.insert_letter(ch);
            println!("{:p}", self.current_node);
        }
        (*self.current_node.borrow_mut()).is_terminal = true;

        println!("-----");
    }

    fn insert_letter(&self, letter: char) -> TrieNode {
        for node in &(*(*self.current_node).borrow()).children {
            match (*node.borrow()).value {
                Some(l) if l == letter => {
                    return Rc::clone(node);
                }
                _ => {
                    continue;
                }
            };
        }

        let new_node = &Rc::new(RefCell::new(Node {
            children: vec![],
            is_terminal: false,
            value: Some(letter),
        }));

        println!("new node: {:p}", Rc::clone(new_node));

        (*(*self.current_node).borrow_mut())
            .children
            .push(Rc::clone(new_node));

        Rc::clone(&new_node)
    }

    fn print_node(&self, node: TrieNode, shift: &str) {
        println!(
            "{shift}{:?} {}",
            (*node.borrow()).value,
            (*node.borrow()).is_terminal,
        );
        for current_node in &(*node.borrow()).children {
            self.print_node(Rc::clone(current_node), &(shift.to_owned() + "  "));
        }
    }

    fn search(&mut self, word: &str) -> bool {
        self.current_node = Rc::clone(&self.root);
        'chars: for ch in word.chars() {
            let children = &(*self.current_node).borrow().children.clone();
            for node in children {
                if (*node).borrow().value == Some(ch) {
                    self.current_node = Rc::clone(node);
                    continue 'chars;
                }
            }
            return false;
        }
        (*self.current_node).borrow().is_terminal
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("hello");
    trie.insert("world");
    trie.insert("hell");
    trie.insert("work");
    trie.insert("hemp");
    trie.insert("hill");
    trie.insert("hi!");

    println!("{}", trie.search("hello"));
    println!("{}", trie.search("hellobabababa"));
    println!("{}", trie.search("work"));
    //println!("\nTrie:\n");
    //trie.print_node(Rc::clone(&trie.root), "");

}
