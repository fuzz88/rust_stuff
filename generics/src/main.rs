use std::fmt::Debug;

fn printer<T: Debug>(item: T) {
    dbg!(item);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> Point<T> {
    fn echo(&self) {
        dbg!(&self);
    }
}


fn main() {
    println!("Hello, world!");
    printer(1);
    printer("boo");
    printer(());

    Point{ x: 1.0, y: 1.0 }.echo();
}
