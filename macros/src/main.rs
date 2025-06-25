macro_rules! box_it {
    ($value:literal) => {
        Box::new($value)
    };
}

fn main() {
    let stuff = box_it!("hello, world");
    println!("{stuff:?}");
}
