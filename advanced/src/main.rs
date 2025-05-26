fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r2 is: {}", *r2);
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
