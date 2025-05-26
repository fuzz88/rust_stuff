fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn main() {
    let x = {
        five()
    };
    // shadowed instead of mutated
    let x = plus_one(x); 

    println!("Hello, world!");
    println!("{}", x);
}
