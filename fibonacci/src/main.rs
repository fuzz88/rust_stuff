fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    println!("About to figure out fib(42):");
    let result = fib(42);
    println!("{result}");
}
